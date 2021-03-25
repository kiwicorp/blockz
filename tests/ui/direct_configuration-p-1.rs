//! Direct configuration ui test #1 - default.

#![cfg(not(all(feature = "envy_configuration")))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
struct EnvConfig {
    server_port: u32,
}

impl Default for EnvConfig {
    fn default() -> EnvConfig {
        EnvConfig { server_port: 53812 }
    }
}

#[tokio::main]
async fn main() {
    let env_config = EnvConfig { server_port: 53812 };
    let conf1 = <EnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <EnvConfig as Configuration>::load(env_config).await.unwrap();
    assert!(conf1 == conf2);
}
