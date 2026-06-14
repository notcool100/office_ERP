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

use super::{dto::{ListAssetsQuery, UpdateAssetDto}, service};
use crate::{db::Db, models::user::User};

fn service_err(e: service::MediaLibraryError) -> Response {
    match e {
        service::MediaLibraryError::NotFound => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
        service::MediaLibraryError::Forbidden => {
            (StatusCode::FORBIDDEN, e.to_string()).into_response()
        }
        service::MediaLibraryError::BadRequest(msg) => {
            (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
        }
        service::MediaLibraryError::TooLarge => {
            (StatusCode::PAYLOAD_TOO_LARGE, e.to_string()).into_response()
        }
        service::MediaLibraryError::Database(err) => {
            tracing::error!("DB error in media_library: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        service::MediaLibraryError::Io(err) => {
            tracing::error!("IO error in media_library: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "File system error").into_response()
        }
    }
}

pub async fn list_assets_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(query): Query<ListAssetsQuery>,
) -> Response {
    match service::list_assets(&db, query).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn get_asset_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::get_asset_meta(&db, id).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn upload_asset_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Response {
    let mut name = String::new();
    let mut description: Option<String> = None;
    let mut category = String::from("other");
    let mut tags: Vec<String> = vec![];
    let mut file_name = String::new();
    let mut content_type_str = String::from("application/octet-stream");
    let mut file_data: Vec<u8> = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "file" => {
                file_name = field
                    .file_name()
                    .unwrap_or("upload")
                    .to_string();
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
            "name" => {
                name = field.text().await.unwrap_or_default();
            }
            "description" => {
                let v = field.text().await.unwrap_or_default();
                if !v.is_empty() {
                    description = Some(v);
                }
            }
            "category" => {
                category = field.text().await.unwrap_or_default();
            }
            "tags" => {
                let v = field.text().await.unwrap_or_default();
                tags = v
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return (StatusCode::BAD_REQUEST, "No file provided").into_response();
    }

    match service::upload_asset(
        &db,
        &user,
        name,
        description,
        category,
        tags,
        file_name,
        content_type_str,
        file_data,
    )
    .await
    {
        Ok(asset) => (StatusCode::CREATED, Json(asset)).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn update_asset_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateAssetDto>,
) -> Response {
    match service::update_asset(&db, &user, id, dto).await {
        Ok(asset) => Json(asset).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn delete_asset_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::delete_asset(&db, &user, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn serve_asset_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    let asset_file = match service::get_asset_file(&db, id).await {
        Ok(f) => f,
        Err(e) => return service_err(e),
    };

    let file = match File::open(&asset_file.file_path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found on disk").into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let disposition = format!("inline; filename=\"{}\"", asset_file.file_name);

    (
        [
            (header::CONTENT_TYPE, asset_file.content_type),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        body,
    )
        .into_response()
}
