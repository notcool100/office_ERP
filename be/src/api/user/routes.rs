use axum::{Router, routing::{get, put, delete as del}};

use crate::api::user::handler::{
    create_user_handler, 
    list_users_handler, 
    get_user_handler,
    update_user_handler,
    delete_user_handler,
    change_password_handler,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users_handler).post(create_user_handler))
        .route("/{id}", get(get_user_handler).put(update_user_handler).delete(del(delete_user_handler)))
        .route("/{id}/password", put(change_password_handler))
}
