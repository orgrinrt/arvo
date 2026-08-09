# 89_probes: outcomes, exact commands, and the numbers file 89 cites

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, every
command run from inside the repository tree (outside it the same command resolves to stable
`1.94.0`, which changes what compiles). Nothing under `mock/crates/` was touched. The bench
harness was **not** run: its orchestrator overwrites every committed CSV in `mock/benches`
(`81:38-44`), and nothing here is a timing claim. Every number below is either an exhaustive
count from a program that asserts it, or an instruction count read out of an object file with
`objdump -d`.

## probe_1_two_counts_on_the_fold.rs

```
rustc --edition 2021 -O probe_1_two_counts_on_the_fold.rs --out-dir out
./out/probe_1_two_counts_on_the_fold
```

**WORKS.** Output:

```
composition                              | finest view, reading B | finest view, reading A
-----------------------------------------|------------------------|-----------------------
Hot, unsigned wrapping                   | Some((Exact, Exact))   | Some((Exact, Exact))
Hot, signed wrapping                     | Some((Exact, Ignore))  | Some((Exact, Exact))
Warm / Cold, saturating                  | None                   | None
Precise, refusing                        | Some((Ignore, Exact))  | Some((Ignore, Exact))
Refuse at one end, reduce at the other   | Some((Ignore, Ignore)) | Some((Ignore, Exact))

CLAIM A holds: reading B reproduces file 37's table, all five rows.
CLAIM B holds: under reading A every surviving composition has event level Exact.
           and the site count is exactly 3 in every grouping of every input, all compositions.
CLAIM C holds: incomparable pairs under B = 1, under A = 0.
CLAIM D holds: witness (-4,-3,3) delivers -4 both ways; firings 2 vs 0, sites 2 vs 2.
```

Exhaustive: every input tuple of every composition (4096 signed and 4096 unsigned four-element
tuples, times five groupings, times nine views, plus the 512-tuple three-element sweep). No
sampling anywhere. The join-closure property (`37:136-143`) is asserted at the computed finest
view rather than assumed.

## probe_2_division_under_both_counts.rs

```
rustc --edition 2021 -O probe_2_division_under_both_counts.rs --out-dir out
./out/probe_2_division_under_both_counts
```

**WORKS.** Output:

```
CLAIM A holds: 240 defined pairs, values agree on all of them; 16 undefined, all at b = 0.
CLAIM B holds: under reading A the round-trip law's finest view is (Ignore, Ignore), the weak-equation corner.
CLAIM C holds: under reading B it is (Ignore, Exact), which is Precise's own point.
CLAIM D holds: general division moves the value on 144/240 defined pairs; reading A charges all 240.
CLAIM E holds: over 240 constant-divisor cells, reading A charges 240 events and reading B charges 144.
           and x / 1 == x, exactly, charges 16 events under reading A and 0 under reading B.
```

CLAIMS A and B reproduce `43_probes/probe_5`'s three claims through an independently written
construction. CLAIM D is the separation check the model owes itself: the divider moves the value
on a strict majority of general pairs, so reading B is not vacuously zero on this model.

## probe_3_binding_time_and_price.rs and probe_3b

```
rustc --edition 2021 -O --crate-type rlib probe_3_binding_time_and_price.rs --emit=obj -o out/probe3.o
objdump -d out/probe3.o
rustc --edition 2021 --crate-type lib probe_3b_reading_b_cannot_be_the_published_type.rs --out-dir out
```

**probe_3: WORKS.** The const-position assertions (`_A_ADD_64`, `_A_MUL_64`, `_A_DIV_64`,
`_A_ROUNDTRIP`) all hold with no value in scope. Instruction counts, 64-element `i64`
fixed-point fold, `FRAC = 4`:

| fold | emitted instructions | steady state per element | unrolled |
|---|---:|---:|---|
| `fold_plain`, no grade | 160 | 2.5 | fully, 64x |
| `fold_grade_reading_a` | 161 | 2.5 | fully, 64x |
| `fold_grade_reading_b`, counting | 32 | 5.0 | 4x |
| `fold_grade_reading_b_presence` | 34 | 5.0 | 4x |

Reading A's entire cost is one instruction outside the loop, `mov w1, #0x40`, at `0x27c`.
Reading B adds `tst`/`cinc` per element (counting) or `and`/`orr` per element (presence), and
the two cost the same: the price is the test, not the accumulation, so the presence level is
not the cheaper one on this shape. Neither fold vectorises, because the accumulator is a serial
dependency in all four; reading B's added work is off the critical path, so an instruction count
overstates its latency cost and a bench is owed before any throughput claim.

**probe_3b: FAILS WITH E0435**, as intended:

```
error[E0435]: attempt to use a non-constant value in a constant
  --> probe_3b_reading_b_cannot_be_the_published_type.rs:57:14
   |
57 |     Folded::<ev>(acc)
   |              ^^ non-constant value
```

The positive control in the same file (`fold_reading_a -> Folded<64>`) compiles, so the refusal
is about reading B and not about the shape of `Folded`.

## probe_4_the_float_door_and_the_flag.rs

```
rustc --edition 2021 -O probe_4_the_float_door_and_the_flag.rs --out-dir out
./out/probe_4_the_float_door_and_the_flag
rustc --edition 2021 -O --crate-type rlib probe_4_the_float_door_and_the_flag.rs --emit=obj -o out/probe4.o
objdump -d out/probe4.o
```

**WORKS.** Output:

```
FPSR before = 0x0, after one inexact add = 0x10, sum = 1.3333333333333333
CLAIM A holds: core::arch::asm! reads FPSR on the pin, no feature gate, and IXC is set.
CLAIM B holds: sticky, clearable, and an exact add (4) leaves it clear.
plain = 4.7438909037057675
reading A: value 4.7438909037057675, count 64   (a literal; the count is the same for every input)
reading B via flag:      value 4.7438909037057675, count 58
reading B via recompute: value 4.7438909037057675, count 58
```

Instruction counts, 64-element `f64` fold:

| fold | emitted instructions | steady state per element | unrolled |
|---|---:|---:|---|
| `ffold_plain` | 98 | 1.5 | fully, 64x |
| `ffold_reading_a` | 100 | 1.5 | fully, 64x |
| `ffold_reading_b_via_flag` | 16 | 11 | not at all |
| `ffold_reading_b_via_recompute` | 18 | 13 | not at all |

The flag route emits `mrs x9, FPSR` and `msr FPSR, x9` inside the loop body and the loop does
not unroll. The recompute route emits Knuth 2Sum (four `fsub`, one `fadd`) plus `fcmp`/`cinc`
per element and also does not unroll.

Re-run of file 50's own grep, fresh, inside the tree: `grep -rl
"fetestexcept\|feclearexcept\|fesetround\|fegetround"` over `library/core` and `library/std` of
the pinned `rust-src` returns nothing, and `fpcr|FPCR|fpsr|FPSR` over `library/core/src/arch*`
returns nothing. Both halves of `50:319-327`'s weaker claim reproduce; its stronger sentence
(`50:319`, "the standard's carrier is not available to us in any case") does not, because
inline asm reaches it.

## probe_5_division_failure_and_the_far_point.rs

```
rustc --edition 2021 -O probe_5_division_failure_and_the_far_point.rs --out-dir out
./out/probe_5_division_failure_and_the_far_point
```

**WORKS.** Output:

```
CLAIM A holds: x/0 with x nonzero resolves to the signed far point at all four Specials members.
CLAIM B holds: 0/0 has no far point at any of them, so IEEE's divideByZero-against-invalid
               split is the presence or absence of a supremum, derived rather than copied.
CLAIM C holds: under alternative 1 exactly one cell has no answer, Hot fixed-point,
               because ReduceModulo has no residue of an unbounded exact result.

CLAIM D, the derived third-position row (alternative 2):
  Hot     : the cheapest defined value the target gives away.
            on aarch64 the integer divide of 7 by 0 yields 0 with no trap.
  Warm    : the nearest defined value, which is the far point in the dividend's sign direction.
  Cold    : as Warm; 'between warm and precise' does not distinguish them here.
  Precise : refuses.
```

## Offline recomputation of file 43's accumulator table

```
python3 -c "import math; ..."   # lcm(1..2^p-1).bit_length() + (2^p-1).bit_length()
```

| p | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|
| this reconstruction | 5 | 12 | 23 | **52** | 95 | 190 | 370 |
| `43:145-148` | 5 | 12 | 23 | **51** | 95 | 190 | 370 |

Six of seven cells reproduce exactly; p = 5 differs by one bit, which is an off-by-one in how
the value-range term is taken rather than a disagreement about the finding. The growth law
reproduces exactly: `K * log2(e)` at `p = 16` gives 94,547 bits against file 43's "on the order
of 94,500" (`43:153`). The finding is unaffected, and file 43's own cross-check against Python's
`math.lcm` is the better authority for the individual cells.

## Gates run for file 89

Canon gate, from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` exits 1, empty; the same with `FullRange\|UTerm\|AddWidth` exits 1, empty.

Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary by parsing every
`test result:` line across 149 binaries: **666 passed, 0 failed, 9 ignored**, matching files 81
through 84.
