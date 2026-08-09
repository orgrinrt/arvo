# The notation vehicle: staged out, not merely munched

Nada Amin, file 61. This is my first file in this review.

**What I read.** `58_consolidation_five.md` in full, the standing instruction's only
required reading, and its two successors, `59_fog_the_lowering_door.md` and
`60_dolan_value_or_datum.md`, the whole of what has landed since it. An `ls` of the panel
directory confirmed nothing has landed after `60`. Behind the consolidation, for
derivations it compresses or for facts I wanted to check myself rather than take as read:
`42_probes/vu_nat_sealed.rs` (the sealed `Pos`/`Nat`/`Gcd` tower, whose shape I reuse
verbatim rather than re-derive, per "keep the shape where it serves"), `56_jhala_the_
diagnostic_fixture.md` sections 3 and 4 in full (the face-survives-declaration finding
and the const-struct-face sealing wall this dispatch's own brief leans on), and
`unstable-features.md` (the forbidden/allowed feature tables, since my brief's
constraints repeat them and I wanted the primary source, not the brief's paraphrase). In
the shipped tree: `~/Dev/clause-dev/notko/notko-macros/Cargo.toml` and `notko-macros-
core/Cargo.toml`, the precedent named in the brief, read to confirm what it actually
depends on rather than trust the description.

**Gates.** Canon: the numeral tower has no shipped source, reproduced fresh from the
repo root, `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the
same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty, matching the consolidation's
own opening paragraph. Everything I built lives under `mock/research/sketches/
202608041500_numeral-notation-vehicle/` and `61_probes/`, neither of which the mock
workspace gate covers (per `mock-workspace.md`'s own stated boundary: the gate protects
per-crate documents and `*.rs` under `mock/crates/*`, nothing under `mock/research/`).
Test: `cargo test --offline --workspace` from `mock/`, summed per binary, **658 passed,
0 failed, 9 ignored**, identical to file 59 and file 60's own reported counts. Nothing in
this dispatch touches a shipped crate, so an unchanged count is the expected result,
confirmed rather than assumed. Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
resolved automatically by `rustup` from the repo's `rust-toolchain.toml` for every
`rustc` invocation below (verified: a bare `rustc --version` from inside the repo tree
prints this exact string; the brief's own warning that a bare `rustc` outside the tree
resolves to stable does not apply here because every command in this file runs with the
repo as the working directory, and I checked this rather than assumed it).

**Compiled, measured, reasoned, kept apart.** Sections 1, 2 and 4 are compiled: five
probes in `61_probes/`, every outcome reproduced verbatim in `61_probes/OUTCOMES.md`.
Section 3 is a working artifact, not a probe: a full sketch under `mock/research/
sketches/202608041500_numeral-notation-vehicle/`, with a `run.sh` that rebuilds and
re-runs every file in order and an `OUTCOME.md` stating WORKS, per `bench-and-sketch-
discipline.md`. Section 5 is compiled (923 assertions, one real bug found and fixed).
Section 6 is compiled twice over, bisected to the exact bit. Section 7 is compiled at
this vehicle's own types. Section 8 is measured, `--emit=metadata`, min-of-3, following
the same instrument files 41, 42, 53, 54 and 56 already used for compile-cost pricing in
this review, not the runtime bench harness, because this measures rustc's own metadata-
emission time on host-side-computed token streams, not the runtime behaviour of
compiled code; nothing here needed a timer over generated machine code, and the
existing panel convention for exactly this shape of question is the probe-based sweep
these six files establish. Where I attribute a cost to a mechanism I say whether the
number is directly measured or derived from a comparison across two sweeps, and I do
not let the second wear the first's clothes.

**One correction to the brief, before anything else.** The brief frames the choice as
"declarative form first, fall back to proc-macro if hairy or slow." Section 1 compiles
the reason this framing needs sharpening: the declarative form does not fail because it
is hairy or slow. It fails because `macro_rules!` cannot see the information it would
need to be hairy about. A muncher needs digits to munch, and a decimal literal is one
atomic token to the lexer, with nothing macro_rules! can do to it that exposes what is
inside. This is a compiled negative result, not an untested argument, and it changes
what "fall back" means: not "the declarative attempt got complicated," but "the
declarative attempt cannot start."

## 1. The digit-decomposition wall

A `macro_rules!` muncher needs, at minimum, to see the individual digits of the literal
it is converting. Two attempts, both legitimate uses of the macro system, neither a
token hack.

**Attempt A: match the literal against two separate `tt` patterns.** If the lexer had
produced two tokens for `37`, this would see them.

```rust
macro_rules! attempt_a {
    ($a:tt $b:tt) => { concat!("two toks: ", stringify!($a), ",", stringify!($b)) };
    ($one:tt) => { concat!("one tok: ", stringify!($one)) };
}
```

`attempt_a!(37)` prints `one tok: 37`. `attempt_a!(3.14)` prints `one tok: 3.14`. Neither
integer nor float literals split.

**Attempt B: capture as `:literal`, restringify, feed the string back through the same
two-token matcher**, in case capturing and re-emitting exposes structure the raw token
stream did not.

```rust
macro_rules! attempt_b_inner {
    ($a:tt $b:tt) => { concat!("string decomposed: ", stringify!($a), ",", stringify!($b)) };
    ($one:tt) => { concat!("string stayed one tok: ", stringify!($one)) };
}
macro_rules! attempt_b { ($lit:literal) => { attempt_b_inner!( $lit ) }; }
```

`attempt_b!(37)` prints `string stayed one tok: 37`. `stringify!` turns tokens into a new
string-literal token, itself atomic; nothing decomposes it either.

*grounded on: `pin`, `flags` (`rustc --edition 2024`, no other codegen flags), `61_probes/
probe_1_digit_decomposition_is_impossible.rs`.*

**Two structurally different splits do exist, and neither reaches the digits.**
`61_probes/OUTCOMES.md` records both, because they explain what a `macro_rules!`
notation could honestly promise if it stopped short of full digit decomposition. A
leading `-` is its own Punct token (`peel_sign!(-37)` matches `- $mag:tt` cleanly). An
explicit `NUM / DEN` rational is three tokens (`peel_ratio!(37/53)` matches `$n:tt /
$d:tt` cleanly). A decimal point inside a float literal does not split (`peel_float!
(3.14)` falls through to the whole-token arm; `3.14` lexes as one float `Literal`,
matching `attempt_a`'s finding rather than contradicting it). So a declarative macro
*could* honestly offer sign-and-rational-slash decomposition as syntax sugar over
digit-tokenised operands (`numeral!(3 7 / 5 3)`, say), but that is not "a consumer writes
any number as a literal" per the ratified intent (58:626); it is a different, worse
notation the intent explicitly does not want.

## 2. The value-to-type escape is also walled: the spine rule fires a seventh time

If digits cannot be matched, could the literal's *value* (known via a `const fn` over
the stringified text, or directly as a `const` generic parameter) drive a recursive
type-level peel instead, sidestepping token matching entirely? The consolidation
compiled this wall once already, for the exponent (58:143-148). I did not take that as
read for a different quantity; I compiled it fresh, for a numeral's magnitude
specifically, because a construction that fails for one const-generic position does not
automatically fail for every position without a shared reason, and the shared reason is
worth stating rather than assumed.

```rust
pub trait FromU64<const V: u64> { type Out: Nat; }
impl<const V: u64> FromU64<V> for () where (): FromU64<{ V / 2 }> {
    type Out = S<<() as FromU64<{ V / 2 }>>::Out>;
}
```

**Bare language:**

```
error: generic parameters may not be used in const operations
   |
   |     ... FromU64<{ V / 2 }> ...
   |                   ^ cannot perform const operation using `V`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

**`min_generic_const_args`, the one further permitted opener the workspace's own
allowed table names (`unstable-features.md`):**

```
error: generic parameters may not be used in const operations
   |
   | ... FromU64<{ const { V / 2 } }> ...
   |                       ^
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

Both refuse identically to the exponent case: `V / 2` is a const operation over a
generic const parameter, needing to appear in the next type's const-generic position,
and neither the bare language nor its one sanctioned successor admits it. The escalation
path the error itself names, `generic_const_args`, needs `-Znext-solver=globally`,
already established in this review as mutually exclusive with the rest of the
arrangement (58:146-147, citing the workspace's own record). No conclusion is drawn from
a third arm of this probe gated on the forbidden `generic_const_exprs`: that arm
self-conflicts on an unrelated bug in its own test scaffolding (two overlapping impls
both matching `V=0`), recorded honestly in `61_probes/OUTCOMES.md` rather than fixed and
presented as a finding, since `generic_const_exprs` is forbidden regardless of what it
would show.

The consolidation's own count of how many times the spine rule has fired disagrees with
itself between files 55 and 56 (58:94-96), and this document does not resolve that
disagreement on their behalf either. What it adds is a position: the notation macro's
own host-side magnitude, driving a hypothetical const-generic peel, is not a new kind of
wall. It is the identical wall, at whatever the next ordinal is, confirming again that
"any future carrier this design mints should expect to hit it before assuming it will
not" (58:98-99) generalises past carriers to *vehicles that would compute a carrier from
a value*.

*grounded on: `pin`, `flags`, `61_probes/probe_2_value_to_type_escape_also_walled.rs`.*

**What the two walls together decide.** Neither the syntax route (macro_rules!
digit-matching) nor the semantics route (const-generic value-to-type recursion) reaches
a decimal literal's digits inside the type system's own machinery. Both dead ends are
compiled, not argued, and together they leave exactly one door: a mechanism that reads
the literal's *text*, outside the type system, and emits the already-decided encoding as
tokens. That door is a proc-macro, because `proc_macro::Literal::to_string()` gives
ordinary `&str` access no `macro_rules!` fragment specifier does.

## 3. The vehicle: a proc-macro with no external dependency, and what it costs to add

`mock/research/sketches/202608041500_numeral-notation-vehicle/crates/numeral_pm.rs`,
compiled with `rustc --crate-type proc-macro`, using only `extern crate proc_macro;`
(`TokenStream`, `TokenTree`, `Literal`, `Ident`, `Punct`, `Group`, all sysroot-shipped,
no `Cargo.toml`, no `syn`, no `quote`, no `proc-macro2`). This is a lighter footprint
than the workspace's own named precedent, not a departure from it: `notko-macros-core`
(`~/Dev/clause-dev/notko/notko-macros-core/Cargo.toml:14-17`) depends on `syn`, `quote`
and `proc-macro2`, stated as an ordinary compile-time dependency with no `#![no_std]`
anywhere in that crate, matching exactly what `no-alloc-no-std-framing.md` already
permits ("proc-macro crates can use `syn`/`quote`/`Vec`/`String` internally at
macro-expansion time. None of that reaches consumer binaries."). Mine needs none of it,
because everything it does is: read a literal's text, do ordinary `u128` arithmetic,
build a bit list by repeated halving, and format the result as a string that
`TokenStream::parse` turns back into tokens. This is worth stating precisely because it
means the notation macro's own dependency footprint is not a design cost this review
needs to weigh; it can be exactly as small as it wants to be.

**Two entry points**, matching file 56 section 4.3's own already-adopted resolution
(58:688-693) rather than inventing a third shape:

```
raw_bias!(EXPR)            -> a type expression: BPos<N, D> / BNeg<N, D> / BZero
numeral_face!(Name = EXPR) -> an item: pub struct Name; impl NumeralFace for Name { ... }
```

`EXPR` is `37`, `-37`, `37/53`, `-37/53`, `3.14`, `-3.14`: sign (its own Punct token),
integer or float literal (opaque, but its *text* is available to the proc-macro), an
optional explicit `/ DEN` (its own Punct token, three-token rational, exactly the split
section 1 found macro_rules! *can* see). Every one of these is ordinary Rust syntax
today; the macro adds no new grammar.

**Everything the digit-decomposition wall named impossible is now the easy half of the
file.** Splitting a float literal at its decimal point is one `str::split_once('.')`
call. Extracting digits is `str::parse::<u128>()`. Folding a decimal point into a
rational (`3.14` -> `314/100`, reduced to `157/50`) is four lines of ordinary
arithmetic. None of it needed a macro trick, because none of it is happening inside the
macro system at all; it is happening in the proc-macro's own Rust code, running on the
host, before a single output token exists.

**Emitting the encoding is the same shape at every magnitude, unbounded by
construction.** `emit_pos` peels bits by ordinary `u128` shifting and pushes `O`/`I`/`H`
tokens; there is no table, no per-width branch, no ceiling in the function itself (the
ceiling lives in a separate, explicit check, section 6). This is what the ratified
intent (58:626, "unbounded range, emitted constructors, zero table") asks for, delivered
by construction rather than by discipline: the function has nothing to be bounded by.

## 4. A tension the seal creates, not addressed by file 56's own probe

File 56 section 4.3 recommends the smart-constructor pattern precisely because a
macro-minted concrete newtype has no attacker position: "nothing outside the trusted
emitter can mint a second inhabitant." That probe (`56_probes/probe_7`) used a
hand-written, *unsealed* toy `NumeralFace` trait. It never tested what happens when the
macro tries to mint a type implementing a trait that actually *is* sealed the way this
design's own carriers are, which is the situation `numeral_face!` is actually in: `Bias`
is sealed via a private supertrait exactly like `Pos`, `Nat`, `Radix`, `Specials`,
`Underflow` and `Exponent` (58 section 1.12, lines 351-368).

```rust
mod tower {
    mod sealed { pub trait Sealed {} }
    pub trait Bias: sealed::Sealed { const NUM: i128; }
    pub struct BZero;
    impl sealed::Sealed for BZero {}
    impl Bias for BZero { const NUM: i128 = 0; }
}
struct MintedByMacro;
impl tower::Bias for MintedByMacro { const NUM: i128 = 37; }
```

```
error[E0277]: the trait bound `MintedByMacro: Sealed` is not satisfied
   |
17 | impl tower::Bias for MintedByMacro {
   |                      ^^^^^^^^^^^^^ unsatisfied trait bound
   = note: `Bias` is a "sealed trait", because to implement it you also need to
     implement `tower::sealed::Sealed`, which is not accessible; this is usually
     done to force you to use one of the provided types that already implement it
```

I did not even need a second crate to demonstrate this. A type declared in a different
*module* than the private `sealed` supertrait already cannot reach it, and a macro's
expansion inserts text into the calling module, which is strictly more restricted than a
different module in the same crate. A proc-macro crate is, in addition, structurally
forced to be separate from the tower crate: Cargo's `proc-macro = true` cannot coexist
with ordinary `pub` exports in one crate target, so `numeral_pm` cannot itself be the
crate that declares `Bias`.

**This sharpens file 56's own recommendation rather than contradicting it, and the
sharpening is worth stating precisely because it changes what "bridge trait" means.**
File 56 already used the phrase "a concrete, non-generic newtype implementing a bridge
trait to it" (58:689) but framed it as one legibility-preserving option among the
description of the smart-constructor pattern, not as a structural necessity. It is a
structural necessity, the moment the trait on the other side of the bridge is sealed:
there is no version of "the macro implements `Bias` directly for its minted type" that
compiles, ever, from outside the crate that declares `Bias`'s private supertrait. So
`numeral_face!`'s emitted `Name` does not implement `Bias`. It implements a *different*,
deliberately unsealed trait this sketch adds to the tower:

```rust
pub trait NumeralFace {
    type Encoding: Bias;
    const DISPLAY: &'static str;
}
```

`NumeralFace` is not sealed, on purpose: it is an unbounded, per-literal vocabulary (one
type per distinct numeral written), the opposite of the four closed carriers section
1.12 enumerates. Sealing it would be the bounded table the design already refused
(49:1004-1007), reached by a different route. `Name::Encoding` points at the same
`BPos<N,D>`/`BNeg<N,D>`/`BZero` the open `raw_bias!` form would have built, and that
type's own `Bias` impl is a blanket generic impl already written once, inside the tower
crate, covering every coprime pair automatically; `numeral_face!` never needs to write a
new `Bias` impl at all, because it never needs to touch `Bias` directly.

*grounded on: `pin`, `flags`, `61_probes/probe_3_macro_cannot_cross_a_seal.rs`; the
general claim (no macro in a separate crate can satisfy a private-supertrait seal) is
structural rather than incidental to this one test, since crate separation is imposed by
Cargo's own proc-macro crate-type constraint, not by a choice this sketch made.*

**Stated as spec text, because it belongs beside file 56's own section 8.** Wherever
this design's notation vehicle mints a concrete face for a sealed encoding, the face
connects to the encoding through an unsealed bridge trait declared in the tower crate,
never by implementing the sealed trait on the macro-minted type. This is not a stylistic
preference between two working shapes; it is the only shape available once sealing is
present, and a future carrier this design seals should expect the same consequence for
any macro that wants to mint concrete faces over it.

## 5. The whole-matrix test, and the bug it caught before this file existed

`consumer_matrix.rs`, 923 `assert_val!` invocations, each checking `<raw_bias!(...) as
Bias>::NUM`/`::DEN` against the value written, read back through the type-level
`Pos::VAL`, never merely "it compiled." Two shapes of coverage, not one: hand-picked
boundary and near-miss cases (the brief's own `37`/`38`/`83` example, sign, rationals
that need real reduction, decimals, powers of two and their neighbours), and an
exhaustively generated block covering every three-digit magnitude, 100 through 999, 900
cases, no sampling. The domain itself is unbounded by design, so "the whole matrix"
means every dimension that could independently break gets covered rather than every
integer to infinity; the one dimension that is itself small and finite (three-digit
magnitudes) is enumerated completely rather than spot-checked, matching `catalogue-edge-
cases-as-tests.md`'s own "never assert a law over a sample of shapes when the whole
matrix is available."

**The first run failed, and it failed on the brief's own near-miss family.**

```
thread 'main' panicked at consumer_matrix.rs:53:5:
assertion `left == right` failed: DEN mismatch for raw_bias!(37 / 53)
  left: 1
 right: 53
```

The cause: combining an explicit `/ DEN` with the macro's already-parsed numerator
multiplied the wrong side. `(num/den) / explicit_den == num / (den * explicit_den)`; an
earlier draft multiplied `num` by `explicit_den` too, copied wrongly from the adjacent
decimal-point-folding branch, which *does* need to scale the numerator because it is
combining an integer part and a fractional part into one fraction, a different
operation. `37/53` (both prime, genuinely coprime) came out as `37/1` because `37 * 53 =
1961`, and `1961/53` reduces right back down to `37/1`, silently. This is exactly the
brief's own warning realised: three numerals differing by one character (or, here, by
one symbol) would have compiled and passed a weaker test, because the corrupted `37/53`
case still produced a *valid*, internally-consistent `Bias`, just the wrong one. The
fix (den scales, num does not) is one line; the finding is that the strict test caught a
real defect in the vehicle before any reviewer would have, which is the whole reason
`catalogue-edge-cases-as-tests.md` and the review's own "red is the lifeblood" discipline
exist. After the fix, all 923 assertions pass.

*grounded on: `pin`, `flags`; `mock/research/sketches/202608041500_numeral-notation-
vehicle/crates/consumer_matrix.rs`, `numeral_pm.rs`'s own inline comment recording the
bug at the fixed line, and `run.sh`, which reproduces the whole sweep from a clean
build.*

## 6. Two ceilings, not one, and the vehicle reports the one that actually bites

**58:618 states one ceiling, "roughly `2^127`."** Read closely, the consolidation
already distinguishes two mechanisms behind it (58:610-622): a "type wall at depth 130"
and a "`u64` readout ceiling," stated in the same paragraph but at two different
figures in two different places (`10^20` at 58:610-611, in the decimal-radix discussion;
`2^63` at 58:621-622, in the general-fact paragraph). I compiled both mechanisms
directly against a binary (radix-2) `Pos`, bisected to the exact bit, because "roughly"
is not a number a notation macro's own refusal message can print, and because the two
consolidation figures do not agree with each other numerically (`10^20 ~ 2^66.4`,
against `2^63`), which is itself worth flagging rather than silently resolving by
picking one.

**The structural wall, isolated from readout entirely.** Naming a `Pos` type and
checking only that it satisfies the `Pos` bound, never reading `VAL`:

| nesting depth | outcome |
|---|---|
| 125, 126, 127, 128 | compiles clean |
| 129, 130, 131, 200 | `error[E0275]: overflow evaluating the requirement ...: Pos` |

Depth 128 succeeds, 129 fails. This sharpens 58:618's "roughly `2^127`" by one bit in
the generous direction: a 128-bit magnitude is nameable as a type, not merely a
127-bit one.

**The readout wall, isolated from structure entirely.** Naming the identical shape of
type at widths well under the structural ceiling, then reading `Pos::VAL`:

| magnitude bit-length | reading `Pos::VAL` |
|---|---|
| 62, 63, 64 | succeeds |
| 65 | `error[E0080]: attempt to compute 2_u64 * 9223372036854775808_u64, which would overflow` |

Exactly at the boundary a `u64` type implies: values up to `2^64 - 1` (64 bits) read
back fine; `2^64` and above (65+ bits) panic at const-eval time. This is not
approximately `2^63`; it is precisely `2^64`, the definition of what a `u64` holds, and I
state it that precisely because the consolidation's own two figures for the same
mechanism disagree with each other and neither is exactly this. I have not determined
whether `10^20` is a separate, genuinely decimal-radix-specific figure (plausible: a
radix-ten `Pos::VAL` accumulates by `10 * P::VAL + digit` rather than doubling, and
`10^19 < u64::MAX < 10^20`, so `10^20` reads as a reasonable just-past-the-ceiling figure
for *that* accumulation pattern specifically) or a looser statement of the same binary
fact this file pins exactly; I did not build the radix-ten accumulator to check, and I
say so rather than guess.

**The two walls are independent, and the tighter one (65 bits) fires for any consumer
who reads this vehicle's own output, at a threshold 64 bits below the looser one.**
`Bias::NUM`/`Bias::DEN` read `Pos::VAL` to populate themselves, so any magnitude between
65 and 128 bits produces a `Pos` type that compiles, composes, and satisfies ordinary
bounds, and then panics the moment anything (this file's own tests, a future `Display`
impl, any diagnostic) tries to read it back. A ceiling guard set at the looser,
consolidation-stated `2^127` figure would let exactly this class of magnitude through,
silently, and the failure would surface later, deep inside whatever first tried to read
the value, as the identical illegible `E0080` this section's own naked reproduction
shows:

```
error[E0080]: attempt to compute `2_u64 * 9223372036854775808_u64`, which would overflow
  --> tower.rs:39:22
   |
39 |     const VAL: u64 = 2 * P::VAL;
   |     evaluation of `<O<O<O<...O<H>...>>>>>>>>>>>>>>>>>>>>>> as Pos>::VAL` failed here
```

**The vehicle's own refusal, instead.** `check_ceiling` refuses at the tighter, honest
threshold, using the actual decimal number computed host-side, never the encoding, since
the macro has known the number since before any type token existed:

```
error: numeral literal 123456789012345678901234567890/1 needs 97 bits; the TYPE is
nameable (under this design's 128-bit structural ceiling) but `Bias::NUM`/`Bias::DEN`
cannot be read back, because `Pos::VAL: u64` (58:610-611, 621-622) overflows past 64
bits. display value for reference: 123456789012345678901234567890/1. This is not a bug
in the notation; it is the open question at 58:1088-1089 (widen the u64 readout, a
multi-limb readout, or a comparison-only interface), reached here because a literal
this large was written.
```

Compare this against the naked reproduction two paragraphs up: same underlying wall,
one prints thirty digits of the actual number and names the exact open design question
it hits, the other prints an ellipsis-truncated nest of `O<`. This is the whole of what
"the diagnostic prints the encoding rather than the number" (the brief's own opening
complaint) needs never to happen: not a diagnostic-attribute trick, not a decoder-ring
workaround, simply never asking the type checker for a number the macro already had.

**How the notation should report hitting it, stated as the design recommendation this
section earns.** Two-tiered, not one: refuse at the tighter, mechanism-specific
threshold that this design's *current* readout spelling (`Pos::VAL: u64`) actually
imposes, name the real decimal magnitude, and say explicitly that the refusal is the
open widening question (58:1088-1089) rather than a defect in the notation. A vehicle
that instead refuses only at the looser structural figure is not being more permissive;
it is deferring an identical failure to a worse, later, less legible position, for every
magnitude in a 64-bit-wide band nobody asked it to accept.

**One honest gap in this implementation, not papered over.** `check_ceiling`'s
structural-wall branch (128 bits) is real, correct design intent and is dead code under
this sketch's own host arithmetic: `Parsed::num`/`den` are `u128`, and `u128::MAX` needs
exactly 128 bits, so no value the macro's own `.parse::<u128>()` can ever produce
requires more than 128 bits. A sixty-digit literal is refused by `parse::<u128>()`
itself, with its own honest ("does not fit u128") message, before `check_ceiling` ever
runs. A vehicle wanting to exercise the structural wall by its own reporting mechanism,
rather than by the accident of a narrower host integer, needs host arithmetic wider than
`u128` (a small bignum, or a fixed `u256`); this sketch does not build one, and says so
rather than silently narrowing its own claim.

*grounded on: `pin`, `flags`, `61_probes/probe_4_val_readout_{62,63,64,65}bits.rs`,
`probe_5_structural_{125..200}bits.rs`, `probe_5b_naked_200bit_no_guard.rs`;
`consumer_ceiling_readout.rs` and `consumer_ceiling_structural.rs` reproduce both
through the vehicle itself.*

## 7. The decoder ring, reproduced at this vehicle's own emitted types

File 56 section 7 already established, against a hand-written stand-in, that a
declaration-site face mismatch is legible (E0308 prints the face's own name) and that an
operation generic over the raw encoding decays it one hop in. I reproduced both against
types `numeral_face!` itself emits, because file 56's own claim that E0308 "always
prints the fully-expanded alias, for a hand-written alias or a macro-emitted one alike"
(58:658-659) deserves checking against a macro that computes its output from digits,
not only against one that copies a hand-typed const generic.

**Declaration-site mismatch, legible, exactly as file 56 found:**

```rust
numeral_face!(N37 = 37);
numeral_face!(N38 = 38);
fn needs_n37(_: N37) {}
fn main() { needs_n37(N38); }
```
```
error[E0308]: mismatched types
   |
   |     needs_n37(N38);
   |     --------- ^^^ expected `N37`, found `N38`
```

**Decayed at the first operation generic over the raw `Bias`, exactly as file 56
predicted for any operation not built on the face:**

```rust
fn needs_encoding<B: Bias>(_: core::marker::PhantomData<B>) {}
needs_encoding::<<N38 as NumeralFace>::Encoding>(
    core::marker::PhantomData::<<N37 as NumeralFace>::Encoding>);
```
```
error[E0308]: mismatched types
   | expected `PhantomData<BPos<O<I<I<O<O<H>>>>>, H>>`,
   |    found `PhantomData<BPos<I<O<I<O<O<H>>>>>, H>>`
```

Both confirm file 56's finding rather than extend it; recorded here because a claim
tested only at a hand-written stand-in and a claim tested at what the actual vehicle
emits are different strengths of evidence, and the review's own discipline (`design-is-
the-oracle.md`'s sibling instruction to check cheap claims yourself) asks for the second
where it is cheap to get, which it was.

*grounded on: `pin`, `flags`, `consumer_diagnostic.rs`, `consumer_diagnostic_decay.rs`.*

## 8. Staging, priced: what is decided when

This dispatch's subject, stated in its own terms now that the vehicle exists to measure.
A literal a person writes is one definition of a number; the macro projects it into
three different forms, each answering to a different stage, and the question worth
pricing is which stage pays for the fraction being reduced to lowest terms.

**Macro-expansion time, on the host, now.** Text parsing, digit extraction (the whole
half section 1 found `macro_rules!` structurally cannot reach), decimal-point folding,
GCD reduction of the numerator and denominator, bit decomposition into the `O`/`I`
nest, `DISPLAY` string formatting. All ordinary native arithmetic; none of it visible to
the type checker at all, because none of it produces a type-level obligation. This
stage is paid once, at the point the macro runs, in native-speed host cycles, and its
cost does not appear in any of the numbers below because it is too small to separate
from process-startup noise at this sample size.

**The consumer's own compile time, later, verification only.** Whether the emitted pair
is coprime, checked by the tower's already-existing `Bias` bound (`N: Pos + Gcd<D, Out =
H>`), never a fresh reduction. Measured, `--emit=metadata`, min-of-3, 60 distinct random
compositions per sweep, each forced to be checked via a trivial `const fn` bound rather
than left as an unused alias (an earlier, uncorrected version of this sweep measured the
*unused* alias, `0.21` ms/composition, which is not a fair figure: nothing forces the
bound in that shape, the identical hazard file 56 section 4.2 already names for an
unchecked spec, and I do not repeat that mistake as a finding):

| shape | marginal ms/composition |
|---|---|
| `raw_bias!`, `Bias`'s own `Gcd` bound forced | **3.10** |
| `numeral_face!`, checked once at the impl site via `NumeralFace::Encoding: Bias` | **2.80** |

**Runtime, never.** These are zero-sized `PhantomData` markers; nothing about a
`Bias`-carrying type has a runtime representation to pay for.

**Does reducing host-side actually save anything, against making the type checker
reduce an unreduced pair itself?** This is the question the staging framing exists to
answer, and I built the comparison rather than assumed the direction. A third macro
entry point, test-only, emits an *unreduced* pair (numerator and denominator multiplied
by a shared common factor of 2 through 7 before emission) wrapped in `ReducedBias<N, D>`,
a type alias forcing the tower's own `Reduce`/`Strip2`/`ExactDivOdd` machinery (ported
verbatim from `42_probes/vu_nat_sealed.rs`) to run at the consumer's compile, rather
than merely verifying:

| shape | marginal ms/composition |
|---|---|
| pre-reduced host-side, verified only (`raw_bias!`, forced) | 3.10 |
| **unreduced, forcing the type-level `Reduce`** | **13.80** |

Roughly **4.5x**. This is a real, measured staging win, not a hypothetical one: the
macro's own host-side reduction is not merely convenient, it removes the more expensive
of the two operations (division-based reduction) from every consumer's compile and
leaves only the cheaper one (gcd-equality verification). I state the multiplier as
approximate rather than to the decimal place, because the two sweeps' magnitude ranges
are close but not identical (`raw_bias!`'s pairs are drawn from 1,000-65,535; the
unreduced sweep's pre-multiplication base is 1,000-30,000, times a factor of 2 through
7), and I would rather understate the precision than overclaim it. The direction and
the order of magnitude are what this section is for, and both are solid.

**This is not directly comparable to section 1.23's own 2.1-143 ms/composition table
(58:892-905), and saying so is more useful than a false "50x cheaper" headline would
have been.** I measured this before I understood why my first pass looked like an
outlier: section 1.23 prices the tower's own *composition* operations (combining two
already-declared numerals' adjustments, e.g. through `mulnum`, which multiplies two
fractions and then must re-reduce a wider product). This section prices *declaration*
of one literal's own `Bias`, never composed with anything. These are different
operations over the same machinery, not the same operation measured twice, and a reader
who wants "does the notation macro make a declared numeral as cheap to use in an
arithmetic composition as the tower's own figures already say" has not been answered by
this section and should not read it as answering that.

**The collapsed tower, stated once, plainly.** A definitional account of "what number
did the consumer write" would interpret the literal at every stage that touches it: the
macro would compute nothing, and every consumer's compile would re-derive the same
reduction from the same digits, paying the review's own measured `Reduce` cost (13.80
ms here, and worse at the tower's own wider profiles) again and again, once per
declaration site, forever. Staging the interpretation into the macro, so that stage does
the arithmetic once and emits the *answer*, is the same move this review's own
`Lowering`/`Encoding` split already makes for hardware floats (58:798-806: a computation
either changes no law-visible fact or it is not free to skip) and the same move
`arvo-compile-time-last.md` already licenses workspace-wide (spend compile time
liberally when it buys a cheaper or more correct shipped artifact). The macro is not a
convenience wrapped around the design; it is the design's own binding-time decision,
made explicit and paid for exactly once, at exactly the stage that has the information
cheapest.

*grounded on: `pin`, `host`, `flags` (`--emit=metadata`, no codegen); `mock/research/
sketches/202608041500_numeral-notation-vehicle/crates/price_{raw,raw_checked,face,
unreduced}_n{0,60}.rs`, each run min-of-3 fresh for this file.*

## 9. What survives as shape, for the next consolidation to take directly

**The vehicle question is closed, not merely narrowed.** `macro_rules!` cannot ingest a
decimal literal's digits at all (compiled, not merely found hairy); the fallback the
brief itself named, a compile-time-only proc-macro with no external dependency, works,
is priced, and reproduces the review's own decoder-ring and face-decay findings at its
own emitted types. The next consolidation can state this without a hedge.

**The two entry points, `raw_bias!` and `numeral_face!`, both emitting from the same
host-computed encoding, connected to the tower's sealed `Bias` by an unsealed bridge
trait (`NumeralFace`) declared inside the tower crate.** This is file 56 section 4.3's
own recommendation, unchanged in shape, with the reason it must take this shape now
compiled rather than assumed: sealing forecloses any other route.

**The ceiling is two independent walls, and a notation vehicle owes both to a reader, at
the tighter one first.** `128` bits, structural (a `Pos` is nameable); `64` bits,
readout (`Pos::VAL: u64` can be read back). The consolidation's own "roughly `2^127`"
should carry both figures precisely, and its own `10^20`/`2^63` disagreement (58:610-611
against 58:621-622) should be resolved (checked against a radix-ten accumulator, or
corrected to one binary figure) rather than repeated forward.

**Reducing host-side, not merely emitting host-side, is the load-bearing half of the
staging decision, and it is measured, not asserted.** A vehicle that emits an unreduced
pair and lets the type checker reduce it pays roughly 4.5x more per declaration than one
that reduces before any token is emitted. This is the number the "what is decided when"
question was asking for.

## 10. Droplist additions

**A pure `macro_rules!` decimal-to-binary muncher, receiving an ordinary decimal
literal as written**: refused structurally, not merely found expensive. No fragment
specifier, no restringify-and-rematch trick, and no const-generic escape (walled
identically to the exponent case, 58:143-148) reaches a literal's digits. The one
partial split available (sign, explicit rational slash) is real but does not reach "a
consumer writes any number as a literal" per the ratified intent, and is not proposed
as a compromise notation.

**Pricing a "checked" `raw_bias!` sweep against an *unused* type alias**: the first
attempt at this file's own section 8 measurement did exactly this and produced a
misleading ~13x gap in the wrong direction (the trusted face looking far more expensive
than the open form), an artifact of the alias never forcing its own `Gcd` bound to be
checked at all, the identical unchecked-spec hazard file 56 section 4.2 already names.
Corrected by forcing the bound in both arms before comparing; recorded so a future
member measuring this design's compile cost checks that whatever they are timing is
actually being verified, not merely declared.

## 11. What this file leaves open

**Whether `10^20` (58:610-611) is a genuine, separate radix-ten-specific readout figure
or a loose restatement of the same binary `2^64` fact this file pins exactly.** I did
not build a radix-ten `Pos::VAL` accumulator to check; the reasoning for why `10^20`
would be the right figure under decimal accumulation is stated in section 6, not
compiled.

**Whether `Adjustment` (58:132, `Implicit<E, A: Adjustment, B: Bias>`'s second signed-
rational position) needs its own notation entry point, or shares this one's underlying
`emit_pos`/reduction machinery under a different wrapping constructor.** This file
targets `Bias` specifically, the axis a written numeral's own value most directly names;
`Adjustment` is structurally identical per 58:341-345 ("`Adjustment`/`Bias` signed
gcd-normalised rationals") but semantically a different per-numeral fact I did not
investigate, and I do not extend today's finding to it without checking.

**Whether every distinct literal a consumer writes twice should resolve to the same
face type, or a fresh one per invocation.** `numeral_face!(Name = EXPR)` requires the
consumer to name the face; I did not attempt automatic, deterministic name derivation
with cross-call-site identity, which the consolidation's own residuals already flag as
unpriced (58:1106-1109) and which is a real design question this file's scope did not
reach.

**A host arithmetic wide enough to exercise the structural (128-bit) ceiling through the
macro's own reporting, rather than through `u128`'s parse failure.** Named in section 6
as a real, priced gap, not built.

**The `float_algebraic`-style second independent vetting question this review carries
elsewhere does not apply here** (this vehicle uses no unstable feature at all, in either
the tower reused from `42_probes` or the proc-macro itself), stated so a reader does not
go looking for a vetting note that has no reason to exist for this file.
