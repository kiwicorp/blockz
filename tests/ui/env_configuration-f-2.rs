//! Env configuration ui test #2 - prefix source is not a String.

#![cfg(feature = "env_configuration")]

use blockz::prelude::*;

use serde::Deserialize;

const MY_PREFIX: &str = "MY_PREFIX_";

#[derive(Configuration, Deserialize, PartialEq)]
#[configuration(env(prefix_source = "MY_PREFIX"))]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    panic!("This should not run!");
}
