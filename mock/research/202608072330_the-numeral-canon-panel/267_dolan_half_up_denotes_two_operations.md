# Seat 267: `half_up` denotes two operations, and the canon already says so

## Canon gate

`mockspace.toml` names `canon_paths = ["mock/registry/*.toml"]`. The question,
`question::which_operation_half_up_denotes`, is a filed panel question with
`decider = "panel"`, aligned with the canon process for resolving an
underspecified name in the ratified six-mode vocabulary. Nothing in the
question conflicts with a ratified row; it asks the panel to settle what a
ratified row's own name denotes. Proceeding.

## Phase one: derivation, before reading seat 229

### The governing row decides the shape before any measurement does

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`
(`ratified_by = "op"`) says:

> The parity suite is built against MATLAB `fi`/`fimath` and IEEE 754.

`ruling::the_standard_is_parity_in_output_not_in_the_internals`
(`ratified_by = "op"`) says:

> The standards bound is met by agreeing with what a standard documents as its
> result, for the cases it covers.

Both are ratified and both bind before any question about `half_up`'s
denotation is asked. So the test for which reading `half_up` should carry is
not a preference between two conventions. It is: does the ratified name, read
either way, reproduce what MATLAB `fi` and IEEE 754 document as their result,
for the cases they cover.

**MATLAB `fi`/`fimath` documents six `RoundingMethod` values, and two of them
are exactly the two readings the question poses, under two different names.**
Checked against MathWorks' own documentation (`mathworks.com/help/fixedpoint`),
2026-09-18:

> `Nearest` — Round to the nearest representable value. ... The nearest
> function rounds ties to the nearest integer toward positive infinity.
>
> `Round` — Round to the closest representable number with ties rounded based
> on the sign of the value. ... The round function rounds ties to the nearest
> integer with greater absolute value.

("toward positive infinity" is reading 1 of the question; "greater absolute
value" on a signed domain is reading 2, ties away from zero.) MATLAB does not
have one name for this family. It has two, `Nearest` and `Round`, because the
two operations disagree at every negative tie and MathWorks needed both
addressable.

**IEEE 754 agrees with the split from its own side.** IEEE 754-2008 names five
rounding-direction attributes: `roundTiesToEven` (the required default),
`roundTiesToAway`, and the three directed roundings. `roundTiesToAway` is
defined as:

> ties are rounded up towards positive infinity for positive values, and for
> negative values, ties are rounded down towards negative infinity

which is reading 2 (magnitude increases on every tie, symmetric about zero) and
is IEEE's own name for it. IEEE 754 has no rounding-direction attribute at all
for reading 1 (ties toward positive infinity regardless of sign); that
convention does not appear in the standard under any name.

So: reading 1 is a real, standard-documented operation (MATLAB `Nearest`, no
IEEE 754 name). Reading 2 is a real, standard-documented operation (MATLAB
`Round`, IEEE `roundTiesToAway`). Neither standard treats them as the same
operation under different words; both standards that the canon is already
bound to reproduce treat them as two operations, and one of MATLAB's own two
names for this pair is spelled differently from `half_up` in either direction.

**This is the governing constraint, and it decides the question's shape before
any measurement of arvo's own code does.** A single arvo name `half_up`
denoting only one of the two readings cannot reproduce, for the cases MATLAB
`fi` covers, whichever of `Nearest` or `Round` it does not implement. The
standards bound is violated by construction the moment `half_up` is asked to
be one operation. `never-ask-which-single-rule-governs.md`: the question as
filed offers three options as if they were competing single answers, but the
third option (two names, one per reading) is not one candidate among equals,
it is the only one of the three that satisfies a ratified row that already
binds. Option 1 and option 2 are each a region where the *other* standard
operation goes unnamed and therefore unreachable under the ratified
vocabulary, which is not a naming inconvenience, it is a standards-parity gap
against rows `ratified_by = "op"`.

**Answer: option 3's shape, restated as arms rather than a pick.** `half_up`
does not get redefined to one reading; the vocabulary needs both readings each
under its own name, because both are standard-documented operations arvo is
already bound to reproduce. Which of the two keeps the name `half_up` is a
second, smaller decision (below), and it is not forced by the standards bound
alone.

### My own probe: the two readings are mathematically distinct operations, not a documentation ambiguity

Built independent of seat 229's probes (not opened at this point):
`267_probes/a_equivariance_proof.rs`, run and output committed at
`267_probes/a_output.txt`.

- **Reading 1 is `floor(x + 1/2)`**, an unconditional closed-form identity:
  `floor(y + k) = floor(y) + k` for every integer `k`, so reading 1 is
  *exactly* translation equivariant, everywhere, not just on the swept domain.
  The sweep (signed 6-bit container, `fraction_width` 0..=5, shifts -4..=4)
  found 35,775 checks, 0 failures.
- **Reading 2 (ties away from zero)** is not translation equivariant: 100
  failures out of 35,775 checks on the identical domain, all at fraction
  widths carrying a representable tie (`fraction_width >= 1`).
- **Control**: `half_even`, which the fma law's own `fails` region already
  lists as non-equivariant, fails 1,260 of 35,775 checks on the same harness,
  proving the check can report a failure rather than passing everything
  vacuously.
- **The two readings differ at every one of 160 negative ties checked**
  (`fraction_width` in {1..5}, every representable tie in the signed 6-bit
  domain), confirming the question's own claim that they differ at every
  negative tie.

`holds for: total_width = 6, fraction_width in 0..=5, signedness = signed,
target_features any (the equivariance identity is architecture-independent),
rounding = the two named readings of half_up plus half_even as control`. The
translation-equivariance identity for reading 1 is a closed-form proof and
holds unconditionally over width and target; the sweep is a check on the
proof, not the source of it.

### This matches, and exposes a live contradiction in, the ratified/measured law row

`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` has
`holds` naming `rounding: in {floor, ceil, half_up}` (the
translation-equivariant set) and `fails` naming
`rounding: in {toward_zero, away from zero, half_even}`. My probe reproduces
this partition exactly under reading 1 and breaks it under reading 2, which is
precisely what the question's own option 2 warns of: read `half_up` as
ties-away-from-zero and the law's `holds` region shrinks from three modes to
two.

**But `arvo-format`'s shipped `Mode::HalfUp` is reading 2**, not reading 1.
`mock/crates/arvo-format/src/apply/tests/mod.rs:268-279`
(`half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot`)
asserts a tie at `-2.5` rounds to `-3` under `Mode::HalfUp`, magnitude
increasing, sign preserved: away from zero. So the measured law row and the
shipped crate already disagree about what `half_up` computes: the law was
measured against seat 226's own probe implementation (reading 1, per the
question row's own note), and the crate that ships under the same name
computes reading 2. **This is a standing defect independent of which way the
naming question is answered**: today,
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` makes
a claim about `Mode::HalfUp` that `Mode::HalfUp` does not satisfy.

### The second-order decision: which reading keeps the name `half_up`

Not forced by the standards bound (either name could denote either reading);
forced by cost, and the costs are asymmetric:

- **Give `half_up` reading 1** (ties toward +inf, matching MATLAB `Nearest`).
  The measured law row needs no correction; it already describes reading 1.
  Cost: `arvo-format`'s shipped `Mode::HalfUp` is currently reading 2 and has
  to change, and its own test at `mod.rs:268` has to be rewritten (the tie at
  `-2.5` would then round to `-2`, not `-3`), and the new operation (reading 2)
  needs a new name and a re-measurement of the law region it moves into.
- **Give `half_up` reading 2** (ties away from zero, matching MATLAB `Round`
  and IEEE `roundTiesToAway`). The shipped crate is already correct and needs
  no change. Cost: the measured law row is wrong as written today and needs
  correcting (`half_up` drops out of `holds`, per the question row's own
  option 2 text), and the new operation (reading 1) needs a new name, under
  which the translation-equivariant identity gets re-established as a law for
  that new name rather than for `half_up`.

Both costs are real and neither is free; I do not pick between them, since
which one costs less is a question about how much else in the corpus already
assumes one reading of `half_up` by name, which is a census I have not run and
is properly the panel's or op's to weigh against the shipped-code cost versus
the measured-law cost. What I can say from the governing row alone: **whichever
reading loses the name `half_up`, both readings need a name**, because both are
standards-documented operations the canon is already bound to reproduce.

## Committing phase one

Committed here, before opening `229_*`.
