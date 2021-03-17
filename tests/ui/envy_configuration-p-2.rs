//! Envy configuration ui test #1.

#![cfg("envy_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(envy(prefix = "PREFIX_"))]
struct EnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    std::env::set_var("PREFIX_SERVER_PORT", "1234");

    let conf1 = <PrefixedEnvConfig as EasyConfiguration>::load()
        .await
        .unwrap();
    let conf2 = <PrefixedEnvConfig as Configuration>::load(())
        .await
        .unwrap();
    assert!(conf1 == conf2);
}
