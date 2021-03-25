//! Direct configuration ui test #1 - no Default impl.

#![cfg(not(all(feature = "envy_configuration")))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
struct EnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    let env_config = EnvConfig { server_port: 53812 };
    let conf1 = <EnvConfig as EasyConfiguration>::load().await.unwrap();
    panic!("Execution should not arrive here!");
}
