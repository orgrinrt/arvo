//! Probe 03: which law does hilavitkutin's parallel merge actually need, and
//! does the shipped merge already violate one?
//!
//! Three merge shapes exist in the engine, at three different stages of
//! reality. Each is modelled below and checked against the one contract the
//! engine states for itself, which `gate2_accumulator.rs:1-8` writes down:
//! the parallel result "must be byte-identical to the single-core `run()`
//! append: same values, same order."
//!
//!   (a) SHIPPED. `MergeAccums::merge_accums`
//!       (hilavitkutin/src/resource/bindings.rs:736-769) forward-compacts each
//!       core's region in core-index order. Core `c` owns records
//!       `[c*per, (c+1)*per)` (bindings.rs:753), so index order is record
//!       order. This is list concatenation: partials combined in sequence
//!       order, never reordered.
//!
//!   (b) SHIPPED, unconsumed. `ConvergenceBuffer::combine(init, fn(T,T)->T)`
//!       (hilavitkutin/src/resource/accumulator.rs:50-58) left-folds N slots
//!       in slot order starting from a caller-supplied `init`. Nothing bounds
//!       the combiner. Nothing relates `init` to it.
//!
//!   (c) DESIGNED, not implemented. Head+tail convergence: "Accumulator on the
//!       tail walker (units flowing backward)" (plan/fiber.rs:159), merged by
//!       `MergeOp` (fiber.rs:95-110), gated on `WorkUnit::COMMUTATIVE`
//!       (fiber.rs:150).
//!
//! Run: rustc -O 03_what_the_scheduler_needs.rs -o /tmp/p03 && /tmp/p03

const MIN: i32 = -4;
const MAX: i32 = 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Arith {
    Wrap,     // Hot
    Saturate, // Warm / Cold
    Exact,
}

fn q(a: Arith, x: i32) -> i32 {
    match a {
        Arith::Wrap => ((x - MIN).rem_euclid(MAX - MIN + 1)) + MIN,
        Arith::Saturate => x.clamp(MIN, MAX),
        Arith::Exact => x,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Op {
    Add, // AccumType::Sum / MergeOp::Add
    Max, // AccumType::Max / MergeOp::Max
}

fn apply(a: Arith, o: Op, x: i32, y: i32) -> i32 {
    match o {
        Op::Add => q(a, x + y),
        Op::Max => x.max(y),
    }
}

/// The zero the engine hands `ConvergenceBuffer::new` and `combine`.
/// `resource_accumulator.rs:22,30,40` passes literal 0 for every case.
const ENGINE_ZERO: i32 = 0;

// ---------------------------------------------------------------------------
// The reference: what one thread doing the obvious thing computes.
// ---------------------------------------------------------------------------

fn sequential(a: Arith, o: Op, init: i32, xs: &[i32]) -> i32 {
    xs.iter().fold(init, |acc, &x| apply(a, o, acc, x))
}

/// (a)/(b): contiguous chunks, partials folded in chunk order.
fn chunked_in_order(a: Arith, o: Op, init: i32, xs: &[i32], cuts: &[usize]) -> i32 {
    let mut partials = Vec::new();
    let mut lo = 0;
    for &c in cuts.iter().chain(core::iter::once(&xs.len())) {
        partials.push(sequential(a, o, init, &xs[lo..c]));
        lo = c;
    }
    partials
        .into_iter()
        .fold(init, |acc, p| apply(a, o, acc, p))
}

/// (c): head walks `[0, mid)` forward, tail walks `[mid, n)` BACKWARD, merged.
fn head_tail(a: Arith, o: Op, init: i32, xs: &[i32]) -> i32 {
    let mid = xs.len() / 2;
    let head = sequential(a, o, init, &xs[..mid]);
    let mut tail = init;
    for &x in xs[mid..].iter().rev() {
        tail = apply(a, o, tail, x);
    }
    apply(a, o, head, tail)
}

/// (b) with idle slots: `ConvergenceBuffer<T, N>` always folds all N slots
/// (accumulator.rs:53 `while i < N`), including ones no core wrote, which
/// still hold the constructor's `zero`.
fn chunked_with_idle_slots(
    a: Arith,
    o: Op,
    init: i32,
    xs: &[i32],
    cuts: &[usize],
    slots: usize,
) -> i32 {
    let mut partials = Vec::new();
    let mut lo = 0;
    for &c in cuts.iter().chain(core::iter::once(&xs.len())) {
        partials.push(sequential(a, o, init, &xs[lo..c]));
        lo = c;
    }
    while partials.len() < slots {
        partials.push(ENGINE_ZERO);
    }
    partials
        .into_iter()
        .fold(init, |acc, p| apply(a, o, acc, p))
}

// ---------------------------------------------------------------------------

fn all_cuts(n: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for mask in 0u32..(1u32 << (n.saturating_sub(1))) {
        let mut c = Vec::new();
        for i in 1..n {
            if mask >> (i - 1) & 1 == 1 {
                c.push(i);
            }
        }
        out.push(c);
    }
    out
}

fn inputs(len: usize) -> Vec<Vec<i32>> {
    let grid = [MIN, -1, 0, 1, MAX];
    let mut out = vec![Vec::new()];
    for _ in 0..len {
        let mut next = Vec::new();
        for base in &out {
            for g in grid {
                let mut v = base.clone();
                v.push(g);
                next.push(v);
            }
        }
        out = next;
    }
    out
}

fn law_assoc(a: Arith, o: Op) -> bool {
    (MIN..=MAX).all(|x| {
        (MIN..=MAX).all(|y| {
            (MIN..=MAX)
                .all(|z| apply(a, o, apply(a, o, x, y), z) == apply(a, o, x, apply(a, o, y, z)))
        })
    })
}

fn law_commut(a: Arith, o: Op) -> bool {
    (MIN..=MAX).all(|x| (MIN..=MAX).all(|y| apply(a, o, x, y) == apply(a, o, y, x)))
}

fn law_identity(a: Arith, o: Op, e: i32) -> bool {
    (MIN..=MAX).all(|x| apply(a, o, e, x) == x && apply(a, o, x, e) == x)
}

fn main() {
    const LEN: usize = 4;
    let cases = inputs(LEN);
    let cuts = all_cuts(LEN);
    println!(
        "model: representable [{}, {}], sequences of {} over a 5-point grid ({} of them), {} partitions\n",
        MIN, MAX, LEN, cases.len(), cuts.len()
    );

    println!(
        "{:<20} {:<7} {:<7} {:<9} | {:<12} {:<12} {:<12}",
        "arith / op",
        "assoc",
        "commut",
        "0 is id",
        "(a)+(b) order",
        "(c) head+tail",
        "(b) idle slots"
    );

    for a in [Arith::Wrap, Arith::Saturate, Arith::Exact] {
        for o in [Op::Add, Op::Max] {
            let init = ENGINE_ZERO;
            let mut bad_chunk = None;
            let mut bad_ht = None;
            let mut bad_idle = None;
            for xs in &cases {
                let want = sequential(a, o, init, xs);
                for c in &cuts {
                    if bad_chunk.is_none() && chunked_in_order(a, o, init, xs, c) != want {
                        bad_chunk = Some((xs.clone(), c.clone()));
                    }
                    // 8 slots, however many cores actually ran.
                    if bad_idle.is_none() && chunked_with_idle_slots(a, o, init, xs, c, 8) != want {
                        bad_idle = Some((xs.clone(), c.clone()));
                    }
                }
                if bad_ht.is_none() && head_tail(a, o, init, xs) != want {
                    bad_ht = Some(xs.clone());
                }
            }
            let f = |b: bool| if b { "yes" } else { "NO" };
            println!(
                "{:<20} {:<7} {:<7} {:<9} | {:<12} {:<12} {:<12}",
                format!("{:?} / {:?}", a, o),
                f(law_assoc(a, o)),
                f(law_commut(a, o)),
                f(law_identity(a, o, ENGINE_ZERO)),
                f(bad_chunk.is_none()),
                f(bad_ht.is_none()),
                f(bad_idle.is_none()),
            );
            if let Some((xs, c)) = &bad_chunk {
                println!(
                    "    (a) counterexample: xs={:?} cuts={:?} seq={} par={}",
                    xs,
                    c,
                    sequential(a, o, init, xs),
                    chunked_in_order(a, o, init, xs, c)
                );
            }
            if let Some(xs) = &bad_ht {
                println!(
                    "    (c) counterexample: xs={:?} seq={} par={}",
                    xs,
                    sequential(a, o, init, xs),
                    head_tail(a, o, init, xs)
                );
            }
            if let Some((xs, c)) = &bad_idle {
                if bad_chunk.is_none() {
                    println!(
                        "    (b) IDLE-SLOT counterexample (partition is fine, the unused slots are not):\n        xs={:?} cuts={:?} seq={} par={}",
                        xs, c, sequential(a, o, init, xs),
                        chunked_with_idle_slots(a, o, init, xs, c, 8)
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------
    // Separating associativity from commutativity. Every op above is
    // commutative, so none of them can tell the two laws apart. Sequence
    // concatenation is associative, NOT commutative, has an identity, and is
    // exactly what the shipped `merge_accums` performs on `Accum<T>` regions
    // (bindings.rs:753-765 memmoves each core's live prefix forward in core
    // order). Model it directly.
    // -----------------------------------------------------------------
    println!("\nassociative but NOT commutative: sequence concatenation (the shipped Accum merge)");
    {
        let xs: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let seq: Vec<i32> = xs.clone();

        // (a) contiguous chunks, concatenated in chunk order.
        let mut ok_a = true;
        for cut in 1..xs.len() {
            let mut got: Vec<i32> = Vec::new();
            got.extend_from_slice(&xs[..cut]);
            got.extend_from_slice(&xs[cut..]);
            if got != seq {
                ok_a = false;
            }
        }

        // (c) head forward over [0,mid), tail BACKWARD over [mid,n), merged.
        let mid = xs.len() / 2;
        let head: Vec<i32> = xs[..mid].to_vec();
        let tail: Vec<i32> = xs[mid..].iter().rev().copied().collect();
        let mut ht = head.clone();
        ht.extend_from_slice(&tail);

        println!("    sequential           = {:?}", seq);
        println!(
            "    (a) chunk + in-order = {}   (every cut point checked)",
            if ok_a { "matches" } else { "DIFFERS" }
        );
        println!(
            "    (c) head+tail        = {:?}   {}",
            ht,
            if ht == seq { "matches" } else { "DIFFERS" }
        );
        println!("    so (a) needs associativity + identity and nothing more;");
        println!("    (c) additionally needs commutativity, which is what");
        println!("    `WorkUnit::COMMUTATIVE` gates at plan/fiber.rs:150.");
    }

    println!("\nDirect reproduction against the shipped ConvergenceBuffer contract:");
    println!("  accumulator.rs:50  pub fn combine(&self, init: T, combine: fn(T, T) -> T) -> T");
    println!("  the shipped test convergence_buffer_combine_max uses T = u32 and values");
    println!("  5, 2, 9, 1, for which 0 happens to be the identity of max. The same call");
    println!("  with a signed payload and four cores of which two ran:");
    let slots_signed = [-3i32, -1, 0, 0];
    let combined = slots_signed.iter().fold(0i32, |a, &b| a.max(b));
    println!(
        "    slots = {:?} (two live, two never written)",
        &slots_signed[..2]
    );
    println!(
        "    combine(0, max) = {}   true max of the live slots = {}",
        combined, -1
    );
    println!("  0 is not the identity of max over a signed numeral, and the fold covers");
    println!("  all N slots whether or not a core wrote them (accumulator.rs:53).");
}
