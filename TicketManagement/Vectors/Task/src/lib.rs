// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.

    let mut cache = vec![Some(0), Some(1)];
    return memoize(n, &mut cache);

    fn memoize(n: u32, cache: &mut Vec<Option<u32>>) -> u32 {
        if let Some(Some(answer)) = cache.get(n as usize) {
            return *answer
        }
        let answer = memoize(n-1, cache) + memoize(n-2, cache);
        cache.push(Some(answer));
        answer
    }
}
