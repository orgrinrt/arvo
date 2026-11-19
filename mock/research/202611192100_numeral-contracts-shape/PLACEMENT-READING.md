# Where the numeral contracts may live: a reading of the placement

This is one seat's reading of the section "Where the contracts live" in `SHAPE.md` (lines 40 to 57).
It is argument, not canon. The canon it is read against is the registry at `mock/registry/*.toml`,
and the workspace rules that bind arvo: `arvo-bridge-home-rule`, `a-crate-holds-only-its-own-thing`,
and the mutation order in `.claude/rules/canon-design-code-chain.md`. The shipped crates are claims
tested here, not context. This is the first read; a second independent read from the canon text is
owed before anyone treats a candidate as permitted.

Canon gate. The question as posed does not rest on a premise the canon forbids. It does rest on one
the canon does not supply: that a contracts crate is the shape the canon asks for. The ratified
paradigm row says contracts are primary and shapes stick before implementations
(`ruling::the_trait_contract_structure_is_a_primary_paradigm`, mock/registry/ruling.toml:1276 to
1282), and its own note records that the registry once carried three contracts crates and a lint
for where traits live, that both are gone, that "no contracts crate is in the tree", and that
"nothing in the tree enforces this row" (ruling.toml:1288). The obligation behind it says the same
about placement: "Nothing in the tree enforces where traits may live"
(`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`,
mock/registry/obligation.toml:162). So no registry row says where a contract lives, and none
requires a crate of its own. The placement is governed by rules, not by a ruling, and the rules
pull in two directions. That is the finding the rest of this file works out.

Suite gate. I relied on no test. Every claim below is about registry text, rule text, a design
sentence or a source line. No suite was run.

## What the surface reaches, checked in the tree

SHAPE.md:42 to 43 says the surface reaches `Self`, `core::cmp::Ordering`, and `Bool` and `Width`
from `arvo-format`. Checked. `Width` and `Bool` are declared in mock/crates/arvo-format/src/width.rs:35
and :102 and re-exported at mock/crates/arvo-format/src/lib.rs:143. `arvo-format` depends on notko
alone (mock/crates/arvo-format/Cargo.toml:9 to 10). `Ordering` is `core`, and `Num::cmp` already
returns it (mock/crates/arvo-numeral/src/order.rs:22). The surface table (SHAPE.md:65 to 80) splits
three ways: contracts returning `Bool` or `Width` (`Order`'s provided methods, `Sign`, `Breadth`,
`BitAccess`), contracts returning `Ordering` (`Order::order`), and contracts that reach only `Self`
(`Sum`, `Difference`, `Product`, `Magnitude`, `Constant`, `Identity`, `BitLogic`, `BitSequence`).

## The rule, and what it does and does not cover

`arvo-bridge-home-rule` (/home/user/.shared/rules/arvo-bridge-home-rule.md.tmpl) reads: "Substrate-bridge
traits live in the lowest layer where their return type is reachable", with the steps: name the
signature return type and any other types in the surface, find the lowest crate that defines or
reaches all of them, and "The trait declaration lives in that crate". Step four puts bare-primitive
impls in the trait-declaring crate and "per-substrate-type impls in the type-declaring crate".

Three limits on its reach, each from its own text.

First, it fires "before authoring a new const-trait bridge", and defines a bridge as "a const-trait
mirroring a stdlib non-const-callable trait". `Order` mirrors `Ord`, and `Sum`, `Difference`,
`Product` mirror `Add`, `Sub`, `Mul`, and SHAPE.md:36 to 39 takes arm three, const traits. `Sign`,
`Magnitude`, `Breadth`, `BitAccess`, `Constant` mirror nothing in std. The rule is silent on whether
it governs them; its decision test generalises, and I read it by analogy for them, marked as such below.

Second, its concrete table is a snapshot of round 202605021800 and names crates that are not in
the tree (`arvo-storage`; and `arvo-strategy` as the home of `Bounded`, while the shipped
`arvo-strategy` is "A name bound to a placement objective and an adaptation selection",
mock/crates/arvo-strategy/Cargo.toml:6). The table is stale as a map. Its logic survives: the
home of `Bool`-returning bridges is the crate that holds `Bool`, which today is `arvo-format`.

Third, its "What this rule is NOT" section says the split between the `Self`-only typed-const
family and the `Bool`-returning family "is intentional and reflects the dependency structure". It
also says the rule is not "put every bridge in the same crate for consistency". SHAPE.md's one
crate for the whole table is a same-crate-for-consistency placement, which the rule disclaims as a
reason, though it is not forbidden as an outcome.

The rule's only stated ground for deviating from its result is a layering conflict, an inversion:
"When a doc CL specifies trait placement and SRC application finds a layering conflict, the
placement is wrong" and "If the latter, you have an inversion; relocate". Neither stated ground is
a taste about crate subjects.

## Candidate one: in `arvo-format`

Bridge-home. This is the rule's answer for every contract that returns `Bool`, `Width` or
`Ordering`, since `arvo-format` is where `Bool` and `Width` are declared and nothing lower reaches
them. For the `Self`-only contracts the rule does not decide: any crate reaches `Self`, and the rule's
own table puts that family apart from the `Bool` family. So the rule requires `arvo-format` for the
`Bool`/`Width` contracts and permits it for the rest. This is the placement SHAPE.md:43 itself
reaches and then declines.

Mutation order. `arvo-format` has a design, `mock/crates/arvo-format/DESIGN.md.tmpl`, which declares
its canon (lines 3 to 15). A contract section is a change to that design, and the rule in
`.claude/rules/canon-design-code-chain.md` is "Changing a design: nuke the code under it first,
never migrate". That is the format crate's code, including the oracle sweep, and the layers that
depend on it are rebuilt above. This is the most expensive candidate. Cost is not a ground the
canon gives for or against; it is only what it is.

Obligations. `obligation::the_surface_expressible_as_contracts_before_anything_implements_it`
(obligation.toml:158 to 165) asks that "a consumer can build against the shape while the
implementation behind it is unfinished". `arvo-format` meets that as well as any crate: its design
says "The carrier is not here and cannot be reached from here, because this crate does not depend on
the crate that has one" (DESIGN.md.tmpl:31 to 34), so a consumer of contracts in `arvo-format` depends
on no carrier, no placement and no rung. SHAPE.md:55 to 57 claims a separate crate is what gives this
and `arvo-numeral` or a trait inside it does not. The second half is true; the first is false as a
claim about uniqueness, since `arvo-format` gives it equally.

The Bits impls. This is where candidate one has a real cost, and SHAPE.md does not say it. Bridge-home
step four puts `Bits<N>` impls in `arvo-bits`, the type-declaring crate. `arvo-bits` "depends on nothing
in this workspace" (mock/crates/arvo-bits/DESIGN.md.tmpl:36), so it cannot name a trait in
`arvo-format` without a design change to `arvo-bits`. The other way, `arvo-format` depending on
`arvo-bits` to hold the impls, breaks the format design's own floor, that the carrier "cannot be
reached from here" (DESIGN.md.tmpl:33 to 34), since `Bits<N>` is an exact-width container. So the bit
tier with an `arvo-format` home needs an `arvo-bits` design edit and a new edge from `arvo-bits` to
`arvo-format`. That edit is a design call and not a canon one: no registry row fixes `arvo-bits`
as dependency-free; the sentence is design tier.

Permitted by the canon? Silent on the crate, permitted by the rule for the `Bool`/`Width` contracts,
costly by the mutation order. The second rule below is the only thing that pushes against it.

## Candidate two: a new crate directly above `arvo-format` and below `arvo-numeral`

Bridge-home. This is not the lowest crate that reaches the types. It is one layer above. The rule's
stated ground for leaving the lowest crate is inversion, and there is none: `arvo-format` depends
on notko only (Cargo.toml:9 to 10), `arvo-numeral` already depends on `arvo-format`
(mock/crates/arvo-numeral/Cargo.toml:10), and nothing forces the contracts upward. So by the rule's
text this is a deviation without the rule's own ground. SHAPE.md:44 to 49 supplies a different ground,
the one the rule does not name: putting the contracts in the format crate "would give the format crate
a second subject".

That ground has a home in the rules, though SHAPE.md does not cite it.
`a-crate-holds-only-its-own-thing` (/home/user/.claude/.rules-b/a-crate-holds-only-its-own-thing.md)
says: "Each crate or module holds only the thing it is for", the tell is "a crate gaining a function,
a type or a table that is about some other crate's domain", and the fix is "where no owner exists, to
make one". Whether it applies depends on whether a contract a consumer bounds on is `arvo-format`'s
domain. Two tree facts cut against SHAPE.md's premise.

One, SHAPE.md:46 to 47 says a contract is "a statement about operations over values rather than about
the set". But `arvo-format` already declares the operations: `add` (mock/crates/arvo-format/src/addition.rs:284),
`sub` (src/addition/subtraction.rs:110), `neg` (src/addition/subtraction.rs:76), `mul`
(src/multiplication.rs:259), and the canon that spine rests on is ratified:
`ruling::the_format_spine_is_canon` (ruling.toml:1479 to 1490) ratifies
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` among its four. The oracle the
contract laws are stated against is in the format crate, as SHAPE.md:23 to 25 itself says. Operations
are the format's subject by ratified canon. What the format crate does not hold is a value type: its
operations take `Slot`, not `Self`. A contract over `Self` is a statement about a value type the
crate cannot name, which is a narrower and truer premise than the one SHAPE.md wrote.

Two, `arvo-format` already declares several traits (`Rounding`, `Slots`, `Quantum`, `Ambient`, `Overflow`,
`Format`, `Adaptation`, `DeclaredSignature`, and others, for example mock/crates/arvo-format/src/adapt.rs:27
and :65), and its own design says it holds `Bool` and `Width` "because it introduces the numeric
category" (DESIGN.md.tmpl:395 to 397). Whether `Sum` and `Order` are another thing than the
signature and its operations is the judgement the rule leaves to the reader. The format design's
sentence "The declared signature, and nothing else" (DESIGN.md.tmpl:31) is design tier and is not
canon; the rule that would give it force is the one under reading.

So the second-subject ground is real but unproven, and it is a rule's ground (`a-crate-holds-only-its-own-thing`),
not a canon row's. The two in_force rules conflict on this candidate. I do not resolve it. See the
verdict.

Mutation order. A new crate has a new design; nothing declares it, so nothing is nuked by adding it
(canon-design-code-chain.md: "Adding a row nukes nothing", and a new design depends on existing canon
without invalidating anything). Its design must declare canon, which it cannot do for placement,
since no row decides it. `arvo-numeral` gets edited either way.

The `arvo-bits` edge. SHAPE.md:51 to 54 says the contracts crate depends on `arvo-bits` so that
`arvo-bits` stays dependency-free. This does keep `arvo-bits` unchanged, and the impl is lawful under
orphan rules because the trait is local. It deviates from bridge-home step four, which puts the
`Bits<N>` impls in `arvo-bits`. The ground for that deviation is the `arvo-bits` design sentence
(DESIGN.md.tmpl:36), a design-tier sentence, not a ruling, and not a rule. It is a ground only in the
sense that the design stands until changed.

Premise check on dependencies. `mock/Cargo.toml:61 to 66` says `arvo-bits` is "listed for the same
publishing reason the other three are, not because anything depends on it yet". That comment is stale:
`arvo-digest` depends on it (mock/crates/arvo-digest/Cargo.toml:12). It does not hurt the proposal;
it only means a contracts crate above `arvo-bits` is a second dependent, not the first.

Premise check on separability. SHAPE.md:55 to 57 says a separate crate is what lets a consumer depend
"on `arvo-contracts` alone and on no carrier". True of a new crate and equally of `arvo-format`, as shown.
It is a reason against candidate three, not for candidate two over candidate one.

Obligations. Same as candidate one, met. No obligation names a crate.

Permitted by the canon? The registry is silent: no ruling requires or forbids a contracts crate, and
the one that mentions contracts crates records their removal and says no argument stands on them
(ruling.toml:1288). Against bridge-home it is a deviation whose rule-stated ground (inversion) is
absent. For `a-crate-holds-only-its-own-thing` it is the fix of the named tell, if and only if the
contracts are not `arvo-format`'s domain.

## Candidate three: in `arvo-numeral`

Bridge-home. Forbidden by the rule's own decision test, step three: the home crate must not depend on
the location of the return types in a way that inverts. A contract bound by a consumer that must not
build a carrier, placement or rung (obligation.toml:159) cannot sit in the crate that holds them:
`arvo-numeral` depends on `arvo-format`, `arvo-placement` and notko (Cargo.toml:10 to 12). The traits
are reachable lower, so by the rule they are declared lower, and `arvo-numeral` holds the impls for
`Num`, as rule four says.

Obligations. The obligation's "before anything implements it" is defeated: a consumer would depend on
the implementing crate, which is the arrangement the obligation's need sentence is written to avoid.
This is SHAPE.md:55 to 57's one sound argument.

Existing state, which is a claim to test. `Identity<M>` is declared in `arvo-numeral`
(mock/crates/arvo-numeral/src/identity.rs:18) and reaches only `Self`. By the same reasoning it is out of
place there, and the rule's own table puts the `Self`-only family outside the value crate. SHAPE.md:73
to 80 reaches the same conclusion and also notes that moving it needs agreement from the lane that
owns `arvo-numeral`. That is a process call, not a canon call.

Mutation order. Editing `arvo-numeral`'s design nukes its code, a crate SHAPE.md:36 to 39 says another
lane is overhauling. This cost is shared by every candidate, since `arvo-numeral` holds the `Num` impls
wherever the traits live. Putting the traits there adds nothing beyond that.

Permitted by the canon? Forbidden by `arvo-bridge-home-rule` step three and by the obligation's own
sentence. No ground for the deviation appears in the registry or the rules.

## Candidates found elsewhere

A split along the rule's own seam. The rule's table separates the `Self`-only typed-const family from
the `Bool`-returning predicate family and calls the split intentional. Read literally, the `Bool`,
`Width` and `Ordering` contracts (`Order`, `Sign`, `Breadth`, `BitAccess`) go to `arvo-format`, and the
`Self`-only contracts (`Sum`, `Difference`, `Product`, `Magnitude`, `Constant`, `Identity`, `BitLogic`,
`BitSequence`) go to a crate that holds no `Bool`. But `MonotoneSum` is `Sum + Order` (SHAPE.md:76 to
77), so a supertrait edge pulls the `Self`-only family up to wherever `Order` lives or forces a downward
edge; and `Order`'s provided methods return `Bool`, so it is not a clean member of either family. The
split does not dissolve the fork, because the marker joins the two sides. I record it as considered
and not recommended; the canon is silent on it.

`arvo-bits` for the bit tier. `BitLogic` and `BitSequence` reach only `Self`, so `arvo-bits` is a
candidate home for them by the rule's logic. It would hold the impls and the traits together. This is
lowest, adds no edge and leaves the design sentence at `arvo-bits` DESIGN.md.tmpl:36 true. But
`arvo-bits`'s design says "no arithmetic" and "a bit container" (DESIGN.md.tmpl:13); whether the
contract surface for bits is the bit container's own thing is, again, a `a-crate-holds-only-its-own-thing`
judgement. `BitAccess` and `Breadth` return `Bool` and `Width`, which `arvo-bits` cannot reach, so
the tier cannot move whole. The canon is silent.

`arvo-placement` or `arvo-strategy`. Both sit above `arvo-format` and below `arvo-numeral` (Cargo.toml
files of each, `arvo-format.workspace = true`). Neither is for contracts
(`arvo-strategy`: "A name bound to a placement objective and an adaptation selection"). Forbidden by
`a-crate-holds-only-its-own-thing` as a place where the need happened to arise.

## The two rules, and why the call is not the builder's

`arvo-bridge-home-rule` and `a-crate-holds-only-its-own-thing` are both in_force workspace rules and
neither is a ratified ruling. Bridge-home puts the `Bool`/`Width`/`Ordering` contracts in `arvo-format`.
The own-thing rule is the only text that could carry a deviation upward to a new crate, and it carries
it only if the contracts are outside `arvo-format`'s domain. That question has evidence on both sides:
the format's ratified spine is arithmetic as an exact operation composed with an adaptation
(ruling.toml:1479 to 1490), and its operations are over `Slot`, not over a value type. The canon does
not settle whether a contract over a value type is the format's domain. The rule text does not say which
of the two rules outranks the other. That is an unsettled call, and under the canon gate it goes back
to the dispatcher and the lead designer, not to a second expert as a tiebreak. Two independent readings
that disagree go to the human; this is the first read, and a second one is owed.

## Verdict

Forbidden. In `arvo-numeral` (candidate three): `arvo-bridge-home-rule` step three, and the obligation
`obligation::the_surface_expressible_as_contracts_before_anything_implements_it` (obligation.toml:159).
In `arvo-placement` or `arvo-strategy`: `a-crate-holds-only-its-own-thing`, and neither is the lowest
reach. An `arvo-bits`-held bit tier for the `Bool`/`Width` members: `arvo-bits` cannot reach `Bool`.

Required by the rule, unless a ground is found: `arvo-format` for the `Bool`, `Width` and `Ordering`
contracts. The rule's only stated ground for deviation, an inversion, is absent from the tree.

Not decidable from the canon: a new crate above `arvo-format`. It deviates from bridge-home without the
rule's stated ground. Its ground, a second subject, is the unratified domain judgement of
`a-crate-holds-only-its-own-thing`, and SHAPE.md does not cite that rule. SHAPE.md's stated reasons
are partly false in the tree: that a separate crate is what gives a consumer the contracts without a
carrier (false, `arvo-format` gives the same by its design at DESIGN.md.tmpl:31 to 34), and that
operations over values are outside the format's subject (false as stated, `arvo-format` exports
`add`, `sub`, `neg`, `mul` and ratifies arithmetic in its spine; true only in the narrower sense that
those take `Slot`, not `Self`). Its reason that `arvo-bits` stays unchanged is true, and the ground for
it is a design sentence, not canon.

Silent in the canon: whether a contracts crate may exist at all; where `Self`-only contracts live; where
the `Bits<N>` impls live when the traits are not in `arvo-bits` (bridge-home step four says
`arvo-bits`, its design says it cannot); whether the rule governs non-mirror contracts such as `Sign`
and `Breadth`; which of the two rules outranks the other; and whether `Identity<M>` in
`arvo-numeral` (identity.rs:18) is itself a deviation.

The decisive artifact for the open call is one sentence from the lead designer: is a contract over a
value type within the domain of the crate that declares the signature those values satisfy. If yes,
`arvo-format`, with an `arvo-bits` design edit for the bit tier. If no, a new crate, with bridge-home
recorded as deviated from by that ruling.
