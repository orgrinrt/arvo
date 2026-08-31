// Probe 2 for seat 215. An ad-hoc quick spike, NOT a bench.
//
// It is called that everywhere it is cited. It can establish that a lowering
// did or did not happen, by reading the emitted assembly, and it cannot price
// anything: no timing, no harness, no competitor arms, no artifact trail of the
// kind `mock/benches/` produces. Any sentence of the form "how much" is outside
// what this file can support.
//
// The question. Row 3 proposes that the law layer's business is whether a law
// reaches a lowering the backend cannot prove. Whether a backend can prove a
// given law is not a matter of opinion, so this asks it.
//
// Three pairs, each two spellings of one computation that a law would let a
// compiler interchange:
//
//   1. wrapping add, left- against right-associated. Wrapping addition is
//      associative and LLVM knows it, so the two should emit the same thing.
//      This is the control: if these two DIFFER, the instrument is measuring
//      spelling rather than reasoning and everything below it is void.
//
//   2. unsigned saturating add, left- against right-associated. Probe 1 proves
//      exhaustively that this operation IS associative at 8 bits. So this is a
//      TRUE law, and the question is whether the backend has it.
//
//   3. a saturating reduction over a slice. Vectorising a reduction requires
//      reassociating it. aarch64 has `uqadd` and can saturate eight or sixteen
//      lanes at once, so if the backend had the law the vector form is
//      available to it. Grep the output for vector registers to find out.
//
// Build and read:
//   rustc -O --emit asm -C opt-level=3 p2_what_the_backend_already_knows.rs
//
// Every function is `#[no_mangle] #[inline(never)]` so the symbols survive and
// can be found in the listing.

#![crate_type = "lib"]

// -- 1. wrapping add, the control --------------------------------------------

#[no_mangle]
#[inline(never)]
pub fn wrap_left(a: u8, b: u8, c: u8, d: u8) -> u8 {
    a.wrapping_add(b).wrapping_add(c).wrapping_add(d)
}

#[no_mangle]
#[inline(never)]
pub fn wrap_right(a: u8, b: u8, c: u8, d: u8) -> u8 {
    a.wrapping_add(b.wrapping_add(c.wrapping_add(d)))
}

// -- 2. unsigned saturating add, a law probe 1 proved and the backend may lack -

#[no_mangle]
#[inline(never)]
pub fn sat_left(a: u8, b: u8, c: u8, d: u8) -> u8 {
    a.saturating_add(b).saturating_add(c).saturating_add(d)
}

#[no_mangle]
#[inline(never)]
pub fn sat_right(a: u8, b: u8, c: u8, d: u8) -> u8 {
    a.saturating_add(b.saturating_add(c.saturating_add(d)))
}

// -- 3. reductions, where reassociation is what buys the vector form ----------

#[no_mangle]
#[inline(never)]
pub fn wrap_reduce(xs: &[u8; 256]) -> u8 {
    let mut acc = 0u8;
    let mut i = 0;
    while i < 256 {
        acc = acc.wrapping_add(xs[i]);
        i += 1;
    }
    acc
}

#[no_mangle]
#[inline(never)]
pub fn sat_reduce(xs: &[u8; 256]) -> u8 {
    let mut acc = 0u8;
    let mut i = 0;
    while i < 256 {
        acc = acc.saturating_add(xs[i]);
        i += 1;
    }
    acc
}

// The same saturating reduction written as an explicit tree, which is what a
// law layer holding the associativity result would be licensed to emit. If the
// backend vectorises THIS and not `sat_reduce`, then the law is exactly the
// missing piece and supplying it is the whole of the win.
#[no_mangle]
#[inline(never)]
pub fn sat_reduce_tree(xs: &[u8; 256]) -> u8 {
    let mut lanes = [0u8; 16];
    let mut i = 0;
    while i < 256 {
        let mut l = 0;
        while l < 16 {
            lanes[l] = lanes[l].saturating_add(xs[i + l]);
            l += 1;
        }
        i += 16;
    }
    let mut acc = 0u8;
    let mut l = 0;
    while l < 16 {
        acc = acc.saturating_add(lanes[l]);
        l += 1;
    }
    acc
}
