// PROBE 2. Op's acceptance criterion: "have the typestate derive the
// matching container and numeral representations, then validate, and
// erase." This probe checks whether that pattern extends to a claimed
// ALGEBRAIC FACT (representability of a multiplicative identity), not just
// to a container width, and whether it is expressible under the pinned
// nightly without any forbidden feature (no generic_const_exprs, no
// generic_const_args, no full specialization, no -Znext-solver=globally).
//
// The format: I integer bits, F fractional bits, raw value packed in a u64
// for probe simplicity (no strategy dispatch; that is a separate axis this
// probe does not need to touch to make its point).
//
// "one" is DERIVED from (I, F) rather than independently declared as a
// constant: raw_one = 1 << F is only a valid encoding of the value 1 when
// I >= 1 (a domain with zero integer bits is [0, 1), and 1 is outside it,
// provably, from the format's own denotation rule, not from a separately
// asserted fact about the type).

const fn derive_raw_one<const I: u32, const F: u32>() -> u64 {
    // VALIDATE, at the point of derivation, per instantiation: this block is
    // only evaluated for <I, F> combinations that are actually monomorphized
    // (i.e. actually called somewhere reachable), not for every combination
    // the type could in principle carry. See probe2b for the failing case.
    const {
        assert!(
            I >= 1,
            "format has no integer bit: the value 1 is outside [0, 1), so ONE cannot be derived for this format"
        );
    }
    1u64 << F
}

fn main() {
    // A domain that includes 1.0 (I=4 integer bits, F=4 fractional bits).
    // This monomorphization is used, so VALIDATE runs at compile time, and
    // having compiled, ERASE has already happened: the check contributes
    // nothing to the emitted function body (checked separately by reading
    // the emitted assembly, see probe2_asm_check.txt).
    let one_4_4 = derive_raw_one::<4, 4>();
    println!("derive_raw_one::<I=4,F=4>() = {}", one_4_4);

    // A wider domain, same claim, different instantiation. Nothing here is
    // an independently declared "one" per width; every call site derives
    // its own from the same one rule.
    let one_11_5 = derive_raw_one::<11, 5>();
    println!("derive_raw_one::<I=11,F=5>() = {}", one_11_5);
}
