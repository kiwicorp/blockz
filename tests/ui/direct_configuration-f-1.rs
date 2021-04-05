//! Direct configuration ui test #1 - typo.

// #![cfg(not(all(feature = "env_configuration")))]
#![cfg(all(
    feature = "configuration",
    not(any(
        feature = "env_configuration",
    ))))]

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
