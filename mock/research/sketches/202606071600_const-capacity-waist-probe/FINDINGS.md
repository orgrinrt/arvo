# FINDINGS: const-capacity-waist-probe (GATE-2 chart R0 / sketch S1)

**Hypothesis.** A `const fn` generic over a `Capacity`-style GAT trait can construct + index `C::Array<T>` in const context (given an ADDITIVE const surface: const `filled` + const index get/set, replacing the non-const `AsRef`/`AsMut`), AND call `[const]` bit-contract methods on a row type in the same const fn body, on the pinned nightly (nightly-2026-05-28). This is U1+U2 from `hilavitkutin/mock/research/202606071500_gate2-corrected-dispatch-chart.md`, the load-bearing feasibility gate for the additive const `waist_detect_const`.

**Outcome: WORKS.**

```
const pred_counts = [0, 1, 2, 1]
const pred_counts N=8 = [0, 1, 2, 1, 1, 0, 2, 1]
R0 PROBE: WORKS
```

Both `COUNTS` (N=4) and `C8` (N=8) are computed in `const` items (forced const evaluation), so the generic const fn const-evaluates at distinct `N` with no per-N GCE blowup, no ICE, no trait-solver explosion.

**What it proves, concretely.**

1. **U2 (the genuinely unproven piece): const construct + index of a `C::Array<T>` GAT through a const trait.** A `const trait ConstCap { type Array<T: Copy>: Copy; const N: usize; fn filled; fn get; fn set; }` with `impl<const N: usize> const ConstCap for Dim<N> { type Array<T> = [T; N]; ... }` compiles and is callable from a `const fn` bounded `C: [const] ConstCap`. The const inherent `get`/`set`/`filled` replace the non-const std `AsRef`/`AsMut` path that blocks the real `Capacity`. This is the minimal additive surface R1a must add to arvo-tensor `Capacity`.

2. **U1: `[const]` trait method calls inside a const fn body.** A `const trait Row` (mirroring arvo `BitAccess`/`BitSequence`) with `impl const Row for W` is callable in a const fn (`R::trailing_zeros`, `R::with_bit_cleared`, `R::is_zero`), including the iterator-free set-bit scan (`while !is_zero { tz; with_bit_cleared }`) that replaces `iter_set_bits`. (Already largely implied by arvo's shipped `clear_lowest_set_bit` default method, now confirmed standalone.)

3. **Syntax note (cost a compile):** the pinned nightly uses the `const trait Foo` KEYWORD form, NOT the `#[const_trait]` attribute (which errors "trait is not const"). Bounds use `T: [const] Trait`; impls use `impl const Trait`. Matches arvo's `pub const trait` surface.

**What it does NOT prove (deferred to later sketches per the chart).**

- It prototypes the const surface locally; it does NOT add it to the real arvo `Capacity` (that is R1a) nor port the real `waist_detect` (R1b). The mechanism is proven; the real-type wiring is the round work.
- It does NOT build a `BitMatrix` in const context from access masks (that is R2-pre / sketch S2, a separate unknown).
- The probe computes predecessor counts as a stand-in; the real waist algorithm (depth → level widths → occupied → strict-local-minima) is more passes, but they use the same const moves proven here (array build + index + scalar compares), so they carry no new toolchain risk.

**Unblocks:** R1a (additive const `Capacity` surface in arvo-tensor) and R1b (additive const `waist_detect_const` in arvo-graph). The GATE-2 chart's foundational gate is cleared.
