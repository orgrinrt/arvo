// p1: the rounding algebra underneath both questions.
//
// Establishes, by exhaustive integer arithmetic and not by argument:
//
//   1. which of the named modes are translation equivariant, which is the
//      property the two fusion law rows say their regions are characterised by;
//   2. that on a non-negative domain `away_from_zero` and `ceil` are the same
//      function, and `toward_zero` and `floor` are the same function, so the
//      unsigned fusion row's five-name holding set denotes three functions;
//   3. that on a signed domain no two of the named modes coincide, so
//      `away_from_zero` there is not a spelling of anything ratified;
//   4. that `half_up` names two different operations on a signed domain,
//      exactly as the retired word did, and that the two differ on whether
//      they are translation equivariant.
//
// Every mode is exact integer arithmetic on i128. `q(x, s)` rounds the rational
// x / 2^s to an integer. No floating point anywhere.
//
// The cases that must fail are run and printed as EXPECTED-FAIL. If any of them
// reports clean the instrument is not measuring and every number below is void.

fn div_floor(x: i128, d: i128) -> i128 {
    x.div_euclid(d)
}
fn div_ceil(x: i128, d: i128) -> i128 {
    -((-x).div_euclid(d))
}
fn div_toward_zero(x: i128, d: i128) -> i128 {
    x / d
}
fn div_away_from_zero(x: i128, d: i128) -> i128 {
    if x >= 0 { div_ceil(x, d) } else { div_floor(x, d) }
}
// ties toward positive infinity: floor(x/d + 1/2)
fn half_up_pinf(x: i128, d: i128) -> i128 {
    div_floor(2 * x + d, 2 * d)
}
// ties away from zero: the other reading of the same name
fn half_up_away(x: i128, d: i128) -> i128 {
    if x >= 0 {
        div_floor(2 * x + d, 2 * d)
    } else {
        -div_floor(-2 * x + d, 2 * d)
    }
}
fn half_even(x: i128, d: i128) -> i128 {
    let lo = div_floor(x, d);
    let rem2 = 2 * (x - lo * d); // in 0..2d
    if rem2 > d {
        lo + 1
    } else if rem2 < d {
        lo
    } else if lo % 2 == 0 {
        lo
    } else {
        lo + 1
    }
}

type Mode = (&'static str, fn(i128, i128) -> i128);

// The ratified six, minus `stochastic`, which is not a function and is treated
// separately and by construction rather than by a sweep. Plus `away_from_zero`,
// which is what the question is about, plus the second reading of `half_up`.
fn modes() -> Vec<Mode> {
    vec![
        ("floor", div_floor as fn(i128, i128) -> i128),
        ("ceil", div_ceil),
        ("toward_zero", div_toward_zero),
        ("half_up(ties->+inf)", half_up_pinf),
        ("half_up(ties->away)", half_up_away),
        ("half_even", half_even),
        ("away_from_zero", div_away_from_zero),
    ]
}

/// Is `R(x + n*d) == R(x) + n` for every x in the scan and every n in the shift
/// range? Returns the first witness against it.
fn equivariance_witness(f: fn(i128, i128) -> i128, s: u32, lo: i128, hi: i128, shifts: i128)
    -> Option<(i128, i128, i128, i128)>
{
    let d = 1i128 << s;
    for x in lo..=hi {
        let base = f(x, d);
        for n in -shifts..=shifts {
            let got = f(x + n * d, d);
            if got != base + n {
                return Some((x, n, got, base + n));
            }
        }
    }
    None
}

/// Same, restricted to even translations.
fn even_equivariance_witness(f: fn(i128, i128) -> i128, s: u32, lo: i128, hi: i128, shifts: i128)
    -> Option<(i128, i128, i128, i128)>
{
    let d = 1i128 << s;
    for x in lo..=hi {
        let base = f(x, d);
        let mut n = -2 * shifts;
        while n <= 2 * shifts {
            let got = f(x + n * d, d);
            if got != base + n {
                return Some((x, n, got, base + n));
            }
            n += 2;
        }
    }
    None
}

/// First x in the scan where two modes disagree.
fn disagreement(a: fn(i128, i128) -> i128, b: fn(i128, i128) -> i128, s: u32, lo: i128, hi: i128)
    -> Option<(i128, i128, i128)>
{
    let d = 1i128 << s;
    for x in lo..=hi {
        let (u, v) = (a(x, d), b(x, d));
        if u != v {
            return Some((x, u, v));
        }
    }
    None
}

fn main() {
    // The scan. `s` is the number of bits discarded, so `d = 2^s` is the grid
    // spacing in the scaled integers. Every x in the range is visited, so each
    // result below is exhaustive over the stated window rather than sampled.
    let s: u32 = 3;
    let d: i128 = 1 << s;
    let lo: i128 = -512;
    let hi: i128 = 512;
    let shifts: i128 = 8;

    println!("p1: the rounding algebra");
    println!("    scaled integers x in {lo}..={hi}, grid spacing d = 2^{s} = {d}");
    println!("    exhaustive over x; translations n in -{shifts}..={shifts}");
    println!();

    // ---- 1. translation equivariance -------------------------------------
    println!("## 1. translation equivariance: R(x + n*d) == R(x) + n");
    println!();
    println!("{:<22} {:<14} {}", "mode", "verdict", "first witness against");
    for (name, f) in modes() {
        match equivariance_witness(f, s, lo, hi, shifts) {
            None => println!("{:<22} {:<14} -", name, "EQUIVARIANT"),
            Some((x, n, got, want)) => println!(
                "{:<22} {:<14} x={x} n={n}: R(x+n*d)={got}, R(x)+n={want}",
                name, "not equivariant"
            ),
        }
    }
    println!();
    println!("half_even restricted to even translations:");
    match even_equivariance_witness(half_even, s, lo, hi, shifts) {
        None => println!("  half_even is EQUIVARIANT under even translations"),
        Some((x, n, got, want)) => {
            println!("  witness x={x} n={n}: {got} vs {want}")
        }
    }
    println!();

    // ---- 2. collapse on the non-negative domain ---------------------------
    println!("## 2. do modes coincide on the non-negative domain (unsigned)?");
    println!();
    let pairs: Vec<(&str, fn(i128, i128) -> i128, &str, fn(i128, i128) -> i128)> = vec![
        ("away_from_zero", div_away_from_zero, "ceil", div_ceil),
        ("toward_zero", div_toward_zero, "floor", div_floor),
        ("half_up(ties->+inf)", half_up_pinf, "half_up(ties->away)", half_up_away),
    ];
    for (na, fa, nb, fb) in &pairs {
        let w = disagreement(*fa, *fb, s, 0, hi);
        match w {
            None => println!("  x >= 0:  {na} == {nb}   (no disagreement in 0..={hi})"),
            Some((x, u, v)) => println!("  x >= 0:  {na} != {nb}   at x={x}: {u} vs {v}"),
        }
    }
    println!();
    println!("## 3. the same pairs on the signed domain");
    println!();
    for (na, fa, nb, fb) in &pairs {
        match disagreement(*fa, *fb, s, lo, hi) {
            None => println!("  signed:  {na} == {nb}   (no disagreement in {lo}..={hi})"),
            Some((x, u, v)) => println!("  signed:  {na} != {nb}   at x={x}: {u} vs {v}"),
        }
    }
    println!();

    // ---- 4. pairwise distinctness on the signed domain --------------------
    println!("## 4. pairwise distinctness of every named mode, signed domain");
    println!();
    let m = modes();
    let mut coincidences = 0;
    for i in 0..m.len() {
        for j in (i + 1)..m.len() {
            if disagreement(m[i].1, m[j].1, s, lo, hi).is_none() {
                println!("  COINCIDE: {} == {}", m[i].0, m[j].0);
                coincidences += 1;
            }
        }
    }
    println!("  {coincidences} coinciding pair(s) out of {} on the signed domain",
        m.len() * (m.len() - 1) / 2);
    println!();
    println!("  and on the non-negative domain:");
    let mut nn = 0;
    for i in 0..m.len() {
        for j in (i + 1)..m.len() {
            if disagreement(m[i].1, m[j].1, s, 0, hi).is_none() {
                println!("  COINCIDE: {} == {}", m[i].0, m[j].0);
                nn += 1;
            }
        }
    }
    println!("  {nn} coinciding pair(s) out of {}", m.len() * (m.len() - 1) / 2);
    println!();

    // ---- 5. the grid is fixed by every mode -------------------------------
    println!("## 5. every mode is the identity on grid points (the retraction property)");
    println!();
    for (name, f) in modes() {
        let mut bad = 0;
        let mut k = lo / d;
        while k <= hi / d {
            if f(k * d, d) != k {
                bad += 1;
            }
            k += 1;
        }
        println!("  {:<22} off-grid results on grid points: {bad}", name);
    }
    println!();
    println!("  and at s = 0, where the grid is the whole value set:");
    for (name, f) in modes() {
        let mut bad = 0;
        for x in lo..=hi {
            if f(x, 1) != x {
                bad += 1;
            }
        }
        println!("  {:<22} non-identity results at d = 1: {bad}", name);
    }
    println!();

    // ---- controls: each of these MUST report a witness --------------------
    println!("## controls. Each must find a witness. A clean line here voids the file.");
    println!();
    let mut controls_ok = true;

    // C1: away_from_zero must NOT be equivariant.
    match equivariance_witness(div_away_from_zero, s, lo, hi, shifts) {
        Some((x, n, g, w)) => println!("  C1 EXPECTED-FAIL ok: away_from_zero not equivariant, x={x} n={n} {g} vs {w}"),
        None => { println!("  C1 BROKEN: away_from_zero reported equivariant"); controls_ok = false; }
    }
    // C2: away_from_zero must differ from ceil somewhere signed.
    match disagreement(div_away_from_zero, div_ceil, s, lo, hi) {
        Some((x, u, v)) => println!("  C2 EXPECTED-FAIL ok: away_from_zero != ceil signed, x={x}: {u} vs {v}"),
        None => { println!("  C2 BROKEN: away_from_zero == ceil on the signed domain"); controls_ok = false; }
    }
    // C3: a mode that is not a retraction must be caught by the check in 5.
    fn floor_plus_one(x: i128, d: i128) -> i128 { div_floor(x, d) + 1 }
    let mut bad = 0;
    let mut k = lo / d;
    while k <= hi / d {
        if floor_plus_one(k * d, d) != k { bad += 1; }
        k += 1;
    }
    if bad > 0 {
        println!("  C3 EXPECTED-FAIL ok: floor_plus_one is not the identity on {bad} grid points");
    } else {
        println!("  C3 BROKEN: floor_plus_one reported as a retraction");
        controls_ok = false;
    }
    // C4: floor MUST be equivariant. If the checker cannot report equivariance
    // at all, C1 and C2 mean nothing.
    match equivariance_witness(div_floor, s, lo, hi, shifts) {
        None => println!("  C4 EXPECTED-PASS ok: floor is equivariant, so the checker can say both"),
        Some(_) => { println!("  C4 BROKEN: floor reported not equivariant"); controls_ok = false; }
    }

    println!();
    println!("controls: {}", if controls_ok { "all four behaved as required" } else { "BROKEN, file is void" });
}
