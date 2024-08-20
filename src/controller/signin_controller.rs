use axum::{routing::post, Router};

use crate::service::user_service::update_redis;

pub fn create_signin_router() -> Router {
    Router::new().route("/", post(update_redis))
}
