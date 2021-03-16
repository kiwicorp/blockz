//! Envy configuration ui test.

#![cfg("envy_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
struct EnvConfig {
    server_port: u32,
}

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(envy(prefix = "PREFIX_"))]
struct PrefixedEnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    // set the required env variables
    std::env::set_var("SERVER_PORT", "1234");
    std::env::set_var("PREFIX_SERVER_PORT", "5678");

    let conf1 = <EnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <EnvConfig as Configuration>::load(None).await.unwrap();
    assert!(conf1 == conf2);

    let conf1 = <PrefixedEnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <PrefixedEnvConfig as Configuration>::load(()).await.unwrap();
    assert!(conf1 == conf2);
}
