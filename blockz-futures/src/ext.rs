//! Extensions for futures provided by blockz.

use std::time::Duration;
use std::time::Instant;

use futures::Future;
use tokio::sync::oneshot;

use crate::cancel::Cancel;
use crate::cancel::CancelChannelFuture;
use crate::cancel::CancelHandle;
use crate::timeout::Deadline;
use crate::timeout::Timeout;

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

mod private {
    pub trait Sealed {}

    impl<T: std::future::Future + Sized> Sealed for T {}
}

impl<T: Future + Sized> BlockzFutureExt for T {}
