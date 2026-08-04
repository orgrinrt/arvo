//! The vehicle: a proc-macro that reads a decimal literal's own TEXT (not
//! its lexer token), computes the value-unique binary encoding host-side
//! with ordinary `u128` arithmetic, and emits the type expression directly
//! as tokens. No `syn`, no `quote`, no `proc-macro2`: everything needed
//! (`TokenStream`, `TokenTree`, `Literal`, `Ident`, `Punct`, `Group`) ships
//! in the sysroot's own `proc_macro` crate. This is a lighter dependency
//! footprint than the workspace's own precedent (`notko-macros-core`
//! depends on `syn`/`quote`/`proc-macro2`), not a departure from it: the
//! precedent is "a proc-macro crate uses std at compile time only, stated
//! honestly" (`no-alloc-no-std-framing.md`), and this crate does exactly
//! that, using only what the toolchain ships.
//!
//! Two entry points:
//!   `raw_bias!(EXPR)`      -> a type expression, `BPos<N, D>` / `BNeg<N,
//!                             D>` / `BZero`, generically constructible,
//!                             bound by `N: Pos + Gcd<D, Out = H>` at
//!                             every position it appears (open form).
//!   `numeral_face!(Name = EXPR);` -> an item: `pub struct Name;` plus
//!                             `impl Bias for Name` and `impl NumeralFace
//!                             for Name`, minted only here, no public
//!                             constructor (trusted-emitter form, file
//!                             56 section 4.3).
//! `EXPR` is one of: `37`, `-37`, `37/53`, `-37/53`, `3.14`, `-3.14`.
//!
//! Everything past "parse the literal's text" is ordinary host arithmetic:
//! this is the half of the problem `61_probes/probe_1` established
//! `macro_rules!` cannot do at all.

extern crate proc_macro;
use proc_macro::{Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// A parsed literal expression, before any binary encoding: sign, then a
/// numerator/denominator pair over the decimal digits as written. Floats are
/// folded into a rational at this stage (`3.14` -> `314/100`), which is the
/// one step `macro_rules!` could never reach because it needs to inspect
/// where the `.` sits inside what the lexer treats as one atomic token.
struct Parsed {
    negative: bool,
    num: u128,
    den: u128,
}

/// Parse the macro's input tokens into a `Parsed` value, or a diagnostic
/// string naming exactly what was wrong. This is the whole of what a
/// `macro_rules!` muncher cannot do: read the digits out of a literal.
fn parse_expr(input: TokenStream) -> Result<Parsed, String> {
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let mut i = 0;
    let mut negative = false;
    if let Some(TokenTree::Punct(p)) = toks.get(i) {
        if p.as_char() == '-' {
            negative = true;
            i += 1;
        }
    }
    let lit = match toks.get(i) {
        Some(TokenTree::Literal(l)) => l.to_string(),
        other => return Err(format!("expected a numeral literal, found {other:?}")),
    };
    i += 1;

    let (num, mut den) = if let Some((int_part, frac_part)) = lit.split_once('.') {
        // float literal: fold the decimal point into a rational directly,
        // by ordinary string slicing. macro_rules! cannot locate this '.'
        // inside the literal at all (61_probes/probe_1 arm A(3.14)).
        if int_part.is_empty()
            || frac_part.is_empty()
            || !frac_part.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(format!("malformed decimal literal: {lit}"));
        }
        let scale = 10u128
            .checked_pow(frac_part.len() as u32)
            .ok_or_else(|| format!("fractional part too long to scale: {lit}"))?;
        let int_val: u128 = int_part
            .parse()
            .map_err(|_| format!("integer part does not fit u128: {lit}"))?;
        let frac_val: u128 = frac_part
            .parse()
            .map_err(|_| format!("fractional part does not fit u128: {lit}"))?;
        let num = int_val
            .checked_mul(scale)
            .and_then(|v| v.checked_add(frac_val))
            .ok_or_else(|| format!("decimal literal does not fit u128 before reduction: {lit}"))?;
        (num, scale)
    } else {
        let num: u128 = lit
            .parse()
            .map_err(|_| format!("literal does not fit u128: {lit}"))?;
        (num, 1u128)
    };

    // optional explicit rational: `NUM / DEN`, three tokens the lexer
    // already separates (61_probes/probe_1 did not need to test this one;
    // `/` is Punct, never fused into a literal).
    if let Some(TokenTree::Punct(p)) = toks.get(i) {
        if p.as_char() == '/' {
            i += 1;
            let den_lit = match toks.get(i) {
                Some(TokenTree::Literal(l)) => l.to_string(),
                other => {
                    return Err(format!(
                        "expected a denominator literal after '/', found {other:?}"
                    ))
                }
            };
            i += 1;
            let explicit_den: u128 = den_lit
                .parse()
                .map_err(|_| format!("denominator does not fit u128: {den_lit}"))?;
            if explicit_den == 0 {
                return Err("denominator is zero: not a rational".to_string());
            }
            // (num/den) / explicit_den == num / (den * explicit_den). The
            // numerator is untouched; only the denominator absorbs the
            // divide. An earlier version of this file multiplied `num` by
            // `explicit_den` too (copied, wrongly, from the decimal-point
            // scaling branch above, which DOES need to scale the numerator
            // because it is converting an integer+fraction pair into a
            // single fraction, a different operation). consumer_matrix.rs's
            // `raw_bias!(37 / 53)` case caught it: DEN came out 1 instead
            // of 53 because the bad numerator inflation made 37/53 look
            // reducible back down to 37/1. Recorded here because it is the
            // whole-matrix test doing its job, not a footnote.
            den = den
                .checked_mul(explicit_den)
                .ok_or_else(|| "denominator overflow".to_string())?;
        }
    }

    if i != toks.len() {
        return Err(format!(
            "trailing tokens after the numeral: {:?}",
            &toks[i..]
        ));
    }

    let g = gcd_u128(num.max(1), den);
    Ok(Parsed {
        negative,
        num: num / g,
        den: den / g,
    })
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// Two ceilings, not one, and they are independent (61_probes/probe_4 and
// probe_5 compile both against a fresh copy of THIS tower's Pos::VAL,
// which is spelled `u64` per the ratified table, 58:857-862, not assumed).
//
// STRUCTURAL_CEILING_BITS: a `Pos` type can be NAMED, and can participate
// in ordinary trait bounds, up to nesting depth ~128 before the
// trait-solver's own recursion limit refuses it (E0275). This is the
// consolidation's own "roughly 2^127" (58:618), sharpened here to the
// exact compiled boundary: depth 128 succeeds, depth 129 fails, on this
// pinned toolchain with no `#![recursion_limit]` override.
//
// VAL_READOUT_BITS: `Pos::VAL: u64` cannot hold a magnitude of 65 bits or
// more; reading it panics at const-eval time (E0080), NOT at the type's
// own declaration. A `Pos` between 65 and 128 bits is a perfectly good
// TYPE (compiles, composes, satisfies bounds) that silently cannot be
// introspected. This is the tighter of the two by a wide margin (64 bits
// against 128), and it is the one THIS tower's `Bias::NUM`/`Bias::DEN`
// actually hit, because they read `Pos::VAL` to populate themselves.
const STRUCTURAL_CEILING_BITS: u32 = 128;
const VAL_READOUT_BITS: u32 = 64;

fn bits_of(v: u128) -> u32 {
    128 - v.leading_zeros()
}

/// Emit a `Pos` type expression for a nonzero magnitude: `H` for the
/// leading bit, then `O<...>` / `I<...>` innermost-out for every bit below
/// it, MSB just outside `H`. This is the whole of the "muncher": ordinary
/// imperative bit-peeling, ordinary token emission, no macro recursion, no
/// type-level arithmetic asked of the type checker at expansion time at
/// all.
fn emit_pos(v: u128) -> TokenStream {
    debug_assert!(v > 0);
    let mut bits = Vec::new();
    let mut x = v;
    while x > 0 {
        bits.push((x & 1) == 1);
        x >>= 1;
    }
    // bits is LSB-first; the last pushed bit is the leading (MSB) 1, which
    // becomes `H`. Walk from MSB-1 down to LSB, wrapping.
    let mut out: TokenStream = "H".parse().unwrap();
    for &bit in bits[..bits.len() - 1].iter().rev() {
        let ctor = if bit { "I" } else { "O" };
        out = format!("{ctor}<{out}>").parse().unwrap();
    }
    out
}

fn checked_parse(input: TokenStream) -> Result<Parsed, TokenStream> {
    parse_expr(input).map_err(|msg| {
        format!(
            "compile_error!(\"numeral literal: {}\")",
            msg.replace('"', "'")
        )
        .parse()
        .unwrap()
    })
}

/// Reports which of the two ceilings a magnitude hits, using the actual
/// decimal value computed host-side (`p.num`/`p.den`, already known before
/// any type token is emitted), never the encoding. This is the whole of
/// what "the diagnostic prints the encoding rather than the number" (the
/// brief's own opening complaint) needs never to happen at all: the macro
/// has the number in hand from the moment it parsed the literal, and never
/// has to ask the type checker for it back.
fn check_ceiling(p: &Parsed, sign: &str, display: &str) -> Result<(), TokenStream> {
    let nb = bits_of(p.num.max(1));
    let db = bits_of(p.den.max(1));
    let worst = nb.max(db);

    if worst > STRUCTURAL_CEILING_BITS {
        let msg = format!(
            "numeral literal {sign}{}/{} needs {} bits; exceeds this design's {}-bit structural Pos ceiling (the trait-solver's own recursion limit, compiled: 58:611-612, this file's 61_probes/probe_5). No type this design can name holds it; express it as an exponent instead of an absorbed rational (58:607-616)",
            p.num, p.den, worst, STRUCTURAL_CEILING_BITS
        );
        return Err(format!("compile_error!(\"{}\")", msg.replace('"', "'"))
            .parse()
            .unwrap());
    }
    if worst > VAL_READOUT_BITS {
        let msg = format!(
            "numeral literal {sign}{}/{} needs {} bits; the TYPE is nameable (under this design's {}-bit structural ceiling) but `Bias::NUM`/`Bias::DEN` cannot be read back, because `Pos::VAL: u64` (58:610-611, 621-622) overflows past {} bits (compiled: this file's 61_probes/probe_4). display value for reference: {display}. This is not a bug in the notation; it is the open question at 58:1088-1089 (widen the u64 readout, a multi-limb readout, or a comparison-only interface), reached here because a literal this large was written",
            p.num, p.den, worst, STRUCTURAL_CEILING_BITS, VAL_READOUT_BITS
        );
        return Err(format!("compile_error!(\"{}\")", msg.replace('"', "'"))
            .parse()
            .unwrap());
    }
    Ok(())
}

/// Test-only, for the staging-cost comparison in the writeup: parses
/// the SAME literal grammar as `raw_bias!` but does NOT reduce host-side.
/// Emits `ReducedBias<N, D>` (an UNREDUCED pair fed through the tower's
/// own type-level `Reduce`), so the consumer pays whatever `Reduce`
/// actually costs, the exact machinery the shipped `raw_bias!`/
/// `numeral_face!` skip by reducing before any token is emitted.
#[proc_macro]
pub fn raw_bias_unreduced(input: TokenStream) -> TokenStream {
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let mut i = 0;
    if let Some(TokenTree::Punct(p)) = toks.get(i) {
        if p.as_char() == '-' {
            i += 1;
        }
    }
    let num: u128 = match toks.get(i) {
        Some(TokenTree::Literal(l)) => l.to_string().parse().unwrap(),
        _ => {
            return "compile_error!(\"raw_bias_unreduced!: expected NUM / DEN\")"
                .parse()
                .unwrap()
        }
    };
    i += 1;
    if !matches!(toks.get(i), Some(TokenTree::Punct(p)) if p.as_char() == '/') {
        return "compile_error!(\"raw_bias_unreduced!: expected '/'\")"
            .parse()
            .unwrap();
    }
    i += 1;
    let den: u128 = match toks.get(i) {
        Some(TokenTree::Literal(l)) => l.to_string().parse().unwrap(),
        _ => {
            return "compile_error!(\"raw_bias_unreduced!: expected a denominator\")"
                .parse()
                .unwrap()
        }
    };
    let n = emit_pos(num);
    let d = emit_pos(den);
    format!("ReducedBias<{n}, {d}>").parse().unwrap()
}

/// `raw_bias!(EXPR)` -> `BPos<N, D>` / `BNeg<N, D>` / `BZero`, a type
/// expression. Open, generic, bound-checked by `Bias`'s own `Gcd`
/// perimeter at every position it is used, exactly like `Adjustment`.
#[proc_macro]
pub fn raw_bias(input: TokenStream) -> TokenStream {
    let p = match checked_parse(input) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let sign = if p.negative { "-" } else { "" };
    let display = format!("{sign}{}/{}", p.num, p.den);
    if let Err(e) = check_ceiling(&p, sign, &display) {
        return e;
    }
    if p.num == 0 {
        return "BZero".parse().unwrap();
    }
    let n = emit_pos(p.num);
    let d = emit_pos(p.den);
    let ctor = if p.negative { "BNeg" } else { "BPos" };
    format!("{ctor}<{n}, {d}>").parse().unwrap()
}

/// `numeral_face!(Name = EXPR);` -> item: a concrete, non-generic newtype
/// minted only here, plus a `NumeralFace` impl whose `Encoding` associated
/// type points at the SAME `BPos`/`BNeg`/`BZero` the open form would have
/// built. `Name` does NOT implement `Bias` directly: `61_probes/probe_3`
/// compiles the reason, a macro-emitted type can never satisfy a trait
/// sealed via a private supertrait it did not originate inside, so the
/// face is connected to its encoding by the bridge (`NumeralFace::
/// Encoding: Bias`, file 56 section 4.3's own vocabulary), not by
/// re-implementing the sealed trait itself. No public constructor exists
/// anywhere else, so there is no attacker position on `Name` itself:
/// nothing outside this macro can mint a second `Name`.
#[proc_macro]
pub fn numeral_face(input: TokenStream) -> TokenStream {
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let name = match toks.first() {
        Some(TokenTree::Ident(id)) => id.to_string(),
        other => {
            return format!(
                "compile_error!(\"numeral_face!: expected `Name = literal`, found {other:?}\")"
            )
            .parse()
            .unwrap()
        }
    };
    let eq_ok = matches!(toks.get(1), Some(TokenTree::Punct(p)) if p.as_char() == '=');
    if !eq_ok {
        return "compile_error!(\"numeral_face!: expected `Name = literal`, missing '='\")"
            .parse()
            .unwrap();
    }
    let rest: TokenStream = toks[2..].iter().cloned().collect();
    let p = match checked_parse(rest) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let sign_display = if p.negative { "-" } else { "" };
    let display = format!("{sign_display}{}/{}", p.num, p.den);
    if let Err(e) = check_ceiling(&p, sign_display, &display) {
        return e;
    }

    let encoding = if p.num == 0 {
        "BZero".to_string()
    } else {
        let n = emit_pos(p.num);
        let d = emit_pos(p.den);
        let ctor = if p.negative { "BNeg" } else { "BPos" };
        format!("{ctor}<{n}, {d}>")
    };
    let body = format!(
        "pub struct {name};\n\
         impl NumeralFace for {name} {{\n\
             type Encoding = {encoding};\n\
             const DISPLAY: &'static str = \"{display}\";\n\
         }}\n"
    );
    body.parse().unwrap()
}

// silence unused-import warnings for the TokenTree variants used only in
// pattern position on some code paths; kept explicit rather than globbed.
#[allow(dead_code)]
fn _unused(_: Ident, _: Literal, _: Punct, _: Spacing, _: Span) {}
