//! Blockz.

#[allow(unused_imports)]
#[macro_use]
extern crate blockz_derive;
#[doc(hidden)]
pub use blockz_derive::*;

pub mod singleton;

pub use singleton::Singleton;
pub use singleton::SingletonFn;
pub use singleton::SingletonFnMut;
