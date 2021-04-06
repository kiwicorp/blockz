//! Singleton test fail #4 - functions with no inputs.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

#[derive(Singleton)]
struct Dummy(Vec<i32>);

impl Dummy {
    #[singleton_fn]
    pub async fn is_vec_empty() -> bool {
        panic!("This should not run!");
    }

    #[singleton_fn]
    async fn clear() {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn get_set() -> Option<i32> {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn check_equals() -> bool {
        panic!("This should not run!");
    }
}

#[tokio::main]
pub async fn main() {
    panic!("This should not run!");
}
