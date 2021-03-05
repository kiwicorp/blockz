//! Implement the singleton static.

use proc_macro2::TokenStream;

use quote::format_ident;
use quote::quote;

use syn::Ident;

use crate::common;
use crate::paths;

use super::lock::SingletonLock;

/// Factory that builds a singleton trait implementation.
pub(super) struct SingletonTraitFactory<'f> {
    static_ident: &'f Ident,
    type_name: &'f Ident,
    lock: &'f SingletonLock,
}

impl<'f> SingletonTraitFactory<'f> {
    /// Create a new singleton trait factory.
    pub fn new(static_ident: &'f Ident, type_name: &'f Ident, lock: &'f SingletonLock) -> Self {
        Self {
            static_ident,
            type_name,
            lock,
        }
    }

    /// Implement Singleton::init_singleton.
    fn impl_init_singleton(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let anyhow = paths::anyhow_path();

        // create lit str error message
        let err_msg =
            common::create_lit_str(format!("{}: singleton: already initialized", type_name));

        // create doc comment
        let doc = common::create_doc(format!("Initialize the singleton for {}.", type_name));

        // create ident for the inner value var name
        let inner_ident = format_ident!("inner");

        // create new lock expression
        let new_lock_expr = self.lock.to_new_lock_expr(&inner_ident);

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            fn init_singleton(#inner_ident: Self::Inner) -> #anyhow::Result<()> {
                if #static_ident.set(#new_lock_expr).is_err() {
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
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using an immutable {}.",
            type_name
        ));

        // create lock ident
        let lock_ident = format_ident!("inner_lock");

        // get lock guard expression
        let lock_guard_expr = self.lock.to_guard(&lock_ident);

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_singleton<F, R>(clojure: F) -> R
            where
                F: for<'c> #blockz::singleton::SingletonFn<'c, #type_name, R> + Send,
                R: Send,
            {
                let #lock_ident = #static_ident.get().unwrap();
                let inner_guard = #lock_guard_expr;
                let inner_deref: &#type_name = &*inner_guard;
                clojure.call_once(inner_deref).await
            }
        }
    }

    /// Implement Singleton::use_mut_singleton.
    fn impl_use_mut_singleton(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using a mutable {}.",
            type_name
        ));

        // create lock ident
        let lock_ident = format_ident!("inner_lock");

        // get mut lock guard expression
        let mut_lock_guard_expr = self.lock.to_mut_guard(&lock_ident);

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_mut_singleton<F, R>(clojure: F) -> R
            where
                F: for<'c> #blockz::singleton::SingletonFnMut<'c, Self::Inner, R> + Send,
                R: Send,
            {
                let #lock_ident = #static_ident.get().unwrap();
                let mut inner_guard = #mut_lock_guard_expr;
                let inner_deref: &mut #type_name = &mut *inner_guard;
                clojure.call_once(inner_deref).await
            }
        }
    }

    /// Implement Singleton::use_singleton_with_arg.
    fn impl_use_singleton_with_arg(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using an immutable {} and an argument.",
            type_name
        ));

        // create lock ident
        let lock_ident = format_ident!("inner_lock");

        // get lock guard expression
        let lock_guard_expr = self.lock.to_guard(&lock_ident);

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
            where
                F: for<'c> #blockz::singleton::SingletonFnWithArg<'c, Self::Inner, A, R> + Send,
                A: Send,
                R: Send
            {
                let #lock_ident = #static_ident.get().unwrap();
                let inner_guard = #lock_guard_expr;
                let inner_deref: &#type_name = &*inner_guard;
                clojure.call_once(inner_deref, arg).await
            }
        }
    }

    /// Implement Singleton::use_singleton_mut_with_arg.
    fn impl_use_mut_singleton_with_arg(&self) -> TokenStream {
        let static_ident = self.static_ident;
        let type_name = self.type_name;

        // get paths to deps
        let blockz = paths::blockz_path();

        // create doc comment
        let doc = common::create_doc(format!(
            "Run an async function using a mutable {} and an argument.",
            type_name
        ));

        // create lock ident
        let lock_ident = format_ident!("inner_lock");

        // get mut lock guard expression
        let mut_lock_guard_expr = self.lock.to_mut_guard(&lock_ident);

        // return implementation
        quote! {
            #doc
            #[automatically_derived]
            async fn use_mut_singleton_with_arg<F, A, R>(clojure: F, arg: A) -> R
            where
                F: for<'c> #blockz::singleton::SingletonFnMutWithArg<'c, Self::Inner, A, R> + Send,
                A: Send,
                R: Send
            {
                let #lock_ident = #static_ident.get().unwrap();
                let mut inner_guard = #mut_lock_guard_expr;
                let inner_deref: &mut #type_name = &mut *inner_guard;
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
