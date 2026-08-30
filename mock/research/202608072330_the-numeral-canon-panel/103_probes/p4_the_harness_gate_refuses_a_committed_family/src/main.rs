//! p4. Apply the harness's own cross-variant gate to a committed family and
//! see whether the committed CSVs could have been produced under it.
//!
//! This is the decisive test, because it does not ask whether I think the arms
//! differ. It asks whether **the harness's own acceptance criterion**, run on
//! the harness's own validation seeds, accepts the two arms that a committed
//! CSV records as having run together.
//!
//! The mechanism, from the pinned harness at
//! `mockspace-bench-core`/`mockspace-bench-harness` `bce17f6`, which is what
//! `mock/benches/Cargo.lock` pins:
//!
//! `validation_plan(outputs_may_differ, max_relative_error)` returns
//! `cross_variant = Some(ByteExact)` when `outputs_may_differ` is false and no
//! tolerance is declared, and `ByteExact` compares each variant's raw output
//! byte buffer against a baseline variant's, refusing on any mismatch.
//!
//! `quantiser-radix-shared` declares neither flag, so it takes both defaults:
//! `outputs_may_differ = false`, `max_relative_error = None`. Its plan is
//! therefore byte-exact cross-variant comparison.
//!
//! `quantiser-radix2` and `quantiser-radix10` are recorded together, as the two
//! arms of one run, in all four committed CSVs for the family
//! (`decimal-quantiser-radix-sweep_n{0,2,8,20}.csv`).
//!
//! So the corpus asserts that these two arms passed byte-exact cross-variant
//! comparison. This probe runs that comparison.
//!
//! The seeds are the harness's own: `Rng::new(0xCAFE_BABE_DEAD_BEEF)` iterated,
//! which is `VALIDATION_ROOT_SEED` and the splitmix-shaped `Rng` from
//! `bench-core/src/counter.rs`, reproduced here so the probe does not depend on
//! a private item. `DEFAULT_VALIDATION_SEEDS` is 100.
//!
//! `quantiser-fadd` is run as the control. It takes the same two defaults and
//! its own documentation claims its software arm is bit-exact against silicon,
//! so it should pass the identical gate. A probe where every arm fails proves
//! only that the probe is broken.

use bench_quantiser_fadd_shared::{hardware_add, software_add, AddSweep};
use bench_quantiser_radix_shared::{run_binary32, run_decimal32, Operands, RadixAdd, Results};
use mockspace_bench_core::Routine;

/// `bench-core/src/counter.rs`'s `Rng`, reproduced. Same constants, same order.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state ^= self.state >> 30;
        self.state = self.state.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        self.state ^= self.state >> 27;
        self.state = self.state.wrapping_mul(0x94D0_49BB_1331_11EB);
        self.state ^= self.state >> 31;
        self.state
    }
}

const VALIDATION_ROOT_SEED: u64 = 0xCAFE_BABE_DEAD_BEEF;
const DEFAULT_VALIDATION_SEEDS: usize = 100;

fn as_bytes<T>(v: &T) -> &[u8] {
    // SAFETY: read-only view of a `#[repr(C)]` POD for comparison only, which
    // is exactly what the harness does with the variant's output buffer.
    unsafe { core::slice::from_raw_parts(v as *const T as *const u8, core::mem::size_of::<T>()) }
}

fn first_diff(a: &[u8], b: &[u8]) -> Option<usize> {
    a.iter().zip(b.iter()).position(|(x, y)| x != y)
}

fn gate_radix<const SPREAD: usize>(seeds: &[u64]) {
    let mut mismatched = 0usize;
    let mut first: Option<(u64, usize)> = None;

    for &seed in seeds {
        let input: Operands = RadixAdd::<SPREAD>::build_input(seed);
        let mut a = Results::default();
        let mut b = Results::default();
        run_binary32(&input, &mut a);
        run_decimal32(&input, &mut b);

        let ba = as_bytes(&a);
        let bb = as_bytes(&b);
        if ba != bb {
            mismatched += 1;
            if first.is_none() {
                first = first_diff(ba, bb).map(|j| (seed, j));
            }
        }
    }

    let verdict = if mismatched == 0 { "ACCEPT" } else { "REFUSE" };
    println!(
        "  SPREAD={SPREAD:<3} seeds={:<4} byte-mismatched seeds={:<4}  gate: {verdict}",
        seeds.len(),
        mismatched
    );
    if let Some((seed, j)) = first {
        println!("           first mismatch: seed={seed} at output byte {j}");
    }
}

fn gate_fadd<const PCT: usize>(seeds: &[u64]) {
    // `quantiser-fadd-hardware` runs `hardware_add` over the array and
    // `quantiser-fadd-software` runs `software_add` over the same array. Both
    // functions are public in the shared crate, so this is the two committed
    // arms' own kernels rather than a reconstruction of them.
    let mut mismatched = 0usize;
    let mut first: Option<(u64, usize)> = None;

    for &seed in seeds {
        let input = AddSweep::<PCT>::build_input(seed);
        let mut hw = <AddSweep<PCT> as Routine>::Output::default();
        let mut sw = <AddSweep<PCT> as Routine>::Output::default();
        for i in 0..bench_quantiser_fadd_shared::N {
            hw.s[i] = hardware_add(input.a[i], input.b[i]);
            sw.s[i] = software_add(input.a[i], input.b[i]);
        }
        let bh = as_bytes(&hw);
        let bs = as_bytes(&sw);
        if bh != bs {
            mismatched += 1;
            if first.is_none() {
                first = first_diff(bh, bs).map(|j| (seed, j));
            }
        }
    }

    let verdict = if mismatched == 0 { "ACCEPT" } else { "REFUSE" };
    println!(
        "  PCT={PCT:<6} seeds={:<4} byte-mismatched seeds={:<4}  gate: {verdict}",
        seeds.len(),
        mismatched
    );
    if let Some((seed, j)) = first {
        println!("           first mismatch: seed={seed} at output byte {j}");
    }
}

fn main() {
    println!("p4. does the harness's own cross-variant gate accept a committed family?");
    println!();
    println!("gate    : validation_plan(outputs_may_differ=false, max_relative_error=None)");
    println!("          => cross_variant = Some(ByteExact), per_variant = true");
    println!("          ByteExact compares each variant's raw output bytes to a baseline");
    println!(
        "seeds   : Rng::new(0xCAFEBABEDEADBEEF) x {DEFAULT_VALIDATION_SEEDS}, the harness's own"
    );
    println!();

    let mut rng = Rng::new(VALIDATION_ROOT_SEED);
    let seeds: Vec<u64> = (0..DEFAULT_VALIDATION_SEEDS).map(|_| rng.next()).collect();

    println!("SUBJECT: quantiser-radix, arms quantiser-radix2 and quantiser-radix10");
    println!(
        "  declared flags: outputs_may_differ DEFAULT(false), max_relative_error DEFAULT(None)"
    );
    println!("  committed together in decimal-quantiser-radix-sweep_n{{0,2,8,20}}.csv");
    gate_radix::<0>(&seeds);
    gate_radix::<2>(&seeds);
    gate_radix::<8>(&seeds);
    gate_radix::<20>(&seeds);
    println!();

    println!("CONTROL: quantiser-fadd, arms quantiser-fadd-hardware and quantiser-fadd-software");
    println!(
        "  declared flags: outputs_may_differ DEFAULT(false), max_relative_error DEFAULT(None)"
    );
    println!("  committed together in quantiser-vs-fadd-subnormal-sweep_n*.csv");
    println!("  the shared crate claims the software model is bit-exact against silicon");
    gate_fadd::<0>(&seeds);
    gate_fadd::<10>(&seeds);
    gate_fadd::<25>(&seeds);
    gate_fadd::<50>(&seeds);
    gate_fadd::<75>(&seeds);
    gate_fadd::<100>(&seeds);
    println!();

    println!("READING");
    println!("  A REFUSE on the subject with an ACCEPT on the control means the gate");
    println!("  works and the subject family cannot have passed it. Its committed CSVs");
    println!("  then record a run that the harness's stated acceptance criterion would");
    println!("  not have permitted, so the criterion cannot be cited as evidence that");
    println!("  the committed arm set computes one value.");
    println!("  A REFUSE on both would mean the probe is measuring the wrong thing.");
}
