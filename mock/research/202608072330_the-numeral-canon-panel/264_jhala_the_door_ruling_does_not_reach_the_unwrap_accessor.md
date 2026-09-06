# 264. The door ruling does not reach the unwrap accessor, and the design owes it a citation anyway

Dispatched as a second, independent read of `feat/the-exact-width-container` against
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`, on
`review/the-numeric-door-second-read`, off `origin/dev`. A first reader had already formed a view
before I started, quoted to me in the dispatch itself; that is disclosed rather than hidden, and the
section below marked "before" is what I wrote from the ruling row alone, before I ran `git show` or
`git diff` against the branch for the first time. Everything after that marker was written with the
branch's actual source open.

## Canon gate

Aligned. The governing canon is `mock/registry/*.toml`, declared by `mockspace.toml`'s `canon_paths`.
The subject row, `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
is `rung = "ratified"`, `ratified_by = "experts"`, checked directly in the file rather than assumed.
`ruling::the_panel_finishes_the_canon_without_him` puts this track outside op's loop, so no
`AskUserQuestion` is owed here; the call is derived from the row's own text.

## Test gate

No suite is being changed by this dispatch, and nothing here is landed as code. The branch itself ships
`mock/crates/arvo-bits/src/tests.rs` and a `tests/compile_fail.rs` pair with two `trybuild` UI cases,
one at `N == 0` and one at `N == 65`. I read both bodies, and the design section they pin against, as
part of judging whether the crate's own claims about itself hold up; I did not run the suite, since
running `cargo mock` or `cargo test` against a branch I am not landing would touch build state this
worktree does not need to hold, and the question asked is about a ruling's scope, not about whether the
branch's suite passes.

## Before: my own account of the ruling, formed from the row alone

What it says, in the `says` field: none of four recorded options for typing the numeral
parameterisation's associated constants works. The leading one is refuted arithmetically, not by
preference (`Width` is a `u32` count of bits, three of ten constants need values it cannot hold, so no
design taste reopens it). The bound under dispute, that the door is two types and no more, is stated to
be a sentence in the crate's own design document and to appear nowhere in the canon. What is ratified
positively is one sentence: "What the door carries out is the coordinate set of the ratified
parameterisation, spelled in types the stack owns." How many types that coordinate set needs is
explicitly left open, "this ruling does not say," because the two blind seats disagree and neither the
canon nor a third reading settles it.

What it forbids, on this reading alone, is three things. First, treating the two-type bound as canon,
since it is named as living only in the crate's design and nowhere in the registry. Second, treating the
refuted first option, `Width` alone typing all ten constants, as viable. Third, treating seat 238's
position rule, its own words being "a bare primitive appear only in the constructor and accessor of a
primitive that crate itself defines, plus the const generic parameter position," as ratified, since the
`note` field says of it in terms that it "is recorded as a proposal rather than carried here."

What is silent: the row never says what a second primitive-introduction door, for a concept other than
the numeral, may or may not carry. Its whole evidentiary base, both the refutation and the positive
coordinate-set statement, is phrased around "the ratified parameterisation," a definite noun phrase that
only resolves to one thing in this canon, arvo-format's numeral coordinates. Whether that positive
statement generalises to any future door crate, or stays local to this one, is not decided by the text in
front of me.

My tentative read at this point, before opening anything else: the clause the branch would have to clear
is the third one above. If its own design cites the position rule, or a shape indistinguishable from it,
as though ratified, that citation is unsupported by this row, whatever the truth of the underlying claim.
Whether that makes the branch's crate wrong or only its sentence wrong is exactly the distinction the
brief asked me to hold onto, and at this point I did not yet know which one the branch's actual text
would turn out to need.

## After: reading the branch, and where the first reader's account does not survive it

`git diff origin/dev...origin/feat/the-exact-width-container --stat` names the touched files. The two
load-bearing ones are `mock/crates/arvo-bits/DESIGN.md.tmpl` and `mock/crates/arvo-bits/src/lib.rs`,
both read whole via `git show`, plus the diff to
`mock/lints/a_contract_coordinate_is_not_a_host_primitive.rs` and, crucially,
`mock/crates/arvo-format/DESIGN.md.tmpl` on `dev`, unmodified by this branch, since that is the document
whose parity the new one claims.

The first fact that reframes the question: arvo-format's own current design already answers what "the
coordinate set... spelled in types the stack owns" is about, and it is not accessors.
`mock/crates/arvo-format/DESIGN.md.tmpl:334-343` on the `dev` tip reads:

> `Width` is a count of bits and `Bool` is a truth value. They live in this crate because it introduces
> the numeric category... Both are `repr(transparent)` with a single declared unwrap accessor, `count`
> and `get`, so the observation surface is exactly what the type establishes and nothing widens it.
>
> They are not the bound on what the door carries. A count of types is not the bound at all.
>
> What the door carries out is the coordinate set of the parameterisation, spelled in types the stack
> owns. A format's coordinates are what an implementor writes, so a crate outside this one has to be
> able to write them, and every one of them is a number or a truth value that the bare-primitive lints
> refuse in every crate but this one.

This is the ruling's own language, carried into the design tier below it, and it draws the line I could
not draw from the registry row alone: the coordinate set is the ten associated constants on the
`Format`-family traits, the positions an outside implementor writes. `Width::count()` and `Bool::get()`
sit on the other side of that line by the design's own admission, one paragraph earlier: plain accessors
on the door's own primitives, returning `u32` and `bool` bare, uncriticised, called out by name as the
sanctioned shape. `mock/crates/arvo-format/src/width.rs:53` and `:113` confirm the signatures match what
the design describes.

The same document also names the position rule directly, and says what "not carried" means for it in
practice, at lines 360 to 363:

> There is a third shape the bound could take, where it stops being a count and becomes a rule about
> positions: a bare primitive appears only in the constructor and the accessor of a primitive this
> crate itself defines, plus the const generic parameter position that is already excepted. Named here
> as open rather than adopted, because this design is not where that bound gets chosen.

"Open rather than adopted" is a claim about a general, exhaustive bound statement replacing the two-type
count, a claim of the shape "nowhere else in this crate may a bare primitive appear." It is not a claim
that the constructor-and-accessor idiom itself is disallowed, and the same document uses that idiom,
unflagged, one section earlier, for `Width` and `Bool`. What is withheld is the universal "only," not the
instance.

Now `arvo-bits`. `Bits<const N: u32>(u64)` is `repr(transparent)`, with `masked(raw: u64) -> Self` as its
one constructor and `raw(self) -> u64` as its one declared unwrap accessor.
`mock/crates/arvo-bits/DESIGN.md.tmpl` names the shape directly: "`Bits::<N>::raw(self) -> u64` is the
unwrap door, the one place a host contract needs the value back, matching the pattern
`arvo_format::Width::count`, `arvo_format::slots::Slot::index` and `arvo_format::Bool::get` already
use." I checked that claim rather than took it: `mock/crates/arvo-format/src/slots.rs:59,96` and
`width.rs:53,113` are exactly those three accessors, and all three return a bare host type. The
structural parity is real, not asserted past what is true.

And `Bits<N>` has no trait surface at all. Its own design says so: "there is no outside implementor of
anything; `Bits<N>` is a concrete type; only its own constructors force the check." The whole factual
weight of the ruling, both the refutation of three of ten constants and the positive statement about the
coordinate set an outside implementor writes, is about a contract other crates satisfy. `arvo-bits` never
asks anything outside itself to supply a coordinate. There is no analogue in this crate to `BASE`, `MIN`,
`Quantum::SLOPE`, `Format::PHASE_DEN`. The ruling's subject matter does not have a foothold here
structurally, independent of whether its language is read narrowly or widely.

So: I disagree with the first reader. The clause being over-read is the note's sentence on seat 238's
proposal, that it "is recorded as a proposal rather than carried here." The first reader reads `masked`
as the constructor and `raw` as the accessor the proposal names, and concludes the branch adopted an
un-ratified bound. What the proposal actually withholds is the word "only," a claim that bare primitives
appear nowhere else in an introducing crate, not the constructor-and-accessor idiom on its own, which
arvo-format's own two primitives already use, unflagged, in the document the ruling's `note` names as
needing correction and which has since been corrected to say so in almost these words. The first
reader's own comparison, that arvo-format's door carries the coordinate set in stack-owned types while
this one carries a bare `u64` in and out, treats the coordinate set as the whole of what arvo-format's
door does. It is not: the door also has to build `Width` and `Bool` themselves, and it builds them by
exactly the constructor-and-accessor shape being called disqualifying here. The design's sentence, that
it "needs its own primitive door for the identical reason `arvo-format` has one," is not false. It is
checked, and it holds.

What does not hold up, and is a real finding rather than a wash: neither `mock/crates/arvo-bits/DESIGN.md.tmpl`
nor `mock/crates/arvo-bits/src/lib.rs` cites `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
anywhere, by slug or by paraphrase with attribution. The design's canon header names exactly two rows,
`obligation::an_exact_width_container_a_consumer_can_alias_and_pin` and
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`. The door ruling answers
`what_the_numeric_introduction_door_may_carry_out`, which is the precise question the design's own "Why
it is not built on `arvo-format`" section is answering when it asserts door parity. Per
`the-canon-design-code-chain.md`, every design document declares the canon it relates to, at file
granularity, because a design serving no canon has no reason to exist. A design that borrows a ruling's
language almost verbatim, compare "spelled in types the stack owns" against arvo-format's design, itself
downstream of the ruling, while never naming the ruling has made a canon claim through a proxy document
rather than through the registry, and `cite-the-canon-before-claiming-what-the-design-says.md` does not
carve out an exception for citing a design instead of the row it descends from. The substance survives
inspection; the citation discipline does not, and the fix is one line naming the row, not a rewrite.

One more thing worth recording, adjacent to the ruling but not itself the ruling. The lint's own prior
comment, still in `mock/lints/a_contract_coordinate_is_not_a_host_primitive.rs` on `dev`, states that
widening the exempt-crate list "is a design decision that would come with its own round, and a lint that
silently widened with the config would be one nobody noticed widening." The branch does not violate
this: it runs its own round, `mock/design_rounds/202609040615/`, keeps the hardcoded, non-config-driven
list (`THE_EXEMPT_CRATES: &[&str]`, not a read from `[primitive-introductions]`), and updates the comment
to say why a second name joined. That mechanism concern is met. It is a separate axis from the ruling
this dispatch was sent to check, and I record it only because a reader following this file to the branch
should not have to re-derive that it was met.

## Verdict

Disagree with the first reader. The ruling does not forbid the shape shipped on
`feat/the-exact-width-container`. The over-read clause is the note's sentence recording seat 238's
position-rule proposal as a proposal rather than as canon, taken as a prohibition on the
constructor-and-accessor idiom itself rather than on treating that idiom as the crate's exhaustive,
only-place-a-bare-primitive-may-appear bound. `Bits::masked`/`Bits::raw` structurally mirror
`Width::count`, `Slot::index` and `Bool::get`, which the ruling never touches and which arvo-format's own
corrected design names as the sanctioned unwrap-accessor shape for a door's own primitives, as distinct
from the contract coordinates the ruling is actually about, a distinction `arvo-bits` does not even have
a surface for, having no trait and no outside implementor.

Since I disagree that the ruling forbids the shape, the crate-versus-justification split the brief asked
me to resolve conditional on agreement does not arise. What I found instead, orthogonal to whether the
ruling forbids anything: the design's substantive parity claim holds up under inspection, but the design
cites no canon for the move it is making, which is a real and separate defect, fixable by adding the
citation rather than by touching the crate.
