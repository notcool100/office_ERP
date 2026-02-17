use crate::{
    api::project::{dto::*, service},
    db::Db,
    models::user::User,
};
use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

fn map_error(err: service::ProjectError) -> StatusCode {
    match err {
        service::ProjectError::NotFound => StatusCode::NOT_FOUND,
        service::ProjectError::Forbidden => StatusCode::FORBIDDEN,
        service::ProjectError::BadRequest(_) => StatusCode::BAD_REQUEST,
        service::ProjectError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn with_admin_role(user: &User, member_role: Option<String>) -> Option<String> {
    if member_role.is_some() {
        member_role
    } else if user.is_admin {
        Some("admin".to_string())
    } else {
        None
    }
}

pub async fn list_projects_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let projects = service::list_projects(&db, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<ProjectResponseDto> = projects
        .into_iter()
        .map(|project| ProjectResponseDto {
            id: project.id,
            project_key: project.project_key,
            name: project.name,
            description: project.description,
            status: project.status,
            created_by: project.created_by,
            created_at: project.created_at,
            updated_at: project.updated_at,
            member_role: with_admin_role(&user, project.member_role),
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn create_project_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateProjectDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let project = service::create_project(&db, &user, payload)
        .await
        .map_err(map_error)?;

    let response = ProjectResponseDto {
        id: project.id,
        project_key: project.project_key,
        name: project.name,
        description: project.description,
        status: project.status,
        created_by: project.created_by,
        created_at: project.created_at,
        updated_at: project.updated_at,
        member_role: Some("owner".to_string()),
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn get_project_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let project = service::get_project_by_id(&db, id, &user)
        .await
        .map_err(map_error)?;

    let response = ProjectResponseDto {
        id: project.id,
        project_key: project.project_key,
        name: project.name,
        description: project.description,
        status: project.status,
        created_by: project.created_by,
        created_at: project.created_at,
        updated_at: project.updated_at,
        member_role: with_admin_role(&user, project.member_role),
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_project_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProjectDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let project = service::update_project(&db, id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = ProjectResponseDto {
        id: project.id,
        project_key: project.project_key,
        name: project.name,
        description: project.description,
        status: project.status,
        created_by: project.created_by,
        created_at: project.created_at,
        updated_at: project.updated_at,
        member_role: None,
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn list_project_members_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let members = service::list_project_members(&db, id, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<ProjectMemberResponseDto> = members
        .into_iter()
        .map(|member| ProjectMemberResponseDto {
            id: member.id,
            project_id: member.project_id,
            user_id: member.user_id,
            user_name: member.user_name,
            email: member.email,
            role: member.role,
            created_at: member.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn add_project_member_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AddProjectMemberDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let member = service::add_project_member(&db, id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = ProjectMemberResponseDto {
        id: member.id,
        project_id: member.project_id,
        user_id: member.user_id,
        user_name: member.user_name,
        email: member.email,
        role: member.role,
        created_at: member.created_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn get_project_board_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let (board, columns) = service::get_board_with_columns(&db, id, &user)
        .await
        .map_err(map_error)?;

    let response = BoardResponseDto {
        id: board.id,
        project_id: board.project_id,
        name: board.name,
        created_at: board.created_at,
        updated_at: board.updated_at,
        columns: columns
            .into_iter()
            .map(|col| BoardColumnResponseDto {
                id: col.id,
                board_id: col.board_id,
                name: col.name,
                display_order: col.display_order,
                created_at: col.created_at,
                updated_at: col.updated_at,
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn list_cards_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<ListCardsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let cards = service::list_cards(&db, id, &user, query)
        .await
        .map_err(map_error)?;

    let response: Vec<CardResponseDto> = cards
        .into_iter()
        .map(|card| CardResponseDto {
            id: card.id,
            project_id: card.project_id,
            column_id: card.column_id,
            title: card.title,
            description: card.description,
            priority: card.priority,
            assignee_id: card.assignee_id,
            assignee_name: card.assignee_name,
            due_date: card.due_date,
            display_order: card.display_order,
            created_at: card.created_at,
            updated_at: card.updated_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn create_card_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateCardDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let card = service::create_card(&db, id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CardResponseDto {
        id: card.id,
        project_id: card.project_id,
        column_id: card.column_id,
        title: card.title,
        description: card.description,
        priority: card.priority,
        assignee_id: card.assignee_id,
        assignee_name: card.assignee_name,
        due_date: card.due_date,
        display_order: card.display_order,
        created_at: card.created_at,
        updated_at: card.updated_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn update_card_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateCardDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let card = service::update_card(&db, id, card_id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CardResponseDto {
        id: card.id,
        project_id: card.project_id,
        column_id: card.column_id,
        title: card.title,
        description: card.description,
        priority: card.priority,
        assignee_id: card.assignee_id,
        assignee_name: card.assignee_name,
        due_date: card.due_date,
        display_order: card.display_order,
        created_at: card.created_at,
        updated_at: card.updated_at,
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_card_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_card(&db, id, card_id, &user)
        .await
        .map_err(map_error)?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Card deleted successfully" })),
    ))
}
