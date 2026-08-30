// PROBE 2b. Companion to probe2: what happens when the pathological
// instantiation (I=0, a format with no representable value >= 1) is actually
// used. This file is expected to FAIL TO COMPILE, and the failure is the
// result: per the workspace's own instrument, a contract that cannot be
// compiled is a stronger finding than a runtime assertion failure, because it
// says the claim ("this format has a representable one") has no expressible
// truth value for this instantiation rather than merely a wrong one.

const fn derive_raw_one<const I: u32, const F: u32>() -> u64 {
    const {
        assert!(
            I >= 1,
            "format has no integer bit: the value 1 is outside [0, 1), so ONE cannot be derived for this format"
        );
    }
    1u64 << F
}

fn main() {
    // I = 0: the domain is [0, 1). This call reaches the pathological
    // monomorphization and is expected to refuse to compile.
    let bad = derive_raw_one::<0, 8>();
    println!("{}", bad);
}
