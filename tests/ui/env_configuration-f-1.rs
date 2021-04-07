//! Env configuration ui test #1 - missing `Deserialize` impl.

#![cfg(feature = "env_configuration")]

use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
#[configuration(env(prefix = "MY_PREFIX_"))]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    panic!("This should not run!");
}
