// p270a: is each reading of `half_up` reachable as an alias over the other
// names of the vocabulary plus one exact addition, whichever reading the name
// is given?
//
// Why it is asked. Seat 267 argues that the standards bound forces both
// readings into the vocabulary as names, because MATLAB `fi` documents both
// (`Nearest` and `Round`). `ruling::the_standard_is_parity_in_output_not_in_the_internals`
// says the bound "does not oblige carrying the standard's operation list", and
// `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
// asks for each convention to be "writable as a first-class alias over arvo's
// own primitives". So the question the bound actually poses is whether the
// reading the name does not denote is writable over what the vocabulary has.
//
// Value model. A W-bit container holds a scaled integer k denoting k / 2^F;
// rounding takes it to an integer. Signed: k in [-2^(W-1), 2^(W-1)). Unsigned:
// k in [0, 2^W). Every k of the container, every W in 2..=16, every F in
// 1..=W-1 (F = 0 has no tie and no fraction, so every rule is the identity).
//
// The references are written from the definitions, by comparing twice the
// remainder against the step, and share no code with the compositions.
//
// The compositions, each built only from `floor`, `toward_zero`, a negation,
// a sign test and one exact addition of the half step:
//
//   C1  nearest ties +inf   = floor(x + 1/2)
//   C2  nearest ties away   = toward_zero(x + sign(x) * 1/2)
//   C3  nearest ties away   = x < 0 ? -C1(-x) : C1(x)
//   C4  nearest ties away   = floor(x + 1/2 - [x < 0] * ulp), the branchless
//                             form, with ulp = 2^-F
//
// The cases that must fail, each stated before the sweep and asserted after:
//
//   N1  floor(x + 1/2) against ties away: must disagree on the signed domain,
//       on exactly 2^(W-1-F) values, and on no unsigned value.
//   N2  toward_zero(x + 1/2), the sign-blind spelling of C2: must disagree
//       with ties away somewhere on every signed row with F >= 1.
//   N3  the cheap hardware form, (k + 2^(F-1)) >> F with the addition done in
//       W-bit wrapping arithmetic: must disagree with ties +inf, and on
//       exactly the 2^(F-1) values at the top of the container where the
//       addition leaves it. This is the one-bit headroom C1 costs.
//
// Run: rustc --edition 2021 -O p270a_each_reading_is_an_alias_over_the_other_names.rs -o /tmp/p270a && /tmp/p270a

fn floor_div(k: i64, s: i64) -> i64 {
    k.div_euclid(s)
}

fn trunc_div(k: i64, s: i64) -> i64 {
    k / s
}

// References, from the definitions.
fn ref_ties_pos_inf(k: i64, f: u32) -> i64 {
    let s = 1i64 << f;
    let q = k.div_euclid(s);
    let r = k.rem_euclid(s);
    if 2 * r > s {
        q + 1
    } else if 2 * r < s {
        q
    } else {
        q + 1
    }
}

fn ref_ties_away(k: i64, f: u32) -> i64 {
    let s = 1i64 << f;
    let q = k.div_euclid(s);
    let r = k.rem_euclid(s);
    if 2 * r > s {
        q + 1
    } else if 2 * r < s {
        q
    } else if k < 0 {
        q
    } else {
        q + 1
    }
}

// Compositions.
fn c1(k: i64, f: u32) -> i64 {
    floor_div(k + (1i64 << (f - 1)), 1i64 << f)
}

fn c2(k: i64, f: u32) -> i64 {
    let half = 1i64 << (f - 1);
    let signed_half = if k < 0 { -half } else { half };
    trunc_div(k + signed_half, 1i64 << f)
}

fn c3(k: i64, f: u32) -> i64 {
    if k < 0 { -c1(-k, f) } else { c1(k, f) }
}

fn c4(k: i64, f: u32) -> i64 {
    let neg = (k < 0) as i64;
    floor_div(k + (1i64 << (f - 1)) - neg, 1i64 << f)
}

// Negative controls.
fn n2(k: i64, f: u32) -> i64 {
    trunc_div(k + (1i64 << (f - 1)), 1i64 << f)
}

// W-bit wrapping add, then arithmetic shift, as a narrow datapath would do it.
fn n3(k: i64, f: u32, w: u32, signed: bool) -> i64 {
    let modulus = 1i64 << w;
    let mut sum = (k + (1i64 << (f - 1))).rem_euclid(modulus);
    if signed && sum >= modulus / 2 {
        sum -= modulus;
    }
    floor_div(sum, 1i64 << f)
}

fn domain(w: u32, signed: bool) -> (i64, i64) {
    if signed {
        (-(1i64 << (w - 1)), (1i64 << (w - 1)) - 1)
    } else {
        (0, (1i64 << w) - 1)
    }
}

fn main() {
    let mut rows = 0u64;
    let mut values = 0u64;
    let mut c_fail = [0u64; 4];
    let mut n1_ok = true;
    let mut n2_ok = true;
    let mut n3_ok = true;
    let mut n3_nonzero = false;
    let mut n1_signed_total = 0u64;

    println!("p270a: each reading of half_up as an alias over the other names");
    println!();
    println!("  W  F  sign      values  C1  C2  C3  C4  N1(pred)      N2   N3(pred)");
    for w in 2u32 ..= 16 {
        for signed in [true, false] {
            for f in 1 .. w {
                let (lo, hi) = domain(w, signed);
                let mut fails = [0u64; 4];
                let mut n1 = 0u64;
                let mut n2c = 0u64;
                let mut n3c = 0u64;
                for k in lo ..= hi {
                    let pi = ref_ties_pos_inf(k, f);
                    let aw = ref_ties_away(k, f);
                    if c1(k, f) != pi {
                        fails[0] += 1;
                    }
                    if c2(k, f) != aw {
                        fails[1] += 1;
                    }
                    if c3(k, f) != aw {
                        fails[2] += 1;
                    }
                    if c4(k, f) != aw {
                        fails[3] += 1;
                    }
                    if c1(k, f) != aw {
                        n1 += 1;
                    }
                    if n2(k, f) != aw {
                        n2c += 1;
                    }
                    if n3(k, f, w, signed) != pi {
                        n3c += 1;
                    }
                    values += 1;
                }
                rows += 1;
                for i in 0 .. 4 {
                    c_fail[i] += fails[i];
                }
                let n1_pred = if signed { 1u64 << (w - 1 - f) } else { 0 };
                let n3_pred = 1u64 << (f - 1);
                if n1 != n1_pred {
                    n1_ok = false;
                }
                if signed {
                    n1_signed_total += n1;
                    if n2c == 0 {
                        n2_ok = false;
                    }
                }
                if n3c != n3_pred {
                    n3_ok = false;
                }
                if n3c > 0 {
                    n3_nonzero = true;
                }
                if w == 4 || w == 8 || w == 16 {
                    println!(
                        "{:3}{:3}  {:8}{:8}{:4}{:4}{:4}{:4}{:6}({:5}){:8}{:6}({:5})",
                        w,
                        f,
                        if signed { "signed" } else { "unsigned" },
                        hi - lo + 1,
                        fails[0],
                        fails[1],
                        fails[2],
                        fails[3],
                        n1,
                        n1_pred,
                        n2c,
                        n3c,
                        n3_pred
                    );
                }
            }
        }
    }
    println!("  (rows printed for W in {{4, 8, 16}}; every row in W 2..=16 is checked)");
    println!();
    println!("rows {rows}, values {values}");
    println!(
        "compositions: C1 {} failures, C2 {}, C3 {}, C4 {}",
        c_fail[0], c_fail[1], c_fail[2], c_fail[3]
    );
    println!("N1 total signed disagreements {n1_signed_total}, per-row prediction held: {n1_ok}");
    println!("N2 disagrees on every signed row: {n2_ok}");
    println!(
        "N3 disagreements equal 2^(F-1) on every row: {n3_ok}, nonzero somewhere: {n3_nonzero}"
    );

    assert!(c_fail.iter().all(|&x| x == 0), "a composition failed");
    assert!(
        n1_ok && n1_signed_total > 0,
        "N1 must fail, and exactly as predicted"
    );
    assert!(n2_ok, "N2 must fail on every signed row");
    assert!(
        n3_ok && n3_nonzero,
        "N3 must fail, and exactly as predicted"
    );
    println!("instrument: sound");
}
