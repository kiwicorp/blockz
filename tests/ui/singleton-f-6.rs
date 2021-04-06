//! Singleton test fail #6 - first function input is not a receiver.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

#[derive(Singleton)]
struct Dummy(Vec<i32>);

impl Dummy {
    #[singleton_fn]
    pub async fn is_vec_empty(recv: i32) -> bool {
        panic!("This should not run!");
    }

    #[singleton_fn]
    async fn clear(bad_recv: u32) {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn get_set(fail: String) -> Option<i32> {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn check_equals(another_fail: Option<()>) -> bool {
        panic!("This should not run!");
    }
}

#[tokio::main]
pub async fn main() {
    panic!("This should not run!");
}
