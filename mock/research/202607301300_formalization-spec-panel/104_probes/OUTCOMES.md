# 104 probe outcomes

All at `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, edition 2024,
resolved from the repo's own `rust-toolchain.toml`. Every probe except `p1` compiles with **no
feature gates at all**, which is a result rather than an accident and is recorded per probe. Commands
are recorded verbatim.

One environment note, checked rather than assumed, because file 61 warned about it: a `rustc`
invoked outside the repo tree resolves to stable and rejects the whole numeral stack (`E0554`). The
scratch crate `p1` builds from therefore carries its own `rust-toolchain.toml` pinning
`nightly-2026-05-28`, verified by `rustc --version` from inside that directory.

## p1: the shipped `bitfield!`'s placement map

Compiled against the shipped facade from a scratch crate outside the repo, because `mock/crates` is
outside the panel's scope and this needed the real macro rather than a model. Manifest:

```toml
[package]
name = "bfprobe"
version = "0.0.0"
edition = "2024"

[dependencies]
arvo = { path = ".../arvo/mock/crates/arvo" }
arvo-bits = { path = ".../arvo/mock/crates/arvo-bits" }

[workspace]
```

plus a `rust-toolchain.toml` copying the repo's pin. `cargo build --offline`, `cargo test --offline
-- --nocapture`.

**A. Overlapping fields compile, and the overlap silently aliases.** `Overlap: 16 { a: 8 at 0, b: 8
at 4 }`. Both fields satisfy the macro's own containment assertion (`$lo + $field_bits <= $n`), so
nothing refuses. Setting `a` to `0xFF` and then `b` to `0x00`:

```
container = 0b0000000000001111, a = 0xf, b = 0x0
a still 0xFF? false
```

`a` reads back `0x0F`. No `unsafe`, no warning, no diagnostic of any kind. **WORKS** in the sense
that it compiles and runs; the finding is that it does.

**B. A field running past the container refuses at declaration, with no construction and no use.**
`OutOfRange: 16 { wide: 8 at 12 }`, declared in a crate that constructs nothing:

```
error[E0080]: evaluation panicked: sub-range wide does not fit within N bits
              evaluation of `OutOfRange::_BOUNDS` failed here
```

**REFUSES**, which is the correct shape. `p4c` establishes why it fires and what it depends on.

## p1b: the missing half, discharged

A model macro carrying the same `at OFFSET` grammar, with containment **and** pairwise disjointness
asserted in a free anonymous const beside the type. No feature gates.

```
rustc --edition 2024 -O -o /tmp/p1b p1b_disjointness.rs
```

**WORKS** on the disjoint declaration (`StrHandle: 32`, origin/reserved/id), printing the fields and
the occupancy mask. With the overlapping declaration uncommented:

```
error[E0080]: evaluation panicked: two fields overlap
```

**REFUSES.** The check is `O(k^2)` over the declared list at const-eval time, on a list whose length
is the number of fields a human wrote.

## p2: does a `place` map compose

```
rustc --edition 2024 -C opt-level=3 -o /tmp/p2 p2_composition.rs
rustc --edition 2024 -C opt-level=3 -C panic=abort --emit asm -o p2.s p2_composition.rs
```

4096 elements at stride 13, fields `(0,3) (3,5) (8,5)`, every element carrying a distinct 13-bit
value so the element space is swept by construction rather than sampled.

```
round-trip mismatches (two-step vs packed input): 0
composition mismatches (one-step vs two-step):    0
period P = 8/gcd(WS,8) = 8, group bytes G = WS*P/8 = 13
  field 0 (o=0, w=3): lane shifts over the period [0, 5, 2, 7, 4, 1, 6, 3]
  field 1 (o=3, w=5): lane shifts over the period [3, 0, 5, 2, 7, 4, 1, 6]
  field 2 (o=8, w=5): lane shifts over the period [0, 5, 2, 7, 4, 1, 6, 3]
```

**WORKS.** The lane-shift rows are the same sequence rotated, because `(j*W_S + o) mod 8` is field
0's sequence offset by `o`: one decode plan per column, reused per field with a constant addend.

Instruction counts, `sum_two_step` 23, `sum_one_step` 24, i.e. the composed form is one instruction
*worse* in this shape, which refuted the prediction that motivated the probe. `p2b` isolates why.

## p2b: binding time, and why the intermediate mask disappears

```
rustc --edition 2024 -C opt-level=3 -C panic=abort --emit asm -o p2b.s p2b_binding_time.rs
```

| body | instructions |
|---|---:|
| `const_two` (placement as literals, element then field) | 23 |
| `const_one` (placement as literals, composed offset) | 24 |
| `dyn_two` (placement as runtime arguments) | 23 |
| `dyn_one` (placement as runtime arguments) | 24 |

All four return the same value. **Two readings, both honest.** The intermediate element mask costs
nothing because `((x >> s) & M_S) >> o & M_w` equals `(x >> (s+o)) & M_w` exactly when
`o + w <= W_S`, which is the containment obligation; LLVM performs the collapse itself. And the
const-versus-runtime columns do not separate here at all, because this loop is not unrolled by the
period, so nothing was waiting on a literal. `p3` states the plan and the columns separate.

## p3: the plan stated, and which composite to emit

```
rustc --edition 2024 -C opt-level=3 -o /tmp/p3 p3_plan_stated.rs
rustc --edition 2024 -C opt-level=3 -C panic=abort --emit asm -o p3.s p3_plan_stated.rs
```

512 groups of P = 8 elements, G = 13 bytes, bodies unrolled by the period so every shift is a
literal, loads unchecked so the comparison prices the work rather than the bounds checks.

```
one-field  two-step = 63488, one-step = 63488, agree = true
all-fields two-step = 141312, one-step = 141312, agree = true
```

| body | instructions per group |
|---|---:|
| `one_field_two_step` | 38 |
| `one_field_one_step` | 38 |
| `all_fields_two_step` | 399 |
| `all_fields_one_step` | 467 |

**WORKS**, and neither composite dominates: reading one field is identical either way, reading all
three favours the two-step by 17 percent because the element load amortises. These are static counts
over differently-unrolled bodies and are **not** a runtime claim; a runtime claim belongs in the
bench harness.

## p4: can a declarative macro derive a bitfield's placement map

```
rustc --edition 2024 -O -o /tmp/p4 p4_declarative.rs
```

**NO FEATURE GATES.** A `macro_rules!` muncher accumulating the prefix sum as an unevaluated token
sequence (`0 $($acc)*`), emitting one getter per field, over two heterogeneous declarations:

```
r = 0b11111111111
g = 0b10101010101
b = 0b1010101010
OCCUPANCY = 32, mask = 0b11111111111111111111111111111111
enable = 1, reserved = 0b1010, divisor = 0b101010101
Reg OCCUPANCY = 14, mask = 0b0011111111111111
16-bit exhaustive rebuild mismatches: 0
```

**WORKS.** The 16-bit case is rebuilt from its fields at every one of the 65,536 container values,
both directions, not sampled. The contrast with file 61 is the point: the notation vehicle needed a
proc-macro because a decimal literal is one atomic token, and a bitfield's widths arrive as separate
tokens with nothing to decompose.

**One correction made mid-probe and recorded rather than smoothed over.** The first version derived
the occupancy mask by `(!0) >> (BITS - OCCUPANCY)`, which on an over-committed declaration overflows
*before* the stated assertion is reached, giving `attempt to compute 32_u32 - 33_u32` instead of the
message the declaration owes its author. The mask is total in the committed version. A
declaration-site refusal has to be the first const-evaluable failure or the reader gets the second
one.

## p4b: the derived form's one obligation

```
rustc --edition 2024 --crate-type lib -o /tmp/p4b.rlib p4b_declarative_refusal.rs
```

`TooWide: u32, 32 { r: 11, g: 11, b: 11 }`, never constructed anywhere:

```
error[E0080]: evaluation panicked: declared fields exceed the container
              evaluation of `TooWide::_FITS` failed here
```

**REFUSES.** With derived offsets, containment is the only obligation left; disjointness is not
checkable because it is not expressible.

## p4c: where a declaration-site refusal has to live to be one

```
rustc --edition 2024 --crate-type lib -o /tmp/p4c.rlib p4c_refusal_locus.rs      # A alone
rustc --edition 2024 --crate-type lib -o /tmp/p4cb.rlib  (B uncommented)
rustc --edition 2024 --crate-type lib -o /tmp/p4cc.rlib  (C uncommented)
```

Same assertion, three placements, against a violating declaration that is never constructed:

| placement | result |
|---|---|
| A. associated const in the inherent impl, mentioned by nothing | **compiles, silent** |
| B. associated const mentioned by a `const fn` in the same impl | `error[E0080]: B: fields exceed container` |
| C. free anonymous const item beside the type | `error[E0080]: C: fields exceed container` |

This is why `p1`'s case B refuses: the shipped macro is shape B, by two `let _ = Self::_BOUNDS;`
lines at `arvo/src/bitfield.rs:393` and `:399` mentioning the const declared at `:377`. The check is
real and it hangs on those two lines rather than on the declaration.

## p5: what a datum-keyed digest masks to when the map has a hole

```
rustc --edition 2024 -C opt-level=3 -o /tmp/p5 p5_occupancy_mask.rs
rustc --edition 2024 -C opt-level=3 -C panic=abort --emit asm -o p5.s p5_occupancy_mask.rs
```

`Reg: 16` with `enable` at 0 (1 bit) and `divisor` at 5 (9 bits); bits 1..5 are an interior hole and
bits 14..16 are container padding.

```
W_F = 14, prefix mask = 0b0011111111111111
       union  mask = 0b0011111111100001
hole  = prefix & !union = 0b0000000000011110
prefix mask: separates 65536 equal pairs, conflates 0 unequal pairs
union  mask: separates 0 equal pairs, conflates 0 unequal pairs
```

Exhaustive over all 65,536 container values against three perturbations each (the hole bits, the
container padding, a real field bit). **The prefix mask separates 65,536 pairs that agree at every
declared field.** The union mask separates none and conflates none.

Emitted cost, aarch64:

```
digest_prefix:  and w0, w0, #0x3fff      ; ret      -> 1 instruction
digest_union:   mov w8, #16353 ; and w0, w0, w8 ; ret -> 2 instructions
```

One extra instruction, and only because `0x3FE1` is not an ARM logical immediate; where the union is
encodable the general form is free.

## p6: the granularity ground, priced

```
rustc --edition 2024 --crate-type proc-macro -o /tmp/p6pm.dylib p6_pm_crate.rs
rustc --edition 2024 --crate-type lib --emit=metadata -o /tmp/p6d.rmeta p6_decl_crate.rs
rustc --edition 2024 --crate-type lib -o /tmp/p6d.rlib p6_decl_crate.rs
```

Min of three, wall clock, this host:

| crate kind | ms |
|---|---:|
| proc-macro crate, full build (host dylib), sysroot `proc_macro` only | **187.1** |
| ordinary lib crate exporting a `macro_rules!`, metadata only | 68.3 |
| ordinary lib crate, full build | 66.4 |

Both source files are trivial, so the difference is the crate kind rather than the contents:
**about 121 ms is the floor price of reaching a proc-macro crate at all**, paid once per build of
the dependency graph. The marginal cost of a second macro living in the same proc-macro crate is a
few hundred lines of host-side Rust, well under that floor.

## p7: the group arithmetic keys on the stride, the mask keys on the field

```
rustc --edition 2024 -C opt-level=2 -o /tmp/p7 p7_stride_and_width.rs
```

Every stride from 1 to 57 (file 81's own range) crossed with every two-field partition of it, 64
elements packed and read both ways at each shape:

```
shapes checked (stride 1..=57 x every two-field partition): 1596
two-step vs one-step vs packed-input disagreements: 0
G*8 == stride*P failures: 0
period/group rows moving with the field split: 0
```

**WORKS**, exhaustive over the swept range rather than sampled. The period and the group byte count
are invariant under every repartition of the same stride; the mask width and the load width move with
the field. At stride 13 the element's own load width is 2 bytes and so is each field's, but at stride
57 the element needs an 8-byte load where a 3-bit field needs 2, which is the one place the composed
read is strictly cheaper rather than merely equal.
