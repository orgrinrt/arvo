//! p5: how many unprojected additions can chain before the carrier becomes
//! observable, and is the obvious closed form tight?
//!
//! Seat 226. `p3` found that an operation with its projection to the declared
//! width omitted is observable exactly where its growth above the declared width
//! exceeds the headroom the narrowest admissible carrier already has. That is an
//! arm with a const predicate, so it is worth knowing how wide the arm is: a
//! chain of k additions accumulated without projecting between steps, then
//! projected once at the end, agrees across carriers up to some k and not past
//! it.
//!
//! Two routes to that boundary, on purpose, because a closed form checked
//! against itself is not checked.
//!
//!   exact   the classes split exactly when the largest reachable sum reaches
//!           the narrowest admissible carrier, and the largest reachable sum is
//!           at all-maximum inputs, so one witness decides each cell.
//!   form    the growth of a (k+1)-term sum is ceil(log2(k+1)) bits, so the
//!           chain is free while that is at most the headroom.
//!
//! Where the two disagree the closed form is wrong, and which way it is wrong is
//! the finding: a conservative form leaves free chain length unclaimed, and an
//! optimistic one would license an unsound elision.
//!
//! The cases that must fail, run and reported:
//!   C1  a deliberately optimistic form, headroom + 1 bits of slack, must be
//!       reported as disagreeing with the exact condition, and must disagree in
//!       the unsound direction. Without it a clean agreement says nothing.
//!   C2  the exact condition must report BOTH outcomes at every width, so no
//!       width is decided by the boundary sitting outside the swept range.

const NATIVE: [u32; 5] = [8, 16, 32, 64, 128];

fn narrowest_carrier(w: u32) -> u32 {
    NATIVE.iter().copied().find(|&c| c >= w).unwrap()
}

/// Do two carriers give different answers for a (k+1)-term unprojected sum?
/// The largest reachable sum is at all-maximum inputs, so one witness decides.
fn splits_exact(w: u32, k: u32) -> bool {
    let cmin = narrowest_carrier(w);
    let max_sum = u128::from(k + 1) * ((1u128 << w) - 1);
    // Two carriers disagree exactly when the narrowest one wraps and a wider one
    // does not. Above 128 bits there is no wider one, which the sweep stays below.
    max_sum >= (1u128 << cmin)
}

fn ceil_log2(n: u32) -> u32 {
    32 - (n - 1).leading_zeros()
}

/// The obvious closed form: free while the growth fits the headroom.
fn splits_by_form(w: u32, k: u32) -> bool {
    let headroom = narrowest_carrier(w) - w;
    ceil_log2(k + 1) > headroom
}

/// C1: the same form given one bit more slack than it has.
fn splits_optimistic(w: u32, k: u32) -> bool {
    let headroom = narrowest_carrier(w) - w + 1;
    ceil_log2(k + 1) > headroom
}

fn main() {
    let widths: Vec<u32> = (3..=16).collect();
    let ks: Vec<u32> = (1..=4096).collect();

    println!("== the free chain length, both routes ==");
    println!("  W  headroom  longest free chain, exact / closed form");
    let mut form_disagrees = 0usize;
    let mut form_unsound = 0usize;
    let mut both_outcomes = true;
    for &w in &widths {
        let headroom = narrowest_carrier(w) - w;
        let last_exact = ks
            .iter()
            .copied()
            .take_while(|&k| !splits_exact(w, k))
            .last();
        let last_form = ks
            .iter()
            .copied()
            .take_while(|&k| !splits_by_form(w, k))
            .last();
        let saw_split = ks.iter().any(|&k| splits_exact(w, k));
        let saw_free = ks.iter().any(|&k| !splits_exact(w, k));
        // C2, split in two because its first form demanded the impossible. At
        // zero headroom the narrowest carrier IS the declared width, so a
        // two-term sum already wraps it and no chain is ever free. Demanding
        // both outcomes there fails on correct data. Where there is headroom
        // both outcomes must appear, and where there is none only the split
        // may, which is a second checkable claim rather than an exemption.
        let ok = if headroom > 0 {
            saw_split && saw_free
        } else {
            saw_split && !saw_free
        };
        if !ok {
            both_outcomes = false;
        }
        for &k in &ks {
            if splits_exact(w, k) != splits_by_form(w, k) {
                form_disagrees += 1;
                // Unsound means the form says free where the exact says it splits.
                if splits_exact(w, k) && !splits_by_form(w, k) {
                    form_unsound += 1;
                }
            }
        }
        println!(
            "  {w:<3}{headroom:<10}{:<8}{:?}",
            last_exact.map_or("0".to_string(), |v| v.to_string()),
            last_form.unwrap_or(0)
        );
    }

    println!("\n== C1 control: the same form with one bit of slack it does not have ==");
    let mut opt_disagrees = 0usize;
    let mut opt_unsound = 0usize;
    for &w in &widths {
        for &k in &ks {
            if splits_exact(w, k) != splits_optimistic(w, k) {
                opt_disagrees += 1;
                if splits_exact(w, k) && !splits_optimistic(w, k) {
                    opt_unsound += 1;
                }
            }
        }
    }
    println!("  cells where the optimistic form disagrees with the exact: {opt_disagrees}");
    println!("  of those, unsound (form says free, exact says it splits):  {opt_unsound}");

    println!("\n== verdict ==");
    println!("  cells where the closed form disagrees with the exact: {form_disagrees}");
    println!("  of those, unsound:                                    {form_unsound}");
    println!(
        "  C1 the optimistic form is caught, and caught unsound:  {}",
        opt_unsound > 0
    );
    println!("  C2 both outcomes where there is headroom, split only where there is none: {both_outcomes}");
    let pass = form_unsound == 0 && opt_unsound > 0 && both_outcomes;
    println!(
        "\n  RESULT: {}",
        if pass {
            "the closed form is sound and conservative; the exact condition is the wider arm"
        } else {
            "INCONCLUSIVE"
        }
    );
    std::process::exit(if pass { 0 } else { 1 });
}
