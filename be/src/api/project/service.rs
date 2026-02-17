use crate::{
    api::project::dto::{
        AddProjectMemberDto, CreateCardDto, CreateProjectDto, ListCardsQuery, UpdateCardDto,
        UpdateProjectDto,
    },
    models::{board::Board, board_column::BoardColumn, project::Project, user::User},
};
use chrono::Utc;
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

#[derive(sqlx::FromRow)]
pub struct CardWithAssignee {
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub assignee_id: Option<Uuid>,
    pub assignee_name: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub display_order: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

    let cards = if let Some(column_id) = query.column_id {
        sqlx::query_as::<_, CardWithAssignee>(
            r#"
            SELECT c.id, c.project_id, c.column_id, c.title, c.description, c.priority,
                   c.assignee_id, u.user_name as assignee_name, c.due_date,
                   c.display_order, c.created_at, c.updated_at
            FROM cards c
            LEFT JOIN users u ON u.id = c.assignee_id
            WHERE c.project_id = $1 AND c.column_id = $2
            ORDER BY c.display_order, c.created_at
            "#,
        )
        .bind(project_id)
        .bind(column_id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, CardWithAssignee>(
            r#"
            SELECT c.id, c.project_id, c.column_id, c.title, c.description, c.priority,
                   c.assignee_id, u.user_name as assignee_name, c.due_date,
                   c.display_order, c.created_at, c.updated_at
            FROM cards c
            LEFT JOIN users u ON u.id = c.assignee_id
            WHERE c.project_id = $1
            ORDER BY c.display_order, c.created_at
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?
    };

    Ok(cards)
}

pub async fn create_card(
    pool: &PgPool,
    project_id: Uuid,
    user: &User,
    dto: CreateCardDto,
) -> Result<CardWithAssignee, ProjectError> {
    ensure_write_role(pool, project_id, user).await?;

    let column_id = resolve_column_id(pool, project_id, dto.column_id).await?;
    let display_order = match dto.display_order {
        Some(order) => order,
        None => next_card_order(pool, column_id).await?,
    };

    let card = sqlx::query_as::<_, CardWithAssignee>(
        r#"
        WITH new_card AS (
            INSERT INTO cards (
                project_id, column_id, title, description, priority,
                assignee_id, due_date, display_order, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
            RETURNING *
        )
        SELECT nc.id, nc.project_id, nc.column_id, nc.title, nc.description, nc.priority,
               nc.assignee_id, u.user_name as assignee_name, nc.due_date,
               nc.display_order, nc.created_at, nc.updated_at
        FROM new_card nc
        LEFT JOIN users u ON u.id = nc.assignee_id
        "#,
    )
    .bind(project_id)
    .bind(column_id)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(dto.priority.unwrap_or_else(|| "medium".to_string()))
    .bind(dto.assignee_id)
    .bind(dto.due_date)
    .bind(display_order)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

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

    let column_id = match dto.column_id {
        Some(column_id) => Some(resolve_column_id(pool, project_id, Some(column_id)).await?),
        None => None,
    };

    let card = sqlx::query_as::<_, CardWithAssignee>(
        r#"
        WITH updated AS (
            UPDATE cards
            SET column_id = COALESCE($1, column_id),
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                priority = COALESCE($4, priority),
                assignee_id = COALESCE($5, assignee_id),
                due_date = COALESCE($6, due_date),
                display_order = COALESCE($7, display_order),
                updated_at = $8
            WHERE id = $9 AND project_id = $10
            RETURNING *
        )
        SELECT u.id, u.project_id, u.column_id, u.title, u.description, u.priority,
               u.assignee_id, usr.user_name as assignee_name, u.due_date,
               u.display_order, u.created_at, u.updated_at
        FROM updated u
        LEFT JOIN users usr ON usr.id = u.assignee_id
        "#,
    )
    .bind(column_id)
    .bind(dto.title)
    .bind(dto.description)
    .bind(dto.priority)
    .bind(dto.assignee_id)
    .bind(dto.due_date)
    .bind(dto.display_order)
    .bind(Utc::now().naive_utc())
    .bind(card_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ProjectError::NotFound)?;

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
