//! Extensions for futures provided by blockz.

use std::error::Error;

use futures::Future;
use futures::TryFuture;
use tokio::sync::oneshot;

use crate::Interrupted;
use crate::cancel::Cancel;
use crate::cancel::CancelChannelFuture;
use crate::cancel::CancelHandle;
use crate::cancel::TryCancel;
use crate::flatten_interrupt::FlattenInterrupt;

/// Extensions for futures provided by blockz.
pub trait BlockzFutureExt: Future + Sized + private::Sealed {
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

/// Extensions for futures provided by blockz.
///
/// These extensions are relevant for futures that return a `Result`.
pub trait BlockzTryFutureExt: TryFuture + Sized + private::Sealed {
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

    /// Flatten interrupts.
    fn flatten_interrupt(self) -> FlattenInterrupt<Self>
    where
        <Self as TryFuture>::Error: Error,
        <Self as TryFuture>::Error: Interrupted<<Self as TryFuture>::Error>,
    {
        FlattenInterrupt::new(self)
    }
}

mod private {
    pub trait Sealed {}

    impl<T: std::future::Future + Sized> Sealed for T {}
}

impl<T: Future + Sized> BlockzFutureExt for T {}

impl<T: TryFuture + Sized> BlockzTryFutureExt for T {}