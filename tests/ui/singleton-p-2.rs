//! Singleton test pass #3 - rw lock and singleton fns.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

#[derive(Singleton)]
#[singleton(lock = "rwlock")]
struct Dummy(Vec<i32>);

impl Dummy {
    #[singleton_fn]
    pub async fn is_vec_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[singleton_fn]
    async fn clear(&mut self) {
        self.0.clear();
    }

    #[singleton_fn]
    pub async fn get_set(&mut self, get_index: usize, set: i32) -> Option<i32> {
        let get_value = self.0.get(get_index).map(|val| *val);
        if let Some(_) = get_value {
            *self.0.get_mut(get_index).unwrap() = set;
            get_value
        } else {
            None
        }
    }

    #[singleton_fn]
    pub async fn check_equals(&self, other: Box<[i32]>) -> bool {
        self.0.as_slice() == &*other
    }
}

#[tokio::main]
pub async fn main() {
    Dummy::init_singleton(Dummy(vec![-5, 3])).unwrap();

    assert!(
        !Dummy::is_vec_empty().await,
        "Dummy should not have had an empty Vec!"
    );

    assert!(
        Dummy::check_equals([-5, 3].into()).await,
        "Dummy should have had a vec that is equal to [-5, 3]!"
    );

    let get = Dummy::get_set(2 as usize, 3 as i32).await;
    assert!(get.is_none());

    let get = Dummy::get_set(0 as usize, -4 as i32).await;
    assert_eq!(get, Some(-5));

    assert!(
        Dummy::check_equals([-4, 3].into()).await,
        "Dummy should have had a vec that is equal to [-4, 3]!"
    );

    Dummy::clear().await;
    assert!(
        Dummy::is_vec_empty().await,
        "Dummy should've had an empty Vec!"
    );
}
