//! Convenient error types for the futures extensions provided by blockz.

use std::error::Error;

use thiserror::Error;

use crate::cancel::Canceled;
use crate::timeout::TimedOut;

/// Trait that defines behaviour for errors that "may be" a certain kind of
/// error. This *SHOULD* be useful for unpacking long chains of
/// `Result<Result<Result<Result..`.
pub trait Interrupted<E: Error>: private::Sealed + Into<MaybeInterrupted<E>> {}

/// Possible outcome of a future.
#[derive(Debug, Error)]
pub enum MaybeInterrupted<E: Error> {
    #[error("{0}")]
    Error(E),
    #[error("{0}")]
    Canceled(Canceled),
    #[error("{0}")]
    TimedOut(TimedOut),
}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::cancel::Canceled {}

    impl<E: std::error::Error> Sealed for crate::cancel::MaybeCanceled<E> {}

    impl Sealed for crate::timeout::TimedOut {}

    impl<E: std::error::Error> Sealed for crate::timeout::MaybeTimedOut<E> {}
}

impl<E, T> Interrupted<E> for T
where
    E: Error,
    T: self::private::Sealed + Into<MaybeInterrupted<E>>,
{}
