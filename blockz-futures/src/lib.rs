//! Utilities for working with futures.

#[macro_use]
extern crate pin_project;

use futures::Future;
use futures::TryFuture;
use tokio::sync::oneshot;

use self::cancel::Cancel;
use self::cancel::CancelChannelFuture;
use self::cancel::CancelHandle;
use self::cancel::TryCancel;

pub mod cancel;
pub mod timeout;

/// Kiwi extensions for futures.
pub trait FutureKiwiExt: Future + Sized {
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
pub trait TryFutureKiwiExt: TryFuture + Sized {
    /// Wrap a future with a cancel handle.
    fn try_cancel(self) -> (TryCancel<Self>, CancelHandle) {
        TryCancel::new(self)
    }

    /// Wrap a future with a custom cancel channel.
    fn try_with_cancel(self, cancel: oneshot::Receiver<()>) -> TryCancel<Self> {
        TryCancel::with_channel(self, cancel)
    }
}

impl<T: Future + Sized> FutureKiwiExt for T {}

impl<T: TryFuture + Sized> TryFutureKiwiExt for T {}
