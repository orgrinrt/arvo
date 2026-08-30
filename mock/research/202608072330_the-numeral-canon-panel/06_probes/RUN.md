# 06_probes: every command, its exit code, and what it established

Seven instruments, in two languages, by three independent methods: exact-rational
enumeration in Python, a closed-form analytic path cross-checked against that
enumeration, and type-level folds compiled by rustc. Everything here is a spike.
Cite it for what it proved, never for how it was written.

**Toolchain.** Pinned, and passed explicitly on every invocation because a bare
`rustc` outside the tree resolves to stable.

    $ cat rust-toolchain.toml
    [toolchain]
    channel = "nightly-2026-05-28"
    components = ["rustfmt", "clippy", "rust-src"]

    $ rustc +nightly-2026-05-28 --version
    rustc 1.98.0-nightly (57d06900f 2026-05-27)
    EXIT=0

**Feature gates used across all Rust instruments: zero.** No
`generic_const_exprs`, no `generic_const_args`, no `min_generic_const_args`, no
`specialization` of either kind, no `adt_const_params`, no const-traits family.
No `-Znext-solver=globally`. No `TypeId`, no `dyn`, no `alloc`. Every file is
`#![no_std]`. Checked rather than asserted:

    $ grep -c 'feature(' p3_formula_vs_extremum.rs p4_cross_family_join.rs p7_tight_form_in_typestate.rs
    0 0 0

The three matches for the forbidden-construct grep in `p3` are all in its header
comment, which names what the file does not use. Verified at
`p3_formula_vs_extremum.rs:16-18`.

---

## p1_site_enumeration.py

**What it asks.** Per arithmetic site: is the result numeral a total function of
the members, is it the least numeral containing the exact result, and is it the
join of the operands?

    $ python3 p1_site_enumeration.py > p1.out
    EXIT=0

**Established.** Inclusion in the unsigned fixed-point family at radix 2 with
zero bias is the componentwise order on `(I, F)`, at 0 disagreements against set
inclusion. The product numeral differs from the join at 1175 of 1296 pairs, and
every one of the 121 coincidences has `min(I1,I2) = 0` and `min(F1,F2) = 0`. The
additive formula equals the join at 0 of 1296. The formulas overshoot: mul_full
tight at 1099/1296, add at 1175/1296, the subtraction candidate at 751/1296.

**A cost note that is part of the record.** The first version of this file did
the least-containing search by enumerating every value set against a box of
shapes. It did not finish in ten minutes and was killed. The slow method is kept
in the file as `least_containing_slow` and is **run as the control** on a small
box against the fast path, at 324 pairs with 0 disagreements (`p1.out` Q1). It
is not deleted, because a method that is too slow to use is still the thing that
validates the method that replaced it.

## p2_overshoot_mechanism.py

**What it asks.** Is the overshoot confined to degenerate operands, and if not,
what is the mechanism?

    $ python3 p2_overshoot_mechanism.py > p2.out
    EXIT=0

**Established.** The overshoot survives on non-degenerate operands: mul_full 127,
add 50, sub 510. The excess is always exactly one integer bit for mul and add.
First non-degenerate witness for all three sites is the same pair, `U<0,1>`
against `U<1,0>`. Corrected forms for add and sub are tight at 1296/1296; the
corrected mul form handling degeneracy alone reaches only 1169/1296, which is
what sent the work to p5.

## p3_formula_vs_extremum.rs

**What it asks.** Is a formula over two numerals' members cheaper, in the
typestate, than an extremum over the same members?

    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type rlib \
        --emit=metadata -o p3.rmeta p3_formula_vs_extremum.rs
    EXIT=0
    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type rlib \
        --emit=link -o p3.rlib p3_formula_vs_extremum.rs
    EXIT=0
    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type staticlib -O \
        -C panic=abort --emit=asm -o p3.s p3_formula_vs_extremum.rs
    EXIT=0
    $ awk '/probe_entry:/,/ret/' p3.s
    _probe_entry:
        mov  w0, #130
        ret

The `--emit=link` run is not redundant. The droplist carries an entry about a
construction that compiled clean at `--emit=metadata` and was caught only at
`--emit=link`, so both are run on every Rust instrument here.

**Established.** Both the sum (a product's widths) and the coordinatewise maximum
(a join) are expressible as ordinary associated-type folds, with zero feature
gates, on the default solver, and both erase completely: the whole file folds to
`mov w0, #130`, which is `44 + 71 + 7 + 8`. **So "formula versus extremum" is not
a feasibility distinction.** Two const assertions in the file state that the join
and the product differ on the same operands.

**Written expecting to need `min_generic_const_args` and not needing it**, because
the arithmetic lives entirely in associated types and never in a const argument
position. That absence is the result rather than the setup.

**One defect, in the instrument and not in the design.** The first build failed
with two `comparison operators cannot be chained` errors, from a macro of mine
that could not carry a `!=`. Fixed by writing those two as plain const
assertions.

## p4_cross_family_join.rs

**What it asks.** When two operands are of different families, whose members are
not the same coordinates, is a common target expressible at all?

    $ rustc +nightly-2026-05-28 ... --cfg arm1 -o p4_arm1.rlib p4_cross_family_join.rs
    EXIT=0
    $ rustc +nightly-2026-05-28 ... --cfg arm2 -o p4_arm2.rlib p4_cross_family_join.rs
    EXIT=0
    $ rustc +nightly-2026-05-28 ... --cfg arm3 -o p4_arm3.rlib p4_cross_family_join.rs
    EXIT=1

**Established, and arm3 is the finding.**

- arm1, a within-family product formula: compiles.
- arm2, a cross-family product formula: **compiles**. The type system accepts any
  cross-family formula written, and has no opinion about whether it is right. So
  feasibility does not constrain the cross-family question at all.
- arm3, the two honest readings of a cross-family join, which `03` section 3.2
  measures as an antichain of width two, declared side by side:

      error[E0119]: conflicting implementations of trait `JoinNum<Flt<_, _, _>>`
                    for type `Uni<_, _>`

**An associated type names exactly one type, so an antichain has no
representation.** A design that infers a cross-family target is picking, not
computing a least upper bound, whatever the order does.

## p5_tight_product_form.py

**What it asks.** The sum-of-widths product form overshoots. What is the tight
closed form, and does it exist?

    $ python3 p5_tight_product_form.py > p5.out
    EXIT=0

**Established.** Analytic least-containing agrees with full set enumeration at
256 of 256 control pairs, so the analytic path is licensed for the wider box. The
derived tight form agrees with the least admitted shape at 6546 of 6561. The
sum-of-widths form is tight at 6100 of 6561 and wastes one bit on 461. The saving
depends only on the two total widths and fires exactly when the narrower
operand's total width is 1, at every width of the wider one. Thirty eight of the
wasteful pairs cross a container-width boundary (tight total 8, 16, 32 or 64
against a formula answer one bit larger).

**A cost note.** The first version ran at `LIM = 6` by full set enumeration and
did not finish; it was killed and rewritten to use the analytic path with the
enumeration retained as the control. The kill is recorded rather than hidden.

## p6_waste_region.py

**What it asks.** Characterise the two populations exactly, because my own
hand-derivation of the waste region disagreed with p5's count by four pairs.

    $ python3 p6_waste_region.py > p6.out
    EXIT=0

**Established.** The 461 waste pairs are 160 where one operand denotes only zero
plus 301 where the narrower operand's total width is 1. The 16 pairs that have a
narrow operand and do **not** waste are listed in full in the output; they are the
15 negative-width pairs, where the clamp hides the waste, plus the doubly
degenerate pair. 461 + 16 = 477, which is every pair with a narrow operand, so
the population is closed and nothing is unaccounted for.

The 15 pairs needing negative integer width have, measured over all of them
rather than sampled, one operand equal to `U<0,1>` and both operands purely
fractional. The count scales linearly against a quadratic pair count: 7/625 at
box 4, 11/2401 at box 6, 15/6561 at box 8. **A corner, not a region**, and the
file says so.

**The hand-derivation this file corrects was mine**, in the working notes, and it
was wrong by exactly those 16 exceptions. Recorded because preferring the
measurement to the derivation is the whole reason the file exists.

## p7_tight_form_in_typestate.rs

**What it asks.** The tight form's predicate looks like it needs exponentials of
type-level naturals. Is it realisable under the permitted feature set?

    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type rlib \
        --emit=link -o p7.rlib p7_tight_form_in_typestate.rs
    EXIT=0   (style warnings only)
    $ rustc +nightly-2026-05-28 --edition 2021 --crate-type staticlib -O \
        -C panic=abort --emit=asm -o p7.s p7_tight_form_in_typestate.rs
    EXIT=0
    $ awk '/probe_entry:/,/ret/' p7.s
    _probe_entry:
        mov  w0, #36
        ret

**Established.** The predicate `2^W1 + 2^W2 - 2 >= 2^(W1+W2-1)` is equivalent to
`min(W1, W2) == 1` for widths at least one, proved in the file's header and
independently measured over the whole box by p6. So the tight product form is a
sum, an equality test against one, and a conditional decrement, all of which
compile gate-free and erase: `36` is `11 + 1 + 8 + 16`.

Four cases asserted, each against a value p5 computed in Python from exact
rational value sets, independently of this file's arithmetic. The container
boundary case is present: `U<0,1>` against `U<1,7>` gives a naive total of 9 bits
and a tight total of 8.

### The negative control, which is why the assertions above count

An assertion nobody has seen fail is not an assertion.

    $ sed 's/<B_T as Widths>::I == 0/<B_T as Widths>::I == 1/' \
        p7_tight_form_in_typestate.rs > p7_negctl.rs
    $ rustc +nightly-2026-05-28 ... -o p7_negctl.rlib p7_negctl.rs
    EXIT=1
    error[E0080]: evaluation panicked: NEGATIVE CONTROL: tight must NOT equal
                  naive here

So the const assertions are structurally capable of failing, and the tight form
genuinely differs from the naive one at the case where it must.

---

## What was NOT run

No bench harness run. Nothing here is priced, and the word **unpriced** is used
in the deliverable rather than reaching for a number. The wasted bit's cost at a
container boundary is a structural claim, not a measurement.

No suite run. The surface this work is about has no shipped source and therefore
no tests; the three zero-count greps in the deliverable's section 0.2 are
measurements of that absence.

No test of the ranged family, of nonzero bias, of the closed-interval adjustment
the record names for normalised channels, or of any radix but two. Every Python
instrument here is unsigned fixed-point, radix 2, zero bias. That is the largest
bound on everything above and the deliverable's section 9 states it.
