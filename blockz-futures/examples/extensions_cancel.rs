//! Example usage of the cancel feature provided by the blockz future
//! extensions.

use std::time::Duration;

use blockz_futures::BlockzFutureExt;

#[tokio::main]
async fn main() {
    // dummy future that waits for 2 ms
    let fut = async {
        println!("future: start sleeping 2 ms");
        tokio::time::sleep(Duration::from_millis(2)).await;
        println!("future: slept 2 ms");
    };

    // create a cancel handle for the original future
    let (fut_with_cancel, cancel_handle) = fut.with_cancel_handle();

    // create a task that sleeps for 1 ms before canceling the original future
    tokio::spawn(async move {
        println!("task: start sleeping 1 ms");
        tokio::time::sleep(Duration::from_millis(1)).await;
        println!("task: slept 1 ms");

        // cancel the future
        // this returns true because the original future has not completed when we
        // canceled it
        assert!(cancel_handle.cancel());
    });

    // the future will return with an error (a cancel error)
    assert!(matches!(fut_with_cancel.await, Err(_)));
    println!("main: future canceled");
}
