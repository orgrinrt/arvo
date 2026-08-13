// PROBE p6. Written AFTER op's steer at `83`, and it exists because of it.
//
// Everything before this probe reasoned inside 80's distinction, typestate
// predicate against trajectory predicate, and asked whether a condition over
// values could be lifted into a fact about a TYPE. Op rejected that axis:
//
//   > the above collapses to whatever is available at const time: Making the
//   > predicates const expressions for example, allows using const functions
//   > and pipe in some data that is outside the typestate. However, being const
//   > time expressions, typestate is usable there too
//
// So the licensed category is const-available, and the typestate is one source
// of const-available data rather than the only one. That opens a route p1 never
// considered, because p1 only asked what a DECLARED RANGE reaches: at a given
// call site, some of the OPERANDS themselves may be const, and then the
// condition over them is a const expression with nothing lifted at all.
//
// This probe enumerates that exhaustively. For every subset S of the three
// operands of 79's P4, and every assignment of const values to the operands in
// S, it asks whether the law's truth is CONSTANT over the operands not in S.
// Three outcomes per configuration:
//
//   licensed  - constant TRUE, so an arm may be selected at const time
//   refused   - constant FALSE, so the arm is provably wrong here and the
//               const expression can say so, which is the admissibility cell
//               nothing in this panel has instrumented
//   undecided - both truths occur over the free operands, so no const
//               expression over S alone decides it
//
// The volume each outcome covers is reported, so "which subsets help" is
// answered with a number rather than an argument.
//
// Section 3 asks the same question of the other measured region this unit
// carries, the signed fold's sign-uniformity condition, to check whether the
// answer is a fact about P4 or about the shape of clamp conditions generally.

fn sat_add_u8(x: u8, y: u8) -> u8 {
    x.saturating_add(y)
}
fn sat_sub_u8(x: u8, y: u8) -> u8 {
    x.saturating_sub(y)
}
fn law(a: u8, b: u8, c: u8) -> bool {
    sat_sub_u8(sat_add_u8(a, b), c) == sat_add_u8(a, sat_sub_u8(b, c))
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Verdict {
    Licensed,
    Refused,
    Undecided,
}

/// Is the law constant over the operands NOT in the mask, given the values of
/// the operands in the mask? `mask` bit 0 is `a`, bit 1 is `b`, bit 2 is `c`.
fn decide(mask: u8, av: u8, bv: u8, cv: u8) -> (Verdict, u64) {
    let a_const = mask & 1 != 0;
    let b_const = mask & 2 != 0;
    let c_const = mask & 4 != 0;
    let mut seen_true = false;
    let mut seen_false = false;
    let mut volume: u64 = 0;
    let a_range: (u16, u16) = if a_const { (av as u16, av as u16) } else { (0, 255) };
    let b_range: (u16, u16) = if b_const { (bv as u16, bv as u16) } else { (0, 255) };
    let c_range: (u16, u16) = if c_const { (cv as u16, cv as u16) } else { (0, 255) };
    for a in a_range.0..=a_range.1 {
        for b in b_range.0..=b_range.1 {
            for c in c_range.0..=c_range.1 {
                volume += 1;
                if law(a as u8, b as u8, c as u8) {
                    seen_true = true;
                } else {
                    seen_false = true;
                }
                if seen_true && seen_false {
                    return (Verdict::Undecided, 0);
                }
            }
        }
    }
    if seen_true {
        (Verdict::Licensed, volume)
    } else {
        (Verdict::Refused, volume)
    }
}

fn name(mask: u8) -> String {
    let mut s = String::new();
    for (bit, ch) in [(1u8, 'a'), (2u8, 'b'), (4u8, 'c')] {
        if mask & bit != 0 {
            s.push(ch);
        }
    }
    if s.is_empty() {
        "{} (nothing const)".to_string()
    } else {
        format!("{{{}}}", s)
    }
}

fn main() {
    println!("p6: which const-available operand subsets decide 79's P4\n");
    println!("law holds on 2,894,336 of 16,777,216 triples (17.2516%), from p1.\n");
    println!(
        "{:<20} {:>10} {:>10} {:>10} {:>12} {:>16} {:>12}",
        "const operands",
        "configs",
        "licensed",
        "refused",
        "undecided",
        "licensed volume",
        "% of holding"
    );

    let holding: u64 = 2_894_336;
    let mut best_partial: (u64, u8) = (0, 0);

    for mask in 0u8..8 {
        let a_n: u32 = if mask & 1 != 0 { 256 } else { 1 };
        let b_n: u32 = if mask & 2 != 0 { 256 } else { 1 };
        let c_n: u32 = if mask & 4 != 0 { 256 } else { 1 };
        let mut licensed = 0u64;
        let mut refused = 0u64;
        let mut undecided = 0u64;
        let mut licensed_volume = 0u64;
        for ai in 0..a_n {
            for bi in 0..b_n {
                for ci in 0..c_n {
                    let (v, vol) = decide(mask, ai as u8, bi as u8, ci as u8);
                    match v {
                        Verdict::Licensed => {
                            licensed += 1;
                            licensed_volume += vol;
                        }
                        Verdict::Refused => refused += 1,
                        Verdict::Undecided => undecided += 1,
                    }
                }
            }
        }
        let configs = (a_n as u64) * (b_n as u64) * (c_n as u64);
        println!(
            "{:<20} {:>10} {:>10} {:>10} {:>12} {:>16} {:>11.4}%",
            name(mask),
            configs,
            licensed,
            refused,
            undecided,
            licensed_volume,
            100.0 * licensed_volume as f64 / holding as f64
        );
        if mask.count_ones() < 3 && licensed_volume > best_partial.0 {
            best_partial = (licensed_volume, mask);
        }
    }

    println!("\nsection 2: what the licensed configurations actually are, for the");
    println!("single-operand subsets, since those are the ones a call site is most");
    println!("likely to have.\n");
    for mask in [1u8, 2u8, 4u8] {
        let mut lic: Vec<u8> = Vec::new();
        let mut ref_: Vec<u8> = Vec::new();
        for v in 0u16..=255 {
            let (verdict, _) = decide(mask, v as u8, v as u8, v as u8);
            match verdict {
                Verdict::Licensed => lic.push(v as u8),
                Verdict::Refused => ref_.push(v as u8),
                Verdict::Undecided => {}
            }
        }
        println!(
            "  const {}: licensed at values {:?}, refused at values {:?}, undecided at the other {}",
            name(mask),
            lic,
            ref_,
            256 - lic.len() - ref_.len()
        );
    }

    println!("\nsection 3: the same lattice for the signed fold's condition, to see");
    println!("whether section 2's answer is about P4 or about clamp conditions in");
    println!("general. Signed saturating addition at width 4, arity 3, asking which");
    println!("const-available operand subsets decide associativity.\n");

    let minv: i32 = -8;
    let maxv: i32 = 7;
    let sadd = |x: i32, y: i32| -> i32 {
        let s = x + y;
        if s > maxv {
            maxv
        } else if s < minv {
            minv
        } else {
            s
        }
    };
    let assoc = |a: i32, b: i32, c: i32| sadd(sadd(a, b), c) == sadd(a, sadd(b, c));

    println!(
        "{:<20} {:>10} {:>10} {:>10} {:>12}",
        "const operands", "configs", "licensed", "refused", "undecided"
    );
    for mask in 0u8..8 {
        let mut licensed = 0u64;
        let mut refused = 0u64;
        let mut undecided = 0u64;
        let vals: Vec<i32> = (minv..=maxv).collect();
        let a_set: Vec<i32> = if mask & 1 != 0 { vals.clone() } else { vec![0] };
        let b_set: Vec<i32> = if mask & 2 != 0 { vals.clone() } else { vec![0] };
        let c_set: Vec<i32> = if mask & 4 != 0 { vals.clone() } else { vec![0] };
        for &av in &a_set {
            for &bv in &b_set {
                for &cv in &c_set {
                    let mut st = false;
                    let mut sf = false;
                    let ar: Vec<i32> = if mask & 1 != 0 { vec![av] } else { vals.clone() };
                    let br: Vec<i32> = if mask & 2 != 0 { vec![bv] } else { vals.clone() };
                    let cr: Vec<i32> = if mask & 4 != 0 { vec![cv] } else { vals.clone() };
                    'inner: for &a in &ar {
                        for &b in &br {
                            for &c in &cr {
                                if assoc(a, b, c) {
                                    st = true;
                                } else {
                                    sf = true;
                                }
                                if st && sf {
                                    break 'inner;
                                }
                            }
                        }
                    }
                    if st && sf {
                        undecided += 1;
                    } else if st {
                        licensed += 1;
                    } else {
                        refused += 1;
                    }
                }
            }
        }
        println!(
            "{:<20} {:>10} {:>10} {:>10} {:>12}",
            name(mask),
            licensed + refused + undecided,
            licensed,
            refused,
            undecided
        );
    }

    println!("\nsection 4: the control. If `decide` could not return Undecided the");
    println!("table above would be meaningless, and if it could not return Refused the");
    println!("admissibility half would be untested. Both appear above with non-zero");
    println!("counts, and here is a witness of each for P4:\n");
    for (m, av, bv, cv) in [(1u8, 0u8, 0u8, 0u8), (1u8, 1u8, 0u8, 0u8), (4u8, 0u8, 0u8, 200u8)] {
        let (v, vol) = decide(m, av, bv, cv);
        println!(
            "  const {} with a={} b={} c={} -> {:?} (volume {})",
            name(m),
            av,
            bv,
            cv,
            v,
            vol
        );
    }
}
