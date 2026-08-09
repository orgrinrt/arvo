# What is settled: the public surface

Theme: type and alias naming, what a domain alias is and who writes one, ergonomics of the primitives,
the bridge and its extensibility, canonicity of a spelling, trait bounds and blanket impls at the API
edge, coherence, diagnostics and error messages, what the consumer actually types. Archival survival
sweep, not design. Provenance decides, never recency or confidence. A claim is reported only if it
survived to the last file of the panel (`150`).

The panel rebuilt the numeral's representation twice inside this stretch (the tower was deleted at
`126`, the numeral was re-keyed at `129` and again at `130`), and the container-selection mechanism was
argued over five files (`131` through `133`, plus checkpoints) without ever being ratified. Most of the
surviving claims below therefore date from `127b` onward; almost everything about the numeral's public
spelling from before `126` was superseded by the deletion of the tower it was written against, and I
have not tried to resurrect any of it.

## Survivors

### 1. The container is never written by a consumer

**Claim.** The consumer writes strategy, widths and sign. The container (which machine primitive or
wide-byte carrier backs the value) is derived from those, never named in the type a consumer writes.
The same derived typestate is specified to be read by downstream optimisation layers, `hilavitkutin-build`
named specifically, so a consumer-pinned container would take a decision away from every later stage.

**Where settled.** `130b:39-53`. Op, verbatim: "Container naming is explicitly wrong. The entire idea of
arvo is that the strategy guides container selection, not the user. User writes strategy and arvo
optimises accordingly. And also, the same semantics and typestate will be used by other optimisation
steps, such as the already well designed hilavitkutin-build." And: "This is not an ergonomic preference.
It is what the crate exists to do."

**Provenance.** RATIFIED.

**Rests on.** Nothing upstream; this is a first-order ruling. It rules out two competing surfaces in the
same act: `129`'s and `130`'s explicit `C: Container` parameter, and (implicitly, confirmed at `133`'s
section 8 and never reopened) a defaulted-but-writable container parameter, which `134b` shows is what
the same ruling forbids on a fair reading ("a default that is never exercised changes nothing; a default
that is exercised is the thing he refused," `134b:90-91`).

### 2. D48's literal arity survives every route, independent of the container mechanism

**Claim.** `UFixed<I, F, S>` (three written parameters: integer width, fraction width, strategy) is the
surface, on every container-derivation route the panel built, whether the container is projected through
a feature-gated tag (`131`, `132`) or derived through a structural, gate-free encoding (`133`). The
surface does not move regardless of which mechanism eventually wins.

**Where settled.** Originated `127b:56-59` ("D48 stands"). Confirmed independent of the fork at
`133:470-485` (a defaulted type parameter accepting a projection over standalone const args, "no gates,"
three written parameters). Restated as closed by `134b:68-77`: "whichever route wins for step A, the
surface does not move... Take it off the fork."

**Provenance.** RATIFIED (the arity itself), with the independence-from-mechanism claim carrying two
independent compiled demonstrations (`131`, `133`) plus the persona-checkpoint's reading of the ratified
line.

**Rests on.** Survivor 1 (the container is never in the parameter list).

### 3. No enumeration in the public surface, ever, if it can be helped

**Claim.** A standing, repeated refusal of any mechanism whose gate-free cost is "one impl per width,"
however small the enumerated set or however rarely it is written. Op refused this exact shape three
separate times across the panel (a width table at `127b`, a per-width `impl ToNat<Mine> for Idx<N>`
population line at `137b`, and a macro-based escape at `139b`), each time on the same ground: the
information already exists in the typestate and what is missing is a way to spell it out, not a table.

**Where settled.** `127b:33-59` (the width table refused; "We shouldn't police it... we shouldn't
explicitly have to define each and every step on the way," `127b:49-50`). `137b:28-41`, the sharpest
statement: "This really looks like just another instance of the spelling out being the problem, all the
heuristics should be there... It should come implicitly from the heavy typestate. No enumerations, if we
can help it; and I think we have much to explore to actually be able to help it." Recorded there as "the
same finding a seventh time." `139b:27-39`: a function-like macro was also refused as a surface, not
because the mechanism is wrong ("we'll gladly take all the proc macro crates we need") but because "a
macro invocation standing where a type should be written is refused, and was refused before."

**Provenance.** RATIFIED, restated at least three times with increasing specificity.

**Rests on.** Nothing; this is a standing instinct op applied consistently across the panel and it is
the strongest recurring pattern in the whole document (four converged panel conclusions overturned on
this ground by `130b`, per its own count).

**What it costs the panel.** By the last file, no gate-free, zero-enumeration mechanism for the
width-to-container derivation had been found and ratified. See Casualties, "the container-derivation
fork."

### 4. A macro invocation may not stand where a type is written

**Claim.** Distinct from survivor 3, and worth separating: proc-macro machinery is welcome anywhere in
the design, but a macro call is never an acceptable *spelling* for a value the surface should type-check
as a plain type. This closes off both `129`'s `UFixed!(13, 3, u16, Warm)` function-like-macro surface and
`139`'s proposed `UFixed!(13, 3, Warm)` twenty-line proc macro as *surfaces*, though not as internal
machinery.

**Where settled.** `139b:27-39`. Op: "using a macro invokation in place of a type is not what we want,
and I've already ruled on this." Reaffirmed at `142c:377-378`: "A macro invocation was already refused as
a surface, and the alias site is where that refusal bites hardest, since an alias is a type definition
and a macro call is not."

**Provenance.** RATIFIED.

### 5. Every named domain type is a convenience alias over one underlying representation

**Claim.** `UFixed`, `IFixed`, `FastFloat`, `Decimal` (and, one level further out, a consumer's own
`StrHandle`) are not a taxonomy of four (or more) families. They are points differing in a small set of
axes over one representation, kept as aliases because op mandated keeping them, and the design's
recurring confusion was treating the aliases as the subject rather than as spellings over it.

**Where settled.** First reached mechanically at `138:9-40` (the design "has no four families... they
are points in a product of four axes"). Ratified at `138b:9-45`. Op, verbatim: "I mandated we keep the
aliases, but they remain convenience aliases to the underlying one representation they all are... find a
way to make it stick." Restated a third time and made concrete with the actual chain at `142c:255-267`:
`StrHandle -> UInt<5> -> UFixed<5, 0> -> Numeral<...>`, "every named type is an alias over one
representation."

**Provenance.** RATIFIED, restated three times (op himself notes this: "the panel has reached this
before and lost it," `138b:32`).

### 6. Three consumer tiers, and where each writes what

**Claim.** Tier one never adopts arvo's typestate at all: algorithms are trait-bound shaped (`T: Add`,
core traits or arvo's own blanket impls), the consumer supplies whatever numeric type they already have,
arvo derives soundness and the compile-time branches underneath, and hands back a type the consumer
recognises (their own `T`, or an output generic whose default is arvo's simplification policy and which
they may override). Tier two, a framework or application taking arvo wholesale, writes domain aliases
once (`StrHandle = UInt<5>`) and then never writes a generic type again. Tier three, the explicit spelling
(`UFixed<13, 3, Warm>` written directly), is the narrowest tier and is mostly only written once, at an
alias definition, rather than at call sites.

**Where settled.** `142c:220-275`, op's own words in full, quoted at length in the checkpoint. The
"inference comes in from the arvo public APIs" paragraph (tier one), the "domain aliases... no generics,
nothing" paragraph (tier two).

**Provenance.** RATIFIED.

**Rests on.** Survivor 5 (aliases as the vehicle tier two writes).

### 7. The ergonomics bar for a domain-alias definition site is `UInt<5>`, and rarity raises the bar rather than lowering it

**Claim.** A framework author writing a domain alias has no context on containers, no practice with
parameter order, and no interest in acquiring either; they will write the line a handful of times and
never build fluency in it, so it must be self-evident on first contact and stay self-evident after six
months away. `UInt<5>` (a name and a number) is the bar; `UFixed<5, 0>` (a name and the two numbers
someone already thinks in) is at the bar. Anything requiring a container type, a type-level magnitude, a
memorised parameter order, or a macro call falls below it and is disqualified at that site specifically.
The ergonomics of the named types are, in op's words, "crucial and perhaps more important than the
plumbing itself."

**Where settled.** `142c:322-386`, op's correction to the panel's own repricing. Verbatim: "To write the
domain aliases, it has to be easy and intuitive to write the types they alias. UInt<5> is great and easy,
intuitive. Ufixed as in the example too. But if there were precision in there, container types, it'd be
fucking ass for anyone who doesn't know nor care about all that plumbing, to write. So the ergonomics of
UFixed, FastFloat, all those, are crucial and perhaps more important than the plumbing itself."

**Provenance.** RATIFIED.

**Rests on.** Survivor 6 (tier two is who this bar is written for). It disqualifies, concretely: a
container parameter in the spelling (survivor 1, restated as an ergonomics finding rather than only a
mechanism finding), a type-level magnitude such as `Fixed<N13, N3, Warm>` (the alias writer would have
to know a width is spelled as a type, and which type), a macro invocation (survivor 4, and the alias site
is where the file argues it "bites hardest"), and extra arity in general (each additional parameter is
one more thing an uninitiated writer must know at the one site with no context to draw on).

### 8. `Warm` is defined by what a consumer expects, and the intent outranks whatever mechanism instantiates it

**Claim.** `Warm` behaves like a native Rust primitive would, including varying wherever Rust itself
varies (for instance overflow panicking under `debug-assertions` and wrapping in release). This is a
standing intent, not a table cell, and a later member finding tension between a ratified mechanism cell
and this intent should conclude the cell is stale rather than escalate.

**Where settled.** `140b:11-35`. Op: "WE are the designers of the strategies and how we define them... My
standing call is 'It should behave like native primitives in regular old rust would.'" And the principle
underneath: "The intent, here, is what matters. The mechanisms and theory may live freely and shift under
and around it, the intent is what remains and matters." Reaffirmed, after the panel brought it back a
third time, at `142b:8-33`: "Intent holds. Whatever feels intuitive and is how rust behaves. That's warm.
You are once again bringing me questions about Warm when I've been clear on this."

**Provenance.** RATIFIED, twice.

**Note.** This is a survivor about intent, not about a mechanism. No specific headroom rule, container
selection or codegen strategy for `Warm` was ratified by the panel's end; those stayed open pending
harness benches (`140b:59-89`).

### 9. Precision is sign-free

**Claim.** `PRECISION` (the significand digit count a numeral's laws are stated over) never includes the
sign bit. `PRECISION = I + F`. The stored physical width, which does include the sign bit for signed
numerals, is a separate derived quantity: `STORED_WIDTH = sign_extra + I + F`. A law folding the sign bit
into precision (as two earlier files did) produces a false multiplicative law that costs a container
rung at the exact point it crosses sixty-four bits.

**Where settled.** Derived necessarily from an already-ratified sentence: `30b:9`, carried at
`110:869-873`, that identity is parameterised in mathematical coordinates rather than encoding
coordinates, and that "precision and the exponent bounds are primitive; total width, the hidden bit, and
field encoding are derived on the physical side." A sign bit is field encoding. Two independent
derivations from the arithmetic (not from each other) reach the same conclusion: `131:745-801` compiles
the false-law counterexample (`mul::<16, 16, 31, ...>` refused where the correct output is asserted), and
`132:559-585` reproduces the arithmetic independently ("a two's-complement product of two n-bit signed
values needs 2n - 1 bits, not 2n"). `134b:18-44` states plainly that this "was never open": the two prior
files that folded the sign in (`129`, `130`) are drift against `30b`'s own line, corrected rather than
escalated.

**Provenance.** RATIFIED (derives directly from `30b`), with the corrected reading independently reached
twice (`131`, `132`) before `134b` names it settled.

**Rests on.** D69's overturn (`30b`, carried at `110:869-873`).

### 10. The container derivation splits into a step that costs nothing and a step that costs a feature

**Claim.** Whatever mechanism eventually selects a numeral's container, the derivation decomposes into
two independent steps: turning `(width, sign, strategy)` into a rung index or byte count (a case split
over generic parameters, which is what needs an unstable feature under const-keying), and turning that
rung into an actual machine type (a case split over literal keys, which is gate-free and lowers to
codegen byte-identical to a hand-written native container). This structural split holds regardless of
which specific mechanism is chosen for the first step.

**Where settled.** Established `132:21-27`. Refined at `133:335-351` (locating the purchase more
precisely as one impl body, not one const expression). Explicitly listed among what survives a later
reframing of the whole stretch at `142c:79-83`: "The structural findings are about mechanism and hold
under any cell values: that the container derivation splits into a step that costs nothing and a step
that costs a feature."

**Provenance.** TWO EXPERTS (`132`, `133`, each deriving and refining independently), confirmed as
surviving by `142c`.

**Caveat.** `133:72-73` and `134b`'s pushback item 3 both note this seam is a fact about the *const-keyed*
route specifically, and dissolves entirely under a structural (type-keyed) encoding, where there is no
rung computation at all, only trait resolution. It is not a neutral frame for comparing every candidate
mechanism, only a true description of one family of candidates.

### 11. The bridge from a written width to a type is consumer-extensible; the cap arvo ships is a convenience, not a forced ceiling

**Claim.** Where a design uses a const-to-type bridge (mapping a written literal width to a corresponding
type), a consumer can populate that bridge for widths arvo never shipped, with no coherence error and no
feature gate, by carrying a marker type parameter through the bridge trait (`impl ToNat<Mine> for
Idx<777>`) rather than implementing a bare foreign trait for a foreign type. An earlier probe (`134`) had
reported `E0117` (a coherence violation) and concluded from it that the bridge must be a capped table
inside arvo; that conclusion does not follow, and it is the marker parameter's absence, not a fact about
Rust coherence, that produced the refusal.

**Where settled.** First compiled at `134c` (a probe by the dispatching agent, explicitly marked "one
probe by one party... it has had no second read"). Independently reproduced on the full construction at
`137:508-514`: "134c showed that a marker-carrying bridge is populated downstream with no E0117... I am
that second read and it reproduces." Never contradicted afterward.

**Provenance.** TWO EXPERTS (`134c` and `137`, with `137` reproducing on the full construction rather
than on the reduced probe).

**Note, important.** This settles a *fact* about coherence (no forced cap), and it settled that fact
without settling the design question it was raised to answer. Op subsequently refused the resulting
mechanism (one populate-line per consumer width) as still being an enumeration in the sense survivor 3
forbids (`137b:30-41`, "the same finding a seventh time"). So the coherence fact stands; the mechanism it
enables was not adopted as the answer to the surface question.

### 12. Both diagnostics for a law violation ship: named-item laws, and a witness set

**Claim.** Two mechanisms ship together for the diagnostic quality of a violated law (for instance a
generic wrapper claiming a wrong output width): (a) laws become named items (a struct with an associated
const, e.g. `ProductFormat<...>::HOLDS`) rather than anonymous `const {}` blocks, so a violation prints
the law's own name and coordinates in the law's own order instead of an anonymous, numbered constant
mixed with unrelated type parameters; and (b) a small, fixed set of concrete instantiation witnesses is
declared per public generic wrapper, discharged in the crate that declares the wrapper and failing that
crate's own `cargo check`, closing the "a wrong generic wrapper compiles until someone downstream
instantiates it" hole.

**Where settled.** `130b:69-80`. Op: "Adopt named-item laws for the diagnostic, adopt the witness set...
Belts and suspenders." Priced and refined at `131:632-741`: roughly four lines per law (one named-item
declaration, two witness lines), the cost curve measured linear and small (0.31s at 64 laws / 4096
checked compositions), and a correction to the naive witness-selection rule (a witness can be a false
negative if chosen carelessly; the correct rule needs two witnesses whose true outputs differ, or one
witness where the claim is against an equality to an input parameter).

**Provenance.** RATIFIED (adoption), with the pricing and the correct selection rule as ONE EXPERT
(`131`, never contested afterward).

**Rests on.** The post-monomorphisation hole being accepted as monomorphisation working as intended
rather than something to design around (`130b:82-92`).

### 13. arvo and notko/hv are separate projects; their axes are not required to correspond

**Claim.** arvo's `Strategy` markers and notko/hilavitkutin's `#[optimize_for]` profile tiers are
different concepts in different projects, sharing synergy but not continuity. A limitation in notko's
scoped rewriter (that it cannot reach through a module-scope type alias, because it operates on token
streams before name resolution) is not an arvo canon finding. Separately: the alias itself carries the
full typestate and is reachable from it, so arvo's own alias story is unaffected by that limitation
regardless.

**Where settled.** `144b:1-27`, op stated twice in one round: "Notko or hv are not directly associated
with arvo. The concepts need not align... They have synergy, but no continuity as such." And again: "arvo
strategy is not the same as notko optimize for profiles. They have synergy, nothing more." The
alias-reach question closed specifically at `144c:16-36`: "This is only a problem on the macro level,
where we do not have typestate... The alias itself does contain the full typestate and is reachable from
there... this thread closes for this panel."

**Provenance.** RATIFIED, twice.

### 14. Canonicity, as originally stated, is withdrawn; what replaces it is narrower and does not identify two different formats

**Claim.** The requirement "two numerals of equal precision are the same type" is withdrawn. It was
inferred from a defect in the deleted tower encoding and, taken literally against the current design,
would make a Q13.3 value and a Q8.8 value interchangeable, which loses the ability to type a scale
mismatch at all (a compiled counterexample decodes a Q13.3 one as thirty-two through this route). What
survives, in the place the withdrawn requirement was actually protecting: two numerals of equal precision
share the same `Precision`, and `Precision` is a type reachable as a bound at a signature; they are not
the same numeral, because they are not the same number.

**Where settled (withdrawal).** `130b:11-30`. Op: "the canonicity was for making I + F evaluate to same
type as F + I, which it probably shouldn't... It seems wrong to me, the premise for this, I can't think
of a use case where the flipped fraction to i place would be meaningfully considered type-equal."

**Where settled (what replaces it, as a bound).** `131:454-474` compiles precision agreement as a bound
checked at a signature (`same_precision(a, b)` where `a: A: Numeral<PRECISION = P>, b: B: Numeral<PRECISION
= P>`), overturning `130`'s own claim that this was unstatable. This specific realisation of "what
replaces it" is the sentence `131:566-574` proposes and marks explicitly as op's to confirm; it was never
contradicted in any later file and no alternative replacement was proposed.

**Provenance.** RATIFIED (the withdrawal itself, `130b`). The specific replacement sentence is ONE EXPERT
(`131`), offered as a reading and never separately ratified by name, but never contested either.

**Casualty attached.** See below: `129`'s precision-keyed numeral and `130`'s written-container surface,
both built to satisfy the now-withdrawn requirement, did not survive as surfaces (though their other
mechanisms, the carry-and-read container discipline and the output-parameter law shape, were kept and
carried forward independently of canonicity).

### 15. The overlap between a by-reference `From` and core's reflexive `From<T> for T` is structurally impossible, not merely untriggered

**Claim.** An `impl<...> From<&Fixed<A>> for Fixed<B>` never collides with core's blanket
`impl<T> From<T> for T`, for any substitution of widths, sign or strategy, because `&Fixed<..>` and
`Fixed<..>` are different type constructors and unification fails at the head constructor, above the
point where any parameter substitution happens. This is a property of the type grammar, not an absence
of a counterexample among cases checked.

**Where settled.** First argued at `146` ("cannot unify," checked at the widths tried). Strengthened to
the structural claim, with the "watertight for a stronger reason" framing, independently, at
`148:527-556`: "Unification of a nominal ADT with a reference type fails at the head constructor... the
argument is not 'no counterexample was found among the cases tried'; it is that the search space has no
counterexample to find." `148` additionally checked and closed the two obvious escape routes (a future
foreign `From<&T> for U` blanket, and orphan-rule well-formedness) before calling it closed. `149:84-87`
records the agreement explicitly: "Two experts now agree on that, independently."

**Provenance.** TWO EXPERTS (`146`, `148`).

### 16. `TryFrom` cannot coexist with the by-reference `From` on the same source shape, and the design does not want it to

**Claim.** `TryFrom<&Fixed<A>> for Fixed<B>` is coherent by itself, but placed beside the by-reference
`From` above it collides with core's own `impl<T, U> TryFrom<U> for T where U: Into<T>`, which the
compiler cannot evaluate to learn which pairs the custom `From` actually covers. This is not treated as a
loss: the design does not want a second, `Result`-carrying fallibility mechanism for numeral conversion
alongside the strategy axis's own fallibility projection (used for `Precise`'s refusals).

**Where settled.** `146`'s enumeration (section 7 dead-routes table, last row): `E0119` against core's
blanket. Independently re-checked in its own spelling at `148:719-724`: "TryFrom does not join, and I
re-checked it in my own spelling on the default solver rather than inheriting it... Its reading of that
as agreement between a coherence wall and a design intent stands."

**Provenance.** TWO EXPERTS (`146`, `148`).

### 17. The gate-free, compliant `From` mechanism: carry the order as a trait bound, not as a computed const argument

**Claim.** The mechanism that actually satisfies op's ratified instruction ("no enumeration, implicit via
blankets and granular bounds where expressing it otherwise fails," survivor 18 below) is: state the
inclusion order between two numerals' widths as a trait bound (`I1: Le<I2>, F1: Le<F2>`), not as a
computed associated-const witness sitting in const-argument position. This compiles on the default
solver, with no `#![feature]` and no `-Z` flag, and works over the design's own width encoding to depths
tested at 128 bits.

**Where settled.** `148:588-625`, correcting `146`'s original spelling, which was shown (same file,
section 6.2) to compile only under `generic_const_args` plus `-Znext-solver=globally`, a configuration
`unstable-features.md` does not permit and which the record elsewhere states is mutually exclusive with
the rest of the arrangement. `149:79-83` confirms: "148 supplies a compliant replacement: carry the
condition as a trait bound rather than as a const argument, which compiles gate-free on the default
solver over the design's own width encoding."

**Provenance.** ONE EXPERT (`148`; `146`'s version does not count toward this specific claim because it
depends on a disallowed configuration, per survivor 15's structural coherence argument standing
separately from the spelling that first carried it).

**Casualty attached.** `146`'s specific spelling (`Picker: EmbedWitness<{ <Pair<...> as Tagged>::TAG }>`)
does not compile inside the permitted configuration and is superseded by `148`'s trait-bound version for
that specific reason, though the coherence argument it made (survivor 15) is unaffected.

### 18. `From`/`TryFrom` on numerals are not blocked by coherence, they are unspelled; "therefore no `From`" is refused

**Claim.** A standing instance of survivor 3, restated specifically for conversions. Op refused the
conclusion that a coherence collision means no cross-numeral `From` can exist. The instruction: "no
enumeration, implicit via blankets and granular bounds where expressing it otherwise fails." Op's own
reasoning for why this should be achievable: a numeral-to-numeral cast is decidable entirely at compile
time and lowers to the same code either way, so there is no runtime question and no missing information;
the obstacle is purely how the impl is written.

**Where settled.** `145b:44-68`. Op, verbatim: "So if, and when, we do the From and TryFrom impls, it's
again, no enumeration, implicit via blankets and granular bounds where expressing it otherwise fails. But
I am not sure what the problem is; From should be clear cut, no? It's a cast and we have all we need to
do it on compile time, all lowered inlined." Reaffirmed unchanged at `145c:66-68` after a narrower
correction to an adjacent, over-broad rule about reading shipped source.

**Provenance.** RATIFIED.

### 19. Diagnostics quality is checked by construction, not by argument, and a specific message shape ships

**Claim.** The message quality for a refused conversion or a violated law is validated by compiling both
positive and negative controls and reading the actual emitted rustc text, not by describing the intended
message. Two concrete shapes were established this way and neither was contested afterward: naming the
target error directly at the consumer's line with both numerals named and the remedy stated in the
design's own words (`error: this numeral does not embed into that one`, survivor 15's route, section
6.4), and `#[diagnostic::do_not_recommend]` on the recursive comparison impls of a type-level width
encoding, which restores that message where a naive recursive encoding would otherwise surface an
internal comparison step instead.

**Where settled.** `148:627-660`, section 6.4, compiled positive and negative controls
(`p4e_binary_negative.rs`, `p4f_do_not_recommend.rs`). Never contested afterward (the panel's last file,
`150`, does not touch it).

**Provenance.** ONE EXPERT (`148`), never contested.

**Caveat, recorded as unsolved rather than resolved.** The same section notes a residual, unfixed
ergonomic cost specific to type-level width encodings: widths print in the encoding's own internal shape
(`Pv<I<O<I<H>>>>`) rather than as the literal a consumer wrote (`13`), and no way to make rustc print the
alias back was found. This is left open, not settled, and is recorded here so it is not mistaken for
solved.

## Casualties

Claims that were proposed, in several cases ratified, and did not survive to the end of the panel.

### The canonicity requirement (full detail on the withdrawal itself)

Proposed as an inference at `126`. Ratified at `127b:22-31` ("Two numerals of equal precision are the
same type. Ratified."). Withdrawn by op's own reversal at `130b:11-30`, on the grounds that the
requirement's origin was a defect in a mechanism (the tower) that had since been deleted, and that,
applied literally, it makes two different scales interchangeable. See survivor 14 for what replaced it.
This is the one requirement in this theme that was proposed, ratified, and later withdrawn by the lead
designer, per the task's standing instruction to watch for exactly this shape.

### `129`'s precision-keyed numeral and its function-like macro surface

`Fx<const P: u32, C: Container, S>` plus `UFixed!(13, 3, u16, Warm)`. Built specifically to deliver the
(then-ratified) canonicity requirement with no feature gate. Killed by two independent defects found
after canonicity was withdrawn: (1) the numeral loses the exponent entirely, so a decode is ambiguous and
a compiled counterexample decodes a Q13.3 one as thirty-two (`130:153-188`); (2) its signed variant folds
the sign bit into `PRECISION`, producing a false multiplicative law that costs a container rung at the
64-bit boundary (`131:745-801`, `132:559-585`, both corrected at `134b:18-44`). Superseded by `130`'s
two-coordinate numeral, itself later corrected. Its macro surface is separately dead per survivor 4.

### `130`'s written-container parameter and its `Format::PRECISION` bug

`Fixed<I, F, C: Container, G, S>` with the container as a written fourth parameter (`UFixed<13, 3, u16,
Warm>`). The container half was refused outright at `130b:39-53` (survivor 1). Its `PRECISION = G::EXTRA
+ I + F` line independently carries the same sign-fold bug as `129`'s, corrected at `131:180-203` and
confirmed by `134b` as "never open" (survivor 9). `130`'s other mechanisms (carry-and-read discipline,
output-parameter laws, the perimeter argument, both diagnostics) were kept and carried forward
independently of the container parameter.

### `130`'s "four families interpreting one contract" section (section 10)

Written with the container as a written parameter, which was refused hours later by `130b`. `130` itself
flagged the section as one expert's first read and asked for a second (`130:704-706`); `131:866-868`
noted it was never redone against the projected container and did not redo it. `138_knuth` independently
rederived the axis/alias framing from scratch (survivor 5) rather than building on it. Its probe
citations were initially reported absent (`138b:41-44`, "there is no `130_probes/`"), then recovered into
the repository at `140b:120-125` along with thirteen other stretches of probes, so the citations
themselves are no longer void; the section's *content* is superseded regardless, by the container ruling
its construction depends on.

### The step A / step B seam as a neutral comparison frame

`132`'s decomposition (rung-selection costs nothing, width-to-rung costs a feature) survives as a true
description of the const-keyed route (survivor 10). It does not survive as a frame for comparing every
candidate mechanism: `133:72-73` and `134b`'s pushback item 3 both note that under a structural (type-
keyed) magnitude there is no rung computation at all, so the seam is an artifact of one encoding choice
rather than a fact about the problem, and using it as a neutral comparison table (which `134`'s brief did)
is flagged as a mistake.

### The GCA container-projection and the structural (Peano/binary) magnitude encoding, as answers

Both were built in detail, both compile (under different, incompatible tradeoffs: GCA needs
`min_generic_const_args` + `generic_const_args` + `-Znext-solver=globally` reaching any consumer whose own
code is width-generic; the structural encoding is gate-free but degrades the diagnostic to an unreadable
digit tower, section 3.3 and 4 of `133`). Neither was ever ratified as the answer. Op's repeated refusal
of enumeration (survivor 3) rules out the plain width table that both routes were built to avoid, but
does not adopt either replacement, and the panel's focus shifted at `142c` to the tiers/inference framing
(survivor 6), which changed what the mechanism needs to satisfy (the explicit spelling is the narrowest,
rarest tier) without choosing one. The fork is explicitly listed as blocked on op and un-dispatched again
by `134b:274-275` and remains open through file `150`.

### The consumer-extensible marker-bridge as the answer to the surface question

The underlying coherence fact survives (survivor 11: the bridge is consumer-extensible, no forced cap).
The mechanism it enables (one `impl ToNat<Mine> for Idx<N>` populate-line per width a consumer wants) was
offered as a resolution and explicitly refused by op at `137b:30-41` as still being an instance of the
same enumeration problem survivor 3 names, "the same finding a seventh time."

### `04b`'s "Thread A", the consumer-facing diagnostic surface for the ten-axis tower encoding

Panellist `04`'s finding (rustc expands type aliases in diagnostics, destroying the error spelling) and
the newtype-faces, then nominal-constructors-plus-per-axis-modifier-types investigation that followed it
across several files. Explicitly kept open by op at `04b:51-60` ("stopping at newtype faces is stopping
too early") through the last consolidation before the rewrite (`124:5587-5599`, still recorded "None of
the three is settled"). Its object of study, the ten-axis tower encoding, was deleted at `126` for
unrelated reasons. Never revisited afterward (no file past `124` mentions "newtype face," "modifier
types," or "nominal constructor"). Not overturned by a ruling; abandoned when its subject was dissolved.

### `144`'s `Ambient` marker mechanism

Built to let a notko scoped rewrite reach through a module-scope type alias. It compiles and its probes
run. Ruled out of scope at `144c:38-41`: "144's proposed mechanism is not needed... it stays in the
record as a worked answer rather than as a proposal, and nothing in arvo's canon should be built on it,"
because the problem it solves belongs to notko, not to arvo (survivor 13).

### `145`'s two-condition inclusion order and its "no new key column needed" claim

Adjacent to this theme via the conversion mechanism rather than squarely in it, noted for completeness
since survivors 15 through 18 depend on the corrected version. `145`'s order (two conditions, over 2,025
pairs, all bias zero) was shown unsound (17,037 false positives on its own sweep) by two independent
derivations (`146`, `148`), each reaching four conditions on its own before reading the other; see
`149:11-33`. Its claim that no new adjudicating-strategy key column was needed for a conversion also does
not hold, per the same two-expert agreement (`149:35-42`). The replacement order and the replacement key
column are both still open at the panel's end (`149`, `150`), so neither is reported as a survivor here;
only the negative (that `145`'s specific claims do not hold) is settled.

## Coverage

Read in full or near-full: `00_context`, `04b`, `06b`, `127b`, `129`, `130`, `130b`, `131`, `132`, `133`,
`134b`, `134c`, `135b`, `137b`, `138b`, `139b`, `140b`, `142b`, `142c`, `144b`, `144c`, `145b`, `145c`,
`149`, and `SETTLED.md` in full. Read in substantial part (the sections bearing on this theme, verified
by section headers and targeted reads rather than skimmed): `137`, `146`, `148`, `150`. That is
twenty-nine files read directly, several of them the longest checkpoints in the panel.

Grepped across all 197 non-probe top-level files with several passes (alias, canonic, ergonom, bridge,
blanket impl, coherence, diagnostic, `From`/`Into`, consumer, domain, container, surface, withdraw,
overturn) to locate theme-relevant material and to verify claims did or did not resurface later; used
those greps to decide which files were worth a direct read versus which could be treated as superseded by
the `126`+ rewrite without individual verification. I did not read consolidations `110` or `124` end to
end (each roughly 7,000 lines); I read the sections other files quote from them directly (`110:3251`,
`110:869-873`, `110:1420-1430`, and similar), and cross-checked several of those quotes against the
consolidation text itself where a citation looked load-bearing. I did not individually read most files in
the `01` to `126` range; I relied on the panel's own repeated, explicit statements that the tower
encoding and the ten-axis framing those files argued over were deleted at `126` for reasons unrelated to
this theme, and I spot-checked that reliance by grepping forward for several of that stretch's specific
terms (canonicity, newtype faces, the width table) to confirm none resurfaced uncorrected. I did not open
any `*_probes/` directory myself; where a survivor rests on a probe, I note the probe directory by name
from the citing file's own reference rather than having opened and re-verified the probe contents.

I did not read `141` (the container-fork bench file) in more than its opening verdict; its subject
(Warm's headroom container selection) sits closer to the strategy-axis theme than to this one, and I
confirmed enough of it to know it does not bear on the survivors or casualties listed above.

The four biggest constraints on what this canon can say, in order: the container is never written by a
consumer and every downstream layer reads it as derived typestate (survivor 1); no enumeration is
acceptable in the public surface, stated and restated as the strongest recurring instinct in the panel
(survivor 3); every named domain type is an alias over one representation, addressed to exactly three
tiers of consumer with a hard ergonomics bar at the tier that matters most (survivors 5, 6, 7); and the
canonicity requirement that shaped five files of numeral-encoding work was withdrawn by its own author,
which is the one clean instance in this theme of a ratification reversing itself.
