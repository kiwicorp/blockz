//! Singleton test fail #2 - bad attribute macro args.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

#[derive(Singleton)]
#[singleton(a = "mutex", b = "rwlock", c)]
struct Dummy(Vec<i32>);

impl Dummy {
    #[singleton_fn]
    pub async fn is_vec_empty(&self) -> bool {
        panic!("This should not run!");
    }

    #[singleton_fn]
    async fn clear(&mut self) {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn get_set(&mut self, get: usize, set: i32) -> Option<i32> {
        panic!("This should not run!");
    }

    #[singleton_fn]
    pub async fn check_equals(&self, other: Box<[i32]>) -> bool {
        panic!("This should not run!");
    }
}

#[tokio::main]
pub async fn main() {
    panic!("This should not run!");
}
