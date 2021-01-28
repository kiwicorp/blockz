//! Singleton macro.

use crate::paths;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

/// Implement the singleton static.
pub fn impl_singleton_static(
    type_name: &Ident,
    singleton_name: &Ident,
) -> proc_macro2::TokenStream {
    let once_cell = paths::once_cell_path();
    let tokio = paths::tokio_path();
    quote! {
        /// Singleton for #type_name.
        static #singleton_name: #once_cell::sync::OnceCell<#tokio::sync::Mutex<#type_name>> =
            #once_cell::sync::OnceCell::new();
    }
}

#[cfg(feature = "singleton_generics")]
fn impl_use_singleton(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    quote! {
        /// Run an async function using an immutable #type_name.
        async fn use_singleton<F, R>(clojure: F) -> #anyhow::Result<R>
        where
            F: for<'c> #blockz::singleton::SingletonFn<'c, #type_name, R> + Send,
            R: Send,
        {
            let inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &#type_name = &*inner;
            clojure.call_once(inner_deref).await
        }
    }
}

#[cfg(feature = "singleton_generics")]
fn impl_use_singleton_with_arg(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    quote! {
        /// Use the singleton with an immutable reference and an argument.
        async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> #anyhow::Result<R>
        where
            F: for<'c> #blockz::singleton::SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
            A: Send,
            R: Send
        {
            let inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &#type_name = &*inner;
            clojure.call_once(inner_deref, arg).await
        }
    }
}

#[cfg(feature = "singleton_generics")]
fn impl_use_singleton_mut(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    quote! {
        /// Run an async function using a mutable #type_name.
        async fn use_mut_singleton<F, R>(clojure: F) -> #anyhow::Result<R>
        where
            F: for<'c> #blockz::singleton::SingletonFnMut<'c, Self::Inner, R> + Send,
            R: Send,
        {
            let mut inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &mut #type_name = &mut *inner;
            clojure.call_once(inner_deref).await
        }
    }
}

#[cfg(feature = "singleton_generics")]
fn impl_use_singleton_mut_with_arg(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    quote! {
        /// Use the singleton with an immutable reference and an argument.
        async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> #anyhow::Result<R>
        where
            F: for<'c> #blockz::singleton::SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
            A: Send,
            R: Send
        {
            let mut inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &mut #type_name = &mut *inner;
            clojure.call_once(inner_deref, arg).await
        }
    }
}

#[cfg(feature = "singleton_boxes")]
fn impl_use_singleton(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let std = paths::std_path();
    quote! {
        /// Run an async function using an immutable #type_name.
        async fn use_singleton<R>(
            clojure: Box<
                dyn for<'c> #blockz::singleton::SingletonFn<
                        SingletonResult = Box<dyn #std::future::Future<Output = #anyhow::Result<R>>>,
                        'c,
                        Self::Inner,
                        R,
                    > + Send
            >,
        ) -> #anyhow::Result<R>
        where
            R: Send,
        {
            let inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &#type_name = &*inner;
            clojure.call_once(inner_deref).await
        }
    }
}

#[cfg(feature = "singleton_boxes")]
fn impl_use_singleton_with_arg(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let std = paths::std_path();
    quote! {
        /// Use the singleton with an immutable reference and an argument.
        async fn use_singleton_with_arg<A, R>(
            clojure: Box<
                dyn for<'c> #blockz::singleton::SingletonFnWithArg<
                        SingletonResult = Box<dyn #std::future::Future<Output = #anyhow::Result<R>>>,
                        'c,
                        Self::Inner,
                        A,
                        R,
                    > + Send,
            >,
            arg: A,
        ) -> #anyhow::Result<R>
        where
            A: Send,
            R: Send
        {
            let inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &#type_name = &*inner;
            clojure.call_once(inner_deref, arg).await
        }
    }
}

#[cfg(feature = "singleton_boxes")]
fn impl_use_singleton_mut(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let std = paths::std_path();
    quote! {
        /// Run an async function using a mutable #type_name.
        async fn use_mut_singleton<R>(
            clojure: Box<
                dyn for<'c> #blockz::singleton::SingletonFnMut<
                        SingletonResult = Box<dyn #std::future::Future<Output = #anyhow::Result<R>>>,
                        'c,
                        Self::Inner,
                        R,
                    > + Send,
            >,
        ) -> #anyhow::Result<R>
        where
            R: Send,
        {
            let mut inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &mut #type_name = &mut *inner;
            clojure.call_once(inner_deref).await
        }

    }
}

#[cfg(feature = "singleton_boxes")]
fn impl_use_singleton_mut_with_arg(singleton_name: &Ident, type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let std = paths::std_path();
    quote! {
        /// Use the singleton with an immutable reference and an argument.
        async fn use_mut_singleton_with_arg<A, R>(
            clojure: Box<
                dyn for<'c> #blockz::singleton::SingletonFnMutWithArg<
                        SingletonResult = Box<dyn #std::future::Future<Output = #anyhow::Result<R>>>,
                        'c,
                        Self::Inner,
                        A,
                        R,
                    > + Send,
            >,
            arg: A
        ) -> #anyhow::Result<R>
        where
            A: Send,
            R: Send
        {
            let mut inner = #singleton_name.get().unwrap().lock().await;
            let inner_deref: &mut #type_name = &mut *inner;
            clojure.call_once(inner_deref, arg).await
        }
    }
}

/// Implement the singleton trait.
pub fn impl_singleton_trait(type_name: &Ident, singleton_name: &Ident) -> proc_macro2::TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let tokio = paths::tokio_path();

    let use_singleton = impl_use_singleton(singleton_name, type_name);
    let use_singleton_mut = impl_use_singleton_mut(singleton_name, type_name);
    let use_singleton_with_arg = impl_use_singleton_with_arg(singleton_name, type_name);
    let use_singleton_mut_with_arg = impl_use_singleton_mut_with_arg(singleton_name, type_name);

    quote! {
        #[async_trait::async_trait]
        impl #blockz::singleton::Singleton for #type_name {
            type Inner = #type_name;

            /// Initialize the singleton for #type_name.
            fn init_singleton(inner: Self::Inner) -> #anyhow::Result<()> {
                if #singleton_name.set(#tokio::sync::Mutex::new(inner)).is_err() {
                    Err(anyhow::anyhow!("#type_name: singleton: already initialized"))
                } else {
                    Ok(())
                }
            }

            #use_singleton

            #use_singleton_mut

            #use_singleton_with_arg

            #use_singleton_mut_with_arg
        }
    }
}
