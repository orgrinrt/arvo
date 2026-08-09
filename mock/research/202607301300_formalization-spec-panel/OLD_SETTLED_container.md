# Survivor sweep: container derivation and erasure

Scope note before the findings. This sweep covers the theme assigned: how a declared width becomes a
machine container, the step A / step B split, the erasure gate (bits and bytes in, derived container,
validation, erasure on lowering), repr-transparency, what survives to the backend, monomorphisation, and
the three consumer tiers (inference, wholesale domain aliases, explicit spelling). It does not cover the
conversion/order/lattice thread (`145`, `146`, `148`, `149`, `150`), the float/decimal model, division, or
identity, which are other agents' themes and which I read only where they touch a container claim directly.

Coverage, stated honestly. I read in full: every checkpoint that bears on this theme
(`127b`, `130b`, `135b`, `137b`, `138b`, `139b`, `140b`, `142b`, `142c`, `143b`, `144b`, `144c`, `145b`,
`145c`, `149`), and the three expert files that carry the core derivation (`132`, `133`, `137`), plus `134c`
(a short dispatcher probe central to the bridge-extensibility claim) and the opening sections of `134` and
`141`. I read the opening of `150` only far enough to confirm it is off-theme (numeral order, not
container). I grepped, but did not read in full, the remaining files in the 128 to 149 range and everything
before file 119, on the judgment that the container/erasure thread as a named, worked question does not
crystallize until `119` to `142`, and the checkpoints after that range consistently summarize and correct
the expert files between them, which is where I would expect a later abandonment to surface if one occurred.
I did not open `110` or `124` directly; every claim I attribute to them here is quoted or cited by a file I
did read, and I trust that citation chain rather than the giant consolidations themselves. I did not read any
`*_probes/` directory contents beyond confirming a few are named in the checkpoints as recovered.

## Survivors

### 1. The erasure gate, stated as four conditions, all at once, no caveats

**Claim.** A design is acceptable for this theme only if all four hold together: the consumer expresses
usage in bits and bytes; the typestate derives the matching container and numeral representation, not the
consumer and not a later layer; it validates (the laws and refusals actually run); and it erases on
lowering, so what reaches the machine is the container and nothing else.

**Where settled.** `135b:12-28`.

**Provenance.** RATIFIED. Op, quoted directly: "There *is* a way to express usage through bits and bytes
*and* have the typestate derive the matching container and numeral representations, then validate, and erase
on lowering to be exactly what you describe before that caveat. Anything less than that, no caveats left, is
unacceptable for this design and canon." (`135b:12-16`)

**Rests on.** Nothing prior; this is the acceptance criterion the rest of the thread is measured against.

**Status at end of panel.** Still governing. `137b:5` records the panel's own closing line: "This is not a
panel note. It is a workspace rule," and the erasure gate is cited as a standing requirement in every
checkpoint through `145c`.

### 2. The gate is met unconditionally, at every width, once the comparison target is corrected

**Claim.** A numeral's operation lowers to the native instruction exactly when its payload is one limb of a
type the hardware operates on in one instruction, and that condition is a derived property of the payload,
not a discipline the operation author has to observe. At and below 128 bits the lowering is byte-identical
to the native primitive (LLVM folds the two into one symbol, with the native functions left with no bodies
at all). Above 128 bits, where no native instruction exists to compare against, the honest bar is what a
competent author writes by hand at that width, and against that bar the derived operation is within 0 to 2
instructions with the same instruction mix, at 192, 256, 512 and 1024 bits.

**Where settled.** `137:9-16` (verdict), `137:486-490` (section 6 restatement), with the earlier
codegen-equality result at `135b:30-51` and its caveat first raised there.

**Provenance.** RATIFIED. `137b:10-26`: "The erasure gate is met, and how it was met matters... File `137`
closed it." Op's own checkpoint states the earlier condition the panel had been carrying, "the operation body
must name a machine type," was never the real condition: "The real condition is that the payload be one
limb of a register-width type, which is a derived property rather than a discipline an author observes,
which is what op said it would be at `135b:65-68`." (`137b:16-21`)

**Rests on.** Item 3 (the container ladder is total and gate-free under structural keying, including above
128 bits) and item 8 (D48's literal spelling survives unchanged).

**Casualty absorbed into this survivor, not a separate loss.** The original framing of "step B" as "rung to
machine type" was carried by four files (`131`, `132`, `133`, `134b`) as the place the erasure gate is
satisfied, and it silently stopped meaning anything above 128 bits, where there is no machine type. `137`
found this (`137:163-172`) and repaired it in one line: "Step B is payload to operation," measured against a
hand-written multi-limb bar rather than a nonexistent native instruction. The corrected form is what
survives; the original "rung to machine type" phrasing does not and should not be repeated as the definition.

### 3. The container ladder is total and gate-free when the magnitude is a type, not a const

**Claim.** Keying the numeral's magnitude structurally (a little-endian binary type, `Term` / `D0<T>` /
`D1<T>`, with the value readable via an associated const) makes the whole derivation from width to container
expressible with no `#![feature]` gate and no `-Z` flag: the native rungs (8, 16, 32, 64, 128 bit containers)
resolve by trait-impl case-split, and the wide rung (above 128 bits) resolves to a `#[repr(C)]` word cons
whose size and alignment fall out of construction rather than an array length, so no width is enumerated
anywhere and there is no ceiling. Under const keying (a `const` generic magnitude), by contrast, the same
derivation is genuinely irreducible without the forbidden `generic_const_exprs` or the WATCH-tier
`generic_const_args`, because Rust's rule is that a generic parameter of any kind may reach type position
only as a standalone argument of the item it parameterises, never mentioned inside an anonymous constant.

**Where settled.** `133:16-24` (const-keyed irreducibility, the categorical result over thirty-plus compiled
positions), `133:271-391` (section 3, the structural construction that has no step A for the native rungs),
`137:99-133` (section 2, closing the one hole the structural encoding had: the wide rung, via `p5_total_ladder.rs`,
"exit 0, 0.04 s, and not one width enumerated anywhere in it").

**Provenance.** ONE EXPERT chain, not two independent agreements. `133` is explicitly dispatched as "second
read on `132`" and reads it before writing (`133:6-9`); `137` is explicitly dispatched to answer the gate
`135b` set, reads `132` and `133` first, and states directly "the appearance of tension is the brief's, not
the files'" between `131:282-284` and `133:651-653`, resolving rather than independently re-deriving
(`137:56-68`). None of these three files derived the structural-keying answer without having read its
predecessor. What elevates the claim past a single unratified reading is op's checkpoint: `137b:10-26`
records the gate as closed on the strength of `137`'s work, with op present and reviewing. I report this as
op-ratified at the outcome level, while flagging that the underlying technical chain is cumulative
(132 -> 133 -> 137), not three independent derivations, per the panel's own admission at `133:639-641` and
`137:583-587` that each is "the first read" on its own additions and needs a genuinely independent check.

**Rests on.** Op's no-ceiling ruling (`127b:118-126`), which is why a fixed-width carrier (item 6) and any
capped table are both foreclosed and the structural catch-all impl is required rather than optional.

### 4. The container is never written by a consumer; it is projected from strategy, widths, and sign

**Claim.** `UFixed<13, 3, Warm>` is the consumer's whole input. The machine container (which native integer,
or which wide-payload shape) is derived by arvo from the strategy together with the widths and the sign, and
is never a parameter a consumer supplies, directly or by default.

**Where settled.** `130b:37-53`.

**Provenance.** RATIFIED. Op, quoted directly: "Container naming is explicitly wrong. The entire idea of
arvo is that the strategy guides container selection, not the user. User writes strategy and arvo optimises
accordingly. And also, the same semantics and typestate will be used by other optimisation steps, such as
the already well designed hilavitkutin-build." (`130b:41-44`)

**Rests on.** Nothing prior; overturns a two-expert-agreed, compiled-refutation-backed `C: Container`
surface parameter that `130` had proposed.

**A near-miss worth naming, not a casualty.** `133` section 5 found a defaulted (but technically overridable)
container type parameter compiles gate-free and asked whether that specific shape is what the ruling
forbids, since a default that is never exercised changes nothing (`133:612-625`). Op never ruled on this
narrower question directly. A persona checkpoint standing in for op (`134b:79-90`, explicitly marked
non-authoritative, "Op has not seen `131`, `132`, `133`, `134`, or this file... nothing here is ratification")
reads the ruling as already foreclosing the defaulted form too, on the grounds that a default that is
exercised is exactly what was refused. That reading is plausible and grounded in a direct quote, but it is
not op's own word on the specific question, and no later file records op addressing it. I report the RATIFIED
claim as stated (no parameter, defaulted or otherwise, is offered) but flag that the defaulted-parameter
variant is technically unruled rather than affirmatively refused by op himself.

### 5. No enumeration, anywhere, if it can be helped: refused repeatedly on the same ground

**Claim.** A per-width (or per-conversion, or per-anything) enumerated table is refused wherever the panel
has offered one, on the standing ground that the information needed is already present in the typestate and
what is missing is the spelling to extract it, not a table to substitute for the derivation.

**Where settled, each an independent instance of the same ruling.** The width enumeration itself:
`127b:33-59`. The wide-rung bridge line (`impl ToNat<Mine> for Idx<7>`, offered as an acceptable one-line
population cost): `137b:28-41`. A macro invocation standing in place of a type at the surface:
`139b:27-40`. A blanket-`From` refusal used to avoid a coherence collision: `145b:47-59`.

**Provenance.** RATIFIED, four separate times, each with op's own words quoted in the checkpoint. On the
bridge specifically: "This really looks like just another instance of the spelling out being the problem,
all the heuristics should be there... No enumerations, if we can help it; and I think we have much to
explore to actually be able to help it." (`137b:34-36`)

**Rests on.** Nothing prior; this is a standing intent statement, restated by op each time the panel drifts
toward a table.

**What does not survive alongside it.** The mechanism that would satisfy this principle for the specific
const-to-type bridge (turning a written literal like `13` into its structural-nat type without a per-width
impl) remains unfound at panel's end. `139b:27-35` records op's refusal of the proc-macro alternative
(`UFixed!(13, 3, Warm)`) with the line "the answer still evades us, and finding it is the job." No file after
`139` claims to have closed it; see the open item at the end of this report.

### 6. A fixed-width carrier is dead on structural grounds, independent of its measured footprint cost

**Claim.** A carrier whose size does not depend on the numeral's width needs a fixed width to size itself,
and a fixed width is a ceiling. Op removed the ceiling. So a fixed-width carrier is foreclosed by a standing
ruling, not merely expensive; the footprint numbers (eight times at a machine word, thirty-two times at the
widest measured shape) are corroborating, not the argument.

**Where settled.** `132:375-399`.

**Provenance.** ONE EXPERT (`132`), never contradicted by any later file. Cited approvingly and left
unchallenged through `133` and `137`.

**Rests on.** The no-ceiling ruling, `127b:118-126`.

### 7. No layer downstream of rustc can change or revisit a chosen container type

**Claim.** Layout is settled at monomorphisation and is emitted to LLVM as bare literals (the stride in the
GEP type, the extent in `dereferenceable`, `size_of` folded to a constant), with no record in the IR of how
those literals were derived. Neither `hilavitkutin-build` (a pragma table and rustc wrapper, by its own
module doc) nor `notko-build` (which copies files) can substitute a type. The one workspace mechanism that
can change a type, `notko-macros`, works only because it runs before name resolution, on tokens, which is
also exactly why it cannot help width-generic code: there are no literals for a token-level macro to compute
with inside a generic function.

**Where settled.** `132:153-259` (sections 2 and 3).

**Provenance.** ONE EXPERT (`132`), unchallenged. This grounds why the erasure gate has to be met inside the
type system rather than deferred to a build pass, and it directly narrows what op's ruling about
`hilavitkutin-build` "using the same semantics and typestate" can mean: reading, not choosing (`132:107-126`,
confirmed against op's exact words at `130b:41-44`).

### 8. D48: the three-parameter literal surface is fixed and is what the consumer writes, regardless of the derivation mechanism underneath

**Claim.** `UFixed<I, F, S>` (three parameters, literal widths, no container parameter, no type-level
magnitude visible to the consumer) is the surface. Internal restructuring of how the container is derived
does not change it.

**Where settled.** Originates as op's decision D48 (dated 2026-07-29 in the consolidations, cited at
`127b:56-59`), reaffirmed at `130b:47-49` alongside the container ruling, and demonstrated compiled as the
unmoved surface across every proposed derivation mechanism in the panel: the GCA projection (`132`), the
structural encoding (`133:342-353`, `r3_alias.rs`), and the final assembled construction (`137:427-436`,
section 5, "Part one, the consumer expresses usage in bits and bytes. The written surface is D48's,
unchanged").

**Provenance.** RATIFIED (the original D48 decision), reaffirmed and compiled against by three independently
attempted mechanisms without ever needing to move.

**Rests on.** Nothing in this theme; it is a standing constraint every proposed mechanism in the thread is
checked against.

### 9. The three consumer tiers: inference, wholesale domain aliases, explicit spelling

**Claim.** There are three distinct ways a consumer meets arvo, and they are not equally common.

Tier one, the consumer who never adopts arvo's typestate: the public API is trait-bound shaped (algorithms
take `Add`, `Mul`, or arvo's own blanket-implemented contracts), a consumer supplies whatever type they
already have, arvo derives its own soundness and behavioural arms underneath, and the return is either the
same `T` the consumer piped in, or a generic output type whose default the consumer may override to ask for
arvo's own simplification and erasure.

Tier two, a framework or application that adopts arvo wholesale: it writes its own domain aliases once
(`StrHandle = UInt<5>`, itself an alias chain down to `UFixed<5, 0>` down to the one underlying
representation, `Numeral<...>`) and then never writes a generic or a strategy marker again; all the code
reads as the plain alias name.

Tier three, the explicit spelling, `UFixed<13, 3, Warm>` written in type position: it has to exist and has
to work, but it is the narrowest tier, and for a tier-two consumer it is written once, at the alias
definition, not at call sites.

**Where settled.** `142c:216-317` (both "clarification" sections, the tier enumeration and the alias chain).

**Provenance.** RATIFIED. Op, quoted at length: "The inference comes in from the arvo public APIs... Then
for frameworks and apps that want to wholesale take arvo, they'll write their own domain aliases like
StrHandle = UInt<5>... and all the code just reads non-verbose and ergonomic StrHandle. No generics,
nothing." (`142c:222-233`)

**Rests on.** D48 (item 8), for what the explicit tier looks like; item 4, for why the aliases never name a
container.

**A correction landed on top of this, and it survives too.** A first pass read "the bridge is exercised
rarely" as "cheap to get wrong" and repriced the surface question as lower stakes (`142c:277-319`). Op
reversed that specific inference without touching the tier framework itself: "It's not deflating it. To
write the domain aliases, it has to be easy and intuitive to write the types they alias... the ergonomics of
UFixed, FastFloat, all those, are crucial and perhaps more important than the plumbing itself." (`142c:329-333`)
The standing bar that survives from this: a container parameter, a type-level magnitude spelled at the
alias site, or a macro invocation at the alias site all fail outright, because the person writing that line
does not know or want to know the plumbing (`142c:374-380`).

### 10. The alias-reach limitation is real, belongs to notko, and does not touch arvo's own erasure story

**Claim.** A notko/hv scoped rewrite attribute (`#[optimize_for(...)]` in the panel's working name; the
workspace rule text has since corrected the attribute's actual name to `#[profile]` and its scope to
per-item lexical rather than build-wide) cannot see through a module-scope type alias, because the rewriter
operates on tokens before type resolution. But the alias itself carries the full typestate and is reachable
from there at the type level, so arvo's own derivation and erasure are unaffected regardless of what notko's
macro can or cannot do.

**Where settled.** `144c:1-52`.

**Provenance.** RATIFIED. Op: "This is only a problem on the macro level, where we do not have typestate and
work on ast level or thereabouts. The alias itself does contain the full typestate and is reachable from
there... I don't think this is meaningful in the big scope but within arvo: The notko synergy is purely
addition and a bonus, but even as such, not our concern." (`144c:19-26`)

**Rests on.** Item 9's tier-two alias chain, and the separate ruling (item 11 below) that arvo strategies and
notko profile tiers are not the same concept.

### 11. arvo strategies and notko profile tiers are not the same concept, and no future brief may assume they align

**Claim.** notko and hilavitkutin are separate projects with separate purposes. arvo's four strategies and
notko's profile-attribute tiers are not required to correspond, share only "synergy," and a finding that
treats one as standing for the other is not an arvo canon finding.

**Where settled.** `144b:8-19`.

**Provenance.** RATIFIED, stated twice by op in the same checkpoint: "Notko or hv are not directly associated
with arvo. The concepts need not align, they are different things for different purposes and in different
projects. They have synergy, but no continuity as such." and, on a separate finding, "Again, arvo strategy is
not the same as notko optimize for profiles. They have synergy, nothing more." (`144b:9-16`)

**Rests on.** Nothing prior; this voids a premise two files had silently adopted.

**Casualties absorbed by this ruling, not separate losses.** Three findings that depended on forcing arvo/notko
correspondence dissolve rather than get answered: the reported "`Cold` name collision" between notko's always-fallible
`Cold` tier and arvo's infallible `Cold` strategy; the claim that "no tier file can express an arvo posture"
is a design gap (it is the separation working, not a gap); and notko's own test-coverage defects, which are
not arvo canon findings at all (`144b:26-40`).

### 12. Every design cell, including every strategy/container table entry, is a function over a domain; a constant is the special case; existing tables are one arm, not the whole shape

**Claim.** There is no category of design element that is "a constant" as distinct from "a function"; a
non-varying cell is a function that happens to be constant over its domain, and that has to be established
rather than assumed. Further, variation is general across the design's elements (not confined to the strategy
axis or to `Warm` alone, which is where the panel first noticed it). The existing preset tables and fixed
cells the panel has been treating as the whole answer are one arm of that function (plausibly the
debug-assertions arm), with the release arm and others still to be written, additively, alongside them.
Nothing in the existing standing base needs retroactive annotation for this; the domain is carried by which
table (arm) a cell sits in, not by an annotation on each cell.

**Where settled.** `143b:1-73` (the ruling proper) and `143b:76-120` (the same-session clarification that
narrows what it obligates).

**Provenance.** RATIFIED, in op's own words, marked by the checkpoint itself as unusual in status: "Function
can also be a constant. It's not either or there. And all things change and act granularly, not just warm. I
call this as intent, settled canon, right now. This small bit in this association now governs future talks."
(`143b:9-11`) And the clarification: "Yes it is incomplete by a lot. I was under impression we are first
tackling the basic shape, perhaps the one we reserve for debug assertions time, and we write separate arms
for release and such then? It's always been incomplete. Nothing changes in standing base." (`143b:82-83`)

**Rests on.** Item 13, the specific case (`Warm`'s behaviour) that surfaced this general rule.

**Status.** Op explicitly marks this "ratified canon by op's own words and does not go stale on the usual
terms; it is intent rather than mechanism, and mechanisms move underneath it" (`143b:67-68`), the strongest
durability statement given to any single ruling in the checkpoints I read.

### 13. `Warm`'s intent is to behave like a native Rust primitive, including varying wherever Rust itself varies (e.g. by build profile); mechanism cells are checked against that intent, not the reverse

**Claim.** `Warm` is defined by what a consumer already expects from plain Rust primitives (wrapping or
panicking overflow depending on build profile, no cold-path or precision tuning, no excessive shedding of
accuracy for speed the way `Hot` does), not by a rule someone derived separately. Where a ratified mechanism
cell (e.g. the `70b` preset table's `Warm` clamp) conflicts with this intent, the cell is stale, not the
intent.

**Where settled.** `140b:9-36` (the original statement), reaffirmed and generalised at `142b:8-33` after
being brought back to op a third time.

**Provenance.** RATIFIED, twice. First: "My standing call is 'It should behave like native primitives in
regular old rust would'... The intent, here, is what matters. The mechanisms and theory may live freely and
shift under and around it, the intent is what remains and matters." (`140b:16-26`) Second, after the same
question returned: "Intent holds. Whatever feels intuitive and is how rust behaves. That's warm. You are once
again bringing me questions about Warm when I've been clear on this. If the rust one is different for
different profiles or targets, then our Warm should too, obviously." (`142b:11-16`)

**Rests on.** Nothing; it is an intent statement that item 12 generalises.

**Casualty this produces.** The `70b`-ratified preset table's fixed `Warm` clamp cell is explicitly declared
stale under this ruling (`142b:24-26`). See casualties list.

### 14. The wide payload above the native rungs is a strategy-axis consequence: ragged (byte-exact) for `Cold` and `Precise`, word-rounded (whole 64-bit limbs) for `Hot` and `Warm`

**Claim.** Above the native container rungs (roughly 128 bits, or 65 for `Warm`/`Precise` under the
now-condemned headroom rule), a numeral's payload has two live shapes with a real, small trade between them:
sized to the exact bit count (ragged, better footprint) or rounded up to whole 64-bit words (word-rounded,
fewer instructions per operation). The design does not pick one; it assigns ragged to `Cold` and `Precise`
and word-rounded to `Hot` and `Warm`, which is exactly the tradeoff the strategy axis exists to carry.

**Where settled.** Originates as a measured proposal at `137:293-311` (section 3), adopted at the checkpoint.

**Provenance.** RATIFIED. `137b:47-53`: "**Adopted.** Above the native rungs a wide payload is **ragged** for
`Cold` and `Precise`, sized to the exact bits, and **word-rounded** to whole 64-bit limbs for `Hot` and
`Warm`. Measured at one numeral: ragged is fourteen instructions and twenty-five bytes, word-rounded is
eleven and thirty-two. Three instructions per operation against seven bytes per value is exactly the trade
the strategy axis exists to carry, so nothing new is invented and the axis absorbs it."

**Rests on.** Item 3 (the structural, gate-free, total container ladder that reaches the wide rung at all).

**Status.** Confirmed still standing, unmodified, in the two latest files that reference it: `139:239-240`
and `140:283, 307` both cite it as adopted and untouched.

### 15. The `From`/`ToNat` structural bridge is consumer-extensible via a marker type parameter; the cap is not forced

**Claim.** A bridge trait that maps a written width literal to its structural-nat type (`impl ToNat<M> for
Idx<N>`) can carry a marker type parameter, and a downstream crate can populate the bridge for its own
widths, with its own marker, with no coherence violation, no feature gate, and no flag. This holds while
D48's literal spelling stays unchanged and the marker defaults so a consumer never writes it. So the range of
widths arvo ships is a convenience, not a hard cap.

**Where settled.** Originates at `134c:26-74`, reproduced independently as a directed second read on a fuller
construction at `137:427-448` (section 5.1).

**Provenance.** Weaker than the others in this list, reported with that caveat attached. `134c` is explicitly
self-marked as "one probe by one party" and "has had no second read," authored by the dispatching agent
rather than a dispatched persona (`134c:3-4, 86-89`). `137`'s section 5.1 is the read that discharges that
obligation, opening with "`134c` showed that a marker-carrying bridge is populated downstream with no
`E0117`... I am that second read and it reproduces, here on the full construction rather than a reduced one"
(`137:427-430`). Because `137` was explicitly dispatched to check this rather than arriving at it
independently, I do not report this as two independent agreements; I report it as one probe plus one
directed, successful reproduction, and note that op has not personally ratified it. No checkpoint after `134c`
or `137` records op addressing the bridge-extensibility claim directly.

**Rests on.** Item 3's structural encoding (the bridge only exists in that encoding) and item 5's
no-enumeration principle (this is offered as satisfying it: population happens over widths a program
actually writes, per `127b:36-50`, without a table).

**What is not settled alongside it.** How the bridge gets populated for arvo's own shipped widths without an
enumeration is still unresolved; see the open item below. The extensibility claim only establishes that
*consumers* are not capped, not that arvo's own construction avoids a per-width impl for its shipped range.

## Casualties

**The canonicity requirement.** Ratified at `127b:22-31` ("Two numerals of equal precision are the same
type... It is now a requirement in its own right"), reached with two independent expert agreements and a
compiled refutation of alternatives. Withdrawn by op himself at `130b:11-30`: "So the canonicity was for
making I + F evaluate to same type as F + I, which it probably shouldn't... I and F denoting the fraction and
integer, as I assume, I + F and F + I are explicitly not the same types and shouldn't interexchange
anywhere." Killed by op's own staleness principle once the mechanism it was inferred from (the width
enumeration) was itself deleted. Later structurally confirmed dead, not merely retracted: `145:74-83` proves
every equal-precision family is an antichain in the numeral order, which is the formal shape of exactly what
the withdrawn requirement would have forbidden.

**The width enumeration (a generated table, one impl per admitted width over a chosen range).** Converged
with two independent expert agreements, three compiled refutations of alternatives, and a full pricing
exercise; ratified nowhere directly, but treated by the panel as settled going into `127b`. Overturned at
`127b:33-59` by op's own objection ("The const param is probably the right *intent* call, just not executed
the best way... This is almost certainly doable so that only used widths realise on const time"), which `126`
then confirmed compiled: three impls, no feature gates, arbitrary widths, no cap. Killed the same day it
would otherwise have entered the standing base.

**The `70b`-ratified `Warm` clamp cell in the preset table.** A previously-ratified fixed cell value.
Declared stale by op's own restated intent ruling: "The `70b` clamp cell for `Warm` is stale, under op's own
staleness principle (`108b:11-20`): it was ratified under an understanding the intent statement has since
superseded." (`142b:24-26`) Absorbed into survivor item 13.

**Warm's headroom rule** (`rung(rung_bits(W)+1)`, giving a 65-bit crossover into the wide rung for `Warm` and
`Precise`). Measured and condemned by op at `139b:12-22` as producing unacceptable inflation (reported as
roughly 1600 instructions against 81 at 64 bits over a naive count, later corrected to 339 against 81 at
`139b:134-144`; the correction does not change the ruling). Both replacement mechanisms `139` offered were
refused in the same breath. A further deletion proposal (`140`, delete headroom for every strategy) was
neither accepted nor rejected: op held it explicitly pending harness benches, "hold calls on it until there
are actual benches" (`140b:69-71`). Harness benches then arrived (`141`, in `mock/benches/`, real competitor
arms) recommending deletion, but op's next word on it treats the whole body of `141`/`142` bench work as "one
instance of evidence, completely unaudited by a second expert set of eyes" (`142b:60-64`), and no later
checkpoint records a ruling. **This is not settled at panel's end**; the finding that headroom is redundant
survives as a structural claim, but no replacement mechanism or deletion was ratified.

**`141`'s claim that the container axis is monotone.** Reported by `142b:84-85` as refuted by `142`: "`141`'s
claim that the container axis is monotone is refuted, and attacking the losing arm recovered 41.0x of a 44x
loss, so the container was never the cost." I did not read `142` itself to see the mechanism of the
refutation; recorded here on the checkpoint's own statement of it.

**`130`'s section 10, and any claim resting on it.** `130` cites five probe files by name; none existed in
the repository (`138b:41-44`: "it cites five probe files by name and none of them exists anywhere in the
repository. There is no `130_probes/`. Its claim of 'compiled, gate-free' could not be checked, only redone,
which `138` did."). Per the panel's own evidence rule this makes the underlying claim void rather than merely
unverified. The probes were later recovered into the repository from scratch during a bulk recovery pass
(`140b:120-125`, fourteen directories, 361 files), but the checkpoint records the claim as having already
been redone independently by `138` rather than vindicated; I treat `130`'s section 10 as a dead citation
whose content was superseded rather than restored.

**"The decoder ring is a confirmed ceiling."** Settled as far back as `58:658-673`, repeated in both giant
consolidations (`110:2501`, `124:2406`). Overturned at `134:16-24`: a base-ten (rather than binary) digit
encoding compiles gate-free with the container ladder attached, at cost comparable to the binary form, with
no ceiling. The earlier claim's own root cause is named: `47:417-419` had concluded "a distinct struct per
number breaks the arithmetic" and never tried a distinct struct per *digit*, which does not. Adjacent to my
core theme (this is about diagnostic naming for the structural encoding, not the container derivation
itself) but included because it directly informs how costly item 3's structural approach is: `137:527-548`
later found the digit-tower diagnostic problem `134` set out to fix bites at only one of two site classes,
so `133`'s original "cost three" was real but overstated, and `134`'s fix addresses less of it than assumed.
None of this reverses the structural approach; it only revises how expensive its diagnostic weakness is.

**The first-pass reading that arvo strategies must align with notko profile tiers**, and the three findings
built on that premise (the `Cold` name collision, "no tier file can express an arvo posture" as a design gap,
notko's test-coverage defects reported as arvo findings). All voided at `144b:8-40` by op's direct correction
that the two projects "have synergy, but no continuity." The tier framework itself (survivor item 9), which
came from op's own words rather than from the forced-alignment premise, is unaffected and survives.

## What is open, not settled either way, at the end of the panel

Two threads bear directly on this theme and were never closed by any file I read or any checkpoint through
`149`.

**How arvo's own shipped bridge range gets populated without an enumeration.** The no-enumeration principle
(survivor 5) is ratified as intent. The mechanism that would satisfy it for the const-to-type bridge itself,
rather than for a consumer's own extension of it, is not found. Op's own words stand as the last word on it:
"the answer still evades us, and finding it is the job, *not* settling for a solution we've already ruled
out." (`139b:34-35`)

**Whether Warm's headroom rule is deleted, and what replaces it.** See the casualty entry above. Structural
findings survive without a ruling attached to them.
