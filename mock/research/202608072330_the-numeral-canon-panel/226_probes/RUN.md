# 226_probes

Seat 226. Six files, `./verify.sh` rebuilds and reruns the lot and checks each
against the outcome it is supposed to have. Toolchain pinned in `toolchain.txt`
and the runner prints both so a drift is visible rather than silent.

Everything here is a spike. Each checks one thing, each takes shortcuts
everywhere else, and none of its incidental spellings is a design decision. The
enumeration in `p2` in particular is a shortcut and is not a proposal that a
design enumerate.

| file | what it asks | must |
|---|---|---|
| `p1_two_ladders_not_one.rs` | do the carrier ladder and the access ladder share jump points | exit 1 |
| `p1b_access_is_not_a_function_of_the_carrier.rs` | is either of the two recoverable from the other | exit 0 |
| `p2_curry_orders_agree.rs` | do the two currying orders of the ladder resolve to one carrier | compile |
| `p2b_negctl_one_cell_disagrees.rs` | `p2` with one cell of table B changed | refuse |
| `p2c_negctl_false_distinguishability.rs` | `p2` with the distinguishability arm made false | refuse |
| `p3_the_count_is_constant_in_the_operation_set.rs` | does the primitive count move as operations are added | exit 0 |

`p1` exits 1 and that is its correct outcome. It asked the wrong question and its
own run said so, which is why `p1b` exists; both are kept because the sequence is
the result. `p3_v1_kept_for_the_trail.rs.txt` and `p3_v1.out` are `p3`'s first
draft and its run, kept for the same reason.

## What each established

**`p1`.** The carrier ladder jumps at 9, 17, 33 and 65 over widths 1 to 128 and
is monotone. Two access rules were computed rather than one, so the answer does
not turn on which packing rule a design ships. Under the tight rule, which uses
the offsets a packed column actually produces, the access ladder jumps at 24
points, **shares all four of the carrier's**, and is **non-monotone**: it returns
`u16` at width 3 and `u8` at width 4. Under the loose rule, worst offset 7 at
every width, it jumps at 2, 10, 26, 58 and 122 and shares **none**.

So the claim in the register that the two partitions share no jump point is
**true under the loose rule and false under the tight one**, and neither file
that carries it says which packing rule it assumed. Reported rather than
smoothed over. The separation survives under both rules, by different arguments:
disjoint jump sets under the loose rule, and monotone against non-monotone under
the tight one, which is the stronger of the two since a non-monotone function is
not a reparameterisation of a monotone one at any width.

**`p1b`.** Jump counting was the wrong instrument. What the ownership clause
needs answered is whether a site holding the carrier can recompute the access
width, which is the question of whether access is a *function* of carrier. It is
not, and neither is the converse, under both packing rules: 432 and 651 violating
pairs forward, 416 and 644 back. The first witness under the tight rule is widths
1 and 3, which share the carrier `u8` and need loads of 8 and 16 bits.

Three controls, all firing. A rule defined equal to the carrier reports zero
violating pairs. A constant rule reports zero forward and 5440 back, so the
search can come back empty and can come back full. A single planted violation at
width 5 is found and nothing else is.

`p1b`'s first run panicked at width 123, where the tight rule needs a 136-bit
window and the widest native type is 128 bits. That is a real state rather than
an error and it is now reported: **the access ladder runs out of native
containers before the carrier ladder does.**

**`p2`.** Both currying orders compile under the pin with no feature gate, under
`#![no_std]`, no `alloc`, no `dyn` and no `TypeId`, and resolve to the same
carrier at all ten widths under all three strategies. The two tables are written
out separately by hand rather than generated from one macro invocation, because
generating both would make the agreement true by construction.

`p2b` is that file with one cell of the second table changed and it refuses,
naming the two types. Its own first draft edited **both** tables, because the two
lines are textually identical, so the orders still agreed and it compiled: a
control that moves both sides of an equality is not a control, and that near-miss
is recorded in the file's header rather than quietly fixed.

`p2c` makes the distinguishability arm false and refuses. Without it the
agreement result would be vacuous, since a table where every objective gave the
same carrier would pass the agreement arm while proving nothing.

**`p3`.** Over widths 3 to 10, exhaustive over the whole value domain and over
every admissible carrier, the number of distinguishable primitives is 1 after
every prefix of `[encode, add, mul, xor, fma]`. It does not move as declared-width
operations are added.

Two controls. Adding a footprint observation splits to **exactly** the admissible
carrier count, 5 at widths 3 to 8 and 4 at 9 and 10, which is the counter's floor
and its ceiling in one arm. And an operation with its projection to the declared
width omitted splits the classes **exactly where the operation's growth exceeds
the headroom the narrowest admissible carrier already has**: an unprojected add
grows by one bit and splits only at width 8, where the headroom is zero; an
unprojected multiply grows by W bits and splits at 5, 6, 7, 8, 9 and 10 and not
at 3 or 4. Sixteen cells, both outcomes present in each arm, biconditional held
in every one.

`p3`'s first draft checked only that the unprojected add splits, and it did not
below width 8. The reason is the finding rather than a broken control, and it is
what the headroom column now reports.

**`p4`.** Exists to earn two entries in `p1b`'s predicate and nothing else.
`p1b` names the total width, the container set and the alignment, and under the
predicate discipline an axis it does not name is one the finding holds nowhere
across. Signedness and fraction width exist for every numeral arvo declares, so
`p1b` unamended holds nowhere, which is not what its instrument shows.

Over 16512 cells, total width 1 to 128 by fraction width 0 to W-1 by both
signednesses, the carrier rule and both access rules move in none. A control
rule that reads the sign bit and the fraction digits moves in 2457, first at
width 8 unsigned against signed, so the comparison can see a difference.

**Its weakness, stated rather than left to be found.** A differential control
over functions that syntactically ignore two of their three arguments is weak
evidence. What it certifies is that these candidate placement rules do not read
those axes, which is what
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` obliges an
instrument to show. It does not establish that no correct placement rule could
read them, and one family plausibly would: an encoding that is not two's
complement, sign-magnitude or offset binary, might want a placement the sign
participates in. That is the hole in the `signedness: any` entry and it is named
in the deliverable rather than buried here.

**`p5`.** `p3`'s law is an arm with a const predicate, so how wide the arm is
matters. A chain of k additions accumulated without projecting between steps and
projected once at the end agrees across carriers up to some k, and two routes to
that boundary were run rather than one, because a closed form checked against
itself is not checked.

The closed form, free while `ceil(log2(k+1)) <= headroom`, is **sound and
conservative**: over widths 3 to 16 and chain lengths 1 to 4096 it disagrees with
the exact condition in 5 cells and is unsound in none of them. The exact
condition, `(k+1) * (2^W - 1) < 2^C`, is the wider arm: at width 3 it allows 35
free additions where the form allows 31, and at width 4 it allows 16 where the
form allows 15. Both are const-computable, so the wider one costs nothing to
prefer.

Two controls. The same form given one bit of slack it does not have disagrees in
313 cells and every one of them is unsound, so the disagreement counter can see
the direction that matters. And both outcomes must be present at every width with
headroom, with only the splitting outcome present where headroom is zero.

That second control's first form demanded both outcomes at every width and failed
on correct data: at zero headroom the narrowest carrier is the declared width, a
two-term sum already wraps it, and no chain is ever free. The criterion was wrong
rather than the data. It is now two claims rather than an exemption.
