// Probe P1. Can a Rust program under I14 distinguish two extensionally equal
// chain implementations, and does every channel that can require a BINDING to an
// intermediate?
//
// WHY THIS DECIDES SOMETHING. 167's delimiter says a region ends where an
// intermediate is named. If every channel that distinguishes representations
// needs a binding, then the naming perimeter and the distinguishing perimeter
// coincide, and the delimiter is a fact about this language under these
// constraints rather than a licence granted by a rule. If some channel
// distinguishes with no binding, the coincidence fails and the delimiter needs
// something the language does not supply.
//
// THE CASES THAT MUST FAIL, stated before the run.
//   C-A  A channel given a binding MUST distinguish. If nothing distinguishes
//        even with a binding, "requires a binding" is vacuous and this probe has
//        measured nothing.
//   C-B  Two IDENTICAL implementations must be indistinguishable on every
//        channel. A difference there is the instrument inventing one.
//   C-C  The two implementations must agree on the final value over the swept
//        domain. If they do not, they are not extensionally equal and the whole
//        question is malformed.
//
// The two implementations: `wide` computes (a + b) - c through an i64
// intermediate and narrows once. `narrow` computes it in i32 throughout with
// wrapping. Both are the same function of (a, b, c) whenever the true result
// fits in i32, because arithmetic mod 2^32 is a ring: an overflow in `a + b`
// cancels in the subtraction.

use core::mem::size_of_val;

#[inline(never)]
fn wide(a: i32, b: i32, c: i32) -> i32 {
    let t: i64 = a as i64 + b as i64; // intermediate at 64 bits
    (t - c as i64) as i32
}

#[inline(never)]
fn narrow(a: i32, b: i32, c: i32) -> i32 {
    let t: i32 = a.wrapping_add(b); // intermediate at 32 bits, may wrap
    t.wrapping_sub(c)
}

/// C-B's pair: byte-identical bodies, so every channel must report no difference.
#[inline(never)]
fn wide_twin(a: i32, b: i32, c: i32) -> i32 {
    let t: i64 = a as i64 + b as i64;
    (t - c as i64) as i32
}

/// The debug-only overflow channel, isolated. `a + b` with the checking operator.
/// In a debug build this panics on overflow; in a release build it wraps.
#[inline(never)]
fn narrow_checked(a: i32, b: i32, c: i32) -> i32 {
    let t: i32 = a + b; // NOT wrapping_add: this is the channel
    t.wrapping_sub(c)
}

fn profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn main() {
    println!("== Probe P1, build profile = {} ==", profile());
    println!();

    // ---------------------------------------------------------------
    // C-C: extensional agreement over the swept domain.
    // ---------------------------------------------------------------
    let mut checked = 0u64;
    let mut disagree = 0u64;
    let mut overflowed = 0u64;
    let mut z: i64 = 0x243F_6A88_85A3_08D3;
    for _ in 0..2_000_000u64 {
        // xorshift64*
        z ^= z >> 12;
        z ^= z << 25;
        z ^= z >> 27;
        let r = (z as u64).wrapping_mul(0x2545_F491_4F6C_DD1D);
        let a = (r >> 32) as i32;
        let b = (r as u32 as i32) >> 1;
        let c = ((r >> 16) as u32 as i32) >> 1;
        // restrict to the region where the true result fits in i32
        let truth = a as i64 + b as i64 - c as i64;
        if truth < i32::MIN as i64 || truth > i32::MAX as i64 {
            continue;
        }
        checked += 1;
        if a.checked_add(b).is_none() {
            overflowed += 1;
        }
        if wide(a, b, c) != narrow(a, b, c) {
            disagree += 1;
        }
    }
    println!("C-C  inputs where the true result fits in i32: {checked}");
    println!("C-C  of those, inputs where the narrow intermediate overflows: {overflowed}");
    println!("C-C  final-value disagreements between wide and narrow: {disagree}   (must be 0)");
    println!(
        "C-C  {}",
        if disagree == 0 && checked > 0 && overflowed > 0 {
            "ok: the pair is extensionally equal on this domain AND the overflow case is exercised"
        } else {
            "FAIL"
        }
    );
    println!();

    // ---------------------------------------------------------------
    // Channels that need a binding to the intermediate.
    // ---------------------------------------------------------------
    println!("-- channels requiring a BINDING to the intermediate --");
    let (a, b, c) = (1_500_000_000i32, 1_400_000_000i32, 2_000_000_000i32);

    let t_wide: i64 = a as i64 + b as i64;
    let t_narrow: i32 = a.wrapping_add(b);
    println!(
        "size_of_val(intermediate):      wide {}   narrow {}   distinguishes: {}",
        size_of_val(&t_wide),
        size_of_val(&t_narrow),
        size_of_val(&t_wide) != size_of_val(&t_narrow)
    );
    println!(
        "Debug of intermediate:          wide {:?}   narrow {:?}   distinguishes: {}",
        t_wide,
        t_narrow,
        format!("{t_wide:?}") != format!("{t_narrow:?}")
    );
    println!(
        "align_of_val(intermediate):     wide {}   narrow {}   distinguishes: {}",
        core::mem::align_of_val(&t_wide),
        core::mem::align_of_val(&t_narrow),
        core::mem::align_of_val(&t_wide) != core::mem::align_of_val(&t_narrow)
    );
    let c_a = size_of_val(&t_wide) != size_of_val(&t_narrow);
    println!("C-A  a channel given a binding distinguishes: {c_a}   (must be true, else vacuous)");
    println!();

    // ---------------------------------------------------------------
    // C-B: the identical twin must be indistinguishable everywhere.
    // ---------------------------------------------------------------
    let mut twin_disagree = 0u64;
    let mut z2: i64 = 0x13198A2E_03707344;
    for _ in 0..200_000u64 {
        z2 ^= z2 >> 12;
        z2 ^= z2 << 25;
        z2 ^= z2 >> 27;
        let r = (z2 as u64).wrapping_mul(0x2545_F491_4F6C_DD1D);
        let a = (r >> 32) as i32 >> 1;
        let b = (r as u32 as i32) >> 1;
        let cc = ((r >> 16) as u32 as i32) >> 1;
        if wide(a, b, cc) != wide_twin(a, b, cc) {
            twin_disagree += 1;
        }
    }
    let t_twin: i64 = a as i64 + b as i64;
    println!("-- C-B, the identical twin --");
    println!("C-B  final-value disagreements: {twin_disagree}   (must be 0)");
    println!(
        "C-B  size_of_val differs: {}   (must be false)",
        size_of_val(&t_wide) != size_of_val(&t_twin)
    );
    println!();

    // ---------------------------------------------------------------
    // The hunt: a channel with NO binding to any intermediate.
    // ---------------------------------------------------------------
    println!("-- the hunt: a channel with NO binding to any intermediate --");
    println!(
        "input (a, b, c) = ({a}, {b}, {c}); a + b overflows i32: {}",
        a.checked_add(b).is_none()
    );
    println!(
        "true result {} fits in i32: {}",
        a as i64 + b as i64 - c as i64,
        (a as i64 + b as i64 - c as i64) >= i32::MIN as i64
            && (a as i64 + b as i64 - c as i64) <= i32::MAX as i64
    );

    let r_wide = std::panic::catch_unwind(|| wide(a, b, c));
    let r_checked = std::panic::catch_unwind(|| narrow_checked(a, b, c));
    println!(
        "wide(a,b,c)           -> {:?}",
        r_wide.as_ref().map(|v| *v).map_err(|_| "PANIC")
    );
    println!(
        "narrow_checked(a,b,c) -> {:?}",
        r_checked.as_ref().map(|v| *v).map_err(|_| "PANIC")
    );
    let distinguished = r_wide.is_ok() != r_checked.is_ok();
    println!();
    println!(
        "BINDING-FREE CHANNEL FOUND AT profile = {}: {}",
        profile(),
        distinguished
    );
    println!("  the caller binds only the FINAL value; no name exists for either intermediate,");
    println!("  and the two implementations are still told apart by whether the program panics.");
}
