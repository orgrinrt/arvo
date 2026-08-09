# The laws under the laws

**Date:** 2026-08-07
**Position:** after `135_lamport_the_law_list.md`, which enumerated fifty-four laws and named three sets of
laws that sit underneath its own table, and after `135b_op_checkpoint_twentynine.md`, which landed while this
file was in flight and which directs at `135b:90-93` that this dispatch should finish rather than take the
gate it states. This file writes the three law sets. Section 7's last subsection is the one place they touch op's gate.

**Status: complete.** Written in pieces and saved as it went, per the dispatch.

`135` harvested every law the design states and found eight operations that ship, pass the admission test and
carry no law. Three of the eight are not ordinary gaps. They sit under the table rather than beside it: the
view homomorphism laws, which make the finest-view column of all fifty-four rows well defined; the identity
laws, which four rows key on; and the order laws, which one row's biconditional is stated over.

The short version of what I found. **Every one of the nine views is a monoid homomorphism**, both equations,
under both readings of the middle detail level, checked exhaustively, so the finest-view theorem's stated
premise is sound and `135:398-404`'s worry does not land where it expected. The premise that fails is a
different one nobody wrote down: **definedness is not recoverable from the grade**, which the design's own
division chapter refutes at `110:1988-1997`, and which makes one of the three literature names at `110:1440`
attach to the wrong point of the lattice whenever `Specials` is populated.

## Contents

1. The premise check
2. The view homomorphism laws
3. The identity laws
4. The order laws
5. M7's three views, resolved
6. The decoder-ring second read
7. Attacking the premise, and what is op's

## 1. The premise check

The brief makes three claims of absence and one claim of fact. I checked all four before writing, because
`135` found the laws were present in the document and absent from the section that defines them, and the
brief's own warning is that the same could be true here.

**The three absences hold, and they hold in the strong form.** Not "absent from section 1.7" but absent from
`110` entirely.

`homomorph` returns nine hits in `110`. One is the view definition at `110:1436` ("a view is a monoid
homomorphism out of the grade"), stated as a definition and never as an equation to check. One is at
`110:1365-1367`, about the ambient-set map for the exact family, a different subject. **The remaining seven
are all the truth-type exit** (`110:4932-4951`, `110:5157`, `110:6864-6866`), where the design states the
homomorphism property, proves the characterisation theorem, and derives a design consequence from it. No hit
asserts either homomorphism equation for any of the nine views.

`x * ONE`, `x + ZERO`, `ONE = x` and `multiplicative identity` as a law return nothing. The four references
to identity in `110` are `110:2246` (`x^0`'s domain corollary falling out of the `Identity` bound),
`110:2260` (the absent element that breaks the multiplicative identity opens the root's overflow band),
`110:3777-3779` (D23 relocating the `Identity` trait to `arvo-algebra-contracts`) and `110:4548` (the
quantiser and `Identity` keyed on value-uniqueness). Every one of them uses the identity law. None states it.

`reflexiv`, `antisymmetr` and `transitive` return two hits in `110` and neither is about an order:
`110:5568` is a test critique about `PartialEq` reflexivity, `110:6440` is about pointer chains. The order
axioms are stated nowhere.

**The fact holds too, and a workspace rule does not.** `rustc +nightly-2026-05-28 --version` reports
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, which is the brief's hash. The workspace rule
`.claude/rules/unstable-features.md` records the same pin as `1.98.0-nightly (cced03bfd)`. The brief is right
and the rule is stale. Small, and worth a line because the rule is what a later reader checks against.

### The correction to the brief's framing, and it is the same shape `135` found

The brief reads as though this design does not state homomorphism laws. It does. It states them for the truth
exit, with the full characterisation (`110:4937-4938`: "`all` is a meet-semilattice map and breaks `or`;
`any` is a join-semilattice map and breaks `and`. The exit is a homomorphism exactly at one lane"), and it
derives a shipped consequence from the failure. That is a design that knows what a homomorphism law is and
knows how to check one.

The difference between the two cases is which end of the argument the homomorphism sits at. For the truth
exit the homomorphism is the **conclusion**: the question was whether the exit preserves structure, so the
answer had to be written down. For the view lattice the homomorphism is the **premise**: it is assumed in
order to derive the finest-view theorem, and a premise never has to be written down to get the conclusion
published. **A design states the properties it concludes and assumes the properties it needs**, and the
second set is where the unchecked things live. That generalises past this file, and I think it is the more
useful half of `135`'s finding restated: the laws that go missing are not the unimportant ones, they are the
ones early enough in the argument that nothing downstream reminds you of them.

### One absence the brief did not name, and it blocks the work it asked for

**The nine views are not enumerated in `110`.** `nine view`, `nine-point` and `nine compositions` return four
hits (`110:1438`, `110:1440`, `110:6118`, `110:6125`) and not one says what the nine are. The standing base
names the lattice, states its cardinality, states that it is not a chain, names three of its points by their
literature names, and never lists it.

So the enumeration had to come from outside the standing base: `37:130` ("three detail levels (Ignore,
Presence, Exact) for each of the two generator classes"), `37:353`, and
`37_probes/probe_1_the_ladder_is_a_view_lattice.rs:99-102`. **A canon cannot state a law about an object it
does not list**, and that is the first thing to fix in section 1.7, ahead of the laws themselves. It is one
sentence and a nine-row table.

## 2. The view homomorphism laws

Five rows. **V1 and V2 are the two equations the brief asked for, they hold for all nine views under both
readings of the middle detail level, and the finest-view theorem's stated premise is sound.** The finding is
not there. It is V4, a law the design needs, has never stated, and which its own division chapter refutes.

### 2.1 The rows

Written to `135`'s seven-column standard. The grade is the free commutative monoid over the five IEEE
clause-7 exception generators, split as `110:2106-2107` splits them: `invalid` and `divideByZero` are causes,
`inexact` and `underflow` are quantisation events, `overflow` is raised by the classification step which is
the quantiser's second half. A view is one detail level from `{Ignore, Presence, Exact}` per class.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| V1 | `v(g1 + g2) = v(g1) + v(g2)` | pairs of grades, for each of the nine views | identity on the grade monoid | the view, the generator set, the class split, the reading of `Presence` | asserted, compiled: 900,000 pairs per `Presence` reading, zero failures | **nowhere**; it is the premise of `110:1436-1440` |
| V2 | `v(0) = 0`, and that image is a two-sided unit of the target | the grade monoid's identity, for each of the nine views | identity | as V1 | asserted, compiled, zero failures over 1,024 grades at nine views | **nowhere**; same premise |
| V3 | The nine-point domain is closed under join, and the join is the componentwise maximum of the two detail levels | pairs of views | identity | the detail-level chain per class | asserted, and true by construction for a product of two three-chains | **nowhere**; `110:1438-1439` asserts join closure of *the holding set*, a different claim which rests on this one |
| V4 | A term is undefined exactly when its cause component is nonempty | the value set of a numeral | identity | `Specials`, the divisor domain, the resolution row in the failing direction | **REFUTED**, against `110:1988-1989` and `110:1993-1997`, for every numeral carrying infinity or NaN | **nowhere**, and section 2.4 is what its absence costs |
| V5 | `R(v1 join v2) = R(v1) intersect R(v2)`, where `R(v)` is the relation the view induces | pairs of views, over pairs of terms | identity | as V3 | asserted, compiled: 84,934,656 comparisons over all 81 ordered view pairs, zero failures | **nowhere**; it is the unstated middle step of `110:1438-1439` |

The check ran under `rustc 1.98.0-nightly (57d06900f)`:

```
grades in model: 1024
PER_CLASS: H1 pairs checked 900000 failures 0 | H2 unit failures 0
PER_GENERATOR: H1 pairs checked 900000 failures 0 | H2 unit failures 0
J2: comparisons 84934656 failures 0
```

The matrix is the whole matrix, not a sample of it: all nine views, both readings of `Presence`, every pair of
grades in the model whose componentwise sum stays inside it, and all 81 ordered view pairs for V5. Nothing was
chosen and nothing was left out.

The load-bearing part of the model, so the claim is reproducible without the scratch tree. `Grade = [u8; 5]`,
indices 0 and 1 the causes, 2 through 4 the events, multiplicities capped at 3.

```rust
fn img(g: &Grade, lo: usize, hi: usize, detail: u8, per_generator: bool) -> u64 {
    match detail {
        IGNORE => 0,
        PRESENCE => if per_generator {
            let mut m = 0u64;
            for i in lo..hi { if g[i] > 0 { m |= 1 << (i as u64); } }
            m
        } else {
            let mut any = 0u64;
            for i in lo..hi { if g[i] > 0 { any = 1; } }
            any
        },
        _ => { let mut m = 0u64; for i in lo..hi { m = m * 16 + g[i] as u64; } m }
    }
}

fn join_img(a: u64, b: u64, detail: u8, per_generator: bool) -> u64 {
    match detail {
        IGNORE => 0,
        PRESENCE => if per_generator { a | b } else if a | b != 0 { 1 } else { 0 },
        _ => { /* componentwise add in base 16 */ }
    }
}

fn view(g: &Grade, dc: u8, de: u8, pg: bool) -> (u64, u64) {
    (img(g, 0, 2, dc, pg), img(g, 2, 5, de, pg))
}
```

V1 asserts `join_img(view(g1), view(g2)) == view(g1 + g2)` per class over every in-range pair. V2 asserts
`view(0)` is a two-sided unit for `join_img`. V5 asserts, for all 81 view pairs and all 1,024 x 1,024 grade
pairs, that `view(g1, join) == view(g2, join)` holds exactly when both constituent views identify them. The
source is at `scratchpad/136/views.rs` and **is owed a home at `136_probes/views.rs`**, which I have not
created because the brief restricts me to one file.

### 2.2 Why V1 and V2 could not have failed, and why checking them was still right

Once the detail levels are written down the proof is three lines, and I state it so a reader can check the
claim without running anything. `Ignore` sends every grade to the unique element of the trivial monoid, which
preserves everything. `Exact` is the identity map. `Presence` is the support map from `N^X` to `2^X`, or its
one-bit collapse, and `support(a + b) = support(a) union support(b)` holds because a sum of naturals is
nonzero exactly when a summand is, with `support(0)` empty. Both readings are homomorphisms, and the nine
views are the nine pairs.

So V1 and V2 are cheap, and that is the point rather than an objection to having checked them. A premise that
turns out to be three lines is a premise nobody was going to notice was missing. `135:398-404` is right that
a column of its table is only conditionally defined without them, and the conditional is now discharged: the
finest-view column of all fifty-four rows is unconditionally defined **as far as the grade is concerned**. The
next subsection is where that qualifier earns its keep.

### 2.3 The reading of `Presence` the design has not fixed, and why it matters

`110:2103-2105` says the grade "over the five clause-7 exceptions with no multiplicity" is "a five-bit word
joined by bitwise or". A five-bit word distinguishes *which* exceptions fired, so that sentence needs
`Presence` to be the support map. Probe 1 models `Presence` as one bit per class, collapsing `invalid` and
`divideByZero` into "some cause occurred"
(`37_probes/probe_1_the_ladder_is_a_view_lattice.rs:112-118`, a single `c` counter). Those are different
quotients and the design does not say which is meant.

Both are homomorphisms, which is why V1 and V2 pass under both, so nothing is unsound either way. What differs
is the cardinality of the lattice. Under the support reading the detail levels are not a three-chain at all:
every partition of the generator set gives a quotient, so the real domain of views is the lattice of
congruences on the grade monoid, and `{Ignore, Presence, Exact}` is a chosen three-element sublattice of it.
That is a defensible choice and it is not recorded as one. **The nine views are a chosen nine, not all of
them**, and V3 is what makes the choice safe (the chosen sublattice happens to be closed under join, so the
uniqueness argument stays inside it). Stating the domain as a choice, with V3 as its justification, is one
sentence and it converts a silent restriction into a designed one.

### 2.4 V4, which the design refutes in its own division chapter

`110:1440` states that "the named relations are three points of a nine-point lattice", and
`37_probes/probe_1_the_ladder_is_a_view_lattice.rs:22-25` fixes which three:

```
    (Ignore,   Ignore) = the WEAK equation
    (Presence, Ignore) = the KLEENE equation
    (Exact,    Exact)  = GRADED equality
```

The Kleene equation is "both terms defined or both undefined, and where defined the values agree". The
design's relation is "the view sends their grades to the same thing and their values agree wherever present"
(`110:1436-1437`). **Those two coincide only if definedness is recoverable from the cause component**,
because the relation never mentions definedness except inside a both-defined guard, which is vacuous whenever
one side is undefined. That recovery is V4, and it is stated nowhere.

Probe 1 satisfies V4 by construction and so could not have caught this. Its `resolve` returns `def: false,
c: 1` for `REFUSE` and `def: true, c: 0` for `CLAMP`, `WRAP` and `SUBZERO`
(`37_probes/probe_1_the_ladder_is_a_view_lattice.rs:129-166`), so in that model a cause exists exactly when
the term is undefined and `(Presence, Ignore)` is the Kleene equation exactly as claimed. **The model made the
invariant true rather than testing it.** That is the setup-that-helps shape, and it is invisible unless you go
looking for the invariant a model happens to maintain, which is why this is worth a row rather than a footnote.

The shipped design breaks it twice, in one paragraph:

> a numeral carrying infinity delivers it as the absorbing far point and raises `divideByZero`
> (`110:1989`)

> the event is **`invalid`**, resolving to NaN exactly where the numeral carries one and leaving the
> operation **partial** at that input where it does not (`110:1994-1997`)

Both cause generators land on **defined** results when `Specials` carries the corresponding value. `x/0` with
infinity available is a defined term carrying a `divideByZero` cause. `0/0` with NaN available is a defined
term carrying an `invalid` cause. The cause component is nonempty, the term is defined, and V4 is false for
every numeral the design ships with `Specials` populated.

The witness, from the same run:

```
definedness witness: (Presence, Ignore) identifies a defined divideByZero term
with an undefined invalid term: true
```

A defined `x/0` under `Specials = infinity` and an undefined `0/0` under `Specials = none` have the same image
at `(Presence, Ignore)`: one cause each, events ignored. The value clause is vacuous because one side is
undefined. The relation holds. The Kleene equation does not. **`(Presence, Ignore)` is not the Kleene
equation.**

### 2.5 What that costs, stated exactly, because it is narrower than it sounds

It does **not** touch the finest-view theorem. V1, V2, V3 and V5 all hold, the domain is a lattice, the
holding set is downward closed and join closed, and every law still has a unique finest view. Nothing in
`135`'s table becomes undefined. I want that said plainly before the damage, because a finding stated without
its boundary gets read as larger than it is and then discounted when someone finds the boundary.

It costs three things and they are naming rather than soundness.

**One cell of `135`'s table is wrong.** D2's view column reads "Kleene, since the claim is about definedness"
(`135:230`). D2 is the division law, its key contains `Specials`, and division under populated `Specials` is
the exact composition where the identification fails. So the one row in fifty-four whose view cell was chosen
*because* the law is about definedness is the row where the view named does not mean definedness. That is not
bad luck. The cell was filled from the literature name, and the literature name attaches to the wrong point
whenever `Specials` is populated, so the rows most likely to be wrong are exactly the rows where someone
reasoned about definedness carefully enough to reach for the name.

**`110:1440`'s identification is false as written**, for one of its three names, keyed on `Specials`. The weak
equation at `(Ignore, Ignore)` and graded equality at `(Exact, Exact)` are both correct unconditionally, and
only the middle name is affected.

**Definedness is a third axis the design has been carrying inside the first one.** The grade has two generator
classes; the design's relation has a definedness dimension that is not in the grade at all. Where V4 holds the
two collapse and nine points suffice. Where it fails they do not.

### 2.6 The repair, suggested and not decided

Three shapes, and the choice is op's because it changes the size of a shipped lattice.

**Add a definedness axis.** `{Ignore, Exact}` on definedness, since a boolean has no presence level, giving
eighteen views with `(Ignore, Ignore, Ignore)` the weak equation, `(Ignore, Ignore, Exact)` the Kleene
equation and `(Exact, Exact, Exact)` graded equality. The nine current points are the `Ignore`-definedness
half. This is the shape under which every literature name lands on a point unconditionally, and the
uniqueness argument survives unchanged because a product of three chains is still a lattice whose join is
componentwise maximum and whose kernels still nest per axis, which is V3 and V5 restated one axis wider.

**Split the cause generators by whether they refuse.** Attractive, and I do not think it works.
`110:1994-1997` has `invalid` refusing or delivering NaN depending on `Specials`, so the same generator sits
on both sides of the split and the split has to be keyed on `Specials` rather than on the generator. That
makes the view domain itself key-dependent, which is a larger change than adding an axis.

**Assert V4 as a precondition and scope the nine-point lattice to numerals where it holds.** Cheapest, and it
narrows the mechanism to exactly the numerals that carry no `Specials`, which is not where the design spends
its chapters.

I would suggest the first and I am not deciding it. What I will say without hedging is that **`110:1440`
should not ship as written**, because it names a lattice point after a relation it is not, under the
configuration most of the design's own chapters assume.

### 2.7 Whether V3 and V5 needed checking

V3 is true by construction and I checked it anyway, because the construction is the thing that was never
written down. V5 is the one I would not have predicted from the prose. `110:1438-1439` states that the holding
set is closed under join and derives uniqueness from it; the step in between is that the relation at the join
is the intersection of the two relations, which is what makes "holds at both" and "holds at the join" the same
statement. File 37 states it in one clause of one sentence (`37:120-122`, "the relation at the pullback is
precisely the conjunction of the two") and `110` drops it entirely. It holds over all 81 view pairs, and it
belongs in the canon as V5 because it is the load-bearing line of a theorem the whole section rests on.

## 3. The identity laws

Seven rows. The design's shipped mechanism for identity is better than `135` implies: `Identity<Op>` already
refuses to exist where the element does not, by a sealed witness with no impl at the failing tag, and it
carries a diagnostic that explains why. **What is missing is not the gate. It is the equation.**

### 3.1 The rows

`e` below is `<N as Identity<Op>>::IDENTITY`.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| I1 | `x + e = x` and `e + x = x`, for `Op = Additive` | the value set of a numeral | identity, and the operation raises no event on this argument | numeral, resolutions | asserted; **stated nowhere** | nowhere; the element is at `arvo-strategy/src/identity.rs:51-54` |
| I2 | `x * e = x` and `e * x = x`, for `Op = Multiplicative` | the value set | identity | numeral, resolutions, the result numeral in the widening form | asserted; **stated nowhere** | nowhere |
| I3 | The identity element of `Op`, where one exists in the value set, is unique | the value set | identity | numeral, `Op` | derived from I1 or I2 by the one-line monoid argument (`e = e * e' = e'`) | nowhere |
| I4 | `Identity<Op>` is inhabited exactly when `Op`'s identity is both a lattice point of the numeral and inside its range | the numerals | identity | bias, adjustment, `I`, `F`, sign domain, `Op` | asserted; the mechanism ships, the predicate it computes is a specialisation (section 3.3) | `arvo-strategy/src/identity.rs:41-45` and `:76-92` |
| I5 | Zero is in the value set exactly when `bias / adjustment` is an integer, which is A1's condition | the numerals | identity | bias, adjustment | **derived from A1**; compiled by search over 1,200 (bias, adjustment, denominator) triples, zero disagreements | nowhere; A1 is at `110:1472-1473` |
| I6 | Multiplicative closure and a representable multiplicative identity are independent conditions, in both directions over rationals and in one direction over integers | the numerals | identity | bias, adjustment | derived, and compiled with witnesses in both failing directions | nowhere |
| I7 | Where the numeral carries an absorbing element for `Op`, `x Op a = a` for every `x` in the value set | the value set | identity | numeral, the operation pair, `Specials` | **OPEN, and it is op's**; section 3.5 derives where it belongs | `110:1672-1674` |

### 3.2 The equation is missing and the element is not, which is the whole problem

The shipped trait is three lines of surface:

```rust
pub const trait Identity<Op>: Sized {
    /// The identity element of `Op` in this type.
    const IDENTITY: Self;
}
```

(`arvo-strategy/src/identity.rs:51-54`.) There is no bound relating `Op` to an operation and no equation
relating `IDENTITY` to it. **Any value of the type satisfies this trait.** The doc comment says what the
constant means and the doc comment is not checked. An impl that sets `IDENTITY` to seven type-checks, and
every consumer of the bound (`E6`'s `x^0`, `110:2246`; the ladder's `Monoid<Op>` at `110:3779`; `float.rs:135`
and `float.rs:169` which construct a zero from it) inherits whatever it says.

This is the perimeter question from `what-you-can-observe-is-what-you-guaranteed.md` asked about a trait
rather than a type. The guarantee "this is the identity" holds over the operations through which the trait
can be observed, and the only observation is a constant with no law attached to it. I1 and I2 are the
perimeter, they are two lines, and they are the cheapest rows in this file.

### 3.3 Where the numeral has no representable one, which the design gets right and states narrowly

The brief asks what happens in the identity-free case. The design already answers it, well, and I want to say
so before saying what is wrong with the answer:

> **Absence is a statement.** An identity element must be a value of the type it is an identity for, so where
> an operation has no identity in the type there is no impl, and naming `IDENTITY` for it fails to resolve
> rather than returning something plausible. `UFixed<0, F, S>` spans `[0, 1)`, which contains zero and does
> not contain one: it impls `Identity<Additive>` and no `Identity<Multiplicative>`.
> (`arvo-strategy/src/identity.rs:41-45`)

The mechanism under it is a sealed witness trait `OneRepresentable<const TAG: usize>` with `impl
OneRepresentable<0> for Picker` and deliberately no impl at tag 1
(`arvo-strategy/src/identity.rs:76-92`), consumed by the fixed-point impls at
`arvo/src/ufixed.rs:69` and `arvo/src/ifixed.rs:104`. That is invariant-by-construction and it is the right
shape. The `#[diagnostic::on_unimplemented]` note is one of the better ones in the tree.

**The narrow part is the predicate.** The tag is computed from the integer-bit count alone:

```rust
pub const fn tag_one_representable(int_bits: u16) -> usize {
    if int_bits >= 1 { 0 } else { 1 }
}
```

(`arvo-strategy/src/identity.rs:70-76`.) One is representable when it is a lattice point of the numeral **and**
inside its range. For an unbiased dyadic fixed-point numeral the lattice half is vacuous, because the lattice
is every multiple of `2^-F` and one is `2^F` of them, which the compiled check confirms at every `F` from one
to four. So the whole content is the range half, and `I >= 1` is exactly it. **The shipped gate is correct for
the family arvo ships today and is a specialisation of the general predicate.**

That is the same sentence `110:1473-1475` writes about `AddClosed`, one operation over: "the shipped
`AddClosed` gate on `Bias = Zero` is the special case of that, which means there exist numerals with nonzero
bias that are additively closed and that the shipped gate would refuse". **The pattern recurs and nobody has
named it as a pattern.** A representability predicate gets written against the numeral family in hand, the
family later gains a parameter, and the predicate is now a strictly narrower condition wearing the general
name. Two instances is enough to state the rule: **a predicate over numerals is keyed on every numeral
parameter or it is keyed on the ones that existed when it was written.**

The concrete forward defect: `110:4467-4473` proposes `Adjustment = 1/(r^F - 1)` (the UNORM rule) as one of
two routes the design should expose, at `I == 0`, with both endpoints exact. Under that adjustment one **is**
in the value set and `tag_one_representable(0)` returns 1, so `Identity<Multiplicative>` would be refused for
a numeral that has a multiplicative identity. The route the design says it should ship is refused by the gate
it already ships.

### 3.4 One correction to the standing base, which `135` inherited

`110:1473` calls `AddClosed` shipped. `AddClosed`, `Bias` and `Adjustment` each return **zero hits** across
`mock/crates/`. Whatever item the sentence means, no item under those names exists in the tree, so the
qualifier "shipped" is wrong and the defect it describes is a design-level one about a designed gate.
`135:143-145` carries it forward as "the shipped `AddClosed` gate keys on `Bias = Zero`", which is a claim
about source state, in the status cell of A1. The law is right, the defect is real, and the word is wrong.
Small, and it is exactly the class of claim `cl-claim-sketch-discipline.md` exists for.

### 3.5 The absorbing element, and why I do not think it belongs on the identity contract

`110:1672-1674` scopes an absorbing `Specials` element "as a requirement on the identity contract discovered
from the algebra side, not designed now", and `135:617-618` records it as op's, correctly. I am not deciding
it. I can narrow what the decision is about, which is the part the brief asks me to work out.

An absorbing element is never absorbing on its own. It absorbs one operation and it is the identity of
another: in a semiring `(V, plus, times)` the annihilation axiom says `Identity<plus>` annihilates `times`,
and in the `(max, +)` dioid the bottom element is both the identity of `max` and the absorber of `+`. So the
element is already named by the contract, at a different `Op`. **What is missing is not a constant. It is a
law over a pair of operations, and `Identity<Op>` names one element per operation and cannot relate two.**

That places it, and the placement is the design's own: D75 at `110:1676-1683` separates the operation-carrier
from the structure and says "the laws stay separate markers per D51" with the ladder written to the depth the
theory goes. A law relating two operations is a `Semiring` rung law, not an `Identity` clause. So my reading
is that `110:1672-1674`'s sentence should be re-scoped from "a requirement on the identity contract" to "a
requirement on the semiring rung", and the identity contract stays sealed exactly as it is.

Whether that makes the `Dioid` rung reachable is then a question about `Specials` and not about `Identity`.
The chain: `Identity<Max>` over a `Ranged` numeral carrying negative infinity is that infinity rather than
`Bounded::MIN`; IEEE already specifies `-inf + x = -inf` for finite `x`; so annihilation holds and N4's four
failing axioms lose one. The other three (N1 wrapping distributivity, N2 saturating associativity, the
partiality of `Precise` addition) are untouched by it, so **an absorbing element is necessary and not
sufficient**, and `135:617-618`'s framing of it as "the only escape from N4" is the necessary half stated as
if it were the whole. Also worth flagging: `-inf + inf` raises `invalid`, which lands back in section 2's V4,
because that is a cause on a term that is defined wherever the numeral carries NaN.

## 4. The order laws

Seven rows. The design says what `TotalOrd` is not (`110:1448-1452`), designs what it should be
(`110:2641-2647`), and never states an axiom. The shipped trait is one method:

```rust
pub const trait TotalOrd {
    /// Return the total ordering of `self` vs `other`.
    fn total_cmp(self, other: Self) -> Ordering;
}
```

(`arvo-numeric-contracts/src/lib.rs:65-68`.) Same shape as `Identity`: a surface with no law, so any
comparison function satisfies it, including one that is neither transitive nor total.

### 4.1 The rows

Stated for the **corrected** `TotalOrd`, the canonicalise-then-compare body adopted at `110:2641-2644`, since
the shipped one is being renamed `totalOrder` and declared non-law-usable by the same sentence.

| ID | Relation | Grouping class | View | Key | Status | Stated at |
|---|---|---|---|---|---|---|
| O1 | Reflexive: `x <= x` | the value set | identity | numeral, `Specials`, the NaN-class placement | asserted; **nowhere** | nowhere |
| O2 | Transitive | the value set | identity | as O1 | asserted; **nowhere** | nowhere |
| O3 | Antisymmetric on the quotient: `x <= y` and `y <= x` implies `x` and `y` canonicalise to the same datum | the value set | identity | as O1 | asserted; **nowhere** | nowhere |
| O4 | Total: for every `x`, `y`, either `x <= y` or `y <= x` | the value set | identity | as O1, plus whether the NaN class is in the domain | asserted; **nowhere**, and its one genuinely open case is `110:2645-2647` | nowhere |
| O5 | Well defined on law equality: data that canonicalise alike compare `Equal`, and `x <= z` iff `y <= z` for law-equal `x`, `y` | the datum set | identity | numeral, `Encoding`, `Specials`, the cohort rule | **REFUTED for the shipped order**, compiled, `E0080`; asserted for the corrected one | `110:2632-2639` |
| O6 | `min` and `max` are the meet and join of the order, so the lattice operations are well defined on the value set | pairs | identity | as O1, plus which IEEE lattice family | derived from O1 through O5; **it is what M7 needs and nobody stated it** | nowhere |
| O7 | Monotone: for a total operation, `x <= y` implies `op(x, z) <= op(y, z)` and `op(z, x) <= op(z, y)` | pairs, over a total operation on a totally ordered value set | M7's own view, keyed | op, numeral, totality | asserted, as the right-hand side of M7's biconditional | `110:1657-1662` |

### 4.2 What M7 needs from the order, and why the `TotalOrd` split is upstream of it

M7 says distributivity over the lattice operations holds exactly when the operation is monotone
(`110:1657-1662`). Both sides of that biconditional are stated over an order, and each needs a different thing
from it.

The **right** side needs O1 through O4. Monotonicity is a statement about an order relation, and it is content-
free unless the relation is at least a preorder. Totality is what makes the design's own phrasing legitimate:
`110:1658` conditions the exact biconditional on "a total operation on a totally ordered value set", so O4 is
already in the hypothesis and is simply not written as a law.

The **left** side needs O6, and O6 needs O5, and this is the part the standing base does not connect. "The
lattice operations" are `min` and `max`. They are the meet and join **of an order**, and which order is not
stated. If it is the shipped `totalOrder`, then `-0.0 < +0.0` strictly (`110:2634-2635`), so `max(-0.0, +0.0)`
is `+0.0` and `max(+0.0, -0.0)` is `+0.0` while the two arguments are law-equal in the other position: the
operation is a function on data and not on values. M7's other side, distributivity, is quantified over the
value set, because `110:1530-1534` says the law's noun is the value set. **So under the shipped order M7's two
sides are quantified over different objects and the biconditional is not well formed.**

That is the sharp consequence and I state it as a dependency rather than as a defect, because the design has
already chosen the repair. The `TotalOrd` split at `110:2641-2644` is not a tidy-up of a badly named trait. It
is a precondition of M7, and `110`'s section 1.9 and section 1.20 do not cite each other. Whoever writes the
canon's algebra section should carry the dependency both ways.

### 4.3 Total on what, exactly, since the brief is right that the cases differ there

Three domains, and they do not all get the same answer.

**The value set.** Total, given O1 through O4, with one exception: `110:2645-2647` leaves open "where the
canonical NaN class sits in the value-level order (a real choice; one working placement puts it above every
finite and infinite value, matching IEEE's own convention)". So the honest statement today is that the order
is total on the value set **minus the canonical NaN class**, and totality across that class is a placement op
has not made. Every law keyed on `Specials` inherits that, which is D2, C3, Q1, E5 and M7.

**The exact-result domain**, which Q1, Q4, Q5, E1 and E5 quantify over rather than the value set. Totality is
inherited for free: exact results live in the ambient ordered field, and `110:1247-1252`'s finding that a
`Ranged` value set is a union of intervals of subgroups and not a subgroup is a statement about the group
structure, not about the order. A subset of a totally ordered set is totally ordered. Nothing is owed here and
saying so is worth a line, because "not a subgroup" reads like it might cost something and it costs nothing
on this axis.

**Terms rather than values.** Not ordered at all, and this is the interesting one. An undefined term has no
value, so it is outside the order's domain entirely. The design handles definedness through the view and not
through the order, which is correct, and it means **the order and the grade between them do not cover
definedness**: the grade does not determine it (section 2.4, V4) and the order does not contain it. Sections 2
and 4 arrive at the same missing axis from opposite ends, and I did not expect that when I started either one.
Two independent routes to one gap is the strongest evidence available that it is a real gap rather than a
framing preference, and it is the single thing in this file I would most want a third read on.

## 5. M7's three views, resolved

`135:160-166` flags that M7 is one law whose key selects among three views, "worth flagging because the
finest-view theorem guarantees uniqueness per law while this row's key selects among three", and does not
resolve it. The homomorphism laws do not bear on it. The key does, and the resolution is in `135`'s own
definition.

`110:1426-1430` says a law is "keyed on every parameter its proof used", and `135:75-84` reads that as a
column. A law is therefore an instantiation, not a schema: **two different key values are two different
laws.** M7's view varies with two key parameters (whether the operation is total, and which IEEE lattice
family is meant), so M7 is not one law with three views. It is three laws, each with one view, sharing a
relation.

So the resolution is that `135`'s table should split M7 into three rows, and the general check is worth
stating because it can be run mechanically over the whole table: **a row whose view cell depends on a key
parameter is a row that has not been fully instantiated.** `135:160-161` already says M7 is the only row in
its table with that property, so the split costs two rows and the count goes from fifty-four to fifty-six
before anything in this file is added.

There is no tension with the uniqueness theorem and there never was. Uniqueness is per law, the key
individuates laws, and a schema is not a law. That the flag was raised at all is a symptom of the table
carrying schemas and instances in the same column without marking which is which, which is a real thing to fix
and is a smaller thing than the flag suggested.

## 6. The decoder-ring second read

`135:485-509` carries a correction to `110:2501-2505` resting on `134` alone and asks for a second read. I
verified it independently rather than restating it. **The correction holds on both halves, and it understates
its own case in one direction.**

**The base half, reproduced.** A digit-tower type at base two and base ten, one mismatch each, under
`rustc 1.98.0-nightly (57d06900f)`:

```
error[E0308]: mismatched types
   = note: expected struct `Fixed<N1<N0<N0<N0<N0<End>>>>>, _>`
              found struct `Fixed<N1<N1<N0<N1<N0<End>>>>>, _>`

error[E0308]: mismatched types
   = note: expected struct `Fixed<N1<N6<End>>, _>`
              found struct `Fixed<N2<N6<End>>, _>`
```

Sixteen against twenty-six, the same two numerals, both bases. Base ten prints the decimal digits in reading
order and is strictly shorter. `134:200-212` and `134:342-345` hold.

**The elision half, and where `134` understates.** I pushed the tower to twenty digits to test "no elision on
the differing coordinate". There is no elision: the note prints all twenty constructors. But rustc's primary
label prints something else, and neither `134` nor `135` records it:

```
   |  ---- ^ expected `N0<N1<End>>`, found `N1<N0<End>>`
```

The label names the innermost differing **subterm**, not the type. At base ten the numeral is short enough
that the label carries the whole thing (`expected Fixed<N1<N6<End>>, Z>`, from the first reproduction). At
base two the label carries a two-digit tail with no positional information, which the consumer cannot map back
to a width at all, and the full tower is relegated to the `note:`. **So binary is worse than `134` measured,
not better, and the gap between the two bases is wider than its table shows.** That strengthens `134`'s
conclusion by a route it did not take, which is the most useful thing a second read can return.

**The lever half.** `134:168-178` is right that the lever covers a declared-accumulator position and not a
plain annotated return, and right that `E0308` and `E0277` print the same type names so the error class is not
where readability comes from. I did not re-derive the reach claim, and I am saying so rather than implying I
checked it: what I verified is the base claim and the elision claim.

**The qualification stands and I would keep `134b`'s word.** `134b:317-328` is right that at `134:373` the
common case still prints `expected N1<N3<End>>, found N1<N6<End>>`, which is a lexical decode step rather than
a consumer reading their own number back. My own base-ten reproduction prints exactly that shape. **Mitigated,
not refuted** is the correct word and I would not soften it.

**Standing after this read.** The correction now rests on two independent compiled arrivals rather than one
expert's first read, which is the two-expert threshold this workspace uses for a call about what is true. It
is not a call about what the canon permits, so nothing further is owed on it beyond landing it: `110:2501-2505`
should not ship as written, and the downstream face layer built on the false ceiling and priced at doubling the
relevant trait surface (`135:508-509`) should be repriced before op sees the fork.

## 7. Attacking the premise, and what is op's

### What this brief takes for granted

**That the three sets are the same kind of thing.** The brief bundles the view homomorphism laws with the
identity laws and the order laws, and `135:565-569` bundles them too ("The identity laws, the order laws, and
the view homomorphism laws can be written today, from the standing base, by whoever next opens the algebra
section. Each is one or two rows"). Having written all three, they are not one kind of thing and the
difference matters for where they go.

The identity laws and the order laws are laws about **a numeral's value set**, which is what `110:1530-1534`
says a law's noun is. They sit in the table with the other fifty-four and the grouping-class and view columns
mean the ordinary thing.

The view homomorphism laws are laws about **the grade monoid and the mechanism over it**. Their noun is not a
numeral. Their view column is degenerate, because the equations hold on the nose. They are the same category
as the finest-view theorem, which `135:100-101` explicitly declines to count as a law, filing it as structure.
**So `135`'s own rule for what to count excludes the very rows it says are the worst gap**, and it does not
notice, because it named the gap in section 5 and set the counting rule in section 2 and the two never met.

I do not think that means they should not be written. It means the canon's algebra section wants two tables,
not one: the laws of the mechanism, which are few and are preconditions of the machinery, and the laws of the
numerals, which are many and are what the machinery is applied to. Filing V1 through V5 in the same table as
Q7 makes the count wrong in a way that is hard to see later, because a reader multiplying the count by a
per-law cost will price a proof about a monoid quotient as though it were an exhaustive numeral sweep.

**That the count is what a canon needs.** `135:578-599` already attacks this well and I will not repeat it,
except to record that writing these fifteen rows made me agree with its conclusion for a reason it does not
give: **the rows I could not write were more informative than the rows I could.** I4's key contains a
parameter (`Adjustment`) that does not exist in the tree, so writing the row is what surfaced the forward
defect at section 3.3. That does not happen when you write a test and it does not happen when you write prose.
It happens when a table has a column that must be filled.

**That the finest-view theorem's premise was the risk.** It was not. The premise holds and is three lines. The
risk was the sentence next to it, `110:1440`'s identification of three lattice points with three literature
names, which nobody flagged because it reads as bookkeeping rather than as a claim. I would generalise: in a
section that establishes a mechanism, the claims most likely to be wrong are the ones that translate the
mechanism back into the vocabulary it replaced, because they are written last, checked least, and are the only
part a reader who knew the old vocabulary will actually read.

### What is op's

Six, one line each, and I am touching none of them.

1. **Whether definedness becomes a third view axis** (section 2.6), making the lattice eighteen points, or
   whether V4 is asserted as a precondition and the nine-point lattice is scoped to numerals carrying no
   `Specials`. It changes the size of a shipped lattice, so it is not a panel call.
2. **Which reading of `Presence` is meant**, one bit per class or the support set (section 2.3). Both are
   homomorphisms. `110:2103-2105`'s five-bit-word convergence needs the support reading and probe 1 models the
   other.
3. **Where the canonical NaN class sits in the value-level order** (`110:2645-2647`), already open, already
   his, and O4's totality is stated over the value set minus that class until it is placed.
4. **Whether the absorbing element is re-scoped from the identity contract to the semiring rung**
   (section 3.5). His either way; my reading only narrows what the question is.
5. **Whether the canon's algebra section carries one law table or two** (section 7 above), which is the
   mechanism-versus-numeral split and which decides whether the count is fifty-six or fifty-six plus five.
6. **The evaluation strategy of a refusing operand's sibling** (`110:1517-1524`), unchanged, listed only
   because section 2's V4 touches it: under the strict reading a sibling's causes accumulate onto a possibly
   defined result, which is another route to the same definedness question and may make item 1 cheaper to
   decide than it looks.

### Where this touches op's gate at `135b`

`135b` landed while this was in flight and directs that this dispatch finish rather than take the gate
(`135b:90-93`). I have not taken it. One finding in here feeds it and should reach whoever does.

The gate's part 2 is that **the typestate derives the matching container and numeral representation**, not the
consumer and not a later layer (`135b:21-23`). Section 3.3 is an instance of that derivation stopping one
parameter short. `tag_one_representable(int_bits)` derives representability of the multiplicative identity
from the integer-bit count alone (`arvo-strategy/src/identity.rs:70-76`), which is the correct predicate for
an unbiased dyadic numeral and the wrong one for the `Adjustment = 1/(r^F - 1)` route the design says it
should expose (`110:4467-4473`). The consumer writes widths, the typestate derives, and the derivation reads
one of the two parameters that determine the answer.

That is a caveat on part 2 of exactly the shape `135b:65-68` rules unacceptable: the derivation works on
condition that the numeral family is the one the predicate was written against. It is smaller than the
machine-type caveat `135b` is about and it is the same kind of thing, which is worth one line to the next
dispatch because a gate with four parts is failed by the cheapest of them as surely as by the dearest. The
repair is not a mechanism: it is keying the predicate on every numeral parameter rather than on the ones that
existed when it was written, which section 3.3 states as a rule and which `110:1473-1475` records a second
instance of.

I am not extending this to the machine-type question, which is the next dispatch's and not mine.

### Standing

This file is agent output on the suspect rung. Every row cites where it is stated or says plainly that it is
stated nowhere. The compiled claims are reproducible from the model descriptions in sections 2.1, 3.1 and 6;
the sources live at `scratchpad/136/{views,identity2,ring,ring2}.rs` and **are owed a home at
`136_probes/`**, which I have not created because the brief restricts me to one file.

Two things in here should not be carried forward as settled by a later file. **V4's repair** (section 2.6) is
one member's reading of a design question that changes a shipped lattice, and only the refutation is
compiled, not the fix. **The mechanism-versus-numeral table split** (section 7) is a suggestion about canon
shape and rests on nothing but the argument given.

One thing should be carried forward without further reads. **`(Presence, Ignore)` is not the Kleene equation
whenever `Specials` is populated**, which is refuted against the standing base's own division chapter at
`110:1988-1997` with a compiled witness, and which makes `110:1440` and `135:230` wrong as written. That does
not need a second opinion. It needs a correction.
