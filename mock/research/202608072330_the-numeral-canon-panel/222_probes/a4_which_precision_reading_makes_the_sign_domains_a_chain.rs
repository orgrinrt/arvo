// a4: does the precision coordinate count the sign digit?
//
// The question is `question::does_precision_count_the_sign_digit`. It is one of exactly two the
// register names as genuinely undetermined, and its `bound` field, written after
// `ruling::the_panel_finishes_the_canon_without_him` returned it to the panel, already carries the
// decision procedure:
//
//   "Nothing decides this mathematically and both readings were computed through to their four
//    consequences, so what is left is a convention, and a convention is chosen for what it makes
//    expressible. Answered inside the arms paradigm: take the reading under which the three sign
//    domains form a structure a const predicate can gate on rather than one leaving a domain
//    incomparable."
//
// So the answer is the output of a computation rather than a preference: build the three sign
// domains under each reading, order them by inclusion, and see which reading gives a chain.
//
// THE MODEL, and it is where the whole result lives, so it is stated first and then varied rather
// than fixed. The three sign domains come from `proposal.toml:153` and `proposal.toml:1220`, which
// both write them as "unsigned and signed two's complement and signed symmetric range". Two words
// there are underdetermined and the corpus does not fix either.
//
//   READING. Reading A counts the sign digit inside the precision, so a signed domain at precision
//   P has P - 1 magnitude digits. Reading B does not, so it has P.
//
//   SYMMETRIC. Two constructions answer to "signed symmetric range" and they are not the same set.
//     sign-magnitude:  { -(r^mag - 1) .. r^mag - 1 }, which is two's complement with the extra
//                      negative removed, the meaning the phrase carries in fixed-point practice.
//     balanced radix:  { -(r^mag - 1)/2 .. (r^mag - 1)/2 }, the centred digit set, which is only
//                      integral at an odd radix and is what "symmetric" means in balanced ternary.
//
// PREDICTIONS, stated before running:
//
// P1. Under reading B with the sign-magnitude symmetric, the three form a chain, unsigned inside
//     symmetric inside two's complement, at every precision and radix.
// P2. Under reading A they do not: unsigned reaches r^P - 1 while both signed domains stop at
//     r^(P-1) - 1, so unsigned is incomparable with both. Reading A is comparing at equal
//     footprint; reading B is comparing at equal magnitude precision. Both orderings are real and
//     only one is a chain.
// P3. Reading A manufactures a singleton at a legitimate precision: the symmetric domain at P = 1
//     denotes exactly { 0 }, at every radix. Reading B produces none at any P >= 1. That is the
//     same singleton `question::inclusion_order_singleton_amendment` exists to handle, so one
//     reading removes the case the other question is about.
// P4. WITHDRAWN BEFORE THE SECOND RUN, and kept here because the instrument refuted it. The first
//     version predicted that unsigned and symmetric collapse to equal cardinality at an odd radix
//     under the sign-magnitude model. They do not, and the run said so: at r = 3, P = 1 they are
//     3 against 5. The collapse the row's `unblocks` names is real and belongs to the BALANCED
//     model, where the centred digit set has exactly r^mag members, the same count as unsigned.
//     The first version's closing prose asserted the collapse in the same breath as printing the
//     numbers that refuted it, which is the defect this corpus documents about prose escaping the
//     checks that audit predicates. It is corrected rather than deleted.
// P5. Added with P4's correction: under the balanced model the chain fails under BOTH readings,
//     because a centred domain never contains the unsigned one. So the answer to the question is
//     conditional on the symmetric model, and that condition has to travel with it.
//
// THE CASES THAT MUST FAIL, printed failing rather than asserted:
//   C1 the chain detector must report NOT a chain for a planted incomparable triple;
//   C2 it must report a chain for a planted nested triple;
//   C3 the singleton detector must find a planted singleton and must not invent one;
//   C4 the inclusion test must report both directions of failure on a planted overlapping pair.
//
// Build: rustc --edition 2024 -O.

/// A domain as an inclusive integer interval. Every set here is an interval, and the probe checks
/// that shortcut is not doing any work by re-testing inclusion as explicit sets where affordable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Domain {
    lo: i128,
    hi: i128,
}

impl Domain {
    fn card(self) -> i128 {
        self.hi - self.lo + 1
    }
    fn contains(self, o: Domain) -> bool {
        self.lo <= o.lo && o.hi <= self.hi
    }
    fn show(self) -> String {
        format!("[{}, {}] |{}|", self.lo, self.hi, self.card())
    }
    fn to_set(self) -> Vec<i128> {
        (self.lo..=self.hi).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Symmetric {
    SignMagnitude,
    Balanced,
}

fn pow(r: i128, e: u32) -> i128 {
    let mut v = 1i128;
    for _ in 0..e {
        v *= r;
    }
    v
}

/// The three domains under a reading and a symmetric model.
///
/// `None` where the balanced model is asked for at an even radix, because `(r^mag - 1)/2` is not
/// an integer there and inventing a rounding would be the probe deciding the thing it measures.
fn domains(
    r: i128,
    p: u32,
    sign_digit_counted: bool,
    sym: Symmetric,
) -> Option<[(&'static str, Domain); 3]> {
    let mag = if sign_digit_counted {
        p.saturating_sub(1)
    } else {
        p
    };
    let m = pow(r, mag);
    let symmetric = match sym {
        Symmetric::SignMagnitude => Domain {
            lo: -(m - 1),
            hi: m - 1,
        },
        Symmetric::Balanced => {
            if (m - 1) % 2 != 0 {
                return None;
            }
            Domain {
                lo: -(m - 1) / 2,
                hi: (m - 1) / 2,
            }
        }
    };
    Some([
        (
            "unsigned",
            Domain {
                lo: 0,
                hi: pow(r, p) - 1,
            },
        ),
        ("twos_complement", Domain { lo: -m, hi: m - 1 }),
        ("symmetric", symmetric),
    ])
}

/// Is the triple totally ordered by inclusion? Returns the chain order when it is.
fn chain_order(ds: &[(&'static str, Domain); 3]) -> Option<[&'static str; 3]> {
    for a in 0..3 {
        for b in 0..3 {
            for c in 0..3 {
                if a == b || b == c || a == c {
                    continue;
                }
                if ds[c].1.contains(ds[b].1) && ds[b].1.contains(ds[a].1) {
                    return Some([ds[a].0, ds[b].0, ds[c].0]);
                }
            }
        }
    }
    None
}

fn incomparable_pairs(ds: &[(&'static str, Domain); 3]) -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    for i in 0..3 {
        for j in (i + 1)..3 {
            if !ds[i].1.contains(ds[j].1) && !ds[j].1.contains(ds[i].1) {
                out.push((ds[i].0, ds[j].0));
            }
        }
    }
    out
}

fn main() {
    println!("=== a4: which precision reading makes the three sign domains a chain ===");
    println!();

    println!("--- Controls, run before anything rests on the detectors ---");
    let incomparable_triple: [(&'static str, Domain); 3] = [
        ("a", Domain { lo: 0, hi: 1 }),
        ("b", Domain { lo: 1, hi: 2 }),
        ("c", Domain { lo: 0, hi: 2 }),
    ];
    println!(
        "  C1 planted incomparable triple ([0,1],[1,2],[0,2]): chain = {:?} (must be None), \
         incomparable pairs {:?} (must be non-empty)",
        chain_order(&incomparable_triple),
        incomparable_pairs(&incomparable_triple)
    );
    let nested_triple: [(&'static str, Domain); 3] = [
        ("inner", Domain { lo: 0, hi: 1 }),
        ("mid", Domain { lo: -1, hi: 2 }),
        ("outer", Domain { lo: -5, hi: 5 }),
    ];
    println!(
        "  C2 planted nested triple: chain = {:?} (must be Some)",
        chain_order(&nested_triple)
    );
    println!(
        "  C3 singleton detector: [0,0] card = {} (must be 1); [0,1] card = {} (must be 2)",
        Domain { lo: 0, hi: 0 }.card(),
        Domain { lo: 0, hi: 1 }.card()
    );
    let x = Domain { lo: 0, hi: 3 };
    let y = Domain { lo: 2, hi: 5 };
    println!(
        "  C4 inclusion both ways on [0,3] and [2,5]: {} and {} (both must be false)",
        x.contains(y),
        y.contains(x)
    );
    println!();

    // -------------------------------------------------------------------------
    // P1, P2, P5: the four combinations of reading and symmetric model.
    // -------------------------------------------------------------------------
    for (sym, sym_label) in [
        (Symmetric::SignMagnitude, "sign-magnitude symmetric"),
        (Symmetric::Balanced, "balanced-radix symmetric"),
    ] {
        for (counted, read_label) in [
            (true, "A: precision COUNTS the sign digit"),
            (false, "B: it does NOT"),
        ] {
            println!("--- {sym_label}, reading {read_label} ---");
            let mut chains = 0u32;
            let mut nonchains = 0u32;
            let mut singletons = 0u32;
            let mut set_disagreements = 0u32;
            let mut skipped = 0u32;
            for r in [2i128, 3, 4, 5, 10] {
                for p in 1u32..=6 {
                    let Some(ds) = domains(r, p, counted, sym) else {
                        skipped += 1;
                        continue;
                    };
                    if chain_order(&ds).is_some() {
                        chains += 1;
                    } else {
                        nonchains += 1;
                    }
                    for (_, d) in &ds {
                        if d.card() <= 1 {
                            singletons += 1;
                        }
                    }
                    if p <= 3 && r <= 4 {
                        for i in 0..3 {
                            for j in 0..3 {
                                let by_interval = ds[i].1.contains(ds[j].1);
                                let si = ds[i].1.to_set();
                                let by_set = ds[j].1.to_set().iter().all(|v| si.contains(v));
                                if by_interval != by_set {
                                    set_disagreements += 1;
                                }
                            }
                        }
                    }
                    if (r == 2 || r == 3) && p <= 3 {
                        println!(
                            "  r={r} P={p}  unsigned {:<16} twos {:<18} sym {:<16}  {}",
                            ds[0].1.show(),
                            ds[1].1.show(),
                            ds[2].1.show(),
                            match chain_order(&ds) {
                                Some(o) => format!("chain: {} < {} < {}", o[0], o[1], o[2]),
                                None => format!("NOT a chain: {:?}", incomparable_pairs(&ds)),
                            }
                        );
                    }
                }
            }
            println!(
                "  over r in {{2,3,4,5,10}} and P in 1..=6: chains {chains}, non-chains \
                 {nonchains}, cells skipped as non-integral {skipped}, domains of card <= 1 \
                 {singletons}"
            );
            println!("  interval-vs-set inclusion disagreements: {set_disagreements} (must be 0)");
            println!();
        }
    }

    // -------------------------------------------------------------------------
    // P3: the singleton, and its link to the inclusion-order question.
    // -------------------------------------------------------------------------
    println!("--- P3: where a singleton numeral comes from ---");
    for r in [2i128, 3, 10] {
        let a = domains(r, 1, true, Symmetric::SignMagnitude).unwrap();
        let b = domains(r, 1, false, Symmetric::SignMagnitude).unwrap();
        println!(
            "  r = {r:>2}, P = 1: reading A symmetric = {}, reading B symmetric = {}",
            a[2].1.show(),
            b[2].1.show()
        );
    }
    println!(
        "  Reading A makes the symmetric domain at precision one denote exactly the zero set,"
    );
    println!("  at every radix. That is a numeral carrying fewer than two values at a legitimate");
    println!("  precision, and it is the case `question::inclusion_order_singleton_amendment`");
    println!("  exists to handle: that row's note records the instrument which first got the");
    println!("  inclusion predicate wrong held exactly one such numeral. Reading B produces none.");
    println!();

    // -------------------------------------------------------------------------
    // P4 corrected: what actually collapses at an odd radix.
    // -------------------------------------------------------------------------
    println!("--- P4 corrected: what collapses at an odd radix, and under which model ---");
    for r in [3i128, 5, 7] {
        for p in 1u32..=3 {
            let sm = domains(r, p, false, Symmetric::SignMagnitude).unwrap();
            let ba = domains(r, p, false, Symmetric::Balanced).unwrap();
            println!(
                "  r={r} P={p}: unsigned {:<14} sign-magnitude sym {:<16} balanced sym {:<14}  \
                 card(unsigned)==card(balanced): {}",
                sm[0].1.show(),
                sm[2].1.show(),
                ba[2].1.show(),
                sm[0].1.card() == ba[2].1.card()
            );
        }
    }
    println!("  The collapse is a cardinality coincidence between unsigned and the BALANCED");
    println!("  symmetric domain, and it does not hold for the sign-magnitude one. The first");
    println!("  version of this probe predicted it for the sign-magnitude model and printed the");
    println!("  numbers refuting itself in the same block. Corrected here rather than deleted.");
    println!();
    println!("=== end a4 ===");
}
