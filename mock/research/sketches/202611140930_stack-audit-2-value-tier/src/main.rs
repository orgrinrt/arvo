//! An ad-hoc spike for the second stack audit. Every line it prints is an
//! ad-hoc spike on one shared virtual machine: a median of repeated runs with no
//! pinning, no counters and no noise gate. It orders arms of very different
//! cost and says nothing finer than that.
//!
//! Each arm is a kernel over columns, `#[inline(never)]`, in this one crate.
//! The value arms call `arvo_numeral::Num`; the hand arms are what a
//! programmer writes on the host integer for the same declared signature. The
//! two are asserted equal on the whole input before anything is timed.

use std::hint::black_box;
use std::time::Instant;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Floor, HalfEven, TowardZero};
use arvo_format::{Adapt, Exact, Slot};
use arvo_numeral::{Divisor, Fix, Int, Quiet, Uint};

type U13S = Uint<13, Adapt<TowardZero, Saturate>, Quiet>;
type U13W = Uint<13, Adapt<TowardZero, Wrap>, Quiet>;
type I32S = Int<32, Adapt<TowardZero, Saturate>, Quiet>;
type I32W = Int<32, Adapt<TowardZero, Wrap>, Quiet>;
type QHS = Fix<32, -16, Adapt<HalfEven, Saturate>, Quiet>;
type QFW = Fix<32, -16, Adapt<Floor, Wrap>, Quiet>;
type Q8HS = Fix<16, -8, Adapt<HalfEven, Saturate>, Quiet>;
// The same signatures under the default check, `Abort`. In a build with debug
// assertions each operation asks its verdict first; elsewhere they are the
// `Quiet` arms again.
type U13SA = Uint<13, Adapt<TowardZero, Saturate>>;
type I32WA = Int<32, Adapt<TowardZero, Wrap>>;
type QHSA = Fix<32, -16, Adapt<HalfEven, Saturate>>;
type QFormat = arvo_format::points::Fixed<32, -16>;

const N: usize = 4096;
const TOP13: u16 = 8191;
const FRAC: u32 = 16;
const HALF: i64 = 1 << (FRAC - 1);
const MASK: i64 = (1 << FRAC) - 1;
/// Three, in Q16.16 slots.
const THREE: i64 = 3 << FRAC;

fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// Median and fifth-percentile nanoseconds per element over `reps` runs of `f`.
fn time(label: &str, reps: usize, per: usize, mut f: impl FnMut()) {
    let mut samples = Vec::with_capacity(reps);
    for _ in 0 .. reps {
        let t = Instant::now();
        f();
        samples.push(t.elapsed().as_nanos() as f64);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "{label:<44} {:>9.3} ns/elem median {:>9.3} p5",
        samples[reps / 2] / per as f64,
        samples[reps / 20] / per as f64
    );
}

// ---- kernels: a unary map, a binary map, a fold ----

macro_rules! map1 {
    ($name:ident, $in:ty, $out:ty, |$x:ident| $body:expr) => {
        #[inline(never)]
        fn $name(a: &[$in], out: &mut [$out]) {
            for (o, &$x) in out.iter_mut().zip(a) {
                *o = $body;
            }
        }
    };
}

macro_rules! map2 {
    ($name:ident, $t:ty, |$x:ident, $y:ident| $body:expr) => {
        #[inline(never)]
        fn $name(a: &[$t], b: &[$t], out: &mut [$t]) {
            for ((o, &$x), &$y) in out.iter_mut().zip(a).zip(b) {
                *o = $body;
            }
        }
    };
}

macro_rules! fold {
    ($name:ident, $t:ty, $zero:expr, |$acc:ident, $x:ident| $body:expr) => {
        #[inline(never)]
        fn $name(a: &[$t]) -> $t {
            let mut $acc: $t = $zero;
            for &$x in a {
                $acc = $body;
            }
            $acc
        }
    };
}

// The doors.
map1!(ingest_sat_from_index, u16, U13S, |x| U13S::from_index(x));
map1!(ingest_sat_adapted, u16, U13S, |x| {
    U13S::adapted(Exact::on_grid(Slot::at(x as i128)))
});
map1!(ingest_sat_hand, u16, u16, |x| {
    if x > TOP13 { TOP13 } else { x }
});
map1!(ingest_wrap_from_index, u16, U13W, |x| U13W::from_index(x));
map1!(ingest_wrap_adapted, u16, U13W, |x| {
    U13W::adapted(Exact::on_grid(Slot::at(x as i128)))
});
map1!(ingest_wrap_hand, u16, u16, |x| x & TOP13);
// Whether a host integer names a member, asked through the two public doors
// and by hand. Added in the audit's second pass.
map1!(verdict_round_trip_sat, u16, bool, |x| {
    U13S::from_index(x).index::<u16>() == x
});
map1!(verdict_round_trip_wrap, u16, bool, |x| {
    U13W::from_index(x).index::<u16>() == x
});
map1!(verdict_hand, u16, bool, |x| x <= TOP13);
map1!(egress_index, U13S, u16, |v| v.index::<u16>());
map1!(egress_slot, U13S, u16, |v| v.slot().index() as u16);
map1!(egress_hand, u16, u16, |x| x);

// The ring operations at an integer quantum.
map2!(add_u13s_value, U13S, |x, y| x.add(y));
map2!(add_u13s_hand, u16, |x, y| {
    let s = x + y;
    if s > TOP13 { TOP13 } else { s }
});
map2!(sub_u13s_value, U13S, |x, y| x.sub(y));
map2!(sub_u13s_hand, u16, |x, y| x.saturating_sub(y));
map2!(mul_u13s_value, U13S, |x, y| x.mul(y));
map2!(mul_u13s_hand, u16, |x, y| {
    let p = x as u32 * y as u32;
    if p > TOP13 as u32 { TOP13 } else { p as u16 }
});
map2!(mul_i32s_value, I32S, |x, y| x.mul(y));
map2!(mul_i32s_hand, i32, |x, y| x.saturating_mul(y));
map2!(mul_i32w_value, I32W, |x, y| x.mul(y));
map2!(mul_i32w_hand, i32, |x, y| x.wrapping_mul(y));

// The default check. The hand arms are the host's own operators, which a build
// with overflow checks on makes panic where the value's `Abort` does.
map2!(add_u13_abort_value, U13SA, |x, y| x.add(y));
map2!(add_u13_abort_hand, u16, |x, y| {
    let s = x + y;
    assert!(s <= TOP13);
    s
});
map2!(mul_i32_abort_value, I32WA, |x, y| x.mul(y));
map2!(mul_i32_abort_hand, i32, |x, y| x * y);
map2!(qmul_abort_value, QHSA, |x, y| x.mul(y));

// The folds.
fold!(sum_u13w_value, U13W, U13W::constant::<0>(), |acc, x| {
    acc.add(x)
});
fold!(sum_u13w_hand_each, u16, 0, |acc, x| {
    acc.wrapping_add(x) & TOP13
});
#[inline(never)]
fn sum_u13w_hand_once(a: &[u16]) -> u16 {
    let mut acc = 0u16;
    for &x in a {
        acc = acc.wrapping_add(x);
    }
    acc & TOP13
}
fold!(sum_u13s_value, U13S, U13S::constant::<0>(), |acc, x| {
    acc.add(x)
});
fold!(sum_u13s_hand, u16, 0, |acc, x| {
    let s = acc + x;
    if s > TOP13 { TOP13 } else { s }
});
fold!(
    max_i32_value,
    I32S,
    I32S::constant::<{ i32::MIN as i128 }>(),
    |acc, x| acc.max(x)
);
fold!(max_i32_hand, i32, i32::MIN, |acc, x| {
    if acc >= x { acc } else { x }
});

// The fixed-point product, by hand.
#[inline(always)]
fn saturate32(v: i64) -> i32 {
    if v > i32::MAX as i64 {
        i32::MAX
    } else if v < i32::MIN as i64 {
        i32::MIN
    } else {
        v as i32
    }
}

#[inline(always)]
fn q_half_even(a: i32, b: i32) -> i32 {
    let p = a as i64 * b as i64;
    let up = (p + HALF) >> FRAC;
    saturate32(up - ((p & MASK == HALF) & (up & 1 != 0)) as i64)
}

#[inline(always)]
fn q_floor_wrap(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> FRAC) as i32
}

#[inline(always)]
fn q_div3_half_even(a: i32) -> i32 {
    let n = (a as i64) << FRAC;
    let q = n.div_euclid(THREE);
    let r = n.rem_euclid(THREE);
    let up = (2 * r > THREE) | ((2 * r == THREE) & (q & 1 != 0));
    saturate32(q + up as i64)
}

#[inline(always)]
fn q_to_q8_half_even(a: i32) -> i16 {
    let p = a as i64;
    let up = (p + 128) >> 8;
    let r = up - ((p & 255 == 128) & (up & 1 != 0)) as i64;
    if r > i16::MAX as i64 {
        i16::MAX
    } else if r < i16::MIN as i64 {
        i16::MIN
    } else {
        r as i16
    }
}

map2!(qmul_hs_value, QHS, |x, y| x.mul(y));
map2!(qmul_hs_hand, i32, |x, y| q_half_even(x, y));
map2!(qmul_fw_value, QFW, |x, y| x.mul(y));
map2!(qmul_fw_hand, i32, |x, y| q_floor_wrap(x, y));
map2!(fmul_f32, f32, |x, y| x * y);
map1!(qdiv3_value, QHS, QHS, |x| {
    x.div(Divisor::<QFormat>::constant::<{ 3 << 16 }>())
});
map1!(qdiv3_hand, i32, i32, |x| q_div3_half_even(x));
map1!(qconv_value, QHS, Q8HS, |x| {
    x.convert::<arvo_format::Signature<
    arvo_format::points::Fixed<16, -8>,
    Adapt<HalfEven, Saturate>,
>>()
});
map1!(qconv_hand, i32, i16, |x| q_to_q8_half_even(x));

#[inline(never)]
fn decay_value(start: QHS, factor: QHS, steps: u32) -> QHS {
    let mut v = start;
    for _ in 0 .. steps {
        v = v.mul(factor);
    }
    v
}

#[inline(never)]
fn decay_hand(start: i32, factor: i32, steps: u32) -> i32 {
    let mut v = start;
    for _ in 0 .. steps {
        v = q_half_even(v, factor);
    }
    v
}

fn main() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut draw = |m: u64| xorshift(&mut seed) % m;

    // Host columns. `any16` covers the whole u16 so both completions act;
    // `in13` are members; `small` keeps a saturating fold off its fixpoint.
    let any16: Vec<u16> = (0 .. N).map(|_| draw(65536) as u16).collect();
    let in13a: Vec<u16> = (0 .. N).map(|_| draw(8192) as u16).collect();
    let in13b: Vec<u16> = (0 .. N).map(|_| draw(8192) as u16).collect();
    let small13: Vec<u16> = (0 .. N).map(|_| draw(200) as u16).collect();
    let tiny13: Vec<u16> = (0 .. N).map(|_| draw(3) as u16).collect();
    let i32a: Vec<i32> = (0 .. N).map(|_| draw(1 << 32) as u32 as i32).collect();
    let i32b: Vec<i32> = (0 .. N).map(|_| draw(1 << 18) as i32 - (1 << 17)).collect();
    // Q16.16 members within plus or minus 64, so a product stays in range.
    let qa: Vec<i32> = (0 .. N).map(|_| draw(1 << 23) as i32 - (1 << 22)).collect();
    let qb: Vec<i32> = (0 .. N).map(|_| draw(1 << 23) as i32 - (1 << 22)).collect();
    let fa: Vec<f32> = qa.iter().map(|&x| x as f32 / 65536.0).collect();
    let fb: Vec<f32> = qb.iter().map(|&x| x as f32 / 65536.0).collect();

    let v13a: Vec<U13S> = in13a.iter().map(|&x| U13S::from_index(x)).collect();
    let v13b: Vec<U13S> = in13b.iter().map(|&x| U13S::from_index(x)).collect();
    let vsmall_a: Vec<U13S> = small13.iter().map(|&x| U13S::from_index(x)).collect();
    let vtiny: Vec<U13S> = tiny13.iter().map(|&x| U13S::from_index(x)).collect();
    let w13a: Vec<U13W> = in13a.iter().map(|&x| U13W::from_index(x)).collect();
    let vi32sa: Vec<I32S> = i32a.iter().map(|&x| I32S::from_index(x)).collect();
    let vi32sb: Vec<I32S> = i32b.iter().map(|&x| I32S::from_index(x)).collect();
    let vi32wa: Vec<I32W> = i32a.iter().map(|&x| I32W::from_index(x)).collect();
    let vi32wb: Vec<I32W> = i32b.iter().map(|&x| I32W::from_index(x)).collect();
    let vqa: Vec<QHS> = qa.iter().map(|&x| QHS::from_index(x)).collect();
    let vqb: Vec<QHS> = qb.iter().map(|&x| QHS::from_index(x)).collect();
    let wqa: Vec<QFW> = qa.iter().map(|&x| QFW::from_index(x)).collect();
    let wqb: Vec<QFW> = qb.iter().map(|&x| QFW::from_index(x)).collect();

    let zero_s = U13S::constant::<0>();
    let zero_w = U13W::constant::<0>();
    let mut out_s = vec![zero_s; N];
    let mut out_w = vec![zero_w; N];
    let mut out_h = vec![0u16; N];
    let mut out_i32s = vec![I32S::constant::<0>(); N];
    let mut out_i32w = vec![I32W::constant::<0>(); N];
    let mut out_i32 = vec![0i32; N];
    let mut out_q = vec![QHS::constant::<0>(); N];
    let mut out_qw = vec![QFW::constant::<0>(); N];
    let mut out_q8 = vec![Q8HS::constant::<0>(); N];
    let mut out_i16 = vec![0i16; N];
    let mut out_f = vec![0f32; N];

    // ---- every value arm agrees with its hand arm on the whole input ----
    macro_rules! agree {
        ($value:expr, $hand:expr, $vs:ident, $hs:ident, $h:ty) => {{
            $value;
            $hand;
            for (v, h) in $vs.iter().zip($hs.iter()) {
                assert_eq!(v.index::<$h>(), *h);
            }
        }};
    }
    agree!(
        ingest_sat_from_index(&any16, &mut out_s),
        ingest_sat_hand(&any16, &mut out_h),
        out_s,
        out_h,
        u16
    );
    agree!(
        ingest_sat_adapted(&any16, &mut out_s),
        ingest_sat_hand(&any16, &mut out_h),
        out_s,
        out_h,
        u16
    );
    agree!(
        ingest_wrap_from_index(&any16, &mut out_w),
        ingest_wrap_hand(&any16, &mut out_h),
        out_w,
        out_h,
        u16
    );
    agree!(
        ingest_wrap_adapted(&any16, &mut out_w),
        ingest_wrap_hand(&any16, &mut out_h),
        out_w,
        out_h,
        u16
    );
    agree!(
        add_u13s_value(&v13a, &v13b, &mut out_s),
        add_u13s_hand(&in13a, &in13b, &mut out_h),
        out_s,
        out_h,
        u16
    );
    agree!(
        sub_u13s_value(&v13a, &v13b, &mut out_s),
        sub_u13s_hand(&in13a, &in13b, &mut out_h),
        out_s,
        out_h,
        u16
    );
    agree!(
        mul_u13s_value(&vsmall_a, &v13b, &mut out_s),
        mul_u13s_hand(&small13, &in13b, &mut out_h),
        out_s,
        out_h,
        u16
    );
    agree!(
        mul_i32s_value(&vi32sa, &vi32sb, &mut out_i32s),
        mul_i32s_hand(&i32a, &i32b, &mut out_i32),
        out_i32s,
        out_i32,
        i32
    );
    agree!(
        mul_i32w_value(&vi32wa, &vi32wb, &mut out_i32w),
        mul_i32w_hand(&i32a, &i32b, &mut out_i32),
        out_i32w,
        out_i32,
        i32
    );
    agree!(
        qmul_hs_value(&vqa, &vqb, &mut out_q),
        qmul_hs_hand(&qa, &qb, &mut out_i32),
        out_q,
        out_i32,
        i32
    );
    agree!(
        qmul_fw_value(&wqa, &wqb, &mut out_qw),
        qmul_fw_hand(&qa, &qb, &mut out_i32),
        out_qw,
        out_i32,
        i32
    );
    agree!(
        qdiv3_value(&vqa, &mut out_q),
        qdiv3_hand(&qa, &mut out_i32),
        out_q,
        out_i32,
        i32
    );
    agree!(
        qconv_value(&vqa, &mut out_q8),
        qconv_hand(&qa, &mut out_i16),
        out_q8,
        out_i16,
        i16
    );
    assert_eq!(
        sum_u13w_value(&w13a).index::<u16>(),
        sum_u13w_hand_each(&in13a)
    );
    assert_eq!(sum_u13w_hand_once(&in13a), sum_u13w_hand_each(&in13a));
    assert_eq!(
        sum_u13s_value(&vtiny).index::<u16>(),
        sum_u13s_hand(&tiny13)
    );
    assert_eq!(max_i32_value(&vi32sa).index::<i32>(), max_i32_hand(&i32a));
    let start = QHS::from_index(1000i32 << 16);
    let factor = QHS::from_index(65530i32);
    assert_eq!(
        decay_value(start, factor, 30_000).index::<i32>(),
        decay_hand(1000 << 16, 65530, 30_000)
    );
    println!("every value arm agrees with its hand arm on the whole input");

    const FAST: usize = 2001;
    const SLOW: usize = 201;

    println!("-- the doors, 13 bits unsigned, from and to u16");
    time("ingest saturate  from_index", FAST, N, || {
        ingest_sat_from_index(black_box(&any16), black_box(&mut out_s))
    });
    time("ingest saturate  adapted (the wide step)", SLOW, N, || {
        ingest_sat_adapted(black_box(&any16), black_box(&mut out_s))
    });
    time("ingest saturate  hand", FAST, N, || {
        ingest_sat_hand(black_box(&any16), black_box(&mut out_h))
    });
    time("ingest wrap      from_index", FAST, N, || {
        ingest_wrap_from_index(black_box(&any16), black_box(&mut out_w))
    });
    time("ingest wrap      adapted (the wide step)", SLOW, N, || {
        ingest_wrap_adapted(black_box(&any16), black_box(&mut out_w))
    });
    time("ingest wrap      hand", FAST, N, || {
        ingest_wrap_hand(black_box(&any16), black_box(&mut out_h))
    });
    let mut out_b = vec![false; N];
    let mut out_c = vec![false; N];
    verdict_hand(&any16, &mut out_c);
    verdict_round_trip_sat(&any16, &mut out_b);
    assert_eq!(out_b, out_c);
    verdict_round_trip_wrap(&any16, &mut out_b);
    assert_eq!(out_b, out_c);
    assert!(out_c.iter().any(|&b| b) && out_c.iter().any(|&b| !b));
    time("verdict          round trip, saturating", FAST, N, || {
        verdict_round_trip_sat(black_box(&any16), black_box(&mut out_b))
    });
    time("verdict          round trip, wrapping", FAST, N, || {
        verdict_round_trip_wrap(black_box(&any16), black_box(&mut out_b))
    });
    time("verdict          hand range test", FAST, N, || {
        verdict_hand(black_box(&any16), black_box(&mut out_c))
    });
    time("egress           index::<u16>", FAST, N, || {
        egress_index(black_box(&v13a), black_box(&mut out_h))
    });
    time("egress           slot().index()", FAST, N, || {
        egress_slot(black_box(&v13a), black_box(&mut out_h))
    });
    time("egress           hand copy", FAST, N, || {
        egress_hand(black_box(&in13a), black_box(&mut out_h))
    });

    println!("-- ring operations at an integer quantum, elementwise");
    time("add u13 saturate value", FAST, N, || {
        add_u13s_value(black_box(&v13a), black_box(&v13b), black_box(&mut out_s))
    });
    time("add u13 saturate hand", FAST, N, || {
        add_u13s_hand(black_box(&in13a), black_box(&in13b), black_box(&mut out_h))
    });
    time("sub u13 saturate value", FAST, N, || {
        sub_u13s_value(black_box(&v13a), black_box(&v13b), black_box(&mut out_s))
    });
    time("sub u13 saturate hand", FAST, N, || {
        sub_u13s_hand(black_box(&in13a), black_box(&in13b), black_box(&mut out_h))
    });
    time("mul u13 saturate value", FAST, N, || {
        mul_u13s_value(
            black_box(&vsmall_a),
            black_box(&v13b),
            black_box(&mut out_s),
        )
    });
    time("mul u13 saturate hand", FAST, N, || {
        mul_u13s_hand(
            black_box(&small13),
            black_box(&in13b),
            black_box(&mut out_h),
        )
    });
    time("mul i32 saturate value", FAST, N, || {
        mul_i32s_value(
            black_box(&vi32sa),
            black_box(&vi32sb),
            black_box(&mut out_i32s),
        )
    });
    time("mul i32 saturate hand", FAST, N, || {
        mul_i32s_hand(black_box(&i32a), black_box(&i32b), black_box(&mut out_i32))
    });
    time("mul i32 wrap     value", FAST, N, || {
        mul_i32w_value(
            black_box(&vi32wa),
            black_box(&vi32wb),
            black_box(&mut out_i32w),
        )
    });
    time("mul i32 wrap     hand", FAST, N, || {
        mul_i32w_hand(black_box(&i32a), black_box(&i32b), black_box(&mut out_i32))
    });

    // Members small enough that nothing leaves the range, so no arm aborts.
    let lo13a: Vec<u16> = (0 .. N).map(|_| draw(4000) as u16).collect();
    let lo13b: Vec<u16> = (0 .. N).map(|_| draw(4000) as u16).collect();
    let lo32a: Vec<i32> = (0 .. N).map(|_| draw(1 << 15) as i32).collect();
    let lo32b: Vec<i32> = (0 .. N).map(|_| draw(1 << 15) as i32).collect();
    let alo13a: Vec<U13SA> = lo13a.iter().map(|&x| U13SA::from_index(x)).collect();
    let alo13b: Vec<U13SA> = lo13b.iter().map(|&x| U13SA::from_index(x)).collect();
    let alo32a: Vec<I32WA> = lo32a.iter().map(|&x| I32WA::from_index(x)).collect();
    let alo32b: Vec<I32WA> = lo32b.iter().map(|&x| I32WA::from_index(x)).collect();
    let aqa: Vec<QHSA> = qa.iter().map(|&x| QHSA::from_index(x)).collect();
    let aqb: Vec<QHSA> = qb.iter().map(|&x| QHSA::from_index(x)).collect();
    let mut out_a13 = vec![U13SA::constant::<0>(); N];
    let mut out_a32 = vec![I32WA::constant::<0>(); N];
    let mut out_aq = vec![QHSA::constant::<0>(); N];
    println!(
        "-- the default check, `Abort`; debug assertions are {}",
        if cfg!(debug_assertions) {
            "ON"
        } else {
            "off, so these are the Quiet arms again"
        }
    );
    time("add u13 saturate Abort value", SLOW, N, || {
        add_u13_abort_value(
            black_box(&alo13a),
            black_box(&alo13b),
            black_box(&mut out_a13),
        )
    });
    time("add u13 checked hand", FAST, N, || {
        add_u13_abort_hand(black_box(&lo13a), black_box(&lo13b), black_box(&mut out_h))
    });
    time("mul i32 wrap Abort value", SLOW, N, || {
        mul_i32_abort_value(
            black_box(&alo32a),
            black_box(&alo32b),
            black_box(&mut out_a32),
        )
    });
    time("mul i32 checked hand", FAST, N, || {
        mul_i32_abort_hand(
            black_box(&lo32a),
            black_box(&lo32b),
            black_box(&mut out_i32),
        )
    });
    time(
        "product Q16.16 half_even saturate Abort value",
        SLOW,
        N,
        || qmul_abort_value(black_box(&aqa), black_box(&aqb), black_box(&mut out_aq)),
    );

    println!("-- folds");
    time("sum u13 wrap     value", FAST, N, || {
        black_box(sum_u13w_value(black_box(&w13a)));
    });
    time("sum u13 wrap     hand, mask each step", FAST, N, || {
        black_box(sum_u13w_hand_each(black_box(&in13a)));
    });
    time("sum u13 wrap     hand, mask once", FAST, N, || {
        black_box(sum_u13w_hand_once(black_box(&in13a)));
    });
    time("sum u13 saturate value", FAST, N, || {
        black_box(sum_u13s_value(black_box(&vtiny)));
    });
    time("sum u13 saturate hand", FAST, N, || {
        black_box(sum_u13s_hand(black_box(&tiny13)));
    });
    time("max i32          value", FAST, N, || {
        black_box(max_i32_value(black_box(&vi32sa)));
    });
    time("max i32          hand", FAST, N, || {
        black_box(max_i32_hand(black_box(&i32a)));
    });

    println!("-- Q16.16 in 32 bits: product, quotient by three, conversion to Q8.8");
    time("product half_even saturate value", SLOW, N, || {
        qmul_hs_value(black_box(&vqa), black_box(&vqb), black_box(&mut out_q))
    });
    time("product half_even saturate hand", FAST, N, || {
        qmul_hs_hand(black_box(&qa), black_box(&qb), black_box(&mut out_i32))
    });
    time("product floor wrap        value", SLOW, N, || {
        qmul_fw_value(black_box(&wqa), black_box(&wqb), black_box(&mut out_qw))
    });
    time("product floor wrap        hand", FAST, N, || {
        qmul_fw_hand(black_box(&qa), black_box(&qb), black_box(&mut out_i32))
    });
    time("product f32", FAST, N, || {
        fmul_f32(black_box(&fa), black_box(&fb), black_box(&mut out_f))
    });
    time("quotient by 3 half_even   value", SLOW, N, || {
        qdiv3_value(black_box(&vqa), black_box(&mut out_q))
    });
    time("quotient by 3 half_even   hand", FAST, N, || {
        qdiv3_hand(black_box(&qa), black_box(&mut out_i32))
    });
    time("convert to Q8.8 half_even value", SLOW, N, || {
        qconv_value(black_box(&vqa), black_box(&mut out_q8))
    });
    time("convert to Q8.8 half_even hand", FAST, N, || {
        qconv_hand(black_box(&qa), black_box(&mut out_i16))
    });

    println!("-- a dependent chain: one Q16.16 value decayed 30000 times, per step");
    time("decay half_even saturate value", 21, 30_000, || {
        black_box(decay_value(black_box(start), black_box(factor), 30_000));
    });
    time("decay half_even saturate hand", 201, 30_000, || {
        black_box(decay_hand(black_box(1000 << 16), black_box(65530), 30_000));
    });
}
