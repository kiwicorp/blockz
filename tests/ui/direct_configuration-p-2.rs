//! Direct configuration ui test #1 - specific.

#![cfg(all(
    feature = "configuration",
    not(any(
        feature = "env_configuration",
    ))))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
#[configuration(direct)]
struct MyConfig {
    server_port: u32,
}

impl Default for MyConfig {
    fn default() -> MyConfig {
        MyConfig { server_port: 53812 }
    }
}

#[tokio::main]
async fn main() {
    let env_config = MyConfig { server_port: 53812 };
    let conf1 = <MyConfig as EasyConfiguration>::load().await;
    let conf2 = <MyConfig as Configuration>::load(env_config).await;
    assert!(conf1 == conf2);
}
