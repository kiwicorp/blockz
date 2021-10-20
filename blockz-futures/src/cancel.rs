//! Futures that can be canceled.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use thiserror::Error;
use tokio::sync::oneshot;

/// Error type for futures that can be canceled.
#[derive(Clone, Copy, Debug, Error)]
#[error("future has been canceled")]
pub struct Canceled(());

/// A future that can be canceled.
#[pin_project]
pub struct Cancel<F, C> {
    #[pin]
    future: F,
    #[pin]
    cancel: C,
}

impl<F> Cancel<F, CancelChannelFuture> {
    /// Create a new `Cancel` future.
    pub(crate) fn new(future: F) -> (Self, CancelHandle) {
        let (tx, rx) = oneshot::channel();
        let cancel = CancelChannelFuture::new(rx);
        (Self { future, cancel }, CancelHandle::new(tx))
    }

    /// Create a `Cancel` future with a `cancel` channel.
    pub(crate) fn with_cancel_channel(future: F, cancel: oneshot::Receiver<()>) -> Self {
        let cancel = CancelChannelFuture::new(cancel);
        Self { future, cancel }
    }
}

impl<F, C> Cancel<F, C> {
    /// Create a new `Cancel` future with a `cancel` future.
    pub(crate) fn with_cancel(future: F, cancel: C) -> Self {
        Self { future, cancel }
    }
}

impl<F: Future, C: Future<Output = ()>> Future for Cancel<F, C> {
    type Output = Result<F::Output, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let cancel: Pin<&mut C> = this.cancel;

        if let Poll::Ready(out) = future.poll(cx) {
            Poll::Ready(Ok(out))
        } else if cancel.poll(cx).is_ready() {
            Poll::Ready(Err(Canceled(())))
        } else {
            Poll::Pending
        }
    }
}

/// Future that produces a value when the underlying channel produces a value
/// or is closed.
#[pin_project]
pub struct CancelChannelFuture(#[pin] oneshot::Receiver<()>);

impl CancelChannelFuture {
    /// Create a new `CancelChannelFuture`.
    pub(crate) fn new(rx: oneshot::Receiver<()>) -> Self {
        Self(rx)
    }
}

impl Future for CancelChannelFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let rx: Pin<&mut oneshot::Receiver<()>> = this.0;
        rx.poll(cx).map(|_| ())
    }
}

/// A handle that can be used for canceling a future.
///
/// This handle will send a cancellation signal when dropped.
pub struct CancelHandle(oneshot::Sender<()>);

impl CancelHandle {
    /// Create a new cancel handle.
    pub(crate) fn new(inner: oneshot::Sender<()>) -> Self {
        Self(inner)
    }

    /// Cancel the future.
    ///
    /// Returns whether the future has been canceled or not.
    ///
    /// This function returns false if the future has been dropped or if it has
    /// finished prior to trying to cancel it.
    pub fn cancel(self) -> bool {
        self.0.send(()).is_ok()
    }
}
