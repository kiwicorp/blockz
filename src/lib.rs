//! Blockz.

pub use blockz_derive;

pub mod component;
pub mod configuration;
pub mod singleton;

/// Prelude for blockz.
pub mod prelude {
    pub use crate::component::ComponentExt;
    pub use crate::configuration::Configuration;
    pub use crate::singleton::Singleton;
}
