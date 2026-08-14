//! p2c. A sound, conservative, const-checkable predicate for the headroom axis.
//!
//! ## Where p2b landed
//!
//! p2b's two-state automaton missed 77 of 729 chains. The misses split two ways
//! and the split is the whole point:
//!
//!   - predicted unobservable, measured observable. **Unsound.** A resolver
//!     acting on this verdict moves an axis the consumer can see. Every
//!     instance involved `sat_at_C`, whose result reads the container width,
//!     so the two arms disagree modulo the declared width immediately rather
//!     than merely holding different high bits.
//!   - predicted observable, measured unobservable. **Conservative.** The
//!     resolver declines a freedom it actually had. `sat_at_C -> sat_at_W ->
//!     wadd` is one: `sat_at_W` clamps both arms to the same value and
//!     re-synchronises a divergence that had already happened.
//!
//! The second kind cannot be removed by any fixed number of static bits per
//! operation, because re-synchronisation depends on the relationship the
//! accumulators happen to be in, which is a fact about the history rather than
//! about the next operation. The first kind can be removed, by one more bit.
//!
//! ## What this version tests
//!
//! Three static bits per operation, all measured rather than asserted, and a
//! three-state automaton over them:
//!
//!   CONGRUENT      reduction to the declared width descends through it
//!   CONTRACTING    on declared-width operands the result is declared-width
//!   CONTAINER-READ its result differs between two containers on operands both
//!                  containers hold identically
//!
//!     state := IDENTICAL
//!     for op in chain:
//!         if CONTAINER-READ(op):                    state := DIVERGED
//!         else if state = DIVERGED:                 state := DIVERGED
//!         else if state = CONGRUENT_ONLY and not CONGRUENT(op):
//!                                                   state := DIVERGED
//!         else if CONTRACTING(op):                  state := IDENTICAL
//!         else:                                     state := CONGRUENT_ONLY
//!     observable := (state = DIVERGED)
//!
//! The claim being tested is **soundness**, not exactness: the automaton may
//! say observable where measurement says otherwise, and may never say
//! unobservable where measurement says observable. Both directions are counted
//! separately, because a predicate whose error rate is reported as one number
//! has hidden which half of it is the dangerous half.
//!
//! Swept at two chain lengths over the full alphabet, exhaustively over every
//! value tuple at each. Not a sample of either.
//!
//! Spike. `std` for printing; no `dyn`, no `TypeId`, no feature gate.
//! Build: rustc -O --edition 2021 p2c_a_sound_conservative_predicate.rs

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    WAdd,
    WSub,
    WMul,
    Shr,
    Div,
    SatAtW,
    SatAtC,
    Min,
    Cmp,
}

const ALL: [Op; 9] = [
    Op::WAdd,
    Op::WSub,
    Op::WMul,
    Op::Shr,
    Op::Div,
    Op::SatAtW,
    Op::SatAtC,
    Op::Min,
    Op::Cmp,
];

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::WAdd => "wadd",
            Op::WSub => "wsub",
            Op::WMul => "wmul",
            Op::Shr => "shr",
            Op::Div => "div",
            Op::SatAtW => "sat_at_W",
            Op::SatAtC => "sat_at_C",
            Op::Min => "min",
            Op::Cmp => "cmp",
        }
    }
    fn raw(self, a: u64, x: u64, w: u32, cmask: u64) -> u64 {
        let wmax = (1u64 << w) - 1;
        match self {
            Op::WAdd => a.wrapping_add(x),
            Op::WSub => a.wrapping_sub(x),
            Op::WMul => a.wrapping_mul(x),
            Op::Shr => a >> (x & 3),
            Op::Div => a / (x | 1),
            Op::SatAtW => {
                let s = a.wrapping_add(x);
                if s > wmax { wmax } else { s }
            }
            Op::SatAtC => {
                let s = a.wrapping_add(x);
                if s > cmask { cmask } else { s }
            }
            Op::Min => if a < x { a } else { x },
            Op::Cmp => if a > x { a - x } else { x - a },
        }
    }
    fn step(self, a: u64, x: u64, w: u32, c: u32) -> u64 {
        let cmask = mask(c);
        self.raw(a, x, w, cmask) & cmask
    }
}

fn mask(c: u32) -> u64 {
    if c >= 64 { u64::MAX } else { (1u64 << c) - 1 }
}
fn idx_of(o: Op) -> usize {
    ALL.iter().position(|&p| p == o).unwrap()
}

fn measure_congruent(op: Op, w: u32, c: u32) -> bool {
    let wsz = 1usize << w;
    let csz = 1u64 << c;
    let wmask = mask(w);
    let mut seen: Vec<i64> = vec![-1; wsz * wsz];
    for a in 0..csz {
        for x in 0..csz {
            let r = (op.step(a, x, w, c) & wmask) as i64;
            let key = ((a & wmask) as usize) * wsz + ((x & wmask) as usize);
            if seen[key] < 0 {
                seen[key] = r;
            } else if seen[key] != r {
                return false;
            }
        }
    }
    true
}

fn measure_contracting(op: Op, w: u32, c: u32) -> bool {
    let wsz = 1u64 << w;
    for a in 0..wsz {
        for x in 0..wsz {
            if op.step(a, x, w, c) > wsz - 1 {
                return false;
            }
        }
    }
    true
}

/// Does the operation's own result differ between the two containers, on
/// operands that both containers hold identically? That is the bit p2b lacked.
fn measure_container_read(op: Op, w: u32, c_min: u32, c_wide: u32) -> bool {
    let wsz = 1u64 << w;
    for a in 0..wsz {
        for x in 0..wsz {
            let lo = op.step(a, x, w, c_min) & mask(w);
            let hi = op.step(a, x, w, c_wide) & mask(w);
            if lo != hi {
                return true;
            }
        }
    }
    false
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum St {
    Identical,
    CongruentOnly,
    Diverged,
}

fn predict(chain: &[Op], congruent: &[bool], contracting: &[bool], creads: &[bool]) -> bool {
    let mut st = St::Identical;
    for &op in chain {
        let i = idx_of(op);
        st = if creads[i] {
            St::Diverged
        } else if st == St::Diverged {
            St::Diverged
        } else if st == St::CongruentOnly && !congruent[i] {
            St::Diverged
        } else if contracting[i] {
            St::Identical
        } else {
            St::CongruentOnly
        };
    }
    st == St::Diverged
}

fn measure_observable(chain: &[Op], w: u32, c_min: u32, c_wide: u32) -> u64 {
    let len = chain.len() + 1;
    let domain = 1u64 << w;
    let wmask = mask(w);
    let mut idx = vec![0u64; len];
    let mut differing = 0u64;
    loop {
        let mut a_min = idx[0];
        let mut a_wide = idx[0];
        for (k, &op) in chain.iter().enumerate() {
            a_min = op.step(a_min, idx[k + 1], w, c_min);
            a_wide = op.step(a_wide, idx[k + 1], w, c_wide);
        }
        if (a_min & wmask) != (a_wide & wmask) {
            differing += 1;
        }
        let mut i = 0;
        loop {
            if i == len {
                return differing;
            }
            idx[i] += 1;
            if idx[i] < domain {
                break;
            }
            idx[i] = 0;
            i += 1;
        }
    }
}

fn sweep(w: u32, c_min: u32, c_wide: u32, chain_len: usize, label: &str) -> (u32, u32, u32) {
    let n = ALL.len();
    let congruent: Vec<bool> = ALL.iter().map(|&o| measure_congruent(o, w, c_wide)).collect();
    let contracting: Vec<bool> = ALL
        .iter()
        .map(|&o| measure_contracting(o, w, c_min) && measure_contracting(o, w, c_wide))
        .collect();
    let creads: Vec<bool> = ALL
        .iter()
        .map(|&o| measure_container_read(o, w, c_min, c_wide))
        .collect();

    println!("--- {label}: W = {w}, containers {c_min} against {c_wide}, chain length {chain_len}");
    println!();
    println!("{:<10} {:<11} {:<13} {:<15}", "op", "congruent", "contracting", "container-read");
    for (i, &op) in ALL.iter().enumerate() {
        println!(
            "{:<10} {:<11} {:<13} {:<15}",
            op.name(),
            congruent[i],
            contracting[i],
            creads[i]
        );
    }
    println!();

    let total = n.pow(chain_len as u32);
    let mut unsound = 0u32;
    let mut conservative = 0u32;
    let mut exact = 0u32;
    let mut unsound_examples: Vec<String> = Vec::new();
    for code in 0..total {
        let mut chain = Vec::with_capacity(chain_len);
        let mut r = code;
        for _ in 0..chain_len {
            chain.push(ALL[r % n]);
            r /= n;
        }
        let p = predict(&chain, &congruent, &contracting, &creads);
        let m = measure_observable(&chain, w, c_min, c_wide) > 0;
        if p == m {
            exact += 1;
        } else if !p && m {
            unsound += 1;
            if unsound_examples.len() < 12 {
                let names: Vec<&str> = chain.iter().map(|o| o.name()).collect();
                unsound_examples.push(names.join(" -> "));
            }
        } else {
            conservative += 1;
        }
    }
    println!("chains swept: {total}, each exhaustive over {} value tuples", (1u64 << w).pow(chain_len as u32 + 1));
    println!("  exact:                        {exact}");
    println!("  conservative (safe direction): {conservative}");
    println!("  UNSOUND (unsafe direction):    {unsound}");
    if !unsound_examples.is_empty() {
        println!("  unsound examples:");
        for e in &unsound_examples {
            println!("    {e}");
        }
    }
    println!();
    (exact, conservative, unsound)
}

fn main() {
    println!("p2c. A sound, conservative, const-checkable predicate for the headroom axis.");
    println!();
    println!("Soundness here means: never predict UNOBSERVABLE where measurement finds");
    println!("the axis observable. That is the direction a resolver acts on, so it is");
    println!("the only direction whose error rate may be zero rather than small.");
    println!();

    let (_e1, _c1, u1) = sweep(4, 4, 12, 3, "setting one");
    let (_e2, _c2, u2) = sweep(4, 4, 12, 4, "setting two, one operation longer");
    let (_e3, _c3, u3) = sweep(5, 5, 13, 3, "setting three, a different width and container");

    println!("=== verdict ===");
    println!();
    println!("total unsound predictions across three settings: {}", u1 + u2 + u3);
    println!();
    println!("The predicate consumes three bits per operation, every one of them a");
    println!("compile-time property of the operation rather than a measurement of a");
    println!("machine, and a three-state scan of the chain. Nothing in it needs the");
    println!("values, so it is available wherever the chain is written.");
    println!();
    println!("What it does NOT deliver is exactness, and the residue is structural:");
    println!("an operation that clamps at the declared width re-synchronises two arms");
    println!("that had already diverged, and no fixed set of per-operation bits sees");
    println!("that, because it is a fact about the relationship the accumulators are");
    println!("in rather than about the next operation.");
}
