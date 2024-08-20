use crate::db;
use crate::model::user_model::User;
use deadpool_redis::redis::AsyncCommands;
use sqlx;

pub async fn list_users() -> Vec<User> {
    // 创建共享状态
    let once_pool = db::OnceDBPool::get_once_dbpool().await;

    let users = sqlx::query_as!(User, "SELECT id, name, email, notes, phone FROM users")
        .fetch_all(&once_pool.postgres_pool.pool)
        .await
        .expect("Failed to fetch users.");

    users
}

pub async fn create_user(payload: User) -> User {
    // 创建共享状态
    let once_pool = db::OnceDBPool::get_once_dbpool().await;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email, notes, phone
        "#,
        payload.name,
        payload.email
    )
    .fetch_one(&once_pool.postgres_pool.pool)
    .await
    .expect("Failed to insert user.");

    user
}

pub async fn delete_user(id: i32) -> bool {
    // 创建共享状态
    let once_pool = db::OnceDBPool::get_once_dbpool().await;

    sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
        .execute(&once_pool.postgres_pool.pool)
        .await
        .expect("Failed to delete user.");

    true
}

pub async fn update_user(id: i32, payload: User) -> User {
    // 创建共享状态
    let once_pool = db::OnceDBPool::get_once_dbpool().await;

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET name = $1, email = $2
        WHERE id = $3
        RETURNING id, name, email, notes, phone
        "#,
        payload.name,
        payload.email,
        id
    )
    .fetch_one(&once_pool.postgres_pool.pool)
    .await
    .expect("Failed to update user.");

    user
}

pub async fn update_redis(payload: User) -> String {
    // 创建共享状态
    let once_pool = db::OnceDBPool::get_once_dbpool().await;

    // 获取 deadpool 连接
    let redis_conn = &mut once_pool
        .redis_pool
        .pool
        .get()
        .await
        .expect("Failed to get Redis connection");

    // 使用 redis::AsyncCommands 进行 Redis 操作
    let _: () = redis_conn
        .set(&payload.name, &payload.email)
        .await
        .expect("Failed to set key");

    // 获取并返回键值
    let value: String = redis_conn
        .get(&payload.name)
        .await
        .expect("Failed to get key");

    value
}
