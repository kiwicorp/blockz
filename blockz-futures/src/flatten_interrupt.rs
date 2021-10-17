//! Future that flattens interrupt signals.

use std::error::Error;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::prelude::*;

use crate::MayInterrupt;
use crate::MaybeInterrupted;

#[pin_project]
pub struct TryFlattenInterrupt<F> {
    #[pin]
    future: F,
}

impl<F> TryFlattenInterrupt<F> {
    pub fn new(future: F) -> Self {
        Self { future }
    }
}

impl<F: TryFuture> Future for TryFlattenInterrupt<F>
where
    <F as TryFuture>::Error: Error,
    <F as TryFuture>::Error: MayInterrupt<<F as TryFuture>::Error>,
{
    type Output = Result<F::Ok, MaybeInterrupted<<F as TryFuture>::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future: Pin<&mut F> = this.future;

        if let Poll::Ready(result) = future.try_poll(cx) {
            match result {
                Ok(out) => Poll::Ready(Ok(out)),
                Err(e) => Poll::Ready(Err(e.into())),
            }
        } else {
            Poll::Pending
        }
    }
}
