use axum::{
    body::Body,
    http::{self, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::{api::auth::jwt::validate_token, errors::AuthError};

pub async fn authenticate(mut req: Request<Body>, next: Next) -> Response<Body> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => match header.to_str() {
            Ok(value) => value,
            Err(_) => {
                return AuthError {
                    message: "Empty header is not allowed".to_string(),
                    status_code: StatusCode::FORBIDDEN,
                }
                .into_response();
            }
        },
        None => {
            return AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            }
            .into_response();
        }
    };

    let mut parts = auth_header.split_whitespace();
    let _bearer = parts.next();
    let token = parts.next();
    fn fun_name(t: &str) -> Option<crate::api::auth::jwt::Claims> {
        validate_token(t)
    }

    let token_data = match token.and_then(fun_name) {
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
