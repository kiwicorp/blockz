//! Direct configuration ui test #1 - no Default impl.

#![cfg(all(feature = "configuration", not(any(feature = "env_configuration"))))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    let env_config = MyConfig { server_port: 53812 };
    let conf1 = <MyConfig as EasyConfiguration>::load().await;
    panic!("Execution should not arrive here!");
}
