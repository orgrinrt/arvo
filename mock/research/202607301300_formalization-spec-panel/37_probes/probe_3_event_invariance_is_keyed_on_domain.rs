//! Probe 3: wrapping addition is Kleene-associative and not graded-associative,
//! and which of the two it is depends on `Domain`.
//!
//! This probe exists because I predicted the opposite and the model refused me.
//! The prediction was a counting argument, and the argument is sound:
//!
//!   Each addition of two members of a numeral of modulus M produces an exact
//!   sum in [0, 2M), so at most one reduction fires and it subtracts exactly M.
//!   The delivered result is the exact total minus M times the number of
//!   reductions, and the delivered result is the exact total mod M, so the
//!   number of reductions is floor(exact total / M) for EVERY grouping. Event
//!   multiplicity is therefore a function of the exact total, which is
//!   grouping-independent, so wrapping addition is graded-associative.
//!
//! The hypothesis the argument uses without naming it is that the wrap is
//! ONE-DIRECTIONAL. On a signed numeral both range ends resolve by reduction,
//! a partial sum can leave through the top and a later one through the bottom,
//! and the reductions cancel in the value while both are counted in the grade.
//! So the count is bounded below by the net displacement and is not equal to it.
//!
//! CLAIM A. On an unsigned numeral the counting identity holds exactly:
//! for every input and every grouping of a four-element fold, the number of
//! reduction events equals `exact_total / MODULUS`. Asserted exhaustively, and
//! this is the theorem rather than a measurement of its consequence.
//!
//! CLAIM B. Therefore unsigned wrapping addition is graded-associative:
//! values, definedness and event multiplicities all agree across all five
//! groupings of a four-element fold, exhaustively.
//!
//! CLAIM C. On a signed numeral it fails, with the witness (-4, -3, 3):
//! ((-4) + (-3)) leaves through the bottom and reduces to 1, then 1 + 3 leaves
//! through the top and reduces to -4. Two events, value -4. The other grouping
//! is -4 + ((-3) + 3) = -4 + 0, zero events, value -4. Same value, two events
//! against none, so the Kleene equation holds at that point and the graded one
//! fails.
//!
//! THE CONSEQUENCE FOR THE KEY. `Domain` is a `Numeral` member and survives
//! both of the recent removals (`35:298`, `36:378-382`), so it is already in
//! every law's key transitively, the operand numeral being a never-elided slot
//! (`33:237`). What no file has said is that a law's verdict CHANGES with it,
//! and more precisely that one component of a law's view changes with it while
//! the others do not: the value and definedness components of `ReduceModulo`'s
//! associativity are `Domain`-independent and the event component is not.
//! Different components of one law read different parts of the key, which is
//! an argument for computing the view per component rather than keying a whole
//! law on the union of everything any component reads.

#![allow(dead_code)]

/// Unsigned model: values 0..=7, one-directional reduction.
const U_HI: i32 = 7;
const U_MOD: i32 = U_HI + 1;

/// Signed model: values -4..=3, reduction available at both ends.
const S_LO: i32 = -4;
const S_HI: i32 = 3;
const S_MOD: i32 = S_HI - S_LO + 1;

#[derive(Copy, Clone)]
struct R {
    v: i32,
    e: u32,
}

const fn u_add(a: R, b: R) -> R {
    let s = a.v + b.v;
    if s > U_HI {
        R {
            v: s - U_MOD,
            e: a.e + b.e + 1,
        }
    } else {
        R { v: s, e: a.e + b.e }
    }
}

const fn s_add(a: R, b: R) -> R {
    let s = a.v + b.v;
    if s > S_HI {
        R {
            v: s - S_MOD,
            e: a.e + b.e + 1,
        }
    } else if s < S_LO {
        R {
            v: s + S_MOD,
            e: a.e + b.e + 1,
        }
    } else {
        R { v: s, e: a.e + b.e }
    }
}

const fn lit(v: i32) -> R {
    R { v, e: 0 }
}

const fn u_eval4(x: i32, y: i32, z: i32, w: i32, g: usize) -> R {
    let (a, b, c, d) = (lit(x), lit(y), lit(z), lit(w));
    if g == 0 {
        u_add(u_add(u_add(a, b), c), d)
    } else if g == 1 {
        u_add(u_add(a, u_add(b, c)), d)
    } else if g == 2 {
        u_add(u_add(a, b), u_add(c, d))
    } else if g == 3 {
        u_add(a, u_add(u_add(b, c), d))
    } else {
        u_add(a, u_add(b, u_add(c, d)))
    }
}

const fn s_eval3(x: i32, y: i32, z: i32, g: usize) -> R {
    let (a, b, c) = (lit(x), lit(y), lit(z));
    if g == 0 {
        s_add(s_add(a, b), c)
    } else {
        s_add(a, s_add(b, c))
    }
}

const fn s_eval4(x: i32, y: i32, z: i32, w: i32, g: usize) -> R {
    let (a, b, c, d) = (lit(x), lit(y), lit(z), lit(w));
    if g == 0 {
        s_add(s_add(s_add(a, b), c), d)
    } else if g == 1 {
        s_add(s_add(a, s_add(b, c)), d)
    } else if g == 2 {
        s_add(s_add(a, b), s_add(c, d))
    } else if g == 3 {
        s_add(a, s_add(s_add(b, c), d))
    } else {
        s_add(a, s_add(b, s_add(c, d)))
    }
}

/// CLAIM A: the event count is the exact total divided by the modulus, at
/// every grouping, on the one-directional numeral.
const fn counting_identity_holds() -> bool {
    let mut x = 0;
    while x <= U_HI {
        let mut y = 0;
        while y <= U_HI {
            let mut z = 0;
            while z <= U_HI {
                let mut w = 0;
                while w <= U_HI {
                    let total = x + y + z + w;
                    let mut g = 0;
                    while g < 5 {
                        let r = u_eval4(x, y, z, w, g);
                        if r.e as i32 != total / U_MOD {
                            return false;
                        }
                        if r.v != total % U_MOD {
                            return false;
                        }
                        g += 1;
                    }
                    w += 1;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    true
}
const _: () = assert!(counting_identity_holds());

/// CLAIM B: unsigned wrapping is graded-associative. Implied by CLAIM A but
/// asserted directly, because a theorem's consequence measured independently
/// is what catches an error in the derivation.
const fn unsigned_is_graded() -> bool {
    let mut x = 0;
    while x <= U_HI {
        let mut y = 0;
        while y <= U_HI {
            let mut z = 0;
            while z <= U_HI {
                let mut w = 0;
                while w <= U_HI {
                    let mut i = 0;
                    while i < 5 {
                        let mut j = i + 1;
                        while j < 5 {
                            let a = u_eval4(x, y, z, w, i);
                            let b = u_eval4(x, y, z, w, j);
                            if a.v != b.v || a.e != b.e {
                                return false;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    w += 1;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    true
}
const _: () = assert!(unsigned_is_graded());

/// CLAIM C: the signed numeral fails event invariance while keeping value
/// agreement. The witness first, then the sweep.
const WL: R = s_eval3(-4, -3, 3, 0);
const WR: R = s_eval3(-4, -3, 3, 1);
const _: () = assert!(WL.v == WR.v);
const _: () = assert!(WL.e == 2 && WR.e == 0);

const fn signed_values_agree() -> bool {
    let mut x = S_LO;
    while x <= S_HI {
        let mut y = S_LO;
        while y <= S_HI {
            let mut z = S_LO;
            while z <= S_HI {
                let mut w = S_LO;
                while w <= S_HI {
                    let mut i = 0;
                    while i < 5 {
                        let mut j = i + 1;
                        while j < 5 {
                            if s_eval4(x, y, z, w, i).v != s_eval4(x, y, z, w, j).v {
                                return false;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    w += 1;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    true
}

const fn signed_events_agree() -> bool {
    let mut x = S_LO;
    while x <= S_HI {
        let mut y = S_LO;
        while y <= S_HI {
            let mut z = S_LO;
            while z <= S_HI {
                let mut w = S_LO;
                while w <= S_HI {
                    let mut i = 0;
                    while i < 5 {
                        let mut j = i + 1;
                        while j < 5 {
                            if s_eval4(x, y, z, w, i).e != s_eval4(x, y, z, w, j).e {
                                return false;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    w += 1;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    true
}

// The two components of one law's view, on one numeral, with opposite answers.
const _: () = assert!(signed_values_agree());
const _: () = assert!(!signed_events_agree());
