# A cold second read of `question::what_a_datum_stands_for`

Read at `origin/dev` = `989e02f9` of `/home/user/arvo`, read only. Sections 1 and 2 were written
before `mock/registry/proposal-the-value-stance.toml` or any proposal file added since 12 November
was opened. Sections 3 to 5 were written after. The order is my word only: this file is in a
scratchpad and carries no commit, so the order is not checkable the way the registry's own
precedents make it checkable (see 5).

What I had read before writing section 2: `mockspace.toml` rung and standing descriptions;
`question.toml:93-106`; `ruling.toml` rows at 269, 473, 930, 949, 1052, 1392, 1417, 1436, 1459,
1497, 1531, 1609, 1622, 1713, 1734, 1750, 1786, 1804, 1824; the four spine rows in
`proposal.toml` (56, 72, 89, 583) which `ruling.toml:1441-1446` ratifies by id; the `need` lines of
`obligation.toml` 92, 113, 137, 147; `PRINCIPLES.md.tmpl:269-275`; `arvo-format/DESIGN.md.tmpl:228-247`.
One leak: a grep over `obligation.toml` printed the tail of a row whose provenance is
`mock::research/202611122300_value-stance-second-read::#kaskis-lane` (topic rounding, a fixed-point
lane for kaski). I saw its keywords and nothing of the first expert's argument.

## 1. The route by which the question closes

The question's own row names the decider: `decider = "panel"` (`question.toml:102`), and its note
says why: "the register names no decider for the question, so `decider` is the default"
(`question.toml:104`).

Who decides, and what convergence buys. `ruling.toml:1397`
(`two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`, ratified, by op):

> Where several experts converge heavily and can reason and spell it out, and the coordinator judges
> the reasoning, the evidence and the stated region sufficient, the proposition is promoted to canon.
> [...] The experts propose; the coordinator holds the gate; convergence is the bar rather than the
> trigger.

His words under it, `ruling.toml:1403-1405`: "if two separate experts agree on it heavily and can
reason and spell it out, that is a ratification for canon, though distinct from something I've
blessed."

That it is not a count, `ruling.toml:1412`: "It is not automatic promotion on a count of agreeing
experts: [...] the coordinator remains the gatekeeper, so the experts still only propose and
somebody has to judge that a proposition is justified, reasoned and demonstrated enough to deserve
canon. And it is not permanent".

That op is not in the loop, `ruling.toml:1719` (`the_panel_finishes_the_canon_without_him`,
ratified, by op): "Every remaining canon question is the panel's. [...] where his words genuinely do
not reach a question the answer is derived from the intent inside its spirit and put through two
independent agreements. Nothing is parked awaiting him." And `ruling.toml:1727`: "the experts
propose, the coordinator gates, and there is no third party to appeal to."

What counts as a second independent read. The schema: `mockspace.toml:1325`, standing
`two_experts` is "each deriving before reading the other", "over the region they share". The
ratified rows show the bar in use, in their `promotion` fields:

- `ruling.toml:1472`: "a non-Dolan second read was owed. Seat 225 is that read, on a third persona,
  and derived blind: its phase-one commit precedes the commit in which it read the prior work, so
  the ordering is checkable rather than asserted."
- `ruling.toml:1741`: "Two instances agree about the intersection of their claims and never the
  union".
- `ruling.toml:1757`: a seat that read the existing proposal first "on the bare headline sentence
  [...] is confirmation rather than a blind third instance", and what the coordinator gated on was
  the mechanism that seat derived without the row.
- `ruling.toml:1474`: a brief that names what it withholds signposts it, and that is recorded as a
  contamination, with the finding being what the signpost did not reach.

So the route: two reads on different personas, each written before the other is opened, agree over
a shared region; the agreement is the intersection only; the coordinator then judges reasoning,
evidence and stated region and writes a `ratified`, `ratified_by = "experts"` ruling carrying a
`promotion` field (`mockspace.toml`, the `promotion` field description: "Present on
`ratified_by = experts`"). The gate is the coordinator's. Neither expert closes anything.

One more ratified bound on the shape of an answer: the option list does not bind
(`ruling.toml:273`, stated rung only: "An option set put to him does not bound the panel"), and
two ratified rows already closed a question by an answer outside its list (`ruling.toml:1474`,
"closed by dissolution rather than by picking a letter"; `ruling.toml:1740`, "None of the four
recorded options answers the question").

## 2. My answer, from the ratified rows

The question row asks about "one datum of a numeral" (`question.toml:95`) and is filed under
`topic::the_primitive`. The brief asks about a datum of a format. These differ and the difference
decides part of the answer, see clause D.

### A. A datum of a format stands for one point of the format's ambient domain: the member of the representable set it is

Rests on `ruling.toml:1447`: "A format is identified by its ambient domain and its representable
set, and that set is a constant of the type. Membership in it is one affine predicate over one
parameterisation". The set is a set of ambient values and a thing either is a member or is not. The
format has nothing else for a datum to be: the identity has two components and the second is the
set of things the format can hold.

Rests also on the factoring, same line: "Arithmetic on a format is an exact operation in the
ambient domain composed with a named total adaptation onto that set". An exact operation in the
ambient domain takes ambient elements. If an operand stood for a set of ambient values the first
factor would have no argument to be exact on, and the adaptation "onto that set" would have no
single element to land on. The factoring is statable only if operand and result are each one
point. This is a derivation of mine, not a sentence in the row.

The law-tier rows speak the same way: `ruling.toml:1506`, "quantified over the values the format
can hold"; `ruling.toml:1791`, "`top + x = top` for every representable `x`".

### B. The top does not denote everything above it; what lies above it is the adaptation's fibre, and the adaptation is not part of what the format is

Rests on `proposal.toml:77`, ratified by id at `ruling.toml:1442`: "Adaptation choice and encoding
are realisation, observable in computed values and in pattern-level properties respectively, and
not part of identity." Its `because`, `proposal.toml:78`: "two's complement and offset binary
denote the same sixteen values".

The absorbing-top reading says the top stands for the set of ambient values a saturating
adaptation sends to it. That set is a function of the adaptation. One format, same ambient domain
and same representable set, under a wrapping adaptation has a different fibre at the same top
(`ruling.toml:1791`: "Under wrapping no element but the additive identity absorbs anything"). So
the reading makes what a datum stands for depend on a thing the ratified identity places outside
the format. A denotation that is the format's own cannot be that. The fibre is a true and useful
fact, and it belongs to the adaptation, which the spine calls "a first-class object with its own
laws" (`ruling.toml:1447`).

What the ratified rows do say about the top is stated as a law of an operation, with a region:
`ruling.toml:1791` (unsigned, absorption half only) and `ruling.toml:1506` ("whether a reduction's
induced operation is associative is decided by absorption"). That is the shape of the first
option's second half, a statement of where a fold is sound. The rows do not themselves contain the
words "rather than a denotation"; clause B is what supplies them, and clause B is my derivation
from the identity row.

Note on strength. The `note` of the ratified proposal row, `proposal.toml:80`, grades its own
halves: "The value-level half, that identity is denotation rather than encoding, is two experts
[...] the locus rider are converged by attack and concession". Clause B uses the adaptation half of
the `says`, which the ruling ratified whole. A reader may hold that the adaptation-is-realisation
sentence is among the weaker-derived parts; the row does not say which group it falls in.

### C. A set-valued datum is not a datum of a format; it is a datum of a composition over formats, which owes its own laws

Rests on `ruling.toml:1829`: "Compositions over formats, stored-pair rationals, intervals,
error-carrying values, are not format instances; they consume the format concept and owe their own
laws."

This places the fourth option. Intervals are not admitted into the format concept, so "a set,
admitted generally" is false of a datum of a format. The costs that option lists (the total order,
multiplicative associativity, the additive inverse) are then costs of one construction and not of
the numeral base.

### D. Whether a composite's datum may stand for a set is a per-construction matter, and nothing ratified forbids it

Rests on `ruling.toml:1809`: "a composite is a primitive under the same definition, so one concept
serves and every contract written for a primitive applies to a composite unchanged. A construction
carries two things of its own: a predicate on its base [...] which must be derived per
construction".

So over the question's own wording, a datum of a numeral or primitive, the answer is two arms and
not one: a format's datum is a point (A); a construction's datum is what the construction says it
is, an interval construction's being a set of base points, with its own laws owed. This is the
arms shape of `ruling.toml:1485` ("small arms and spans that hold where they are optimal and
nowhere else") and not a single policy for a category.

### E. A type may hold a datum as a value, and the ratified rows presuppose that one does at sole occupancy

Rests on `ruling.toml:1470`: "The footprint is nonetheless observable, through exactly one
observation the design does not own, the host language's layout observation on a sized type, and
that observation exists only where the value is the sole logical occupant of its allocation."
A layout observation on a sized type whose allocation "the value" solely occupies is a type holding
one datum as a value. The row states its footprint as observable, which a zero-sized marker cannot
satisfy with anything but zero.

Rests on `ruling.toml:1536`: "The container derivation is a placement rather than a semantics, and
its output count moves with occupancy: one output at sole occupancy, the carrier; three at shared,
the carrier, the access width and the stride." Sole occupancy has an output, the carrier. A value
type at sole occupancy is that arm of the one placement.

Rests on `ruling.toml:1756`, which draws the format and storage line by binding time and speaks of
"a sibling datum" that "survives to runtime": data exist at run time beside one another, and the
format is what stays constant across them.

Rests on the demand side, which carries no rung of its own but whose constraint is `in_force`
(`ruling.toml:935`: "public API positions using the stack's own primitives rather than bare
integers, floats, bool or usize"): `obligation.toml:148`, "A primitive of arvo's own for every
public API position across the stack where a bare integer, float, bool or platform-sized index
would otherwise sit". A position a `u32` sat in holds a run-time number. A marker holding no value
cannot take it.

No ratified row says the opposite. I found none that forbids a value-holding type.

### F. What such a type's behaviour is a function of, and what it may not claim

- Its operations are functions of the declared signature and never of the carrier:
  `ruling.toml:1470`, "every operation the design declares is a function of the declared width and
  never of the machine carrier"; `ruling.toml:1536`, "an operation is admitted exactly when it is a
  function of the declared signature".
- Its identity is the format's and not the carrier's: `ruling.toml:1470`, "Identity saturates at
  the declared signature set."
- What it stores is realisation: `proposal.toml:77`, "encoding [is] realisation, observable [...]
  in pattern-level properties". So a stored slot, a stored pattern, two's complement or offset
  binary are choices below the denotation, and two of them denote the same points.
- Its value set may not depend on a sibling datum: `proposal.toml:77`, "a value set that depends on
  other data is not a format but storage".
- Membership is not checked at run time on a release build: `ruling.toml:954`, "Never any runtime
  checks, ever: invalids are caught at compile time", with `ruling.toml:1058` permitting the debug
  panic "for what cannot be caught at compile time through the typestate".

### G. The two designs, tested against the rows

`PRINCIPLES.md.tmpl:271-274`: the formats are "each a
zero-sized marker holding no value; what value carries a format is
`question::what_a_datum_stands_for`, and it is open, so no numeric value type exists yet". As a
report of the tree this may be true. As a reading of the question it joins two things the rows
keep apart. What a datum stands for is a denotation question (A to D). Whether a type holds one is
a placement question that `ruling.toml:1470` and `:1536` already answer for sole occupancy (E). The
open question does not have to close for E to stand.

`arvo-format/DESIGN.md.tmpl:243-244`: "a numeral value type carrying a stored slot, which no row
licenses and which would be a second placement beside the one the derivation performs by
occupancy". Against the rows: "no row licenses" is right only in the narrow sense that no ratified
row has the sentence "a value type exists"; `ruling.toml:1470` presupposes one. "A second
placement" does not follow from `ruling.toml:1536`: the derivation has a sole-occupancy arm whose
one output is the carrier, and a value type whose representation is that carrier is that arm, not
a second one beside it. It would be a second placement only if the value type fixed a carrier by
some rule other than the derivation's. The design's own consequence, "so addition takes and
returns slots", I take no position on beyond F: the slot is realisation.

### What the ratified rows leave open, clause by clause

- A: nothing I can find, for a format. Open at the edge: whether "one datum of a numeral" in the
  question means a format's datum or any primitive's. I read it as the wider and answer with arms.
- B: the rows do not contain the sentence that the top is not a denotation. The second option's own
  open item, the necessary condition for soundness at the endpoint, is untouched; under B it
  becomes a question about the saturating adaptation's fibre. Signed top and bottom:
  `ruling.toml:1796`, "Unsigned only". Monotonicity: `ruling.toml:1791`, "stay at one expert".
- C: which topic the statability clause is filed under (`ruling.toml:1834`). The companion list
  "stays at one expert".
- D: whether arvo ships an interval or other set-valued construction at all, and what laws it owes.
  No ratified row admits one or refuses one. What a normalising construction's equality is:
  `ruling.toml:1814`.
- The third option, the constructor-level clause: no ratified row speaks to whether the denotation
  clause is a statement about the constructor. Under A every denotation is a singleton, so the
  partitioning against overlapping distinction it draws is trivially partitioning for a format and
  live only for a construction (D). I do not claim the rows decide the option's meta-claim.
- The sub-fork in the question's note, soundness against bestness of a derived numeral: no
  ratified row. The design names it open (`DESIGN.md.tmpl:234-235`).
- E: how many value-holding types and under what names (`ruling.toml:1740`: "How many types that
  is, this ruling does not say"). Whether the value type lives in `arvo-format` or elsewhere.
  Shared occupancy: `ruling.toml:1470`, "At a shared placement no per-element footprint exists to
  observe", so what a type holding a datum is at a packed placement is not given.
- F: which realisation is stored, slot or pattern; how a value is constructed without a run-time
  membership check, which is a typestate or const construction the rows require and do not design;
  wrap or clamp as a default (`ruling.toml:1038`, rung `open`).

## 3. The comparison with the first expert's rows

Opened after the above: `mock/registry/proposal-the-value-stance.toml`, added by `f2c88600`
(2026-11-13 02:50), the only registry file added since 12 November; the same commit added one
obligation row and `mock/research/202611122300_value-stance-second-read.md`, of which I read the
first thirty lines and a grep. The file holds 21 proposals, all `standing = "one_expert"`. Two bear
on the question. One thing I found after opening it and add to clause B as support, not as part of
the blind answer: `question.toml:524` records `adaptation_in_identity_or_realisation` as answered
"Realisation. Settled already by `ruling::the_format_spine_is_canon`".

### `a_datum_of_a_format_is_one_point_of_its_ambient_domain` (`proposal-the-value-stance.toml:9`)

Clause by clause against its `says` (line 15):

- "A datum of a format is one point of its ambient domain: a member of the format's representable
  set." Same as my A, from the same row (`ruling.toml:1447`).
- "A set-valued datum is a composition's datum and never a format's". Same as my C, from the same
  row (`ruling.toml:1829`).
- "the many exact results a saturating adaptation sends to one end of the set are a property of
  that adaptation, while the end itself stays one point." Same as my B, with a difference in the
  ground. The row grounds it on the spine placing saturation on the named adaptation and on the
  min-plus ruling stating absorption as a law. I ground it on the identity clause: the adaptation
  is realisation and "not part of identity" (`proposal.toml:77`, ratified at `ruling.toml:1442`),
  so a denotation that varied with it would not be the format's. The two grounds do not conflict
  and mine is the one that excludes the second option rather than only not asserting it. The
  ratified rows decide this difference in neither direction; both grounds are in ratified text.
- Its `note`: formats only, the third option not excluded, the soundness and bestness sub-fork
  untouched. Same as my open list, all three.
- Its `gap`: a composition's datum is the composition's to say. Same as my D.
- Its `because` says the result "is the question's first option in its own words". Same with a
  difference: the first option's second half, "rather than a denotation", is in no ratified row; it
  is derived, by both of us, by different routes.
- Its `answers = ["what_a_datum_stands_for"]`. A difference I state: the row answers the question
  for a format's datum and its own note says so, while the question asks about "one datum of a
  numeral" under `topic::the_primitive`, and `ruling.toml:1809` makes a composite "a primitive
  under the same definition". So the edge claims the whole question for a part answer. No ratified
  row decides whether a part answer may carry the edge; the precedent at `ruling.toml:1740` is a
  ruling that carries the edge and says in its `says` what it does not decide.

### `a_value_is_the_sole_occupancy_placement_of_a_declared_signature` (`:24`)

- That a sized type holding one value at sole occupancy is presupposed by `ruling.toml:1470`, and
  that it is the derivation's sole-occupancy arm and not a second placement beside it
  (`ruling.toml:1536`). Same as my E and my reading of the design sentence in G. The row adds the
  ratified fourth container proposal, "one primitive with two placements rather than two
  primitives" (`proposal-the-later-topics.toml:770`, ratified by id at `ruling.toml:1468`), which I
  had not opened. It strengthens E and I checked the quotation.
- "Every operation on it is a function of the declared width; the carrier is observable through
  the host's layout observation and through nothing the design declares." Same as my F, same row.
- The two obligations as demand. Same as my E, though I used only the second
  (`obligation.toml:148`).
- "one member's slot [...] in one canonical encoding". Not reached. My F says what is stored is
  realisation and the rows do not pick slot or pattern, nor that there is one encoding. No ratified
  row decides it: `proposal.toml:77` makes encoding realisation and `ruling.toml:1470` says only
  that "arithmetic and encoding are stated over the declared width".
- The definition itself, "A value of a declared signature is that signature's sole occupancy
  placement". Same with a difference: `ruling.toml:1470` uses "the value" as a thing that either is
  or is not the sole occupant of its allocation, so the ratified text has values at shared
  placements too. The rows support "a value type is the sole-occupancy placement" and do not
  support defining "value" as that placement. The row's `gap` concedes the shared case is not
  covered; its `defines = "value"` reaches past its gap. `ruling.toml:1470` decides this against
  the wider wording.
- Its `note`: "the value type belongs to a crate above" the format crate, and the carrier rung "is
  frozen by the first release because the layout observation reads it". Not reached; I left the
  crate open. No ratified row decides either.
- This row carries no `answers` edge to the question, which I agree with: whether a type holds a
  datum is a placement matter and the question is a denotation matter (my G).

### The other nineteen rows

`ingest_of_a_host_value_is_the_declared_adaptation`, `the_exact_intermediate_is_named_beside_the_value`,
`the_first_slice_answers_to_hilavitkutin_and_kaski` and the sixteen old-crate rows do not bear on
what a datum stands for or on whether a type may hold one. Not reached, all of them; I derived
nothing on ingest, the accumulator, the slice or the old crates and this read is no second
instance for any of them. Two contacts only: the ingest row's "a rejecting constructor is a
runtime check" is the same use of `ruling.toml:954` as my F, and its premise that a host value "is
a point of the ambient domain" is my A applied one step further, with the NaN gap the row itself
names.

## 4. Whether the two reads close the question

By the route in 1, the two reads do not close it, and the gate is not ours in any case. What they
supply is a convergence the coordinator can gate, over this intersection and no more:

1. A datum of a format is one point of the ambient domain, a member of the representable set.
2. A set-valued datum is a composition's and never a format's.
3. A saturating end is one point, and what maps to it is a property of the adaptation.
4. A sized type holding one value at sole occupancy is presupposed by the ratified container and
   placement rows and is the derivation's own sole-occupancy arm.
5. Open on both reads: the third option, the soundness and bestness sub-fork, a composition's
   datum, the shared placement.

Outside the intersection, at one expert still: "slot" and "one canonical encoding", the crate the
value type lives in, the freezing of the rung, the word "value" defined as the placement.

What stands between this convergence and a promotion, each for the coordinator to judge:

- Items 1 to 3 are a part answer. They close the question for a format's datum. The question as
  asked, a datum of a numeral, has a second arm (a construction's datum) that both reads leave to
  the construction. So the honest closing is the shape `ruling.toml:1474` and `:1740` used: an
  answer outside the option list, saying in its `says` what it does not decide.
- Both briefs signposted. The first read's brief named the rows at 1436, 1459, 1786 and 1824 and
  the two obligations (its research note, line 5); mine named 1447, 1470, 1536 and 1740 and two
  design passages, and both asked about "a datum of a format". So the restriction to formats is
  the dispatcher's on both sides and neither read found it. `ruling.toml:1474` records this kind of
  exposure as a contamination to state, not as a bar. What neither brief signposted and both reads
  reached: the adaptation fibre reading of the top, and the sole-occupancy arm against the design's
  "second placement".
- Both reads are normative and definitional, with no instrument. That is the same region the
  statability ruling was promoted in (`ruling.toml:1833`, "normative, no measured region [...]
  carries no instrument"), so it is not disqualifying, and it is the weakest kind the gate admits.
- Personas. I carry no named persona and do not know the first read's. `question.toml:1880` is
  open on exactly this and the coordinator's check is the only one there is.
- My order is asserted and not checkable (see 5).

If the coordinator promotes, what would be written, by the registry's own pattern
(`mockspace.toml:1223`, "A blessing is a new row naming what it promotes rather than a flag flipped
on the old one"):

- One new `[[ruling]]` in `ruling.toml`, `kind = "ruling"`, `rung = "ratified"`,
  `ratified_by = "experts"`, `topic = "the_primitive"`, with
  `ratifies = ["a_datum_of_a_format_is_one_point_of_its_ambient_domain"]`,
  `answers = ["what_a_datum_stands_for"]`, a `promotion` field naming both reads, what each was
  briefed with, and the intersection above, and a `gap` naming the construction arm, the third
  option and the sub-fork. Whether `answers` is carried on a part answer is the coordinator's call;
  the alternative is to leave the edge off and split the question.
- If item 4 is promoted too, either the same ruling also ratifies
  `a_value_is_the_sole_occupancy_placement_of_a_declared_signature` with its `says` narrowed to the
  intersection (no "slot", no "one canonical encoding", "value type" for "value"), or a second
  ruling does. It takes no `answers` edge to this question. Since a ruling ratifies a proposal's
  `says` whole (the first read's own note: "the proposal's own `says` was read as the ratified
  text"), the two clauses outside the intersection would have to be cut from the proposal row, or
  excluded in the ruling's `says` and `gap` as `ruling.toml:1829` and `:1791` do for their welded
  rows.
- Whether the proposal rows change `standing` I cannot say from the pattern: the spine rows read `two_experts` (`proposal.toml:75`) and the ratified container row still reads `one_expert` (`proposal-the-later-topics.toml:768`). Left as written, they stay
  `one_expert` under a ratifying ruling. The question row `question.toml:93` gains an `answered` field
  (`mockspace.toml:798`) in the manner of `question.toml:524`, naming the ruling and what stays
  open.
- Design corrections owed after, design tier first: `PRINCIPLES.md.tmpl:271-274` ("it is open, so
  no numeric value type exists yet") and `arvo-format/DESIGN.md.tmpl:243-244` ("which no row
  licenses and which would be a second placement").

Nothing of this is written by me.

## 5. What I could not establish

- That my section 2 preceded my reading of the proposals, to anyone but me. No commit, no
  worktree base. The registry's precedents make the ordering "checkable rather than asserted"
  (`ruling.toml:1472`); mine is asserted.
- Whether the first read's parts (a) and (b) were written before its stance file was opened. Its
  note says so (line 3 and line 61, "set against the stance file after the above was written"). I
  did not open the stance file and do not know who wrote it. If its author also wrote the design
  sentence at `DESIGN.md.tmpl:243`, or briefed either read, the count changes.
- Which persona the first read ran on, and so whether this is a second persona.
- Whether the adaptation-is-realisation sentence of `proposal.toml:77` is in the two-expert half or
  the attack-and-concession half of that row's own grading (`proposal.toml:80`). It is ratified
  either way; `question.toml:524` treats it as settled.
- The panel files behind the question (`OPTIONS`, `07`, `28`): not opened. I do not know what the
  constructor-level option's author meant beyond the option's one sentence, so my remark that it is
  trivial for a format is a reading of that sentence only.
- `mock/registry/retirement.toml` and the law files were grepped, not read. A retired or law row
  bearing on denotation that uses none of the words I grepped for would have been missed.
- Whether any ratified row forbids a value-holding type: I found none by reading every ratified
  ruling's `says` from 1392 on and the ids before it. The rows before 1392 I read by id and four by
  `says`; an earlier ratified intent could bear on it and I did not read each.
- Nothing was compiled or run, by instruction. Every clause here is a reading.
