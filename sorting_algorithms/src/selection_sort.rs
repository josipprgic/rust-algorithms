use log::debug;

pub fn sort<T: std::cmp::Ord + std::fmt::Debug>(mut collection: Vec<T>) -> Vec<T> {
    select_sort(&mut collection, 0);
    collection
}

fn select_sort<T: std::cmp::Ord + std::fmt::Debug>(collection: &mut Vec<T>, start: usize) {
    if start == collection.len() - 1 {
        return
    }

    debug!("Sorting pass #{}: (First {start} elements are sorted)", start + 1);

    let mut min = start;

    for i in start + 1..collection.len()  {
        if collection[i] < collection[min] {
            min = i;
        }
    }

    debug!("Smallest element of the unsorted subarray({:?}) is at index: {}", collection, min);

    if min != start {
        let elem = collection.remove(min);
        collection.insert(start, elem)
    }

    select_sort(collection, start + 1)
}