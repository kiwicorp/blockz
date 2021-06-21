//! Support for singletons.
//!
//! # Example
//!
//! This example showcases how you can use the Singleton trait by leveraging procedural macros.
//!
//! ```
//! # #[cfg(feature = "singleton")]
//! # {
//! # use blockz::prelude::*;
//! # use std::error::Error;
//! # use std::io;
//! # use std::net::SocketAddr;
//! # use std::str::FromStr;
//! // For the sake of simplicity, in this example the database connection pool contains only the
//! // connection parameters.
//! #[derive(Singleton)]
//! #[singleton(lock = "rwlock")]
//! struct DbConnPool {
//!     addr: SocketAddr,
//!     user: String,
//!     pass: String,
//! }
//!
//! impl DbConnPool {
//!     #[singleton_fn]
//!     async fn addr(&self) -> SocketAddr {
//!         self.addr.clone()
//!     }
//!
//!     #[singleton_fn]
//!     async fn new_connection(&mut self,
//!                             new_addr: SocketAddr,
//!                             new_user: String,
//!                             new_pass: String) -> io::Result<()> {
//!         self.addr = new_addr;
//!         self.user = new_user;
//!         self.pass = new_pass;
//!         Ok(())
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! DbConnPool::init_singleton(DbConnPool {
//!     addr: SocketAddr::from_str("127.0.0.1:8080")?,
//!     user: "tuxthepenguin".to_string(),
//!     pass: "NotASafePassword42".to_string(),
//! });
//!
//! let addr = DbConnPool::addr().await;
//! println!("DbConnPool address: {}.", addr); // 127.0.0.1:8080.
//! # assert_eq!(addr, SocketAddr::from_str("127.0.0.1:8080")?);
//!
//! DbConnPool::new_connection(
//!     SocketAddr::from_str("192.168.42.42:4242")?,
//!     "hello".to_string(),
//!     "world".to_string()
//! ).await.expect("Failed to create a new connection!");
//! println!("DbConnPool address: {}.", DbConnPool::addr().await); // 192.168.42.42:4242.
//! # assert_eq!(DbConnPool::addr().await, SocketAddr::from_str("192.168.42.42:4242")?);
//! # Ok(())
//! # }
//! # }
//! # #[cfg(not(feature = "singleton"))]
//! # fn main() {}
//! ```

use std::future::Future;

/// A singleton.
#[async_trait::async_trait]
pub trait Singleton {
    /// Inner value contained by the singleton.
    type Inner;

    /// Initialize the singleton.
    ///
    /// This can fail if the singleton has already been initialized.
    fn init_singleton(inner: Self::Inner) -> anyhow::Result<()>;

    /// Use the singleton with an immutable reference.
    // F: Function to be run.
    // R: Function result.
    async fn use_singleton<F, R>(clojure: F) -> R
    where
        F: for<'c> SingletonFn<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
    where
        F: for<'c> SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
        R: Send;

    /// Use the singleton with a mutable reference.
    // F: Function to be run.
    // R: Function result.
    async fn use_mut_singleton<F, R>(clojure: F) -> R
    where
        F: for<'c> SingletonFnMut<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result.
    async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
    where
        F: for<'c> SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
        R: Send;
}

/// Trait that defines the behaviour of a function that uses an immutable singleton.
// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function.
pub trait SingletonFn<'i, I, R>
where
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = R> + Send;

    fn call_once(self, inner: &'i I) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function.
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, I, R, F, FR> SingletonFn<'i, I, R> for F
where
    I: 'i,
    F: FnOnce(&'i I) -> FR,
    FR: Future<Output = R> + Send + 'i,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult = FR;

    fn call_once(self, inner: &'i I) -> Self::SingletonResult {
        self(inner)
    }
}

/// Trait that defines the behaviour of a function that uses an immutable singleton and an argument.
// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function.
pub trait SingletonFnWithArg<'i, I, A, R>
where
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = R> + Send;

    fn call_once(self, inner: &'i I, arg: A) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function.
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, A, I, R, F, FR> SingletonFnWithArg<'i, I, A, R> for F
where
    I: 'i,
    F: FnOnce(&'i I, A) -> FR,
    FR: Future<Output = R> + Send + 'i,
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult = FR;

    fn call_once(self, inner: &'i I, arg: A) -> Self::SingletonResult {
        self(inner, arg)
    }
}

/// Trait that defines the behaviour of a function that uses a mutable singleton.
// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function.
pub trait SingletonFnMut<'i, I, R>
where
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = R> + Send;

    fn call_once(self, inner: &'i mut I) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function.
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, I, R, F, FR> SingletonFnMut<'i, I, R> for F
where
    I: 'i,
    F: FnMut(&'i mut I) -> FR,
    FR: Future<Output = R> + Send + 'i,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult = FR;

    fn call_once(mut self, inner: &'i mut I) -> Self::SingletonResult {
        self(inner)
    }
}

/// Trait that defines the behaviour of a function that uses an immutable singleton and an argument.
// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function.
pub trait SingletonFnMutWithArg<'i, I, A, R>
where
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = R> + Send;

    fn call_once(self, inner: &'i mut I, arg: A) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function.
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, A, I, R, F, FR> SingletonFnMutWithArg<'i, I, A, R> for F
where
    I: 'i,
    F: FnOnce(&'i mut I, A) -> FR,
    FR: Future<Output = R> + Send + 'i,
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult = FR;

    fn call_once(self, inner: &'i mut I, arg: A) -> Self::SingletonResult {
        self(inner, arg)
    }
}
