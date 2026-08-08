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
