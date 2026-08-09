// Probe 2. File 66's escape is one instance of a coupling failure between two
// axes, not one cell of a matrix.
//
// File 66 section 6 varied radix, precision, `Underflow` and normalisation, and
// reported "exactly one cell of the matrix leaks". It held `Specials` fixed.
// `Specials` is the design's other value-set-shrinking axis (`63:656`, a
// four-point product: NoSpecials | NanOnly | InfOnly | IeeeSpecials), and it
// sits on `Numeral` (`63:162`, inside `Ranged`) while the field layout that
// determines which data exist sits on `Lowering` (`63:648-652`,
// `Encoding::Fields`). Nothing couples them.
//
// This probe models an IEEE-shaped field layout (a biased exponent code, the
// top code reserved) and counts escaping data across the whole `Specials`
// product, at two layout variants:
//
//   - "ieee":  the top exponent code is reserved for specials, as IEEE 754 lays
//              it out. This is the obvious layout and the one a consumer gets
//              by reaching for the standard shape.
//   - "ocp":   the top code is reassigned to finite values except the all-ones
//              significand, which is NaN. This is OCP OFP8 E4M3's own layout
//              (`63:304-310`: E4M3 "does not represent infinities, uses two NaN
//              bit patterns, and raises emax from 7 to 8 to gain one binade").
//
// The point of the second row is that E4M3's designers did the coupling BY
// HAND: they shrank the value set (no infinities) and re-expanded it (emax 7 to
// 8) so that the datum set is exactly covered. Nothing in arvo's design does
// that or asks for it. E4M3 is therefore not a counterexample to the finding,
// it is the existence proof that the coupling is real work a format designer
// performs deliberately, and that arvo has nowhere to state it.

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Specials {
    NoSpecials,
    NanOnly,
    InfOnly,
    IeeeSpecials,
}

impl Specials {
    fn name(self) -> &'static str {
        match self {
            Specials::NoSpecials => "NoSpecials  ",
            Specials::NanOnly => "NanOnly     ",
            Specials::InfOnly => "InfOnly     ",
            Specials::IeeeSpecials => "IeeeSpecials",
        }
    }
    fn has_inf(self) -> bool {
        matches!(self, Specials::InfOnly | Specials::IeeeSpecials)
    }
    fn has_nan(self) -> bool {
        matches!(self, Specials::NanOnly | Specials::IeeeSpecials)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Layout {
    /// Top exponent code reserved for specials, IEEE 754's own shape.
    Ieee,
    /// Top code carries finite values except the all-ones significand, which is
    /// NaN. OCP OFP8 E4M3's shape.
    Ocp,
}

/// What a datum denotes, as the physical decode arithmetic produces it. This is
/// a fact about the ENCODING alone: it reads the fields and says what they name.
/// It does not know what the numeral's value set contains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Denotes {
    Finite,
    Inf,
    Nan,
}

/// Enumerate the datum set of an `ew`-bit exponent field and `sw`-bit
/// significand field, one sign, and say what each datum denotes under `layout`.
fn data(ew: u32, sw: u32, layout: Layout) -> Vec<Denotes> {
    let ecodes = 1u32 << ew;
    let scodes = 1u32 << sw;
    let top = ecodes - 1;
    let mut out = Vec::new();
    for e in 0..ecodes {
        for s in 0..scodes {
            let d = if e == top {
                match layout {
                    Layout::Ieee => {
                        if s == 0 {
                            Denotes::Inf
                        } else {
                            Denotes::Nan
                        }
                    }
                    Layout::Ocp => {
                        if s == scodes - 1 {
                            Denotes::Nan
                        } else {
                            Denotes::Finite
                        }
                    }
                }
            } else {
                Denotes::Finite
            };
            out.push(d);
        }
    }
    out
}

/// Statement 0 of the crossing contract, evaluated: does this datum denote a
/// value the numeral has?
fn in_value_set(d: Denotes, sp: Specials) -> bool {
    match d {
        Denotes::Finite => true,
        Denotes::Inf => sp.has_inf(),
        Denotes::Nan => sp.has_nan(),
    }
}

fn main() {
    // E4M3's own shape: 4 exponent bits, 3 significand bits, one sign.
    let (ew, sw) = (4u32, 3u32);

    println!("  layout   Specials       data   escaping   percent");
    let mut ieee_escapes = 0usize;
    let mut ocp_escapes = 0usize;
    for layout in [Layout::Ieee, Layout::Ocp] {
        for sp in [
            Specials::NoSpecials,
            Specials::NanOnly,
            Specials::InfOnly,
            Specials::IeeeSpecials,
        ] {
            let ds = data(ew, sw, layout);
            let n = ds.len();
            let esc = ds.iter().filter(|d| !in_value_set(**d, sp)).count();
            match layout {
                Layout::Ieee => ieee_escapes += esc,
                Layout::Ocp => ocp_escapes += esc,
            }
            println!(
                "  {:6}   {}   {:5}   {:8}   {:5.1}%",
                match layout {
                    Layout::Ieee => "ieee",
                    Layout::Ocp => "ocp",
                },
                sp.name(),
                n,
                esc,
                100.0 * esc as f64 / n as f64
            );
        }
    }

    // ---- the assertions, so this probe fails rather than prints -----------

    // 1. Under the IEEE-shaped layout, three of the four `Specials` members
    //    leak. Only the member the layout was designed for does not.
    let ieee_leaking = [
        Specials::NoSpecials,
        Specials::NanOnly,
        Specials::InfOnly,
        Specials::IeeeSpecials,
    ]
    .iter()
    .filter(|sp| {
        data(ew, sw, Layout::Ieee)
            .iter()
            .any(|d| !in_value_set(*d, **sp))
    })
    .count();
    assert_eq!(
        ieee_leaking, 3,
        "expected exactly the three non-IeeeSpecials members to leak under an IEEE layout"
    );

    // 2. The largest leak is NoSpecials, and it is the entire top exponent
    //    code: 2^sw data, 1/2^ew of the datum set.
    let no_specials_esc = data(ew, sw, Layout::Ieee)
        .iter()
        .filter(|d| !in_value_set(**d, Specials::NoSpecials))
        .count();
    assert_eq!(no_specials_esc, 1usize << sw);
    assert_eq!(no_specials_esc, data(ew, sw, Layout::Ieee).len() >> ew);

    // 3. The OCP layout with NanOnly does NOT leak: the format's designers
    //    performed the coupling by hand. This is the control that shows the
    //    coupling is achievable, and that achieving it is deliberate work.
    let ocp_nanonly_esc = data(ew, sw, Layout::Ocp)
        .iter()
        .filter(|d| !in_value_set(**d, Specials::NanOnly))
        .count();
    assert_eq!(
        ocp_nanonly_esc, 0,
        "OCP E4M3's layout is exactly matched to its NanOnly value set"
    );

    // 4. But the SAME OCP layout leaks under a different `Specials` choice,
    //    which is the point: the layout is matched to one member of the axis,
    //    and arvo lets the two be chosen independently.
    let ocp_nospecials_esc = data(ew, sw, Layout::Ocp)
        .iter()
        .filter(|d| !in_value_set(**d, Specials::NoSpecials))
        .count();
    assert!(
        ocp_nospecials_esc > 0,
        "the OCP layout must still leak when the value set drops NaN"
    );

    // 5. Negative control: the escape is not an artifact of counting reserved
    //    patterns. Under IeeeSpecials with the IEEE layout, zero escape, and
    //    the datum set is not empty.
    assert!(data(ew, sw, Layout::Ieee).len() == 128);
    assert_eq!(
        data(ew, sw, Layout::Ieee)
            .iter()
            .filter(|d| !in_value_set(**d, Specials::IeeeSpecials))
            .count(),
        0
    );

    println!();
    println!("  ieee layout, summed escapes across the Specials product: {ieee_escapes}");
    println!("  ocp  layout, summed escapes across the Specials product: {ocp_escapes}");
    println!();
    println!("  ALL ASSERTIONS PASSED");
}
