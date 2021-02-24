//! Singleton ui test.

#![cfg(feature = "singleton")]

use blockz::prelude::*;

use std::sync::Arc;

#[derive(Singleton)]
struct Dummy(Vec<i32>);

impl Dummy {
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
        if let Some(value) = get_value {
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
    Dummy::init_singleton(Dummy(vec![-5, 3])).unwrap();

    assert!(
        !Dummy::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "Dummy should not have had an empty Vec!"
    );

    let arr = Arc::new(vec![-5, 3]);
    assert!(
        Dummy::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "Dummy should have had a vec that is equal to [-5, 3]!"
    );

    let get = Dummy::get_set(2 as usize, 3 as i32)
        .await
        .expect("Failed to do Dummy::get_set");
    assert!(get.is_none());

    let get = Dummy::get_set(0 as usize, -4 as i32)
        .await
        .expect("Failed to do Dummy::get_set");
    assert_eq!(get, Some(-5));

    let arr = Arc::new(vec![-4, 3]);
    assert!(
        Dummy::check_equals(arr.clone())
            .await
            .expect("Failed to check if two vecs are equal"),
        "Dummy should have had a vec that is equal to [-4, 3]!"
    );

    Dummy::clear().await.expect("Failed to clear Dummy!");
    assert!(
        Dummy::is_vec_empty()
            .await
            .expect("Failed to check if vec is empty!"),
        "Dummy should've had an empty Vec!"
    );
}
