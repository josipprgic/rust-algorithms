// Algorithm iterates through vector (or any other indexable collection) and tries to find the smallest element
// that's "to the right" of the current element (has bigger index). Once such an element is found it is inserted
// into the index from which we started finding the smallest element.
//
// Time complexity: O(n^2) -> For each element there is a loop that runs until the end of array
pub fn sort<T: Ord>(mut collection: Vec<T>) -> Vec<T> {
    select_sort(&mut collection, 0);
    collection
}

fn select_sort<T: Ord>(collection: &mut Vec<T>, sorted_subarray_length: usize) {
    if sorted_subarray_length == collection.len() - 1 {
        return;
    }

    //debug!("Sorting pass #{}: (First {sorted_subarray_length} elements are sorted)", sorted_subarray_length + 1);

    let mut min = sorted_subarray_length;
    for i in sorted_subarray_length + 1..collection.len() {
        if collection[i] < collection[min] {
            min = i;
        }
    }

    //debug!("Smallest element of the unsorted subarray({:?}) is at index: {}", collection, min);

    if min != sorted_subarray_length {
        let elem = collection.remove(min);
        collection.insert(sorted_subarray_length, elem)
    }

    select_sort(collection, sorted_subarray_length + 1)
}
