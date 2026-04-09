use crate::{
    api::project::dto::{
        AddProjectMemberDto, CreateCardCommentDto, CreateCardDto, CreateProjectDto, CreateSprintDto,
        ListCardsQuery, UpdateCardDto, UpdateProjectDto, UpdateSprintDto,
    },
    models::{board::Board, board_column::BoardColumn, project::Project, user::User},
};
use chrono::Utc;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("Project not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Payload too large")]
    PayloadTooLarge,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

#[derive(sqlx::FromRow)]
pub struct ProjectWithRole {
    pub id: Uuid,
    pub project_key: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub member_role: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ProjectMemberInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Clone)]
pub struct CardWithAssignee {
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub sequence_no: i32,
    pub card_key: String,
    pub title: String,
    pub description: Option<String>,
    pub card_type: String,
    pub parent_id: Option<Uuid>,
    pub parent_card_key: Option<String>,
    pub is_migrated: bool,
    pub sprint_name: Option<String>,
    pub priority: String,
    pub assignee_id: Option<Uuid>,
    pub assignee_name: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub display_order: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub sprint_id: Option<Uuid>,
}

#[derive(sqlx::FromRow)]
pub struct SprintInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct CardLinkWithDetails {
    pub id: Uuid,
    pub source_card_id: Uuid,
    pub target_card_id: Uuid,
    pub source_card_key: String,
    pub target_card_key: String,
    pub source_title: String,
    pub target_title: String,
    pub link_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct CardCommentWithUser {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct CardAttachmentInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub uploader_name: Option<String>,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct CardAttachmentFile {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub file_data: Vec<u8>,
}

#[derive(sqlx::FromRow)]
pub struct CardActivityInfo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_name: Option<String>,
    pub action_type: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

fn can_write(role: &str) -> bool {
    matches!(role, "owner" | "admin" | "member")
}

fn can_manage(role: &str) -> bool {
    matches!(role, "owner" | "admin")
}

fn is_valid_role(role: &str) -> bool {
    matches!(role, "owner" | "admin" | "member" | "viewer")
}

pub async fn list_projects(
    pool: &PgPool,
    user: &User,
) -> Result<Vec<ProjectWithRole>, ProjectError> {
    let projects = if user.is_admin {
        sqlx::query_as::<_, ProjectWithRole>(
            r#"
            SELECT 
                p.id, p.project_key, p.name, p.description, p.status,
                p.created_by, p.created_at, p.updated_at,
                pm.role as member_role
            FROM projects p
            LEFT JOIN project_members pm
                ON pm.project_id = p.id AND pm.user_id = $1
            ORDER BY p.created_at DESC
            "#,
        )
        .bind(user.id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, ProjectWithRole>(
            r#"
            SELECT 
                p.id, p.project_key, p.name, p.description, p.status,
                p.created_by, p.created_at, p.updated_at,
                pm.role as member_role
            FROM projects p
            INNER JOIN project_members pm
                ON pm.project_id = p.id
            WHERE pm.user_id = $1
            ORDER BY p.created_at DESC
            "#,
        )
        .bind(user.id)
        .fetch_all(pool)
        .await?
    };

    Ok(projects)
}

pub async fn get_project_by_id(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<ProjectWithRole, ProjectError> {
    let project = if user.is_admin {
        sqlx::query_as::<_, ProjectWithRole>(
            r#"
            SELECT 
                p.id, p.project_key, p.name, p.description, p.status,
                p.created_by, p.created_at, p.updated_at,
                pm.role as member_role
            FROM projects p
            LEFT JOIN project_members pm
                ON pm.project_id = p.id AND pm.user_id = $1
            WHERE p.id = $2
            "#,
        )
        .bind(user.id)
        .bind(project_id)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as::<_, ProjectWithRole>(
            r#"
            SELECT 
                p.id, p.project_key, p.name, p.description, p.status,
                p.created_by, p.created_at, p.updated_at,
                pm.role as member_role
            FROM projects p
            INNER JOIN project_members pm
                ON pm.project_id = p.id
            WHERE p.id = $1 AND pm.user_id = $2
            "#,
        )
        .bind(project_id)
        .bind(user.id)
        .fetch_optional(pool)
        .await?
    };

    project.ok_or(ProjectError::NotFound)
}

pub async fn create_project(
    pool: &PgPool,
    user: &User,
    dto: CreateProjectDto,
) -> Result<Project, ProjectError> {
    let mut tx = pool.begin().await?;
    let now = Utc::now().naive_utc();
    let project_key = dto.project_key.trim().to_uppercase();
    let project_name = dto.name.trim();

    if project_key.is_empty() {
        return Err(ProjectError::BadRequest(
            "Project key is required".to_string(),
        ));
    }

    if project_name.is_empty() {
        return Err(ProjectError::BadRequest(
            "Project name is required".to_string(),
        ));
    }

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (project_key, name, description, status, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, 'active', $4, $5, $5)
        RETURNING *
        "#,
    )
    .bind(&project_key)
    .bind(project_name)
    .bind(&dto.description)
    .bind(user.id)
    .bind(now)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| map_unique_error(err, "Project key already exists"))?;

    sqlx::query(
        r#"
        INSERT INTO project_members (project_id, user_id, role, created_at)
        VALUES ($1, $2, 'owner', $3)
        "#,
    )
    .bind(project.id)
    .bind(user.id)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    let board = sqlx::query_as::<_, Board>(
        r#"
        INSERT INTO boards (project_id, name, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING *
        "#,
    )
    .bind(project.id)
    .bind(format!("{} Board", project.name))
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;

    create_default_columns(&mut tx, board.id, now).await?;

    tx.commit().await?;

    Ok(project)
}

pub async fn update_project(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    dto: UpdateProjectDto,
) -> Result<Project, ProjectError> {
    ensure_manage_role(pool, project_id, user).await?;

    let project = sqlx::query_as::<_, Project>(
        r#"
        UPDATE projects
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            updated_at = $4
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(dto.name)
    .bind(dto.description)
    .bind(dto.status)
    .bind(Utc::now().naive_utc())
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    Ok(project)
}

pub async fn list_project_members(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<Vec<ProjectMemberInfo>, ProjectError> {
    ensure_member(pool, project_id, user).await?;

    let members = sqlx::query_as::<_, ProjectMemberInfo>(
        r#"
        SELECT pm.id, pm.project_id, pm.user_id, u.user_name, u.email, pm.role, pm.created_at
        FROM project_members pm
        INNER JOIN users u ON u.id = pm.user_id
        WHERE pm.project_id = $1
        ORDER BY pm.created_at ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(members)
}

pub async fn add_project_member(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    dto: AddProjectMemberDto,
) -> Result<ProjectMemberInfo, ProjectError> {
    ensure_manage_role(pool, project_id, user).await?;

    let role = dto.role.trim().to_lowercase();
    if !is_valid_role(&role) {
        return Err(ProjectError::BadRequest("Invalid project role".to_string()));
    }

    let member = sqlx::query_as::<_, ProjectMemberInfo>(
        r#"
        WITH inserted AS (
            INSERT INTO project_members (project_id, user_id, role, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, project_id, user_id, role, created_at
        )
        SELECT inserted.id, inserted.project_id, inserted.user_id,
               u.user_name, u.email, inserted.role, inserted.created_at
        FROM inserted
        INNER JOIN users u ON u.id = inserted.user_id
        "#,
    )
    .bind(project_id)
    .bind(dto.user_id)
    .bind(role)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await
    .map_err(|err| map_unique_error(err, "User is already a project member"))?;

    Ok(member)
}

pub async fn get_board_with_columns(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<(Board, Vec<BoardColumn>), ProjectError> {
    ensure_member(pool, project_id, user).await?;

    let board = sqlx::query_as::<_, Board>(
        "SELECT * FROM boards WHERE project_id = $1 ORDER BY created_at LIMIT 1",
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    let columns = sqlx::query_as::<_, BoardColumn>(
        r#"
        SELECT * FROM board_columns
        WHERE board_id = $1
        ORDER BY display_order, created_at
        "#,
    )
    .bind(board.id)
    .fetch_all(pool)
    .await?;

    Ok((board, columns))
}

pub async fn list_cards(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    query: ListCardsQuery,
) -> Result<Vec<CardWithAssignee>, ProjectError> {
    ensure_member(pool, project_id, user).await?;

    let mut sql = String::from(
        r#"
        SELECT c.id, c.project_id, c.column_id, c.sequence_no, c.card_key,
               c.title, c.description, c.card_type, c.parent_id, pc.card_key as parent_card_key, c.is_migrated,
               c.sprint_name, c.priority, c.assignee_id,
               u.user_name as assignee_name, c.due_date, c.display_order,
               c.created_at, c.updated_at, c.sprint_id
        FROM cards c
        LEFT JOIN users u ON u.id = c.assignee_id
        LEFT JOIN cards pc ON pc.id = c.parent_id
        WHERE c.project_id = $1
        "#,
    );

    let mut bind_index = 2;
    if query.column_id.is_some() {
        sql.push_str(&format!(" AND c.column_id = ${}", bind_index));
        bind_index += 1;
    }
    if query.sprint_id.is_some() {
        sql.push_str(&format!(" AND c.sprint_id = ${}", bind_index));
    }

    sql.push_str(" ORDER BY c.display_order, c.sequence_no, c.created_at");

    let mut q = sqlx::query_as::<_, CardWithAssignee>(&sql).bind(project_id);

    if let Some(column_id) = query.column_id {
        q = q.bind(column_id);
    }
    if let Some(sprint_id) = query.sprint_id {
        q = q.bind(sprint_id);
    }

    let cards = q.fetch_all(pool).await?;

    Ok(cards)
}

pub async fn list_sprints(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<Vec<SprintInfo>, ProjectError> {
    ensure_member(pool, project_id, user).await?;

    let sprints = sqlx::query_as::<_, SprintInfo>(
        r#"
        SELECT * FROM sprints
        WHERE project_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(sprints)
}

pub async fn create_sprint(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    dto: CreateSprintDto,
) -> Result<SprintInfo, ProjectError> {
    ensure_manage_role(pool, project_id, user).await?;

    let name = dto.name.trim();
    if name.is_empty() {
        return Err(ProjectError::BadRequest("Sprint name is required".to_string()));
    }

    let now = Utc::now().naive_utc();
    let sprint = sqlx::query_as::<_, SprintInfo>(
        r#"
        INSERT INTO sprints (project_id, name, goal, start_date, end_date, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'planning', $6, $6)
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(name)
    .bind(dto.goal)
    .bind(dto.start_date)
    .bind(dto.end_date)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(sprint)
}

pub async fn update_sprint(
    pool: &PgPool,
    project_id: Uuid,
    sprint_id: Uuid,
    user: &User,
    dto: UpdateSprintDto,
) -> Result<SprintInfo, ProjectError> {
    ensure_manage_role(pool, project_id, user).await?;

    let now = Utc::now().naive_utc();
    let sprint = sqlx::query_as::<_, SprintInfo>(
        r#"
        UPDATE sprints
        SET name = COALESCE($1, name),
            goal = COALESCE($2, goal),
            start_date = COALESCE($3, start_date),
            end_date = COALESCE($4, end_date),
            status = COALESCE($5, status),
            updated_at = $6
        WHERE id = $7 AND project_id = $8
        RETURNING *
        "#,
    )
    .bind(dto.name)
    .bind(dto.goal)
    .bind(dto.start_date)
    .bind(dto.end_date)
    .bind(dto.status)
    .bind(now)
    .bind(sprint_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    Ok(sprint)
}

pub async fn delete_sprint(
    pool: &PgPool,
    project_id: Uuid,
    sprint_id: Uuid,
    user: &User,
) -> Result<(), ProjectError> {
    ensure_manage_role(pool, project_id, user).await?;

    let result = sqlx::query("DELETE FROM sprints WHERE id = $1 AND project_id = $2")
        .bind(sprint_id)
        .bind(project_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound);
    }

    Ok(())
}

pub async fn create_card(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    dto: CreateCardDto,
) -> Result<CardWithAssignee, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    let title = dto.title.trim();
    if title.is_empty() {
        return Err(ProjectError::BadRequest(
            "Card title is required".to_string(),
        ));
    }

    let column_id = resolve_column_id(pool, project_id, dto.column_id).await?;
    let display_order = match dto.display_order {
        Some(order) => order,
        None => next_card_order(pool, column_id).await?,
    };
    let priority = normalize_priority(dto.priority.as_deref())?;
    let sprint_name = normalize_optional_text(dto.sprint_name.as_deref(), 100);
    let description = normalize_optional_text(dto.description.as_deref(), 20_000);
    let card_type = dto.card_type.unwrap_or_else(|| "task".to_string()).trim().to_lowercase();
    let parent_id = dto.parent_id;
    let now = Utc::now().naive_utc();

    validate_card_hierarchy(pool, project_id, &card_type, parent_id).await?;

    let mut tx = pool.begin().await?;
    let project_name = sqlx::query_scalar::<_, String>("SELECT name FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ProjectError::NotFound)?;
    let sequence_no = next_card_sequence(&mut tx, project_id).await?;
    let card_key = build_card_key(&project_name, sequence_no);

    let card = sqlx::query_as::<_, CardWithAssignee>(
        r#"
        WITH new_card AS (
            INSERT INTO cards (
                project_id, column_id, sequence_no, card_key, title, description,
                card_type, parent_id, is_migrated,
                sprint_name, priority, assignee_id, due_date, display_order,
                created_at, updated_at, sprint_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false, $9, $10, $11, $12, $13, $14, $14, $15)
            RETURNING *
        )
        SELECT nc.id, nc.project_id, nc.column_id, nc.sequence_no, nc.card_key,
               nc.title, nc.description, nc.card_type, nc.parent_id, pc.card_key as parent_card_key, nc.is_migrated,
               nc.sprint_name, nc.priority,
               nc.assignee_id, u.user_name as assignee_name, nc.due_date,
               nc.display_order, nc.created_at, nc.updated_at, nc.sprint_id
        FROM new_card nc
        LEFT JOIN users u ON u.id = nc.assignee_id
        LEFT JOIN cards pc ON pc.id = nc.parent_id
        "#,
    )
    .bind(project_id)
    .bind(column_id)
    .bind(sequence_no)
    .bind(&card_key)
    .bind(title)
    .bind(description)
    .bind(card_type)
    .bind(parent_id)
    .bind(sprint_name)
    .bind(priority)
    .bind(dto.assignee_id)
    .bind(dto.due_date)
    .bind(display_order)
    .bind(now)
    .bind(dto.sprint_id)
    .fetch_one(&mut *tx)
    .await?;

    log_card_activity_tx(
        &mut tx,
        project_id,
        card.id,
        Some(user.id),
        "created",
        format!("Created card {}", card.card_key),
        None,
    )
    .await?;

    tx.commit().await?;

    // Send assignment email if assignee is set
    if let Some(assignee_id) = card.assignee_id {
        let pool_clone = pool.clone();
        let card_clone = card.clone();
        tokio::spawn(async move {
            if let Ok(user) = crate::api::user::service::get_by_id(&pool_clone, assignee_id).await {
                // Get project name
                if let Ok(project) =
                    sqlx::query_scalar::<_, String>("SELECT name FROM projects WHERE id = $1")
                        .bind(card_clone.project_id)
                        .fetch_one(&pool_clone)
                        .await
                {
                    let to = user.email.clone();
                    let assignee_name = user.user_name.clone();
                    let task_title = card_clone.title.clone();
                    let project_name = project.clone();
                    let priority = card_clone.priority.clone();
                    let send_result = tokio::task::spawn_blocking(move || {
                        let mailer = crate::api::user::mailer::Mailer::new();
                        mailer.send_task_assignment_email(
                            &to,
                            &assignee_name,
                            &task_title,
                            &project_name,
                            &priority,
                        )
                    })
                    .await;

                    match send_result {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => tracing::error!("Task assignment email failed: {}", e),
                        Err(e) => tracing::error!("Task assignment email task panicked: {}", e),
                    }
                }
            }
        });
    }

    Ok(card)
}

pub async fn update_card(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
    dto: UpdateCardDto,
) -> Result<CardWithAssignee, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    let existing = get_card_with_assignee(pool, project_id, card_id).await?;
    let resolved_column_id = match dto.column_id {
        Some(column_id) => Some(resolve_column_id(pool, project_id, Some(column_id)).await?),
        None => existing.column_id,
    };
    let sprint_name = normalize_optional_text(dto.sprint_name.as_deref(), 100);
    let description = normalize_optional_text(dto.description.as_deref(), 20_000);
    let priority = match dto.priority.as_deref() {
        Some(value) => Some(normalize_priority(Some(value))?),
        None => None,
    };
    
    let card_type = match dto.card_type.as_deref() {
        Some(val) => val.trim().to_lowercase(),
        None => existing.card_type.clone()
    };
    
    let parent_id = match dto.parent_id {
        Some(id) => Some(id), 
        None => existing.parent_id
    };

    if existing.card_type != card_type || existing.parent_id != parent_id {
        validate_card_hierarchy(pool, project_id, &card_type, parent_id).await?;
    }
    
    if existing.column_id != resolved_column_id {
        check_completion_rules(pool, project_id, card_id, &card_type, resolved_column_id).await?;
    }

    let card = sqlx::query_as::<_, CardWithAssignee>(
        r#"
        WITH updated AS (
            UPDATE cards
            SET column_id = $1,
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                sprint_name = COALESCE($4, sprint_name),
                card_type = COALESCE($5, card_type),
                parent_id = COALESCE($6, parent_id),
                priority = COALESCE($7, priority),
                assignee_id = COALESCE($8, assignee_id),
                due_date = COALESCE($9, due_date),
                display_order = COALESCE($10, display_order),
                updated_at = $11,
                sprint_id = COALESCE($14, sprint_id)
            WHERE id = $12 AND project_id = $13
            RETURNING *
        )
        SELECT u.id, u.project_id, u.column_id, u.sequence_no, u.card_key,
               u.title, u.description, u.card_type, u.parent_id, pc.card_key as parent_card_key, u.is_migrated,
               u.sprint_name, u.priority, u.assignee_id,
               usr.user_name as assignee_name, u.due_date,
               u.display_order, u.created_at, u.updated_at, u.sprint_id
        FROM updated u
        LEFT JOIN users usr ON usr.id = u.assignee_id
        LEFT JOIN cards pc ON pc.id = u.parent_id
        "#,
    )
    .bind(resolved_column_id)
    .bind(dto.title)
    .bind(description)
    .bind(sprint_name)
    .bind(dto.card_type)
    .bind(dto.parent_id)
    .bind(priority)
    .bind(dto.assignee_id)
    .bind(dto.due_date)
    .bind(dto.display_order)
    .bind(Utc::now().naive_utc())
    .bind(card_id)
    .bind(project_id)
    .bind(dto.sprint_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    if existing.column_id != card.column_id {
        let old_column = get_column_name(pool, existing.column_id)
            .await?
            .unwrap_or_else(|| "Unknown".to_string());
        let new_column = get_column_name(pool, card.column_id)
            .await?
            .unwrap_or_else(|| "Unknown".to_string());
        log_card_activity(
            pool,
            project_id,
            card_id,
            Some(user.id),
            "moved",
            format!("Moved card from {old_column} to {new_column}"),
            None,
        )
        .await?;
    }

    let mut changed_fields: Vec<&str> = Vec::new();
    if existing.title != card.title {
        changed_fields.push("title");
    }
    if existing.description != card.description {
        changed_fields.push("description");
    }
    if existing.sprint_name != card.sprint_name {
        changed_fields.push("sprint");
    }
    if existing.priority != card.priority {
        changed_fields.push("priority");
    }
    if existing.assignee_id != card.assignee_id {
        changed_fields.push("assignee");
    }
    if existing.due_date != card.due_date {
        changed_fields.push("due date");
    }
    if existing.display_order != card.display_order {
        changed_fields.push("order");
    }

    if !changed_fields.is_empty() {
        log_card_activity(
            pool,
            project_id,
            card_id,
            Some(user.id),
            "updated",
            format!("Updated {}", changed_fields.join(", ")),
            None,
        )
        .await?;
    }

    // Send assignment email if assignee is set/changed
    if let Some(assignee_id) = card.assignee_id {
        let pool_clone = pool.clone();
        let card_clone = card.clone();
        tokio::spawn(async move {
            if let Ok(user) = crate::api::user::service::get_by_id(&pool_clone, assignee_id).await {
                // Get project name
                if let Ok(project) =
                    sqlx::query_scalar::<_, String>("SELECT name FROM projects WHERE id = $1")
                        .bind(card_clone.project_id)
                        .fetch_one(&pool_clone)
                        .await
                {
                    let to = user.email.clone();
                    let assignee_name = user.user_name.clone();
                    let task_title = card_clone.title.clone();
                    let project_name = project.clone();
                    let priority = card_clone.priority.clone();
                    let send_result = tokio::task::spawn_blocking(move || {
                        let mailer = crate::api::user::mailer::Mailer::new();
                        mailer.send_task_assignment_email(
                            &to,
                            &assignee_name,
                            &task_title,
                            &project_name,
                            &priority,
                        )
                    })
                    .await;

                    match send_result {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => tracing::error!("Task assignment email failed: {}", e),
                        Err(e) => tracing::error!("Task assignment email task panicked: {}", e),
                    }
                }
            }
        });
    }

    Ok(card)
}

pub async fn delete_card(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
) -> Result<(), ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    let result = sqlx::query("DELETE FROM cards WHERE id = $1 AND project_id = $2")
        .bind(card_id)
        .bind(project_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound);
    }

    Ok(())
}

pub async fn list_card_comments(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
) -> Result<Vec<CardCommentWithUser>, ProjectError> {
    ensure_member(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    let comments = sqlx::query_as::<_, CardCommentWithUser>(
        r#"
        SELECT cc.id, cc.project_id, cc.card_id, cc.user_id, u.user_name,
               cc.comment, cc.created_at, cc.updated_at
        FROM card_comments cc
        INNER JOIN users u ON u.id = cc.user_id
        WHERE cc.project_id = $1 AND cc.card_id = $2
        ORDER BY cc.created_at ASC
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .fetch_all(pool)
    .await?;

    Ok(comments)
}

pub async fn create_card_comment(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
    dto: CreateCardCommentDto,
) -> Result<CardCommentWithUser, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    let comment = dto.comment.trim();
    if comment.is_empty() {
        return Err(ProjectError::BadRequest(
            "Comment cannot be empty".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let created = sqlx::query_as::<_, CardCommentWithUser>(
        r#"
        WITH inserted AS (
            INSERT INTO card_comments (
                project_id, card_id, user_id, comment, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING id, project_id, card_id, user_id, comment, created_at, updated_at
        )
        SELECT i.id, i.project_id, i.card_id, i.user_id, u.user_name,
               i.comment, i.created_at, i.updated_at
        FROM inserted i
        INNER JOIN users u ON u.id = i.user_id
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .bind(user.id)
    .bind(comment)
    .bind(now)
    .fetch_one(pool)
    .await?;

    let short_comment = if comment.chars().count() > 80 {
        format!("{}...", comment.chars().take(80).collect::<String>())
    } else {
        comment.to_string()
    };

    log_card_activity(
        pool,
        project_id,
        card_id,
        Some(user.id),
        "commented",
        format!("Added a comment: {short_comment}"),
        None,
    )
    .await?;

    Ok(created)
}

pub async fn list_card_attachments(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
) -> Result<Vec<CardAttachmentInfo>, ProjectError> {
    ensure_member(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    let attachments = sqlx::query_as::<_, CardAttachmentInfo>(
        r#"
        SELECT ca.id, ca.project_id, ca.card_id, ca.uploaded_by, u.user_name as uploader_name,
               ca.file_name, ca.content_type, ca.file_size, ca.created_at
        FROM card_attachments ca
        LEFT JOIN users u ON u.id = ca.uploaded_by
        WHERE ca.project_id = $1 AND ca.card_id = $2
        ORDER BY ca.created_at DESC
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .fetch_all(pool)
    .await?;

    Ok(attachments)
}

pub async fn upload_card_attachment(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
    file_name: String,
    content_type: String,
    file_data: Vec<u8>,
) -> Result<CardAttachmentInfo, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    if file_data.is_empty() {
        return Err(ProjectError::BadRequest("Attachment is empty".to_string()));
    }

    if file_data.len() > MAX_ATTACHMENT_SIZE_BYTES {
        return Err(ProjectError::PayloadTooLarge);
    }

    let now = Utc::now().naive_utc();
    let file_size = i64::try_from(file_data.len())
        .map_err(|_| ProjectError::BadRequest("Attachment is too large".to_string()))?;

    let attachment = sqlx::query_as::<_, CardAttachmentInfo>(
        r#"
        WITH inserted AS (
            INSERT INTO card_attachments (
                project_id, card_id, uploaded_by, file_name, content_type, file_size, file_data, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, project_id, card_id, uploaded_by, file_name, content_type, file_size, created_at
        )
        SELECT i.id, i.project_id, i.card_id, i.uploaded_by, u.user_name as uploader_name,
               i.file_name, i.content_type, i.file_size, i.created_at
        FROM inserted i
        LEFT JOIN users u ON u.id = i.uploaded_by
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .bind(user.id)
    .bind(file_name.as_str())
    .bind(content_type.as_str())
    .bind(file_size)
    .bind(file_data)
    .bind(now)
    .fetch_one(pool)
    .await?;

    log_card_activity(
        pool,
        project_id,
        card_id,
        Some(user.id),
        "attachment_uploaded",
        format!("Uploaded attachment {}", attachment.file_name),
        None,
    )
    .await?;

    Ok(attachment)
}

pub async fn get_card_attachment_file(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    attachment_id: Uuid,
    user: &User,
) -> Result<CardAttachmentFile, ProjectError> {
    ensure_member(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    let attachment = sqlx::query_as::<_, CardAttachmentFile>(
        r#"
        SELECT id, file_name, content_type, file_data
        FROM card_attachments
        WHERE id = $1 AND project_id = $2 AND card_id = $3
        "#,
    )
    .bind(attachment_id)
    .bind(project_id)
    .bind(card_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    Ok(attachment)
}

pub async fn list_card_history(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
) -> Result<Vec<CardActivityInfo>, ProjectError> {
    ensure_member(pool, project_id, user).await?;
    ensure_card_exists(pool, project_id, card_id).await?;

    let activities = sqlx::query_as::<_, CardActivityInfo>(
        r#"
        SELECT ca.id, ca.project_id, ca.card_id, ca.actor_id, u.user_name as actor_name,
               ca.action_type, ca.description, ca.created_at
        FROM card_activities ca
        LEFT JOIN users u ON u.id = ca.actor_id
        WHERE ca.project_id = $1 AND ca.card_id = $2
        ORDER BY ca.created_at DESC
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .fetch_all(pool)
    .await?;

    Ok(activities)
}

const MAX_ATTACHMENT_SIZE_BYTES: usize = 25 * 1024 * 1024;

fn normalize_priority(priority: Option<&str>) -> Result<String, ProjectError> {
    let value = priority.unwrap_or("medium").trim().to_ascii_lowercase();
    if matches!(value.as_str(), "low" | "medium" | "high") {
        Ok(value)
    } else {
        Err(ProjectError::BadRequest(
            "Priority must be low, medium, or high".to_string(),
        ))
    }
}

fn normalize_optional_text(value: Option<&str>, max_len: usize) -> Option<String> {
    let trimmed = value?.trim();
    if trimmed.is_empty() {
        return None;
    }

    let normalized = trimmed.chars().take(max_len).collect::<String>();

    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn build_card_key(project_name: &str, sequence_no: i32) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for ch in project_name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-');
    let mut short_slug = trimmed.chars().take(12).collect::<String>();
    short_slug = short_slug.trim_matches('-').to_string();
    if short_slug.is_empty() {
        short_slug = "project".to_string();
    }

    format!("pro-{short_slug}-{sequence_no:02}")
}

async fn next_card_sequence(
    tx: &mut Transaction<'_, Postgres>,
    project_id: Uuid,
) -> Result<i32, ProjectError> {
    let locked_project =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM projects WHERE id = $1 FOR UPDATE")
            .bind(project_id)
            .fetch_optional(&mut **tx)
            .await?;

    if locked_project.is_none() {
        return Err(ProjectError::NotFound);
    }

    let next_sequence = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(sequence_no), 0) + 1 FROM cards WHERE project_id = $1",
    )
    .bind(project_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(next_sequence)
}

async fn get_card_with_assignee(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
) -> Result<CardWithAssignee, ProjectError> {
    let card = sqlx::query_as::<_, CardWithAssignee>(
        r#"
        SELECT c.id, c.project_id, c.column_id, c.sequence_no, c.card_key,
               c.title, c.description, c.card_type, c.parent_id, pc.card_key as parent_card_key, c.is_migrated,
               c.sprint_name, c.priority, c.assignee_id,
               u.user_name as assignee_name, c.due_date, c.display_order,
               c.created_at, c.updated_at, c.sprint_id
        FROM cards c
        LEFT JOIN users u ON u.id = c.assignee_id
        LEFT JOIN cards pc ON pc.id = c.parent_id
        WHERE c.project_id = $1 AND c.id = $2
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

    Ok(card)
}

async fn ensure_card_exists(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
) -> Result<(), ProjectError> {
    let exists =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM cards WHERE project_id = $1 AND id = $2")
            .bind(project_id)
            .bind(card_id)
            .fetch_optional(pool)
            .await?;

    if exists.is_some() {
        Ok(())
    } else {
        Err(ProjectError::NotFound)
    }
}

async fn get_column_name(
    pool: &PgPool,
    column_id: Option<Uuid>,
) -> Result<Option<String>, ProjectError> {
    let Some(column_id) = column_id else {
        return Ok(None);
    };

    let name = sqlx::query_scalar::<_, String>("SELECT name FROM board_columns WHERE id = $1")
        .bind(column_id)
        .fetch_optional(pool)
        .await?;

    Ok(name)
}

async fn log_card_activity(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    actor_id: Option<Uuid>,
    action_type: &str,
    description: String,
    metadata: Option<Value>,
) -> Result<(), ProjectError> {
    sqlx::query(
        r#"
        INSERT INTO card_activities (
            project_id, card_id, actor_id, action_type, description, metadata, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .bind(actor_id)
    .bind(action_type)
    .bind(description)
    .bind(metadata)
    .bind(Utc::now().naive_utc())
    .execute(pool)
    .await?;

    Ok(())
}

async fn log_card_activity_tx(
    tx: &mut Transaction<'_, Postgres>,
    project_id: Uuid,
    card_id: Uuid,
    actor_id: Option<Uuid>,
    action_type: &str,
    description: String,
    metadata: Option<Value>,
) -> Result<(), ProjectError> {
    sqlx::query(
        r#"
        INSERT INTO card_activities (
            project_id, card_id, actor_id, action_type, description, metadata, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .bind(actor_id)
    .bind(action_type)
    .bind(description)
    .bind(metadata)
    .bind(Utc::now().naive_utc())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn ensure_member(pool: &PgPool, project_id: Uuid, user: &User) -> Result<(), ProjectError> {
    if user.is_admin {
        return Ok(());
    }

    let role = get_member_role(pool, project_id, user.id).await?;
    match role {
        Some(_) => Ok(()),
        None => Err(ProjectError::Forbidden),
    }
}

async fn ensure_write_role(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<(), ProjectError> {
    if user.is_admin {
        return Ok(());
    }

    let role = get_member_role(pool, project_id, user.id).await?;
    match role {
        Some(role) if can_write(&role) => Ok(()),
        Some(_) => Err(ProjectError::Forbidden),
        None => Err(ProjectError::Forbidden),
    }
}

async fn ensure_manage_role(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
) -> Result<(), ProjectError> {
    if user.is_admin {
        return Ok(());
    }

    let role = get_member_role(pool, project_id, user.id).await?;
    match role {
        Some(role) if can_manage(&role) => Ok(()),
        Some(_) => Err(ProjectError::Forbidden),
        None => Err(ProjectError::Forbidden),
    }
}

async fn get_member_role(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Option<String>, ProjectError> {
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM project_members WHERE project_id = $1 AND user_id = $2",
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(role)
}

async fn resolve_column_id(
    pool: &PgPool,
    project_id: Uuid,
    column_id: Option<Uuid>,
) -> Result<Uuid, ProjectError> {
    if let Some(column_id) = column_id {
        let exists = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT bc.id
            FROM board_columns bc
            INNER JOIN boards b ON b.id = bc.board_id
            WHERE b.project_id = $1 AND bc.id = $2
            "#,
        )
        .bind(project_id)
        .bind(column_id)
        .fetch_optional(pool)
        .await?;

        return exists.ok_or(ProjectError::BadRequest(
            "Invalid column for this project".to_string(),
        ));
    }

    let default_column = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT bc.id
        FROM board_columns bc
        INNER JOIN boards b ON b.id = bc.board_id
        WHERE b.project_id = $1
        ORDER BY bc.display_order, bc.created_at
        LIMIT 1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    default_column.ok_or(ProjectError::BadRequest(
        "Project has no columns yet".to_string(),
    ))
}

async fn next_card_order(pool: &PgPool, column_id: Uuid) -> Result<i32, ProjectError> {
    let next = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(display_order), 0) + 1 FROM cards WHERE column_id = $1",
    )
    .bind(column_id)
    .fetch_one(pool)
    .await?;

    Ok(next)
}

async fn create_default_columns(
    tx: &mut Transaction<'_, Postgres>,
    board_id: Uuid,
    now: chrono::NaiveDateTime,
) -> Result<(), ProjectError> {
    let defaults = ["Backlog", "In Progress", "Review", "Done"];
    for (idx, name) in defaults.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO board_columns (board_id, name, display_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            "#,
        )
        .bind(board_id)
        .bind(name)
        .bind(idx as i32)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

fn map_unique_error(err: sqlx::Error, message: &str) -> ProjectError {
    if let sqlx::Error::Database(db_err) = &err
        && db_err.code().as_deref() == Some("23505")
    {
        return ProjectError::BadRequest(message.to_string());
    }
    ProjectError::Database(err)
}

async fn validate_card_hierarchy(
    pool: &PgPool,
    project_id: Uuid,
    card_type: &str,
    parent_id: Option<Uuid>,
) -> Result<(), ProjectError> {
    if card_type == "epic" && parent_id.is_some() {
        return Err(ProjectError::BadRequest("Epic cannot have a parent".to_string()));
    }
    
    if card_type != "epic" && parent_id.is_none() {
        return Err(ProjectError::BadRequest(format!("{} must have a parent", card_type)));
    }
    
    if let Some(pid) = parent_id {
        let parent_type = sqlx::query_scalar::<_, String>("SELECT card_type FROM cards WHERE id = $1 AND project_id = $2")
            .bind(pid)
            .bind(project_id)
            .fetch_optional(pool)
            .await?
            .ok_or(ProjectError::BadRequest("Parent card not found".to_string()))?;
            
        match card_type {
            "story" => if parent_type != "epic" { return Err(ProjectError::BadRequest("Story parent must be an Epic".to_string())); },
            "task" => if parent_type != "story" { return Err(ProjectError::BadRequest("Task parent must be a Story".to_string())); },
            "bug" => if parent_type != "story" && parent_type != "task" { return Err(ProjectError::BadRequest("Bug parent must be a Story or Task".to_string())); },
            _ => (),
        }
    }
    
    Ok(())
}

async fn check_completion_rules(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    card_type: &str,
    column_id: Option<Uuid>,
) -> Result<(), ProjectError> {
    let Some(col_id) = column_id else { return Ok(()) };
    let is_done = sqlx::query_scalar::<_, bool>("SELECT is_done FROM board_columns WHERE id = $1")
        .bind(col_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or(false);
        
    if !is_done { return Ok(()) };
    
    if card_type == "epic" {
        let incomplete_stories = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT count(c.id) FROM cards c 
            LEFT JOIN board_columns bc ON c.column_id = bc.id 
            WHERE c.parent_id = $1 AND (bc.is_done = false OR c.column_id IS NULL) AND c.project_id = $2
            "#
        )
        .bind(card_id)
        .bind(project_id)
        .fetch_one(pool)
        .await?;
        if incomplete_stories > 0 {
            return Err(ProjectError::BadRequest("Cannot complete Epic because it has incomplete Stories".to_string()));
        }
    } else if card_type == "story" {
        let incomplete_children = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT count(c.id) FROM cards c
            LEFT JOIN board_columns bc ON c.column_id = bc.id
            WHERE (c.parent_id = $1 OR c.parent_id IN (SELECT id FROM cards WHERE parent_id = $1 AND card_type = 'task' AND project_id = $2))
              AND (bc.is_done = false OR c.column_id IS NULL) AND c.project_id = $2
            "#
        )
        .bind(card_id)
        .bind(project_id)
        .fetch_one(pool)
        .await?;
        if incomplete_children > 0 {
            return Err(ProjectError::BadRequest("Cannot complete Story because it has incomplete Tasks or Bugs".to_string()));
        }
    }
    
    Ok(())
}

pub async fn list_card_links(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
) -> Result<Vec<CardLinkWithDetails>, ProjectError> {
    ensure_member(pool, project_id, user).await?;

    let links = sqlx::query_as::<_, CardLinkWithDetails>(
        r#"
        SELECT cl.id, cl.source_card_id, cl.target_card_id, 
               sc.card_key as source_card_key, tc.card_key as target_card_key,
               sc.title as source_title, tc.title as target_title,
               cl.link_type, cl.created_at
        FROM card_links cl
        JOIN cards sc ON sc.id = cl.source_card_id
        JOIN cards tc ON tc.id = cl.target_card_id
        WHERE (cl.source_card_id = $1 OR cl.target_card_id = $1)
          AND sc.project_id = $2 AND tc.project_id = $2
        ORDER BY cl.created_at DESC
        "#,
    )
    .bind(card_id)
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(links)
}

pub async fn create_card_link(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    user: &User,
    dto: crate::api::project::dto::CreateCardLinkDto,
) -> Result<CardLinkWithDetails, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    if card_id == dto.target_card_id {
        return Err(ProjectError::BadRequest("Cannot link card to itself".to_string()));
    }

    let link_type = dto.link_type.trim().to_lowercase();
    if !["depends_on", "relates_to"].contains(&link_type.as_str()) {
        return Err(ProjectError::BadRequest("Invalid link type".to_string()));
    }

    ensure_card_exists(pool, project_id, dto.target_card_id).await?;

    let link_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO card_links (source_card_id, target_card_id, link_type)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(card_id)
    .bind(dto.target_card_id)
    .bind(&link_type)
    .fetch_one(pool)
    .await
    .map_err(|err| map_unique_error(err, "Link already exists"))?;

    let link = sqlx::query_as::<_, CardLinkWithDetails>(
        r#"
        SELECT cl.id, cl.source_card_id, cl.target_card_id, 
               sc.card_key as source_card_key, tc.card_key as target_card_key,
               sc.title as source_title, tc.title as target_title,
               cl.link_type, cl.created_at
        FROM card_links cl
        JOIN cards sc ON sc.id = cl.source_card_id
        JOIN cards tc ON tc.id = cl.target_card_id
        WHERE cl.id = $1
        "#,
    )
    .bind(link_id)
    .fetch_one(pool)
    .await?;

    log_card_activity(
        pool, project_id, card_id, Some(user.id), "linked",
        format!("Linked to {} ({})", link.target_card_key, link_type), None
    ).await?;

    Ok(link)
}

pub async fn delete_card_link(
    pool: &PgPool,
    project_id: Uuid,
    card_id: Uuid,
    link_id: Uuid,
    user: &User,
) -> Result<(), ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    let result = sqlx::query("DELETE FROM card_links WHERE id = $1 AND (source_card_id = $2 OR target_card_id = $2)")
        .bind(link_id)
        .bind(card_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound);
    }
    
    log_card_activity(
        pool, project_id, card_id, Some(user.id), "link_removed",
        "Removed card link".to_string(), None
    ).await?;

    Ok(())
}
