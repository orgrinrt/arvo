//! The read-off spelling over ratios that carry out of `[0, 1)`, so the position
//! lies past either end of the index where the slot sits at one.

use super::{
    Spelling,
    complete,
    dens,
    i8_saturate,
    i8_wrap,
    i64_saturate,
    i64_wrap,
    nums,
    oracle_step,
    slots,
};

/// The oracle over a ratio that carries, completed: the position is
/// `s + num/d`, which may lie past either end of the index.
pub fn oracle_carried(s: i128, num: i64, d: i64, lo: i128, hi: i128, wrap: bool) -> i128 {
    let whole = num.div_euclid(d) as i128;
    let rem = num.rem_euclid(d) as i128;
    let d = d as i128;
    let at = s.checked_add(whole);
    let negative = match at {
        Some(v) => v < 0,
        None => whole < 0,
    };
    let step = if negative {
        -(d - 2 * rem).div_euclid(2 * d)
    } else {
        (2 * rem + d).div_euclid(2 * d)
    };
    if wrap {
        let span = hi - lo + 1;
        (s.rem_euclid(span) + whole.rem_euclid(span) - lo.rem_euclid(span) + step).rem_euclid(span)
            + lo
    } else {
        match at {
            Some(v) => complete(v, step, lo, hi, false),
            None if whole > 0 => hi,
            None => lo,
        }
    }
}

/// Numerators outside `[0, d)`.
fn carried_nums(d: i64) -> Vec<i64> {
    let h = d / 2;
    [
        Some(-1),
        Some(-h),
        d.checked_neg().map(|x| x + h),
        d.checked_neg(),
        d.checked_neg().and_then(|x| x.checked_sub(h)),
        Some(d),
        d.checked_add(h),
        d.checked_mul(2).and_then(|x| x.checked_add(h)),
        Some(i64::MAX),
        Some(i64::MIN + 1),
    ]
    .into_iter()
    .flatten()
    .filter(|&n| n < 0 || n >= d)
    .collect()
}

pub fn run() {
    // The carried oracle is the plain one wherever the ratio does not carry.
    for s in slots() {
        for d in dens() {
            for n in nums(d) {
                for (lo, hi, wrap) in [(-128, 127, true), (-128, 127, false)] {
                    assert_eq!(
                        oracle_carried(s, n, d, lo, hi, wrap),
                        complete(s, oracle_step(s, n, d), lo, hi, wrap)
                    );
                }
            }
        }
    }
    let tables: [(&str, Spelling, Spelling, (i128, i128), bool); 4] = [
        (
            "Integer<8>, Wrap",
            i8_wrap::select,
            i8_wrap::half_up_alone,
            i8_wrap::range(),
            true,
        ),
        (
            "Integer<8>, Saturate",
            i8_saturate::select,
            i8_saturate::half_up_alone,
            i8_saturate::range(),
            false,
        ),
        (
            "Integer<64>, Wrap",
            i64_wrap::select,
            i64_wrap::half_up_alone,
            i64_wrap::range(),
            true,
        ),
        (
            "Integer<64>, Saturate",
            i64_saturate::select,
            i64_saturate::half_up_alone,
            i64_saturate::range(),
            false,
        ),
    ];
    for (sig, select, control, (lo, hi), wrap) in tables {
        let (mut cells, mut wrong, mut ties, mut control_wrong) = (0, 0, 0, 0);
        for s in slots() {
            for d in dens() {
                for num in carried_nums(d) {
                    cells += 1;
                    ties += usize::from(2 * (num.rem_euclid(d) as i128) == d as i128);
                    let want = oracle_carried(s, num, d, lo, hi, wrap);
                    if select(s, num, d) != Some(want) {
                        wrong += 1;
                        if wrong < 4 {
                            println!(
                                "  wrong at {s} + {num}/{d}: {:?} for {want}",
                                select(s, num, d)
                            );
                        }
                    }
                    control_wrong += usize::from(control(s, num, d) != Some(want));
                }
            }
        }
        println!(
            "select over carried ratios, {sig}: {cells} cells, {ties} ties, {wrong} wrong; \
             half_up_alone {control_wrong} wrong"
        );
    }
}
