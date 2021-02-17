//! Singleton macro.

mod derive_static;
mod derive_trait;
mod singleton_fns;

use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;
use syn::ItemFn;

/// #[derive(Singleton)]
pub(crate) fn derive_singleton(input: DeriveInput) -> TokenStream {
    // create the singleton type identifier
    let type_name = &input.ident;
    // create the singleton static identifier
    let singleton_name = &derive_static::create_singleton_static_name(type_name);
    // create the singleton static implementation
    let singleton_static = derive_static::impl_singleton_static(type_name, singleton_name);
    // create the singleton trait implementation
    let impl_singleton = derive_trait::impl_singleton_trait(type_name, singleton_name);
    // return the implementation
    quote! {
        #singleton_static
        #impl_singleton
    }
}

/// #[singleton_fn]
pub(crate) fn singleton_fn(function: ItemFn) -> TokenStream {
    // create base function
    let base = &function;
    // create impl fn
    let impl_fn = singleton_fns::impl_singleton_fn(base);
    // create facade fn
    let facade_fn = singleton_fns::impl_singleton_fn_facade(base, &impl_fn);

    quote! {
        #impl_fn
        #facade_fn
    }
}

/// #[singleton_fn_with_arg]
pub(crate) fn singleton_fn_with_arg(function: ItemFn) -> TokenStream {
    quote! {
        #function
    }
}

/// #[singleton_fn_mut]
pub(crate) fn singleton_fn_mut(function: ItemFn) -> TokenStream {
    // create base function
    let base = &function;
    // create impl fn
    let impl_fn = singleton_fns::impl_singleton_fn_mut(base);
    // create facade fn
    let facade_fn = singleton_fns::impl_singleton_fn_mut_facade(base, &impl_fn);

    quote! {
        #impl_fn
        #facade_fn
    }
}

/// #[singleton_fn_mut_with_arg]
pub(crate) fn singleton_fn_mut_with_arg(function: ItemFn) -> TokenStream {
    quote! {
        #function
    }
}
