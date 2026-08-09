# 73. The byte image: two crossings nested outside the ratified one, and one of them does not exist per value

Hans-Kristian Arntzen, file 73. I wrote files 31 and 42. File 42 found a perimeter hole two layers
below where a predecessor had placed it (the seal covered the constructors and missed a public field
that reconstructed the same illegal state); a later file found a hole one layer below that again. I
hold this file to the same standard a byte image demands more than any other surface in this design:
a guarantee about what a value looks like in memory is worthless if it only covers the API someone
remembered to gate, because a byte image is observable by definition, through mechanisms this design
does not control.

**What I read.** `68_consolidation_seven.md` in full, `72_giesen_the_unexamined_ground.md` in full
(the dispatch's stated basis), the lead designer's `68b_op_checkpoint_sixteen.md` and
`70b_op_checkpoint_seventeen.md` in full, both required. `69_ringer_the_source_justification_sweep.md`,
`70_wronski_the_presets_re_derived.md` and `71_smith_the_far_point_without_infinity.md`, all read in
full rather than skimmed, because the preset re-derivation settles `StoredWidth` and `Layout` per
preset, and both are load-bearing for what a byte image contains. One `ls` of the panel directory
before starting: files `00` through `72` plus checkpoints and probe directories, nothing after `72`.
Targeted reads driven by grep rather than memory: every panel occurrence of `StoredWidth`, `Layout`,
`StorageLayout`, `Encoding::Fields`, `repr(transparent)`, `cohort`, `padding`, `endianness`, `byte
order`; `54_kiselyov_the_type_level_float_and_decimal.md` in full for the cohort census and
`Encoding::Canonical`'s existing shape; `67_rompf_what_the_layers_key_on.md` sections on `Crosses` and
the layer-keying rule's four checks; `32_aaltonen_does_identity_lower_well.md` section 4 for the one
existing measurement of bitpacked field extraction, which turns out to model a different packing
discipline than the one my own probe needed to test, and the gap between the two is one of this
file's findings. Tree reads, evidence only, never design meaning, checked against the panel's
`tree-fact`/`tree-meaning` split ratified at `70b`: `mock/crates/arvo-storage/src/bits.rs` for the
`repr(transparent)` declaration, `mock/crates/arvo-strategy/src/width.rs` for `Width`'s shape,
`~/Dev/clause-dev/.claude/rules/what-you-can-observe-is-what-you-guaranteed.md` for the perimeter
rule this file leans on hardest.

**Gates.** Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate, summed per binary rather than trusted from a headline: `cargo test --offline --workspace`
from `mock/`, 658 passed, 0 failed, 9 ignored, matching `68:64-65` and every consolidation since.
Toolchain, confirmed two ways in this session: `rustc --version` from inside the repo tree resolves
to `1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, matching `rust-toolchain.toml`;
the identical command run from `/tmp`, outside the repo tree, resolves to `1.94.0 (4a4ef493e
2026-03-02)` **stable**, confirming the dispatch's own warning that a bare `rustc` outside this tree
is not the pin. Every probe below was compiled from inside `73_probes/`, inside the repo, and every
compile in this file's evidence trail used the pinned nightly, verified by the `rustc --version`
check immediately before the compile runs recorded in `73_probes/OUTCOMES.md`.

**What is compiled, what is reasoned, what is tree-fact, what is open.** Three probes in
`73_probes/`, four files (`probe_1` and its expected-fail companion `probe_1b`, `probe_2`, `probe_3`),
compiled and run fresh this session, `--edition 2024`, no `#![feature(...)]` line in any of them,
outcomes verbatim in `73_probes/OUTCOMES.md`. Two arguments below are proofs rather than
measurements (the padding-purity argument in section 4, the byte-sharing law in section 5's general
form): the compiled probe is a witness confirming the proof holds on one instance, not the basis of
the claim, and I say which is which at each step. Method per `70b:42-50`, applied literally: no
shipped doc comment is cited as design justification anywhere below, and every claim was checked
against the deletion test before it entered.

## 1. The question, answered first: two crossings, not one, and one of them is not a crossing at all

File 72 asked whether the byte boundary is a third coordinate system or the physical one viewed
differently, and left it named rather than answered (`72:107-114`, "quantified over a boundary the
review has not yet pointed them at"). It is neither, cleanly. **The byte image is the ratified
crossing contract's own datum, `D`, run through two further structural maps that the design has not
yet named, and the two maps are not the same shape as each other or as the one they extend.**

`D` (`63:179-181`, `68:204-207`) is the space of physical field values `Encoding::Fields` recognises,
a numeral's own logical bit pattern. It is not yet a byte-addressable object: nothing in the crossing
contract's statements (`68:183-274`) says how wide `D`'s carrier is, whether the carrier holds
anything `D` does not, or how the carrier's bits land on octets in memory. Those are three separate
facts, decided by three separate axes already sitting on `Lowering` (`Encoding`, `StoredWidth`,
`Layout`, `68:568-575`), and the byte image sits downstream of all three at once:

1. **`embed : D -> Carrier`**, where `Carrier` is a bit pattern of exactly `StoredWidth` bits. This
   map is a genuine crossing in the ratified sense, and it takes the crossing contract's statement
   structure almost verbatim: a round-trip statement on `D`, a canonicalisation statement on
   `Carrier`, and a derived injectivity boolean. Section 4 works this out and finds the design has
   less freedom here than file 72 suggested; the canonicalisation statement is not a policy choice, it
   is forced.
2. **`materialise : Carrier -> Bytes`**, laying the carrier's bits onto octets. For a `Carrier` that
   owns its own contiguous storage (every `Layout::Dense` numeral, at any `StoredWidth`), this is a
   pure relabelling: a bijection between bit positions and byte-and-offset pairs, decided once by a
   target's native representation, needing no statement-0 of its own because every bit pattern the
   carrier can hold has some byte materialisation and vice versa. **For a `Layout::Bitpacked` numeral
   packed with zero inter-value padding, this map does not exist at the per-value granularity at
   all.** Section 5 compiles why, and the finding sharpens what "bitpacked" has been assumed to mean
   across the whole review so far.

Nothing above is a fourth coordinate system alongside value, datum and whatever byte was going to be
called. It is the general form file 72 already stated correctly (`72:107-114`, a crossing or a
projection, reused rather than invented) applied to two boundaries in sequence rather than one, and
the second of the two boundaries turns out to be well-formed only for one of the two `Layout` values.
*Grounded on: settled shapes (`68:175-181`, `68:204-207`, `68:568-575`), reasoned (the two-map
decomposition).*

## 2. Where this sits against the crossing contract's own precondition, and why it does not repeat the ill-typed mistake

File 67 found that the ratified crossing contract's statements 2 and 3 are ill-typed without a
precondition (statement 0: `decode`'s output must lie in `encode`'s domain, `68:183-274`). The same
question has to be asked here before either new map gets its own statements: is `embed`'s codomain
well-defined without a precondition on `D`?

It is, and for a reason worth stating plainly because it is the one place this file's boundary is
strictly easier than the one it extends. `encode : V -> D` crosses from mathematics into physical
bits, and `V` is a proper subset of what `D` can hold (that is the entire content of statement 0's
gap). `embed : D -> Carrier` crosses from one physical bit-width to a wider or equal one, and every
element of `D`, by `Encoding::Fields`' own definition, is already a legal physical field value; there
is no analogous "is this datum even a real datum" question at this boundary, because `D` is defined
to be exactly the domain `embed` needs to cover. **The only open item this file inherits rather than
resolves is whether `Encoding::Fields` itself ever declares a non-full domain** (some bit pattern that
`Encoding::Fields` reserves as not a datum at all, distinct from a datum that fails to denote a
value). I found no instance of this in the corpus and no member has needed it so far; I flag it
rather than assume it, because assuming a domain is full without checking every impl is exactly the
kind of universal claim file 65's sketch got burned on (`68:669-684`). *Reasoned, flagged rather than
resolved.*

## 3. The two identity notions the layer-keying rule already has gain a third, and it is the finest one yet

`68:116-141`'s layer-keying rule collapsed a wrongly-inherited three-layer framing to two: face
identity (expansion-time, syntactic) and encoding-equals-value identity (type-check-time,
structural). File 72 correctly completed the rule's display clause for the runtime case (`72:116-127`,
a computed value has no face, so display splits into value-keyed and datum-keyed images). Neither of
those two files needed a third identity notion because neither reached below the datum.

**Carrier identity is a genuine third notion, and it is strictly finer than datum identity, not
coarser.** Two carriers can differ (in their padding bits) while denoting the same datum; `72_probes/
probe_4` already compiled exactly this (`72:296-301`, two `u16` carriers agreeing on all thirteen
datum bits, an FNV-1a digest over the raw carrier separating them). What that probe did not need to
say, because its subject was digests rather than the byte image itself, is that this makes carrier
identity the finest of the three: face (syntax) and encoding-equals-value (type-check structure) sit
above the datum in the type system's own binding times; carrier identity sits below the datum, at
runtime, and distinguishes things the datum layer is defined to treat as one. The layer-keying rule's
own test ("coarsest layer whose identity a fact depends on") answers immediately what carrier identity
is *for*: **almost nothing should ever be keyed on it.** A fact that depends on carrier identity
rather than datum identity is, per the rule's own standard (`68:126-127`), a false statement about the
numbers, because the padding bits carry no denotational content by construction (section 4). The one
fact that legitimately does depend on carrier identity is the byte image itself, which is this file's
subject and the reason the third notion needs naming at all: without it, "what does this value look
like as bytes" has no layer to be an honest question about.

*Grounded on: settled shapes (`68:116-141`), compiled (`72_probes/probe_4`, corroborating rather than
establishing, since the finding there was about digests), reasoned (the third-notion naming and its
fineness relative to the other two).*

## 4. The padding statement is not a policy choice between two coherent options. It is forced, and I found the argument file 72 did not have

File 72 offered canonical-at-rest padding as a suggestion with a cost stated, leaving
declared-don't-care as the alternative on the table (`72:241-251`, "offered as a suggestion with its
cost stated"). Stress-testing both, I find they are not two coherent options for the same operation.
They are the correct answers to two *different* operations, and only one of those operations is what
a constructor does.

**Construction is a pure function of the datum, by the shape of `From`, and a pure function cannot
express "preserve whatever was already there" because there is nothing already there.**
`embed : D -> Carrier` is, wherever the design ships it as `From<D> for Carrier` or as the tower's own
equivalent, a one-argument operation: same datum in, same carrier out, every time, with no access to
any prior state. "Declared-don't-care" as a semantics for *this* operation would mean the padding
bits are unspecified output of a deterministic function, which is not a policy, it is a category
error: a pure function has no source of variation to leave unspecified from. **Compiled**
(`73_probes/probe_2`): a `From<u16> for Carrier` impl that zero-pads is checked to be pure (two calls
on the same datum produce bit-identical carriers). A second operation,
`embed_preserving_padding(old: Carrier, new_datum: D) -> Carrier`, is written to show what
"preserve existing padding" actually requires, and it requires the old carrier as a second argument.
That is not the same operation with a different policy; it is a different, strictly more general
operation (an *update*, not a construction), and no `From` impl can carry it, because `From` has one
parameter.

**This closes the design question file 72 left open, and closes it as a law rather than a switch.**
Every tower-generated constructor of a carrier from a datum canonicalises its padding to a fixed
value (zero is the obvious and cheapest choice; nothing in the argument above depends on the value
being zero specifically, only on it being fixed and independent of any prior carrier). No new axis is
needed on `Encoding` or `Lowering` for this: it is not a per-numeral choice, it is a structural fact
about what a one-argument pure function can and cannot depend on, true of every construction path the
tower will ever generate. What the design does need, and what section 6 below states, is a
declaration-time obligation on the one place this law cannot reach: a carrier that arrives from
*outside* the tower, where nothing enforced canonical padding on the way in.

**The perimeter argument sharpens why this matters more than a cost tradeoff.** `Bits<N, S, Sign>` is
`repr(transparent)` (tree-fact, `mock/crates/arvo-storage/src/bits.rs:56`, cited for the fact of the
attribute existing, not for what any comment says it means), and per this workspace's own perimeter
rule, a `repr(transparent)` type's byte layout is observable to *any* consumer holding the value,
through a bit-cast or a raw pointer read, whether or not arvo ever ships a `to_bytes()` method at
all. `73_probes/probe_2`'s second half compiles this directly: a `transmute` sees exactly the bytes
`From<u16>` committed to, with zero dependence on any declared API. So the padding policy is not a
convenience decided at the point some future method gets written; it is decided, permanently, the
moment the constructor is, because the perimeter through which it will be observed is not a door
arvo controls the existence of. A "declared-don't-care" policy, even if it were coherent for
construction (it is not), would mean every unsafe reader in this stack, including `arvo-hash`'s own
digest computations over raw bytes, inherits undefined content it cannot mask away without first
knowing where the datum bits end, which is exactly the defect `72_probes/probe_4` already showed live
for a *foreign* carrier and which a tower-generated one now cannot exhibit at all.

*Grounded on: compiled (`73_probes/probe_2`, both halves), tree-fact (`bits.rs:56`, cited as evidence
of the attribute, deletion-test-survives: the perimeter argument holds for any `repr(transparent)`
newtype regardless of which one this design ships), settled shapes (`72:241-251` as the question being
resolved), reasoned (the purity argument, which is the load-bearing step and is not itself a claim
about the shipped tree).*

## 5. Bitpacked storage has been carrying two different meanings, and only one of them has a per-value byte image

File 32's own measurement of bitpacked field extraction (`32:207-230`) models four sixteen-bit slots
packed into a sixty-four-bit word, "no padding" at the word level, extracting a thirteen-bit logical
value from each slot. Read closely, that model has padding: each field's own thirteen live bits sit
inside a sixteen-bit-aligned slot, with three padding bits per slot, exactly section 4's case, and
every field's byte image is independently addressable because slot boundaries land on byte
boundaries.

**That is not the only way to read "stores as small as possible", and the other reading has no
per-value byte image.** If `Layout::Bitpacked` means what its name most directly suggests, packing
values with *zero* inter-value padding rather than rounding each one up to a byte- or slot-aligned
width, the per-value byte image stops existing as a well-formed request. `73_probes/probe_3` models
four thirteen-bit fields packed contiguously into one sixty-four-bit word, the densest packing that
still fits arvo's own bitfield examples' shape (`32:210-211`), and checks every byte of the word
against which logical fields' bits it contains: three of the word's eight bytes are shared between
two adjacent fields, and the compiled reason generalises past this one instance rather than sampling
it. **The general law**: byte boundaries occur at every multiple of eight bits; a packed field of
width `W` occupies `[i*W, (i+1)*W - 1]`; a byte is shared between two fields exactly when some
multiple of eight falls strictly inside a field's range, which happens for some field boundary in any
packing of two or more same-width fields whenever `W mod 8 != 0`. Thirteen does not divide eight;
sixteen does. That is the entire difference between file 32's model and mine, and it is why one has
independent field bytes and the other does not.

**This is a real, currently-unstated ambiguity in what `Layout::Bitpacked` promises, and it has
opposite consequences for the byte image depending on which reading the design settles on.** Under
the byte-aligned-slot reading (file 32's), every value has its own byte image, `embed` and
`materialise` compose exactly as section 1 describes, and the only obligation is section 4's padding
law. Under the zero-inter-value-padding reading, `materialise` for a single value **does not exist**;
only the whole packed word, or more usefully the whole column, has a byte image, and any claim this
design makes about "the byte image of a `Number<N, S>`" needs a `Layout`-conditional scope: per-value
under `Layout::Dense` (and under a byte-aligned bitpacked reading), per-column under a densely-packed
`Layout::Bitpacked`. Given `arvo-toolbox-not-policer.md`'s own framing of `Cold` as the reason arvo
exists, and given `Cold`'s own re-derived intent this stretch, "stores as small as possible" literally
(`70:174-178`), the dense reading is the one that reading points toward, but I state it as a lean and
not a ruling, exactly the discipline the review has used for every other open cell this stretch.
Whichever reading locks, the spec sentence has to say so explicitly, because the two readings are not
a cost tradeoff on one shared byte-image guarantee, they are two different claims about whether a
per-value byte image is a coherent object at all.

*Grounded on: settled shapes (`32:207-230`, read closely rather than taken at its own summary),
compiled (`73_probes/probe_3`, the model instance and the general law it witnesses), tree-fact
(arvo's own container dispatch maps every `N <= 128` numeral to a native primitive per the strategy,
which is why file 32's slot-aligned reading is the one the shipped container table happens to
implement today, cited as evidence the ambiguity is live rather than hypothetical, not as design
meaning), reasoned (the general divisibility law, the `Cold`-intent lean).*

## 6. The declaration-time obligation, and why it is one widened trait rather than a second one

File 72 named the shape correctly and left the fork open: "whether it is a second trait beside
`Crosses` or a widening of `Crosses` to name its byte side is a shape call I leave open with both
spellings on the table" (`72:261-263`). I take the fork, on the strength of section 4's finding.

**Widen `Crosses`, do not add a sibling.** `Crosses<N: Numeral>: Lowering`'s existing safety
condition is a declaration-time obligation on whoever brings a hand-laid `Lowering` into the tower:
for every datum this encoding can hold, `decode` lands in `V(N)` (`68:250-274`). Section 4 established
that the tower's own constructors need no such obligation for padding, because purity forces the
answer; the obligation is needed only where a `Lowering` impl is `unsafe impl`, i.e. exactly the same
population `Crosses` already exists to police. Adding a second trait would ask a foreign-format
author to make two separate unsafe declarations at two separate places for two conditions that arise
from the identical fact about their impl (that it is hand-laid rather than tower-generated), which is
more ceremony for the same population `Crosses` already reaches, not more precision. The widened
condition:

```rust
// Statement 0 (unchanged, `68:262`): for every datum d, decode(d) is in V(N).
// Statement P (new): for every carrier c this Lowering can produce, the bits
// outside Encoding::Fields' width are exactly the padding this Lowering
// declares (canonical-zero, or whatever fixed value a future axis names);
// section 4's purity argument is why the tower's own generated impls satisfy
// this for free, and why an unsafe impl is where the obligation actually
// bites.
pub unsafe trait Crosses<N: Numeral>: Lowering { }
```

Where `embed` is one the tower generates, the impl is blanket and safe, per D16, identically to
statement 0 today. Where a consumer hand-lays a carrier's field layout, both conditions are theirs to
state at the one declaration site, which is the design's own stated preference for binding time
(`68:250-274`, `72:257-263`). Rewrite cost against the shipped tree is zero, matching every other
crossing-contract instance this stretch (`68:271-272`): no shipped source names `Crosses` at all.

**The shape-check obligation at the byte layer is a third, and different, kind of precondition, and
it does not fit inside `Crosses` at all.** Whether a raw byte buffer is even long enough, and whether
its bit-packing convention matches what this build's `Layout` and `materialise` map assume, is a
question about *arity*, checked before any value-membership or padding question is reachable, not a
statement quantified over `N: Numeral` the way `Crosses` is. This is file 01's orphaned boundary
(`01:332`) reached a second time, and file 72 already generalised it once (`72:253-263`); what I add
is that the byte layer's version is not the same shape as the datum layer's version, because a wrong
byte count is not a value that fails to denote, it is not a value at all yet. I name this rather than
design it: a build layer (or an explicit `TryFrom<&[u8]>`-shaped constructor the tower generates)
owns the length check, and the design's obligation is only to make the expected length a const,
derivable quantity, which section 7 shows it already can.

*Grounded on: settled shapes (`68:250-274`, `72:253-263`), reasoned (widening over a sibling trait,
the arity-versus-membership distinction).*

## 7. The byte-count is the eleventh firing of the spine rule, compiled, gate-free

`68:98-101`'s spine rule (a computed quantity that has to appear in a type is a type) has fired ten
times by the consolidation's own count, the tenth being file 72's `ShortCap` for the print buffer
(`72:212-224`). A numeral's byte width is exactly the same shape of quantity: computed from
`StoredWidth`, has to size an array (sizes const at type level, no alloc, per the workspace's own
standing constraint). **Compiled, both halves.** `73_probes/probe_1b` refuses under the const-
expression route with the identical error shape file 72's `probe_3b` reproduced for the text buffer:
`generic parameters may not be used in const operations`, rustc's own help text naming the forbidden
`generic_const_exprs`. `73_probes/probe_1` ships the fix in the rule's own shape, an associated
`type ByteCap: ByteCapacity` with a declaration-site coverage check
(`const _: () = assert!(C4::BYTES >= byte_width_of(N::STORED_WIDTH_BITS))`), gate-free,
`--edition 2024`, no `#![feature(...)]` line, and, per the brief's own "everything wants a
const-callable form" constraint, `zero_bytes::<N>()` is itself `const fn` and is exercised in a
`const` context (`_CHECK_13`, `_CHECK_64`), which `ShortCap`'s own probe did not need to demonstrate
since the print kernel's const-callability was checked separately (`72_probes/probe_5`). This is the
eleventh firing, not a new rule; I record the count because the consolidation's own convention
(`68:98-101`) is to keep it, and because eleven independent firings of the identical rule across
unrelated quantities (grade projections, notation faces, seal witnesses, and now a byte-width) is
itself evidence the rule is a property of this design's shape rather than a coincidence noticed
repeatedly.

*Grounded on: settled shapes (`68:98-101`), compiled (`73_probes/probe_1`, `probe_1b`).*

## 8. What arvo can promise on its own, and what it needs from a layer it does not ship

**Scoped honestly, and the scope follows from a call already made rather than from a new one.** Op's
ratified `Warm` intent, "works and behaves as f32 and f64 etc in rust today without any framework on
top of it" (`68b:62-67`), is itself a same-process claim: a plain `f32` gives no cross-target byte-
order guarantee either, because Rust's native integer and float types materialise in whatever the
compilation target's own native representation is, silently, with no framework on top. **Arvo's byte
image, by the identical logic that already settled `Warm`'s door, is a same-process, same-build-target
guarantee, not a wire format.** `materialise` needs no `Lowering::ByteOrder` axis to make this true:
the native representation is a target fact, decided once per compile the way `HostImplemented` already
decides which float operations reach hardware (`59:161-173`, `70:208-231`), and it costs the design
nothing new because nothing chooses it: it is whatever `#[repr(transparent)]` over the chosen native
container already gives, per target, unconditionally.

**What this design cannot promise, and states plainly rather than smoothing over, is portability
across processes, targets, or time.** A column written on one target and read on another, or a format
persisted and read back after a byte-order-differing rebuild, is outside every guarantee this file
states, and inventing a byte-order axis to cover it would be exactly the kind of thing this design
cannot verify on its own: byte order at the wire is a *deployment* fact, structurally identical to the
float control-register fact `HostFloat<E: FloatEnv>` already declines to make arvo's problem
(`59` section 2.5, `70:461-478`). **This is a downstream-contract item, in the `16c_op_the_downstream_contract.md` sense
(the design states what it needs back, not how to build it): a transport or
persistence layer that wants cross-target portability needs the format's identity (radix, precision,
exponent form and bounds, domain, `Specials`, `Underflow`, plus `StoredWidth` and `Layout`) to travel
with the bytes or be agreed out of band, and every one of those is already a closed, sealed, const-
derivable bundle of type parameters (`68:549-587`), not a registry. File 72 already flagged this as
owed with no mechanism proposed (`72:272-278`); I add the reason arvo does not need to solve it
itself, rather than merely restating that it should be solved somewhere: the same `HostFloat<E>`
precedent that already scoped the design's control-register promise scopes its byte-order promise
identically, for the identical reason, and inventing a second mechanism to do the same job the first
one already does would be the toolbox-not-policer failure this workspace's own rule names, guessing at
a deployment fact the design cannot know.

*Grounded on: ratified (`68b:62-67`), settled shapes (`59` section 2.5, `70:461-478`, `68:549-587`),
tree-fact (zero occurrences of `to_le_bytes`/`to_be_bytes` as a designed axis anywhere in the shipped
tree beyond one incidental internal call, `mock/crates/arvo-bits-contracts/src/narrow_from.rs:104`,
cited as corroborating evidence that no byte-order axis exists today, not as a reason one should),
reasoned (the same-process scoping, the downstream-contract framing).*

## 9. The value-unique mirror, answered in full: yes, at every layer, and the design already owns the pattern that explains it

File 42's own question, generalised by this dispatch's brief: can two byte images denote one value?
**Yes, and the answer recurs at every one of the three boundaries this file names, each time as the
identical pattern, a canonical representative chosen from a non-trivial fibre, which this design has
now used four times without ever stating it as one thing.**

- `V -> D`: cohorts. A decimal numeral's unnormalised significand gives more than one datum per value
  (`54:281-330`, the census: zero's cohort spans the whole exponent range times both signs, ten
  members at the model instance). `Encoding::Canonical` picks the representative. Ratified, shipped
  in intent, not in source.
- `D -> Carrier`: padding. Section 4 of this file. Multiple carriers per datum whenever `StoredWidth`
  exceeds the datum's own width, which per section 5's finding is almost always, under every `Layout`
  this review has examined so far except the exact case where the logical width happens to be a
  multiple of eight. Canonical-zero picks the representative, forced rather than chosen.
- `Carrier -> Bytes`: nothing, under `Layout::Dense` (section 1's pure relabelling has no fibre to
  collapse; it is a bijection, not a many-to-one map). This is worth stating precisely because it is
  the one boundary in the chain that is *not* an instance of the pattern, and saying so forecloses a
  fourth `Canonical`-style axis nobody needs.
- Value equality against the digest: the layer-keying rule's fourth instance, already found by file
  72 (`72:284-308`), a projection rather than a crossing, but the same shape again: many data collapse
  to one hash key through the layer's own canonicalising projection.

**The design has one mechanism for this pattern, used four times, and I name it rather than let it
keep arriving as a surprise per boundary.** Every layer boundary in this tower that is a many-to-one
map (not every boundary is; `Carrier -> Bytes` under `Layout::Dense` is the standing counterexample)
owes exactly one canonicalising projection, established once, at the layer where the redundancy is
introduced, consumed by every downstream consumer through that projection and no other door. That is
`Encoding::Canonical` at the value/datum boundary, section 4's forced padding law at the datum/carrier
boundary, and the layer-keying rule's own general statement (`68:138-139`) at the equality/digest
boundary, three instances of one pattern rather than three separate design decisions, which is the
same kind of consolidation the layer-keying rule itself performed on `TotalOrd`, the spectral defect
and the notation face (`68:135-141`).

*Grounded on: settled shapes (`54:281-330`, `68:135-141`, `72:284-308`), reasoned (section 4's own
result restated as the second instance, the pattern-naming across all three).*

## 10. Costs and binding times

Every mechanism above resolves at monomorphisation. `ByteCap` (section 7) is a type, chosen at the
call site through the numeral's own `Lowering`, no `dyn`, no `TypeId`. The widened `Crosses` (section
6) is a trait bound, checked at compile time, with the `unsafe impl` obligation living at the
declaration site of a hand-laid format, exactly `Crosses`'s existing binding time. The padding law
(section 4) has zero runtime cost for the tower's own constructors: it is a fact about what the
generated code already does (write a fixed value into bits nothing else touches), not an added
check, and `73_probes/probe_2`'s compiled constructor performs exactly one masking operation, the
same operation `72_probes/probe_4`'s digest-side fix already needed, so no new runtime work is
introduced anywhere this file's mechanisms touch that the review has not already priced once. The
byte-sharing finding (section 5) has no runtime cost of its own; it is a statement about which
requests are well-formed, resolved entirely by which `Layout` reading the design locks, a compile-time
fact once locked. No performance number in this file is a runtime claim about shipped code; all six
compiles finished under 0.2 seconds each, `aarch64-apple-darwin`, and those numbers are the probes'
own cost, recorded so a future member can budget re-running them, not a claim about the mechanisms at
real widths, per the review's own standing distinction between compile-and-run probe cost and shipped
runtime cost.

## 11. What this hands forward

**For the next consolidation, the chapter's spine in provenance form.** *A value's byte image is the
ratified crossing contract's datum, `D`, carried through two further structural maps, `embed : D ->
Carrier` and `materialise : Carrier -> Bytes`. `embed` is a crossing needing no precondition of its
own, since `Encoding::Fields` already defines `D` to be exactly `embed`'s domain; its canonicalisation
is forced rather than chosen, because a one-argument pure constructor cannot express "preserve
existing padding", and the design's own `repr(transparent)` perimeter means whichever padding a
constructor commits to is what every consumer, safe API or not, will observe. `materialise` is a pure
relabelling under `Layout::Dense`, needing no crossing statements at all, and does not exist at the
per-value granularity under a `Layout::Bitpacked` reading that packs values with zero inter-value
padding, which is a real, currently unstated ambiguity in what `Layout::Bitpacked` means: file 32's
own model and this file's each answer differently, and the spec must pick one. The declaration-time
obligation on a foreign, hand-laid `Lowering` is a widening of `Crosses`, not a second trait, carrying
both the value-membership condition and the padding condition at the same site a foreign-format
author already owns. A raw byte buffer's shape (length, packing convention) is a third, arity-shaped
precondition distinct from both, owned by whatever build layer or constructor accepts foreign bytes.
Arvo's own byte-image guarantee is scoped to one process, one build target, by the identical logic
that already scoped `Warm`'s hardware door; cross-target portability is a downstream-contract item, a
format descriptor the design can derive but does not itself transport. The value-unique tower's
guarantee mirrors down through every many-to-one boundary as the identical canonicalising-projection
pattern, four instances now named as one mechanism rather than four separate findings.* (Grounded:
ratified `68b:62-67`; settled shapes `68:175-274`, `54:281-330`, `72:107-308`, `32:207-230`; compiled
`73_probes/` all four; tree-fact `bits.rs:56`, `width.rs`, `narrow_from.rs:104`.)

**Open, stated rather than resolved.** Whether `Layout::Bitpacked` locks to the byte-aligned-slot
reading or the zero-inter-value-padding reading (section 5); the choice is a design call with opposite
byte-image consequences and I state a lean, not a ruling. Whether `Encoding::Fields` ever declares a
non-full domain for some impl the review has not yet built (section 2); I found no instance, and did
not prove there is none. The exact fixed padding value (I use zero throughout because it is the
obvious and cheapest choice; the purity argument forces *a* fixed value, not that specific one). The
format-descriptor's own concrete shape, named as owed by file 72 and again here, with no mechanism
proposed by either.

**Flagged for whoever next builds the byte-serialisation stubs.** `Crosses`'s widened statement-P
needs its second read before it hardens, the same discipline every crossing-contract instance this
stretch has carried (`68:250-274`, `67b`'s own note on the base trait). The exact-expansion capacity
question file 72 raised for text (`72:225-226`) has a byte-side sibling once `Layout::Bitpacked`'s
reading locks: a packed column's own byte capacity is a function of the whole column's field count and
width, not of one value's `ByteCap`, and needs its own associated quantity once the design has
somewhere to put a column-shaped capacity.

## 12. Table-diff self-check and verification

Every citation to `68`, `70`, `71`, `72`, `54`, `32` and `59` above was checked against the source
line at the moment of writing, not against a memory of a prior read; the two places I quote a probe's
own numbers (`72_probes/probe_4`'s hex digests, `32:207-230`'s slot width) were checked against the
cited file rather than restated from summary. The canon gate, test gate, and toolchain check were
reproduced fresh at the top of this document, including the outside-the-tree stable-toolchain check
the dispatch specifically asked for. All four probes in `73_probes/` compiled and ran this session,
inside the repo tree, under the confirmed pinned nightly; `73_probes/OUTCOMES.md` carries the verbatim
output, including the one expected-fail (`probe_1b`) with its error text reproduced in full. Every
design conclusion in this file survives deleting its tree citations: the padding-purity argument
(section 4) is a fact about what a one-argument function can depend on, not about arvo's shipped
constructors; the byte-sharing law (section 5) is a fact about arithmetic on bit widths, witnessed by
one packing but true of the whole divisibility class; the same-process scoping (section 8) follows
from op's own ratified `Warm` intent, not from any doc comment. I checked this sentence by sentence
against `69`'s own deletion test before the document stood, the same discipline `72:369-372` records
for itself.
