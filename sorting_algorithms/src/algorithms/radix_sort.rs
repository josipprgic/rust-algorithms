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
    
    let pow = collection.len().next_power_of_two();

    let mut col = Vec::new();

    for c in &collection {
        col.push((c - min) as usize);
    }

    return radix_sort(&mut col[..], max as usize, pow).into_iter()
        .map(|i| i as isize + min)
        .collect::<Vec<isize>>();
}

fn radix_sort(collection: &mut [usize], max: usize, pow: usize) -> Vec<usize> {
    let mut col = Vec::new();

    for i in collection {
        col.push(*i);
    }

    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % pow;

        let mut count_arr = vec![0; pow];
        for &x in col.iter() {
            count_arr[digit_of(x)] += 1;
        }
        for i in 1..pow {
            count_arr[i] += count_arr[i - 1];
        }

        for &x in col.to_owned().iter().rev() {
            count_arr[digit_of(x)] -= 1;
            col[count_arr[digit_of(x)]] = x;
        }

        place *= pow;
    }

    return col;
}