//! Extensions for futures.

use std::time::Duration;
use std::time::Instant;

use futures::Future;
use tokio::sync::oneshot;

use crate::cancel::Cancel;
use crate::cancel::CancelChannelFuture;
use crate::cancel::CancelHandle;
use crate::timeout::Deadline;
use crate::timeout::Timeout;

/// Extensions for futures.
pub trait BlockzFutureExt: Future + Sized + private::Sealed {
    /// Get a cancel handle for this future.
    fn with_cancel_handle(self) -> (Cancel<Self, CancelChannelFuture>, CancelHandle) {
        Cancel::new(self)
    }

    /// Force this future to complete before the other future.
    fn with_cancel_future<C: Future<Output = ()>>(self, cancel: C) -> Cancel<Self, C> {
        Cancel::with_cancel(self, cancel)
    }

    /// Force this future to complete before the channel produces a value or
    /// is closed.
    fn with_cancel_channel(
        self,
        cancel: oneshot::Receiver<()>,
    ) -> Cancel<Self, CancelChannelFuture> {
        Cancel::with_cancel_channel(self, cancel)
    }

    /// Force this future to complete before a point in time.
    fn deadline(self, deadline: Instant) -> Deadline<Self> {
        Deadline::new(self, deadline)
    }

    /// Force this future to complete in a time interval.
    fn timeout(self, timeout: Duration) -> Timeout<Self> {
        Timeout::new(self, timeout)
    }
}

mod private {
    pub trait Sealed {}

    impl<T: std::future::Future + Sized> Sealed for T {}
}

impl<T: Future + Sized> BlockzFutureExt for T {}
