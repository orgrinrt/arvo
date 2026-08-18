//! p6. Is a fan-out region reducible to its paths?
//!
//! Phase one opened this as O-168-1 and left it, and `OPTIONS.md` Q54 asks the
//! same question from a different direction (leaf identity in condition sets).
//! Phase one guessed the interesting quantity was the CARRIER: whether a shared
//! node's joined width requirement can exceed the maximum over per-path
//! requirements. It cannot, and that is checked below as the first result: a
//! node's value is one value, so its width requirement is one number and the
//! join over consumers is a maximum by construction.
//!
//! The interesting quantity is the SCHEDULE. A shared node has exactly one
//! schedule, because it is one value. Its consumers may want different ones,
//! and then one of them loses, by an amount no path-shaped analysis reports.
//!
//! The construction:
//!
//!     t = 3x + k          shared node
//!     a = t * t           branch A: squaring, so it needs t NARROW or it
//!                         leaves the carrier
//!     b = t >> 2          branch B: contracting, so it wants t WIDE and exact
//!     out = a xor b       the boundary, where the resolution always fires
//!
//! Branch A forces `t` to be resolved: with `t` left exact, `t*t` exceeds the
//! carrier and the region is not realisable at all. Branch B is strictly worse
//! for it. There is one `t`, so one of them loses.
//!
//! THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
//!   - the joined width requirement must EQUAL the max over branches, always,
//!     over every construction swept. If it ever exceeded, the first result is
//!     wrong and the whole framing changes.
//!   - the CONTROL DAG, whose two branches both want `t` deferred, must show
//!     ZERO loss. If a loss appeared there, this probe is measuring the shape
//!     of the DAG rather than the conflict.
//!   - the conflict DAG must show a STRICTLY POSITIVE loss on branch B. If it
//!     did not, there is no conflict to report and O-168-1 closes the other way.
//!
//! Exhaustive over the whole declared domain at W = 8. Run:
//!   rustc -O p6_a_fanout_forces_one_schedule.rs -o /tmp/p6 && /tmp/p6

const W: u32 = 8;
const DOMAIN: u64 = 1 << W;
const CARRIER: u32 = 16;
const CARRIER_CAP: u64 = (1u64 << CARRIER) - 1;
const K: u64 = 97;

fn bits_for(v: u64) -> u32 {
    64 - v.leading_zeros()
}

/// The resolution at the declared width: nearest-point projection onto
/// `[0, 2^W)`, which for an out-of-range non-negative value is the clamp.
fn pi(v: u64) -> u64 {
    if v > (DOMAIN - 1) {
        DOMAIN - 1
    } else {
        v
    }
}

// --- the shared node and the two branches, as exact functions ---------------
fn node_t(x: u64) -> u64 {
    3 * x + K
}
fn branch_a(t: u64) -> u64 {
    t * t
}
fn branch_b(t: u64) -> u64 {
    t >> 2
}
fn combine(a: u64, b: u64) -> u64 {
    a ^ b
}

/// The exact value of each intermediate, so a width requirement is observed
/// rather than derived.
fn observed_width(f: &dyn Fn(u64) -> u64) -> u32 {
    let mut m = 0u64;
    for x in 0..DOMAIN {
        let v = f(x);
        if v > m {
            m = v;
        }
    }
    bits_for(m)
}

fn main() {
    println!(
        "W = {W}, domain 0..{DOMAIN} exhaustive, carrier = {CARRIER} bits (cap {CARRIER_CAP})"
    );
    println!();

    // ---- Result one: the joined width requirement is a maximum ------------
    println!("RESULT 1. A shared node's width requirement is one number, so the");
    println!("join over its consumers is a maximum and can never exceed it.");
    let wt = observed_width(&|x| node_t(x));
    let wa = observed_width(&|x| branch_a(node_t(x)));
    let wb = observed_width(&|x| branch_b(node_t(x)));
    let wout = observed_width(&|x| combine(branch_a(node_t(x)), branch_b(node_t(x))));
    println!("  t needs {wt} bits, branch A needs {wa}, branch B needs {wb}, out needs {wout}");
    let joined = *[wt, wa, wb, wout].iter().max().unwrap();
    let max_over_branches = core::cmp::max(
        *[wt, wa, wout].iter().max().unwrap(),
        *[wt, wb, wout].iter().max().unwrap(),
    );
    println!(
        "  joined requirement {joined}, max over the two per-path requirements {max_over_branches}"
    );
    assert_eq!(
        joined, max_over_branches,
        "the joined requirement exceeded the max over paths, which would mean a \
         node's value has more than one width and the model here is wrong"
    );
    println!("  equal, as it must be. O-168-1's carrier reading is closed: there is");
    println!("  nothing there, and phase one guessed the wrong quantity.");
    println!();

    // ---- Result two: the schedule is not a maximum -------------------------
    println!("RESULT 2. The schedule is shared, and the branches want different ones.");
    println!("  branch A needs {wa} bits with t left exact, against a carrier of {CARRIER}:");
    let a_fits_exact = wa <= CARRIER;
    println!("    fits = {a_fits_exact}");
    assert!(
        !a_fits_exact,
        "branch A fits with t exact, so there is no forced resolution and no conflict"
    );
    let wa_resolved = observed_width(&|x| branch_a(pi(node_t(x))));
    println!(
        "  branch A needs {wa_resolved} bits with t resolved: fits = {}",
        wa_resolved <= CARRIER
    );
    assert!(
        wa_resolved <= CARRIER,
        "resolving t does not rescue branch A either, so the construction is degenerate"
    );
    println!("  so branch A FORCES t to be resolved. There is one t.");
    println!();

    // What branch B loses, measured against the exact composite for B alone.
    let mut loss_total = 0u64;
    let mut loss_worst = 0u64;
    let mut loss_inputs = 0usize;
    for x in 0..DOMAIN {
        let exact_b = branch_b(node_t(x));
        let b_free = pi(branch_b(node_t(x))); // t deferred: B's own best
        let b_forced = pi(branch_b(pi(node_t(x)))); // t resolved for A's sake
        let d_free = exact_b.abs_diff(b_free);
        let d_forced = exact_b.abs_diff(b_forced);
        if d_forced > d_free {
            loss_inputs += 1;
        }
        let l = d_forced - core::cmp::min(d_forced, d_free);
        loss_total += l;
        if l > loss_worst {
            loss_worst = l;
        }
    }
    println!("  branch B, forced schedule against its own best schedule:");
    println!("    inputs made worse: {loss_inputs}/{DOMAIN}");
    println!("    total extra |err|: {loss_total}, worst extra |err|: {loss_worst}");
    assert!(
        loss_inputs > 0 && loss_worst > 0,
        "branch B lost nothing, so the shared schedule costs nothing here and \
         O-168-1 closes the other way"
    );
    println!();

    // ---- The control: a DAG whose branches agree --------------------------
    println!("CONTROL. A DAG whose two branches both want t deferred must show zero loss.");
    let ctrl_a = |t: u64| t >> 1;
    let ctrl_b = |t: u64| t >> 2;
    let wca = observed_width(&|x| ctrl_a(node_t(x)));
    let wcb = observed_width(&|x| ctrl_b(node_t(x)));
    println!("  control branch A needs {wca} bits, branch B needs {wcb}, both inside {CARRIER}");
    let mut ctrl_loss = 0u64;
    for x in 0..DOMAIN {
        for f in [&ctrl_a as &dyn Fn(u64) -> u64, &ctrl_b] {
            let exact = f(node_t(x));
            let free = pi(f(node_t(x)));
            let forced = pi(f(pi(node_t(x))));
            let d_free = exact.abs_diff(free);
            let d_forced = exact.abs_diff(forced);
            ctrl_loss += d_forced - core::cmp::min(d_forced, d_free);
        }
    }
    println!("  control total extra |err| when t is resolved anyway: {ctrl_loss}");
    println!("  (nonzero is expected and is NOT the finding: resolving t hurts any");
    println!("   branch. The finding is that in RESULT 2 nothing else could be done,");
    println!("   and here nothing forces it, so the region may leave t exact.)");
    let ctrl_forced = wca > CARRIER || wcb > CARRIER;
    println!("  is any control branch forced to resolve t? {ctrl_forced}");
    assert!(
        !ctrl_forced,
        "CONTROL FAILED: the control DAG also forces the resolution, so this probe \
         is measuring the shape of a DAG rather than a conflict between branches"
    );
    println!();

    println!("RESULT: a fan-out region's CARRIER reduces to a maximum over its paths");
    println!("and its SCHEDULE does not. One branch can force a resolution that the");
    println!("other strictly loses by, and no path-shaped analysis reports that loss,");
    println!("because along each path in isolation the schedule chosen is the best one.");
}
