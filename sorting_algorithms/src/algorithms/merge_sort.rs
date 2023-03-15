// TODO Add doc
pub fn sort<T: Ord + Clone>(collection: Vec<T>) -> Vec<T> {
    return merge_sort(&collection[..]);
}

fn merge_sort<T: Ord + Clone>(collection: &[T]) -> Vec<T> {
    if collection.len() == 1 {
        return (*collection).to_vec()
    }

    let middle = collection.len() / 2;

    let mut first = merge_sort(&collection[0..middle]);
    let mut second = merge_sort(&collection[middle..]);
    let mut col = Vec::new();

    loop {
        loop {
            if second.len() == 0 || (first.len() != 0 && second[0] > first[0]) {
                break;
            }
            let el = second.remove(0);
            col.insert(col.len(), el);
        }

        if first.len() == 0 {
            break;
        }

        if second.len() == 0 || second[0] > first[0] {
            let el = first.remove(0);
            col.insert(col.len(), el)
        }
    }   

    return col;
}