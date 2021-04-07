//! Env configuration ui test #0 - default opts.

#![cfg(feature = "env_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env())]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    // set the required env variables
    std::env::set_var("SERVER_PORT", "1234");
    std::env::set_var("MANUAL_PREFIX_SERVER_PORT", "5678");

    let conf1 = <MyConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <MyConfig as Configuration>::load(None).await.unwrap();
    assert!(conf1 == conf2);

    let conf = <MyConfig as Configuration>::load(Some("MANUAL_PREFIX_".into()))
        .await
        .unwrap();
    assert!(conf.server_port == 5678 as u32);
}
