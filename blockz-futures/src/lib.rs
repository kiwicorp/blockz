//! Utilities for working with futures.

#[macro_use]
extern crate pin_project;

use std::error::Error;

use futures::Future;
use futures::TryFuture;
use thiserror::Error;
use tokio::sync::oneshot;

use self::cancel::Cancel;
use self::cancel::CancelChannelFuture;
use self::cancel::CancelHandle;
use self::cancel::Canceled;
use self::cancel::TryCancel;
use self::timeout::TimedOut;

pub mod cancel;
pub mod timeout;

/// Extension for futures provided by `blockz`.
pub trait FutureKiwiExt: Future + Sized + private::Sealed {
    /// Wrap a future with a cancel handle.
    fn cancel(self) -> (Cancel<Self, CancelChannelFuture>, CancelHandle) {
        Cancel::new(self)
    }

    /// Wrap a future with a cancel future.
    fn with_cancel<C: Future<Output = ()>>(self, cancel: C) -> Cancel<Self, C> {
        Cancel::with_cancel(self, cancel)
    }

    /// Wrap a future with a cancel channel.
    fn with_cancel_channel(
        self,
        cancel: oneshot::Receiver<()>,
    ) -> Cancel<Self, CancelChannelFuture> {
        Cancel::with_cancel_channel(self, cancel)
    }
}

/// Kiwi extensions for futures.
pub trait TryFutureKiwiExt: TryFuture + Sized + private::Sealed {
    /// Wrap a future with a cancel handle.
    fn try_cancel(self) -> (TryCancel<Self, CancelChannelFuture>, CancelHandle) {
        TryCancel::new(self)
    }

    /// Wrap a future with a custom cancel channel.
    fn try_with_cancel<C: Future<Output = ()>>(self, cancel: C) -> TryCancel<Self, C> {
        TryCancel::with_cancel(self, cancel)
    }

    /// Wrap a future with a custom cancel channel.
    fn try_with_cancel_channel(
        self,
        cancel: oneshot::Receiver<()>,
    ) -> TryCancel<Self, CancelChannelFuture> {
        TryCancel::with_cancel_channel(self, cancel)
    }
}

/// Trait that defines behaviour for errors that "may be" a certain kind of
/// error. This *SHOULD* be useful for unpacking long chains of
/// `Result<Result<Result<Result..`.
pub trait Maybe<E: Error>: private::Sealed {
    fn into_maybe_error(self) -> MaybeError<E>;
}

/// Possible outcome of a future.
#[derive(Error)]
pub enum MaybeError<E: Error> {
    #[error("{0}")]
    Error(E),
    #[error("{0}")]
    Canceled(Canceled),
    #[error("{0}")]
    TimedOut(TimedOut),
}

mod private {
    pub trait Sealed {}

    impl<T: std::future::Future + Sized> Sealed for T {}

    impl<E: std::error::Error> Sealed for crate::cancel::MaybeCanceled<E> {}

    impl<E: std::error::Error> Sealed for crate::timeout::MaybeTimedOut<E> {}
}

impl<T: Future + Sized> FutureKiwiExt for T {}

impl<T: TryFuture + Sized> TryFutureKiwiExt for T {}
