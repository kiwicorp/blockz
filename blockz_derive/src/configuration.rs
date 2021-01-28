//! #[derive(Configuration)].

use crate::paths;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub fn impl_configuration_trait(type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let envy = paths::envy_path();
    quote! {
        #[async_trait::async_trait]
        impl #blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;
            type Opts = Option<String>;

            async fn load(opts: Self::Opts) -> #anyhow::Result<Self::Inner> {
                if let Some(prefix) = opts {
                    Ok(#envy::prefixed(prefix).from_env::<Self::Inner>()?)
                } else {
                    Ok(#envy::from_env::<Self::Inner>()?)
                }
            }
        }
    }
}
