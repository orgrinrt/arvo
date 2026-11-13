# What op said on 13 November 2026, verbatim

Two answers reached this job as part of its brief, both from the coordinator's
question rounds of 13 November 2026. They are copied here verbatim, with the
options each was asked among, because the registry rows this job proposes carry
them as quotes and a quote with no record of what it was chosen among is worth
little to a later reader.

The typography is his, including the apostrophes. The readings under each answer
are this job's and are marked as such. Nothing here is his reading of his own
words.

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
> it's up to date and sensible etc)

This job's reading, not his words. He took none of the four as put. The intent:
the behaviour is not fixed one way, a default ships and is easy to override per
case, the default is a generic with a default, and the default may follow one of
the parity standards. The wrapper spelling is floated as a candidate mechanism
("Perhaps", a question) rather than ordered. The last clause is an instruction
about what downstream users should primarily reach for: the standard-paired
implementations of the theory and the contracts.

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

## Two earlier answers the same day, for context

On arvo's next steps, among [a: claim first, then rework the same branch in one
round, then continue the plan] [b: contracts first] [c: back to a short panel]
[d: land as built and correct in follow-ups], he answered: "Claim, then rework in
one round (Recommended)".

On arvo's public name, among [a: a thin crate named `arvo` again that re-exports]
[b: no facade, consumers import the inner crates] [c: decide at the port, with
the import list in hand], he answered: "Decide at the port".
