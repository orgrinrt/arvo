//! p2b: the family member that defeats every band below the shipped width.
//!
//! p2 defeats p2c's band 2..=5 with L_16, whose threshold is 15. A defender of
//! the mechanism answers: widen the band. This probe is the reply. L_64, the
//! product of 64 consecutive integers, has threshold v2(64!) = 63: it is TRUE
//! at every width 1 through 63 and FALSE exactly from width 64 up, with the
//! wrapping product at the witness x = 64 equal to 2^63 on the nose
//! (p1_threshold_family.rs pins the residue; the sweep confirms truth as far
//! as a sweep reaches).
//!
//! So for this law there is NO model band that can disagree with the constant-
//! true closed form: at every width except the shipped one the closed form is
//! RIGHT. The failure is not that the band was too narrow. The failure is that
//! band agreement carries no information about the gated width at all: for the
//! wrapping fragment, truth in width is an initial segment, agreement on
//! [2..B] states only that the threshold is at least B, and the family
//! realises a threshold at 63 as cheaply as at 3.
//!
//!   rustc --edition 2021 -O p2b_no_band_can_catch_it.rs -o p2b     (licenses)
//!   rustc --edition 2021 -O --cfg audit p2b_... .rs                (refused at 64)
//!
//! The const band here is 2..=8 (arity 1, ~32k const multiplies, comfortably
//! under the evaluator's default budget); the runtime sweep goes to width 22.
//! Toolchain: pinned nightly-2026-05-28. No feature gates.

const MODEL_MIN_W: u32 = 2;
const MODEL_MAX_W: u32 = 8;

const K: u64 = 64;

const fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

const fn falling_prod(x: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc: u64 = 1;
    let mut i: u64 = 0;
    while i < K {
        acc = acc.wrapping_mul(x.wrapping_sub(i) & m) & m;
        i += 1;
    }
    acc
}

const fn closed_verdict(_w: u32) -> bool {
    true
}

const fn swept_verdict(w: u32) -> bool {
    let n = 1u64 << w;
    let mut x = 0u64;
    while x < n {
        if falling_prod(x, w) != 0 {
            return false;
        }
        x += 1;
    }
    true
}

const fn closed_agrees_with_sweep() -> bool {
    let mut w = MODEL_MIN_W;
    while w <= MODEL_MAX_W {
        if closed_verdict(w) != swept_verdict(w) {
            return false;
        }
        w += 1;
    }
    true
}

const AGREEMENT: () = assert!(
    closed_agrees_with_sweep(),
    "closed form disagrees with the sweep inside the model band"
);

struct Numeral64;
trait Licensed {
    const CHECK: ();
}
impl Licensed for Numeral64 {
    const CHECK: () = {
        let () = AGREEMENT;
        assert!(closed_verdict(64), "law does not hold at this width");
    };
}
fn consumer<L: Licensed>() -> &'static str {
    let () = <L as Licensed>::CHECK;
    "licensed"
}

#[cfg(audit)]
const AUDIT: () = assert!(
    falling_prod(64, 64) == 0,
    "the licensed law is FALSE at the gated width: 64! = 2^63 * odd, and the \
     wrapping product at x = 64 is exactly 1 << 63, nonzero mod 2^64"
);

fn main() {
    println!("p2b: L_64 licensed at width 64 by a band no width below 64 can improve");
    println!(
        "  const model band: widths {}..={}",
        MODEL_MIN_W, MODEL_MAX_W
    );
    println!(
        "  agreement over the band, compile time: {}",
        const { closed_agrees_with_sweep() }
    );
    println!("  consumer::<Numeral64>() = {}", consumer::<Numeral64>());
    // Runtime: sweep as wide as affordable, and pin the witness residue.
    let mut false_widths = 0u32;
    for w in 1..=22u32 {
        let n = 1u64 << w;
        let mut x = 0u64;
        let mut bad = false;
        while x < n {
            if falling_prod(x, w) != 0 {
                bad = true;
                break;
            }
            x += 1;
        }
        if bad {
            false_widths += 1;
        }
    }
    println!(
        "  exhaustive sweep, widths 1..=22: {} false widths (theory: true through 63)",
        false_widths
    );
    println!(
        "  wrapping product at x = 64, width 64: {:#x} (nonzero, so the law fails at 64)",
        falling_prod(64, 64)
    );
    assert!(false_widths == 0);
    assert!(falling_prod(64, 64) == 1u64 << 63);
}
