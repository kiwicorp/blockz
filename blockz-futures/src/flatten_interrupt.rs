//! Future that flattens interrupt signals.

// use std::convert::Infallible;
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

// impl<F, O, I> Future for TryFlattenInterrupt<F>
// where
//     F: Future<Output = Result<O, I>>,
//     I: MayInterrupt<Infallible>,
// {
//     type Output = Result<O, MaybeInterrupted<Infallible>>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.project();
//         let future = this.future;
//         match future.poll(cx) {
//             Poll::Ready(Ok(out)) => Poll::Ready(Ok(out)),
//             Poll::Ready(Err(interrupt)) => Poll::Ready(Err(interrupt.into())),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

// impl<F, O, E, I, Ie> Future for TryFlattenInterrupt<F>
// where
//     F: Future<Output = Result<Result<O, E>, I>>,
//     E: Error,
//     I: MayInterrupt<Ie>,
//     Ie: Error,
// {
//     type Output = Result<O, MaybeInterrupted<Ie>>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.project();
//         let future = this.future;
//         match future.poll(cx) {
//             Poll::Ready(Ok(Ok(out))) => Poll::Ready(Ok(out)),
//             Poll::Ready(Ok(Err(error))) => Poll::Ready(Err(MaybeInterrupted::Error(error))),
//             Poll::Ready(Err(interrupt)) => Poll::Ready(Err(interrupt.into())),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }
