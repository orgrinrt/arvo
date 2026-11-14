//! Spike p1. A model of crossings in plain integer arithmetic, with no arvo
//! crate in it, so that nothing here agrees with the shipped code by sharing it.
//!
//! A system is a window `[lo, hi]` of integers plus an overflow kind, or a grid
//! plus a rounding mode. Sections:
//!
//! A. the measured cell of the question row (226 of 256), reproduced.
//! B. a signedness crossing at one width.
//! C. composition of overflow crossings through an intermediate, per rule.
//! D. composition of rounding crossings through an intermediate grid, per rule.
//! E. widen, multiply exactly, narrow back: which rule reproduces the format
//!    spine's "exact operation then one adaptation onto the result's set".
//! F. an interval: two components with the same endpoints that need two
//!    different reductions.
//!
//! Every section has a control that is expected to come out the other way.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Wrap,
    Sat,
}
const KINDS: [Kind; 2] = [Kind::Wrap, Kind::Sat];

#[derive(Clone, Copy)]
struct Win {
    lo: i64,
    hi: i64,
}
const S8: Win = Win {
    lo: -128,
    hi: 127,
};
const S6: Win = Win {
    lo: -32,
    hi: 31,
};
const S4: Win = Win {
    lo: -8,
    hi: 7,
};
const U8: Win = Win {
    lo: 0,
    hi: 255,
};
const U6: Win = Win {
    lo: 0,
    hi: 63,
};
const U4: Win = Win {
    lo: 0,
    hi: 15,
};

fn reduce(k: Kind, w: Win, v: i64) -> i64 {
    match k {
        Kind::Sat => v.clamp(w.lo, w.hi),
        Kind::Wrap => {
            let m = w.hi - w.lo + 1;
            (v - w.lo).rem_euclid(m) + w.lo
        },
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    TowardZero,
    Floor,
    Ceil,
    HalfUp,
    HalfEven,
}
const MODES: [Mode; 5] = [Mode::TowardZero, Mode::Floor, Mode::Ceil, Mode::HalfUp, Mode::HalfEven];

/// `n / d` rounded to an integer under `m`, `d > 0`. `HalfUp` is ties toward
/// positive infinity, `floor(x + 1/2)`, as the ratified row has it.
fn round_div(n: i64, d: i64, m: Mode) -> i64 {
    let f = n.div_euclid(d);
    let r = n.rem_euclid(d);
    match m {
        Mode::Floor => f,
        Mode::Ceil => {
            if r == 0 {
                f
            } else {
                f + 1
            }
        },
        Mode::TowardZero => {
            if n < 0 && r != 0 {
                f + 1
            } else {
                f
            }
        },
        Mode::HalfUp => {
            if 2 * r >= d {
                f + 1
            } else {
                f
            }
        },
        Mode::HalfEven => {
            if 2 * r > d || (2 * r == d && f.rem_euclid(2) != 0) {
                f + 1
            } else {
                f
            }
        },
    }
}

fn section_a() {
    println!("== A. the measured cell: signed 8 bits onto signed 4 bits, wrap against saturate");
    let (mut differ, mut lossy, mut lossy_agree, mut inside_differ) = (0, 0, 0, 0);
    for v in S8.lo ..= S8.hi {
        let w = reduce(Kind::Wrap, S4, v);
        let s = reduce(Kind::Sat, S4, v);
        let inside = (S4.lo ..= S4.hi).contains(&v);
        if !inside {
            lossy += 1;
            if w == s {
                lossy_agree += 1;
            }
        } else if w != s {
            inside_differ += 1;
        }
        if w != s {
            differ += 1;
        }
    }
    println!(
        "differ {differ}/256, lossy {lossy}, lossy yet agreeing {lossy_agree}, inside the target and differing {inside_differ}"
    );
    assert_eq!(
        (differ, lossy, lossy_agree, inside_differ),
        (226, 240, 14, 0)
    );
    // Control: the widening direction loses nothing, so nothing may differ.
    let mut widen = 0;
    for v in S4.lo ..= S4.hi {
        if reduce(Kind::Wrap, S8, v) != reduce(Kind::Sat, S8, v) {
            widen += 1;
        }
    }
    println!("control, widening 4 onto 8: differ {widen}/16");
    assert_eq!(widen, 0);
}

fn section_b() {
    println!("== B. signedness at one width: signed 8 bits onto unsigned 8 bits");
    let mut differ = 0;
    for v in S8.lo ..= S8.hi {
        if reduce(Kind::Wrap, U8, v) != reduce(Kind::Sat, U8, v) {
            differ += 1;
        }
    }
    println!("differ {differ}/256 (every negative member: wrap gives v + 256, saturate gives 0)");
    assert_eq!(differ, 128);
}

/// Whether going through `b` equals going direct, on every member of `a`.
fn composes(a: Win, b: Win, c: Win, first: Kind, second: Kind, direct: Kind) -> (bool, u32) {
    let mut wrong = 0;
    for v in a.lo ..= a.hi {
        let two = reduce(second, c, reduce(first, b, v));
        let one = reduce(direct, c, v);
        if two != one {
            wrong += 1;
        }
    }
    (wrong == 0, wrong)
}

fn section_c() {
    println!("== C. composition of overflow crossings, A onto B onto C against A onto C");
    for (name, a, b, c) in [("signed 8, 6, 4", S8, S6, S4), ("unsigned 8, 6, 4", U8, U6, U4)] {
        println!("-- {name}");
        let (mut src_ok, mut tgt_ok) = (0, 0);
        for ka in KINDS {
            for kb in KINDS {
                for kc in KINDS {
                    // Source's: each step is reduced by the kind of the system left.
                    let (s, sw) = composes(a, b, c, ka, kb, ka);
                    // Target's: each step is reduced by the kind of the system entered.
                    let (t, tw) = composes(a, b, c, kb, kc, kc);
                    println!(
                        "A={ka:?} B={kb:?} C={kc:?}: source's rule composes {s} ({sw} wrong), target's rule composes {t} ({tw} wrong)"
                    );
                    src_ok += s as u32;
                    tgt_ok += t as u32;
                }
            }
        }
        println!("source's rule composes at {src_ok}/8 assignments, target's at {tgt_ok}/8");
        assert_eq!((src_ok, tgt_ok), (4, 4));
        // One reduction named for the whole path, whatever the endpoints selected.
        let mut named_ok = 0;
        for k in KINDS {
            let (n, _) = composes(a, b, c, k, k, k);
            named_ok += n as u32;
        }
        println!("one kind named for the whole path composes at {named_ok}/2");
        assert_eq!(named_ok, 2);
    }
    // Control on the instrument: a window that is not nested must break even the
    // same-kind wrap path, or `composes` reports success unconditionally.
    let odd = Win {
        lo: 0,
        hi: 9,
    };
    let (c, wrong) = composes(U8, odd, U4, Kind::Wrap, Kind::Wrap, Kind::Wrap);
    println!("control, wrap through a ten-member window: composes {c} ({wrong} wrong)");
    assert!(!c);
}

/// Grids at quanta 1/32, 1/8 and 1/2: a step of 4 and then a step of 4, against
/// one step of 16.
fn rounding_composes(first: Mode, second: Mode, direct: Mode) -> bool {
    (-128i64 ..= 127)
        .all(|n| round_div(round_div(n, 4, first), 4, second) == round_div(n, 16, direct))
}

fn section_d() {
    println!("== D. composition of rounding crossings, quantum 1/32 onto 1/8 onto 1/2");
    let (mut src_ok, mut tgt_ok) = (0, 0);
    let mut src_pairs = Vec::new();
    let mut tgt_pairs = Vec::new();
    for ma in MODES {
        for mb in MODES {
            for mc in MODES {
                let s = rounding_composes(ma, mb, ma);
                let t = rounding_composes(mb, mc, mc);
                src_ok += s as u32;
                tgt_ok += t as u32;
                if s && !src_pairs.contains(&(ma, mb)) {
                    src_pairs.push((ma, mb));
                }
                if t && !tgt_pairs.contains(&(mb, mc)) {
                    tgt_pairs.push((mb, mc));
                }
            }
        }
    }
    println!("source's rule composes at {src_ok}/125 assignments, target's at {tgt_ok}/125");
    println!(
        "source's rule, the (first, second) mode pairs that compose to the first: {src_pairs:?}"
    );
    println!(
        "target's rule, the (first, second) mode pairs that compose to the second: {tgt_pairs:?}"
    );
    // The prediction written here first was that the two counts are equal, as
    // they are for the overflow kinds in section C. It was refuted, 15 against
    // 20, and is kept as a refuted prediction: a floor is absorbed by a later
    // half-up, and nothing is absorbed by an earlier one.
    assert_eq!((src_ok, tgt_ok), (15, 20));
    let mut named = Vec::new();
    for m in MODES {
        if rounding_composes(m, m, m) {
            named.push(m);
        }
    }
    println!("one mode named for the whole path composes for: {named:?}");
    assert_eq!(named, [Mode::TowardZero, Mode::Floor, Mode::Ceil]);
    // First witness of the double rounding for each nearest mode.
    for m in [Mode::HalfUp, Mode::HalfEven] {
        let n = (-128i64 ..= 127)
            .find(|&n| round_div(round_div(n, 4, m), 4, m) != round_div(n, 16, m))
            .unwrap();
        println!(
            "{m:?} first witness: slot {n} at 1/32 goes to {} in two steps and {} in one",
            round_div(round_div(n, 4, m), 4, m),
            round_div(n, 16, m)
        );
    }
}

fn section_e() {
    println!("== E. widen, multiply exactly, narrow back, against the spine's one adaptation");
    // The product of two signed 4-bit members is in [-56, 64], inside signed 8 bits,
    // so the wide system's own reduction never fires.
    let mut fired = 0;
    for a in S4.lo ..= S4.hi {
        for b in S4.lo ..= S4.hi {
            for kw in KINDS {
                if reduce(kw, S8, a * b) != a * b {
                    fired += 1;
                }
            }
        }
    }
    println!("the wide system's reduction fired on {fired} of 512 (product, kind) cases");
    assert_eq!(fired, 0);
    for kw in KINDS {
        for kt in KINDS {
            let (mut src_wrong, mut tgt_wrong) = (0, 0);
            for a in S4.lo ..= S4.hi {
                for b in S4.lo ..= S4.hi {
                    let spec = reduce(kt, S4, a * b);
                    let wide = reduce(kw, S8, a * b);
                    if reduce(kw, S4, wide) != spec {
                        src_wrong += 1;
                    }
                    // By construction zero: this is the spine's sentence restated,
                    // and is printed as the analytic arm, not as a measurement.
                    if reduce(kt, S4, wide) != spec {
                        tgt_wrong += 1;
                    }
                }
            }
            println!(
                "wide={kw:?} result={kt:?}: source's rule misses the spine's product on {src_wrong}/256 pairs, target's on {tgt_wrong}/256 (analytic)"
            );
            assert_eq!(src_wrong == 0, kw == kt);
            assert_eq!(tgt_wrong, 0);
        }
    }
}

fn section_f() {
    println!("== F. an interval at quantum 1/8 crossing onto the integers");
    // An interval [lo, hi] in eighths. A crossing is an enclosure when the image
    // contains the source, and tight when no narrower integer interval does.
    let mut total = 0u32;
    let mut one_mode = [(0u32, 0u32); 5];
    let (mut split_unsound, mut split_loose) = (0u32, 0u32);
    for lo in -40i64 ..= 40 {
        for hi in lo ..= 40 {
            total += 1;
            let tight = (round_div(lo, 8, Mode::Floor), round_div(hi, 8, Mode::Ceil));
            for (i, m) in MODES.iter().enumerate() {
                let got = (round_div(lo, 8, *m), round_div(hi, 8, *m));
                let encloses = got.0 * 8 <= lo && got.1 * 8 >= hi;
                if !encloses {
                    one_mode[i].0 += 1;
                } else if got != tight {
                    one_mode[i].1 += 1;
                }
            }
            let got = (round_div(lo, 8, Mode::Floor), round_div(hi, 8, Mode::Ceil));
            if !(got.0 * 8 <= lo && got.1 * 8 >= hi) {
                split_unsound += 1;
            } else if got != tight {
                split_loose += 1;
            }
        }
    }
    for (i, m) in MODES.iter().enumerate() {
        println!(
            "one mode for both components, {m:?}: not an enclosure on {}/{total}, enclosing but loose on {}",
            one_mode[i].0, one_mode[i].1
        );
        assert!(one_mode[i].0 > 0);
    }
    println!(
        "floor below and ceil above: not an enclosure on {split_unsound}/{total}, loose on {split_loose}"
    );
    assert_eq!(split_unsound, 0);
}

fn main() {
    section_a();
    section_b();
    section_c();
    section_d();
    section_e();
    section_f();
    println!("== every assertion held");
}
