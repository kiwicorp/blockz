//! Utilities for working with futures.

#[macro_use]
extern crate pin_project;

mod ext;

pub mod cancel;
pub mod timeout;

pub use self::ext::*;

#[cfg(test)]
mod test {
    use std::time::Duration;

    use futures::FutureExt;

    use crate::BlockzFutureExt;

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_handle_future_dropped() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        };

        let (cancel, cancel_handle) = fut.with_cancel_handle();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => {},
            _ = cancel => {
                panic!("cancelable future completed before the sleep");
            },
        }

        // the future has not been canceled since it has been already dropped
        // when the `sleep` finished earlier
        assert!(!cancel_handle.cancel());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_handle_future_completed() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        let (cancel, cancel_handle) = fut.with_cancel_handle();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(2)) => {
                panic!("sleep completed before cancelable future");
            },
            _ = cancel => {},
        }

        // the future has not been canceled since it has finished already
        assert!(!cancel_handle.cancel());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_handle_cancel_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        };

        let (cancel, cancel_handle) = fut.with_cancel_handle();
        // we create a shared future so that it doesn't get dropped when the
        // `sleep` finishes earlier
        let shared = cancel.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => {},
            _ = shared.clone() => {
                panic!("cancelable future completed before the sleep");
            },
        }

        assert!(cancel_handle.cancel());

        let result = shared.await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_handle_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        let (cancel, cancel_handle) = fut.with_cancel_handle();
        // we create a shared future so that it doesn't get dropped when the
        // `sleep` finishes earlier
        let shared = cancel.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(2)) => {},
            _ = shared.clone() => {},
        }

        assert!(!cancel_handle.cancel());

        let result = shared.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_future_cancel() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        };

        let cancel = fut.with_cancel_future(async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        });

        let result = cancel.await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_future_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        let cancel = fut.with_cancel_future(async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        });

        let result = cancel.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_channel_cancel() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        };

        let (tx, rx) = tokio::sync::oneshot::channel();

        let cancel = fut.with_cancel_channel(rx);
        // we create a shared future so that it doesn't get dropped when the
        // `sleep` finishes earlier
        let shared = cancel.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => {},
            _ = shared.clone() => {
                panic!("cancelable future completed before the sleep");
            },
        }

        assert!(tx.send(()).is_ok());

        let result = shared.await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_with_cancel_channel_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        let (_tx, rx) = tokio::sync::oneshot::channel();

        let cancel = fut.with_cancel_channel(rx);

        let result = cancel.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_timeout() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        };

        assert!(fut
            .timeout(std::time::Duration::from_millis(1))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_timeout_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        assert!(fut
            .timeout(std::time::Duration::from_millis(2))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_deadline() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(3)).await;
        };

        assert!(fut
            .deadline(std::time::Instant::now() + std::time::Duration::from_millis(1))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_blockz_future_ext_deadline_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        };

        assert!(fut
            .deadline(std::time::Instant::now() + std::time::Duration::from_millis(2))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_interrupt_chain_timed_out() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(3)).await;
        };

        let (fut, cancel) = fut.timeout(Duration::from_millis(1)).with_cancel_handle();
        let fut = fut.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(2)) => {},
            _ = fut.clone() => {},
        };

        assert!(!cancel.cancel());
        assert!(matches!(fut.await, Ok(Err(_))));
    }

    #[tokio::test]
    async fn test_interrupt_chain_canceled() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(3)).await;
        };

        let (fut, cancel) = fut.timeout(Duration::from_millis(2)).with_cancel_handle();
        let fut = fut.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => {},
            _ = fut.clone() => {},
        };

        assert!(cancel.cancel());
        assert!(matches!(fut.await, Err(_)));
    }

    #[tokio::test]
    async fn test_interrupt_chain_ok() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            Ok::<_, &str>(())
        };

        let (fut, cancel) = fut.timeout(Duration::from_millis(2)).with_cancel_handle();
        let fut = fut.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(2)) => {},
            _ = fut.clone() => {},
        };

        assert!(!cancel.cancel());
        assert!(matches!(fut.await, Ok(Ok(Ok(())))));
    }

    #[tokio::test]
    async fn test_interrupt_chain_err() {
        let fut = async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            Err::<(), _>("error")
        };

        let (fut, cancel) = fut.timeout(Duration::from_millis(2)).with_cancel_handle();
        let fut = fut.shared();

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(2)) => {},
            _ = fut.clone() => {},
        };

        assert!(!cancel.cancel());
        assert!(matches!(fut.await, Ok(Ok(Err("error")))));
    }
}
