// 160 P1. The adequacy certificate has two branches, and a witness-only certificate
// cannot tell a refinement from a spurious axis.
//
// CLAIM UNDER TEST
//   157 S-14 states completeness as: "every pair of distinct parameter assignments is
//   separated by some input". 112:934-937 classifies an axis by how many DIRECTIONS
//   admit a total denotation-preserving map: two = spurious (must not be a parameter),
//   one = refinement (may be, with the map as its weakening), zero = declared semantics
//   (must be). These two statements are not equivalent as written: a refinement pair
//   has NO separating input (111 r1: 1753 arm changes, 0 answer changes) and is still
//   a legitimate parameter pair. So a certificate that accepts only separation
//   witnesses rejects the refinement parameters the realisation-map topic requires.
//
//   The repair this probe compiles: the per-pair obligation is a DISJUNCTION,
//     separation-witness  XOR  one-directional-weakening,
//   and the refusal case (neither, i.e. maps in BOTH directions and no witness) is a
//   compile failure. Direction count and witness are both computed at const time.
//
// THE CASE THAT MUST FAIL, declared before the run
//   Building with --cfg carry_spurious carries the (trunc, ties-even) rounding pair at
//   F = 0 under a grid-closed integer signature as a parameter pair. Its direction
//   count is 2 and it has no witness, so `pair_is_admissible` must be FALSE and the
//   const assertion must REFUSE TO COMPILE (E0080). If that build succeeds, the
//   two-branch scheme accepts a spurious axis and this probe proves nothing.
//
// SECOND CONTROL, also declared before the run
//   `witness_only_accepts` must return FALSE for BOTH the refinement pair and the
//   spurious pair, demonstrating that the witness-only scheme (S-14 as written)
//   cannot distinguish them. If it returns true for either, the hole this probe
//   exists to demonstrate does not exist and the finding is withdrawn.
//
// Model: unsigned fixed-point, F = 0, width W, policy in {WRAP, SAT}, rounding in
// {TRUNC, TIES_EVEN} (inert at F = 0 over integer arguments), refinement = declared
// upper bound on operand values. Direction count enumerated exhaustively at the model
// width; the sat/wrap witness is closed-form and checked at every width 1..=64.

#![allow(dead_code)]

const WRAP: u8 = 0;
const SAT: u8 = 1;

const fn max_of(w: u32) -> u64 {
    if w >= 64 { u64::MAX } else { (1u64 << w) - 1 }
}

// One realisation map, two regions (110 section 2). Rounding region is the identity
// at F = 0 over integer exact results, for both modes, so it is a parameter R never
// reads on this signature: the spurious axis by construction.
const fn realise(exact: u128, w: u32, policy: u8) -> u64 {
    let hi = max_of(w) as u128;
    match policy {
        SAT => if exact > hi { hi as u64 } else { exact as u64 },
        _ => {
            let span = hi + 1;
            (exact % span) as u64
        }
    }
}

// ---------- branch one: separation, closed-form, all widths ----------

const fn separates_policy(w: u32) -> bool {
    let exact = max_of(w) as u128 + 1;
    realise(exact, w, WRAP) != realise(exact, w, SAT)
}

const fn policy_separates_every_width() -> bool {
    let mut w = 1u32;
    while w <= 64 {
        if !separates_policy(w) { return false; }
        w += 1;
    }
    true
}

// ---------- direction count, exhaustive at the model width ----------
//
// A total denotation-preserving map A -> B over extent [0, bound_a] exists when every
// value in A's extent lies in B's extent (totality: extent inclusion), the map is the
// identity on denotations, and it commutes with add-then-realise under the two
// assignments (the algebras agree on A's extent). Denotation preservation forces the
// identity, so existence collapses to: extent_a <= extent_b AND the two realisation
// maps agree on every add reachable from A's extent in one step.
// One step suffices for THIS model's separation structure at the model width; the
// probe checks the claim it makes and no more, and says so.

const MODEL_W: u32 = 6;

const fn maps_a_to_b(
    bound_a: u64, policy_a: u8, bound_b: u64, policy_b: u8,
) -> bool {
    if bound_a > bound_b { return false; } // not total
    let mut x = 0u64;
    while x <= bound_a {
        let mut y = 0u64;
        while y <= bound_a {
            let ea = (x as u128) + (y as u128);
            if realise(ea, MODEL_W, policy_a) != realise(ea, MODEL_W, policy_b) {
                return false; // does not commute
            }
            y += 1;
        }
        x += 1;
    }
    true
}

const fn direction_count(
    bound_a: u64, policy_a: u8, bound_b: u64, policy_b: u8,
) -> u32 {
    let mut n = 0;
    if maps_a_to_b(bound_a, policy_a, bound_b, policy_b) { n += 1; }
    if maps_a_to_b(bound_b, policy_b, bound_a, policy_a) { n += 1; }
    n
}

// A separating witness inside the joint extent, exhaustive at the model width.
const fn has_witness(
    bound_a: u64, policy_a: u8, bound_b: u64, policy_b: u8,
) -> bool {
    let joint = if bound_a < bound_b { bound_a } else { bound_b };
    let mut x = 0u64;
    while x <= joint {
        let mut y = 0u64;
        while y <= joint {
            let ea = (x as u128) + (y as u128);
            if realise(ea, MODEL_W, policy_a) != realise(ea, MODEL_W, policy_b) {
                return true;
            }
            y += 1;
        }
        x += 1;
    }
    false
}

// ---------- the two schemes ----------

// 157 S-14 as written: separation or nothing.
const fn witness_only_accepts(ba: u64, pa: u8, bb: u64, pb: u8) -> bool {
    has_witness(ba, pa, bb, pb)
}

// The repaired scheme: separation, or a weakening in exactly one direction.
const fn pair_is_admissible(ba: u64, pa: u8, bb: u64, pb: u8) -> bool {
    if has_witness(ba, pa, bb, pb) { return true; }   // declared semantics
    direction_count(ba, pa, bb, pb) == 1              // refinement (weakening exists,
                                                      // tightening does not)
}

const M: u64 = 63; // max at MODEL_W = 6

// Pair 1: declared semantics. Same extent, sat against wrap. Witness must exist.
const P1_WITNESS: bool = has_witness(M, SAT, M, WRAP);
const P1_DIRS: u32 = direction_count(M, SAT, M, WRAP);

// Pair 2: refinement. Same policy, bounds 10 against 40. No witness, one direction.
const P2_WITNESS: bool = has_witness(10, SAT, 40, SAT);
const P2_DIRS: u32 = direction_count(10, SAT, 40, SAT);

// Pair 3: spurious. Same extent, same policy, rounding differs (inert at F = 0 over
// this signature): modelled as two identical assignments, which is exactly what an
// unread parameter is to the realisation map. No witness, two directions.
const P3_WITNESS: bool = has_witness(M, SAT, M, SAT);
const P3_DIRS: u32 = direction_count(M, SAT, M, SAT);

// The demonstrated hole: witness-only cannot separate pair 2 from pair 3.
const HOLE: bool = witness_only_accepts(10, SAT, 40, SAT) == witness_only_accepts(M, SAT, M, SAT);

// The repaired scheme separates all three.
const _: () = assert!(pair_is_admissible(M, SAT, M, WRAP));   // separation branch
const _: () = assert!(pair_is_admissible(10, SAT, 40, SAT));  // weakening branch
const _: () = assert!(policy_separates_every_width());        // closed form, W 1..=64

#[cfg(carry_spurious)]
const _: () = assert!(pair_is_admissible(M, SAT, M, SAT));    // MUST FAIL TO COMPILE

fn main() {
    println!("P1 declared-semantics pair : witness={} directions={}", P1_WITNESS, P1_DIRS);
    println!("P2 refinement pair         : witness={} directions={}", P2_WITNESS, P2_DIRS);
    println!("P3 spurious pair           : witness={} directions={}", P3_WITNESS, P3_DIRS);
    println!("witness-only conflates P2 and P3 : {}", HOLE);
    println!("policy_separates_every_width()   : {}", policy_separates_every_width());
}
