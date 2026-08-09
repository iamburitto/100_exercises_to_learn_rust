// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {


    // loop through 5 4 3 2 1 0 
    
    let end = n;
    let mut f = 1;
    for i in 1..(end + 1) {
        f = f*i;
    }
    f
    
    
    
    /*
    let end = 5;
    let mut sum = 0;

    for i in 1..(end + 1) {
        sum += i;
    }
     */
}
