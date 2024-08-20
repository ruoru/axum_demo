use once_cell::sync::OnceCell;
use postgres_pool::PostgresPool;
use redis_pool::RedisPool;
use std::sync::Arc;
use axum::extract::Extension;
pub mod postgres_pool;
pub mod redis_pool;

pub struct DBPool {
    pub postgres_pool: PostgresPool,
    pub redis_pool: RedisPool,
}

impl DBPool {
    pub async fn new() -> Self {
        // 初始化 Redis 和 PostgreSQL 连接池
        let postgres_pool = PostgresPool::new().await;
        let redis_pool = RedisPool::new();

        DBPool {
            postgres_pool,
            redis_pool,
        }
    }
}

pub struct OnceDBPool {
    pub pools: Arc<DBPool>,
}

impl OnceDBPool {
    pub async fn get_once_dbpool() -> Extension<Arc<DBPool>> {
        let db_pool = DBPool::new().await;
        static POOL: OnceCell<Arc<DBPool>> = OnceCell::new();
        let once_pool = POOL.get_or_init(|| Arc::new(db_pool)).clone();
        Extension(once_pool)
    }
}
