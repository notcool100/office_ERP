use axum::{
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
};
use sqlx::FromRow;
use std::sync::Arc;
use tower::Layer;
use uuid::Uuid;

use crate::{db::Db, errors::AuthError, models::user::User};

#[derive(Clone)]
pub struct RbacConfig {
    nav_path: Arc<str>,
}

#[derive(Copy, Clone, Debug)]
enum PermissionAction {
    Create,
    Read,
    Update,
    Delete,
}

impl PermissionAction {
    fn from_method(method: &Method) -> Option<Self> {
        match *method {
            Method::GET | Method::HEAD => Some(Self::Read),
            Method::POST => Some(Self::Create),
            Method::PUT | Method::PATCH => Some(Self::Update),
            Method::DELETE => Some(Self::Delete),
            _ => None,
        }
    }
}

#[derive(Debug, FromRow)]
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

pub fn require_permission(nav_path: &str) -> impl Layer<Router> {
    let state = RbacConfig {
        nav_path: Arc::from(nav_path),
    };
    axum::middleware::from_fn_with_state(state, authorize)
}

async fn authorize(
    State(config): State<RbacConfig>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if req.method() == Method::OPTIONS {
        return next.run(req).await;
    }

    let action = match PermissionAction::from_method(req.method()) {
        Some(action) => action,
        None => return next.run(req).await,
    };

    let db = match req.extensions().get::<Db>() {
        Some(db) => db,
        None => {
            return AuthError {
                message: "Database connection missing".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
            .into_response();
        }
    };

    let user = match req.extensions().get::<User>() {
        Some(user) => user,
        None => {
            return AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            }
            .into_response();
        }
    };

    if user.is_admin {
        return next.run(req).await;
    }

    match fetch_permissions(db, user.id, &config.nav_path).await {
        Ok(Some(perms)) if perms.allows(action) => next.run(req).await,
        Ok(_) => AuthError {
            message: "You do not have permission to perform this action".to_string(),
            status_code: StatusCode::FORBIDDEN,
        }
        .into_response(),
        Err(_) => AuthError {
            message: "Failed to validate permissions".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response(),
    }
}

async fn fetch_permissions(
    db: &Db,
    user_id: Uuid,
    nav_path: &str,
) -> Result<Option<PermissionFlags>, sqlx::Error> {
    sqlx::query_as::<_, PermissionFlags>(
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
    .bind(user_id)
    .bind(nav_path)
    .fetch_optional(db)
    .await
}
