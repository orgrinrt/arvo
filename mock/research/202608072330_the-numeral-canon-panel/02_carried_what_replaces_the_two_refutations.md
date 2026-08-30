# 02. What replaces the two refutations (carried from the closed panel)

**This is a full, unedited copy of `202607301300_formalization-spec-panel/151_rompf_what_replaces_the_two_refutations.md`.**

It was dispatched into the previous panel and delivered **after** this panel was opened, so it is the last file of the closed panel rather than a member of this one. Read it as such:

- Its reading list, its citations and its numbering all point into the closed panel, not into this one. A reference of the form `148:396` means that panel's file 148.
- It was written before op's answers in `01`, so it does not account for them, and in particular it predates the correction to what ratification means.
- It is carried here so this panel is self-contained, and because its two answers are live. It is not a precedent for this panel's format.

Its probes are at `02_probes/`, copied whole from `151_probes/`.

---

# 151. What replaces the two refutations

**Date:** 2026-08-07
**Position:** after `150_knuth_what_structure_the_numerals_form.md`. Supplies the positive answers to the
two questions `SETTLED.md:106-107` carries as open, where the refutation is settled and the replacement is not.
**Probes:** `151_probes/`, four Python instruments and six Rust ones, with their outputs beside them.
`151_probes/RUN.md` carries every command and its exit code, including the two Rust probes whose
refusal is the finding, so an expected refusal is distinguishable from a broken probe.
**Method:** both answers were derived, written down and probed before `145`, `146` or `148` were opened on
either question. `SETTLED.md` and `150` were read first because the dispatch names them, and `SETTLED_strategy.md`
was read for the ratification record on the strategy axis. The order of work is visible in the probe files:
`sign_domain.py` states the four questions it asks before it computes any of them, and
`p2_adjudicator_as_free_key.rs` states its expected refusal in its header.

## Verdict, stated before the argument

**Question one. The sign domain is one of two inputs to the range coordinate, coupled with precision,
touching neither the grid coordinate nor the phase.** That is `148`'s answer and I keep it. What I add is
that it can be proved rather than swept. Two independent arguments close it, one of which needs no
enumeration at all, and the answer survives an ambiguity in the record that neither file noticed: the design
has not said whether `Precision` counts the sign digit, and its own family table gives both answers on
adjacent rows.

**Question two. No column. A sentence, and the sentence is discharged by putting a possessive on the column
that already exists rather than by prose beside it.** That is `148`'s conclusion and I keep it. What I add is
that the repair is the same repair `146` already proposed for its other finding, so `146`'s two findings do
not need two different remedies; the one it accepted serves both. The sentence is written at section 2.4.

**The commitments-versus-descriptions step: `148` is right, and the conclusion survives.** `146`'s two
identity sentences are glosses on ratified table cells, not ratified statements, and I confirm that against
the ratification record's actual wording rather than against `148`'s report of it. But there is a third
statement, which both files pass over, that op declared in his own words to be an intent outranking its
mechanism. It refutes both alternative readings on its own, and it is the ground the answer should rest on.

**What is genuinely undetermined, and is op's:** whether `Precision` counts the sign digit; and whether the
order's own predicate is amended to identify shapes that denote the same value set, which turns out to be a
precondition for `150`'s open question two rather than a separate matter.

## Gates

**The canon gate.** There is no ratified canon for arvo yet. This panel is writing the first one, and
`SETTLED.md:20-24` says so directly: it is an index seeded from checkpoints, with four thematic sweeps in
flight. So the defend-the-canon posture has no target here and the governing material is the narrower set
that records op in the loop: the acceptance criterion at `135b:12-16`, the six ratified rows at
`SETTLED.md:45-50`, the six at `:56-61`, and the ten survivors of `SETTLED_strategy.md`. Everything else in
the panel, including `145`, `146`, `148`, `150` and this file, is agent output and is presumed wrong where it
conflicts with those. Nothing below asks for anything the ratified material forbids. Gate passed.

**The test gate, and what it has to work with.** The suite exists: 16 crates under `mock/crates`, 91 files
under a `tests` path, 83 files containing `#[test]`, all counted with `find` and `grep` piped to `wc -l`. It
has nothing to say about either question, and that is a measurement rather than an excuse. Grepping the
shipped source for the terms the two questions are about:

| term | occurrences in `mock/crates/**/*.rs` |
|---|---:|
| `SignDomain` | 0 |
| `AsymmetricLow` | 0 |
| `Quantisation` | 0 |
| `quantise` | 0 |
| `trait Numeral` | 0 |
| `adjudicat` | 0 |
| `NonNegative` | 25, every one of them `IsNonNegative`, the value predicate in `arvo-numeric-contracts` |

Produced by `for pat in ...; do grep -rn --include='*.rs' "$pat" mock/crates | wc -l; done`. The last row is
the one worth stating: the only hit is a different concept wearing a similar name, and a reader counting
occurrences without opening one would have recorded the sign domain as present in source.

So there is no suite to audit for these questions, because there is nothing under test. The brief also
declares `mock/crates` to be nuked and forbids citing it as evidence about what is correct, which is the
mutation order in `the-canon-design-code-chain.md` doing its job: the tier has to be detached for canon work
to be permitted at all. Running the suite would have measured a tier that has been declared dead and
produced a number with no bearing on either question. I did not run it, and I am saying so rather than
implying it passed.

**The brief's cheap factual claims, checked before reasoning from them.** `rust-toolchain.toml` pins
`nightly-2026-05-28`, and `rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f
2026-05-27)`. Both as stated. The design does declare three sign domains: `110:915` and `124:888` both carry
`type Domain: SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a value fact`. `146` and `148` do
both refute the partition reading, at `146:186-188` and `148:311`. And `146`'s cited probe
`146_probes/n1_quantise_key.rs` is checked into the repository, so its counts are citable rather than void.

One thing in the brief is worth qualifying rather than accepting. It says `146` "makes it a coordinate of the
order in its own right". `146` does write "the sign domain is a coordinate of the order and not a partition
of it" at `146:186-188`, and eight lines later it writes "**With three sign domains the order is not
componentwise at all**" and measures 91 failures. Those two sentences are in tension inside one section, and
the second is the one that survives. So the disagreement between the two files is narrower than the framing
suggests: it is about what word to use for a thing they both measured the same way, and about whether the
positive statement gets written down at all. That matters for how this file should be read: not as
adjudicating a dispute, but as supplying the sentence neither wrote.

---

# 1. Question one: what the sign domain is

## 1.1 The order does not mention it, and that is where to start

The four-condition order is settled, agreed by two experts and adopted unchanged by `150:86-94`. Written on
the value set of a numeral whose points are equally spaced with step $q$, phase $b$, floor $L$ and
ceiling $G$:

$$V_1 \subseteq V_2 \iff
  \underbrace{q_2 \mid q_1}_{\text{grid}} \wedge
  \underbrace{b_1 \equiv b_2 \!\!\pmod{q_2}}_{\text{phase}} \wedge
  \underbrace{L_2 \le L_1}_{\text{floor}} \wedge
  \underbrace{G_1 \le G_2}_{\text{ceiling}}$$

**The sign domain does not appear.** The order is a function of $(q, b, L, G)$ and of nothing else. So the
question "is the sign domain a coordinate of the order" has a mechanical test rather than a taste: a
coordinate is something the order is separately monotone in, and a presentation parameter is something that
feeds a coordinate. The sign domain is one of these two and the arithmetic decides which.

That reframing is the whole of section 1, and it is the same move `150:69` makes for the structural question:
separate the order from the shape space that feeds it, and the disagreement stops being a disagreement.

## 1.2 Two arguments, and one of them needs no enumeration

**Argument A, from antisymmetry.** A coordinate of a product order must carry a partial order of its own, and
the inclusion order forces two relations on the three domains that no partial order can carry at once.

At equal radix, equal precision and equal grid, `Symmetric` is contained in `AsymmetricLow` and not the
reverse, since the latter reaches exactly one quantum further below zero. That forces
$\mathrm{Sym} \le \mathrm{Asym}$.

At the same grid, `AsymmetricLow` at some precision is contained in `Symmetric` at a higher precision, since
raising the precision buys reach on both sides. That forces $\mathrm{Asym} \le \mathrm{Sym}$, because the
precision component is already satisfied and a product order would then need the domain component satisfied
too.

Antisymmetry gives $\mathrm{Sym} = \mathrm{Asym}$, and the two denote different value sets. So **no partial
order on the three sign domains is a factor of the inclusion order**, and the sign domain is not a coordinate
in the product sense.

The argument is four lines and it quantifies over nothing. It is the durable form, and the probes below are
its instances rather than its support.

**Argument B, from the quotient.** `150:55` defines the poset by quotienting the preorder by $V(a) = V(b)$,
which is forced: two numerals denoting the same values are the same point of the order. A coordinate of that
order must therefore be recoverable from the value set. The sign domain is not, because at an odd radix
`Symmetric` and `AsymmetricLow` denote the same value set. At radix three, precision one, the code count is
three, and both domains place one code below zero and one above, giving $\{-1, 0, 1\}$ under each.

Argument B is the sharper of the two where it applies, because it closes the question without ever asking
whether the order is componentwise. It is also the narrower: it holds under one of the two readings of
`Precision` discussed in section 1.4 and not the other. **Argument A holds under both.** Where the two
disagree in scope I rely on A and report B as the additional instance it is.

## 1.3 The probes

Four instruments, in `151_probes/`, with their outputs committed beside them.

**`sign_domain.py`** builds the three domains as value sets over radices two, three and ten, precisions zero
through five, and four grids, and asks the four questions its header states. Every candidate partial order on
three labelled elements is enumerated (19 of them, generated rather than listed) and tested. Output in
`sign_domain.out`.

**`sign_domain2.py`** exists because the first run's witnesses were all worthless. Every collision and every
factorisation failure `sign_domain.py` printed was at precision zero, where all three domains denote the zero
set and any two things agree. A conclusion resting on that rests on a degeneracy, so the whole thing is re-run
over numerals carrying at least two values. Output in `sign_domain2.out`. **Recording this is the point of
recording it:** the first instrument returned the right answer for the wrong reason, and if the second had
not been built the file would have shipped a true claim with a fake witness.

**`sign_domain3.py`** re-runs the factorisation test over the non-degenerate region. A first version
materialised every value set and did not finish inside two minutes at radix ten, so it decides inclusion from
the shape alone; the structural predicate is cross-checked against materialised sets before it is used.
Output in `sign_domain3.out`:

| reading | radix | non-degenerate numerals | partial orders tried | surviving |
|---|---:|---:|---:|---:|
| P1 | 2 | 80 | 19 | **0** |
| P1 | 3 | 60 | 19 | **0** |
| P1 | 10 | 36 | 19 | **0** |
| P2 | 2 | 88 | 19 | **0** |
| P2 | 3 | 64 | 19 | **0** |
| P2 | 10 | 40 | 19 | **0** |

and in every one of the six the forced relation set contains both `Sym<=Asy` and `Asy<=Sym`, which is
argument A arriving as a measurement. The non-degenerate witnesses it names, at radix two under P1:
`Symmetric(p=2)` spanning $[-1, 1]$ quanta strictly inside `AsymmetricLow(p=2)` spanning $[-2, 1]$, and
`AsymmetricLow(p=1)` spanning $[-1, 0]$ strictly inside `Symmetric(p=2)` spanning $[-1, 1]$.

**`sign_domain3.py`'s crosscheck reported 48 disagreements out of 576 ordered pairs**, and that is a defect in
my own instrument reported rather than smoothed over. `sign_domain4.py` locates it, and the result is section
1.6, which turns out to be worth more than the thing it was checking.

**`p5_domain_derives_range.rs` and `p6_run_checks.rs`** are the third instrument and they are independent of
the other two: the endpoints are derived inside Rust's type system from radix, precision and domain, with no
Python and no set materialised. Compiled and run under the pin, gate-free. The six relations come out as
argument A predicts, and the radix-three collapse reproduces:

```
   Symmetric(4) inside AsymmetricLow(4)     true
   AsymmetricLow(4) inside Symmetric(4)     false
   AsymmetricLow(4) inside Symmetric(5)     true
   Symmetric(5) inside AsymmetricLow(4)     false
   NonNegative(4) inside AsymmetricLow(4)   false
   AsymmetricLow(4) inside NonNegative(4)   false

   NonNegative(4)                  codes=16    lo=0     hi=15
   Symmetric(4)                    codes=16    lo=-7    hi=7
   AsymmetricLow(4)                codes=16    lo=-8    hi=7
   Symmetric(1)                    codes=2     lo=0     hi=0
   Symmetric(2) at radix three     codes=9     lo=-4    hi=4
   AsymmetricLow(2) at radix three codes=9     lo=-4    hi=4
```

The `Symmetric(4)` and `AsymmetricLow(4)` rows reproduce the ratified `SC_SAT_SYM` cell restored at
`110:1029-1031`, where the identical `TowardNegative` clamp delivers $-8$ under `AsymmetricLow` and $-7$
under `Symmetric`. That the derived endpoints land on a ratified number nobody fed them is the one check in
this section that could have caught a wrong model, and it did not fire.

## 1.4 What it is instead, stated for the canon

> The numeral's declared members determine a value set, and the order is defined on value sets. The radix and
> the exponent form determine the grid and the phase. The sign domain determines nothing on its own: together
> with the precision and the radix it fixes where the value set's two endpoints fall, and it moves neither the
> grid nor the phase. It is a presentation parameter of the range, not a coordinate of the order, and no
> partial order on the sign domains is a factor of the inclusion order.

Permanence: still true after any rewrite, since it is a statement about what determines what. Equivalence:
three teams implementing it produce the same value sets and the same order, because the endpoints are
determined and the order is defined on them.

**And one thing the record has not said, which this sentence needs before it can be made precise.** The design
has not settled whether `Precision` counts the sign digit, and its own family table gives both answers on
adjacent rows at `138:92-96`: `IFixed<I, F>` carries precision `1 + I + F`, the sign digit counted, while
`FastFloat<P, EMIN, EMAX>` carries precision `P` with IEEE 754's sign bit outside it. The two readings differ
in what the sign domain does:

| | what the domain moves | the three domains at equal precision and grid |
|---|---|---|
| **P1**, sign digit inside precision | both endpoints | `Symmetric` under `AsymmetricLow`, `NonNegative` incomparable to both |
| **P2**, sign outside precision | the floor only | a chain, `NonNegative` under `Symmetric` under `AsymmetricLow` |

measured at `sign_domain2.py`'s section C. Everything in section 1 is computed under both and comes out the
same, which is deliberate: the answer must not depend on a question the record has not answered. But
`148:309-312`'s sentence, that the sign domain and the precision "are coupled because both move the same
endpoints", is true under P1 and half true under P2, where the domain moves the floor and the precision moves
both. That is worth a line in the canon rather than a footnote, because it is the difference between a
consumer writing `IFixed<3, 4>` and getting eight integer bits or seven.

## 1.5 What the third domain does

The dispatch asks what the third domain does to an answer formed while thinking of two. `146` did not form its
answer that way and says so at `146:178-181`, so this is not a correction to it. Five things follow from the
third domain that neither file records, and the last two are the ones that reach other open questions.

**One. It makes the word "signed" ambiguous, and the panel has already used it for two different value sets.**
`138:92` declares `IFixed<I, F>` to have `Domain = Symmetric`, equal reach in both directions. `150:110-111`
models the signed fixed-point family as $(L, G) = (-2^{I-1},\ 2^{I-1} - 2^{-F})$, which reaches one quantum
further down than up and is `AsymmetricLow`, the two's complement shape. Both files are careful and neither is
wrong on its own terms; they are using one word for two of the three declared domains. With two domains
"signed" is unambiguous and the ambiguity cannot arise. With three it is the default failure, and it has
already happened twice in this panel.

**Two. `Symmetric` sits strictly below `AsymmetricLow` at equal precision and equal grid, differing by exactly
one value.** That is an order relation *inside* a precision level. A two-domain reading has no instance of one,
because a non-negative and a signed numeral at equal precision are always incomparable. This is a sharper
refutation of the partition reading than the one on record: `146:183-185` refutes it by exhibiting a
cross-precision containment, and the third domain refutes it at the smallest scale the order has.

**Three. It is the only domain that leaves a code unspent.** Measured at `sign_domain.py` Q5, at precision
three: `NonNegative` and `AsymmetricLow` spend every code at every radix tested, and `Symmetric` leaves
exactly one at radix two and radix ten under both readings. That spare code is where the negative zero, the
repurposing clause and the whole `SignIndexing` axis come from, and it is why `110:1033` records the split of
a single three-instance `Sign` axis into `SignDomain` (a value fact) and `SignIndexing` (a datum fact) as
having a payoff. **The third domain is the reason that split had to happen**, and a two-domain reading makes
the split look like tidiness.

**Four. Under P1 at radix two it supplies the order's bottom without a zero-width numeral.** `Symmetric` at
precision one has two codes and denotes exactly $\{0\}$, confirmed in Python at `sign_domain2.py` section D
and independently in Rust at `p6_run_checks`'s `Symmetric(1) codes=2 lo=0 hi=0`. `150:377-379` hands op the
question of whether a zero-width numeral exists, on the ground that the meet's existence turns on it. Under P1
the answer may already be forced by the three-domain declaration, at no cost in vocabulary. Under P2 it is
not: no numeral at precision one or more denotes $\{0\}$ under that reading. So this is a fifth thing the
`Precision` question decides, and it decides one that was already in front of op.

**Five. At an odd radix two of the three domains collapse.** Under P1, `Symmetric` and `AsymmetricLow` denote
the same value set at every precision at radix three: 20 non-degenerate collisions in the box, at
`sign_domain2.py` section A, with `Symmetric(2)` and `AsymmetricLow(2)` both spanning $[-4, 4]$ confirmed in
Rust. So the three domains are three *declarations* and not always three value sets, and how many blocks they
cut the space into depends on the radix. That is argument B, and it is the cleanest available statement of why
"coordinate" is the wrong word: a coordinate cannot vary in arity with a different member's value.

## 1.6 A correction owed to the four-condition order, and it reaches `150`'s open question

`sign_domain3.py`'s crosscheck between my structural predicate and materialised value sets reported 48
disagreements out of 576 ordered pairs at radix two, and I did not use the predicate until I knew why.
`sign_domain4.py` classifies every one:

| reading | radix | ordered pairs | disagreements | source carries fewer than two values | unexplained |
|---|---:|---:|---:|---:|---:|
| P1 | 2 | 576 | 48 | 48 | **0** |
| P1 | 3 | 576 | 36 | 36 | **0** |
| P1 | 10 | 576 | 36 | 36 | **0** |
| P2 | 2, 3, 10 | 576 each | 24 each | 24 each | **0** |

and restricted to pairs where both sides carry two or more values, the disagreement count is zero everywhere.
So the predicate is sound over the region section 1.3's result is computed over, and the defect is entirely
one thing:

**The grid and phase clauses of the four-condition order are vacuous on a numeral carrying fewer than two
values.** A singleton lies on every grid and in every phase. So the four conditions are *sufficient* for
inclusion always, and *necessary* only where the source carries at least two values. As a characterisation of
inclusion they are correct on the non-degenerate region and strictly too strong below it.

`SETTLED.md:77` records the four-condition form as settled by two experts, in the words "Inclusion between
numerals needs the grid, phase and both endpoint conditions", and that row is right about the direction it was
established in: the two-condition form is unsound and admits conversions that lose values. Nothing above
touches that. What is owed is the qualification on the other direction, which nobody had reason to look for
because nobody was looking at singletons.

**And it turns `150`'s open question two from one question into two.** `150:377-379` asks whether a
zero-width numeral exists, on the ground that admitting one makes the meet total. Admitting it is not
sufficient. A numeral denoting $\{0\}$ has to be declared at some grid, and under the four conditions as
written, $\{0\}$ declared at a fine grid is not below a coarse-grid numeral, though its value set plainly is.
So the shape space gains a bottom and the order predicate does not see it as one. Either the predicate is
amended to test inclusion rather than to test the four clauses, or the quotient at `150:55` is made part of
the predicate rather than part of the surrounding prose. That is a real precondition on a call already in
front of op, and it is cheap: one clause, and it changes no answer above the degenerate region.

## 1.7 Where `146` and `148` land

**`148`'s answer is kept, in full.** Its statement at `148:309-312`, that the sign domain is one of two inputs
to the range coordinate with precision the other, and that it touches the grid and the phase not at all, is
what I derived independently and what three instruments confirm. Its self-assessment is also right: it says
the statement is "sharper than either file's and it is checkable", and it is both.

What I add is that `148` offers it as a sweep, and it does not need one. Its evidence is that "sweeping the
sign domain with everything else fixed changes $v^-$ and never changes $q$ or $b$", which is an observation
over a grid, and `150:420-424` has already recorded, twice, that a count over a truncated shape space is worth
nothing here. Argument A quantifies over nothing and closes it, and a canon that states the sentence should
state it with the argument rather than with the sweep behind it.

**`146` is right about everything it measured and its label does not survive.** Its cross-sign containment at
`146:183-185`, its 91 componentwise failures with the third domain at `146:190-196`, and its diagnosis that
this is "the same failure as the phase one wearing different clothes, both cases of a range or grid fact being
encoded in a coordinate that is not the fact" at `146:207-209`, are all correct and the last is the right
mechanism. That sentence and the word "coordinate" eight lines earlier cannot both stand, and `146`'s own
mechanism is what removes the word.

**Neither file is contradicted about the refutation itself.** The sign domain is not a partition, both said so,
and both were right.

---

# 2. Question two: which strategy adjudicates a conversion

## 2.1 What I derived before opening either file

A conversion has a source numeral and a target numeral, each carrying a strategy, and a strategy supplies a
rounding direction for in-range results and a disposition for out-of-range ones. **Enumerate the events a
conversion can raise and the answer falls out of the enumeration.**

The source value is exactly representable in the target: no event, nothing to adjudicate. The source value is
inside the target's range but off its grid: a rounding event, and a direction is wanted. The source value is
outside the target's range: an over-range or under-range event, and a disposition is wanted. The source
carries a special the target cannot express: a question about what to write where there is no representation.

**Every one of those is a question about the target's grid or the target's range.** There is no fourth kind,
and there is nothing on the source side to decide, because decoding a datum to the exact value it denotes is
total and raises nothing. So the target governs, and the reason is not a preference between two candidates: it
is that only one of the two sides has a question in front of it.

I then attacked that, because it looked too clean, and the attacks are in section 2.6.

## 2.2 The gap is real, and here is its size in the record

`145:755-761` claims the key schema needs no extension, and gives the conversion's key as "the identity
operation marker, the source numeral, the target numeral, the target strategy's five resolutions and its
in-range `Direction`". `146` and `148` both find that the schema, as opposed to `145`'s prose, does not say
whose. Both are right, and the gap is measurable.

Across the three files that carry law tables, the key cells that name resolutions or a direction read, in
full:

```
| source, target, resolutions, `Direction` |
| the three numerals, resolutions, `Direction` |
| the numeral, resolutions |
| source, target, resolutions |
| source, target, `OverRange`, `UnderRange`, `Direction` |
```

Five cells, no possessive on any of them. Produced by the grep recorded in `151_probes/counts.out`. Sweeping
the whole panel for any possessive attached to the word, the only hit is `strategy's five resolutions`, once,
which is `145`'s own prose sentence and not a table cell. So the schema names a resolution set and never says
whose, in every row that names one, for as long as the record has existed. That is the gap, and it is not a
subtlety.

`146` also measures what the gap costs, at `146_probes/n1_quantise_key.rs`, which is checked in: over 331,776
conversions of which 298,368 lossy, the target reading and the source reading disagree on 98,628, and the
target and join readings on 49,314 each. **And on the embedding region all three readings agree at 33,408
checks with zero disagreements.** That last number is the most valuable thing in `146`'s section 6, `148:515`
says so and I agree: it explains why `145`'s three compiled checks could not have found the ambiguity, since
they live exactly where it is invisible. I cite that probe for what it established and have not rebuilt it.

## 2.3 Column or sentence, and this part compiles

`146` asks for a column, `148` for a sentence. The question has a compiled answer under the pin, and the
probes were written before either file was opened on this question, with their expected outcomes in their
headers.

**A column that is a free key member is not expressible.** `p2_adjudicator_as_free_key.rs` adds an
adjudicating strategy as its own parameter on top of the shape `p1_target_keyed.rs` proves compiles, and
changes nothing else:

```
error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
help: use the type parameter `A` in the `Num` type and use it in the type definition
```

rustc's own help line is the finding. An adjudicator can exist only if one of the operands carries it, which
is to say only if it is a member of a numeral. The numeral's members are the ratified four at `110:911-916`,
`Radix`, `Precision`, `Exponent`, `Domain`, and none of them is a strategy.

**A column whose value varies from instance to instance is not expressible either.**
`p4_two_rules.rs` states two adjudication rules over the same operand pair, one taking the target and one the
source, and gets `E0119: conflicting implementations`. The overlap is at the head constructor, not under a
substitution, so it is structural in exactly the sense `SETTLED.md:71` records for the `From` coherence
question. **Adjudication is therefore uniform or it does not exist.** That is worth stating on its own,
because it is what makes a single sentence sufficient: there is no per-pair choice for a column to record.

**A column whose value is derived compiles, and that is why it is not a column.**
`p3_adjudicator_derived.rs` carries the adjudicator as an associated type computed from the pair, compiles
gate-free, and reads `type Governs = TS;` in its only impl. Under `SETTLED_strategy.md:92-104`, which quotes op
declaring it settled canon in his own words, a constant is a function and a stated value holds over a domain.
A function whose value is derivable from members already in the key is not a new member of the key. `148:450`
reaches this and I reach it independently; the compiled version adds that the derivation is expressible under
the pin with no forbidden gate, which the prose version leaves open.

**And the target-keyed shape itself needs no gate.** `p1_target_keyed.rs` carries the source's strategy in the
trait parameter, never mentions it in the body, and compiles with no `#![feature]` line anywhere in the file.
Its runnable form in `p6_run_checks` shows the asymmetry at four values:

```
-- four source strategies, everything else held fixed
   v=   9  [Some(7), Some(7), Some(7), Some(7)]
   v=  20  [Some(7), Some(7), Some(7), Some(7)]
-- four target strategies, everything else held fixed
   v=   9  [Some(-7), Some(7), Some(7), None]
   v=  20  [Some(4), Some(7), Some(7), None]
```

**What that last pair of runs does not prove, said plainly.** The source strategy is inert there because my
impl body does not mention it, which is a property of the model and not a discovery. It demonstrates that the
shape is expressible and self-consistent. The argument that the source strategy *must* be inert is section
2.1's enumeration and section 2.6's ratified ground, and it does not rest on this probe.

## 2.4 The sentence, and the repair it belongs in

`148` asks for a sentence and writes one. I keep the conclusion and would write the sentence differently, for
a reason internal to the schema.

`148:453-455` gives: "A conversion reads its source through the source's lowering and writes its target
through the target's policy. The schema's resolutions column is the target's because the value being produced
is of the target's type; the source contributes the datum's shape and nothing else." That is true. But it
repairs a **law key** by naming `Lowering`, and the schema's own layer rule at `110:1454-1458` says
`Lowering` cannot be named from where laws live. Under the ratified erasure gate at `135b:12-16` the lowering
layer erases, so a sentence licensing a key column should not reach into it. The source's container is a real
fact and it belongs one chapter down, where lowering is the subject.

So, at the layer the key lives at:

> A conversion's `Quantisation` resolutions and its `Direction` are the target numeral's. The source
> contributes the exact value its datum denotes and nothing else, because every event a conversion can raise
> is a question about the target's grid or the target's range, and a strategy determines a numeral's container
> rather than its value set.

Permanence: it survives any rewrite, since it names what determines what. Equivalence: three teams
implementing it produce the same answer for every source datum and target numeral, because the events are
enumerated and each is assigned.

**And the repair is smaller than either file's remedy, because it is one that `146` already accepted.**
`146:427-431` finds the schema's result-numeral guard reads "for a widening operation" where the condition it
is reaching for is "whenever the result numeral is not determined by the operands", and calls that "a repair
to the schema's wording rather than a new column". The resolutions column has the same defect from the same
cause. The schema was written for operations whose result is derived from the operands, where `Resolve`
supplies one strategy and "the resolutions" can only mean that one's. **The resolutions have always been the
result's.** A conversion is the first case where the result is named rather than derived, which is precisely
the condition `146` identified for the other column.

So one clause covers both:

> Where the result numeral is not determined by the operands, the result numeral and its resolutions are both
> named in the key. Where it is determined, `Resolve` discharges both and the key reads as it always did.

That is not a new column, it is not prose beside the schema, and it is not conversion-specific. `146` applied
the wording-repair reading to one of its two findings and asked for a column for the other; its own reading
serves both. **I would put this forward as the shape, and it is a keep of `146`'s method rather than a
rejection of its finding.**

Why a possessive rather than a sentence elsewhere: a column name is checked against every row of every law
table, and a sentence in the surrounding prose is not. Five cells in the current record carry no possessive
and nobody noticed for as long as they have existed, which is the measurement in section 2.2 and is itself the
argument for putting the fix where a reader cannot pass it.

## 2.5 The commitments-versus-descriptions step

The dispatch asks me to test the step and the conclusion separately, and they come apart.

**The step does not hold as stated, and `148` is right about why.** `146:483-499` derives the target reading
by refuting the other two: the source reading "breaks `Precise`" because a `Precise` numeral would hold a
value `Precise`'s own row would have declined to produce, and the join reading "breaks `Hot`" because `Hot` is
"unconditional and infallible by construction". Both require that those identity sentences constrain every
value of the type, rather than describing what the quantiser does under that preset.

Checked against the ratification record rather than against `148`'s report of it. `SETTLED_strategy.md:35-38`
carries op's own words for what each preset is: "`Hot` is as fast as possible, `Cold` stores as small as
possible, `Precise` is the most precise at the price of both storage and compute, `Warm` is the compromise
that suits most default cases and behaves intuitively." Speed, storage, precision, compromise. **Refusal is not
in `Precise`'s ratified intent and infallibility is not in `Hot`'s.** Both come from table cells,
`Refuse`/`Refuse` and `ReduceModulo`/`ReduceModulo`, ratified at `70b:6-23` as cells of a quantiser table.
`146`'s two sentences are the panel's glosses on those cells, and `148:483-486` is right that reading a cell
as a type invariant is the unratified step.

**The conclusion survives, on two grounds.** `148:494-506` supplies one from the preset key and one from the
doubled-storage cell, and I have not rebuilt either; the first in particular is strong, because a preset name
denotes two rows, one per number kind, which is ratified at `SETTLED_strategy.md:65-80`, and applying a
fixed-point row's `ReduceModulo` to a float target is not a bad choice but a category error.

**One gap in that ground, and then the ground that closes it.** `148`'s preset-key argument refutes "take the
source's row". It does not refute a third reading nobody has stated: take the source's preset *name* and
evaluate it against the target's number kind. That reading is well typed at a mixed-kind conversion and
survives the argument as written.

## 2.6 The ground that needs no gloss, and it closes the third reading too

There is one strategy statement op declared, in his own words, to be an intent that outranks its mechanism,
and neither file uses it. `SETTLED_strategy.md:47-50` quotes him at `140b:16-21`:

> My standing call is "It should behave like native primitives in regular old rust would"... The intent, here,
> is what matters. The mechanisms and theory may live freely and shift under and around it, the intent is what
> remains and matters.

Ratified, and restated at `142b:12-13` because it kept failing to stick. `SETTLED_strategy.md:57-63` calls it
"the single most load-bearing survivor in the theme". **This is exactly a commitment rather than a description,
and it says so about itself:** the mechanisms may shift, the intent remains.

Now ask what a Rust primitive conversion does. It is decided by the destination type, in every form the
language has. A narrowing cast truncates according to the destination's width; a fallible conversion fails
according to the destination's range; no Rust value carries an overflow discipline from where it came, and no
Rust conversion consults one. **There is no reading of "behaves like native primitives in regular old rust
would" under which the destination's behaviour depends on the provenance of the source.**

So:

- **The source reading is refuted.** A `Warm` target whose disposition changed with the source's tag is not
  what Rust does, under any of the three ways of taking "the source's": its row, its name evaluated at the
  target's kind, or its name at its own kind.
- **The join reading is refuted by the same sentence.** A `Warm` target acquiring a refusing branch because
  the source happened to be tagged `Precise` is not what Rust does either.

That refutes both alternatives at one strategy, and one strategy is enough, because `p4_two_rules.rs` shows
adjudication cannot vary per pair without a coherence conflict. **A rule refuted at `Warm` is refuted
everywhere.** The composition is a ratified commitment plus a compiled uniformity result, and neither half is
a gloss.

I would put this forward as the ground the canon rests the answer on, ahead of `148`'s two, because `148`'s
first is a typing argument that leaves the third reading standing and its second names a cost rather than a
contradiction. This one is a contradiction with a sentence op declared canon.

## 2.7 Where `146` and `148` land

**`148`'s conclusion is kept: no column.** Its reasoning from `143b` is right, I reached it independently, and
the compiled work in section 2.3 supports it from a direction prose cannot.

**`146`'s finding is kept and its remedy is not.** The gap it found is real, its measurement of the gap is the
most useful thing in the section, and its observation that `145`'s three checks lived where the ambiguity is
invisible should outlive everything else in the dispute. Its remedy asks for a new column where its own repair
to the neighbouring column serves. Its derivation of *which* side governs reaches the right answer by an
argument that does not carry, and `148` is right to say an argument resting on an unratified gloss will be
reopened by the first reader who checks.

**`145`'s conclusion stands and its argument for it should be deleted rather than repaired.** `148:518-520`
says this and I agree independently: "no column is needed, therefore the narrowing is the quantiser" has no
direction, because a derivable fact needs no column whatever the mechanism is. The narrowing is the quantiser
with the operation set to the identity, and that is established by C1, C2 and C3, not by the absence.

---

# 3. What I could not settle, and the routes that closed

Recorded so the next pass starts from a list.

**`148`'s 81 unbiased radix-two join failures.** Still unresolved, still the discrepancy `SETTLED.md:102-105`
names. Nothing here touches it and I did not build a third instrument for it, because it is a question about
the shape space of a different family and my probes are about the sign domain. It stands where `150:392-394`
left it.

**Whether `Precision` counts the sign digit.** I could not settle this from the record. Both readings are
attested in one table at `138:92-96`, no checkpoint rules on it, and `SETTLED.md` does not carry it. Every
result above is computed under both so that nothing here depends on it, but four things do depend on it and
are listed in section 4.

**A route I tried and dropped: rescuing "coordinate" by weakening it.** I spent time asking whether the sign
domain could be a coordinate of a weaker structure than a product order, for instance a coordinate whose order
depends on the precision. It goes nowhere useful, and the reason is worth recording so nobody repeats it: an
order on the domains that varies with another member is not an order on the domains, it is a function of the
pair, which is the input reading with more machinery. The word "coordinate" cannot be saved by weakening it
without saving it into the other answer.

**A route I tried and dropped: settling the adjudication from the erasure gate alone.** The gate at
`135b:12-16` says the typestate erases on lowering, and I hoped that would force the answer directly, on the
ground that anything erased cannot adjudicate. It does not work. Both strategies erase, so erasure tells you
neither of them can adjudicate at the lowering layer and says nothing about which adjudicates above it. What
the gate does give is the smaller point in section 2.4, that a sentence repairing a law key should not reach
into the lowering layer, which is why `148`'s wording is worth changing.

**A route that closed with a diagnostic: the adjudicator as a free key member.** `E0207`, section 2.3. Closed,
with rustc's own help line naming the only alternative.

**A route that closed with a diagnostic: adjudication varying per operand pair.** `E0119`, section 2.3.
Closed structurally rather than by the choice of shapes.

**A defect in my own instrument, found and located rather than absorbed.** `sign_domain3.py`'s crosscheck
disagreed with materialised sets at 48 of 576 pairs, and `sign_domain4.py` shows every one has a singleton
source and none is unexplained. The finding that came out of chasing it, section 1.6, is worth more than the
factorisation result it was checking.

**What is unpriced, and is named as unpriced.** Nothing in this file carries a magnitude. No harness bench
bears on either question, the Rust probes are compile and run checks rather than measurements, and the
Python instruments are enumerations. Any statement here about what something costs would be an ad-hoc quick
spike with no substance, so there are none.

# 4. What is op's

Four calls, in the order they gate each other. Two are new and two are sharpenings of calls already in front
of him.

**One, and it is new: does `Precision` count the sign digit?** The record answers both ways on adjacent rows
of one table at `138:92-96` and no checkpoint rules on it. It decides four things: what the sign domain moves
(both endpoints, or the floor alone); whether the three domains at equal precision form a chain or leave
`NonNegative` incomparable; whether `Symmetric` at precision one denotes exactly the zero set, which bears on
the next call; and whether two of the three domains collapse at an odd radix. None of those is decided by
mathematics and the answer is a statement about what a consumer writing `IFixed<3, 4>` should get.

**Two, a precondition on a call already in front of him.** `150:377-379` asks whether a zero-width numeral
exists, because the meet's existence turns on it. Section 1.6 shows that admitting one is not sufficient: the
four-condition order does not see a singleton declared at a fine grid as being below a coarse-grid numeral,
though its value set is. So the call is really two: admit the shape, and amend the predicate so it identifies
shapes denoting the same value set. The second half is cheap, changes no answer above the degenerate region,
and is not optional if the first half is taken. And under one answer to call one, the shape may already exist
and the vocabulary question may not arise at all.

**Three, whether the sentence at section 1.4 is the one he wants.** It is a keep of `148`'s answer and I have
supplied it an argument rather than a sweep. Two experts now agree on it, independently, which is what the
two-expert rule asks for, and I am the second. What is not settled by that agreement is whether the canon
states it at all, or states only the negative that is already in `SETTLED.md:80`.

**Four, whether the repair at section 2.4 is a wording repair or a column.** `148` and I both say no column,
independently derived, so that is two experts. Where I go past `148` is in saying the sentence should be
discharged by the column's name rather than by prose, and in saying the same clause covers `146`'s other
finding. That part is one expert and is owed a second read. I have deliberately not restated the three
readings of which side governs, because `146` and `148` and now this file all reach the target, and
re-proposing a settled answer is the failure mode the panel's own rules name.

**Owed under the two-expert rule, listed so nothing here is mistaken for settled:** section 1.2's two
arguments, section 1.5's five consequences of the third domain, section 1.6's correction to the
four-condition order, section 2.4's unification of the two schema repairs, and section 2.6's ratified ground
for the adjudication. Each is a first read. Section 1.4's answer and section 2.7's "no column" are the two
that now carry two independent derivations.
