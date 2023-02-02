// Using log and env_logger crates to log some info. Set env variable RUST_LOG=debug for
// for more detailed output (on unix run export RUST_LOG=debug before running the app). 
// Use log level info for less details
use log::info;

pub mod algorithms;

use algorithms::{selection_sort, insertion_sort, bubble_sort, merge_sort};

fn main() {
    env_logger::init();

    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    info!("{:?}", selection_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    info!("{:?}", insertion_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", bubble_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", merge_sort::sort(v));
}
