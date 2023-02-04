pub fn swap<T: Copy>(collection: &mut [T], i: usize, j: usize) -> &mut [T] {
    if i != j {
        let elem = collection[i];
        collection[i] = collection[j];
        collection[j] = elem;
    }
    collection
}