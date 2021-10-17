//! Convenient error types for the futures extensions provided by blockz.

use std::error::Error;

use thiserror::Error;

use crate::cancel::Canceled;
use crate::timeout::TimedOut;

/// Error types that may be caused by an interrupt signal.
pub trait MayInterrupt<E: Error>: private::Sealed + Into<MaybeInterrupted<E>> {}

/// Error returned by a future that may have been interrupted.
#[derive(Clone, Debug, Error)]
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

impl<E, T> MayInterrupt<E> for T
where
    E: Error,
    T: self::private::Sealed + Into<MaybeInterrupted<E>>,
{
}
