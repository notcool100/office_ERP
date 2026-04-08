use crate::{
    api::project::{dto::*, service},
    db::Db,
    models::user::User,
};
use axum::{
    Json,
    extract::{Extension, Multipart, Path, Query},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

fn map_error(err: service::ProjectError) -> StatusCode {
    match err {
        service::ProjectError::NotFound => StatusCode::NOT_FOUND,
        service::ProjectError::Forbidden => StatusCode::FORBIDDEN,
        service::ProjectError::BadRequest(ref msg) => {
            tracing::warn!("Bad request: {}", msg);
            StatusCode::BAD_REQUEST
        }
        service::ProjectError::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
        service::ProjectError::Database(ref e) => {
            tracing::error!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
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
    tracing::info!(user_id = %user.id, "list_projects");
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
    tracing::info!(
        user_id = %user.id,
        project_key = %payload.project_key,
        name = %payload.name,
        "create_project"
    );
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
    tracing::info!(user_id = %user.id, project_id = %id, "get_project");
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
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        name = ?payload.name,
        status = ?payload.status,
        "update_project"
    );
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
    tracing::info!(user_id = %user.id, project_id = %id, "list_project_members");
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
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        member_user_id = %payload.user_id,
        role = %payload.role,
        "add_project_member"
    );
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
    tracing::info!(user_id = %user.id, project_id = %id, "get_project_board");
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
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        column_id = ?query.column_id,
        "list_cards"
    );
    let cards = service::list_cards(&db, id, &user, query)
        .await
        .map_err(map_error)?;

    let response: Vec<CardResponseDto> = cards
        .into_iter()
        .map(|card| CardResponseDto {
            id: card.id,
            project_id: card.project_id,
            column_id: card.column_id,
            sequence_no: card.sequence_no,
            card_key: card.card_key,
            title: card.title,
            description: card.description,
            sprint_name: card.sprint_name,
            priority: card.priority,
            assignee_id: card.assignee_id,
            assignee_name: card.assignee_name,
            due_date: card.due_date,
            display_order: card.display_order,
            created_at: card.created_at,
            updated_at: card.updated_at,
            sprint_id: card.sprint_id,
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
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        column_id = ?payload.column_id,
        title = %payload.title,
        "create_card"
    );
    let card = service::create_card(&db, id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CardResponseDto {
        id: card.id,
        project_id: card.project_id,
        column_id: card.column_id,
        sequence_no: card.sequence_no,
        card_key: card.card_key,
        title: card.title,
        description: card.description,
        sprint_name: card.sprint_name,
        priority: card.priority,
        assignee_id: card.assignee_id,
        assignee_name: card.assignee_name,
        due_date: card.due_date,
        display_order: card.display_order,
        created_at: card.created_at,
        updated_at: card.updated_at,
        sprint_id: card.sprint_id,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn list_sprints_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(user_id = %user.id, project_id = %id, "list_sprints");
    let sprints = service::list_sprints(&db, id, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<SprintResponseDto> = sprints
        .into_iter()
        .map(|sprint| SprintResponseDto {
            id: sprint.id,
            project_id: sprint.project_id,
            name: sprint.name,
            goal: sprint.goal,
            start_date: sprint.start_date,
            end_date: sprint.end_date,
            status: sprint.status,
            created_at: sprint.created_at,
            updated_at: sprint.updated_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn create_sprint_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSprintDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(user_id = %user.id, project_id = %id, name = %payload.name, "create_sprint");
    let sprint = service::create_sprint(&db, id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = SprintResponseDto {
        id: sprint.id,
        project_id: sprint.project_id,
        name: sprint.name,
        goal: sprint.goal,
        start_date: sprint.start_date,
        end_date: sprint.end_date,
        status: sprint.status,
        created_at: sprint.created_at,
        updated_at: sprint.updated_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn update_sprint_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, sprint_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateSprintDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(user_id = %user.id, project_id = %id, sprint_id = %sprint_id, "update_sprint");
    let sprint = service::update_sprint(&db, id, sprint_id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = SprintResponseDto {
        id: sprint.id,
        project_id: sprint.project_id,
        name: sprint.name,
        goal: sprint.goal,
        start_date: sprint.start_date,
        end_date: sprint.end_date,
        status: sprint.status,
        created_at: sprint.created_at,
        updated_at: sprint.updated_at,
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_sprint_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, sprint_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!(user_id = %user.id, project_id = %id, sprint_id = %sprint_id, "delete_sprint");
    service::delete_sprint(&db, id, sprint_id, &user)
        .await
        .map_err(map_error)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_card_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateCardDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        column_id = ?payload.column_id,
        title = ?payload.title,
        "update_card"
    );
    let card = service::update_card(&db, id, card_id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CardResponseDto {
        id: card.id,
        project_id: card.project_id,
        column_id: card.column_id,
        sequence_no: card.sequence_no,
        card_key: card.card_key,
        title: card.title,
        description: card.description,
        sprint_name: card.sprint_name,
        priority: card.priority,
        assignee_id: card.assignee_id,
        assignee_name: card.assignee_name,
        due_date: card.due_date,
        display_order: card.display_order,
        created_at: card.created_at,
        updated_at: card.updated_at,
        sprint_id: card.sprint_id,
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_card_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "delete_card"
    );
    service::delete_card(&db, id, card_id, &user)
        .await
        .map_err(map_error)?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Card deleted successfully" })),
    ))
}

pub async fn list_card_comments_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "list_card_comments"
    );
    let comments = service::list_card_comments(&db, id, card_id, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<CardCommentResponseDto> = comments
        .into_iter()
        .map(|comment| CardCommentResponseDto {
            id: comment.id,
            project_id: comment.project_id,
            card_id: comment.card_id,
            user_id: comment.user_id,
            user_name: comment.user_name,
            comment: comment.comment,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn create_card_comment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateCardCommentDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "create_card_comment"
    );
    let comment = service::create_card_comment(&db, id, card_id, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CardCommentResponseDto {
        id: comment.id,
        project_id: comment.project_id,
        card_id: comment.card_id,
        user_id: comment.user_id,
        user_name: comment.user_name,
        comment: comment.comment,
        created_at: comment.created_at,
        updated_at: comment.updated_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn list_card_attachments_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "list_card_attachments"
    );
    let attachments = service::list_card_attachments(&db, id, card_id, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<CardAttachmentResponseDto> = attachments
        .into_iter()
        .map(|attachment| CardAttachmentResponseDto {
            id: attachment.id,
            project_id: attachment.project_id,
            card_id: attachment.card_id,
            uploaded_by: attachment.uploaded_by,
            uploader_name: attachment.uploader_name,
            file_name: attachment.file_name,
            content_type: attachment.content_type,
            file_size: attachment.file_size,
            created_at: attachment.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn upload_card_attachment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "upload_card_attachment"
    );

    let mut file_name = None;
    let mut content_type = None;
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        if field.name() != Some("file") {
            continue;
        }

        file_name = field.file_name().map(ToOwned::to_owned);
        content_type = field.content_type().map(ToOwned::to_owned);
        let bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        file_data = Some(bytes.to_vec());
        break;
    }

    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    let file_name = file_name.unwrap_or_else(|| "attachment.bin".to_string());
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());

    let attachment = service::upload_card_attachment(
        &db,
        id,
        card_id,
        &user,
        file_name,
        content_type,
        file_data,
    )
    .await
    .map_err(map_error)?;

    let response = CardAttachmentResponseDto {
        id: attachment.id,
        project_id: attachment.project_id,
        card_id: attachment.card_id,
        uploaded_by: attachment.uploaded_by,
        uploader_name: attachment.uploader_name,
        file_name: attachment.file_name,
        content_type: attachment.content_type,
        file_size: attachment.file_size,
        created_at: attachment.created_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn download_card_attachment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id, attachment_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        attachment_id = %attachment_id,
        "download_card_attachment"
    );
    let attachment = service::get_card_attachment_file(&db, id, card_id, attachment_id, &user)
        .await
        .map_err(map_error)?;

    let mut headers = HeaderMap::new();
    let content_type = HeaderValue::from_str(&attachment.content_type)
        .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));
    headers.insert(CONTENT_TYPE, content_type);

    let sanitized_name = attachment.file_name.replace(['\r', '\n', '"'], "_");
    let disposition = format!("attachment; filename=\"{sanitized_name}\"");
    let content_disposition = HeaderValue::from_str(&disposition)
        .unwrap_or_else(|_| HeaderValue::from_static("attachment"));
    headers.insert(CONTENT_DISPOSITION, content_disposition);

    Ok((StatusCode::OK, headers, attachment.file_data))
}

pub async fn list_card_history_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    tracing::info!(
        user_id = %user.id,
        project_id = %id,
        card_id = %card_id,
        "list_card_history"
    );
    let history = service::list_card_history(&db, id, card_id, &user)
        .await
        .map_err(map_error)?;

    let response: Vec<CardActivityResponseDto> = history
        .into_iter()
        .map(|activity| CardActivityResponseDto {
            id: activity.id,
            project_id: activity.project_id,
            card_id: activity.card_id,
            actor_id: activity.actor_id,
            actor_name: activity.actor_name,
            action_type: activity.action_type,
            description: activity.description,
            created_at: activity.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}
