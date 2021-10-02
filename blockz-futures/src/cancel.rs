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

#[pin_project]
pub struct TryCancel<F> {
    #[pin]
    future: F,
    #[pin]
    cancel: oneshot::Receiver<()>,
}

impl<F> TryCancel<F> {
    /// Create a new `TryCancel` future.
    pub fn new(future: F) -> (Self, CancelHandle) {
        let (tx, rx) = oneshot::channel();
        (Self { future, cancel: rx }, CancelHandle::new(tx))
    }

    /// Create a `TryCancel` future with a `cancel` channel.
    pub fn with_channel(future: F, cancel: oneshot::Receiver<()>) -> Self {
        Self { future, cancel }
    }
}

impl<F: TryFuture> Future for TryCancel<F> {
    type Output = Result<F::Ok, F::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let cancel: Pin<&mut oneshot::Receiver<()>> = this.cancel;

        if let Poll::Ready(out) = future.try_poll(cx) {
            return Poll::Ready(out);
        }
        if let Poll::Ready(cancel) = cancel.poll(cx) {
            match cancel {
                Ok(()) => todo!(),
                Err(e) => panic!("cancel dropped"),
            }
        }

        Poll::Pending
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
        match rx.poll(cx) {
            Poll::Ready(Ok(_)) => Poll::Ready(()),
            Poll::Ready(Err(e)) => panic!("cancel channel dropped"),
            Poll::Pending => Poll::Pending,
        }
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
