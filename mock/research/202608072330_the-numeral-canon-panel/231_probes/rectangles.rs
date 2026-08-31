// Can the predicate notation express the region `138 #1` states?
//
// `229` calls `or unsigned with signed` a tokenisation artifact and concludes
// the three spans carrying it are portable, agreeing with `span_verdicts.sh`'s
// own footer that the honest portable count is 5 of 64 rather than 4. `230`
// calls it a value binding two axes and concludes they are not portable and
// want an `intermediate_signedness` row.
//
// Both are wrong, and the reason is structural rather than a matter of taste.
// The source value is
//
//     signedness = signed, or unsigned with signed intermediates
//
// which names a region over TWO coordinates: the declared format's signedness
// and the intermediate's. Under `a_predicate_names_an_axis_once`, a hard-error
// lint, a predicate carries at most one entry per axis, so the region a
// predicate denotes is the CARTESIAN PRODUCT of its per-axis spans. A product
// of spans is a rectangle.
//
// This enumerates every region over a two-coordinate space and every product
// of per-coordinate spans, and reports which regions are expressible. If the
// source's region is not among them, adding a row does not make the span
// writable and neither reading's repair works.
//
// Controls, outcomes written before the run:
//   R1  a region that IS a product must come out expressible. `{signed} x
//       {signed, unsigned}` is the check; if it does not, the matcher is
//       broken and every negative below is worthless.
//   R2  the empty region and the full region must both come out expressible,
//       since both are products.
//   R3  the count of expressible regions must equal (2^2 - 1 + 1)^2 = 16, one
//       per pair of subsets, deduplicated. Stated as an independent arithmetic
//       check on the enumeration rather than read off it.
//   R4  at least one region must come out INEXPRESSIBLE, or the notation
//       expresses everything and the question is empty.

/// A point in the two-coordinate space: (declared signedness, intermediate
/// signedness), each `false` = unsigned, `true` = signed.
type Point = (bool, bool);

const ALL: [Point; 4] = [(false, false), (false, true), (true, false), (true, true)];

fn name(p: Point) -> &'static str {
    match p {
        (false, false) => "unsigned/unsigned",
        (false, true) => "unsigned/signed",
        (true, false) => "signed/unsigned",
        (true, true) => "signed/signed",
    }
}

/// A region is a subset of the four points, as a 4-bit mask over `ALL`.
fn members(mask: u8) -> Vec<Point> {
    ALL.iter()
        .enumerate()
        .filter(|(i, _)| mask >> i & 1 == 1)
        .map(|(_, p)| *p)
        .collect()
}

fn show(mask: u8) -> String {
    let m = members(mask);
    if m.is_empty() {
        return "{}".into();
    }
    format!(
        "{{{}}}",
        m.iter().map(|p| name(*p)).collect::<Vec<_>>().join(", ")
    )
}

/// Every product of a span on each coordinate, as masks.
fn products() -> Vec<u8> {
    let mut out = Vec::new();
    for a in 0..4u8 {
        // subset of {unsigned, signed} for coordinate 1
        for b in 0..4u8 {
            let mut m = 0u8;
            for (i, p) in ALL.iter().enumerate() {
                let in_a = if p.0 { a >> 1 & 1 } else { a & 1 } == 1;
                let in_b = if p.1 { b >> 1 & 1 } else { b & 1 } == 1;
                if in_a && in_b {
                    m |= 1 << i;
                }
            }
            out.push(m);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn main() {
    let prods = products();
    println!("### every region a product of per-axis spans can denote");
    for m in &prods {
        println!("  {}", show(*m));
    }
    println!("  {} distinct regions of 16 possible subsets", prods.len());

    // The source's region: everything except plain unsigned with an unsigned
    // intermediate. `signedness = signed` covers both signed rows whatever the
    // intermediate is; `or unsigned with signed intermediates` adds one more.
    let source: u8 = ALL
        .iter()
        .enumerate()
        .filter(|(_, p)| p.0 || p.1)
        .fold(0u8, |acc, (i, _)| acc | 1 << i);

    println!("\n### the region `138 #1` states");
    println!("  {}", show(source));
    let ok = prods.contains(&source);
    println!(
        "  expressible as a product of per-axis spans: {}",
        if ok { "YES" } else { "NO" }
    );

    println!("\n### the nearest products, and what each gets wrong");
    for m in &prods {
        let extra: Vec<_> = members(*m ^ (m & source))
            .iter()
            .map(|p| name(*p))
            .collect();
        let missing: Vec<_> = members(source & !*m).iter().map(|p| name(*p)).collect();
        if extra.len() + missing.len() == 1 {
            println!(
                "  {:<52} claims too much: {:?}  misses: {:?}",
                show(*m),
                extra,
                missing
            );
        }
    }

    println!("\n### R1, a region that is a product must be expressible");
    let r1: u8 = ALL
        .iter()
        .enumerate()
        .filter(|(_, p)| p.0)
        .fold(0u8, |acc, (i, _)| acc | 1 << i);
    println!(
        "  {{signed}} x {{signed, unsigned}} = {} -> {}",
        show(r1),
        if prods.contains(&r1) { "PASS" } else { "FAIL" }
    );
    println!("### R2, the empty and the full region");
    println!(
        "  empty {} full {}",
        if prods.contains(&0) { "PASS" } else { "FAIL" },
        if prods.contains(&0b1111) {
            "PASS"
        } else {
            "FAIL"
        }
    );
    println!("### R3, the enumeration's own count");
    println!(
        "  {} distinct products from 16 subset pairs: {}",
        prods.len(),
        if prods.len() == 10 {
            "PASS, 16 pairs collapse to 10 because every pair with an empty side is the empty region"
        } else {
            "FAIL, the enumeration is not doing what the comment says"
        }
    );
    println!("### R4, at least one region must be inexpressible");
    let bad = (0..16u8).filter(|m| !prods.contains(m)).count();
    println!(
        "  {} of 16 subsets are not products: {}",
        bad,
        if bad > 0 { "PASS" } else { "FAIL" }
    );
}
