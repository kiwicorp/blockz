//! Example usage of chaining multiple interrupts.

use std::time::Duration;

use blockz_futures::BlockzFutureExt;

async fn timeout_cancel_timed_out() {
    // dummy future that waits for 3 ms
    let fut = async {
        tokio::time::sleep(Duration::from_millis(3)).await;
    };

    // assign a timeout and a cancel handle to the original future
    let (fut_with_interrupts, cancel_handle) =
        fut.timeout(Duration::from_millis(1)).with_cancel_handle();

    // create a task that sleeps for 3 ms before canceling the original future
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(3)).await;
        // the cancel handle returns false - the future has already finished
        assert!(!cancel_handle.cancel());
    });

    // the future will return with an error (a timeout error)
    assert!(matches!(fut_with_interrupts.await, Ok(Err(_))));
}

async fn timeout_cancel_canceled() {
    // dummy future that waits for 3 ms
    let fut = async {
        tokio::time::sleep(Duration::from_millis(3)).await;
    };

    // assign a timeout and a cancel handle to the original future
    let (fut_with_interrupts, cancel_handle) =
        fut.timeout(Duration::from_millis(3)).with_cancel_handle();

    // create a task that sleeps for 1 ms before canceling the original future
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1)).await;
        // the cancel handle returns true - the future has been canceled
        assert!(cancel_handle.cancel());
    });

    // the future will return with an error (a cancel error)
    assert!(matches!(fut_with_interrupts.await, Err(_)));
}

async fn timeout_cancel_ok() {
    // dummy future that waits for 1 ms
    let fut = async {
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok::<&str, &str>("ok")
    };

    // assign a timeout and a cancel handle to the original future
    let (fut_with_interrupts, cancel_handle) =
        fut.timeout(Duration::from_millis(3)).with_cancel_handle();

    // create a task that sleeps for 5 ms before canceling the original future
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(3)).await;
        // the cancel handle returns false - the future has already finished
        assert!(!cancel_handle.cancel());
    });

    // the future will return successfully
    assert!(matches!(fut_with_interrupts.await, Ok(Ok(Ok("ok")))));
}

async fn timeout_cancel_err() {
    // dummy future that waits for 1 ms
    let fut = async {
        tokio::time::sleep(Duration::from_millis(1)).await;
        Err::<&str, &str>("err")
    };

    // assign a timeout and a cancel handle to the original future
    let (fut_with_interrupts, cancel_handle) =
        fut.timeout(Duration::from_millis(3)).with_cancel_handle();

    // create a task that sleeps for 3 ms before canceling the original future
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(3)).await;
        // the cancel handle returns false - the future has already finished
        assert!(!cancel_handle.cancel());
    });

    // the future will return successfully
    assert!(matches!(fut_with_interrupts.await, Ok(Ok(Err("err")))));
}

#[tokio::main]
async fn main() {
    tokio::join!(
        timeout_cancel_timed_out(),
        timeout_cancel_canceled(),
        timeout_cancel_ok(),
        timeout_cancel_err(),
    );
}
