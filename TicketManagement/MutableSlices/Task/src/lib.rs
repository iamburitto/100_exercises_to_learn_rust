// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

pub fn squared(slice: &mut [i32]) {
    slice.iter_mut().for_each(|item| *item *= *item);
}

// Version to use if chaining at the end of an iterator:
// pub fn squared(slice: &mut [i32]) {
//     slice.iter_mut().for_each(|item| *item = item.pow(2));
// }

// more embedded version
// pub fn squared(slice: &mut [i32]) {
//     for item in slice {
//         *item = item.saturating_pow(2)
//     }
// }

// pub fn squared2(slice: &mut [i32]) {
//     for item in slice {
//         *item = item.saturating_pow(2)
//     }
// }



