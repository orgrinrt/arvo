# 40. What the axes actually are

**Date:** 2026-08-08. **Author:** the `leijen` persona. **Predecessor:** `39_op_the_strategy_set_is_not_closed.md`.
**Probes:** `40_probes/`. **Mode:** explore, do not settle.

**Status: COMPLETE.** Written to disk early and extended in place.

## 0. The answer, before the working

There are two spaces here and the panel has been treating them as one. That is why the axis question
has been hard to state, and it is why `39`'s reopening changes more than it looks like it does.

**The mechanism space.** Points are assignments to headroom, layout, overflow policy, intermediate
precision, and whatever else turns out to belong. This is what the preset table's cells contain and
what `25` section 4 decomposes.

**The objective space.** Points are weightings over measurements, together with which measurements are
weighted terms and which are hard constraints. This is what op describes at `36`, `37`, `38` and `34`,
and `38:16-17` states it outright: "All of them should be decided by measurement, just measuring
different things, and, this is I think the mental unlock: They weigh different measurements
differently."

A strategy lives in the **objective** space. A mechanism assignment is what a strategy produces when
it is applied to evidence. Written as a relation:

```
resolve :  objective  ×  evidence   ->   mechanism
```

`25` characterises a strategy as a named section over the product of the mechanism axes
(`25:341-349`, `25:528-537`). That is the graph of `resolve(s, ·)`, and it is correct. What it does not
carry is the thing that generates the graph, and op supplied that afterwards. **The strategy is the
weighting; the section is what the weighting produces.** That is a refinement of `25` rather than a
replacement, and `25`'s own sentence survives every attack in this file.

Three consequences follow and they reorganise the question.

**The strategy set cannot be counted by counting mechanism cells.** Asking whether four is the right
number of cells in a product of headroom against layout is a question about the wrong space.

**A weighting is not a compile-time object.** The type system has no measurements, so `resolve` is
evaluated by a person or a build step and only its output is visible to rustc. The table is therefore
a **design-tier rendering of a canon-tier intent**, which is exactly op's own "the present tables are
one arm and were always incomplete" (`25:186-189`) restated in the tier vocabulary. A canon that states
the objective is permanent. A canon that states the table is wrong the next time somebody measures.

**And the axes divide into two classes with different governance**, which is the piece I think is new.
An axis is **observable** when moving it changes the value the program computes, and **unobservable**
when it changes only cost. Headroom and layout are unobservable. Overflow policy, intermediate
precision, rounding and reduction shape are observable. The weighting model applies cleanly to the
unobservable axes and cannot apply in the same way to the observable ones, because there the thing
being weighed is how often the answer is wrong. That is `34`'s per-strategy soundness condition, and it
falls out of the classification rather than being an extra rule.

### On the count, which is what the dispatch asked

**Four survives, and it survives for a different reason than the one on the record.** Op's four stated
intents are three objectives and their barycentre: time (`Hot`), residency (`Cold`), accuracy
(`Precise`), and best-overall (`Warm`). Three vertices of a simplex plus its centroid is a coherent
sampling, and it explains "four" far better than the two-by-two does. I would keep the number and I
would keep three of the four names.

**The two-by-two does not survive as evidence for anything.** `p1` establishes two things. Every one of
the 24 placements of four labels into a two-by-two is "an exact bijection with every cell filled and
none repeated", so the property is `4 = 2 × 2` restated rather than an observation about arvo. And the
placement is not determined by op's stated intents: reading `36`'s "throwing out all cold or hot axis
optimisations" as opposition, the shipped placement is **not admissible**, with 2 survivors of 24 which
both put `Hot` in the packed cell; reading it as indifference, since layout cannot cost accuracy
anything, the shipped placement is **one of four equally consistent choices**. Section 4.4 keeps both,
because they agree on the part that matters: under neither is the tiling evidence about the
decomposition. It is a property of the grid.

**Four names cannot pin a point in the mechanism product, and none of them currently does.** `p2`
counts it: with the four axes the record names, the product has 16 points, the four names denote 8
between them, and the number a consumer can actually **request** is **0 of 16**, because every one of
the four is silent on exactly one axis. Two of the unreachable points have a consumer already named in
the panel record, and both requests are on observable axes.

**And the objectives do not agree.** `38` names the test and nobody had run it. `p4` runs it over the
committed harness family `mock/benches/bitpack-carrier-width_n*`: a pure-time objective and a pure-
residency objective pick the same arm at **0 of 6** record counts. A pure-time objective does not even
give one answer, picking **4 distinct arms** across the six workloads. And a compromise weighting is
not redundant: at 3 of 6 record counts it selects an arm neither pure objective selects.

**And the axes are not independently resolvable, which nobody had asked.** `25` section 4.2 establishes
that overflow policy and intermediate precision are different questions, so a table needs a value on
each. That is independently **stateable**. Whether the best value on one axis is unaffected by the value
on another is a different property, and `mock/benches/` carries the controlled experiment for one pair,
uncited: two matched families, same workload, same arms, same widths, wrapping against saturating.
`p7` compares the **set of containers in contention** rather than a strict argmin, because at several
widths three arms sit within a nanosecond, and reports at three tolerances. The contending set is
identical at **3 of 6** widths at 5% tolerance and **1 of 6** at 2%, with a second matched pair at 2 of
6. So an unobservable axis's resolution depends on an observable axis's value, and a name that hides
the overflow value has hidden the input to its own headroom answer.

**What a name buys, and what it costs.** It buys the place where the intent lives, since op's
statements are per strategy and a bare product has nowhere to put "Hot may sacrifice soundness for a
provable meaningful gain". It buys the delegation of every unobservable axis, which is the half a
consumer genuinely cannot resolve. It costs `p2`'s coverage bound on the observable axes, where the
consumer is making a correctness statement rather than expressing a preference. So the shape my
evidence supports is **name the objective, expose the observable axes**, and `p3` compiles that with
per-axis overrides at no cost to the common case and with downstream bounds written against properties
rather than names.

**Two things found on the way that are outside my question and larger than it.** `precise-container-width-l1`'s
`kernel` arm returns a flat 63 to 68 nanoseconds at every declared width from 8 to 64, against a field
of 5,400 to 10,700, while the same arm in its wrapping sibling sits in the pack. Work that does not grow
with the declared width is work that is not being done, and the committed findings file calls it
"a safe default pick for this workload shape". And nothing in the corpus would have caught it: across
**214 committed CSVs and 82,960 data rows the digest column is zero everywhere**, as are the
instructions and cycles columns, so no committed bench run in this repository cross-checks that its arms
computed the same answer. Section 11.

### What is not mine to settle, and what I could not determine

Whether a fifth name is wanted on the time-against-residency edge is a design question, and `p4` gives
it the first evidence either way rather than an answer. Whether `Warm` is the right name for a
barycentre is a naming question I raise and do not propose to change. And the exchange rate at which a
strategy's stated preference yields to a measurement is unset in **two** places in op's own words,
`34`'s "meaningful" and `38`'s "consistently just worse", and section 8 argues they are one hole rather
than two.

## 1. The gates

**Canon gate: passed, and there is nothing to defend.** There is no ratified canon. `39` demoted the
one entry in `INTENTS.md` that would have closed my question, on op's direct word, so the set of four
is open by his instruction rather than by my initiative. What is fixed is the workspace discipline, the
forbidden-feature list, and op's own stated intents, and I have checked my work against all three. Every
probe here compiles with no feature gate of any kind: `grep -c 'feature(' 40_probes/*.rs` returns 0 on
both Rust files, recorded in `p3.out` and `p5.out` alongside the toolchain banner.

I flag one thing rather than resolving it, per the gate's ambiguity clause. `39` reads `25`'s
two-by-two finding as "evidence about **what the axes actually are**" (`39:60-62`) and names that as the
live question, which is the question I was dispatched on. `p1` establishes that the two-by-two is not
evidence about anything. That does not conflict with `39`, because `39` is describing what the finding
would mean **if** it held, and op's own counterweight in the same file says a name surviving scrutiny
is kept and a name inherited without scrutiny is not the same thing. I have treated it as a hypothesis
to test rather than a premise, which is what `39` asks for.

**Test gate: run, and the honest report is that there is no suite.**

```
$ cd mock && cargo test --workspace
error: manifest path `.../arvo/mock` contains no package: The manifest is virtual,
       and the workspace has no members.
$ grep -rn '#\[test\]' crates/ | wc -l
0
$ ls crates/ | wc -l
0
```

`mock/crates` was emptied on 2026-08-08 and `mock/Cargo.toml` carries `members = []`. So the suite is
empty by construction rather than decorative, there is nothing to audit, and there is nothing here for
the gate to refuse. I report the commands rather than the absence, because a claim that a suite is
empty is a claim about a place and is checkable in one line.

## 2. Method, and what I refused to count

I derived the structure from op's own stated intents and from the arithmetic of naming, then built
instruments. Four of the five probes are independent in kind rather than in wording: arithmetic over
op's words (`p1`), arithmetic over the axis product (`p2`), the compiler (`p3`, `p5`), and committed
harness output (`p4`).

**I did not read `mock/crates`**, which is empty, and I did not go to git history for it. The only
repository artifact I read is `mock/benches/`, which `RULES.md:224-228` names as the one thing in this
workspace that can price anything.

**`p4` is not a bench and I do not call it one.** It is an analysis of committed harness output plus
arithmetic. It re-measures nothing, and every timing figure in it belongs to the run that produced it.
Its space coordinate is arithmetic from the arm's declared carrier width rather than a measurement, and
that is stated in the probe's own header.

**One thing I refused to count as corroboration.** The prior-art memory `arvo-strategy-is-a-preset`,
which `25` section 4.1 cites, reaches "preset, not axis" from four industrial fixed-point systems. It
agrees with my two-level reading and I am not counting it, because it was already counted by `25` and
because agreement between an artifact and the artifact that cited it is not two instances.

## 3. The two levels, and why collapsing them cost the panel two questions

### 3.1 What `25` established, and the one thing it does not carry

`25:528-537` is the definition on the record, and it is right about the shape:

> A strategy assigns one value on every axis, and each assignment is a function of the build condition,
> a constant assignment being one case of that. Strategies are therefore named sections over a product
> of axes rather than values of a single axis.

The relation form at `25:341-349` is `strategy × build condition -> headroom × layout × overflow ×
intermediate`. Fix the strategy and you have a function from build condition to a point in the product.

What that says is that a strategy **is** its section. What it does not say is where the section comes
from. Under `25`'s reading a strategy could assign any function at all, and the four names would be
four arbitrary sections that someone wrote down. Op's `38` says they are not arbitrary: each is
**generated** by weighing measurements, and the strategies differ in the weighting rather than in the
assignment.

The refinement is small to state and it changes what the canon can say:

```
objective  :  a weighting over measurable quantities, plus which of them
              are hard constraints rather than weighted terms

resolve(objective, evidence)  =  the mechanism point that objective prefers,
                                 given that evidence

section(objective)            =  evidence  ->  resolve(objective, evidence)
```

`25`'s section is `section(objective)`. The objective is the primitive and the section is derived. A
canon stating the section has to be rewritten every time a measurement moves. A canon stating the
objective does not, which is the permanence test in `RULES.md:79-83` applied to exactly this sentence.

### 3.2 The consequence that decides where the table lives

A weighting needs measurements. The type system has none. So `resolve` cannot be evaluated at type-check
time, and what rustc sees is not the objective but a **table** somebody produced by evaluating it.

That is not a limitation to work around. It is the tier boundary, stated mechanically:

- **Canon.** `Hot` weights time heavily and holds accuracy at a finite exchange rate. Permanent, and it
  survives a rewrite in another language.
- **Design.** In this arm, `Hot` resolves to minimum headroom, addressable layout, wrapping overflow.
  Moves whenever a measurement moves, which is what op says at `25:186-189`: "It's always been
  incomplete. Nothing changes in standing base."
- **Code.** The marker type and the container it projects to.

`p3`'s `armswap` section compiles this. Two resolutions of one preset name, one consumer source, byte
for byte identical, compiled against both. The consumer whose bound touches only unobservable
coordinates compiles under both arms. The consumer whose bound touches an observable coordinate
compiles under one and is refused under the other, at exactly one site, with two errors:

```
### ARM release_arm
error[E0277]: this strategy's overflow policy does not give the numeral an absorbing top
   --> p3_axes_presets_properties.rs:279:19
    |
279 |     shortest_path(w)
    |     ------------- ^ min-plus and other tropical folds stand infinity on the top and need it to absorb
```

Both errors land at line 279, `armswap_consumer_observable`. Line 283,
`armswap_consumer_unobservable`, compiles in both arms. That is the cut demonstrated rather than
argued, and section 5 is what it means.

### 3.3 Two panel questions that are asking about the wrong space

**`OPTIONS.md` Q5, "Is the arithmetic column one axis or two".** The column is a column of a table, and
the table is one evaluation of `resolve`. Whether it has one axis or two is a fact about the mechanism
space, which is worth knowing, and it is not a fact about the strategies. Under the two-level reading
the question splits: the mechanism space has however many axes it has, independently of how many names
exist, and the strategies have however many objectives they have, independently of how the mechanism
space is cut. Q5's three readings are all readings of the first half only.

**The phrase "the strategy axis", which both panels have used.** `25:351-355` already flags it as the
wrong phrase and gives the reason: it makes four names look like four values of one dimension. The
two-level reading gives a second reason. Even corrected to "a section over a product of axes", the
phrase locates the strategy in the mechanism space. It is not there.

I am reporting these as framing defects rather than as errors of fact. Nobody wrote anything false; the
questions were posed in the vocabulary available at the time, and `38` postdates both.

## 4. What the two-by-two is worth

`25` section 4.1 reports the four names filling the two-by-two of headroom against layout exactly, and
calls it "the finding that settles the question" (`25:238`). `39:58-62` carries it forward as evidence
about what the axes actually are. `OPTIONS.md` Q5 lists it first among four corroborations of the
two-axis reading of the arithmetic column.

It does not survive, on two independent grounds, and one of them is arithmetic.

### 4.1 Every placement is an exact bijection

```
PART A: is 'an exact bijection, zero cells spare' a measurement?
  placements of 4 distinct labels into a 2x2 grid: 24
  of those, exact bijections with every cell filled and none repeated: 24
  fraction: 24/24
```

There are 24 ways to place four distinct labels in four cells and all 24 are bijections. The property
is `4 = 2 × 2` and it distinguishes arvo's table from no other placement. `p1` part A.

This is not a small correction. `25`'s inference is that four names filling a two-by-two "is what a
product of two binary axes looks like when somebody gives each corner a name". The premise is true and
the inference does not follow from it, because it would follow equally from any four names and any two
binary axes chosen after the fact. The finding would carry information only if the **placement** were
determined by something independent of the table, and that is what part B tests.

### 4.2 Op's stated intents do not admit the shipped placement

`p1` part B encodes four constraints, each my reading of a verbatim quote, each carried next to the
quote in the probe source, each switchable so a reader who rejects one can re-run without it:

- **C1, `Cold` is packed.** `36`: "Cold is optimised for cold paths and cold storage, which means, it
  aggressively minimises and bitpacks." The word is op's.
- **C2, `Cold` is minimum headroom.** `36`: "it aggressively minimises"; `37:106-107` quoting op: "it
  should remain small for memory or disk storage, because it's just sitting basically."
- **C3, `Precise` is not packed.** `36:42-43`: `Precise` is "throwing out all cold or hot axis
  optimisations to be accurate and precise". Two steps: C1 establishes in op's own words that
  bitpacking is `Cold`'s mechanism, so it is a cold-axis optimisation, so `Precise` throws it out.
- **C4, `Warm` is not packed.** `37` quoting op's standing call: `Warm` "should behave like native
  primitives in regular old rust would", and a native Rust primitive is individually addressable.

`Hot` is left unconstrained on both coordinates, deliberately. Op gives `Hot` an objective and no
mechanism, and under `38` the mechanism follows from measurement, which `27` shows moves with the
detected core count. Nothing op has said excludes a packed `Hot`.

The survivor counts, from `p1.out`:

| active constraints | survivors of 24 | shipped placement survives |
|---|---|---|
| C1 | 12 | yes |
| C1+C2 | 6 | yes |
| C1+C3 | 8 | **no** |
| C1+C4 | 8 | yes |
| C1+C2+C3 | 4 | **no** |
| C1+C2+C4 | 4 | yes |
| C1+C3+C4 | 4 | **no** |
| C1+C2+C3+C4 | 2 | **no** |

C3 alone kills it, and C3 is the one that reads `Precise` off op's own sentence. The two survivors under
the full set are:

```
Hot=(doubled, packed), Warm=(minimum, addressable), Cold=(minimum, packed), Precise=(doubled, addressable)
Hot=(doubled, packed), Warm=(doubled, addressable), Cold=(minimum, packed), Precise=(minimum, addressable)
```

Both put `Hot` in the packed cell. Nobody believes that, which is the point: **the four intents do not
tile the grid.** `p1` part C drops the bijection requirement and asks what each name admits:

```
    Hot      admissible cells: all four
    Warm     admissible cells: [(minimum, addressable), (doubled, addressable)]
    Cold     admissible cells: [(minimum, packed)]
    Precise  admissible cells: [(minimum, addressable), (doubled, addressable)]
    cells no name but Hot may occupy: [('doubled', 'packed')]
```

`Warm` and `Precise` compete for the same two cells and `(doubled, packed)` is wanted by nobody. A
bijection forces some name into a cell its intent excludes, and in the shipped table that name is
`Precise`.

### 4.3 What this leaves standing, and what it takes from the register

**`25`'s section 4.2 is untouched.** The finding that the arithmetic column contains cells answering two
different questions, three presets stating an overflow policy and saying nothing about intermediate
precision while the fourth does the reverse (`25:277-279`), is a claim about the cells' content. It is
sound and I have not attacked it. Section 6 says what I think the silence means, and that reading makes
it more interesting rather than less.

**`25`'s section 4.3 is more right than `25` gave it credit for.** It already says the table "is
structurally incapable of answering the question", with four presets consuming all four cells and zero
degrees of freedom left, and that "a table with exactly as many rows as its axes have combinations can
be read back as almost any decomposition". `p1` is that observation carried to its conclusion: not
almost any, exactly any, all 24.

**One correction the register owes.** `OPTIONS.md` Q5's "Two axes" entry lists four corroborations, and
the first is "arvo's own preset table decomposes into an exact two-by-two of headroom against layout
with zero cells to spare". That is a claim about the **storage** column, offered as corroboration of a
claim about the **arithmetic** column, and per `p1` it carries no information about either. Three
corroborations remain and they are real: `25`'s own section 4.2 argument, the four industrial systems,
and the 34-run `warm-clamp-arity` family. The entry should say three. I am not editing the register.

### 4.4 My own C3 has a second reading, and the finding is stronger under it

I went back at the constraint my sharpest claim rests on, because a claim resting on one reading of one
sentence is a claim resting on my reading.

C3 reads `36:42-43`, "throwing out all cold or hot axis optimisations", as **opposition**: `Precise`
refuses a packed layout. There is a second reading and it is at least as good. Under the weighting
model `Precise` optimises accuracy, and layout is unobservable, so **packing cannot cost `Precise`
anything and `Precise` is indifferent to it**. Under that reading "throws out" is either vacuous, since
there is nothing to throw out, or it names a term the stated objective does not contain.

The two readings say different things about the shipped cell and the same thing about what it is worth:

- **Opposition.** The shipped `Precise` cell is **wrong**, contradicting op's own sentence. That is
  `p1`'s survivor table with C3 active: 2 survivors of 24, shipped not among them.
- **Indifference.** The shipped `Precise` cell is **arbitrary**, because the objective does not
  determine that coordinate and any of the four cells is equally consistent with it. That is `p1`'s
  survivor table without C3: `C1+C2+C4` leaves 4 survivors, the shipped placement among them and three
  others exactly as good.

**Under neither reading is the placement evidence about the decomposition**, which is what the register
carries it as. The finding is now independent of which way the sentence reads, and I would rather say
that than defend the reading that makes my own table look sharper.

The readings differ in one thing that matters more than the cell, so section 8 records it as a tension
rather than resolving it: whether `Precise` carries a term its stated objective does not name.

## 5. The cut that decides how an axis may be governed

This is the piece I think is new, and everything in section 6 rests on it.

### 5.1 Observable and unobservable

An axis is **observable** when moving it changes the value the program computes, and **unobservable**
when it changes only what the computation costs. The panel's own measurements classify every axis
currently on the table.

| axis | class | evidence |
|---|---|---|
| headroom | unobservable, conditionally | changes bytes and cycles. Conditional, see 5.2 |
| layout | unobservable | changes bytes and cycles; `25:429-430` records that a strategy "does not change which values a numeral denotes" |
| overflow policy | **observable** | `35:201-202`: the top absorbs at 63 of 63 cells under saturation and 0 of 63 under wrapping; `35:222-223`: 48.9% of in-range DAG shortest paths wrong under wrapping |
| intermediate precision | **observable** | `35` section 3.2: an accumulator one bit narrower than `W + ceil(log2 C)` is insufficient, with negative controls |
| rounding | **observable** | `35` section 3.10: round-to-nearest drops one downstream invariant's failure rate from 87.5% to 12.5% |
| reduction shape | **observable** | `35:52`: signed saturating folds give a different answer under a different split at 70.1% of 16.7M vectors |
| sign domain | **observable** | same measurement; it is half of the pair that fails |
| lane count | unobservable, conditionally | free exactly when the operation is associative, which is the previous row |

`25` section 5.3 already draws this line for one axis, carefully: "a strategy does not change which
values a numeral denotes; it does change which functions its operations denote." The classification is
that sentence made total over the axis list, and the second half of it is what "observable" names.

### 5.2 Headroom is unobservable only because of a convention, and the convention is load-bearing

Headroom is on the unobservable side **only if** the overflow policy is applied at the logical width
rather than at the container width. If a doubled container wraps at the container width, then two
numerals differing only in headroom compute different answers, and headroom becomes observable.

The record has the argument in the other direction already. `OPTIONS.md` Q6 records `20` section 1.3:
under wrapping a lazy headroom arm provably cannot win, "because reduction modulo `2^W` factors through
reduction modulo `2^C` for any `C >= W`". That factoring is exactly the statement that wrapping at the
logical width is stable under widening the container, which is what keeps headroom free.

So the convention "the policy applies at the declared width" is what puts headroom on the cheap side of
the cut, and it is a canon-shaped sentence nobody has written. Without it, half the mechanism space
moves to the expensive side.

### 5.3 The two classes are governed differently, and `34` falls out

**Unobservable axes.** `38`'s model applies without qualification. Resolve by measurement, weighted per
objective, varying with the build condition and the detected regime. Nothing a consumer can rely on
moves, so an arm may resolve them however it likes and a later arm may resolve them differently.

**Observable axes.** The same model applies only in a reading that has to be said out loud: what is
being measured is **how often the answer is wrong**, and the weighting is between wrongness and speed.
`35` supplies that measurement for the one axis where it exists, at 48.9%.

And that is `34`. Op: "'without sacrificing soundness' is property of all of them except Hot. Hot *can*
sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing, instead,
provable meaningful gains" (`34:16-18`). In the weighting vocabulary: **accuracy is lexicographically
prior for every objective except `Hot`, and finitely weighted for `Hot`.** A lexicographic term refuses
any trade at any exchange rate; a finite one trades when the gain clears the rate. Both are weightings,
so `38`'s "all of them are decided by measurement" holds, and `34`'s exception is a property of one
term rather than a separate rule.

I record this as a derivation rather than a proposal, and I record what it does not supply: `34` says
the rate must be cleared by a "provable meaningful gain" and leaves "meaningful" unset. The
lexicographic reading has no free parameter; the finite one has exactly one, and it is unset. Section 8
argues that same parameter is unset a second time in `38`.

### 5.4 The consequence an arm swap makes concrete

**An observable axis may not be resolved per arm without the program computing different answers per
arm.** `p3`'s `release_arm` is that stated as a compile.

This is not automatically wrong. Rust does exactly it: integer overflow panics under debug assertions
and wraps in release, and op's `Warm` intent is to imitate Rust. So the design may well want it. What it
may not do is arrive at it silently, and `25:186-189` records op describing the present table as
"perhaps the one we reserve for debug assertions time, and we write separate arms for release and such
then". Under the cut, that sentence is much larger than it reads for the observable columns and free for
the unobservable ones.

The canon-shaped question, offered as a question: **which axes is an arm permitted to move?** If the
answer is all of them, the design has adopted Rust's debug-and-release semantic split for every numeral,
and that belongs in the canon in one sentence rather than being inherited from a table's provenance.

## 6. What four names can and cannot reach

### 6.1 None of the four pins a point

`p2` builds the product from the axes the record names, with the values the record exhibits, which is a
lower bound on each axis rather than a claim of completeness.

```
  product size with these values: 16

  Hot      denotes 2 point(s); silent on ['intermediate']
  Warm     denotes 2 point(s); silent on ['intermediate']
  Cold     denotes 2 point(s); silent on ['overflow']
  Precise  denotes 2 point(s); silent on ['intermediate']

  reachable-as-stated:   8 of 16  (50.0%)
  reachable-determinate: 0 of 16  (0.0%)
```

The second number is the one to keep. A name that is silent on an axis does not let a consumer choose
that coordinate; it lets the implementation choose it. **Every one of the four is silent on exactly one
axis, so the number of points a consumer can request by writing a name is zero.**

That is `25` section 4.2's half-filled table restated as a coverage figure, and it is a stronger
statement than `25` made. `25` reports the silence as an incompleteness to be filled in. Section 6.3
argues the silence is information rather than a gap, which makes the zero a design consequence rather
than a defect.

### 6.2 The unreachable points with a named consumer are both observable-axis requests

```
  ('minimum', 'addressable', 'saturate', 'same-width')
  ('minimum', 'packed', 'saturate', 'same-width')
```

Both are requests for saturating arithmetic at minimum headroom: a min-plus relaxation on a hot path,
and a stored graph weight column feeding one. `35:201-202` and `35:222-223` are the consumer, and
`37:106-107` quoting op is where the stored column's residency requirement comes from. The only name
that saturates is `Precise`, and `Precise` carries doubled headroom, so a consumer who needs absorption
and cannot afford the headroom has no name to write.

The general form matters more than the two instances. **A consumer asking for saturating arithmetic is
making a correctness statement, not expressing a preference**, and requiring them to find a name whose
objective happens to imply it is asking them to reach a correctness property through a performance
choice. That is the cut of section 5 showing up as an ergonomics failure.

The bound behind it is arithmetic and is not a criticism of four in particular:

| axes | product size | fraction four names can pin |
|---|---|---|
| 2 | 4 | 100.0% |
| 3 | 8 | 50.0% |
| 4 | 16 | 25.0% |
| 5 | 32 | 12.5% |
| 6 | 64 | 6.2% |
| 7 | 128 | 3.1% |

A name is a point and there are four names. `p2` lists three further axis candidates the record already
names (rounding, sign domain, reduction shape), so the row that applies today is somewhere below the
fourth.

### 6.3 The silence is information at one cell, a gap at another, and open at two

`25` reads the half-filled arithmetic column as a defect: "four cells covering two axes, with each cell
filled on one axis and silent on the other" (`25:275-277`). Under the two-level reading it is what a
correct table looks like.

A mechanism cell is an argmax. An argmax over a product is determined only on the coordinates the
objective actually discriminates. `Cold`'s objective is residency, and wrapping and saturating occupy
the same bits, so residency does not discriminate the overflow axis at all. `Hot`'s objective is time,
and time does not discriminate the intermediate precision axis when the wider accumulator still fits a
register. **A preset is silent on an axis exactly where its objective is indifferent to it.**

That reading is testable and I have not tested it, which I say plainly in section 10. What it does
change is what a canon owes. Under `25`'s reading the canon owes a value in every cell, sixteen cells
for four names over four axes, and the design grows by filling in a table. Under this reading the canon
owes something different and smaller: **which coordinates each objective determines, and a rule for the
rest.** The rest is where the observable-axis question of 6.2 bites, because indifference is not an
answer when the consumer can observe the difference.

**And the reading is checkable on the four silences the record actually has, without a harness.** `25`
reports four: `Cold` on overflow, and `Hot`, `Warm` and `Precise` on intermediate precision
(`25:277-279`). Taking them one at a time:

- **`Cold` on overflow: indifference, and it is arithmetic rather than a hypothesis.** Wrapping and
  saturating occupy the same bits, so residency cannot discriminate the axis at any width. The silence
  is the objective having nothing to say, exactly as 6.3 predicts.
- **`Precise` on intermediate precision: not indifference, and the reading is refuted here.** Accuracy
  discriminates that axis by construction, and `36:44-45` puts `Precise`'s own domain there, "within
  chains and ops, not only alone", which is where intermediate precision lives and nowhere else. So
  `Precise`'s objective determines the coordinate and the table does not state it. That is a genuine
  gap rather than an indifference.
- **`Hot` on intermediate precision: plausibly indifference, untested.** A wider accumulator that still
  fits a register costs nothing in time, so time may not discriminate it. It would stop being
  indifferent the moment the wider accumulator no longer fits, which is a width-dependent answer and is
  exactly the shape `20`'s interior-safety crossover has.
- **`Warm` on intermediate precision: undetermined**, and it inherits whatever `Warm`'s weighting turns
  out to be.

One explained, one refuted, two open. The reading is not general and it is not empty, which is a more
useful result than either. It also sharpens 4.4: `Precise`'s real distinguishing coordinate is
intermediate precision, the table states it for `Cold` and not for `Precise`, and the coordinate the
table does give `Precise` is one its objective is indifferent to.

### 6.4 Independently stateable is not independently resolvable, and the record has the experiment

`25` section 4.2 establishes that overflow policy and intermediate precision are different questions,
so a table needs a value on each. That is **independently stateable**. A second property does not
follow from it and nobody has tested it: whether the best value on one axis is unaffected by the value
taken on another, so that a resolver may settle them one at a time. Call that **independently
resolvable**.

The distinction decides how much work `resolve` is. If the axes are independently resolvable, a
strategy's assignment is a product of per-axis argmins and each axis can be settled by its own
measurement. If they are not, `resolve` ranges over the product and every axis has to be settled
against every other, which is the difference between k measurements and their product.

**`mock/benches/` carries the controlled experiment for one pair and nobody in this panel has cited
it.** Two matched families, same workload description, same arm names, same declared widths, one
coordinate changed:

| pair | wrapping family | saturating family |
|---|---|---|
| elementwise, 4 ops/element | `warm-elementwise-width-l1` | `precise-elementwise-width-l1` |
| container fork, 3 ops/element | `warm-container-width-l1` | `precise-container-width-l1` |

`p7` compares them. It compares **rankings only**, never magnitudes, because the two families are
separate harness runs and cross-run absolute times are not comparable. It compares the **set of arms in
contention** at a stated tolerance rather than a strict argmin, because at several widths three arms sit
within one nanosecond of each other and a strict argmin over those measures the noise. And it reports at
three tolerances so the answer's sensitivity is visible rather than hidden in a threshold I picked.

Its parse is checked before anything is reported: 126 medians recomputed against the committed findings
files, 0 mismatches.

Pair one, elementwise, no arm flagged:

```
  ARMS IN CONTENTION, tolerance 5% of the fastest
   width  under wrapping                              under saturating
       8  kernel, minimum, native                     minimum                       NO
      13  minimum, native                             minimum, native, plusone      NO
      16  kernel, minimum, native                     kernel, minimum, native       yes
      32  kernel, minimum, native                     kernel, minimum, native       yes
      60  headroom, kernel, minimum, native, plusone  minimum, native, plusone      NO
      64  kernel, minimum, native                     kernel, minimum, native       yes
  widths where the contending set is identical: 3 of 6
```

At 2% tolerance it is 1 of 6, at 5% and 10% it is 3 of 6. Pair two, after excluding a flagged arm
(6.5), is 2 of 6 at all three tolerances.

**So the overflow policy changes which containers are in contention, at half the declared widths or
more, on both pairs.** The two axes are independently stateable and are not independently resolvable.

Two things that follow, and one that does not.

**`resolve` ranges over the product, not over the axes one at a time.** A design that settles headroom
by one measurement and overflow by another has assumed something the committed data contradicts at a
majority of widths.

**And the interaction crosses the cut of section 5.** Headroom is unobservable and overflow is
observable, so the resolution of a free coordinate depends on the value of a coordinate the consumer
must be told about. That is the strongest argument I have found for exposing the observable axes rather
than bundling them: a name that hides the overflow value has hidden the input to its own headroom
answer.

**What does not follow** is that the effect is large. `p7` compares set membership, so it says the
contending set changes and says nothing about by how much. The magnitude is unpriced and the word for
it here is unpriced.

### 6.5 One arm in that experiment did not do the work, and nothing in the corpus would have caught it

Reported because it bears on 6.4's own evidence, and section 11 reports what it implies more widely.

`precise-container-width-l1`'s `kernel` arm is flagged by `p7` at every one of its six widths:

```
    kernel: flagged in 6 of 12 runs
      saturate  width 8           64 ns against      10588 ns for the next arm  (165x)
      saturate  width 13          63 ns against       8048 ns for the next arm  (127x)
      saturate  width 16          65 ns against       8168 ns for the next arm  (126x)
      saturate  width 32          66 ns against       5420 ns for the next arm  (82x)
      saturate  width 60          63 ns against       8036 ns for the next arm  (128x)
      saturate  width 64          68 ns against       5572 ns for the next arm  (83x)
```

The same arm under wrapping, in the sibling family, sits in the pack: 483 ns against 250 for the
fastest at width 8. A constant 63 to 68 nanoseconds, independent of declared width, against a field two
orders of magnitude above it, is the signature of work that was removed rather than work that was fast.
That arm is excluded from 6.4's pair-two figures and the exclusion is printed rather than silent.

The committed findings file does not treat it as suspect. Its own highlights read
"warm-container-kernel dominates: 318353% faster than the next best" and "a dominant, well-separated
winner is a safe default pick for this workload shape".

## 7. Is four the right sampling

### 7.1 Of the objective space, yes, and I would keep it

Op's four stated intents are three measurable objectives and one that is not of the same kind.

- **`Hot`: time.** `36`: "performance, efficiency, even at the cost of accuracy or soundness."
  `37:109-110`: "If throughput gives *performance* wins and efficiency gains, then that is for Hot."
- **`Cold`: residency.** `37:106-107`: "it should remain small for memory or disk storage, because it's
  just sitting basically." The reason is in the sentence, which is what makes the objective narrow
  rather than a preference.
- **`Precise`: accuracy.** `36:42-45`: "throwing out **both** the hot and the cold axis optimisations",
  and "especially within chains and ops, not only alone", which scopes the objective to composed
  computation.
- **`Warm`: best overall.** `38`: "its intent is to be intuitive best choice for most every use case,
  and the intuitive part demands it mimics, but it does not make it absolutely required, if mimicking is
  consistently just worse choice."

Three of those name a quantity to minimise. The fourth names no quantity: "best for most every use
case" is a statement that no quantity dominates, which is a **weighting** rather than an objective, and
specifically the near-uniform one. Familiarity enters as a tie-break under it, which is exactly what
`38` says: mimicry is demanded by intuitiveness and dropped when it is "consistently just worse".

So the four are the three vertices of a simplex over (time, residency, accuracy) and its centroid. That
is a coherent sampling, it is the natural one, and it explains the number four with something better
than a coincidence. **Four survives. I would keep it, and I would keep `Hot`, `Cold` and `Precise` as
the names of the vertices.**

Where a fifth would go, if one is ever wanted, is an **edge** rather than a new vertex: a workload that
is both hot and large, weighting time and residency and not accuracy. `27`'s four-core measurements are
exactly that regime, with `u32` changing sign and `u16` going from a loss to a win. Nothing here says a
name is needed there. It says that is where to look, and that looking for a fifth name by finding an
unreached mechanism cell is looking in the wrong space.

### 7.2 Warm's name is on a different axis from Warm's role, and that has cost four restatements

An observation rather than a proposal, and I am not asking for a rename.

`Hot`, `Warm` and `Cold` read as a chain: one scale, three points, `Warm` in the middle. `Precise` is
off that scale entirely. But the objective space is a triangle with `Precise` as its third vertex, and
`Warm` is not the middle of the `Hot`-to-`Cold` edge. It is the centroid of the whole triangle,
including the vertex the temperature metaphor cannot name.

**The vocabulary is one dimension short of the structure it names.** A reader who takes the metaphor
seriously will place `Warm` between fast and small, which is where a compromise on two objectives sits,
and `Warm`'s intent is a compromise on three.

The record already shows the cost. `37:43-44` reports the `Warm` call being restated twice in two days
because it kept failing to stick, `INTENTS.md` I3 counts four statements of the same call, and `37`
diagnoses it as "a presentation defect rather than a content dispute". A presentation defect that
survives four restatements by the same person is worth a mechanical explanation, and I think this is
one. It is not the only possible one.

What I would keep either way: `Hot`, `Cold` and `Precise` name what they optimise, they are short, and
each survived every check in this file.

### 7.3 The objectives do not agree, on the one dataset where two of them are both available

`38` names the test and states the instinct:

> For the most part, they probably agree, because in general, the best answer fits all, because it
> fights none of their intent. But perhaps my instinct is wrong there, and all truly differ for the most
> part.

`p4` runs it against the committed family `mock/benches/bitpack-carrier-width_n*`, six record counts,
six variants, 40 warm-mode samples per variant, `mockspace-bench-harness`,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, Apple M1. Time is the committed measurement; residency is
arithmetic from each arm's declared carrier width, which the family's own title states.

The probe validates its own parse before reporting anything, by recomputing medians the committed
findings files already state:

```
SELF-CHECK: recomputed medians against the committed findings files
  medians compared: 36, mismatches: 0
```

That check earned its place. The first version of the probe pooled the cold-mode rows with the warm and
read the mean column where it meant to read the median, and reported 36 mismatches of 36 rather than a
result. Both defects are recorded in the probe's header.

```
     records  time-argmin (fastest)        space-argmin (smallest)       agree?
       16384  bitpack-carrier-d64          bitpack-carrier-packed-simd       NO
      131072  bitpack-carrier-d32          bitpack-carrier-packed-simd       NO
     1048576  bitpack-carrier-d32          bitpack-carrier-packed-simd       NO
     2097152  bitpack-carrier-d16          bitpack-carrier-packed-simd       NO
     4194304  bitpack-carrier-d16          bitpack-carrier-packed-simd       NO
     8388608  bitpack-carrier-d16-control  bitpack-carrier-packed-simd       NO

  record counts where the two objectives agree: 0 of 6
```

**Zero of six.** And the obvious objection is correct and is the point: of course the packed arm always
minimises bytes and rarely minimises time, because a trade exists here. Where no trade exists the
objectives agree trivially, and where they agree there is nothing for a strategy to decide. **So the
region where the strategy axis does any work is precisely the region where the objectives disagree**,
and inside that region the measurement says they disagree everywhere. Op's instinct and this result are
compatible over the whole space and incompatible over the part that matters, which is a sharper thing to
report than either alone. He named the possibility himself.

**And the space column is constant, which is not a defect in the method but the fact that dissolves a
disagreement.** Residency is **computed**, not measured: bytes per element at a declared width is a
compile-time quantity, `ceil(N * W / 8)` for a packed column and the carrier size for a dense one.
Nothing needs a harness to establish it.

That resolves what `37` and `38` disagreed about. `37` wrote that `Cold` is "decided by **intent**, and
possibly not by measurement at all", and went further with "a measurement could not overturn this". `38`
corrected it: "All of them should be decided by measurement, just measuring different things." Both are
right once "measurement" is not assumed to mean "timing". `Cold`'s objective is decided by a
measurement, and the measurement is static, so no harness run can overturn it and none is owed. The
disagreement was about the word.

Which also re-prices the "footprint is unpriced" line that `26`, `27`, `35` and `38` all carry. Two
things sit under it and only one is real. **The bytes saved** were never unpriced; they are arithmetic
and `p4`'s space column is them. **What a consumer gains from the bytes**, beyond the time the whole
`bitpack-*` corpus already measures, is a constraint-satisfaction question rather than a measurement:
does the working set fit a budget. `mock/benches/` carries three families named `bitpack-footprint-*`,
and their own titles say what they measure, "sequential sum swept past L1 and L2", which is time with
footprint as the independent variable. So the register's owed measurement, as worded, is either already
taken or is not a measurement.

**One more consequence, and it is `38`'s `Cold` clarification landing mechanically.** `p4`'s space
column breaks ties on time, so where two arms carry identical bytes it takes the faster. `packed` and
`packed-simd` both store 13 bits and it selects `packed-simd` at 6 of 6 record counts. That is exactly
"`Cold` does not *have to* drop efficiency wins elsewhere. It can use the same paths `Hot` uses, not
because it needs to by intent, but nothing in its intent would fight it", realised as a selection rule:
a lexicographic objective on (residency, time). Op's sentence and the tie-break are the same object.

Two further counts from the same run, both of which bear on the number.

```
  pure-time objective, distinct winning arms across the six record counts: 4  ['d16', 'd16-control', 'd32', 'd64']
  pure-space objective, distinct winning arms across the six record counts: 1  ['packed-simd']
```

A single objective's mechanism answer is not a constant. That is `25` section 4.4's claim that a
strategy assigns functions rather than values, holding on an axis `25` did not measure, from committed
harness output rather than from op's ruling. Two instruments, opposite directions.

```
     records   |par|  Pareto set                                    |hul|  linear-weighting winners, in order
       16384       4  d16, d32, d64, packed-simd                       4  packed-simd -> d16 -> d32 -> d64
      131072       3  d16-control, d32, packed-simd                    3  packed-simd -> d16-control -> d32
     1048576       3  d16, d32, packed-simd                            3  packed-simd -> d16 -> d32
     2097152       2  d16, packed-simd                                 2  packed-simd -> d16
     4194304       2  d16, packed-simd                                 2  packed-simd -> d16
     8388608       2  d16-control, packed-simd                         2  packed-simd -> d16-control
```

The Pareto set is the set of arms no other arm beats on both objectives at once, so any objective
monotone in both can select from it and from nothing else. Its size is an upper bound on how many
mechanism answers the objective space distinguishes at that workload: **between 2 and 4**, workload
dependent. The hull set is the subset reachable by a non-negative **linear** weighting, and the probe
asserts hull is contained in Pareto rather than assuming it. The two coincide here.

The sets are invariant under rescaling either axis by a positive affine map, so they do not depend on
the probe's normalisation. The weight at which the winner switches does depend on it, and the probe
reports switch points labelled as an artifact rather than as a finding.

### 7.4 A compromise weighting is not redundant, measured

The sharpest thing `p4` produces for the naming question. If the interior of the weighting range only
ever selected what one of the two ends selects, a barycentre objective would be a name for nothing.

```
     records  count  arms only an interior weighting selects
       16384      2  d16, d32
      131072      1  d16-control
     1048576      1  d16
     2097152      0  (none)
     4194304      0  (none)
     8388608      0  (none)

  record counts with at least one interior-only arm: 3 of 6
  interior-only selections in total: 4
```

At three of six record counts a compromise weighting selects an arm neither pure objective selects, and
at the smallest it selects two. **`Warm` as a barycentre names something the vertices do not reach**,
which is the first empirical support I have found for the fourth name being more than a default.

The bound on that, stated because it is real: two objectives, one axis pair, one workload family, one
host, reads only, sequential only, and the residency coordinate is arithmetic rather than measured. It
is an existence result about the interior, not a claim that the interior always matters. At three of the
six sizes the interior collapsed.

## 8. Where op's stated intents pull against each other

Named rather than resolved, per the dispatch.

**T1. `38`'s "they probably agree" against `38`'s own model.** Under the model, two objectives agree
exactly where no trade exists. Where no trade exists the strategy axis decides nothing. So the
agreement region and the region the axis exists for are complements, and 7.3 measures zero agreement
inside the second. His own hedge anticipates it, which is why this is a tension in the design rather
than an error in the statement.

**T2. `36`'s `Precise` against the shipped table's `Precise`.** `36:42-43` says `Precise` throws out
cold-axis optimisations; the table at `25:132` gives `Precise` the bitpacked cell. `p1` C3. Under the
provenance ladder the table loses, and section 4.4 records that under the second reading of the same
sentence the cell is arbitrary rather than wrong. The tension is live under the first reading and
dissolves into T6 under the second.

**T3. `34`'s "provable meaningful gains" against `38`'s "consistently just worse choice", and they are
one hole.** `34` says `Hot` may trade soundness against a gain that is provable and **meaningful**, and
leaves "meaningful" unset, saying so. `38` says `Warm`'s mimicry is dropped if mimicking is
**consistently just worse**, and leaves "consistently just worse" unset.

Both are the same quantity: **the exchange rate at which a strategy's stated preference yields to a
measurement.** `Hot`'s is between accuracy and time; `Warm`'s is between familiarity and everything
else. A canon that names one owes the other, and a canon that names neither has stated four objectives
without stating what any of them would take for an answer.

Nobody should invent a number for either, and `34` says so about its own. What is worth saying is that
they are one shape, because two unset thresholds look like two loose ends and one unset mechanism looks
like what it is.

**T4. `32`'s adaptation intent against `34`'s soundness condition, on the reduction shape.** `35` found
this and stated it: op's intent that arvo adapts to the cores it finds is blocked, for signed saturating
folds, by op's own soundness condition, at a measured 70.1% split-dependence (`35:52`, `35` section
3.6).

My addition is that the block is not a coincidence of one law. **The reduction shape is an observable
axis**, so it sits on the side of section 5's cut where per-regime resolution changes answers. Every
observable axis is blocked from regime-sensitive resolution by the same argument, for every strategy but
`Hot`. `35`'s Q12 is therefore a question about the cut rather than a question about folds, and its four
options read differently in that light: "specify the reduction shape" is the move that takes the axis
off the observable side by making the answer a function of the input alone.

**T5, recorded as resolved rather than live.** `36`'s `Cold` with "more leeway to do things
non-efficient" against `38`'s `Cold` that "does not *have to* drop efficiency wins elsewhere". `38`
corrects `36` and the correction is on the record. Under the weighting model the correction is exactly
right and needs no further argument: a low weight on time is not a preference for slowness, it is
indifference, and indifference takes the fast path when the fast path is free.

**T6. `Precise`'s stipulation against `Precise`'s objective.** `36:42-43` has `Precise` "throwing out
all cold or hot axis optimisations". Layout is unobservable, so under the weighting model `Precise`
cannot be made less accurate by a packed layout and its objective is indifferent to it. Either
`Precise` carries a term its stated objective does not name, or the clause is vacuous over the
unobservable axes and is really a statement about the one axis where accuracy does live, intermediate
precision, which `36`'s "especially within chains and ops" points at directly.

I lean to the second and I have not established it. It matters because it decides whether `Precise`'s
storage coordinates are wrong (first reading) or arbitrary (second), which is section 4.4, and because
a hidden term in one objective is a hidden term in the model.

**T7. `38`'s "all decided by measurement" against `Cold`'s objective being static.** Resolved rather
than live, and recorded because the resolution is the useful part: residency is a measurement that
needs no harness, so `37` and `38` were disagreeing about the word rather than the design. Section 7.3.

## 9. Bearing on the live options

Per `OPTIONS.md`'s own instruction. I cite it by section and by a phrase verified with `grep -F`, never
by line.

**Q5, is the arithmetic column one axis or two.** *Fits the product reading well; fits all three
badly in one respect; and gains a question none of the three asks.* Section 6.4 is the addition:
independently **stateable** is what `25` section 4.2 established and what all three readings are about,
and independently **resolvable** is a different property that nobody has tested. `p7` tests it on the
matched pair `warm-elementwise-width-l1` against `precise-elementwise-width-l1` and finds the
contending container set identical at 3 of 6 widths at 5% tolerance and 1 of 6 at 2%, with a second
pair at 2 of 6. So the overflow policy moves the container answer, and an axis list that is correct
about the questions can still be wrong about the resolution order. *And the rest of the entry:* The register's product reading says "named sections over it rather than a
partition of it", which is `25`'s definition, and my two-level refinement strengthens it: the sections
are generated by weightings rather than written down. What fits badly across all three readings is that
Q5 asks about the mechanism space while the strategies live in the objective space, so no answer to it
counts the strategies (section 3.3). **One corroboration under the two-axis entry should be struck**
(section 4.3): the exact-two-by-two claim is about the storage column and per `p1` carries no
information. The other three stand.

**Q6, does `Warm` wrap or clamp.** *Re-priced rather than answered, and re-scoped.* Under section 5 this
is a question on an **observable** axis, so it may not be resolved per arm without the program computing
different answers per arm, and `p3`'s `release_arm` compiles exactly that consequence. That does not
choose between wrapping and clamping. It says the choice is not the kind of thing an arm may make
quietly, which is a constraint the entry does not currently carry. The entry's third option, that the
question dissolves under Q5's two-axis answer, survives and gains a reason: `Warm` being silent on
intermediate precision is its objective being indifferent to that coordinate (section 6.3), while its
overflow value is observable and cannot be left to indifference.

**Q7, which carrier the packing claim is about.** *Fits the regime-sensitive option well; one of its
entries should be reworded.* The "footprint rather than throughput" entry says the footprint benefit
"has not been priced at all by any file in the panel". Section 7.3 argues that conflates two things:
the bytes saved are arithmetic and were never unpriced, and what a consumer gains from them beyond
time is a constraint question rather than a measurement. `38`'s reversal of `37` on the same point
inherits the same conflation. *And the rest of the entry:* `p4` reads a different family from `26` and `27` and reaches
a compatible shape: the answer moves with the workload, and a claim naming a carrier without naming its
conditions is underspecified. What I add is that **the packing axis is unobservable**, so it is the
clean case for regime-sensitive resolution: an arm may resolve it differently per detected regime and no
consumer's correctness moves. That is a genuine argument for the regime-sensitive option over the
regime-free inequality, and it applies to layout and not to overflow.

**Q11, what the numeral guarantees to a fold.** *Fits the "numeral names its algebraic structure"
option well, and `p3` compiles it.* `35` proposed it and nobody had built it. `p3` shows a downstream
routine bounded on `S::Overflow: AbsorbingTop + MonotoneAdd` compiling gate-free, refusing a wrapping
preset at the call site, and doing so through both the direct bound and a blanket-implemented ergonomic
alias. The properties are implemented on the **axis value** rather than declared per preset, so a preset
inherits them and they cannot drift from the policy. The diagnostic carries the measurement:

```
error[E0277]: this strategy's overflow policy does not give the numeral an absorbing top
    = note: saturating overflow supplies it; wrapping does not, and a wrapping top plus one is zero
```

**Q12, is the reduction order specified.** *Re-scoped by section 5, per T4.* The reduction shape is
observable, which puts the "say nothing, let it depend on the core count" option on the wrong side of
the cut for every strategy but `Hot`, and gives the "specify the reduction shape" option a reason
beyond determinism: it moves the axis off the observable side.

**The candidate reframing spanning Q5 and Q6, which `35` proposed.** *Compiled, and priced.* `p5` is the
half `35` did not have. Stating properties instead of policies is cheap when the property **factors
through one axis** and expensive when it does not.

```
### ARM negative
error[E0119]: conflicting implementations of trait `NotBoth` for type `Strat<Saturate, Signed>`
```

Absorption, monotonicity and invertibility each depend on the overflow policy alone, so each is one
impl and adding an axis adds nothing. Exact reassociability depends on overflow **and** sign domain
jointly: three of four cells hold and the fourth fails at 70.1% (`35:52`). The satisfying set is not a
product of per-axis sets, so it cannot be stated as a conjunction, and the cheap spelling ("holds for
everything except this one") is negative reasoning, which the forbidden-feature set has no route to:
full `specialization` is forbidden and `negative_impls` does not disarm coherence. `p5` compiles the
attempt and records the `E0119`.

So the cost, stated as a relation rather than measured:

- A property that is a **conjunction of per-axis facts** over k axes costs k impls. Adding an axis adds
  one.
- A property whose satisfying set **does not factor** costs one impl per satisfying assignment, up to
  the product size minus one. Adding an axis multiplies it.

The reframing is cheap exactly for the properties that factor and expensive for the ones that do not,
and **which properties those are is a measurement rather than a matter of taste**. One non-factoring
property is known so far. `p5`'s positive arms compile at 3 impls over 4 cells, so the cost today is
small; the scaling is what to watch.

**Q1, Q2, Q3, Q4, Q8, Q9, Q10, and the container-derivation outputs.** *No bearing found, and I looked
at each.* One observation on Q4 rather than a finding: the absorbing-top denotation reading and the
saturating overflow value are the same coordinate seen from two vocabularies, so a decision on one is a
decision on the other. `35` section 5 already reaches this from the algorithm side.

## 10. What the register should gain

I am not editing `OPTIONS.md` or `INTENTS.md`. These are for whoever does.

**A new question, and the one I would rank first. Q13: which axes may an arm move?** Section 5 is the
argument. Its live options, in full:

- **Any axis.** An arm resolves everything, including overflow policy and intermediate precision. The
  design has then adopted Rust's debug-and-release semantic split for every numeral, which is coherent
  and is what `Warm`'s imitation intent points at. Costs: the same source computes different answers
  under different arms, and a downstream bound that holds in one arm fails in another, which `p3`
  compiles.
- **Unobservable axes only.** An arm resolves headroom, layout, container and lane count freely, and
  every observable axis is fixed across arms. Buys a program whose answers do not depend on how it was
  built, and keeps `38`'s weighting model intact where it applies without qualification. Costs the
  imitation intent on the one axis where Rust's own behaviour is arm-dependent.
- **Unobservable axes always, observable axes for `Hot` only.** The per-strategy form, which fits `34`
  exactly: `Hot` may move an observable axis per arm against a provable meaningful gain, and the rest
  may not. Costs an axis-classification the canon must carry, and inherits `34`'s unset threshold.
- **No classification; state per axis whether it is arm-resolvable.** The enumerated form. Cheapest to
  state and it does not say why, so a new axis has no rule to be classified by.

**A second new question. Q14: what is the exchange rate at which a strategy's preference yields?**
Section 8's T3. It is op's, and it is unset in two of his own sentences, and naming it once is what
makes both answerable. Its options are a shape rather than a number: a stated rate per objective, a
lexicographic ordering with no rate at all (which is what every strategy but `Hot` already has), a rate
supplied by the consumer, or silence with the consequence that "meaningful" is decided case by case by
whoever writes the arm.

**An addition to Q5's product entry.** The axis list should carry the observable-or-not classification
alongside each axis, because it is what decides how that axis may be governed and it is not derivable
from the axis's name. And the entry's corroboration count should read three rather than four
(section 4.3).

**An addition to the "what a strategy is" entry** under the questions op has not been asked. `25`'s
definition is there as ONE EXPERT wanting a second, order-inverted read. I did not read `25` before
deriving, so I am not that second read on its own terms, and I say what I am instead: I read `25` after
op's `38` existed, and my finding is that the definition holds and is incomplete in one specific way,
namely that it identifies a strategy with its section and does not name what generates the section
(section 3.1). That is an addition to the entry rather than an agreement with it.

**A third new question. Q15: are the axes independently resolvable, and in what order?** Section 6.4.
Distinct from Q5, which asks whether they are independently stateable. Its options:

- **Independently resolvable.** A strategy's assignment is a product of per-axis argmins, each settled
  by its own measurement. Cheapest by far, and contradicted at a majority of widths on both matched
  pairs `p7` examines.
- **Resolvable in a stated order**, with earlier coordinates fixed before later ones are measured. If
  the observable axes come first, this is coherent with section 5's cut: the consumer fixes what they
  can observe and the resolver settles the rest against it. Costs the ordering, which the canon then
  has to justify rather than assert.
- **Jointly resolvable only.** `resolve` ranges over the product. Most faithful to `p7` and most
  expensive, since the measurement matrix is the product rather than the sum.
- **Resolvable per axis with the interactions named as exceptions.** The pragmatic form. Costs a list
  that grows by discovery and has no rule behind it.

**An addition to Q7's "footprint rather than throughput" entry.** Its claim that the benefit is
unpriced should separate the bytes saved, which are arithmetic and always were, from what a consumer
gains beyond time, which is a constraint question. Section 7.3.

**Two droplist entries, each with its diagnostic and what would reopen it.**

*The two-by-two of headroom against layout, as evidence about the decomposition.* Closed by `p1`: all 24
placements of four labels are exact bijections, so the property carries no information about arvo; and
under the four constraints read off op's own intent statements the shipped placement is not among the 2
survivors, with C3 alone sufficient to exclude it. **Reopened by:** an independent determination of the
placement, from something other than the table, that agrees with the table. The finding that the
mechanism space has a headroom axis and a layout axis is **not** dropped; only the bijection's status as
evidence is.

*Reading the four names as a sampling of the mechanism product.* Closed by `p2`: the four names pin 0 of
16 points, so they are not a sampling of that space at all, and by section 3, which locates the strategy
in the objective space. **Reopened by:** a reading under which each name's silence is filled, which
would make them four points of sixteen and put the question back on coverage grounds rather than on
category grounds.

**A candidate reframing rather than an option, and it is `35`'s with a price attached.** State per
strategy which properties the arithmetic has rather than which policy it takes. `p3` compiles it, `p5`
prices it, and the price is a function of whether the property factors through a single axis. Both
halves belong in the register together, because the proposal reads free without the second.

## 11. Reported outside my question, because the standing instruction says to

Two findings about the repository, neither in my dispatch, both bearing on work the panel has already
built on. Stated plainly rather than softened.

### 11.1 No committed bench run in this repository verifies that its arms computed anything

The harness emits a `digest` column so that arms can be cross-checked against each other, and
`instructions` and `cycles` columns for hardware counters. Across the whole committed corpus:

```
$ cd mock/benches
$ awk -F, 'FNR>1{print $17}' *.csv | sort -u
0
$ awk -F, 'FNR>1{print $13, $14}' *.csv | sort -u
0 0
$ ls *.csv | wc -l
214
$ awk -F, 'FNR>1' *.csv | wc -l
82960
```

**214 files, 82,960 data rows, and every digest is zero.** So nothing in the committed output
cross-checks that two arms of a comparison produced the same answer, and nothing records an instruction
count against which a suspiciously fast arm could be sanity-checked.

That is not a claim that any particular result is wrong. It is a claim that the corpus has no defence
against one specific failure, **and section 6.5 locates an instance**: `precise-container-width-l1`'s
`kernel` arm returns a flat 63 to 68 nanoseconds at every declared width from 8 to 64, against a field
of 5,400 to 10,700, while the identical arm in the wrapping sibling family sits in the pack. Work that
does not grow with the declared width is work that is not being done. The findings file celebrates it:
"warm-container-kernel dominates: 318353% faster than the next best" and "a dominant, well-separated
winner is a safe default pick for this workload shape".

The panel's own rules already say a bench needs real competitor arms rather than a strawman. A
competitor that was optimised away is a strawman the author did not choose, which is worse, because
nothing in the process flags it. **A digest that differs between arms is the cheapest defence there is
and it is presently switched off everywhere.**

I have not audited the corpus for further instances and I am not proposing that anyone should. `p7`'s
flag rule is fifteen lines and any file consuming a bench family can run it.

### 11.2 The register carries a corroboration that corroborates nothing

Section 4.3. `OPTIONS.md` Q5's two-axis entry lists four corroborations and the first is about a
different column of the table from the claim it supports, and per `p1` carries no information about
either. Three real ones remain. I am not editing the register.

## 12. What I could not determine

**Whether a preset's silence is its objective's indifference, at the two cells still open.** Section 6.3
settles two of the four by argument: `Cold` on overflow is indifference, established by arithmetic, and
`Precise` on intermediate precision is not, because accuracy discriminates that axis by construction and
`36` puts `Precise`'s own domain there. `Hot` and `Warm` on intermediate precision are open. The test is
well defined and needs a family that varies the accumulator width against a fixed workload under a
non-clamping policy. **`mock/benches/` does not have one**: of the 40 committed families, 18 name an
overflow policy in their title and **not one names two**, so no family varies the policy internally, and
the accumulator sweeps that exist (`warm-clamp-arity-*`, `warm-clamp-chain-l1`) are all clamping. Two
matched **pairs** exist across families, which is what `p7` uses, and pairing works for a ranking
comparison and not for the discrimination question, because cross-run magnitudes are not comparable.
So the instrument for the two open cells does not exist in the repository, and building it outside the
harness would be an ad-hoc spike with no substance.

**Whether the objective list is three.** Time, residency and accuracy are read off op's four statements,
and I have no argument that they are complete. Compile time is a candidate the workspace already treats
as a real quantity, `arvo-compile-time-last.md` ranking it last rather than absent, and no strategy
weights it. Energy is another. I did not look for a fourth systematically.

**Whether familiarity is a tie-break or a fourth term.** Section 7.1 reads `Warm` as the barycentre with
familiarity breaking ties, which fits `38`'s "the intuitive part demands it mimics, but it does not make
it absolutely required". A different reading, that familiarity is a genuine fourth weighted term with a
small coefficient, fits the same sentence. The two differ observably: under the tie-break reading
mimicry never overrides a measured difference, and under the fourth-term reading it can override a
small one. `38`'s "consistently just worse" is closer to the tie-break reading. I did not settle it and
it is the same unset quantity as T3.

**The magnitude of anything.** Nothing in this file prices a design. `p4` reads prices somebody else
took, on one host, one workload family, reads only, sequential only. Every other number here is a count
or a compiler diagnostic.

**Whether the observable classification is total.** I classified eight axes and I do not know that the
list of axes is eight. `25` section 9 leaves the completeness of its four-item list explicitly
unverified and names the method that found the fourth, and that method has not been run again.

**Whether an axis can be partly observable.** Headroom is unobservable conditional on a convention
(section 5.2). I did not check whether other axes have similar conditionals hiding in them, and if the
conventions are the load-bearing part then the classification is a classification of conventions rather
than of axes.

**How large the axis interaction is.** `p7` compares set membership, so it establishes that the
contending container set moves with the overflow policy and says nothing about by how much. The
magnitude is unpriced, and it would need a family that varies both coordinates in one run, which does
not exist.

**Whether `precise-container-width-l1`'s `kernel` arm is the only collapsed arm in the corpus.** I
flagged it in the two families I read and did not sweep the other 38. `p7`'s flag rule is fifteen lines
and would sweep them, and I did not run it corpus-wide because a sweep producing a list I cannot
diagnose is a worse artifact than a located instance with its diagnosis.

## 13. Coverage, bounded honestly

**Read end to end:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `34`, `36`, `37`, `38`, `39`, `25`, `35`,
`OPTIONS.md` (all 984 lines, in two reads).

**Read in the region I cite, by opening the lines:** none beyond the above. Every `file:line` in this
document was opened with `sed -n` and its content checked against the claim before the claim was
written, not merely resolved. `OPTIONS.md` and `INTENTS.md` are cited by section and by phrase, per the
dispatch, because both are repaired after they land.

**Not read:** `01`, `02` through `24`, `26` through `33`, `DROPLIST.md`, `PERSONA_CALLS.md`, `seed/`,
`archive/`. Where I refer to a finding in one of those I rely on `OPTIONS.md`'s account or on `25`'s and
`35`'s accounts and say so in the text each time. **The specific risk:** `20`'s factoring argument
(section 5.2) and `26`/`27`'s carrier results (section 7.1, section 9) reach me only through
`OPTIONS.md`, and if it misrepresents them those two paragraphs inherit it. `01` is named in the brief as
required reading and I did not read it; I read `RULES.md`'s account of its section 0, which is the part
that governs how everything else is weighed, and I flag the gap rather than claiming it did not matter.

**Read in the repository:** `mock/benches/`. In full, the `bitpack-carrier-width_n*` family, six CSVs,
six findings files and one meta file. In the region I use, the four families of `p7`'s two matched
pairs, their CSVs and their findings tables, plus one findings file's highlights section. By title
only, all 40 committed families, for the overflow-policy census in section 12. I did not read the
variant sources under `variants/` for any family, so I cite what a family measured and not how its arms
are written, which is why section 11.1 says an arm did not do the work and does not say why.
`mock/crates` is empty and I did not read git history for it.

**Probes:** `40_probes/`, committed with sources, raw compiler output and run logs.

| probe | kind | what it establishes |
|---|---|---|
| `p1` | arithmetic over op's verbatim words | all 24 placements are bijections; the shipped placement is not among the 2 survivors of op's own intents; C3 alone excludes it, and 4.4 records the second reading under which it is arbitrary instead |
| `p2` | arithmetic over the axis product | 16 points, 8 denoted, **0** consumer-requestable; two unreachable points with a named consumer, both observable-axis |
| `p3` | compiler, 4 arms, gate-free | properties-as-bounds compile and refuse correctly; per-axis override compiles; an arm swap breaks an observable-axis consumer and not an unobservable one |
| `p4` | analysis of committed harness output | time and residency argmins agree at 0 of 6; one objective gives 4 answers across 6 workloads; Pareto and hull sets sized 2 to 4; interior-only arms at 3 of 6 |
| `p5` | compiler, 3 arms, gate-free | a property that factors costs one impl; one that does not costs one impl per satisfying assignment, and the cheap spelling is `E0119` |
| `p6` | citation checker | opens every `file:line` citation in this document and tests each against a word it must contain: **24 checked, 0 failures**, and a cross-check confirms the probe's list and the document's citations are the same set |
| `p7` | analysis of two matched committed families | the contending container set differs between wrapping and saturating at 5 of 6 widths at 2% tolerance and 3 of 6 at 5% and 10%, with a second pair at 4 of 6; one arm flagged as eliminated at 6 of 6 widths |

**Self-checks that fired, reported rather than hidden.** Four, and each changed a number I would
otherwise have reported.

`p4`'s median cross-check against the committed findings reported **36 mismatches of 36** on its first
run, from two independent parse defects: pooling cold-mode first-touch rows with warm-mode steady-state
rows, and reading the mean column while intending the median, because several tables in one findings
file share a row shape and the last match won. Both are recorded in the probe header.

`p4`'s hull computation carries an assertion that the hull set is contained in the Pareto set. It
**failed on the first run**, because a weakly dominated arm won a tie at weight zero and entered the
hull. The tie-break was fixed and the assertion now holds.

`p7`'s first version compared strict argmins and reported disagreement at widths where three arms sat
within one nanosecond of each other, which measures the noise. It now compares contending sets at three
tolerances. It also had no dead-arm rule, so it reported an arm that returns a flat 63 nanoseconds at
every width as the fastest container under saturating, at all six widths, which is section 11.1.

`p6` opens every citation in this document and tests its content rather than its resolution. **First
run, 1 failure of 23**, which turned out to be the checker flattening a quoted sentence across a
blockquote marker rather than a wrong citation; the strip was added and the check is clean at 24 of 24.
I report the failure anyway, because a checker that has never failed is a checker nobody has tested,
and a second script confirms the probe's citation list and the document's are the same set rather than
trusting me to have kept them in step.

**Not verified:** that my reading of any of the four constraints in `p1` is op's. Each is my reading of
a quote, each quote is carried beside it in the probe source, and each constraint is switchable so a
reader who rejects one can re-run without it. The table of survivor counts under every subset is printed
for exactly that reason.

**Status: COMPLETE.**
