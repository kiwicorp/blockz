//! Singleton fns utilities.

use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Block;
use syn::FnArg;
use syn::Ident;
use syn::ItemFn;
use syn::Pat;
use syn::PatType;
use syn::Stmt;
use syn::Type;
use syn::TypeTuple;
use syn::Visibility;

/// Prefix for an impl fn used by a singleton fn.
const SINGLETON_FN_PREFIX: &str = "blockz_singleton_fn_";
/// Prefix for an impl fn used by a singleton fn with arg.
const SINGLETON_FN_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_with_arg_";
/// Prefix for an impl fn used by a singleton mut fn.
const SINGLETON_FN_MUT_PREFIX: &str = "blockz_singleton_fn_mut_";
/// Prefix for an impl fn used by a singleton mut fn with arg.
const SINGLETON_FN_MUT_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_mut_with_arg_";

/// Simplified type for function inputs (arguments).
type FnInputs = Punctuated<FnArg, syn::token::Comma>;

/// A factory that builds singleton fns.
struct SingletonFnFactory<'f> {
    // the base function
    base: &'f ItemFn,
    // the function type that will be built by the factory
    fn_type: SingletonFn,
}

impl<'f> SingletonFnFactory<'f> {
    /// Create a new singleton fn factory.
    pub fn new(base: &'f ItemFn) -> Self {
        Self {
            base,
            fn_type: SingletonFn::from(base),
        }
    }

    /// Build the singleton fn facade and impl.
    pub fn build(&self) -> TokenStream {
        todo!("write implementation");
        quote! {}
    }
}

/// SingletonFn type.
enum SingletonFn {
    NonMut,
    NonMutWithArg(SingletonFnArg),
    Mut,
    MutWithArg(SingletonFnArg),
}

/// Argument for a SingletonFn.
enum SingletonFnArg {
    Simple,
    Tuple {
        ident: Ident,
        arg_type: TypeTuple,
        replace_legend: Vec<(String, String)>,
    },
}

impl From<&FnInputs> for SingletonFnArg {
    fn from(src: &FnInputs) -> Self {
        if src.len() == 0 {
            // panic if there is an attempt to build a singleton fn arg from 0
            // fn inputs
            panic!("singleton fn arg: attempted to construct from a function that has no inputs");
        } else if src.len() == 1 {
            // if the function has just one arg, the impl fn does not have to
            // have it's block "fixed"
            Self::Simple
        } else {
            // create the ident for the args tuple
            let ident = Ident::new("args", Span::call_site());
            // collect the fn args types
            let fn_args = src
                .iter()
                .filter(|arg| {
                    if let FnArg::Receiver(_) = arg {
                        false
                    } else {
                        true
                    }
                })
                .map(|arg| {
                    if let FnArg::Typed(val) = arg {
                        val
                    } else {
                        panic!("singleton fn arg: attempted to use receiver as tuple arg type")
                    }
                })
                .collect::<Vec<&PatType>>();
            // create the tuple arg type
            let arg_type = {
                let elems = fn_args
                    .iter()
                    .map(|arg| *arg.ty.clone())
                    .collect::<Punctuated<Type, syn::token::Comma>>();
                TypeTuple {
                    paren_token: syn::token::Paren {
                        span: Span::call_site(),
                    },
                    elems,
                }
            };
            // create the replacement legend for fixing the impl fn block
            let replace_legend = fn_args
                .iter()
                .enumerate()
                .map(|(index, arg)| {
                    let arg_pat = &arg.pat;
                    (
                        // the name of the argument
                        format!("{}", quote! {#arg_pat}),
                        // the tuple element replacement
                        format!("{} . {}", quote! {#ident}, index),
                    )
                })
                .collect::<Vec<(String, String)>>();
            Self::Tuple {
                ident,
                arg_type,
                replace_legend,
            }
        }
    }
}

impl<'f> From<&'f ItemFn> for SingletonFn {
    fn from(base: &'f ItemFn) -> Self {
        // panic if the function has no inputs
        if base.sig.inputs.len() == 0 {
            panic!(
                "{} {} {}",
                "singleton fn: from item fn: attempted to construct",
                "singleton fn from fn that has no inputs (must have at",
                "least a receiver)",
            );
        }
        // get the receiver
        // panic if the receiver is not either self, &self or &mut self
        let receiver = if let FnArg::Receiver(val) = base
            .sig
            .inputs
            .first()
            .expect("singleton fn: from item fn: attempted to construct a singleton fn from an fn that has no inputs")
        {
            val
        } else {
            panic!("singleton fn: from item fn: attempted to construct a singleton fn from a fn that does not have a receiver");
        };
        // panic if the receiver is not a reference
        if receiver.reference.is_none() {
            panic!("singleton fn: from item fn: attempted to construct a singleton fn from a fn whose receiver is not a reference")
        }
        // check whether the function has other args or not
        let has_args = base.sig.inputs.len() > 1;
        // return the singleton fn type
        if receiver.mutability.is_none() {
            if !has_args {
                Self::NonMut
            } else {
                Self::NonMutWithArg(SingletonFnArg::from(&base.sig.inputs))
            }
        } else {
            if !has_args {
                Self::Mut
            } else {
                Self::MutWithArg(SingletonFnArg::from(&base.sig.inputs))
            }
        }
    }
}

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

/// Implement a SingletonFnMut.
pub(super) fn impl_singleton_fn_mut(base: &ItemFn) -> ItemFn {
    // clone the base
    let mut impl_fn = base.clone();
    // make the impl fn private
    impl_fn.vis = Visibility::Inherited;
    // rename the function
    rename_fn(
        &mut impl_fn,
        format!("{}{}", SINGLETON_FN_MUT_PREFIX, base.sig.ident.to_string()),
    );
    // return the impl fn
    impl_fn
}

/// Implement a SingletonFnWithArg.
pub(super) fn impl_singleton_fn_with_arg(base: &ItemFn) -> ItemFn {
    // clone the base
    let mut impl_fn = base.clone();
    // make the impl fn private
    impl_fn.vis = Visibility::Inherited;
    // rename the function
    rename_fn(
        &mut impl_fn,
        format!(
            "{}{}",
            SINGLETON_FN_WITH_ARG_PREFIX,
            base.sig.ident.to_string()
        ),
    );
    // get the tuple type that represents the args
    let arg_type = fn_inputs_to_type_tuple(
        base.sig
            .inputs
            .iter()
            .filter(|arg| {
                if let FnArg::Receiver(_) = arg {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<&FnArg>>()
            .as_slice(),
    );
    // // create the args pattern
    // let arg_pat: Pat = syn::parse2(quote! {args: #arg_type}).expect(
    //     format!(
    //         "Failed to parse singleton fn args pattern: {}",
    //         quote! {args: #arg_type}
    //     )
    //     .as_str(),
    // );
    // let arg_pat = if let Pat::Type(val) = arg_pat {
    //     val
    // } else {
    //     panic!("Failed to parse singleton fn type pattern!")
    // };
    // fix the fn inputs
    fn_fix_inputs(&mut impl_fn, &arg_type);
    // fix the fn block
    impl_fn.block = Box::new(fn_fix_block(
        *base.block.clone(),
        base.sig
            .inputs
            .iter()
            .filter(|arg| {
                if let FnArg::Receiver(_) = arg {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<&FnArg>>()
            .as_slice(),
        "args", // don't harcode(although it's the same as when making arg_pat)
    ));
    // return the impl fn
    impl_fn
}

/// Implement a SingletonFnWithArg.
pub(super) fn impl_singleton_fn_mut_with_arg(base: &ItemFn) -> ItemFn {
    // clone the base
    let mut impl_fn = base.clone();
    // make the impl fn private
    impl_fn.vis = Visibility::Inherited;
    // rename the function
    rename_fn(
        &mut impl_fn,
        format!(
            "{}{}",
            SINGLETON_FN_MUT_WITH_ARG_PREFIX,
            base.sig.ident.to_string()
        ),
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

/// Implement the facade for a SingletonFnMut.
pub(super) fn impl_singleton_fn_mut_facade(base: &ItemFn, impl_fn: &ItemFn) -> ItemFn {
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
            Self::use_mut_singleton(Self::#impl_fn_ident).await
        },
    );
    // return the facade fn
    facade_fn
}

/// Implement the facade for a SingletonFnWithArg.
pub(super) fn impl_singleton_fn_with_arg_facade(base: &ItemFn, impl_fn: &ItemFn) -> ItemFn {
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
            Self::use_singleton_with_arg(Self::#impl_fn_ident, (other)).await // remove hardcoding
        },
    );
    // return the facade fn
    facade_fn
}

/// Implement the facade for a SingletonFnWithArg.
pub(super) fn impl_singleton_fn_mut_with_arg_facade(base: &ItemFn, impl_fn: &ItemFn) -> ItemFn {
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

/// Convert the args of a function into a tuple and a map for mapping old types
/// to tuple variants.
fn fn_inputs_to_type_tuple(src: &[&FnArg]) -> TypeTuple {
    let elems: Punctuated<Type, Comma> = src
        .iter()
        .map(|elem| {
            if let FnArg::Typed(val) = elem {
                *val.ty.clone()
            } else {
                panic!("Cannot convert receiver to type for tuple!")
            }
        })
        .collect();

    // panic!("Elements: {}", quote!{ #elems });

    TypeTuple {
        paren_token: syn::token::Paren {
            span: Span::call_site(),
        },
        elems,
    }
}

/// Fix the inputs of a function.
///
/// Replaces all non-receiver args by a single tuple.
fn fn_fix_inputs(src: &mut ItemFn, tuple: &TypeTuple) {
    let tuple_arg: FnArg =
        syn::parse2(quote! {args: #tuple}).expect("Failed to parse new fn inputs.");
    // if let FnArg::Receiver(_) = tuple {
    //     panic!("Tuple arg must not be a receiver!");
    // }
    let receiver = src
        .sig
        .inputs
        .first()
        .cloned()
        .expect("Expected the fn receiver!");
    src.sig.inputs.clear();
    src.sig.inputs.push(receiver);
    src.sig.inputs.push(tuple_arg);
}

/// Replace all inputs in the block with an element of the tuple arg.
fn fn_fix_block(block: Block, src: &[&FnArg], tuple_name: &str) -> Block {
    let args_str = src
        .iter()
        .map(|arg| {
            if let FnArg::Typed(val) = arg {
                val
            } else {
                panic!("fn arg must not be receiver")
            }
        })
        .map(|arg| *arg.pat.clone())
        .enumerate()
        .map(|(i, arg)| {
            (
                format!("{}", quote! {#arg}),
                format!("{} . {}", tuple_name, i),
            )
        })
        .collect::<Vec<(String, String)>>();
    // let stmts: Vec<Stmt> = block
    //     .stmts
    //     .iter()
    //     .cloned()
    //     .map(|stmt| {
    //         let mut stmt_str = format!("{}", quote! {#stmt});
    //         args_str.iter().for_each(|arg| {
    //             stmt_str = stmt_str.replace(arg.0.as_str(), arg.1.as_str());
    //         });
    //         syn::parse_str::<Stmt>(stmt_str.as_str()).expect("Failed to parse mapped statement!")
    //     })
    //     .collect();
    // syn::parse_str::<Block>(
    //     stmts.join(';')
    // )
    // .expect("Failed to parse block!")
    let mut block_str = format!("{}", quote! {#block});
    args_str.iter().for_each(|arg| {
        block_str = block_str.replace(arg.0.as_str(), arg.1.as_str());
    });
    syn::parse_str::<Block>(block_str.as_str()).expect("Failed to parse block!")
}
