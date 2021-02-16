//! Singleton ui test.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

#[derive(Singleton)]
struct Dummy(Vec<i32>);

impl Dummy {
    #[singleton_fn]
    async fn get_set(&mut self, get: usize, set: i32) -> Option<i32> {
        let get_value = self.0.get(get).map(|val| val.clone());
        if get_value.is_none() {
            None
        } else {
            self.0.insert(get, set);
            get_value
        }
    }

    #[singleton_fn]
    async fn check_equals(&self, other: &[i32]) -> bool {
        self.0.as_slice() == other
    }
}

#[tokio::main]
async fn main() {
    Dummy::init_singleton(Dummy(vec![-5, 3])).unwrap();

    let get = Dummy::get_set(2 as usize, 3 as i32).await;
    assert!(get.is_none());

    let get = Dummy::get_set(0 as usize, -4 as i32).await;
    assert_eq!(get, Some(-5));

    let arr: [i32; 2] = [-4, 3];
    assert!(Dummy::check_equals(&arr).await);
}
