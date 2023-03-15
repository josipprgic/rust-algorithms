
pub fn sort<T: Ord + Copy>(col: &mut Vec<T>) -> &Vec<T> {
    let mut gap = col.len() / 2;
    loop {
        shell_sort(col, gap);

        if gap <= 1 {
            break;
        }
        gap = gap / 2;
    }
    col
}

fn shell_sort<T: Ord + Copy>(col: &mut Vec<T>, gap: usize) {
    for i in gap..col.len() {
        for j in 0..(col.len() / gap) {
            if (j + 1) * gap > i {
                break;
            }
            let e = &col[i - j * gap];
            if e < &col[i - (j + 1) * gap] {
                swap(col, i - j * gap, i - (j + 1) * gap)
            } else {
                break;
            }
        }
    }
}

fn swap<T: Copy>(col: &mut Vec<T>, first: usize, second: usize) {
    let tmpf = col[first];
    let tmps = col[second];
    col.remove(first);
    col.insert(first, tmps);
    col.remove(second);
    col.insert(second, tmpf);
}