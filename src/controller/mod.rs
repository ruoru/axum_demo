use axum::Router;

mod signin_controller;
mod user_controller;

pub fn create_app_router() -> Router {
    Router::new()
        .nest("/users", user_controller::create_user_router())
        .nest("/signin", signin_controller::create_signin_router())
}
