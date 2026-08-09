//! The vocabulary's own wording attacked, member by member. File 87 names
//! the set as "NonZero<T> at every native width, bool, char, references,
//! and whatever else std documents" (87:213-214). The one audited entry's
//! trusted fact is shaped for a single excluded run at zero with a
//! bias-by-one debias; this probe checks each named member against that
//! shape, in const position, so every claim is a compile fact.
#![no_std]

// char: the validity set is NOT one excluded run at zero. A non-inhabitant
// sits strictly between two inhabitants, so no bias-by-k debias maps the
// inhabitant set onto a contiguous numeral domain [0, N). The entry's
// trusted sentence is false for char, and char is in the vocabulary as
// worded.
const _: () = {
    assert!(char::from_u32(0xD7FF).is_some());
    assert!(char::from_u32(0xD800).is_none()); // the hole in the middle
    assert!(char::from_u32(0xE000).is_some());
};

// bool: exactly 2 = 2^1 inhabitants, so ordinary field-shrinking already
// expresses the domain and the trusted-base entry buys nothing; bool in
// the vocabulary is redundant rather than unsound, which still means the
// audited entry's "genuinely needed" claim is false for it.
const _: () = assert!(2u32 == 1u32 << 1);

// The NonZero family: exactly one excluded pattern, at zero, inhabitant
// count 2^w - 1, never a power of two (the collision, re-checked at both
// widths file 87 used), so field shrinking cannot express it and the entry
// is genuinely needed there, and only there among the named members.
const _: () = {
    let mut k = 0u32;
    while k <= 16 {
        assert!(1u32 << k != 65535);
        assert!(1u32 << k != 4095);
        k += 1;
    }
};

// The inhabitant-totality condition the entry owes, priced at a concrete
// instance: a bounded numeral of 2^13 values biased into NonZeroU16 leaves
// (2^16 - 1) - 2^13 = 57343 inhabitants with no decode, an unenforced
// domain side-condition, the exact thing statement 0's hardening forbids
// at the fields level (80:99-102). The condition is a const equation over
// type parameters (domain cardinality against inhabitant count), refusable
// at declaration; it does not belong in the trusted sentence.
const _: () = {
    let inhabitants: u32 = (1 << 16) - 1;
    let bounded_domain: u32 = 1 << 13;
    assert!(inhabitants - bounded_domain == 57343);
};

// The exact-fit case: a numeral (or a Maybe-shaped fallible tier) whose
// value set has exactly 2^w - 1 members is total over the inhabitants
// value-uniquely; anything smaller must either refuse at declaration or
// declare a cohort-style many-to-one decode, exactly the two options the
// fields level already has.
const _: () = {
    let inhabitants: u32 = (1 << 16) - 1;
    assert!(inhabitants == 65535);
};
