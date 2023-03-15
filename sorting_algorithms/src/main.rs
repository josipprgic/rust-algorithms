// Using log and env_logger crates to log some info. Set env variable RUST_LOG=debug for
// for more detailed output (on unix run export RUST_LOG=debug before running the app). 
// Use log level info for less details
use log::info;

mod algorithms;
mod util;

use algorithms::{selection_sort, insertion_sort, bubble_sort, merge_sort,
    quick_sort, heap_sort, counting_sort, radix_sort, sleep_sort, bingo_sort, shell_sort};

fn main() {
    env_logger::init();

    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    info!("{:?}", selection_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", insertion_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", bubble_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", merge_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", quick_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    println!("{:?}", heap_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111, -111, 32, 1, 1, 1];
    println!("{:?}", counting_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111, -111, 32, 1, 1, 1];
    println!("{:?}", radix_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111, -111, 32, 1, 1, 1];
    println!("{:?}", sleep_sort::sort(v));
    let mut v = vec![1, 32, -2, 23, 3, 5, 0, -111, -111, 32, 1, 1, 1];
    println!("{:?}", bingo_sort::sort(&mut v));
    let mut v = vec![1, 321, -2, -23, 3, 15, 0, -111, -111, 32, 12, 1, 1];
    println!("{:?}", shell_sort::sort(&mut v));

    let mut v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    let steps = insertion_sort::sorting_steps(v.clone());
    println!("{:?} {:?}", v, steps);

    for s in steps {
        let elem = v.remove(s.0);
        v.insert(s.1, elem);
        println!("{:?}", v)
    }
}
