//! Redis database abstractions.

use deadpool::Runtime;
use deadpool_redis::Hook;
use deadpool_redis::Manager;
use deadpool_redis::Pool;
use deadpool_redis::PoolBuilder;
use deadpool_redis::PoolConfig;
use deadpool_redis::Timeouts;

use super::DatabasePool;

pub trait RedisPool: DatabasePool {}

fn playground() {
    let manager = Manager::new("").unwrap();
    let pool = Pool::builder(manager)
        .create_timeout(None)
        .recycle_timeout(None)
        .wait_timeout(None)
        .max_size(32)
        .runtime(Runtime::Tokio1)
        .post_create(Hook::sync_fn(|conn, metrics| {
            todo!();
            Ok(())
        }))
        .pre_recycle(Hook::sync_fn(|conn, metrics| {
            todo!();
            Ok(())
        }))
        .post_recycle(Hook::sync_fn(|conn, metrics| {
            todo!();
            Ok(())
        }))
        .build()
        .unwrap();
}
