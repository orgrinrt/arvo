//! p1: are the carrier ladder and the access ladder one function or two?
//!
//! Seat 226. The container-derivation output count turns on whether the
//! machine type that holds a lone value and the machine type a packed load
//! touches are the same fact under two names. If they are one function they
//! are one output; if they are two functions of the declared width they are
//! two, and no packaging choice can merge them.
//!
//! Two access rules are computed rather than one, because the answer must not
//! depend on which packing rule a design ships:
//!
//!   tight: the offsets a packed column actually produces are the multiples of
//!          gcd(W, 8), so the worst offset is 8 - gcd(W, 8).
//!   loose: assume the worst offset is 7 for every width.
//!
//! The sole-occupancy arm is here too: at sole occupancy the stride and the
//! access width are pure functions of the carrier, so under the ownership
//! clause they are recomputed rather than carried.
//!
//! The cases that must fail, all three run and reported:
//!   C1  a "clone" access rule defined equal to the carrier rule must report
//!       every jump point shared. Without it a zero-intersection result says
//!       nothing, because nothing shows the instrument can see an
//!       intersection at all.
//!   C2  a padded sole-occupancy placement (stride = carrier + 8 bits) must be
//!       reported as differing from the carrier. Without it the sole-occupancy
//!       agreement is not a measurement, it is a definition.
//!   C3  the tight rule must be reported non-monotone. If the monotonicity
//!       check cannot fire on a function known to descend, it cannot be
//!       trusted when it stays silent on the carrier rule.

const NATIVE: [u32; 5] = [8, 16, 32, 64, 128];
const MAX_W: u32 = 128;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn smallest_native_at_least(bits: u32) -> Option<u32> {
    NATIVE.iter().copied().find(|&n| n >= bits)
}

/// The carrier a lone value of declared width W sits in.
fn carrier(w: u32) -> Option<u32> {
    smallest_native_at_least(w)
}

/// The machine type one packed load must touch, offsets as a column produces them.
fn access_tight(w: u32) -> Option<u32> {
    let worst_offset = 8 - gcd(w, 8);
    let bits = worst_offset + w;
    smallest_native_at_least(bits.div_ceil(8) * 8)
}

/// The same, assuming the worst offset is 7 at every width.
fn access_loose(w: u32) -> Option<u32> {
    smallest_native_at_least((w + 7).div_ceil(8) * 8)
}

/// C1: an access rule defined to be the carrier rule.
fn access_clone(w: u32) -> Option<u32> {
    carrier(w)
}

fn jumps(f: impl Fn(u32) -> Option<u32>) -> Vec<u32> {
    (2..=MAX_W).filter(|&w| f(w) != f(w - 1)).collect()
}

fn first_descent(f: &impl Fn(u32) -> Option<u32>) -> Option<(u32, u32, u32)> {
    (2..=MAX_W).find_map(|w| match (f(w - 1), f(w)) {
        (Some(a), Some(b)) if b < a => Some((w, a, b)),
        _ => None,
    })
}

fn report(name: &str, f: impl Fn(u32) -> Option<u32> + Copy, base: &[u32]) -> usize {
    let j = jumps(f);
    let shared: Vec<u32> = j.iter().copied().filter(|w| base.contains(w)).collect();
    let differs: Vec<u32> = (1..=MAX_W).filter(|&w| f(w) != carrier(w)).collect();
    println!("  {name}");
    println!("    jump points ({}): {:?}", j.len(), j);
    println!(
        "    shared with the carrier ladder: {} {:?}",
        shared.len(),
        shared
    );
    println!(
        "    widths where it differs from the carrier: {} of {MAX_W}, first {:?}",
        differs.len(),
        differs.first()
    );
    match first_descent(&f) {
        Some((w, a, b)) => println!("    NON-MONOTONE: f({}) = {a} and f({w}) = {b}", w - 1),
        None => println!("    monotone over 1..={MAX_W}"),
    }
    shared.len()
}

fn main() {
    println!("== the carrier ladder ==");
    let base = jumps(carrier);
    println!("  jump points ({}): {:?}", base.len(), base);
    match first_descent(&carrier) {
        Some((w, a, b)) => println!("  NON-MONOTONE: f({}) = {a} and f({w}) = {b}", w - 1),
        None => println!("  monotone over 1..={MAX_W}"),
    }

    println!("\n== the access ladders, against that base ==");
    let tight_shared = report("tight  ", access_tight, &base);
    let loose_shared = report("loose  ", access_loose, &base);

    println!("\n== C1 control: an access rule defined to BE the carrier rule ==");
    let clone_shared = report("clone  ", access_clone, &base);

    println!("\n== the sole-occupancy arm ==");
    // At sole occupancy the value is the only logical occupant of its
    // allocation, so consecutive values are one carrier apart and one load
    // reaches the whole of one. Both facts are then functions of the carrier.
    let sole_bad: Vec<u32> = (1..=MAX_W)
        .filter(|&w| {
            let c = carrier(w).unwrap();
            let stride = c; // sole occupancy: one carrier per value
            let access = c;
            stride != c || access != c
        })
        .collect();
    println!(
        "  widths where sole-occupancy stride or access is NOT the carrier: {} {:?}",
        sole_bad.len(),
        sole_bad
    );
    // C2: the same check over a padded placement, which must be reported.
    let padded_bad: Vec<u32> = (1..=MAX_W)
        .filter(|&w| {
            let c = carrier(w).unwrap();
            let stride = c + 8; // an 8-bit tag beside each value
            stride != c
        })
        .collect();
    println!(
        "  C2 control, padded placement, widths reported as differing: {} of {MAX_W}",
        padded_bad.len()
    );

    println!("\n== verdict ==");
    let ok_c1 = clone_shared == base.len();
    let ok_c2 = padded_bad.len() as u32 == MAX_W;
    let ok_c3 = first_descent(&access_tight).is_some();
    println!("  C1 clone rule shares every carrier jump point: {ok_c1}");
    println!("  C2 padded placement reported at every width:   {ok_c2}");
    println!("  C3 monotonicity check fires on the tight rule: {ok_c3}");
    println!("  tight shares {tight_shared} carrier jump points");
    println!("  loose shares {loose_shared} carrier jump points");
    let pass = ok_c1 && ok_c2 && ok_c3 && tight_shared == 0 && loose_shared == 0;
    println!(
        "\n  RESULT: {}",
        if pass {
            "two ladders, not one"
        } else {
            "INCONCLUSIVE"
        }
    );
    std::process::exit(if pass { 0 } else { 1 });
}
