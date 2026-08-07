# The law list

**Date:** 2026-08-07
**Position:** after `134c_the_bridge_is_extensible.md`. Written against the standing base
`110_consolidation_eleven.md` and the checkpoint `134b_persona_checkpoint.md` that ordered it.

`110:1426-1430` says what a law is, in one sentence that leaves nothing out. A law is a claim that the terms
of one grouping class stand in a relation, under a stated view, over the value set of a numeral, quantified
over the class rather than pairwise, and keyed on every parameter its proof used. `110:1438-1440` then
establishes that the set of views under which a law holds is downward closed and closed under join, so every
law has a unique finest view, and that view is the law's content.

Both sentences are about laws in general. Neither is about any law in particular, and the section that
carries them ends at `110:1534` without naming one. This file names them.

The first thing to say is what a list like this is for, because the answer changes what belongs in it. It is
not a summary of the algebra section, which is complete on its own terms. It is the extensional half of a
definition that has only been given intensionally, and the test of it is not whether it reads well but
whether a reader can take a row, write the const fn `110:1428-1430` describes, and get a compiler to accept
or reject it. Every column below exists because that sentence demands it, and where I add a column I say
which sentence demanded it.

## Contents

1. The premise check, and what this file assumes that it should not
2. What a law row has to carry, derived from `110:1426-1430`
3. The enumeration
4. The derivations, and what they remove from the count
5. What the operation surface implies and nobody has written
6. The count, and what the count prices
7. The decoder-ring correction, carried per the brief
8. `134c`, and what it moves
9. What cannot be written yet, and what blocks each
10. What is op's


## 1. The premise check

The brief asserts that the algebra section defines a law completely and enumerates none. I checked it before
reasoning from it, because a panel that accepts its premises produces confident work pointed in whatever
direction the brief was pointed.

**The claim holds.** Section 1.7 runs from `110:1420` to `110:1534` and section 1.8 opens at `110:1536`. Read
end to end, 1.7 contains: the definition (`110:1426-1430`), the finest-view mechanism and its lattice
(`110:1432-1446`), law equality as the canonical quotient (`110:1448-1452`), the key (`110:1454-1458`), the
`IS_EXACT` correction (`110:1460-1468`), the `Direction` keying predicate and the two closure predicates
(`110:1470-1479`), the transfer rule (`110:1494-1507`), the mechanism's price against the marker alternative
(`110:1509-1515`), the owed evaluation-strategy sentence (`110:1517-1524`), the IEEE sticky-flag convergence
(`110:1526-1528`), and a caution about the law's noun (`110:1530-1534`).

Every one of those is a statement about laws. Not one is a law. The closest the section comes is the pair of
closure predicates at `110:1472-1479`, and those are conditions under which a law holds rather than the law
itself. Neither names a relation, a grouping class, or a view.

**One correction to the brief's framing, and it changes what the count means.** The laws are not absent from
the document. They are absent from the section that defines them, and they are present, stated in prose,
scattered across nine other sections. Section 1.4 carries three, 1.5 carries seven, 1.8 carries eight, 1.9
carries twelve counting the refutations, 1.13 carries three, 1.16 carries seven, 1.30 carries twelve. So this
is not a design that failed to think about its laws. It is a design that thought about them one operation at
a time and never put them in one place, and the consequence is the one the checkpoint names: nobody can
multiply, because nobody has a count.

That distinction changes the work. Harvesting is most of it and inventing is little of it, which is why this
was always cheaper than three files' worth of reporting implied, and it is also why nobody did it. The value
appears only at the end, when the rows are in one table, and until then the work looks like transcription.

## 2. What a law row has to carry

The definition at `110:1426-1430` is the specification of the table, read as one sentence:

> A law is a claim that the terms of one grouping class stand in a relation, under a stated view, over the
> value set of a numeral, quantified over the class rather than pairwise, and keyed on every parameter its
> proof used.

Five things are named there and each becomes a column. The **relation** is what is claimed. The **grouping
class** is what it is quantified over, and "rather than pairwise" is load-bearing: a law about all
associations of a fold is one row, not one row per pair of associations. The **view** is the monoid
homomorphism out of the grade under which the claim holds, and `110:1438-1440` establishes that a unique
finest one exists, so the column is single-valued and is not a set. The **value set** is the numeral's, and
`110:1530-1534` is explicit that this is the law's noun rather than the type. The **key** is enumerated at
`110:1454-1458`: the operation marker, the operand numerals, the result numeral for a widening operation, the
`Quantisation` resolutions, the `Direction` where a quantiser sits between the exact operation and the result,
and for a fold the accumulator numeral and the arity. `Growth` is excluded and `Lowering` is unnameable from
where laws live.

Two columns the definition does not name and the design demands anyway.

**Asserted or derived.** `110:1428-1430` already makes a distinction with these words: a law is derived by
blanket construction over the composition rather than declared per type, safe when derived and `unsafe impl`
when asserted (D16, D51). That is about how a law reaches a type. The brief asks for a different axis, whether
the law's *content* follows from another law's content. The two are not the same and they do not always agree,
so the table carries the second and I say where they diverge.

**Where it is stated.** Every row cites the line that states it. That is the column which makes this table
auditable rather than one more agent artifact, and it is the column that took the work.

One thing I decline to add, because it would inflate the number op asked to be given honestly. Several
harvested claims are not laws under the definition. `110:1470-1471`'s `Direction` predicate is a rule about
the key. `110:1550-1553`'s statement that interior and total safety are related by the refinement order is a
relation between two laws. The finest-view theorem is a theorem about the table. These are recorded in
section 4 as structure and they are not counted.

## 3. The enumeration

Fifty-four rows in nine families. The view column uses the design's own vocabulary: **identity** is the finest
view, preserving value, definedness and the quantisation-event multiset at once; **weak** preserves values
where both sides are defined; **Kleene** preserves values and definedness. Where the design has not stated a
view, the cell says so and that is itself a finding, carried to section 9.

### 3.1 The fold, eight rows

Stated at `110:1536-1618`. The grouping class throughout is the set of associations of a fold of arity `n`,
which is why these are eight rows and not eight times the Catalan number of them.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| F1 | All associations of the fold are equal | associations of `fold(n)` over `N` with accumulator `M` | identity | op, `N`, `M`, `n`, resolutions, `Direction` | asserted, compiled | `110:1540-1544`, `110:1563-1566` |
| F2 | The fold equals `quantize ∘ exact_sum` | associations of `fold(n)`, plus the specification as a term | identity | op, `N`, `M`, `n`, resolutions | asserted, compiled | `110:1546-1548` |
| F3 | All associations of the MAC are equal | associations of `mac(n)` over `mulnum(N1, N2)` | identity | op, `N1`, `N2`, `M`, `n`, resolutions | derived from F1 | `110:1557-1561` |
| F4 | The MAC equals `quantize ∘ exact_dot` | as F3, plus the specification | identity | as F3 | derived from F2 | `110:1557-1561` |
| F5 | A float fold's exact accumulator is an `Implicit` numeral of quantum `radix^(EMIN-p+1)` and width `(EMAX+1)-(EMIN-p+1)+ceil(log2 n)` | associations of `fold(n)` over a `Ranged` `N` | identity | op, `N`, `n`, `EMIN`, `EMAX`, `p`, radix | derived from F1 and Q7 | `110:1569-1584` |
| F6 | A fixpoint's grade is `join(seed grade, step grade)`, independent of trip count | iterations of the fixpoint | identity | op, seed grade, step grade | asserted, compiled over the whole four-point carrier at widths one to four | `110:1591-1595` |
| F7 | A renormalising step has per-step bounded arity, so interior safety holds at an `Unbounded` trip count | iterations of the fixpoint | identity | op, `N`, `M`, step arity | asserted, `unsafe impl` under D16, and the first consumer-side asserted fact | `110:1595-1602` |
| F8 | A digest computed by partitioning into morsels and combining partials equals the sequential fold at every partition | partitions of the record range | identity | combine op, digest width, partition set | asserted, compiled | `110:1609-1618` |

F5 is derived in content and expensive in evidence: 2,924,207 triples exactly representable at the predicted
width, against 23.17 percent of triples disagreeing when the same folds are held in-format (`110:1577-1579`).
Deriving a law does not make its witness free, which is a point section 6 needs and which is why the pricing
there is not simply a function of the asserted count.

F8 is the one row in this family that is not an instance of the fold machinery, and `110:1609-1613` says so:
a hash accumulator has no value set to leave and no quantiser in its interior, so the two safety conditions do
not apply. It is filed here because it shares the grouping class and nothing else.

### 3.2 Addition, two rows

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| A1 | The value set is closed under addition exactly when `bias / adjustment` is an integer | pairs of in-range operands | identity | `N`, bias, adjustment | asserted, compiled exhaustively in both directions | `110:1472-1475` |
| A2 | `Precise` addition never rounds in range | pairs of in-range operands | identity | `N`, resolutions | derived from A1 and the `Direction` predicate | `110:1470-1472` |

A1 carries a live defect the standing base already records and the implementation phase would otherwise
inherit: the shipped `AddClosed` gate keys on `Bias = Zero`, which is one special case of the predicate, so
there exist numerals with nonzero bias that are additively closed and that the shipped gate refuses
(`110:1473-1475`). The law is right and the gate is narrower than the law.

### 3.3 Multiplication, eight rows

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| M1 | The numeral-level map `mulnum` is associative | triples of numerals | identity | the three numerals | asserted | `110:1645-1647` |
| M2 | `mul_full` is associative | triples of in-range operands | identity | operand numerals, result numerals | derived from M1, and does not typecheck without it | `110:1645-1647` |
| M3 | The product numeral is `bias = B1*B2`, `adjustment = gcd(A1A2, A1B2, A2B1)` | pairs of numerals | identity | `N1`, `N2`, biases, adjustments | asserted, compiled | `110:1649-1651` |
| M4 | The `n`-factor product numeral is the all-bias monomial over the gcd of every monomial carrying an adjustment, associative and commutative | `n`-tuples of numerals | identity | the `n` numerals | derived from M3 by symmetry of the monomial set, checked at arity three with a negative control | `110:1651-1655` |
| M5 | The narrowed product is closed exactly when adjustment and bias are both integers and the adjustment divides `bias^2 - bias` | pairs of in-range operands | identity | `N1`, `N2`, biases, adjustments | asserted, compiled | `110:1476-1479` |
| M6 | Multiplication needs `mul_full` and addition does not | the two operations | identity | operand numerals | derived from A1 and M5 | `110:1477-1479` |
| M7 | Distributivity over the lattice operations holds exactly when the operation is monotone | pairs, for a total operation on a totally ordered value set | identity when total, weak when partial | op, `N`, totality, which IEEE lattice family is meant | asserted, compiled as a biconditional both ways | `110:1657-1662` |
| M8 | `mul_full`'s exponent sum is equivariant under a window shifted by twice the offset | pairs of in-range operands | identity | `N1`, `N2`, the offset | asserted, 254,830,080 instances, zero failures | `110:1690-1694` |

M7 is the one row in this table whose finest view genuinely depends on a key parameter rather than being
fixed by the relation. `110:1658-1662` splits it three ways: for a total operation the biconditional is exact,
for a partial operation monotonicity gives only the weak-equation-level implication, and the Kleene-level
statement additionally depends on which of IEEE's two lattice-operation families is meant, `maximum` which
propagates an undefined operand or `maximumNumber` which suppresses it. Both are required by the standards
test. That is three views for one relation selected by two key parameters, and it is worth flagging because
the finest-view theorem guarantees uniqueness per law while this row's key selects among three.

### 3.4 Refutations, four rows

Laws that fail, stated as findings. They are counted because a law the design considered and refuted is a
claim the design makes and a test the implementation owes, and because `110:1666-1674` uses them to decide
that a ladder rung is derived rather than declared.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| N1 | Wrapping addition does not distribute over the lattice operations | pairs | none, fails at every view | `Hot`, wrapping resolution | asserted | `110:1664-1665` |
| N2 | Saturating addition is not associative | triples | none | saturating resolution | asserted | `110:1665` |
| N3 | Saturating addition fails the annihilation axiom | pairs with the absorbing element | none | saturating resolution, `Specials` | asserted, and separately from N2 | `110:1665-1666` |
| N4 | No preset the design ships or can spell is a dioid over `(max, +)` | the preset table | none | the preset table, `Specials` | derived from N1, N2, N3 and the partiality of `Precise` addition | `110:1664-1666` |

N4's consequence is a design rule rather than another law: the `Dioid` rung is derived rather than declared,
reporting a correct "no" with the failing axiom named (`110:1670-1672`). A numeral carrying an absorbing
`Specials` element could make the rung non-empty, and `110:1672-1674` scopes that as a requirement on the
identity contract discovered from the algebra side. It is open, and it is in section 9.

### 3.5 The crossing contract, three rows

Stated at `110:1096-1098` over the finite datum set of a numeral rather than its value set, which makes this
the one family where the law's noun is not what `110:1530-1534` says it usually is.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| C1 | `decode ∘ encode = id` on values | the value set | identity | `N`, `Encoding` | asserted, always | `110:1096` |
| C2 | `encode ∘ decode` is idempotent on data | the datum set | identity | `N`, `Encoding` | asserted, always | `110:1097` |
| C3 | `encode ∘ decode = id` on data exactly when the encoding is injective | the datum set | identity | `N`, `Encoding`, `Specials`, `Underflow`, cohort rule, sign domain, negative-zero repurposing | asserted, a derived boolean, checked exhaustively over the configuration matrix | `110:1098`, `110:1104-1110` |

C3 has the longest key in the table and it is the reason keys are worth writing down. `110:1112-1118` records
that before file 54 the boolean's only compiled witness in the whole review was signed zero, so a statement
with a seven-parameter key was exercised vacuously for twenty-four files while reading as checked. That shape,
a long key and a one-parameter witness, is exactly what an enumeration makes visible and prose does not.

### 3.6 The quantiser, seven rows

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| Q1 | Rounding on the target grid extended upward without bound, then classifying, agrees with all three test standards | the exact-value domain | identity | `N`, resolutions, `Direction`, `Specials`, `Underflow` | asserted, 41,380,159 operations against binary32 with zero mismatches | `110:1200-1206` |
| Q2 | For all nonzero `y`, `quantise(x + y) != x` | pairs in the value set | identity | `N`, `p`, `EMIN`, `EMAX` | asserted, exhaustively true at exponent span `p` and false at span `p + 1` | `110:1215-1217` |
| Q3 | A rounding tie is reachable only at an even radix | the exact-value domain | identity | radix, `p` | asserted, radices two through thirteen with rounding counts recorded | `110:1207-1211` |
| Q4 | The overflow band is empty unless some point of the exact-result lattice lies strictly inside `(max_r, max_r + q_r/2)` | pairs of in-range operands | identity | op, the three quanta | asserted, zero under-predictions over 5,184 triples | `110:1261-1264` |
| Q5 | That lattice point must be an actual exact result of two in-range operands | pairs of in-range operands | identity | op, operand numerals, result numeral | asserted, and it is precisely what Q4 over-predicts | `110:1265-1266` |
| Q6 | Round-to-odd at the intermediate makes a two-step narrowing agree with a single rounding exactly when `W >= F + 2` | pairs of roundings | identity | intermediate width `W`, destination `F`, resolutions | asserted, measured, and below the precondition round-to-odd is worse than the naive two-step it repairs | `110:1318-1323` |
| Q7 | A `Ranged` numeral's value set is a union of intervals of subgroups whose generators form a geometric chain, and that union is not a subgroup | the value set | identity | radix, `p`, `EMIN`, `EMAX`, `Underflow` | asserted, and the machine agrees | `110:1247-1252` |

Q2 is the sharpest entry in the table and section 7 returns to it. `110:1217-1220` states what it refutes:
same precision, same code, same feature bans, `EMAX` moved by one, and the property's truth value moved with
it. **Every law here whose key contains `EMIN` or `EMAX` inherits that warning**, which is Q1, Q2, Q7 and F5,
and for those four the transfer of a model-width check to a real width is not available by the uniformity
argument that covers the rest of the table. That is four rows out of fifty-four, it was never written down as
a set, and it is the single most useful thing this enumeration produced that was not in anyone's brief.

Q7 is the row three other results follow from, per `110:1252-1255`: the overflow band is inhabited, the fold
needs an exact accumulator, and associativity fails at the format width while holding through the accumulator.
That last clause is why F1 and F5 exist, so Q7 sits upstream of two rows in section 3.1.

### 3.7 Division, three rows

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| D1 | `div_floor` and `rem` satisfy the Euclidean law | pairs with nonzero divisor | identity | operand numerals, divisor domain | asserted, compiled | `110:1935-1937` |
| D2 | The exact quotient's solution set has three shapes, and the failure vocabulary is a function of the shape rather than of a target | pairs over the whole divisor domain including zero | Kleene, since the claim is about definedness | operand numerals, divisor domain, resolutions, `Specials` | asserted, confirmed in all three clauses | `110:1985-1990` |
| D3 | Division by a fixed nonzero representable constant is exact | pairs with the divisor lifted into type position | identity | dividend numeral, the typed divisor | asserted, at zero new mechanism | `110:1945-1948` |

D2 pays for itself twice. `110:2230-2233` records that it classifies every domain event in the
elementary-function family without edit, which is the strongest evidence available that it is the design's
general failure classifier rather than a division-specific mechanism. A list recording it only under division
would hide that, so E2 through E5 cross-reference it.

### 3.8 The elementary functions, seven rows

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| E1 | Correct rounding of same-grid unsigned `sqrt` is the single comparison `r > m`, where `m^2 + r = k * 2^F` | the value set | identity | `P`, `F`, sign domain | asserted, compiled exhaustively at nine `(P, F)` shapes against an oracle that never computes a root | `110:2250-2253` |
| E2 | `sqrt` has no rounding ties, since a tie requires `4r = 4m + 1` | the value set | identity | `P`, `F` | asserted, a parity theorem, zero ties at every sweep | `110:2256-2257` |
| E3 | `sqrt`'s overflow band is inhabited exactly on the identity-free numerals | the value set | identity | `N`, whether the identity element exists | asserted, closed form | `110:2258-2261` |
| E4 | For a radix-power exponential on a matching grid, exact hits occur only at integer exponents and ties never | the value set | identity | radix, `F`, the grid | asserted, exact integer comparisons | `110:2267-2270` |
| E5 | A transcendental result lands on no grid point and no midpoint, off a finite removable list per function | the value set | identity | the function, `N` | asserted from external mathematics, cited rather than compiled | `110:2274-2281` |
| E6 | Integer `pow` is an iterated `mul_full` with one quantisation at the root | associations of the iteration | identity | base numeral, exponent, the `Identity` bound | derived from F1 and M2 | `110:2245-2247` |
| E7 | `recip` is `quantize(1/x)` | the value set | identity | operand numeral, resolutions | derived from D2 | `110:2242-2244` |

E3 deserves a pause. It says the same absent element that breaks the multiplicative identity opens the root's
overflow band, and `110:2259-2261` records that two members found this independently in the same stretch. It
is the only row whose key contains the existence of an identity element, which makes it the row that catches
an identity-free numeral shipping with an `Identity` impl it should not have. That is not a hypothetical: the
design's own worked example of a broken identity is exactly this class of numeral.

E5's provenance is unlike every other row: asserted from Lindemann-Weierstrass and the membership theorem,
cited rather than compiled, per the review's standing practice for standard mathematics no toolchain here
could check. Section 6 prices it separately, because its witness is not a hand-computed value and pretending
otherwise would make the pricing wrong in the one direction that flatters it.

### 3.9 The truth contract, twelve rows

Stated as a contract at `110:5058-5062` and as an owed suite at `110:5583-5589` and `110:5621-5623`. The
grouping class throughout is the truth type's own carrier, and the quantification the design demands is over
**every truth type the design ships and at every width, not a sample of them**, which `110:5586-5589` states
in as many words and grounds on the same reasoning as the sampled-law rule.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| T1 | `and` is associative and commutative with identity `TRUE` | the carrier | identity | truth type, lane count, width | asserted, and asserted nowhere in the tree | `110:5058-5059`, `110:5621-5623` |
| T2 | `or` is associative and commutative with identity `FALSE` | the carrier | identity | as T1 | asserted, unasserted in the tree | as T1 |
| T3 | `and` distributes over `or` | the carrier | identity | as T1 | asserted, unasserted | as T1 |
| T4 | `or` distributes over `and` | the carrier | identity | as T1 | asserted, unasserted | as T1 |
| T5 | `not` is a complement: `x and not x = FALSE` and `x or not x = TRUE` | the carrier | identity | as T1 | asserted, unasserted | as T1 |
| T6 | De Morgan, both directions | the carrier | identity | as T1 | asserted, unasserted | `110:5621-5623` |
| T7 | Absorption, both forms | the carrier | identity | as T1 | asserted, unasserted | `110:5621-5623` |
| T8 | Double complement | the carrier | identity | as T1 | asserted, unasserted | `110:5621-5623` |
| T9 | Idempotence of `and` and of `or` | the carrier | identity | as T1 | asserted, unasserted | `110:5621-5623` |
| T10 | The same axioms on `Mask`'s own vocabulary, not only on `Bool`'s | the carrier | identity | as T1 | asserted, unasserted | `110:5583-5585`, `110:4911-4913` |
| T11 | A finite product of truth types satisfies the contract | products of carriers | identity | the factor types, the lane count | derived from closure of a variety under direct products, cited | `110:4919`, `110:5060-5062` |
| T12 | The structure-preserving maps out of an `n`-lane truth algebra are exactly the `n` coordinate projections | the carrier | identity | lane count | derived, and it is what makes the exit partial | `110:5065-5068` |

This is where the enumeration changes a decision rather than recording one. `110:5585-5588` states that the
truth-contract fork's ground is a theorem about a class, that membership of the design's own two candidates in
that class is asserted nowhere, and that 672 green tests say nothing about it. T11 is derived from a theorem
whose minor premise is T1 through T10, and T1 through T10 are every one of them unasserted in the tree. **So
the fork at section 1.30 rests on twelve rows, of which ten are unchecked and two are derived from the ten.**
That is `110:5585-5588`'s own finding with the arithmetic performed, and performing the arithmetic is what a
list is for.

T10 is counted separately from T3 and T4 rather than folded in, because `110:4911-4913` records that the
design currently ships two Boolean algebras with disjoint vocabularies, and `110:5583-5585` names the absence
on `Mask` specifically. If the implementation asserts one suite generically over every truth type, T10
collapses and the count is fifty-three. I have kept them apart because a generic suite is a proposal and the
two vocabularies are a fact.

### 3.10 The count

| Family | Rows | Asserted | Derived |
|---|---|---|---|
| Fold | 8 | 6 | 2 |
| Addition | 2 | 1 | 1 |
| Multiplication | 8 | 6 | 2 |
| Refutations | 4 | 3 | 1 |
| Crossing contract | 3 | 3 | 0 |
| Quantiser | 7 | 7 | 0 |
| Division | 3 | 3 | 0 |
| Elementary functions | 7 | 5 | 2 |
| Truth contract | 12 | 10 | 2 |
| **Total** | **54** | **44** | **10** |

Forty-four asserted, ten derived. Every derived row has its derivation named in its status cell, and section 4
walks the four that are worth arguing about.

Two honest qualifications on the number, because a count that hides its own softness is worse than no count.
**It is a floor, not a total.** Section 5 lists operations that carry no law and should, and each one that
gains a law raises this. **It is the fixed-point and truth-type count.** The four numeral families beyond
fixed point carry the same operations and `110:2379-2381` makes a claim to the world about the decimal ones,
and their laws are not written anywhere, so they are not here. Section 9 says what that costs.

## 4. The derivations, and what they remove

Ten of the fifty-four are derived. A list that repeated them as independent would overstate the count and the
test burden by about a fifth, which is exactly the error the checkpoint's request was designed to avoid.

Six are mechanical and I state them once. F3 and F4 are F1 and F2 with the destination numeral `N` replaced by
`mulnum(N1, N2)` and the accumulator repaired for biased operands (`110:1557-1561`). M4 is M3 plus the
observation that the monomial set is symmetric under permutation of the factors (`110:1651-1655`). M6 is the
conjunction of A1 and M5 read against each other: no fixed-point numeral with a fractional digit satisfies
M5's first conjunct, and A1's condition is satisfiable, so the multiplicative case needs `mul_full` and the
additive case does not (`110:1477-1479`). N4 is N1, N2, N3 and the partiality of `Precise` addition, one
failing axiom each (`110:1664-1666`). E7 is D2 with the dividend fixed at one (`110:2242-2244`).

Four are worth arguing about, and in three of them the derivation is load-bearing in a way the prose does not
make obvious.

**M2 from M1 is a typechecking dependency, not only a mathematical one.** `110:1645-1647` says `mul_full`'s
own associativity "does not typecheck until the numeral-level map's associativity is established first", and
calls that a precondition nobody had stated of the multiplicative half's own headline claim. So M1 is not an
optional lemma that a diligent implementation would prove on the way to M2. It is the thing without which M2
cannot be written down, which puts it in the same class as the compile failure that
`a-test-that-cannot-compile-is-the-finding.md` treats as the strongest available signal. An implementation
that tries to assert M2 first will get a compiler error, and the correct reading of that error is M1.

**A2 from A1 is a derivation through the `Direction` predicate, which is not itself a law.** `110:1470-1472`
states that `Direction` enters a law's key exactly when the exact result can leave the operand lattice, and
names A2 as one of the two separately-measured facts that predicate replaced. So the chain is: A1 gives the
closure condition, the predicate turns closure into a statement about whether a quantiser fires, and A2
follows. The middle link is a keying rule. It belongs in the table's structure and not in its rows, and I
have put it there, but a reader tracing A2's derivation needs it and would not find it among the laws.

**F5 from F1 and Q7 crosses two families and neither section says so.** Section 1.8 derives the accumulator
width and section 1.5 states that a `Ranged` value set is not a subgroup, and `110:1252-1255` lists "the fold
needs an exact accumulator" as one of three results following from that single sentence. The dependency is
stated, once, in the section that does not contain the law. That is the drift shape
`canonical-design-outranks-intermediate-rounds.md` describes, arriving inside one document rather than across
several: a fact stated where it was discovered rather than where it is used, and invisible to a reader of
either section alone.

**E6 from F1 and M2 is the fold chapter reused verbatim.** `110:2245-2247` says integer `pow` is an iterated
`mul_full` with one quantisation at the root, "the fold chapter verbatim", with `x^0`'s domain corollary
falling out of the `Identity` bound with no new text. The word verbatim is doing real work: if it is right,
E6 owes no witness of its own beyond the instantiation. It is also the third place the multiplicative half's
machinery is reused in one stretch (`110:1696-1698`), and reuse at that rate is usually evidence a concept is
correct rather than evidence a shortcut is being taken.

**What the derivations do not remove.** A derived law still needs an instantiation witness, because the
derivation establishes the content and the witness establishes that the implementation instantiates it. F5 is
the proof: derived, and carrying 2,924,207 measured triples. So the ten derived rows reduce the *proof*
burden and not the *test* burden, and section 6 prices both bases for that reason.

## 5. What the operation surface implies and nobody has written

The admission test at `110:2321-2324` says an operation joins the surface by stating five things, one of which
is its solution-set characterisation, and that an operation absent from the table and not admitted through the
test is not in the design. That is a good test and it does not ask for a law. So an operation can pass the
admission test, ship, and carry no row in section 3. Eight do.

I state each as either a gap or a deliberate silence, per the brief, and I do not pretend to certainty where
the design has simply not spoken.

**The identity laws are missing, and this is the worst of the eight.** The design ships an `Identity` bound,
E3 keys on whether the identity element exists, E6's domain corollary falls out of the `Identity` bound
(`110:2245-2247`), and N4's escape hatch is a numeral carrying an absorbing element (`110:1672-1674`). Four
rows in section 3 refer to identity. **No row asserts `x * ONE = x` or `x + ZERO = x` over any grouping class,
because the design states neither anywhere.** A design whose own worked example of a defect is an
identity-free numeral, and which has four laws keyed on identity's existence, does not state the identity law.
This is a gap and not a silence, it is one row or two, and it is the cheapest high-value addition available.

**The order laws are missing and the design knows there is a question.** `110:1448-1452` records that the
shipped `TotalOrd` induces a datum-level order which separates signed zeros and orders NaN payloads, matching
`f64::total_cmp` and IEEE's `totalOrder`, and that it therefore cannot be the definition of law equality. What
is stated is that it is not law equality. What is not stated is what it *is*: no row asserts totality,
antisymmetry, transitivity, or the relation between the datum-level order and the canonical quotient. M7's
biconditional is about monotonicity with respect to an order that has no stated axioms. Gap, and it is
upstream of M7.

**The view homomorphism law is missing, and the finest-view theorem depends on it.** `110:1436-1437` defines a
view as a monoid homomorphism out of the grade, and `110:1438-1440` derives from that definition that the set
of views under which a law holds is downward closed and closed under join, hence a unique finest view exists.
The derivation is sound and it is conditional on every shipped view actually being a homomorphism, which is
two equations per view (`view(g1 join g2) = view(g1) join view(g2)` and `view(empty) = empty`) and is asserted
for none of the nine. Gap, and it is upstream of the mechanism the whole section is built on. If a shipped
view is not a homomorphism, the finest-view column of section 3 is not well defined.

**The transfer rule's two directions are asserted as one.** `110:1496-1498` says a regrouping publishes
"exactly" the grade generator classes its law fails to preserve, and `110:1506-1507` says understating refuses
while overstating compiles and is merely pessimistic. That is two claims: soundness (the published grade
contains every class the law fails to preserve) and tightness (it contains no others). The design relies on
the type system to catch understatement and explicitly tolerates overstatement, so tightness is a quality
property rather than a correctness one. **Silence, deliberately**, and it should be labelled as such in the
canon so a later reader does not read "exactly" as a checked equality.

**The conversion relations carry no laws.** `134b:240-256` records that the conversion story is one expert's
first read on the replacement for an op withdrawal, with five relations named (identity, inferred at the
operation, `widen` as the only candidate for `From`, `rescale` as never implicit, and refused). The obvious
laws are `narrow ∘ widen = id` and its failing converse, and neither is written. Gap, and it is dispatch three
of the checkpoint's own list, so it is already scheduled and I am not duplicating it.

**The bitfield and composite level carries no laws.** `110:5578-5581` records that all four `bitfield!`
declarations in the suite are pairwise disjoint and that nothing in the macro requires that, so the overlap
path is entered by none of them. The law the tests are missing is the one nobody stated: pack-then-unpack is
the identity on each field, and it holds exactly when the field set is disjoint. Gap, and it is one row plus
the two tests already owed at `110:5624-5625`.

**The digest's second property is a candidate row I have not counted.** `110:1614-1617` shows grouping
invariance and order sensitivity are independent, with a naive chained running hash order-sensitive and not
grouping-invariant, and a positional combine both. F8 is the grouping half. Whether the design *wants* order
sensitivity as a stated law of its combine, or merely observed it while separating the two, is not something
the text decides. **Ambiguous, and I am not resolving it.** If it is wanted, the count is fifty-five.

**The evaluation strategy of a refusing operand's sibling is a stated hole, and it is one row when it closes.**
`110:1517-1524` names it as one sentence the design owes, measures it to change the published grade and no
law's verdict at every composition tested, and records that file 39's standards test tilts toward the strict
reading without deciding it. Op's call, already on the record as his, and it is a law-shaped hole rather than
an unnoticed one. **Silence, correctly labelled**, and it is the model for how the other seven should read.

## 6. The count, and what it prices

Op adopted both diagnostics at `130b:74-76` on an estimate he wrote as a question and asked to have checked:
"I think it's almost free to conjure up both of those, no?" `130b:78-80` states the check owed, which is what
a law-per-item costs in items, compile time and diagnostic quality at the real law count, and what the witness
set costs to maintain as laws are added. The real law count is fifty-four. Here is the multiplication.

**The named-item diagnostic: 108 items, and op's estimate holds.** At two items per law, fifty-four laws give
108 named items. That is the whole cost on the item axis and it is small against a design that already
projects containers over four strategies and every width. The estimate was right and it is right for a reason
worth stating: the item count is linear in the law count and the law count is two orders of magnitude below
the impl count. **I am not able to check the compile-time and diagnostic-quality halves of his question**, and
saying so is more useful than a guess: both need the items to exist, `mock/crates` is out of bounds for this
panel, and a measurement at a model of the item table would separate nothing that matters.

**The witness set: 108 lines, and op's estimate does not survive contact with the quantification.** At two
lines and one hand-computed value per law the arithmetic is 108 lines and 54 values, and if that were the
whole story it would also be almost free. Three things break the linearity, and all three are in the standing
base already.

The first is the truth contract's own quantification. `110:5586-5589` demands the suite be asserted over every
truth type the design ships **and at every width, not a sample**, because a law checked at a chosen subset of
widths is a choice about what not to find out. Ten of the twelve truth rows are subject to that, so their
witness count is `10 * (truth types) * (widths)` rather than ten. The design ships two Boolean algebras with
disjoint vocabularies (`110:4911-4913`) and the width set is the whole supported range. **That single family
is larger than the other eight combined and nobody has written the multiplication down**, which is precisely
the state the checkpoint predicted at `134b:206-208`.

The second is the four rows that cannot transfer. Q1, Q2, Q7 and F5 all key on `EMIN` or `EMAX`, and Q2 is the
compiled demonstration that a property's truth value moves when `EMAX` moves by one with the precision, the
code and the feature bans all held fixed (`110:1215-1220`). The whole model-width-to-real-width transfer
argument is unavailable for those four. Their witnesses are needed at the real exponent spans, exhaustive
checking at a real width is refused by the const-eval wall, and the standing base's own figure is 28.45
seconds at eight bits with rustc refusing at nine. **So four of fifty-four rows have no cheap witness and the
design already knows why**, and that number was not available to anyone before this list.

The third is that derivation does not discount the witness. F5 is derived and carries 2,924,207 measured
triples (`110:1577-1579`). So the witness base is fifty-four and not forty-four, and E5's is a citation rather
than a computation, leaving 53 computable, of which 4 are not computable at the widths that matter and 10 are
multiplied by the truth quantification.

**The honest summary for op.** The named-item half is almost free, as he estimated, at 108 items. The witness
half is almost free for 39 of the 54 rows, is unpriced-and-large for the 10 truth rows until someone states
the truth-type count and the width set, is genuinely hard for 4 rows for a reason the design has compiled, and
is a citation for 1. The right next act is not to build either diagnostic. It is to state the truth-type count
and the width set, which is one line of fact and turns the largest unknown into a number.
