use crate::util::swap;

pub fn sort<T: Ord + Copy>(collection: Vec<T>) -> Vec<T> {
    let mut col = collection.to_owned();
    heap_sort(&mut col)
}

fn heap_sort<T: Ord + Copy>(collection: &mut [T]) -> Vec<T> {

    let mut col = collection;
    for i in (0..col.len() / 2).rev() {
        col = heapify(col, col.len(), i)
    }
 
    for i in (0..col.len()).rev() {
        col = swap(col, 0, i);
        col = heapify(col, i, 0);
    }

    col.to_vec()
}

fn heapify<T: Ord + Copy>(col: &mut [T], n: usize, i: usize) -> &mut [T] {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut col_cp = col;

    if left < n && col_cp[left] > col_cp[largest] {
        largest = left;
    }
 
    if right < n && col_cp[right] > col_cp[largest] {
        largest = right;
    }

    if largest != i {
        col_cp = swap(col_cp, i, largest);
        col_cp = heapify(col_cp, n, largest);
    }
    col_cp
}