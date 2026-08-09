# 87. Partiality and mutation: one perimeter question asked twice, and the offered answer to the first half does not survive being asked plainly what it trusts

Hans-Kristian Arntzen, file 87. I wrote file 31 (settling the identity contract, since superseded and
carried forward on nothing), file 42 (the observation surface, which found a perimeter hole one layer
below where file 41 had sealed it: `Bias`'s own trait was sealed correctly and the blanket impl it sat
on was not, so a foreign crate fabricated a `Pos` claiming false coprimality and widened the tower
through the layer nobody had looked at), and file 73 (the byte image, which this dispatch's second half
extends: the crossing contract's datum run through two further structural maps, `embed` and
`materialise`, with the padding law forced by a purity argument rather than chosen, and the perimeter
rule cited as the reason the choice matters at all, because `repr(transparent)` makes every carrier bit
observable "whether or not arvo ever ships a `to_bytes()` method"). The habit both files taught me and
this one repeats a third time: a guarantee closes when every operation that reaches the value is
checked, not when the operation someone thought to write is checked. Twice already in this review that
distinction found a hole one layer below where the previous pass believed it had sealed one.

## What I read

`78_consolidation_eight.md` in full, the standing base, and its verification section reproduced fresh
against the tree. `79_dolan_what_capacity_is.md`, `79b_op_the_verification_mandate.md`,
`80_leroy_the_verification_bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`,
`82_pesce_the_stretch_assembled.md`, `82b_op_checkpoint_twenty.md`, `83_lattner_how_many_widths.md`,
`84_leijen_failure_that_is_not_a_range_event.md`, `85_chlipala_the_closure_audit.md`,
`86_giesen_the_levels_assembled.md`, `86b_op_checkpoint_twentyone.md`, each in full, in order, since the
question is stated across three of them (the hardening at file 80, the construction at file 84, the
collision and the mutation gap both at file 86) and op's own hold at `86b` is the dispatch itself. One
`ls` of the panel directory, current through `86_probes`. My own prior files, `31`, `42`, `73`, re-read
at the source rather than through this stretch's summaries of them, since the dispatch names them as
what I am extending. The workspace perimeter rule,
`~/Dev/clause-dev/.claude/rules/what-you-can-observe-is-what-you-guaranteed.md`, read fresh: its own
"how to apply" item 1 already names "anything that hands out interior references" as part of the
observation surface, which is the mutation half of this dispatch stated before this review had the
question. `arvo-always-optimal-internals.md` and `arvo-toolbox-not-policer.md`, both read fresh for the
`from_raw`/`to_raw` door and the diagnostic-not-directive line the second half's repair leans on.

The shipped tree I touched for nothing beyond the standing canon-gate greps. No claim below reads it
for meaning, and every conclusion below survives deleting every tree citation, there being none beyond
the gate.

## Gates

Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate: `cargo
test --offline --workspace` from `mock/`, summed per binary by parsing every `test result:` line myself
rather than trusting a headline: **666 passed, 0 failed, 9 ignored**, matching files 81 through 86
exactly, from a clean tree at HEAD (`2cc8e37`). The one disqualifying test on record,
`arvo-tensor/tests/capacity.rs:14-18`, stands exactly as `78:874-876` and files 82 through 86 carry it:
three tautological lines (`cap(3) == cap(3)` after monomorphisation), flagged for deletion rather than
improvement, outside this panel's scope to touch. Toolchain `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, `aarch64-apple-darwin`, resolved from `rust-toolchain.toml`, confirmed inside the tree
this session; the identical command outside the tree resolves to stable `1.94.0`, reconfirmed. The
bench harness was not run; nothing below is a runtime performance claim.

**What is compiled, what is reasoned.** Sections 1 and 2 each carry a compiled component, in
`87_probes/` (four files, commands and outcomes verbatim in `87_probes/OUTCOMES.md`), built and where
applicable run fresh this session on the pinned toolchain, three of them independent constructions
rather than recompiles of the files under review (a different width for the domain collision, a fresh
padding-mutation model, a fresh two-tier repair), and one a fresh negative control. Everything else is
reasoned, and says so at each claim. Two of the compiled probes are executed, not merely built: probe 2
and probe 3 both produce runtime output I report verbatim, because the claim under test in each (a
silent divergence between two observations of the same value) is not a compile-time fact and a
type-checking result alone would not establish it. Everything here is a suggestion; the calls are op's.

---

## 0. The verdict, stated first

**Neither hole closes with the clause offered for it, and both fail for the identical reason: each
clause treats a fact that Rust's own soundness model puts in the trusted-base tier as if it had graduated
to the provable tier, the exact confusion this design's whole method exists to catch. The repair for
both is the same shape, already in the design under a different name: split the claim into a safe half
that is structurally, unconditionally total, and an unsafe half that is a named, audited, trusted-base
postcondition, exactly the way `Crosses` already splits derived-safe construction from asserted-`unsafe
impl`.**

For the collision: statement 0's hardening (`80:99-102`) is right and stays as written, unamended. The
niche-biased lowering does not, and should not, satisfy it as a blanket, safe, D16-derived construction.
It is instead a new, closed, sealable vocabulary (the finite set of std types carrying a
language-declared validity range) that the tower audits once, at the point it chooses to reuse a niche,
exactly the way a hand-laid `Crosses` impl is audited once at its own declaration site. "Quantify over
inhabitants" as a blanket amendment to statement 0 is not that; it re-admits, under a new name, the
unenforced side-condition the hardening was written specifically to forbid, and it does so by conflating
a compile-time-checked closure with a runtime-trusted one, which the compiler's own diagnostic on the
excluded construction (a warning, not a refusal) already tells the reader apart.

For the mutation gap: it is real, present, and not speculative, because it follows from two decisions
this review has already ratified (`repr(transparent)` on the carrier, needed for the read-side perimeter
argument to work at all; and the toolbox-not-policer stance's own commitment to shipping raw, unsafe
doors rather than hiding capability), not from a future API choice nobody has made yet. It is also not
confined to statement C, or to the byte image: it is a gap in the layer-keying rule's own general
statement, inherited by every one of its instances, cohorts included, because the rule as written polices
which door a *read* goes through and says nothing about what a *write* may leave behind for that door to
later mask. The repair, compiled at model scale (section 2, probe 3): the safe mutation surface never
exposes a raw accessor below the fields' own width, which closes the gap unconditionally for anything
reached without `unsafe`; the raw escape hatch the design will ship regardless, per its own stated
philosophy, carries the re-canonicalisation obligation as a documented postcondition on the door itself,
one entry, audited once, not re-derived per call site and not enforced by the type system, because
nothing in Rust can enforce it and pretending otherwise is the failure this review has already spent a
stretch correcting once (`78:723`, the `UFixed<0, F>::ONE` defect: an invariant nobody checked, trusted
by convention, silently wrong).

The two halves interact, and the interaction is the finding worth carrying forward on its own: both
holes are the same move, seen at a different axis. Section 1 is about the *domain* the guarantee is
quantified over, section 2 is about the *operations* it is checked across, and both times a claim that
looked total collapsed the moment it was asked to survive `repr(transparent)` plus one further step
(an unsafe transmute; an ordinary `&mut`). Section 3 states the general form.

---

## 1. The collision: form the reading, then attack the resolution

### 1.1 What the hardening actually says, checked against the collision rather than assumed

File 80's amendment, quoted rather than paraphrased because the wording is what the collision turns on:
"statement 0 quantifies over every bit pattern of `Encoding::Fields`' width; an encoding whose decode is
partial on that set does not satisfy `Crosses`, and partiality is expressed by shrinking the fields, not
by a domain side-condition" (`80:99-102`). The reason given, immediately before it: `repr(transparent)`
"makes every pattern reachable regardless of any shipped API, so the guarantee would be quantified over
less than what can be observed" (`80:96-99`). This is the identical argument file 73 already made for
the byte image (`73:416`, "whichever padding a constructor commits to is what every consumer, safe
API or not, will observe") and the identical argument file 42 made for the tower's own seal, citing the
workspace perimeter rule itself verbatim (`42:126-127`, quoting
`what-you-can-observe-is-what-you-guaranteed.md:12`: "a guarantee about a type holds only over the
operations through which the type can be observed"). It is
not a new move in this review; it is the same move, applied a third time, and I want to be honest that I
am the author of two of its three prior applications before I test whether the fourth application (a
niche) is the same shape or a different one wearing the same words.

File 84's construction, quoted for the same reason: bias a bounded numeral's datum by one and store it in
`core::num::NonZero`, spending the zero pattern, so the whole refusing tier becomes the same width as the
infallible tier (`84:354-383`), measured at nine extra instructions across sixty-four elements with no
branch (`84:373-375`). Compiled over its whole domain, round-trip asserted (`84_probes/probe_6`). This is
a real, valuable, measured optimisation, not a toy.

File 86's finding is that these two sentences are inconsistent as written, and it is compiled, not
argued: "the biased carrier's decode is partial on exactly one bit pattern of the 16-bit fields' width...
no field shrink expresses that domain: a width-shaped domain has exactly `2^w` members, and `2^w !=
65535` is asserted at every `w` in `0..=16`" (`86:66-72`). I reproduce this independently, at a second
width the prior probe did not use (a 12-bit biased domain, 4095 members, alongside the full 16-bit case)
so the claim is checked at a second instance rather than merely re-read: no `k` in `0..=16` gives `2^k`
equal to either `4095` or `65535`, checked exhaustively in const position
(`87_probes/probe_1_niche_domain_and_reachability.rs`, part A). It holds. The collision is real; I have
nothing to correct in its statement.

### 1.2 The resolution, tested rather than adopted: what "compiler-enforced validity range" actually is

File 86's repair: "statement 0 quantifies over every inhabitant of the carrier type at the fields' width;
partiality is expressed by shrinking the fields or by a compiler-enforced validity range on the carrier,
never by an unenforced side-condition. ... a niche-shaped domain is not an unenforced side-condition; it
is a **compiler-declared validity range**, ... a strictly stronger perimeter than field-shrinking, being
enforced by the language against every reachable path rather than by the tower's construction discipline"
(`86:78-90`).

I do not think "strictly stronger" survives being asked what "enforced by the language" actually means
for this specific mechanism, and I built the comparison to check rather than assert it.

**What field-shrinking closes, compiled.** A field-shrunk domain is an ordinary closed sum type: `enum
Shrunk { A, B }`, decoded by an exhaustive match with no wildcard arm. Adding a third variant without
updating the match is a hard compile error, `E0004`, "non-exhaustive patterns," under any code path,
safe or unsafe, that could reach the decode (`87_probes/probe_1b_exhaustiveness_negative_control.rs`,
run this session, reproduces the refusal exactly). There is no route to the excluded state at all: it
does not exist as a value of the type, full stop, and the compiler proves the decode total against the
type's own declared shape, independent of anything any crate promises to do.

**What a validity range closes, compiled, and it is a different claim.** `NonZeroU16::new(0)` returns
`None`: no *safe* constructor reaches the excluded pattern (`87_probes/probe_1`, part B, asserted in
const position). But the excluded pattern is not unreachable the way `Shrunk::C` is unreachable. It is
reachable through an ordinary, compiling, unsafe function:

```rust
unsafe fn reach_the_excluded_pattern() -> NonZeroU16 {
    unsafe { transmute::<u16, NonZeroU16>(0) }
}
```

never called anywhere in the probe (I grepped the file for its own name to confirm zero call sites before
writing this sentence), which compiles clean, exit 0. rustc's own diagnostic on it, quoted verbatim
because the exact wording is the whole finding: "the type `NonZero<u16>` does not permit
zero-initialization... this code causes undefined behavior when executed... `#[warn(invalid_value)]` on
by default" (`87_probes/OUTCOMES.md`). That is a **lint**, warn-level, not a type error. The compiler is
telling the reader, in its own words, that the code *compiles* and only misbehaves *at execution*. Set
beside probe 1b's hard `E0004` for the field-shrunk case, the asymmetry is not a matter of degree, it is
a difference in kind: one closure the type system proves against the type's own closed declaration, no
escape, ever; the other closure the optimiser is *licensed to assume* for code generation purposes,
which any unsafe code anywhere in the compiled artifact can violate, silently, with no refusal, no
panic, and no bound on the consequence, because that is what undefined behaviour means.

**This is not a special defect of `NonZero`. It is how every validity-range invariant in Rust works,
including every one std itself ships, and the design's own carve-out procedure already has a name for
exactly this status.** `unstable-features.md`'s std-internal carve-out reads: a feature "unlikely to
[stabilise, or be] badly broken in practice... every use site carries a `SAFETY:`-style justification
comment explaining why the use is sound, and the feature's known gotchas... are recorded so the use stays
inside the safe envelope." File 84 itself applied this procedure to the mechanism underneath `NonZero`
(`pattern_types`, internal-features-flagged, do-not-adopt-directly, stable wrapper named, `84:531-534`).
What file 86's amendment does not carry forward is that the carve-out's whole *point* is that the fact
being relied on is a **trusted, documented promise the type system consumes and cannot check**, the same
status file 80 itself already gave `Crosses`'s own hand-laid impls one section earlier in the same file:
"every consumer-site `unsafe impl Crosses` is an entry in the trusted base: an assertion the type system
consumes and cannot check" (`80:104-108`). A niche's validity range is that sentence, word for word,
applied to a different obligation. Treating it instead as a peer of field-shrinking inside statement 0's
own quantifier, with no accounting, is exactly the move D16's derived-safe/asserted-`unsafe
impl` split (`74:683`) exists to forbid: it launders an asserted fact into a derived one by giving it the
same sentence as the provably-total case.

**And the amendment as stated is broader than the mechanism the design can actually license, which
matters independently of the trusted-base point.** "A compiler-enforced validity range on the carrier,"
read plainly, invites a future numeral author to declare their *own* custom niche (exclude, say, the
all-ones pattern of a 13-bit field for some unrelated reason) the same way they can shrink a field. No
such general, stable, single-feature mechanism exists on the permitted feature set. I checked
independently rather than trusting file 84's own vetting: `pattern_type!` needs a second, undocumented
gate (`pattern_type_macro`, not present anywhere in `unstable-features.md`'s tables) even inside the
tree, and `#![feature(pattern_types)]` alone refuses outright outside it with `E0554`
(`87_probes/probe_1c_custom_niche_unavailable.rs`, both attempts). The only niches actually available to
a Lowering author are the finite, closed set std already ships (`NonZero<T>` at every native width,
`bool`, `char`, references, and whatever else std documents), and the amendment's own wording does not
say so. A future reader taking "a compiler-enforced validity range" at face value would believe a general
capability exists that the toolchain, and this rule's own prior vetting, both refuse.

### 1.3 The reading I offer instead: a closed vocabulary, sealed once, accounted as trusted base, not a widened quantifier

Statement 0 stays exactly as file 80 hardened it: quantified over every bit pattern of `Encoding::Fields`'
width, unconditionally, because that is the claim the type system can independently verify and it is the
claim `repr(transparent)` actually exposes to an adversary. A niche-biased lowering does not, and cannot
honestly, satisfy it as a blanket, D16-derived, safe construction, because its totality rests on a fact
(the excluded pattern is unreachable) that the type system trusts and does not prove. What changes is not
the quantifier. It is that the design recognises a **second, closed, sealable vocabulary**, exactly the
shape the carrier-at-birth rule already names ("a closed vocabulary that a guarantee quantifies over
owes its seal and its adversary at birth, not after three passes," `78:131-135`): the finite set of
`NicheCarrier` types std ships, sealed the same way `Pos`/`Nat` are sealed (a private supertrait no
foreign crate can implement), with its adversary already discharged for free, not by construction but by
the toolchain's own refusal: no stable route exists for a foreign crate to fabricate membership by
declaring a custom validity range, per section 1.2's own check.

The tower ships **one** `unsafe impl Crosses` for this closed vocabulary, written once, reviewed once,
naming the fact it trusts (the niche carrier's own documented validity contract, inherited from `core`,
not established by arvo) exactly as explicitly as a hand-laid format's own `unsafe impl` already names
statement 0/P. Every numeral that then chooses a niche carrier for its lowering composes through that one
entry and writes no `unsafe` of its own, the same way every consumer of a tower-generated `Crosses` impl
today writes none. This preserves the ergonomic promise file 84's construction earns (a numeral author
gets the trick for free, `arvo-toolbox-not-policer.md`'s diagnostic-not-directive stance intact,
`:82`) while keeping the trusted-base accounting honest: the *one* place the design trusts an
optimiser-enforced rather than type-checked fact is named, auditable as a list item, exactly parallel to
every hand-laid `Crosses` entry, rather than folded silently into a blanket amendment that reads as if it
strengthened a proof.

Scope condition file 86 already found (the carrier's top pattern must be spare or the shift must not
wrap, checked at every witness including E4M3, `86:92-102`) lands inside this one entry's own proof
obligation, unchanged in content, relocated to where trusted-base obligations belong.

*Grounded on: settled shapes (`80:90-112`, `84:354-411`, `82:296-303`, the D16 split at `74:683`,
`78:131-135` the carrier-at-birth rule, `78:723`), compiled (`87_probes/probe_1`, `probe_1b`,
`probe_1c`, all fresh this session), ratified (`unstable-features.md`'s carve-out procedure, applied by
file 84 and re-checked here), reasoned (the trusted-base reclassification and the sealed-vocabulary
resolution, mine, offered as a suggestion).*

---

## 2. The mutation gap: what it is quantified over, what a mutable door permits, and what actually repairs it

Op's own framing at `86b`: "the design's only-door rule covers observation and not mutation, so a
mutable door onto a container silently decorrelates a value's byte image from its digest while every
observation of the value stays correct." And the refusal of the one-clause fix: "hold, and see the
derivation first."

### 2.1 What exactly is at risk, named precisely rather than gestured at

The claim under threat is statement C (and, one level in, statement P): the bits outside the carrier
(container level) or outside the fields (carrier level) "are canonical, established by the projection's
pure constructor," with "every value- or datum-keyed observation" consuming the container "through the
canonicalising projection as its only door" (`83:194-199`, ratified via `82b`). File 73's own version of
the identical claim, one level down, is the padding law: "every tower-generated constructor of a carrier
from a datum canonicalises its padding to a fixed value... a structural fact about what a one-argument
pure function can and cannot depend on" (`73:163-166`).

Both sentences are **postconditions of construction**. They describe what `embed`, and the container
projection one level up, produce, once, at the moment they run. Neither sentence says, and neither
mechanism can say by itself, whether the postcondition survives everything that happens to the value
*afterward*, for the value's whole lifetime. The only-door sentence, restated at `68:138-139` and applied
four times by file 73 (`73:375-384`), is explicitly an **observation-time** rule: "a value-keyed
operation must consume its operand through a canonicalising projection." It says nothing about what a
write is permitted to leave behind for that same projection to mask on the next read, and that silence is
not an oversight in one instance, it is a scope the rule's own wording never had. Op's diagnosis is
exactly right and I want to state precisely why before proposing anything: **the guarantee, as written,
is quantified over reads. Mutation is not inside its domain at all, not narrowly violating it.**

### 2.2 What a mutable door actually permits, checked against what the design has already ratified rather than assumed to exist

The question that decides whether this is a real gap or a hypothetical one nobody has to answer yet: does
such a door exist, or would it require a future API choice the design has not made? I do not think it is
hypothetical, and the argument is the same argument file 73 already used for the *read* side, applied to
writes, which is the interaction section 3 names.

`repr(transparent)` (tree-fact for `Bits<N, S, Sign>`, cited by file 73 at `bits.rs:56` for the
attribute's existence, not its meaning) is a **layout** commitment, not an API-surface one. It exists
precisely so that FFI, raw pointer arithmetic, and bit-level reinterpretation see the type as identical to
its underlying container. That commitment runs both directions: if it makes every carrier bit
*observable* through a transmute regardless of any shipped read API (`73:172-178`, the argument file 80's
own hardening leans on directly, `80:96-99`), the identical fact makes every carrier bit *writable*
through a transmute regardless of any shipped write API. Nothing about `repr(transparent)` is
read-only; it is a claim about bit-for-bit identity with the underlying primitive, full stop.

And this design has already, independently, committed to shipping exactly this kind of door.
`arvo-always-optimal-internals.md` states plainly: "`UFixed::from_raw` / `to_raw` are the unwrap doors.
Internal hot paths use them freely" (`:36`). `arvo-toolbox-not-policer.md` forbids the alternative: "no
hardcoded limits... no stripping a capability because some consumer might use them wrong" and states the
line "diagnostic, not directive" (`:82`) as the standing posture against refusing to expose a primitive.
Taken together, these are not proposals; they are ratified commitments this review has cited approvingly
in five files this stretch alone (`78:718-727`, `78:457-461`, `84:169-171`, `84:451-452`). A raw,
`from_raw`/`to_raw`-shaped mutable door onto the container is not a possible future addition to arvo's
API. It is what the design's own stated philosophy already requires it to ship, for the same reason
`always-optimal-internals` requires the read-side unwrap doors to exist: `Bits`'s repr-transparent
guarantee is described in that exact document as existing so internals can flow through arvo-typed
boundaries "with zero overhead," which is meaningless if the flow is read-only.

**So the mutation gap is real and present today, in design terms, not speculative.** It follows from two
decisions this review has already ratified, composed, not from anything left to decide.

### 2.3 Which statements the gap touches, and which it does not

Walking the crossing contract's own layers rather than treating "mutation" as one undifferentiated
threat:

**Statement 0 (fields width, `[0, W_F)`) is unaffected, and this is worth stating precisely because it is
easy to conflate with the other two.** A raw write that changes bits *inside* the fields' own width
produces a new bit pattern of exactly the width statement 0 already quantifies over, unconditionally
(`80:99-102`). Statement 0's own hardening was written to be total over every such pattern regardless of
how it arrived, so a mutation confined to the fields is not a new case; it is precisely the case the
hardening already covers.

**Statement P (`[W_F, W_S)`) and statement C (`[W_S, W_C)`) are both affected, identically, by the same
mechanism one level apart.** Both are construction-time postconditions with no stated claim about
survival past construction, and both are reachable by the same raw-write argument, because the tower's
own three-level bookkeeping (file 83) places both regions inside one `repr(transparent)` carrier or
container, with no level-specific protection either enjoys that the other does not.

**The layer-keying rule's own general statement is affected, not merely its instances at P and C.** The
rule reads, in full: "a value-keyed operation must consume its operand through a canonicalising
projection, and that projection must be the only door" (`68:138-139`). Nothing in that sentence is
specific to padding. File 73 named four instances of the identical canonicalising-projection pattern:
cohorts (`V -> D`, `Encoding::Canonical`), padding (`D -> Carrier`), the digest projection, and, since
file 86's own correction, the container level (`Carrier -> Container`) (`73:375-384`, `86:175-179`). The
mutation gap is not a defect local to the byte-image chapter. It is a gap in the rule these four
instances all specialise, inherited by all four uniformly. A raw write into a decimal datum's own bits,
bypassing `Encoding::Canonical`, produces exactly the same silent divergence between a value-keyed read
(still correct, masked through the canonical representative) and a datum-keyed one (now reflecting a
non-canonical cohort member nobody constructed through the pattern's own door), for the identical reason.
I have not compiled this fourth instance; I name it because a repair scoped to statement C alone would
patch one symptom of a shared cause and leave three siblings exposed, which is exactly the shape file 42
already found once in this review (a seal on `Bias` correctly closed, the shared root cause one layer
below left open, a fabricated `Pos` walked straight past it). Op's own reason for holding the one-clause
fix, "this review has had three successive passes each believe a perimeter was closed when it was not"
(`86b:47-48`), is this exact risk, generalised across the rule's instances, not merely across passes.

### 2.4 The repair, built at model scale, in two tiers, neither of them new machinery

Rust has no reference-expiry hook: there is no `Drop` for a `&mut`, so "re-canonicalises on release,"
stated as a general obligation on the type, has no mechanism to attach to. The one clause file 86 offered
does not say how it would be enforced, and I think that omission is exactly why op held it: a perimeter
sentence with no enforcement behind it is a promise, and this review has already spent a full stretch
correcting a design that trusted a promise it never checked (`78:723`).

**The honest split, in the same shape sections 1's own resolution used.** A safe surface that never hands
out a raw accessor below the fields' own width closes the gap **unconditionally, structurally**, for
every path reached without `unsafe`: every safe mutation is value in, value out, composed of the same
`embed`-generating construction every ordinary construction already uses, so statement P/C's
postcondition is re-established on every call by the identical mechanism that establishes it the first
time, with nothing to track and nothing to enforce beyond what already exists. This is not a new
mechanism. It is a design decision (never ship a safe `&mut Carrier` or `&mut Container` accessor at any
width below the fields) that this review has never, in eighty-six prior files, proposed shipping the
opposite of; making it explicit closes the gap for the entire safe surface at zero new cost.

**Compiled, run, at model scale.** `87_probes/probe_3_two_tier_repair.rs`: a `Carrier(u16)` with a
13-bit field and 3 padding bits, a safe `set`/`add_wrapping` surface with no raw accessor, and an unsafe
`to_raw_mut` carrying a documented `# Safety` postcondition worded exactly parallel to `Crosses`'s own
statement P: the caller must leave the padding canonical before the borrow's last use, "a trusted-base
obligation, identical in kind and identical in enforcement to a hand-laid Lowering's own `unsafe impl
Crosses`." Run this session: ten arbitrary safe mutations leave the padding canonical every time,
unconditionally, by construction, not by discipline; the unsafe door, honoured, leaves the padding
canonical; the unsafe door, deliberately violated, reproduces exactly probe 2's decorrelation and nothing
worse, "exactly as the door's own safety contract said would happen." All assertions passed
(`87_probes/OUTCOMES.md`).

**The decorrelation itself, compiled and executed first, independently of any repair, so the claim under
repair is established rather than assumed.** `87_probes/probe_2_mutation_decorrelates_byte_image.rs`:
the same minimal carrier, embedded once (`embed(5000)`), a raw `&mut u16` obtained the way an ordinary,
entirely safe `&mut self.0` already would be (no unsafe transmute needed at all, since the model carrier
itself carries no validity range, which makes this the weakest possible form of the attack, present even
where section 1's niche mechanism is absent entirely), and the padding bits alone dirtied. Run: the
value-keyed read stays exactly correct (`5000`); the raw byte image decorrelates from a fresh construction
of the identical value (`0xf388` against `0x1388`); the fresh construction reproduces the pre-mutation
image exactly (`0x1388` both times), confirming the divergence is the mutation's doing and nothing else.
Op's own framing, checked rather than merely restated: the byte image and any datum-keyed observation
below the door (a digest reading raw bytes, which `arvo-pseudorand`'s own row already anticipates,
"what a hash of a `Number<N, S>` consumes is the digest law, not the hash's own business," `78:684`)
decorrelate together, from the tower's own canonical form, while every value-keyed observation stays
blind to it, which is exactly the shape this review has already found and closed at three other layers
(the `TotalOrd` split, the spectral NaN-payload defect, file 80's own nine-bit companion, all instances
of the layer-keying rule's own failure mode, `68:126-141`).

*Grounded on: ratified (`68:138-139` the rule, `83:194-199` statement C, `73:163-166` the padding law,
`arvo-always-optimal-internals.md:36`, `arvo-toolbox-not-policer.md:82`, `78:723`), settled shapes
(`73:172-184`, `73:375-384`, `86:159-179`, `78:684`), compiled and run (`87_probes/probe_2`, `probe_3`,
both fresh this session, outcomes in `87_probes/OUTCOMES.md`), reasoned (2.2's reachability argument,
2.3's layer-by-layer walk, the two-tier repair, mine, offered as a suggestion).*

---

## 3. Why these are the same question, and what the shared answer is worth stating once

Op named it directly: both halves are about what a perimeter is quantified over, one at the type's
domain, one at its operations, and asked whether they interact. They do, and the interaction is not a
coincidence of timing (two files raised them days apart, unable to see each other, exactly as op noted at
`86b:29-30`); it is one mechanism observed from two directions.

**Both collisions have the identical shape: a claim that is provably total against what the type system
independently checks was restated as if it were total against what an optimiser merely assumes.**
Section 1's collision: statement 0's fields-width totality is provable, closed, checked by rustc against
the encoding's own declared shape, no escape. A niche's totality is assumed, backed by UB on violation,
checked by nobody. Section 2's collision: the only-door rule's read-time totality is provable, the moment
every read genuinely routes through the canonical projection (which this review has compiled repeatedly).
Padding's survival across a value's *entire lifetime* is assumed, backed by nothing, checked by nobody,
because Rust has no mechanism to check it at all.

In both cases the confusion is not sloppiness on the part of the files that proposed the resolutions;
file 84's construction and file 86's own analysis of it are both careful, and file 86's mutation clause
correctly names the gap before proposing anything for it. The confusion is a **category error the design's
own vocabulary makes easy to commit**: this review has one word, "unsafe impl `Crosses`," for the
trusted-base tier, and it is scoped narrowly, to hand-laid `Lowering` declarations. Every other trusted
fact this stretch has produced (a niche's validity range, a raw door's write-time postcondition) needed
the identical accounting and had no obvious place to put it, so both times the easier move, extending an
existing *safe*-tier sentence rather than naming a new trusted-base entry, was the one that got made. That
is worth stating as a general finding, because a third instance of the identical confusion is exactly
what the next stretch's own capacity or exponent work risks producing if the vocabulary gap stays
unnamed.

**The general sentence, offered for the design's own use, and for the workspace perimeter rule it draws
on.** *A claim about a closed domain or a maintained invariant is provable only over the operations the
type system itself checks. `repr(transparent)`, together with any hand-off of the underlying
representation, whether a niche's validity range or a raw mutable accessor, moves the claim from the
provable tier to the trusted-base tier, whatever mechanism the language uses to make the hand-off
convenient. The design's obligation is to say so explicitly, once, at the point the hand-off is declared,
and to account for it the way `Crosses` already accounts for a hand-laid `Lowering`: never to phrase a
trusted-base claim in the provable tier's own words, because a reader checking the guarantee later has no
way to tell the two apart from the sentence alone, only from re-deriving what the sentence rests on.* The
workspace perimeter rule already carries half of this (item 1's "anything that hands out interior
references" is exactly the mutation half, named before this review reached it, and item 3's "`Transparent`-
style typed unwrap doors being the documented exception because they are declared as such" is exactly the
trusted-base half). What it does not yet say, and what both collisions in this file independently argue
for adding, is the distinction between a closure the type system proves and a closure the optimiser
merely assumes, stated as its own clause rather than left implicit in the examples. I offer the wording
above as a possible addition; the rule is workspace-wide, not the panel's to amend, and I flag it as a
suggestion for whoever next touches it.

---

## 4. What a consolidation could take, close to verbatim

*Statement 0's hardening (`80:99-102`) stands unamended: quantified over every bit pattern of
`Encoding::Fields`' width, because that is the claim `repr(transparent)` makes checkable and the claim the
type system independently proves. A niche-biased lowering (storing a datum shifted by one into a
`core::num::NonZero`, spending the excluded pattern, halving a refusing column's width) does not satisfy
this as a blanket, derived-safe construction: the excluded pattern is reachable through an ordinary,
compiling, unsafe transmute (a warn-level lint, "this code causes undefined behavior when executed," not
a compile refusal), and no field shrink expresses a niche-shaped domain, because a domain of size `2^w -
k` for small nonzero `k` is never `2^w` for any `w`, checked exhaustively at two independent widths. The
repair is a second, closed, sealable vocabulary, not a widened quantifier: `NicheCarrier`, sealed the way
`Pos`/`Nat` are sealed, over the finite, enumerable set of std types carrying a language-declared
validity range, with its adversary already discharged by the toolchain's own refusal of a general,
stable, custom-niche mechanism. The tower ships one audited `unsafe impl Crosses` for this vocabulary,
naming the trusted fact (the excluded pattern is unreachable in safe code, per the niche type's own
documented contract, not per anything the tower itself proves) exactly as explicitly as a hand-laid
format's own impl already names statement 0 and statement P; every numeral choosing a niche carrier
composes through it and writes no `unsafe` of its own. The scope condition (the carrier's top pattern
spare, or the shift's not wrapping) is this one entry's own proof obligation, unchanged in content,
correctly located.*

*The only-door rule (`68:138-139`) is, as written, quantified over reads: it says which door a
value-keyed or datum-keyed observation must pass through and says nothing about what a write may leave
behind for that door to mask. This is a gap in the rule itself, inherited by all four of its known
instances (cohorts, padding, the digest projection, the container level added at file 86), not a defect
local to statement C. The gap is real and present, following from two already-ratified decisions
composed (`repr(transparent)` on the carrier, needed for the read-side perimeter argument to hold at all;
and the toolbox-not-policer stance's own commitment to shipping raw, unsafe `from_raw`/`to_raw` doors
rather than withholding capability), not from any future API choice. Compiled and executed at model
scale: a raw write confined to a padding zone, reachable through an ordinary safe `&mut` and needing no
niche at all, leaves every value-keyed read correct while the raw byte image and any raw-byte-keyed
digest decorrelate silently from what a fresh, canonical construction of the identical value produces.
The repair is two-tiered rather than one clause. The safe mutation surface never exposes a raw accessor
below the fields' own width; every safe mutation is value-in, value-out, re-embedding through the same
pure constructor every ordinary construction uses, which closes the gap unconditionally and
structurally, at zero new mechanism, for the entire safe surface, compiled over ten arbitrary mutations.
The unsafe escape hatch the design will ship regardless carries the re-canonicalisation obligation as a
documented, trusted-base postcondition on the door itself, worded exactly parallel to `Crosses`'s own
statement P, one entry, audited once; violating it produces exactly the decorrelation already
demonstrated and nothing hidden beyond it, because Rust has no mechanism to enforce a write-time
postcondition and pretending one exists is exactly the shape of trusted-but-unchecked claim this review
has already spent a stretch correcting once.*

*Both collisions share one root: a claim provable against what the type system independently checks was
restated as if it were provable against what the optimiser merely assumes to hold, once for a niche's
domain closure and once for a construction-time postcondition's survival across mutation. The general
clause, offered for the design and for the workspace perimeter rule alike: `repr(transparent)`, together
with any hand-off of the underlying representation, moves a claim from the provable tier to the
trusted-base tier regardless of which language mechanism makes the hand-off convenient, and the design's
obligation is to name the hand-off once, where it is declared, in the trusted-base's own vocabulary,
never in the provable tier's words.*

---

## 5. What this leaves open

- **The sealed `NicheCarrier` vocabulary is proposed, not built.** I did not compile a sealed trait over
  the closed std-niche set, only the two artefacts that motivate it (the collision at a second width, the
  provable/trusted asymmetry). A member with reason to distrust the sealing argument should build it the
  way file 42 built `Pos`'s own seal, with a fabrication attack against it.
- **Section 2.3's fourth instance (cohorts) is named, not compiled.** I did not build the decimal
  cohort-mutation analogue; I reasoned it from the identical shape the padding case already established
  and flagged the reasoning as such. It is the cheapest of the open items here to check, since the
  machinery (`Encoding::Canonical`, the cohort census) already exists at file 54.
- **Whether the sealed `NicheCarrier` vocabulary and the mutation repair's own trusted-base door compose
  cleanly is unchecked.** A numeral using a biased niche carrier that also exposes a raw `to_raw_mut`
  door inherits two trusted-base obligations at once (the niche's own, and the write-time
  re-canonicalisation), and I did not build the combined case to confirm the two proof obligations do not
  interact in some way neither alone would show.
- **The general clause in section 3, offered for the workspace perimeter rule, is a suggestion outside
  this panel's own scope to adopt.** The rule is workspace-wide; I flag the possible addition and do not
  presume to write it into the rule file myself.
- **Whether "one audited entry, ergonomically free to the numeral author" is the shape op actually wants,
  versus a stricter per-declaration accounting that trades ergonomics for a shorter trusted-base list, is
  a design taste question I have a lean on and no standing to settle.** I built the ergonomic shape
  because it matches the design's own stated diagnostic-not-directive posture; a member who weighs
  auditability more heavily than convenience should attack this choice specifically.
- **Both halves are one-pass**, on a collision between two one-pass proposals, touching a ratified
  precondition and a rule this review has already found decayed once. Per the review's own standing
  discipline, neither should harden without the second independent read op's own hold at `86b` already
  requires.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion, not a
ruling.

*Grounded on: ratified (`78:131-135`, `78:723`, `68:138-139`, `80:96-112`, `83:194-199`,
`arvo-always-optimal-internals.md:36`, `arvo-toolbox-not-policer.md:82`, `unstable-features.md`'s
carve-out procedure), settled shapes (`73` in full, `84:354-411`, `86` in full, `74:683`), compiled and
run (`87_probes/probe_1`, `probe_1b`, `probe_1c`, `probe_2`, `probe_3`, all fresh this session on the
pinned toolchain inside the tree, commands and outputs verbatim in `87_probes/OUTCOMES.md`), reasoned
(sections 1.3, 2.2, 2.3, 2.4, and section 3 in full, mine, offered as suggestions and evidence, not as
rulings).*
