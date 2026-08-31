# 231. Dolan: the two readings attacked, and what the disagreement turned out to be

I wrote `229`. `230` is the second independent reading and I had not seen it. This attacks it, concedes to
it where it is right about me, and reports one thing neither of us had.

The short version, because the headline disagreement dissolves rather than resolving: **`association` and
`leaf_aliasing` are two orthogonal axes, both real, and neither of us had both.** `230` is right that
"term shape" is the wrong name and right that leaf aliasing is an axis; it is wrong that the compound
contains nothing else. I was wrong to call the compound one axis and wrong to name it after its shape.

## 0. The two gates

**Canon gate: passed.** `mockspace.toml:31` still declares `mock/registry/*.toml` as `canon_paths`, the
set is 22 rows unchanged, and nothing in this file edits it. Two ratified rows govern parts of the answer
and I use both: `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`.

**Test gate: run.** Lint pack 540 passed, 0 failed, 13 ignored, every ignore an `#[ignore = "catalogue:
..."]` naming its gap, all thirteen in the six source-side `CrateLint`s that have no source left to lint.
`cargo mock --lint-only` over the real rows: 695 rows, schema check passed, all lints passed. The bench
tree still does not terminate under `cargo mock test`, unchanged from `229` section 0 and not re-run.

**One gate finding of the ordinary kind.** This worktree was fresh, so `mock/target/hooks/` did not exist
and the first commit was refused outright with `BLOCKED: mockspace is not initialised in this repo`. The
repair is the one the message names, `cargo mock`, which wrote 84 agent files. Worth saying only because
the refusal is loud and correct and a seat that reads it as a broken gate will reach for `--no-verify`.

## 1. What I checked of `230`'s findings, and what each is worth

I ran the checkable ones rather than taking them.

**The census is one persona's: confirmed, and understated.** `230` says the eight files producing spans
are all Leroy's. All **twelve** files `axis_census.sh` names are Leroy's, including the four that produce
zero. That is readable off the filenames without an instrument.

**60 files, 372 paragraphs, 21 personas: confirmed by a second extractor.** Mine reports 62 and 378, and
the difference is exactly `229` and `230` themselves, which quote predicates and so match a `holds for:`
sweep. Removing the two readings from the corpus they are readings of gives 60 and 372, and 21 personas
independently. `231_probes/nonrectangular.out`, control R1.

**`occupancy` at rank zero: confirmed, and I had half of it before reading them.** `229` measured
`occupancy`, `ambient_domain` and `access_pattern` at zero predicate entries in the registry. `230` adds
the other half, zero occurrences in the 372 `holds for:` paragraphs, and draws the inference I did not:
an axis found by measuring what a derivation's output does has rank zero on a frequency ranking of
already-written spans **by construction**, so the ranking cannot find the kind of axis most recently
declared. That is the sharpest single point in either file and it is theirs.

**The axis uptake vector: independently reproduced, all 22 counts.** Their reader is an awk state machine
that tracks array context over all twelve registry files; mine is a flat grep over the four
predicate-bearing ones. Different mechanisms, and every count matches:
`operation` 73, `signedness` 66, `fraction_width` 66, `total_width` 65, `overflow_policy` 61, `arity` 53,
`threads` 36, `target_features` 25, `chain_length` 20, `rounding` 17, `container` 14, `build_profile` 11,
`toolchain` 7, `integer_width` 4, `operand_window` 3, `radix` 2, `alignment` 2, `strategy` 1,
`accumulator_width` 1, and three at zero. `231_probes/uptake_cross_check.out`.

**And my reader is the worse one, which its own control caught.** Control X2 asked whether widening from
four files to twelve adds anything. It does: 529 rather than 527. Both extras are on one line, in the
`note` of `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, which quotes two predicate
entries as **prose examples** of the two dialects. A flat grep cannot tell a quoted example from an array
element and an array-aware reader can. So 527 stands, the lint's `PREDICATE_FIELDS` walk is complete, and
`229`'s figure was right only because it happened to be scoped to four files.

**No predicate names a strategy: confirmed, and the entry is worse than "one".** The single `strategy`
entry reads `strategy: strategy = product order over two independent axes, generic labels rather than
Hot/Warm/Cold/Precise`. `dimension::strategy`'s `grammar` admits `S = <name>` or `S in {<set>}`. That
value is neither, so the count of predicate fields naming a strategy the grammar recognises is **zero**,
not one. Under the absence rule every predicated row in the canon claims to hold nowhere a strategy
exists, which is everywhere in arvo. Their reading, confirmed.

**The container premise is still called blocking in two live rows: confirmed.** `dimension::container`'s
`note` carries "it is blocking: no wording of several downstream clauses is true on both branches" and
`topic::the_container_premise` carries "Blocking: no wording of several downstream clauses is true on both
branches", both after a ratified ruling closed the fork. The `dimension` row is the bad one, for the
reason they give: it is the file every predicate is written against.

## 2. The headline, and it is not the disagreement it looked like

### 2.1 Their citations resolve, with one off by three

`111:1237`, `111:1383`, `114:974`, `119:491` and `122:475` all carry "every leaf occurs at most once"
exactly as cited. The one the coordinator asked me to open, `111:1391`, does not: the sentence
*"Sufficient and not necessary: it does not fire on `x - x` or `(x + y) - x`, both of which are exact"* is
at **`111:1388`**. Three lines, and the quotation is verbatim, so this is a slip rather than a fabrication.
It matters only because that citation is the one their verdict turns on.

### 2.2 Their decomposition has no slot for association, and that is the whole of it

They split the corpus's compound `term shapes = every term at 2 and 3 leaf slots with every leaf
identification` into depth (`chain_length`), operator arity (`arity`), leaf slot count (close enough to
`chain_length`), and a residue, `leaf identification`, which they take as `leaf_aliasing`.

**Nothing in that list is the parenthesisation.** `(a+b)+c` and `a+(b+c)` agree on depth, on arity, on
leaf count and on aliasing. `dimension::chain_length`'s `what` is *"How many operations run before a
result is observed"*, a count, and both terms run two. So the two declared axes they lean on do not pin
association, and their residue does not contain it.

### 2.3 The decisive artifact, exhaustive rather than sampled

`231_probes/p1_shape_vs_aliasing.rs`, W = 4 signed, F = 0, arity 2, one thread, container `i32` wide
enough that only the policy clamps. Two arms, each varying one coordinate with the other held fixed, over
the whole finite domain rather than a sample.

```
## arm A: association varies, every leaf a distinct occurrence
  saturating, full range      : 16268 of 65536 tuples disagree
      witness [-8, -8, -8, 1]  left = -7  balanced = -8

## C1 negative control: same arm under wrapping, must be 0
  wrapping, full range        : 0 of 65536            PASS

## C2 negative control: saturating on a declared non-negative window, must be 0
  saturating, operands in [0, 7] : 0 of 4096          PASS

## arm B: aliasing varies, association fixed at (x op y) op z
  declared operand boxes swept: 136
  (x + y) - z, leaves distinct : rule disagrees with oracle on 0 boxes
  (x + y) - x, leaf 3 aliases 1: rule disagrees with oracle on 5 boxes
      witness box [-4, 1]: rule says may-overflow = true, oracle says false

## C3 negative control: the rule must be exact on the distinct-leaf term
                                                      PASS
```

**Association moves the answer with aliasing fixed. Aliasing moves the verdict with association fixed.**
Three negative controls, all passing, and C2 reproduces the committed `82_probes` result exhaustively
where that one sampled.

### 2.4 The number is not mine and I did not know that when I ran it

`16268 of 65536`, witness `[-8, -8, -8, 1]`, `left = -7`, `tree = -8`. I went looking for that integer
afterwards and found it already in the corpus, at `80_rompf_when_the_deriving_happens.md:169-173`:

```
      Wrap  declares AssocAdd: yes   vectors: 65536   left-fold != tree-fold: 0
 SatSigned  declares AssocAdd: yes   vectors: 65536   left-fold != tree-fold: 16268
            witness [-8, -8, -8, 1]: left=-7 tree=-8

control, arity 2 (no grouping choice exists): Wrap 0, SatSigned 0
```

Same count, same witness, same wrapping control at zero, from a model I wrote without having read that
file. Quoted again at `81_persona_checkpoint_seven.md:120-123`.

**And Rompf's own control is the refutation of `230`'s decomposition, written two hundred files earlier.**
*"control, arity 2 (no grouping choice exists): Wrap 0, SatSigned 0 ... so the instrument is measuring
grouping and not something else."* That control exists precisely because its author knew grouping is a
coordinate separate from arity, and put an arm in to prove the instrument was not reading arity instead.

### 2.5 In `230`'s own currency, association clears the bar better than aliasing does

They settle candidacy partly on a persona count, on the ground that `ambient_domain` and `radix` were
declared at two. Counted the same way over the whole panel, with the two readings of this question
excluded because they are the reading rather than the corpus (`231_probes/who_writes_association.out`):

- **association: 5 personas.** `35_mcsherry`, `60_stam`, `80_rompf`, `82_jhala`, `196_dolan`, plus the
  `81` checkpoint, which is not counted because a checkpoint is not a persona.
- **leaf aliasing: 2 personas.** `111_jhala` and Leroy across `114`, `118`, `119`, `122`.

Both clear the bar. Applied evenly, the criterion ranks the coordinate they rejected above the one they
took. They applied it to one and not the other.

### 2.6 Verdict, and the replacement

**Two axes, orthogonal, neither reducible to the other nor to any declared pair.**

- **`association`.** Which parenthesisation of a fixed operation multiset the claim is about. Values
  `left-nested`, `balanced`, a named enumeration, or `any`. **The payoff is concrete: `association: any` is
  exactly what an associativity law asserts**, and no committed associativity row can say it today.
- **`leaf_aliasing`.** Whether a term's leaves are distinct occurrences or repeat. Values `distinct`,
  `repeated`, `any`. `230`'s case for it stands unchanged and I add nothing to it beyond arm B.

**What I got wrong**: `229` §3.4 called the compound one axis and named it `term shape`, which absorbed
leaf identification into a name that does not describe it. `230` is right about the name and right that
the residue is an axis. **What they got wrong**: subtracting the declared coordinates and stopping at one
residue, when the compound holds two.

**What would change my mind:** a showing that a design fixes its association before any law is stated, so
the coordinate has one value like the container premise now does. Nothing in the registry does that, and
`ruling::arms_over_regions_are_the_fundamental_heart` points the other way, since a reassociated arm is
one of the arms a strategy chooses among.

## 3. Where we converge, and whether the routes are independent

The coordinator asked for this explicitly and the answer is mostly no.

**Shared premise, not corroboration.** `overflow limit read at the declared width` is not an axis: both
readings, and both read the same ratified ruling and drew the same inference from it. That is one instance
wearing two hats. `230` states the general principle better than I did, that a ratified ruling closing a
fork removes an axis, but the principle and my argument are the same argument.

**Shared source, separate arrival.** `declarations` and `restrictions` reducing to `operand_window`, and
`assignment set` decomposing into declared factors. Both readings opened the same spans and the same
`dimension` rows. Weak independence: nothing was measured on either side.

**Genuinely different arguments, same verdict.** `discharge check` is not a region. Mine: it names a
method, so it belongs on the probe row. Theirs: it is the claim's own content sitting in the region slot.
Theirs is the better statement and the two do not share a step.

**Genuinely independent instruments.** The uptake vector, above, and the paragraph census. Those are the
only two places in the two files where agreement is worth the word.

**What neither instance ranged over, which is what the tier is over.** Neither reading varied `strategy`,
`threads`, `target_features`, `toolchain`, `access_pattern` or `occupancy` in any measurement, because no
measurement in either file has more than one value on any of them. Every agreement above therefore holds
at `threads = 1` and says nothing at any other value. **The intersection of what the two instances ranged
over is: the registry as committed, at one thread, on one host, under one toolchain.** A convergence
claim over anything wider is not available from these two files.

## 4. What `230` corrects in me, conceded

**The `span_verdicts.sh` tail correction, and I endorsed it without checking it.** `229` §2 called the
tail *"stated by its author and correct"* and said I had confirmed it independently. I had confirmed the
half under it, that `or unsigned with signed` is a splitter artifact rather than a key, and then relayed
the conclusion built on top, that the three spans are therefore portable and the honest count is 5 of 64.
Opening `132:359-360`, `136:373-374` and `138:49-50`, the intact value is `signedness = signed, or
unsigned with signed intermediates`, a disjunction over the declared signedness **and** the intermediate's,
and there is no declared axis for the second. The spans are not portable and the count is 4.

That is exactly the failure of relaying a finding whose evidence you opened for a different question. The
diagnosis was mine and correct; the conclusion was the author's and I passed it on with my name on it.

**`term shape` as a name, and as one axis.** Conceded above.

**The intermediate as a second format is a better synthesis than my narrowing schedule.** `229` §3.5
proposed one axis whose value is a schedule. `230` §3.7 unifies `F_intermediate` with the intermediate's
*signedness* and says the corpus is repeatedly trying to state coordinates of a second format while the
notation has one entry per axis for the declared one, with `dimension::accumulator_width` already that
format's total width. That subsumes my reading and explains a case mine did not reach.

## 5. Where I correct `230`, beyond the headline

**Their intermediate-format fix is undersized by one.** `132:340` names **three** fraction widths in one
span: `F_exact in {4, 5}; F_intermediate in {2, 3}; F_final in {1, 2}`. A two-row fix
(`intermediate_fraction_width` beside the declared `fraction_width`) holds two of them. And `F_exact` is
not derivable from the others: doubling `F_final in {1, 2}` gives `{2, 4}`, and the source says `{4, 5}`,
so it is stated rather than computed. **Either the fix is three rows, or the design bounds staging at one
intermediate and somebody has to say so.** The corpus never exhibits two intermediates, so the bound may
well be right; it is not written anywhere.

**The `111:1391` citation is at `111:1388`.**

**Their persona criterion, applied evenly, contradicts their own ordering**, per 2.5.

## 6. What neither of us had: the region a predicate cannot name, and is being written anyway

A predicate is a set of per-axis entries and `a-predicate-names-an-axis-once` makes a second entry on
one axis a hard error, on the stated ground that two entries are "two regions with nothing saying
which governs". So the region a predicate names is a cartesian product of its per-axis spans: an
axis-aligned box. A source span naming a **union** of boxes has no form, and splitting it per axis
gives the product, which contains cells the source does not claim.

**That is the opposite failure from the absence rule. Absence narrows, silently, and has a rule. This
widens, silently, and has nothing.** I looked for a ratified sentence saying a region is a product or
licensing a union and there is none, so the product reading is derived from the once-per-axis lint plus
the entry grammar rather than stated. Silence is not permission either way.

### 6.1 In the panel source: three claims, six spans

`231_probes/nonrectangular.out`, over all 60 predicate-bearing files rather than Leroy's eight, by text
search so it survives the three dialects. Four controls passing, one of which (R3) failed on the first
run because the arm read a display file `cut` had truncated before the phrase; that run is kept.

The clearest instance is `132`'s commutation span, whose two sides are both declared axes:

> every deterministic member against saturation, and every translation-equivariant member against wrapping

The region is `(deterministic x {saturate}) union (translation-equivariant x {wrap})`. Written per axis
it becomes `rounding: {deterministic union translation-equivariant}` and `overflow_policy: {saturate,
wrap}`, which additionally claims `(translation-equivariant, saturate)` and `(deterministic, wrap)`.
**The source claims neither cell. Two cells invented by the notation, on a row whose argument kind is
equivariance.**

### 6.2 In the committed canon: ten entries, and one of them is the span 230 says cannot be written

This is the part that matters and neither of us had it. **The corpus does not stop at the notation's
edge. It writes the second region into the values side and ships the row.** `231_probes/
values_side_binds_two.out`, over the same 527 entries every instrument here reads:

```
"signedness: signed, or unsigned with signed intermediates"
"threads: threads = 1 for the timed instance and threads any for the compile-time artifacts"
"arity: arity = 3 for the grouping kind, and arity in 2..=5 for the schedule kind"
"arity: arity = 3 for the algebraic family and 2 for the order family"
"total_width: W in {3, 4} for the end-to-end run, and W in 2..=10 for the absorption sweep"
"fraction_width: F = 0 for the end-to-end run, and F in 0..=W for the absorption sweep"
"operation: ... for the wrapping fragment, and ... for the saturating one"
"build_profile: default const-eval guard, and guard-allowed for the three-bit measurement"
"rounding: rounding = nearest, against a phase-zero mutant"
"arity: arity in {2, 3} for the asserted join laws"
```

Ten entries across eight axes, four shapes. `, or` is a union over two axes. `against` is a pairing of
two. `for the ...` attaches a region to a sub-case that is not an axis at all, usually **which run it
came from**, which is coverage sitting in the region slot. And `, and` states **two regions on one
axis inside one entry**, which is precisely what the once-per-axis lint forbids across two entries.

**`threads: threads = 1 for the timed instance and threads any for the compile-time artifacts`** is the
one to look at twice. It writes both of the notation's positive states for one axis in one entry, and
the second half is the compile-time universal `229` section 4 said nobody had written. Somebody wrote
it. It is unreadable by anything, because it is prose after a colon.

**And the first line is 230's span.** They argue from `132`, `136` and `138` that it is not portable,
because the value is a disjunction over the declared signedness and the intermediate's and no axis
exists for the second. **They are right, and it was ported anyway**, into `law::
quantise_then_reduce_commutes`'s `fails` field. So their finding is not a warning about a port that
might go wrong. It is a description of one that already did.

**Nothing can see any of the ten.** `every-predicate-names-a-declared-axis` says in its own doc that
"The values side is not checked, and deliberately", and `a-predicate-names-an-axis-once` splits on
slugs. Every one of these entries names one declared axis and carries values, so both pass. The
warrant ruling's own note already reports this class at six and says "the shipped arm checks the slug
side only and sees none of them"; this names which ten and what shape each takes.

### 6.3 What this does to the repair

**Declaring the missing axis is necessary and not sufficient.** 230 proposes
`intermediate_signedness`. With it declared, `signed, or unsigned with signed intermediates` is still a
union: the first branch constrains only the declared signedness and the second constrains both, so
per-axis entries still give the product and still over-claim. Their finding and mine are the same
finding at two depths, and the deeper one moves the fix.

I am not proposing the fix. The entry grammar is ratified, op refused a fourth region state for reasons
that hold, and a union of boxes is not a fourth state but it is a change to a ratified grammar, which
is not a seat's call. **What I will say is that the gap is real, that ten rows are already through it,
and that no shipped arm can find them.**

*holds for: the registry as committed at `507dfc95` and the 60 predicate-bearing files of this panel;
threads = 1. Both censuses are text matchers over named constructions and are floors rather than
counts: a span correlating two axes in wording the matcher does not know is invisible to both.*

## 7. What I could not settle

**Whether `association` wants values or a structure.** `{left-nested, balanced, any}` covers every span in
this corpus, and at more than four leaves the parenthesisations are not three but Catalan-many. Whether
the axis is a small named set or a shape language is a design call and the corpus does not force it.

**Whether the non-rectangular gap is worth a grammar change.** It costs a ratified edit and buys three
spans in this corpus. I can state the defect and I cannot price the fix, and pricing it is not a shell
script's job.

**Whether `F_exact` is free or derived.** The arithmetic above says stated, from one span. One span is one
instance.

**What I did not test in `230`, named so the record carries it.** Their section 4.2 rests on a
compiled witness about binding time and `overflow-checks`, with `230_probes/p1_binding_time/` behind
it, and their 4.2.1 says they attacked their own result with a second mechanism and broke one sentence
of it. **I did not re-run any of that**, so everything in my file about the compile-time question is
about `229`'s treatment and the ratified warrant ruling, and says nothing about theirs. Their 3.8
(`feature gates`), 3.6 (`coupling`) and 3.9 I read and did not check; I agree with 3.6 because `229`
reached the same verdict, which is a shared reading of the same spans rather than a second instrument.

**And I did not reach the web at all this round.** Nothing here needed prior art; the corpus carried
its own witnesses.

## 8. Leavings and unlicensed mechanisms

**The `Cargo.toml` at the root of the `arvo-coord` clone is not mine.** I have never invoked cargo with a
package in this work; my only compile is `rustc -O` inside `231_probes`, and the file is dated `Sep 1
01:58`, the same minute as every file in `230_probes/`, which is before this worktree existed. Its content
is `[workspace]`, `name="ctl"`, `[profile.dev] overflow-checks=false`, and `overflow-checks` under
`[profile.dev]` is the exact subject of `230` §4.2.2. `230`'s committed scripts use `rustc` directly and
none of them writes it, so it is an uncommitted leaving of a step that did not get committed. I have not
touched it. A bare `[workspace]` at a repo root changes resolution for everything beneath it and that
clone is shared, so it should go, and it is the other seat's to remove.

**`183_probes/axis_census.sh:119` still prints a sentence it does not check**, `radix: PASS, present as a
bare key and no dimension row declares it`, while `dimension::radix` sits in the file. `230` found this
too. The arm's body is a grep over `keys.txt` and never opens `dimension.toml`.

**The compiled probe binary is gitignored and the build line is in the source**, since it is megabytes and
one `rustc` rebuilds it.

**This worktree sits inside the `arvo-coord` clone rather than beside it**, at
`arvo-coord/.worktrees/axes-attack-229`, from a relative path in the brief resolving against the shell's
directory rather than against `-C`. Flagged by the coordinator, left where it is mid-flight, and named
here so the record carries it.

## 9. What a next seat should attack in this

**Arm B is thin.** It establishes that aliasing moves the interval rule's verdict on 5 of 136 boxes at one
width, on a model of the rule I wrote from the corpus's description rather than from anybody's committed
implementation. The corpus's own instrument is at `111_probes`, and somebody should run this against that
rather than against my reading of it.

**Arm A's model is four leaves and one operation.** Rompf's is the same shape. `82`'s is eight leaves at
width 8, sampled. Nobody has varied the operation, and multiplication under saturation is where the corpus
expects the worst behaviour.

**The persona count is a proxy and a poor one.** It counts who wrote a phrase, not who established
anything, and I used it because `230` used it and a comparison needs one currency. It should not decide
anything on its own.

**The thing I would attack first if I were the next seat** is section 6, and I have already done the check
I was going to leave open: I grepped the registry for a ratified sentence saying a region is a product or
licensing a union, and there is none. So the product reading is derived rather than stated, and a union is
neither licensed nor forbidden. That makes section 6 a finding about a silence, and silence is not
permission in either direction. The ten rows through the gap are facts whatever the silence means.
