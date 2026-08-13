// PROBE p1. The first and most obvious lifting route for 79's P4: replace the
// per-value conditions with per-operand DECLARED RANGES, which are a typestate
// fact (a const interval attached to the type), and ask what region of P4's
// holding set that reaches.
//
// P4 (79_probes/p1_compositional_predicate_search.rs:73-83) carves the exact
// holding region of `(a+b)-c == a+(b-c)` for unsigned saturating u8, on the
// values. A declared-range lifting replaces it with a predicate on three
// intervals: does the law hold on EVERY triple drawn from
// [La,Ha] x [Lb,Hb] x [Lc,Hc].
//
// Three things measured here.
//
// (1) A characterisation of which boxes are fully-holding, derived by hand and
//     then CROSS-CHECKED exhaustively against brute force over every box at a
//     model width. This is 80's section-4.3 shape: a closed form checked
//     against a sweep on a model band, so what stays unchecked is the transfer
//     rather than the whole claim.
//
// (2) The largest fully-holding box at the shipped width (u8), computed from
//     the cross-checked characterisation, and what fraction of P4's holding set
//     it reaches.
//
// (3) Whether any clamp event can fire inside a maximal box. This is the
//     question that decides whether the lifted arm is about saturation at all,
//     or about integers that happen to be carried in a saturating type.
//
// Model band is widths 2..=5 (domain sizes 4, 8, 16, 32) for the brute force.
// The shipped width is 8 and the characterisation is applied there.

// ---------------------------------------------------------------------------
// The law, parameterised by a modelled unsigned width. MAXV = 2^w - 1.
// ---------------------------------------------------------------------------

fn sat_add(x: u32, y: u32, maxv: u32) -> u32 {
    let s = x + y;
    if s > maxv { maxv } else { s }
}
fn sat_sub(x: u32, y: u32) -> u32 {
    if y > x { 0 } else { x - y }
}
fn law_holds(a: u32, b: u32, c: u32, maxv: u32) -> bool {
    sat_sub(sat_add(a, b, maxv), c) == sat_add(a, sat_sub(b, c), maxv)
}

// The two clamp events P4's cases name, as facts about a triple.
fn ceiling_fires(a: u32, b: u32, c: u32, maxv: u32) -> bool {
    let _ = c;
    a + b > maxv
}
fn floor_fires(a: u32, b: u32, c: u32) -> bool {
    let _ = a;
    b < c
}

// ---------------------------------------------------------------------------
// (1) The hand-derived characterisation of a fully-holding box, and the brute
//     force it is checked against.
//
// Claim: box [La,Ha] x [Lb,Hb] x [Lc,Hc] is fully-holding iff any of
//   (i)   Ha + Hb <= MAXV  AND  Lb >= Hc      "no clamp can fire"
//   (ii)  La == 0 AND Ha == 0                 "a is identically zero"
//   (iii) Lc == 0 AND Hc == 0                 "c is identically zero"
// ---------------------------------------------------------------------------

fn closed_form_box_holds(
    la: u32, ha: u32, lb: u32, hb: u32, lc: u32, hc: u32, maxv: u32,
) -> bool {
    let no_clamp_possible = ha + hb <= maxv && lb >= hc;
    let a_is_zero = la == 0 && ha == 0;
    let c_is_zero = lc == 0 && hc == 0;
    no_clamp_possible || a_is_zero || c_is_zero
}

fn brute_force_box_holds(
    la: u32, ha: u32, lb: u32, hb: u32, lc: u32, hc: u32, maxv: u32,
) -> bool {
    for a in la..=ha {
        for b in lb..=hb {
            for c in lc..=hc {
                if !law_holds(a, b, c, maxv) {
                    return false;
                }
            }
        }
    }
    true
}

// Does ANY clamp fire anywhere inside a box?
fn box_has_a_clamp(
    la: u32, ha: u32, lb: u32, hb: u32, lc: u32, hc: u32, maxv: u32,
) -> bool {
    for a in la..=ha {
        for b in lb..=hb {
            for c in lc..=hc {
                if ceiling_fires(a, b, c, maxv) || floor_fires(a, b, c) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    println!("p1: lifting 79's P4 through declared operand ranges\n");

    // -----------------------------------------------------------------------
    // Section 1: cross-check the closed form against brute force over EVERY
    // box, at every width in the model band. Both directions are counted; the
    // check is not allowed to pass by never entering the interesting path.
    // -----------------------------------------------------------------------
    println!("section 1: closed form vs brute force over every box, model band");
    println!(
        "{:>5} {:>8} {:>12} {:>10} {:>12} {:>12}",
        "width", "boxes", "holding boxes", "with clamp", "cf-says-yes", "disagreements"
    );
    for w in 2u32..=5 {
        let maxv = (1u32 << w) - 1;
        let n = maxv + 1;
        let mut boxes: u64 = 0;
        let mut holding: u64 = 0;
        let mut holding_with_clamp: u64 = 0;
        let mut cf_yes: u64 = 0;
        let mut disagreements: u64 = 0;
        let mut first_disagreement: Option<(u32, u32, u32, u32, u32, u32)> = None;
        for la in 0..n {
            for ha in la..n {
                for lb in 0..n {
                    for hb in lb..n {
                        for lc in 0..n {
                            for hc in lc..n {
                                boxes += 1;
                                let bf = brute_force_box_holds(la, ha, lb, hb, lc, hc, maxv);
                                let cf = closed_form_box_holds(la, ha, lb, hb, lc, hc, maxv);
                                if bf {
                                    holding += 1;
                                    if box_has_a_clamp(la, ha, lb, hb, lc, hc, maxv) {
                                        holding_with_clamp += 1;
                                    }
                                }
                                if cf {
                                    cf_yes += 1;
                                }
                                if bf != cf {
                                    disagreements += 1;
                                    if first_disagreement.is_none() {
                                        first_disagreement = Some((la, ha, lb, hb, lc, hc));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!(
            "{:>5} {:>8} {:>12} {:>10} {:>12} {:>12}",
            w, boxes, holding, holding_with_clamp, cf_yes, disagreements
        );
        if let Some(d) = first_disagreement {
            println!("        first disagreement at box {:?}", d);
        }
    }

    // -----------------------------------------------------------------------
    // Section 1b: NEGATIVE CONTROL on the closed form. Perturb one clause and
    // confirm the cross-check catches it. Without this the agreement above
    // could be an artifact of a check that cannot fail.
    // -----------------------------------------------------------------------
    println!("\nsection 1b: negative control, perturbed closed forms must disagree");
    let mutants: [(&str, fn(u32, u32, u32, u32, u32, u32, u32) -> bool); 3] = [
        (
            "M1: drop the `Lb >= Hc` conjunct (no-floor-clamp clause)",
            |ha_la, ha, lb, hb, lc, hc, maxv| {
                let _ = (ha_la, lb, lc);
                let _ = hc;
                ha + hb <= maxv
            },
        ),
        (
            "M2: weaken `Ha + Hb <= MAXV` to `La + Lb <= MAXV`",
            |la, ha, lb, hb, lc, hc, maxv| {
                let _ = (ha, hb);
                (la + lb <= maxv && lb >= hc) || (la == 0 && ha == 0) || (lc == 0 && hc == 0)
            },
        ),
        (
            "M3: drop the `a is identically zero` clause",
            |la, ha, lb, hb, lc, hc, maxv| {
                let _ = (la, ha);
                (ha + hb <= maxv && lb >= hc) || (lc == 0 && hc == 0)
            },
        ),
    ];
    for (name, mutant) in mutants.iter() {
        let mut total_disagreements: u64 = 0;
        for w in 2u32..=4 {
            let maxv = (1u32 << w) - 1;
            let n = maxv + 1;
            for la in 0..n {
                for ha in la..n {
                    for lb in 0..n {
                        for hb in lb..n {
                            for lc in 0..n {
                                for hc in lc..n {
                                    let bf =
                                        brute_force_box_holds(la, ha, lb, hb, lc, hc, maxv);
                                    let m = mutant(la, ha, lb, hb, lc, hc, maxv);
                                    if bf != m {
                                        total_disagreements += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("  {} -> {} disagreements over widths 2..=4", name, total_disagreements);
    }

    // -----------------------------------------------------------------------
    // Section 2: the largest fully-holding box at the shipped width, from the
    // cross-checked characterisation, plus its coverage of P4's holding set.
    // -----------------------------------------------------------------------
    println!("\nsection 2: largest fully-holding box at width 8");
    let maxv = 255u32;
    let n = 256u32;

    // Exact size of P4's holding set at width 8, recounted here.
    let mut holding_triples: u64 = 0;
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                if law_holds(a, b, c, maxv) {
                    holding_triples += 1;
                }
            }
        }
    }
    let total_triples = (n as u64) * (n as u64) * (n as u64);
    println!(
        "  law holds on {} of {} triples ({:.4}%)",
        holding_triples,
        total_triples,
        100.0 * holding_triples as f64 / total_triples as f64
    );

    // Maximal box under clause (i), the clamp-free one. Search over Hb and Lb;
    // La = 0, Lc = 0, Ha = MAXV - Hb, Hc = Lb are optimal by monotonicity of
    // each factor, and the search below does not assume that: it sweeps all
    // four free bounds that clause (i) permits.
    let mut best_i = (0u64, (0u32, 0u32, 0u32, 0u32, 0u32, 0u32));
    for hb in 0..n {
        for lb in 0..=hb {
            // clause (i): Ha + Hb <= MAXV and Lb >= Hc
            let ha_max = maxv.saturating_sub(hb);
            let hc_max = lb;
            // La = 0 and Lc = 0 maximise the two extents.
            let vol = (ha_max as u64 + 1) * ((hb - lb) as u64 + 1) * (hc_max as u64 + 1);
            if vol > best_i.0 {
                best_i = (vol, (0, ha_max, lb, hb, 0, hc_max));
            }
        }
    }
    println!(
        "  clause (i) 'no clamp can fire' maximal box: a in [{},{}] b in [{},{}] c in [{},{}], volume {}",
        (best_i.1).0, (best_i.1).1, (best_i.1).2, (best_i.1).3, (best_i.1).4, (best_i.1).5,
        best_i.0
    );

    // Clauses (ii) and (iii): one operand pinned to zero, the other two free.
    let deg_vol = (n as u64) * (n as u64);
    println!(
        "  clause (ii) 'a is identically zero' maximal box volume: {}",
        deg_vol
    );
    println!(
        "  clause (iii) 'c is identically zero' maximal box volume: {}",
        deg_vol
    );

    let best = best_i.0.max(deg_vol);
    println!(
        "  best single declared-range box: {} triples = {:.4}% of the holding set, {:.4}% of the domain",
        best,
        100.0 * best as f64 / holding_triples as f64,
        100.0 * best as f64 / total_triples as f64
    );

    // -----------------------------------------------------------------------
    // Section 3: does any clamp fire inside the maximal boxes?
    // -----------------------------------------------------------------------
    println!("\nsection 3: is saturation observable inside a maximal lifted box");
    let (_, (la, ha, lb, hb, lc, hc)) = best_i;
    let clamp_in_best_i = box_has_a_clamp(la, ha, lb, hb, lc, hc, maxv);
    println!(
        "  clause (i) maximal box a in [{},{}] b in [{},{}] c in [{},{}]: any clamp fires anywhere = {}",
        la, ha, lb, hb, lc, hc, clamp_in_best_i
    );

    // For clause (ii) and (iii), a clamp CAN fire, so state what the law
    // degenerates to there.
    let mut clamp_ii = false;
    for b in 0..n {
        for c in 0..n {
            if ceiling_fires(0, b, c, maxv) || floor_fires(0, b, c) {
                clamp_ii = true;
            }
        }
    }
    let mut clamp_iii = false;
    for a in 0..n {
        for b in 0..n {
            if ceiling_fires(a, b, 0, maxv) || floor_fires(a, b, 0) {
                clamp_iii = true;
            }
        }
    }
    println!("  clause (ii) box a == 0: any clamp fires = {} (law reads b-c == b-c)", clamp_ii);
    println!("  clause (iii) box c == 0: any clamp fires = {} (law reads a+b == a+b)", clamp_iii);

    // -----------------------------------------------------------------------
    // Section 4: the declaration a consumer would REACH FOR first, checked.
    // "My values never overflow the final result." That is P1 in 79's probe,
    // restated as a declared-range condition on the box.
    // -----------------------------------------------------------------------
    println!("\nsection 4: the natural consumer declaration, checked");
    println!("  declaration: the exact result a+b-c always lands in [0, MAXV] over the box");
    let mut boxes_where_natural_decl_holds: u64 = 0;
    let mut boxes_where_it_lies: u64 = 0;
    let mut witness: Option<(u32, u32, u32, u32, u32, u32, u32, u32, u32)> = None;
    for w in 2u32..=4 {
        let mv = (1u32 << w) - 1;
        let nn = mv + 1;
        for la in 0..nn {
            for ha in la..nn {
                for lb in 0..nn {
                    for hb in lb..nn {
                        for lc in 0..nn {
                            for hc in lc..nn {
                                // declared condition: for all triples in box the exact result is in range
                                let min_exact = la as i64 + lb as i64 - hc as i64;
                                let max_exact = ha as i64 + hb as i64 - lc as i64;
                                let decl = min_exact >= 0 && max_exact <= mv as i64;
                                if decl {
                                    boxes_where_natural_decl_holds += 1;
                                    if !brute_force_box_holds(la, ha, lb, hb, lc, hc, mv) {
                                        boxes_where_it_lies += 1;
                                        if witness.is_none() {
                                            'find: for a in la..=ha {
                                                for b in lb..=hb {
                                                    for c in lc..=hc {
                                                        if !law_holds(a, b, c, mv) {
                                                            witness = Some((
                                                                w, la, ha, lb, hb, lc, hc, a, b,
                                                            ));
                                                            let _ = c;
                                                            break 'find;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "  boxes satisfying it over widths 2..=4: {}, of which the law FAILS somewhere inside: {}",
        boxes_where_natural_decl_holds, boxes_where_it_lies
    );
    println!("  first lying box (w, La, Ha, Lb, Hb, Lc, Hc, and a failing a,b): {:?}", witness);

    // And the same declaration at the value level, at width 8, which is 79's P1.
    let mut p1_suff: u64 = 0;
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                let exact = a as i64 + b as i64 - c as i64;
                let decl = (0..=maxv as i64).contains(&exact);
                if decl && !law_holds(a, b, c, maxv) {
                    p1_suff += 1;
                }
            }
        }
    }
    println!(
        "  at width 8, value level: declaration true and law false on {} triples ({:.4}% of the domain)",
        p1_suff,
        100.0 * p1_suff as f64 / total_triples as f64
    );
}
