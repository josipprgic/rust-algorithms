//TODO add doc
pub fn sort<T: std::cmp::Ord + Copy>(collection: Vec<T>) -> Vec<T> {
    let mut col = collection.clone();
    quick_sort(&mut col[..]).to_vec()
}

fn quick_sort<T: std::cmp::Ord + Copy>(collection: &mut [T]) -> Vec<T> {
    
    let len = collection.len();
    if len <= 1 {
        return collection.to_vec()
    }

    let mut col = collection;
    let pivot = &col[len - 1].clone();
    let mut last_replaced:isize = -1;

    for i in 0..len-1 {
        println!("{:?} {:?}", pivot, col[i]);
        if pivot > &col[i] {
            last_replaced += 1;
            col = swap(col, i, last_replaced as usize);
        }
    }

    last_replaced += 1;
    col = swap(col, len - 1, last_replaced as usize);

    let mut result = Vec::new();
    if last_replaced > -1 {
        let res_left = quick_sort(&mut col[0..(last_replaced as usize)]);
        for c in res_left {
            result.push(c);
        }
    }
    result.push(*pivot);
    let res_right = quick_sort(&mut col[((last_replaced + 1) as usize)..len]);
    for c in res_right {
        result.push(c);
    }

    return result;
}

fn swap<T: Copy>(collection: &mut [T], i: usize, j: usize) -> &mut [T] {
    if i != j {
        let elem = collection[i];
        collection[i] = collection[j];
        collection[j] = elem;
    }
    collection
}