use axum::{
    routing::{delete, get},
    Router,
};

use crate::service::user_service::{create_user, delete_user, list_users, update_user};

pub fn create_user_router() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", delete(delete_user).put(update_user))
}
