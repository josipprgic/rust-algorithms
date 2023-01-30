use log::{info};

mod selection_sort;

fn main() {
    env_logger::init();

    let v = vec![1, 32, -2, 23, 3, 5, 0, -111];

    info!("{:?}", selection_sort::sort(v))
}
