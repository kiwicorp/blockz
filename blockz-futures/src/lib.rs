//! Utilities for working with futures.

#[macro_use]
extern crate pin_project;

mod error;
mod ext;

pub mod cancel;
pub mod flatten_interrupt;
pub mod timeout;

pub use self::error::*;
pub use self::ext::*;

#[cfg(test)]
mod test {
    use std::time::Duration;

    use futures::FutureExt;

    use crate::BlockzFutureExt;
    use crate::BlockzTryFutureExt;

    #[tokio::test]
    async fn test_blockz_future_ext_cancel_future_dropped() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        };

        let (cancel, cancel_handle) = fut.cancel();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            _ = cancel => {
                panic!("cancelable future completed before the sleep");
            },
        }

        // the future has not been canceled since it has been already dropped
        // when the `sleep` finished earlier
        assert!(!cancel_handle.cancel());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_cancel_future_completed() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };

        let (cancel, cancel_handle) = fut.cancel();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(200)) => {
                panic!("sleep completed before cancelable future");
            },
            _ = cancel => {},
        }

        // the future has not been canceled since it has finished already
        assert!(!cancel_handle.cancel());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_cancel() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        };

        let (cancel, cancel_handle) = fut.cancel();
        // we create a shared future so that it doesn't get dropped when the
        // `sleep` finishes earlier
        let shared = cancel.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            _ = shared.clone() => {
                panic!("cancelable future completed before the sleep");
            },
        }

        assert!(cancel_handle.cancel());

        let result = shared.await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_flatten_interrupt() {
        let fut = async { Ok::<_, Box<dyn std::error::Error>>(42) };

        let (fut, cancel) = fut.timeout(Duration::from_millis(100)).cancel();
        let fut = fut.try_flatten_interrupt();
        let out = fut.await;
    }
}
