//! Impl fn factory.

use std::collections::HashMap;
use std::ops::DerefMut;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::format_ident;
use quote::quote;

use syn::ItemFn;

use super::singleton_fns::SingletonFnType;

/// Prefix for an impl fn used by a singleton fn.
const SINGLETON_FN_PREFIX: &str = "blockz_singleton_fn_";
/// Prefix for an impl fn used by a singleton fn with arg.
const SINGLETON_FN_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_with_arg_";
/// Prefix for an impl fn used by a singleton mut fn.
const SINGLETON_FN_MUT_PREFIX: &str = "blockz_singleton_fn_mut_";
/// Prefix for an impl fn used by a singleton mut fn with arg.
const SINGLETON_FN_MUT_WITH_ARG_PREFIX: &str = "blockz_singleton_fn_mut_with_arg_";

/// Factory that builds the implementation of singleton fns.
pub(super) struct ImplFnFactory<'f> {
    base: &'f ItemFn,
    fn_type: &'f SingletonFnType<'f>,
}

impl<'f> ImplFnFactory<'f> {
    /// Create a new ImplFnFactory.
    pub fn new(base: &'f ItemFn, fn_type: &'f SingletonFnType) -> Self {
        Self { base, fn_type }
    }

    /// Renames a function according to the appropriate singleton fn type.
    fn rename_fn(&self, target: &mut ItemFn) {
        target.sig.ident = match self.fn_type {
            SingletonFnType::NonMut { .. } => {
                format_ident!("{}{}", SINGLETON_FN_PREFIX, target.sig.ident)
            }
            SingletonFnType::NonMutWithArg { .. } => {
                format_ident!("{}{}", SINGLETON_FN_WITH_ARG_PREFIX, target.sig.ident)
            }
            SingletonFnType::Mut { .. } => {
                format_ident!("{}{}", SINGLETON_FN_MUT_PREFIX, target.sig.ident)
            }
            SingletonFnType::MutWithArg { .. } => {
                format_ident!("{}{}", SINGLETON_FN_MUT_WITH_ARG_PREFIX, target.sig.ident)
            }
        };
    }

    /// Fixes the args of a function, if necessary.
    fn fix_fn_args(&self, target: &mut ItemFn) -> syn::Result<()> {
        let impl_fn_arg: Option<_>;
        match self.fn_type {
            SingletonFnType::NonMut => impl_fn_arg = None,
            SingletonFnType::NonMutWithArg(arg) => impl_fn_arg = Some(arg),
            SingletonFnType::Mut => impl_fn_arg = None,
            SingletonFnType::MutWithArg(arg) => impl_fn_arg = Some(arg),
        }
        if let Some(value) = impl_fn_arg {
            target.sig.inputs = target
                .sig
                .inputs
                .iter()
                .cloned()
                .take(1)
                .chain(vec![value.build_impl_fn_sig_arg()?])
                .collect();
        }
        Ok(())
    }

    /// Recursevly apply a replace legend to a token stream.
    fn apply_replace_legend(stream: TokenStream, legend: &HashMap<String, TokenStream>) -> TokenStream {
        stream.into_iter()
            .map(|tt| {
                match tt {
                    TokenTree::Ident(ident) => {
                        if let Some(value) = legend.get(ident.to_string().as_str()) {
                            TokenTree::Group(Group::new(Delimiter::None, value.clone()))
                        } else {
                            TokenTree::Ident(ident)
                        }
                    },
                    TokenTree::Group(group) => {
                        let delim = group.delimiter();
                        let tokens = Self::apply_replace_legend(group.stream(), legend);
                        TokenTree::Group(Group::new(delim, tokens))
                    },
                    other => other,
                }
            })
            .collect::<TokenStream>()
    }

    /// Fixes the block of a function, if necessary.
    fn fix_fn_block(&self, target: &mut ItemFn) -> syn::Result<()> {
        // create the replacement legend for fixing the impl fn block
        let replace_legend: Option<_>;
        match self.fn_type {
            SingletonFnType::NonMut => replace_legend = None,
            SingletonFnType::NonMutWithArg(arg) => {
                replace_legend = arg.build_impl_fn_replacement_legend()
            }
            SingletonFnType::Mut => replace_legend = None,
            SingletonFnType::MutWithArg(arg) => {
                replace_legend = arg.build_impl_fn_replacement_legend()
            }
        }

        // if fixing the impl block is not required, return
        if replace_legend.is_none() {
            return Ok(());
        }

        // unwrap the replace legend
        let replace_legend = replace_legend.unwrap();

        // get the block
        let block = target.block.deref_mut();
        *block = syn::parse2(Self::apply_replace_legend(quote! { #block }, &replace_legend))?;
        // panic!("{}", quote! {#block});

        // {
        //     let mut block_str = format!("{}", quote! { #block });

        //     // do the replacements
        //     replace_legend.into_iter().for_each(|(from, to)| {
        //         block_str = block_str.replace(from.as_str(), to.as_str());
        //     });

        //     *block = syn::parse_str(block_str.as_str())?;
        // }

        // block.stmts.iter_mut().for_each(|stmt| {
        //     let mut stmt_str = format!("{}", quote! { #stmt });
        //     replace_legend.iter().for_each(|arg| {
        //         stmt_str = stmt_str
        //             .split(' ')
        //             .collect::<Vec<&str>>()
        //             .into_iter()
        //             .map(|val| {
        //                 // if val == arg.0.as_str() {
        //                 val.replace(arg.0.as_str(), arg.1.as_str())
        //                 // } else {
        //                 //     val.to_string()
        //                 // }
        //             })
        //             .collect::<Vec<String>>()
        //             .join(" ");
        //     });
        //     // panic!(stmt_str);
        //     *stmt =
        //         syn::parse_str(stmt_str.as_str()).expect(format!("{}", quote! { #stmt }).as_str());
        // });

        Ok(())
    }

    /// Make a function private.
    fn make_fn_private(target: &mut ItemFn) {
        target.vis = syn::Visibility::Inherited;
    }

    pub fn build(&self) -> syn::Result<ItemFn> {
        // create the working copy
        let mut impl_fn = self.base.clone();
        // make the impl fn private
        Self::make_fn_private(&mut impl_fn);
        // rename the function
        self.rename_fn(&mut impl_fn);
        // fix the fn args
        self.fix_fn_args(&mut impl_fn)?;
        // fix the fn block
        self.fix_fn_block(&mut impl_fn)?;
        // return the function
        Ok(impl_fn)
    }
}
