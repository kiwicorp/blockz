//! A future wrapped with a timeout.

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use futures::prelude::*;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
#[error("future timed out")]
pub struct TimedOut;

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
                Err(_) => Poll::Ready(Err(TimedOut)),
            }
        } else {
            Poll::Pending
        }
    }
}

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
                Err(_) => Poll::Ready(Err(TimedOut)),
            }
        } else {
            Poll::Pending
        }
    }
}
