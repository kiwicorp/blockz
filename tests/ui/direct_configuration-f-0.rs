//! Direct configuration ui test #0 - typo.

#![cfg(all(feature = "configuration", not(any(feature = "env_configuration"))))]

use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
#[configuration(direc)]
struct MyConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    panic!("This should not run!");
}
