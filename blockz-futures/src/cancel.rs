//! A future wrapped with a cancel signal.

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::prelude::*;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Clone, Copy, Debug, Error)]
#[error("future has been canceled")]
pub struct Canceled;

#[derive(Clone, Copy, Debug, Error)]
pub enum MaybeCanceled<E: std::error::Error> {
    #[error("{0}")]
    Error(E),
    #[error("{0}")]
    Canceled(Canceled),
}

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
            Poll::Ready(Err(Canceled))
        } else {
            Poll::Pending
        }
    }
}

/// Typed future that executes a graceful shutdown routine in case of an
/// interrupt.
#[pin_project]
pub struct Graceful<F> {
    #[pin]
    future: F,
}

#[pin_project]
pub struct TryCancel<F, C> {
    #[pin]
    future: F,
    #[pin]
    cancel: C,
}

impl<F> TryCancel<F, CancelChannelFuture> {
    /// Create a new `TryCancel` future.
    pub fn new(future: F) -> (Self, CancelHandle) {
        let (tx, rx) = oneshot::channel();
        let cancel = CancelChannelFuture::new(rx);
        (Self { future, cancel }, CancelHandle::new(tx))
    }

    /// Create a `TryCancel` future with a `cancel` channel.
    pub fn with_cancel_channel(future: F, rx: oneshot::Receiver<()>) -> Self {
        let cancel = CancelChannelFuture::new(rx);
        Self { future, cancel }
    }
}

impl<F, C> TryCancel<F, C> {
    /// Create a new `Cancel` future with a `cancel` future.
    pub(crate) fn with_cancel(future: F, cancel: C) -> Self {
        Self { future, cancel }
    }
}

impl<F: TryFuture, C: Future<Output = ()>> Future for TryCancel<F, C>
where
    <F as TryFuture>::Error: std::error::Error,
{
    type Output = Result<F::Ok, MaybeCanceled<F::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let cancel: Pin<&mut C> = this.cancel;

        if let Poll::Ready(result) = future.try_poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(e) => Poll::Ready(Err(MaybeCanceled::Error(e))),
            }
        } else if cancel.poll(cx).is_ready() {
            Poll::Ready(Err(MaybeCanceled::Canceled(Canceled)))
        } else {
            Poll::Pending
        }
    }
}

/// Typed future for a cancel channel.
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

/// A cancel handle for canceling a future.
pub struct CancelHandle(oneshot::Sender<()>);

impl CancelHandle {
    /// Create a new cancel handle.
    pub(crate) fn new(inner: oneshot::Sender<()>) -> Self {
        Self(inner)
    }

    /// Cancel the future.
    ///
    /// Returns whether the future has been canceled or not. A future will not
    /// be canceled if it has finished already.
    pub fn cancel(self) -> bool {
        self.0.send(()).is_ok()
    }
}
