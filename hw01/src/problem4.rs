/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
/// The algorithm uses a simple dynamic programming approach based
/// on the observation that moving N pegs from SRC to DST can be
/// broken down into moving N-1 pegs from SRC to AUX, then moving
/// the remaining (largest) peg to DST, and finally moving the
/// previously moved N-1 pegs again from AUX to DST.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    if num_discs == 1 {
        return vec![(src, dst)];
    }
    // Move n-1 discs from src to aux.
    let mut moves = hanoi(num_discs - 1, src, dst, aux);
    // Move nth disc to dst.
    moves.push((src, dst));
    // Move n-1 discs from aux to dst.
    moves.append(&mut hanoi(num_discs - 1, aux, src, dst));
    moves
}

