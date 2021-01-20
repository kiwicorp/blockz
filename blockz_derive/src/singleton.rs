//! Singleton macro.

use proc_macro2::Ident;

/// Implement the singleton static.
pub fn impl_singleton_static(
    type_name: &Ident,
    singleton_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        /// Singleton for #type_name.
        static #singleton_name: once_cell::sync::OnceCell<tokio::sync::Mutex<#type_name>> =
            once_cell::sync::OnceCell::new();
    }
}

/// Implement the singleton trait.
pub fn impl_singleton_trait(type_name: &Ident, singleton_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #[async_trait::async_trait]
        impl blockz::Singleton for #type_name {
            type Inner = #type_name;

            /// Initialize the singleton for #type_name.
            fn init_singleton(inner: Self::Inner) -> anyhow::Result<()> {
                if #singleton_name.set(tokio::sync::Mutex::new(inner)).is_err() {
                    Err(anyhow::anyhow!("#type_name: singleton: already initialized"))
                } else {
                    Ok(())
                }
            }

            /// Run an async function using an immutable #type_name.
            async fn use_singleton<F, R>(clojure: F) -> anyhow::Result<R>
            where
                F: for<'c> blockz::SingletonFn<'c, #type_name, R> + Send,
                R: Send,
            {
                let inner = #singleton_name.get().unwrap().lock().await;
                let inner_deref: &#type_name = &*inner;
                clojure.call_once(inner_deref).await
            }

            /// Run an async function using a mutable #type_name.
            async fn use_mut_singleton<F, R>(clojure: F) -> anyhow::Result<R>
            where
                F: for<'c> blockz::SingletonFnMut<'c, Self::Inner, R> + Send,
                R: Send,
            {
                let mut inner = #singleton_name.get().unwrap().lock().await;
                let inner_deref: &mut #type_name = &mut *inner;
                clojure.call_once(inner_deref).await
            }
        }
    }
}
