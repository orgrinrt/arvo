// PROBE p1b. p1 established a characterisation of fully-holding boxes and
// cross-checked it. This probe measures the consequence DIRECTLY rather than
// deriving it from the characterisation, because the consequence is p1's
// headline and deriving it from a closed form would make the closed form
// load-bearing twice.
//
// The question: is there any fully-holding declared-range box in which a clamp
// event actually fires and which is NOT one of the two degenerate shapes
// (a identically zero, c identically zero)?
//
// If the answer is no, then every non-degenerate region a declared-range
// lifting of P4 can reach is a region on which saturating arithmetic and exact
// arithmetic compute the same function, so the arm the lifting licenses is not
// about saturation.
//
// Counted at every width in the model band, both the population and the
// residue, with a negative control that has to be non-zero for the instrument
// to be measuring anything.

fn sat_add(x: u32, y: u32, maxv: u32) -> u32 {
    let s = x + y;
    if s > maxv {
        maxv
    } else {
        s
    }
}
fn sat_sub(x: u32, y: u32) -> u32 {
    if y > x {
        0
    } else {
        x - y
    }
}
fn law_holds(a: u32, b: u32, c: u32, maxv: u32) -> bool {
    sat_sub(sat_add(a, b, maxv), c) == sat_add(a, sat_sub(b, c), maxv)
}
fn clamp_fires(a: u32, b: u32, c: u32, maxv: u32) -> bool {
    a + b > maxv || b < c
}

fn main() {
    println!("p1b: is every non-degenerate lifted box clamp-free\n");
    println!(
        "{:>5} {:>12} {:>14} {:>16} {:>22} {:>18}",
        "width", "hold boxes", "hold + clamp", "hold + clamp +", "non-degenerate", "control:"
    );
    println!(
        "{:>5} {:>12} {:>14} {:>16} {:>22} {:>18}",
        "", "", "", "non-degen", "clamp-free hold boxes", "boxes w/ clamp"
    );

    for w in 2u32..=5 {
        let maxv = (1u32 << w) - 1;
        let n = maxv + 1;
        let mut holding: u64 = 0;
        let mut holding_with_clamp: u64 = 0;
        let mut holding_with_clamp_nondegenerate: u64 = 0;
        let mut holding_clampfree: u64 = 0;
        let mut any_box_with_clamp: u64 = 0;
        let mut witness: Option<(u32, u32, u32, u32, u32, u32)> = None;

        for la in 0..n {
            for ha in la..n {
                for lb in 0..n {
                    for hb in lb..n {
                        for lc in 0..n {
                            for hc in lc..n {
                                let mut all_hold = true;
                                let mut a_clamp_somewhere = false;
                                for a in la..=ha {
                                    for b in lb..=hb {
                                        for c in lc..=hc {
                                            if !law_holds(a, b, c, maxv) {
                                                all_hold = false;
                                            }
                                            if clamp_fires(a, b, c, maxv) {
                                                a_clamp_somewhere = true;
                                            }
                                        }
                                    }
                                }
                                if a_clamp_somewhere {
                                    any_box_with_clamp += 1;
                                }
                                if all_hold {
                                    holding += 1;
                                    let a_is_zero = la == 0 && ha == 0;
                                    let c_is_zero = lc == 0 && hc == 0;
                                    let degenerate = a_is_zero || c_is_zero;
                                    if a_clamp_somewhere {
                                        holding_with_clamp += 1;
                                        if !degenerate {
                                            holding_with_clamp_nondegenerate += 1;
                                            if witness.is_none() {
                                                witness = Some((la, ha, lb, hb, lc, hc));
                                            }
                                        }
                                    } else {
                                        holding_clampfree += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!(
            "{:>5} {:>12} {:>14} {:>16} {:>22} {:>18}",
            w,
            holding,
            holding_with_clamp,
            holding_with_clamp_nondegenerate,
            holding_clampfree,
            any_box_with_clamp
        );
        if let Some(wt) = witness {
            println!(
                "      WITNESS non-degenerate holding box with a clamp: {:?}",
                wt
            );
        }
    }

    println!("\nreading: column 4 is the residue. Zero there means every fully-holding");
    println!("declared-range box in which a clamp can fire is one of the two boxes that");
    println!("pin an operand to zero. Column 6 is the control: boxes in which a clamp");
    println!("fires at all, holding or not, and it is large, so the clamp detector is");
    println!("not silently returning false.");
}
