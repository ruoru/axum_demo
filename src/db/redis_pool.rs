use deadpool_redis::{Config, Pool, Runtime};
use std::env;

pub struct RedisPool {
    pub pool: Pool,
}

impl RedisPool {
    pub fn new() -> Self {
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let cfg = Config::from_url(redis_url);
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis pool");
        RedisPool { pool }
    }
}
