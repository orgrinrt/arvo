//! The timing half of the spike. Every line it prints is an ad-hoc spike on one
//! shared virtual machine: a median of repeated runs with no pinning, no
//! counters and no noise gate. It orders arms of very different cost and says
//! nothing finer than that.

use std::hint::black_box;
use std::time::Instant;

use arvo_format::{Exact, Slot};
use stack_audit_value_yardsticks::*;

const N: usize = 4096;
const REPS: usize = 2001;

fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// Median nanoseconds per element over `REPS` runs of `f`.
fn time(label: &str, per: usize, mut f: impl FnMut()) {
    let mut samples = Vec::with_capacity(REPS);
    for _ in 0 .. REPS {
        let t = Instant::now();
        f();
        samples.push(t.elapsed().as_nanos() as f64);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = samples[REPS / 2];
    let low = samples[REPS / 20];
    println!(
        "{label:<34} {:>8.3} ns per element (median), {:>8.3} (5th percentile)",
        median / per as f64,
        low / per as f64
    );
}

fn main() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;

    // Small members, so a saturating fold does not sit at its fixpoint and a
    // host sum of two of them does not leave sixteen bits.
    let host_small: Vec<u16> = (0 .. N).map(|_| (xorshift(&mut seed) % 3) as u16).collect();
    let host_any: Vec<u16> = (0 .. N)
        .map(|_| (xorshift(&mut seed) % 8192) as u16)
        .collect();
    let host_half: Vec<u16> = (0 .. N)
        .map(|_| (xorshift(&mut seed) % 6000) as u16)
        .collect();

    let v_small: Vec<U13S> = host_small
        .iter()
        .map(|&x| ingest_u13_saturating(x))
        .collect();
    let w_any: Vec<U13W> = host_any.iter().map(|&x| ingest_u13_wrapping(x)).collect();
    let s_any: Vec<U13S> = host_any.iter().map(|&x| ingest_u13_saturating(x)).collect();
    let s_half: Vec<U13S> = host_half
        .iter()
        .map(|&x| ingest_u13_saturating(x))
        .collect();
    let full_any: Vec<U16W> = host_any
        .iter()
        .map(|&x| U16W::adapted(Exact::on_grid(Slot::at(x as i128))))
        .collect();

    // The answers agree before anything is timed.
    assert_eq!(
        egress_u13(fold_u13_saturating(&v_small)),
        host_fold_u13_saturating(&host_small)
    );
    assert_eq!(
        fold_u13_wrapping(&w_any).slot().index() as u16,
        host_fold_u13_wrapping(&host_any)
    );
    assert_eq!(
        fold_u16_wrapping(&full_any).slot().index() as u16,
        host_fold_u16_wrapping(&host_any)
    );
    for &x in &[0u16, 1, 8191, 8192, 40000, 65535] {
        assert_eq!(
            egress_u13(ingest_u13_saturating(x)),
            host_ingest_u13_saturating(x)
        );
        assert_eq!(
            ingest_u13_wrapping(x).slot().index() as u16,
            host_ingest_u13_wrapping(x)
        );
    }

    println!("-- the doors, a column of {N} host integers in and out");
    let mut sink = vec![s_any[0]; N];
    time("ingest u13 saturating, value", N, || {
        for (o, &x) in sink.iter_mut().zip(black_box(&host_any)) {
            *o = ingest_u13_saturating(x);
        }
        black_box(&sink);
    });
    let mut hsink = vec![0u16; N];
    time("ingest u13 saturating, host", N, || {
        for (o, &x) in hsink.iter_mut().zip(black_box(&host_any)) {
            *o = host_ingest_u13_saturating(x);
        }
        black_box(&hsink);
    });
    let mut wsink = vec![w_any[0]; N];
    time("ingest u13 wrapping, value", N, || {
        for (o, &x) in wsink.iter_mut().zip(black_box(&host_any)) {
            *o = ingest_u13_wrapping(x);
        }
        black_box(&wsink);
    });
    time("ingest u13 wrapping, host", N, || {
        for (o, &x) in hsink.iter_mut().zip(black_box(&host_any)) {
            *o = host_ingest_u13_wrapping(x);
        }
        black_box(&hsink);
    });
    time("egress u13, value", N, || {
        for (o, &x) in hsink.iter_mut().zip(black_box(&s_any)) {
            *o = egress_u13(x);
        }
        black_box(&hsink);
    });

    println!("-- the fold of {N} members");
    time("fold u13 saturating, value", N, || {
        black_box(fold_u13_saturating(black_box(&v_small)));
    });
    time("fold u13 saturating, host", N, || {
        black_box(host_fold_u13_saturating(black_box(&host_small)));
    });
    time("fold u13 wrapping, value", N, || {
        black_box(fold_u13_wrapping(black_box(&w_any)));
    });
    time("fold u13 wrapping, host", N, || {
        black_box(host_fold_u13_wrapping(black_box(&host_any)));
    });
    time("fold u16 wrapping, value", N, || {
        black_box(fold_u16_wrapping(black_box(&full_any)));
    });
    time("fold u16 wrapping, host", N, || {
        black_box(host_fold_u16_wrapping(black_box(&host_any)));
    });

    println!("-- the elementwise sum of two columns of {N}");
    let mut out = vec![s_half[0]; N];
    time("map u13 saturating, value", N, || {
        map_u13_saturating(black_box(&s_half), black_box(&s_any), &mut out);
        black_box(&out);
    });
    let mut hout = vec![0u16; N];
    time("map u13 saturating, host", N, || {
        host_map_u13_saturating(black_box(&host_half), black_box(&host_any), &mut hout);
        black_box(&hout);
    });

    println!("-- a Q16.16 product over two columns of {N}, written by hand");
    let qa: Vec<i32> = (0 .. N).map(|_| xorshift(&mut seed) as i32 >> 8).collect();
    let qb: Vec<i32> = (0 .. N).map(|_| xorshift(&mut seed) as i32 >> 8).collect();
    let fa: Vec<f32> = qa.iter().map(|&x| x as f32 / 65536.0).collect();
    let fb: Vec<f32> = qb.iter().map(|&x| x as f32 / 65536.0).collect();
    let mut qo = vec![0i32; N];
    let mut fo = vec![0f32; N];
    time("floor, wrap", N, || {
        qmul_floor_wrap(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("toward_zero, wrap", N, || {
        qmul_toward_zero_wrap(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("half_up, wrap", N, || {
        qmul_half_up_wrap(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("half_even, wrap", N, || {
        qmul_half_even_wrap(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("floor, saturate", N, || {
        qmul_floor_saturate(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("half_even, saturate", N, || {
        qmul_half_even_saturate(black_box(&qa), black_box(&qb), &mut qo);
        black_box(&qo);
    });
    time("f32 multiply", N, || {
        fmul_f32(black_box(&fa), black_box(&fb), &mut fo);
        black_box(&fo);
    });

    println!("-- a decay of one Q16.16 value, a loop-carried chain of 100000 steps");
    const STEPS: u32 = 100_000;
    let start = 1000 << 16;
    let factor = 65_530; // just under one
    time("decay floor", STEPS as usize, || {
        black_box(decay_floor(black_box(start), black_box(factor), STEPS));
    });
    time("decay toward_zero", STEPS as usize, || {
        black_box(decay_toward_zero(
            black_box(start),
            black_box(factor),
            STEPS,
        ));
    });
    time("decay half_up", STEPS as usize, || {
        black_box(decay_half_up(black_box(start), black_box(factor), STEPS));
    });
    time("decay half_even", STEPS as usize, || {
        black_box(decay_half_even(black_box(start), black_box(factor), STEPS));
    });
    time("decay half_even, saturate", STEPS as usize, || {
        black_box(decay_half_even_saturate(
            black_box(start),
            black_box(factor),
            STEPS,
        ));
    });

    println!("-- where each mode leaves the decayed value, against the real-valued answer");
    for steps in [1_000u32, 10_000, 30_000] {
        let real = 1000.0f64 * (factor as f64 / 65536.0).powi(steps as i32);
        let show = |v: i32| v as f64 / 65536.0;
        println!(
            "steps {steps:>6}: real {real:>12.6}  floor {:>12.6}  toward_zero {:>12.6}  half_up {:>12.6}  half_even {:>12.6}",
            show(decay_floor(start, factor, steps)),
            show(decay_toward_zero(start, factor, steps)),
            show(decay_half_up(start, factor, steps)),
            show(decay_half_even(start, factor, steps)),
        );
    }
}
