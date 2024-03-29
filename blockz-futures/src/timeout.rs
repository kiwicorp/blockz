//! Futures constrained by time.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use thiserror::Error;

/// Error type for futures that ran out of time.
#[derive(Clone, Copy, Debug, Error)]
#[error("future timed out")]
pub struct TimedOut(());

/// A future that must complete in a certain time interval.
#[pin_project]
pub struct Timeout<F> {
    #[pin]
    future: tokio::time::Timeout<F>,
}

impl<F: Future> Timeout<F> {
    /// Create a new `Timeout` future.
    pub fn new(future: F, timeout: Duration) -> Self {
        Self {
            future: tokio::time::timeout(timeout, future),
        }
    }
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, TimedOut>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut tokio::time::Timeout<F>> = this.future;

        if let Poll::Ready(result) = future.poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(_) => Poll::Ready(Err(TimedOut(()))),
            }
        } else {
            Poll::Pending
        }
    }
}

/// A future that must complete before a moment in time.
#[pin_project]
pub struct Deadline<F> {
    #[pin]
    future: tokio::time::Timeout<F>,
}

impl<F: Future> Deadline<F> {
    /// Create a new `Deadline` future.
    pub fn new(future: F, deadline: Instant) -> Self {
        Self {
            future: tokio::time::timeout_at(deadline.into(), future),
        }
    }
}

impl<F: Future> Future for Deadline<F> {
    type Output = Result<F::Output, TimedOut>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut tokio::time::Timeout<F>> = this.future;

        if let Poll::Ready(result) = future.poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(_) => Poll::Ready(Err(TimedOut(()))),
            }
        } else {
            Poll::Pending
        }
    }
}
