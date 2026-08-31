// `196` section 2.3 is right that `35`'s arm E compiles, that `35`'s own section
// title carries the disjunct `191` dropped, and that `191`'s paraphrase of
// "closed" was unquantified. I concede all three in `199` section 1.
//
// This probe asks the question the concession leaves open: **does arm E answer
// the question?** `196` reads it as showing a runtime-length fold may widen, and
// therefore that the count's staticness is not an axis. Arm E's own comment in
// `35_probes/p1_fold_cannot_widen.rs:255-257` says something narrower:
//
//   "The accumulator width is chosen by whoever writes this signature, not
//    derived from the element width."
//
// Chosen, not derived. So the shape compiles at any slice length and is correct
// only up to the length its author guessed. This probe measures the gap between
// those two, which is the whole of what `191`'s axis was reaching for under the
// wrong name.
//
// ARMS. Two must succeed and two must fail, and the failures are the result.
//   A  arm E's shape over a slice inside the accumulator's headroom
//                                             MUST COMPILE and MUST be correct
//   B  the same code, same types, longer slice
//                                             MUST COMPILE and MUST be WRONG
//   C  the capacity-derived shape at a capacity the accumulator covers
//                                             MUST COMPILE and MUST be correct
//   D  the capacity-derived shape at a capacity it does not cover
//                                             MUST be REFUSED at compile time
//
// B is the case that has to fail for anything here to count. If B computes the
// right answer the probe has measured nothing, because then the hand-chosen
// accumulator was wide enough all along and there is no gap to talk about.
//
// Modelled in miniature the way `35_probes/p1` models the width algebra: `Num<W>`
// carries a W-bit unsigned value in a u32 and masks on every add, so overflow is
// the numeral's overflow rather than the host's.
#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Num<const W: u32>(u32);

impl<const W: u32> Num<W> {
    pub const MASK: u32 = if W >= 32 { u32::MAX } else { (1u32 << W) - 1 };
    pub fn new(v: u32) -> Self {
        Num(v & Self::MASK)
    }
    // Wrapping inside the declared width. Any policy would do; what matters is
    // that the value cannot silently use bits the declared width does not have.
    pub fn cadd(self, o: Self) -> Self {
        Num((self.0.wrapping_add(o.0)) & Self::MASK)
    }
    pub fn get(self) -> u32 {
        self.0
    }
}

pub trait WidenInto<A> {
    fn widen_into(self) -> A;
}
impl<const A: u32, const B: u32> WidenInto<Num<B>> for Num<A> {
    fn widen_into(self) -> Num<B> {
        Num(self.0)
    }
}

// --- arm E, transcribed from `35_probes/p1_fold_cannot_widen.rs:270-276` ------
// 4-bit elements into an 8-bit accumulator. The 8 is a choice, not a derivation.
pub fn arm_e(xs: &[Num<4>]) -> Num<8> {
    let mut acc: Num<8> = Num::new(0);
    for x in xs {
        let widened: Num<8> = (*x).widen_into();
        acc = acc.cadd(widened);
    }
    acc
}

// --- the capacity-derived shape, `35` section 3.2's acc_width(W, C) ----------
// Type-level capacity, so the bound is a type and the derivation is a relation
// the trait solver decides. `191_probes/p2` established this shape refuses at
// the definition site; here it only has to refuse at all.
pub struct Cap<const C: usize>;

pub const fn ceil_log2(c: usize) -> u32 {
    let mut n = 1usize;
    let mut k = 0u32;
    while n < c {
        n *= 2;
        k += 1;
    }
    k
}

pub struct Fold<const W: u32, const C: usize, const ACC: u32>;
impl<const W: u32, const C: usize, const ACC: u32> Fold<W, C, ACC> {
    // Sufficiency, per 35 section 3.2: a sum of at most C values each below 2^W
    // is below 2^(W + ceil(log2 C)).
    const SUFFICIENT: () = assert!(
        ACC >= W + ceil_log2(C),
        "accumulator too narrow for this capacity"
    );
    pub fn sum(xs: &[Num<W>]) -> Num<ACC> {
        let _ = Self::SUFFICIENT;
        assert!(xs.len() <= C, "more elements than the declared capacity");
        let mut acc: Num<ACC> = Num::new(0);
        for x in xs {
            let w: Num<ACC> = (*x).widen_into();
            acc = acc.cadd(w);
        }
        acc
    }
}

#[cfg(arm_d)]
pub fn arm_d() -> Num<8> {
    // capacity 64 with 4-bit elements needs 4 + 6 = 10 bits. 8 is not enough.
    let xs = [Num::<4>::new(15); 64];
    Fold::<4, 64, 8>::sum(&xs)
}

fn exact(xs: &[Num<4>]) -> u32 {
    xs.iter().map(|x| x.get()).sum()
}

fn main() {
    // ARM A. 16 elements of value 15 sum to 240, which fits in 8 bits.
    let short = [Num::<4>::new(15); 16];
    let got_a = arm_e(&short).get();
    let want_a = exact(&short);
    println!(
        "ARM A  arm E, len 16   got={got_a} exact={want_a}  {}",
        if got_a == want_a {
            "correct     (required: correct)"
        } else {
            "WRONG   *** NOT AS REQUIRED ***"
        }
    );

    // ARM B. 32 elements of value 15 sum to 480, which does not.
    // Same function, same types, nothing recompiled. Only the length moved.
    let long = [Num::<4>::new(15); 32];
    let got_b = arm_e(&long).get();
    let want_b = exact(&long);
    println!(
        "ARM B  arm E, len 32   got={got_b} exact={want_b}  {}",
        if got_b != want_b {
            "WRONG       (required: wrong)"
        } else {
            "correct *** NOT AS REQUIRED, the probe measured nothing ***"
        }
    );

    // ARM C. The derived shape at a capacity the accumulator covers:
    // 4-bit elements, capacity 32, needs 4 + 5 = 9 bits, declared 9.
    let got_c = Fold::<4, 32, 9>::sum(&long).get();
    println!(
        "ARM C  derived, cap 32 got={got_c} exact={want_b}  {}",
        if got_c == want_b {
            "correct     (required: correct)"
        } else {
            "WRONG   *** NOT AS REQUIRED ***"
        }
    );

    #[cfg(arm_d)]
    {
        // `191` section 2.5 measured that an inherent associated const is
        // evaluated only where it is forced. The first run of this probe
        // reproduced that defect on itself: arm D was `pub` and unreferenced,
        // and it compiled. Calling it is what forces `SUFFICIENT`.
        println!("ARM D returned {}", arm_d().get());
    }
    println!();
    println!("ARM D is a separate compile, under --cfg arm_d, and must be REFUSED.");
}
