# 104. What the bitfield is: a product of numerals under a placement map, whose declared half is the only half that owes anything, and whose crate holds notation rather than substance

Oleg Kiselyov, file 104. I wrote file 02 (the type-level encoding), file 36 (the normal form and its
price), and file 54 (the type-level float and decimal, which overturned one of file 36's own claims by
compiling it). This dispatch sends me to the third and last of the periphery rows file 101 left
content-unread, with the instruction to say which of three things D25 is and, where it stands
untouched, to spend the file on what it does not cover.

D25 stands. Its content is not what its row says, and neither of its two stated grounds carries it as
stated: one is now measurable and points the other way, the other prices at about a tenth of a second.
What carries it is a third thing the design acquired after D25 locked and nobody has connected to it.

The larger answer is that a bitfield is not a distinct mechanism. It is the heterogeneous instance of a
structure the design already ships homogeneously, and every law it needs is one the design already has,
composed once. The one thing it genuinely adds is an obligation, the shipped macro documents that
obligation as the author's, and the cost of that is compiled below.

## What I read

The op-ratified round `mock/design_rounds/202607300800/`, from the topic files rather than from any
compression: `202607291210_topic.bitfield-becomes-its-own-crate.md` in full (D25, 34 lines),
`202607290500_topic.the-placement-calls.md:77` and `202607290600_topic.the-taxonomy-talk.md:29` (the two
places the round left `bitfield!` unplaced before D25 settled it),
`202607291700_topic.storage-decomposes-and-refit-comes-home.md` (D27, because the crate D25's expansion
target lives in is the crate D27 renames), and `202607300700_topic.consolidated-round-state.md:1027-1052`
and `:1129-1145` (the round's own record of the placement question and of D25).

From the panel: `102_consolidation_ten.md` in full as the standing base,
`101_knuth_the_periphery_re_audited.md` in full, `103_leijen_platform_and_the_predicate.md` in full,
`101b_persona_checkpoint_twentyfour.md` in full. Behind them, at the derivations this file needs rather
than at their compressions: `73_arntzen_the_byte_image.md:195-235` (the byte-sharing law and the two
readings of `Bitpacked`), `81_fog_is_the_bitpack_cost_inherent.md:176-244` and `:283-300` (the plan
consts, the binding-time failure, and the decoder table), `83_lattner_how_many_widths.md:255-316` (the
three levels, the bitfield row, and section 6's own level-naming clause),
`91_consolidation_nine.md:530-660` and `:725-770` (the maps, statements P and C, the `Crosses`
declaration, the crate table), `74_lattner_the_taxonomy_rechecked.md:69` and `:265-271` (the only-door
law's fifth site), `61_amin_the_notation_vehicle.md:64-120` and `:184-230` and `:513-627` (the
digit-decomposition wall, the vehicle, and the staging price), `32_aaltonen_does_identity_lower_well.md:
200-220`, `11_current_shape_draft.md:40-60`. One `ls` of the panel directory, current through `103_probes`.
One `ls` of `mock/design_rounds/` root, and I opened what it returned.

From the shipped tree, for the two licensed purposes only: `arvo/src/bitfield.rs` in full (459 lines),
`arvo-bits/tests/bitfield.rs` in full, `arvo/tests/bitfield_const_eq_default.rs` in full. Every design
judgement below survives deleting its shipped-source citation; where a citation is doing more than
saying why a sentence is needed I say so and withdraw the claim.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `5c952a3`, run 2026-08-05 06:41. The governing
material is the op-ratified round `202607300800`, which outranks every panel file including this one;
`102` is the panel's standing base beneath it. Gate passed. One framing note rather than a refusal: the
dispatch offers three outcomes and the honest answer is two of them at once, ratified-and-untouched on
the decision, overtaken on both of the decision's stated grounds, and section 0 says which is which
rather than picking one.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries, 672
passed, 0 failed, 9 ignored**, matching `102` and `103` exactly, from a clean committed tree.

Test bodies read rather than counted, in the surface this file touches. There are exactly two, and
between them they hold every `bitfield!` declaration in the tree.

- `arvo-bits/tests/bitfield.rs`, eight tests, all real: round-trips, a field write that must preserve
  its siblings, a setter truncation, a `from_bits`/`to_bits` round trip, and the emitted masks checked
  against independently written literals. No tautology; nothing asserts a value against itself. One is
  thin (`bits_sizes_match_container` re-checks the container dispatch its own doc comment says is
  covered by `bits.rs`), and thin is not a disqualification.
- `arvo/tests/bitfield_const_eq_default.rs`, three declarations and six const-position pins plus two
  runtime tests, all real, with a negative present (`_STRHANDLE_DIFFERENT_VALUES_NOT_EQ`).

**And the gate obliges me to name what all four declarations have in common.** Fresh grep, `grep -n " at
" ` over both files, 2026-08-05 06:56: `origin: 1 at 31, reserved: 3 at 28, id: 28 at 0`; `flag: 1 at
0`; `kind: 8 at 56, id: 56 at 0`; and the first triple again. **Every declaration in the suite is
pairwise disjoint, and nothing in the macro requires that.** The assertions are real and the path that
breaks is never entered, which is the test gate's own "setup that helps" in its exact shape. It does not
disqualify the suite, which is honest about being smoke tests, and it does mean the one obligation this
file finds unmet has no test in either direction. Section 3 compiles what that costs. Two tests owed,
named in section 8.

The three known tautologies (`arvo-tensor/tests/capacity.rs:14-18`,
`arvo-tensor/tests/const_capacity.rs:49-53`, `arvo-hash/tests/aliases.rs:16-23`) are still present at
source, re-verified this session, still in the green total, still disposed at `95b` as op's own trivial
commit outside the panel.

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, confirmed inside the tree. Probes in `104_probes/` with `OUTCOMES.md`; compiled,
measured and reasoned are tagged per claim throughout.

---

## 0. The answer, first

**What a bitfield is, in the design's current vocabulary, in one sentence the ratifying text can take:**
*a bitfield is a heterogeneous product of numerals sharing one container under a declared placement map,
where a bitpacked column is the homogeneous product of one numeral under a derived one, and the two are
the same structure differing in whether the index is a `Nat` or an hlist and whether the offsets are
derived or declared.*

Everything a bitfield needs the design already has. The maps are `embed`, `place`, `materialise`
unchanged. Statement C governs the container's bits outside the map. The only-door projection governs a
field read, which is `74:69`'s fifth site, already ratified. The byte-sharing law decides which fields
have byte images. The digest chapter's two stopping points apply per field. The group arithmetic
transfers verbatim into a bitpacked column of bitfields, keyed on the element stride, invariant under
every repartition of that stride (compiled at 1,596 shapes, `p7`). **No new mechanism, and I looked for
one.**

**The one thing it adds is an obligation, and it is half-met.** A declared placement map owes two facts
its derived counterpart proves: every field contained in the container, and the fields pairwise
disjoint. The shipped macro asserts the first and documents the second as the author's
(`arvo/src/bitfield.rs:28-30`, "Overlap detection is deferred to a future macro version (for now,
authors are responsible)"). Compiled: an overlapping declaration compiles clean, and writing one field
silently truncates its neighbour, with no `unsafe`, no warning, and no diagnostic of any kind (`p1`).
Both obligations are const-evaluable at declaration over a list a human wrote, and discharging the
second costs an `O(k^2)` loop in a free const item (`p1b`, refuses with `E0080`).

**D25 is ratified and untouched, and both of its stated grounds are overtaken.** Ground one, that the
macro must have its own crate because it may later become a proc-macro crate, rests on the upgrade being
forced. It is not: a bitfield's arithmetic is prefix sums over widths the consumer wrote as separate
tokens, and `macro_rules!` reaches it with **no feature gates at all** (`p4`, exhaustive at 65,536
container values). That is the exact opposite of file 61's compiled result for the notation vehicle,
where the wall is real because a decimal literal is one atomic token, and the contrast is the finding.
Ground two, granularity, prices at about **121 ms**, the floor cost of reaching a proc-macro crate at
all, against a marginal cost per additional macro in the same crate well below it (`p6`), so it does not
distinguish one proc-macro crate from two.

**What carries D25 now is a thing that did not exist when it locked: the crate holds notation, and the
design has since given notation a vehicle, a binding-time discipline and a price.** The bitfield's
substance, the placement map, belongs where `place` belongs, which the panel gave to `arvo-container`
(`91:764`, `83:255-268`). What is left in `arvo-bitfield` is the surface that turns a field list into
that map, which is the same kind of object as `raw_bias!` and `numeral_face!`: a host-side stage that
computes an answer once and emits a type. That is a crate with real content, and its kind question is
the notation vehicle's question rather than the bitfield's.

**Three things the composition with a bitpacked column turns up, none of which any file has looked at.**
The composite of two placements is a placement, exactly and at every shape (`p2`, `p7`, zero
disagreements over the swept space). Which composite to emit is not a property of the layout: reading
one field is identical either way, reading three favours materialising the element once (`p3`), which is
`81:283`'s own conclusion arrived at independently one level in. And under `Bitpacked` with a stride not
a multiple of eight, no field of any element has a per-field byte image, which is file 73's law composed
once and which closes a question rather than opening one.

**And one correction to a ratified sentence, small and exact.** The datum-keyed digest "masks the
container straight to the fields' own width", a prefix mask, which is right for a numeral and wrong for
a placement map with an interior hole. Exhaustively over a 16-bit container, the prefix mask separates
**65,536** pairs that agree at every declared field (`p5`). The repair is one word: mask to the
placement map's *occupancy* rather than to its extent, of which the prefix is the contiguous special
case, at a cost of one extra instruction where the constant is not an encodable immediate and zero where
it is.

---

## 1. What D25 says, and what it was deciding

D25 (op, 2026-07-29, `202607291210_topic.bitfield-becomes-its-own-crate.md:7-29`) is one call with two
grounds and one consequence.

The call: `bitfield!` gets `arvo-bitfield` rather than following its expansion target into `arvo-bits`
or staying beside the rest of the facade. The first ground is stated as decisive and op says so in his
own words: "**The reason is what it becomes, not what it is.** As a declarative macro it would sit
reasonably in `arvo-bits`, since it expands to `Bits` accessors and never mentions `UFixed`. But a macro
crate can later be a **proc-macro crate** ... A proc-macro crate cannot be the same crate as an ordinary
library, so putting the macro where it would have to move from is choosing a migration later instead of
a crate now." The second ground is one sentence: "It also matches the granularity settled for kirjo: a
consumer reaching for bitfields compiles the bitfield surface and nothing else." The consequence is
recorded for the changelist: `arvo-bitfield` is the workspace's first crate that may become a
proc-macro crate, which activates `no-alloc-no-std-framing.md`'s configurable exemption and gives
`mockspace.toml`'s `proc_macro_crates` its first entry, "neither applies while it stays declarative;
both apply the moment it does not".

Two things about this are worth stating precisely before anything is measured.

**D25 is a decision about a crate, not about a mechanism.** Nothing in it says what a bitfield is, what
laws govern it, or what it owes. The round did not treat it as a design question: the two earlier topics
that reach it (`202607290500:77`, `202607290600:29`) both file it under *placement of declared symbols,
unresolved*, and the round's own consolidated state carries it in exactly that list
(`202607300700:1027`, `:1052`). So "ratified-unread in content" (`101:168`) is right, and what is
unread is not a decision that was made; it is a subject nobody had reason to open.

**Its first ground is a conditional, and the condition is checkable.** "A macro crate *can* later be a
proc-macro crate" is true of every macro crate. The argument only bites if the upgrade is one this macro
will actually want, and D25 names what it would buy: attribute and derive macros, `#[bitfield]` on a
struct. Section 6 checks that, and it is the one place in this file where a compiled result changes what
the ground is worth.

---

## 2. What a bitfield is, now

### 2.1 The structure, and the four cells

Take the sentence a bitfield is usually explained by: it packs several values of declared widths into
one container and hands back typed access to each. Every clause of that is now something the design has
settled, and the settled forms compose into a shape the design already ships.

*Several values* is a product. *Of declared widths* is the fields extent per member. *Into one
container* is `place`, the middle map. *Typed access to each* is a projection indexed by the product's
own index type. The design already ships one product of numerals in one container run: a column under
`Layout::Bitpacked`, `P = 8/gcd(W_S, 8)` elements in `G = W_S * P / 8` whole bytes, zero inter-value
padding (`91:544-550`, ratified). That product is homogeneous, indexed by a `Capacity: Nat`, and its
offsets are `i * W_S` by derivation.

The bitfield differs on exactly two axes.

| | offsets derived | offsets declared |
|---|---|---|
| **index is a `Nat`** (homogeneous) | a column: `Dense` (stride is the projected container) or `Bitpacked` (stride is `W_S`). Both ratified. | a foreign array: the stride is whatever an external document fixed, including its padding. A `Crosses`-shaped claim. |
| **index is an hlist** (heterogeneous) | a bitfield with no written offsets. Compiled at `p4`, named by nothing in the corpus. | a bitfield as shipped: `field: W at LO`. D25's subject. |

All four cells have referents and the design has vocabulary for all four. The reading that unifies the
right-hand column is worth stating on its own, because it is what the whole of section 3 turns on:
**offsets get declared when an external document fixed them.** An array-of-structs from C has a declared
stride because a compiler somewhere chose the padding; a hardware register has declared field positions
because a datasheet chose them. Where nothing external fixed anything, the offsets are a prefix sum and
writing them by hand is transcribing a derivation.

Both generalisations were already forced elsewhere. The design already needed heterogeneous type-level
lists (`notko-hlist`, D5/D9, ratified) and already needed a declared correspondence to a foreign layout
(`Crosses`'s hand-laid impls, `91:733-745`). The bitfield is their composition, and that is the whole of
what it is.

### 2.2 The negative, which is sharper than the positive

**A bitfield is not a numeral, and this is not a quibble about the seal.** A numeral is value-unique: it
denotes one number, and the ratified machinery keyed on that (the quantiser, `Identity`, `Bounded`, the
far point, the crossing contract's value-layer clauses, the resolution row, the whole of the operation
surface's admission test at `102:518-522`) has no subject when the thing in the container is a tuple.

What does apply to a bitfield is exactly the datum-layer machinery and nothing else: `place`, statements
P and C, `materialise`, the byte image, the two digest stopping points, the only-door projection. That
is a complete list, checked against `102` section by section, and it has a consequence worth one
sentence in the chapter: **the composite level is datum-only by construction, which is why every
ratified law about a composite is a statement about placement, materialisation or digests, and why none
of them is about a quantiser.** A column has the same property for the same reason. The bitfield is the
second instance, and having two makes the property a fact about composites rather than a fact about
columns.

Consequences that fall out with no further work, each already ratified for the column and inherited
unchanged:

- **A field read must land on the canonical datum.** `74:265-271` gave the only-door law its fifth site
  here, and the reason it fires is now sayable in one line: a field read manufactures a datum out of a
  slice of container, and the container's own bits are only canonical because statement C says so, so a
  read that bypasses the canonicalising projection is reading a region whose guarantee it did not
  consume.
- **Statement C's region is the container's bits outside the map.** Not outside the last field; outside
  the union. That is the same widening section 5 finds for the digest and it is the same word.
- **The byte-sharing law decides whether a field has a byte image at all.** `73:205-215`: a field of
  width `W` at offset `o` has an independently addressable byte image only when `8 | o` and `8 | W`.

### 2.3 What the shipped grammar is, read as evidence

`arvo/src/bitfield.rs` spells the (hlist, declared) cell, at `Bits<N, S>` with `N <= 64` (`:379`), an
explicit strategy arm at `:276-288`, `repr(transparent)`, per-field getters and `with_` setters typed
`Bits<W, S>`, per-field `_MASK` consts, and `from_bits`/`to_bits`. Read as why-evidence, three things in
it point at the design questions rather than at the code.

It caps at 64 bits and dispatches its own container by a per-N macro (`__bitfield_container_ty!`),
which is the container projection written a second time. The design has one of those and it is not this
one.

It declares `N` and calls it the bitfield's width. Under the ratified three levels, `W_S` is the one
declared level and `W_C` is never declared (`91:534-541`). A hardware register's 32 is its container,
because a device fixed it. A design-internal bitfield's 32 is a carrier at best and is more honestly the
occupancy. **Which level a bitfield's declared width is, is a real question the grammar does not ask,
and it is different in the two cells of section 2.1's right-hand column.** I name it open rather than
answer it, because it interacts with `StoredWidth` and with whether a bitfield may be bitpacked at all,
and section 4 shows the interaction is not free.

And its documentation states the missing obligation in prose (`:28-30`). A hand-enforced invariant in a
doc comment is the shape this review has now corrected three times at other layers.

---

## 3. The placement map, and the one thing a bitfield actually owes

### 3.1 Two obligations, one of them the author's

A derived placement proves both of its well-formedness facts by construction: the offsets are a prefix
sum, so the fields are disjoint by monotonicity, and the total is the occupancy, so containment is one
comparison. A declared placement proves neither and owes both.

**Containment is asserted and it fires.** `arvo/src/bitfield.rs:377-386` asserts `$lo + $field_bits <=
$n` per field. Compiled against the shipped macro from a scratch crate outside the repo (manifest and
commands in `OUTCOMES.md`, since `mock/crates` is outside the panel's scope): a declaration of `wide: 8
at 12` in a 16-bit container, in a crate that constructs nothing and uses nothing, refuses with
`error[E0080]: evaluation panicked: sub-range wide does not fit within N bits` (`p1`, case B). That is
the correct shape.

**Disjointness is not asserted, and the shipped macro says so.** `p1` case A, compiled and executed
against the shipped macro:

```
Overlap: 16 { a: 8 at 0, b: 8 at 4 }        // bits 4..8 belong to both

container = 0b0000000000001111, a = 0xf, b = 0x0
a still 0xFF? false
```

`a` was set to `0xFF` and reads back `0x0F`, because writing `b` cleared four of its bits. Safe code,
no `unsafe` anywhere, no warning, nothing in the emitted diagnostics at any level. This is the mutation
gap's own signature (`102:274-294`) reached without a raw door, without a niche and without a transmute,
by an ordinary setter on an ordinary declaration, and it is the same shape file 100 found in `BitMatrix`
at rank 2. Two independent instances of one shape is worth more than either.

**Both are const-evaluable, and the second costs an `O(k^2)` loop over a list a human wrote.** `p1b`
discharges it and refuses the overlapping declaration with `error[E0080]: evaluation panicked: two
fields overlap`. `k` is the number of fields in one declaration; at the largest shipped declaration `k`
is three.

**The refusal is the default, not the law.** A deliberately aliasing view field is a real bitfield
idiom: a 32-bit register with a `word: 32 at 0` beside its named fields is how consumers read and write
the whole thing. Refusing that outright would be the policer posture the toolbox rule names. The honest
form is that overlap is refused unless declared, so an overlap is a statement the author made rather
than one they failed to notice, which is `102:771-779`'s own cannot-check-versus-cannot-provide
distinction reappearing one layer down: an overlap that is stated is a declaration, an overlap that is
silent is a falsehood the compiler can see.

### 3.2 Where a declaration-site refusal has to live to be one

The containment check in `p1` case B fires, and it is worth knowing why, because the answer is not "it
is an associated const on the type".

`p4c` compiles the same assertion at three placements against a violating declaration that is never
constructed:

| placement | result |
|---|---|
| associated const in the inherent impl, mentioned by nothing | **compiles, silent** |
| associated const mentioned by a `const fn` in the same impl | `E0080`, refuses |
| free anonymous const item beside the type | `E0080`, refuses |

The shipped macro is the middle row: `_BOUNDS` is declared at `:377` and mentioned by `let _ =
Self::_BOUNDS;` at `:393` and `:399`, inside `new()` and `from_bits()`. **The check is real and it hangs
on those two lines.** A refactor that drops or renames either constructor takes the guarantee with it,
silently, and the `ConstDefault` impl at `:370-374` already constructs a value without mentioning it.

This is file 100's `AGREES` finding at a second type, from the other side. There, the check lived at one
construction door and a second route bypassed it; here, the check lives on a mention and the type's own
declaration does not require the mention. The general form both instances want is the same and the
tenth consolidation already carries it for capacity: **the well-formedness of a placement is a fact
about the type, and it is stated where the type is, not where a value is made.** The free const item is
the cheapest spelling and it costs one line of emission.

### 3.3 What this makes of "hand-laid"

`91:765` calls a bitfield "precisely a hand-laid `place` map", which is right and which two files have
now carried without asking what hand-laid buys. It buys exactly this: the two facts a derivation proves
become declarations, and a declaration owes a check. That is the entire content of the phrase, and once
the check exists the map is provable-tier, not trusted-base. Section 7 says where the trusted base does
enter, and it is a different place.

---

## 4. The composition nobody has examined: a bitfield in a bitpacked column

Fresh search, `grep -rn "bitfield" *.md | grep -i "column\|packed\|stride"` over the panel directory,
2026-08-05 06:53: one hit, `81:411`, and it uses "bitfield" for the ARM instruction class (`ubfx`, a
two-literal bitfield extract), not for this design's bitfield. **No panel file composes the two.** The
dispatch is right that this is where two settled mechanisms meet, and the meeting is friendlier than I
expected in one direction and less free than expected in another.

### 4.1 The composite of two placements is a placement

Element `i` of a bitpacked column sits at absolute bit `i * W_S`. Field `f` sits at intra-element bit
`o`. So field `f` of element `i` sits at `i * W_S + o`, and reading it is either two slices composed or
one slice at the composed offset. Compiled, at 4096 elements of stride 13 with fields `(0,3) (3,5)
(8,5)`, every element carrying a distinct 13-bit value so the element space is swept rather than
sampled: **zero disagreements between the two forms and zero against the packed input** (`p2`). Swept
again at every stride from 1 to 57 crossed with every two-field partition of that stride, 1,596 shapes:
**zero disagreements** (`p7`).

The algebra behind it is one line and it is worth putting in the chapter because it is what closes the
category: `slice(slice(b, i*W_S, W_S), o, w) = slice(b, i*W_S + o, w)` whenever `o + w <= W_S`, which is
the containment obligation. **The composite is a placement, and the reason it is one is the obligation
section 3 says a declaration owes.** The check is not bureaucracy; it is what makes the composition
valid.

The decode plan composes the same way. `p2` prints the lane shifts per field over one period:

```
field 0 (o=0, w=3): [0, 5, 2, 7, 4, 1, 6, 3]
field 1 (o=3, w=5): [3, 0, 5, 2, 7, 4, 1, 6]
field 2 (o=8, w=5): [0, 5, 2, 7, 4, 1, 6, 3]
```

Each row is field 0's sequence rotated, because `(j*W_S + o) mod 8` is `((j*W_S) mod 8 + o) mod 8`. So a
column of bitfields needs **one decode plan, reused per field with a constant addend**, not one plan per
field.

### 4.2 The group arithmetic keys on the stride, and file 83 already said so

`81:199-214` tabulates seven quantities and says each is a function of `W`. Under a bitfield, `W` splits:
the period `P` and the group byte count `G` are functions of the element stride, while the mask and the
load width are functions of the field. `p7` checks the invariance directly: over every stride from 1 to
57 and every two-field partition of it, **the period and group rows never move when the field split
moves**, 1,596 shapes, zero exceptions.

**This is not a correction to `81`, and I want to be exact about that, because it would be an easy
finding to overclaim.** File 83 section 6 already made the general statement, in a stronger form than I
would have: *"a const derived from a width names the level it is a function of (fields, stored,
container), because two levels coinciding at the measured preset is how a compile-time fact computed
from the wrong level survives review"* (`83:305-308`). My result is that clause's first instance where
the fields side is a **list** rather than a scalar `W_F`: a bitfield has one occupancy but several field
widths, and a per-field mask names one of them.

**And the clause is not in the consolidation.** Fresh search, `grep -rn "names the level\|names its
level\|derived from a width" *.md`, 2026-08-05 06:53: exactly two files, `83:290,305,370` where it is
offered, and `84:137` where it is cited. Neither the ninth nor the tenth consolidation carries it, by
name or in general form; what they carry is its particular assignments (`91:530-545`). Whether the
general clause was absorbed into the layer-keying rule, which is close enough that absorption is
plausible, or dropped without a droplist entry, is a one-line disposition the next consolidation owes.
I raise it rather than resolve it, and my own material is the argument for the general form over the
assignments, since the assignments are a scalar table and the bitfield needs the rule.

### 4.3 Which composite to emit, and it is not a property of the layout

`p3` states the plan as file 81's own binding-time finding requires (`81:216-243`: a fact that must be
settled at compile time has to be written in a const position to be settled there), unrolls by the
period, uses unchecked loads so the comparison prices the work rather than the bounds checks, and
compares the two composites at two consumer shapes:

| body | instructions per group of 8 |
|---|---:|
| one field, two-step (materialise the element, then slice) | 38 |
| one field, one-step (slice at the composed offset) | 38 |
| all three fields, two-step | 399 |
| all three fields, one-step | 467 |

Reading one field is identical either way. Reading all three favours the two-step by 17 percent, because
the element load amortises across the fields and the one-step has nothing to amortise. **So the answer
is: neither, and the choice belongs to the consumer's access pattern.** That is `81:283`'s own sentence
("which decode is best is not a property of the layout; it is a property of the layout together with
what the consumer does next") arriving independently one level in, at a different pair of alternatives,
which is the second time this stretch a statement has been re-derived from the other side.

These are static instruction counts over differently-unrolled bodies and are **not** a runtime claim;
per `bench-and-sketch-discipline.md` a runtime claim goes in the bench harness, and this one has not
earned a bench.

One asymmetry the counts do not show and `p7` does: at wide strides the composed read can use a
narrower load. At stride 57 the element needs an eight-byte load (`57 + 7 = 64` bits) where a three-bit
field needs two (`3 + 7 = 10` bits). That is the one place the one-step form is strictly cheaper rather
than merely equal, and it is the row of file 81's table that keys on the field rather than the stride.

**One prediction of mine that the compiler refuted, recorded because refutations are the point.** I
expected the composite to need stating, on the reasoning that the intermediate element mask is dead work
the optimiser cannot see through. It is not: `((x >> s) & M_S) >> o & M_w` equals `(x >> (s+o)) & M_w`
whenever the field is contained, and LLVM performs the collapse itself, at both binding times (`p2b`,
four bodies, 23/24/23/24 instructions, the composed forms one instruction *worse* because of the address
add). The composition is a statement about what is true, not an optimisation to emit.

### 4.4 The byte image, closed rather than opened

File 73's law composed once: field `f` of element `i` has an independently addressable byte image when
`8 | (i*W_S + o)` and `8 | w`, for every `i`, which needs `8 | W_S` and `8 | o` and `8 | w`. Under
`Layout::Bitpacked` the interesting case is exactly `W_S mod 8 != 0` (at `8 | W_S` the period is one and
the packing is byte-aligned storage under another name). **So under a genuinely bitpacked column of
bitfields, no field of any element has a per-field byte image, and the only byte image is the column's.**

That closes a question rather than opening one, and it closes it in the direction `73:225-235` leans:
`materialise` for a single field does not exist, only `materialise` for the column, and any claim the
design makes about a field's bytes needs the same `Layout`-conditional scope a value's does. It also
means the mutation perimeter's byte owner under `Bitpacked` is the column group for a bitfield exactly
as it is for a numeral (`102:280-284`), with no bitfield-specific clause.

### 4.5 The interaction that is not free

One thing does not compose, and it is the level question section 2.3 named open.

A bitfield laid against a foreign format has a declared container: the register is 32 bits because the
device says so, and the fields sit at offsets measured from the container's bit 0. Packing such a
bitfield into a column at a stride below 32 moves every field's absolute offset and destroys the
correspondence the declaration exists to state. **A foreign bitfield is pinned to `W_S = W_C` and cannot
be bitpacked without ceasing to mean what it was declared to mean.**

An internal bitfield has no such pin. Its offsets are relative to the occupancy, its carrier is the
occupancy, its container is projected, and it packs at stride equal to the occupancy like any other
carrier.

The shipped grammar spells only the first and the tree uses it for the second. Naming the two in the
chapter costs one sentence and removes a question that would otherwise be discovered by the first
consumer who tries to pack one.

---

## 5. What a datum-keyed digest masks to

The ratified datum-keyed digest "masks the container straight to the fields' own width, undoing
statement C and statement P in one operation because both canonicalise to the identical fixed value"
(`91:628-631`, carried at `102` section 1.22). That is a prefix mask, `(1 << W_F) - 1`, and it is
correct for a numeral, whose `Encoding::Fields` occupy a contiguous low run.

A placement map need not be contiguous. Reserved bits in a foreign register, an ignored lane, a field
removed from a declaration without renumbering its neighbours: all leave a region strictly inside `[0,
W_F)` that belongs to no field. **No ratified statement covers it.** Statement P's domain is `[W_F, W_S)`
and statement C's is `[W_S, W_C)`; both are suffixes. Fresh search, `grep -rin "interior padding\|interior
hole\|declared hole\|reserved bits" *.md` over the panel directory, 2026-08-05 06:53: two hits, both in
files 94 and 95 and both about environment-receipt fields, a different subject. The region is unnamed.

`p5` exhibits the consequence exhaustively. `Reg: 16` with `enable` at 0 (1 bit) and `divisor` at 5 (9
bits) leaves bits 1..5 an interior hole and bits 14..16 container padding. Over all 65,536 container
values against three perturbations each:

```
prefix mask = 0b0011111111111111    union mask = 0b0011111111100001
prefix mask: separates 65536 equal pairs, conflates 0 unequal pairs
union  mask: separates 0 equal pairs, conflates 0 unequal pairs
```

**The prefix mask separates 65,536 pairs that agree at every declared field.** The union mask separates
none and conflates none. The repair is one word in an already-ratified sentence: *a datum-keyed digest
masks the container to the placement map's occupancy*, of which "the fields' own width" is the
contiguous special case, so the numeral's own statement is unchanged and the bitfield's is an instance
of it rather than an exception to it.

Cost, measured on this host: `and w0, w0, #0x3fff` against `mov w8, #16353 ; and w0, w0, w8`. One extra
instruction, and only because `0x3FE1` is not an ARM logical immediate; where the occupancy is encodable
the general form is free, and the mask is an associated const of the type either way, per the pricing
pillar and `81:216-243`'s binding-time finding.

**And the interior hole belongs to statement P's family, not to a new statement.** Its content is the
same content: the bits are canonical, established once by the pure constructor, and every datum-keyed
observation discards them. The only thing that changes is that the region P quantifies over stops being
a suffix. That is one word in P as well, and it is the same word.

---

## 6. The proc-macro upgrade: same mechanism, second one, or unnecessary

D25's first ground is that a proc-macro upgrade is coming and forces a crate now. Three questions, in
order.

### 6.1 Is the upgrade needed for capability

**No, and the contrast with the notation vehicle is exact.** File 61's wall is compiled and structural:
a decimal literal is one atomic token, `macro_rules!` cannot decompose it, and a notation macro
therefore cannot start declaratively (`61:64-113`, "not 'the declarative attempt got complicated,' but
'the declarative attempt cannot start'").

A bitfield's arithmetic is prefix sums over widths, and the widths arrive as separate tokens because the
consumer typed them apart. Nothing needs decomposing. `p4` compiles a `macro_rules!` muncher that
accumulates the prefix sum as an unevaluated token sequence and hands it to the const evaluator, over
heterogeneous declarations, **with no feature gates at all**, checked by rebuilding a 16-bit declaration
from its fields at every one of the 65,536 container values, both directions, zero mismatches. The
shipped macro already does the harder version of this, with `macro_metavar_expr_concat` for the setter
names, and that gate is WATCH-tier and already in use.

So the upgrade is not a capability. What it would buy is real and it is ergonomics: `#[bitfield] struct
Flags { .. }` reuses Rust's own struct grammar, which means rustfmt formats it, an editor completes its
field names, and go-to-definition lands on them. That is a good reason and it is a different reason from
the one D25 gives.

### 6.2 If it happens, is it the same mechanism as the notation vehicle

**Not the same macro and not the same wall, but the same crate kind and the same staging discipline.**
A proc-macro crate exports function-like, derive and attribute macros together; nothing requires one
crate per macro. So the design's `#[bitfield]`, if it is built, could live in the crate `raw_bias!` and
`numeral_face!` live in, and D25's own load-bearing sentence, that a proc-macro crate cannot be an
ordinary library, is an argument for *a* proc-macro crate rather than for one per macro.

The deeper connection is the staging one, and it is the thing that makes `arvo-bitfield` a real crate
rather than an empty one. File 61 section 8 prices the notation vehicle's binding time: the macro does
the arithmetic once on the host and emits the answer, and letting the type checker redo it instead costs
roughly 4.5x per declaration. Its closing sentence generalises: "the macro is not a convenience wrapped
around the design; it is the design's own binding-time decision, made explicit and paid for exactly
once, at exactly the stage that has the information cheapest" (`61:596-600`).

A bitfield macro is the same object. It takes a field list, computes a placement map on the host, and
emits the map plus the accessors that read through it. The alternative, a type-level fold over an hlist
of widths computing offsets at trait-solving time, is available and is the same trade file 61 measured
and refused. **So the design's two macros are two instances of one thing: a host-side stage that turns a
notation into a type, and the reason each exists is a binding-time decision rather than a convenience.**

That is what `arvo-bitfield` holds, and it is what the crate should be described as holding.

### 6.3 What granularity costs, measured

D25's second ground prices at `p6`, min of three on this host:

| crate kind | ms |
|---|---:|
| proc-macro crate, full build as a host dylib, sysroot `proc_macro` only | 187.1 |
| ordinary lib crate exporting a `macro_rules!`, metadata only | 68.3 |
| ordinary lib crate, full build | 66.4 |

Both sources are trivial, so the gap is the crate kind: **about 121 ms is the floor price of reaching a
proc-macro crate at all**, paid once per build of the dependency graph. The marginal cost of a second
macro living inside an existing proc-macro crate is its own source, a few hundred lines of host-side
Rust, comfortably under that floor.

So the granularity ground does not distinguish one proc-macro crate from two. A consumer reaching only
one macro pays the same floor either way; a consumer reaching both pays it once if they share a crate
and twice if they do not. On cost alone, sharing wins. `arvo-compile-time-last.md` puts both numbers
near the bottom of what should decide anything.

### 6.4 What this leaves D25 standing on

The decision stands. It stands on the crate holding notation, which is a category the design now has,
has priced, and has given a discipline to. Both of its written grounds are overtaken: the upgrade is not
forced, and the granularity is a tenth of a second.

**One thing I am not proposing**, because it is not mine and because the priced difference is small:
whether `arvo-bitfield` and the notation vehicle's crate are one crate or two, if and when either
becomes a proc-macro. That is a packaging call of exactly the kind `102`'s open list already routes to
op, and the input this file adds is that the two are the same kind of object and cost the same floor
either way.

**And one definitional-completeness item, small and mechanical.** "Bitfield" is the ISA's own name for
the instruction class the design's own bitpacked decode is built from: file 81 writes "two-literal
bitfield extracts" (`81:411`) meaning `ubfx`, in a file whose whole subject is a different mechanism.
One token, two ratified senses, which is the widened line's first mechanical address (`102:714-722`).
The remedy is the usual one, a boundary sentence at whichever definition the reader reaches first, and
the cheap version is that the design's own object is `bitfield!`'s output while the instruction is an
extract.

---

## 7. What a bitfield costs the trusted base

`Crosses<N: Numeral>: Lowering` is per-numeral (`91:733-745`): its three statements quantify over a
numeral's encoding, its lowering's carriers, and its container. A bitfield is not a numeral, so it does
not instantiate `Crosses`, and asking which `Crosses` entry a bitfield needs is the wrong question.

The right one decomposes. An internal bitfield's fields are each a numeral with its own lowering, each
already covered by its own `Crosses` entry or by the tower's generated one, and its placement map is
derived and const-checked. **An internal bitfield adds no trusted-base entry at all.**

A foreign bitfield adds exactly one, and it is genuinely new: **the placement map's correspondence to the
external document.** No compiler can check that bit 31 of a device register is the enable flag. The
artifact is the datasheet or the wire specification, cited by position, and the accounting is the one
`102:503-505` already established for a cited hardness constant, "a trusted-base entry with the citation
as its named artifact, the same accounting as every hand-laid `Crosses` entry".

Fresh search, `grep -rn "Crosses" *.md | grep -i "bitfield"`, 2026-08-05 06:56: one hit, `74:298`, which
lists the two subjects in one sentence about row survival and does not connect them. **No file assigns a
bitfield's foreign correspondence anywhere.** One sentence in the chapter does it, and it is worth
having because it is the only place a bitfield is expensive.

---

## 8. What this file does not decide

The crate question, which is op's: whether `arvo-bitfield` stays its own crate, becomes a proc-macro,
or shares one with the notation vehicle. All three are priced above and none of the prices is large.

Whether a bitfield's declared width is its carrier or its container in the internal case (section 2.3,
section 4.5). I state that the two cells need different answers and do not pick, because it interacts
with `StoredWidth`'s ratified reading and with whether an internal bitfield packs.

Whether the disjointness refusal admits a declared overlap, and by what spelling. I claim only that the
default is refusal and that a stated overlap is a legitimate idiom the toolbox rule protects.

Whether `83:290-316`'s level-naming clause was absorbed by the layer-keying rule or dropped (section
4.2). The disposition is the next consolidation's or op's; my material is an argument for keeping it.

Owed artifacts, each named with what closes it:

- **A second read on section 2.1's classification**, which is the load-bearing frame for everything after
  it and is one pass, mine. The attack surface I would point a second reader at: whether "derived versus
  declared" is a real axis or whether a derived placement is just a declared one with a cheap author,
  since `Layout::Dense`'s stride is also derived and nobody calls it a placement map.
- **Two tests for the overlap path**, one declaring an overlap and asserting the refusal, one asserting
  a stated overlap behaves as stated. Owed before the obligation moves from the author to the evaluator,
  and note that no test in the tree currently enters the path in either direction (gate section above).
  **Artifact:** the two tests, in `mock/crates`, which is op's boundary and not the panel's.
- **The occupancy-mask repair at a real `Numeral`-bearing model**, since `p5` models the container
  arithmetic rather than instantiating the tower. **Artifact:** one compile against the same
  `Specials`-bearing instance the float-division compile already owes.
- **A bench, if and only if a consumer arrives who reads several fields of a packed bitfield column**,
  because section 4.3's instruction counts are static and the two composites differ by 17 percent in
  emitted size at one shape, which is exactly the size of question the harness exists for and does not
  yet have a consumer to justify.
- **`arvo-bitfield`'s own remaining content review is closed by this file**; what is left of the row is
  the crate-kind call above.

---

## 9. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions.
*Placement map* (section 2.1): defined at first use as the finite set of `(offset, width)` pairs
assigning each member of a product to a bit range of one container; it is the object `91:765`'s
"hand-laid `place` map" names, given a noun so that "derived" and "declared" have something to qualify.
*Occupancy* and *occupancy mask* (section 5): defined as the union of a placement map's ranges and its
characteristic word, distinguished there from the *extent*, which is the maximum of `offset + width` and
which is what the ratified sentence's "the fields' own width" means. *Interior hole* (section 5): a bit
range inside `[0, extent)` belonging to no field, distinguished from statement P's and statement C's
regions, both suffixes. *Derived* and *declared placement* (section 2.1): defined by whether the offsets
are a prefix sum of the widths or written by the consumer. *Internal* and *foreign bitfield* (section
4.5): defined by whether an external document fixed the offsets, which is also what decides the trusted
base in section 7. *Element stride* (section 4.2): defined at first use and **explicitly distinguished
from file 81's "group stride" `G`**, which is a byte count over a period; mine is the bit distance
between consecutive elements, equal to `W_S` under `Bitpacked`. Terms used from the record without
redefinition: the three width levels, the three maps, statements 0, P and C, the only-door projection,
the byte-sharing law, the pricing pillar, the layer-keying rule, the toolbox rule, `Crosses`, `Capacity`,
hlist, `Layout::Dense`/`Bitpacked`. Named open rather than defined: which level a bitfield's declared
width is (sections 2.3, 4.5), which this file does not define because defining it would presume the
internal-versus-foreign split it also proposes.

**The separation requirement, performed.** Two models are this file's own.

The first is section 2.1's two-axis classification. It is **nonvacuous at the shipped bitfield against a
bitpacked column**, where the two share the placement machinery entirely (the group arithmetic is
invariant under repartition at 1,596 shapes, `p7`; the decode plan is one plan plus a constant addend,
`p2`) and differ entirely in the accessor surface (index by name against index by `Nat`), so a one-axis
reading has to pick one and lose the other. **Where it is vacuous I say so:** at a one-field bitfield,
where the hlist has length one, the classification adds nothing over "a numeral in a container", and my
verdicts there rest on the ratified column material alone.

The second is section 3's split between the obligations a derived and a declared placement owe. It is
**nonvacuous at exactly the disjointness obligation**: a declared map can violate it and does (`p1`,
compiled and executed), a derived map cannot express it (`p4`). **Where it is vacuous I say so:** at
containment, both forms owe the identical fact and both refuse identically (`p1` case B, `p4b`), so
the split says nothing there and section 3 does not let it imply otherwise.

**The freshly-performed-search requirement, performed.** Every universally quantified negative above
carries its own search, run this session, quoted with its date.

- "No panel file composes a bitfield with a bitpacked column": `grep -rn "bitfield" *.md | grep -i
  "column\|packed\|stride"` over the panel directory, 2026-08-05 06:53, one hit (`81:411`), which uses
  the token for the ARM instruction class.
- "No ratified statement covers an interior hole": `grep -rin "interior padding\|interior hole\|declared
  hole\|reserved bits" *.md`, 2026-08-05 06:53, two hits (`94:445`, `95:445`), both about
  environment-receipt fields.
- "No file names a heterogeneous product of numerals": `grep -rin "heterogeneous" *.md`, 2026-08-05
  06:53, four hits, all about the hlist crate itself.
- "The level-naming clause is in no consolidation": `grep -rn "names the level\|names its level\|derived
  from a width" *.md`, 2026-08-05 06:53, exactly `83` (offering it) and `84` (citing it).
- "No file assigns a bitfield's foreign correspondence to a trusted-base entry": `grep -rn "Crosses"
  *.md | grep -i "bitfield"`, 2026-08-05 06:56, one hit (`74:298`), a row-survival list.
- "The bitfield's proc-macro question appears in no panel file in content": the three files containing
  both "bitfield" and "proc-macro" are `11`, `74`, `101`, 2026-08-05 06:56, and all three carry it as a
  table row.
- "Every `bitfield!` declaration in the test suite is pairwise disjoint": `grep -n " at "` over both test
  files, 2026-08-05 06:56, nine field lines across four declarations, checked by hand.

The honest limit, inherited from files 97, 98, 101 and 103 and stated rather than assumed: these
performances verify that this file's terms are placed, its models have content, and its negatives were
searched with my vocabulary. They do not verify that a discussion using none of my search terms exists
somewhere in one hundred and three files. After file 98 nobody should treat a grep as exhaustive because
it was theirs, and a second reader with different terms is the check on mine.

---

## 10. Standing

The bitfield is ratified as a crate and was never read as a subject, and reading it yields what the three
previous periphery reviews yielded: no decision overturned, and corrections to what the decision's ground
was said to be. Both of D25's written grounds are overtaken, the first by a compiled result that runs
opposite to the notation vehicle's, the second by a measurement of about a tenth of a second, and the
decision stands on a category the design acquired after it locked.

What a bitfield is, the design already had: a product of numerals under a placement map, heterogeneous
where the column is homogeneous, declared where the column is derived, datum-only like every composite,
governed by statements the design already ratified for other reasons and by nothing else. Its one
addition is an obligation, and half of it is currently in a doc comment.

The composition with a bitpacked column is exact, its decode plan is one plan reused with a constant
addend, its byte image is the column's alone, and which composite to emit belongs to the consumer.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (the round `202607300800` at D25 `202607291210:7-29`, D27 `202607291700:8-39`,
and the placement questions at `202607300700:1027-1052`; `102` sections 1.4, 1.12, 1.22, 1.25, 1.26 and
its opening section at the lines cited in place; the persona-tier `95b`/`101b` as marked), settled shapes
(`73:195-235`, `74:69` and `:265-271`, `81:176-244` and `:283-300`, `83:255-316`, `91:530-660` and
`:725-770`, `61:64-113`, `:184-230`, `:513-627`, `32:200-220`, `11:40-60`), compiled (`104_probes/p1`,
`p1b`, `p2`, `p2b`, `p3`, `p4`, `p4b`, `p4c`, `p5`, `p7`, all at the pin, edition 2024, every one except
`p1` with no feature gates; commands and outcomes in `104_probes/OUTCOMES.md`), measured (the instruction
counts in sections 4.3 and 5 from `p2.s`, `p2b.s`, `p3.s`, `p5.s`, committed beside the probes; the crate
compile times in section 6.3, min of three, commands inline), verified at source
(`arvo/src/bitfield.rs:28-30`, `:276-288`, `:370-374`, `:377-386`, `:393`, `:399`, `:410-430`;
`arvo-bits/tests/bitfield.rs` and `arvo/tests/bitfield_const_eq_default.rs` in full; HEAD `5c952a3`),
reasoned (section 2.1's classification, section 2.2's datum-only reading, section 4.5's pin, section 6.2's
same-kind reading, and section 7's trusted-base decomposition, all mine, all one pass, offered as
suggestion and not as a ruling).*
