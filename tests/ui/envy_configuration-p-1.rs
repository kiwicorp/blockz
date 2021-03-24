//! Envy configuration ui test #1 - default.

#![cfg(feature = "envy_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(envy())]
struct EnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    // set the required env variables
    std::env::set_var("SERVER_PORT", "1234");
    std::env::set_var("MANUAL_PREFIX_SERVER_PORT", "5678");

    let conf1 = <EnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <EnvConfig as Configuration>::load(None).await.unwrap();
    assert!(conf1 == conf2);

    let conf = <EnvConfig as Configuration>::load(Some("MANUAL_PREFIX_".into()))
        .await
        .unwrap();
    assert!(conf.server_port == 5678 as u32);
}
