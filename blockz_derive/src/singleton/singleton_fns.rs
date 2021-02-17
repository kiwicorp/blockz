//! Singleton fns utilities.

use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::Block;
use syn::FnArg;
use syn::Ident;
use syn::ItemFn;
use syn::Visibility;

/// Prefix for an impl fn used by a singleton fn.
const SINGLETON_FN_PREFIX: &str = "blockz_singleton_fn_";
/// Prefix for an impl fn used by a singleton fn with arg.
const SINGLETON_FN_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_with_arg_";
/// Prefix for an impl fn used by a singleton mut fn.
const SINGLETON_FN_MUT_PREFIX: &str = "blockz_singleton_fn_mut_";
/// Prefix for an impl fn used by a singleton mut fn with arg.
const SINGLETON_FN_MUT_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_mut_with_arg_";

/// Implement a SingletonFn.
pub(super) fn impl_singleton_fn(base: &ItemFn) -> ItemFn {
    // clone the base
    let mut impl_fn = base.clone();
    // make the impl fn private
    impl_fn.vis = Visibility::Inherited;
    // rename the function
    rename_fn(
        &mut impl_fn,
        format!("{}{}", SINGLETON_FN_PREFIX, base.sig.ident.to_string()),
    );
    // return the impl fn
    impl_fn
}

/// Implement the facade for a SingletonFn.
pub(super) fn impl_singleton_fn_facade(base: &ItemFn, impl_fn: &ItemFn) -> ItemFn {
    // clone the base
    let mut facade_fn = base.clone();
    // remove the receiver
    remove_fn_receiver(&mut facade_fn);
    // get impl fn ident
    let impl_fn_ident = &impl_fn.sig.ident;
    // replace the function impl
    replace_fn_block(
        &mut facade_fn,
        quote! {
            #[allow(unused_imports)]
            use blockz::singleton::Singleton;
            Self::use_singleton(Self::#impl_fn_ident).await
        },
    );
    // return the facade fn
    facade_fn
}

/// Remove the receiver from a function.
fn remove_fn_receiver(function: &mut ItemFn) {
    if let FnArg::Typed(recv) = function
        .sig
        .inputs
        .first()
        .expect(format!("Function {} should have had a receiver", function.sig.ident).as_str())
    {
        panic!(
            "Function {} must have either a &self or &mut self receiver. Found receiver: {:?}.",
            function.sig.ident, recv.ty
        );
    }
    function.sig.inputs = function
        .sig
        .inputs
        .iter()
        .cloned()
        .filter(|arg| {
            if let FnArg::Receiver(_) = arg {
                false
            } else {
                true
            }
        })
        .collect();
}

/// Rename a function.
fn rename_fn(function: &mut ItemFn, name: String) {
    function.sig.ident = Ident::new(name.as_str(), Span::call_site());
}

/// Replace the block of a function.
///
/// The token stream must not include the braces.
fn replace_fn_block(function: &mut ItemFn, block: TokenStream) {
    let block: Block = syn::parse2(quote! {
        {
            #block
        }
    })
    .expect(format!("Failed to parse replacement block: {}", block).as_str());
    function.block = Box::new(block);
}
