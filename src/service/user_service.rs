use crate::model::user_model::User;
use crate::repository::user_repository;
use axum::{extract::Path, Json};

pub async fn list_users() -> Json<Vec<User>> {
    let users = user_repository::list_users().await;

    Json(users)
}

pub async fn create_user(Json(payload): Json<User>) -> Json<User> {
    let user = user_repository::create_user(payload).await;

    Json(user)
}

pub async fn delete_user(Path(id): Path<i32>) -> String {
    let user = user_repository::delete_user(id).await;

    user.to_string()
}

pub async fn update_user(Path(id): Path<i32>, Json(payload): Json<User>) -> Json<User> {
    let user = user_repository::update_user(id, payload).await;

    Json(user)
}

pub async fn update_redis(Json(payload): Json<User>) -> Json<User> {
    let value = user_repository::update_redis(payload).await;

    let user1 = User {
        id: 12,
        name: "123123".to_string(),
        email: Some(value),
        phone: Some("Hello".to_string()),
        notes: Some("aa".to_string()),
    };

    Json(user1)
}
