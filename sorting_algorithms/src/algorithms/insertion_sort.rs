// Algorithm iterates through vector (or any other indexable collection) and tries to put the next element
// in the correct position on the "left part" of the array. Once a position is found where the element to the left 
// is smaller or equal and the right element is bigger or equal than the current one, we insert said element in between.
//
// Time complexity: O(n^2) -> For each element there is a loop that can run until the beginning of the array 
// (worst case; reverse ordered collection)
pub fn sort<T: Ord>(mut collection: Vec<T>) -> Vec<T> {
    insert_sort(&mut collection, 1);
    collection
}

// Steps are written as a tuple where first member is index from which to remove an element and the
// second is the index into which to insert the aforementioned element.
pub fn sorting_steps<T: Ord + Clone>(collection: Vec<T>) -> Vec<(usize, usize)> {
    let mut to_sort = collection.clone();
    let mut steps = Vec::new();

    show_steps(&mut to_sort, &mut steps, 1);
    steps
}

fn insert_sort<T: Ord>(collection: &mut Vec<T>, sorted_subarray_length: usize) {
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


fn show_steps<T: Ord>(collection: &mut Vec<T>, steps: &mut Vec<(usize, usize)>, sorted_subarray_length: usize) {
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
        collection.insert(min_index, elem);
        steps.push((sorted_subarray_length, min_index))
    }

    show_steps(collection, steps, sorted_subarray_length + 1)
}