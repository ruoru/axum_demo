use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub struct PostgresPool {
    pub pool: PgPool,
}

impl PostgresPool {
    pub async fn new() -> Self {
        // 从环境变量中获取数据库 URL
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // 创建 PostgreSQL 连接池
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_lazy(&database_url)
            .expect("Failed to create pool");

        PostgresPool { pool }
    }
}
