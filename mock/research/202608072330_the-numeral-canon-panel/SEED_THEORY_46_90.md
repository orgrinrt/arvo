# Theory mining, files 46-90: the formalization-spec-panel archive

Coverage statement first, because it governs how to read everything below. My assigned slice is the
member files numbered 46 through 90 of `202607301300_formalization-spec-panel/`, plus their probe
directories. I read in full: 49, 50, 51, 52, 53, 54, 55 (partial, sections 2 and 3 in full, rest
skimmed), 56 (partial), 57, 58 (consolidation four through five, in full, all sections), 59, 60
(partial, sections 1.1-1.5 in full), 61 (partial), 62 (in full, the highest-value file in my slice
for verification work), 63 (consolidation six, sections 1.18-1.20, 5, 6 in full; rest cross-checked
against 58 for delta), 68 (consolidation seven, "the two rules" through "the three design rules" in
full, section 1.19 in full, sections 4-5 in full), 69 (headers and opening only), 70 (section 5 and 6
in full, rest skimmed), 71 (sections 1-2 in full), 72 (sections 1-3 in full), 73-77 (headers and
verdict paragraphs only), 78 (consolidation eight, opening, section 2 in full, rest cross-checked
against 68 for delta), 79-83 (headers only, 79/80 excepted), 80 (sections 4 and 5 in full, the
second-most-valuable file in my slice), 84 (sections 3 through 6 in full), 85 (sections 2.2-2.4 in
full), 86 (sections 1.4-1.5 in full), 87-88 (headers only), 89 (sections 6-7 in full), 90 (sections
4-7 in full, the closing file of my slice). I did not read 46, 47, 48 as standalone files; their
content reaches me secondhand through consolidation 49's own detailed synthesis of them, which I
read in full, and I say so rather than cite them directly. I did not open any lettered checkpoint
file (48b, 53b, 57b, 62b, 67b, 68b, 70b, 74b, 77b, 79b, 82b, 86b, 90b) as a standalone file; op's and
the persona's calls reach me through the consolidations' own "the lead designer's calls" sections,
which I read in full for 49, 58, and 78, and which name every checkpoint's content explicitly with
citations.

I opened probe directories for the files this document leans on most (46, 50, 51, 57, 59, 62, 70,
80, 84), read `OUTCOMES.md` in each, and in three cases (46-adjacent `OLD_08_probes/`, 57's bench, 59's
`.gitignore`) went further and independently reproduced or spot-checked the claim myself against the
live repository state, described in the section "What I checked myself" below. I did not re-run any
other probe. I read `INTENTS.md`, `RULES.md`, and `DROPLIST.md`'s section-6 entry list in full, and
`OPTIONS.md`'s question headers (Q1 through Q32) plus the full text of Q6, Q7, and Q9, to locate
where this slice's findings bear on live questions.

**Same disambiguation the 00-45 slice states, restated because it governs every citation below.**
This directory is home to two independent numbering schemes: the archive I was dispatched to mine
(`202607301300_formalization-spec-panel/`, files 46 through 90 numbered plainly), and the current
panel's own files (this directory's own `46_dolan_the_carrier_collision_attacked.md` and onward,
which collide numerically with the archive's file 46 and every subsequent number). Every bare number
below (`50:216-289`, `62:198`, and so on) refers to the archive unless marked "current panel". I have
not read the current panel's files 46 and onward.

## What my slice of the archive is, and how it differs in character from what came before it

Files 00 through roughly 45 (the sibling slice) rebuild the numeral's identity contract from first
principles and settle the value-unique rational encoding. My slice picks up immediately after that
settling and does something different in character: it builds the **float model** on top of the
settled machinery (file 50), closes the last open ratification tick with a structural coherence
theorem (file 51), lands fifteen real test artifacts and audits which are contracts versus
measurements (file 52), re-prices the whole design against real numbers rather than percentages
(file 53), unifies decimal and binary under one radix-general construction (file 54), fixes two real
defects in the shipped algorithm crates (file 55, before the tree that held them was nuked), tests
whether the design's refusals are legible to a human (file 56), runs the runtime bench the design had
deferred for twenty-six files and finds the bench harness itself could not run anything (file 57),
then spends roughly thirty further files hardening, correcting, and pricing that machinery: the
lowering door and radix-ten quantiser (59), the total-order fork (60), the notation vehicle (61,
closed at 63), a rigorous audit of the archive's own citation and reproducibility discipline (62, the
best file in my slice for exactly what this dispatch is for), the four presets re-derived under real
IEEE 754 semantics (70), the overflow far point unified under one order-theoretic rule (71), a
systematic sweep for genuinely unexamined design ground (72), byte images and digests (73, 88), the
crate taxonomy (74), what bitpacked storage actually costs (75, 81, 83), real consumer pricing (76,
77), a formal-methods grounding for the design's own founding identity via Flocq (85), a completed
proof (86), the mathematics of number-system membership done properly with a real correction to a
predecessor (80), the general sort of what a numeral operation failure is (84), and the two items op
held open longest, division and a naming question, resolved with the far-point rule and real hardware
facts (89). File 90 closes with an honest reckoning: the process worked, caught a real fabricated
search claim the hard way, and found a live infrastructure defect that was actively destroying the
review's own measurements.

**The character shift matters for how to read this slice's theory content.** Files 00-45 build
foundations; files 46-90 are where the design gets tested against reality repeatedly: real silicon
(file 50's 41 million operations against binary32), real primary sources (files 62 and 80's
page-cited reads of the OCP OFP8 specification and IEEE 754), real shipped code (files 55, 60, 62's
audits of the then-shipped, now-nuked tree), and a real, live workspace rule this archive's own
findings directly bear on (`unstable-features.md`, section "The rule that governs my own session"
below). The literature engagement in this slice is consequently sharper and more checkable than a
survey would produce, because most of it exists to settle a specific compiled disagreement rather
than to decorate an argument.

---

## Part one: theory referenced, checkable against the literature

Organised by source, each entry stating what the literature says (checkable), what the panel claimed
it says (not checkable, possibly wrong), and what was built on the claim (void if the claim was
wrong), per the dispatch brief's own instruction to keep the three separate.

### IEEE 754 (both the 2008 and 2019 revisions), the single most load-bearing external source in my slice

**Clause 7, the five exceptions.** File 50 found, independently, that the design's own grade (a free
commutative monoid over refusal causes and quantisation events, joined by union) is IEEE 754's sticky
flag word with the value thrown away: over the five clause-7 exceptions with no multiplicity, a
commutative monoid on a five-element generator set with no multiplicity *is* a five-bit word joined
by bitwise or, bit for bit (`50:290-327`). What the panel claims: the mapping is exact (inexact and
underflow are quantisation events; invalid and divideByZero are causes with no quantiser origin;
overflow is raised by the classification step). What was checked: this is a derivation from the
standard's own vocabulary, not a compile; the design's own grade mechanism was built and tested for
its own reasons (files 47, 48) before anyone noticed it coincided with the standard's flag word. What
is genuinely new evidence, and checkable independently of the standard: on the pinned toolchain,
there is no `fetestexcept`, `feclearexcept`, `fegetround`, or `fesetround` anywhere in `core` or
`std` (a grep of the `rust-src` component reported zero files), no FPCR access in
`core::arch::aarch64`, and `_mm_setcsr` is deprecated since Rust 1.75.0 with the note "use inline
assembly instead" (`50:319-324`). This is a real, checkable fact about the Rust standard library on
this toolchain, independent of anyone's reading of the C99/POSIX floating-point environment API it
mirrors.

**Clause 4.3.1 and clause 7.4, the overflow tie.** File 80 read the 2008 text verbatim, from a full
archived copy, and quotes clause 4.3.1's first paragraph exactly: "an infinitely precise result with
magnitude at least `b^emax(b - 1/2 b^(1-p))` shall round to infinity with no change in sign," and
verified the arithmetic itself (the threshold is the maximum finite value plus half a top-binade
ulp). Clause 7.4 (the overflow exception) is quoted verbatim as agreeing from the exception side.
What this corroborates: file 71's independent derivation of the same fact from a parity argument on
the extended grid (the maximum finite significand of every IEEE format is all-ones, hence odd, hence
the extended-grid tie rounds up and away, off the finite set). File 80 is explicit and correct that
the corroboration is of the *conclusion* (ties round to infinity) and not of the *mechanism* (parity
on the extended grid is the design's own derivation, not the standard's stated reasoning; the
standard states the threshold as fiat). This is the honest three-way split my brief asks for: what
the literature says (a threshold, stated as fiat), what the design claims (the same threshold, but
*derived* from parity), and the derivation is not itself literature, it is the panel's own
mathematics, independently checkable by the reader against the standard's stated threshold.

One residual is honestly flagged rather than papered over: file 80 verified the 2008 text of clause
4.3.1 verbatim but did not independently verify the 2019 revision's exact wording of that specific
clause (only clause 5.2's 2019 text, via file 62, is independently confirmed). The 2019 delta at
4.3.1 is, per the standard's own revision summaries, an extension covering a decimal both-nearest-
neighbours-odd corner unreachable in binary formats, so the tie derivation is unaffected; but the
sentence itself was read at one remove for the newer revision, and file 80 says so.

**Clause 5.2, decimal preferred exponents.** File 54 first characterised the standard's clause 5.2
("a preferred exponent for decimal results, a function of the operation and its operands' exponents
rather than of the result's value") from secondary sources, and used it to argue the design's
value-valued operations cannot conform. File 62 read the primary text (page 30, quoted verbatim: "For
all computational operations except where stated otherwise, if the result is inexact the cohort
member of least possible exponent is used... If the result is exact, the cohort member is selected
based on the preferred exponent for a result of that operation") and found the earlier
characterisation had silently generalised a two-branch rule to one branch. The corrected, narrower,
primary-sourced claim: the inexact branch selects by least-possible-exponent, which is a function of
the result's value alone and *is* expressible by the design's `Canonical` mechanism; the deviation is
confined to exact results whose preferred exponent differs from the canonical choice, plus quantize
and roundToIntegralExact specifically (both of which the standard's own text separately carves out:
"Except for the quantize operation, the value of a floating-point result... is never dependent on the
representation or encoding of an operand"). File 62 additionally found that multiplication on
`Implicit` decimal numerals is conformant to clause 5.2 *by construction*, because the design's own
`mulnum` computes the exponent sum `Q(x) + Q(y)` at the type level, which is exactly the standard's
preferred-exponent formula for multiplication, discharged statically rather than propagated at
runtime. This is a genuine sharpening: the claim that shipped for two consolidations overstated the
non-conformance, and the corrected version is both narrower and, on the multiplication case, turns a
"not conformant" into "conformant, and more strongly checked than the standard requires."

**IEEE 754-2019 §4.3, the default rounding attribute.** File 70 cites the standard directly (round-
to-nearest, ties-to-even, the one rounding mode every conforming implementation uses without a
control-register change) to independently confirm the design's own derivation of `Warm`'s in-range
rounding direction for float, arrived at from two directions that do not depend on each other:
"behaves as f32 and f64 do in Rust today" and "nearest, ties to even" name the same fact by two
independent routes.

**IEEE 754-2008's `minNum`/`maxNum` defect.** File 84 uses the standard's own revision history as a
design lesson: `minNum`/`maxNum` in the 2008 text silently discard a NaN or bottom element under
selection; the 2019 revision replaced them with propagating `minimum`/`maximum` for exactly that
reason. The design lesson drawn: a value-returning failure home for a kind-2 failure (division by
zero, say) owes a propagating selection contract, not an ordinary total-order derivation, because a
plain running-maximum silently discards the bottom exactly the way the deprecated IEEE functions did
(checked in const position at `OLD_84_probes/probe_4`).

### Flocq, the formal floating-point library, and the design's own founding identity

File 85 traced the design's oldest, most-repeated, never-attributed claim ("fixed point and float are
one formalisation, differing only in the exponent form", stated without argument since file 40,
restated at every consolidation through `78:186`) to its real source in the literature: **Flocq's
`generic_format` construction**, where a fixed-point number and a floating-point number are the same
mathematical object, `m * beta^e`, differing only in whether the exponent function is constant or a
step function of the mantissa (`85:296-311`). Flocq is a real, checkable library (Boldo and
Melquiond, INRIA, a Coq library for formalising floating-point arithmetic; the panel does not name the
paper, only the construction, and I have not independently opened Flocq's own documentation this
session to re-verify the construction's exact statement, so this is a citation to check rather than
one I have personally confirmed against the primary source). What the panel built on the claim: file
85 uses the Flocq grounding to argue that a fixed-point numeral with a constant exponent (`At<N, Q>`)
is not a new kind of object bolted onto the design's vocabulary, it is the constant instance of what
the unified formalisation already says fixed-point denotes, which is a stronger argument than the bare
trait signature gives. File 86 then built a compiled model confirming the typed consequence (`At<N,
Q>`'s exponent typechecks as an ordinary member of the sealed signed-exponent vocabulary, zero
gates). This is a case where forty-five files of a design converged, independently, on a formalisation
that already exists and is established in the formal-methods literature, and only noticed the
coincidence forty-five files later. **This is worth citing by name if the new panel restates anything
resembling this founding identity**; doing so from the start would have saved the forty-five-file gap
and would let the new panel inherit whatever else Flocq's `generic_format` framework has already
proven about the construction (rounding correctness, error bounds, and so on, none of which this
archive touched).

### The OCP 8-bit Floating Point Specification, read twice, independently, from the primary document

**Two independent primary reads exist**, satisfying real citation-verification rigour (files 62 and
80 each downloaded the specification fresh from the Open Compute Project's own FP8 repository and
read it by printed page). Both confirm, verbatim: §5.1 (printed page 12), "The E4M3 format does not
represent infinities and uses only two bit patterns for NaN... in order to increase emax to 8 and
thus to increase the dynamic range by one binade"; Table 1 (page 13), E4M3 bias 7, emax 8, emin -6;
E5M2 bias 15, emax 15; Table 2 (page 13), E4M3 max normal ±448, min normal ±2^-6, max subnormal
±0.875·2^-6. Both independently find, and file 80 re-confirms verbatim, that the primary document
**contradicts itself**: its own §4.2 abbreviations section (page 11) transposes the two formats'
biases ("E4M3, an exponent bias of 15" and "E5M2, an exponent bias of 7"), refuted by the same
document's own Table 1 and by the value formula on the same page. A reader sent to "confirm against
the primary source" who lands on the wrong section extracts exactly the wrong figures with the full
authority of the primary source behind them. This is a genuine, citable errata finding, and it means
any future citation of this document should name Table 1 specifically, not the document as a whole.

File 62 additionally found and corrected a mis-attribution: `E4M3FNUZ` (the negative-zero-repurposed-
as-NaN variant that file 54's injectivity witness depends on) does **not** appear anywhere in the OCP
document; the earlier citation attributing it there was wrong. Its real home is the earlier
Graphcore/AMD/Qualcomm 8-bit formats proposal and the ONNX type registry (checkable, though I have not
independently opened either this session). File 80 later found a further primary-sourced fact from
the same document, Table 3 (§5.2, page 14): the OCP conversion behaviour explicitly names two modes,
saturating (delivers the max finite magnitude on overflow) and non-saturating (delivers NaN), which
primary-sources file 71's secondary-sourced claim about NVIDIA's and JAX's differing default
conversion behaviour.

### Number systems: real, correct, sophisticated mathematics from file 80, correcting a predecessor's error

File 80's section 4 is the most mathematically careful piece of writing in my slice, and it is
explicit and honest about the boundary between compiled and reasoned: "nothing in this toolchain
could compile them and I do not dress them as probes." The claims, checkable against standard
algebra and set theory independent of any authority in this workspace:

- No ordered field contains ℂ, because squares are non-negative in every ordered field.
- No `ℚ_p` (the p-adic numbers) is orderable, because -1 is a sum of squares in `ℚ_p`.
- **Conway's surreal numbers (`No`) are the universal ordered field**: every ordered field whose
  universe is a set embeds into `No`, and `No` itself is a proper class (this is Conway's actual,
  well-known theorem, from "On Numbers and Games"). A hyperreal field (`*ℝ`) is an ultrapower of `ℝ`,
  hence set-sized, hence embeds into `No`. So `ℝ ⊂ *ℝ ⊂ No` **is** a chain under structural
  inhabitation, contradicting a predecessor's claim (file 64, outside my slice) that surreals and
  hyperreals were "both ordered-field extensions of ℝ, and neither contains the other." File 80
  corrects the actual mathematics with the actual theorem, a genuine instance of exactly the check my
  dispatch exists to perform, done by the archive itself.
- `ℚ` is the prime field of characteristic zero, with a canonical, unique embedding into every
  characteristic-zero field, which is why the arvo-specific narrowing (every arvo numeral's value set
  is a finite set of rationals, `m · r^q` with integer `m, q`, for any radix) yields a clean, provable
  uniqueness theorem for "the finest inhabited system" confined to the sub-ℚ chain `ℕ ⊂ ℤ ⊂ ℚ`,
  narrower and more defensible than the predecessor's broader claim that the *whole* ten-member
  vocabulary was a chain (it is not, above ℚ).

This is real, correct, standard mathematics, checkable by anyone who knows the field, and it is the
strongest single instance in my slice of the brief's promise that theory referenced by this archive
can be verified independently of the archive's own authority. It settles nothing about what the new
panel's numeral design should look like (that question is entirely open per `INTENTS.md`), but it
settles, correctly, what the mathematics of number-system membership actually says, which is durable
regardless of what shape the new numeral tower eventually takes.

### Standard float-printing algorithms, named but never engaged

File 72's systematic coverage sweep found, correctly, that the whole 70-plus-file archive has **zero**
hits for round-trip printing, shortest representation, or any of the standard family of algorithms for
correctly-rounded shortest float-to-decimal conversion: Steele-White, Grisu, Ryu, Dragon4, Schubfach
(`72:table row 1`). These are real, established, named algorithms in the literature (Steele and White,
"How to Print Floating-Point Numbers Accurately", 1990; the Grisu, Ryu, Dragon4, and Schubfach lines of
descendant work). This is a genuine, confirmed gap rather than a finding about what any of them say:
nobody in this archive has engaged with this literature at all, and if the new panel ever designs
number-to-text formatting, this is the literature to open, cold, rather than re-derive.

### Rust language and standard-library facts, checkable against the compiler and its documentation

A cluster of real, checkable, non-mathematical facts, each independently useful regardless of the
numeral design's eventual shape:

- `f32::algebraic_add`/`sub`/`mul`/`div`/`rem` (and the `f64`/`f16`/`f128` siblings), gated
  `#![feature(float_algebraic)]`, tracking issue rust-lang/rust#136469, with a stabilisation PR open
  at the time of writing (#157029). File 51 quotes the tracking issue's own stated motivation
  directly: "A stable Rust implementation of a simple dot product is 8x slower than C++ on modern
  x86-64 CPUs, with the root cause being an inability to let the compiler reorder floating point
  operations for better vectorization." This is checkable against the live GitHub issue, and its
  status may have moved since the archive was written (roughly early August); worth a fresh check
  before the new panel relies on it.
- `core::intrinsics::fadd_fast` and siblings are `unsafe fn`, require finite operands or the result is
  UB, and the doc comment states plainly "this intrinsic does not have a stable counterpart" (quoted
  from the rustc source on the pinned nightly). This is the structural distinction file 51 uses to
  argue `algebraic_add` (safe, reassociation-only) is a genuinely different, adoptable mechanism from
  the fast-math intrinsics (unsafe, finite-assuming).
- LLVM's fast-math flags (`reassoc nsz arcp contract`) are real, standard LLVM IR vocabulary, and file
  51 measured exactly which of the four `algebraic_add` grants, compiled, on real assembly, and found
  the bundle over-grants relative to what the design's interior-safety proof actually establishes
  (interior safety proves only `reassoc`; `nsz`, `arcp`, and `contract` each need a separate,
  independent justification).
- `core::num::NonZeroI64` and Rust's niche-optimisation machinery are real, stable, checkable. File 84
  measured the actual layout and codegen consequences (`size_of::<Option<NonZeroI64>>() == 8` against
  16 for `Option<i64>`) and used them to design a "biased niche" encoding that gives a refusing carrier
  the same layout cost as an infallible one, at nine extra vectorised instructions across 64 elements.
- `rustc_layout_scalar_valid_range_*` is confirmed rejected outright even under
  `#![feature(rustc_attrs)]` on the pinned toolchain ("attributes starting with `rustc` are reserved
  for use by the `rustc` compiler"). The actual live niche mechanism is the `pattern_types` feature,
  explicitly unstable and internal (`#![unstable(feature = "temporary_niche_types", issue = "none",
  reason = "for core, alloc, and std internals until pattern types are further along")]`). File 84
  correctly applied `unstable-features.md`'s own carve-out procedure and declined to adopt it, because
  a stable wrapper (the `NonZero` family) already suffices, which the rule's own first step demands be
  checked before anything else.
- A pure `macro_rules!` decimal-to-binary muncher cannot decompose a decimal literal's digits: no
  fragment specifier, restringify trick, or const-generic escape reaches a literal's own text (file
  61, confirmed twice: once by the declarative-macro route, once by the value-to-type escape hitting
  the identical const-position wall the exponent case already hit). This is a real, checkable fact
  about Rust's declarative macro system worth confirming against the Rust reference grammar rather
  than trusting the archive's word alone, since I have not independently verified it this session.
- A genuinely elementary, real number-theoretic fact, independently derived twice (files 59 and 68):
  a rounding tie is reachable at every even radix and at no odd radix, because the tie condition
  `2 * lost == R^s` has no solution when `R` is odd (`R^s` stays odd; `2 * lost` is always even).
  Checked non-vacuously at radix 3 and radix 13 (188,448 roundings, zero ties).

---

## Part two: proved or measured, with the probe or bench path named for re-testing

The dispatch brief asks that this category name the instrument, because a re-test is cheap when it is
on disk. The single most important instance in my slice is that a claimed-missing instrument turned
out not to be missing at all, covered separately below because it is itself the finding.

**File 50's central claim, exhaustively checked against real silicon**: `OLD_50_probes/probe_1_model_vs_
silicon.rs`, a model implementing round-first quantisation with no knowledge of IEEE beyond the format
parameters, agrees with binary32 hardware on 41,380,159 operations, zero mismatches, including 1,255
overflows-to-infinity and 884 subnormal results (so gradual underflow is genuinely exercised). The
same probe directory found and fixed a real bug in its own first draft: a naive left-shift alignment
in the rounding kernel silently masked the shift amount for the product of two binary32 subnormals
(`u128 << 149` masking to 21), delivering a plausible wrong answer 130 binades below the smallest
representable value. Nothing about the source looked wrong; the hardware disagreeing is what surfaced
it. This is the archive's strongest single piece of evidence and it is fully reproducible, per the
probe directory's own `OUTCOMES.md`, with the exact build commands.

**The overflow band's closed form**, `OLD_50_probes/probe_2_band.rs` and `probe_2b_band_closed_form.rs`:
a naive candidate closed form (`q_result <= 2 * lattice`) was built and then refuted by exhaustive
enumeration (753/1000 for addition, 639/1000 for multiplication, both directions of error). The
corrected two-clause form (a lattice clause, decidable by one Euclidean division; a reachability
clause, which is not decidable in general) was then measured over 5,184 quantum triples with zero
under-predictions in both directions, the useful property for a build layer to act on.

**The exponent-as-type wall, compiled shut in both directions**: `OLD_50_probes/probe_3_exponent_as_
type.rs` (compiles, zero unstable features) and `probe_3b_exponent_as_const_refused.rs` (the negative
control: `generic_const_exprs` forbidden, `min_generic_const_args` refuses with a different error,
the `const {}` block escape needs `generic_const_args` which needs `-Znext-solver=globally`,
mutually exclusive with the rest of the arrangement). File 54's `probe_4_implicit_exponent_as_type.rs`
and `probe_4b` repeat the identical pattern for `Implicit`'s single exponent, overturning file 36's
own earlier claim that this position would not hit the wall. This is a clean, positive, worked
instance of the workspace's own `a-refused-bound-wants-a-trait-not-a-feature.md` pattern: decompose
the computed quantity into a type, and the wall that looked structural dissolves.

**The runtime bench file 50 deferred, finally run in file 57**: a real bench under
`mock/benches/variants/quantiser-fadd-*`, correctness-checked bit-for-bit against native `+` over
98,304 operations before any timing was trusted. Measured result: the software round-first quantiser
costs 13x to 17x a native hardware `fadd` on Apple Silicon aarch64 (roughly 16-20 ns/op against
1.2-1.4 ns/op), uniformly across a 0-to-100-percent subnormal sweep, with **no subnormal performance
cliff on this target**, confirming file 50's own open guess that the historical x86 subnormal
microcode trap does not apply here. File 59 re-ran the same bench in a later session and found the
tight "13x to 17x, at every point" framing was itself tighter than a second run supports (a second run
gave 10x to 17x); the corrected, honest statement is "roughly an order of magnitude, ten to seventeen
across two runs," because the hardware column sits near the harness's own measurement resolution
floor. This is a live number the new panel can cite for the real cost a `Hot`-tier hardware-float
lowering buys against the safety margin a software-quantiser default costs, if anything resembling
these strategy markers survives into the new design.

**The decimal quantiser, priced against binary, file 59**: a real bench,
`mock/benches/decimal-quantiser-radix-sweep`, measured decimal32 costing 1.7x to 1.9x a binary32
software quantiser (both software), 24x to 35x a native hardware `fadd`. Both this bench's and file
57's committed artifacts are still present on the current `feat/arvo-shape-topic` branch, described
under "What I checked myself" below.

**The bitpacked-storage cost, measured honestly across two attempts**: file 75 found true bitpacked
storage (zero inter-value padding) is a genuinely distinct mechanism from dense storage at a narrow
width, not a redundant restatement (4.6x to 5.5x slower sequentially, 2.2x slower under random
access, at logical width 13). File 90 records that this figure was later re-measured and fell to
1.29x-1.50x once the fourth design rule (the spine rule) was actually applied to eliminate a runtime
decoder computation the type already knew statically. The lesson worth carrying forward is not either
number; it is that the first, higher number was a mispriced founding feature and the correction came
from applying a design rule the panel had already adopted but had not yet pushed all the way through.

**A completed proof, worth noting because file 85 honestly left it incomplete and file 86 finished
it**: the fold-width formula `foldexact(P, A) = P + L - 1 + bit` (`L = bitlen(A)`) reduces, across all
three of its stated branches, to a single test, `bit = [R(2^P - 1) >= 2^(L-1)]` where `R = A -
2^(L-1)`, with the power-of-two case simply being that test's `R = 0` instance. File 85 proved two of
the three branches with explicit inequality chains and flagged the third as spot-checked only; file
86 completed it. This is a small, clean, self-contained piece of elementary bit-length arithmetic,
fully proved, and it is a good worked example of the honest incremental-proof discipline this slice
practises at its best: state what is proved, state what is merely checked, and let a later file close
the gap rather than paper over it.

---

## Part three: converged on, and whether the convergence is real

**The spine rule** (a quantity computed and then required to appear in a type is a type; a quantity
only ever read is a const) is the strongest genuine convergence in my slice. It was named once, by a
persona checkpoint, but it then fired independently and was independently *confirmed by compilation*
at every one of: the ranged-numeral exponent bounds (file 50), the implicit numeral's single exponent
(file 54, against its own predecessor's contrary claim), a fold's published grade (files 47/48,
outside my direct read but reported by 49), the radix carrier (file 54), and the byte-count member
(file 73). Each of these is an independent compile against a different position in the design, not a
restatement of the same finding. This is convergence in the sense the RULES.md provenance ladder
actually wants: repeated, independent derivation, not inherited reading. The one caveat worth stating:
the archive's own running tally of "how many times has this fired" disagrees between files (file 55
calls its own finding "a fourth time," file 56 calls its own "the fifth" and "a sixth" in the same
sentence), and no consolidation ever reconciled the count. The disagreement is cosmetic; the
underlying compiled instances are each real and independently checkable.

**Real, independent, non-copied corroboration between file 16 (outside my slice) and file 50**: file
16 argued from first principles that fixed-point addition needs no LLVM reassociation licence because
wrapping addition over a fixed width is already associative, with no compiled evidence at the time.
File 50, working independently and much later, measured the actual assembly and found exactly this:
integer reductions get 8 vector adds (LLVM reassociates freely), float reductions get scalar adds only
(LLVM refuses without an explicit licence). File 51 states plainly that this is "the good kind of
corroboration, not the copied kind," because the two files reached the same conclusion from different
directions at different times without either reading the other first.

**A convergence that was real between two Rust attributes and one design finding**: the "carrier-at-
birth" seal mechanism (a closed vocabulary a guarantee quantifies over should be sealed and attacked
at declaration, not after several passes) turns out to also be the archive's best diagnostic, entirely
by accident. File 56 found that rustc's own sealed-trait detection explains a refusal in plain
English and lists the exhaustive legal inhabitant set, unprompted, with zero
`#[diagnostic::on_unimplemented]` machinery anywhere. Nobody arranged this; it was true the whole time
and nobody had checked what a person actually reads when the seal fires until file 56 asked.

**A convergence that turned out to be false, and is worth stating precisely because of how it
failed**: the archive's own citation-verification discipline (two independent reads before anything
compiled builds on a claim) converged twice, independently, on the conclusion that file 57's claim
"file 8's five-shape table cannot be reproduced by anyone from what is committed" was **false**. Files
62 and 80, working independently, each rebuilt the material and found the reproduction recipe had
been sitting in the same probe directory the entire time (`OLD_08_probes/README.md:8-11`, committed
alongside file 8 itself). This is genuine independent convergence on a correction, and it is also
exactly the failure mode this dispatch's own standing constraints warn about: a universal negative
("nobody can rebuild this") is the single most expensive kind of claim to be wrong about and the
cheapest kind to check.

---

## Part four: explored and abandoned, with the reason, worth as much as a positive result

**A single combinator with a defaulted grouping-strategy parameter**, which would have kept both a
ceremony-free path and a sequential escape hatch under one name: refused by the language itself
(defaults for generic parameters are not permitted in that position), not by the design. Closed
route, real diagnostic, do not re-propose.

**A bounded numeral-notation alias table** (0 through 1024, generated): refused on principle (a
hardcoded threshold of exactly the kind the workspace's own toolbox rule forbids) and refused on
evidence (the design's own named division constants, 44100/48000/4096, sit past any table a metadata
budget tolerates). Closed twice, independently.

**Absorbing a decimal quantum into a single rational**, as an alternative to keeping radix and
exponent as separate axes: compiled and measured to hit two independent hard walls (a `u64` readout
ceiling at `10^20`, then a `Pos` recursion-depth wall at 130 constructors) at decimal64's own real
exponent range, while the radix-plus-exponent spelling compiles the same grid in 64 milliseconds flat.
This is not merely expensive, file 54 found; it **does not exist** at any real decimal format's
exponent range. Closed, with the measurement that makes the closure decisive rather than merely
plausible.

**Two mechanisms threaded through a policy parameter to make growth policy-dependent**: refused twice,
compiled both directions (file 51). Threading `Policy` into a growth trait's own parameter list
compiles and computes nothing (a dead parameter, checked with a const assertion forcing two policy
instantiations to unify). Making it live any other way needs two conflicting trait impls over the
identical generic domain, which Rust's coherence rules refuse outright (`E0119`), independent of which
two numerals are involved. The stronger result is the second: not "checked several operations, found
none," but "no operation this type system's dispatch discipline can express has policy-dependent
growth," a structural theorem rather than an enumeration.

**File 79's own search claim, named directly because the archive itself names it this way**: "I
searched `[Aa]rity` across every file; the hits are all fold-arity." The search as described was not
run; the actual grep hits span fifty-plus files including the sealed `Arity` carrier's own proposal,
seal, and compiled forgery. File 90 calls this "the panel's equivalent of a tautological test," and
the phrase is exact: it occupies precisely the space where a real search would be noticed missing.
The correct underlying conclusion survived on grounds a different file supplied; the false provenance
did not, and the archive itself names the two sentence swaps its own next consolidation owed to fix
it. This is worth reporting to the new panel not as a citation of the archive's authority but as a
worked example of the failure this workspace's own `the-test-gate.md` names for tests, occurring
independently in a different domain (a search claim rather than a test assertion), which is some
evidence the underlying failure mode is general rather than test-specific.

---

## The rule that governs my own session, and what this slice does to it

`unstable-features.md`, a currently ratified workspace rule I operate under in this very dispatch,
states: "With either [`specialization`, `TypeId`] available, a check at the model width establishes
nothing about the real one... Without them, monomorphisation is uniform and the transfer is sound."

Files 66 through 68 of this archive (file 68's consolidation, section 1.19, is fully within my slice
and is where I read this) found the last sentence conflates two different things: implementation
uniformity (one parametric body, no instantiation gets a different body, which the bans genuinely and
mechanically guarantee) is not the same as property uniformity (whether a checked property's truth
value stays stable as parameters move, which the bans do not by themselves guarantee). Compiled,
twice, with the bans in force: "absorption-freedom" (for all nonzero `y`, `quantise(x + y) != x`) is
exhaustively true at exponent span `p` and exhaustively false at span `p + 1`, with precision, code,
and the bans all held fixed. Same parametric body throughout, no specialisation, no `TypeId`, and the
property's truth value moved anyway. This directly falsifies "monomorphisation is uniform, therefore
the transfer is sound" as a general statement, under the exact conditions the rule's own sentence
describes.

The archive traces this to its own root cause: the rule's source document (file 10, outside my slice)
originally stated the transfer argument as four separate legs and explicitly flagged three of the four
as "arguable in prose... never mechanical" (not proven). The ratified rule text compresses four legs
into one sentence and silently keeps only the mechanical leg's conclusion while carrying the stronger,
unproven claim the other three legs would have needed. This is a real, concrete, in-the-wild instance
of exactly the failure `a-compression-is-checked-by-someone-else.md` (a current workspace rule) warns
about, found in a document that fed a currently-ratified rule.

The same file also found a third mechanism for instantiation-dependent behaviour that the rule's own
"two mechanisms" enumeration (`specialization`, `TypeId`) does not name and does not close: const-tag
container dispatch, compiled and shipped (at the time, in the now-nuked tree) at
`arvo-strategy/src/container.rs:254-280`, with no forbidden feature and no gate. A type observing
which instantiation it is in and behaving differently, achieved through a route neither ban touches.
File 62 second-expert-confirmed this at the whole-crate grain: 16 refusal sites in `arvo-strategy`,
478 in the facade `arvo`, both closing only under the forbidden feature, confirming these two shipped
crates were structurally load-bearing on exactly the feature the rule forbids (which is, itself,
strong background for why this whole redesign effort exists).

None of this weakens the actual ban. Full `specialization` and `TypeId` remain correctly forbidden on
their own soundness grounds, independent of this transfer argument. What this slice's material
changes is the *justification* the rule currently gives for why the ban is sufficient, and it changes
it in a direction the rule's own stated purpose ("verification infrastructure, not only hygiene")
should want fixed rather than left standing. The archive itself proposes a rigorous replacement
methodology (a "transfer ground" scheme: `symmetry`, `saturation`, `induction`, `unargued` as the
honest default) and drafts three specific, non-ban-touching wording edits, none of which I have
independently vetted for wording quality, only for the compiled facts underneath them. I flag this as
the single most directly actionable item in my entire slice, because it names a real gap in a rule
this workspace is currently relying on, with compiled evidence attached, and because fixing the
wording costs nothing but the wording.

**One correction that composes with the above and is worth stating in the same breath**, from file
68: the "wall at nine bits" that `unstable-features.md` also cites is a total-step-count budget, not
a width ceiling. File 68 compiled a cheaper predicate that clears nine bits and refuses only at ten,
under the identical bans. So the rule's second load-bearing figure needs the same kind of care: it is
true of the specific check file 8 ran, not of the width in general.

---

## What I checked myself, as current evidence rather than archive authority

Three checks, run fresh this session against the live repository and toolchain, reported as current
evidence per the dispatch brief's own instruction that a re-run I perform myself may be cited as
such.

**The "unreproducible" claim, independently re-confirmed false, a third time.** Before discovering
that files 62 and 80 had already done this, I independently set up the exact crate layout
`OLD_08_probes/README.md` specifies (`a_union.rs` as `src/lib.rs`, `b_spare_pattern_decides_delivery.rs`
as `src/spare.rs`, `c_split_does_not_bind.rs` as `src/fusion.rs`) under the pinned
`nightly-2026-05-28` toolchain, confirmed present on this machine as an installed rustup toolchain.
It compiles clean, both `cargo build` and `cargo build --bin e_codegen --release` (the source for the
disputed "five-shape instruction table"). This makes three independent instances of the same
correction (mine, file 62's, file 80's), which satisfies this workspace's own preference for three or
more independent instances of evidence over one. I did not re-derive the specific "28.45 seconds at
eight bits" wall-clock figure `unstable-features.md` cites; that traces to file 8, outside my slice,
and remains a narrower, still-open question for whoever holds that slice, separate from the
"unreproducible" claim itself, which is now closed three times over.

**The bench `.gitignore` fix, verified as landed and current.** File 59 found and fixed, on its own
branch, that `mock/benches/.gitignore` was silently discarding every `.csv`/`.meta.json`/`_findings.md`
artifact ever produced, meaning zero bench artifacts had ever been committed anywhere in the
repository. I checked the current `feat/arvo-shape-topic` branch: `mock/benches/.gitignore` now reads
"Bench artifacts (CSV, meta, findings) are TRACKED, not ignored," and `mock/benches/` currently holds
dozens of committed CSV/meta/findings triples, including the exact
`decimal-quantiser-radix-sweep_*` and `quantiser-vs-fadd-subnormal-sweep_*` families files 57 and 59
produced, plus many more from later, unrelated work. Commit `b5421cbb`, "bench: measure the radix-ten
quantiser, and stop ignoring bench artifacts," is in the current git log. This fix is closed and does
not need re-raising; the underlying measured data (real numbers about the software-quantiser-versus-
hardware trade) is still sitting there, committed, and usable as evidence if the new numeral design
keeps anything resembling strategy-marker-driven lowering.

**The bench-orchestrator overwrite defect, flagged but not verified.** File 90 (and file 81 before
it) found a second, distinct infrastructure defect: the bench orchestrator's overwrite behaviour was
destroying every committed CSV, meta, and findings file on each run, forcing six consecutive files to
decline running the harness at all. I did a shallow spot-check of the live `mockspace/bench-harness`
source (`harness.rs:750`'s `write_csv`, and a comment at `matrix.rs:98` acknowledging a collision-
overwrite hazard by name) that suggests the issue may be at least partially known or addressed, but I
did not do the kind of full verification I did for the `.gitignore` fix. **This is genuinely open**:
whoever next touches mockspace's bench-harness overwrite semantics should check whether this specific
historical defect (an orchestrator run silently destroying previously committed artifacts) is actually
resolved, because my check here does not settle it either way.

---

## Bearing on the current panel's live options

**Q6 ("Does `Warm` wrap, or clamp?")** is the live question my slice bears on most directly, even
though the archive's specific four-preset design is explicitly not authoritative per `INTENTS.md`'s
demotion of I1. Files 70 and 71 did substantial, careful work on exactly this shape of question for
the archive's own (now-superseded) strategy markers: the "far point is the supremum of the ordered
representable set" rule (file 71) that unifies clamp-to-finite-maximum and overflow-to-infinity as one
mechanism, derived from an order-theoretic property rather than chosen per case; the real derivation
that `Warm`'s float behaviour cannot simply reuse fixed-point `Warm`'s clamp semantics, because a real
FPU does not clamp, it saturates to signed infinity; and the honest, unresolved open question of what
a `Warm`/`Cold` numeral with no representable infinity should do on overflow (flagged, not settled, in
both files). None of this is authority for the new panel's own answer, since I1 (the four-preset set)
is explicitly open, but the *mechanism* (supremum-of-the-order as the unifying rule, and the honest
naming of the no-infinity gap as a real open question rather than smoothing it over) is directly
reusable regardless of what the new panel's strategy axis ends up looking like.

**Q7 (which carrier the packing claim is about) and the wide-rung bench** connect to my slice's
extensive bitpacked-storage cost work (files 75, 81, 83, and file 90's report that the cost dropped
from 4.6x-5.5x to 1.29x-1.50x once the spine rule was actually pushed through the decoder). The
general lesson, not the specific number, is what transfers: a bitpack-cost multiple measured before a
design rule is fully applied can be a mispriced founding feature rather than an inherent cost, and the
new panel's own strategy-marker or storage-cost work should check whether an apparent cost is
structural or merely unoptimised before it hardens into a design constraint.

**Q9 (the crossing between a consumer's written number and the type system)** connects to the spine
rule's whole arc across my slice (files 50, 54, 55, 73, each an independent firing) and to files 61
and 63's closed work on the notation vehicle (a proc-macro, adopted, after the declarative-macro route
was shown to be structurally walled off from a literal's own digits, twice over). The current panel's
own C0 through C4 arrangements in Q9 are working the same underlying tension (a const-generic surface
versus a nat-keyed algebra, bridged once rather than re-entered) that my slice's spine rule names
generally: a quantity computed and then required in a type is a type, a quantity only read is a const.
Whatever the new panel's own answer to Q9 turns out to be, this general principle is worth checking it
against, since every compiled instance of the rule in my slice held without exception.

**Neither of op's two in-flight questions** (which verb "validate" is in his acceptance criterion,
and whether the long-standing `no_std`/no-`alloc`/const-sizes/no-`dyn`/no-`TypeId` constraints are his
intents at all) is directly answered anywhere in my slice. The closest my slice comes is the
`unstable-features.md` material above, which bears on the `TypeId` half of the second question: my
slice's material does not argue for or against banning `TypeId`, but it does show, compiled, that the
ban's own stated justification (the transfer argument) needs the wording correction described above
regardless of whether the ban itself is ever revisited. This is offered because it bears on an open
question, not as an answer to either.

---

## What I did not cover, stated plainly

I did not read files 46, 47, 48, 64, 65, 66, 67, 69, 73 through 77, 81 through 83, 87, or 88 as
standalone documents; my knowledge of their content comes from consolidations 49, 58, 63, 68, and 78
(read in full) plus targeted section reads and verdict paragraphs, named individually above. I did
not open any lettered checkpoint file directly. I did not independently verify the OCP OFP8 or IEEE
754 primary-source quotes myself; I am relaying files 62 and 80's own account of having read the
primary documents, one level removed from their read, the same status this archive's own rules assign
a claim it has not personally re-derived. I did not re-run any probe beyond the three described under
"What I checked myself." I did not chase the specific "28.45 seconds at eight bits" figure to its
source in file 8, which sits outside my slice.
