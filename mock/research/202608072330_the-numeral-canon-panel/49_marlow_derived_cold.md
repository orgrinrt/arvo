# 49. What a numeral's type must make available at a lowering site

**Marlow. Phase one, cold.** Written having read only `INTENTS.md`, `00_brief.md`, and the workspace
discipline under `~/Dev/clause-dev/.claude/rules/`. No numbered panel file, `OPTIONS.md`, `DROPLIST.md`,
`RULES.md`, probe directory, git log, or commit message was read before this section was committed. That
is a fact about how this section was produced, offered so a later reader can weigh it as one independent
instance among however many others this panel has produced under the same protocol, not as a claim of
special reliability.

## The question, restated

A consumer declares a numeral by its usage in bits and bytes. Something has to determine how it is stored
and how a sequence of them is laid out. What must that determination produce, how many distinct facts,
and in what form must each be available at a site that lowers the numeral, versus what can that site
compute for itself?

## The premises this derivation rests on

Quoted from `INTENTS.md`, with the entry label:

- **I1.** The strategy set is not closed at exactly four; `Hot`/`Warm`/`Cold`/`Precise` are a prior
  attempt at the intent, not the intent.
- **I2.** Each preset names a stated intent, not a derived rule: Hot is as fast as possible, Cold stores
  as small as possible, Precise is the most precise at the cost of storage and compute, Warm is the
  intuitive default.
- **I5.** Hot may sacrifice soundness for a proven, meaningful gain.
- **I6.** Cold is for cold storage: it "aggressively minimises and bitpacks", and its second quote is
  load-bearing here: "Cold does not have to drop efficiency wins elsewhere. It can use the same paths Hot
  uses, not because it needs to by intent, but nothing in its intent would fight it."
- **I7.** Precise is accurate across chains and operations, not only per operation, and sacrifices
  performance/efficiency to get there.
- **I8.** The strategies weigh different measurements differently; they usually agree, but not always.
- **I9.** "The strategies aren't orthogonal to the threaded question... strategies are the variables that
  change what the correct answer is for what we choose as the path." Storage is therefore not a pure
  function of width; strategy participates.
- The acceptance criterion in `00_brief.md`: the consumer expresses usage in bits and bytes, the typestate
  derives the matching container **and numeral representations** (plural, op's own words, quoted at
  `seed/SETTLED_container.md:33-35` per the brief), it validates, it erases on lowering. All four, no
  caveats.
- The hard constraints: `no_std`, no `alloc`, no `dyn`, no `TypeId`, sizes const, `generic_const_exprs`
  and full `specialization` forbidden.

## The derivation

### Storage is not a pure function of width (I9)

If storage were a pure function of logical bit width alone, the strategy axis would be decorative. I9
says the opposite directly: the strategy is what makes an answer correct. So the derivation from "N bits"
to "how it is stored" needs strategy as an input, not merely width. Two numerals declared at the same
width under different strategies may legitimately land in different containers, and neither is wrong; each
is answering a different question, per I8's "weigh different measurements differently."

### The acceptance criterion's plural is the load-bearing clue

"Container and numeral representations" is two things, not one, and the second is plural. I read this as:
**container** names the single concrete storage type or storage shape a value occupies at rest, and
**numeral representations** (plural) names the fuller set of representational facts a numeral's type must
carry: at minimum, the storage-at-rest form and a separate compute/operand form, because those two can
diverge.

I6's second quote is the direct textual support for the divergence being real rather than a scope
question I'm inventing: "Cold... can use the same paths Hot uses... nothing in its intent would fight
it." A path is a compute path. For Cold's stated intent (aggressive bitpacking, because the data is "just
sitting") to coexist with reusing Hot's compute path, the type occupied while the value is at rest
(bitpacked, minimal) and the type occupied while an operation is being performed on it (whatever Hot's
path wants) must be allowed to be different types. If they were forced to be the same type, Cold could
never legitimately borrow Hot's path without abandoning its own storage intent, and I6 explicitly says it
can.

So: **storage representation and compute/operand representation are two distinct facts**, coincident for
some strategies (there is no reason for Warm's storage and compute forms to differ; I3/I4 says Warm
imitates a native Rust primitive, and a native primitive does not usually convert itself before an
operation) and divergent for others when the strategy's own intent calls for it (Cold, on I6's word;
plausibly Precise too, widening its operand to reduce intermediate rounding across a chain per I7, while
its storage stays at the logical width since Precise has no stated reason to spend extra storage at rest).

### The packing model for a sequence is a third fact, not a derived one

A single numeral in isolation still occupies whole bytes; nothing below one byte is addressable in
isolation. The interesting layout decision only exists once numerals are composed into a sequence: does
element `i` sit at a byte/word-aligned offset (an array of individually addressable storage slots), or
does the sequence exist as a genuine bitstream where element boundaries fall at arbitrary bit offsets
inside a byte another element also occupies?

I6's "aggressively minimises and bitpacks" reads as the second shape for Cold specifically: if a 13-bit
value's isolated storage form already rounds up to the smallest byte-aligned container (2 bytes), further
"aggressive" minimisation only has anything left to buy by going below that rounding once several values
are composed together, i.e. by genuinely packing the 3 wasted bits of consecutive elements against each
other. That is not a property recoverable from the storage type alone (a 2-byte storage type says nothing
about whether the *sequence* of them wastes 3 bits per element or not); it is an independent policy
decision the strategy makes about composition, and it needs to be exposed as its own fact (an alignment
value, and a marker for whether the sequence composes as an array or a bitstream) rather than left for
whatever assembles the sequence to guess at from the per-value storage type.

### What is a genuine fact versus a derivable quantity

A quantity earns a place as an independent, exposed fact when getting its value requires consulting the
strategy as an actual decision (not a formula), and when an entity other than the numeral itself (a
sequence type composing many of them) needs the answer and cannot safely re-derive it without risking
disagreement with what the numeral's own definition intended. A quantity does not need its own fact when
it is a pure, strategy-independent function of facts already exposed: byte size of a value at rest given
its storage type (`size_of`), a mask for extracting the logical bits from a wider container, a shift
amount for sign extension. All of these are the same formula regardless of which strategy produced the
storage type, so recomputing them at the lowering site costs nothing and duplicates nothing; storing them
separately would only be another place for the number to drift from the type it was supposed to describe.

By this test the facts a numeral's type must expose are:

1. **Logical bit width.** An input the consumer supplied, not an output of the derivation, but it must
   still be exposed on the type, because it cannot be recovered from the stored bytes alone once storage
   has been chosen (see below). Downstream code that needs "how many bits are semantically significant
   here" has no other place to ask.
2. **Sign.** Also an input, orthogonal to width, needed at every lowering site that selects a signed
   versus unsigned instruction.
3. **Storage representation.** The concrete type (or shape) a single value occupies at rest. A genuine
   strategy-derived fact per I9.
4. **Compute/operand representation.** The concrete type a value occupies mid-operation. Coincides with
   (3) for most strategies, diverges when the strategy's intent calls for it (I6).
5. **Alignment for sequence composition.** A strategy-derived fact, not a function of the storage type
   alone, since two strategies can share a storage type and still want different alignment behaviour in a
   sequence (a padded native container versus a value that composes into a packed stream).
6. **The packing model itself**, i.e. whether a sequence of these composes as an array of individually
   addressable storage slots or as a genuine bitstream. I hold this as a "must be available" fact rather
   than a derivable one under a conservative default: I could not convince myself it reduces cleanly to a
   pure function of (storage type, alignment) without asserting a further design choice (that Cold's
   isolated-value storage type is defined as "exactly N raw bits with no rounding" rather than "smallest
   native or byte-rounded type that fits") that the premises do not settle. I flag this explicitly as the
   weakest link in the enumeration; see "What I could not settle" below.

Validation is not a seventh fact the lowering site reads. It is a property of whether the type exists at
all: under the forbidden-features constraint (no `generic_const_exprs`, no full `specialization`), the
mechanism has to be a trait implemented per `(Strategy, Sign, N)`, and a combination with no impl simply
fails to compile with a missing-trait-bound error. That is the validation the acceptance criterion asks
for, achieved as negative space rather than as an output anyone consults.

Erasure follows from the same mechanism: every fact above must resolve to a concrete monomorphised type or
an associated `const`, never a runtime field, so that nothing about strategy, width, or packing model
survives past compile time into the value's actual bit pattern.

### Does a bit width determine storage?

No. I9 rules this out directly, and the derivation above depends on that ruling: storage is a function of
`(width, strategy, sign)`, not width alone.

### Can a consumer's declared width be recovered from what is stored?

No, not from the bytes alone. Two numerals of different declared widths can share the same physical
storage type (a 13-bit and a 16-bit unsigned value under Warm both plausibly land in a native `u16`), so
looking only at the raw bit pattern in memory cannot tell you which logical width produced it. This is
exactly what "erasure" describes and it is the reason fact 1 (logical width) has to survive as a
compile-time associated constant on the type: it is genuinely lost at the byte level, and the type is the
only remaining place to ask.

### Does the answer change with strategy, and is that a property of the strategy or of what the strategy selects?

The **schema** (which facts exist, and in what form: two representational types, two markers/consts,
one width, one sign) reads as uniform across strategies to me: every strategy answers the same six
questions. What differs by strategy is the **values** filled into that schema. This maps directly onto
the trait-decomposition instinct in `a-refused-bound-wants-a-trait-not-a-feature.md`: one trait names the
schema, and per-`(Strategy, Sign, N)` impls (ordinary Rust, no inline const-generic expression) supply the
values, which sidesteps the forbidden-feature wall entirely rather than needing to argue around it.

## The probes

Four files, `49_probes/`, compiled against the pinned `nightly-2026-05-28` (`rustc 1.98.0-nightly
(57d06900f 2026-05-27)`), no `cargo`, no `mock/crates` reference (it does not exist).

- **`p1_fact_schema.rs`** (+ `_shared_schema.rs`, the shared, attribute-free body both later probes
  `include!`): one trait, `NumeralFacts<Sign, const N: usize>`, with associated types `Storage`/`Operand`
  and associated consts `ALIGN`/`PACKED`/`WIDTH`. Filled for four strategies (`Hot`/`Warm`/`Cold`/`Precise`,
  used descriptively per I2, not asserted as the settled set per I1) at three widths (8, 13, 17,
  unsigned). Compiles clean, no unstable feature gate, no warning. Ten `const _: () = assert!(...)`
  compile-time checks confirm: Hot and Cold's storage byte size coincide at N=13 (2 bytes) while their
  `PACKED` flag diverges (false vs true); Cold's storage/operand types diverge (`[u8; 2]` vs `u16`) while
  its operand coincides with Hot's storage type, exactly the I6 "reuse Hot's path" shape; Precise's
  operand widens past its storage at N=13 (`u16` storage, `u32` operand); Cold collapses to the
  unpacked/coincident shape at the byte-exact N=8, matching I6's "nothing in its intent would fight it."
- **`p2_validation_is_a_missing_impl.rs`**: asks for `<Cold as NumeralFacts<Unsigned, 999>>::Storage`,
  a combination with no impl. Confirmed **FAILS TO COMPILE** with `E0277: the trait bound
  Cold: NumeralFacts<Unsigned, 999> is not satisfied`, once the associated type is actually forced to
  resolve (a bare unused `type Bogus = ...` alias does not trigger the check; a function returning it
  does). Validation is the trait solver's ordinary refusal, not a separate mechanism.
- **`p3_erasure_check.rs`**: three `extern "C"` functions moving values between the facts above,
  compiled at `-O3` to `aarch64` assembly (`p3.s`, committed, 26 lines). `cold13_storage_roundtrip` lowers
  to a single `and x0, x0, #0xffff; ret`. `cold13_storage_to_operand` (the storage-to-operand conversion
  for the strategy where they diverge) lowers to `and w0, w0, #0xffff; ret`. `precise13_operand_widen`
  lowers to a bare `ret` (the widening is free at this ABI boundary). No branch, no vtable, no tag: a
  qualitative compiled check that the facts erase, not a timing claim.
- **`p4_sign_axis_is_orthogonal.rs`**: adds `Signed` impls for `Warm`/`Cold` at N=13 alongside the
  existing `Unsigned` ones for the same strategies, through the same trait, no special case. Compiles
  clean. Supports treating sign as an independent axis of the same schema rather than a fact needing its
  own separate derivation mechanism.

## What I could not settle, from the premises alone

**Whether the packing model (fact 6) is a genuinely independent fact or a derivable consequence of a
further design choice about what Cold's isolated-value storage type means.** I described two readings in
the derivation above and could not close between them from I1 through I9 alone. If Cold's isolated
storage type is defined as "exactly N raw bits, no rounding at all" (as opposed to "smallest byte-rounded
container"), the packing model plausibly falls out of that definition for free, and fact 6 collapses into
a refinement of fact 3 rather than standing on its own. I chose the conservative default (keep it
separate) because I could not find textual support in the premises for either reading, and getting it
wrong in the direction of "assume it is derivable" risks silently defaulting a Cold sequence to a
Warm-shaped layout, which directly contradicts I6.

**What "usage in bytes" (as distinct from "usage in bits") means as a consumer-facing input.** The
acceptance criterion says "bits and bytes" together; I have treated the consumer's declaration as
fundamentally a bit width and treated byte-level facts (alignment, storage byte size) as outputs of the
derivation rather than as a second independent input the consumer states directly. I could not find
anything in `INTENTS.md` or `00_brief.md` that settles whether a consumer can also directly state a byte
constraint (e.g. "I want this to occupy exactly 4 bytes regardless of logical width") as an input
alongside or instead of a bit width. If they can, that is a further input fact this derivation has not
accounted for.

**Whether alignment (fact 5) is itself the numeral's own fact, or a fact of the composing sequence type
that merely consults the numeral's strategy.** I treated it as belonging to the numeral because the
sequence type has to get it from somewhere and the numeral is where the strategy lives, but I did not
derive why it could not instead live as a separate strategy-keyed lookup performed entirely inside the
sequence type, consulting only the strategy marker and never touching the numeral's own associated
consts. I do not have a premise that forces one placement over the other.

**Coverage bound.** This derivation covers unsigned widths 8, 13, 17 and one signed width (13), across
four strategies, by direct compiled example. It does not attempt an exhaustive sweep of widths (that
would need per-width impls out to whatever the maximum representable width is, which is design work this
derivation was not asked to settle) and it does not attempt more than one strategy pair's worth of
storage/operand divergence (Cold, Precise). The schema-uniformity claim (six facts, same shape, per
strategy) is supported by every cell I filled agreeing on shape and none refusing to fit; it is not
proven for cells I did not fill.

---

## Phase two: reading the panel

Read after the section above was committed (commit `2430fad`): `15`, `16`, `44`, `45` (including its
sections 11-12 reply to `46`), `46`, `47`, `48`, and the "derivation's outputs" section of `OPTIONS.md`
(lines 703-870). This section reconciles; phase one above is untouched.

### The panel is the same question, four files ahead, and checkpoint 48 dispatched exactly this file

`48` (the checkpoint immediately before this dispatch) named "derive it cold, with the reading order
inverted, and forbid the panel until the answer is on disk" as the single highest-value remaining act on
this topic, and specified giving the expert only `INTENTS.md`, `RULES.md`, `00_brief.md` and the question
(`48:436-456`). That is this dispatch, word for word. So phase one above is the artifact `48` asked for,
and this section reports where it lands against the unit `48` was auditing.

### Where phase one stands, corrected in the panel's favour

**The two required outputs (my facts 3 and the packing-model question, fact 6) match the panel's carrier
and stride, and my hedge on fact 6 was well-placed but under-resolved by me and over-resolved by the
panel's later files.** `15` and `16` establish, independently and by different routes (`15` by fixing two
bugs in a stride formula, `16` by an injectivity argument on `Cold`), that a derivation needs (a) the
machine type an operation lowers to and (b) the bit distance between consecutive elements of an aggregate,
and that neither is recoverable from the other. My phase-one fact 3 (storage representation) is their
carrier. My phase-one fact 6 (packing model) is a strictly *weaker* encoding of their stride: I asked only
whether a sequence packs (`PACKED: bool`), where the panel's stride is the exact bit count. My own probe
already computed the exact number in one place (`storage_byte_size`) and I did not go the last step to
name it as a first-class fact; the panel did, and it is the more complete answer. I concede this
straightforwardly: **stride, not a packed/unpacked flag, is the correct form of my fact 6.**

**My fact 5 (alignment) is not a separate fact and I over-counted it.** `16` establishes `align_of` is a
property of a type (`16:605-613`, confirmed independently by `47:299-302`), so it rides on the carrier
(my fact 3, Storage) and needs no independent associated const. My probe's `const ALIGN: usize` was
redundant with `core::mem::align_of::<Self::Storage>()`, and I should have recomputed it rather than
stored it. This directly narrows my six-fact enumeration to closer to what the panel converged on: width
and sign as inputs the type already carries, plus carrier and stride as the two required derivation
outputs, plus a possible third (operand/compute carrier) contingent on unsettled strategy semantics.

### Where phase one is wrong, and the panel's evidence is better than mine

**My Cold `Storage` diverging from Warm's `Storage` (`[u8; 2]` versus `u16` at N=13) does not match the
panel's doubly-corroborated finding, and I think the panel is right and I was wrong.** `15:317-319`: "a
lone `UFixed<13,3>` is a `u16` whatever strategy you asked for." `16` reaches the identical conclusion
independently, from the opposite direction (a lone packed value has to have a size, so `Cold` cannot be a
statement about the standalone type at all, `16` section 2). `OPTIONS.md` records this as "TWO EXPERTS,
both self-report independent arrival" (`OPTIONS.md:750-757`). My own probe's own compile-time assertion
(`p1_fact_schema.rs`: `storage_byte_size::<Hot, Unsigned, 13>() == storage_byte_size::<Cold, Unsigned,
13>()`) already established the *sizes* agree, and I stopped short of asking whether the *types* should
too. They should, on the panel's evidence: a `[u8; 2]` standing alone buys nothing over a `u16` standing
alone (same two bytes, no packing possible until there is a second element to pack against), so giving
Cold a distinct standalone `Storage` type is cosmetic where I intended it to be substantive. The real
saving is entirely a fact about the aggregate (stride), never about the lone value, and I built a
type-level distinction to carry a fact that only exists once you have more than one value. I would redo
my probe with `Storage` identical across `Hot`/`Warm`/`Cold` at a shared width and sign, and carry the
packing fact on stride alone.

**The consequence for my Storage/Operand split, derived from I6, is narrower than I wrote it.** I derived
the split from "Cold... can use the same paths Hot uses" (I6), reading it as requiring a distinct Operand
type for Cold, wider than its Storage. Once Cold's Storage is corrected to equal Hot's and Warm's (previous
paragraph), I6's quote is satisfied trivially and without a separate Operand slot: Cold already computes in
the same type Hot does, because it is the same standalone carrier. So my own motivating textual evidence
(I6) does not, on reflection, force what I built it to force. **The genuine Storage/Operand divergence
question in this design is `Precise`'s alone**, exactly where the panel has spent four files
(`16` section 6, `44` section 6, `45` sections 1-4, `46` section 5, `47` sections 4 and 9) and has not
settled it. My probe's `Precise: Storage = u16, Operand = u32` happens to instantiate the "Precise widens"
reading the panel debates, but I chose it because it fit I7's prose intuitively, not because I derived it
from anything as rigorous as `45`'s pigeonhole argument (an information-theoretic proof that no
fixed-width intermediate, under any rounding rule, can match the once-truncated exact chain answer for a
two-step multiply chain, `45` section 3.2). My phase-one probe is a compatible instance, not independent
corroboration of the widening reading; I want to be precise about that rather than let agreement read as
more than it is.

### What phase one got right, that the panel had not sharpened the same way

**The schema-is-uniform, values-differ-per-strategy shape directly answers one of `48`'s named open
items for the next dispatch, independently.** `48` section 7.2 names "is the fact set closed under `I1`"
(the open strategy set) as an unaddressed, cheap, expert-level question: "a fact set that is a list of
four answers does not survive a fifth strategy; one that is a set of questions does." My phase-one section
"Does the answer change with strategy" answers exactly this, arrived at before I had read `48` or seen the
question posed: the schema (which facts exist, and in what form) is strategy-independent; only the values
filling it vary per strategy, which is precisely the shape that survives `I1`'s open strategy set, because
a new strategy adds impl bodies rather than new fact-slots. I did not use `48`'s vocabulary and did not
know the question had been named; I offer this as one instance toward it rather than as a settled answer,
since `48` is explicit that this item wants an expert dispatch of its own, and one persona checkpoint's
redirect plus one independent cold answer is not that dispatch.

**The trait-decomposition mechanism (name the schema as a trait, fill it per `(Strategy, Sign, N)` impl,
never inline the arithmetic in a bound) is exactly what the panel converged on independently, and my
compiled evidence adds a small, real thing theirs does not: validation as a missing-impl refusal.**
`16_probes/p6_trait_form_recovers_both.rs`, `45_probes/p5_third_output_is_mechanically_free.rs`, and
`47_probes/p1_single_type_output.rs` all use the same shape I used in `49_probes/p1_fact_schema.rs`,
independently arrived at four separate times now (theirs, and mine before I had read theirs). My own
contribution the panel had not stated explicitly: `49_probes/p2_validation_is_a_missing_impl.rs` shows
that asking for a fact at an unimplemented `(Strategy, Sign, N)` combination is refused by ordinary trait
resolution (`E0277`, not a panicking const-eval, not a runtime check), which is the acceptance criterion's
"it validates" clause falling out of the mechanism for free. Nobody in `15`, `16`, `44`, `45`, `46`, `47`,
or `48` states this explicitly as a distinct clause of the acceptance criterion answered; they are
occupied with the derivation's outputs (the "container and numeral representations" clause) rather than
its validation clause. I did not find this contradicted anywhere and I did not find it stated either.

### Where my own evidence has exactly the blindness the panel names, and I did not catch it myself

**My erasure probe (`49_probes/p3_erasure_check.rs`) is a scalar check, and `16` and `17` (cited by `16`)
establish precisely that a scalar check is structurally blind to whether the second output (stride,
packing) actually holds.** `16:518-521`: "the erasure and codegen-equality check... its method is
comparing one operation against one native instruction, so its instrument is a scalar and it has no array
in it and cannot have one." My `p3` does exactly this: three functions, each moving one value between two
representations, checked at `-O3` against emitted assembly. It genuinely establishes that per-value
conversions between my Storage and Operand forms erase to bare masks and moves, which is real and I stand
by it as far as it goes. It establishes nothing about whether an *aggregate* of Cold-strategy values
actually achieves the packed stride I claimed for it, because (per the correction two sections up) my
probe never built a sequence at all. Had I read `16` first, I would have known to build a second, array-
level check (`16`'s own `p3_blind_suite.rs` is exactly the shape: a packed round-trip at a nonzero bit
phase, with data that fills the declared width rather than small counter values, since `16` section 7 also
found the array-level check itself is blind on small test data). I did not build one, and I am naming the
gap rather than filling it now, since filling it would mean re-deriving `16`'s own probe rather than adding
anything.

### The kind boundary (type versus const) is the sharper form of a distinction I used but did not name

`47` names precisely why the trait-decomposition mechanism (used by all of us) works and why the naive
"recover one fact from another via inline const arithmetic" route does not: a `type -> const` projection
is total and gate-free (any associated type can yield a `size_of`/`align_of` for free), while a
`const -> type` projection is refused, because reaching a type from an arithmetic expression on a generic
is exactly what `generic_const_exprs` gates, and that feature is forbidden (`47` section 2.2-2.3, six
independent compiled refusals across `47`, `45`, and `16`'s probes, all naming the same forbidden feature
from different starting points). My own phase-one derivation used this distinction implicitly (I put
`Storage`/`Operand` as associated types and `ALIGN`/`WIDTH`/`PACKED` as associated consts, which is exactly
sorting facts by which side of the kind boundary they belong on) but never stated the boundary as the
reason. `47`'s naming of it, and `48`'s further sharpening (a fact belongs in the result when a downstream
site would otherwise have to re-derive the strategy's own rule, not merely when it happens to be
unrecoverable in principle, `48` section 5) is a better statement of what I was doing by instinct than
anything I wrote in phase one. I would adopt `48`'s "re-derivation" test over my own "consumer did not
write it / machine needs it / cannot be recovered" criterion (borrowed unknowingly from `16:100-101`,
which I never read until phase two, since I derived a nearly identical three-clause test independently:
compare my phase-one "A quantity earns a place... when getting its value requires consulting the strategy
as an actual decision... and when an entity other than the numeral itself... needs the answer and cannot
safely re-derive it" against `16:100-101`'s "the consumer did not write it, the machine needs it, and a
downstream site... cannot recover it." These are close enough in shape that `48`'s critique of `16`'s
criterion, that it is applied two different ways in the same file and one of those ways kills the finding
(`48` section 5), likely applies to mine as well, and I did not catch it any more than `16` did.

### Did reading the panel change my answer

**Yes, in the specific places named above, and the change is a narrowing, not a reversal.** The count goes
from six facts to closer to the panel's two-required-plus-one-contingent (carrier/storage, stride, and a
contingent compute/operand carrier), with width and sign correctly kept as inputs the type already carries
rather than derivation outputs, matching `16`'s own self-correction (`16` section 10.1) that I arrived at
independently and then, on reading, found already made. My alignment fact was over-counted and recomputes
from the carrier. My packing-model fact was under-specified (a boolean where a bit count is the real
answer) and the panel's stride is the correct form of it. My Cold-diverges-from-Warm-in-Storage-type claim
does not survive contact with two independently-arrived-at panel findings and I would redo the probe.

What did not change: the underlying mechanism (name the schema as a trait, fill values per strategy
through ordinary impls, let validation fall out of missing-impl refusal, let erasure fall out of
monomorphisation), the observation that the schema's shape is strategy-independent while the values are
not (which answers, independently, one of `48`'s named open items for the next dispatch), and the
compiled evidence that this is expressible with zero forbidden features throughout. That the six-fact
enumeration and the Cold-storage-divergence claim needed correcting, while the mechanism did not, is
itself informative: a cold derivation from the intent statements alone gets the *shape of the answer*
right and gets *specific instantiations* wrong in exactly the places where only compiled, cross-checked,
multiply-attacked evidence (four files' worth, on this narrow a question) settles it. That is the result
this dispatch exists to produce, and it argues for the panel's own standing discipline (derive, then
attack, then attack the attack) over any single pass, cold or not.
