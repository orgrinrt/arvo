// P2b. My first hypothesis was wrong and its run is kept as partial_v1_REFUTED.out.
//
// v1 built a pair whose interior division used the SAME divisor in both
// realisations, so both were undefined on the same 37 of 256 inputs: 0
// definedness splits, at both profiles. That shows the pair SHARES a definedness
// domain, so clause 1's hypothesis holds and the pair is indistinguishable,
// which is the opposite of what I set out to check. Widening a carrier does not
// by itself move which inputs are undefined.
//
// The split needs the widening to CHANGE the divisor: A computes the divisor at
// the declared width where it wraps, B computes it wider and narrows once. Then
// the two divide by different values and one can be zero where the other is not.
// That is 172's "constructed witness", and this is my independent construction
// of one.
//
// THE CASES THAT MUST FAIL
//   C-A  on inputs where both are defined, the two must AGREE on the value,
//        else they are not one boundary function
//   C-B  a TOTAL-interior control pair must show zero splits at both profiles
//   C-C  the split count must be > 0, else nothing is shown
//   C-D  v1's shared-divisor pair must still show ZERO splits here, so the
//        difference is the construction and not the harness

use std::panic;

fn profile() -> &'static str {
    if cfg!(debug_assertions) { "debug-assertions=on" } else { "debug-assertions=off" }
}

// A: divisor computed at the declared width, where the multiply wraps.
#[inline(never)]
fn split_a(x: u8) -> u8 {
    let d = x.wrapping_mul(37).wrapping_sub(60);   // unbound interior
    255u8 / d
}
// B: same divisor computed wider, narrowed once. Same boundary function wherever
// both are defined, because the narrow is the low byte of the wide value.
#[inline(never)]
fn split_b(x: u8) -> u8 {
    let d = (((x as u32) * 37).wrapping_sub(60) & 0xFF) as u8;
    255u8 / d
}

// v1's pair, kept as C-D: same divisor in both, so no split is possible.
#[inline(never)]
fn same_a(x: u8) -> u8 { let d = x % 7; 255u8 / d }
#[inline(never)]
fn same_b(x: u8) -> u8 { let d = ((x as u32) % 7) as u8; 255u8 / d }

// C-B: total interior.
#[inline(never)]
fn tot_a(x: u8) -> u8 { x.wrapping_mul(37).wrapping_sub(60).wrapping_add(1) }
#[inline(never)]
fn tot_b(x: u8) -> u8 { ((((x as u32) * 37).wrapping_sub(60) & 0xFF) as u8).wrapping_add(1) }

fn sweep(a: fn(u8) -> u8, b: fn(u8) -> u8) -> (u32, u32, u32, u32) {
    let (mut both_ok, mut both_bad, mut split, mut dis) = (0, 0, 0, 0);
    for x in 0..=255u8 {
        let ra = panic::catch_unwind(|| a(x));
        let rb = panic::catch_unwind(|| b(x));
        match (ra, rb) {
            (Ok(va), Ok(vb)) => { both_ok += 1; if va != vb { dis += 1; } }
            (Err(_), Err(_)) => both_bad += 1,
            _ => split += 1,
        }
    }
    (both_ok, both_bad, split, dis)
}

fn main() {
    panic::set_hook(Box::new(|_| {}));   // the panics are the measurement, not the output
    println!("== P2b, profile = {} ==", profile());
    println!("{:>34} {:>9} {:>10} {:>7} {:>12}", "pair", "both ok", "both bad", "SPLIT", "value dis");
    for (name, a, b) in [
        ("constructed: divisor widened", split_a as fn(u8) -> u8, split_b as fn(u8) -> u8),
        ("C-D  v1's shared divisor", same_a as fn(u8) -> u8, same_b as fn(u8) -> u8),
        ("C-B  total interior", tot_a as fn(u8) -> u8, tot_b as fn(u8) -> u8),
    ] {
        let (ok, bad, sp, dis) = sweep(a, b);
        println!("{name:>34} {ok:>9} {bad:>10} {sp:>7} {dis:>12}");
    }
    let (_, _, sp, dis) = sweep(split_a, split_b);
    let (_, _, sp_same, _) = sweep(same_a, same_b);
    let (_, _, sp_tot, dis_tot) = sweep(tot_a, tot_b);
    println!();
    println!("C-A  value disagreements where both defined: {dis}   (must be 0)");
    println!("C-B  total-interior splits: {sp_tot}, disagreements: {dis_tot}   (both must be 0)");
    println!("C-C  constructed splits: {sp}   (must be > 0)");
    println!("C-D  v1's shared-divisor splits: {sp_same}   (must be 0)");
    println!();
    println!("VERDICT at {}: a partial interior gives a binding-free definedness", profile());
    println!("  channel: {}", sp > 0);
}
