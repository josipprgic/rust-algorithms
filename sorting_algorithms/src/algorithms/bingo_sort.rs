pub fn sort<T: std::cmp::Ord + Copy>(col: &mut Vec<T>) -> &Vec<T> {
    let mut i = 0;
    loop {
        let bingo = smallest(&col[..], i);
        i += bingo_sort(col, bingo, i);

        if i == col.len() {
            break;
        }
    }

    col
}

fn bingo_sort<T: std::cmp::Ord>(col: &mut Vec<T>, bingo: T, start: usize) -> usize {
    let mut swap_count = 0;
    for i in start..col.len() {
        if col[i] == bingo {
            // Not swapping as this seems a bit easier with same output
            let elem = col.remove(i);
            col.insert(start, elem);
            swap_count += 1;
        }
    }
    
    swap_count
}

fn smallest<T: std::cmp::Ord + Copy>(col: &[T], start: usize) -> T {
    let mut min = &col[start];
    for i in start..col.len() {
        if col[i] < *min {
            min = &col[i];
        }
    }

    *min
}