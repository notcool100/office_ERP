use axum::{
    body::Body,
    http::{self, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::{api::auth::jwt::validate_token, errors::AuthError};

pub async fn authenticate(mut req: Request<Body>, next: Next) -> Response<Body> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);
    let token = if let Some(header) = auth_header {
        let value = match header.to_str() {
            Ok(v) => v,
            Err(_) => {
                return AuthError {
                    message: "Invalid header value".to_string(),
                    status_code: StatusCode::FORBIDDEN,
                }
                .into_response();
            }
        };
        let mut parts = value.split_whitespace();
        let _bearer = parts.next();
        parts.next().map(|s| s.to_string())
    } else {
        // Fallback to query parameter for WebSockets
        req.uri().query().and_then(|q| {
            q.split('&')
                .find(|p| p.starts_with("token="))
                .map(|p| p.replace("token=", ""))
        })
    };

    let token = match token {
        Some(t) => t,
        None => {
            return AuthError {
                message: "Authentication token missing".to_string(),
                status_code: StatusCode::FORBIDDEN,
            }
            .into_response();
        }
    };
    let token_data = match validate_token(&token) {
        Some(data) => data,
        None => {
            return AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            }
            .into_response();
        }
    };

    let db = match req.extensions().get::<crate::db::Db>() {
        Some(db) => db,
        None => {
            return AuthError {
                message: "Database connection missing".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
            .into_response();
        }
    };

    let current_user = match crate::api::user::service::get_by_id(db, token_data.sub).await {
        Ok(user) => user,
        Err(_) => {
            return AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            }
            .into_response();
        }
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
}
