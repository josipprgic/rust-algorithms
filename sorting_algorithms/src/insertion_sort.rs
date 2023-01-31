// use log::debug;

pub fn sort<T: std::cmp::Ord>(mut collection: Vec<T>) -> Vec<T> {
    insert_sort(&mut collection, 1);
    collection
}

fn insert_sort<T: std::cmp::Ord>(collection: &mut Vec<T>, sorted_subarray_length: usize) {
    if sorted_subarray_length == collection.len() {
        return
    }

    // debug!("Sorting pass #{sorted_subarray_length}: (First {sorted_subarray_length} elements are sorted); inserting elem: {:?}", collection[sorted_subarray_length]);

    let mut min_index: usize = sorted_subarray_length;
    for i in (0..sorted_subarray_length).rev()  {
        if collection[i] > collection[sorted_subarray_length] {
            min_index = i
        }
    }

    if min_index != sorted_subarray_length {
        // debug!("Smallest element of the sorted subarray({:?}) that can be switched is at index: {}", collection, min_index);
        let elem = collection.remove(sorted_subarray_length);
        collection.insert(min_index, elem)
    }

    insert_sort(collection, sorted_subarray_length + 1)
}