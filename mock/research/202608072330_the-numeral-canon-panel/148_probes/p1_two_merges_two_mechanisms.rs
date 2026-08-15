//! p1 (148): is the one-sided-clamp congruence the mechanism of `140`'s refuted P3?
//!
//! `146` section 1.1 scopes the shared-workspace-rule contamination to three
//! places, one of which is "`140`'s own refuted P3". If the congruence is the
//! mechanism there, then `139` and `140` agreeing about that cell is one
//! instance rather than two, and the ledger is right to say so.
//!
//! Two things merge at unsigned F=0 and they are not the same merge:
//!
//!   * `139:173-176` reports "the two saturating INTERMEDIATE values merged",
//!     mechanised by the congruence: "reducing early and reducing late land in
//!     the same place". That is a relocation question, and it is exactly the
//!     workspace rule's sentence.
//!   * `140`'s P3 predicted the class count at unsigned F=0 addition would equal
//!     the OVERFLOW axis cardinality (3) and measured 2. The two saturating
//!     overflow POSITIONS merged. `140` gave the mechanism as the low clamp
//!     being unreachable when two non-negative values are added.
//!
//! Those are different axes. This probe measures whether they are also different
//! mechanisms, which decides whether the scoping is right about P3.
//!
//! Method. At unsigned F=0 addition, over the exhaustive domain:
//!   A. count how often the branch distinguishing SaturateBoth from
//!      SaturateHighOnly is actually taken, i.e. how often the running value is
//!      negative. If it is never taken, the overflow merge is reachability and
//!      the congruence has nothing to do with it.
//!   B. check whether the congruence proposition even applies. It is a statement
//!      about relocating a reduction across an operation, so it needs two
//!      reduction sites. A single addition has one. If the two intermediate
//!      positions are extensionally equal on a single operation for ALL overflow
//!      positions including wrapping, the intermediate merge there is vacuity
//!      rather than congruence.
//!   C. exhibit the cell where the congruence IS the mechanism and the
//!      reachability story is not available: a two-reduction-site operation
//!      where the low branch IS reachable, so the two accounts come apart.
//!
//! THE CASE THAT MUST FAIL. A probe that reports "the branch is never taken"
//! because it never looked, or because its counter is dead, proves nothing. So:
//!   (i) a POSITIVE control on the low branch: at unsigned SUBTRACTION the low
//!       branch must be taken often, and the two saturating overflow positions
//!       must then separate. If they do not, the counter is not counting.
//!   (ii) a POSITIVE control on the congruence: at SIGNED saturating the two
//!       intermediate positions must separate on the multiply-add, because a
//!       two-sided clamp is not a congruence. If they do not, part C is empty.
//! Both must fire or the verdict is void.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    SaturateBoth,
    SaturateHighOnly,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sign {
    Unsigned,
    Signed,
}

fn lo(w: u32, s: Sign) -> i128 {
    match s {
        Sign::Unsigned => 0,
        Sign::Signed => -(1i128 << (w - 1)),
    }
}
fn hi(w: u32, s: Sign) -> i128 {
    match s {
        Sign::Unsigned => (1i128 << w) - 1,
        Sign::Signed => (1i128 << (w - 1)) - 1,
    }
}

/// Reduce into range, and report whether the LOW branch was the one that fired.
/// The low branch is the only place SaturateBoth and SaturateHighOnly differ.
fn reduce_tracked(v: i128, w: u32, s: Sign, o: Overflow) -> (i128, bool) {
    let l = lo(w, s);
    let h = hi(w, s);
    let below = v < l;
    let out = match o {
        Overflow::SaturateBoth => {
            if v < l {
                l
            } else if v > h {
                h
            } else {
                v
            }
        }
        Overflow::SaturateHighOnly => {
            if v > h {
                h
            } else {
                let m: i128 = 1i128 << w;
                (v - l).rem_euclid(m) + l
            }
        }
        Overflow::Wrap => {
            let m: i128 = 1i128 << w;
            (v - l).rem_euclid(m) + l
        }
    };
    (out, below)
}

fn values(w: u32, s: Sign) -> Vec<i128> {
    (lo(w, s)..=hi(w, s)).collect()
}

/// Part A: at unsigned F=0 addition, how often is the low branch reachable, and
/// do the two saturating overflow positions agree?
fn part_a(w: u32) -> (u64, u64, bool) {
    let mut low_taken = 0u64;
    let mut total = 0u64;
    let mut agree = true;
    for &a in values(w, Sign::Unsigned).iter() {
        for &b in values(w, Sign::Unsigned).iter() {
            let sum = a + b;
            let (x, below) = reduce_tracked(sum, w, Sign::Unsigned, Overflow::SaturateBoth);
            let (y, _) = reduce_tracked(sum, w, Sign::Unsigned, Overflow::SaturateHighOnly);
            total += 1;
            if below {
                low_taken += 1;
            }
            if x != y {
                agree = false;
            }
        }
    }
    (low_taken, total, agree)
}

/// Control (i): the same question at SUBTRACTION, where the low branch is
/// reachable. The two positions MUST separate here.
fn control_i(w: u32) -> (u64, u64, bool) {
    let mut low_taken = 0u64;
    let mut total = 0u64;
    let mut agree = true;
    for &a in values(w, Sign::Unsigned).iter() {
        for &b in values(w, Sign::Unsigned).iter() {
            let d = a - b;
            let (x, below) = reduce_tracked(d, w, Sign::Unsigned, Overflow::SaturateBoth);
            let (y, _) = reduce_tracked(d, w, Sign::Unsigned, Overflow::SaturateHighOnly);
            total += 1;
            if below {
                low_taken += 1;
            }
            if x != y {
                agree = false;
            }
        }
    }
    (low_taken, total, agree)
}

/// Part B and C: the intermediate axis, which is what the congruence is about.
/// `early` reduces the product before adding; `late` reduces once at the end.
/// On a single addition there is no product, so the two coincide by construction.
fn intermediate_differs(w: u32, s: Sign, o: Overflow, two_sites: bool) -> u64 {
    let mut diffs = 0u64;
    let vs = values(w, s);
    for &a in vs.iter() {
        for &b in vs.iter() {
            for &c in vs.iter() {
                let (early, late) = if two_sites {
                    // multiply-add: two reduction sites, so relocation is a real
                    // question and the congruence is what decides it.
                    let p = a * b;
                    let (pr, _) = reduce_tracked(p, w, s, o);
                    let (e, _) = reduce_tracked(pr + c, w, s, o);
                    let (l, _) = reduce_tracked(p + c, w, s, o);
                    (e, l)
                } else {
                    // a single addition: one reduction site. there is nothing to
                    // relocate, so the two positions are the same expression.
                    let (e, _) = reduce_tracked(a + b, w, s, o);
                    let (l, _) = reduce_tracked(a + b, w, s, o);
                    (e, l)
                };
                if early != late {
                    diffs += 1;
                }
            }
        }
    }
    diffs
}

fn main() {
    let w = 5u32;
    let mut failures = 0usize;

    println!("p1 (148): are the two unsigned F=0 merges one mechanism or two?");
    println!("W={w}, F=0, exhaustive.\n");

    // ---- Part A ----
    let (low, total, agree) = part_a(w);
    println!("=== A. the OVERFLOW merge at unsigned addition (140's refuted P3) ===");
    println!("low branch taken: {low} of {total} operand pairs");
    println!(
        "SaturateBoth and SaturateHighOnly agree: {}",
        if agree { "yes, on every pair" } else { "no" }
    );
    println!(
        "  -> mechanism is {}",
        if low == 0 && agree {
            "REACHABILITY: the branch that distinguishes them is never entered"
        } else {
            "something else"
        }
    );

    // ---- Control (i) ----
    let (clow, ctotal, cagree) = control_i(w);
    println!("\n=== control (i): the same two positions at unsigned SUBTRACTION ===");
    println!("low branch taken: {clow} of {ctotal} operand pairs");
    println!("the two positions agree: {}", if cagree { "yes" } else { "no, they separate" });
    if clow == 0 || cagree {
        println!("  !! CONTROL FAIL: the low branch is supposed to be reachable here and");
        println!("  the two positions are supposed to separate. the counter is not counting.");
        failures += 1;
    } else {
        println!("  control fires: the counter counts and the positions can separate,");
        println!("  so A's zero is a real zero rather than a blind one.");
    }

    // ---- Part B ----
    println!("\n=== B. does the congruence proposition even apply at a single addition? ===");
    for o in [Overflow::Wrap, Overflow::SaturateBoth, Overflow::SaturateHighOnly] {
        let d = intermediate_differs(w, Sign::Unsigned, o, false);
        println!("  single addition, {o:?}: early vs late differ at {d} triples");
    }
    println!("  -> a single reduction site has nothing to relocate, so the two");
    println!("     intermediate positions coincide at EVERY overflow position,");
    println!("     including wrapping, where the congruence says nothing.");

    // ---- Part C ----
    println!("\n=== C. where the congruence IS the mechanism: two reduction sites ===");
    let mut cong_control_fired = false;
    for s in [Sign::Unsigned, Sign::Signed] {
        for o in [Overflow::SaturateBoth, Overflow::SaturateHighOnly] {
            let d = intermediate_differs(w, s, o, true);
            println!("  multiply-add, {s:?}, {o:?}: early vs late differ at {d} triples");
            if matches!(s, Sign::Signed) && matches!(o, Overflow::SaturateBoth) && d > 0 {
                cong_control_fired = true;
            }
        }
    }
    println!("\n=== control (ii): signed two-sided saturation must NOT be a congruence ===");
    if !cong_control_fired {
        println!("  !! CONTROL FAIL: signed SaturateBoth showed zero differences, so this");
        println!("  sweep cannot see a congruence failure and part C establishes nothing.");
        failures += 1;
    } else {
        println!("  control fires: the sweep can see a congruence failure when there is one.");
    }

    println!("\n=== verdict ===");
    println!("The two merges at unsigned F=0 are on DIFFERENT AXES with DIFFERENT");
    println!("MECHANISMS:");
    println!("  139's merge: the INTERMEDIATE axis, on an operation with two reduction");
    println!("    sites, mechanised by the one-sided-clamp congruence. That is the");
    println!("    workspace rule's sentence and 139 says so itself.");
    println!("  140's refuted P3: the OVERFLOW axis, on a single-site addition,");
    println!("    mechanised by the low branch being unreachable. The congruence is");
    println!("    not available as an explanation because there is nothing to relocate.");

    if failures > 0 {
        println!("\n{failures} control failures. the verdict above is void.");
        std::process::exit(1);
    }
}
