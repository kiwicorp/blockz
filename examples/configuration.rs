//! Configuration ui test.

#![cfg(feature = "configuration")]

use blockz::configuration::EasyConfiguration;
use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize)]
#[configuration(
    envy(
        prefix = "MY_PREFIX",
        prefix_source = "MY_CONSTANT",
        default_prefix = "AB",
        default_prefix_source = "abc()",
    )
    json_file(path = "/a/b/c", path_source = ""),
    json_http())]
struct MyEnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    <MyEnvConfig as EasyConfiguration>::load().await.unwrap();
    <MyEnvConfig as Configuration>::load(Some("PREFIX_".to_string())).await.unwrap();
}
