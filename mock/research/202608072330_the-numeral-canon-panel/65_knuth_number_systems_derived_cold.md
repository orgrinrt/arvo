# 65. Number systems, derived cold

**Author lens:** Knuth. Analysis of algorithms, positional and non-positional arithmetic,
the seminumerical tradition.
**Protocol:** cold derivation, phase one. Written from the premises only: `INTENTS.md`, the
workspace rules, arvo's own rules, and op's acceptance criterion, quoted below. No panel
file, no option register, no droplist, no probe of any other member, no git log of this
directory was read before this file was committed. Phase two, the reconciliation, is
appended after this file's own commit and never edits what stands here.
**Probes:** `65_probes/derive_validate_erase.rs`, committed with this file, outcome WORKS
on the pinned `nightly-2026-05-28` with zero feature gates. Its perimeter is stated in
`65_probes/README.md` and again in section 10.

**The question.** What are the number systems arvo's canon must account for, and what is
a "number system" as a canon-level concept?

**The governing sentence.** Op's acceptance criterion for what the typestate must do:

> have the typestate derive the matching container and numeral representations, then
> validate, and erase

Both the plural in "representations" and the word "numeral" are op's, and I will treat
both as load-bearing, because on my reading they are exactly right, and the whole shape
of the answer falls out of taking them seriously.

## 0. Gates

**Canon gate.** There is no canon: `mock/canon/` does not exist and `mock/crates/` is
empty with `members = []`, both by the declared mutation order (the manifest says so in
its own comment). The panel is writing the canon; there is nothing to defend and nothing
that binds except op's intents in `INTENTS.md`. This dispatch is the canon work that
emptiness licenses. Aligned; proceed.

**Test gate.** There is no suite to run. `mock/Cargo.toml` has no members; no shipping
source, no tests. This is not a defect but the same mutation order. Vacuously passed,
stated rather than skipped. The probes committed with this file carry their own
compile-time validation, and its perimeter is stated.

**One thing to report, unsoftened, per the standing instruction.** The generated agent
instructions in this repo (`.claude/CLAUDE.md` and the per-crate lint rule files) still
describe the nuked sixteen-crate tree in the present tense and present `Hot`, `Warm`,
`Cold`, `Precise` as **the** four markers, while `INTENTS.md` I1 demotes the closure of
the strategy set to OPEN on op's direct word. Those files sit one directory above the
intents that contradict them, they read as current authority, and an agent dispatched
into this repo without this panel's brief would obey them. They describe dead code and a
demoted decision. They should be regenerated or carry a demotion notice; until then they
are exactly the kind of confident, detailed, wrong artifact the provenance ladder warns
about.

## 1. Number, numeral, system, representation, format

The oldest distinction in the subject is the one op's sentence already makes: a
**number** is an abstract value, and a **numeral** is a name for one. Seventeen, XVII,
0x11, and the bit pattern `10001` are four numerals for one number. Arithmetic is about
numbers; machines manipulate numerals; the entire craft of computer arithmetic lives in
choosing the naming scheme so that the manipulation is cheap and the correspondence is
faithful.

From that, three concepts, and the canon needs all three kept apart:

**A number system** is a carrier set of values together with an operation family and the
laws that family satisfies, including a statement of which textbook laws it does *not*
satisfy and how the failure is bounded. The integers with addition and multiplication
are a number system. So are the integers modulo 2^n. So are the dyadic rationals of
bounded exponent. So, and this matters, are the two-element Boolean algebra and the
vector space GF(2)^n, which are number systems that are not about magnitude at all.

**A representation** (a numeral system) is a relation between finite digit or bit
configurations and values of some system: a naming scheme. Two's complement,
sign-magnitude, biased offset, signed-digit, residue vectors, (sign, exponent,
significand) triples, (numerator, denominator) pairs. A representation has properties of
its own, independent of the system it names values in, and section 3 enumerates them.

**A format** is a representation pinned to a concrete container: widths, field
positions, alignment, byte order. IEEE 754 binary32 is a format; "binary floating point
with hidden bit and biased exponent" is the representation it pins; "a certain finite
subset of the rationals, plus infinities and NaN, with rounding as the correctness
relation" is the system it names values in.

The test for which level a change touches, and I offer it as a candidate canon sentence:
change the container width or field layout and no named value changes, you changed the
**format**. Change the map from configurations to values and the values named change but
the operations' meaning does not, you changed the **representation**. Change what the
operations mean, which laws hold, or what counts as the correct answer, you changed the
**system**.

One consequence deserves stating immediately, because it is the sharpest fact in the
whole area: **the format does not determine the system.** The same eight-bit container,
under the same identity map on bit patterns, is simultaneously a numeral of Z/256 (with
wrapping add), a window on Z (with checked add), a bounded chain (with saturating add),
an element of GF(2)^8 (with xor as addition, which never carries), and a mask in a
Boolean lattice. Five systems, one format. Any canon that identifies "the type" with
"the bit layout" has already made the mistake this section exists to prevent.

## 2. What a number system is, at canon level

I propose the canon define a number system by four declarations, none of which mention
bits:

1. **A carrier.** The set of values. Usually a subset of Z or Q, but not necessarily:
   Booleans, bit vectors over GF(2), and (if the panel admits them) intervals are
   carriers too, and the concept should not be worded so as to exclude them by accident.
   Whether set-valued carriers such as intervals are inside the concept or explicitly
   scoped out is a genuine open question; I carry it in section 12 rather than deciding
   it.

2. **An operation family.** Which operations exist, with their arities, and whether each
   is total or partial on the carrier. Checked arithmetic is honest partiality: the
   operation family of "integers in a window, overflow is failure" contains partial
   functions, and saying so is better than pretending totality and panicking.

3. **A law inventory.** Which identities hold, which fail, and how each failure is
   bounded. Not "this is a ring" as a blanket claim, but the itemised truth:
   Z/2^n *is* a commutative ring, every law intact, which is precisely why wrapping
   arithmetic composes so well. Saturating arithmetic on a signed window is **not**
   associative; the probe pins the counterexample `(7 sat+ 7) sat+ -7 = 0` against
   `7 sat+ (7 sat+ -7) = 7`, found by exhaustive search in const context. The same
   saturating policy on an *unsigned* window **is** associative (truncated addition on a
   bounded chain is a monoid), also verified exhaustively. The law outcome flips with the
   window's signedness alone, under one policy, on one numeral. Floating point keeps
   commutativity of addition and loses associativity, with the failure bounded by the
   rounding relation. A law inventory is a computable object, not prose, and the probe
   demonstrates that at a model width the computation is a compile-time constant.

4. **A correctness relation.** When the exact result of an operation is not in the
   carrier, what answer is correct? Rounding (in which direction, with which
   tie-breaking), wrapping (the answer is the representative mod 2^n), saturating (the
   answer is the nearest carrier bound), failing (there is no answer). This is where
   op's I9 plugs in, and it plugs in exactly here and nowhere else:

   > strategies are the variables that change what the "correct" answer is

   Read against this definition, **the strategy is a parameter of the system's
   correctness relation and law inventory**, not a parameter of the representation or
   the format. Two types can share a numeral and a container and be in different number
   systems because their strategies select different correctness relations. That is not
   a defect of the framing; it is I9 stated as mathematics.

On this definition the relationship between a system and "the algebraic structure its
operations form" (the dispatch's phrasing) is: the system *declares* its structure via
the law inventory, and is under no obligation to form the textbook one. IEEE floating
point is not a field and never claimed to be; pretending otherwise is where a generation
of numerical bugs came from. The canon should demand the inventory be stated, not demand
the structure be pretty.

## 3. What a representation is, and its four properties

A representation is a relation R between configurations (finite strings of digits in
some digit set, which for arvo means bit configurations of a container) and carrier
values. Four properties characterise it, and each is a property the derivation must be
able to reason about:

**Validity.** Is every configuration a numeral? Two's complement over a full container:
yes, total. A biased 4-bit field in an 8-bit container: no, the high nibble is not part
of the numeral, and configurations outside the field are not names of anything. IEEE
adds configurations that name non-numbers (NaN payloads). Partial validity is where
"validate" in op's sentence gets one of its meanings: a representation derived for a
demand must say which configurations are legal, and the pipeline must be able to check
membership or prove it unreachable.

**Canonicity, or its absence: redundancy.** Does each value have exactly one numeral?
Two's complement: yes, bijective on its window, which is why it won: equality is bit
equality, hashing works, comparison is one instruction. Sign-magnitude: no, two zeros.
Unnormalised floating point: no. A carry-save pair: emphatically no, and *that is its
value*; section 8 takes this up. Redundancy is neither defect nor tool in itself; it is
a property whose worth depends on the role the representation plays.

**Coverage.** Which subset of the carrier gets named, at what resolution. A window
[lo, hi] and a grid within it. The derivation's first validation is coverage: the
demanded window fits inside the derived representation's window. The probe checks
exactly this, as a const assertion over associated consts.

**A cost profile.** Which operations are cheap in this representation and which are
dear. This is where representations genuinely differ, and it is the entire reason more
than one exists. Two's complement: add, subtract, compare, shift all cheap; nothing
dear. Residue vectors: add and multiply carry-free and parallel; comparison, overflow
detection and division dear. Logarithmic: multiply, divide, powers cheap; addition dear.
Rational pairs: exact field arithmetic; normalisation (a gcd) dear, growth unbounded.
Whether the cost profile belongs *in the canon* or in the designs beneath it is open; my
own position is that the canon states that every representation *has* a cost profile and
that derivation consults it (that is I8, "decided by measurement, weighing different
measurements differently"), while the numbers themselves live in bench artifacts. The
canon carries the axis; the harness carries the values.

**Roles, and op's plural.** The criterion says "representations", plural, and I read
that as the concept the whole pipeline needs: **one demand may derive several
representations at once, keyed by role.** The obvious roles are storage (what sits in a
column, bitpacked, biased, mixed-radix), compute (what the ALU sees between load and
store, usually a native-width two's complement or a redundant intermediate), and
interchange (what crosses a serialisation boundary). Cold's intent (I6) is a statement
about the storage role; Hot's (I5) about the compute role; Precise's (I7) about the
compute role *across a chain*, which is a role with an extent longer than one operation.
Whether the role set is exactly these three, or open, I carry as a question rather than
settle; but that the concept is role-keyed and plural I would defend, because op's own
sentence already says so.

The probe establishes that a demand deriving two role-keyed representations over one
container, with coverage and round-trip laws validated exhaustively in const context and
the whole apparatus erased to the container's size, is expressible today with zero
feature gates, width held as a type-level marker.

**A unification worth writing down: packing is a numeral system.** A record packed into
a container as fields of sizes a, b, c bits is a numeral in a mixed-radix positional
system with radices 2^a, 2^b, 2^c. A field constrained to [0, 11] packed in the
tightest arithmetic code is a digit in radix 12. The factorial number system (radices
1, 2, 3, ...) ranks permutations; the combinatorial number system ranks k-subsets; both
are the natural index representations for a combinatorics crate, and both are just
mixed-radix points in the same space. So Cold's bitpacking, the bitfield macro of the
old tree, and the ranking functions a comb crate wants are not three mechanisms; they
are one concept, positional representation with per-position radix, and the canon can
say so in one sentence and collapse a family of apparent special cases.

## 4. The pipeline, read against the criterion

The acceptance sentence, expanded through sections 1 through 3, becomes a pipeline of
four verbs, each with a precise object:

**Derive.** The consumer states a demand: a carrier intent (a window and resolution, or
a system chosen outright such as "bit vector over GF(2)^n" or "index in [0, N)"), plus a
strategy. The typestate derives, from that pair: the system (the correctness relation
and law inventory the strategy selects, per I9), the representation *per role*, and the
container per representation. The consumer names none of the machinery; that is the
point of derivation.

**Validate.** At compile time, because everything being validated is a compile-time
fact: coverage (the representation's window contains the demand's), faithfulness (the
round-trip laws between role representations through the abstract value), normalisation
laws where a redundant representation participates (section 8), and the law inventory
(the laws the derived system claims are the laws exhaustive model-width computation
confirms, and an algorithm's law bounds are satisfied or the composition refuses to
compile). The probe demonstrates each of these at a model width as `const` assertions
and demonstrates the refusal as an E0277.

**Erase.** At runtime there are only formats. The system and the representations are
compile-time facts with no witness in the object code; the carrier newtype is
`repr(transparent)` over the container and has its size, which the probe asserts. This
is the existing discipline of the house (monomorphisation is the dispatch, no `dyn`, no
`TypeId`) stated as a property of the pipeline rather than as a list of bans.

One further consequence of I7 deserves its own paragraph. "Precise is accurate across
chains, not only per operation" means the unit to which a representation is attached is
not always a single value; it is sometimes an **expression extent**. A chain of
operations under Precise may run in a wider or redundant intermediate representation and
round once at the boundary, rather than rounding at every step. Error-free
transformations (a sum represented exactly as an unevaluated pair of floats), long
accumulators that make dot products exact, and compensated summation are all instances
of one idea: *the chain has its own compute representation, distinct from the endpoints'
storage representation*. The role-keyed plural covers this if roles may attach to
extents; I flag the design question and move on, but the canon should not word the
concept so that a representation can only ever belong to a value.

## 5. The kernel the intents demand

Which systems must the canon account for? I derive a kernel, item by item, each from a
stated intent, and then an open ring (section 6). The kernel is what the intents make
obligatory; the ring is what the concept makes admissible.

**K1. The residue rings Z/2^n, and the windowed integers over the same numeral.** I3
demands Warm behave as native Rust primitives behave. Native integer arithmetic *is*
arithmetic in Z/2^n when wrapping, a partial operation family on a window of Z when
checked, and a bounded-chain family when saturating: three systems on one numeral (two's
complement) as section 2 laid out. Rust surfaces all three; parity requires all three.
Note that Z/2^n is the *good* one algebraically: every ring law intact. The canon
should resist the common slur that wrapping is "incorrect"; it is exactly correct for
the ring it implements, and I9 says correctness is relative to the chosen system.

**K2. Fixed-point interpretations of integer numerals.** The reason arvo exists at all.
Canon-level fact: fixed-point is *not* a new representation. It is a new **value map**
over the integer numeral: the configuration named m now names m times 2^(-f) (or, if
the panel wants the generality, m times beta^(-f) plus an offset; scaled and biased
windows subsume temperature-in-half-degrees and its relatives). Every bit-level
operation, every container decision, every packing consideration is shared with K1
wholesale. The canon should state this identification, because it is what licenses one
storage layer to serve both.

**K3. Binary floating point, IEEE 754 shape.** I3 again: Rust's `f32`/`f64` are part of
native behaviour, and downstream consumers will hand arvo floats. Canon-level fact,
and the second great unification: **a floating-point numeral is a pair of fixed-point
fields under an interpretation map**, value = (-1)^s times m times 2^(e - bias), with
m itself a fixed-point numeral with a hidden leading digit. Floating point is a point
in the same space, not a separate kingdom: representation-wise it is two K1/K2 fields
in a mixed-radix package; system-wise its carrier adds infinities and NaN and its
correctness relation is rounding. What is genuinely new in it is only the correctness
relation and the law inventory (commutative, non-associative, bounded by half an ulp
under round-to-nearest). A canon that states K1 through K3 as one family, "scaled
integer significands under interpretation maps, differing in which parts are static",
has said something Flocq-shaped and true: the static/dynamic split of the scale factor
is the *only* structural difference between fixed and floating point.

**K4. Bounded naturals as indices: the finite ordinals [0, N).** I11: the value of arvo
is the algorithm crates that compose on top, and every one of them indexes. An index is
not a Z/2^n value (wrapping an index is never meaningful) and not a checked integer
window chosen for magnitude; it is a value of the system "ordinals below N", whose
bound is part of the type and whose only operations are order, successor within bound,
and offset arithmetic that must stay within bound. The old tree's `Cap` and `USize`
gesture at this. The canon should name the system rather than the wrapper.

**K5. The Boolean algebra, and bit vectors as GF(2)^n and as lattices.** The mask layer
and the hash layer are not integer arithmetic. Xor is addition in GF(2)^n, carry-free,
every element self-inverse; and/or/not form the Boolean lattice; popcount is a norm.
These share formats with K1 exactly and share systems with it not at all. The canon
accounting for them as systems, rather than as "integer types used weirdly", is what
keeps a mask crate's contracts honest.

**K6. Redundant compute intermediates.** Licensed by I5 (Hot may buy performance with
proofs) and I7 (Precise wants exactness across chains), and by hardware fact: every
multiplier already contains a carry-save or signed-digit core, every division a redundant
quotient-digit selection. Section 8. The kernel includes the *concept*; which redundant
forms ship is measurement's question (I8).

The kernel, then: windowed integers in their three policy systems over two's-complement
numerals; fixed-point interpretation maps over the same; IEEE-shaped floating point as
the static/dynamic unification; finite ordinals; Boolean/GF(2) structures; and redundant
intermediates as first-class compute-role citizens. Everything in this list is demanded
by a quoted intent, not by my taste.

## 6. The open ring

Systems and representations the concept admits, none demanded by a stated intent, each
kept open with its cost stated, per the explore-do-not-settle mode. I would close none
of these and commit to none.

**Residue number systems.** Value held as residues modulo pairwise-coprime m_1 ... m_k.
Addition and multiplication are digit-parallel and carry-free, which is as good as it
sounds for long products; comparison, sign detection, overflow detection and division
are dear (conversion via CRT or base extension). Serves Precise (exact big products)
and conceivably Hot in shapes with many multiplies and rare comparisons. A pleasing
unification the canon can have for free: **K1's wrapping arithmetic is already an RNS
with a single modulus 2^n.** The general RNS is the same idea with more moduli.

**Logarithmic number systems.** Value held as sign plus fixed-point logarithm. Multiply,
divide, powers and roots become adds and shifts; addition needs a table or an
approximation. Serves Hot in multiply-dominant chains (graphics, some DSP). Cost:
addition, and conversion at the boundary.

**Exact rationals, in three costumes.** Pairs (p, q) with gcd normalisation: exact field
arithmetic until the numerators grow. Fixed-slash and floating-slash formats (Matula's
and my own old work): rationals in a fixed budget with mediant rounding, and a
correctness relation with genuinely interesting number-theoretic properties
(best-approximation guarantees via continued fractions and the Stern-Brocot tree).
Continued-fraction representations with Gosper's algorithm for arithmetic directly on
partial quotients: elegant, lazy, and rarely worth it in fixed-budget hardware terms,
which is the cost statement. All serve Precise. The canon need only ensure the concept
admits pair-shaped numerals with a normalisation law; the three costumes then fit.

**Intervals.** A pair of endpoint numerals naming a *set* of values, with outward
rounding as the correctness relation; the honest way to carry error bounds through a
chain, at roughly double cost and with the dependency problem as the known failure. If
Precise's "accurate within chains" is ever to be *certified* rather than merely pursued,
this is the standard instrument. Open question flagged in sections 2 and 12: admitting
intervals means admitting set-valued carriers into the concept.

**Decimal, and radix generally.** The container is binary; that is a premise, imposed by
hardware, and pretending otherwise would be theatre. But the **representation radix is a
decision**, distinct from the container's, and the concept should carry radix as a
parameter (K2 already wants beta in its value map; mixed-radix packing already breaks
radix uniformity anyway). Whether any decimal instantiation ever ships is a measurement
and demand question; nothing downstream named so far wants it. Keep the parameter, defer
the instantiation.

**Signed-digit and balanced systems as first-class rather than transient.** Balanced
ternary is the prettiest number system there is, and signed-digit binary (digits -1, 0,
1) is what Booth recoding secretly is; both make negation free and addition carry-limited.
As shipped storage they have no hardware; as compute intermediates they are already
inside every multiplier. Likely permanent residents of the compute role only; stated so
the concept's digit-set parameter is visibly not fixed to {0, 1}.

**Posits and other tapered floats.** Same family as K3 with a different static/dynamic
split (regime bits trade exponent range against significand length dynamically). No
hardware in our targets; conversion costs real; kept as evidence that K3's family has
more members than IEEE, which is itself a reason the canon should describe the family
rather than the single member.

**Factoradic and combinadic index representations.** Mixed-radix numerals whose
positional weights are factorials or binomial coefficients; the canonical ranking and
unranking representations for permutations and k-subsets. Almost certainly wanted by a
combinatorics crate; included in the ring rather than the kernel only because no quoted
intent names combinatorial ranking.

**Gray codes.** A numeral system in the strict sense (a bijection between configurations
and [0, 2^n)) with no arithmetic affinity at all; its property is unit Hamming distance
between successors. Useful for enumeration and hardware boundaries; carried to make the
point that a representation may exist to serve a *non-arithmetic* property, which the
cost-profile axis expresses naturally (successor cheap in a specific sense; everything
else dear).

## 7. Open or closed, and what decides

The intents decide this cleanly. I1 demoted even the strategy set to open; the
toolbox-not-policer rule forbids hardcoding usage policy; harness-the-type-system
demands contracts over enumerations. So:

**The concept is closed; the inventory is open.** The canon defines once what a number
system is (section 2), what a representation is (section 3), and what admission
requires. The set of admitted instances is open, and a new one earns admission by
supplying the concept's obligations, not by amending the canon.

**The admission contract**, which is the closed concept's whole content: a candidate
system supplies its carrier, operation family with totality statements, law inventory
with bounded failures, and correctness relation (per strategy where the strategy varies
it). A candidate representation supplies its container demand, validity predicate,
canonicity statement with normalisation law where redundant, coverage, and cost-profile
axes, with the numbers established on the harness rather than asserted. Validation of
the laws at a model width must be executable at compile time; the probe shows that this
is not an aspiration but a `const` block.

**The kernel is guaranteed.** Openness must not be read as "nothing is promised". The
K-items of section 5 are demanded by quoted intents, and a canon that admitted anything
but promised nothing would fail I3 on its face.

## 8. Redundancy: defect and tool, keyed by role

One value, several numerals: is that a defect or a tool? The answer is not a compromise
but a function of role, and I would put the following three sentences forward as
canon-shaped:

At the **storage and interchange roles**, canonicity is the requirement: equality must
be bit equality, hashing must be well-defined on values rather than numerals, and a
representation with redundancy at these roles owes a normalisation to canonical form at
the boundary. (Sign-magnitude's two zeros and unnormalised floats are the classic
sins.)

At the **compute role**, redundancy is a tool with a hundred and fifty years of
pedigree: carry-save pairs break carry chains (the probe validates the 3:2 compressor's
value-preservation law exhaustively at the model width), signed-digit recoding bounds
carry propagation, double-double pairs hold what a single float cannot. Hardware
already agrees; there is a redundant representation inside every multiplier ever
shipped. Refusing redundancy at the compute role would be refusing how arithmetic
actually gets fast, and I5 explicitly licenses the trade when the gain is proven.

The **bridge obligation**: wherever a redundant representation participates, the
normalisation map to the canonical form is part of the representation's definition, and
the law "normalise after compute agrees with the abstract operation" is part of what
"validate" validates. The probe demonstrates the shape of that check.

## 9. Systems, algebraic structure, and the algorithm crates

I11 says the value of arvo is what composes on top: graph, sparse, spectral,
combinatorial crates, generic over numeric bounds. The connection to this file's
question is direct: **an algorithm's real requirement is a law set, not a type.**
Topological sort wants a partial order. Shortest path wants an ordered semiring, and for
the classic case it is the tropical one (min as addition, plus as multiplication), whose
"numbers" are costs; a canon whose concept of number system already covers "carrier,
operations, laws" gets the tropical semiring for free, and with it the honest typing of
a whole family of dynamic-programming algorithms. Spectral methods want an approximate
field with bounded rounding, which is K3's law inventory doing load-bearing work.

Two consequences:

First, the algorithm crates bound on laws, and the law inventory of section 2 is what
they bound *against*. An algorithm demanding associativity refuses, at compile time, a
(window, policy) pair that lost it; the probe pins this as an E0277 on the saturating
signed op, and the refusal is the design. This is "then validate" reaching all the way
up to the composition layer.

Second, I9 gains teeth. If the strategy selects the correctness relation, and the
correctness relation shifts the law inventory, then a strategy change can change *which
algorithms are willing to compile against the type*. That is not a bug; it is the
typestate telling the truth about a real mathematical event. The canon should say this
out loud so nobody later files it as a usability regression.

## 10. Doability

The claim needing establishment was that the criterion's pipeline is expressible, today,
on the pinned toolchain, without forbidden features. `65_probes/derive_validate_erase.rs`
establishes, with the compile transcript committed beside it:

1. Derivation: a demand window projecting through plain trait impls to a storage
   representation (biased, non-power-of-two window, the Cold shape) and a distinct
   compute representation (two's complement), each naming its container; width as a
   type-level marker, zero feature gates.
2. Validation: coverage, round-trips, the carry-save normalisation law, associativity of
   wrapping addition, non-associativity of signed saturation (with the counterexample
   pinned), associativity of unsigned saturation, and agreement of the wrap system with
   mod-16 arithmetic on representatives: all exhaustive at the 4-bit model width, all as
   `const` assertions, so a violation is a compile error.
3. Law-as-contract: a marker-trait bound refusing an op that lacks the law, recorded as
   an E0277 in `compile_fail_negative_case.txt`.
4. Erasure: the `repr(transparent)` carrier has the container's size, asserted.

What it does not establish, stated so nobody cites it beyond its worth: the general
projection from an arbitrary const `N` to a container (a separate and known-hard
question that this file does not depend on); any performance figure (nothing here is a
bench and nothing is priced); and transfer beyond the model width except by uniformity
of construction. One probe is one instance; the constructions are standard, but a later
member re-deriving any of it independently would upgrade the evidence, and I say so
rather than imply three hats on one head.

## 11. Candidate canon sentences

Phrased at intent level, each of which I believe survives the permanence test (still
true after any rewrite, in any language, in any decade). Offered as candidates for the
consolidation, not as settlements.

1. A number is not its numeral, and a numeral is not its container. The canon keeps
   system, representation and format as three concepts, with the change-test of
   section 1 as the boundary.
2. A number system is a carrier, an operation family with totality statements, a law
   inventory with bounded failures, and a correctness relation. It owes an inventory,
   not a textbook structure.
3. The strategy is a parameter of the correctness relation and law inventory: it selects
   *which system* a demand lands in, before it selects any mechanism. (I9, restated.)
4. The format does not determine the system: one container hosts many systems, and the
   canon types the system, not the box.
5. A demand derives representations, plural, keyed by role; storage, compute and
   interchange are the roles so far named, and a chain may carry a compute
   representation of its own. (The plural is op's.)
6. A representation owes four things: validity, a canonicity statement with
   normalisation law where redundant, coverage, and a cost profile whose numbers live on
   the harness.
7. Redundancy is forbidden at rest and licensed in flight: canonical at storage and
   interchange, a tool at compute, with the normalisation law validated.
8. Packing is positional representation with per-position radix; bitfields, tight
   enum packing, and combinatorial ranking are one concept.
9. Fixed point is an interpretation map over integer numerals; floating point is the
   same significand under a dynamic scale; they are one family whose members differ in
   what is static. The container radix is a premise; the representation radix is a
   parameter.
10. The concept is closed and the inventory is open: admission is by supplying the
    concept's obligations, the kernel of section 5 is guaranteed, and everything else
    is admitted by contract and decided by measurement.
11. Validation is compile-time computation, not review: coverage, round-trips,
    normalisation laws and law inventories are checked exhaustively at a model width,
    and algorithms bound on laws so that a lost law is a refused composition.
12. After validation, erasure: at runtime there are only formats.

## 12. What I could not settle, carried open

1. **Set-valued carriers.** Intervals (and any future error-tracking pair) make the
   carrier a set of sets. Admitting them generalises the concept cleanly; scoping them
   out keeps the concept smaller. Both are coherent; the decision shapes whether
   certified accuracy is expressible inside the system concept or bolted beside it.
2. **The role set.** Storage, compute, interchange, and possibly chain-extent as a
   fourth. Closed small set or open like the inventory? A closed set is checkable; an
   open set anticipates roles nobody has named. I lean closed-with-amendment but hold
   it open.
3. **Where cost profiles live.** The axis in the canon and the numbers on the harness is
   my proposal; the alternative (canon stays silent on cost entirely) is defensible and
   smaller.
4. **How the law inventory is named.** As marker contracts per law (the probe's shape),
   as a structured declaration the derivation reads, or both. Expressibility of the
   simplest form is established; which form the canon should *describe* (without
   spelling implementation) needs the panel's other lenses.
5. **The general width-to-container projection.** Deliberately not probed here; my
   pipeline claim stands on width-as-marker. Whether the canon should even speak of
   arbitrary-const-width demands, or leave width families to design, interacts with the
   forbidden-features boundary and deserves its own evidence.

---

# Phase two: reconciliation against the panel

Written after phase one's commit (`4c4353a1`), reading the panel for the first time. Phase one
above is untouched; where it is wrong, the correction is here and only here.

**Reading order and coverage, stated per the panel's convention.** Read end to end: `63` (the
format-concept consolidation, both pages), `53` (the container-derivation consolidation), `66`
(the parallel cold derivation on this same question, phase one; its phase two is a stub at this
writing), `OPTIONS.md` Q1 through Q17 plus the unasked-questions section and the wrapping entry,
the droplist's panel-closed section, `08`'s verdict and gates (my own persona's earlier file), and
`55`'s section 3c at the source. Read by grep for specific claims: the `m * r^q` scoping theorem's
carriers, the tropical entry in `DROPLIST.md`, and the absence of any prior report on the stale
agent instructions. Not read: the remaining member files; every statement below about `55` through
`62` is sourced to `63` and marked so, and if `63` misread a member, the corresponding row here
inherits the error. I re-ran no panel instrument; `63` section 1 reports all thirty-two of unit
two's re-run byte-identically and I rely on that report.

**Independence bookkeeping for this file and `66`.** Both cold derivations answer the same
dispatch from the same premises. Mine committed at `4c4353a1`, `66`'s phase one at `4a856b0c`,
after mine; neither read the other (both blind by protocol, ordering checkable in the history, the
same verification `63` section 1 ran for `55` and `60`). So where we agree, that is two
independent instruments over one shared premise set: worth more than a read, less than two
arrivals from nothing, the same discount `63` applies to `55`/`60`'s shared literature. I apply it
to every agreement claimed below.

## 1. Where the panel and phase one converge, rung by rung

**The three-level split is the panel's split, in different words, and now has three independent
arrivals.** My system / representation / format triple, `66`'s five-level hierarchy (number,
system, scheme, format, container), and unit two's identity-and-realisation decomposition
(`63` sections 3.2, 3.5) cut the same space. The mapping, stated so a consolidator can merge
rather than adjudicate:

| this file | `66` | unit two (`63`) |
|---|---|---|
| carrier + operations + laws + correctness | number system (levels 1-2) | ambient domain D + selected adaptation + law layer |
| representation (validity, canonicity, coverage, cost) | scheme (level 3) | representable set Q + encoding E |
| format (pinned to container) | format + container (levels 4-5) | realisation onto the container |

The cuts are not identical and section 2 below carries the one real difference. But the shared
content is now at the strongest rung anything in this question holds: the number/numeral
distinction, the strategy attaching at the correctness relation (my candidate 3, `66`'s identity
result, C4's I9 attachment), and the derive/validate/erase pipeline being expressible gate-free on
the pin (my probe, `66`'s `derive_validate_erase_pipeline.rs`, and the unit's five builds listed
at `53` section 8) all have at least two blind instruments plus the unit's converged text.

**"The format does not determine the system" survives and sharpens.** My probe's same-numeral
three-policy result is the small edition of what the unit measured at scale: `55_probes/p4`'s
induced-algebra ladder and `56_probes/q1`'s four inhabited law-role cells (both per `63` section
3.4). My specific cells reproduce theirs: signed saturating addition non-associative (my pinned
witness; their 952 divergent triples at w = 4), unsigned saturating addition associative (their
commutative-monoid row), wrapping associative (their ring row at F = 0). And my "the law outcome
flips with the window's signedness alone" is a special case of `57b`'s closed form as `63` section
4.2 states it: additive congruence iff the range is sign-confined. A blind third instrument
agreeing with a measured theorem-shaped result is exactly what that result wanted; I claim no more
for it than that.

**Role-keyed plural representations converge from three directions.** My storage/compute/
interchange roles, `66`'s working-against-storage representation split, and the container
derivation's own outputs: S3's per-value and per-aggregate questions, and S6's contingent
compute-form fact ("a strategy entitled to diverge what an operation computes in from what a value
occupies at rest forces a further carried fact", `53` section 7). S6 is my compute role arriving
from the derivation side. The convergence matters for op's `Precise` question in `53` section 3.4:
my phase-one reading of I7 (the chain has its own compute representation, distinct from the
endpoints' storage representation) is the same reading `60`'s window mechanism formalises (`63`
section 5), and it lands on the same open op question: does the wide product get carried between
operations, and does `Precise` pay its I2 price at rest.

**The chain material.** My section 4's paragraph on expression extents is `60`'s derivation in
miniature, and theirs is far deeper (schedules as part of the function's meaning, the exactness
grades, the window with derivable capacity, the statability argument that a concept closing its
ops over the format cannot express I7 at all). Nothing in my paragraph adds to C9; I fold mine
into it and note that my phase one, blind, landed on the D-B direction's content without
considering D-A or D-C, which slightly strengthens D-B's claim to be the natural reading and
proves nothing about the other two, which remain live per `63` section 5.

**Laws as compile-time contract.** My marker-trait probe (with the committed E0277) is a third
instance beside `35_probes/p7` and `42_probes/p2` (per Q11's "both" option), and Q11's second
option ("the numeral names its algebraic structure") is my candidate 11 already in the register.
The droplist's tropical entry (`35`'s interior-wrapping min-plus failure at 12.6% of 622M
instances) is a measured instance of my section 9 claim that algorithms bound on laws rather than
types, made before I read it.

## 2. The one substantive difference, located precisely

My phase-one definition folds the correctness relation into the system's identity: wrap and
saturate over one window are two systems. Unit two's converged position (C2, argued by attack and
concession per `63` section 3.2) puts adaptation choice outside identity: a format is (D, Q), the
adaptation space is derived, and a strategy selects a member per operation. `66` hit the identical
fork blind and carried it open (its first carried-forward item: shape alone, or shape jointly with
strategy).

Having now read the unit's argument, I think the difference is **mostly nominal and worth one
sentence of care rather than a fight**. Every arvo type carries its strategy, so on *type*
identity the two cuts agree; they differ on which layer the word "format" (or "system") names.
The unit's cut earns its keep at Q10 (format equality decided on (D, Q), with the wrapping
sections' observational-equivalence argument behind it), and mine earns its keep at the law layer
(the induced algebra is a property of the pair, format plus selected member, which is what an
algorithm's bound actually consumes). Both facts survive under either vocabulary. What I would
change in phase one: state the definition as the pair explicitly, "a number system in this file's
sense is a format together with a selected adaptation member per operation", which makes it a
*view* over the unit's decomposition rather than a rival to it. The fork `66` and I both carried
then dissolves into vocabulary, except for one residue that is genuinely op's: whether the
canon's own word "numeral" names the (D, Q) identity or the pair. That is a naming call, it is
cheap, and it belongs on the pile with `53` section 9's "are the criterion's nouns canon
vocabulary" question.

Second correction of the same kind: my phase-one candidate 2 says the law inventory is
"declared". The unit's H1/H2 frame (`63` section 4.2) derives it: congruence verdicts per
operation, computed from range geometry, with the twenty-four-cell prediction at zero residue. A
derived inventory is strictly better than a declared one, for exactly the reason my own candidate
11 wanted validation to be computation. C6 supersedes my candidate 2's "declares" verb; the rest
of the candidate (an inventory owing bounded failures, not a textbook structure) survives intact.

Third, a scope correction my phase one could not have known: my candidate 7 ("redundancy is
forbidden at rest") would foreclose the register's open redundant-encoding question (`63` section
3.5: redundant encodings are wholly unexamined and could conceivably buy raw order and the raw
adder together, the hole `59`'s untested theorem sketch would close). Weaken it to a default: at
rest and at interchange, canonical unless a stated normalisation and equality discipline is
supplied. The compute-role half of the candidate stands and gains the unit's own pullback
mechanism as company.

## 3. Where the scoping theorem meets my open ring

The panel carries a theorem my premises did not contain: every arvo value is `m * r^q` (TWO
EXPERTS in the predecessor record, carried but flagged unrederived by `63` section 3.6). That
boundary sorts my phase-one section 6 cleanly, and mostly into homes that already exist:

- **RNS**: inside by value set, per `08`'s own classifier (an integer interval; only the datum map
  differs). My phase-one filing of it as a compute-role representation of Z/M is compatible and,
  I would now say, the correct slot: the scoping theorem constrains carriers, not compute-role
  encodings. The wrap-is-single-modulus-RNS unification survives untouched.
- **Carry-save, signed-digit, mixed radix**: same, inside by value set, encoding-axis citizens,
  which is where my phase one put them (compute role). `08` and my phase one agree from opposite
  directions: it classified them by value set; I filed them by role.
- **LNS**: genuinely outside `m * r^q`, which `08` measured (8 of 64 rational). My open-ring entry
  stands but its admission cost is now concrete: it is not a new format, it is an amendment to a
  carried theorem, which is op-tier and should be priced as such.
- **Stored-pair rationals, intervals, error-carrying pairs**: the panel files them as compositions
  over formats, not format instances (`55` section 3c, read at source; uncontested through seven
  files per `63` section 3.6). That answers my carried open question 1 (set-valued carriers)
  better than either of my phase-one alternatives: the concept stays small, the molecules get a
  home, and the composition layer owes its own laws. I adopt that filing and withdraw my open
  question 1 in its phase-one form; what remains of it is Q4's measured fourth option, already
  priced (order lost, 42.05% comparability at `U<2,2>`, per the register), and Q16's word for
  sense two.
- **Decimal**: my "container radix is a premise, representation radix a parameter" matches the
  concept's radix parameter (the slot function is per-radix; `55` section 3c's arbitrary-rational
  quantum widening covers currency-style scales). Nothing to change.
- **Posits**: already classified inside the general concept and outside every named shape
  (`08`), stronger than my open-ring entry; mine defers to it.
- **Factoradic, combinadic, Gray, and packing-as-mixed-radix**: nowhere in the panel. These
  survive as this file's contribution; see section 5.

## 4. On `66`, plainly, per the standing instruction

`66`'s phase one is good work and its convergence with mine is the real kind. One defect must be
named without softening, because it is the exact failure the mutation order exists to prevent:
`66:60-68` uses arvo's `.claude/CLAUDE.md` crate table (`Bits` in `arvo-storage`, `UFixed` over
it) as a "genuine cross-check" that its five-level hierarchy "predicts the layering the existing
crate table already describes". That crate table describes the nuked tree. It is a design-tier
document for code that was deleted precisely so canon work would not consult it
(`mock/Cargo.toml`'s own header; `canon-design-code-chain.md`, "an agent that consults a live
dependent design or its shipped source while editing the canon... is reattaching a tier that had
to be detached"). The generated instructions were in our premise list, which explains the reach
and does not license the use: they are the one premise that is *stale by declaration*, and my
phase-one gate report flagged them as exactly this trap before reading `66`. The cross-check
paragraph should be struck or reframed as "consistent with the prior attempt", carrying no
evidential weight. `66`'s hierarchy does not need it; it stands on op's wording alone.

The same goes for `66`'s uses of `arvo-bridge-home-rule.md` and the cross-strategy resolution
sketch in `arvo-toolbox-not-policer.md` as "prior design attempts offered as evidence of a
workable shape": `66` marks them correctly as evidence-not-adoption, which is the right register,
and a consolidator should keep that marking, because both describe the dead tree's designs.

And the report stands for the dispatching layer, sharpened: **nobody before files 65 and 66 had
flagged that the repo's generated agent instructions contradict `INTENTS.md` I1 in the present
tense.** Two blind derivations both tripped over the same stale premise within hours, one
reporting it and one building a cross-check on it. That is the measured cost of leaving them
unregenerated, and it will be paid again by every future dispatch into this repo until they are
regenerated or carry a demotion notice.

## 5. What this file contributes that the panel does not already hold

Stated so the next consolidation can take or refuse each by name, with rungs:

1. **The format-does-not-determine-the-system sentence** (phase one, section 1), as an explicit
   canon-candidate line: one container hosts Z/2^n, the checked window, the bounded chain,
   GF(2)^n and the Boolean lattice at once. The unit has the arithmetic three; nobody states the
   principle, and nobody puts the GF(2)/lattice systems inside the same concept. `66`
   independently raises the Bool/mask scope question and carries it open; my phase one answers it
   (broad reading: the concept is carrier-plus-ops-plus-laws and does not require magnitude),
   `66` declines to answer, so this is ONE EXPERT plus an independent posing of the question.
   The mask and hash layers are real consumers (I11); a canon silent on what algebra they compute
   in has a gap the first mask contract will expose.
2. **Bounded naturals as a named system** (finite ordinals with the bound in the type), distinct
   from Z/2^n and from checked windows: indices never wrap meaningfully. ONE EXPERT; the panel is
   silent; `Cap`/`USize` in the dead tree gesture at it and are not evidence.
3. **Packing is mixed-radix positional representation**, unifying bitfield packing, tight
   non-power-of-two windows, and combinatorial ranking (factoradic, combinadic) under one concept
   the canon can state in a sentence. ONE EXPERT, no probe, and it composes with rather than
   competes against the container derivation's stride work: stride is the aggregate-repetition
   question, mixed radix is the within-container field question. A cheap probe (rank/unrank round
   trip at model sizes, const-evaluated) would move it; not written here.
4. **The RNS-is-wrap-with-one-modulus observation**, which costs one sentence and buys the
   general-RNS door staying visibly open without any new mechanism.
5. **A third blind instrument** on the cells named in section 1, and the carry-save 3:2
   normalisation law checked exhaustively in const context, which no panel probe carries (`08`
   classified carry-save by value set only; nobody checked its law).
6. **The Gray-code point**: a representation may exist to serve a non-arithmetic property, which
   the cost-profile axis should be worded to admit. One sentence, ONE EXPERT.

And one contribution withdrawn: my phase-one open question 5 (the general width-to-container
projection) is not open; it is the container-derivation unit's settled ground (the kind boundary,
`53` section 3.3, four probe files), and my deliberate non-probing of it was the right call for
the wrong reason. The refused direction is const-to-type; my width-as-marker probe sits on the
accepted side of exactly that boundary, which I now cite instead of hedging around.

## 6. What I would change in phase one, summarised

Nothing is edited above; this is the change list a successor should apply when consuming it.
Candidate 2: "declares its laws" becomes "derives its laws, and declares which failures are
bounded and how" (C6 supersedes). Candidate 3: survives, now TWO-plus arrivals, and should be
stated as C4 states it (selection per operation, in the derived space). Candidate 7: weaken the
at-rest half to a default with a stated escape (the redundant-encoding hole is open). Candidates
1, 4, 5, 10, 12: survive; 5's role set gains `66`'s working/storage vocabulary as a candidate
naming and `53`'s S6 as its derivation-side twin. Candidate 8 (packing as mixed radix): survives
as a proposal owing a probe. Candidate 9: the fixed/float one-family sentence is `55`/`08`'s
ground with better instruments; mine defers to theirs, and `66`'s Reading B (law-set separation)
should ride along as the carried alternative. Open question 1: withdrawn in favour of the
composition filing. Open question 2 (the role set): still open, now with three vocabularies to
reconcile. Open question 4 (how the law inventory is named): partially answered by the unit,
marker-contracts and derived verdicts are not rivals; the verdicts compute what the contracts
assert, and the probe evidence for their composition exists (`42_probes/p2` per Q11).

**Nothing here settles anything.** The mode is explore; this file and `66` go to the same
consolidation, and the located disagreement in section 2 plus the contribution list in section 5
are what that consolidation should argue about.
