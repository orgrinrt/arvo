# 231. Kiselyov: attacking `229`, and the two places it beat me

`229` is the other independent reading of the axis question and `230` is mine.
Neither saw the other while it was written. This is me reading it afterwards and
attacking it, which is a different job from writing a third reading and I have
not done that.

**The short version, so nobody has to read to the end for it.** `229` is better
than mine on the phrase-by-phrase verdicts and I concede three of them outright.
Its headline result and mine are not in conflict and I was wrong to have written
a denial into mine. One of its verdicts is wrong, and so was the opposing one in
my file, for a reason neither of us saw. And its most actionable finding is four
times larger than it says.

---

## 0. The two gates

**Canon gate: passed.** Against `mock/registry/*.toml`, which `mockspace.toml`
declares as `canon_paths`. Four ratified rows do work below and I name them
where they act: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`,
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`ruling::arms_over_regions_are_the_fundamental_heart` and
`ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`. Nothing
here proposes a declaration; two readings have to agree before anything is
declared and this is a third act rather than a second reading.

**Test gate: run.** The lint pack, which is the surface both files touch:
**540 passed, 0 failed, 13 ignored**, run in this worktree against its own target
directory. That is the same figure `229` reports and I ran it rather than reading
its number. The thirteen ignores all carry `#[ignore = "catalogue: ..."]` naming
their own gap.

`cargo mock test` does not finish, and `229` section 0 is right about why and
right about the defect: it descends into `mock/benches/variants/` and
`bitpack-write-contend-shared` runs thousands of unoptimised four-thread
concurrency trials. Mine has been sitting in that crate for the length of this
sitting, as its did. **I confirm its diagnosis of
`stress.rs:97-112` on its merits**: `naive_kernel_corruption_rate_under_real_concurrency`
asserts nothing, its own comment argues correctly that a scheduler-dependent rate
should not be thresholded, and the conclusion that it should therefore be a
`#[test]` printing to stderr does not follow. It is a bench arm in a test's
clothes and it is charged to everybody who runs the suite.

**One thing in my dispatch, reported under the standing instruction.** The brief
sent me to `229_probes/82_probes/p2_output.txt`. **There is no such path.** The
file is `82_probes/p2_output.txt`, at the panel root, and it belongs to seat 82
rather than to `229`. `229` cites it correctly, at `82_probes/p2_output.txt:27-31`.
The dispatcher's version reads as though the measurement were the reading's own,
and it is a prior seat's, which changes what it is worth: under
`conceding-is-an-answer-and-expert-code-is-a-spike` a probe is one instance of
proof for the one thing it was built to check, and `229` is careful about that
where the brief was not.

---

## 1. What reproduces, before any attacking

`229` makes four claims the coordinator says it verified. I re-measured three of
them with a different reader and checked the fourth against the sources. All four
hold. `231_probes/verify_229.out`.

**527 predicate entries, and the reader matters.** My reader is
`230_probes/entries.sh`, which recovers entries from arrays written on one line
as well as from arrays spread over several lines. That distinction is not
pedantry: **my own first reader lacked it and reported 517**, and two of my
controls passed on the wrong number because they shared the blind spot. `229`'s
reader gets 527 and names 19 distinct axes; mine gets 527 and names 19. Since a
reader without the one-line case cannot reach 527, the two are genuinely
different instruments reaching one number, and the ratified note in
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` reports 527 from
a third. **Three readers, one number.**

- **Zero of 527 entries carry a warrant token.** Reproduced, with a planted
  `total_width: W in 3..=7: swept, the whole span` that the matcher finds, so the
  zero is a fact about the corpus rather than about the pipeline. The mechanism
  is ratified and unexercised.
- **`access_pattern`, `ambient_domain` and `occupancy` are named by no predicate
  entry.** Reproduced exactly, with `total_width` at 65 as the positive control.
  This one is also in my own file from a third instrument, so it now has two
  independent measurements and no dissent.
- **Nothing enforces append-only on `dimension.toml`.** Reproduced two ways. The
  word grep returns zero with `keywords` in four files as its positive control.
  And a structurally different arm: **only two lints read the `dimension`
  namespace at all**, `every_predicate_names_a_declared_axis.rs` and
  `panel_corpus.rs`, and neither compares the set against any stored previous
  state. The one hit for history-shaped words in those two files is a sentence in
  a doc comment. The pattern finds real ratchet state in seven other lints, so it
  works. **The header says a check pins the set, twice, and no check exists.**
- **`term shapes` is named by no declared axis.** Checked against the 22 rows and
  their `keywords`. Nothing carries association shape. `chain_length` is a count,
  `arity` is operands per operation, `operation` is which operation.

---

## 2. The headline disagreement, and it dissolves

`229` names `term shapes` its strongest result. I named `leaf_aliasing` and wrote
that the axis "is not the right name for it". **Both instruments are sound and
neither varied the other's coordinate**, which is exactly the shape where two
readings look like they disagree and are in fact silent about each other.

- `82_probes/p2` section 3 draws eight independent operands. **Every leaf is
  distinct in all four million samples**, so aliasing never moves and the
  instrument cannot report on it.
- `111`'s condition, "every leaf occurs at most once", is stated across both
  discharge checks, so **association never moves there** and that instrument
  cannot report on association.

### 2.1 The instrument that varies both

`231_probes/shape_vs_aliasing.rs`, exhaustive at width 4 signed, saturating at
every step, on `l1 - l2 + l3 - l4` under two associations and two aliasings. Two
questions at every cell: does the **value** depend on the association, which is
`82`'s question, and is the **corner rule exact**, which is `111`'s.

```
### Q1: does the VALUE depend on the association?
declared window          aliasing      divergent           of
full range, straddles    distinct          16368        65536
full range, straddles    aliased             820         4096
non-negative             distinct            588         4096
non-negative             aliased               0          512
non-positive             distinct            996         6561
non-positive             aliased               8          729
narrow [-1, 1]           distinct              0           81
narrow [-1, 1]           aliased               0           27

### Q2: is the corner rule exact?
non-negative             distinct   left     exact    [-8, 7]   [-8, 7]
non-negative             distinct   tree     exact    [-8, 7]   [-8, 7]
non-negative             aliased    left     WIDE     [-8, 7]   [-7, 7]
non-negative             aliased    tree     WIDE     [-8, 7]   [-7, 7]
narrow [-1, 1]           distinct   left     exact    [-4, 4]   [-4, 4]
narrow [-1, 1]           distinct   tree     exact    [-4, 4]   [-4, 4]
narrow [-1, 1]           aliased    left     WIDE     [-4, 4]   [-2, 2]
narrow [-1, 1]           aliased    tree     WIDE     [-4, 4]   [-2, 2]
```

Four controls, all passing. K1 asserts the aliased term really has one fewer free
variable than leaf positions rather than assuming it. K3 checks that without
clamping the two associations agree everywhere, so they are re-associations of
one expression and Q1 is measuring association rather than two different terms.
K4 requires the corner rule to be exact for the distinct term in every window, so
a WIDE reading is a finding rather than a broken corner computation. K2 requires
zero divergence at a window narrow enough that no clamp is reachable, so Q1 is
measuring the association rather than the arithmetic.

### 2.2 Two cells settle it, and they point in opposite directions

- **`non-negative, distinct`**: Q1 is 588 of 4096, Q2 is exact. **The association
  moves the answer and the aliasing is inert.**
- **`narrow [-1, 1], aliased`**: Q1 is 0 of 27, Q2 is WIDE. **The aliasing moves
  the answer and the association is inert.**

One witness each way. **Neither coordinate subsumes the other.** And the stronger
statement is in the second table: **Q2's verdict is identical for `left` and
`tree` in all sixteen rows**. Corner-rule exactness does not move with
association at all, anywhere in the enumeration.

### 2.3 Which of us was wrong

**I was, and about the part I stated most confidently.** `230` §3.4 says
"`leaf_aliasing` is an axis, on the corpus's own evidence, and `term shape` is
not the right name for it". The first clause survives. **The second is a denial I
had no instrument for**, made from a corpus reading rather than a measurement,
and `229` had the measurement. Association shape is an axis and I said it was not.

**`229` is right about association and its naming folds two things into one.**
Its own §7 puts this at the top of what a second reader should attack, and it is
right to: `119`'s `term shapes = every term at 2 and 3 leaf slots **with every
leaf identification**` writes both coordinates under one key, so a single
`term_shape` row would inherit that fusion. **The once-per-axis lint is what makes
that consequential**: `a_predicate_names_an_axis_once` is a hard error, so a claim
ranging over every association at fixed aliasing cannot say both under one slug.

**So the answer is two rows and not one, and neither of our names is quite
right.** `association` or `fold_shape` for the first; `leaf_aliasing` for the
second. Whether the first should also carry the operator labelling of a term's
nodes is a real question and it is in §6.

### 2.4 What my probe is not

**It is not a reproduction of `82_probes/p2`.** Mine is width 4, four leaves,
mixed `+`/`-`, exhaustive; theirs is width 8, eight leaves, addition, sampled at
four million. The visible consequence is in the table: at the **non-negative**
window `82` reports 0 divergent of 4,000,000 and I report 588 of 4096. Both are
right. A one-sided operand window kills association-dependence for a pure
addition fold, because no partial sum can clamp at the far end; it does not kill
it for a signature containing subtraction, because a subtraction walks back
toward the other bound.

**`229`'s predicate already honours that** and says `operation = add`. The
sentence around it does not, and a reader taking the 0% row as a general fact
about one-sided windows would be wrong. That is a limit on the operand-window
half of its finding rather than on the association half, and its predicate is the
part that got it right.

---

## 3. Where `229` beat me, conceded

Three verdicts. In each I opened the evidence it cites before conceding, per the
rule that a claim does not travel on somebody else's reading of it.

**`selector` is an axis and I left it unsettled.** `146:473` reads: "`144` F144-8
measures the plain one selecting a dominated arm at 11 of 41 weights and the
augmented one at 0." A claim true at one value and false at the other, measured.
I declined it in `230` §6.1 because I could not decide whether the selector is
per-strategy or design-wide, and **that question does not bear on whether it is an
axis**, which is what I was asked. `229` did not need to answer it and I made it a
precondition. One rider survives from my §6.1 and it is a rider rather than a
disagreement: under `230` §2's criterion an axis stops being one when a ruling
closes the fork, so if
`question::which_selector_does_the_design_ship_linear_or_augmented_chebyshev` is
answered, this row becomes a design constant the way the container premise did.
That is a statement about its future and not about today.

**`baseline` is an axis and I folded it into `strategy` without checking.**
`146:530-532` states the no-op condition at `baseline uniform across coordinates`
with "every non-uniform baseline tried moving a selection on the first arm set
drawn". True at one value, false at others. **And my own argument for folding it
into `strategy` is unavailable for a reason neither file states**, which is §6.3.

**`cost coordinates` is an axis and I called it a parameter.** `229`'s route is
the decisive one: `146:521` writes `cost coordinates any` under an
order-preservation theorem, `146:525` writes `= 3`, `146:492` writes `in {2, 3}`.
**All three notation states on one phrase.** A run parameter does not get written
`any` with a theorem behind it. And `151:360-363` says "three cost coordinates is
what `139` proposes", so it is a design proposal. I leaned on
`dimension.toml`'s own parameter example and on three `one_expert` proposal notes
that all say the same thing, which is agreement among unratified artifacts and is
worth what that is worth.

**One thing I can add to it rather than only conceding.** `230_probes/who_writes_it.out`
counts distinct personas writing a phrase as a predicate key across all 60
predicate-bearing files: **`cost coordinates` is written by 7 personas in 9
files**. That is an independent route to the same conclusion, and it is genuinely
independent, because it varies author and `229`'s route varies notation state.
**It is also weaker than `229`'s and I want that stated**: author spread shows the
phrase is not one seat's habit and is silent on whether a claim moves along it.
The two agree over the intersection of nothing; they are two facts about one
phrase, and only `229`'s is evidence that it indexes a situation.

---

## 4. Where `229` is wrong, and where I was wrong about the same thing

### 4.1 `or unsigned with signed` is not portable, and it does not want the row I proposed either

`229` §2 calls the phrase a tokenisation artifact, agrees with
`span_verdicts.sh`'s footer that the honest portable count is 5 of 64 rather than
4, and puts it in its summary table as "not a key". `230` §1.4 says the opposite:
the spans are not portable and want an `intermediate_signedness` row.

**Both are wrong.** The source value at `132:359-360`, `136:373-374` and
`138:49-50` is

```
signedness = signed, or unsigned with signed intermediates
```

which names a region over **two** coordinates: the declared format's signedness
and the intermediate's. `231_probes/rectangles.rs` enumerates every region over
that two-coordinate space and every product of per-coordinate spans:

```
### the region `138 #1` states
  {unsigned/signed, signed/unsigned, signed/signed}
  expressible as a product of per-axis spans: NO

### the nearest products, and what each gets wrong
  {unsigned/signed, signed/signed}    claims too much: []  misses: ["signed/unsigned"]
  {signed/unsigned, signed/signed}    claims too much: []  misses: ["unsigned/signed"]
  {unsigned/*, signed/*}              claims too much: ["unsigned/unsigned"]  misses: []
```

Four controls pass: a region that is a product comes out expressible, the empty
and full regions do, the enumeration's own count of ten distinct products from
sixteen subset pairs checks out, and six of sixteen subsets are not products, so
the notation does not express everything.

**Why this is structural rather than a matter of taste.** `a_predicate_names_an_axis_once`
is a hard-error `RepoLint`, so a predicate carries at most one entry per axis, so
**the region a predicate denotes is the Cartesian product of its per-axis spans.
A product of spans is a rectangle.** The region here is a rectangle with one
corner removed, and no rectangle equals it.

**So `229`'s repair writes a wrong region** whichever way a reader parses the
prose, and it does it while smuggling an undeclared coordinate inside a declared
slug, which is the first of the two violation classes
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` names in its own
`note`. **And my repair does not help**: adding `intermediate_signedness` leaves
the region still non-rectangular.

**The construction that works is already ratified and is not a notation change.**
`ruling::arms_over_regions_are_the_fundamental_heart`, `rung = "ratified"`,
`ratified_by = "both"`: the space is filled by small arms holding where they are
optimal and nowhere else, composed. **A disjunctive region is two arms, not one
predicate.**

```
arm 1   signedness: signedness = signed
arm 2   signedness: signedness = unsigned
        intermediate_signedness: signed
```

Two rectangles whose union is the region. **The `intermediate_signedness` row is
still needed, for arm 2, so my §1.4 was half right and stopped one step early;
what it was missing is that no single predicate holds the region however many
rows exist.** The honest portable count is neither 4 nor 5: the three spans are
writable as six rows rather than three, and that is a fact about arms rather than
about the ranking.

### 4.2 One thing in `229` §2 I would not put that way

It says the coverage/region split in `span_verdicts.sh` "is conservative in the
right direction: anything not on the coverage list blocks rather than being
silently dropped". True of that script. **It is the wrong direction for the
ranking built on top of it**, because a phrase that blocks is a phrase that gets
counted, and the ranking's whole output is a count of blockers. Conservatism in
the classifier inflates the ranking, and the ranking is what a reader takes a
declaration from. `229`'s own §3.2 measures that inflation at eight phrases for
one axis's values, so it has the fact and files it under the wrong sign.

---

## 5. Where we agree, and whether the agreement is worth anything

The coordinator asked which shared verdicts are independent and over which values
each instrument ranged. Answering honestly costs more than asserting convergence.

| verdict | `229`'s route | mine | independent? |
|---|---|---|---|
| `declarations` / `restrictions` are `operand_window`, grammar too narrow | prose at `119:212` reading them as declared extents | the four values read against `operand_window`'s `what` | **Partly.** Same nine spans, different sentences inside them. Neither varied anything |
| `overflow limit read at the declared width` is not an axis | the ratified dissolution ruling | the same ruling | **No.** One premise, two readers. It is the right premise and this is not corroboration |
| `discharge check` is not an axis | it names a method | it is the claim's own content in the region slot, `119:495` | **Yes**, and its route is better: a claim true under one check and false under another would be a bug in a check, which closes the question rather than answering it |
| `assignment set` is a product of declared axes | named its three factors | named its three factors | **No.** Same reading twice |
| `coupling` is a candidate axis | the corpus writes it in two notation states | 2 personas, 5 files | **Yes, and both weak.** Neither of us has a measurement. `229` says so plainly and I did not |
| the staged-narrowing family cannot be held | the once-per-axis lint plus a law row's own `gap` | the same lint plus the intermediate-format framing | **Partly.** `229` goes further and prefers `narrowing_schedule`; I abstained. Its preference is better argued than my abstention and I have nothing against it |
| `access_pattern`, `ambient_domain`, `occupancy` unused | its `axis_usage.txt` | my `entries.sh` | **Yes.** Two readers that differ on the one-line-array case, one number |
| `183_probes/axis_census.sh`'s C2 message is now false | read the arm | read the arm | **No.** The same three lines |

**The two `No` rows are the ones that would read as convergence in a
consolidation** and they are one instance each. The `overflow limit` one is
nonetheless safe, because its single instance is a ratified ruling rather than an
agent's reading, which is a different kind of authority from a second opinion.

---

## 6. What neither of us said

### 6.1 The most actionable finding is four times larger than `229` says

`229` §3.6 says clause `146:444` is blocked by a retired phrase **and by nothing
else**, so it is writable today with no declaration. That is right:
`183_probes/blocked_inventory.tsv:63` carries `NO-AXIS` with the single blocker
`overflow limit read at the declared width`.

**It is one of four rows in that file with a single blocker, and every one of the
four is now unblocked:**

```
136:320  5.2 no quantisation is additive off the grid    blocker: domain
136:355  5.4a the family invariance and the commutations blocker: domain
146:444  5.5c a lossless storage container adds no ...   blocker: overflow limit read at the declared width
146:452  5.5d narrowing an accumulator is a cost choice  blocker: accumulator width
```

`domain` is `dimension::ambient_domain`, whose `keywords` carry it.
`accumulator width` is `dimension::accumulator_width`. The third is retired by a
ratified ruling. **Four clauses, four single acts, no declaration and no
argument.**

Two more from `229`'s own list of five cleared clauses have every co-blocker
cleared too: `122:397` on `ambient domain; radix`, and `122:425` on `ambient
domain; radix; restrictions`.

**Of those six, five are writable and one is not, and the difference is worth
keeping.** `122:425`'s `restrictions` value is `upper bounds in {1, 3, 7}`, and
`dimension::operand_window`'s grammar admits `declared non-negative`, `full
range` and `any`. **So `122:425` is blocked by a declared axis's grammar rather
than by a missing axis**, which `229` §3.3 identifies as a class and does not
carry into its writability count. Those want different repairs and only one of
them is free.

### 6.2 `operation`'s `in {...}` has three readings, not two

`229` §3.9 finds two incompatible readings of `operation in {add, mul}`: the
elementwise one a law takes, and the observer-set one a distinguishability claim
needs. That is right and it is a defect in a declared row.

**There is a third and it sits next to the phrase `229` calls its strongest
result.** `119 #7` writes `operations in {add, sub, mul}, term shapes = every
term at 2 and 3 leaf slots with every leaf identification`. Here the set is
**the alphabet the term's nodes are labelled from**. A term using `+` at one node
and `-` at another is not "holds at add and holds at sub", and it is not "an
observer holding both". It is one term over a three-letter signature.

`dimension::operation`'s `moves` licenses only the first: "A law is a statement
relating operations, and a claim about addition transfers to multiplication only
where somebody showed it does." **Seventy-three committed entries name
`operation` and nothing says which reading each takes.**

This bears directly on §2.3's naming question. If the association axis is meant to
carry arbitrary terms over a signature rather than the association of one fixed
operator sequence, then it needs the alphabet, and the alphabet currently lives in
`operation` under an unlicensed third reading.

### 6.3 "It folds into `strategy`" is unavailable, which is why §3's concessions were forced

My reason for calling `selector`, `baseline` and the cost-coordinate set
parameters was that all three are components of a strategy, and
`proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`
says a strategy is a declared semantics plus a weighting.

**That argument cannot be written down.** `dimension::strategy`'s grammar admits
`S = <name>` or `S in {<set>}` and forbids `any`. There is no admissible spelling
for "at every baseline", "at the linear selector" or "at three cost
coordinates" on that slug, because those are not strategy names. **So even if all
three are parts of what a strategy is, `strategy` cannot carry a claim quantified
over them**, and the choice is a separate row or a grammar change on `strategy`.

`229` reaches the right verdict without this and it is what makes the verdict
robust: it holds whether or not the components belong to the strategy.

### 6.4 A number worth keeping beside `229` §1

`229` says the last round of four declarations "bought five clearable clauses and
zero written ones". That is the right shape and here is the matching figure from
the other side: **91 predicate fields, mean 5.8 of 22 axes named, silent on 16.2
on average, and the richest field in the canon names 13**
(`230_probes/axis_uptake.out`). Under the absence rule that is what the canon
currently claims. **Declaring is cheap, porting is not, and neither file found
anybody doing the porting.**

---

## 7. What a third reader should attack

- **§2's probe is one term at one width.** `l1 - l2 + l3 - l4` at width 4,
  exhaustive. The separation it shows is clean and it is one signature. A term
  with multiplication would be the obvious next arm, because the corner rule's
  inexactness under aliasing gets worse with multiplication and the association
  question changes shape.
- **§2.3 says two rows and I have not written either grammar.** Whether
  `association` takes values like `left-nested` and `balanced` or takes a tree
  shape, and whether `leaf_aliasing` takes a two-valued flag or a partition of
  leaf positions, are both open and both consequential under append-only.
- **§4.1's arms answer is a construction and not a measurement.** I have shown
  the region is not a rectangle and shown two rectangles whose union it is. I
  have not checked whether the corpus's other blocked spans are disjunctive in
  the same way, and if many are, the finding is much bigger than three spans.
- **§6.2's third reading of `operation` rests on one span.** `119 #7`. I did not
  sweep for others and there are 73 entries to sweep.
- **The three concessions in §3 were made by opening `229`'s citations.** If any
  of `146:473`, `146:530-532` or `151:360-363` is being read out of context, the
  concessions go with it, and I read the surrounding paragraph for each rather
  than the cited line alone.

---

**Region.** Everything here is about this registry and this panel on
`research/attack-230`, with `dimension.toml` at 22 rows and 527 predicate
entries. The declared vocabulary has no axis over the canon's own contents, which
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` records
and `229` §4 develops, so these regions are prose.

The finding in §2 has one and I write it with the entries the current set cannot
accept, since a predicate that shows its own gap is worth more than one that
hides it:

```
total_width:      W = 4
signedness:       signedness = signed
container:        container = i32 host, values held to [-8, 7]
fraction_width:   F = 0
overflow_policy:  overflow policy = saturate
operation:        operations {add, sub}, as the alphabet the term's nodes carry
arity:            arity = 2
chain_length:     chain length = 3
operand_window:   {full range, non-negative, non-positive, [-1, 1]}: exhaustive,
                  every assignment in each
association:      {left-nested, balanced}: exhaustive, both associations of the
                  operator sequence
leaf_aliasing:    {distinct, one variable at two leaf positions}: exhaustive
threads:          threads = 1
toolchain:        rustc = nightly-2026-05-28, edition 2021
```

Three entries name axes nothing declares: `association`, `leaf_aliasing`, and
`operand_window` carrying a value its grammar does not admit. The `operation`
entry is written in the third reading of §6.2, which is also unlicensed.
`every-predicate-names-a-declared-axis` would refuse this at `HARD_ERROR` and it
is correct to.

**Instruments.** `231_probes/verify_229.sh` (three of `229`'s claims,
re-measured, with a planted-token control and a second structurally different arm
on the append-only question), `231_probes/shape_vs_aliasing.rs` (the 2x2 with
four controls), `231_probes/rectangles.rs` (which regions a product-form
predicate can denote, with four controls). Sources and outputs committed beside
each.
