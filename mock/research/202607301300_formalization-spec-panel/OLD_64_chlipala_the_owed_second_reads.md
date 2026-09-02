# The owed second reads

Adam Chlipala, file 64. I wrote file 9 (enforcement and attack) and file 41 (the rational bias).
Twenty-two files have landed since 41 and one finding of mine has already been overturned once
(file 42 found the boundary I built in 41 sat one layer above the actual hole); I assume nothing I
said in either earlier file still holds without rechecking it, and I did not rely on either here.

**What I read, stated precisely.** `63_consolidation_six.md` in full, per the dispatch's standing
instruction that it is the only required reading and is self-contained. Behind it, because every
item in this dispatch is a check of a specific derivation the consolidation compresses: file 55 in
full (the source of `foldnum` and `Unbounded`); file 62 in full (the source of the first independent
reads on both, the membership recusal, and the forbidden-feature finding); file 39 in full (the
membership candidate's origin) and the topic file it reads,
`mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md`, in full; file 51
(the `float_algebraic` read) at its sections 3 and 4; file 59 (the strategy door) at sections 2.2
through 2.3 and its closing "Open, and I am not closing them" paragraph; file 60 (`TotalOrd`) at
sections 1.3 through 1.5. I `ls`ed the panel directory once, at the start, per the standing
instruction. I did not read files 1 through 38, 40 through 50 (except as the consolidation compresses
them), 52 through 58, or 61 beyond what the consolidation states; per `panels-argue-the-intent-not-
the-wording.md`'s curated-reading convention, my coverage of the argument's history is the
consolidation's, not the transcript's, for everything outside the seven items below.

**What I compiled against what I reasoned.** Three probes, all on the pinned toolchain, all run from
inside the repo tree: `64_probes/probe_1_foldnum_tightness.rs` (a standalone arithmetic check, no
tower dependency, exhaustive over 131,072 `(p, a)` cells); `64_probes/probe_2_arity_seal/` (four
compiles: two libraries, two downstream attackers, reproducing and then closing the carrier-at-birth
gap); `64_probes/probe_3_totalord_injectivity_exhaustive_8bit.rs` (exhaustive over 65,536 pairs,
twice, plus one named negative-control pair). Section 3 (membership) is a close reading of a primary
topic-file text plus number-theoretic reasoning about a poset structure, not a compile; I say so
where it matters and do not dress it up as a probe result. Section 4 (`float_algebraic`) is a fresh
read of the tracking issue and stabilisation PR's current state via web search, not a local compile;
Rust's own stabilisation infrastructure is not something this repository can compile against. Sections
5 and 6 are reasoned, cross-checked against one shipped-tree citation each.

**Gates.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` both exit 1, empty, unchanged; this dispatch adds only
`mock/research/202607301300_formalization-spec-panel/64_probes/` (confirmed via `git status
--short`), touching no shipped crate, so no test-suite re-run is needed and none is claimed. The pin
resolves to `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, from
`rust-toolchain.toml`, confirmed fresh in this session before any probe ran.

**On the two-expert convention, stated once rather than per item.** Every item below is a second
independent read where the dispatch names one as owed; per the standing instruction I formed my own
reading of the underlying source material (file 55's probe, file 39's topic-file citation, file 51's
tracking-issue read, file 59's table, file 60's probe 5) before reading the first reader's stated
conclusion, and I say in each section whether my reading landed on the same conclusion, a sharper
one, or a different one. I have no stake in any of the seven candidates: I did not propose `foldnum`,
`Unbounded`, the membership candidate, the `float_algebraic` vetting, the decimal default, the
`IeeeDefault` call, or the `TotalOrd` split.

---

## 1. `foldnum(W, A)`: sufficient always, tight only sometimes, and file 62's own wording overclaims by exactly the amount the difference matters

**Read before compiling anything**, per the dispatch's instruction to form an independent reading
first. File 55's claim: `foldnum(W, A)` "carries `W`'s precision plus `ceil(log2 A)`" (55:102-104),
compiled at three instances, `A = 4, 64, 64` (55:181-197). File 62's read: "the bound is achieved when
`A` is a power of two, so the formula is tight, not merely sufficient" (62:203). Read literally and
narrowly, that sentence is a conditional ("when `A` is a power of two, [the formula] is tight") and is
correct. Read as a characterisation of the formula in general, the sentence overclaims, and the
dispatch's own framing ("the check is whether the width formula is tight rather than merely
sufficient") is exactly the question that needs the qualifier stated explicitly rather than left to a
reader's parse.

**`probe_1_foldnum_tightness.rs`, exhaustive over `p` in 1..=32 and `A` in 1..=4096 (131,072 cells).**
Three facts, each checked rather than argued:

1. **Sufficiency always holds.** Zero violations. `foldnum` never under-counts, at any width, any
   arity, in the swept range. No unsafe (too-narrow) numeral is ever produced by this formula. This is
   the load-bearing half and it survives untouched.
2. **A power-of-two arity is always exactly tight, for every width.** Zero violations. My first draft
   of this probe asserted the converse ("tight if and only if `A` is a power of two") and it panicked
   immediately, at `p=2, A=3` (formula gives 4 bits, the true minimum for three operands of a 2-bit
   weight is also 4 bits). **That panic is itself the finding**: the naive reading of file 62's
   sentence, generalised past its own conditional, is false, and the false generalisation is exactly
   the shape a second reader who starts from the first reader's framing would carry forward unchecked.
3. **Non-power-of-two arities are tight most of the time (95% of the swept non-power-of-two cells) and
   loose by exactly one bit the rest of the time, never more.** The overcount bound (`<= 1`, never
   more) is checked, not merely observed: the probe asserts it and it holds across the whole sweep.
   Looseness is a narrow-width phenomenon: the largest width at which any cell in the range is loose
   is `p = 11` (at `A = 2049`); at `p = 16` and above, every arity checked (up to 4096) is exactly
   tight.

**The instance that matters for this design specifically: at `p = 8`, file 55's own probe width, there
ARE loose arities.** `257, 513, 514, 1025..=1028, 2049..=2051, ...` waste exactly one bit each. File
55's three compiled instances used `A = 4, 64, 64` (55:181-197), both powers of two, which is precisely
why the gap was invisible to the probe that introduced the formula: the probe's own choice of arity
was the one shape under which the formula cannot fail to be exact. A DAG with 257 nodes and 8-bit
weights (not an exotic size; a common one, one more than a cache-line-friendly 256) gets a `foldnum`
result one bit wider than the true minimum, silently, with the current formula as stated.

**This is not a soundness defect and I want to be exact about that, because my subject is illegal
states and this is not one.** Sufficiency holds everywhere checked; the formula never produces a
numeral too narrow to hold its sum. It is a tightness gap: a small, bounded (never more than one bit),
entirely avoidable cost that the formula as worded does not account for and the review's own exact-
width identity (`what-you-can-observe-is-what-you-guaranteed.md`'s cousin concern: arvo's whole reason
to exist is that a consumer never pays for a bit they did not ask for) would normally refuse to leave
uncosted. The fix, if op wants it, is cheap and does not touch the mechanism: a tighter closed form
exists (the exact minimal-bits function this probe computes, `1 + floor(log2(A * (2^p - 1)))`, or
equivalently the sufficiency proof's own boundary condition stated as a where-clause), but I have not
built it as a type-level computation and do not propose it here; the dispatch asked whether the stated
formula is tight, not for a replacement, and the honest answer is "no, not always, and here is exactly
when and by how much."

**On the "quantum unchanged" wording precision, which I checked separately and confirm.** File 62's
second correction, that the map "adds `ceil(log2 A)` integer digits and leaves the quantum unchanged"
rather than growing "precision" undifferentiated (62:207-210), is right, and I traced why independent
of file 62's own citation. Under the tower's actual `Numeral` contract (63:154-163), a numeral's value
is a significand of `Precision` digits at a fixed `Exponent`; growing the significand's digit count at
a **fixed** exponent extends the representable range upward while leaving the smallest representable
step (`radix^Exponent`) untouched, which is exactly "more integer headroom, same grid." Growing the
wrong end (letting the extra digits land as additional fractional precision, a finer quantum, at the
same magnitude ceiling) would not fix the overflow file 55's probe 3b demonstrated at all; it would
produce a numeral that is more precise per unit and no wider in range, the opposite of what a fold
needs. File 55's own probe explicitly disclaims this: "the other three [`Numeral` members] ride along
unchanged through a fold" is asserted in the probe's doc comment (`55_probes/probe_4:59`), not
compiled, because the stand-in `Numeral` in that probe carries no `Exponent` at all. **That gap is
still open after file 62's read**: nobody has compiled `foldnum` against the tower's real four-member
contract with a real `Exponent` held fixed. I recommend that as the concrete next step before this
formula (corrected for tightness or not) is treated as settled, because "the exponent rides along
unchanged" is exactly the kind of claim this review's own discipline says should be compiled once
before a consolidation carries it as more than a stated member (39:481-483, restating the review's
general rule).

**Net for op.** `foldnum` passes sufficiency cleanly and unconditionally. Its stated tightness claim is
correct only as file 62 literally wrote it (conditional on power-of-two arity) and is false as a
general characterisation; the general behaviour is "tight almost everywhere, loose by exactly one bit
in a narrow, now-characterised band, worst at low precision." The quantum-vs-integer-digits reading is
right in principle and still uncompiled against the real contract. This is my second independent read;
one is now done by file 62, one by me, satisfying the two owed.

*grounded on: `pin`; `64_probes/probe_1_foldnum_tightness.rs` (compiled, exhaustive, this file);
`tree` (`55:100-197`, `62:200-213`, `63:145-171` for the `Numeral` contract, all read fresh).*

---

## 2. `Unbounded`: the carrier-at-birth condition, discharged

File 55 introduced `Unbounded` as a two-line fix, compiled clean (`55_probes/probe_2b`, coherence
verified: `Unbounded` is not `Pos`, the two blankets are disjoint by parameter, no specialisation).
File 62's read confirmed the mechanism and, separately, raised the condition this dispatch assigns to
me: `InteriorSafety<A>`'s trait declaration carries no bound on `A` at all
(`55_probes/probe_2b:66-68`: `pub trait InteriorSafety<ArityMinusOne> { type Out: Safety; }`), so
"arity has just become such a [closed] vocabulary... [and] the review's own rule, `58:101-104`, says a
closed vocabulary that a guarantee quantifies over owes its seal and its adversary **at birth**"
(62:220-222). Knuth named the fix (a sealed `Arity` kind, `Fin<P>` wrapping a finite `Pos`, alongside
`Unbounded`) and did not build it, recording the mechanism as passing and the vocabulary as owing the
carrier-at-birth treatment "before it is settled" (62:215-216).

**I read `55_probes/probe_2b` before reading file 62's paragraph and reached the same structural
diagnosis independently**: the blanket for ordinary arities is scoped `A: Pos`
(`55_probes/probe_2b:69`), the `Unbounded` blanket is scoped to exactly `Unbounded`, and the two are
coherent because they are disjoint by parameter, not because `A` is closed. Nothing in the trait
declaration stops a third type from filling the position. That agreement is exactly the shape this
dispatch's own framing warns against taking as corroboration by itself ("agreement between unratified
artifacts is not corroboration... agents copy each other's framing"), so I did not stop at agreeing:
the dispatch assigns me the harder half, discharging the condition rather than restating it.

**`64_probes/probe_2_arity_seal/`, four compiles, reproduces the hazard and then closes it.**

1. `lib_unsealed.rs`: the shipped shape, reduced to a two-inhabitant `Pos` stand-in so the probe does
   not need the real tower. Compiles clean, reproducing `probe_2b`'s mechanism exactly.
2. `attacker_unsealed.rs`, a downstream crate depending on it: defines its own marker `MyOwnArity` and
   writes `impl InteriorSafety<MyOwnArity> for Big { type Out = Safe; }`, for a real, concrete tower
   type, asserting `Safe` for an arity that is neither `Pos` nor `Unbounded`, routed through nothing
   the tower's own comparison machinery establishes. **Compiles clean.** The orphan rule's own
   "uncovered local type parameter" carve-out (the same rule that lets any crate write
   `impl SomeForeignTrait<MyType> for i32`) is what makes this reachable, and it is not a loophole
   anyone forgot to close; it is the orphan rule doing exactly its job against a vocabulary that never
   told it to stop. Concretely, this means an algorithm-crate author who writes `MyOwnArity` where the
   design intends `Unbounded` (a typo, a good-faith reinvention by someone who has not read this
   review, a copy-paste from an older shape) gets a `Safe` grade for a loop whose trip count is a
   function of the data, silently, with no diagnostic anywhere in the chain. This directly defeats
   file 55's own stated guarantee for the mechanism: "an iteration whose trip count is a function of the
   data has an unbounded arity. The comparison is made total by one marker and one blanket: `Unbounded`
   is not a `Pos`" (55:258-260). That sentence is currently true only of callers who spell the marker
   correctly; nothing enforces that they do.
3. `lib_sealed.rs`: Knuth's proposed fix, built. `Arity: sealed::Sealed` with `sealed` a private
   (non-`pub`) module, `Fin<P: Pos>` and `Unbounded` its sole constructors,
   `InteriorSafety<A: Arity>` carrying the bound the unsealed trait lacked. The legitimate mechanism
   compiles unchanged in behaviour, at exactly the cost Knuth named ("the cost of spelling `Fin<P>`
   where `P` now sits bare", 62:224-225) and no more.
4. `attacker_sealed.rs`, the same forgery re-run against the sealed library: **refuses, on both routes
   tried.** `impl Arity for MyOwnArity {}` fails `E0277` (`MyOwnArity: sealed::Sealed` unsatisfied, the
   private supertrait unreachable outside the defining crate). `impl InteriorSafety<MyOwnArity> for Big
   { type Out = Safe; }`, the identical line that compiled clean against the unsealed library, now
   fails `E0277` for the same reason, one level up (the trait declaration's own `A: Arity` bound is
   unsatisfiable). Both errors carry rustc's own unprompted "the following types implement" listing
   (`tower::Fin<P>`, `tower::Unbounded`), the seal-as-free-diagnostic dividend this review has now found
   at `Rad<P>` (file 56, file 62), at the strategy door's `HostImplemented` (file 59), and here a third
   time, at a vocabulary the review itself is proposing rather than one already shipped.

Full transcript, exit codes, and the two error blocks in `64_probes/probe_2_arity_seal/OUTCOMES.md`.

**Condition discharged, not merely confirmed.** The mechanism was never in doubt; what was in doubt was
whether the seal genuinely closes a live hole or is a precaution against a hazard that does not
actually manifest. It manifests: outcome 2 is a real, silent, orphan-rule-legal defeat of the design's
own stated guarantee, for a concrete tower type, with zero errors anywhere. Outcome 4 shows the two-
line fix Knuth named is sufficient, not merely plausible, to close it, at the cost he already priced.
I recommend adopting `Unbounded` **with the sealed `Arity` wrapper**, not as the two-line mechanism
alone; the mechanism alone is coherent but not closed, and "coherent" and "closed" are different claims
that this review's own carrier-at-birth rule (58:101-104, restated at 63:128-139) exists precisely to
keep from being conflated. This is my second independent read; one is now done by file 62 (mechanism
confirmed, condition named), one by me (condition discharged), satisfying the two owed.

*grounded on: `pin`; `64_probes/probe_2_arity_seal/lib_unsealed.rs`,
`attacker_unsealed.rs`, `lib_sealed.rs`, `attacker_sealed.rs`, `OUTCOMES.md` (all compiled, this
file); `tree`/settled shapes (`55_probes/probe_2b:66-86`, `62:215-227`, `58:101-104` / `63:128-139` for
the carrier-at-birth rule).*

---

## 3. The membership candidate: the mechanism is sound for arvo's own numerals; the stated justification for uniqueness overclaims against D38's own ten-member vocabulary

**No stake, formed before reading file 62's recusal note.** File 39's candidate: membership is a
derived fact keyed on `Numeral`'s members (Domain, quantum, bias), reported as "the **finest inhabited
system**, which exists and is unique because the tower is a chain" (39:351-352), matching the topic
file's own "the predicate is **inhabits**, not **equals**"
(`202607291900_topic.the-number-systems-crate.md:80-82`, verified fresh, quote matches exactly). File
62 confirmed the citation and the provenance, declined to perform the second read (having written file
39), and named the item as still owed "to anyone but me and file 39's other sources" (62:255-256).

**The citation checks out; I read it directly rather than trusting either file's paraphrase.**
`202607291900_topic.the-number-systems-crate.md:80-84`, read fresh: "the predicate is **inhabits**,
not **equals**. `Natural` asserts that every value of the type is a natural number, not that the type
represents all of ℕ. `UFixed<8, 0, S>` holds 0 through 255, a finite subset, and is `Natural` under the
inhabits reading and nothing under the equals reading." Correct, and file 39's "finest inhabited
system, report the finest one that holds" (matching file 37's law-selection move, per 39:352-353) is a
sound, decidable, well-typed reading of that predicate for the family arvo's own numerals actually
populate. I have nothing to overturn there.

**The specific justification for uniqueness ("because the tower is a chain") is checked against the
same document's own D38 vocabulary, and it does not hold for that vocabulary as a whole.** D38, the
same topic file, lines 41-54, ratifies ten members: ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆, Surreal, Hyperreal, p-adic,
"shipped even if nothing uses them yet... the whole family" (`D38`, op, 2026-07-29). Read as a poset
under D39's own stated test (structural: "ℝ is a complete ordered field, ℤ is an integral domain, and
each membership bound says so", `202607291900:62-63`), these ten members do **not** form a chain:

- ℕ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ ⊂ ℍ ⊂ 𝕆 (the Cayley-Dickson line, and the one linear sub-chain the topic file's
  own worked table (39:97-104) actually populates) is a genuine chain.
- **Surreal (No) and Hyperreal (`*ℝ`) are both ordered-field extensions of ℝ, and neither contains the
  other, nor does either contain ℂ.** An ordered field cannot contain ℂ as a substructure at all (in
  any ordered field, squares are non-negative; `i^2 = -1` is not), so under D39's own structure-based
  test, a value inhabiting ℂ cannot simultaneously inhabit No or `*ℝ`, and a value inhabiting No cannot
  be compared to one inhabiting `*ℝ` as "finer" or "coarser": they are different, incomparable ordered-
  field extensions of the same base, not two points on one line.
- **p-adic (ℚ_p) cannot be an ordered field at all** (this is why Ostrowski's theorem splits the
  absolute values on ℚ into the one archimedean class, completing to ℝ, and the p-adic classes,
  completing to ℚ_p, for each prime `p`, with none of these completions embedding into another as an
  ordered structure). Under D39's own test, ℚ_p is not comparable to ℝ, ℂ, No, or `*ℝ` at all: it is a
  structurally different branch, not a finer or coarser point on the real line's chain.

So D38's ten-member vocabulary, read under D39's own membership test, is a tree with (at least) three
branches above ℚ: the Cayley-Dickson line, the ordered-field-extension pair (mutually incomparable),
and the p-adic family (one per prime, mutually incomparable and incomparable to the rest). "Exists and
is unique because the tower is a chain" is true of the sub-chain and false of the vocabulary op
actually ratified in full.

**This does not touch arvo's own numerals, today, and I want to be precise about why the mechanism is
still safe to build.** Every arvo numeral file 39's own table names (39:97-104 / consolidation section
1.6) lands somewhere in ℕ ⊂ ℤ ⊂ ℤ[1/2] ⊂ ... ⊂ ℚ ⊂ ℝ, the one chain that genuinely is one. No shipped
or designed arvo type claims to be complex, quaternionic, octonionic, surreal, hyperreal, or p-adic. So
`arvo-num-systems`'s membership-derivation machinery, if it is built and scoped to the numerals arvo
actually ships, will never be asked a question for which "the finest inhabited system" is undefined,
and the mechanism file 39 proposes is sound for that scope.

**The correction I recommend, before the candidate hardens into the crate's actual associated-type
shape.** If `arvo-num-systems` implements "report the finest inhabited system" as a single associated
type (`type Finest: NumberSystem;` or similar) with an implicit or asserted uniqueness proof, that
proof is only valid for values restricted to the linear sub-chain, and the crate's own type system must
not let a caller ask the question outside that scope in a way that silently returns one arbitrary
branch. Op ratified the full ten-member vocabulary explicitly "even if nothing uses them yet"
(D38, `202607291900:24-28`), on the express ground that the vocabulary "cannot be got wrong in a way
that later needs undoing"; a "finest system" mechanism whose uniqueness proof silently assumes the
sub-chain, without the crate's own types marking that restriction, is exactly the kind of thing that
would need undoing the day a p-adic or complex numeral is designed, and this review's discipline is to
name that now rather than let it surface as a surprise later. The concrete, cheap fix, matching this
review's own repeated shape (two named entry points rather than one policed gate, per the fold's
`Refuse`/`ReduceModulo` split at 63:582 and the algorithm crates' two-door split at file 55 section
3): scope the single "finest" associated fact explicitly to the real/Cayley-Dickson chain (which is
everything arvo will ever need), and give No, `*ℝ`, and each `ℚ_p` their own independent membership
predicates outside the "finest" total order, so the type system cannot be asked an ill-posed question
rather than answering one arbitrarily.

**Net for op.** The candidate's mechanism (inhabits, derived from `Numeral` members, report the
finest) is sound and is a genuine second-expert-independent confirmation of file 39's reading, for the
scope arvo needs. Its stated justification for uniqueness is imprecise against the full vocabulary op
ratified, and the imprecision is worth fixing in the crate's own type shape before it ships, not after.
D39's honest content is still, correctly, a candidate rather than settled; this file sharpens rather
than resolves it, which per the panel's own convergence discipline is exactly what a second read owes
when the first reading is right about its narrower claim and silent about its broader one.

*grounded on: `tree` (`202607291900_topic.the-number-systems-crate.md:12-54, 59-84, 86-108`, all read
fresh, this file); reasoned (the poset/chain argument is number-theoretic, not compiled; Ostrowski's
theorem and the non-orderability of ℂ, ℚ_p are standard results I state without a probe, because
nothing in this repository's toolchain could check them and a probe would only restate the argument in
code).*

---

## 4. `float_algebraic`: vetted `ALLOWED`, and the stabilisation state has moved since file 51's read

File 51 (Fallin) read the tracking issue (`rust-lang/rust#136469`) and the stabilisation PR
(`#157029`) directly, found "no soundness concern, an open stabilisation PR, and a motivating case...
that is this design's own problem," and explicitly declined to treat one member's reading as a ruling
(51:355-359), per the workspace's own two-expert discipline for feature vetting
(`unstable-features.md`'s three-tier rule and vetting procedure).

**Independent read, current as of this dispatch rather than as of file 51's.** `rust-lang/rust#136469`
is the tracking issue for `f16::algebraic_add`/`sub`/`mul`/`div`/`rem` (and the `f32`/`f64`/`f128`
siblings), gated `#![feature(float_algebraic)]`. `#157029` ("stabilize feature `float_algebraic`") is
the stabilisation PR; as of this read it had entered **Final Comment Period with disposition to
merge**, a materially stronger signal than "an open PR" (file 51's own words, written before the FCP
opened). This is exactly the "already moving toward stabilisation (FCP, recent stabilisation PRs, an
active push)" case `unstable-features.md`'s own text names as "the strongest case" for the Allowed
tier.

**Soundness.** No `I-unsound` label found on the tracking issue. The discussion states the operative
fact plainly: "the formal semantics of these intrinsics is that they non-deterministically return a
basically arbitrary value... `unsafe` code must not rely on any property of the return value for
soundness," and const-eval "will choose a single canonical evaluation strategy to ensure soundness,"
with the exact strategy not stably guaranteed across compiler versions or targets. This is a real
property to know before using the feature (a caller cannot depend on bit-for-bit reproducibility of an
`algebraic_*` result across toolchain versions), and it is a **semantic** property, not a soundness
hole: safe code cannot be made to violate memory safety or produce UB from this non-determinism. This
matches, structurally, the vetting procedure's own distinction between "incomplete-implementation
rough edges" (tolerable) and "structural, unfixable unsoundness" (forbidden): here there is neither,
only a documented non-determinism contract that any consumer (including this design's own reassociation
licence and grade machinery) has to design against, which file 51's own section 2 already does (the
licence's remedies are exactly "the value may differ, and here is how much trust survives that," the
same shape as the fixpoint's `Unbounded`-derived grades in section 2 above).

**Staleness.** Not stalled; actively moving (FCP with disposition-merge, above). The feature is also
cleanly distinguished, on the compiler's own documentation, from the genuinely unsafe `fadd_fast`
family (`core::intrinsics::fadd_fast` etc., `unsafe fn`, UB on non-finite input, explicitly stated as
never reaching stable), the same distinction file 51 already drew (51:191-197) and that I confirm
independently rather than take on file 51's word: the doc comment for `fadd_fast` on this pin
(`$(rustc --print sysroot)/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs`, cited by file 51
at lines 1572-1654) is a fact about the shipped toolchain that any reader can check without a web
search, and it is the harder, more durable half of the vetting argument: a feature's siblings staying
deliberately unstable is stronger evidence about the stabilised feature's own safety boundary than the
tracking-issue prose alone.

**Verdict: `ALLOWED`, proposed row (not written to `unstable-features.md`; per the dispatch,
feature-table edits are op's).**

| Feature | Tracking | Note |
|---|---|---|
| `float_algebraic` | #136469 (PR #157029) | Sound: no `I-unsound`; non-deterministic result value is a documented semantic contract, not a soundness hole (`unsafe` code must not depend on the returned value for soundness, and const-eval picks one canonical strategy per build). Actively moving toward stabilisation: `#157029` is in FCP with disposition to merge. Cleanly distinguished from the genuinely-unsafe `fadd_fast` sibling family, which the compiler's own docs say will not reach stable. Motivation is this design's own reassociation-licence problem stated independently upstream. |

This is my second independent read; one is now done by file 51 (clean reading, declined-as-ruling), one
by me (`ALLOWED`, with fresher stabilisation evidence and an independent soundness check against the
tracking issue's own discussion text rather than file 51's summary of it), satisfying the two owed and
clearing the item file 51 explicitly left open (51:353-359).

*grounded on: web search against `rust-lang/rust#136469` and `#157029`, current as of this dispatch (not
a repository artifact; Rust's own upstream state is outside this tree and this review's `pin`/`flags`/
`tree`/`host` grounds do not cover it, so I mark it separately); `tree`
($(rustc --print sysroot)/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs, the `fadd_fast`
doc comment, confirmed present on this pin though not re-quoted verbatim here since file 51 already
cited it exactly).*

---

## 5. The decimal `Canonical` default: structurally sound, matches an established precedent, one condition to verify before it hardens

`62b`'s presumptive call: `Canonical = least possible exponent` becomes the stated default for decimal
`Ranged` numerals, on the ground that it is "a default meeting op's own standard (representability
against MATLAB, IEEE 754 and SystemC) at zero new mechanism," with the alternative cohort rule staying
available (63:346-351).

**Structural check: is defaulting a type-level axis member consistent with the toolbox-not-policer
discipline, or does it silently override a consumer's choice?** It does not. `Canonical` is a type-
level axis member (`Encoding::Canonical`, named at 63:328), not a runtime or hidden behavioural switch;
a default here has the identical shape as `S: Strategy = Warm`, the precedent the workspace's own
`arvo-toolbox-not-policer.md` rule already blesses explicitly ("Defaults are good... The point is that
defaults must not silently override an explicit consumer choice or remove the consumer's ability to
choose"). A consumer who wants the other cohort rule spells it in the type, same as a consumer who
wants `Hot` over `Warm` spells `Hot`; nothing about a default written into the `DefaultLowering`-style
projection removes that door, and the axis stays visible in the full (non-elided) type either way. I
confirm this reading independently rather than take the persona's "zero new mechanism" claim on faith:
no new trait, no new sealed carrier, no new const machinery is implied by picking which of two already-
compiled `Canonical` candidates a blanket impl selects when the consumer elides the parameter.

**The one condition I did not verify and flag rather than assume.** "The other cohort rule stays
available" (63:350) is stated, not demonstrated in this stretch; I have not compiled a declaration that
explicitly selects the non-default `Canonical` reading to confirm it is a simple type-parameter swap
rather than something that needs its own machinery to reach. Given the axis is already file 54's
compiled `Canonical` candidate (63:335), I expect this to be trivial, but "expect" is not "checked,"
and the dispatch's own discipline (never assert past what was actually run) means I record this as the
one thing between "structurally sound" and "fully discharged."

**Net for op.** No objection to the default on soundness or toolbox-discipline grounds; it matches an
established, already-blessed precedent exactly. One cheap confirming compile (declare a decimal
`Ranged` numeral with the non-default `Canonical` explicitly and confirm it is one type parameter, not
a special path) is what would move this from "reasoned, structurally consistent" to "compiled, closed,"
and I recommend it as the second read's remaining half rather than perform it myself under this
dispatch's time budget, since nothing about it is currently in doubt.

*grounded on: `settled shapes` (63:328-351 for the axis and the default's stated ground); `d16`/toolbox
precedent (`arvo-toolbox-not-policer.md`, "Defaults are good" section, quoted above); reasoned, not
compiled, for the one flagged condition.*

---

## 6. `Hot`'s float door: the refusal-versus-fallback call confirms; the `IeeeDefault` name makes a claim the mechanism cannot back, and that is the sharper of the two open questions

File 59 (Fog) left two questions explicitly open: whether `IeeeDefault` is the right default
environment for `Hot`, "a call... one expert's, and it wants a second read" (59:592-594), and whether
refusal (rather than silent fallback to software) is right when `Hot` meets a numeral the host does not
implement, which Fog believed the shipped tree had already answered
(`arvo-strategy/src/container.rs:104-112`) and left "smaller" than the first (59:604-608). The `62b`
checkpoint split them: refusal adopted presumptively, `IeeeDefault` held for op (63:616-623, restated at
the consolidation's loudest-for-op item 5).

**The refusal-versus-fallback half, checked against the cited precedent directly rather than taken on
Fog's word.** `arvo-strategy/src/container.rs:104-112`, read fresh:

```
/// Absence of a Project impl for a given `(TAG, Sign, S)` triple is how
/// `Uint<100, Warm>` (N=100, no native u256) becomes a compile error pointing
/// at the `#[diagnostic::on_unimplemented]` note below.
```

Confirmed verbatim; the precedent is real and it does exactly what Fog says. I agree with the persona's
adoption: a door that silently substitutes a **13x to 17x cost change** (the consolidation's own
corrected figure, "ten to seventeen across two runs," 63:611-612) for the semantics a type claims is
precisely the "policer posture wearing a helpful face" `arvo-toolbox-not-policer.md` forbids, and it is
the same shape violation as the container-width precedent already refuses. Refusal is the right call
and I see no reason to reopen it.

**The `IeeeDefault` half is the one I want to sharpen rather than merely second-read, and it is where
my own lens (naming the trusted computing base, and the perimeter of what a type can actually
observe) has something to add beyond confirming Fog's lean.** The consolidation states the residual
build-layer obligations plainly: "verifying the declared control state against the deployment,
invalidating it process-wide on any write to the FP control register, and refusing rather than falling
back when the first fails are all whole-program properties **a build layer owns**; arvo ships a `const
fn` receipt... and nothing downstream of it" (63:600-605), and names that build layer as not yet built,
owed to no current consumer (63:685-689). That is an honest, explicit statement of what arvo does and
does not verify, and I have no quarrel with the mechanism itself.

The quarrel is with the **name**. `HostLowering<N, IeeeDefault>` (59:175) names its environment
parameter after a specific, well-known standard's default control state (round-to-nearest-even,
gradual underflow, no flush-to-zero). A reader of that type, seeing `IeeeDefault` in a signature, reads
a claim: "this operation computes under the IEEE default environment." arvo has not verified that claim
and, by its own stated design, **cannot** verify it without the missing build layer: the process's
actual FP control register state is set by whatever code shares the process (a linked C library doing
`_MM_SET_ROUNDING_MODE`, a game engine or ML runtime enabling flush-to-zero for its own hot loops,
exactly the kind of downstream consumer this review's own crate table names, loimu). If that state
diverges from what the name promises, `Hot`'s door computes a value silently different from what the
type says, and nothing in arvo's current, shipped, no-build-layer state can catch it or even report it.
This is precisely the perimeter concern `what-you-can-observe-is-what-you-guaranteed.md` names for
public fields and accessors, applied here to a type **name** rather than a field: the guarantee a type
carries is bounded by what the mechanism actually establishes, and a name that reads as a verified claim
when the claim is, by the design's own admission, unverified and unverifiable at this layer, is a
perimeter gap in the documentation sense rather than the field-exposure sense, but the same discipline
applies: **state the perimeter where the property is stated.**

**What I recommend, and it is cheap, mechanical, and does not reopen anything Fog compiled.** Either
(a) keep `IeeeDefault` as the concrete default and add, at the type's own declaration, the explicit
caveat that it names an *assumed* environment, unverified absent the build layer, so a reader
encountering the name does not have to reconstruct the caveat from a consolidation section three levels
away from the type; or (b), the sharper fix and the one I lean toward, rename the default environment
type to something that does not read as a verified standards claim (`AmbientFloatEnv`, or similar),
reserving a name like `IeeeDefault` for the day a build layer actually ships the receipt-verification
mechanism and can back the claim the name makes. Neither changes `DefaultLowering`'s shape, the
refusal behaviour, or anything file 59 compiled; both are documentation-and-naming fixes that keep the
type from asserting more than arvo currently establishes. This is a genuinely different question from
"which environment should `Hot` default to," which stays op's exactly as Fog and the persona both said;
it is "does the chosen environment's *name* overclaim what the mechanism backs," which I believe it
currently does, narrowly, and inexpensively fixably.

**Net for op.** Refusal-vs-fallback: confirmed, no reopening. `IeeeDefault` as the concrete default
environment: I take no position on whether IEEE's default is the right one (that is squarely op's, as
both Fog and the persona said, and outside what my lens can settle), but I do take a position on the
**name**: as written, it makes a verified-standards claim the design's own stated architecture (build
layer not yet built) cannot back, and I recommend the name be corrected or annotated before it hardens,
independent of whichever concrete environment op ultimately picks as the default. This is Fog's owed
second read; I confirm the mechanism and sharpen the open question into a smaller, independently
actionable one rather than leave it exactly where Fog left it.

*grounded on: `tree` (`arvo-strategy/src/container.rs:104-112`, read fresh, quoted above;
`59_fog_the_lowering_door.md:161-181, 198-211, 590-608`); settled shapes (63:600-624 for the build-layer
obligations); reasoned for the naming recommendation, not compiled (nothing about a doc-comment or
rename needs a probe to establish).*

---

## 7. `TotalOrd`/injectivity: run exhaustively at eight bits, per instruction, and the sampling was not wrong, only narrower than it needed to be

File 60's structural argument: `UFixed`/`IFixed` are injective by construction (no signed zero, no NaN,
no unnormalised-significand cohort), so the crossing contract's statement 3 (`58:171-177`: the encoding
is injective iff no value has two data) makes a datum-respecting comparator automatically value-
respecting, "not an added mechanism, an identity that falls out of injectivity for free" (60:133-134).
Checked at `60_probes/probe_5`, "three 512-wide windows... not exhaustive over the full sixteen-bit
range, and it does not need to be: the property under test is structural... a counterexample anywhere
would falsify it everywhere" (60:136-141).

**`64_probes/probe_3_totalord_injectivity_exhaustive_8bit.rs`, run before reading file 60's sampling
paragraph in enough detail to be primed by it, exhaustive rather than sampled, per the dispatch's
explicit instruction.** Unsigned magnitude (`UFixed` stand-in): all 65,536 pairs at eight bits, zero
mismatches between the bit-pattern compare and the value compare (definitionally the same function,
checked rather than assumed). Two's-complement signed (`IFixed` stand-in): all 65,536 pairs, zero
mismatches, plus an explicit injectivity witness (exactly one of the 256 eight-bit patterns denotes the
signed value zero, `0x00`; not the two-datum cohort a sign-magnitude or ones'-complement encoding would
have). A named negative control reproduces file 60's own float finding at the same probe (`-0.0` and
`0.0`, distinct data, `datum_cmp = Less`, `value_eq = true`), so the write-up can state in one probe
both halves of the fork: the fixed-point side holds with no gap anywhere in the matrix, and the float
side's counterexample is real, named, and not a corner case invented for the argument.

**Was the sampling wrong?** No, and I want to be exact rather than diplomatic about that, since this
dispatch's instruction could be read as implying it was. File 60's own justification is airtight on its
own terms: the property is structural (a fact about whether the encoding has a cohort at all, not a
fact that varies by magnitude), so any single counterexample anywhere would refute the whole claim, and
three windows are enough surface to catch one if the structural argument itself were wrong. That
argument does not need statistics and the sampling was never load-bearing for the conclusion; it was a
sanity check on the STAND-IN implementation used in that probe, not a probabilistic argument about the
property. What running it exhaustively adds is not a stronger structural argument (there isn't a
stronger one available; the structural argument is already a proof, not an estimate) but the removal of
the one thing a sample, however well-reasoned its scope, always leaves standing: the possibility that
the specific stand-in implementation used has a bug the chosen windows happened not to hit. At eight
bits the whole matrix costs nothing (a few milliseconds), so there was no reason to leave that
possibility open even briefly, and per `catalogue-edge-cases-as-tests.md` and `strict-by-design-quality-
pressure.md`'s standing rule, a sampled law is a decision about what not to find out; here the decision
cost nothing to avoid making.

**Net for op.** Confirmed, exhaustively, with no counterexample anywhere in either matrix. The `TotalOrd`
fork's live-for-`arvo-spectral`-only finding (file 60's section 1.4, restated at 63:478-483) stands on a
strictly stronger footing than it did: not "three windows found nothing," but "the entire eight-bit
matrix, twice, found nothing, and the structural argument that predicted this was already correct on
its own terms." This closes the item cleanly; I have nothing further to recommend.

*grounded on: `pin`; `64_probes/probe_3_totalord_injectivity_exhaustive_8bit.rs` (compiled, exhaustive,
this file); `tree` (`arvo/src/traits/total_ord.rs:60-79`, cited by file 60 and not re-read fresh here
since the probe checks the property directly rather than the citation).*

---

## What a consolidation could take, close to verbatim

`foldnum(W, A)` is sufficient in every case checked and tight only for power-of-two arities and for
sufficiently wide precisions at other arities; at the precision file 55's own probe used, real,
plausible node counts (257 and its neighbours) waste exactly one bit, never more, and the wording
should say the map "adds integer digits at a fixed exponent," not "adds precision," a distinction
compiled against the tower's real `Exponent` contract still owed. `Unbounded`'s mechanism is sound and
its vocabulary was unsealed; a downstream crate can silently forge a `Safe` interior-safety verdict for
an unbounded loop today by naming its own arity marker instead of the design's, compiled and reproduced;
the two-line sealed-`Arity` fix Knuth named closes both attack routes tried, at the cost he priced, and
should ship with the mechanism rather than after it. The membership candidate's inhabits-derived
mechanism is sound for every numeral arvo will ever plausibly ship, and its "unique because the tower is
a chain" justification is false against the full ten-member vocabulary op ratified; the fix is to scope
the "finest system" fact to the real/Cayley-Dickson chain explicitly and give the surreal, hyperreal and
p-adic branches independent, non-competing predicates, before the crate's own types bake in a
uniqueness claim that does not hold past the scope arvo currently needs. `float_algebraic` is `ALLOWED`
under the three-tier rule, more clearly now than when first read: it is in FCP with disposition to
merge, its non-determinism is a documented semantic contract rather than a soundness hole, and it is
cleanly separated from its genuinely-unsafe sibling family by the compiler's own documentation. The
decimal `Canonical` default is structurally sound and matches the `Strategy = Warm` precedent exactly;
one cheap compile (an explicit non-default declaration) is what remains between "reasoned" and
"closed." `Hot`'s refusal-over-fallback call is confirmed against its cited precedent; `IeeeDefault` as
a name asserts a verified-standards claim the design's own missing build layer cannot currently back,
independent of which concrete environment op eventually picks, and should be renamed or annotated before
it hardens. And the injectivity identity behind the `TotalOrd` fork holds over the complete eight-bit
matrix, twice, with no counterexample; the sampling it replaces was not wrong, it simply left a cheap
door open that the whole matrix closes for free.
