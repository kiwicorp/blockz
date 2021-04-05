//! Env configuration ui test #1 - prefix.

#![cfg(feature = "env_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env(prefix = "PREFIX_"))]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    std::env::set_var("PREFIX_SERVER_PORT", "1234");

    let conf1 = <MyConfig as EasyConfiguration>::load()
        .await
        .unwrap();
    let conf2 = <MyConfig as Configuration>::load(())
        .await
        .unwrap();
    assert!(conf1 == conf2);
}
