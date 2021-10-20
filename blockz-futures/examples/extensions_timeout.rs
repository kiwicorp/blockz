//! Example usage of the timeout feature provided by the blockz future
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

    // assign a 1 ms timeout to the original future
    let fut_with_timeout = fut.timeout(Duration::from_millis(1));

    // the future will return with an error (a timeout error)
    assert!(matches!(fut_with_timeout.await, Err(_)));
    println!("main: future timed out and returned early");
}
