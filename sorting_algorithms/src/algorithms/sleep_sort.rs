use std::{thread, sync::Mutex, sync::Arc};
use core::time::Duration;

pub fn sort(collection: Vec<i32>) -> Vec<i32> {
    let mut min = collection[0];
    for i in &collection {
        if i < &min {
            min = *i;
        }
    }

    let col = collection.into_iter().map(|n| n - min).collect::<Vec<i32>>();

    sleep_sort(col).into_iter().map(|n| n + min).collect::<Vec<i32>>()
}

fn sleep_sort(col: Vec<i32>) -> Vec<i32> {
    let mut threads = Vec::new();
    let tmp = Vec::new();
    let m = Arc::new(Mutex::new(tmp));
    for i in col {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_micros((i * 100) as u64));
            let mut n = m.lock().unwrap();
            n.push(i);
        });
        threads.push(handle)
    }

    for handle in threads {
        handle.join().unwrap();
    }
    
    let n = m.lock().unwrap();

    n.to_vec()
}