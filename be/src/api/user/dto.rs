use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub person_id: Uuid,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub is_admin: Option<bool>,
}
