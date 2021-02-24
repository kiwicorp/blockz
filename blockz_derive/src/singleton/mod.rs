//! Singleton macro.

mod derive_static;
mod derive_trait;
mod facade_fn;
mod impl_fn;
mod singleton_fns;

use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;
use syn::ItemFn;

use std::convert::TryFrom;

use self::facade_fn::FacadeFnFactory;
use self::impl_fn::ImplFnFactory;
use self::singleton_fns::SingletonFnType;

/// A factory that builds singleton fns.
pub(crate) struct SingletonFnFactory<'f> {
    // the base function
    base: &'f ItemFn,
    // the function type that will be built by the factory
    fn_type: SingletonFnType<'f>,
}

impl<'f> SingletonFnFactory<'f> {
    /// Create a new singleton fn factory.
    pub fn new(base: &'f ItemFn) -> syn::Result<Self> {
        Ok(Self {
            base,
            fn_type: SingletonFnType::try_from(base)?,
        })
    }

    /// Build the singleton fn facade and impl.
    pub fn build(&self) -> syn::Result<TokenStream> {
        let impl_fn = ImplFnFactory::new(self.base, &self.fn_type).build()?;
        let facade_fn = FacadeFnFactory::new(self.base, &self.fn_type, &impl_fn).build()?;
        Ok(quote! {
            #facade_fn
            #impl_fn
        })
    }
}

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
