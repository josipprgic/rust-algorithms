use log::info;

pub mod algorithms;

use algorithms::{selection_sort, insertion_sort};

fn main() {
    env_logger::init();

    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    info!("{:?}", selection_sort::sort(v));
    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];
    info!("{:?}", insertion_sort::sort(v))

}
