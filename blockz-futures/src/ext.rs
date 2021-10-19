//! Extensions for futures provided by blockz.

use std::error::Error;
use std::time::Duration;
use std::time::Instant;

use futures::Future;
use futures::TryFuture;
use tokio::sync::oneshot;

use crate::cancel::Cancel;
use crate::cancel::CancelChannelFuture;
use crate::cancel::CancelHandle;
use crate::cancel::TryCancel;
use crate::flatten_interrupt::TryFlattenInterrupt;
use crate::timeout::Deadline;
use crate::timeout::Timeout;
use crate::timeout::TryDeadline;
use crate::timeout::TryTimeout;
use crate::MayInterrupt;

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

    fn deadline(self, deadline: Instant) -> Deadline<Self> {
        Deadline::new(self, deadline)
    }

    fn timeout(self, timeout: Duration) -> Timeout<Self> {
        Timeout::new(self, timeout)
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
    fn try_flatten_interrupt<E: Error>(self) -> TryFlattenInterrupt<Self>
    where
        <Self as TryFuture>::Error: Error,
        <Self as TryFuture>::Error: MayInterrupt<E>,
    {
        TryFlattenInterrupt::new(self)
    }

    fn try_deadline(self, deadline: Instant) -> TryDeadline<Self> {
        TryDeadline::new(self, deadline)
    }

    fn try_timeout(self, timeout: Duration) -> TryTimeout<Self> {
        TryTimeout::new(self, timeout)
    }
}

mod private {
    pub trait Sealed {}

    impl<T: std::future::Future + Sized> Sealed for T {}
}

impl<T: Future + Sized> BlockzFutureExt for T {}

impl<T: TryFuture + Sized> BlockzTryFutureExt for T {}
