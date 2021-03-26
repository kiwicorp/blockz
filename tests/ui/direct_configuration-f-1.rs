//! Direct configuration ui test #1 - typo.

#![cfg(not(all(feature = "envy_configuration")))]

use blockz::prelude::*;

#[derive(Configuration, PartialEq)]
#[configuration(direc)]
struct EnvConfig {
    server_port: u32,
}

#[tokio::main]
async fn main() {
    panic!("This should not run!");
}
