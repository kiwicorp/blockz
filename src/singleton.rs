//! Singletons.

/// A singleton.
#[async_trait::async_trait]
pub trait Singleton {
    /// Inner value contained by the singleton.
    type Inner;

    /// Initialize the singleton.
    fn init_singleton(inner: Self::Inner) -> anyhow::Result<()>;

    /// Use the singleton with an immutable reference.
    async fn use_singleton<F, R>(clojure: F) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFn<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with a mutable reference.
    async fn use_mut_singleton<F, R>(clojure: F) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnMut<'c, Self::Inner, R> + Send,
        R: Send;
}

/// Trait that defines the behaviour of a function that uses an immutable singleton.
pub trait SingletonFn<'s, S, R>
where
    R: Send,
{
    type Res: std::future::Future<Output = anyhow::Result<R>> + Send;
    fn call_once(self, inner: &'s S) -> Self::Res;
}

impl<'s, S, R, F, FR> SingletonFn<'s, S, R> for F
where
    S: 's,
    F: FnOnce(&'s S) -> FR,
    FR: std::future::Future<Output = anyhow::Result<R>> + Send + 's,
    R: Send,
{
    type Res = FR;
    fn call_once(self, inner: &'s S) -> Self::Res {
        self(inner)
    }
}

/// Trait that defines the behaviour of a function that uses a mutable singleton.
pub trait SingletonFnMut<'s, S, R>
where
    R: Send,
{
    type Res: std::future::Future<Output = anyhow::Result<R>> + Send;
    fn call_once(self, inner: &'s mut S) -> Self::Res;
}

impl<'s, S, R, F, FR> SingletonFnMut<'s, S, R> for F
where
    S: 's,
    F: FnMut(&'s mut S) -> FR,
    FR: std::future::Future<Output = anyhow::Result<R>> + Send + 's,
    R: Send,
{
    type Res = FR;
    fn call_once(mut self, inner: &'s mut S) -> Self::Res {
        self(inner)
    }
}
