# The Remainder Beside the Quotient

Seat two of two on ticket `what_a_remainder_over_a_fractional_arvo_format_is_beside_the`, written in
worktree `remainder-seat-2` without sight of the other seat. The question is what a remainder is in arvo,
as an operation beside the quotient, and over which formats it is defined. The ticket offers three
options: the integer quotient's remainder at every exponent (a), the remainder at exponent zero only (b),
and no remainder in arvo (c).

The short answer is option a, corrected in two places where the ticket states it wrongly, and with its
region drawn along the coordinate that actually decides it. The exponent is not that coordinate. The
remainder by the integer quotient reads neither the radix nor the exponent at all: at a whole phase its
slot is an integer expression in the value indices of the two operands, and it is exact and on the grid
at every radix and every exponent. What decides it is the phase. At a phase whose reduced denominator `d`
is past one, `a - b q` is on the grid exactly where `d` divides `q`, so the remainder is not closed there,
and option a's claim to be "total over the formats the quotient admits" is false, because the quotient
admits exactly those formats and even gives them a door of their own (`Divisor::of`,
`mock/crates/arvo-format/src/division.rs:156`).

Everything below is derived from the canon rows named in the first section and checked by one committed
sketch, `mock/research/sketches/202611192130_the-remainder-on-the-grid/`, whose record is in the section
on the sketch, since the harness this seat ran under refused to write a file named `FINDINGS.md`. No
timing of any kind was taken.

## The canon gate

The gate passes. Nothing in the registry settles the question and nothing reserves it, and the question
as posed is aligned with the canon, though one of its framings is wrong (the exponent as the dividing
line), which is corrected in the answer rather than refused.

I asked the registry for every row whose text names a remainder, a quotient, a division, a modulus or a
modulo, across every namespace (a parse of `mock/registry/*.toml`, then `cargo mock query` on each hit).
No `ruling`, `question` or `proposal` row defines a remainder operation. The one row with the word in its
slug, `proposal::an_exact_position_carries_its_remainder_over_an_unsigned_128_bit_denominator`, is about
the residue an exact position carries below its floor, not an operation, and it is a proposal at
`one_expert`. No `question` row and no `rung = "open"` row names a remainder, so no decider other than
this panel holds the call.

The rows that govern the answer, by rung:

1. `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`, ratified by
   experts, answering `question::which_operation_set_the_design_ships` (decider `panel`): "an operation
   is admitted exactly when it is a function of the declared signature, and where two realisations of one
   name disagree, the signature is missing a coordinate." Its note leaves "membership above the floor"
   open in the form "what the framework must be able to express, and which of those ship now against
   marked later", which is the panel's own question, not a reservation.
2. `ruling::the_format_spine_is_canon`, ratified by both: "Arithmetic on a format is an exact operation
   in the ambient domain composed with a named total adaptation onto that set."
3. `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`, ratified by op from the written
   history: "Writing the arithmetic contracts on the standards is arvo's own ground."
4. `ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`, ratified by op: "The parity suite is
   built against MATLAB `fi`/`fimath` and IEEE 754."
5. `ruling::never_a_runtime_check_and_one_lowered_path`, ratified by op: "Never any runtime checks, ever:
   invalids are caught at compile time."
6. `ruling::arvo_is_a_library_and_the_value_composes_on_top`, ratified by op: "how end users will use it
   is unknown."
7. `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`, ratified by
   experts, which is why the pair below is a composition with its own law rather than a format operation.
8. `ruling::the_panel_finishes_the_canon_without_him`, ratified by op: the call is the panel's, derived
   from what he said, through two independent agreements.
9. `ruling::the_standard_is_parity_in_output_not_in_the_internals`, stated: the bound is "parity in
   output, not so much the internals", and a standard may ship with marked holes while "a hole in the
   framework is a defect."

Two open rows border the answer and it fills neither. `question::which_chain_carrier_ships` is open; the
pair below is two results of one exact step, not a carrier for a chain. `question::whose_reduction_governs_a_lossy_crossing`
is open; the remainder lands on its own declared signature exactly, so no lossy crossing occurs, and the
pair's quotient is the operation's own result landing on the signature the caller declares for it, which
I read as an operation's landing and not a crossing between two signatures of one value. That reading is
mine and is the first thing the other seat should attack.

This seat is one of two. Under the two-expert rule no call about what the canon permits is made on one
read, so this file is one instance and a second independent read is owed before anything here is stated
in the registry.

## The suite audit

The whole suite ran first, as `CARGO_INCREMENTAL=0 cargo mock test --full` (the workspace hook refused the
incremental form for disk reasons). The members ran 1061 tests across 60 binaries, 1061 passed and one
was skipped by nextest's filter; the tool trees passed; the bench crates passed 118 of 118. The lint
crate's own 1097 tests passed, and `mock test` still marked that tree failed, with the message "another
worktree rewrote the lint crate at /root/.cache/mockspace/lints/9ecec96946bdf528 while its tests ran, so
the result is not of this worktree's lints." That is contention with the other seat over a shared cache,
not a red in this tree, and I did not rerun it.

Then the bodies, every test in the surface the brief names that covers the quotient.

`mock/crates/arvo-numeral/tests/the_quotient.rs` holds three tests. The sweep (lines 63 to 81) divides
every member by every non-zero member at widths one to four, radix two, exponent zero, under five modes
and two policies, and compares the slot and the `fits_div` verdict with `common::oracle::quotient`, which
scales the slots' ratio by the quantum and rounds it itself rather than calling the format. It counts
wrong slots, asserts zero, and asserts the size of the sweep. The named quotients (lines 86 to 160) write
slots out by hand: `-7 / 2` under each mode, `-128 / -1` past the top under saturation and wrapping, one
over three eighths in eighths, and three over an eighth saturating. The constant test (162 to 168)
evaluates a quotient in a `const`. None is tautological.

`mock/crates/arvo-format/src/division/tests/` holds the format's: `the_divisor.rs` (where `of` is admitted,
against the oracle's own search for a zero slot, every condition on both sides of its boundary),
`the_oracle.rs` (every member pair up to eight bits against the applied map's oracle fed the exact
rational position, with a planted step that forgets the phase's term and must be reported, and a
control), `the_edges.rs` (widths nine to 64 at the divisor's extremes against `Big`), `the_far_quotient.rs`
(positions past the slot index, ties read by their own floor) and `the_member_door.rs` (the member door
against the oracle with planted verdicts). Each compares the step against an expected value formed
outside the step, and the two sweeps carry planted mutants. No test in `mock/crates/arvo-format/src/tests/`
proper covers the quotient beyond the shared helpers (`ratio.rs`, `big.rs`) these read.

No test pins a remainder, and none asserts anything a remainder by the integer quotient would contradict.
Two facts in them bear on the answer. The format's quotient lands on the operands' grid, not on the
integers: `pair` in `the_oracle.rs` (lines 73 to 109) holds `div::<S>` to the oracle's rounding of the
exact position on `S`'s grid, and `the_quotient.rs` line 139 asserts that one over three eighths is 21
eighths under floor. So the pair's quotient defined below is not `div::<S>` and must never be described as
it; at that very pair the integer quotient is 2 and the grid quotient is 2.625. And the helper `point` in
`the_oracle.rs` line 42 splits a rational by `rem_euclid`, which is the oracle's floor and residue, not an
operation.

Two observations about the quotient's suite that do not block this dispatch. Its exhaustive sweeps are a
sample of shapes: eleven per width at radixes two and ten in `the_oracle.rs` (lines 175 to 189), radix two
at exponent zero only in `the_quotient.rs`; the ternary radix is reached only in the numeral's
`the_quotient_at_every_shape_ternary.rs`. That is what the design's "laws this section owes tests for"
asks under Division, and nothing in this file rests on those tests: every claim here rests on the
derivation and on this seat's own sketch. And `Divisor::assumed` (`division.rs:199`) is a test-only door
that skips the type; the sweeps filter the zero out with `divisors` (`division/tests/mod.rs:66`), which
reads the oracle's value, not the step's, so the setup does not help the step.

## The answer

A remainder in arvo is an operation of `arvo-format`, beside the quotient, read through `arvo-numeral` by
equality as every other operation is. It is the scaled residue of rounding the exact quotient onto the
integers, by the mode the caller declares.

### The definition

Let `F` be a format whose quantum is constant, `Q = r^e`, whose phase `n / d` in lowest terms has `d = 1`
(a whole phase), and whose slot `s` names the value `(n + s) Q`. Write `k = n + s` for the value index of
a slot. Let `S` be a declared signature over `F` with rounding mode `M` and overflow policy `P`, let `a` be
a member of `F`, and let `b` be a `Divisor<F>`, a slot non-zero by type through any of its doors. Then

```
q  = round_M(a / b) = round_M(k_a / k_b)        an integer; stochastic reads the dither as the quotient does
r  = a - b q                                     exact, as a rational
slot(r) = k_a - k_b q - n                        an integer, so r is a point of F's grid
rem::<S>(a, b) = adapt_S(r)
```

Because `r` is a point of the grid, the adaptation's rounding never acts on it; its completion acts
exactly where `slot(r)` leaves `F`'s slot range, which `fits_rem` reports as every other `fits_` does.

The laws a test is written from, each over the region in the next subsection:

1. `r` is on the grid at every radix and every exponent, and `slot(r)` reads neither: it is the integer
   remainder of the value indices, `k_a - k_b round_M(k_a / k_b)`, shifted by `-n`.
2. `|r| < |b|` under every mode, and `|r| <= |b| / 2` under `half_up` and `half_even`.
3. Where `r` is not zero its sign is the dividend's under `toward_zero`, the divisor's under `floor`, and
   the divisor's opposite under `ceil`; the nearest modes give either sign. `stochastic` gives the floor's
   remainder or the ceiling's, one of the two.
4. `r = b (a / b - round_M(a / b))`: the residue of the integer rounding, scaled by the divisor. This is the
   identity that ties the remainder to the spine: it is the adaptation's residue onto the integers, and the
   mode that makes it is the declared one.
5. Closure on the range. Under `toward_zero` and `floor`, `r` lies between zero and `a`, or zero and `b`,
   both members, so `r` is a member wherever the range holds the zero member. On a two's complement range
   at phase zero every mode is closed, since `|r| < |b| <= 2^(w-1) Q` and `-MIN` is one past the top. Under
   `ceil` and the nearest modes an unsigned range is left (any pair with a negative remainder, `3 rem 4`
   under `half_even` is `-1`), and so is a whole-phase range that does not hold the zero, under every mode.
   Where it is left the policy completes it, as the spine says.
6. At exponent zero and a whole phase, `q` is the value of `div::<S>(a, b)` wherever that quotient is in
   range, and `rem::<S>(a, b) = a - b div::<S>(a, b)`; this is option b's pair, and it is the only region
   where the format's quotient and the integer quotient coincide. At every other exponent they differ.
7. The Euclidean remainder (`r >= 0` whatever the signs) is `rem` under `floor` where the divisor is
   positive and under `ceil` where it is negative. At a whole phase every `Divisor` door fixes that sign at
   build: `constant` names the slot, `member` takes a zero-free whole-phase range, which lies wholly on one
   side of its zero slot, and `of` is refused there. So the Euclidean remainder is a constant choice
   between two modes, never a branch on a value.

The pair. `divrem::<S, T>(a, b)` returns `q` landed on a signature `T` the caller names and `rem::<S>(a, b)`,
from one `q` rounded once. `T` must be a whole-phase signature whose grid holds the integers (a constant
quantum `r^e` with `e <= 0`, at any radix), so `q` lands on the grid and only `T`'s policy can act; a `T` whose grid does not hold the integers is refused at build, because
landing `q` there would round it a second time, the chain the conversion section already forbids
(`mock/crates/arvo-format/DESIGN.md.tmpl`, section "Conversion, one member placed on another signature's
grid"). The pair's law: `a = b q + r` exactly, as rationals, wherever neither landing's policy acted. The
span of `q` is the value indices' quotient span, which reads neither radix nor exponent: at phase zero,
over a `w`-bit range, `max |q|` is `2^(w-1)` signed (`MIN / -1` at the divisor next to the zero) and
`2^w - 1` unsigned, so a signed `w + 1`-bit integer holds every pair's quotient, and a member divisor's
narrower range narrows it. The ticket's example: in `Fix<8, -4>`, 7.9375 by 0.0625 is `q = 127`, `r = 0`,
and `q` is not a member of `Fix<8, -4>` (it would sit at slot 2032 of a range ending at 127); `-8 / -0.0625`
is 128, which needs the ninth bit.

Which mode. The remainder reads the mode of the signature it is called under, and that is the only
coordinate the four standard remainders differ by:

| Convention | Mode the integer quotient is rounded by |
|---|---|
| IEEE 754 `remainder` (2019, clause 5.3.1) | `half_even` |
| C `fmod`, C and Rust integer `%`, MATLAB `rem` on doubles | `toward_zero` |
| MATLAB `mod` on `fi` (documented as `m = x - floor(x./y).*y`), Python `%`, TAOCP's `x mod y` | `floor` |
| Rust `rem_euclid`, the Euclidean remainder | `floor` by a positive divisor, `ceil` by a negative one, by type |

Under the admission rule the remainder is admitted with no coordinate missing: its realisations disagree
only in the integer rounding direction, the declared signature carries one rounding coordinate, and at a
whole phase nothing else in this operation reads it, since `r` lands on the grid. A site that holds a value
under `half_even` and wants C's `fmod` crosses to the sibling signature `WithRounding<S, TowardZero>` with
`under`, which moves nothing (`mock/crates/arvo-numeral/DESIGN.md.tmpl`, "A value under a sibling
signature"), and calls `rem` there; the standards' aliases are exactly that crossing with the mode fixed,
which is parity in output as `ruling::the_standard_is_parity_in_output_not_in_the_internals` asks, whatever
the value's own mode is. The IEEE standard says its remainder is defined "regardless of the
rounding-direction attribute", and MATLAB's `fi` page says `mod` ignores the inputs' `fimath`; both mean the
direction is fixed per function, which the alias does, not that the direction cannot be a declared
coordinate.

### The region

| Region | What holds |
|---|---|
| Constant quantum, whole phase; every radix, exponent, signedness, mode and policy; every divisor door the quotient has there | Admitted. `r` exact and on the grid, laws 1 to 7, closure as in law 5. |
| Constant quantum, phase denominator `d > 1` | `r` is on the grid exactly where `d` divides `q`, and every such format has pairs where it does not. Not yet admitted (see below). |
| Magnitude-indexed quantum | Not yet admitted, under the quotient's own ticket `arvo_quotient_and_conversion_over_a_magnitude_indexed_quantu`. When it is: `r` is a member at every pair under `toward_zero`, `half_up` and `half_even`, and not under `floor`, `ceil` or `stochastic`. |
| Whole phase, range holding the zero, divisor read at run time of either sign | No door, inherited from the quotient (`division.rs:121`, `FIXME(may_arvo_divide_by_a_signed_divisor_read_at_run_time_of_eith)`). |
| A step past the four limbs | Not yet admitted, inherited from the quotient's third condition. |

The derivation of the two non-trivial rows.

At a phase `n / d` with `d > 1`, the members are `(n / d + s) Q`, so `a - b q` has position
`((1 - q) n / d + s_a - q s_b)`, and that is an integer exactly where `n q / d` is, which with `n` and `d`
coprime is where `d | q`. The zero itself is not a member at such a phase (no slot names it), so even
`b | a` has no member to land on. The exact remainder lies on the value-index lattice of quantum `Q / d`,
which is a format of arvo's only where `d` divides a power of the radix. There are two realisations of a
remainder here and they disagree: round `r` onto `F` under `S`'s mode, which reads the one mode twice for
two different roundings and breaks the identity at every pair where `d` does not divide `q`; or restrict
`q` to multiples of `d`, which is a remainder modulo `d b` with `|r| < d |b|`. By the admission rule's
second clause, two realisations of one name that disagree mean the signature is missing a coordinate. So
this region is not yet admitted, refused at build with its own message and a `FIXME` naming a ticket, as
the crate's other not-yet-admitted conditions are; it is not refused by design, because the exact step is
still a function of the signature.

On a magnitude-indexed grid the operands' quanta differ, the coarser is a multiple of the finer, and `r` is
a multiple of the finer. Under `toward_zero`, a dividend smaller than the divisor gives `q = 0` and `r = a`,
and a larger one gives `|r| < |b|` at no finer quantum than `b`'s, so `r` is a member. Under the nearest
modes the same, plus the case `|b| / 2 < |a| < |b|`, where `r = a -+ b` has `|r| < |a|` and is a multiple of
`a`'s quantum, so a member again. That is why IEEE 754 can say "the remainder is always exact". Under
`floor` and `ceil`, a small dividend of the opposite sign gives `r = a + b` with `|r|` near `|b|` at `a`'s
fine quantum, which needs more digits than the format has. The sketch checks all of this on toy floats.

### Why an integer quotient, and why the operation is arvo's

The integer is forced, not chosen. For `a - b q` to stay on a grid of quantum `Q` for every divisor, `b q`
must be a multiple of `Q` for every member `b`, which at the divisor `Q` itself forces `q` to be an integer.
A quotient on the operands' own grid at a negative exponent puts `b q` on the grid of `Q^2`, which is the
ticket's own observation; a quotient on a coarser grid, at a positive exponent, is a remainder modulo a
multiple of `b`. The integers are the one grid that keeps `r` on the operands' grid at every exponent.

The operation is arvo's because the rows say so and because the consumer cannot get it right from what
ships. `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` puts the arithmetic contracts
on the standards on arvo's own ground, and the two standards in scope each carry a remainder, IEEE 754 as a
required operation and MATLAB `fi` as `mod`. Option c would move that contract into every consumer, and
the sketch shows the obvious composition is wrong. Through the shipped quotient, `round_M(div::<S>(a, b))`
is the integer rounding for the directed modes at a negative exponent and is not for the nearest modes:
it misses exactly where the radix is even and the divisor's slot reaches `r^-e` at an odd value, because
the first rounding can move `a / b` onto the half, which is a grid point only at an even radix. With
`Q = 1/2`, IEEE's `remainder(1, 1.5)` is `-0.5`; through the grid quotient, `1 / 1.5` rounds to `0.5` on the
halves, then to `0` under `half_even`, and the composed remainder is `1`, which breaks law 2. At a positive
exponent every mode misses. And `a - (b q)` on the operands' signature fails under saturation for every
mode but `toward_zero`, because `b q` leaves the range where `r` does not: in a signed byte, `127 rem -128`
under `floor` has `q = -1` and `b q = 128`, which saturates to 127, giving 0 where the remainder is `-1`.
Wrapping gets it right, as the wrap is a ring map on the slots. A consumer who knows the slot identity of
law 1, avoids the double rounding, widens or wraps the intermediate and picks the divisor door again can
build it; four facts, each a place to be wrong, are the definition the ticket's option c would scatter.

The locus is the format, because the format is the tier that states every operation as an exact step and
one adaptation under a declared signature, and the value's operations are the format's by equality
(`mock/crates/arvo-numeral/DESIGN.md.tmpl`, "What this crate is"). How the format computes it inside is
internals, under `ruling::the_standard_is_parity_in_output_not_in_the_internals`: an implementation may
form the integer quotient on the value indices directly, or by the quotient onto an integer grid with the
product and difference widened, so long as the output is the one defined.

## The sketch, as its findings record

`mock/research/sketches/202611192130_the-remainder-on-the-grid/`, a standalone crate with its own
`[workspace]`, depending on nothing, every value an exact rational formed in it. Outcome: WORKS. It
unblocks the definition and region above, the refutation of the exponent as the dividing line, and the
refutation of the composition over the operands' signature. The harness refused a `FINDINGS.md` in the
sketch directory, so this section is that record, and the directory holds `src/main.rs`, `Cargo.toml`,
`Cargo.lock`, the passing run `out.txt` and one output per plant.

Hypothesis: the seven laws and the region table above, at the instances swept.

Attempt 1 stated the double-rounding claim as "the nearest modes miss at every negative exponent" and
stopped:

```
  radix 10 e -2: [(TowardZero, 0), (Floor, 0), (Ceil, 0), (HalfUp, 0), (HalfEven, 0), (StochasticDown, 0), (StochasticUp, 0)]

thread 'main' panicked at src/main.rs:446:17:
HalfUp at radix 10 e -2: expected the composition to miss
```

A miss needs `a / b` within `Q / 2` of a half without being one; the closest a non-tie gets is
`1 / (2 |s_b|)` at an odd divisor slot, so it needs an odd `|s_b| >= r^-e`, which widths up to six do not
reach at `10^-2`. The claim was restated and the widths taken to eight.

Attempt 2, with that condition, stopped at radix three:

```
thread 'main' panicked at src/main.rs:455:25:
assertion `left == right` failed: HalfUp at radix 3 e -2 over (0, 15)
  left: false
 right: true
```

At an odd radix the half is not a grid point but the midpoint of two, so the first rounding keeps the side
of the half and nothing misses. The condition became: an even radix, and an odd divisor slot with
`|s| >= r^-e`.

Attempt 3 is the committed source. Every claim held and every control fired. The three plants each fail
on the check planted against: `plant-quotient-on-the-grid` (a quotient in eighths) at "TowardZero: a
remainder off the grid at a whole phase"; `plant-sign-of-the-dividend` (floor's quotient rounded toward
zero) at "Floor: a remainder of the wrong sign"; `plant-float-checker-accepts-all` at "the membership
check accepts every sum, so it is blind". The numbers, from `out.txt`:

1. Whole phase (`whole_phase`, `main.rs:230`), radixes 2, 3 and 10, exponents -3 to 2, phases 0, 1 and
   -2, signed and unsigned widths one to five, 141876 pairs per mode: zero off the grid, zero
   disagreements with the value-index form, zero past the bound, zero of the wrong sign, zero leaving a
   range that holds the zero under `toward_zero` or `floor`, zero leaving a phase-zero signed range under
   any mode. The integer quotient is not a member of the operands' format at 53162 to 65562 pairs per mode.
   Option b's remainder agrees at every pair at exponent zero and phase zero and departs at 75200 to 85366
   pairs per mode overall. Unsigned ranges are left under `ceil` at 56070 pairs, `half_up` 27252,
   `half_even` 25092.
2. The quotient's span (`quotient_span`, `main.rs:349`): `max |q|` is `2^(w-1)` signed and `2^w - 1`
   unsigned at widths one to six, one value across every radix and exponent; the `Fix<8, -4>` example as
   above.
3. Fractional phase (`fractional_phase`, `main.rs:390`), phases 1/2, -1/2, 1/3, -2/3, 3/4, 1/5, radixes
   2 and 10, exponents -2, 0, 1: on the grid at 254208 mode-pairs, off at 433248, on exactly where `d | q`,
   every format with a pair off, the exact remainder always on the lattice of quantum `Q / d`.
4. Double rounding (`double_rounding`, `main.rs:434`), widths one to eight: zero misses for the directed
   modes at every negative exponent; for `half_up` and `half_even`, 15470 and 15476 over a signed byte at
   `2^-1`, 56 and 56 at `10^-2`, none at `3^-2` at any width, and zero exactly where the condition says;
   at exponent one every mode misses at tens of thousands of pairs.
5. Composition on the operands' signature (`composed_on_the_signature`, `main.rs:496`): under saturation
   `toward_zero` 0 wrong, `floor` 464, `ceil` 360, `half_up` 196, `half_even` 220, counted only at pairs
   whose true remainder is a member; under wrapping 0 wrong for every mode.
6. Euclid (`euclid`, `main.rs:543`): 3906 pairs, the floor's remainder by a positive divisor and the
   ceiling's by a negative one, never negative.
7. Toy floats (`magnitude_indexed`, `main.rs:587`), radix 2 at precisions 2 to 4, radix 10 at 1 and 2,
   radix 3 at 2, subnormals included: `toward_zero`, `half_up` and `half_even` give a member at every pair;
   `floor` and `ceil` miss (992 pairs each at radix 2, precision 4; 46620 each at radix 10, precision 2).
   The control finds non-member sums in every format.

What it does not establish: the sweeps are instances, and the general statements rest on the derivations
above. It runs no arvo step, so the shipped machinery's four-limb bounds, its dither rule for
`stochastic` (swept here only as its two outcomes) and width 64 are untested. The toy floats carry no
infinity, no signed zero and no overflow.

## The arguments against this answer, and how they fall

The strongest is that `S`'s mode is a coordinate of the adaptation onto `S`, and reusing it as the
integer quotient's direction overloads it. It falls at a whole phase and stands at a fractional one, and
that is exactly where the region is cut. At a whole phase `r` lands on the grid, so in this operation the
mode has one job, and it is the same job it does in the quotient: round the exact `a / b`, there onto the
grid of `Q` and here onto the grid of one. At a fractional phase the landing rounds too, the one mode
would be read for two roundings, and that is why the region is held out rather than admitted.

Second, that both standards fix the direction independently of their rounding attribute, so the direction
is not a signature coordinate. Parity is in output: the alias fixes the direction by crossing to a sibling
signature with `under`, which moves nothing, and the output is the standard's whatever the value's own mode
is. A standard's global rounding attribute is ambient state, which arvo does not have; its mode is per
signature and per call, so there is no attribute to be independent of.

Third, option b's caution: ship the integers now and the fractional formats when a use is known. The
remainder's slot reads neither the radix nor the exponent (law 1, and the sweep), so a refusal at a
fractional exponent refuses a thing identical to an admitted one by a coordinate the operation does not
read. The admission rule admits it, and refusing what the rule admits needs a canon ground nobody has
cited. The known use is already named by the ratified standards bound: MATLAB's `mod` on a `fi` with a
positive fraction length, and IEEE's remainder at every exponent. And `ruling::arvo_is_a_library_and_the_value_composes_on_top`
says in op's words that how end users will use it is unknown, so a known use is not arvo's gate for an
operation; `ruling::the_ingest_door_waits_on_a_known_use` is about a runtime door, not an operation, and
does not transfer. Option b's construction, `q = div::<S>(a, b)`, is also correct only at exponent zero
(law 6); extended to a scaled integer at a positive exponent it is a remainder modulo a multiple of `b`.

Fourth, option c under the parity ruling: if the output matches, a composition is enough. The ruling makes
the route free for arvo, not for the consumer, and `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`
puts the contract on arvo's ground. The composition the ticket names, the shipped product and difference on
the operands' signature, is wrong under saturation and wrong through the shipped quotient (sketch, items 4
and 5). A marked hole for one standard is honest under the parity ruling; "no remainder in arvo" is not a
marked hole, it is a framework position.

Fifth, that the pair's quotient is not a member of `S`, so the pair is not `V x V -> V`. True, and that is
what `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim` says such a thing
is: a composition with its own law. The remainder alone is a format operation; the pair is the remainder
and an integer landed on a second declared signature, and its law (`a = b q + r` where neither policy
acted) is stated on the pair.

Sixth, that a remainder completed by the policy on an unsigned range is garbage (`3 rem 4` under
`half_even` is `-1`, saturating to 0 or wrapping to the top). It is what the spine says an operation is, and
`fits_rem` reports it. Refusing `ceil` or the nearest modes on unsigned ranges at build would decide a
policy for the consumer, which the workspace's toolbox rule forbids; the combination is legal and its
consequence is visible.

Seventh, that a stochastic remainder is meaningless. It is a function of the signature and the dither, as
the stochastic quotient is, and its value is one of two exact remainders; the admission rule admits it,
and the magnitude-indexed region is where it differs (it can leave a float format, like `floor` and `ceil`).

## What the answer requires, by crate

`arvo-format`. A section "The remainder, by the integer quotient" beside the quotient in `DESIGN.md.tmpl`,
stating the definition, the seven laws and the region. An operation `Rem<S>` of arity two; an exact step,
`remainder_position`, forming `k_a - k_b round_M(k_a / k_b) - n` from the slots and the reduced phase, which
reads neither radix nor exponent, so its bound is on the value indices alone and no power of the radix
enters it; `rem::<S>`, the step composed with `adapt` under `S` with the dither passed through; and a
verdict `is_remainderable` over one array of conditions with an obligation that cannot disagree with it, in
the crate's existing pattern: the phase denotes (refused by design), the phase is whole (not yet admitted,
with a `FIXME` naming a ticket filed for the fractional-phase remainder), the quantum is constant (not yet
admitted, under `arvo_quotient_and_conversion_over_a_magnitude_indexed_quantu`), the step is formed. The
divisor is the quotient's `Divisor`, unchanged. The pair as `divrem::<S, T>` with `T`'s grid holding the
integers refused at build otherwise, and a verdict on whether `T`'s range holds the whole span of `q`. Under
`standards`, the MATLAB `fi` `mod` alias at `floor` with a parity test against MathWorks' documented
definition at positive fraction lengths, and the IEEE 754 `remainder` alias at `half_even`, marked with the
magnitude ticket's `FIXME` until floating formats divide. Tests owed: every member pair at widths one to
eight against an oracle forming `a - b round_M(a / b)` in rationals, every mode and shipped policy, both
families, exponents on both sides of zero, radixes two, three and ten, whole phases zero and non-zero; the
sign, bound and closure laws asserted as laws; a planted remainder from the grid's own quotient that must be
reported away from exponent zero and must agree at it; the Euclidean alias at both divisor signs; compile-fail
pairs for a half-offset phase beside a whole one and a magnitude-indexed quantum beside a constant one; an
edge sweep at widths nine to 64 at the corners, `MIN rem -1` among them.

`arvo-numeral`. `rem` and `fits_rem` on `Num`, each equal to the format's on every member and every
divisor, with the divisor the re-exported `Divisor`; the pair taking a second value type `T: Sole` for its
quotient; nothing stated a second time. The two sentences that say the remainder is not here go
(`DESIGN.md.tmpl:342` and `:549`). The fixed-point aliases stay keyed on the consumer's declared rounding,
as `obligation::a_deterministic_fixed_point_lane_with_declared_rounding` asks; a consumer wanting `fmod`
beside a `half_even` lane crosses with `under`.

No other crate. The remainder needs nothing of `arvo-placement` or `arvo-bits`, and it is not an algorithm
crate's.

## What is left open

The fractional-phase remainder. I found two candidate definitions and no canon row that chooses between
them: round `r` onto the format (one mode read twice), or restrict the quotient to multiples of `d` (a
remainder modulo `d b`). A third, landing the exact remainder on a caller-named signature on the value-index
lattice, works only where `d` divides a power of the radix. No standard in scope has a phased grid, so no
parity test bears on it. It needs either a second rounding coordinate in the signature or a use that
prefers one candidate.

The magnitude-indexed remainder's exact region is established here by derivation and a toy sweep; what
arvo's floating format does at `floor` and `ceil`, where `r` is not a member, waits on the same ticket as
the quotient. IEEE's rule that a zero remainder carries the sign of `x` has no member to land on, since
arvo has no signed zero; it is a marked hole in the IEEE alias.

A remainder whose quotient rounds ties away from zero is not reachable by any of the six modes, and no
standard in scope asks for one. MATLAB's `rem` on a `fi` I could not confirm from the documentation I read.

The pair's quotient landing on `T` by `T`'s policy is my reading of an operation's landing, not a crossing;
if the other seat reads it as a lossy crossing, `question::whose_reduction_governs_a_lossy_crossing`
governs the region where `q` leaves `T` and that region waits on it.

Whether `remquo`'s convention (the sign and low bits of `q`) is reachable as a `T` under wrapping, sign and
magnitude rather than two's complement, I did not work out.

And the two-expert rule: this is one read. The other seat's independent answer, and a reconciliation that
starts from neither, are owed before any of this is filed as a proposal.

Sources consulted outside the repository: the MathWorks page for `mod` on `fi` objects,
https://www.mathworks.com/help/fixedpoint/ref/mod.html, for the documented definition and the output type.
