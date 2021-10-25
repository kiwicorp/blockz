//! Flatten multiple interrupts into a single error.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// A future that flattens multiple interrupts into a single error.
#[pin_project]
pub struct FlattenInterrupts<F> {
    future: F,
}

impl<F> Future for FlattenInterrupts<F> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
