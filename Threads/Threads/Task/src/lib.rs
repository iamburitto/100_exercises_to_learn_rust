// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // Split the vector into two halves
    let mid = v.len() / 2;
    let first_half : Vec<i32> = v[..mid].iter().copied().collect();
    let second_half : Vec<i32>= v[mid..].iter().copied().collect();

    // use spawn and join to sum each half in a separate thread

    let handle1 = thread::spawn( move || {
        first_half.iter().sum::<i32>()
    });

    let handle2 = thread::spawn( move || {
        second_half.iter().sum::<i32>()
    });

    let sum = handle1.join().unwrap() + handle2.join().unwrap();

    sum
}


