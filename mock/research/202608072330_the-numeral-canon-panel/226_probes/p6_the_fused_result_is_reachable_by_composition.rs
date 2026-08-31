//! p6: is the fused multiply-add's RESULT reachable without an fma operation?
//!
//! Seat 226, after op's `227`. He corrected the standards bound to parity in
//! output rather than in the internals, which removes the ground under "IEEE
//! names the fma, so the fma must be in arvo's operation set". What survives is
//! a question about results: is exactly-once-rounded `a*b + c` reachable by
//! composing operations arvo already has, or does reaching it require a distinct
//! declared operation?
//!
//! The composition under test is the obvious one, and it is one rounding:
//!
//!     fused    = adapt_F(a*b + c)      the product kept exact, adapted once
//!     composed = adapt_F(a*b) + c      the product adapted, then c added
//!
//! On an affine grid of step 2^-F, `c` is a multiple of the step, so the two
//! agree exactly when the adaptation commutes with translation by a grid point.
//! That is translation equivariance, and it is the same equation the register's
//! fusion rows already measured from the other side: "fusing preserves the
//! answer" and "the fused answer is reachable by composition" are one sentence
//! read twice. This is an independent instance at widths those rows did not
//! reach, sweeping the whole triple space rather than sampling it.
//!
//! The cases that must fail, run and reported:
//!   C1  at F = 0 the grid is the integers and every mode is the identity on it,
//!       so every mode must agree at every width. A mode disagreeing there means
//!       the harness is wrong rather than the mode.
//!   C2  at F > 0 at least one mode must disagree. A sweep where everything
//!       agrees cannot tell an equivariant mode from a broken comparison.
//!   C3  a planted non-equivariant adaptation, round-half-away-from-a-fixed-
//!       offset, must be reported as disagreeing at every F > 0.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUp,
    HalfEven,
    PlantedBias,
}

const MODES: [Mode; 6] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::HalfUp,
    Mode::HalfEven,
];

/// Place the exact value `num / den` on the integer grid, `den` a power of two.
fn adapt(num: i128, den: i128, m: Mode) -> i128 {
    let q = num.div_euclid(den);
    let r = num.rem_euclid(den); // 0 <= r < den
    if r == 0 {
        return q;
    }
    let twice = 2 * r;
    match m {
        Mode::Floor => q,
        Mode::Ceil => q + 1,
        Mode::TowardZero => {
            if num < 0 {
                q + 1
            } else {
                q
            }
        }
        Mode::AwayFromZero => {
            if num < 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::HalfUp => {
            if twice >= den {
                q + 1
            } else {
                q
            }
        }
        Mode::HalfEven => {
            if twice > den {
                q + 1
            } else if twice < den {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
        // C3. Its first form was `3 * r >= den`, which agreed everywhere and
        // failed as a control. The reason is the finding: translating by a grid
        // point adds a multiple of `den` to `num`, which leaves `r` untouched, so
        // ANY rule reading only `r` is equivariant by construction. A rule that
        // cannot commute has to read `q`, which is what this one does and what
        // half-even does.
        Mode::PlantedBias => {
            if q.rem_euclid(3) == 0 {
                q + 1
            } else {
                q
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Shape {
    w: u32,
    f: u32,
    signed: bool,
    sat: bool,
}

impl Shape {
    fn lo(&self) -> i128 {
        if self.signed {
            -(1i128 << (self.w - 1))
        } else {
            0
        }
    }
    fn hi(&self) -> i128 {
        if self.signed {
            (1i128 << (self.w - 1)) - 1
        } else {
            (1i128 << self.w) - 1
        }
    }
    fn place(&self, v: i128) -> i128 {
        if self.sat {
            v.clamp(self.lo(), self.hi())
        } else {
            let span = 1i128 << self.w;
            let m = v.rem_euclid(span);
            if self.signed && m > self.hi() {
                m - span
            } else {
                m
            }
        }
    }
}

/// Three routes rather than two, because where the range policy is applied is a
/// modelling choice and not a detail: a real stepwise implementation places the
/// adapted product into the declared format BEFORE adding c, so it applies the
/// policy twice. `composed` applies it once at the end, `composed_twice` applies
/// it at both steps. Under wrap the two coincide, because wrapping is a ring
/// homomorphism; under saturation they do not, because clamping is not.
fn disagreements(s: Shape, m: Mode, twice_placed: bool) -> (u64, Option<(i128, i128, i128)>) {
    let den = 1i128 << s.f;
    let mut n = 0u64;
    let mut first = None;
    for a in s.lo()..=s.hi() {
        for b in s.lo()..=s.hi() {
            let prod = a * b; // raw scale 2^-2F
            for c in s.lo()..=s.hi() {
                let fused = s.place(adapt(prod + c * den, den, m));
                let composed = if twice_placed {
                    s.place(s.place(adapt(prod, den, m)) + c)
                } else {
                    s.place(adapt(prod, den, m) + c)
                };
                if fused != composed {
                    n += 1;
                    if first.is_none() {
                        first = Some((a, b, c));
                    }
                }
            }
        }
    }
    (n, first)
}

fn main() {
    let mut c1_ok = true; // every mode agrees at F = 0
    let mut c2_ok = true; // some mode disagrees at F > 0
    let mut c3_ok = true; // the planted mode disagrees at every F > 0
    let mut free: Vec<String> = Vec::new();
    // C4: the isolated cell. At signed saturating the realistic two-placement
    // composition must reach the fused result for no mode at any fraction width,
    // and everywhere else it must reach it for at least one. Checked rather than
    // left to be read off the dump.
    let mut c4_signed_sat_empty = true;
    let mut c4_elsewhere_nonempty = true;

    for &signed in &[false, true] {
        for &sat in &[false, true] {
            for w in 5..=7u32 {
                let policy = if sat { "sat " } else { "wrap" };
                let sign = if signed { "signed  " } else { "unsigned" };
                println!("\n== {sign} {policy} W={w} ==");
                for f in 0..w {
                    let s = Shape { w, f, signed, sat };
                    let mut agreeing = Vec::new();
                    let mut agreeing2: Vec<String> = Vec::new();
                    let mut any_disagree = false;
                    for &m in &MODES {
                        let (n, _) = disagreements(s, m, false);
                        let (n2, _) = disagreements(s, m, true);
                        if n == 0 {
                            agreeing.push(format!("{m:?}"));
                        } else {
                            any_disagree = true;
                        }
                        if n2 == 0 {
                            agreeing2.push(format!("{m:?}"));
                        }
                        if f == 0 && n != 0 {
                            c1_ok = false;
                        }
                    }
                    let (pn, _) = disagreements(s, Mode::PlantedBias, false);
                    if f > 0 && pn == 0 {
                        c3_ok = false;
                    }
                    if f > 0 && !any_disagree {
                        c2_ok = false;
                    }
                    println!(
                        "  F={f}  one placement: {}",
                        if agreeing.is_empty() {
                            "none".into()
                        } else {
                            agreeing.join(", ")
                        }
                    );
                    println!(
                        "        two placements: {}",
                        if agreeing2.is_empty() {
                            "none".into()
                        } else {
                            agreeing2.join(", ")
                        }
                    );
                    if signed && sat {
                        if !agreeing2.is_empty() {
                            c4_signed_sat_empty = false;
                        }
                    } else if agreeing2.is_empty() {
                        c4_elsewhere_nonempty = false;
                    }
                    if !agreeing.is_empty() {
                        free.push(format!("{sign}/{policy}/W={w}/F={f}: {}", agreeing.len()));
                    }
                }
            }
        }
    }

    println!("\n== verdict ==");
    println!("  C1 every mode agrees at F = 0:                    {c1_ok}");
    println!("  C2 some mode disagrees at every F > 0:            {c2_ok}");
    println!("  C3 the planted non-equivariant mode is caught:    {c3_ok}");
    println!(
        "  C4 signed saturating reaches it for no mode, two placements: {c4_signed_sat_empty}"
    );
    println!("  C4 every other policy reaches it for some mode:   {c4_elsewhere_nonempty}");
    println!(
        "  cells with at least one mode reaching it, one placement: {}",
        free.len()
    );
    let pass = c1_ok && c2_ok && c3_ok && c4_signed_sat_empty && c4_elsewhere_nonempty;
    println!(
        "\n  RESULT: {}",
        if pass {
            "the fused result is reachable by composing multiply and add, in a nameable region and not outside it"
        } else {
            "INCONCLUSIVE"
        }
    );
    std::process::exit(if pass { 0 } else { 1 });
}
