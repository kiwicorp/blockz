//! Implement the singleton static.

use crate::common;
use crate::paths;

use proc_macro2::TokenStream;

use quote::quote;

use syn::Ident;

/// Factory that builds a singleton trait implementation.
pub(super) struct SingletonTraitFactory<'f> {
    static_ident: &'f Ident,
    type_name: &'f Ident,
}

impl<'f> SingletonTraitFactory<'f> {
    /// Create a new singleton trait factory.
    pub fn new(static_ident: &'f Ident, type_name: &'f Ident) -> Self {
        Self {
            static_ident,
            type_name,
        }
    }

    /// Implement Singleton::init_singleton.
    fn impl_init_singleton(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();
        let tokio = paths::tokio_path();

        // create lit str error message
        let err_msg =
            common::create_lit_str(format!("{}: singleton: already initialized", type_name));

        // create doc comment
        let doc = common::create_doc(format!("Initialize the singleton for {}.", type_name));

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            fn init_singleton(inner: Self::Inner) -> #anyhow::Result<()> {
                if #static_ident.set(#tokio::sync::Mutex::new(inner)).is_err() {
                    Err(anyhow::anyhow!(#err_msg))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Implement Singleton::use_singleton.
    fn impl_use_singleton(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using an immutable {}.",
            type_name
        ));

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_singleton<F, R>(clojure: F) -> #anyhow::Result<R>
            where
                F: for<'c> #blockz::singleton::SingletonFn<'c, #type_name, R> + Send,
                R: Send,
            {
                let inner = #static_ident.get().unwrap().lock().await;
                let inner_deref: &#type_name = &*inner;
                clojure.call_once(inner_deref).await
            }
        }
    }

    /// Implement Singleton::use_mut_singleton.
    fn impl_use_mut_singleton(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using a mutable {}.",
            type_name
        ));

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_mut_singleton<F, R>(clojure: F) -> #anyhow::Result<R>
            where
                F: for<'c> #blockz::singleton::SingletonFnMut<'c, Self::Inner, R> + Send,
                R: Send,
            {
                let mut inner = #static_ident.get().unwrap().lock().await;
                let inner_deref: &mut #type_name = &mut *inner;
                clojure.call_once(inner_deref).await
            }
        }
    }

    /// Implement Singleton::use_singleton_with_arg.
    fn impl_use_singleton_with_arg(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using an immutable {} and an argument.",
            type_name
        ));

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> #anyhow::Result<R>
            where
                F: for<'c> #blockz::singleton::SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
                A: Send,
                R: Send
            {
                let inner = #static_ident.get().unwrap().lock().await;
                let inner_deref: &#type_name = &*inner;
                clojure.call_once(inner_deref, arg).await
            }
        }
    }

    /// Implement Singleton::use_singleton_mut_with_arg.
    fn impl_use_mut_singleton_with_arg(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using a mutable {} and an argument.",
            type_name
        ));

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> #anyhow::Result<R>
            where
                F: for<'c> #blockz::singleton::SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
                A: Send,
                R: Send
            {
                let mut inner = #static_ident.get().unwrap().lock().await;
                let inner_deref: &mut #type_name = &mut *inner;
                clojure.call_once(inner_deref, arg).await
            }
        }
    }

    /// Build the trait implementation.
    pub fn build(self) -> syn::Result<TokenStream> {
        // get paths to deps
        let blockz = paths::blockz_path();

        // create implementations
        let init_singleton = self.impl_init_singleton();
        let use_singleton = self.impl_use_singleton();
        let use_singleton_mut = self.impl_use_mut_singleton();
        let use_singleton_with_arg = self.impl_use_singleton_with_arg();
        let use_singleton_mut_with_arg = self.impl_use_mut_singleton_with_arg();

        // return implementation
        let type_name = self.type_name;
        Ok(quote! {
            #[async_trait::async_trait]
            impl #blockz::singleton::Singleton for #type_name {
                type Inner = #type_name;
                #init_singleton
                #use_singleton
                #use_singleton_mut
                #use_singleton_with_arg
                #use_singleton_mut_with_arg
            }
        })
    }
}
