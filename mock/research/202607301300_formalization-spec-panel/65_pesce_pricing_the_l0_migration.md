# Pricing the L0 spine-rule migration

Angelo Pesce, file 65. I wrote file 30, which assembled the identity half. Thirty-five files have
landed since and I assumed nothing in it still holds; I did not rely on it here and nothing below
descends from it.

**What I read, stated precisely.** `63_consolidation_six.md` in full, per the standing instruction
that it is the only required reading and is self-contained, and `64_chlipala_the_owed_second_reads.md`
in full, being the only deliverable since. I `ls`ed the panel directory once at the start. Beyond the
review I read the shipped tree extensively, which is unusual here and is what this dispatch is for:
`arvo-strategy/src/{lib.rs, container.rs, width.rs, identity.rs, arith.rs, arith_macros.rs}`,
`arvo-storage/src/{bits.rs, meta_bits.rs}`, `arvo/src/{lib.rs, ufixed.rs, ifixed.rs, aliases.rs,
strategy.rs, markers.rs, fixed_scale.rs, layout_assertions.rs}` and `arvo/src/traits/*`, plus the
ratified rules this decision sits under (`unstable-features.md`, `arvo-toolbox-not-policer.md`,
`arvo-compile-time-last.md`, `arvo-bridge-home-rule.md`, `no-bare-primitives.md`). I also read
`mock/research/sketches/202607282100_container-projection-without-gce/`, which nobody in this review
has cited and which turns out to matter a great deal; section 2 is about that.

**What I compiled or measured against what I reasoned.** Nine probes plus one whole-crate migration,
all on the pinned toolchain, artifacts in `65_probes/` with `OUTCOMES.md`. Everything in sections 3
through 8 is a compile result or a wall-clock measurement and says which. Sections 9 and 10 (the
estimate and the sequencing) are reasoned from those numbers and are labelled as reasoned; they are
the only place I extrapolate, and I say what the extrapolation rests on. Where I project past the
last measured point I say "projected" and give the fit.

**Gates.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` both exit 1, empty, run fresh from the repo root. This dispatch adds only
`mock/research/202607301300_formalization-spec-panel/65_probes/`; **no file under `mock/crates/` was
edited**, the repository being mid-round at TOPIC. The whole-crate migration work in section 7 was
done on a copy of `mock/` outside the tree, at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/arvo-copy/mock`,
and its diff is committed here as an artifact rather than applied. The pin resolves to
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, confirmed fresh in this
session before any probe ran. A bare `rustc` invoked outside the tree resolves to stable and answers
`E0554` to every gate rather than the question, exactly as the dispatch warns; every probe below was
run from inside the tree.

---

## 1. The consolidation's own claims, checked before anything was built on them

Four factual claims in `63` bear directly on this dispatch. I checked all four rather than inherit
them, per the instruction to check cheap claims including the consolidation's.

**"Sixteen refusal sites in `arvo-strategy`" (63:120-121, 63:809-810). Confirmed exactly, and it is
narrower than the number suggests.** Gate stripped from `arvo-strategy/src/lib.rs:11` and nothing else
touched: 16 diagnostics, every one `error: generic parameters may not be used in const operations`.
All sixteen are in **one file**, `container.rs`, on **eight distinct source lines** (254-280), inside
**four impl blocks**, produced by **two const fns** (`tag_hot_cold`, `bytes_for_u16`) each appearing
twice per block. The count sixteen is a count of diagnostic spans, not of things to fix. There is one
mechanism here, instantiated four times.

**"Four hundred seventy-eight in the facade" (63:121, 63:810). Confirmed exactly, and its structure is
the load-bearing part.** 478 diagnostics, all the same error, on **246 distinct source lines** across
**eight files**. Parsed out of the JSON diagnostic stream by which const fn each primary span sits in:

| const fn in the span | spans | distinct lines |
|---|---:|---:|
| `ufixed_bits(I, F)` | 237 | 119 |
| `ifixed_bits(I, F)` | 209 | 105 |
| `frac(F)` | 22 | 14 |
| `ibits(N)` / `fbits(N)` (the four public aliases) | 6 | 4 |
| `tag_one_representable(I.raw())` | 2 | 2 |
| `is_fractional(F)` (the two static asserts) | 2 | 2 |

**93% of the facade's refusals are one expression**: the logical width, `I + F` unsigned and
`1 + I + F` signed. Six mechanisms, not 478 problems.

**"`min_generic_const_args` refuses identically; the const-block escape it names in turn needs
`-Znext-solver=globally`" (63:812-814). Confirmed on the first half, and I have the exact wording of
the second.** `probe_1` reproduces the shipped `UFixed` shape under `min_generic_const_args` and gets
`error: complex const arguments must be placed inside of a const block`, two sites. `probe_1b` wraps
in `const { }` as instructed and the escalation is to **`generic_const_args`**, not to a solver flag
directly: `= help: add #![feature(generic_const_args)] to allow generic expressions as the RHS of
const items`, four sites. The solver-flag exclusivity is recorded at
`unstable-features.md:132` and I did not re-derive it. The consolidation's sentence is right; the
compiler's own words are one step shorter than it states.

**"Touching every consumer of `Bits`, `UFixed` and `IFixed`" (63:816-817). This is the claim that is
wrong, and it is the one the two-orders-of-magnitude spread rests on.** Sections 6 and 7 measure it.
The short version: `arvo-strategy`'s half touches no consumer at all, and the facade's half leaves
`Uint<N>`, `Int<N>` and `Bits<N>`, which are what consumers actually write, spelled exactly as they
are today.

---

## 2. There is a committed sketch that already did a third of this, and nobody in this review has cited it

`mock/research/sketches/202607282100_container-projection-without-gce/`, dated 2026-07-28, six days
before the consolidation asked for this pricing. Its `FINDINGS.md` records **WORKS, zero feature
gates required**, reproduces the projection's full shape, and states the move: the bucket stops being
an integer computed by a const fn and becomes a closed vocabulary of types, with a `WidthFor<Family>`
trait mapping a width to its bucket as an associated type. It ran the same three-build ladder file 59
and file 62 later ran independently and I ran again this morning, and got the same three answers.

I am not scolding anyone for this. `panels-argue-the-intent-not-the-wording.md`'s curated-reading
convention is exactly why a long panel stops reading the tree, and it is the right convention. But the
consequence is worth naming plainly, because it will recur: **the review spent two dispatches
establishing a ladder that was already committed with its outcome, and then named pricing as
63:850's "highest-leverage item" while a sketch holding a third of the answer sat one `ls` away.** The
standing instruction to `ls` the surrounding directories exists for this. `mock/research/sketches/` is
a directory the panel has never listed.

The sketch is also **wrong about one thing, in the optimistic direction, and it is the thing that
would have set the estimate**. It states: "The `arvo` facade's only live GCE constructs are two static
asserts of the form `[(); 1 / is_fractional(F)]:` (`ufixed.rs:274`, `ifixed.rs:308`)." The measurement
in section 1 says otherwise: those two asserts are 2 spans out of 478, and the other 476 are the width
computation the sketch does not mention. The two citations are correct; the universal "only" is not. A
reader who took that sentence at face value would have priced the facade at two lines, which is
approximately the "afternoon" figure the dispatch names as one of the two wrong answers on offer, and
would have found out during the work.

That is the second time in three stretches that a universal claim about the tree has been made from a
partial read (file 57's "cannot be reproduced by anyone", corrected by file 62). The convention `62b`
adopted for "cannot" claims should extend to "only" claims: **a universal statement about the shipped
tree owes a whole-crate compile before it ships, not a read of the files the author happened to
open.** One `cargo check` with the gate stripped costs four seconds and would have caught this.

---

## 3. What the fix actually is, which is not one change

The dispatch says the shape of the fix is known and not in question: the computed width becomes a
type. That is right as a slogan and it hides a fork that decides the price by a factor of ten. Making
the width a type is available in three different ways, and they are not variations on one migration.
I compiled all three.

**Route Y is available and neither the consolidation nor the sketch names it.** The width does not
have to be *computed* at all. `UFixed`'s first parameter can **be** the total width, with the fraction
point as the second, so no addition ever appears in type position. `probe_6` compiles this: `UFixed<
const W: Width, const F: FBits, S>` with `Bits<Wid<W>, S>`, `W` passed straight through as a
standalone const argument, which the grammar already permits. Only `adt_const_params`, already ALLOWED
at `unstable-features.md:87`. The integer bit count is recovered in value position (`W.0 - F.0`) where
arithmetic has always been free. For `IFixed` the sign bit folds into the declared total and the
`1 +` that pushed `ifixed_bits` into type position **disappears**, which also deletes the `N - 1` in
the public `Int<N>` alias.

**Route Z is the tower's shape.** `I` and `F` become type-level `Nat`s and the width is a type-level
sum. `probe_3` compiles it, zero feature gates, including the addition with its carry case and a
container dispatch driven off the width type.

**Route X, keeping `I` and `F` as consts and lifting only the width to a type, does not exist.**
`probe_2` tries the only shape that could reach it, a const-keyed projection `ToNat<{ ufixed_bits(I,
F) }>::Out`, and is refused six times: the addition is what sits in const position, and wrapping the
result in a type does not move it. `probe_2b` tries the recursive peel and is refused twice over,
once by the const operation and once by `E0119` coherence, which only full `specialization` could
break and that is forbidden at `unstable-features.md:73`. This closes the route rather than leaving it
open, and it is worth having closed because it is the intuitive one.

So the fix is a fork, not a slogan, and the two live branches have different prices, different blast
radii, and different residuals.

---

## 4. The container dispatch, which is the piece under the piece

Both live routes have to answer the same question at `arvo-strategy`: with the width no longer a
const the bucket cannot be a const fn, so where does it come from. Today that is
`container.rs:60-91`, about thirty lines of `if`. The sketch answers with a per-width impl table. The
tower answers structurally. I built both and checked both against the shipped const fn.

**The structural derivation, `probe_4`.** `bucket(W) = clamp(bitlen(W - 1) - 3)`, which is exact
because arvo's boundaries are powers of two. It decomposes into three type-level functions the const
fn got for free: a predecessor (four impls, because the `P = H` case splits), a bit length (four
impls), and a length-to-tag map (nine impls). Thirty impls total, no feature gate. It is checked
against `tag_hot_cold`'s own body by a `const` assertion at every boundary and its neighbours.

**`probe_5` asserts that law over every width 1..=512, not a sample**, because a law asserted over
chosen instantiations is a decision about what not to find out. 512 of 512, clean. The negative
control matters more than the pass: moving one boundary by one in the const fn (`n <= 8` to `n <= 9`)
makes the build fail at `W9` with `E0080, evaluation panicked`, so the law is not vacuous.

**The per-width table, `probe_7` and `probe_8`.** Two impls per width, one per family. It works and it
is trivial to generate. Section 8 prices it, and the price is the finding.

One thing the table shape buys that is easy to miss and worth keeping: when a width is not in the
table, rustc prints its own unprompted "the following other types implement trait `WidthFor<F>`"
listing, the same seal-as-free-diagnostic dividend this review has now found at `Rad<P>` (files 56,
62), at the strategy door's `HostImplemented` (file 59), and at the `Arity` seal (file 64). That is
four independent arrivals at the same dividend and it is starting to look like a general property of
the design rather than a series of happy accidents.

---

## 5. The four predicates, and why they are the real fork

The facade carries four const-fn predicates. Three are trivial under every route. The fourth is the
whole argument, and it is the one the review already cares about for an unrelated reason.

`OneRepresentable` (`arvo-strategy/src/identity.rs:70-91`) withholds `Identity<Multiplicative>` from a
type with zero integer bits. Its doc comment says so in as many words, and it is **the guard the
review installed for the `UFixed<0, F>::ONE` defect that files 30, 33 and 39 spent a stretch on**. It
is not incidental machinery; it is the fix, in shipped form.

Under route Y its condition becomes `W > F`, a comparison of two const-generic parameters, and a
comparison of two consts has no expression under the permitted feature set except a two-dimensional
impl table, which section 8 shows is quadratic in one dimension and would be quadratic in two.

Under route Z it is one line. `probe_9`:

```rust
pub trait OneRepresentable {}
impl<P: Pos> OneRepresentable for Pz<P> {}
// deliberately no `impl OneRepresentable for Z`
```

The negative control refuses with `the trait bound Z: OneRepresentable is not satisfied` plus rustc's
own "the trait is implemented for `Pz<P>`" note. `IntegerLike` and `FractionLike` are one impl each
the same way, and the width is the type-level sum.

**So the honest statement of the fork is that route Y makes 99% of the work mechanical and leaves one
predicate with no expression, and route Z makes every predicate trivial at the cost of type-level
arithmetic that has to be built.** That is a genuine engineering trade and it is where op's decision
actually lies, not in the gate-removal question the workspace rule frames.

---

## 6. `arvo-strategy`: done, on the real crate, and it is a day's work

I performed this one rather than pricing it, because it turned out to be small enough that performing
it was the cheaper way to price it.

`container.rs` rewritten to the bucket-as-type shape with a table to 256, gate removed from
`arvo-strategy/src/lib.rs:11`, nothing else in the workspace touched.

- `cargo check --offline --workspace --all-targets`: **clean**, 19.6s.
- `cargo test --offline --workspace`: **658 passed, 0 failed, 9 ignored**, matching the
  consolidation's own baseline (63:76-79) exactly.
- Clean `cargo check -p arvo-strategy`, three runs each: **3.48 / 3.49 / 3.50s baseline against 3.29 /
  3.48 / 3.44s migrated.** Compile-time neutral at a 256-row table.
- **Zero public signature changes.** `BitsContainerFor<const N: u16, Sign>` keeps its shape;
  `Wid<const N: u16>` and `WidthFor<Family>` are internal and never appear in a signature. `Bits`,
  `UFixed`, `IFixed` and every one of the 34 files across seven crates that mention
  `BitsContainerFor` are untouched.

Artifact: `65_probes/migration/container_migrated.rs`.

**One of the two remaining live gates in this repository comes off for one file's work, with no API
change, no test change, no consumer change, and no compile-time cost.** That is available today,
independently of everything else in this review, and it is by a distance the cheapest compliance win
on the board. It also does not need the numeral tower, does not need op to settle any of the open
identity questions, and cannot be invalidated by whichever way route Y against route Z goes, because
both routes need this piece and both get it in this shape.

The `Project`/`Picker` surface is `pub use`d at `arvo-strategy/src/lib.rs:81`, so the bucket
vocabulary is technically a public change even though nothing outside the crate names it. That is one
`grep`'s worth of confirmation and a line in the src CL, not a design question.

---

## 7. The facade: measured by doing it, 478 down to 8

Route Y, applied to the copy, instrumented. I report the descent rather than a single number, because
the descent is the answer to "how many are mechanical and how many are decisions", and because the
shape of the descent is what a careless estimate gets wrong.

| pass | what it was | errors after |
|---|---|---:|
| start | gate stripped, nothing changed | 478 spans, 246 lines |
| 1 | nine regex rules over eight files (`{ ufixed_bits(I, F) }` to `W`, the parameter lists, `{ frac(F) }` to `F`) | 56 |
| 2 | one more regex (`UFixed<I, F, Hot>` and friends, which pass 1 missed because the strategy was concrete) | 25 |
| 3 | about twenty targeted edits: the value-position `I` references, the `IntegerLike` impls, the four aliases, the two static asserts | 2 |
| 4 | the `OneRepresentable` guard relaxed, **instrumented, not a migration step** (it reinstates the `UFixed<0, F>::ONE` defect on purpose, to see what was behind it) | 103 |
| 5 | a broader regex pass over the whole facade | 102 on **38 lines** |
| 6 | literal call sites and macro bodies rewritten arithmetically; the fraction carrier retyped | 8 |

**Pass 4 is the iceberg, and it is the single thing a line-count estimate misses.** Fixing one class
did not reduce the total, it revealed the next class, because the relaxed bound stopped
short-circuiting impls that had never been type-checked. Anyone who strips the gate, counts 478, and
divides by an edit rate will be wrong about the shape of the work even if they land near the right
total.

**And the good news is bigger than the iceberg.** From pass 4 onward the residual is **one error
class**, `expected u16, found IBits` and `expected u16, found FBits`, repeated. It never branched. 102
errors sat on 38 distinct lines, and 46 of those 102 came from a **single line inside a single macro**
(`from_constant.rs:24`), expanded 46 times. The tail converges; it does not fan out.

The final 8 are `FBitsShim` against `FBits`, an artifact of my own stand-in: the shift carrier is
consumed by `UArith::u_mul_fixed` in `arvo-strategy`, which sits **below** `arvo-storage` where
`FBits` is declared, so the carrier cannot be named there. The real fix is to relocate `FBits` down
one layer, which is the identical move `Width` already made in round 202605031400 under
`arvo-bridge-home-rule.md` ("substrate-bridge traits live in the lowest layer where their return type
is reachable"). Cited precedent, one file, no design question.

Artifact: `65_probes/migration/instrumented_migration.diff`, 20 files, 833 lines added and 711
removed including the generated table. **Read it for the shape and the counts, never as a proposal**;
it carries the relaxed guard and the stand-in carrier and would ship a known defect.

### The mechanical-against-judgement split, which is what was asked for

Counting distinct source lines rather than diagnostics, and separating what a script does from what a
person has to decide:

**Mechanical, roughly 260 edits.** 224 lines of `{ ufixed_bits(I, F) }` and `{ ifixed_bits(I, F) }`
to `W`. 15 value-position `I` references. 24 concrete literal call sites where `ibits(8), fbits(0)`
becomes the arithmetic sum. Four macro-internal forms. Two `IntegerLike` impls. About 24 declaration
sites where the shift carrier is retyped. Every one of these is a regex or a two-line arithmetic
rewrite, and I did most of them in this session by script.

**Judgement, six lines and four decisions.** They are:

1. **Does the fraction count stay a `ConstParamTy` newtype in const-generic position?** If `F` stays
   `FBits`, `FBits` relocates to `arvo-strategy` (precedent above). If it becomes a plain `const F:
   u16`, `no-bare-primitives.md`'s definition-site exception 2 already covers it and the relocation is
   unnecessary. Cheap either way, but it is a call.
2. **What do `Fixed<I, F>` and `Signed<I, F>` become?** Their first parameter's meaning changes from
   integer bits to total width. This is the public API break and the only one. Section 9 counts the
   call sites and there are 21 of them across the whole workspace.
3. **The two `[(); 1 / is_fractional(F)]:` static asserts.** The sketch names `const { assert!(...) }`
   as the replacement; I used a marker trait as a placeholder and did not evaluate either. Small, real,
   needs one round.
4. **`OneRepresentable`.** Section 5. This is the one that is not small, and it is the fork.

**Say which this is: it is nine hundred mechanical edits and four genuine decisions, not four hundred
judgement calls.** The dispatch asks me to distinguish those two objects and the tree is emphatically
the first, which is the better one. What makes it non-trivial is not the edit count, it is that
decision 4 has no cheap answer and decision 2 is a public break.

---

## 8. Where the estimate is dominated by a part nobody examined

The 2026-07-28 sketch's cost section says: "Per-width impls, one row per supported width per family.
The sketch covers the boundaries plus representative interior widths; **the real crate expands its
full range by macro**." That sentence is doing an enormous amount of unexamined work, and it is where
I would put the budget's risk.

I built the table at eight ceilings and timed each twice, wall clock,
`rustc --edition 2024 --crate-type=lib`:

| table ceiling | impls | run 1 | run 2 | ratio per doubling |
|---:|---:|---:|---:|---:|
| 128 | 256 | 0.15s | 0.16s | |
| 256 | 512 | 0.41s | 0.42s | 2.7 |
| 512 | 1024 | 1.42s | 1.40s | 3.4 |
| 1024 | 2048 | 5.37s | 5.31s | 3.8 |
| 2048 | 4096 | 22.70s | 25.18s | 4.5 |
| 4096 | 8192 | 101.68s | 131.16s | 4.9 |
| 8192 | 16384 | exceeded 25 minutes, killed | | |

Quadratic, drifting worse at the top. **Projected** on a 4x-per-doubling fit from the 4096 point:
16384 around 30 minutes, 32768 around 2 hours, 65535 around 8 hours. I did not measure past 4096 and
label those as projections.

Against it, the structural derivation, thirty impls total, timed by the number of distinct widths
**instantiated** rather than tabulated:

| distinct widths used | run 1 | run 2 |
|---:|---:|---:|
| 256 | 0.22s | 0.16s |
| 512 | 0.29s | 0.28s |
| 1024 | 0.52s | 0.46s |
| 2048 | 0.94s | 0.87s |
| 4096 | 1.70s | 1.69s |

Linear, about 0.4ms per distinct width, **and zero for widths nobody uses.**

The two columns do not measure the same variable and that is the whole point. **The table pays for its
ceiling on every build by every consumer forever. The structural form pays only for what is
instantiated.** That is the axis my subject cares about most and it is the axis nobody has costed.

Now the part that makes it a decision rather than a preference. `arvo-toolbox-not-policer.md:60`:

> No bit-width cap below the largest container the substrate is willing to dispatch through. If we
> dispatch up to 256 bits via multi-value containers, the meta-newtypes (`IBits`, `FBits`, `Width`)
> carry that range.

`Width` is `#[repr(transparent)] pub struct Width(pub u16)` (`arvo-strategy/src/width.rs:34`), so the
declared axis is 65535 wide, and `WideBits<BYTES>` has no stated ceiling. The tree's own maximum
instantiated width is 256 (`Bits<256, ...>`, 24 occurrences). So the rule's illustrative figure and
the tree agree at 256, where the table costs 0.42s and is entirely fine, and
`arvo-compile-time-last.md:35` explicitly licenses "4 strategies x 64+ widths x 2 sign = hundreds of
impls" for exactly this trade.

**But a table turns the width ceiling from a type-system fact into a build-time budget, and every
future widening is quadratic.** Today's ceiling is affordable. A consumer asking for `Uint<1024>` in
two years costs five seconds on every build of the workspace; `Uint<4096>` costs two minutes. That is
the "maintenance cost over years" question, and it has an answer that is invisible at the moment of
the decision and unpleasant afterwards. The structural form does not have it.

I am not ruling. I am saying the table's ceiling is a number that has to be **written down and owned**,
with the rate at which widening it costs written down beside it, and that the sketch's "expands its
full range by macro" is not a plan, it is the place the estimate hides.

---

## 9. The estimate

Reasoned from the measurements above. I give it as work rather than as calendar because calendar
depends on how many rounds op wants and how much of the review's own convention (two independent
reads, table-diff obligations, the CL grammar) each piece attracts.

**`arvo-strategy`: one to two days.** One file rewritten, one generated table, one gate removed. I did
the engineering in this session and it is green on the real crate with the real test suite. The
remaining day is the mockspace round, the src CL's `## CHANGE:` blocks, the DESIGN.md.tmpl edit at
`arvo-strategy/DESIGN.md.tmpl:368` which describes `tag_one_representable` and the tag-compression
pattern, the ceiling decision from section 8, and a review pass. **Not an afternoon, and nowhere near
a quarter.**

**The facade: one to three weeks, and the spread is entirely decisions, not edits.** The edits are
about 260, they are script-shaped, and they converge. What fills three weeks is:

- Decision 4 (`OneRepresentable`) needs a design round of its own and, by this review's own two-expert
  convention, two independent reads before it hardens. It is the fork between routes Y and Z and it
  should not be resolved by whoever is holding the keyboard during a mechanical sweep.
- Decision 2 is a public API break and by `no-legacy-shims-pre-1.0.md` there is no transition period,
  so every call site changes in the same commit.
- 71 doc-comment lines name a width, and doc comments compile. They are in the mechanical count but
  they are the part that gets missed, and `documentation-writing.md` puts rustdoc on `pub` items at
  Tier 1 where the doc-vs-source consistency discipline applies.
- Downstream: **28 lines in hilavitkutin** (14 `UFixed<`, 14 `Fixed<`), **zero in vehje**, **zero in
  kolli**. Each consuming repo needs its own round and its own PR.
- The instrumented run says one class of residual repeated. It does not say there is no second
  iceberg behind the relaxed `OneRepresentable` bound, because I relaxed it rather than solving it. A
  real migration finds out whether resolving it properly reveals a third wave. **That is the residual
  risk in this estimate and I want it stated rather than smoothed**: everything up to the guard is
  measured, the guard's own downstream is not.

**Two to four weeks total, for both pieces, one engineer.** The quarter figure is wrong by roughly
four times. The afternoon figure is wrong by roughly forty. Neither of the two costs currently on
offer is close, and they are wrong in opposite directions for the same reason: both were formed
without a whole-crate compile.

### Why the blast radius is so much smaller than the tree suggests

Because consumers write aliases, and three of the four aliases survive the change unchanged. Counted
across the whole workspace:

| what a consumer writes | occurrences in arvo | hilavitkutin | vehje | kolli | changes? |
|---|---:|---:|---:|---:|---|
| `Bits<N, ...>` | 770 lines, 91 files | 81 | 4 | 0 | **no** |
| `Uint<N, S>` | 24 | 38 | 8 | 1 | **no** |
| `Int<N, S>` | 19 | 0 | 5 | 0 | **no**, and the `N - 1` in its definition goes away |
| `Fixed<I, F, S>` / `Signed<I, F, S>` | 7 lines, 3 files | 14 | 0 | 0 | **yes**, first parameter becomes total width |
| `UFixed<...>` / `IFixed<...>` directly | 277 + 221 lines | 14 | 0 | 0 | yes, but almost all inside arvo itself |

`Uint<12>` still reads `Uint<12>` after the migration. That is not a lucky accident, it falls out of
route Y: `Uint<const N: u16, S> = UFixed<N, { fbits(0) }, S>` passes `N` straight through, and
`{ fbits(0u16) }` involves no generic parameter so it was never a refusal site. The consolidation's
"touching every consumer" is true of `UFixed` spelled out and false of what consumers write.

---

## 10. Sequencing

The dispatch asks whether this lands atomically or behind something that lets both shapes coexist.
The measured answer is better than either.

**`arvo-strategy` needs no coexistence mechanism because it has no API change.** The four
`BitsContainerFor` impls are replaced in place; their signature, their `#[diagnostic::on_unimplemented]`
message and their consumers are untouched. It is a single self-contained commit that leaves the
workspace green and the test count identical. It can land this week and it blocks nothing.

**The facade must land atomically**, because `UFixed`'s parameter meaning changes and there is no
shape in which the old and new spellings coexist: two const parameters cannot mean two different
things at once, and `no-legacy-shims-pre-1.0.md` forbids the deprecation alias that would otherwise
bridge it. That is a one-commit, one-PR change touching about 260 lines in arvo plus 28 in
hilavitkutin, with the consuming repos' PRs sequenced immediately after and their `Cargo.toml` git
refs updated per `branch-pr-flow.md`.

So the order is: strategy first, alone, cheap, reversible; then the four decisions, each through its
own round with the two-expert reads the review already demands; then the facade in one commit; then
the consumers. **The two pieces are independent and pretending otherwise is what made the estimate a
factor of forty wide.** The workspace rule's single drift entry, naming both gates on one line
(`unstable-features.md:74`), is what invites the conflation.

---

## 11. What it buys beyond compliance

The dispatch asks whether this is a cost or an investment arriving early. It is both, in different
proportions per piece, and the honest split is:

**`arvo-strategy` is nearly pure dividend.** It removes a forbidden gate, it costs nothing at
runtime, nothing at compile time (measured, section 6), nothing at the API, and nothing to author
against. It also removes the last thing standing between this crate and dropping `adt_const_params`
one day, though not yet, because `Width` and the meta-bit newtypes still use it. And it converts a
silent projection failure into rustc's own exhaustive "the following other types implement" listing,
the diagnostic dividend now found at four independent carriers.

**The facade is genuinely an investment arriving early, and the consolidation is right that the tower
is its target shape.** Three things fall out that this review wants anyway:

- Under route Z, `OneRepresentable`, `IntegerLike` and `FractionLike` stop being const-fn tags with
  hand-maintained impl tables and become one impl each, structurally. The
  `UFixed<0, F>::ONE` guard, which cost this panel a stretch to find, becomes a fact that cannot be
  got wrong rather than a table someone has to keep in sync.
- `IFixed`'s `1 +` disappears from type position entirely, which is the cleanest simplification in
  the whole exercise: the sign bit stops being an arithmetic correction applied at every use and
  becomes part of the declared width. `Int<N>`'s `N - 1` goes with it.
- The type-level `Nat`, `AddN`, and the comparison the predicate needs are the tower's `Nat`,
  `Pos` and `Cmp`. Built here, they are built for the tower. Not built here, the tower builds them
  later and the facade carries a quadratic table in the meantime.

**Against that, the honest cost of the tower shape, which nobody has priced and I did not either.** My
`probe_3` adder handles the carry case and compiles, but it is a demonstration, not `AddN` over the
full binary `Pos` with subtraction and comparison, and it is not attacked. File 61 measured the tower's
own composition costs (63:670-679) at 2.1ms to 143ms per distinct composition and found a cliff. **A
width computed as a type-level sum at every `UFixed` instantiation is a composition, and nobody has
measured what that does to a real consumer's build.** `63:877` already carries "the real-consumer
compile-cost bench" as an untouched open item. If route Z is the answer, that bench stops being
optional and becomes the thing that decides whether the answer is affordable.

---

## What a consolidation could take, close to verbatim

The workspace rule's drift entry (`unstable-features.md:74`) names two gates on one line and prices
them as one thing, and that framing is what produced two estimates two orders of magnitude apart. They
are two independent changes with two different prices and two different blast radii. The
`arvo-strategy` gate comes off with a rewrite of one file (`container.rs`, the bucket becomes a closed
vocabulary of types keyed by an internal `Wid<N>` typestate, the shape a committed sketch established
on 2026-07-28), no public signature change, no consumer edit anywhere, no compile-time cost measured
against baseline at three runs each, and the workspace green at 658 passed, 0 failed, 9 ignored,
identical to the review's own baseline. It is one to two days including the round, it blocks nothing,
and it is available now independently of every open identity question. The facade's gate is the real
work: 478 refusals on 246 lines, of which 93% are one expression, the logical width, and the fix is to
re-parameterise `UFixed` and `IFixed` on total width and fraction point so nothing is ever computed in
type position. That is roughly 260 mechanical edits and exactly four decisions, and the descent was
measured by performing it, 478 to 56 to 25 to 2, then 103 when a relaxed guard revealed a second wave,
then 102 on 38 lines, then 8, with the residual staying one error class throughout rather than fanning
out. It is one to three weeks, the spread being decisions rather than edits, and the largest is that
the `OneRepresentable` guard, which is the shipped fix for the `UFixed<0, F>::ONE` defect, becomes a
comparison of two const parameters with no expression under the permitted features, whereas under the
tower's shape it is one impl. Consumers barely notice either way: `Bits<N>`, `Uint<N>` and `Int<N>` are
spelled exactly as they are today, only `Fixed<I, F>` and `Signed<I, F>` change meaning, and that is 7
lines in arvo and 14 in hilavitkutin, with vehje and kolli at zero. The estimate's remaining risk is
the per-width impl table's ceiling, which is quadratic and measured at 0.42s for 256 widths, 5.3s for
1024, 116s for 4096 and over 25 minutes at 8192, paid on every build by every consumer for the
ceiling's whole range whether instantiated or not, against a structural derivation at thirty impls
costing 0.4ms per width actually used and nothing for widths nobody names. Today's ceiling of 256 is
affordable and licensed by `arvo-compile-time-last.md:35`; the number and the rate at which widening it
costs both have to be written down and owned, because `Width` is sixteen bits wide and
`arvo-toolbox-not-policer.md:60` forbids a cap below what the substrate dispatches. Finally, a
convention: the sketch that established the migration shape also states that the facade's only live
refusals are two static asserts, which the whole-crate compile refutes at 2 out of 478, so the
convention `62b` adopted for universal "cannot" claims should extend to universal "only" claims about
the shipped tree, and the cost of honouring it is one `cargo check` with the gate stripped, four
seconds.

*grounded on: `pin`, `host`, `flags`; `65_probes/probe_1` through `probe_9` and
`65_probes/migration/` (all compiled or measured, this file); `65_probes/timings.csv` and
`timings_structural.csv` (wall clock, two runs each, this machine, so machine-shaped in absolute terms
and structural in the ratios); `tree` (`arvo-strategy/src/lib.rs:11`, `container.rs:60-91,114,254-280`,
`width.rs:34`, `identity.rs:70-91`, `arvo-storage/src/bits.rs:57-61`, `meta_bits.rs:284-299`,
`arvo/src/lib.rs:25`, `ufixed.rs:35-38,100,265,274`, `ifixed.rs:301,308`, `aliases.rs:45,57,71,85`,
`fixed_scale.rs:16-28`, `traits/from_constant.rs:24`, all read fresh this session);
`mock/research/sketches/202607282100_container-projection-without-gce/FINDINGS.md` (read fresh, its
"WORKS, zero gates" result confirmed by my own whole-crate run, its "only two static asserts" sentence
refuted by the same run); ratified rules (`unstable-features.md:74,87,132`,
`arvo-toolbox-not-policer.md:60`, `arvo-compile-time-last.md:35`, `arvo-bridge-home-rule.md`,
`no-legacy-shims-pre-1.0.md`, `no-bare-primitives.md` exception 2); reasoned, not compiled, for
sections 9 and 10 and for the projections past 4096 in section 8, each marked where it occurs.*
