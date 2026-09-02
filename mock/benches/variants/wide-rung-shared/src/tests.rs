//! The checks that have to hold before any number this crate produces means
//! anything.
//!
//! Two of them exist specifically because `20`'s audit found the previous
//! bench family failing them, and both are asserted over the **whole** declared
//! key list rather than a chosen subset. Choosing which keys to check is
//! choosing what not to find out, and in the case that produced this file the
//! unchecked region was the one that was broken.

use super::*;
use crate::column::{build_bytes, key_d, key_n, key_w, REGION_BYTES};
use crate::shape::{rag_bytes, wr_bytes, SWEPT_WIDTHS};

/// Builds an input for a key and views it as a `Column`.
fn built(key: usize, seed: u64) -> (std::vec::Vec<u8>, *const Column) {
    let bytes = build_bytes(key, seed);
    let p = bytes.as_ptr() as *const Column;
    (bytes, p)
}

fn all_arms(key: usize, col: &Column) -> [(&'static str, [u64; 4]); 5] {
    [
        ("ragged", arms::ragged(key, col)),
        ("ragged-overread", arms::ragged_overread(key, col)),
        ("wordround", arms::wordround(key, col)),
        ("wordround-alias", arms::wordround_alias(key, col)),
        ("align16", arms::align16(key, col)),
    ]
}

/// The declared key list is the manifest. If a key names a width no arm
/// declares, that key panics at run time inside the harness, which reads as a
/// crashed variant rather than as a bench that was never wired up.
#[test]
fn every_declared_key_names_a_swept_width_and_a_declared_operation_count() {
    for &key in ALL_KEYS {
        assert!(
            SWEPT_WIDTHS.contains(&key_w(key)),
            "key {key} names width {} which no arm declares",
            key_w(key)
        );
        assert!(
            [0usize, 1, 2, 3, 4, 8].contains(&key_d(key)),
            "key {key} names operation count {} which the dispatch table does not declare",
            key_d(key)
        );
    }
}

/// Every arm computes the same answer, at every key, on real built input.
///
/// This is what the harness's cross-variant byte comparison also checks, and
/// it is here as well because a failure at build time is cheaper to read than
/// a failure inside a hundred-seed validation pass.
#[test]
fn all_five_arms_agree_at_every_declared_key() {
    for &key in ALL_KEYS {
        let (_buf, p) = built(key, 0xA5A5_1234);
        let col = unsafe { &*p };
        let got = all_arms(key, col);
        let base = got[0].1;
        for (name, v) in got.iter() {
            assert_eq!(
                *v,
                base,
                "arm `{name}` disagrees with `ragged` at key {key} (W={}, n={}, D={})",
                key_w(key),
                key_n(key),
                key_d(key)
            );
        }
    }
}

/// Every arm agrees with a computation in a different radix that shares no
/// code with any of them.
///
/// Agreement between the arms establishes only that they agree. `20` section
/// 2.1 is what that looks like when they all compute the same wrong thing.
#[test]
fn every_arm_matches_the_independent_oracle_at_every_declared_key() {
    for &key in ALL_KEYS {
        let (buf, p) = built(key, 0x1357_9BDF);
        let col = unsafe { &*p };
        let w = key_w(key);
        let n = key_n(key);
        let vals = crate::column::decode(&buf, REGION_RAGGED, n, rag_bytes(w), w);
        let want = oracle::reference(&vals, w, key_d(key));
        for (name, v) in all_arms(key, col).iter() {
            assert_eq!(
                *v, want,
                "arm `{name}` disagrees with the 128-bit-radix oracle at key {key}"
            );
        }
    }
}

/// **The check the previous family did not have where it mattered.**
///
/// If the answer does not depend on the input, an arm that read no data at all
/// passes every agreement check, and the optimiser is free to delete the loop.
/// `20` found six committed cells in exactly that state, reporting throughput
/// 48 to 66 times above this host's arithmetic roofline.
///
/// Asserted at **every declared key**, and at three positions in the column
/// including the last, because a fixpoint absorbs the tail of a column and a
/// perturbation of the first element would still show through.
///
/// No `black_box` anywhere. `20` section 2.1 found the previous family's
/// diagnostic passing because `black_box` on the operand hid the constant that
/// made the fixpoint provable, so the test exercised a configuration the bench
/// never ran. This runs the bench's own arms on the bench's own input.
#[test]
fn the_answer_moves_when_any_single_element_moves() {
    for &key in ALL_KEYS {
        let w = key_w(key);
        let n = key_n(key);
        let stride = rag_bytes(w);
        let wstride = wr_bytes(w);
        let a16 = crate::shape::a16_bytes(w);

        let mut buf = build_bytes(key, 0x0BAD_C0DE);
        let base = arms::ragged(key, unsafe { &*(buf.as_ptr() as *const Column) });

        for &i in &[0usize, n / 2, n - 1] {
            // Flip the lowest bit of element `i` in all three regions, so the
            // column stays consistent and every arm sees the same change.
            buf[i * stride] ^= 1;
            buf[REGION_BYTES + i * wstride] ^= 1;
            buf[2 * REGION_BYTES + i * a16] ^= 1;

            let col = unsafe { &*(buf.as_ptr() as *const Column) };
            for (name, v) in all_arms(key, col).iter() {
                assert_ne!(
                    *v, base,
                    "arm `{name}` at key {key} returned the same answer after element {i} \
                     of {n} changed, so the answer does not depend on that element and the \
                     timed loop may be deleted without the cross-variant check noticing"
                );
            }

            buf[i * stride] ^= 1;
            buf[REGION_BYTES + i * wstride] ^= 1;
            buf[2 * REGION_BYTES + i * a16] ^= 1;
        }

        // And restoring the column restores the answer, so the perturbation
        // above was the only thing that moved.
        let col = unsafe { &*(buf.as_ptr() as *const Column) };
        assert_eq!(arms::ragged(key, col), base, "restore failed at key {key}");
    }
}

/// A different seed gives a different answer, at every key.
///
/// Weaker than the per-element check above and it catches a different thing: a
/// transform that collapsed the whole column to a constant would pass a
/// single-element perturbation only if that element happened to survive, and
/// would fail this outright.
#[test]
fn the_answer_depends_on_the_seed_at_every_declared_key() {
    for &key in ALL_KEYS {
        let (_a, pa) = built(key, 11);
        let (_b, pb) = built(key, 22);
        let x = arms::ragged(key, unsafe { &*pa });
        let y = arms::ragged(key, unsafe { &*pb });
        assert_ne!(x, y, "the answer is seed-independent at key {key}");
    }
}

/// The answer exercises **every bit** of the declared width.
///
/// A wide-rung comparison in which the top of the width carried no information
/// would be a comparison of two ways to move zeroes around, and the ragged
/// shape's whole claim is about the bytes at the top.
///
/// Stated as bit coverage over a set of seeds rather than as "the top limb is
/// non-zero", which is what a first version asserted and which is wrong at
/// `W = 129`: there the top limb holds a single bit, so it is zero for about
/// half of all inputs and the test failed on a correct implementation. Coverage
/// over thirty-two seeds is the property that was actually meant.
#[test]
fn the_answer_exercises_every_bit_of_the_declared_width() {
    for &key in ALL_KEYS {
        let w = key_w(key);
        let limbs = crate::shape::limbs_of(w);
        let mut seen = [0u64; 4];
        for seed in 0u64..32 {
            let (_buf, p) = built(key, seed);
            let v = arms::ragged(key, unsafe { &*p });
            for l in 0..4 {
                seen[l] |= v[l];
            }
        }
        for l in 0..limbs - 1 {
            assert_eq!(
                seen[l],
                u64::MAX,
                "at key {key} limb {l} of the answer never set every bit across \
                 thirty-two seeds, so part of the declared width is dead"
            );
        }
        assert_eq!(
            seen[limbs - 1],
            crate::shape::top_mask(w),
            "at key {key} the top limb of the answer never covered its {} declared bits",
            crate::shape::top_mask(w).count_ones()
        );
        for l in limbs..4 {
            assert_eq!(
                seen[l], 0,
                "at key {key} a limb above the declared width was set"
            );
        }
    }
}

/// The two collision widths collide, on real answers rather than on the
/// stride arithmetic alone.
///
/// At `W = 192` and `W = 256` the ragged and word-rounded payloads are the
/// same size, so those arms are reading identically-shaped regions. This is
/// the bench's free second noise-floor reading and it is worth pinning that it
/// really is one.
#[test]
fn the_arms_read_identical_shapes_at_the_whole_limb_widths() {
    for w in [192u32, 256] {
        assert_eq!(rag_bytes(w), wr_bytes(w));
        let key = w as usize * 1000 + 3;
        let (_buf, p) = built(key, 7);
        let col = unsafe { &*p };
        assert_eq!(arms::ragged(key, col), arms::wordround(key, col));
    }
}
