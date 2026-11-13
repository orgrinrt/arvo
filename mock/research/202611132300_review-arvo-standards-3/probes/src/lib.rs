//! Review 3 probes for `work/arvo-standards` at `366d2e15`. Ad-hoc quick spikes.

#[cfg(test)]
mod tests {
    use arvo_format::addition::is_addable;
    use arvo_format::ambient::UnsignedBinaryRationals;
    use arvo_format::conversion::is_convertible;
    use arvo_format::division::is_divisible;
    use arvo_format::format::{Format, is_admissible_format};
    use arvo_format::multiplication::is_multipliable;
    use arvo_format::slots::is_admissible;
    use arvo_format::standards::ieee754::{
        Binary16,
        Binary32,
        Decimal32,
        Interchange,
        is_sign_and_magnitude,
    };
    use arvo_format::standards::matlab::Fi;

    /// The generic verdicts against the significand's own obligation.
    #[test]
    fn the_generic_verdicts_admit_what_the_obligation_refuses() {
        type U = Interchange<UnsignedBinaryRationals, 24, 127>;
        let slots = is_admissible::<<U as Format>::Slots>().get();
        let format = is_admissible_format::<U>().get();
        let own = is_sign_and_magnitude::<UnsignedBinaryRationals, 24>().get();
        println!(
            "slots::is_admissible {slots}, is_admissible_format {format}, is_sign_and_magnitude {own}"
        );
        assert!(slots && format && !own);
    }

    /// Every operation verdict, and conversion both ways, on the IEEE formats.
    #[test]
    fn every_operation_and_conversion_refuses_an_interchange_format() {
        let row = [
            ("add b32", is_addable::<Binary32>().get()),
            ("mul b32", is_multipliable::<Binary32>().get()),
            ("div b32", is_divisible::<Binary32>().get()),
            ("add d32", is_addable::<Decimal32>().get()),
            ("fi32 -> b32", is_convertible::<Fi<32, 0>, Binary32>().get()),
            ("b32 -> fi32", is_convertible::<Binary32, Fi<32, 0>>().get()),
            ("b16 -> b32", is_convertible::<Binary16, Binary32>().get()),
            (
                "fi8 -> fi16 control",
                is_convertible::<Fi<8, 0>, Fi<16, 4>>().get(),
            ),
        ];
        println!("{row:?}");
        assert!(row[.. 7].iter().all(|&(_, v)| !v));
        assert!(
            row[7].1,
            "the control must convert, or the zeros above say nothing"
        );
    }
}
