//! blockz-databases
//!
//! Utilities for working with databases:
//!
//! - database pools
//! - configuration
//! - etc.

use std::collections::HashMap;

use serde::Deserialize;
use url::Url;

pub mod redis;

/// A database pool.
pub trait DatabasePool {
    /// The name of this database pool.
    fn name() -> &'static str;
}

#[derive(Deserialize)]
pub struct Config {
    default_pool_size: usize,
    #[serde(flatten)]
    config: HashMap<String, DatabasePoolConfig>,
}

/// Configuration for a database connection pool.
#[derive(Deserialize)]
pub struct DatabasePoolConfig {
    /// The address of the database.
    pub addr: Url,
    /// Pool size for this database.
    pub pool_size: Option<usize>,
}
