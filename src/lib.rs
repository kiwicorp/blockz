//! Blockz.

#[allow(unused_imports)]
#[macro_use]
extern crate blockz_derive;
pub use blockz_derive::*;

pub mod singleton;

pub use singleton::Singleton;
