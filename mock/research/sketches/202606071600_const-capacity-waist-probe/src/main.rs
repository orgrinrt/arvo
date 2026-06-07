//! Sketch (GATE-2 chart R0 probe): can a `const fn` generic over a
//! `Capacity`-style GAT trait construct + index `C::Array<T>` in const context,
//! AND call `[const]` bit-contract methods on a row type, on the pinned nightly?
//!
//! Context: the GATE-2 corrected dispatch needs an ADDITIVE const
//! `waist_detect_const` in arvo-graph, generic over `C: Capacity` (per op +
//! numeric-position-convention #649: the `Capacity` type-dispatch form is
//! GCE-free, native `[T; cap_size(N)]` arrays explode the trait solver / ICE).
//! The blocker (arvo source): `Capacity::filled` and `C::Array<T>: AsRef/AsMut`
//! are NOT const, so a const fn cannot construct/index the GAT today. arvo's bit
//! CONTRACTS are already `pub const trait` and `clear_lowest_set_bit`'s default
//! method already calls `[const] BitAccess + [const] BitSequence` from a
//! const-trait context, so const-trait-calling-const-trait inside a const fn is
//! largely proven. The genuinely unproven piece (U2) is const construct + index
//! of a `C::Array<T>` GAT through a const trait.
//!
//! This probe prototypes the MINIMAL additive const surface a `Capacity`-style
//! trait needs (const `filled` + const index get/set, NO AsRef/AsMut), mirrors
//! arvo's exact `const trait` / `[const]` / `impl const` syntax, and writes a
//! const fn that does the waist algorithm's core moves (build a depth array,
//! index it, scan a row's set bits via const methods). Success here means R1a
//! (add the same additive const methods to the real `Capacity` in arvo-tensor)
//! and R1b (port `waist_detect` to a const fn over it) are mechanically sound.
//!
//! Outcome at the bottom + in FINDINGS.md.

#![allow(dead_code)]
#![feature(const_trait_impl)]

// ---- Minimal Capacity-style GAT trait with an ADDITIVE const surface. ----
// Mirrors arvo-tensor `Capacity` (type Array<T>; const CAP; fn filled), but the
// access goes through const inherent methods instead of the non-const std
// AsRef/AsMut, which is exactly the additive surface R1a must add.
const trait ConstCap {
    type Array<T: Copy>: Copy;
    const N: usize;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
    fn get<T: Copy>(a: &Self::Array<T>, i: usize) -> T;
    fn set<T: Copy>(a: &mut Self::Array<T>, i: usize, v: T);
}

// One generic marker, any exact N (the `Dim<const N>` shape).
struct Dim<const N: usize>;

impl<const N: usize> const ConstCap for Dim<N> {
    type Array<T: Copy> = [T; N];
    const N: usize = N;
    #[inline]
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }
    #[inline]
    fn get<T: Copy>(a: &[T; N], i: usize) -> T {
        a[i]
    }
    #[inline]
    fn set<T: Copy>(a: &mut [T; N], i: usize, v: T) {
        a[i] = v;
    }
}

// ---- Minimal bit-row const trait, mirroring arvo BitAccess/BitSequence. ----
const trait Row: Copy {
    fn trailing_zeros(self) -> usize;
    fn with_bit_cleared(self, idx: usize) -> Self;
    fn is_zero(self) -> bool;
    fn bit(self, idx: usize) -> bool;
}

#[derive(Copy, Clone)]
struct W(u64);

impl const Row for W {
    #[inline]
    fn trailing_zeros(self) -> usize {
        self.0.trailing_zeros() as usize
    }
    #[inline]
    fn with_bit_cleared(self, idx: usize) -> Self {
        W(self.0 & !(1u64 << idx))
    }
    #[inline]
    fn is_zero(self) -> bool {
        self.0 == 0
    }
    #[inline]
    fn bit(self, idx: usize) -> bool {
        (self.0 >> idx) & 1 == 1
    }
}

// ---- The probe: a const fn generic over the Capacity GAT + the row contract. ----
// Does the core of what waist_detect_const needs: construct a `C::Array<usize>`
// via the const trait, write/read it by index, and scan each row's set bits via
// the `[const] Row` methods in a while-loop (the `iter_set_bits` replacement).
// Returns, per node, the count of its predecessors (a stand-in for the depth /
// level-width passes), proving const array build + const-trait scan compose.
const fn pred_counts<C: [const] ConstCap, R: [const] Row>(adj: &C::Array<R>) -> C::Array<usize> {
    let mut out = C::filled(0usize);
    let mut i = 0;
    while i < C::N {
        let row = C::get(adj, i);
        // Scan set bits via const Row methods (the iterator-free const path).
        let mut r = row;
        let mut count = 0;
        while !R::is_zero(r) {
            let _b = R::trailing_zeros(r);
            r = R::with_bit_cleared(r, R::trailing_zeros(r));
            count += 1;
        }
        C::set(&mut out, i, count);
        i += 1;
    }
    out
}

// Force const evaluation: a 4-node adjacency. Node 0: preds {}, node 1: pred {0},
// node 2: preds {0,1}, node 3: pred {2}. Expected pred counts [0,1,2,1].
const ADJ: [W; 4] = [W(0b0000), W(0b0001), W(0b0011), W(0b0100)];
const COUNTS: [usize; 4] = pred_counts::<Dim<4>, W>(&ADJ);

fn main() {
    // Runtime echo of the const-evaluated result.
    println!("const pred_counts = {:?}", COUNTS);
    assert_eq!(COUNTS, [0, 1, 2, 1], "const eval over the Capacity GAT + const Row scan");

    // Also confirm it const-evaluates at a different N (no per-N GCE blowup).
    const ADJ8: [W; 8] = [
        W(0), W(1), W(0b11), W(0b100), W(0b1000), W(0), W(0b110000), W(0b1000000),
    ];
    const C8: [usize; 8] = pred_counts::<Dim<8>, W>(&ADJ8);
    println!("const pred_counts N=8 = {:?}", C8);
    assert_eq!(C8, [0, 1, 2, 1, 1, 0, 2, 1]);

    println!("R0 PROBE: WORKS");
}
