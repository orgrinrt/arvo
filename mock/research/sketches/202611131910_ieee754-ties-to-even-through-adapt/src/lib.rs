//! Whether an IEEE 754 `roundTiesToEven` parity arm can be written through `adapt`.
//!
//! Binary32 is the shipped `Floating` point at 25 two's complement slots, the
//! smallest step `2^-149` and 254 magnitudes, and nothing in `arvo-format`'s
//! source is changed. `adapt` carries no magnitude coordinate, so the magnitude
//! is chosen by the caller: the finest magnitude at which the rounded position
//! lands in the slot range, read off `panic_on_overflow`. That is IEEE 754's own
//! rounding onto a floating format, the nearest member at the precision the
//! binade allows, and a carry out of a binade is the next magnitude answering.
//!
//! Outcome: WORKS, for finite results. The host's `f64 as f32`, which Rust
//! defines as round to nearest with ties to even, is the oracle, so no arvo code
//! sits on the expected side. A result past the largest finite member is where
//! the standard answers infinity and the format has no member, which the sketch
//! does not reach.

#[cfg(test)]
mod tests {
    use arvo_format::adapt::{Adapt, Signature};
    use arvo_format::apply::{Dither, Exact, Remainder, adapt, panic_on_overflow};
    use arvo_format::format::step_exponent;
    use arvo_format::overflow::Saturate;
    use arvo_format::points::Floating;
    use arvo_format::quantum::Magnitude;
    use arvo_format::rounding::{HalfEven, HalfUp};
    use arvo_format::slots::Slot;

    type Binary32 = Floating<25, -149, 254>;

    /// `n * 2^k` rounded onto binary32 under the signature's mode, as a
    /// significand and an exponent, or nothing past the largest finite member.
    fn through_adapt<S: arvo_format::adapt::DeclaredSignature>(
        n: i64,
        k: i32,
    ) -> Option<(i128, i32)> {
        for m in 0 .. 254u32 {
            let s = step_exponent::<Binary32>(Magnitude::at(m)).power();
            let shift = k - s;
            let exact = if shift >= 0 {
                if shift > 62 {
                    continue;
                }
                Exact::on_grid(Slot::at((n as i128) << shift))
            } else {
                let den = 1i128 << (-shift);
                Exact::at(
                    Slot::at((n as i128).div_euclid(den)),
                    Remainder::of((n as i128).rem_euclid(den) as u128, den as u128),
                )
            };
            if panic_on_overflow::<S>(exact, Dither::UNUSED).get() {
                continue;
            }
            return Some((adapt::<S>(exact, Dither::UNUSED).index(), s));
        }
        None
    }

    /// The host's answer, decoded from its bits.
    fn host(n: i64, k: i32) -> (i128, i32) {
        let x = (n as f64) * f64::powi(2.0, k);
        let bits = (x as f32).to_bits();
        let sign = if bits >> 31 == 1 { -1 } else { 1 };
        let field = ((bits >> 23) & 0xFF) as i32;
        let frac = (bits & 0x7F_FFFF) as i128;
        if field == 0 {
            (sign * frac, -149)
        } else {
            (sign * (frac | (1 << 23)), field - 150)
        }
    }

    fn same(a: (i128, i32), b: (i128, i32)) -> bool {
        let (hi, lo) = if a.1 >= b.1 { (a, b) } else { (b, a) };
        hi.0 << (hi.1 - lo.1) == lo.0
    }

    /// Positions chosen to land on ties, both sides of them, a binade carry and
    /// the subnormal edge.
    const CASES: &[(i64, i32)] = &[
        ((1 << 24) + 1, 0),
        ((1 << 24) + 3, 0),
        ((1 << 25) - 1, 0),
        (-((1 << 24) + 1), 0),
        (-((1 << 24) + 3), 0),
        (3, -150),
        (5, -150),
        (1, -150),
        (-3, -150),
        ((1 << 25) - 1, -150),
        (0x0123_4567_89AB, -40),
        (-0x0123_4567_89AB, -40),
    ];

    #[test]
    fn ties_to_even_through_adapt_matches_the_host() {
        for &(n, k) in CASES {
            let got = through_adapt::<Signature<Binary32, Adapt<HalfEven, Saturate>>>(n, k);
            let want = host(n, k);
            assert!(
                got.is_some_and(|g| same(g, want)),
                "{n} * 2^{k}: {got:?} against {want:?}"
            );
        }
    }

    #[test]
    fn the_planted_mode_is_reported() {
        let wrong = CASES
            .iter()
            .filter(|&&(n, k)| {
                let got = through_adapt::<Signature<Binary32, Adapt<HalfUp, Saturate>>>(n, k);
                !got.is_some_and(|g| same(g, host(n, k)))
            })
            .count();
        assert!(
            wrong > 0,
            "half_up passed every tie, so the arm does not separate the modes"
        );
    }
}
