# Op's thirty-seventh checkpoint: notko is not arvo, and the dispatching agent framed it wrong twice

**Date:** 2026-08-07
**Position:** after `144_wingo_the_profile_axis_second_read.md`.
**Required reading**, because it voids framing carried by two panel files and by the briefs that produced
them.

## The ruling, stated twice by op in one round

> Notko or hv are not directly associated with arvo. The concepts need not align, they are different things
> for different purposes and in different projects. They have synergy, but no continuity as such.

And again, on a separate finding:

> Again, arvo strategy is not the same as notko optimize for profiles. They have synergy, nothing more.

**notko and hilavitkutin are separate projects with separate purposes. arvo's strategies and notko's profile
tiers are not the same concept, are not required to align, and share only synergy.**

## What this voids

The dispatching agent read op's earlier "roughly translate" and "pretty much analogous" as "must align", and
wrote that reading into the briefs for `143` and `144`. Both files inherited it. Three of their findings
dissolve rather than being answered:

**The `Cold` name collision is not a collision.** `144` found that notko's `Cold` tier is always `Outcome`
and therefore fallible, while arvo's `Cold` strategy is infallible, and reported one attribute value giving
two answers about whether a function can fail. Under the ruling these are two words in two projects that
happen to coincide. There is no shared vocabulary to reconcile.

**"No tier file can express an arvo posture" is not a gap.** `144` found that a notko tier file's keys are
`based_on`, `inline` and `panic_fmt` over notko's own three-element carrier, and reported that as the real
finding behind `Precise` having no tier. A notko tier file is not supposed to express an arvo posture. The
absence is the separation working.

**notko's test coverage is not the panel's concern.** `144` reported that notko's suite never exercises the
release arm, that its doc comment claims a verification the file does not contain, and that a rewrite path
discarding an `Err` arm has no test. Those may be real defects in notko. They are not arvo canon findings
and they do not belong to this panel.

## What survives, and it is most of both files

The separation does not touch the parts of `143` and `144` that are about arvo. Independently reached by
both, and therefore carrying two groundings:

- A bare `T: Add<Output = T>` carries nothing about width, range, resolution or container, and the
  representation travels with the `T` the consumer supplied. arvo reads it rather than deriving it.
- A total signature excludes a refusing posture.
- The bare-primitive path is `Warm` by derivation rather than by default.
- An explicit declaration beats an enclosing annotation.
- A scoped mechanism must not reach through a domain alias.
- An attribute may select a strategy and must not resolve a cell.

And `144`'s two corrections to `143` stand, since neither depends on the false premise:

**The output generic does have a spelling.** `143` said it has none in Rust. It ships in core as
`Add<Rhs = Self>`, and two shapes compile on the pin: the output as the trait's own defaulted type
parameter, and a defaulted mode marker with the output projected from it. **The correct law is narrower: a
type-parameter default fills where the trait is written and never where it is inferred.** So it is a
mechanism for signatures rather than for expressions.

**The join question is closed on better grounds.** `143` closed it on D72's "nothing else", which is a
crate-contents table whose own text resolves `arvo-strategy`'s identity by emptying it. The ratified
argument is that the view lattice is not a chain, with `Hot` on a signed numeral and `Precise` below
interior safety at incomparable points. Same conclusion, and it does not evaporate if a fifth posture is
ever added.

## The question the dispatching agent asked badly, restated

Op, on a question put to him about what the attribute reaches:

> I really don't understand what this is trying to say and frame? Either I'm confused or you are.

The agent was. The question underneath it is one sentence and it is still open:

**When someone writes `StrHandle = UInt<5>` and then annotates a function, is the annotation supposed to
affect the code using `StrHandle`, or not?**

Everything in `143`'s reported limitation and `144`'s proposed answer hangs on it. `143` established that a
scope selection cannot reach a module-scope alias and filed it as a limit. `144` observed that every type in
a wholesale-adoption body **is** a module-scope alias, so under those routes the attribute does nothing in
the tier op described as the main one, and proposed resolving at the operation instead. Whether that
proposal is needed at all depends on the answer, and it is op's.

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds. Experts are dispatched one at a time, each reading the ones
before it, each writing incrementally, each going down the rabbit hole rather than reporting blockers, and
each taking small wins because many of them are the program.

**And a standing correction for every future brief: do not assume arvo and notko concepts correspond.**
Where a brief needs to relate them, it states the relation as a question rather than as a premise.
