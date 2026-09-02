# Op's thirty-eighth checkpoint: the alias-reach question is closed and out of scope

**Date:** 2026-08-07
**Position:** after `144b`. Closes a thread that `143` opened as a limitation and `144` built a mechanism
for. **Required reading before any further work on the profile axis.**

## The question

When a framework writes `StrHandle = UInt<5>`, uses it everywhere, and someone annotates a function whose
body is full of `StrHandle`, does the annotation affect that code?

`143` established that a scoped rewrite cannot reach through a module-scope alias, and filed it as a limit.
`144` observed that in the wholesale-adoption tier **every** type is a module-scope alias, so under those
routes the annotation does nothing in the tier op called the main one, and proposed a mechanism to fix it.

## Op's answer

> This is a good catch, which is one of the rare occasions where I legitimately didn't see it. However, this
> is only a problem on the macro level, where we do not have typestate and work on ast level or thereabouts.
> The alias itself does contain the full typestate and is reachable from there, but proc macro that rewrites
> bodies as tokens, doesn't have that, that is worrying that I didn't see it.
>
> However, I don't think this is meaningful in the big scope but within arvo: The notko synergy is purely
> addition and a bonus, but even as such, not our concern. If there is an answer for this, I'd be curious,
> but I am almost certain there simply can't be because the attribute macro sees tokens and token streams.
> It's a minor inconvenience but nothing too critical to pivot to.

## What this settles

**The limitation is real and it is not arvo's.** It exists at the macro level, where there is no typestate
and the rewriter sees tokens. It does not exist at the type level: **the alias carries the full typestate
and is reachable from there**, so arvo's own story is unaffected.

**So the thread closes for this panel.** The notko synergy is an addition and a bonus rather than a
dependency, and per `144b` the two projects have synergy and no continuity. A limitation in notko's rewriter
is not an arvo canon finding.

**And `144`'s proposed mechanism is not needed.** The `Ambient` marker with resolution at the operation was
built to solve this, it compiles and its probes run, and it is solving a problem that belongs to another
project. It stays in the record as a worked answer rather than as a proposal, and nothing in arvo's canon
should be built on it.

## The answer to op's curiosity, recorded because he asked

Op's reasoning is that no answer can exist, because the attribute macro sees only tokens. That premise is
right and the conclusion does not follow.

**A proc macro cannot resolve types, but it does not have to.** It only needs to emit something the type
system resolves afterwards. The macro injects a marker into the region; the type-level machinery performs
the resolution when the operation is type-checked. The alias keeps one identity, the macro never learns what
`StrHandle` means, and its token-level blindness stops mattering because it is not the thing deciding.

That is precisely the shape `144` arrived at independently, and its probes show the pieces work: the alias
identity is unchanged, a region's posture reaches through the alias, an explicit operand still refuses
inside a region that would otherwise retarget it, and the apparatus folds away in the emitted code.

So it is possible in principle. Whether it is worth building is a separate question and belongs to notko
rather than to this panel.

## What the panel does with the affected material

`143`'s limitation stands as a true statement about a macro-level rewriter, scoped to that. `144`'s
mechanism stands as a compiled demonstration that the limitation is surmountable, scoped the same way.
Neither is arvo canon and neither blocks anything here.

The parts of both files that are about arvo are untouched and remain live, as recorded at `144b`.

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds. Experts are dispatched one at a time, each reading the ones
before it, each writing incrementally, each going down the rabbit hole rather than reporting blockers.

**Do not assume arvo and notko concepts correspond**, and do not treat a notko limitation as an arvo
finding. Where a brief needs to relate them, it states the relation as a question rather than as a premise.
