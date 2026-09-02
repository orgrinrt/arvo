# The profile axis, second read

**Date:** 2026-08-07
**Position:** after `143_orchard_the_profile_axis_and_the_tiers.md`. The independent second read the
two-expert rule requires, formed before opening `143`.
**Probes:** `144_probes/`, eleven, with `run.sh` and the captured `output.txt`.

Hey hackfolk. I was sent to check a file that proposes mechanism nobody else has checked, on a question
that only entered the panel's standing base one checkpoint ago. So the useful thing I can do is not to
grade `143`. It is to answer the same seven questions from the source and the compiler, write down what I
got, and then say where the two answers diverge and what the divergence costs.

They agree on more than they disagree on, and the agreements are worth having as agreements because we
reached them separately. Where they diverge, three of the divergences are corrections and one is a hole
`143` walked up to, looked at, and recorded as scenery.

The hole is the one I spent the dispatch on. `143` establishes, correctly and with a compiled diagnostic,
that a scoped mechanism cannot reach through a domain alias without breaking the alias's type identity. It
files that under the precedence rule as a limit that falls out for free. What it does not ask is what the
attribute is then *for*, because a domain alias at module scope is op's tier two, and tier two is the path
op describes as wholesale adoption. Under `143`'s preferred route the attribute does exactly nothing over
a tier-two body. That is not a limit, it is the mechanism missing its largest consumer, and the rest of
this file is the attack on it.

## Verdict

The seven questions have answers. Six of `143`'s seven survive my read with amendments ranging from
cosmetic to structural, and the seventh, the output generic, is stated more strongly than the evidence
carries: the mechanism op describes does have a spelling in Rust, and it is the spelling `core::ops::Add`
already ships under.

The mechanism I would put to op is not a type rewrite at any granularity. It is two pieces that together
reach every tier without breaking any of them. **The elision is a marker**, so a posture-neutral spelling
is one type everywhere and a domain alias keeps one identity. And **the resolution happens at the
operation**, so the enclosing scope retargets the arithmetic without touching the stored type. Precedence
then stops being a heuristic a macro applies to spellings it can see, and becomes a total function on
types that the compiler checks. It compiles on the pin, its laws hold over the whole grid rather than a
sample, its negative control fails as it must, and the entire apparatus folds to the same symbol as a
bare `u32` add.

## How this read was conducted

The canon gate first. This dispatch is canon work under the panel's standing shape, so what I owe is
intent that survives an implementation rewrite and that three independent implementers would reproduce.
Nothing here proposes source, `mock/crates` was read and not written, and the probes carry the viability
evidence rather than the design.

I checked the brief's cheap factual claims before reasoning from them, because `143` found its own
predecessor citing a rule that named an attribute which does not exist. The brief's claims about the
shipped attribute reproduce: `notko-macros/src/lib.rs:22` declares `pub fn profile`, the argument is a
bare ident (`notko-macros-core/src/parse.rs:14-18`), the built-in markers are three
(`tiers.rs:53,61,69`), and `rewrite/mod.rs:24` parses the annotated item as `syn::ItemFn`. One brief claim
does not survive, and it is `143`'s rather than the brief's: see question seven.

I read `142c` in all four of its parts, `142b`, `140b`, `137b`, `135b`, and the standing base at `124` by
section rather than whole, since it is 576 kilobytes and the sections that bear on this are named. I then
formed answers and built probes. Only then did I open `143` and its probes.

**On op's granularity ruling, which landed mid-dispatch.** Op called it as settled canon: a constant is a
function that does not vary over its domain, so "constant or function" was never a fork, and all things
change and act granularly rather than `Warm` alone. And in the clarification that followed: nothing
changes in the standing base, the existing preset tables are one arm, plausibly the arm reserved for
debug-assertions time, and further arms are written separately and later. So the useful question about a
cell is which arm its table is and which arms are not written yet. I have taken that as governing, I have
catalogued no unstated variation, and where my work implies a second arm I name the arm and what would
distinguish it rather than proposing an edit to anything existing.

## The shipped attribute, read from the source up

I read the representation before the prose, which in this case means the four files under
`notko-macros-core/src/` and the two under `notko-macros/`, about 1200 lines including tests and READMEs.
Then I built it and put the attribute on things.

The shape is clean and the shape is the finding. A **tier** is a name resolved at expansion time. A
**rewrite strategy** is a three-element behavioural set, `Passthrough`, `Hot`, `Cold`
(`tiers.rs:78-86`). Tiers map onto strategies many-to-one, and the map is not injective because `Warm`
maps to `Passthrough` (`tiers.rs:64`). Resolution searches built-ins, then
`$CARGO_MANIFEST_DIR/notko-optimizers/<Name>.rs`, then `$NOTKO_OPTIMISERS_PATH/<Name>.rs`, then errors
(`discover.rs:39-40,82,93`). A tier file carries exactly three keys: `based_on`, `inline`, `panic_fmt`
(`discover.rs:122-124,137,151,166`).

So the name space is open and the behaviour space is closed at three. `143` says the same and calls the
shape good, which it is. What follows from it is the part that decides question seven, and I take it up
there.

Four things I established by putting the attribute on things rather than by reading the parse
(`144_probes/p07_notko_granularity/NOTES.md`). An `impl` block and a `mod` are both refused with
`error: expected fn`. A **trait method is not refused**: it parses, the return type is rewritten to
`::notko::Outcome`, and the impl then fails to match the trait it implements with `E0053`. And
`#[profile(Precise)]` is refused with a diagnostic that names the three built-ins and prints the absolute
path where a custom tier would live, which is a good error and worth copying.

## Question one: what the attribute rewrites, and at what granularity

**Today, and as a fact rather than a design position: exactly a `syn::ItemFn`, and the rewrite is
value-level.** It replaces `sig.output`, then walks the body replacing `Ok(x)`, `Err(e)`, and one `match`
shape. It does not touch types other than the return type, and it knows nothing about arvo.

Two amendments to the way `143` states this.

The unit is not "a function item". It is **anything shaped like an `ItemFn`**, which includes a trait
method, and on a trait method the rewrite is silently wrong until the trait bound is checked. That is a
different failure from the `impl` and `mod` refusals, and it is worth knowing because a trait method is
exactly where an arvo algorithm's operations would live.

And the rewrite is already **conditional**, in the shipped code, on two things at once: `debug_assertions`
and a cargo feature (`rewrite/hot.rs:25,28`). This is the one place in the workspace where op's arm
vocabulary exists in shipped form, and it is worth naming as the precedent. `Hot` has two arms. `Cold` and
`Warm` have one each. Under op's ruling that is not a defect and not a special property of `Hot`; it is
that `Hot`'s second arm has been written and the others' have not.

**What I would put in the canon for this question**, as intent rather than mechanism:

> A profile annotation names a posture over a lexical region. The region is a declaration made by a
> consumer at a site, and its extent is the scope of the item it annotates. What the annotation supplies
> is a posture, never a resolved cell: a resolved cell depends on conditions the annotating layer cannot
> see, and every layer that has tried to resolve one has had to emit both arms instead.

The second sentence is the durable half. `143` reaches it from the same evidence and states it as "an
attribute can select the strategy; it must not try to resolve the cell". We agree, independently, and I
would keep that sentence.

## Question two: what inference derives from a bare bound

**From `T: Add<Output = T>`, arvo derives nothing about width, range, resolution, container or
signedness.** I agree with `143` completely here and reached it the same way, which is that there is
nothing in the bound to read. The representation does not get derived; it arrives with the `T` the
consumer supplied, and arvo reads it. An algorithm deriving a width from its own bound would be answering
a question it asked, which is the direction that never composes.

Where I would go further than `143` is on the one case where something *is* determined, because `143`
calls the bare-primitive bridge "derived rather than default" and stops one step short of the reason that
makes it stick.

**The primitive bridge is not a default and not merely a derivation. It is the unique assignment under
which the embedding preserves behaviour.** `140b:16-21` records op defining `Warm` as behaving like a
native Rust primitive and `142b:19-22` states it as the ruling. So for a bare `u32` the numeral is forced
by the bit count and the sign, and the posture is forced by the definition: any posture other than `Warm`
changes the observable behaviour of `u32 + u32`, and an embedding that changes what it embeds is not an
embedding. There is no free parameter, and there is
no choice that could have been made differently.

That matters for the canon in a way a default does not. A default is a decision someone could revisit. A
uniqueness statement is a theorem with one hypothesis, and the hypothesis is the `Warm` definition. It
belongs next to that definition, because it is a consequence of it that nobody reading either one in
isolation would connect, and because `140b:49-57` records that the `Warm` intent has failed to stick three
times. A consequence attached to an intent is one more surface the intent can be recovered from.

**And the totality inference is real.** `Add<Output = T>` asserts that addition returns a `T`, and a
refusing posture returns a fallible shape, so a bound written for totality excludes it. `143` establishes
this with `E0271` and I agree. I would state it in the canon in the form that survives a rewrite: **the
shape of an operation's result is a function of the posture, so a signature that fixes the result shape
constrains the posture.** That is one sentence, it is checkable against each posture, and it is why the
fallibility axis and the numeric axis are not independent, which is the whole of question four.

## Question three: precedence

An explicit declaration wins over an enclosing annotation. Op said it, `143` says it, I say it, and I
reached the same ratified ground independently: `124:3579`, "a build layer reads every axis, acts freely
on `Lowering`, acts on `Policy` only inside its own declared envelope, and never acts on `Numeral`". A
posture swap changes the over-range row, which is `Policy`, so it is licensed only inside a declared
envelope, and the annotation is that declaration. An explicitly written posture is a `Policy` fact stated
outside the envelope and the build layer may not act on it.

Two experts, two independent readings, both grounded in that quoted sentence. I record that the way the
two-expert rule asks: I formed this before reading `143` and I am the second read, so the agreement is
corroboration rather than confirmation.

**Where I disagree is the argument against a join, and the correction matters because `143`'s argument
evaporates under a change op might well make.**

`143` argues that a join is forced out because the four postures are not a lattice and the missing join
point would need a fifth marker, which D72 forbids by giving `arvo-strategy` "`Hot`, `Cold`, `Warm`,
`Precise`, and nothing else" (`124:3602`).

That reads a crate-contents table as a closure property. D72 is titled "One crate per contract, and
`arvo-strategy` keeps only the presets" (`124:3594`), and `124:3619-3620` says what its "nothing else"
does: "It resolves C1's question about `arvo-strategy`'s identity by **emptying it**". The sentence is
about the crate holding no machinery. Deriving "no fifth posture may ever exist" from it is a written
artifact standing in for a derivation, which is the defect `142c:94-97` names as the pattern and asks the
canon to say so where the tables are introduced.

**The right argument is already ratified and sits four hundred lines away.** `124:1386-1391`:

> the set of views under which a law holds is downward closed and closed under join, so **every law has a
> unique finest view** [...] The named relations are three points of a nine-point lattice, **and the
> lattice is not a chain**: `Hot` on a signed numeral and `Precise` below its accumulator's
> interior-safety threshold sit at incomparable points (one preserves values and definedness while losing
> quantisation events, the other preserves values and events while losing definedness).

So the join of two postures **exists**, in the view lattice, and it is **not a posture**. That is a
statement about the mathematics rather than about a crate's contents. It survives an implementation
rewrite, three teams would reproduce it, and it does not depend on the posture set staying at four: if op
adds a fifth tomorrow, the join of two incomparable views is still a view rather than a preset, and the
disposition is still precedence or refusal.

`143`'s conclusion is right. Its ground is a table cell and mine is a theorem, and I would carry mine.

**On the disposition itself I agree with `143` and would sharpen the report.** Precedence plus a report,
per `arvo-toolbox-not-policer.md`'s "diagnostic, not directive". What I would add is what the report must
say, because a lint saying "this was not rewritten" is not actionable. It should name **which** posture
won and **where the losing one was declared**, since the consumer who wrote the annotation and the
consumer who wrote the alias are frequently different people in different files, which is exactly op's
tier split.

## Question four: profile against strategy, and why op said "roughly translate"

This is where I part company with `143` on substance rather than on ground.

`143` reads the residue in "roughly translate" as a sort error plus a totality failure: a tier name is a
lexical key, a strategy is a type-level marker, and the map is not total because `Precise` has no tier.
That is true of the shipped artifact and it is the smaller half of the answer.

**The larger half is that a profile value is not a name for a strategy. It is a name for a posture, and a
posture selects a point in whatever carrier the library it is applied to happens to have.**

notko's carrier is fallibility, with three points: keep the source shape, drop the error and pay nothing,
report the error and pay for it. arvo's carrier is the product of a rounding direction, an over-range and
under-range disposition, a stored width and a layout (`124:2601-2607`), and four points are named on it.
`Hot` names one posture and it lands on a different coordinate in each carrier. That is why the word
"roughly" is there: a posture is not a strategy, it **selects** one, per carrier, and a selection map is
not an identity however similar the names look.

Two consequences, and the first is the sharpest thing I found that `143` did not.

**notko `Cold` and arvo `Cold` are different behaviours under one name, and they disagree on whether an
operation can fail.** notko's `Cold` is "always `Outcome<T, E>`" (`notko-macros/README.md:12`,
`tiers.rs:68-74`), which is the refusing, reporting point of the fallibility carrier. arvo's `Cold` is
`ToEven`, clamp, minimum stored width, bitpacked (`124:2601-2607`), which is **infallible**. On the fallibility
coordinate specifically, notko `Cold` coincides with arvo `Precise`, not with arvo `Cold`. They agree on
the intent framing, seldom computed so it may pay more (`124:2617-2619`), and diverge on what
that intent buys in each carrier.

So a consumer writing `#[profile(Cold)]` over arvo code today gets the fallibility rewrite of a refusing
posture, and would get the clamping numeric posture if the arvo half ever landed by name-matching. Those
two disagree about whether the annotated function can fail. That is a live name collision spanning two
crates inside one attribute, and it is not a spelling problem: it is two carriers whose most conservative
points are not the same posture.

**And the direction of `143`'s totality failure inverts under this reading.** The map runs posture to
carrier point. It is total on notko's carrier for the three postures notko names, and what is missing is
an entry for the posture `Precise` on notko's carrier, whose most conservative point is already spoken
for. Saying "`Precise` has no tier" locates the gap in the tier table. Locating it in the carrier is more
useful, because it says what a fourth entry would have to be rather than that a row is blank.

**What I would put to op as intent:**

> A profile names a posture: how much this region may pay, and in which currency. Each library that a
> posture reaches instantiates it over its own carrier, and the instantiation is a named, total map from
> the posture set to that carrier's points. Two libraries may map one posture to points that behave
> differently, and that is correct, since the carriers are different. What is not permitted is two
> libraries mapping one posture name to points that disagree about whether an operation can fail, because
> the result shape is observable in a signature that both libraries rewrite.

The last clause is the load-bearing one, it is checkable, and today it is violated by `Cold`.

## Question five: agreement between the tiers

I agree with `143` that the agreement discharges by construction rather than by care, and my question-two
uniqueness statement is the same argument made one notch stronger: the tier-one bridge has no free
parameter, so there is nothing to get wrong.

I also agree with the observation `143` makes and that I would not have found as quickly: **under an
annotation the two tiers genuinely diverge**, correctly, because tier one's bare primitive picks up the
ambient posture while tier two's alias keeps its own. `143` says the canon should state that rather than
let a reader infer unconditional agreement, and it is right.

But I would state the obligation differently, because "a cell reached by inference and a cell reached
through an alias must land in the same place" is not quite what the design needs.

**What must agree is not the cell. It is the behaviour at the boundary between the two.** Two consumers
computing under different postures is the design working. What would be a defect is a value produced under
one posture flowing into code that assumes another without anyone being told. So the obligation to write
down is:

> Where a value crosses out of the region whose posture produced it, either its type records the posture
> it was produced under, or the crossing is refused. A posture may not be silently forgotten at a
> boundary.

That is checkable, it is what the compiler already does under every route anyone has proposed, and it
survives the tiers diverging, which the cell-agreement form does not.

## Question six: the output generic and its default

`143` establishes that "the mechanism as literally described has no spelling in Rust" and it does so four
ways. I reproduced the central one and I agree with the narrow claim: a default on a **function's** own
type parameter is refused (`p06a`, and worth noting it is a future-incompatible lint,
`invalid_type_param_default`, issue #36887, rather than a parse error, so it once compiled and is on its
way to a hard error).

**But the general claim is stated too strongly and the overreach costs a real mechanism.** Op's sentence
is "it will have a generic to describe the output, and they can override its default". That mechanism
ships in `core`. It is spelled `Add<Rhs = Self>`: a defaulted type parameter on a **trait**, which fills
in wherever the trait is named and which the consumer overrides by naming it.

I built both candidate shapes and both compile and run on the pin (`p06b`). The output as the trait's own
defaulted parameter, with two non-overlapping impls, reached through a bound that elides it. And a mode
marker with a default, with the output an associated type projected from the mode. Both give op's intent:
do nothing and get the identity back, name the override and get the other shape.

`143`'s stronger claim also carries one sub-claim that does not hold. It says the output cannot be an
associated type "because two impls for one self type is `E0119`, and the only route past `E0119` is
specialisation, which is forbidden". `E0119` bites only if the output is an **associated** type. As a
trait **parameter** the two impls do not overlap and there is no coherence problem at all: `p06b` compiles
`impl Sum<u32> for u32` and `impl Sum<Erased> for u32` side by side. So the closed route is narrower than
stated.

**The correct law, which is the one to put in the canon, is narrower than `143`'s and wider than nothing:**

> A type-parameter default fills where the trait is written and never where the trait is inferred.

I checked the failing half independently and it holds in both shapes: at a method call the default does
not apply and the call is ambiguous, `E0283` for the output-as-parameter shape (`p06c`) and `E0284` plus
`E0283` for the mode-marker shape (`p06d`). `143`'s `p4`, `p5` and `p9` establish the same half by three
other mechanisms, so this now rests on five independent instances and I regard it as settled.

What follows is a design consequence rather than a limitation. **The mechanism is available exactly where
op's tier one lives and unavailable exactly where it does not.** A tier-one consumer writes
`fn mine<T: Algo>(...)`, which names the trait, so the default fills. A bare `x.algo(y)` with two
candidate impls does not name it, so it does not. Op's tier one is described as trait-bound shaped
(`142c:236-239`), so the shape the mechanism serves is the shape op described.

**And there is a fourth route neither of us listed, which I would rank first on the ergonomic bar.** Put
the override on the **result** rather than on the call: one impl, no default, no parameter, and the
consumer who wants the erased form writes a postfix on the value. It has no ambiguity anywhere, it needs
no defaulting rule, its diagnostics are the best available since a missing method names itself, and it
meets `142c:352-364`'s standard of a name and nothing else. Its cost is that the erased form is not
reachable by naming a type in a signature, which a consumer who wants it in a signature can spell
directly. I would want op to weigh that against `143`'s projection route, which is also good and which
solves a slightly different problem.

I share `143`'s uncertainty about what "simplification and arvo erasure for the return value" means, and I
have nothing to add beyond agreeing that the two readings are different mechanisms.

## Question seven: where the boundary sits

The ratified answer is `124:3579` and both of us found it independently, so the agreement stands with two
groundings. The annotation is the envelope declaration, a posture swap is a `Policy` act, and precedence
and the alias reach limit are both entailed rather than chosen.

**Where I disagree is the claim that this was already settled and nobody connected it.** The principle
was. The vehicle is not capable of carrying it, and that is a different report to give op.

A custom tier's file carries three keys and only three: `based_on`, `inline`, `panic_fmt`
(`discover.rs:122-124`). `based_on` accepts exactly the three built-in names (`tiers.rs:93-103`). So the
behaviour space a tier can select from is notko's three-element fallibility set, and **there is no arvo
coordinate in it at all**. The extension path is open on names and closed on behaviour, and the closed
behaviour has no slot for a posture on arvo's carrier.

I confirmed the consequence rather than inferring it. `#[profile(Precise)]` resolves once a four-line
`notko-optimizers/Precise.rs` exists, and what it produces is Cold's rewrite: the expansion is
`pub fn precise_fn(x: u32) -> ::notko::Outcome<u32, E>` (`p07_notko_granularity/NOTES.md`). A `Precise`
tier defined this way is indistinguishable from `Cold` in every observable.

So "`Precise` has no tier" is true, and it is true for a reason worse than a blank row: **no tier file can
express a posture arvo would recognise, because the file's vocabulary is notko's carrier.**

**The slot is already reserved and unread.** `tiers.rs:125-130` declares `CustomTier::source_path` with
the comment "Currently unread; reserved for the notko-build cross-crate accumulation path and future
helper-module injection", and `notko-macros-core/tests/extension.rs:38` asserts it is `None` for a
marker-derived tier. That is a hole in the shipped code shaped like the missing mechanism. What the design
needs is not a new mechanism in notko: it is one more key in the tier file naming the posture on the other
carrier, and the reserved field read.

**And one shape in the extension path is worth op's eye because it is the shape he has refused three
times.** The tier file is resolved from `$CARGO_MANIFEST_DIR` (`discover.rs:93`), which is the
**consumer's** manifest directory. So making `#[profile(Precise)]` available means dropping the same
four-line file into every crate that uses it, or setting `$NOTKO_OPTIMISERS_PATH` in the environment. That
is a per-consumer enumeration of a set arvo already names, and `137b:29-42` records op refusing an
enumeration three times on the ground that the information is present and only the spelling is missing.
The information is present here too: arvo names its postures. I am not proposing the fix; I am naming that
the shape is the refused one, in op's own mechanism, where nobody had looked.

**What stays on each side, which is where `143` and I agree exactly.** Types, postures, contracts and
projections in arvo, and arvo builds nothing that reads an attribute. Attribute, name space, search path
and rewrite in notko. The interface is thin. `143` says the interface is "arvo publishes one alias set per
strategy and the attribute selects one"; I say the interface is narrower still and takes a different
shape, which is the next section.

## The hole: under `143`'s route the attribute does nothing for op's tier two

This is the part I went down, and it starts from something `143` gets right.

A scoped mechanism cannot reach through a domain alias. `143` proves it from the type side and I proved it
independently from the resolution side before reading `143`: `p02` case D shows an injected inner `use`
retargeting a body's own spellings while a module-scope alias defined over the same name keeps its
original posture, with no special handling and no way to intervene. `143` is also right about why the
limit is not negotiable. If a scoped mechanism could make `StrHandle` mean one type inside a function and
another outside it, the two occurrences denote different types and every function boundary breaks.

Now put that next to what op says tier two is (`142c:231-233`):

> Then for frameworks and apps that want to wholesale take arvo, they'll write their own domain aliases
> like StrHandle = UInt<5> [...] and all the code just reads non-verbose and ergonomic StrHandle. No
> generics, nothing.

Every type in a tier-two body is a module-scope alias. So under `143`'s preferred route R3, and under R1,
and under R3b, `#[profile(Hot)]` over a tier-two function retargets **nothing at all**. R2 reaches bare
primitives, which tier two does not write either. `143` records this under the precedence rule as a limit
that falls out for free, and under tier agreement as correct behaviour, and both statements are true. What
neither says is that the tier the mechanism cannot reach is the tier op describes as wholesale adoption.

I want to be plain about the size of this rather than diplomatic. A profile mechanism that is inert over
the code path most arvo consumers will write is not a mechanism with a limit. It is a mechanism aimed at
tier one, which by op's own description is the consumer who never adopts arvo at all, and at tier three,
which by `142c:271-276` is written once per alias definition. The two tiers it reaches are the two that
write the fewest lines.

### Why every type-rewriting route hits the same wall

The routes `143` enumerates all act on the **spelling** of a type. That is the shared premise, and the
wall is a consequence of it rather than of any particular route. A spelling can be rewritten only where
the macro can see it, and a domain alias is precisely the construct whose purpose is that the spelling is
not there. Tier two's ergonomics and the macro's reach are the same property with opposite signs.

So the attack has to leave the premise. Two things can be retargeted at a use site: the type of the value,
or the operation performed on it. Every route on the table takes the first. I took the second.

### The attack: make the elision a marker, resolve at the operation

Two pieces, and neither is novel on its own. Together they clear the wall.

**The elision is a marker.** A posture-neutral spelling is not a spelling with a parameter missing; it is
a spelling whose parameter is the marker meaning "no declaration was made here". So `UInt<5>` is
`Num<5, Ambient>`, one type, everywhere, and `type StrHandle = UInt<5>` carries the elision into the alias
rather than resolving it at the definition site. Tier two's one-name-one-thing property is preserved by
construction, because nothing ever changes what `StrHandle` denotes.

**The resolution happens at the operation.** The annotation does not touch types. It rewrites the
operations in its region to name the posture, exactly as notko's rewrite already names the fallibility
tier, and the operation resolves the operand's declared posture against the region's by a type-level
function. `Ambient` yields to the region. Anything declared wins.

`p08` builds it and runs it, and four properties hold at once:

The alias keeps one identity. `core::any::type_name::<StrHandle>()` is `Num<5, Ambient>` in every region,
so no value crossing any boundary meets an `E0308`, which is the failure `143`'s `p6` and my `p02` case D
both establish for the type-rewriting routes.

The region's posture reaches through the alias. `hot_scope` and `plain` take and return the same
`StrHandle` and perform different arithmetic.

An explicit declaration beats the enclosing region, at the type level rather than by a macro heuristic. A
`Checked = Num<5, Precise>` operand inside a `Hot` region still refuses on overflow, and the probe prints
`true` for that.

And the result shape follows the resolved posture, so a refusing declaration inside a fast region produces
a fallible result and the signature has to say so. That is the same rewrite notko already performs
(`rewrite/outcome.rs:40-42` sets the return type), which is the point: **one annotation drives the numeric
posture and the fallibility because under this shape they are one decision.**

### The laws, over the whole grid, with a control that fails

`p09` asserts the resolution function's laws as type equalities the compiler checks. Not a sample: the
elision against all three postures, declaration invariance over the full three by three, and idempotence.
Fourteen assertions compile.

The negative control is the fifteenth and it is the only error the file produces:

```
error[E0277]: the trait bound `Precise: Same<Hot>` is not satisfied
  --> p09_resolve_laws_and_control.rs:38:19
   |
38 |     assert_same::<<Precise as Resolve<Hot>>::Out, Hot>();
```

If that line compiled, precedence would not be implemented and the fourteen laws above it would be
vacuous. It does not compile, so they are not.

### The apparatus erases

`p10`: an ambient-posture region, a hot region, a declared operand under a hot region, and a bare `u32`
wrapping add, compiled at `-O`. The object file defines **one** body. Identical-code-folding emitted three
aliases:

```
_bare = _ambient_scope
_declared_under_hot = _ambient_scope
_hot_scope = _ambient_scope
```

So the elision marker, the resolution projection, the posture dispatch and the newtype are all gone, and
what reaches the machine is the bare add. That is the same evidence shape `135b:39-49` accepted for the
numeral, reproduced for the precedence apparatus specifically.

**And the scope check, because the fold above could be measuring the wrong thing.** If two postures
lowered identically, the aliasing would say nothing about the apparatus. `p11` gives two postures
different arithmetic and confirms they do not fold: `under_hot` is `add w0, w1, w0` and `under_cold` is
`adds w8, w0, w1` then `csinv w0, w8, wzr, lo`, with no aliasing between them. So `p10`'s fold is about
the apparatus and `p11` is why that reading is available.

Both are existence claims about structure, not measurements. Nothing here is priced, no instruction count
decides anything, and the harness has not run on any of it.

### What this route costs, stated as plainly as what it buys

**Every operation becomes a rewrite site.** The annotation's arvo half is a `VisitMut` arm over binary
expressions rather than one injected `use`. That is more macro than `143`'s R3 and it is the same kind of
work notko's rewriter already does for `Ok` and `Err`, so the cost is real and the shape is precedented.

**An operation entry point has to exist for every operation.** That is a surface, and I have not counted
it.

**The `Ambient` marker is a fifth marker in the posture position**, which is exactly the kind of thing
`143` correctly flagged as needing op rather than an expert. I would argue it is not a fifth **posture**:
it is the marker for the absence of one, it has no cell in any table, and it cannot survive an operation
since resolving is what operations do to it. But that is an argument, not a ruling, and D72's crate table
would gain a row either way.

**A region containing no operations gets nothing**, since there is nothing to rewrite. Storage-only code
under an annotation is unaffected. I think that is correct rather than a gap: storage layout is what the
alias declared and a caller's local posture has no business changing it. But it means the annotation
retargets arithmetic and not representation, which is narrower than "retargets the substrate" and should
be said in those words.

**And the two routes are not exclusive.** `143`'s scope selection reaches spellings written inside the
region, which this route leaves alone. Retargeting the operation reaches through aliases, which the scope
selection cannot. A design could take both, and if it does, the composition needs stating, because a
region that both retargets a spelling and retargets an operation over it applies the posture twice. Under
`p09`'s idempotence that is harmless, and idempotence is why I asserted it.

## The granularity ruling, and what it costs `143`

Op has ruled and the ruling is broader than either position the brief set up. All things change and act
granularly, a constant is a function that happens not to vary, and the existing tables are one arm with
further arms written separately and later. Nothing is owed retroactively.

**What that costs `143`.** Its imitation rule, at `143:119-121`:

> A cell is a constant unless the strategy's definition is an imitation, in which case the cell is
> whatever the imitated thing does, including where that varies.

That sentence is built on the dichotomy op just dissolved, and its clause "is a constant unless" makes
constancy the default and variation the exception. Under the ruling the polarity is the other way round.
So the rule does not survive as stated, and `143` had already marked it as the first thing it wanted op
to look at, which was the right call.

**What survives, and it is the useful half.** The observation underneath it is still true and still worth
having: `Warm`'s definition is a reference to something external and the other three definitions are not,
so `Warm`'s arms are inherited while the others' arms have to be written. That is a statement about where
the arms come from rather than about which cells are constant, and in that form it is compatible with the
ruling and predicts something. I would keep it in the canon in roughly this shape:

> A posture defined by imitation inherits the arms of the thing it imitates. A posture defined by a rule
> has the arms someone writes for it. Both vary; they differ in who supplies the variation.

**What it costs `142c`.** Its "general property of the axis" framing was right in outcome and reached
through a claim about the strategy axis specifically. The ruling is not about the strategy axis; it is
about everything. So `142c` was closer, and it was closer for a reason narrower than the ruling it
anticipated.

**And the one thing my own work implies about arms**, offered additively and not as a correction to
anything. The shipped `Hot` tier already has two arms and they are keyed on `debug_assertions` **and** a
cargo feature literally named `internal` (`rewrite/hot.rs:25,28`). That gate is expanded into the
consumer's crate, so `feature = "internal"` names a feature of whichever crate is being compiled, which I
confirmed rather than inferred: every build of my probe crate emits `unexpected_cfgs` for it, "originates
in the attribute macro `profile`". So `Hot`'s second arm is unreachable in any crate that does not itself
happen to declare a cargo feature by that name. If the arm vocabulary is going into the canon, the
question of **what an arm may be keyed on** wants an answer, because the one shipped instance is keyed on
something accidental. That is naming an arm and what would distinguish it, which is the additive shape,
rather than proposing an edit anywhere.

## Where I agree with `143`, and where I do not

Collected so a later reader does not have to reconstruct it from seven sections.

**Agreed, independently, and I would treat these as carrying two groundings each.** The attribute is
`#[profile]` and not `#[optimize_for]`. The shipped granularity is a `syn::ItemFn`. A bare `T: Add` bound
carries nothing about width, range, resolution or container, and the representation travels with the `T`
the consumer supplied. A total signature excludes a refusing posture. The bare-primitive bridge is
derived rather than defaulted. Explicit declaration beats an enclosing annotation. A scoped mechanism must
not reach through a domain alias, and the boundary between arvo and notko is the envelope sentence at
`124:3579`. An attribute may select a posture and must not resolve a cell.

**Disagreed on ground, same conclusion.** The join. `143` closes it on D72's crate-contents table
(`124:3602`); I close it on the ratified non-chain view lattice (`124:1386-1391`). Same disposition,
precedence or refusal. Mine does not evaporate if op ever adds a posture.

**Disagreed on scope of a claim.** The output generic. `143` says the mechanism has no spelling in Rust;
the correct law is that a type-parameter default fills where the trait is written and never where it is
inferred, so the mechanism is available exactly at op's tier-one shape and unavailable at a bare method
call. And `E0119` closes the associated-type route only, not the trait-parameter route.

**Disagreed on where a gap is located.** `143` reports "`Precise` has no tier" as a missing row. It is a
missing row, and it cannot be filled meaningfully, because a tier file's vocabulary is notko's carrier and
has no arvo coordinate. Locating it there says what a fix would have to be.

**Disagreed on what a limit means.** The alias reach limit is correct as a limit and is also the mechanism
missing op's tier two entirely, which `143` does not say.

**Overtaken by op's ruling.** The imitation rule, as discussed above. `143` flagged it first for op, which
was right.

## What neither of us checked

Five things, and the first two are defects in the shipped vehicle rather than open design questions.

**The `internal` feature gate resolves in the consumer's crate.** `rewrite/hot.rs:25,28` emits
`#[cfg(any(not(feature = "internal"), debug_assertions))]` into the annotated crate, so `Hot`'s release arm
never fires anywhere that does not itself declare a cargo feature called `internal`. Neither `143` nor I
would have found this by reading; it showed up as an `unexpected_cfgs` warning on every build of my probe
crate. If the arm vocabulary enters the canon, what an arm may be keyed on is a question, and this is the
only shipped answer.

**The attribute on a trait method fails late.** Not refused, rewritten, then `E0053`. That is the position
an arvo algorithm's operations would occupy.

**The notko macro suite never exercises the release arm, and never exercises the match rewrite at all.**
Eleven tests, all green, all with real assertions, none tautological, and I ran them. The gap is what is
absent. `notko-macros/tests/smoke.rs:3-9` states the release path is "covered by compile-only verification
that the cfg branches don't collide"; there is no such test in the file, and grep finds none. And
`HotRewriter::rewrite_match` (`hot.rs:176-195`) rewrites `match scrut { Ok(x) => body, Err(_) => _ }` into
`{ let x = scrut.unwrap(); body }`, **discarding the Err arm**, which is the most destructive rewrite in
the crate and has no test of any kind. A green suite over a rewriter whose most dangerous arm is untested
is not evidence about that arm. I am not dispatched on notko and I propose no edit there; I report it
because a standing rule says to report what I notice.

**Nothing here is priced.** Not one number in this file or in `143` came from `mock/benches/`. The
operation-retarget route adds a rewrite site per operation and an entry point per operation, and I have
counted neither. `p05`, `p10` and `p11` are existence claims about structure read off emitted assembly,
which is an ad-hoc spike and cannot decide a fork. If the choice between the two families ever turns on
how much, the harness has to run.

**And the composition of the two families is unexamined by both of us.** `143` proposes selecting a
posture for a region by importing an alias set; I propose resolving a posture at each operation. They are
not exclusive, they reach disjoint cases, and a design taking both would apply a posture twice in the
overlap. `p09`'s idempotence says that is harmless for the resolution function, and nobody has checked it
for the alias-set route.

## The alternatives I did not take

Recorded so the next member starts from a list. Each is closed by something specific.

**Rewriting the alias's definition rather than its uses.** The macro would have to see the alias, which is
in another item and frequently another file. Closed on reach, and it would break tier two anyway by making
one name mean two types.

**A blanket `Posture` impl with a specific override, so any type could be retargeted.** Closed by
coherence: the blanket and the specific impl overlap and the only route past that is `specialization`,
which the workspace forbids outright (`unstable-features.md`, forbidden table).

**Making the elided posture a defaulted type parameter on the alias**, so `type StrHandle = Num<5>` picks
up a scope-supplied default. Closed by `p06b` and `p06c` together: a default resolves at the definition
site and never at the use site, so the alias would freeze whatever was ambient where it was written, which
is the current behaviour with extra machinery.

**A join at disagreement.** Closed on the view lattice, above.

**Refusal at disagreement.** Sound and loud, and it makes the annotation unusable over any body that pins
one value. Kept as the fallback if the report cannot be implemented as a lint, which is `143`'s
disposition and I agree with it.

**Overriding the output through a second impl selected by inference.** Closed by `p06c` at `E0283` and by
`143`'s `p4`. The override has to be named, at the bound or at the value.

**Putting the override on the result as a postfix** rather than on the call or the type. Not closed. I
rank it first on the ergonomic bar and I did not take it because `143`'s projection route solves a
slightly different problem and op should see both.

## What is op's

Six, and I would order them by what blocks the most.

**Whether a profile names a posture or a strategy.** Question four turns on it and so does the shape of
any fix to question seven. If it names a posture, each library maps the posture to its own carrier and the
map is per-library data. If it names a strategy, arvo's four are the vocabulary and notko is expressing
them badly with three.

**The `Cold` collision.** notko's `Cold` is fallible and arvo's `Cold` is not. One attribute value, two
carriers, two answers about whether the annotated function can fail. This needs a call rather than an
expert, and it is cheap to make now and expensive later, since both names are shipped.

**Whether the annotation may retarget operations.** That is the whole of my proposal and it is a larger
claim than the type-rewriting family, because it changes what an annotated region means: the arithmetic
inside it, rather than the types written inside it. It is what reaches tier two, and it is a real
semantic change to code an alias author did not write.

**Whether `Ambient` is admissible.** A marker for the absence of a declaration, sitting in the posture
parameter position. I argue it is not a fifth posture; op decides whether that argument holds, and D72's
table gains a row either way.

**Which arm the current tables are, and what an arm may be keyed on.** Op suggested debug-assertions time
as a question rather than an answer. The one shipped instance of a second arm is keyed on
`debug_assertions` and an accidentally-named cargo feature, which is unlikely to be what the canon wants.

**What "simplification and arvo erasure for the return value" means.** Two readings, two mechanisms,
neither settled, and `143` and I agree on the uncertainty.

## Probe index

All committed under `144_probes/`, with `run.sh` regenerating `output.txt` on the pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`. Every one is a spike: it checks one thing, its names and
arities are scaffolding rather than proposals, and it should be cited for what it established and never
for how it was written.

| Probe | Result | What it establishes |
|---|---|---|
| `p02_scope_shadowing.rs` | runs | an injected inner use retargets a body, not a signature, not a module-scope alias, not an explicit spelling |
| `p03_boundary_is_a_type_error.rs` | `E0308` | a retargeted value cannot silently cross the signature |
| `p04_op_level_retarget.rs` | runs | retargeting the operation with the result shape projected from the posture |
| `p05_posture_param_erases.rs` | ICF aliases | the posture parameter folds into the bare add |
| `p06a_fn_default_type_param.rs` | refused | a function type parameter takes no default, as a future-incompat lint |
| `p06b_output_via_mode_marker.rs` | runs | the output generic spelled two working ways through a written bound |
| `p06c_method_call_ambiguity.rs` | `E0283` | the output-as-parameter route is inert at a method call |
| `p06d_mode_method_call.rs` | `E0284`+`E0283` | so is the mode-marker route |
| `p07_notko_granularity/` | mixed | impl and mod refused, trait method fails at `E0053`, `Precise` resolves once a tier file exists |
| `p08_ambient_marker_reaches_tier_two.rs` | runs | alias identity, reach through the alias, declaration wins, shape follows the resolution |
| `p09_resolve_laws_and_control.rs` | 14 pass, control fails | the precedence laws over the whole grid, with a control that must fail and does |
| `p10_resolve_erases.rs` | ICF aliases | the precedence apparatus folds into the bare add |
| `p11_postures_do_not_fold.rs` | distinct bodies | `p10`'s fold is about the apparatus, not about the postures |

Seven carry a compile failure or an intentional refusal, which is the point: a contract with no
expressible form says more than one returning a wrong value. `p09` is the one that carries both colours,
and its red half is the control.

**Independence, stated so it can be checked rather than asserted.** That a type-parameter default never
reaches an inference variable now rests on five instances across two files and two mechanisms
(`p06c`, `p06d`, and `143`'s `p4`, `p5`, `p9`). That a scoped mechanism cannot reach a module-scope alias
rests on three (`p02` case D from the resolution side, `143`'s `p6` from the type side, and `124:3579`
from the contract side). The operation-retarget proposal rests on `p08`, `p09`, `p10` and `p11`, which are
four probes but **one author and one model**, so by `140b:112-116` they are closer to one instance wearing
four hats than to four. It wants a second expert's independent construction before anything is built on
it.

Happy hacking.
