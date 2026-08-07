# The laws under the laws

**Date:** 2026-08-07
**Position:** after `135_lamport_the_law_list.md`, which enumerated fifty-four laws and named three sets of
laws that sit underneath its own table. This file writes those three.

**Status: partial, extended section by section. Sections 1 and 2 are complete.**

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

In progress.

## 4. The order laws

In progress.

## 5. M7's three views, resolved

In progress.

## 6. The decoder-ring second read

In progress.

## 7. Attacking the premise, and what is op's

In progress.
