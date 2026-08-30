//! probe 1, crate `unify`: the payoff. The capacity crate's semantics reach
//! the numeral crate's precision with zero glue, because both domains name
//! one carrier. This is the anti-fragmentation claim, compiled.
#![no_std]
use capacity::Capacity;
use numeral::{Binary13, Numeral};

// the numeral's precision IS a capacity, through capacity's own blanket impl,
// with no bridge trait, no conversion, no second encoding anywhere.
const _: () = assert!(<<Binary13 as Numeral>::Precision as Capacity>::SIZE == 13);

// and the two domains' names unify as one type, checked structurally:
const fn same_type<T>() {}
const _: () = same_type::<capacity::Cap13>();
const _: () = {
    // if Cap13 and Binary13::Precision were different types this would not compile
    const fn eq<T>(_: core::marker::PhantomData<T>, _: core::marker::PhantomData<T>) {}
    eq::<capacity::Cap13>(
        core::marker::PhantomData,
        core::marker::PhantomData::<<Binary13 as Numeral>::Precision>,
    );
    ()
};
