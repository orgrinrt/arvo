# Does it still earn its keep, asked again of what it became

Linus Torvalds, file 53. I wrote file 04, when the spec was one document proposing ten axes over a
type nobody had compiled, and I said the decomposition earned its keep and much of the superstructure
did not yet. Forty-nine files later the superstructure is a different object and my earlier verdicts
get no inheritance; where one of them turns out to have been answered rather than dodged, I say so by
name, because a reviewer who only ever restates his old objections is a reviewer who stopped reading.

**What I read.** `49_consolidation_four.md` in full, per the standing instruction, then the three
deliverables since it (`50_fog_the_float_model.md`, `51_fallin_the_last_tick_and_the_licence.md`,
`52_ringer_the_tests_that_were_owed.md`), then `ls` of the panel directory, then my own file 04 to
quote rather than remember it. Behind those I opened only the artifacts my own measurement composes
with: `41_probes/price/` and `42_probes/price/` (the committed compile-cost CSVs and their
generators), `42_probes/vu_bias_sealed.rs` / `vu_nat_sealed.rs` (the sealed tower, copied unmodified
into `53_probes/` exactly as files 46, 50 and 52 copied it), and the shipped `ifixed.rs` for one
claim check. I did not reason from `49:117`; file 50 named it a defect against section 1.15
(`50:224-229`) and I take that as settled.

**Gates.** Test gate, run fresh: `cargo test --workspace` from `mock/`, summed per binary, 654
passed, 0 failed, 122 result lines, matching every file since 41. Canon gate: the surface under
judgment has no shipped source; the consolidation's own corrected grep
(`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"`, and the
`FullRange\|UTerm\|AddWidth` variant) both exit 1, reproduced from the repo root for this dispatch.
So the rewrite-cost side of the ledger is what 49:699-703 says it is, and I verified its one concrete
claim rather than inheriting it: `mock/crates/arvo/src/ifixed.rs:8,43` does declare the width as
`{1+I+F}`, computed from precision-shaped parameters. Toolchain: every build in `53_probes/` ran on
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, and I verified the pin
resolves from the probe directory before running anything, because file 52 paid for that lesson and
I decline to pay for it twice.

**Compiled or measured against reasoned.** Section 3 is measured: the sweep in `53_probes/price/`
(generator, script, CSV, min-of-2 per point, `--emit=metadata`, trait-solve-only, the identical
build shape files 36, 41 and 42 used), plus two committed CSVs from 41 and 42 read as evidence.
Sections 1, 2, 4 and 5 are reasoning, and each judgment in them cites the compiled fact it stands
on at the point it is made. Nothing here is a timing claim about runtime; the one runtime cost this
review still owes (the software quantiser against hardware `fadd`) stays owed to `mock/benches/`
exactly as file 50 section 7 left it.

## 1. What is actually under judgment: the contract shrank while the derivation grew

The brief says the design is enormously larger than what I assessed. That is true of the review and
false of the thing a consumer or maintainer touches, and the whole verdict turns on keeping those
two apart.

Count the consumer-facing contract in the ratified table (`49:671-697`): `Numeral` has four members,
`Policy` has one (after file 51 closed tick 3, `51:139-145`), `Lowering` has three. Eight associated
types across three traits. Now count what the review has removed from that surface since file 04:
`Widening` gone (`49:279-281`), `Growth` gone from the key and now from `Policy` entirely
(`51:139-145`), `Int` dropped (`49:337-346`), flush-to-zero moved out of `Numeral` into a
`Quantisation` resolution (`50:490-497`), the three-relation fork replaced by a derived lattice
rather than three asserted relations (`49:220-227`), the bounded alias table refused (`49:1004-1007`),
division held rather than shipped (`49:423-427`). The droplist is over forty entries long
(`49:887-1027`). The review's kill rate is the strongest single piece of evidence available on the
brief's question, because "is any part of it elaborate machinery serving a case nobody has" is a
question this review has been asking itself continuously, with compiled refusals, and the machinery
that failed that test is in the droplist, not in the design.

What grew is the depth under the eight members: the value-unique tower, the seal, the gcd machinery,
the grade join, the adversary, the compile-fail pins. All of it is interior. A consumer writes a
numeral (through the notation macro, because file 47 proved they cannot write one by hand), an
operation, and a fold combinator; none of the four-deep stack appears in their signature. So the
brief's framing question, whether the whole thing can be held in a head, splits honestly into two:
the contract, which got smaller and is holdable by inspection, and the derivation, which nobody
needs to hold, because that is what the consolidation is for. Section 4 tests that claim against the
three files that just ran.

*Reasoned; the removals and counts each cite the consolidation or deliverable lines above.*

## 2. The four-deep stack is sound structure, and the review already extracted the proof

The brief asks whether encoding, then seal, then adversary, then compile-fail pins is a sound
four-layer structure or a sign the first step was wrong. Sound, and here is the argument at the only
level it can be settled: what the alternatives to step one compile to, and what the absence of each
subsequent layer measurably produced.

**Step one had no alternative under the stated constraints.** A quantity that is computed and has to
appear in a type is a type; the const routes are not merely unfashionable, they are compiled shut.
`50_probes/probe_3b_exponent_as_const_refused.rs` walks every permitted door: bare const arithmetic
refuses with "add `generic_const_exprs`" (forbidden), `min_generic_const_args` refuses the shape,
`generic_const_args` demands a solver mode the workspace records as mutually exclusive with the rest
of the arrangement (`50:242-263`). The normalisation-operator alternative (carry unreduced values,
reduce at use) is dead twice over in the droplist: referential uniqueness fails the ordinary case of
storing a product (`49:945-946`), and `Reduce` in a consumer-reachable signature diverges with
`E0275`, compiled by file 41 and re-compiled as a collision in a consumer combinator by file 48
(`49:306-324`). Given no dependent types, no GCE, and sizes const at type level, values-as-types is
the only door that opens. The first step was forced, not chosen.

**Each subsequent layer is the standard anatomy of a type-carried guarantee, and the review has a
measured counterfactual for every one of them.** The seal is the workspace's own
`what-you-can-observe-is-what-you-guaranteed` rule, which was written FROM this review's file 10; an
invariant whose carrier can be inhabited from outside is not an invariant, and the review watched
that happen, three times, at three layers (`49:349-362`). The adversary is what "sealed" means once
you refuse to let it mean "nobody on the panel thought of an attack": file 46's enumeration by
introduction route (`49:364-378`) is the only falsifiable form of the claim. The compile-fail pins
are the workspace's `a-test-that-cannot-compile-is-the-finding` discipline pointed at upkeep: a
refusal nothing pins can be deleted by accident, and file 52's classification of which pins are
contracts and which are measurements (`52:96-128`) is exactly the maintenance manual the stack
needed. Strip any layer and you get the defect the review actually observed at that layer's
absence: file 36's unsealed tower, files 41/42 believing the seal closed when it was open, the
solver-dependence a pinless adversary would rot into.

**What was actually wrong was the process, and the review both named it and fixed it.** Four passes
to close the seal is a real indictment, and the carrier-at-birth rule (`49:74-87`) is its repair:
run the two-obligation checklist at declaration time. The rule has now been applied prospectively
twice, to `Grade` (file 48, two lines, one pass) and to the exponent (file 50 section 4.2, two
lines, one pass), and both took one pass at zero measured cost. And the seal itself costs nothing at
scale: `42_probes/price/results.csv` has `alias` at 6,474 ms and `alias_sealed` at 6,431 ms for 400
compositions, indistinguishable. A four-layer structure whose marginal cost is two lines per carrier
and zero milliseconds, whose absence produced three real holes, and whose construction cost has
already been amortised into a checklist, is not a tumor. It is what carrying an invariant in a type
costs, and the design is paying it in the one place (the sealing crate) rather than exporting it to
consumers.

*Reasoned; every counterfactual cites a compiled probe or a committed CSV.*

## 3. The aggregate compile cost, measured: the multiplication nobody ran

This is the brief's sharpest instruction and it was right to give it: the review's per-composition
figures are milliseconds, favourable, and quoted in prose, while the aggregates have been sitting
unstated in committed CSVs. The clearest instance: `41_probes/price/results.csv` records **63,665 ms
for 400 full bias compositions at 16-bit operands**, linear all the way up, and that figure appears
in no prose anywhere in the review. The consolidation quotes the same data as "19.10 ms/composition
at that width and roughly an order of magnitude more at 16-bit operands" (`49:327-330`). Both
sentences are true. Only one of them makes a reader feel the sixty-four seconds. That is precisely
the quiet-reassurance pattern the brief suspects, and the fix is a number, so here are the numbers.

`53_probes/price/` (generator, sweep, CSV; sealed tower; min-of-2; same `--emit=metadata` build
shape as every prior sweep). Measured, this pin, this host:

| profile | marginal cost | linearity |
|---|---|---|
| dyadic composition (every shipped fixed-point numeral's shape) | **2.1 ms** each | linear to 200 (this sweep) |
| 16-bit random rational x rational | **143 ms** each | linear to 100 (this sweep), to 400 (41's CSV, 159 ms unsealed) |
| repeated site of an already-instantiated 16-bit composition | **28 ms** per site | 100 sites over 5 distinct = 3.36 s |
| `div_exact` by 44100, 48000, 4096 over a Q0.15 quantum | **~5 ms for all three** | trivially |
| one chained sample-rate conversion, 1/32768 x 48000/44100 | **~30 ms** once | n/a |

Four findings fall out, two of them corrections to what the review has been implying.

**First, the cost model has two terms, not one, because repetition is not free.** I expected the
solver cache to make repeated instantiations of one composition approximately free, and it does not:
a repeated 16-bit site costs ~28 ms against 143 for a fresh one. Roughly five-to-one, not
infinity-to-one. Any spec statement of the cost owes both coefficients: per distinct composition,
and per additional site.

**Second, the design's own headline use case lives in the cheap band, and the review's "harder, more
realistic case" wording (`49:329-330`) has it backwards.** The 16-bit-random-rational sweep is the
WORST case, not the realistic one: it multiplies two large co-random magnitudes so the Stein gcd
does maximal work. The realistic shapes are dyadic quanta (2.1 ms), division by real constants
(product numerator 1, gcd trivial, ~2 ms each), and the occasional chained conversion (~30 ms,
once). I generated the design's own named constants from `49:583-585` (44100, 48000, 4096) and all
three together cost five milliseconds. The expensive band exists, is real, and is inhabited by a
profile no current consumer has.

**Third, the aggregate for the consumers that exist is invisible.** hilavitkutin, the design's
heaviest real consumer, has twenty `UFixed`/`IFixed` sites (verified in file 04, section 0), all
dyadic: under 0.1 s against the 6.5 s whole-workspace check baseline file 04 measured. An audio
consumer with a dozen dyadic numerals, three sample-rate constants and two chained conversions is
under half a second. These are compile-once, `--emit=metadata` costs in the crate that declares the
numerals, re-paid on each edit of that crate; at these profiles the inner loop does not notice.

**Fourth, the cliff has a name and the spec should print it rather than leave it in a CSV.** The
profile that pays is many DISTINCT non-dyadic rational compositions: one hundred costs 14.3 s, four
hundred costs a minute. That profile is not fictional; it is what a code generator importing a
MATLAB fixed-point model with per-signal slope/bias pairs would emit, and MATLAB interop is the
axis's stated reason for existing (`49:301-304`). The toolbox rule says arvo documents tradeoffs
instead of policing them, so the obligation is one paragraph in the spec: the two-term cost model
with its measured coefficients, the statement that cost scales with distinct compositions, and the
note that dyadic and unit-numerator shapes are two orders of magnitude cheaper than arbitrary
rational pairs. A consumer who then chooses to emit four hundred arbitrary rationals does it with
the number in hand.

One boundary stated honestly: all of this prices the bias/adjustment composition machinery, the only
part of the design that exists in compilable form at width. The grade projection, the exponent sums,
and the notation macro's const-assert matrix are separately priced by their own files as cheap at
the single-composition grain, and none has been priced at aggregate scale; by the linearity result
here I expect them linear, and expectation is not measurement. The real-consumer compile-cost bench
(`49:855-857`) remains open; this sweep narrows it (the shapes and coefficients are now known) but a
synthetic sweep is still not a real consumer crate.

*grounded on: `pin`, `host`, `flags` (`--edition 2021 --crate-type lib --emit=metadata`); the 41/42
figures on `tree` at their committed CSVs; consumer counts on file 04's verified grep.*

## 4. Can it be maintained by someone who did not sit through fifty files? Tested, yes; with one defect class to kill

The brief asks whether a person can hold this in their head and whether someone who missed the
derivation can maintain it. The review has, without meaning to, been running that experiment for
three files straight, and the results are the strongest evidence either way that exists.

Files 50, 51 and 52 each entered on the standing instruction that the consolidation is the only
required reading. Working from it, file 50 built the largest missing piece (the float model) and
found the settled machinery absorbed it with "no new mechanism" (`50:471-472`); file 51 closed the
last ratification tick with a structural theorem the consolidation's own constraints supplied
(`51:124-131`); file 52 landed fifteen test artifacts whose classification leaned entirely on
distinctions the consolidation states. Three members, none of whom held the fifty-file derivation,
each extended the design correctly from the compressed form. That is what maintainability IS, and it
is demonstrated rather than hoped. The load-bearing content compresses to roughly a page: the
three-contract cut, one formalisation with an exponent function, the value-unique sealed carriers,
the two rules, the grade as a value-carried flag word, laws keyed on const-fn parameters, seal the
carriers and open the contracts. A maintainer holds that page; the registry holds what each claim
rests on; the probes hold the reproductions.

The defect class that would rot it: **the consolidation contradicting itself internally, which it
already does once.** `49:117` spells the exponent bounds `const` while section 1.15 derives that
they must be types, and a face-value reader builds the wrong thing and hits the wall three files
later, which is file 50's own words (`50:224-229`). One instance is a typo; the mechanism that
produced it is not. A consolidation hand-copies its tables from its sections, and hand-copies drift,
which is the exact failure file 44 diagnosed for prose claims (`49:611-619`) recurring in the
document that exists to prevent it. The next consolidation should treat every declaration line in
its tables as a claim with a ground like any other, and the eventual `*.md.tmpl` graduation should
render tables from the registry rather than by hand (`49:663-667` already names the mockspace tier
for this). Until then: one member of every consolidation pass diffs the tables against the sections
they compress. It is minutes, and `49:117` is what skipping it costs.

*Reasoned; the three-file experiment and the defect cite the lines above.*

## 5. What I would cut, walked honestly, including my own file-04 cuts re-judged

The brief says "this is a lot" is not an argument and asks for a part that serves nothing, a simpler
shape that loses nothing real, or a cost that will not be paid once it is a number. I walked the
current shape looking for all three. The honest result: the interior machinery survives the walk,
mostly because the review already cut everything that did not, and the droplist is where those
bodies are. What follows is what the walk actually caught, smallest first, then my own file-04 cuts
re-judged against what the design became, because three of them were answered and one is still
open and mine to sharpen.

**`Policy` is now a one-member trait, and the spec should say why that is fine before a reviewer
files it as vestigial.** After tick 3, `Policy` carries `Quantisation` alone (`51:139-145`). A
one-member trait looks like a wrapper begging to be inlined into `Numeral`. It is not, and the
reason is the ratified three-way cut itself: identity, policy and lowering answer three different
questions under D54's sorting test, and quantisation is genuinely not identity (two numerals with
identical value sets can quantise differently; the law keys depend on the distinction). Folding it
into `Numeral` would save one trait and destroy the sorting test. Keep it, and write the sentence,
because the sentence is cheaper than the future round that re-derives it.

**The `Specials` middle instance has no witness and should carry `unknown` until it gets one.** File
50 proposes three instances: none, infinities-only, IEEE (`50:506-510`), justifying the middle by "a
saturating format with an infinity but no NaN is a real shape". Plausible, uncited, and exactly the
shape of claim this review has struck before (`49:196-202`). The instance costs nothing to declare
and `ExactWindow`'s gate does want the distinction, so this is not a cut; it is a provenance demand.
Name a witness format or ground the instance `unknown` in the registry, per the registry's own slot
for that (`49:637-639`).

**The licence and receipt machinery stays spec text until a build layer exists to consume it.**
Files 50 section 5.3 and 51 section 2.4 together specify two receipt families (environment pinning,
algebraic-intrinsic licensing) for a post-monomorphisation verifier that nobody has built. Under the
panel's own rule this is designing the downstream contract rather than reporting the boundary, and
the hazards backing it are measured, not imagined (the one-instruction destruction of
`fold_compensated`, `51:259-272`, is the single best correctness cliff this stretch found). Keep
every word of it. But it is contract text owed TO a build layer, and the moment someone starts
implementing receipt plumbing inside arvo ahead of a consumer, that is the speculative layer this
design has otherwise refused to grow. `49:707-708` already states arvo grows no build harness; the
checkpoint should restate it next to the receipts specifically, and note in passing that
`49:707`'s "untouched by every deliverable in this stretch" is now stale in the harmless direction:
files 50 and 51 both extended the downstream contract, and the next consolidation's section 1.19
should absorb both receipt families rather than carry the "unchanged" sentence forward.

**The grounding registry must stay a review convention, not become shipped mechanism.** It has
earned its place operationally (it caught the `Int` drop, the seal conditionality, the struck float
member, `49:643-651`), and its two unbuilt tiers are correctly named-not-built. The failure mode
worth naming now, while it is cheap: a registry that starts costing more to update than the claims
cost to re-derive is process admiring itself. Its current form (a slug per load-bearing claim,
attachment by evidence bin) is under that line. Hold the line.

Now my own file-04 cuts, re-judged.

**The float axes: I said they described arithmetic arvo would never execute, and the design answered
by building the arithmetic.** File 04 section 1 called `Stored<BITS, U>` "simultaneously
inexpressive of real IEEE formats and operationally uninhabited". The current design's `Ranged` is
neither: the identity contract expresses the real formats in mathematical coordinates, and file 50's
software quantiser is the executable semantics, validated against silicon on 41 million operations
with zero mismatches (`50:49-51`) and made the DEFAULT lowering by derivation from a ratified
invariant (`50:553-566`). That is the right way to answer an uninhabited-axis objection: inhabit it.
The objection is dead and I am glad to be the one to bury it.

**The algebra ladder: my cut landed, in substance if not in wording.** File 04 section 5 said
declare the rungs the derivation consumes and backlog the rest. The current design's ladder is
derived rather than asserted, every named rung is consumed by the law machinery or by a compiled
"no" with the failing axiom named (`49:276-280`), and the uncalled `Magma`-style vocabulary never
returned. Nothing left to cut there.

**The `Precise` exile from the algorithm crates: still unanswered, and it is now the oldest open
consumer question in the review.** File 04 section 3 established that a fallibility-carrying numeral
does not satisfy the total `Add` bounds of `arvo-graph`/`arvo-sparse`/`arvo-comb`/`arvo-spectral`
(`arvo-graph/src/lib.rs:10-12`, verified then). Fifty files on, the review has a grade lattice, a
`Definite` bound, and a droplist entry refusing to gate the algorithm crates on `AddAssoc`
(`49:896-898`), and still no statement of how the four L2/L3 crates, the design's oldest real
consumers in its own repo, consume the graded surface. File 47 proved the method that finds these
defects: type the surface from the outside (`49:881-885` carries it forward for three other
surfaces). The algorithm crates are the fourth, and I name them as the next consumer-typing dispatch
after the ones already queued. Whoever runs it should expect my file-04 finding to have grown teeth:
the fold combinators now PUBLISH grades, and a topo sort accumulating rank weights has to say which
combinator it folds with and what its published grade means for the returned ordering.

**The newtype faces: my own proposal, shrunk honestly against the current design.** File 04 sections
2 and 7 proposed concrete faces over the composition so diagnostics name what the consumer wrote,
decidable by a fixture sketch. The current design changes the balance in both directions. The
notation macro (ratified intent, `49:749-752`) fixes WRITING numerals but explicitly not READING
errors: `49:600-603` concedes rustc expands aliases in diagnostics and no route was found. That
makes the diagnostic problem worse than file 04 measured, since a macro-emitted numeral is deeper
than anything I probed then. But the face fix is also smaller than file 04 thought: operations are
keyed on the numeral type, so `mul_full` over two faces delivers a `Number`, not a face, and the
unreadable type reappears one operation into any expression. Faces fix declaration-boundary errors
only. The honest residue of my proposal: the decoder-ring open item (`49:839-842`) and the face idea
are one item, its ceiling is "the first error in an expression names the face", and the one-day
fixture sketch from 04 section 7 is still the way to decide whether that ceiling is worth the
forwarding layer. I would run it before the notation macro's vehicle sketch, because the two share
the fixture corpus and the macro's design should know whether it is emitting aliases or faces.

*Reasoned throughout; each judgment cites its lines.*

## 6. Verdict, in the form the checkpoint can take

The design earns its keep, and this time the sentence has numbers and a demonstrated maintainer
experiment behind it instead of a decomposition I liked and a superstructure I distrusted. The
consumer-facing contract is smaller than the one I reviewed at file 04. The four-deep stack is the
standard anatomy of a type-carried invariant, each layer's absence has a measured casualty in the
review's own history, its marginal cost is two lines per carrier and zero milliseconds at scale,
and the process defect that took four passes is repaired by a rule that has since delivered
one-pass seals twice. The aggregate compile cost is linear, two-term (per distinct composition plus
per repeated site), invisible for every consumer profile that exists (~2 ms dyadic, ~5 ms for the
design's three headline constants together), and has one real, nameable cliff (143 ms per distinct
arbitrary 16-bit rational composition; 14.3 s per hundred) that belongs in the spec as a printed
cost model rather than in a CSV nobody quotes. Maintainability is not conjecture: three consecutive
members extended the design correctly from the consolidation alone.

What I ask the checkpoint to carry: the two-term cost-model paragraph into the spec, with this
file's coefficients until a real consumer crate re-prices them; the table-versus-section diff as a
standing consolidation obligation, with `49:117` as the exhibit; the one-sentence defence of
one-member `Policy`; a witness or an `unknown` ground for infinities-only `Specials`; the L2/L3
algorithm crates as the next consumer-typing dispatch, carrying file 04's `Precise` finding with its
new graded-fold teeth; and the decoder-ring diagnostic merged with the face question into one
fixture sketch, run before the notation macro's vehicle sketch. I would cut nothing that ships. The
review already cut everything I would have, and the droplist is the receipt.

I suggest; op decides.

## Provenance summary

Compiled or measured, this dispatch, on the pinned nightly from inside the repo tree: the
`53_probes/price/` sweep (eleven points, min-of-2, CSV committed), the smoke builds of the headline
and chained generators, the test-gate run (654/0 across 122 binaries), both canon-gate greps, and
the `ifixed.rs:8,43` claim check. Read as committed evidence rather than re-run:
`41_probes/price/results.csv` (16-bit, to 400 compositions), `42_probes/price/results.csv` (8-bit,
sealed against unsealed). Reasoned: sections 1, 2, 4, 5 and 6, each judgment citing the compiled
fact or consolidation line it rests on. Quoted rather than recalled: my own file 04, re-read for
this dispatch.
