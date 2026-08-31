// p4: the adequacy test op named, run against the three candidate answers.
//
// WHY THIS RUNS. Op returned `the_container_premise` with a bound rather than
// an answer: "it is bounded by soundness and also the rule that demands we
// provide first-class matlab and ieee754 compatible apis as aliases over arvo
// primitives and such". `obligation::every_standard_convention_expressible_as
// _an_alias_over_the_primitives` records it as an ADEQUACY test: a convention
// that cannot be written as an alias is a gap in the primitives. So the two
// named standards are a refutation instrument, and this is that instrument.
//
// THE QUESTION PUT TO EACH STANDARD. If arvo states its encoding over X, is
// the standard's own surface writable as an alias? Three candidate X:
//   W       the declared width.
//   MIN     the minimum native container, `rung(W)`. Hot and Cold's rule.
//   HEAD    `rung(rung_bits(W) + 1)`. The shipped Warm and Precise rule at
//           `warm-container-shared/src/lib.rs:5-11`.
//
// THE STANDARDS, quoted rather than remembered.
//
// MATLAB `fi`, two observations that the documentation gives DIFFERENT rules:
//   `bin(a)`             "the unsigned binary representation of the stored
//                        integers", exactly WordLength characters per element.
//                        Documented example: WordLength 8 gives '10000000'.
//   `storedInteger(a)`   "The data type of the output determined based on the
//                        signedness and word length (WL) of the stored
//                        integer": WL<=8 -> int8/uint8, 8<WL<=16 -> int16,
//                        16<WL<=32 -> int32, 32<WL<=64 -> int64, and above 64
//                        it is an error. That is `rung(W)`, not an
//                        implementation's carrier.
//
// IEEE 754-2019:
//   clause 2   "interchange format: A format that has a specific fixed-width
//              encoding defined in this standard."
//   sec 3.4    "Representations of floating-point data in the binary
//              interchange formats are uniquely encoded in k bits".
//   Table 3.5  "k, storage width in bits ... 1 + w + t".
//   sec 3.2    "A conforming implementation of a supported interchange format
//              shall provide means to read and write that format using a
//              specific encoding defined in this clause".
//
// WHAT MUST FAIL, declared before the run. An adequacy checker that passes
// everything has tested nothing, so three arms must be REFUSED:
//   F1  the HEAD arm must fail MATLAB `bin`, at every swept width.
//   F2  the HEAD arm must fail the IEEE interchange width.
//   F3  a deliberately broken W arm, off by one bit, must fail both, so the
//       checker is not reporting PASS on the strength of reading `W`.
// And one arm must fail that closes the obvious escape:
//   F4  the MIN arm must fail MATLAB `bin` at every non-exactly-filled width,
//       which is what makes the answer independent of which container rule
//       ships.
//
// SCOPE. W in {3, 8, 13, 14, 16, 23, 27, 31, 32, 47, 60, 64}, signedness
// unsigned and signed, radix 2, IEEE interchange widths scored at {16, 32, 64}
// and reported unscored at {128, 256}, threads = 1, target features any,
// toolchain: the repository pin.

// ---------------------------------------------------------------------------
// The container rules under test.
// ---------------------------------------------------------------------------

/// The smallest native rung holding `w` bits.
fn rung(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else {
        128
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum StatedOver {
    /// The declared width.
    W,
    /// `rung(W)`: the minimum native container, Hot and Cold's rule.
    Min,
    /// `rung(rung(W) + 1)`: the shipped Warm and Precise rule.
    Head,
    /// A deliberately broken reading of the declared width, W + 1. The control
    /// that stops a PASS meaning only "this arm mentioned W".
    WBroken,
}

impl StatedOver {
    fn encoding_bits(self, w: u32) -> u32 {
        match self {
            StatedOver::W => w,
            StatedOver::Min => rung(w),
            StatedOver::Head => rung(rung(w) + 1),
            StatedOver::WBroken => w + 1,
        }
    }
}

// ---------------------------------------------------------------------------
// MATLAB `fi`.
// ---------------------------------------------------------------------------

/// What `bin` must return: exactly WordLength characters.
fn matlab_bin_required_chars(w: u32) -> u32 {
    w
}

/// What `storedInteger` must return, in bits, per the documented mapping.
/// `None` above 64, where MATLAB documents an error rather than a type.
fn matlab_stored_integer_required_bits(w: u32) -> Option<u32> {
    if w > 64 { None } else { Some(rung(w)) }
}

// ---------------------------------------------------------------------------
// IEEE 754-2019 binary interchange formats.
// ---------------------------------------------------------------------------

/// `w`, the exponent field width, from Table 3.5.
fn ieee_exponent_field(k: u32) -> Option<u32> {
    match k {
        16 => Some(5),
        32 => Some(8),
        64 => Some(11),
        128 => Some(15),
        // "in general for any multiple of 32 bits of at least 128 bits",
        // w = round(4 * log2(k)) - 13.
        _ if k >= 128 && k % 32 == 0 => {
            Some((4.0f64 * (k as f64).log2()).round() as u32 - 13)
        }
        _ => None,
    }
}

/// Table 3.5's own closure: `k = 1 + w + t`. Used as an internal control on
/// the parameter table before any arm is judged against it.
fn ieee_k_from_fields(k: u32) -> Option<u32> {
    let w = ieee_exponent_field(k)?;
    let p = k - w;
    let t = p - 1;
    Some(1 + w + t)
}

fn hdr(s: &str) {
    println!("{s}");
}

fn row(label: &str, detail: &str, pass: bool, required_pass: bool, note: &str) -> bool {
    let ok = pass == required_pass;
    println!(
        "  {:<12} {:<44} {:<8} required={:<8} {:<30} {}",
        label,
        detail,
        if pass { "PASS" } else { "REFUSED" },
        if required_pass { "PASS" } else { "REFUSED" },
        note,
        if ok {
            "as required"
        } else {
            "*** NOT AS REQUIRED ***"
        }
    );
    ok
}

const WIDTHS: [u32; 12] = [3, 8, 13, 14, 16, 23, 27, 31, 32, 47, 60, 64];

fn main() {
    println!("### p4. the standards adequacy test, over three candidate statements of arvo's");
    println!("### encoding. toolchain: the repository pin.");
    println!();

    let mut all_ok = true;

    // -- control zero: the IEEE parameter table closes on itself ------------
    hdr("CONTROL 0. Table 3.5's own closure k = 1 + w + t, before judging any arm");
    for k in [16u32, 32, 64, 128, 256] {
        let got = ieee_k_from_fields(k);
        all_ok &= row(
            "T3.5",
            &format!("binary{k}: 1 + w + t"),
            got == Some(k),
            true,
            &format!("w={:?} gives k={:?}", ieee_exponent_field(k), got),
        );
    }
    println!();

    // -- MATLAB fi ----------------------------------------------------------
    for arm in [
        StatedOver::W,
        StatedOver::Min,
        StatedOver::Head,
        StatedOver::WBroken,
    ] {
        // `bin` must be exactly W characters at EVERY swept width. One failure
        // refuses the arm, which is what an adequacy test means.
        let mut bin_bad: Vec<u32> = Vec::new();
        let mut si_bad: Vec<u32> = Vec::new();
        for &w in WIDTHS.iter() {
            if arm.encoding_bits(w) != matlab_bin_required_chars(w) {
                bin_bad.push(w);
            }
            // `storedInteger`'s rule is `rung(W)`, so an arm stating the
            // encoding over `rung(W)` satisfies THIS observation and an arm
            // stating it over W does not, which is the whole reason MATLAB
            // gives the two observations different names.
            if Some(rung(arm.encoding_bits(w))) != matlab_stored_integer_required_bits(w) {
                si_bad.push(w);
            }
        }
        let required_bin_pass = matches!(arm, StatedOver::W);
        // Both W and Min land on rung(W) after the rung; Head and WBroken do not
        // at every width, so this is a real discriminator rather than a constant.
        let required_si_pass = matches!(arm, StatedOver::W | StatedOver::Min);
        hdr(&format!("MATLAB `fi`, arvo's encoding stated over {arm:?}"));
        all_ok &= row(
            "bin",
            "exactly WordLength characters, every swept width",
            bin_bad.is_empty(),
            required_bin_pass,
            &format!("fails at {} of {} widths", bin_bad.len(), WIDTHS.len()),
        );
        all_ok &= row(
            "storedInt",
            "the documented rung(WL) type, every swept width",
            si_bad.is_empty(),
            required_si_pass,
            &format!("fails at {} of {} widths", si_bad.len(), WIDTHS.len()),
        );
        println!();
    }

    // -- IEEE 754 -----------------------------------------------------------
    //
    // Scored over k in {16, 32, 64} only, and the restriction is the honest
    // one rather than a convenience. The Min and Head arms are defined in
    // terms of `rung`, whose table stops at 128 because that is where arvo's
    // native rungs stop, so at k = 128 and k = 256 those two arms are not
    // wrong, they have no referent: there is no native container to state an
    // encoding over. The first cut scored all five and the Min arm "failed" at
    // 256 for exactly that reason, which was the model speaking rather than
    // the design. That run is kept at `p4_v1_min_arm_failed_at_k256.out`, and
    // the undefined region is reported below rather than scored.
    for arm in [
        StatedOver::W,
        StatedOver::Min,
        StatedOver::Head,
        StatedOver::WBroken,
    ] {
        let mut bad: Vec<u32> = Vec::new();
        for k in [16u32, 32, 64] {
            // The interchange format's declared width IS k. An alias states
            // its encoding over one of the candidates; conformance demands the
            // encoding move exactly k bits.
            if arm.encoding_bits(k) != k {
                bad.push(k);
            }
        }
        // Min passes here, because every interchange width at or below 64 is
        // already a native rung. That is a fact about the standard's chosen
        // widths rather than about the rule, which is why MATLAB's non-native
        // widths are the case that discriminates.
        let required_pass = matches!(arm, StatedOver::W | StatedOver::Min);
        hdr(&format!(
            "IEEE 754-2019 binary interchange, arvo's encoding stated over {arm:?}"
        ));
        all_ok &= row(
            "sec 3.4",
            "uniquely encoded in k bits, k in {16, 32, 64}",
            bad.is_empty(),
            required_pass,
            &format!("fails at {:?}", bad),
        );
        println!();
    }

    hdr("REPORTED, NOT SCORED. k = 128 and k = 256, where `rung` has no value");
    for k in [128u32, 256] {
        println!(
            "  binary{k}: W gives {}, Min gives {}, Head gives {}. The standard defines the",
            StatedOver::W.encoding_bits(k),
            StatedOver::Min.encoding_bits(k),
            StatedOver::Head.encoding_bits(k)
        );
        println!(
            "  format; arvo has no native rung there, so only the W statement has a referent."
        );
    }
    println!();

    println!("### reading");
    println!("### Only the W arm satisfies every observation of both standards. The MIN arm");
    println!("### passes IEEE (every interchange width is already a native rung) and passes");
    println!("### `storedInteger` (whose own rule is rung(WL)), and it fails `bin` at every");
    println!("### width that is not exactly a rung, so switching arvo's container rule does");
    println!("### not rescue a canon that states the encoding over the container.");
    println!("### The HEAD arm, which is what arvo ships for Warm and Precise, is refused");
    println!("### by every observation of both standards.");
    println!();
    println!(
        "### overall: {}",
        if all_ok {
            "every arm as required"
        } else {
            "*** AT LEAST ONE ARM NOT AS REQUIRED ***"
        }
    );
    if !all_ok {
        std::process::exit(1);
    }
}
