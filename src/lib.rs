//! Blockz.

pub use blockz_derive as derive;

pub mod component;
#[cfg(feature = "configuration")]
pub mod configuration;
#[cfg(feature = "singleton")]
pub mod singleton;

/// Prelude for blockz.
pub mod prelude {
    pub use crate::component::ComponentExt;
    #[cfg(feature = "configuration")]
    pub use crate::configuration::Configuration;
    pub use crate::derive::*;
    #[cfg(feature = "singleton")]
    pub use crate::singleton::Singleton;
}
