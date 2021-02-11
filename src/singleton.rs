//! Singletons.

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
    // R: Function result (to be wrapped by anyhow::Result).
    async fn use_singleton<F, R>(clojure: F) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFn<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result (to be wrapped by anyhow::Result).
    async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
        R: Send;

    /// Use the singleton with a mutable reference.
    // F: Function to be run.
    // R: Function result (to be wrapped by anyhow::Result).
    async fn use_mut_singleton<F, R>(clojure: F) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnMut<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with an immutable reference and an argument.
    // F: Function to be run.
    // A: Function argument.
    // R: Function result (to be wrapped by anyhow::Result).
    async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
        R: Send;
}

/// Trait that defines the behaviour of a function that uses an immutable singleton.
// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function (to be wrapped by anyhow::Result).
pub trait SingletonFn<'i, I, R>
where
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = anyhow::Result<R>> + Send;

    fn call_once(self, inner: &'i I) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function (to be wrapped by anyhow::Result).
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, I, R, F, FR> SingletonFn<'i, I, R> for F
where
    I: 'i,
    F: FnOnce(&'i I) -> FR,
    FR: Future<Output = anyhow::Result<R>> + Send + 'i,
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
// R: the result of the function (to be wrapped by anyhow::Result).
pub trait SingletonFnWithArg<'i, I, A, R>
where
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = anyhow::Result<R>> + Send;

    fn call_once(self, inner: &'i I, arg: A) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function (to be wrapped by anyhow::Result).
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, A, I, R, F, FR> SingletonFnWithArg<'i, I, A, R> for F
where
    I: 'i,
    F: FnOnce(&'i I, A) -> FR,
    FR: Future<Output = anyhow::Result<R>> + Send + 'i,
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
// R: the result of the function (to be wrapped by anyhow::Result).
pub trait SingletonFnMut<'i, I, R>
where
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = anyhow::Result<R>> + Send;

    fn call_once(self, inner: &'i mut I) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// R: the result of the function (to be wrapped by anyhow::Result).
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, I, R, F, FR> SingletonFnMut<'i, I, R> for F
where
    I: 'i,
    F: FnMut(&'i mut I) -> FR,
    FR: Future<Output = anyhow::Result<R>> + Send + 'i,
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
// R: the result of the function (to be wrapped by anyhow::Result).
pub trait SingletonFnMutWithArg<'i, I, A, R>
where
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult: Future<Output = anyhow::Result<R>> + Send;

    fn call_once(self, inner: &'i mut I, arg: A) -> Self::SingletonResult;
}

// 'i: the lifetime of the inner value of the singleton.
// I: the inner value of the singleton.
// A: the argument to be consumed by the function.
// R: the result of the function (to be wrapped by anyhow::Result).
// F: the function to be executed.
// FR: the future produced by the function F.
impl<'i, A, I, R, F, FR> SingletonFnMutWithArg<'i, I, A, R> for F
where
    I: 'i,
    F: FnOnce(&'i mut I, A) -> FR,
    FR: Future<Output = anyhow::Result<R>> + Send + 'i,
    A: Send,
    R: Send,
{
    /// The result of a singleton function (a Future).
    type SingletonResult = FR;

    fn call_once(self, inner: &'i mut I, arg: A) -> Self::SingletonResult {
        self(inner, arg)
    }
}
