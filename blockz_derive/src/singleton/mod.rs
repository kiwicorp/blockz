//! Singleton macro.

use crate::paths;

use convert_case::Case;
use convert_case::Casing;

use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;
use syn::ItemFn;

thread_local! {
    /// Map for getting singleton identifiers for constructing singleton fns
    /// without having to inject any useless code into the generated code.
    static SINGLETON_IDENTS: RefCell<HashMap<String, Ident>> = RefCell::new(HashMap::new());
}

pub(crate) fn derive_singleton(input: DeriveInput) -> TokenStream {
    // create an identifier string for the singleton static
    let upper_snake = {
        let original = format!("_BLOCKZ_SINGLETON_{}", &input.ident);
        original.to_case(Case::UpperSnake)
    };

    // create the singleton and type identifiers
    let singleton_name = Ident::new(upper_snake.as_str(), Span::call_site());
    let type_name = input.ident;

    // create the singleton trait and static implementations
    let impl_singleton = impl_singleton_trait(&type_name, &singleton_name);
    let singleton_static = impl_singleton_static(&type_name, &singleton_name);

    // add the name of the singleton static to the map
    // these can be used later for singleton fns
    SINGLETON_IDENTS.with(move |idents| {
        let mut idents = idents.borrow_mut();
        let type_name = type_name.to_string();
        if idents.contains_key(type_name.as_str()) {
            panic!(
                "Singleton already implemented for {}: found existing singleton ident.",
                type_name.as_str()
            );
        }
        idents.insert(type_name, singleton_name);
    });

    // return the implementation
    quote! {
        #singleton_static
        #impl_singleton
    }
}

pub(crate) fn singleton_fn(function: ItemFn) -> TokenStream {
    quote! {
        #function
    }
}

/// Implement the singleton static.
fn impl_singleton_static(type_name: &Ident, singleton_name: &Ident) -> proc_macro2::TokenStream {
    let once_cell = paths::once_cell_path();
    let tokio = paths::tokio_path();
    quote! {
        /// Singleton for #type_name.
        static #singleton_name: #once_cell::sync::OnceCell<#tokio::sync::Mutex<#type_name>> =
            #once_cell::sync::OnceCell::new();
    }
}

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

/// Implement the singleton trait.
fn impl_singleton_trait(type_name: &Ident, singleton_name: &Ident) -> proc_macro2::TokenStream {
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
