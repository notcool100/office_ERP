use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub new_password: String,
}
