# 219. The marker spelling, and what a checker can hold it to

Seat 219. The question is the one `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`
left open in its own `note`: the marker's spelling, and where a checker enforces the distinction.

## The two gates

The canon gate passes. I read the ratified ruling row, `dimension.toml` in full, the workspace rule
`.claude/rules/every-finding-carries-its-predicate.md`, and the predicate strings in `proposal.toml`.
The work asked for is the work that ruling's `note` assigns to the panel, in its words, and nothing
in the ratified material forbids it. What I do challenge, below, is narrower than the assignment and
is reported as a finding rather than as a refusal.

The test gate passes, and it took reading rather than counting. `cargo test -p arvo-checks` from
`mock/` runs 144 tests in 19 binaries, all green. Green is the weakest signal available, so I read
the bodies of everything in the surface I touch: `every_predicate_names_a_declared_axis.rs`,
`the_axis_vocabulary_is_append_only.rs`, the region arms of
`what_one_field_obliges_another_to_carry.rs`, and `src/predicate.rs` and `src/shape.rs` behind them.
They are not decorative. Every arm has a planted control, several have controls in both directions,
and `the_axis_vocabulary_is_append_only.rs` carries an explicit third test whose entire job is to
prove the other two are comparing real sets rather than a list against itself. The ratchet in
`what_reaches_each_obligation.rs` goes further and declines to assert on a quantity it says is
derivable from two others, on the ground that a decorative assertion that cannot fail on its own is
the tautology the suite exists to refuse. I have nothing to delete and no arm to report as fake.

One thing I will note because it bears directly on this question rather than on the suite's quality:
the checker reads the slug side of a predicate entry and not the values side, and
`checks/src/predicate.rs:17` says so as a deliberate design decision, because the value grammars
differ per axis. That decision is correct and it is also the reason two live canon violations sit in
the registry unseen. More on both further down.

## Blindness, and one leak I have to report

I derived and committed before reading any other panel file. The registry rows, `dimension.toml` and
the workspace rule were the whole of my reading, plus the `arvo-checks` crate, which is instrument
rather than argument.

The leak: when creating my probe directory I ran `ls` on the panel directory and piped it through
`head -5`, which put four member filenames in front of me. Filenames only, no content, and none of
them is the seat deriving this question in parallel as far as I can tell, since I do not know which
seat that is. I did not open any of them. It was avoidable and I should have created the directory
without listing its parent. Recorded because a claimed independence is worth less than a reported
contamination, however small.

A second thing worth stating plainly: my derivation and any parallel one share an input neither of
us varied, namely `every-finding-carries-its-predicate.md` and the `dimension.toml` header, which
between them state the three-states-and-no-fourth mechanism and name the unnamed third notation
state outright. If the parallel seat and I agree about those, we agree because we read the same two
paragraphs. Convergence on anything downstream of that is shared premise, not corroboration, and a
consolidation should intersect our instruments rather than our conclusions.

## What the ruling actually asks, stated exactly

The ratified `says` distinguishes three things. A measurement carries the region it was swept over.
A proof carries a marker saying its argument is width-free by construction. A finding established
across a bounded whole-container range, exhaustively over every value a container holds, carries its
own spelling. The title names two markers, and the `says` describes three states. That is not a
conflict and the resolution falls out of the derivation below: two of the three are new spellings and
the third is the ordinary case, which needs a name only so an author can be explicit about it.

The test it gives is one sentence and it is the whole of the enforcement problem: name the
construction that makes the width unable to enter, or it was a sweep.

## The first thing to settle, because everything else depends on it

The three things are not three regions. They are three **warrants**, and warrant is orthogonal to
extent.

A region is a subset of an axis: `W in 3..=7`, `F = 0`, `threads any`. A warrant is what establishes
that the claim holds over that subset. As is well known from any setting where a proposition and its
derivation are tracked separately, the two do not determine each other, and here is the concrete
proof that they do not:

A construction can be bounded. An argument of the form "the intermediate cannot overflow, therefore
the width does not enter" holds while the intermediate fits and stops holding above it. That is a
construction warrant over `W <= 32`, a proper sub-range, with no sweep anywhere in it.

A sweep can be total. An axis with finitely many values can be walked in full, and then the extent is
the whole axis while the warrant remains enumeration. That is precisely the third state the ruling
names.

So the two coordinates cross. Any spelling that encodes warrant by inventing new extent keywords, for
instance reserving `any` for proofs and adding `all` for exhaustive walks, collapses a product into a
sum and loses exactly the two cells just named. I tried that shape first, for about as long as it took
to write down "construction over a sub-range", and it fails on its own terms rather than on taste.

The consequence is that the notation needs a second coordinate per entry, not a richer vocabulary in
the first one.

## Granularity: the marker is per axis, and this is measured rather than argued

The obvious cheap answer is that `sentence_kind` already carries this. A row is `measured` or
`theorem` or `argument`, and those are exactly the warrants. If that worked it would be the right
answer, because it costs nothing and the field already exists.

It does not work, and the reason is a fact about the corpus rather than a preference. A single row
routinely establishes different axes by different routes. `219_probes/warrant_census.rs` measures it
over the committed registry: of 527 predicate entries in 677 rows, **25 rows carry an axis at its
universal together with a bounded or fixed axis in the same predicate**. Their `sentence_kind` values
are seven `argument`, seven `measured`, four `theorem`, and seven rows carrying none at all.

Seven `measured` rows carrying a universal is the shape that matters. Under a row-level marker their
universals would inherit the mark `measured`, and four of the eleven `target features any` entries in
the corpus sit on rows of exactly that kind. Nobody swept every target feature set. A row-level marker
does not merely fail to express the distinction on those rows; it asserts something false about them.

The rescue attempt worth recording, because a later seat will think of it: keep the row-level mark and
add a convention that universals are construction and bounded entries are swept. That fails on
`proposal::no_multiplicative_structure_survives_a_nonzero_fraction_width`, which carries
`rounding any` resting on a structural argument its own `note` describes as not a sweep over modes,
alongside `total_width: W in 3..=7` which is a measured band, on one row, and which already carries a
`gate` refusing promotion because its prose reaches past its predicate. One row, two warrants, and the
row is the corpus's own worked example of the failure being guarded.

So the marker attaches to an axis entry. `sentence_kind` stays what it is, a statement about the
sentence, and gains nothing and loses nothing.

## What an unmarked entry claims, which is the hinge

This is where I changed my answer, and the measurement is what changed it.

My first derivation had the unmarked case default to `swept`, on the reasoning that the notation
refuses a doubt state, so silence must mean something definite, and the weakest warrant is the safe
assignment. It is a tidy argument and it is wrong.

There are four candidates and three of them are refuted.

**Unmarked means swept.** Refuted by measurement. There are 38 unmarked universal entries in the
registry. Eleven are `target features any` and three are `threads any`. Reading those as swept adds a
claim to each that is false and absurd: nobody enumerated the instruction-set feature sets. It also
breaks the ratified `note` directly, which says every existing predicate still means exactly what it
meant.

**Unmarked means construction.** Refuted by the ruling itself. That silently promotes all 38 to
proofs, which is the exact failure the ruling exists to prevent, applied wholesale and retroactively.

**Unmarked means unstated, as a hedge.** Refused by the ratified material. There is no vocabulary for
doubt in this notation, because a hedge and a proof look alike once written down.

**Unmarked claims no warrant.** This is the one that survives, and it is not a fourth state smuggled
back in. The three-states-and-no-fourth rule governs **region** semantics: an axis listed with a range
or the universal, an axis listed with a fixed value, or an axis absent, and absent is the strongest
negative statement in the notation. A warrant is not a region. It is an audit property, of exactly the
same kind as `evidence` or `standing`, and those already admit absence without anybody calling it a
hedge.

The distinction that keeps that honest, and it is sharp: **a region is gateable and a warrant is not.**
An arm gates on `F = 0`. Nothing gates on how anybody found out. `dimension.toml`'s own header makes
the same move from the other side when it demotes gateability from the test for an axis to a corollary
about which axes arms can be built over. The predicate's region half is the part a const predicate
consumes; the warrant half is the part a reader and a checker consume.

Under this reading every one of the 527 committed entries means precisely what it meant this morning:
a region, and no claim about the route. The ratified `note` is then literally true rather than
approximately true, which is the outcome to prefer when a ratified sentence is on the line.

What stops a new row taking the old free ride is a ratchet, not a semantics. See the arms below.

## Where the marker sits, and the measurement that decided it

Five positions, and I built the census partly to decide between them.

**On the values side, as a prefix.** `total_width: construction W any`. Parseable without a per-axis
grammar, because the first token after the colon is at a fixed position. The census confirms it is
available: **zero** values sides in the committed registry begin with any of the three tokens, so
nothing collides.

**On the values side, as a bracketed suffix.** `total_width: W any [construction]`. Also parseable,
and worse, because predicate entries get quoted mid-prose and a trailing bracket is what a quotation
drops first.

**As a sibling list on the row.** A `warrant = ["total_width: construction"]` array beside
`predicate`. Rejected: two lists to hold in step, and it detaches the mark from the thing marked.
This corpus has a check named `no_note_describes_a_row_it_is_not_on` because it has already been
bitten by a field describing something it is not attached to.

**On `sentence_kind`.** Refuted above, by 25 rows.

**On the slug side, as a suffix.** `total_width/construction: W any`. This is the answer, and the
census is why.

The slug side is clean. **All 527 entries carry a slug side that is a bare lowercase-and-underscore
slug**, with no exceptions. The values side is not clean: five entries write English inside it,
including `arity = 3 for the grouping kind, and arity in 2..=5 for the schedule kind`. A marker riding
in front of that prose inherits the prose's untrustworthiness, and worse, it sits in the half of the
entry the checker deliberately does not parse. A marker in an unparsed field is a marker that will
drift, and `checks/src/predicate.rs:17` is an explicit statement that the field will stay unparsed.

The slug side is also the half a checker already splits, at `checks/src/predicate.rs:52`. Extending
`split` to take a second cut on `/` is a two-line change that leaves `undeclared_dimensions`,
`repeated_dimensions` and the append-only vocabulary check working unaltered, provided the slug is
stripped of its suffix before lookup.

And the separation reads correctly. The slug side names which axis participates; the warrant says how
it participates. The values side names the region. Region on the payload, warrant on the tag.

The full spelling, then:

```
<axis-slug>[/<warrant>]: <region>
```

```
total_width/construction: W any
total_width/exhaustive: W in 1..=64
total_width/swept: W in 3..=7
total_width: W in 3..=7
fraction_width: F = 0
```

The last two are the unmarked form and mean a region with no warrant claimed, which is what every
committed entry is.

## The three tokens

`swept`, `construction`, `exhaustive`. Each is the ruling's own word, which is the tiebreaker I would
reach for last and which here happens to coincide with the words that survive on their merits.

`construction` rather than `proof`. The marker does not say the claim is proved; it says this axis
cannot enter the argument. Those come apart: a construction-warranted axis can sit on a row whose
overall standing is one expert. `proof` would be a claim about the sentence made in a field that is
about an axis, which is the row-level confusion this whole derivation is trying to avoid. It is also
the word that most invites being written by somebody who has not got one.

Rejected alongside it: `argued`, which is the exact register in which a hedge and a proof look alike,
and which the ruling's test is written against; `free`, as in `W free`, which reads as a free variable
and means the opposite of what is meant, since the axis is not free but irrelevant; and `structural`,
which describes the argument's flavour rather than its obligation.

`exhaustive` rather than `total` or `complete` or `closed`. `total` collides with `total_width` in
every sentence about widths, which is the axis this marker is mostly for. `complete` is overloaded in
logic to the point of uselessness. `closed` is already in service in this corpus for the open-versus-
closed question about the operation and strategy vocabularies.

`swept` rather than `measured` or `sampled`. `measured` collides with `sentence_kind = "measured"` and
would reintroduce the row-level confusion by vocabulary alone. `sampled` is judgmental and is simply
wrong for a fixed value: `F = 0` is not a sample of the fraction widths, it is the only one the claim
holds at.

`swept` is admissible and optional, never required. It exists so an author who means it can say it,
and so that a reviewer can tell a considered sweep from an entry written before the marker existed.
Requiring it would break the ratified `note`, since it would make every committed entry non-compliant.

## What a checker enforces

Eight arms. Six are about the warrant. Two are not, and they are here because the census found them
and because without them the marker is a lock on a door in a wall with a hole in it.

Each carries the case that must fail, because an arm that has never returned a non-zero establishes
nothing, and this corpus knows that: `shape.rs:202` through `shape.rs:229` is a three-version
post-mortem of a prose matcher that guessed at exactly this kind of property and was wrong in both
directions twice before being replaced by a declared value. That comment is the strongest single piece
of evidence in the repository about how to build this, and its conclusion is the one I am adopting:
**a declared value beats a better parser.**

**Arm 1. `construction` names its construction, as a citation.** The row carries a `construction`
field keyed by axis, whose value is the id of a registry row or a probe that states the mechanism.
Not a sentence. The ruling's test is "name the construction that makes the width unable to enter", and
a citation is a thing another seat can open and attack, where a sentence is a thing another seat can
only agree with. Must fail on: an entry marked `construction` with no `construction` entry for that
axis, and one whose citation names nothing that exists. Reuses `provenance.rs` and the resolution
machinery `a_standing_is_reachable_from_what_it_cites.rs` already exercises.

**Arm 2. `construction` may not carry a single fixed value.** If the axis cannot enter the argument,
the argument does not hold at one point of that axis and nowhere else. `construction F = 0` is the
fraction width entering as a precondition, which is an ordinary fixed region with no warrant. Must
fail on `fraction_width/construction: F = 0`. Bounded ranges stay admissible, because the bounded
construction is real, as established above.

**Arm 3. `swept` may not carry the universal.** A sweep walks a set and the universal is not a set
anybody walked. If a finite axis was walked in full, the honest spelling is `exhaustive` with the
bound written out. Must fail on `total_width/swept: W any`. This arm fires on nothing in the committed
corpus, because no committed entry carries a marker at all, which is exactly the property that lets
it be a clean assertion rather than a ratchet.

**Arm 4. `exhaustive` may not carry the universal, and obliges the row to name the set it walked.**
For a width axis that set is the container, so the predicate must carry a `container` entry;
`exhaustive W in 1..=64` says nothing checkable until somebody says which container has 64 of them.
Must fail on an `exhaustive` entry with no `container` entry beside it, and on
`total_width/exhaustive: W any`.

A consequence of Arm 4 worth stating because it is a coherence check on the whole scheme rather than a
rule anybody has to remember: `exhaustive` is inadmissible exactly where the universal is
inadmissible, and for the same reason. `dimension::operation` and `dimension::strategy` both declare
`any` not admissible because the set is open, and a set nobody has closed is a set nobody can have
walked. The two prohibitions fall out of one fact.

**Arm 5. The three tokens are reserved.** No `dimension` grammar and no values side may begin with
one. This pins the census's zero forward, so the day somebody names a container `exhaustive` the
checker says so rather than silently reinterpreting a region as a warrant. Must fail on a planted
values side beginning with a token.

**Arm 6. A ratchet on unmarked universals.** The count may not rise above 38. New universals carry a
warrant; the 38 committed ones are named and not blocked. This is the arm that gives the scheme teeth
without a retroactive edit, and the retroactive edit is genuinely forbidden rather than merely
awkward: a predicate is never widened in place, the original is not updated, and a correction lands in
a later deliverable and reaches canon through consolidation. Adding `construction` to a committed
entry is a rewrite of that entry's claim, so it belongs in a new row rather than in a `sed`. The idiom
already exists at `tests/what_reaches_each_obligation.rs:61`, along with a written argument about why
a ceiling is placed on what is answered rather than on what is outstanding, which applies here
unchanged.

The two arms that are not about warrants:

**Arm 7. A values side may not write a spelling its own axis's grammar declares inadmissible.**
Live violation, one, found by `219_probes/values_side_admissibility.rs`:
`proposal::three_topics_independently_terminate_on_the_strategy_axis_as_their_shared_placeholder`
writes `operation: operation any`, and `dimension::operation` says, in bold, in its own `grammar`
field, that `operation any` is not admissible because `any` quantifies over a set nobody has closed.
That row also writes `overflow policy any` and `rounding any`, which are fine, and this one, which is
not. The `dimension::operation` `note` predicted this exactly: it says a grammar admitting a spelling
its own note forbids is a gate with a gap the width of the failure it guards, because a checker reads
a grammar and cannot read a note. The gap was closed in the note and the spelling was written anyway,
in a different file, where nothing looks.

**Arm 8. A values side may not bind more than one thing.** Five live violations, and they split into
two classes that want different fixes.

Two pack an undeclared axis inside a declared slug.
`proposal::the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times` writes
`build_profile: emission in {metadata only, full codegen}, debug-assertions any`, and nothing declares
`emission`. `proposal::a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body` writes
`build_profile: no_std, feature gates = 0, opt level = 3`, and nothing declares `feature gates`. Both
are invisible to `undeclared_dimensions` because the slug they arrive under is perfectly legitimate.
This defeats the exact guarantee `dimension.toml`'s header argues for at length, that an undeclared
axis silently converts the strongest negative statement in the notation into a shrug, and it defeats
it in the one place nobody thought to look, which is inside a legal axis rather than beside one.

Three state two regions on one axis inside one entry.
`proposal::chain_laws_split_by_whether_a_lifting_theorem_exists` writes
`arity: arity = 3 for the grouping kind, and arity in 2..=5 for the schedule kind`, and
`proposal::a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none` does the same on both
`total_width` and `fraction_width`. There is a check called `predicate-names-one-axis-twice` whose
message is "Two regions on one axis and nothing says which holds", and it cannot see any of these,
because they are one array element rather than two. The check is right about what it refuses and the
corpus routes around it by punctuation.

I want to be exact about the numbers here rather than quote my own probe's headline. The probe's
"binds an undeclared name" column reports five, and that column overstates: three of the five strays
are my left-hand-side extractor reading `and arity` and `and W` and `and F` as names, which they are
not. The honest breakdown is two genuinely undeclared axes and three double-region entries. The
multi-binding count of five is exact and is what the arm should be written over.

## What I am challenging, and it is not the ruling

The three-way split is real and the notation is the right place for it. Warrant belongs beside the
region it warrants, for the same reason a predicate belongs beside the claim it bounds: prose escapes
every mechanism here, and the corpus says so about itself in
`proposal::no_multiplicative_structure_survives_a_nonzero_fraction_width`'s `gate`, which observes
that every mechanism audits predicates and the sentence is what a later reader quotes. A warrant
living in a `note` is a warrant nothing will ever check.

What I do challenge is the standing of one of the three states. **The registry contains no
whole-container range.** The census looked for `1..=64` and `0..=63` across all 527 entries and found
zero, and a wider grep for the shape across the registry finds it only in `dimension.toml`'s own prose,
where the `total_width` `note` names it as the third notation state that has no name. So `exhaustive`
is being spelled for a case the registry does not currently hold.

That is not a reason to refuse it. The ruling is ratified and the panel corpus plainly contains such
findings even though the registry has not yet received one, since `dimension.toml` describes the
failure as having already happened, a proof recorded as a sweep that stopped at three widths. It is a
reason to say three things out loud. The spelling is derived rather than validated, because there is
no live row to try it against. Arm 4 is what stops it decaying into a fancier `any`, and Arm 4 is
therefore not optional garnish on this state but the whole of its content. And the first row that
needs one should be written with the marker rather than written without and retrofitted, since
retrofitting is the thing the never-widen-in-place rule forbids.

I also want to record a reading I considered and rejected, because a later seat may prefer it.
`exhaustive` could be a **region** keyword rather than a warrant, on the argument that "every value
the container holds" is an extent. It reads well and it fails for the reason the whole first section
gives: it cannot express an exhaustive walk over a sub-range, and it would put one of the three states
on a different coordinate from the other two, so a checker would need two mechanisms where one does.
The warrant reading keeps all three on one axis of the design.

## What I could not settle

Whether adding the marker to an existing registry row is permitted at all. The ruling says the markers
apply going forward and no existing file is restated, and its `because` makes clear the declined
option was a per-file pass over panel member files. A registry row is not a member file. But the
never-widen-in-place rule is about predicates rather than about files, and it points the other way.
I have designed around the ambiguity by making Arm 6 a ratchet rather than an assertion, which is
correct under either reading, so nothing I propose depends on resolving it. It still wants resolving
before anybody edits a committed predicate, and I am not the one to resolve it, because two of the 38
sit on a row op ratified.

That row is worth naming. `proposal::an_additive_verdict_is_independent_of_the_fraction_width` carries
`total_width: W any` and `fraction_width: F any`, and its own `note` says the `W any` is derived from
the route rather than from a sweep and that the source calls it inspection rather than measurement.
Under my scheme those two entries want `construction`, and the construction is already written down
and already ratified: the `promotion` field of the very ruling that ratified it says the mechanism it
names, that addition at a common scale performs no rescale, is why the width cannot enter. So the
marker's first real use is a row where op has already supplied the citation Arm 1 would demand. That
is the strongest evidence I have that the scheme fits the corpus rather than being imposed on it, and
it is also the row that makes the amendment question sharp, since I would be adding a mark to a
ratified predicate.

## For the seat that attacks this next

The alternatives, with what each is good at, so this is a starting list rather than a single answer.

The values-side prefix is genuinely viable and is the runner-up. It is parseable, it collides with
nothing today, and it reads more like a sentence: `construction W any` scans better than
`total_width/construction: W any`. Everything I have against it is about the values side being an
unparsed prose field, which is a fact about the checker's current design rather than a law. If a later
seat decides the values side should be parsed per axis after all, the prefix becomes the better
position and the slug-side suffix becomes redundant machinery.

The sibling-list form deserves one more look if the marker ever needs to carry more than a token,
such as a confidence, a date, or a second citation. A tag on a slug has room for one word. If the
warrant turns out to want structure, the tag position is the wrong shape and the list is the right
one, and that is a reason to revisit rather than a reason to have started there.

The row-level form is dead and I would rather nobody spend another dispatch on it. The number is 25
and the probe is committed.

The thing I did not attack and would attack first in your position: whether `construction` should be
one token or two. There is a real distinction inside it between "the operation does not read this
axis", which is a statement about the code, and "the axis was varied and nothing moved", which is a
differential control and is not by itself a proof of anything beyond the range varied. The additive
row contains both, and its `because` gives the differential evidence, the broken counts transferring
at 952 across every measured fraction width, alongside the structural argument. I treated the
differential run as corroboration for a construction rather than as a warrant of its own, because a
run over a range is a sweep over that range whatever it found. I believe that is right. I am not
certain it is right, and if it is wrong then Arm 1 is demanding the wrong citation.

## The probes

Both are in `219_probes/`, with their sources, their binaries' output and their planted controls
committed. Both build with plain `rustc` and no dependencies. Neither reads anything but the registry.

`warrant_census.rs` and `census.out` answer the seven questions the derivation turns on. Its control
fired twice during development and both firings are recorded in the source rather than smoothed away.
The first was a mis-stated expectation of mine: I wrote a want of one mixed row into a fixture that
contains two by inspection, and the fixture was right. The second was worse and was a real defect: I
added the slug-shape and bare-universal arms and they printed zeros for a run with nothing in the
fixture reaching them, which is a vacuous arm however correct the code behind it, so I planted the
cases and both now return non-zero before the real run is believed.

`values_side_admissibility.rs` and `values_side.out` answer the two questions outside the brief. Its
control also caught a real defect: the first version of its clause splitter looked for `=` and `in`
and therefore could not see `debug-assertions any`, which binds with no separator at all, and that is
the exact entry the probe was written to find. It could not have found it. The fix is in the source
with the reason attached.

Every registry citation in this file is by row slug rather than by line, per
`tests/no_line_citation_into_the_registry.rs`, which carries a ceiling of 45 on member-file line
citations and an instruction not to raise it. The repair it names is briefing the seat to write slugs.
Consider this seat briefed.
