// PROBE 2 (erase check). A named, unmangled entry point so the emitted
// assembly for one specific, validated monomorphization can be read
// directly. The claim under test: once VALIDATE has passed at compile time,
// the compiled call site is a bare constant, with no branch, no panic
// machinery, no residue of the check that produced it. That is what "erase"
// has to mean for op's acceptance criterion to be more than a hope.

const fn derive_raw_one<const I: u32, const F: u32>() -> u64 {
    const {
        assert!(
            I >= 1,
            "format has no integer bit: the value 1 is outside [0, 1), so ONE cannot be derived for this format"
        );
    }
    1u64 << F
}

#[no_mangle]
pub extern "C" fn get_one_4_4() -> u64 {
    derive_raw_one::<4, 4>()
}
