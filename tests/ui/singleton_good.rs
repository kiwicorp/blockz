//! Singleton ui test.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

use std::sync::Arc;

#[derive(Singleton)]
#[singleton(lock = "rwlock")]
struct DummyRwLock(Vec<i32>);

impl DummyRwLock {
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
    pub async fn check_equals(&self, other: Arc<Vec<i32>>) -> bool {
        self.0.as_slice() == *other
    }
}

#[derive(Singleton)]
#[singleton(lock = "mutex")]
struct DummyMutex(Vec<i32>);

impl DummyMutex {
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
    pub async fn check_equals(&self, other: Arc<Vec<i32>>) -> bool {
        self.0.as_slice() == *other
    }
}

#[tokio::main]
pub async fn main() {
    DummyMutex::init_singleton(DummyMutex(vec![-5, 3])).unwrap();
    DummyRwLock::init_singleton(DummyRwLock(vec![-5, 3])).unwrap();

    assert!(
        !DummyMutex::is_vec_empty().await,
        "DummyMutex should not have had an empty Vec!"
    );
    assert!(
        !DummyRwLock::is_vec_empty().await,
        "DummyRwLock should not have had an empty Vec!"
    );

    let arr = Arc::new(vec![-5, 3]);
    assert!(
        DummyMutex::check_equals(arr.clone()).await,
        "DummyMutex should have had a vec that is equal to [-5, 3]!"
    );
    assert!(
        DummyRwLock::check_equals(arr.clone()).await,
        "DummyRwLock should have had a vec that is equal to [-5, 3]!"
    );

    let get = DummyMutex::get_set(2 as usize, 3 as i32).await;
    assert!(get.is_none());
    let get = DummyRwLock::get_set(2 as usize, 3 as i32).await;
    assert!(get.is_none());

    let get = DummyMutex::get_set(0 as usize, -4 as i32).await;
    assert_eq!(get, Some(-5));
    let get = DummyRwLock::get_set(0 as usize, -4 as i32).await;
    assert_eq!(get, Some(-5));

    let arr = Arc::new(vec![-4, 3]);
    assert!(
        DummyMutex::check_equals(arr.clone()).await,
        "DummyMutex should have had a vec that is equal to [-4, 3]!"
    );
    let arr = Arc::new(vec![-4, 3]);
    assert!(
        DummyRwLock::check_equals(arr.clone()).await,
        "DummyRwLock should have had a vec that is equal to [-4, 3]!"
    );

    DummyMutex::clear().await;
    assert!(
        DummyMutex::is_vec_empty().await,
        "DummyMutex should've had an empty Vec!"
    );
    DummyRwLock::clear().await;
    assert!(
        DummyRwLock::is_vec_empty().await,
        "DummyRwLock should've had an empty Vec!"
    );
}
