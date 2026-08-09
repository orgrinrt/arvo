# Probes for panel file 37, which relation

Seven `.rs` artifacts and one compile-time sweep, all against the workspace pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with `rustc --version` from inside the repo
(file 36's methodological note about resolving to stable 1.94 from outside the repo applies and was
observed). Build command for every probe:

```
rustc --edition 2021 --crate-type lib <file> --out-dir <tmp>
```

Three of the seven are committed refusing, on purpose, and their refusal is the claim.

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_ladder_is_a_view_lattice.rs` | Are the three relations three relations, or one relation at three settings of one parameter, and is that parameter's domain a chain? | WORKS, 65s of const eval. One relation, one parameter, and the domain is NOT a chain. Nine views (Ignore / Presence / Exact per generator class) over an exhaustive four-element fold on a signed three-bit numeral, all five groupings, all pairs, all 4096 inputs, nine compositions. The holding set is downward closed AND join closed at every composition, so every law has a unique finest view (asserted, not assumed). Four distinct finest views are realised: interior-safe `Precise` at (Exact, Exact); `Hot` signed wrapping at (Exact, Ignore); `Precise` below interior safety at (Ignore, Exact); refuse-top/wrap-bottom at (Ignore, Ignore). `Warm` saturating and `SubstituteZero` hold under no view at all. Interior safety sends the finest view to the top for EVERY resolution shape, not only the refusing one: `Warm` and `Hot` with a sufficient accumulator both reach (Exact, Exact), which is what makes a preset table's at-interior-safety column uniform rather than extrapolated from one row. Kleene and (Ignore, Exact) are INCOMPARABLE: `Hot` satisfies the first and fails the second, `Precise` is the reverse. |
| `probe_2_reification_preserves_no_view.rs` | Is file 34's reification lemma (`34:176-190`, "the one relation invariant under the Refuse-to-special reification is the graded one") true in general? | WORKS, and the lemma is true of one reifier and false in general. Under an out-of-set absorbing special, every view keeping any cause information IS preserved pointwise (Kleene and graded both), and the weak equation is not: that is file 34's finding, confirmed, and it is narrower than what it claimed, since Kleene is preserved too. Under `SubstituteZero`, one of the design's own four `Resolution` instances, NO view is preserved, graded included, all nine. Witness at (3, 3, 1): under `Refuse` both groupings refuse with one cause each so every view holds; under `SubstituteZero` the left delivers 1 and the right delivers 3, and the grades are IDENTICAL on both sides, so the graded relation had all the information it needed and still flipped. The stable hypothesis is about the reifying element (outside the value set AND absorbing), never about the relation. |
| `probe_3_event_invariance_is_keyed_on_domain.rs` | Is wrapping addition graded-associative? | WORKS, and no, on a signed numeral. This refuted my own prediction before I had written a line of the report. The counting argument (each addition reduces at most once and by exactly the modulus, so the reduction count is the exact total over the modulus and is grouping-independent) is sound and needs the wrap to be ONE-DIRECTIONAL. Asserted exhaustively on an unsigned numeral: the count equals `total / MODULUS` at every grouping, and unsigned wrapping is graded-associative. On a signed numeral both ends reduce, reductions cancel in the value and both are counted, and the witness is (-4, -3, 3): same value, two events against none. So `ReduceModulo`'s value and definedness components are `Domain`-independent and its event component is not, which is one law whose components read different parts of the key. |
| `probe_4_view_as_a_return_type_and_the_transfer.rs` | Can the mechanism be one const fn returning a lattice element instead of three derived marker traits, and can a regrouping publish what its law fails to preserve rather than waiving it? | WORKS, after the compiler killed my first design. All six compositions compile at their own published grade, with no unstable feature and no `generic_const_exprs`. FIRST DRAFT REFUSED: it had the consumer declare a required view, checked the law against it, AND published a deficit, and two of its own call sites died with `error[E0080]: evaluation panicked: this composition's fold law does not hold at the required view`. The two halves were fighting, because the licence check refuses exactly the case the transfer exists to handle. The repair removed a parameter: where the WEAK equation fails the values diverge and no publication rescues anything, so that is a hard refusal; everywhere else the regrouping is always sound and what it must publish is DERIVED, not requested. No consumer declares a view, so there is no consumer-supplied index left to be too rich. |
| `probe_4b_no_law_at_any_view_refuses.rs` | REFUSING, on purpose. | `error[E0080]: evaluation panicked: this composition's fold has no associativity law at any view: regrouping it changes the delivered value, so no published grade makes the regrouping honest. Widen the accumulator until the fold is interior-safe, or do not regroup.` Saturating (`Clamp` both ends, the shipped `Warm`/`Cold` shape). |
| `probe_4c_understated_grade_refuses.rs` | REFUSING, on purpose. | `error[E0080]: evaluation panicked: a regrouping must publish every grade generator class its law does not preserve: tolerance is a transfer, never a waiver.` `Precise` below interior safety declaring `Folded<0>`. |
| `probe_4d_caller_contract_refuses.rs` | REFUSING, on purpose. | `error[E0308]: mismatched types ... expected `0`, found `1` ... expected struct `Folded<0>` found struct `Folded<1>`.` A caller needing a definedness-faithful fold, handed the `Precise` regrouping. Plain type checking, no bespoke machinery: the coeffect discharged into an effect. |
| `price/` (`gen.py`, `run.sh`, `results.csv`) | What does the mechanism cost against the shape file 33 proposes? | MEASURED, and the first comparison was unfair to my own proposal's favour in one direction and against it in another. See the table below. |

## The price

Build shape `rustc --edition 2021 --crate-type lib --emit=metadata`, which is type checking, trait
selection and const eval with no codegen, and that is the honest shape because both mechanisms are
entirely compile-time. Counts 0, 50, 100, 200, 400 distinct compositions, min of three runs,
`count = 0` subtracted as fixed cost. Scaling is linear across the whole range for all three shapes,
so the 400-point figures are slopes rather than a knee. Run-to-run noise between two full sweeps was
2 to 3 percent.

| shape | ms per composition | metadata bytes per composition |
|---|---|---|
| A: five marker traits, impls ASSERTED | 0.060 | 785 |
| B: one const fn returning the finest view | 0.130 | 907 |
| C: five marker traits, impls DERIVED | 0.193 | 1854 |

Shape A is the cheapest and it is the shape D51 forbids. A marker impl is a claim about a
composition, and nothing in shape A checks that `impl EventExact<Add> for C7` is true of `C7`; D51
rules that law markers are derived and that a derived property cannot lie, which is why it is a plain
safe impl rather than an `unsafe impl` (`33:428-430`). Deriving the marker means computing the view,
which is shape B, and then carrying the impls on top, which is shape C.

So the comparison that matters is B against C: **1.48x cheaper in compile time and 2.04x smaller in
metadata, while expressing nine views where C's five markers span thirty-two combinations of which
nine mean anything.** Shape B is not an alternative to the derivation; it IS the derivation, with the
marker layer removed rather than added.

Calibration, because an absolute number means nothing alone: 0.130 ms per composition is two orders
of magnitude below file 36's 5.08 ms per composition for the type-level gcd. The law mechanism is not
where this design's compile time goes. This prices the mechanism SHAPE and is a neighbour rather than
an answer to `26:668-674`, which asks for the cost against a real consumer's composition set; the
same distinction file 36 drew about its own numbers (`36:470-471`).

## The two refusals that changed something

Kept in the probe headers rather than only here, because a probe that only ever passed is not
evidence that it was checking anything.

**Probe 1's parameter domain was wrong the first time.** I had it as the set of grade generators a
consumer tolerates, a subset lattice. Under that domain the holding family is not closed under meet,
so a law can have several incomparable minimal answers and "the law's content" is not one object. The
fix is that the parameter is a QUOTIENT of the grade rather than a subset of its generators, because
Kleene equality collapses cause multiplicities to a boolean rather than dropping them, and a
collapse is not a projection. Once the domain is quotients the family closes under pullback and the
unique finest view exists. CLAIM A asserts that closure rather than assuming it.

**Probe 4's mechanism had two halves fighting.** Recorded above and in the probe header, with the
verbatim E0080. The repair made the mechanism smaller by one const parameter and removed the only
place a consumer could have declared a permission, which is the place the review has already measured
a permission-shaped fact going undetected when corrupted (`26:213-215`, and the `ViewC` droplist entry
at `26:735-742`).

## What is not here

No runtime timing claim of any kind. Everything above is compile-time or exhaustive const evaluation;
the emitted-code question for a regrouping combinator belongs in `mock/benches/` per
`bench-and-sketch-discipline.md` and is not attempted.

No probe covers multiplication or division. The view mechanism is stated over one operation family
and the atom set's survival past addition is already a standing open item (`26:676-681`).
