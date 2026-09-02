# Probe H: is the third carrier buildable at all, gate-free

The doability question for section 6's third carrier, a description of the region that lowers at the
observation point. `RULES.md` requires that a canon establish a thing is doable before it states the
intent; this is that check and nothing more.

```sh
# Build 1 is the claim: the machinery compiles under #![no_std] with no feature gate.
rustc --edition 2024 -O --crate-type=lib -o libexpr_nostd.rlib lib_nostd.rs

# Build 2 is the identical machinery under std so the checks can print.
rustc --edition 2024 -O -o run_std run_std.rs && ./run_std
```

`expr.rs` is the machinery, `include!`d by both. Output in `staged.out`, build log in `build_nostd.txt`.
Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the committed pin.

## Result

```
NC20  forward width computed from the description alone, no sink: 33
NC19  work width at demand 8 = 8, at demand 63 = 33, moved = true
NC18a congruence-only chain: narrow 221 vs wide-then-mask 221, agree = true
NC18b with a blocking Shr node: narrow 103 vs wide-then-mask 103, agree = true
NC18c a lowering that WRONGLY passes the demand through the shift gives 7, correct is 103, differ = true
PASSES_DEMAND: Add true, Mul true, Shr false
size_of description = 24 bytes, no vtable, no allocation
```

**Gate-free and constraint-clean, verified rather than asserted:**

| checked | count | required |
|---|---|---|
| `#![feature(...)]` attributes across all three files | **0** | 0 |
| `dyn` | 0 | 0 |
| `TypeId`, `core::any` | 0 (the only textual hits are in the forbidden-list comment) | 0 |
| `Box<`, `Vec<`, `alloc::` | 0 | 0 |
| `#![no_std]` build exit status | 0 | 0 |

`size_of` is 24 bytes for a three-leaf description, which is exactly the three `i64` values: the operator
structure costs nothing at run time.

## The controls, and what each would have caught

**NC20** the forward width is an associated const computed from the children's, evaluated with no
observation at all. Had it needed the sink, the description would not have been a description.

**NC19** the same description observed at two demands works at two widths, 8 and 33. Had the width not
moved, the consumer's demand would not be reaching the intermediate and the whole point would be absent.

**NC18a** on a congruence-only description the narrowed evaluation equals the wide one masked, matching
probe B's licensed set.

**NC18b and NC18c together are the real control.** `Shr` refuses to pass the demand, and NC18c builds the
lowering that wrongly passes it: **7 against the correct 103.** So refusing matters, and the
`PASSES_DEMAND` flag is load-bearing rather than decorative. Without NC18c, NC18b's agreement would be
consistent with a description that ignored the flag entirely.

## What this establishes and what it does not

**Establishes:** a description carrying the operator structure, computing a forward width with no
observation, and lowering under the consumer's demand at the observation point, is buildable under I14
and I15 with no forbidden feature, no allocation and no dynamic dispatch, on the committed pin.

**Does not establish:** anything about cost. Whether this lowers to the same machine code as the direct
form is **unpriced**, and pricing it needs the bench harness. Nor anything about ergonomics, which
section 6.1 says is op's call. Nor that this shape is the right one: a probe's spelling is scaffolding
and not a design.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, the four node kinds in
expr.rs, demands in {8, 63}, threads = 1`
