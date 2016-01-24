/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut is_prime: Vec<bool> = vec![true; n as usize];
    let mut res: Vec<u32> = Vec::new();
    for i in 2..n {
        if is_prime[i as usize] {
            res.push(i);
            for mult in 2..(n/i) {
                is_prime[(i * mult) as usize] = false;
            }
        }
    }

    res
}
