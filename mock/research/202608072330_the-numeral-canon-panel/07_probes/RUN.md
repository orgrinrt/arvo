# 07_probes: every command, its exit code, and what it produced

Toolchain pin verified before anything ran:

    $ cat rust-toolchain.toml
    [toolchain]
    channel = "nightly-2026-05-28"
    components = ["rustfmt", "clippy", "rust-src"]
    $ rustc +nightly-2026-05-28 --version
    rustc 1.98.0-nightly (57d06900f 2026-05-27)
    $ python3 --version
    Python 3.14.6

`rustc` is always invoked with the explicit `+nightly-2026-05-28`, because a bare
`rustc` outside the repository tree resolves to stable.

## The brief's factual claim, checked before reasoning from it

The brief states that this panel and its predecessor never used the adjunction
frame, and asks for that to be verified rather than accepted.  Counts only; the
predecessor tree's file contents were not read.

    $ cd mock/research/202608072330_the-numeral-canon-panel
    $ for t in Galois adjunc adjoint "abstract interpretation" "closure operator" \
               coreflection comonad monad residuat concretis concretiz; do
        printf '%-26s %s\n' "$t" "$(grep -ciE "$t" CANON_CANDIDATE.md)"; done
    # every term: 0

    $ grep -rciE "<term>" seed/            # every term: 0
    $ D=../202607301300_formalization-spec-panel
    $ ls $D | wc -l
    320
    $ for t in ...; do grep -rliE "$t" $D | wc -l; done
    # every term: files=0 hits=0

    $ D=../202607281616_prior_art
    $ grep -rniE "abstract interpretation" $D
    02_number_systems_and_fixed_point.md:415: ... abstract interpretation to find the
    minimal input precision an embedded computation can tolerate.   [one hit, citing
    an external tool (POP) in a prior-art survey, not used as a frame for the design]

So the claim holds.  320 files, zero hits on every term, and the single prior-art
mention is a citation of somebody else's tool.

## The test gate

Re-run rather than inherited.  Same three zeros `06` reports, arrived at
separately, plus two of my own on the vocabulary this file introduces.

    $ find mock/crates -path '*tests*' -name '*.rs' | wc -l
    91
    $ grep -rl '#\[test\]' mock/crates --include='*.rs' | wc -l
    83
    $ grep -rniE 'galois|adjoint|postfix|post_fix|Sufficient' mock/crates --include='*.rs' | wc -l
    0

There is no suite to audit for this question, because the surface has no source.
The suite was not run and this file does not imply that it passed.

## p1: is a rounding mode an adjoint to the embedding

    $ python3 p1_rounding_adjunction.py > p1.out
    exit=0

23 numeral shapes, 34,976 (x, v) pairs in range and 40,096 with out-of-range
values present.  Headline: in range, `toward +inf` satisfies the lower-adjoint
biconditional at 0 failures and `toward -inf` the upper-adjoint one at 0; all
four other modes fail both, with witnesses.  With out-of-range values admitted
and clamped, both directed modes fail at 184 each.

## p2: the Moore condition, by enumeration in a box

    $ python3 p2_moore_family.py > p2.out
    exit=0

Q1 and Q3 are the load-bearing halves and they agree exactly: at the strict
policy 36 of 351 pairs have an intersection that is not a value set, and 36 of
351 have no meet; admitting the origin takes both to 0.

**Q2 and Q4 of this instrument are dominated by box truncation** and are reported
as such rather than deleted: 2,523,262 of 2,796,636 "no least containing numeral"
answers in Q2 are sets whose answer lies above the box ceiling, and Q4's "no best"
column is the same artifact.  p3 replaces both by removing the ceiling.  The
instrument is kept because the method that had to be replaced is what licenses
the replacement.

## p3: the best abstraction in closed form

    $ python3 p3_best_abstraction_closed_form.py        # KILLED after 25 minutes
    $ pkill -f p3_best_abstraction
    $ python3 p3_best_abstraction_closed_form.py 4 > p3.out
    exit=0

The default Q1 box (wmax 6) did not finish: its three-subset enumeration is cubic
in a universe of several hundred rationals.  It is left in the file unchanged and
the box actually run is a command-line argument, so the file records both what was
attempted and what was measured.

Results at wmax 4: closed form against enumeration, **0 disagreements** over 3,163
comparable sets.  Codomain over 2,796,636 sets: the origin is needed by exactly 1,
`I < 0` by 5,487, both at once by 0.  The join equals the closed form applied to
the union at 120 of 120 comparable pairs, 0 differ.  Multiplication over 400
operand pairs: the sum-of-widths form is the best abstraction at 324 and
overshoots at 76; **file 06's corrected form is the best abstraction at 400 of
400**.

The 400-of-400 is against the raw codomain with no admission policy applied,
where `06` compared against the admitted space with `I` floored at zero, which is
why `06` reports 15 residuals and this reports none.  The two are consistent and
the difference is the floor.

## p4: composition, and what each mode is actually adjoint to

    $ python3 p4_composition_and_forced_adjoint.py > p4.out
    exit=0

Q1 came out the opposite way from the prediction and the appendix that follows it
was written after that.  **Every one of the six modes is a lower adjoint to
something**, at 0 biconditional failures against its own forced upper map, because
the forced candidate works exactly when the map is monotone and all six are.  Only
`toward +inf` has the embedding as that upper map.

Q1b prints the gap `g'(v) - v` per mode: 0 for `toward +inf`, a full cell for the
three other directed modes, half a cell for the two nearest modes.

Q2 and Q2b: on nested grids the four directed modes compose at 0 failures and both
nearest modes fail; on non-nested grids everything fails.  Q2b repeats it over a
signed range so `toward zero` stops coinciding with `floor`, and it still composes,
**which refutes the prediction that only the two infinity-directed modes would**.

Q4 is the decisive test for the refined claim, added after that refutation.  A
family of "away from pivot p" modes: composition fails at 0 when the pivot is a
point of the coarse grid (three pivots tried) and at 7 when it is not (four
pivots tried).

## p5: the accumulator as a post-fixpoint

    $ python3 p5_postfixpoint_accumulator.py > p5.out
    exit=0

Q1, monotonicity of the resolved step: saturating 0 violations at three shapes;
wrapping and substitute-zero both fail at identical counts (84 of 224, 84 of 224,
680 of 1920).

Q2 and Q3, soundness of an n-step fold: under the point reading, all three
resolutions are unsound and saturating is no better than wrapping.  Under the
absorbing reading, where the top denotes "at least this", **saturating is sound at
0 failures for every n from 1 to 8, over 65,536 sequences at n=8**, while wrapping
and substitute-zero grow to 55,085 and 56,313 failures at n=8.

Q4 is the decisive one, added after Q3: sweeping the accumulator's fraction width
against the element's gives 0 unsound sequences on and above the diagonal
`F_A >= F_P` and nonzero strictly below, at every cell.

## p6: the sufficiency condition as a bound, and erasure

    $ mkdir -p out
    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O \
        p6_sufficiency_as_a_bound.rs --out-dir out
    exit=0        # gate-free, no feature attributes anywhere in the file

    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O \
        p6_negctl_arm_b.rs --out-dir out
    error[E0277]: the trait bound `Sk<Sk<Z>>: Le<Z>` is not satisfied
      --> p6_negctl_arm_b.rs:13:18
    note: required for `U<3, Sk<Z>>` to implement `GridSufficientFor<U<1, Sk<Sk<Sk<Z>>>>>`

    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O \
        p6_negctl_arm_a.rs --out-dir out
    error[E0080]: evaluation panicked: accumulator grid is coarser than the element grid
    note: the above error was encountered while instantiating
          `fn fold_arm_a::<U<3, Sk<Z>>, U<1, Sk<Sk<Sk<Z>>>>>`

Both negative controls refuse, at different moments: arm B at type-check naming
the relation, arm A during monomorphisation.

Erasure:

    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -O -C codegen-units=1 \
        --emit=asm p6_sufficiency_as_a_bound.rs --out-dir out
    $ python3 cmp_asm.py out/p6_sufficiency_as_a_bound.s | tee p6_asm.out
    ...call_b_equal   10
    ...fold_bare      10
    identical to fold_bare after label normalisation: True  <- ...call_b_equal
    aliased symbols:
      ...call_a = ...call_b_equal
      ...call_b = ...call_b_equal

**The first version of that comparison reported False**, because it compared raw
text and `LBB0_3` against `LBB1_3` read as a difference.  Local label indices are
allocated per function and carry no content.  The wrongness is named in
`cmp_asm.py`'s own docstring rather than quietly fixed, on the same footing as the
killed instruments above.

## p7: what a best abstraction across kinds would cost

    $ python3 p7_moore_completion_cost.py > p7.out
    exit=0

The fixed-point family with the origin admitted is already closed under
intersection at 0 added sets.  The union of a fixed-point family and a float
family is not: the closure adds 2 to 53 sets depending on the box, **none of them
named by either family**, and every one a segmented grid.  Swept against the float
exponent span, the ratio of added to original settles between 0.16 and 0.34, so
the completion is a constant-factor enlargement rather than an explosion.

`03`'s witness is reproduced independently: `U<0,1>` against `U<2,0>` has exactly
two minimal upper bounds across both kinds, `U<2,1>` at 8 values and the float at
precision 2, exponents -1 to 1, at 7 values, and the union itself is named by
neither family.

## Every number in this directory is a count

No bench harness run bears on anything here.  The `p6` assembly read is an
existence claim about erasure, not a measurement.  Where magnitude would be the
question, the word is **unpriced**.
