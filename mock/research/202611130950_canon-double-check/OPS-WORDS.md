# What op said on 13 November 2026, verbatim

His words of 13 November 2026 that bear on arvo, as they reached this job in its
brief and in the coordinator's records of the day's question rounds. The first
two answers are the ones the registry rows this job added carry as quotes, and
they are copied with the options each was asked among, because a quote with no
record of what it was chosen among is worth little to a later reader. The rest
are the earlier answers and the unprompted remarks of the same day that the
brief also quotes.

The typography is his, including the apostrophes, and every quotation below was
copied from its source by command and compared against it byte for byte. The
readings under each answer are this job's and are marked as such. Nothing here
is his reading of his own words.

## The overflow default

Asked, with the priced page `arvo-warm-priced` and the plan `arvo-plan-2` in
hand: what a `Warm` value becomes when it overflows in a release build.

Options as put to him: [a: wraps, integers and fixed point alike] [b: saturates,
integers and fixed point alike] [c: integers and platform-width indices wrap,
fixed-point quantities saturate] [d: `Warm` names no overflow behaviour, every
alias keeps naming its adaptation].

Answer, verbatim:

> I think this shouldn’t be called one way, but a default arvo may ship that is
> easy to override per case. Perhaps a value should be returnable wrapped as
> Saturating<Foo> or Wrapping<Foo> or something as the mechanism? The actual
> theory already is based on flexible numbers built on arbitrary choices behind
> the scenes that are composable from scratch if someone wants. On the defaults
> we build, I say it should be a generic with default perhaps following one of
> the parity standards like matlab or whatever would be the sensible default to
> use. And even then it might make more sense to primarily have downstream users
> use the spec paired impls of the theory and contracts like already designed in
> arvo (which you would know had someone done the canon double check and ensure
> it’s up to date and sensible etc)

This job's reading, not his words. He took none of the four as put. The intent:
the behaviour is not fixed one way, a default ships and is easy to override per
case, the default is a generic with a default, and the default may follow one of
the parity standards. The wrapper spelling is floated as a candidate mechanism
("Perhaps", a question) rather than ordered. The last clause is hedged the same
way, "it might make more sense": that downstream users primarily use the
standard-paired implementations of the theory and the contracts.

## The old crates that a row says do not return

Asked: six one-expert rows say an old arvo crate does not return. Options as put
to him: [a: supersede them with one row saying every family of old arvo returns
on the current fundamentals, a crate's old name may go] [b: walk the six one by
one] [c: leave them as proposals that bind nothing].

Answer, verbatim:

> I want them to return, in function/feature, but the shape the contracts and
> traits and default impls etc take are up for a whole rewrite and also the
> taxonomy. However, definitely arvo terrain to write the actual arithmetic
> contracts on the standard too…

This job's reading, not his words. Every old family returns in function and in
feature. The shape, meaning the contracts, the traits and the default
implementations, and the taxonomy, meaning which crate holds what and under
which name, are open to a whole rewrite. Writing the arithmetic contracts on the
standards is arvo's own ground rather than a consumer's.

## Three earlier answers the same day, on arvo

From the coordinator's question round of about 06:15 UTC, after the check of
the night's arvo work against arvo's record. The question, the options and the
answer of each are copied as the round recorded them, the options whole, since
what he chose is in the option's text and not in the four words of his answer.

On arvo's next steps. Asked:

> the value crate branch `work/numeral-integers` has four things beside
> arvo's record (carrier table in the wrong crate, panic tied to the overflow
> marker, seals against an open inventory, aliases without the strategy
> parameter) and was built before the datum question was closed. Which shape?

The options:

> [a: claim first (promote the datum and value definition by a cold second read
> and the gate), then rework the same branch in one round (carrier table to
> `arvo-placement`, debug abort as its own configurable axis, seals opened,
> aliases keyed on strategy defaulting to `Warm`), then continue the plan with
> the id step re-scoped onto a shared placement] [b: contracts first: write the
> contracts a consumer bounds on, then fit the value type under them] [c: back
> to a short arvo panel before any value code lands] [d: land as built, correct
> the four in follow-ups].

His answer:

> "Claim, then rework in one round (Recommended)"

This job's reading, not his words. He chose option a, whole. That option orders
the claim before the rework, and its rework names four things: the carrier table
to `arvo-placement`, the debug abort as its own configurable axis, the seals
opened, and aliases keyed on strategy defaulting to `Warm`. An earlier version
of this file cut the option down to its first clause, which dropped the content
he approved.

On arvo's public name. Asked:

> hilavitkutin has 419 `arvo::` import sites against the old crate named
> `arvo`, deleted on purpose; a one-expert row says "no facade". What do
> consumers import?

The options:

> [a: a thin crate named `arvo` again that re-exports the value types and
> contracts] [b: no facade, consumers import the inner crates by name and
> hilavitkutin's sites are rewritten at its port] [c: decide at the port, with
> the import list in hand].

His answer:

> "Decide at the port"

On `Warm` for integers. Asked:

> the record nearly derives wrap on release and assert on debug, but
> `ruling.toml:1038` holds it open in his name. A usable first slice needs it
> claimed.

The options:

> [a: `Warm` integers wrap, the debug check is the consumer's configurable
> axis] [b: `Warm` integers saturate, the debug check configurable the same
> way] [c: bring him both readings priced with the committed numbers, as a
> question ticket; the slice ships `Warm` unclaimed meanwhile and aliases name
> their adaptation].

His answer:

> "Bring me both priced"

## On the debug abort, the same day

From the round before it. Asked whether an operation aborts in a debug build
when it saturates and the type declared saturation as its meaning. The options:

> [a: abort by default, a marker in the type says completion is the meaning and
> then no abort] [b: always abort, as `ruling.toml:1052-1058` is written] [c:
> never for a declared policy] [d: arvo's panel decides, per
> `ruling.toml:1713`].

His answer:

> "I’d say this should be either a cargo feature or const/generic/trait
> configurable"

## Said unprompted the same day

At about 05:18 UTC, as the workspace's capture of his words records it:

> Same thing on arvo: no parallel shit as a shortcut or workaround. Proper impl
> as intended

At about 06:45 UTC, in a longer message about hilavitkutin. The three sentences
that bear on arvo, in the order he wrote them:

> And whatever gets added, matches in taste and execution, is not a drop looser
> in typestate or strictness with no std, no alloc, unless behind a feature
> that is entirely opt in and not required for things to work
>
> Same for arvo of course
>
> If you look at the old arvo, the one hila pins for example. It used to have a
> whole lot going, and we'd like to bring it all back, too, just using the more
> sound and expressible, configurable, explicit framework and fundamentals

This job's reading, not his words. "Same for arvo of course" carries the
sentence before it onto arvo: whatever is added matches in taste and execution
and is no looser in typestate or strictness, with no std and no alloc, unless
behind a feature that is entirely opt in. The last sentence is the one the
families answer above restates: the old arvo's breadth comes back, on the
current fundamentals.

## On the eighteen ratified rows, the same day

Asked by the coordinator about 12:00 UTC, after he had chosen "Show me the
eighteen first" among [Apply it] [Show me the eighteen first] [Only the notes]
[Leave them] and been shown the list. The record is `briefs/fixup/op-answers-17.md`
on the channel, and this seat read it there.

The question: "Do the eighteen get `ratified_by = "op"` and the three notes
their corrections?"

The options: [All of it] [The eighteen, not the notes] [Some are not mine]
[Leave them]

> The eighteen, not the notes

This job's reading, not his words. The eighteen ratified rulings that carry no
`ratified_by` take `ratified_by = "op"`, which landed on `dev` as `b93f42d4`.
The three note rewrites are not applied and stay a
proposal, as `ratified-notes.proposal.patch`.

## Three answers at 20:25, the same day

Copied from the capture `202611132021_op-on-this-should-follow-the-same-pattern-as-ctx.md`, lines 34 to 108, with the questions as they were put and the options offered.

### The Warm overflow default, asked again

  ```text
  arvo: PRINCIPLES.md.tmpl:353-356 states a Warm overflow default that no canon row carries, and four FIXMEs plus the value tier's shipped-default round wait on it. What does Warm do on overflow by default?
  ```

Options offered:

1. Wrap

   Warm wraps by default; the per-case override picks anything else.

2. Clamp

   Warm saturates by default; the per-case override picks anything else.

3. No default

   Warm ships no overflow default: the consumer names the policy in the type, and leaving it out is a compile error.

4. Typestate decides

   The default falls out of the format's own declared constants (signedness, range kind), composed generically; no single answer for Warm as a whole.

Answered in their own words:

> I have answered this already. Make sure the answers I’ve given have been filed from before.

### Which of the two finish rows stands

  ```text
  arvo canon: ruling::the_run_ends_at_ratification_or_when_he_clears_it (your August addendum) runs into ruling::the_panel_finishes_the_canon_without_him. Which stands today?
  ```

Options offered:

1. Panel finishes, I ratify

   The panel finishes the canon without you; its rows stay provisional (stamped) until you ratify them. The August row is demoted.

2. Ends at my ratification

   The August row stands: nothing counts as finished until you ratify or clear it. The panel row is demoted.

3. Both, scoped

   The August row binds only that overnight goal; the panel row is the standing rule. Keep both, each saying its scope.

Chosen:

- Both, scoped

### The ingest door

  ```text
  arvo: proposal::validation_is_two_acts has a second act, a runtime membership door at ingest for each datum. ruling::never_a_runtime_check_and_one_lowered_path allows one exception that only you bless. Is the ingest door that exception?
  ```

Options offered:

1. Yes, blessed

   The ingest membership door is the one runtime check arvo has; everything past it is proven.

2. No runtime door

   Validation is compile-time only; data from outside is the consumer's to bring in already typed, and the second act is struck from the proposal.

3. Door, but not arvo's

   A runtime door exists at ingest but belongs to the consumer or a crate above arvo; arvo offers only the typed constructor it calls.

Answered in their own words:

> If it’s a valid use case with known use then sure, we can consider it. However, I am not sure it’s actually ever needed if the typestate works as intended and designed
