//! Env configuration ui test #7 - prefix source (constant).

#![cfg(feature = "env_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

const ENV_CONFIG_PREFIX: &str = "SOURCED_PREFIX_";

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env(prefix_source = "self::ENV_CONFIG_PREFIX.to_string()"))]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    // set the required env variables
    std::env::set_var("SOURCED_PREFIX_SERVER_PORT", "1234");

    let conf1 = <MyConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <MyConfig as Configuration>::load(()).await.unwrap();
    assert!(conf1 == conf2);
}
