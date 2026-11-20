# Is a contract over a value type the format crate's subject

Gate. The canon gate passed: the question presumes only that `arvo-format` declares the signature, which
is what `mock/crates/arvo-format/DESIGN.md.tmpl:31` says and what `ruling::the_format_spine_is_canon`
(rung `ratified`) gives the format topic as its spine, so nothing in it
is forbidden. The suite gate needed no run: this reading relies on no test. The checks were made against
the registry declared in `mockspace.toml:31`, the two rule templates under `/home/user/.shared/rules/`,
and, for what the crate is as written, `arvo-format`'s design and source. This is the first read of the
question against the canon. A second independent read is owed before anything is placed on it, and
agreement with `SHAPE.md:44-48`, which reached a similar conclusion with no row or rule named, is not
corroboration of this one.

## The question

`arvo-format` declares a format paired with an adaptation, the representable set, the rounding modes,
the overflow policies, and the exact operations over slots composed with an adaptation. The contracts
`SHAPE.md:66-76` lists (sum, difference, product, order, identities, sign, magnitude, construction from
a constant, a monotone-sum marker) are traits a value type implements, and their surface reaches
`Self`, `core::cmp::Ordering`, and `arvo_format::{Bool, Width}` (`mock/crates/arvo-format/src/lib.rs:143`,
defined at `mock/crates/arvo-format/src/width.rs:35` and `:102`).

The question is whether such a contract is within that crate's domain: whether it is the format crate's
own thing. It is not the question of where the contracts are placed, and it is not answered by what any
placement would cost. Those two questions come apart in this case, and the reading below has to keep
them apart, because one of the three sources answers only the second.

## What the crate is, as written and as canon

As written, the crate is "The declared signature, and nothing else" (`DESIGN.md.tmpl:31`), and it declares
as its canon the format spine and its four propositions (`DESIGN.md.tmpl:3-5`). Its own contracts are
contracts over formats and adaptations, not over values: `Format`, `Ambient`, `Quantum`, `Slots`,
`Rounding`, `Overflow`, `Adaptation`, `DeclaredSignature`, `Operation` (`DESIGN.md.tmpl:334-342`). Its
addition takes and returns slots (`mock/crates/arvo-format/src/addition.rs:284`), and the design says in
so many words that a value type is not here: a value type at sole occupancy is what `arvo-numeral`
carries, "so this crate's addition still takes and returns slots, and a value's sum is this sum read
through that arm" (`DESIGN.md.tmpl:244-250`). `Bool` and `Width` are in the crate for a reason that is
not about values: it introduces the numeric category, the one door for the stack's own primitives
(`DESIGN.md.tmpl:395-397`, `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`).

The design is a claim, so the domain has to be read from the canon rather than from it. The canon on the
format topic says:

- `ruling::the_format_spine_is_canon` (ratified, both): a format is identified by its ambient domain and
  its representable set; membership is one affine predicate; arithmetic on a format is an exact
  operation composed with a named total adaptation, the adaptation first-class with its own laws; the
  concept is closed and the inventory open. Under it,
  `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set` adds that
  adaptation choice and encoding are realisation and not part of identity.
- `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`
  (ratified, experts, topic `the_format`): compositions over formats, stored-pair
  rationals, intervals, error-carrying values, are not format instances; they consume the format concept
  and owe their own laws. A concept whose operations have signature `V x V -> V` cannot state a
  composite claim.
- `ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`
  (ratified, experts, topic `the_primitive`): choosing a format fills in a record;
  composition takes an algebra to an algebra; a composite is a primitive under the same definition, so
  one concept serves and every contract written for a primitive applies to a composite unchanged.
- `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` (rung
  ratified): a datum of a format is one point of its ambient domain. Its gap leaves
  open, as the panel's, "the count and crate of value types" and that a type may hold a datum as a value
  at sole occupancy as a statement of its own.

No registry row names the crate `arvo-format` as the holder of any subject. The crate's subject is
therefore the format concept as the spine states it, carried to the crate by the design's own
declaration and by the brief's premise. That link is the one non-canon step in this reading, and it is
stated rather than hidden.

## The reading per source

### `arvo-bridge-home-rule`

The rule is a locus rule. Its sentence is "Substrate-bridge traits live in the lowest layer where their
return type is reachable" (`arvo-bridge-home-rule.md.tmpl:10`), decided by finding "the lowest crate in
the substrate dependency graph that defines or reaches all those types" (`:15-16`). It names no subject
test, and its "What this rule is NOT" section sharpens the criterion to reach rather than widening it
to anything else (`:60-64`).

Applied: `Bool` and `Width` are defined in `arvo-format` (`width.rs:35`, `width.rs:102`). `arvo-format`
depends only on `notko` (`mock/crates/arvo-format/Cargo.toml:9-10`), which does not define either, and
every other arvo crate reaching them depends on `arvo-format` (`arvo-digest`, `arvo-numeral`,
`arvo-placement`, `arvo-strategy`, each at `Cargo.toml:10`). So for every contract whose surface names
`Bool` or `Width`, the rule's literal output is `arvo-format`. `SHAPE.md:48-49` reads the rule as asking
only reach and concludes that "one crate directly above `arvo-format` reaches the same types". That
misstates the rule: the rule asks for the lowest such crate, a crate above is by construction not it,
and SHAPE.md departs from the rule's output while presenting itself as applying it. The departure may be
licensed by the second source; it is not licensed by this one.

Two qualifications on the rule's reach, neither of which reverses its pull. Its scope is const traits
mirroring a stdlib non-const-callable trait (`:28-32`), with `Identity` admitted in its own table
(`:39`) though it mirrors none; `Sum`, `Difference`, `Product` and `Order` fit that description under
arm three, while `Sign`, `Magnitude`, `Constant` and `MonotoneSum` mirror nothing and sit in the rule's
scope only by the same extension `Identity` got. And its concrete placement table (`:36-48`) is stale:
it puts `Bool` in `arvo-storage`, which is not a crate in `mock/crates/`, and it places `ConstDefault`
by "organisational consistency" (`:45`) while its own `:65` refuses consistency as a criterion. The
general sentence at `:10-16` survives the stale table; the table cannot be cited for anything.

Which way it pulls on the question asked: it does not answer it. It says where a trait goes by the
location of its types and is silent on whether the trait is that crate's subject. Its pull is on
placement, toward `arvo-format`, and so indirectly toward yes: if its output is obeyed and the second
source is also obeyed, the contracts would have to be the format crate's own thing.

### `a-crate-holds-only-its-own-thing`

This is the source that asks the question. "Each crate or module holds only the thing it is for"
(`a-crate-holds-only-its-own-thing.md.tmpl:8`), and "Ask it of every addition, before it is written:
what is this for, and which crate is for that?" (`:12`). Its intent paragraph closes the door the first
source opens: "a crate's contents are decided by what it is for, never by where the need arose"
(`:26`). The tell it names is a crate gaining an item about some other domain "because the place that
needed it was here" (`:10`).

What a value contract is for: a type whose inhabitants are values, closed under `Self x Self -> Self`
for sum, difference and product and under a total order. What the format crate is for, by the canon
above: a representable set, its membership predicate, and arithmetic factored as an exact operation
composed with a first-class adaptation (`ruling::the_format_spine_is_canon`). Three ratified rows make
the two different things rather than two views of one:

1. Every contract written for a primitive applies to a composite unchanged
   (`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`), and composites
   are not format instances; they consume the format concept
   (`ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`). So the
   implementor set of a sum or order contract, by canon, contains things that are not formats and that
   sit above the format concept. A contract whose subject strictly contains things outside the format
   concept is not the format concept's subject.
2. The same configuration ruling places the format as a field of a record ("Choosing a format, a system
   or a strategy fills in a record") and the contract on the algebra the record configures. The contract
   is about the algebra; the format is one coordinate of it.
3. The contracts' operations have exactly the signature `V x V -> V` that the format ruling says the
   format concept, closed that way, cannot state composite claims with; the spine keeps the format's
   arithmetic in the factored form, which is the form `arvo-format` ships (`addition.rs:284`) and the
   form the contracts' law reads through as its oracle (`SHAPE.md:61-64`). The value contract is the
   fused reading of that arithmetic through a value, which the design itself assigns away from this
   crate (`DESIGN.md.tmpl:244-250`).

What pulls the contracts toward `arvo-format` is that `Bool` and `Width` are there. By this rule that is
exactly "where the need arose" and is refused as a reason (`:26`). The rule's remedy, giving the owner
what it needs or making an owner where none exists (`:10`), is placement work and outside the question.

Which way it pulls: no, firmly.

### `obligation::the_surface_expressible_as_contracts_before_anything_implements_it`

The obligation asks that every part of arvo's surface be stated as
a trait contract that holds and is checkable before an implementation exists, so a consumer builds
against the shape while the implementation is unfinished. It rests on
`ruling::the_trait_contract_structure_is_a_primary_paradigm` (ratified, op). Its own
gap says "Nothing in the tree enforces where traits may live", and nothing in its `need`, `why` or `gap`
assigns a contract to a crate by subject.

It bears on the question in one way only. It applies to every part of the surface, `arvo-format`'s
included, and `arvo-format` already meets it for its own part with contracts over formats and
adaptations (`DESIGN.md.tmpl:334-342`). That confirms the crate has contracts of its own subject; it does
not say whether a value contract is among them. The separation it asks for, shape from implementation,
is satisfied by a contract in any crate the value's implementation does not live in, and `arvo-format`
is not where a value type is implemented (`DESIGN.md.tmpl:244-247`), so the obligation is met either way.

Which way it pulls: neither. It is neutral on the domain question.

## Do the sources rank

On the question asked, the domain question, the sources do not conflict. Only
`a-crate-holds-only-its-own-thing` has a subject criterion, and it answers no; the bridge-home rule has
no subject criterion and does not answer; the obligation does not answer.

They do conflict one step later, on placement, and that conflict is reported because the brief asks
whether anything ranks them. The bridge-home rule puts any trait reaching `Bool` or `Width` in
`arvo-format`; the crate rule refuses a thing that is not the crate's own. Both rules are `kind: reflex`
in their frontmatter (`arvo-bridge-home-rule.md.tmpl:4`, `a-crate-holds-only-its-own-thing.md.tmpl:4`),
neither carries a tier, and `homma rules` exposes no ranking. The bridge-home rule's only ranking
sentence ranks it over a doc CL's placement when SRC application finds a layering conflict, not over
another rule. The crate rule's `:26` refuses "where the need arose" as a reason, which is a statement
against reach-driven placement in general, but it does not name the bridge-home rule and cannot be read
as superseding it without a call. No registry row ranks them: the registry governs arvo's canon and the
rules are workspace rules, and no ruling found under `cargo mock query ruling` speaks of trait placement
at all. That placement conflict is not this reading's to resolve and it is handed back as unsettled.

## The answer

No. A contract over a value type is not within the domain of the crate that declares the signature.

The ground is `a-crate-holds-only-its-own-thing` read against three ratified rows: the format concept's
subject is a set, its membership and factored arithmetic (`ruling::the_format_spine_is_canon`); every
contract written for a primitive applies to a composite unchanged
(`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`); and composites are
not format instances but consume the format concept
(`ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`). A value contract's
implementor set therefore reaches past the format concept, so it is about something the format crate is
not for. The bridge-home rule does not bear on domain and pulls only on placement, toward `arvo-format`;
the obligation is neutral. The one step not resting on a row is the identification of the crate's domain
with the format concept, which the design declares (`DESIGN.md.tmpl:3-5`, `:31`) and the brief supplies
as a premise. If that identification is rejected, the answer becomes undecidable from these sources, since
no registry row names `arvo-format`'s subject, and the open "count and crate of value types"
(the gap of `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point`) does not supply one.

Two things travel with the answer and are not answered by it. The placement conflict between the two
rules is unranked by anything in the sources or the registry and is the dispatcher's, or the human's, to
call. And `SHAPE.md:48-49` misreads the bridge-home rule as a bare reach rule; its conclusion on domain
matches this one, but it reached it by asserting the crate's subject without naming a rule or row, and
its departure from the bridge-home rule's output is licensed, if at all, only by the crate rule it does
not cite.
