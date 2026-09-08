use axum::{
    Json,
    body::Body,
    extract::{Extension, Multipart},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::db::Db;

use super::{dto::*, service};

fn error_response(err: service::CompanySettingsError) -> (StatusCode, Json<serde_json::Value>) {
    let status = match err {
        service::CompanySettingsError::NotFound => StatusCode::NOT_FOUND,
        service::CompanySettingsError::BadRequest(_) => StatusCode::BAD_REQUEST,
        service::CompanySettingsError::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
        service::CompanySettingsError::Database(ref e) => {
            tracing::error!("[company_settings] database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        service::CompanySettingsError::Io(ref e) => {
            tracing::error!("[company_settings] io error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    (
        status,
        Json(serde_json::json!({ "message": err.to_string() })),
    )
}

fn row_to_dto(row: service::CompanySettingsRow) -> CompanySettingsResponseDto {
    CompanySettingsResponseDto {
        name: row.name,
        logo_url: row.logo_path.map(|_| "/company-settings/logo".to_string()),
        favicon_url: row
            .favicon_path
            .map(|_| "/company-settings/favicon".to_string()),
        latitude: row.latitude,
        longitude: row.longitude,
        location_name: row.location_name,
        updated_at: row.updated_at,
    }
}

pub async fn get_settings_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    match service::get_settings(&db).await {
        Ok(row) => (StatusCode::OK, Json(serde_json::json!(row_to_dto(row)))).into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn update_details_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<UpdateCompanyDetailsDto>,
) -> impl IntoResponse {
    match service::update_details(
        &db,
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.location_name,
    )
    .await
    {
        Ok(row) => (StatusCode::OK, Json(serde_json::json!(row_to_dto(row)))).into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

async fn read_image_field(mut multipart: Multipart) -> Result<(String, String, Vec<u8>), Response> {
    let mut file_name = String::from("upload");
    let mut content_type = String::from("application/octet-stream");
    let mut data: Vec<u8> = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            file_name = field.file_name().unwrap_or("upload").to_string();
            if let Some(ct) = field.content_type() {
                content_type = ct.to_string();
            }
            match field.bytes().await {
                Ok(b) => data = b.to_vec(),
                Err(_) => {
                    return Err((StatusCode::BAD_REQUEST, "Failed to read file").into_response());
                }
            }
        }
    }

    Ok((file_name, content_type, data))
}

pub async fn upload_logo_handler(
    Extension(db): Extension<Db>,
    multipart: Multipart,
) -> impl IntoResponse {
    let (file_name, content_type, data) = match read_image_field(multipart).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };

    match service::update_logo(&db, &file_name, &content_type, &data).await {
        Ok(row) => (StatusCode::OK, Json(serde_json::json!(row_to_dto(row)))).into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn upload_favicon_handler(
    Extension(db): Extension<Db>,
    multipart: Multipart,
) -> impl IntoResponse {
    let (file_name, content_type, data) = match read_image_field(multipart).await {
        Ok(v) => v,
        Err(resp) => return resp,
    };

    match service::update_favicon(&db, &file_name, &content_type, &data).await {
        Ok(row) => (StatusCode::OK, Json(serde_json::json!(row_to_dto(row)))).into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

async fn serve_stored_file(stored: service::StoredFile) -> Response {
    let file = match File::open(&stored.file_path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found on disk").into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    (
        [
            (header::CONTENT_TYPE, stored.content_type),
            (header::CACHE_CONTROL, "public, max-age=300".to_string()),
        ],
        body,
    )
        .into_response()
}

pub async fn serve_logo_handler(Extension(db): Extension<Db>) -> Response {
    match service::get_logo_file(&db).await {
        Ok(stored) => serve_stored_file(stored).await,
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn serve_favicon_handler(Extension(db): Extension<Db>) -> Response {
    match service::get_favicon_file(&db).await {
        Ok(stored) => serve_stored_file(stored).await,
        Err(e) => error_response(e).into_response(),
    }
}
