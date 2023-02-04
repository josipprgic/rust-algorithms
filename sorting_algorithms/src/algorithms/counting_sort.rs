// Works only with isize type since the numbers in collection are used to index
// temporary array
pub fn sort(collection: Vec<isize>) -> Vec<isize> {
    if collection.len() == 0 {
        return collection;
    }

    let mut min = collection[0];
    let mut max = collection[0];

    for i in 0..collection.len() {
        if collection[i] < min {
            min = collection[i];
        }
        if collection[i] > max {
            max = collection[i]
        }
    }

    if max - min > 200 {
        // At high enough range this algorithm will allocate too big counting array 
        // and may cause memory problems on host computer.
        panic!("Difference between min and max values too big. Use different sort");
    }

    counting_sort(collection, min, max)
}

fn counting_sort(collection: Vec<isize>, min: isize, max: isize) -> Vec<isize> {
    let mut count_arr = Vec::new(); 
    for _ in 0..max-min+1 {
        count_arr.push(0)
    }

    let mut col: Vec<isize> = Vec::new();

    for i in 0..collection.len() {
        count_arr[(collection[i] - min) as usize] += 1;
    }

    for i in 0..count_arr.len() {
        let e = count_arr[i];

        for _ in 0..e {
            col.push(min + i as isize)
        }
    }

    return col;
}