//! Env configuration ui test #0 - typos.

#![cfg(feature = "env_configuration")]

use blockz::prelude::*;

use serde::Deserialize;

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(en())]
struct MyConfig {
    server_port: u32,
}

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env(prefx = "ABCD_"))]
struct MySecondConfig {
    server_port: u32,
}

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env(prefix_src = "\"abc_\".to_string()"))]
struct MyThirdConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    panic!("This should not run!");
}
