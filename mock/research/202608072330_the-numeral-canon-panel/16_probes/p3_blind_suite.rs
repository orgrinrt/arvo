// p3: the suite that is green over a derivation that has lost half its answer.
//
// Checks ONE thing: that the obvious per-value correctness checks all pass against a
// carrier-only derivation of a Cold numeral, and that the two checks which catch it are both
// checks nobody writes unless they already believe a second output exists.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p3_blind_suite.rs -o bin/p3 && ./bin/p3
//
// Nothing here is timed. Sizes are compile-time; the phase round trip is a correctness run.
//
// Spike. Presume it flawed. In particular the "codegen equality" check below is a weak stand-in
// for the panel's real one, which compares emitted symbols; see the note where it appears.

use std::mem::size_of;

const W: u32 = 13; // the declared width
const N: usize = 1000; // elements in the contiguous run
const MASK: u32 = (1u32 << W) - 1;

// ============================================================================
// The one-output derivation: the answer is a carrier, and that is the whole answer.
// ============================================================================
type Carrier = u16;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Num(Carrier);

impl Num {
    const fn from_raw(r: Carrier) -> Self {
        Num(r)
    }
    const fn raw(self) -> Carrier {
        self.0
    }
    // wrapping add at the declared width, the way a Warm-ish numeral would
    const fn add(self, o: Self) -> Self {
        Num(((self.0 as u32 + o.0 as u32) & MASK) as Carrier)
    }
}

// The native reference the erasure check compares against.
#[inline(never)]
fn native_add(a: u16, b: u16) -> u16 {
    ((a as u32 + b as u32) & MASK) as u16
}
#[inline(never)]
fn derived_add(a: Num, b: Num) -> Num {
    a.add(b)
}

// ============================================================================
// The checks a reasonable person writes. Every one of these is green.
// ============================================================================

fn check_round_trip() -> Result<String, String> {
    for r in 0..=MASK {
        let n = Num::from_raw(r as Carrier);
        if n.raw() as u32 != r {
            return Err(format!("round trip failed at {r}"));
        }
    }
    Ok(format!(
        "raw(from_raw(r)) == r for all {} representable r",
        MASK + 1
    ))
}

fn check_arithmetic_agreement() -> Result<String, String> {
    let mut n = 0u64;
    let mut a = 0u32;
    while a <= MASK {
        let mut b = 0u32;
        while b <= MASK {
            let d = derived_add(Num::from_raw(a as u16), Num::from_raw(b as u16)).raw();
            let r = native_add(a as u16, b as u16);
            if d != r {
                return Err(format!("disagreement at ({a}, {b}): {d} vs {r}"));
            }
            n += 1;
            b += 419; // stride the inner loop; full cross is 67M and this probe is not a bench
        }
        a += 1;
    }
    Ok(format!("derived == native over {n} pairs"))
}

fn check_codegen_equality() -> Result<String, String> {
    // NOTE: this is a weak stand-in. The panel's real erasure check compares emitted symbols
    // and reports that LLVM folds the two into one. What matters for THIS probe is not the
    // strength of the check but its ARITY: it takes one value, applies one operation, and
    // compares against one native instruction. There is no array in it and there cannot be,
    // because a native primitive has no packed-array form to compare against.
    let a = Num::from_raw(0x0AAA);
    let b = Num::from_raw(0x0555);
    if derived_add(a, b).raw() != native_add(0x0AAA, 0x0555) {
        return Err("derived operation does not match the native instruction".into());
    }
    if size_of::<Num>() != size_of::<Carrier>() {
        return Err("the newtype is not transparent".into());
    }
    Ok("one operation, one value, byte-identical to the native primitive".into())
}

fn check_size_of_matches_the_ladder() -> Result<String, String> {
    // This one is the fabricated-diligence shape. It asks the derivation to agree with itself.
    // It looks like a layout test. It is a restatement of the thing under test, and it is
    // structurally incapable of failing while the ladder and the newtype agree, which they do
    // by construction because the newtype IS the ladder's output.
    if size_of::<Num>() != 2 {
        return Err("size mismatch".into());
    }
    Ok("size_of::<Num>() == 2, which the ladder also says. tautology.".into())
}

// ============================================================================
// The two checks that catch it. Both require knowing the second output exists.
// ============================================================================

fn check_array_extent() -> Result<String, String> {
    let actual_bits = size_of::<[Num; N]>() * 8;
    let promised_bits = N * W as usize;
    if actual_bits != promised_bits {
        return Err(format!(
            "[Num; {N}] occupies {actual_bits} bits, the Cold declaration promised {promised_bits} \
             ({} bytes against {} bytes, {:.1}% over)",
            actual_bits / 8,
            promised_bits.div_ceil(8),
            (actual_bits as f64 / promised_bits as f64 - 1.0) * 100.0
        ));
    }
    Ok("array extent matches the declared width".into())
}

// A packed store, which is what the second output would let you build.
struct Packed {
    bytes: [u8; 1625], // ceil(13 * 1000 / 8), written out: a generic length is an expression
}

impl Packed {
    fn new() -> Self {
        Packed { bytes: [0u8; 1625] }
    }
    fn write(&mut self, k: usize, v: u32) {
        let bit = k * W as usize;
        for i in 0..W as usize {
            let b = bit + i;
            let set = (v >> i) & 1 == 1;
            if set {
                self.bytes[b / 8] |= 1 << (b % 8);
            } else {
                self.bytes[b / 8] &= !(1 << (b % 8));
            }
        }
    }
    // Read using an access type of the given byte width, little endian.
    fn read_with_access(&self, k: usize, access_bytes: usize) -> u32 {
        let bit = k * W as usize;
        let byte = bit / 8;
        let phase = bit % 8;
        let mut acc = 0u64;
        for i in 0..access_bytes {
            acc |= (*self.bytes.get(byte + i).unwrap_or(&0) as u64) << (8 * i);
        }
        ((acc >> phase) as u32) & MASK
    }
}

fn check_phase_round_trip(access_bytes: usize) -> Result<String, String> {
    let mut p = Packed::new();
    // one full phase cycle and then some; gcd(13, 8) = 1 so phase cycles with period 8
    let vals: Vec<u32> = (0..64u32).map(|k| (k * 617 + 1) & MASK).collect();
    for (k, &v) in vals.iter().enumerate() {
        p.write(k, v);
    }
    let mut bad: Vec<(usize, u32, u32, usize)> = Vec::new();
    for (k, &v) in vals.iter().enumerate() {
        let got = p.read_with_access(k, access_bytes);
        if got != v {
            bad.push((k, v, got, (k * W as usize) % 8));
        }
    }
    if bad.is_empty() {
        Ok(format!(
            "64 values round-tripped through a packed store with a {}-byte access",
            access_bytes
        ))
    } else {
        let phases: Vec<usize> = {
            let mut ps: Vec<usize> = bad.iter().map(|t| t.3).collect();
            ps.sort_unstable();
            ps.dedup();
            ps
        };
        Err(format!(
            "{} of 64 wrong with a {}-byte access. failing bit-phases {:?}. first: index {} wrote {} read {}",
            bad.len(),
            access_bytes,
            phases,
            bad[0].0,
            bad[0].1,
            bad[0].2
        ))
    }
}

fn report(name: &str, r: Result<String, String>) -> bool {
    match r {
        Ok(m) => {
            println!("  PASS  {name}\n          {m}");
            true
        }
        Err(m) => {
            println!("  FAIL  {name}\n          {m}");
            false
        }
    }
}

fn main() {
    println!("declaration under test: UFixed<{W}, 0, Cold>, unsigned");
    println!(
        "derivation under test : one output, the carrier, which is u{}",
        size_of::<Carrier>() * 8
    );
    println!();
    println!("the suite a reasonable person writes:");
    let mut green = 0;
    green += report("round trip", check_round_trip()) as u32;
    green += report(
        "arithmetic agreement with the native reference",
        check_arithmetic_agreement(),
    ) as u32;
    green += report(
        "erasure / codegen equality (per value)",
        check_codegen_equality(),
    ) as u32;
    green += report(
        "size_of matches the ladder",
        check_size_of_matches_the_ladder(),
    ) as u32;
    println!("  -> {green} of 4 green. the derivation is certified.");

    println!();
    println!("the two checks that catch it:");
    report("array extent equals N * W", check_array_extent());
    report(
        "packed round trip, access width = size_of(carrier) = 2 bytes",
        check_phase_round_trip(2),
    );
    report(
        "packed round trip, access width from the extent = 3 bytes",
        check_phase_round_trip(3),
    );

    println!();
    println!("the second of those is the sharper one. it is not that the carrier-only derivation");
    println!("stores wrong, it is that a site handed only the carrier picks the carrier as its");
    println!("access type, and that load is too narrow to cover a {W}-bit field at high phase.");
}
