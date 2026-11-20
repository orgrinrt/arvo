//! The shape's contract table says `Sum` is "total, under the instance's own
//! completion", and the impl it inherits from the sketch is over every
//! `Num<S, C>`, the default debug check `Abort` included. This test runs the
//! saturating sum of 100 and 100 through the contract under `Abort` and under
//! `Quiet` in a debug build. Under `Quiet` it is the completion, 127. Under
//! `Abort` the contract's call aborts: the law "the slot of the result is the
//! slot the format's function gives" is not total over the instances the
//! impl covers, in the build profile every test runs in.

#[cfg(test)]
mod tests {
    use arm_plain::Sum;
    use arvo_format::Adapt;
    use arvo_format::overflow::Saturate;
    use arvo_format::rounding::TowardZero;
    use arvo_numeral::{Abort, Int, Quiet};

    type Q8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;
    type A8 = Int<8, Adapt<TowardZero, Saturate>, Abort>;

    #[test]
    fn under_quiet_the_contract_gives_the_completion() {
        let x = Q8::constant::<100>();
        assert_eq!(x.sum(x), Q8::constant::<127>());
    }

    #[test]
    fn under_abort_the_contract_does_not_return() {
        let r = std::panic::catch_unwind(|| {
            let x = A8::constant::<100>();
            x.sum(x)
        });
        assert!(r.is_err(), "the Abort sum returned {:?}", r.ok());
    }

    #[test]
    fn under_abort_an_in_range_sum_returns() {
        let x = A8::constant::<50>();
        assert_eq!(x.sum(x), A8::constant::<100>());
    }
}
