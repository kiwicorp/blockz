//! A future wrapped with a timeout.

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use futures::future::Pending;
use futures::prelude::*;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
#[error("future timed out")]
pub struct TimedOut;

#[derive(Clone, Copy, Debug, Error)]
pub enum MaybeTimedOut<E: std::error::Error> {
    #[error("{0}")]
    Error(E),
    #[error("{0}")]
    TimedOut(TimedOut),
}

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
pub struct TryTimeout<F> {
    #[pin]
    future: F,
    #[pin]
    timeout: tokio::time::Timeout<Pending<()>>,
}

impl<F: Future> TryTimeout<F> {
    /// Create a new `TryTimeout` future.
    pub fn new(future: F, timeout: Duration) -> Self {
        Self {
            future,
            timeout: tokio::time::timeout(timeout, futures::future::pending()),
        }
    }
}

impl<F: TryFuture> Future for TryTimeout<F>
where
    <F as TryFuture>::Error: std::error::Error,
{
    type Output = Result<F::Ok, MaybeTimedOut<F::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let timeout: Pin<&mut tokio::time::Timeout<futures::future::Pending<()>>> = this.timeout;

        if let Poll::Ready(result) = future.try_poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(e) => Poll::Ready(Err(MaybeTimedOut::Error(e))),
            }
        } else if let Poll::Ready(Err(_)) = timeout.poll(cx) {
            Poll::Ready(Err(MaybeTimedOut::TimedOut(TimedOut)))
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

#[pin_project]
pub struct TryDeadline<F> {
    #[pin]
    future: F,
    #[pin]
    timeout: tokio::time::Timeout<Pending<()>>,
}

impl<F: Future> TryDeadline<F> {
    /// Create a new `TryDeadline` future.
    pub fn new(future: F, deadline: Instant) -> Self {
        Self {
            future,
            timeout: tokio::time::timeout_at(deadline.into(), futures::future::pending()),
        }
    }
}

impl<F: TryFuture> Future for TryDeadline<F>
where
    <F as TryFuture>::Error: std::error::Error,
{
    type Output = Result<F::Ok, MaybeTimedOut<F::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;
        let timeout: Pin<&mut tokio::time::Timeout<futures::future::Pending<()>>> = this.timeout;

        if let Poll::Ready(result) = future.try_poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(e) => Poll::Ready(Err(MaybeTimedOut::Error(e))),
            }
        } else if let Poll::Ready(Err(_)) = timeout.poll(cx) {
            Poll::Ready(Err(MaybeTimedOut::TimedOut(TimedOut)))
        } else {
            Poll::Pending
        }
    }
}
