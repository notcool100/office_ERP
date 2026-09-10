use axum::{
    body::Body,
    extract::{Extension, Multipart, Path, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use super::{
    dto::{
        CreateFolderRequest, DocCategory, ListLocationQuery, OwnerQuery, UpdateDocumentRequest,
    },
    service,
};
use crate::{db::Db, models::user::User};

fn service_err(e: service::DocumentsError) -> Response {
    match e {
        service::DocumentsError::NotFound => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
        service::DocumentsError::BadRequest(msg) => {
            (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
        }
        service::DocumentsError::TooLarge => {
            (StatusCode::PAYLOAD_TOO_LARGE, e.to_string()).into_response()
        }
        service::DocumentsError::Database(err) => {
            tracing::error!("DB error in documents: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        service::DocumentsError::Io(err) => {
            tracing::error!("IO error in documents: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "File system error").into_response()
        }
    }
}

pub async fn list_location_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(_user): Extension<User>,
    Query(query): Query<ListLocationQuery>,
) -> Response {
    match service::list_location(&db, category, query.owner_id, query.folder_id).await {
        Ok(listing) => Json(listing).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn create_folder_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateFolderRequest>,
) -> Response {
    match service::create_folder(&db, &user, category, req).await {
        Ok(folder) => (StatusCode::CREATED, Json(folder)).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn delete_folder_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<OwnerQuery>,
) -> Response {
    match service::delete_folder(&db, &user, category, query.owner_id, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn get_document_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<OwnerQuery>,
) -> Response {
    match service::get_document_meta(&db, category, query.owner_id, id).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn update_document_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<OwnerQuery>,
    Json(dto): Json<UpdateDocumentRequest>,
) -> Response {
    match service::update_document(&db, category, query.owner_id, id, dto).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn delete_document_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<OwnerQuery>,
) -> Response {
    match service::delete_document(&db, category, query.owner_id, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn serve_document_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<OwnerQuery>,
) -> Response {
    let doc_file = match service::get_document_file(&db, category, query.owner_id, id).await {
        Ok(f) => f,
        Err(e) => return service_err(e),
    };

    let file = match File::open(&doc_file.file_path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found on disk").into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let disposition = format!("inline; filename=\"{}\"", doc_file.file_name);

    (
        [
            (header::CONTENT_TYPE, doc_file.content_type),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        body,
    )
        .into_response()
}

pub async fn upload_document_handler(
    Extension(db): Extension<Db>,
    Extension(category): Extension<DocCategory>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Response {
    let mut owner_id: Option<Uuid> = None;
    let mut folder_id: Option<Uuid> = None;
    let mut title = String::new();
    let mut doc_type: Option<String> = None;
    let mut description: Option<String> = None;
    let mut expiry_date: Option<chrono::NaiveDate> = None;
    let mut file_name = String::new();
    let mut content_type_str = String::from("application/octet-stream");
    let mut file_data: Vec<u8> = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("upload").to_string();
                if let Some(ct) = field.content_type() {
                    content_type_str = ct.to_string();
                }
                match field.bytes().await {
                    Ok(b) => file_data = b.to_vec(),
                    Err(_) => {
                        return (StatusCode::BAD_REQUEST, "Failed to read file").into_response();
                    }
                }
            }
            "ownerId" => {
                let v = field.text().await.unwrap_or_default();
                owner_id = Uuid::parse_str(&v).ok();
            }
            "folderId" => {
                let v = field.text().await.unwrap_or_default();
                folder_id = Uuid::parse_str(&v).ok();
            }
            "title" => {
                title = field.text().await.unwrap_or_default();
            }
            "docType" => {
                let v = field.text().await.unwrap_or_default();
                if !v.is_empty() {
                    doc_type = Some(v);
                }
            }
            "description" => {
                let v = field.text().await.unwrap_or_default();
                if !v.is_empty() {
                    description = Some(v);
                }
            }
            "expiryDate" => {
                let v = field.text().await.unwrap_or_default();
                expiry_date = chrono::NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok();
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return (StatusCode::BAD_REQUEST, "No file provided").into_response();
    }

    match service::upload_document(
        &db,
        &user,
        category,
        owner_id,
        folder_id,
        title,
        doc_type,
        description,
        expiry_date,
        file_name,
        content_type_str,
        file_data,
    )
    .await
    {
        Ok(doc) => (StatusCode::CREATED, Json(doc)).into_response(),
        Err(e) => service_err(e),
    }
}
