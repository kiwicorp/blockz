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
pub struct Cancel<F> {
    #[pin]
    future: F,
    #[pin]
    cancel: oneshot::Receiver<()>,
}

impl<F> Cancel<F> {
    /// Create a new `Cancel` future.
    pub fn new(future: F) -> (Self, CancelHandle) {
        let (tx, rx) = oneshot::channel();
        (Self { future, cancel: rx }, CancelHandle::new(tx))
    }

    /// Create a `Cancel` future with a `cancel` channel.
    pub fn with_channel(future: F, cancel: oneshot::Receiver<()>) -> Self {
        Self { future, cancel }
    }
}

impl<F: Future> Future for Cancel<F> {
    type Output = Result<F::Output, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let cancel: Pin<&mut oneshot::Receiver<()>> = this.cancel;

        if let Poll::Ready(out) = future.poll(cx) {
            return Poll::Ready(Ok(out));
        }
        if let Poll::Ready(cancel) = cancel.poll(cx) {
            match cancel {
                Ok(()) => return Poll::Ready(Err(Canceled)),
                Err(e) => panic!("cancel dropped"),
            }
        }

        Poll::Pending
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

/// A cancel handle for canceling a future.
pub struct CancelHandle(oneshot::Sender<()>);

impl CancelHandle {
    /// Create a new cancel handle.
    pub fn new(inner: oneshot::Sender<()>) -> Self {
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
