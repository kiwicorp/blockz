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

    /// Use the singleton with an immutable reference and an argument.
    async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
        R: Send;

    /// Use the singleton with a mutable reference.
    async fn use_mut_singleton<F, R>(clojure: F) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnMut<'c, Self::Inner, R> + Send,
        R: Send;

    /// Use the singleton with an immutable reference and an argument.
    async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> anyhow::Result<R>
    where
        F: for<'c> SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
        A: Send,
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

/// Trait that defines the behaviour of a function that uses an immutable singleton and an argument.
pub trait SingletonFnWithArg<'s, S, A, R>
where
    A: Send,
    R: Send,
{
    type Res: std::future::Future<Output = anyhow::Result<R>> + Send;
    fn call_once(self, inner: &'s S, arg: A) -> Self::Res;
}

impl<'s, A, S, R, F, FR> SingletonFnWithArg<'s, S, A, R> for F
where
    S: 's,
    F: FnOnce(&'s S, A) -> FR,
    FR: std::future::Future<Output = anyhow::Result<R>> + Send + 's,
    A: Send,
    R: Send,
{
    type Res = FR;
    fn call_once(self, inner: &'s S, arg: A) -> Self::Res {
        self(inner, arg)
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

/// Trait that defines the behaviour of a function that uses an immutable singleton and an argument.
pub trait SingletonFnMutWithArg<'s, S, A, R>
where
    A: Send,
    R: Send,
{
    type Res: std::future::Future<Output = anyhow::Result<R>> + Send;
    fn call_once(self, inner: &'s mut S, arg: A) -> Self::Res;
}

impl<'s, A, S, R, F, FR> SingletonFnMutWithArg<'s, S, A, R> for F
where
    S: 's,
    F: FnOnce(&'s mut S, A) -> FR,
    FR: std::future::Future<Output = anyhow::Result<R>> + Send + 's,
    A: Send,
    R: Send,
{
    type Res = FR;
    fn call_once(self, inner: &'s mut S, arg: A) -> Self::Res {
        self(inner, arg)
    }
}
