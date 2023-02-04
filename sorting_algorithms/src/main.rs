// Using log and env_logger crates to log some info. Set env variable RUST_LOG=debug for
// for more detailed output (on unix run export RUST_LOG=debug before running the app). 
// Use log level info for less details
use log::info;

mod algorithms;
mod util;

use algorithms::{selection_sort, insertion_sort, bubble_sort, merge_sort,
    quick_sort, heap_sort};

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
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", quick_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", heap_sort::sort(v));
}
