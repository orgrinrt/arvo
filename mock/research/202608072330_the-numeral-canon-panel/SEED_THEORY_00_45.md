# Theory mining, files 00-45: the formalization-spec-panel archive

Coverage statement first, because it governs how to read everything below. I read all 45 member
files in my assigned range (`OLD_00_context.md` through `OLD_45_leroy_what_each_claim_rests_on.md`,
including the lettered checkpoints `04b`/`06b`/`08b`/`13c`/`16b`-`16d`/`17b`/`24b`/`30b`/`34b`/`39b`/
`44b` and the two full consolidations `OLD_26_consolidation_two.md` and `OLD_40_consolidation_three.md`)
end to end, in full, not skimmed. I opened the probe directory listings for every member file in
range (`OLD_02_probes/` through `45_probes/` inclusive) and spot-read probe source for the files whose
claims this document leans on most (02, 05, 08, 10, 13, 17, 19, 20, 23, 25, 33, 34, 35, 36, 37, 38,
41, 42, 43, 44), rather than compiling any myself. I did not re-run any probe. Everything below that
is stated as a fact about a probe is a claim I am relaying from the member file that ran it, one
level removed from the compile, exactly the status this archive itself is entitled to under the
panel's own rules: reasonable to name because it is committed and checkable, not something I have
independently reproduced.

I did not open `mock/design_rounds/202607301200_topic.the-formalization-spec.md` or any of the
inherited-state / talk transcripts the member files cite by line. Every citation of that spec text
below is therefore a citation of a citation, at one further remove than the member files' own, and
should be re-opened before anyone treats a "the spec says X" sentence below as settled. I read
`INTENTS.md`, `RULES.md`, part of `OPTIONS.md` (the question headers plus the full text of Q4-Q7,
Q9, Q12, and Q17, to locate where this archive's findings bear on live questions) and `DROPLIST.md`'s
section headers, not its full body.

**A necessary disambiguation, stated once here because it governs the final section below.** The
current numeral canon panel (this directory) has its own member files, numbered from `01` onward,
running its own independent dive and citing its own probes (its file `35`, `42`, `55b`, `58`, `60`,
and so on, none of which I have read). Those numbers collide with the archive's own numbering. Every
citation in this document of a bare number belongs to the archive at
`202607301300_formalization-spec-panel/` unless explicitly marked "current panel". Confusing the two
numbering schemes is a live risk for the next reader and I have tried to make every citation below
resolve unambiguously by always saying "archive file NN" in the cross-reference section.

## What this archive is, structurally, and why that matters for how to use it

This is not one panel. It is a sequence of roughly five sub-panels, each ending in a consolidation,
each explicitly instructed by op to build on the last rather than restart. The shape matters because
the same claim was frequently stated, refuted, corrected, and restated across the arc, and the
member files are unusually disciplined about naming which version is current. Reading only the two
consolidations (`26`, `40`) would recover most of the settled content faster than reading all 45
files; reading only the member files without the consolidations would recover the same content with
far more noise and several superseded intermediate states. I read both because the brief asked for
the theory the members reached for, which the consolidations compress out along with their working.

The arc: files 01-11 review a one-day-old draft spec (numeral/policy/lowering, ten axes) by
compiling against it (11 is a design-round document restating the shape, not a persona file, written
by whoever ran the round rather than a panellist). Files 12-26 are a second review focused on the
algebraic-laws question, ending in `OLD_26_consolidation_two.md`. Files 27-40 rebuild the *identity*
half of the design from first principles (what a number is, radix, precision, exponent form,
encoding) and re-derive the laws against it, ending in `OLD_40_consolidation_three.md`. Files 41-45
harden two loose ends of that consolidation: the rational `Bias` encoding's perimeter (41, 42),
division (43), and a provenance-tracking discipline for the review's own claims (44, 45), gated by
op's tenth checkpoint (`44b`).

**The single most load-bearing methodological fact in the whole archive, stated by the archive
itself repeatedly and worth restating because it is exactly what this dispatch's brief warned
about**: a claim that "compiles" or "was checked" is worthless without knowing *what it was checked
against*, because the design's own coordinates moved underneath several claims and the claims did
not notice. File 44 (`OLD_44_ringer_what_the_overturn_left_behind.md`) is an explicit audit for this
failure mode and finds three real instances plus one near-miss; file 45 proposes and executes a
fix (a `grounded on: <slug>` provenance field). This is not incidental to the theory-mining task,
it is a warning shot for how this document should itself be read: a finding cited below as
"current" in the archive's own accounting was current as of file 45. Nothing in this document
updates that.

---

## Part 1: Theory referenced (checkable against the literature, independent of the panel)

This is the category the correction from op elevates. Everything below can be verified by opening
the cited source, independent of any authority in this workspace.

### 1.1 Kulisch's theory of computer arithmetic (file 01, section 14)

File 01 (Knuth persona) proposes that the spec's "quantisation is one map, laws are derived from
its properties" reinvents, in pieces, Ulrich Kulisch's framework: a machine operation is
`a (op') b = round(a op b)`, where `round` is a monotone idempotent projection from the exact
structure onto the representable subset, with laws derived from three axioms on the projection
(idempotence on the subset, monotonicity, sign symmetry). This is the single most-cited piece of
external theory in the whole archive: files 05, 07, 10 and 33 all independently reach for it or
cite file 01's naming of it as the frame that the "round-then-classify" quantiser pipeline
(settled by file 28, see 1.4 below) instantiates "verbatim". **Checkable claim**: Kulisch's
projection-with-properties construction is standard in interval/verified computer arithmetic
literature; the specific citation of "round then resolve" as this construction is the panel's
own reading and should be checked against Kulisch's actual axioms (idempotence, monotonicity,
sign symmetry) rather than assumed to match. File 04 (Torvalds persona) partially dissents on
scope: "adopt the frame where it deletes something ... and cite it in prose everywhere else,"
warning against a vocabulary rewrite for its own sake.

### 1.2 Partial-algebra equational hierarchy: weak, existence, and Kleene equations (file 33)

File 33 imports a "hundred years old" vocabulary it attributes to Kleene's usage as
"systematised by Burmeister for partial algebras": three strengths of equation over a partial
operation. Weak equation (`t1 =w t2`): if both sides are defined, they are equal. Existence
equation (`t1 =e t2`): both sides are defined, and equal. Kleene equation (`t1 ~= t2`): both
defined and equal, or both undefined. This is presented as *import, not invention* -- "nothing
there needed inventing, only importing" -- and file 33's own section 1.1 states the three
relations precisely, with the weak equation explicitly noted as reflexive and symmetric but
**not transitive** (a standard fact about partial algebra, not a design finding), and the
consequence traced through: the class-quantified statement ("regroupings that return, return the
same value," over a whole grouping class rather than a pair of terms) is what restores
transitivity, which file 37 later calls "the sharpest vindication of file 33's five-slot frame"
because it shows two of the "slots" (relation, quantifier) interact in a way neither alone
predicts. **Checkable**: Burmeister's partial-algebra terminology (weak/existence equation) is
real published mathematics; the specific attribution to "Kleene's usage" for the third should be
checked (Kleene's three-valued logic gives the natural reading of the "both undefined" clause,
but the panel's phrase "Kleene equation" as a name for this specific relation is the panel's own
naming, not necessarily standard terminology, and this matters for anyone trying to locate the
term in a textbook).

### 1.3 Tropical / max-plus and min-plus algebra, dioids (files 14, 15, 33)

File 14 (Dolan persona) names the standard reference literature directly: Baccelli, Cohen,
Olsder, Quadrat, *Synchronization and Linearity* (the canonical max-plus algebra reference), and
Gondran and Minoux, *Graphs, Dioids and Semirings* (2008), specifically citing the latter as
about exactly arvo's own graph-algorithm use case. It also cites a prior-art research pass in the
same workspace (`202607281616_prior_art/04_algebraic_structure_hierarchies.md`) which had
already found "tropical algebra ... connects to the shortest-path and scheduling work
`arvo-graph` already does," citing PALMA (a fixed-point integer tropical-algebra library for
embedded ARM, no-alloc) as "the only surveyed algorithm family that arrived already satisfying
the constraints." This is a **theory-plus-prior-art convergence worth flagging**: the same
mathematical structure (max-plus/dioid) was independently reached by two different research
efforts inside this workspace, at different times, from different starting points (one from the
algebra dive's own measurements, one from a library survey). File 33 (Lamport persona) then
compiles a direct test: no shipped preset (`Hot`, saturating `Warm`/`Cold`, `Precise`) satisfies
the dioid axioms over `(max, +)` -- wrapping addition is associative but does not distribute;
saturating addition distributes but is neither associative nor annihilating (a compiled
counterexample: `sat(-8, 3) = -5`, so the bottom element does not annihilate); `Precise` addition
is partial, so it is not a total semiring at all. File 14's own theorem (see 1.5 below) explains
this precisely.

### 1.4 IEEE 754-2019 and Flocq's floating-point formalization

Recurring, precise citations to specific clauses of the current standard, used both as an
external oracle to check the design against and as a source of vocabulary. Named clauses:
clause 3.2 (format parameters: radix, precision, emin, emax), clause 3.5 (encodings, e.g.
decimal's BID/DPD dual-encoding-of-one-format), clause 4 (rounding-direction attributes, chosen
per operation not per format), clause 5.10 (`totalOrder`, `maximum`/`minimum` vs
`maximumNumber`/`minimumNumber`, the 2019 revision's replacement for 2008's `minNum`/`maxNum`),
clause 6.3 (the sign of an exact zero result, e.g. `x + (-x)` under `roundTowardNegative` is
positive zero under every attribute except that one), clause 7 (status flags: sticky,
accumulated, and the invalid/divideByZero distinction between `x/0` for finite nonzero x and
`0/0`). Flocq (the Coq formalization CompCert's floating-point reasoning is built on) is cited
by file 28 (Leroy persona, who has first-hand professional history with it) as the source of the
"two-coordinate" architecture the identity contract eventually adopts: a semantic type
parameterised by radix, precision, exponent bound (its own `FLX`/`FLT`/`FTZ` triple for
unbounded/gradual/flushed underflow), kept entirely separate from an encoding layer (field
widths, hidden bit, exponent bias, reserved codes), with the correspondence between the two
stated as **round-trip theorems** rather than assumed. File 30 later finds file 28's stated
version of these theorems self-contradicting (see Part 3 below) and repairs it to a
section-retraction triple. **Checkable, high value**: every one of these clause citations is
independently verifiable against the published standard text, and several member files (30, 31,
39) explicitly state they wrote independent oracle functions from the standard's own text (not
from any panel member's paraphrase) before checking agreement, which is exactly the discipline
the correction from op asks for.

### 1.5 Ordered algebra: torsion-free groups, lattice-ordered monoids (file 14)

File 14 proves, not merely cites, a structural theorem, from standard group theory: an
orderable group must be torsion-free. If `(G, +, <=)` has a total order compatible with `+`
(translation-invariant), and some nonzero element `x` has finite order `n`, then (WLOG `x > 0`)
`0 < x < 2x < ... < nx = 0`, a contradiction. Wraparound addition on a fixed width is exactly the
finite cyclic group `Z/2^N Z`, every nonzero element of which has finite order, hence it is a
torsion group, hence **no total order is ever compatible with it, at any width, signed or
unsigned**. This is a proof, not a search result, and file 14 states it explicitly replaces what
an earlier member (file 13) had found only by exhaustive search at small widths: "this is
stronger than what either probe shows by search: it is a proof that the search could not have
come out any other way, on any width." A second theorem is stated as standard rather than proved
in full: a monotone (order-preserving) total map fixing an interval pointwise is uniquely the
nearest-point retraction onto that interval, i.e. clamp. File 14 further cites Fuchs, *Partially
Ordered Algebraic Systems*, as "the classical reference for the whole family: ordered semigroup,
ordered monoid, ordered group, lattice-ordered group," and frames "restricting an unbounded
totally-ordered abelian group to a bounded window" as having exactly two structurally different
completions: quotient by a subgroup (mod-n, preserves the operation, destroys order because a
torsion group cannot carry one) or retract onto a convex sublattice (clamp, preserves order,
destroys associativity on a two-sided interval). This dichotomy (wrap XOR clamp, never both
properties at once, on a signed domain) recurs across the whole archive and is one of its most
solid, independently re-derivable results.

### 1.6 Equality saturation, e-graphs, ægraphs (file 15)

File 15 (Willsey persona, whose own field this is) is explicit about scope: the question "should
arvo build an e-graph" is answered no, on structural grounds (`no_std`, no `alloc`, no growing
term DAG or union-find is expressible), but the *field's* answer to "which of several
algebraically-equal regroupings is fastest, checked before committing" is cited precisely: Tate,
Lattner et al., "Equality Saturation: a New Approach to Optimization," POPL 2009 (the
`a*(b+c)` vs `a*b+a*c` expand/factor example is named as the field's canonical motivating case
for why greedy single-direction rewriting misses optima either direction alone would find). Chris
Fallin's own ægraphs paper is cited: "ægraphs: Acyclic E-graphs for Efficient Optimization in a
Production Compiler," CGO 2023, describing Cranelift's mid-end optimiser as an acyclic e-graph in
egg's lineage, replacing single-pass greedy peephole rewriting. File 15's conclusion (the atomic
facts the design derives are "the applicability-condition oracle a rewrite system would need,
not a rewrite system") is a genuine theory-vs-engineering distinction worth separating from the
panel's opinion of it: the *congruence-versus-extraction* separation ("is this regrouping legal"
vs "which legal regrouping is fastest") is standard e-graph vocabulary, used here to diagnose a
real found bug (`hilavitkutin`'s `ConvergenceBuffer::combine`, see Part 3) as "a missed
equivalence... precisely what an unmerged e-class looks like when you translate it out of my
field's vocabulary."

### 1.7 IEEE 754's own historical bug and fix: minNum/maxNum vs minimum/maximum (files 05, 34, 39)

Cited three separate times, independently, as external corroboration for a design finding: file
05 identifies that a naive total-order-based selection silently discards an absorbing "bottom"
value, and states this is "precisely the defect IEEE 754-2008 shipped in `minNum` and `maxNum`,
which return the non-NaN operand and therefore discard the very thing they were supposed to
propagate, and which 754-2019 replaced with `minimum` and `maximum` that propagate." File 34
independently rediscovers the same structural distinction (a strict, poisoning family vs a
suppressing family) while splitting a distributivity theorem, citing the same 2008-to-2019 IEEE
revision as the reason both variants must be in the design's vocabulary rather than only one.
**Checkable**: this is real, published standards history (IEEE 754-2008's `minNum`/`maxNum` were
deprecated and replaced in the 2019 revision specifically for NaN-propagation reasons), and the
panel's use of it as a precedent for "a naive `TotalOrd`-based selection is the wrong contract
for algorithm crates that must not lose an absorbed value" is a real, externally-grounded
argument rather than an invented analogy.

### 1.8 Fixed-point signal-processing literature: rounding bias, error feedback, dither (files 24, 29)

File 24 (Smith persona, citing decades of DSP practice) states, without further citation but as
established field knowledge: multiply-accumulate hardware carries guard bits sized to the trip
count (the Motorola DSP56000's eight guard bits above a 48-bit product are exactly
`ceil(log2(256))`, sized for 256 MAC steps); the field's standard error-bound tooling is Higham's
`gamma_n` bound for compounded relative rounding error (`n*u / (1 - n*u)`); and per-stage
renormalisation, block floating point, and log-domain representation are the field's standard
disciplines against exponential dynamic-range growth in nested products. File 29 (Wronski
persona, citing the same literature by name) cites Lipshitz, Vanderkooy and Wannamaker's 1984
survey as "the standard citation" for dither theory: rectangular-PDF dither decorrelates the
*mean* of quantisation error from the signal (removing harmonic distortion/banding) but not its
*variance*; triangular-PDF dither (sum of two independent rectangular draws) removes both, at a
further known noise-floor cost. First-order error feedback (Floyd-Steinberg diffusion in
imaging, delta-sigma noise shaping in audio/RF converters) is named as the field's standard
weapon against sustained quantisation drift, contrasted against the toward-negative-infinity
truncation's linear DC ramp (measured at roughly `-K/2` quanta after K operations, a real
"textbook DC ramp" / limit-cycle mechanism). File 29 is explicit about where its field's
guarantees do **not** transfer: every cited decorrelation theorem is proven for a *uniform*
quantiser (constant quantum), and whether the classical guarantees survive a *variable* quantum
(a float composition crossing a binade boundary mid-sequence) is stated as "a genuinely open
question in my own field as far as I know it," with no citation offered. This is a rare, valuable
instance of a member naming the precise boundary of a field's own established results rather
than over-claiming.

### 1.9 Type-level number encodings: Coq's `positive`/`N`/`Z`, and `typenum`'s Stein's algorithm (file 36)

File 36 (Kiselyov persona) builds the value-unique width/precision/adjustment tower directly on
Coq's `positive` (`Pos ::= H | O<P: Pos> | I<P: Pos>`), `N` (`Nat ::= Z | Pz<P: Pos>`), and `Z`
(`Int ::= Z0 | Zpos<P: Pos> | Zneg<P: Pos>`) type families, attributed to "Barras et al." (the
Coq kernel authors). This is a genuine literature import, chosen specifically because the leading
digit is the terminator in this encoding, so there is no representable position for a leading
zero, which is what makes the value-uniqueness proof structural (an induction, with no
normalisation operator required) rather than an algorithm. File 36 also reports reading
`typenum`'s actual source (`typenum-1.20.1/src/uint.rs`, `src/private.rs`) rather than assuming
its algorithm, and corrects its own working assumption after doing so: `typenum`'s `Gcd` is
**already Stein's binary algorithm**, not Euclid's remainder-based algorithm as one might assume
from the crate's public API alone. The panel's own claimed speedup over `typenum` (measured, see
Part 2) is therefore attributed entirely to the value-unique encoding making three of Stein's
five steps pure type-constructor selection, plus eliminating a halving step in the odd/odd case,
never to any algorithmic novelty. **Checkable, and already checked once inside the archive**:
this is one of the few places a member explicitly went and read a real crate's source before
building a comparison against it, rather than assuming what the comparison would find.

### 1.10 Exact division by an odd divisor: Jebelean / Hensel form (file 36)

File 36 names the classical algorithm behind the type-level rational reduction's division step:
"exact division by an odd divisor has a classical least-significant-digit-first algorithm
(Jebelean's exact division, the 2-adic or Hensel form, used in multiprecision libraries for
exactly this reason)," and states precisely why the chosen encoding (least-significant-digit
outermost) is unusually well-suited to it: each step is one parity match, one subtraction, one
structural halving, with no comparison and no trial-digit retraction, because exactness is a
precondition rather than something discovered mid-algorithm. This is contrasted explicitly with
`typenum`'s own `Div`, described as "MSB-first long division with a comparison per digit."
**Checkable**: Jebelean's exact-division algorithm and its Hensel/2-adic framing are real,
published multiprecision-arithmetic literature; the specific comparison to `typenum`'s division
direction is the panel's own reading of the source it says it read.

### 1.11 DOT calculus and path-dependent types (file 22)

File 22 (Amin persona, the DOT calculus's own co-author) frames Rust's associated-type
projection through a bound (`F::N` reachable wherever `F: Numeric` is in scope) as "Rust's
nominal approximation of a DOT-style abstract type member projected through a path," explicitly
naming the correspondence: `F::N` is `F.N` spelled with `::`, upper-bounded the way a DOT member
is, reachable from any context holding the path's prefix. This is offered as a precise technical
correspondence, not a loose analogy, and is one of the only places in the archive where a member
names the *type theory* underneath a Rust mechanism rather than treating the mechanism as
Rust-specific. The same file states plainly where the correspondence stops: Rust has no true
dependent types (a type parameterised by a runtime value; the const-generic substitute reaches
only compile-time-known values in restricted positions) and no higher-kinded abstraction
(genericity over a type constructor), and states as its own finding that the second absence was
checked directly against this design and "does not bite anywhere the ten axes currently ask
for" -- a negative result, explicitly not merely assumed.

### 1.12 Staging and binding-time analysis: Lightweight Modular Staging (file 21)

File 21 (Rompf persona, LMS's own author) applies binding-time-analysis vocabulary to the
design's key-parameter question (a derived fact's "stage" is when each of its parameters becomes
known: type-write time, operation-apply time, fold-run time), then, unusually, **retracts most of
the analogy's payoff in its own section 10**: "there is no residualisation, so this is not
staging in the technical sense... What transfers is the binding-time analysis, which is
bookkeeping about which parameters are known when. What does not transfer is the
specialisation, which is where the value normally is." The file goes on to state that the real
mechanism doing the work is not staging theory but the compiler's own refusal of
`generic_const_exprs` (see Part 3), read as a binding-time discipline the language already
enforces rather than a wall to work around: "arvo is not missing a staging discipline. It is
fighting one it did not choose." This self-correction, a member importing a field's vocabulary
and then honestly reporting how much of the field's actual payoff survives the import, is
methodologically the strongest instance of exactly the discipline op's correction calls for:
distinguishing the literature's genuine content from the panel's own borrowed framing.

### 1.13 MATLAB Fixed-Point Designer, SystemC (IEEE 1666), as literature rather than folklore

Multiple files (13, 24, 30, 39) treat MATLAB's Fixed-Point Designer and SystemC's `sc_fixed` not
as folklore-level "vendor conventions" but as concrete, checkable specification text. File 39
cites MathWorks' own documentation pages directly (URLs given): "Compute Slope and Bias"
(stating slope and bias "can take on any value") and the `numerictype` reference (stating the
slope-adjustment factor lies in `[1, 2)` with automatic renormalisation). This literature
citation directly *falsifies* an intermediate panel claim (`Bias` as a plain signed integer,
file 36) by comparing it against the vendor's own stated domain, which is the strongest single
instance in the whole archive of "the literature check found the panel wrong," discussed further
in Part 3. SystemC's construction (`sc_fixed<W, IW, Q, O, N>`, per-mode-name table: `SC_TRN`,
`SC_RND`, `SC_RND_CONV`, `SC_WRAP`, `SC_SAT`, `SC_SAT_ZERO`, `SC_SAT_SYM`) is spelled precisely
in file 39's probe against the merged shape, and file 39 records one construction the archive
explicitly never built: `SC_WRAP<n_bits>` and `SC_WRAP_SM<n_bits>` for `n_bits > 0` (wrap while
keeping the top n bits saturated), flagged as an unconstructed cell of the design's own "no
gaps" claim.

---

## Part 2: Proved or measured (with the probe path, so a re-test is cheap)

Every entry below names the member file and the probe directory the panel committed. None of
these have I re-run; they are relayed at one remove, as the panel's own conventions ask.

### 2.1 The torsion-group/clamp dichotomy, compiled at small models

Files 13 and 14. `OLD_13_probes/01_stability_vs_associativity.rs` enumerates all 65536 total
recovery maps fixing a signed `[-2, 1]` representable set over an exact domain `[-6, 5]`: exactly
1 monotone map (the whole monotone family is `clamp`), exactly 1 translation-stable map (wrap),
1024 fold-associative maps, 1023 associative-but-not-stable (fine for a fold, refused by the
draft's over-strict criterion), and **0 stable-but-not-associative** (the soundness check for
the criterion). `OLD_14_probes/02_monotone_equals_stable.rs` and `03_unsigned_saturate_both.rs`
independently reproduce the same disjointness across four signed/unsigned model widths, and
confirm the theorem (1.5 above) generalises: unsigned saturating addition is *both* associative
*and* order-monotone (no conflict for a one-sided clamp), while signed wrapping is associative
but not monotone at any width tested.

### 2.2 The multiplicative half's exact-product mechanism, and its measured cost

Files 24, 25, 34, 35, 36. `OLD_25_probes/03_typelevel_binary_addwidth.rs` builds a type-level
binary width adder (`UTerm`/`UInt<Hi, Lo>`) with no unstable feature, exhaustive at a 2-bit
model (16 pairs, all compile-time asserted) and checked at realistic scale (13+7=20, 3+2=5,
the exact widths a `UFixed<13,3>` * `UFixed<7,2>` product needs). `OLD_25_probes/05_composed_exact_
product.rs` disassembles a concrete `mul_full` instantiation to exactly four instructions
(the standard `umulh`/`madd`/`madd`/`mul` widening-multiply sequence), with zero symbols
referencing the phantom width types anywhere in the emitted output. File 24's own probe 01
(`OLD_24_probes/01_the_mac_discipline.rs`) measures a per-operation-quantised fold's grouping
diameter reaching 15 raw units on a 16-value range by a five-element fold (per-op discipline)
against a wide-accumulator fold's diameter of 0 at every arity by construction. File 35's probes
1 and 3 re-run the same three-way fold (direct wrapping multiply / composite `mul_full`-then-
`quantize` / exact-widening call) at native width (all three fold to one instruction) and at a
genuinely harder multi-limb width (128-bit operands, real 256-bit intermediate, where a
truncated result needs fewer limb-products than a full one in principle); measured, all three
still fold to the direct hardware multiply's four instructions once the optimiser can see
through the composition, and forcing the composite opaque (`#[inline(never)]`, a negative
control) pays a real, non-folded cost (24 lines against 7).

### 2.3 The value-unique numeral encoding, compiled and priced

Files 36, 41, 42. The core result: under Coq's `positive`/`N` encoding sealed with a private
supertrait, uniqueness holds by induction with **no normalisation operator anywhere in the
design**, because there is nothing for one to do (`OLD_36_probes/vu_nat.rs`, `probe_2`/`2b`/`5`/`5b`).
Priced (`--emit=metadata`, min-of-3, 400 compositions, 16-bit operands): the gcd (Stein's, on
this encoding) costs 5.08 ms/composition against `typenum`'s own measured 15.55 ms, a 3.06x
win decomposed into 1.65x from algorithm formulation and 1.87x from the encoding itself; full
reduction (gcd + exact division) costs 12.07 ms; over dyadic adjustments (every composition
arvo ships today) the cost drops to 0.50 ms because the gcd terminates on its first impl; zero
symbols emitted at any composition count (`nm -g` checked). File 41 built the rational-bias
extension of the same tower and priced it separately: at a comparable 8-bit-operand width the
bias magnitude alone costs 13.61 ms/composition (essentially the same order as the primitive it
composes), the full sign-plus-magnitude composition 19.10 ms; at a harder, previously-unswept
16-bit width, 102.60 ms and 159.42 ms respectively; over dyadic magnitudes, ~1.55 ms.

**A real defect found and repaired inside this sub-thread, worth flagging on its own**: file 41's
own passing sentence in an earlier file (file 34) claiming "the shipped width chain already
satisfies [value-uniqueness]" was itself false, compiled as false by file 36
(`UInt<UTerm, B0>` inhabits `Width` with value zero exactly as `UTerm` does, a second spelling;
the adder propagates the spelling rather than normalising it, refused with `E0308` one layer
below where the same defect was first found).

**A perimeter defect found by a *later* member attacking an *earlier* member's own construction,
worth flagging as the sharpest single instance of the "verify, do not trust" discipline in the
archive**: file 41 built `Bias`, sealed it correctly, and *believed* the fix for `Adjustment`'s
missing seal (which it also found) was the whole story. File 42 (a different persona, same
brief-required rebuild-fresh discipline) went one layer further and found that `Bias`'s own
blanket impl composes with **unsealed** `Pos`/`Nat` -- the seal that "exists" lives only in a
standalone demonstration file (`OLD_36_probes/probe_5_sealed_perimeter_lib.rs`) that nothing else in
the review actually imports; the tower everyone composes with is an unsynced, unsealed copy.
File 42 compiled the attack (a fabricated `Pos` type with a `Gcd` impl that claims unconditional
coprimality with no computation performed, constructing an unreduced `BPos<Fabricated, D4>`
denoting 4/4, a value nothing in the design ever verified was reduced) and confirmed the same
attack defeats `Adjustment` directly, with no `Bias` involved at all, meaning file 41's own
recommended fix (seal `Adjustment` itself, mirroring `Bias`'s seal) would **not** have closed
this, because the attack never implements `Adjustment` in the first place. File 42's actual fix
(seal `Pos`/`Nat` at the layer everything composes with) closed both attacks, measured at
statistically indistinguishable compile cost from the unsealed tower (15.486 ms/composition
against 15.407, well inside noise).

### 2.4 The finest-view lattice, compiled

File 37 (`OLD_37_probes/probe_1_the_ladder_is_a_view_lattice.rs`). A signed three-bit numeral, a
resolution per range end, a four-element fold over all five groupings, all pairs, all 4096
inputs, at nine views (three detail levels for each of two generator classes), roughly 65
seconds of const evaluation: both closure properties asserted directly rather than argued
(downward closure: a law holding at a finer view holds at every coarser one; join closure: a
law holding at two views holds at their join), which together establish that every law has a
unique finest view. The archive's own headline finding here: `Hot` on a signed numeral and
`Precise` below its accumulator's interior-safety threshold sit at **incomparable** points of
this lattice (one preserves values and definedness while losing quantisation events, the other
preserves values and events while losing definedness), which is offered as the reason a
three-name "weak/Kleene/graded" fork had resisted resolution across several prior files: the
vocabulary had no name for the point either preset actually occupies. Priced against the
five-derived-marker-trait alternative file 33 first proposed: 0.130 ms/composition and 907
bytes against 0.193 ms and 1854 bytes, at `--emit=metadata`, with the marker shape covering
eight combinations of which five are unreachable ("junk").

### 2.5 The overflow band's per-operation-family membership, compiled and corrected twice

Files 30, 31, 33, 39, 43, 44. The band (the region between the largest representable value and
half a quantum past it, where "round then classify" and "classify then round" disagree) was
measured empty for same-format addition (files 28/30/31) and inhabited for multiplication
(files 30/31, reproduced independently by file 33). A carried consolidation sentence then
asserted it "inhabited for multiplication, division, mixed-format addition and every float
operation" with only the first two members ever compiled. File 43 compiled division and found
the blanket claim **false**: empty for same-precision division at every precision from 2 to 8
bits tested, inhabited only once operand and result precisions decouple (as the ratified MATLAB
`SpecifyPrecision` requirement makes first-class). File 44, dispatched specifically to hunt for
this failure pattern, compiled the third member (mixed-format addition,
`OLD_44_probes/probe_1_the_overflow_band_for_mixed_format_addition.rs`): the band **can** be
inhabited (two independently-shaped witnesses), but is **not unconditional** (a 40-triple sweep
splits 36 inhabited / 4 empty, with the 4 empty cases structurally distinct: one operand's
quantum divides the other, collapsing the "mixed-format" pair into disguised single-quantum
arithmetic). The fourth member ("every float operation") was never checked by anyone at any
point in the archive and file 45 recommends it be struck from the sentence rather than carried
as "unverified," on the grounds that "a member with no findable derivation is not a weak claim,
it is not a claim."

### 2.6 The quantiser's round-first amendment against IEEE 754-2019 clause 7, directly

Files 28, 30, 31, 39. File 30's `OLD_30_probes/probe_2_ieee_overflow_falls_out_of_round_first.rs`
checks, at a model float (radix 2, precision 3, emax 2), exhaustive agreement over reals 1
through 9 against three oracles written **independently from the standard's own text, not from
the design's pipeline**: `roundTiesToEven`, `roundTowardZero`, `roundTowardPositive`. All three
agree exactly, including the boundary case, with no new axis and no new resolution. File 31
independently recompiled the same probe rather than trusting `OUTCOMES.md`, and extended it to
the signed case, which none of the prior probes had covered: under `roundTowardPositive`,
positive overflow correctly delivers `+infinity`, deep negative overflow correctly delivers the
**negative largest finite** rather than `-infinity` (rounding toward positive never selects the
more negative of two candidates), confirming the pipeline is asymmetric in exactly the way the
standard requires and the design's earlier symmetric intuition would have gotten wrong.

### 2.7 The generic_const_exprs / generic_const_args compiler wall, hit and mapped repeatedly

Files 02, 19, 25, 36, 41, 42 all independently rediscover variants of the same refusal. The
canonical statement (file 25, extended by file 36): computing a type-level width sum as
`type const OUT: u16 = A + B` under `min_generic_const_args` (the permitted successor to the
forbidden `generic_const_exprs`) refuses at the definition site with `generic parameters may not
be used in const operations`, and the compiler's own suggested remediation
(`#![feature(generic_const_args)]`) names a feature that is neither the permitted
`min_generic_const_args` nor the forbidden `generic_const_exprs`, and is itself unvetted under
the workspace's own `unstable-features.md` procedure. File 36 records trying the "obvious"
fourth escape (project a trait-level computation back down into an ordinary const parameter, so
value-uniqueness could be dodged entirely) and having it refused the same way, closing that
route explicitly rather than leaving it as an open temptation for a later member. File 41 and 42
independently hit a **different**, previously unrecorded wall in the same family while composing
`Reduce` generically: naming a trait with exactly one matching blanket impl as a bound (rather
than as a concrete alias) forces the solver to select and eagerly discharge that impl's own
where-clauses, recursing without a base case for an abstract input (`error[E0275]: overflow
evaluating the requirement ...`). File 42 isolated this further than file 41 had: the trigger is
narrower than "any wrapper position diverges" (file 42 built synthetic counter-examples of bare
wrapper positions that do *not* diverge), and file 42 additionally corroborated the divergence a
second, independent way: raising `#![recursion_limit]` per the compiler's own suggestion does
not produce a deeper clean answer, it **crashes the compiler** (SIGBUS inside
`rustc_trait_selection`'s `OpportunisticVarResolver`), reproduced identically twice. File 42's
own repair (spell the constituent where-clauses of `Reduce` directly rather than naming `Reduce`
itself as a bound) let it build the exact generic `BiasMul` trait file 41 had concluded could
not exist -- a direct, compiled contradiction of a predecessor's stated conclusion, corrected in
the next file.

### 2.8 arvo's own shipped `Monotone` law and `arvo-num-systems`, checked against the tree

File 18 (Lamport persona) checks a passing sentence in the consolidated draft ("the one shipped
`Monotone` law implementation...") against source and finds it **does not exist as any
implementation anywhere in `mock/crates/`**; the only two grep hits are a test function name and
an unrelated doc comment. What exists is an unlocked, open design-round proposal, and checked
against its own admitted compositions it is **false**: `ReduceModulo`/`ReduceModulo` gives
`phi(-24) = -4 > phi(-8) = -8`, a monotonicity violation the same torsion-group theorem (1.5)
already rules out for every width. Two prior members (13, 14) had built on the false "shipped"
claim before file 18 caught it with a single grep, and file 19 (Ringer persona) names this the
sharpest instance in that stretch of "prose about the design's own state, checked by nothing."
File 39 independently opens and reads `arvo-num-systems`'s own topic file (rather than
summarising it through eight prior members' flags) and finds it already carries the reading
("inhabits, not equals") that two panel members had, unread, re-derived worse from scratch as a
"vacuity worry" against D39; file 39 also finds the topic's own agent-derived inhabits table is
stale against the ratified identity contract in three separate ways (every fixed-point type
credited to Z[1/2], missing the radix-ten case, the `FullRange` case, and any rational bias).

### 2.9 Vectorisation, erasure, and the check-build/shipping-build flag distinction, measured repeatedly

Files 20, 23, 32, 34, 35, 38. File 20 first measured that monomorphisation prints the full
composition into the v0-mangled symbol table (not erasing it), but that this channel is empty
in an ordinarily-inlined shipping build; file 23 corrected this to "the channel is legible in
whatever build you ask for it in" by adding `-Cno-prepopulate-passes -Zinline-mir=no` (defeating
*both* of rustc's two inliners, MIR-level and LLVM-level, which file 23 found act
independently). File 32 hit an apparent vectorisation anomaly (a loop that autovectorises
standalone stops vectorising embedded in a larger crate) and could not pin the cause within its
own dispatch. File 34 diagnosed it as a pure methodology artifact: `-C lto=fat` on an unlinked
`--emit=asm` build defers the loop vectoriser to the link-time LTO backend, which never runs
without an actual link step, so the "anomaly" affected every function in every crate under that
flag combination, including a byte-for-byte copy of the vectorising control. File 35 and file 38
each independently re-confirm the corrected discipline at a second operation (multiplication)
and a second, rebuilt encoding, respectively, each time finding the identity-contract path and
a hand-written baseline fold to **byte-identical machine code, one symbol address**, under the
build shape a consumer actually ships.

### 2.10 Division's finite accumulator, tested and found to overturn a prior prediction on its own coordinates

File 43. `OLD_43_probes/01` tests the consolidation's own carried prediction ("no finite accumulator
solution exists for division at all") directly, and finds it **true in the coordinates it was
originally stated in** (dyadic-only adjustments, where 2^F is never divisible by 3, checked by
residue iteration to F=1000) and **false in the ratified coordinates** (rational adjustments): a
finite accumulator exists, with width growing as `Theta(2^p)` bits (5, 12, 23, 51, 95, 190, 370
bits for operand precision p = 2..8, exact figures from an exhaustive/lcm-based construction
cross-checked against Python's `math.lcm`), against multiplication's linear `2p`. File 43 also
compiles the exact-division-by-a-representable-constant subfamily (division by any fixed nonzero
type-level rational, not only a power of the radix as an earlier file had assumed under the
pre-overturn dyadic coordinates) at zero new mechanism, reusing files 41/42's rational
multiplication with the constant's numerator and denominator swapped, and finds it total by
construction (the constant's numerator position is `Pos`-bounded, so zero has no spelling).

---

## Part 3: Converged on (and whether the convergence was independent or inherited)

### 3.1 Real, independently-derived convergence

**The datum/value distinction and its consequence for law equality.** File 28 first proposes it
(identity as mathematical coordinates, encoding as a separate physical layer that "may change
which datum carries a value" but "may never change which value is carried"). File 34
independently discovers, by compiling rather than arguing, that the shipped `TotalOrd` trait
(cited by an earlier file as the natural definition of "law equality") actually induces a
*datum*-level order (matching `f64::total_cmp`/IEEE's own `totalOrder`, separating signed zeros
and ordering NaN payloads), which if used as the definition of law equality would let a law's
truth flip on the sign of a zero. File 34 repairs this by defining law equality as the
"canonical quotient" through the identity half's own canonicalisation mechanism
(`Encoding::Canonical`), a definition that later files (37, 38) carry unchanged. This is real
convergence because the repair uses machinery built for an unrelated purpose (the
identity-crossing contract) rather than a fresh invention, and because file 34's finding was a
genuine surprise to itself, discovered by compiling rather than assumed.

**The three-part crossing contract, arrived at by contradiction rather than by proposal.** File
28 states "two round-trip theorems" as the whole contract between mathematical and encoded
coordinates. File 30 finds file 28's own section 2 (three named entrances to non-injective
interpretation: signed zero, NaN payloads, decimal cohorts) directly falsifies its own section 1
(one of the two theorems). File 31 recompiles the repair (a section-retraction triple: identity
on values always, idempotence on data always, injectivity as a derived rather than assumed
boolean) independently rather than trusting file 30's `OUTCOMES.md`. This is the archive's
clearest example of a member's own two sections disagreeing and a *later* member catching it
mechanically -- worth flagging because it recurs (file 38 explicitly generalises it: "this
review has now found that pattern four times... proposals that each compile alone are not
thereby compatible, and nobody has been checking").

**The torsion-free-group theorem (1.5) independently reconfirmed by a wholly different
instrument.** File 17 (Orchard persona, using a graded-monad/coeffect framework entirely
separate from file 14's group-theoretic one) measures, via an exhaustive diameter sweep over
fourteen tree shapes and all inputs, that unsigned saturating addition has diameter 0 at every
fold length while signed saturating grows to the whole representable range by five elements.
File 17 states explicitly: "that reproduces Dolan's torsion-group and one-sided-clamp argument
on a completely different instrument, which is worth one line because it is the kind of
agreement that means something: two constructions with no shared code reaching the same fact."
This is genuine independent corroboration in the panel's own strict sense (arrived at
differently, by different mechanisms), not the "agreement between unratified artifacts" the
workspace's own rules warn is usually shared drift.

**Multiple, structurally different arguments converging on "the fallible carrier must be
arvo's own sealed type with a single accessor."** File 05 reaches it from layout cost (a
fallible return doubles every intermediate's size unless a spare bit pattern exists). File 06
reaches it from Rust's orphan rule (arvo cannot implement `Add` on a foreign `notko::Outcome`, so
one unwrap per operation is forced unless the carrier is local). File 07 reaches it from the
graded-monad presentation directly (the carrier is naturally "the interpretation of the grade").
File 10 states the design principle this converges on
(`what-you-can-observe-is-what-you-guaranteed.md`, later cited as a workspace rule, tracing back
to this exact panel finding: the union's `Poison` carrier shipped with public fields, which the
panel found voids every guarantee its own surrounding machinery establishes).

### 3.2 Convergence that was, on inspection, inherited rather than independent

**The three-relation ladder, before file 37's correction.** File 34's own section 3.3 asserted
"three relations, ordered" (weak, then Kleene, then graded) as a fact, and this reads as settled
in the text that immediately preceded file 37. File 37 shows the ordering claim was never
compiled as a chain and is in fact false: the actual object is a nine-point lattice, not a
chain, with two shipped presets sitting at genuinely incomparable points. The prior "ordering"
was a plausible-sounding restatement of an unverified assumption that several files repeated
without independently deriving it. This is flagged explicitly in file 40's droplist as an
overturned reading, not merely a refinement.

**"The shipped width chain already satisfies value-uniqueness" (file 34), inherited by no one
who checked, corrected by the very next file that had to build against it.** File 34 states this
in passing as a reassurance ("Integer adjustments under file 31's biased formula satisfy it for
free... the formula is self-normalising, a property nobody had named"). File 36, the immediate
successor tasked with building the mechanism file 34's sentence assumed already existed, finds
it false by attempting to build against it and hitting a compile failure one layer below the
first known instance of the same defect class.

**Three consecutive members restating a four-member ratified trait from memory, each getting it
wrong differently.** File 38's own section 2.1 finds files 35, 36, and 38's citations of 35 each
state a different `Numeral` member list, none matching the ratified contract: one resurrects a
member D69 had explicitly made derived rather than primitive (`LogicalWidth`), one silently
drops `Radix` entirely, and file 37's own citation of file 35 points at line numbers that do not
exist in the cited file. File 38 states plainly: "the failure mode is not carelessness; it is
that prose about the design's own state is checked by nothing, and the only defence found so far
is the one exercised here: re-grep the claim before building on it." This is directly the
failure mode this dispatch's own standing constraints warn about (never cite a `file:line` not
opened), demonstrated inside the archive it is now being mined from.

**The general lesson the archive states about itself, worth carrying forward as the single most
transferable methodological finding**: file 45 finds that *even a compiled, verified claim goes
stale silently* the moment a design coordinate it depended on changes, because nothing in the
review's process ever asked "is this claim checked against what the design currently is" rather
than merely "is this claim checked." The proposed fix (a `grounded on: <slug>` provenance field,
naming which ratified decision, settled shape, or physical fact -- toolchain pin, target,
model width, forbidden-feature-list transfer basis -- a claim's derivation actually used) is
itself a piece of theory worth naming: it is the review applying to its own prose the exact
discipline software verification calls proof repair (file 19's own field), and finding, by
running it, that roughly half the archive's claims rest on no ratified decision at all but on
physical facts about the compiler pin and target that the archive had, until file 45, never even
named (the specific pin `1.98.0-nightly (57d06900f 2026-05-27)`, the specific target
`aarch64-apple-darwin`, neither stated anywhere before file 45 despite dozens of prior
instruction-count claims resting on both).

---

## Part 4: Explored and abandoned, with the diagnostic that closed each route

Every entry below is a route this archive tried, compiled, and closed, with the specific reason.
Several of these are directly relevant to the current panel's live options and are marked as such.

- **A three-parameter split of the composition type (`Number<N, P, L>` instead of the fused
  `Number<N, S>`)**, tried to make "a law may not read `Lowering`" a typing fact. Compiled and
  refuted: an eleven-line counterexample types a law naming the `Lowering` member cleanly under
  the split (`OLD_08_probes`), and the split cost roughly 1.8x rendered type length in diagnostics
  with no delivered typing guarantee, since the invariant it was meant to secure was later closed
  a different way entirely (a phantom carrier with no `Lowering` bound at all, independent of any
  parameter-count choice). **Relevant to any live option about the composition's arity.**

- **A crate-boundary alone as the mechanism preventing a law from reading `Lowering`.** Compiled
  across several genuinely separate crates (file 09): the crate boundary does make the *fact's
  own derivation* provably independent of `Lowering` and does stop a foreign, unrelated crate
  from injecting a conflicting law (both via Rust's ordinary orphan rule, which predates this
  design entirely). It does **not** stop the one crate that legitimately must own the physically
  real numeral type from itself conditioning a law on `Lowering`, because that crate's own type
  definition structurally requires `Lowering` in scope. Closed by a different mechanism (a
  purely phantom carrier type with no `Lowering` bound anywhere in its own definition, so no
  where-clause in any crate, honest or not, has anything to name).

- **`f64::mul_add` as a "fidelity liberty" (a permission the backend may or may not take, either
  answer acceptable).** Compiled and found categorically wrong: it lowers to `llvm.fma`, an
  exact IEEE operation with **one defined answer**, not a permission at all, and on a target
  with no hardware FMA unit it compiles to a **pessimising** libm call, the opposite of what a
  liberty should cost (file 20). The genuine permission (`Contract`, "either answer is
  acceptable") is real and remains an unclosed residue, requiring either a forbidden compiler
  intrinsic or a receipt-and-verification mechanism the archive designed but did not build.

- **A consumer-declared "required view" as the mechanism gating a law's transfer through a
  regrouping combinator.** Built, and killed by the compiler mid-dispatch (file 37): the licence
  check refused exactly the case the mechanism existed to handle, which turned out to mean two
  genuinely different things (a hard refusal where values diverge; a mere data-shaped
  publication everywhere else) had been conflated into one mechanism. Replaced by "a regrouping
  publishes what it fails to preserve; tolerance is a transfer, never a waiver," with no
  consumer-declared parameter at all.

- **A pushed, registered build-layer manifest (a `linkme`/`inventory`-style record per
  composition, emitted at each consumer's declaration site) as the channel for a build layer to
  read arvo's own axis choices.** Considered and rejected in favour of the pull-shaped
  symbol-table read (file 20): a pushed manifest records what a consumer *declared* rather than
  what actually got *instantiated*, silently misses every composition reached only through
  generic code, and (independently) cannot even be written for a generic function at all,
  because Rust forbids an item declared inside a generic function body from naming that
  function's own type parameters.

- **Predicting the accumulator-agreement threshold from a recovery map's monotonicity.** File 21
  built this prediction from a plausible-looking counting argument (monotone recovery maps
  should reach grouping-agreement strictly earlier than non-monotone ones) and then measured it
  false: every non-homomorphism resolution reaches the identical threshold (`K = n - 1`,
  "interior safety") regardless of whether it is monotone. Explicitly kept in the probe as a
  refuted first draft rather than discarded.

- **Bounding a regrouping combinator on a numeric "diameter budget" instead of a boolean law.**
  Tried directly (file 17) and refused by measurement: signed saturating addition's regrouping
  diameter grows to the entire representable range by a five-element fold, so there is no
  useful budget to bound against for the arithmetic that motivated the idea.

- **Referential uniqueness (never let a consumer name a derived numeral by literal; every
  signature names it only by projection) as a cheaper alternative to value-uniqueness.**
  Considered and rejected by file 36 with the argument recorded explicitly (so a later reader
  who wants to overturn it knows what has to be defeated): it fails the ordinary case of a
  consumer storing a product's result in a numeral they declared by hand, and it is "an
  invariant living in a signature-writing convention, the class of invariant this review has
  repeatedly found rots without announcing itself."

- **"Past the top is unreachable" once `Specials` (infinity) is representable.** Proposed by
  file 27, found false by file 30: infinity does not remove the over-range position, it changes
  what its neighbour is; the midpoint deciding ties-to-even overflow lives on the round-first
  amendment's unbounded grid, not between the largest finite and infinity, where no midpoint
  exists at all. **Directly relevant to any live option touching Specials/infinity semantics.**

- **A generic order-N error-feedback (noise-shaping) filter, built as one fully general
  mechanism.** File 29 explicitly declines to attempt this and states why: under this
  workspace's forbidden-feature list a fully generic "arbitrary filter order N" shape either
  needs const-generic array arithmetic in a position already forbidden, or needs a per-filter
  stability proof, "a much larger undertaking than this file is proposing." The buildable
  alternative offered instead (a small, closed set of named, individually-verified shaper
  markers, mirroring the design's existing closed vocabulary for `Direction`/`Resolution`) is
  presented as the honest substitute, explicitly not a workaround.

- **The `Sign` axis as a single three-instance member of the numeral identity
  (`Unsigned`/`TwosComplement`/`SignMagnitude`).** Proposed by file 28. File 30 finds it bundles
  a value fact (the representable range, e.g. `-8` vs `-7` for the same clamp direction) with a
  datum fact (how many bit patterns carry zero), split into `SignDomain` on the identity side and
  `SignIndexing` on the encoding side, with the direct consequence that `SC_SAT_SYM` stops being
  its own quantisation mode and becomes an ordinary consequence of a symmetric numeral. **Directly
  relevant to OPTIONS.md Q4 ("what does a datum stand for") and Q6/Q7 (arithmetic-column,
  packing-claim carrier questions).**

- **Growth as a `Policy` axis with instances `Exact`/`Narrowed<W, A>`.** Both the accumulator-in-
  the-key half of this axis (removed from the law's key entirely, ratified) and the whole-axis
  removal from `Policy` (argued, corroborated by all three vendor standards independently placing
  "growth" on the *operation's* signature rather than on a unary numeral property, but never
  compiled, and explicitly left open by op's own checkpoints as a genuine, unresolved tick).
  **Relevant to Q5 (is the arithmetic column one axis or two) and Q12 (associativity
  requirements).**

- **`Widening` as a `Lowering` axis with instances `None`/`InContainer`/`PerOperation`.**
  Ratified removed (file 35, corroborated by op's own checkpoint's demand for a standards test,
  which file 39 ran and found the removal was not merely tolerated by MATLAB/SystemC but
  *required*, since MATLAB's `SpecifyPrecision` mode names a third, consumer-chosen destination
  numeral the old three-instance axis structurally could not spell). **Directly relevant to
  OPTIONS.md Q7/Q9, the packing-carrier and width-surface questions.**

---

## Explicit flags for the two questions op named as in flight

**On "which verb 'validate' is in op's acceptance criterion":** nothing in files 00-45 speaks to
this directly; the archive predates the current panel's phrasing of this criterion entirely. The
closest structural analogue in the archive is its own four-bin evidence discipline (file 38: by
construction / by bounded exhaustion / measured / reasoned-without-artifact), later sharpened by
file 45 into a provenance-grounding scheme. If "validate" is being read against that discipline,
the archive's own hardest-won lesson is that a claim in the *strongest* bin (bounded exhaustion at
a model width) is not thereby "validated" in any sense that survives a change to the coordinates
it was checked in -- the archive's three worst-found defects (`Bias = Int`, the division
prediction, the overflow band's uncompiled members) were each carried as checked, correctly, right
up until a later coordinate change silently invalidated the derivation and nobody had a way to
notice mechanically.

**On whether the long-standing constraints (`no_std`, no `alloc`, const sizes, no `dyn`, no
`TypeId`) are op's intents at all:** these appear throughout the archive as **operative, load-
bearing constraints that member files repeatedly cite as the reason a given mechanism is closed
off**, but never as something any member attributes to op directly, and never quoted in op's own
voice anywhere in files 00-45 (the checkpoints, which do carry op's verbatim words, never state
these constraints; they state design *calls* about the shape being built under them). The clearest
evidence for their load-bearing status independent of who ratified them: file 10 (Leroy persona)
finds and states explicitly that the ban on `specialization` and `TypeId` is not merely a soundness
policy but **verification infrastructure** -- "the forbidden-features list is load-bearing
verification infrastructure... The day someone un-forbids `specialization`, every small-model
transfer argument in this design silently loses its ground, and nothing will fail," because a
function that cannot ask "which width am I at" is exactly what makes a property checked at a
small model transfer to every real width. File 45 confirms this is still the transfer basis every
bounded-exhaustion claim in the whole 45-file archive rests on. So: whatever their provenance, the
constraints are not decorative in this archive. A large fraction of the archive's strongest
results (every "machine-checked by bounded exhaustion" claim, which is most of the load-bearing
mathematics in files 01-45) is unconditionally void the moment those two specific bans lift,
independent of whether the bans themselves were ever stated as op's intent. This corroborates,
from an independent source and an independent panel, the identical finding the current arvo
`.claude/CLAUDE.md` states about the same two bans ("erasure half of op's acceptance criterion is
argued... from the absence of `dyn` and `TypeId`. That argument is only as settled as the
constraint under it"): the archive shows this is not a one-off observation but a load-bearing
premise the design has leaned on repeatedly across two independent panels, under two independent
sets of authors, neither of which has ever traced it to a ratified statement in op's own voice.

---

## Cross-reference against the current panel's live options (OPTIONS.md)

Read: the full text of Q4 through Q7, Q9, Q12, and Q17 (not the whole register), plus every `## Q`
header for orientation. **Every citation in this section that names a bare file number (e.g. "file
`35`", "file `42`") belongs to the current panel's own dive and is relayed from OPTIONS.md's own
text, not opened by me.** I have not read those files; I report only what OPTIONS.md itself states
about them, so a reader can weigh this the same way the panel's own rules weigh any secondhand
citation. Citations of the form "archive file NN" are mine, from files 00-45 as read directly.

**Q4, what does a datum stand for, and its "absorbing top" and "constructor-level clause"
readings.** No direct hit. The closest material is archive file 28's datum/value split (3.1
above) and file 34's finding that a naive `TotalOrd`-based order conflates datum-level and
value-level equality (the same failure mode Q4's own "constructor-level clause" reading is
trying to name precisely: a clause written as a per-datum property when the design work is
actually per-constructor). This bears on the question but does not settle it; the archive never
built or measured an absorbing-top denotation directly.

**Q5, is the arithmetic column one axis or two, and Q6, does `Warm` wrap or clamp.** These bear
most directly on archive file 14's torsion-free-group theorem and Fuchs' ordered-algebra
dichotomy (1.5 above): wrap and clamp are structurally exclusive properties of the *same*
operation (translation-invariance versus order-preservation on a bounded domain), never a choice
between two values of one axis in the sense that a consumer could smoothly interpolate between
them. That mathematical fact does not by itself decide Q5's own question (whether overflow
policy and intermediate precision are one axis or two), because Q5 and Q6 are about a different
pair of properties (which policy an operation takes, versus what precision the intermediate
carries) than the wrap/clamp pair file 14 proves exclusive. But it is directly relevant
background: whatever the final axis decomposition, wrap and clamp are proven, not merely
observed, to sit on opposite sides of a hard dichotomy, at every width, for every signed or
unsigned domain, and Q12's own "wrapping's four properties are one theorem" drafting note (see
below) is the same style of finding archive file 14 already produced for the associativity half
of this same dichotomy.

**Q7, which carrier is the packing claim about.** No direct hit in files 00-45. This question, as
stated in OPTIONS.md, concerns a contention/core-count regime the archive never measured (its
own multi-core / contention material, if any, would be in files outside my range).

**Q9, the width-surface crossing (arrangements C0 through D).** This entire question, as
currently framed in OPTIONS.md, concerns the current panel's own dive (files 10-16 of the
*current* panel, not the archive), and none of the archive's own files 00-45 discuss a "width
surface crossing" or a bridge table between consumer literals and type-level nats in these
terms. The closest archive material is file 25's `OLD_25_probes/03_typelevel_binary_addwidth.rs`
(2.2 above), which builds and exercises a type-level binary width adder without hitting the
bridge-table ceiling Q9 describes, because file 25's construction never needed to cross back
from a computed width to a consumer-facing literal (Q9's arrangement B/D distinction). This is
worth flagging precisely because it is a near-miss: the archive built and priced a mechanism
adjacent to what Q9 is asking about, under a different motivating question (multiplicative
width growth, not literal-to-nat crossing), and never noticed the overlap. A reader chasing Q9
may find file 25's adder relevant as an existence proof that type-level width arithmetic without
`generic_const_exprs` is buildable at all (which Q9's own material appears to take as given), but
should not read file 25 as having addressed Q9's actual question, which it does not.

**Q12, is the reduction order specified, or is associativity required, including the "wrapping's
four properties are one theorem" drafting note and the commutativity-is-free finding.** This is
the strongest bearing of any live option on my slice. Archive file 14's torsion-free-group
theorem is precisely the "one theorem" Q12's drafting note gestures at without proving: wrapping
addition on a fixed width realises the cyclic group `Z/2^N Z`, and every one of the four
separately-measured properties Q12 names (associativity, commutativity, identity, inverse) is a
restatement of "this structure is a group," not four independent facts. File 14 proves this
directly from group axioms rather than by exhaustive measurement, which is a stronger and cheaper
result than the measurement-only path Q12 describes taking. Archive file 17's independent
diameter-sweep corroboration of the same dichotomy on a completely different instrument
(coeffect/graded framework rather than group theory, 3.1 above) is also directly relevant: it is
exactly the kind of "two constructions with no shared code reaching the same fact" convergence
Q12's own exhaustive-measurement results (n=8 over 16.7M vectors) are independently reproducing
for the associativity half specifically. On the signed-saturating divergence mechanism itself
(the "one clamp associates, two do not" versus "pullback" contest recorded as CONTESTED in
OPTIONS.md), the archive has nothing to add; that mechanism question was never raised in files
00-45 in this form.

**Q17, the fraction boundary and integer-only results, and the multiplicative fold's need for
linear growth with no closed form.** Archive file 24's DSP-literature citations (1.8 above) bear
on this directly and predate the current panel's own measurement of it: file 24 already states,
from established signal-processing field knowledge rather than from a fresh measurement, that
multiplicative folds face exponential/linear dynamic-range growth against additive folds'
logarithmic growth, and names the field's standard countermeasures (per-stage renormalisation,
block floating point, log-domain representation) precisely because eager fixed-width
multiplication cannot supply a closed-form bound. This is the same shape of result Q17 reports
(current panel files `58` and `60`) reaching by direct measurement and correction after an
initial bug. The archive's contribution here is not a repair of the current panel's own
measurement (which I have not checked and cannot check without opening those files) but an
independent, decades-old field precedent for *why* the shape should be expected: the
multiplicative half was never going to have a closed-form accumulator bound of the same kind as
the additive half, because that is a known property of fixed-point multiply-accumulate more
generally, not an artifact particular to this design.

## What I did not cover

I did not open or verify: the design-round spec text and talk transcript the archive cites by
line number throughout (`202607301200_topic.the-formalization-spec.md`,
`202607301100_topic.the-formalization-talk.md`, the D-numbered decision set in
`202607301000_topic.inherited-state-from-the-formalization-round.md`); any probe source beyond
the files named in Part 2 (I read the outcome tables and the member-file prose describing every
other probe, not the source); `mock/design_rounds/202607300800/` and its sibling topic files that
individual members reach into by exception (D38/D39's own topic file, read in full by file 39 and
relayed here at one remove); the prior-art research pass file 14 and 15 cite
(`202607281616_prior_art/`); `arvo-num-systems` and `notko-hlist`'s own design notes beyond what
file 39 relays (file 39 read both in full; I relay its findings, not the source documents). I did
not check any of the archive's dozens of `file:line` citations into its own predecessor files
against those files directly; where I quote a specific claim I quote the member file that states
it, not the underlying probe or spec text it in turn cites. I did not read the current panel's own
member files (its own `01` through `73`+, distinct from the archive's `01` through `45` I was
assigned), `DROPLIST.md`'s body, or `PERSONA_CALLS.md`; the cross-reference section above relays
what OPTIONS.md itself states about those files' content, one remove further than everything else
in this document.
