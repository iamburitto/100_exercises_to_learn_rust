/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
pub fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        12
    } else if n % 3 == 0 {
        13
    }
    else {
        17
    }
}

// let result = if true == false {
// "oh no"
// } else if "something" == "other thing" {
// "oh dear"
// } else if let Some(200) = "blarg".parse::<i32>().ok() {
// "uh oh"
// } else {
// println!("Sneaky side effect.");
// "phew, nothing's broken"
// };