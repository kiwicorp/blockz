//! Direct configuration ui test #0 - default.

#![cfg(all(feature = "configuration", not(any(feature = "env_configuration"))))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

/// Default server port for my configuration.
const MY_CONFIG_DEFAULT_SERVER_PORT: u32 = 53812;

#[derive(Configuration, PartialEq)]
struct MyConfig {
    server_port: u32,
}

impl Default for MyConfig {
    fn default() -> MyConfig {
        MyConfig { server_port: MY_CONFIG_DEFAULT_SERVER_PORT }
    }
}

#[tokio::main]
async fn main() {
    let env_config = MyConfig { server_port: MY_CONFIG_DEFAULT_SERVER_PORT };
    let conf1 = <MyConfig as EasyConfiguration>::load().await;
    let conf2 = <MyConfig as Configuration>::load(env_config).await;
    assert!(conf1 == conf2);
}
