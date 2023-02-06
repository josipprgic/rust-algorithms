// TODO finish
pub fn sort(collection: Vec<isize>) -> Vec<isize> {
    let mut max = collection[0];
    let mut min = collection[0];

    for c in &collection {
        if *c > max {
            max = *c;
        }
        if *c < min {
            min = *c;
        }
    }

    max -= min;
    
    let mut pow = 0;
    for i in 1..collection.len() {
        if 2 ^ i > collection.len() {
            pow = i;
            break;
        }
    }

    let mut col = Vec::new();

    for c in &collection {
        col.push((c - min) as usize);
    }

    return radix_sort(&mut col[..], max as usize, pow).into_iter()
        .map(|i| i as isize + min)
        .collect::<Vec<isize>>();
}

fn radix_sort(collection: &mut [usize], max: usize, exp: usize) -> Vec<usize> {
    let mut col = Vec::new();
    let mut output = Vec::new();

    for i in collection {
        col.push(*i);
        output.push(0);
    }

    let pow = 1 << exp;
    let mut n = 0;
    for i in 0..max {
        if i * exp > max {
            n = i;
            break
        }
    }

    for j in 0..n {
        let mut count_arr = Vec::new();
        for _ in 0..max+1 {
            count_arr.push(0)
        }

        for i in 0..col.len() {
            count_arr[(col[i] >> (j * exp)) % pow] += 1;
        }

        for i in (0..col.len()).rev() {
            output[count_arr[(col[i] >> (j * exp)) % pow] - 1] = col[i];
            count_arr[(col[i] >> (j * exp)) % pow] -= 1;
        }

        println!("{:?}", col);
        col = output.clone()
    }

    return output;
}