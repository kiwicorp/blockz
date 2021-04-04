//! Envy configuration ui test #6 - prefix source (constant).

#![cfg(feature = "envy_configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

fn get_prefix() -> Result<String, envy::Error> {
    std::env::var("BLOCKZ_UI_TEST_ENV_PREFIX")
        .map_err(|err| envy::Error::Custom(format!("{}", err)))
}

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(envy(prefix_source = "self::get_prefix()?"))]
struct EnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    // set the required env variables
    std::env::set_var("BLOCKZ_UI_TEST_ENV_PREFIX", "SOURCED_PREFIX_");
    std::env::set_var("SOURCED_PREFIX_SERVER_PORT", "1234");

    let conf1 = <EnvConfig as EasyConfiguration>::load().await.unwrap();
    let conf2 = <EnvConfig as Configuration>::load(()).await.unwrap();
    assert!(conf1 == conf2);
}
