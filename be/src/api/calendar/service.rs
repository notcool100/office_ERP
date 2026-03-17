use crate::{
    api::{calendar::dto::{CreateCalendarEventDto, ListCalendarEventsQuery, UpdateCalendarEventDto}, user::mailer::Mailer},
    models::{calendar_event::CalendarEvent, user::User},
};
use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

const CALENDAR_NAV_PATH: &str = "/admin/settings/calendar";

#[derive(Debug, thiserror::Error)]
pub enum CalendarError {
    #[error("Event not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, sqlx::FromRow)]
struct PermissionFlags {
    can_create: bool,
    can_read: bool,
    can_update: bool,
    can_delete: bool,
}

impl PermissionFlags {
    fn allows(&self, action: PermissionAction) -> bool {
        match action {
            PermissionAction::Create => self.can_create,
            PermissionAction::Read => self.can_read,
            PermissionAction::Update => self.can_update,
            PermissionAction::Delete => self.can_delete,
        }
    }
}

#[derive(Copy, Clone)]
enum PermissionAction {
    Create,
    Read,
    Update,
    Delete,
}

pub async fn list_events(
    pool: &PgPool,
    user: &User,
    query: ListCalendarEventsQuery,
) -> Result<Vec<CalendarEvent>, CalendarError> {
    let can_read_shared = has_permission(pool, user, PermissionAction::Read).await?;
    let dept_id = get_user_department_id(pool, user.id).await?;

    let mut sql = String::from(
        r#"
        SELECT *
        FROM calendar_events
        WHERE (
            (scope = 'personal' AND created_by = $1)
            OR (scope = 'company')
            OR (scope = 'department' AND (department_id = $2 OR $3 = true))
        )
        "#,
    );

    let mut bind_index = 3;
    let mut params: Vec<NaiveDateTime> = Vec::new();

    if let Some(start) = query.start_date {
        bind_index += 1;
        sql.push_str(&format!(" AND end_at >= ${}", bind_index));
        params.push(start.and_hms_opt(0, 0, 0).unwrap());
    }

    if let Some(end) = query.end_date {
        bind_index += 1;
        sql.push_str(&format!(" AND start_at <= ${}", bind_index));
        params.push(end.and_hms_opt(23, 59, 59).unwrap());
    }

    sql.push_str(" ORDER BY start_at ASC");

    let mut query_builder = sqlx::query_as::<_, CalendarEvent>(&sql)
        .bind(user.id)
        .bind(dept_id)
        .bind(can_read_shared);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    let events = query_builder.fetch_all(pool).await?;

    Ok(events)
}

pub async fn create_event(
    pool: &PgPool,
    user: &User,
    dto: CreateCalendarEventDto,
) -> Result<CalendarEvent, CalendarError> {
    validate_time_range(dto.start_at, dto.end_at)?;

    let scope = normalize_scope(dto.scope);
    let (scope, department_id) = resolve_scope(
        pool,
        user,
        scope,
        dto.department_id,
        PermissionAction::Create,
    )
    .await?;

    let event = sqlx::query_as::<_, CalendarEvent>(
        r#"
        INSERT INTO calendar_events (
            title, description, location, start_at, end_at, all_day,
            scope, department_id, created_by, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
        RETURNING *
        "#,
    )
    .bind(dto.title.trim())
    .bind(dto.description)
    .bind(dto.location)
    .bind(dto.start_at)
    .bind(dto.end_at)
    .bind(dto.all_day.unwrap_or(false))
    .bind(scope)
    .bind(department_id)
    .bind(user.id)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

    // Send notifications
    let event_title = event.title.clone();
    let event_desc = event.description.clone().unwrap_or_default();
    let event_scope = event.scope.clone();
    let event_dept_id = event.department_id;
    let user_email = user.email.clone();
    let pool_clone = pool.clone();

    tokio::spawn(async move {
        println!("[CALENDAR] Starting automated notification for event: {} (Scope: {})", event_title, event_scope);
        match event_scope.as_str() {
            "personal" => {
                let subject = format!("Event Reminder: {}", event_title.as_str());
                let to = user_email.clone();
                let body = event_desc.clone();
                let send_result = tokio::task::spawn_blocking(move || {
                    let mailer = Mailer::new();
                    mailer.send_email(&to, &subject, &body)
                })
                .await;

                match send_result {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => println!("[CALENDAR] Personal email failed: {}", e),
                    Err(e) => println!("[CALENDAR] Personal email task panicked: {}", e),
                }
            }
            "company" => {
                println!("[CALENDAR] Fetching all user emails for company notice");
                let emails = sqlx::query_scalar::<_, String>("SELECT email FROM users WHERE email IS NOT NULL")
                    .fetch_all(&pool_clone)
                    .await
                    .unwrap_or_default();
                println!("[CALENDAR] Sending company notice to {} users", emails.len());
                let subject = format!("Company Notice: {}", event_title.as_str());
                let title = event_title.clone();
                let content = event_desc.clone();
                let send_result = tokio::task::spawn_blocking(move || {
                    let mailer = Mailer::new();
                    mailer.send_broadcast_email(emails, &subject, &title, &content)
                })
                .await;

                match send_result {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => println!("[CALENDAR] Company broadcast failed: {}", e),
                    Err(e) => println!("[CALENDAR] Company broadcast task panicked: {}", e),
                }
            }
            "department" => {
                if let Some(dept_id) = event_dept_id {
                    println!("[CALENDAR] Fetching department emails for dept_id: {}", dept_id);
                    let emails = sqlx::query_scalar::<_, String>(
                        r#"
                        SELECT u.email FROM users u 
                        JOIN employees e ON u.person_id = e.person_id 
                        WHERE e.department_id = $1 AND u.email IS NOT NULL
                        UNION
                        SELECT u.email FROM users u 
                        JOIN interns i ON u.person_id = i.person_id 
                        WHERE i.department_id = $1 AND u.email IS NOT NULL
                        "#
                    )
                    .bind(dept_id)
                    .fetch_all(&pool_clone)
                    .await
                    .unwrap_or_default();
                    println!("[CALENDAR] Sending department notice to {} users", emails.len());
                    let subject = format!("Department Notice: {}", event_title.as_str());
                    let title = event_title.clone();
                    let content = event_desc.clone();
                    let send_result = tokio::task::spawn_blocking(move || {
                        let mailer = Mailer::new();
                        mailer.send_broadcast_email(emails, &subject, &title, &content)
                    })
                    .await;

                    match send_result {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => println!("[CALENDAR] Department broadcast failed: {}", e),
                        Err(e) => println!("[CALENDAR] Department broadcast task panicked: {}", e),
                    }
                }
            }
            _ => {
                println!("[CALENDAR] No notification needed for scope: {}", event_scope);
            }
        }
        println!("[CALENDAR] Notification task finished for event: {}", event_title);
    });

    Ok(event)
}

pub async fn update_event(
    pool: &PgPool,
    user: &User,
    event_id: Uuid,
    dto: UpdateCalendarEventDto,
) -> Result<CalendarEvent, CalendarError> {
    let existing = get_event_by_id(pool, event_id).await?;
    ensure_event_access(pool, user, &existing, PermissionAction::Update).await?;

    let start_at = dto.start_at.unwrap_or(existing.start_at);
    let end_at = dto.end_at.unwrap_or(existing.end_at);
    validate_time_range(start_at, end_at)?;

    let scope_input = dto.scope.or(Some(existing.scope.clone()));
    let (scope, department_id) = resolve_scope(
        pool,
        user,
        normalize_scope(scope_input),
        dto.department_id.or(existing.department_id),
        PermissionAction::Update,
    )
    .await?;

    let event = sqlx::query_as::<_, CalendarEvent>(
        r#"
        UPDATE calendar_events
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            location = COALESCE($3, location),
            start_at = $4,
            end_at = $5,
            all_day = COALESCE($6, all_day),
            scope = $7,
            department_id = $8,
            updated_at = $9
        WHERE id = $10
        RETURNING *
        "#,
    )
    .bind(dto.title.map(|t| t.trim().to_string()))
    .bind(dto.description)
    .bind(dto.location)
    .bind(start_at)
    .bind(end_at)
    .bind(dto.all_day)
    .bind(scope)
    .bind(department_id)
    .bind(Utc::now().naive_utc())
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(event)
}

pub async fn delete_event(pool: &PgPool, user: &User, event_id: Uuid) -> Result<(), CalendarError> {
    let existing = get_event_by_id(pool, event_id).await?;
    ensure_event_access(pool, user, &existing, PermissionAction::Delete).await?;

    let result = sqlx::query("DELETE FROM calendar_events WHERE id = $1")
        .bind(event_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(CalendarError::NotFound);
    }

    Ok(())
}

async fn get_event_by_id(pool: &PgPool, event_id: Uuid) -> Result<CalendarEvent, CalendarError> {
    let event = sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(pool)
        .await?
        .ok_or(CalendarError::NotFound)?;

    Ok(event)
}

async fn ensure_event_access(
    pool: &PgPool,
    user: &User,
    event: &CalendarEvent,
    action: PermissionAction,
) -> Result<(), CalendarError> {
    if user.is_admin {
        return Ok(());
    }

    match event.scope.as_str() {
        "personal" => {
            if event.created_by == Some(user.id) {
                Ok(())
            } else {
                Err(CalendarError::Forbidden)
            }
        }
        "department" | "company" => {
            if has_permission(pool, user, action).await? {
                Ok(())
            } else {
                Err(CalendarError::Forbidden)
            }
        }
        _ => Err(CalendarError::Forbidden),
    }
}

async fn resolve_scope(
    pool: &PgPool,
    user: &User,
    scope: String,
    department_id: Option<Uuid>,
    action: PermissionAction,
) -> Result<(String, Option<Uuid>), CalendarError> {
    let scope = match scope.as_str() {
        "personal" | "department" | "company" => scope,
        _ => return Err(CalendarError::BadRequest("Invalid scope".to_string())),
    };

    if scope == "personal" {
        return Ok((scope, None));
    }

    if !has_permission(pool, user, action).await? {
        return Err(CalendarError::Forbidden);
    }

    if scope == "company" {
        return Ok((scope, None));
    }

    let user_dept = get_user_department_id(pool, user.id).await?;
    let dept_id = department_id.or(user_dept);
    let dept_id = dept_id.ok_or(CalendarError::BadRequest(
        "Department is required for department events".to_string(),
    ))?;

    if !user.is_admin {
        if let Some(user_dept) = user_dept {
            if user_dept != dept_id {
                return Err(CalendarError::Forbidden);
            }
        } else {
            return Err(CalendarError::Forbidden);
        }
    }

    Ok((scope, Some(dept_id)))
}

fn normalize_scope(scope: Option<String>) -> String {
    scope
        .unwrap_or_else(|| "personal".to_string())
        .trim()
        .to_lowercase()
}

fn validate_time_range(
    start_at: NaiveDateTime,
    end_at: NaiveDateTime,
) -> Result<(), CalendarError> {
    if end_at < start_at {
        return Err(CalendarError::BadRequest(
            "End time cannot be before start time".to_string(),
        ));
    }
    Ok(())
}

async fn get_user_department_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<Uuid>, CalendarError> {
    let dept_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT department_id FROM employees
        WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
        UNION
        SELECT department_id FROM interns
        WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(dept_id)
}

async fn has_permission(
    pool: &PgPool,
    user: &User,
    action: PermissionAction,
) -> Result<bool, CalendarError> {
    if user.is_admin {
        return Ok(true);
    }

    let perms = sqlx::query_as::<_, PermissionFlags>(
        r#"
        SELECT 
            COALESCE(MAX(rp.can_create::int), 0)::bool as can_create,
            COALESCE(MAX(rp.can_read::int), 0)::bool as can_read,
            COALESCE(MAX(rp.can_update::int), 0)::bool as can_update,
            COALESCE(MAX(rp.can_delete::int), 0)::bool as can_delete
        FROM role_permissions rp
        INNER JOIN navigation_items n ON n.id = rp.navigation_item_id
        INNER JOIN (
            SELECT department_id, position_id FROM employees 
            WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
            UNION
            SELECT department_id, position_id FROM interns 
            WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
        ) r ON (rp.department_id = r.department_id OR rp.position_id = r.position_id)
        WHERE n.path = $2
        "#,
    )
    .bind(user.id)
    .bind(CALENDAR_NAV_PATH)
    .fetch_optional(pool)
    .await?;

    Ok(perms.map(|p| p.allows(action)).unwrap_or(false))
}
