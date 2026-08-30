# P2: does a partial interior give a binding-free definedness channel at every profile

```sh
for da in off on; do rustc --edition 2024 -C opt-level=3 -C debug-assertions=$da -o s_$da partial3.rs; done
./s_off; ./s_on          # output in partial3.out
```

`171` tested six channels and named four untested. **Partiality was in neither list**, and `172`'s
definedness bound found it. This is my independent check of that bound and of my own gap.

## Two constructions refuted before one worked, both runs kept

**v1** (`partial_v1_REFUTED.out`): both realisations used the same divisor, so both were undefined on
the same 37 of 256 inputs. **0 splits, at both profiles.** Widening a carrier does not by itself move
which inputs are undefined.

**v2** (`partial2_v2_REFUTED.out`): I "widened" the divisor as `(x*37-60) & 0xFF` in `u32`, which is
**equal by construction** to the `u8` wrapping form. 0 splits again. A widening that is a no-op modulo
the container is not a widening.

## What works, and it is a real design case rather than a toy

An **algebraic simplification of the interior**: `(x*x)/x` against `x`. Both agree on every input where
the first is defined, and the second is additionally defined at `x = 0`.

At both `debug-assertions = off` and `on`, inputs 0..4096:

| pair | both defined | both undefined | **split** | value disagreements |
|---|---|---|---|---|
| `(x*x)/x` against `x` | 4095 | 0 | **1** | 0 |
| C-D v1, shared divisor | 3510 | 586 | 0 | 0 |
| C-D v2, no-op widening | 4080 | 16 | 0 | 0 |
| C-B no partial operation | 4096 | 0 | 0 | 0 |

All five controls clean, identical at both profiles. **C-E is the sharp one: a value-only equivalence
check that skips undefined inputs CERTIFIES the pair**, which is exactly the check clause 1 refuses.

**The witness is one input in 4096**, which is why a random search finds nothing and why `172` reports
constructing its witness after 200,000 random inputs found zero occurrences of a `2^-32` event. On
`u32` the split here is exactly `x = 0`, a `2^-32` event, which is the same figure from a different
construction.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, u32 and u8
containers, inputs 0..4096, opt-level = 3, debug-assertions in {on, off}, threads = 1`
