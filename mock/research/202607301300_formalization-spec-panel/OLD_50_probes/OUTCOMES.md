# Probe outcomes, file 50

All probes built and run against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, confirmed with `rustc -vV` from inside the repo. Every emitted-code and
instruction claim is that target's. No probe contains a timer; the two measurements that would need one
are named as owed benches in the file's section 7.

`vu_nat_sealed_adj.rs` and `vu_bias_sealed_adj.rs` are byte copies of `46_probes/`'s files of the same
name, unmodified: the sealed tower as file 46 left it. `probe_3` composes with them rather than
declaring its own copy.

`model.rs` is shared by probes 1, 2, 4 and 6 via `#[path = "model.rs"] mod model;`. It is a model binary
float format plus a round-first quantiser over exact rationals, and it knows nothing about IEEE beyond
the format parameters: the whole point is that if it reproduces binary32 then the design's settled
quantiser plus one grid-selection step is sufficient.

Build and run:

```
rustc --edition 2021 -O probe_1_model_vs_silicon.rs      -o /tmp/p1  && /tmp/p1
rustc --edition 2021 -O probe_2_band.rs                  -o /tmp/p2  && /tmp/p2
rustc --edition 2021 -O probe_2b_band_closed_form.rs     -o /tmp/p2b && /tmp/p2b
rustc --edition 2021 --crate-type lib probe_3_exponent_as_type.rs
rustc --edition 2021 --crate-type lib probe_3b_exponent_as_const_refused.rs   # expected to FAIL
rustc --edition 2021 -O probe_4_accumulator.rs           -o /tmp/p4  && /tmp/p4
rustc --edition 2021 -O probe_5_execution_environment.rs -o /tmp/p5  && /tmp/p5
rustc --edition 2021 -O probe_6_specials.rs              -o /tmp/p6  && /tmp/p6
```

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_model_vs_silicon.rs` | Does round-first over a magnitude-selected grid reproduce what the FPU delivers, bit for bit? | **WORKS.** 9,380,157 adds, 16,008,001 multiplies, 15,992,001 divides against binary32, **0 mismatches**. 1,255 overflows to infinity, 884 subnormal results, 2,090 overflow-band cases. Add pairs are filtered to an exponent spread of at most 90 because the model computes in `u128` and an exact binary32 sum spans up to 277 bits; multiply and divide sweep the full cross product unfiltered. |
| (the bug it found) | The first rounding kernel aligned the quantum by shifting the denominator left. | **FAILS, silently.** For the product of two binary32 subnormals the scale is -298 and `u128 << 149` masks the shift to 21, delivering a nonzero subnormal for a product 130 binades below the smallest representable value. The hardware disagreeing is what surfaced it; nothing in the source looked wrong. Repaired by exact long division plus a below-half-a-quantum short circuit (`model.rs:198-262`). |
| `probe_2_band.rs` part 1 | Is the overflow band inhabited for float operations, the member struck from the sentence for having no derivation? | **INHABITED, exhaustive at three model widths.** add 48 / 144 / 384, mul 0 / 20 / 48, div 0 / 0 / 0. The control row is the argument: a one-binade format (a fixed-point numeral in a float's clothes) reports **add 0**, reproducing the consolidation's compiled same-format fixed-point result with the same code. First witness at p=4: `0.015625 + 30`, exact 30.015625, max finite 30. On silicon: `f32::MAX + 1.0 == f32::MAX`. |
| `probe_2_band.rs` part 2 | Is the band's emptiness decided by `q_result <= 2 * lattice`? | **FAILS.** 753/1000 for addition, 639/1000 for multiplication, disagreeing in both directions. Kept as the record of the refutation rather than rewritten. Two failure modes: the criterion assumed `max_r` sits on the lattice (true for every dyadic case the review had compiled, false in general), and it assumed exact results fill their lattice (false for products: `15.25 = 61/4` needs an index product of 61). |
| `probe_2b_band_closed_form.rs` | The corrected two-clause form, against exhaustive enumeration over 5,184 quantum triples. | **WORKS as a one-sided certificate.** Addition 5,006/5,184 with **0 under-predictions**; multiplication 4,057/5,184 with 1,127 over-predictions and **0 under-predictions**. The lattice clause never claims empty when the band is inhabited, so it is sound for the direction a build layer would act on. Residual over-predictions are all reachability. |
| `probe_3_exponent_as_type.rs` | The signed exponent as a type, sealed at birth, and `mulnum` over two `Ranged` numerals. | **WORKS, no unstable feature at all.** `EZero \| EPos<P> \| ENeg<P>` over the sealed `Pos`, nine constructor-headed `ESum` impls, every claim const-asserted and forced through a signature. `M1(p=4, e in [-3,4]) * M2(p=3, e in [-2,3]) -> p=7, e in [-5,7]`; `binary32 * binary32 -> p=48, e in [-252,254]`. |
| (the repair it forced) | The first negative-plus-positive impl reused the difference helper with swapped arguments. | **FAILS, `the trait bound Z: Dec is not satisfied`.** `Cmp<7,4> = Gt` selects the branch computing `4 - 7`, walking the natural subtraction off the bottom of `Nat`. The refusal is the tower working. Repaired with a three-impl `NegE`: compute the magnitude difference once, apply the sign after, which is the separation `Bias` already makes. |
| `probe_3b_exponent_as_const_refused.rs` | The same `mulnum` with the exponent bounds as const parameters. | **FAILS, and every permitted route is closed.** Plain: `error: generic parameters may not be used in const operations ... add #![feature(generic_const_exprs)]` (forbidden). Under `min_generic_const_args`: `complex const arguments must be placed inside of a const block`. As a `const { }` block under that feature: `generic parameters may not be used in const operations ... add #![feature(generic_const_args)]`, which needs `-Znext-solver=globally`. |
| `probe_4_accumulator.rs` | Does the fold's accumulator sufficiency condition survive a moving exponent, and at what width? | **WORKS, finite, and the width formula is tight.** 2,924,207 ordered triples exactly representable, widest magnitude 13 bits against a formula predicting 13; at n=8 worst case, 14 against 14. 139,721 orderings (rotations and reversals of 4- to 8-tuples) all agree. The same folds with the accumulator held in the format: 2,052,336 inexact interior quantisations and **23.17% of triples deliver a different result under left- against right-association**. Real formats: binary32 sum 277 bits, dot product 554; binary64 sum 2,098, dot product 4,196, each plus `ceil(log2 n)`. |
| `probe_5_execution_environment.rs` | Does the machine honour a rounding direction and an underflow policy that a type declares? | **NO, measured by writing FPCR through inline assembly.** Entry FPCR is `0x0` (nearest-even, FZ off). `1.0/3.0` gives `0x3eaaaaab` at entry and `0x3eaaaaaa` under RMode=toward-zero or toward-negative. `MIN_POSITIVE * 0.5` gives `0x00400000` at entry and `0x00000000` with FZ=1. **Const-folded against runtime disagree** on both (`0x3eaaaaab` vs `0x3eaaaaaa`, `0x00400000` vs `0x00000000`). `mul_add` differs from multiply-then-add on real inputs (`5.9604638e-8` against `0`). |
| (emitted code, same target) | What the compiler actually emits. | `f_add: fadd s0, s0, s1` (no rounding-mode operand). `f_fma: fmadd s0, s0, s1, s2`. A float sum loop: 5 scalar `fadd s`, **0 vector fadds**. The identical integer loop: **8 vector adds**. LLVM refuses to reassociate a float reduction and reassociates an integer one freely. |
| (rust-src grep, same pin) | Can a Rust program read IEEE's flags or the rounding mode? | **NO.** Zero occurrences of `fetestexcept`, `feclearexcept`, `fegetround` or `fesetround` in `library/core/src` or `library/std/src`. No FPCR access in `core::arch::aarch64`. `_mm_setcsr` is deprecated since 1.75.0, note: "use inline assembly instead". |
| `probe_6_specials.rs` | Does the design's class-level specials table agree with the machine, and does the cause split compile? | **WORKS, 300 cases, 0 mismatches**, over every combination of `{+0,-0,+1,-1,+3,+inf,-inf,qNaN,qNaN',sNaN}` under add, multiply, divide. The cause split holds: `x/0` finite nonzero raises divideByZero only and delivers a signed infinity; `0/0` and `inf/inf` raise invalid and deliver a quiet NaN; `inf/0` raises nothing. |
| (the three repairs it forced) | The first table had 12 mismatches, all mine. | Zero-product sign is the xor of operand signs, not the matched operand's. Division's `(Inf, Zero)` case was missing and fell through to the finite arm. And **finite plus finite is not decidable at the class level**: exact cancellation delivers a zero, a different class, so the specials table cannot be a total function from classes to classes. `1.0 + (-1.0)` is the counterexample. |
| (NaN payloads, same target) | Which payload survives, and is it commutative? | `qNaN(1) + qNaN(2) -> 0x7fc00001`, `qNaN(2) + qNaN(1) -> 0x7fc00002`. **Addition is commutative at the value level and not at the datum level.** `sNaN(1) + 1.0 -> 0x7fc00001` (quieted, payload preserved). `0.0/0.0` and `inf - inf` give the default NaN `0x7fc00000`. Constant folding does not preserve payloads: const-folded `qNaN(1) + 1.0` is `0x7fc00000` where the runtime gives `0x7fc00001`, and this one holds even at the default FPCR. |

## Verbatim error heads

`probe_3b_exponent_as_const_refused.rs`, plain:

```
error: generic parameters may not be used in const operations
  --> probe_3b_exponent_as_const_refused.rs:26:21
   |
26 |     type Out = Fl<{ P1 + P2 }, { E1N + E2N }, { E1X + E2X }>;
   |                     ^^ cannot perform const operation using `P1`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `P1`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Same file with `#![feature(min_generic_const_args)]`:

```
error: complex const arguments must be placed inside of a `const` block
```

Same, written as `const { P1 + P2 }` under that feature:

```
error: generic parameters may not be used in const operations
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

The first draft of `probe_3`'s negative-plus-positive exponent sum:

```
error[E0277]: the trait bound `Z: Dec` is not satisfied
    |
258 | const _: () = assert!(<<ENeg<P7> as ESum<EPos<P4>>>::Out as Exponent>::VAL == -3);
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
note: required for `Gt` to implement `SignedDiff<O<O<H>>, I<I<H>>>`
```

## What a shipped suite would take from here

Three compile-fail pairs and one property suite, none of them built as tests in this dispatch:

- `probe_3` as the positive control and `probe_3b` as the negative, pinning that the exponent is a type
  and that the const form is refused. The negative's expected stderr is toolchain-sensitive (the
  diagnostic differs by feature gate), so it wants the `tests/ui` shape the repo already uses at
  `mock/crates/arvo/tests/ui/`, not a hand-compared string.
- The overflow-band table of `probe_2` part 1, as a whole-matrix property over every model format in a
  swept range rather than the three widths run here. The one-binade control row is the one that must not
  be dropped, because it is what isolates the cause to the exponent.
- `probe_6`'s 300-case class table, whole-matrix, against the model rather than against `f32`, so it runs
  on a target without an FPU.
- `probe_4`'s width formula as a const assertion at every format the design ships, since it is arithmetic
  on the format parameters and costs nothing to check.
