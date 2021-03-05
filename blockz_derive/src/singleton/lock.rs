//! Singleton lock.

use darling::FromMeta;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use crate::paths;

/// The lock behing a singleton.
#[derive(FromMeta)]
pub(super) enum SingletonLock {
    /// tokio::sync::Mutex
    #[darling(rename = "mutex")]
    Mutex,
    /// tokio::sync::RwLock
    #[darling(rename = "rwlock")]
    RwLock,
}

impl Default for SingletonLock {
    fn default() -> Self {
        Self::Mutex
    }
}

impl SingletonLock {
    /// Create a new lock type with the inner type.
    pub fn to_type(&self, inner: &Ident) -> TokenStream {
        let tokio = paths::tokio_path();
        match self {
            Self::Mutex => quote! { #tokio::sync::Mutex<#inner> },
            Self::RwLock => quote! { #tokio::sync::RwLock<#inner> },
        }
    }

    /// Create a lock initialization expression that sets the inner value to the ident.
    pub fn to_new_lock_expr(&self, inner: &Ident) -> TokenStream {
        let tokio = paths::tokio_path();
        match self {
            Self::Mutex => quote! { #tokio::sync::Mutex::new(#inner) },
            Self::RwLock => quote! { #tokio::sync::RwLock::new(#inner) },
        }
    }

    /// Create a new lock guard from a lock identifier.
    pub fn to_guard(&self, lock: &Ident) -> TokenStream {
        match self {
            Self::Mutex => quote! { #lock.lock().await },
            Self::RwLock => quote! { #lock.read().await },
        }
    }

    /// Create a new mutable lock guard from a lock identifier.
    pub fn to_mut_guard(&self, lock: &Ident) -> TokenStream {
        match self {
            Self::Mutex => quote! { #lock.lock().await },
            Self::RwLock => quote! { #lock.write().await },
        }
    }
}
