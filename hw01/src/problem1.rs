/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for v in slice {
        total += *v;
    }
    total
}

/// Deduplicate items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, in order.
pub fn dedup(vector: &Vec<i32>) -> Vec<i32> {
    // TODO
    let mut res: Vec<i32> = Vec::new();
    for i in vector {
        if !res.contains(i) {
            res.push(*i);
        }
    }
    res
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    // TODO
    let mut res: Vec<i32> = Vec::new();
    for i in vs {
        if pred(*i) {
            res.push(*i);
        }
    }
    res
}

