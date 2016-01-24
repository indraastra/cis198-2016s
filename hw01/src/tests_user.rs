#![cfg(test)]

//use problem1::{sum, dedup, filter};
//use problem2::mat_mult;
//use problem3::sieve;
use problem4::{hanoi, Peg};

#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![
        (Peg::A, Peg::C),
        (Peg::A, Peg::B),
        (Peg::C, Peg::B),
        (Peg::A, Peg::C),
        (Peg::B, Peg::A),
        (Peg::B, Peg::C),
        (Peg::A, Peg::C)
    ], result);
    assert_eq!(7, result.len());
}

#[test]
fn test_hanoi_4_disks() {
    let result = hanoi(4, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![
        (Peg::A, Peg::B),
        (Peg::A, Peg::C),
        (Peg::B, Peg::C),
        (Peg::A, Peg::B),
        (Peg::C, Peg::A),
        (Peg::C, Peg::B),
        (Peg::A, Peg::B),
        (Peg::A, Peg::C),
        (Peg::B, Peg::C),
        (Peg::B, Peg::A),
        (Peg::C, Peg::A),
        (Peg::B, Peg::C),
        (Peg::A, Peg::B),
        (Peg::A, Peg::C),
        (Peg::B, Peg::C)
    ], result);
}
