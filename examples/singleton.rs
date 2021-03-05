//! Singleton ui test.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

use std::sync::Arc;

#[derive(Singleton)]
#[singleton(lock = "rwlock")]
struct DummyRwLock(Vec<i32>);

impl DummyRwLock {
    #[singleton_fn]
    pub async fn is_vec_empty(&self) -> anyhow::Result<bool> {
        Ok(self.0.is_empty())
    }

    #[singleton_fn]
    async fn clear(&mut self) -> anyhow::Result<()> {
        self.0.clear();
        Ok(())
    }

    #[singleton_fn]
    pub async fn get_set(&mut self, get_index: usize, set: i32) -> anyhow::Result<Option<i32>> {
        let get_value = self.0.get(get_index).map(|val| *val);
        if let Some(_) = get_value {
            *self.0.get_mut(get_index).unwrap() = set;
            Ok(get_value)
        } else {
            Ok(None)
        }
    }

    #[singleton_fn]
    pub async fn check_equals(&self, other: Arc<Vec<i32>>) -> anyhow::Result<bool> {
        Ok(self.0.as_slice() == *other)
    }
}

#[derive(Singleton)]
#[singleton(lock = "mutex")]
struct DummyMutex(Vec<i32>);

impl DummyMutex {
    #[singleton_fn]
    pub async fn is_vec_empty(&self) -> anyhow::Result<bool> {
        Ok(self.0.is_empty())
    }

    #[singleton_fn]
    async fn clear(&mut self) -> anyhow::Result<()> {
        self.0.clear();
        Ok(())
    }

    #[singleton_fn]
    pub async fn get_set(&mut self, get_index: usize, set: i32) -> anyhow::Result<Option<i32>> {
        let get_value = self.0.get(get_index).map(|val| *val);
        if let Some(_) = get_value {
            *self.0.get_mut(get_index).unwrap() = set;
            Ok(get_value)
        } else {
            Ok(None)
        }
    }

    #[singleton_fn]
    pub async fn check_equals(&self, other: Arc<Vec<i32>>) -> anyhow::Result<bool> {
        Ok(self.0.as_slice() == *other)
    }
}

#[tokio::main]
pub async fn main() {
    DummyMutex::init_singleton(DummyMutex(vec![-5, 3])).unwrap();
    DummyRwLock::init_singleton(DummyRwLock(vec![-5, 3])).unwrap();

    assert!(
        !DummyMutex::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "DummyMutex should not have had an empty Vec!"
    );
    assert!(
        !DummyRwLock::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "DummyRwLock should not have had an empty Vec!"
    );

    let arr = Arc::new(vec![-5, 3]);
    assert!(
        DummyMutex::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "DummyMutex should have had a vec that is equal to [-5, 3]!"
    );
    assert!(
        DummyRwLock::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "DummyRwLock should have had a vec that is equal to [-5, 3]!"
    );

    let get = DummyMutex::get_set(2 as usize, 3 as i32)
        .await
        .expect("Failed to do DummyMutex::get_set");
    assert!(get.is_none());
    let get = DummyRwLock::get_set(2 as usize, 3 as i32)
        .await
        .expect("Failed to do DummyRwLock::get_set");
    assert!(get.is_none());

    let get = DummyMutex::get_set(0 as usize, -4 as i32)
        .await
        .expect("Failed to do DummyMutex::get_set");
    assert_eq!(get, Some(-5));
    let get = DummyRwLock::get_set(0 as usize, -4 as i32)
        .await
        .expect("Failed to do DummyRwLock::get_set");
    assert_eq!(get, Some(-5));

    let arr = Arc::new(vec![-4, 3]);
    assert!(
        DummyMutex::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "DummyMutex should have had a vec that is equal to [-4, 3]!"
    );
    let arr = Arc::new(vec![-4, 3]);
    assert!(
        DummyRwLock::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "DummyRwLock should have had a vec that is equal to [-4, 3]!"
    );

    DummyMutex::clear().await.expect("Failed to clear DummyMutex!");
    assert!(
        DummyMutex::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "DummyMutex should've had an empty Vec!"
    );
    DummyRwLock::clear().await.expect("Failed to clear DummyRwLock!");
    assert!(
        DummyRwLock::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "DummyRwLock should've had an empty Vec!"
    );
}
