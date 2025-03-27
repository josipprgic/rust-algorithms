// Iterates through the vector and checks adjacent elements' order; if they
// aren't in correct order swap them. Continue looping until there are no swaps.
//
// time complexity: O(n^2) (complexity is optimized by stopping the search if no swaps occurr)
pub fn sort<T: Ord>(collection: Vec<T>) -> Vec<T> {
    bubble_sort(collection)
}

fn bubble_sort<T: Ord>(mut collection: Vec<T>) -> Vec<T> {
    let mut has_swapped = false;

    loop {
        for i in 0..collection.len() - 1 {
            if collection[i] > collection[i + 1] {
                let element = collection.remove(i);
                collection.insert(i + 1, element);
                has_swapped = true
            }
        }

        if !has_swapped {
            break;
        }

        has_swapped = false
    } 

    collection
}