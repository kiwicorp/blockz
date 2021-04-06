//! Direct configuration ui test #2 - derived default.

#![cfg(all(feature = "configuration", not(any(feature = "env_configuration"))))]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

#[derive(Configuration, Default, PartialEq)]
struct MyConfig {
    optional: Option<()>,
}

#[tokio::main]
async fn main() {
    let env_config = MyConfig { optional: None };
    let conf1 = <MyConfig as EasyConfiguration>::load().await;
    let conf2 = <MyConfig as Configuration>::load(env_config).await;
    assert!(conf1 == conf2);
}
