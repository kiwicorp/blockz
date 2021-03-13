//! Configuration ui test.

#![cfg(feature = "configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(envy(prefix = "PREFIX_"))]
struct MyEnvConfig {
    server_port: u32,
}

#[derive(Configuration, Deserialize, PartialEq)]
struct OtherEnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    let conf1 = <MyEnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <MyEnvConfig as Configuration>::load(()).await.unwrap();
    assert!(conf1 == conf2);

    let conf1 = <OtherEnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <OtherEnvConfig as Configuration>::load(None).await.unwrap();
    assert!(conf1 == conf2);
}
