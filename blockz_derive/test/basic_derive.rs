//! Basic derive test.

#[macro_use]
extern crate blockz_derive;

use serde::Deserialize;

#[derive(Configuration, Deserialize)]
#[env_prefix("PREFIX")]
struct SingletonExampleConfig;

#[derive(Singleton)]
struct SingletonExample;

fn main() {}
