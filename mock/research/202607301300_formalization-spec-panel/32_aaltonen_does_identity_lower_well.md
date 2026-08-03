# 32. Does identity lower well

**Member:** Sebastian Aaltonen. Basement lens: data-oriented layout, cache and bandwidth behaviour,
and what happens when a structure meets a real column of millions of elements rather than one value in
a probe. The question is never "does it compile," it is "what is the memory access pattern, and is the
hardware happy." A design that is elegant and costs one branch per element on a bitpacked column of
ten million rows is not elegant; it is a ten-million-branch design wearing a trait bound.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, identical to file 31's own gate result four files ago, so nothing regressed underneath this
dispatch. I did not re-run file 31's `identity_laws.rs` / ui-test audit; that surface is unchanged by
anything below and file 31 already checked it in detail (`31:8-19`). Canon gate: the governing calls
are the D-numbered ones in `202607301200_topic.the-formalization-spec.md`, subordinate to op's seventh
checkpoint (`30b_op_checkpoint_seven.md`, D69 overturned, D39 held) and to file 31's settled identity
contract (`31` section 4), which is my primary input per the brief. Nothing below argues against a
D-numbered call or against `30b`; where I extend the settled contract, I say so as an addition, not a
reopening.

**What I read:** `26_consolidation_two.md` in full (this stretch's consolidated shape and the lead
designer's calls). `30b_op_checkpoint_seven.md` in full. `31_arntzen_settling_the_identity_contract.md`
in full, section 4 as the primary input per the brief. `202607301200_topic.the-formalization-spec.md`
sections on `Lowering` (lines 38-186) and the crate table (line 297), to confirm the shape
`StoredWidth`/`Widening`/`StorageLayout` carry before file 31 nested `Encoding` inside it, read
directly rather than trusted from file 31's citations. Directory listed once (`ls`, 32 files plus
probe directories); no other panel file fetched, per the brief's instruction that files 27-30 are for
a specific derivation only, and section 4 supplied everything I needed. Two workspace rules named in
the brief: `arvo-always-optimal-internals.md` and `arvo-toolbox-not-policer.md`, both read in full
before writing.

**What I compiled and measured, separated from what is reasoned.** Every claim under headings 1
through 6 below is a compiled or measured fact: a `const` assertion the compiler either accepted or
refused, a disassembled function body, or a timed sweep. Nothing in those sections is inferred from
reading the design and imagining what it would do. Heading 7 states what follows from those facts for
the settled contract, and is reasoning built on measurement, marked as such. The artifacts are
`32_probes/identity_model/` (a standalone crate, `[workspace]`-isolated so `cargo init` could not
splice it into the real `mock/Cargo.toml`, modelling the settled contract from `31:326-359`
faithfully enough to compile and disassemble) and `32_probes/gen_identity_sweep.py` (the compile-time
sweep generator, mirroring `08_probes/i_gen_monomorphisation_sweep.py`'s exact methodology for direct
comparability). Full reproduction commands and a summary table are in `32_probes/OUTCOMES.md`. All of
it ran against `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the pin file 31 also gates against.

## 0. The verdict, stated first

Yes, with one open item. The settled identity contract (Radix, Precision, `ExponentForm` nesting
`Adjustment`/`Bias`/`Underflow`/`Specials`, `Domain` on `Numeral`; `SignIndexing`/`FieldLayout`/
`Canonicalisation` nested inside `Lowering` as `Encoding`) lowers to exactly the bytes and exactly the
instructions the underlying arithmetic needs, with two measured exceptions that are not free and one
open question about whether the design needs to say anything about them. Every axis is a zero-sized
type, confirmed by the compiler refusing to build otherwise. `Specials` costs nothing on a composition
that does not declare it, and this is not a runtime-eliminated branch, it is a branch with no source
text to write, which is a stronger guarantee than DCE and worth stating in those terms in the spec.
Canonicalisation costs nothing when the datum has no cohort to canonicalise (every fixed-point
composition shipped today) and a small, branchless, measured amount when it does. The one open item is
not about the identity contract at all: whether the shape a consumer writes vectorises turned out to
be a fragile, compiler-context-sensitive question on this toolchain, independent of anything the
identity contract adds, and that finding is itself useful, because it says the design should not lean
on autovectorisation as a guarantee for anything, which is exactly what `arvo-always-optimal-internals.md`
already says for unrelated reasons.

## 1. What a distinct identity-side composition costs to compile

**Measured.** `26_consolidation_two.md:399-407` already has this number for the policy and delivery
side: about 5.2 milliseconds of `monomorphization_collector_graph_walk` time per distinct composition,
linear in the count, zero symbols in the shipped binary. Nothing equivalent existed for the identity
side, per the brief. Section 3 of `26` even names this gap explicitly (`26:668-674`): "Compile-time and
monomorphisation cost of the newly proposed mechanisms is asserted, not measured, in several places...
None of these belong in this document as claims; they belong in `mock/benches/`."

I ran the same methodology `08_probes/i_gen_monomorphisation_sweep.py` used, against a model of the
settled identity contract carrying four identity axes beyond width (Radix, Precision, the nested
`ExponentForm`, `Domain`) and three encoding axes nested inside `Lowering` (`SignIndexing`,
`FieldLayout`, `Canonicalisation`), swept across width, domain, sign indexing, and Hot-versus-Cold
lowering, calling `add` through the full stack for each of K distinct compositions:

| distinct compositions | `monomorphization_collector_graph_walk` | total build | symbols (`nm -U`) |
|---|---|---|---|
| 1 | 0.012s | 2.282s | 557 |
| 10 | 0.001s | 2.127s | 557 |
| 40 | 0.002s | 2.137s | 557 |
| 100 | 0.003s | 2.184s | 557 |
| 200 | 0.004s | 2.210s | 557 |
| 400 | 0.008s | 2.307s | 557 |

Symbol count is flat across the full 400x range, matching file 08's own finding on the policy side:
every instantiation inlines away completely and the identity axes leave nothing in the binary either.
The collector-walk time is not clearly linear in this range and stays under ten milliseconds at K=400,
against file 08's clearly linear curve reaching roughly 2.1 seconds at the same K on the policy side.

**Scoped, so this number is not misread against file 08's.** File 08's `add` call went through the
full union: the recovery-map witness, the structural classification, the graded carrier, the derived
law table. Mine goes through the identity/encoding axis nesting and one arithmetic op with a
canonicalisation call. The two numbers measure different things by design and should not be read as
"identity is 500 times cheaper than policy." What they do establish, honestly: the marginal compile
cost the consolidation's own droplist worried about attaches to the verification and law-derivation
apparatus (already measured, already priced), not to the raw count of identity axes. Four more identity
axes and three more encoding axes, as pure associated-type nesting with no derived proof machinery
riding on them, is not where arvo's compile time is going to go, in the same sense file 08 concluded
for the per-width table (`08:385-386`). This closes the specific gap `26:668-674` named for the
identity side; the gap for a real consumer's full composition set, crossed with the verification
apparatus, is a different, larger question that neither this file nor file 08 answers and that stays
open.

## 2. What `Specials` costs the path that never has one

**Measured, and the strongest form of the claim available.** File 31 states the structural fact:
`Implicit<const E, A, B>` carries no `Specials` parameter at all, because "a constant exponent has no
bottom to fall off" (`202607301200:98-99`, applied by file 27 and adopted by file 30 and 31). I built
the settled quantiser pipeline's classify step (`31:378-384`, "round on the unbounded-exponent
extension of the grid... classify the rounded result against the range and resolve") as a real
`ExponentForm` trait method, one impl for `Implicit`, one for `Ranged`, and disassembled both
monomorphisations.

`Implicit`'s classify (`32_probes/identity_model/src/lib.rs` `probe_classify_implicit`), the common
fixed-point path:

```
cmp  x0, x1
csel x8, x0, x1, gt
cmp  x0, x2
csel x0, x2, x8, gt
ret
```

`Ranged`'s classify with `Specials = WithInfNaN` (`probe_classify_ranged`):

```
mov  x8, #-0x8000000000000000
cmp  x0, x1
csel x8, x8, x0, lt
mov  x9, #0x7fffffffffffffff
cmp  x0, x2
csel x0, x9, x8, gt
ret
```

Both are branchless (`csel`, not a conditional branch: no misprediction risk either way). The float
side costs one extra instruction (6 against 5) to materialise the two sentinel constants. That is the
whole cost of `Specials`, measured, and it is proportional to what specials actually delivers
(infinity sentinels instead of a clamped finite value), never a runtime flag read on a shared body.
There is no third instruction sequence where the `Implicit` path pays for a specials check it never
takes: the two functions are separately compiled from separately-shaped source, because `Implicit`
genuinely has nothing to name. This is stronger than "the branch got eliminated by the optimiser,"
which is a claim about what LLVM happened to do this build. It is a claim about what the type permits
you to write, checked once at design time rather than re-verified on every compiler upgrade.

**What I would add to the settled contract, stated as design text rather than as a finding:**
`ExponentForm`'s two constructors are not two configurations of one classify function; they are two
classify functions, and a composition's `Specials` cost is exactly the cost of the arm its own
`ExponentForm` instance can express, never a shared arm skipped at runtime. Worth one sentence in
section 4.1 of the next consolidation, next to the `CARRIES_SPECIALS`-shaped fact file 31 already
implies but does not name outright.

## 3. What a canonicalisation obligation costs between a load and an arithmetic op

**Measured.** File 31's crossing contract (section 4.3, `31:368-374`) states `encode . decode`
idempotent as canonicalisation, and section 2 of the same file (`31:232-279`) derives
`DatumDeterministic` from paying that canonicalisation "at every step that could deliver a
non-canonical datum." I built `add` exactly that way: `wrapping_add`, then
`<S::Encoding as Encoding>::Canonical::canonicalize`, and disassembled the monomorphisation for a
fixed-point composition (`Canonical = IdentityCanon`, the case file 31 names as free: "every
composition with `Specials = None`... is datum-deterministic for free," `31:407-408`) against a
float-shaped one (`Canonical = NaNCanon`, a real NaN-band collapse to one canonical representative).

Fixed-point, `Canonical = IdentityCanon`:

```
add x0, x1, x0
ret
```

Two instructions, identical to a bare `i64::wrapping_add`. `IdentityCanon::canonicalize` did not
inline to a no-op that the optimiser then removed; there was never a second instruction to remove.
`DatumDeterministic`'s free case, verified in the object file rather than argued from the type shape.

Float-shaped, `Canonical = NaNCanon`:

```
mov  x8, #-0x7ff0000000000001
add  x9, x1, x0
add  x8, x9, x8
mov  x10, #0x7ffffffffffff
mov  x11, #0x7ff8000000000000
cmp  x8, x10
csel x0, x11, x9, lo
ret
```

Seven instructions, five more than the identity case, still fully branchless: LLVM turned the
two-sided NaN-band range check into a single offset-and-unsigned-compare, the same trick it used for
the `Specials` classify above. No branch misprediction risk, no defeat of instruction-level
parallelism, in this model.

**Scoped honestly.** My `NaNCanon` collapses any bit pattern in a marker range to one fixed canonical
representative, the simplest canonicalisation policy that exists. A real IEEE-shaped canonicalisation
that has to inspect and possibly propagate a NaN's payload, or a decimal cohort's preferred exponent
that depends on more than a range test, is a different and possibly not-branchless shape, and I have
not measured it. What is measured, and can go in the spec as measured: canonicalisation is a per-op
function call resolved at monomorphisation, its cost for the common (no-cohort, `Specials = None`)
case is exactly zero, and its cost for a simple range-based cohort collapse is a small, fixed,
branchless constant. What is not measured, and should not be asserted: that every plausible
`Canonicalisation` impl stays branchless. That is the next thing to check if a richer
`Canonicalisation` instance (real NaN payload rules, decimal preferred-exponent selection) is ever
proposed.

## 4. What the datum-versus-value split costs a packed column

**Measured, and this is the question that matters most given `arvo-toolbox-not-policer.md`'s own
framing: bitpacked storage is not the exotic case, it is the reason arvo exists, because its primary
consumers run millions of entities in contiguous columns where every saved bit compounds.** I built a
column of `Q(13,3)`-shaped fields (16 bits, four per 64-bit word, no padding, the same non-power-of-two
shape arvo's own bitfield examples use) and extracted through `Encoding::Fields`
(`FieldLayout::HIDDEN_BIT`, `::ENCODING_BIAS`, both read as monomorphised constants, not runtime
fields) against a hand-written shift-and-mask baseline with no trait in the way.

The two disassembled bodies (`probe_bitpacked_column_sum`, `probe_bitpacked_column_sum_raw_baseline`)
are byte-identical: fourteen instructions each, the same opcodes at the same relative offsets,
differing only in the branch-target labels a linker assigns to two different symbols. The `Encoding`
trait's field-layout projection costs nothing over hand-rolled extraction for the trivial
(`PlainFields`, `HIDDEN_BIT = false`, `ENCODING_BIAS = 0`) case, which is every fixed-point composition
shipped today. This is the direct answer to the brief's question about what the datum-versus-value
distinction costs a packed column: for the identity fact that is actually in play on Cold storage
today (no hidden bit, no encoding bias, no reserved codes), it costs nothing, verified.

**What this probe does not cover, stated so it is not overclaimed.** My packing was deliberately clean:
sixteen bits, four fields per sixty-four-bit word, no field crosses a word boundary. Real `Bits<N, S>`
packing for an arbitrary `N` (thirteen bits with no rounding to sixteen, the actual common case for a
tightly bitpacked column) needs cross-word extraction for some fraction of the fields in a run, which
is a genuinely harder shift-and-mask shape (two loads, two shifts, an or). I did not measure that case.
The question this probe answers is narrower and still load-bearing: does the `Encoding::Fields` trait
indirection itself cost anything over hand-rolled bit-twiddling, holding the packing scheme fixed. It
does not. Whether the packing scheme the trait sits on top of is itself optimal for arbitrary N is a
`Layout`/`arvo-bitmask` question, not an `Encoding` question, and is out of this dispatch's scope.

## 5. Whether the type-level numeral machinery leaves anything at runtime

**Measured, by the compiler refusing to build otherwise, which is the strongest form this claim can
take.** Nine `const` assertions at the bottom of `identity_model/src/lib.rs` check `size_of`/`align_of`
of the fully assembled `Number<N, S>` on both the fixed-point side (`Implicit` exponent, `Domain =
Symmetric`, `Encoding = FixEncoding<TwosComplement>`) and the float-shaped side (`Ranged` exponent,
`Specials = WithInfNaN`, `Encoding = FloatEncoding<1023>`, `Canonical = NaNCanon`), plus every
individual axis marker type in isolation: `Two`, `P<13>`, `Implicit<-5, Unit, ZeroBias>`,
`Ranged<-100, 100, Gradual, WithInfNaN>`, `FixEncoding<TwosComplement>`, `FloatEncoding<1023>`,
`IdentityCanon`, `NaNCanon`. The crate compiles. Every marker is zero-sized; `Number<N, S>` is exactly
the width of its raw payload on both sides, `Ranged` and `Specials` included. There is no shadow field,
no discriminant, no vtable pointer anywhere in the type. A future reader who wants to re-check this
after a design change does not need to trust this file; they need to run the same nine assertions,
which is the honest form file 20's downstream-contract dive already established for exactly this
shape (`26_consolidation_two.md` section 1.6, "a working precedent... already ships in arvo today,"
citing `arvo-storage/src/layout_assertions.rs`, which pins declared axis instances against actual
discriminants the identical way).

## 6. Whether the shape a consumer writes is the shape that vectorises

**Measured, and the result is not what I expected going in, which is exactly the finding worth
reporting rather than the one I would have preferred to report.** A minimal, standalone control (a
two-function crate: `for i in 0..a.len() { out[i] = a[i].wrapping_add(b[i]); }` plus a panic handler,
nothing else) autovectorises cleanly on this toolchain and target at `-C opt-level=3`: real NEON,
`ldp q0,q1` / `add.2d` / `stp q0,q1` processing four `i64` lanes per unrolled iteration, with the
compiler-inserted runtime disjointness check between `out` and `a`/`b` that a safe vectorisation of
aliasable slices needs.

The identical scalar shape, reached through the full identity contract (`add_inlinable::<FixNumeral<...>,
HotLowering<...>>`, confirmed via `nm` to have inlined away with no residual symbol, confirmed via
disassembly to reduce to the identical two-instruction scalar body from section 3 above), did **not**
vectorise, in the same crate, under the identical build flags. I did not stop at that result, because
it would have been the wrong finding to hand back: I ablated the generic dispatch entirely (a bare,
non-generic function with the identical loop body, no identity contract involved) and it also did not
vectorise in this crate. I then embedded a byte-for-byte copy of the vectorising standalone control
inside this crate, alongside everything else, and it stopped vectorising too, under the same flags
that vectorised it standalone. That isolates the effect to something about this crate's build context
(size, function count, `#![feature(const_trait_impl)]`'s interaction with the codegen path, or
something in the LLVM pass pipeline's cost model I did not identify) rather than to anything the
identity contract or the encoding nesting contributes. I could not, inside this dispatch's scope, pin
the exact mechanism; `-C remark=loop-vectorize` produced no remarks I could read through the codegen
paths I tried, and chasing it further is `rustc`/LLVM pass-pipeline archaeology, not identity-contract
design review.

**What this is worth to the design, stated as reasoning built on the measurement, not as another
measurement.** It would be a mistake to read this as "the identity contract defeats vectorisation":
the control test proves the opposite, that the exact scalar shape the contract lowers to is one LLVM's
vectoriser can and does handle, on this toolchain, in isolation. What it actually shows is that
relying on the optimiser's loop vectoriser to fire is fragile in a way that has nothing to do with
identity, policy, or lowering axes and everything to do with how much else is in the compilation unit,
which pass-manager heuristics happen to trigger, and which nightly you are on. That is precisely the
premise `arvo-always-optimal-internals.md` already states for the substrate's actual hot paths: cfg
gated `core::arch` intrinsics and hand-written microkernels, never a hope that the autovectoriser sees
through a loop. This measurement does not weaken that rule; it is independent evidence for it, from a
different angle than the rule's own stated reasoning ("compiler version evolution, MIR shape changes,
and LLVM autovec heuristic drift are all real failure modes"). The identity contract's actual job
toward vectorisation, given that framing, is not "compile to something the autovectoriser likes." It
is "leave the layout flat enough that a hand-written kernel can treat an array of `Number<N, S>` as a
raw array of the container width with zero per-element overhead from the wrapper," which is exactly
what section 5 above measured and confirmed.

## 7. What this means for the settled contract, stated as design

Four small, concrete additions the next consolidation could take close to verbatim, each following
directly from a measurement above rather than from a new argument:

**To section 4.1 (`Numeral`).** `ExponentForm`'s two constructors do not share a classify body gated
on a runtime `Specials` check. Each constructor licenses its own classify implementation, and
`Implicit`'s has no arm a `Specials`-carrying composition would need, because there is no `Specials`
type in scope to write one against. A composition's specials cost is the cost of the arm its own
`ExponentForm` instance can express; it is never the cost of an arm a shared body skips. Measured:
`Implicit` classify is 5 branchless instructions, `Ranged`/`WithInfNaN` classify is 6, both
`csel`-based, no branch.

**To section 4.2 (`Encoding` nested inside `Lowering`).** The datum-versus-value split costs nothing
on the path that has one datum per value. Measured on both halves of the split: a trivial
`Canonicalisation` (identity, every `Specials = None` composition) adds zero instructions to an
operation body; a trivial `FieldLayout` (`PlainFields`, no hidden bit, no encoding bias) produces
byte-identical code to a hand-rolled shift-and-mask extractor on a packed column. The richness the
crossing contract added (section 4.3, the section-retraction triple) is paid exactly where the design
says it is paid: at the operations that can produce a non-canonical datum, never as ambient overhead on
compositions that structurally cannot.

**A new, short subsection, "Erasure," stated once rather than left implicit across every axis
description.** `Number<N, S>` is exactly the width of its raw payload for every composition, identity
and encoding axes included. This is checked by nine `const` assertions in
`32_probes/identity_model/src/lib.rs`, following the same pattern
`arvo-storage/src/layout_assertions.rs` already ships (`26_consolidation_two.md` section 1.6): the
crate refusing to build is the check, and any future axis addition should add its own assertion to the
same file the way that file already asks for `Lowering` size/alignment claims specifically.

**A caution against a claim the design should not make**, drawn from section 6 above. The identity
contract's obligation toward a hardware-friendly consumer is that its layout stays flat and its
per-element operations stay inlinable, both measured and confirmed above. It is not, and should not
become, an obligation that any particular build's optimiser autovectorises the resulting loop. That
guarantee does not exist on this toolchain for even the plainest possible scalar shape once embedded
in a large-enough compilation unit, independent of arvo entirely. Where the design wants a vectorised
column operation, the existing standing rule already names the mechanism (`arvo-always-optimal-internals.md`:
cfg-gated `core::arch` intrinsics, hand-written per-architecture kernels, bench-validated), and nothing
above changes that; if anything, it is now measured evidence for why that rule is the correct posture
rather than an assertion.

## 8. What remains open

Four items, none of them closed by anything above, stated so a later member does not have to
rediscover the gap.

1. **Cross-word-boundary bitpacked extraction is unmeasured.** Section 4 confirms the `Encoding`
   trait costs nothing for a clean, no-boundary-crossing packing (`Q(13,3)` rounded to 16 bits, four
   per word). The harder case, an arbitrary `N` whose fields genuinely straddle word boundaries in a
   tightly packed run (the actual worst case for Cold storage), was not built or disassembled.

2. **A richer `Canonicalisation` (real NaN payload rules, a decimal preferred-exponent selector more
   complex than a range test) may not stay branchless.** Section 3's measurement is honest about
   covering only the simplest possible policy. Whether a payload-preserving canonicaliser keeps the
   `csel`-only shape or forces a real branch is the next thing to check before the spec asserts
   canonicalisation is unconditionally branchless.

3. **The compile-time sweep in section 1 measures pure axis-shape cost, not the verification
   apparatus's cost against a full consumer composition set.** `26_consolidation_two.md:668-674`'s
   larger gap, what the atomic-fact ladder and the const-fn-key discipline cost as the product of a
   real consumer's composition set, is unaddressed by this file and remains exactly as open as the
   consolidation left it.

4. **The vectorisation fragility in section 6 has an unidentified root cause.** I isolated it to a
   crate-level build-context effect and ruled out the identity contract as the cause, but did not pin
   which specific thing about the larger compilation unit defeats the loop vectoriser. Whoever next
   wants a real answer should diff `--emit=llvm-ir` before and after the loop-vectorize pass between
   the standalone control and the same function embedded in a larger crate; I did not have a clean way
   to get LLVM's own pass remarks out of this build pipeline within this dispatch's scope.

## 9. Standing

Nothing here overturns a D-numbered call, op's seventh checkpoint, or anything file 31 settled.
Sections 1 through 6 are new measurements answering a question no prior file in this stretch asked:
whether the identity contract's shape, not just its meaning, is one the machine is happy with. Section
7 states four small, concrete additions to the settled contract, each following directly from a
measurement rather than from a new argument, in the form the next consolidation could take close to
verbatim, which is what op's checkpoint asked every member from here to produce. Section 8 names what
those measurements did not reach, honestly bounded rather than extrapolated past what was actually
run.
