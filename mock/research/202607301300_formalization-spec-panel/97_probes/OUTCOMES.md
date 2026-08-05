# 97_probes outcomes

All builds on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, resolved from `rust-toolchain.toml`
inside the tree, host `aarch64-apple-darwin`. Commands verbatim; every number below is a compile
outcome, an executed program's stdout, or an `objdump`/asm read. No bench was run. The workspace
tree carried a concurrent dispatch's uncommitted `mock/benches` + manifest changes throughout; no
probe here touches either.

## probe_1_foreclosed_region.rs (run)

```
rustc --edition 2021 -O probe_1_foreclosed_region.rs -o out/probe_1
./out/probe_1
```

```
declarable pairing: CARD=65535, inhabitants=65535
inhabitants stored through the niche-typed door: 65535
orphaned stores (decode has no answer): 0
door is domain-preserving with no door-side equation: true
```

Full sweep of the inhabitant set, no sampling. With the entry-level totality refusal (the shape
ratified at `95b:115-118`) in the model, the only declarable value-unique pairing has a total
decode, and file 92's niche-typed door is domain-preserving with no door-side equation anywhere.

## probe_1b_pairing_refused.rs (compile-fail, expected)

```
rustc --edition 2021 probe_1b_pairing_refused.rs -o out/probe_1b
```

```
error[E0080]: evaluation panicked: value-unique decode partial over the carrier's inhabitants:
this pairing is refused at declaration
  --> probe_1b_pairing_refused.rs:20:31
...
note: the above error was encountered while instantiating `fn Bounded::<8192>::embed`
```

File 95's probe_1 case B (`Bounded<8192>` over `NonZeroU16`, value-unique), refused at the FIRST
construction, before any door exists. The refusal fires at `embed`, not at `typed_mut`: the same
const equation as `95_probes/probe_1b`, relocated to the ratified site, gates every construction
path. The door-side copy's guarded region is therefore empty.

## probe_2_receipt_verdict_split.rs (asm + run)

```
rustc --edition 2021 --target aarch64-apple-darwin --crate-type=lib -O --emit=asm \
  -o out/probe_2_aarch64.s probe_2_receipt_verdict_split.rs
rustc --edition 2021 -O --cfg run_it probe_2_receipt_verdict_split.rs -o out/probe_2
./out/probe_2
```

Emitted `receipt_checked` body: 4 instructions (`mrs`, `mov w9 #29884416`, `tst`, `cset`),
identical to `94_probes/probe_5`'s derived form. Run output:

```
CHECKED_MASK = 0x1c80000
UNCHECKED_IDS = 0b1000 (bit 3 = precision control)
boolean-shape receipt would report: true; split shape reports: (checked ok, 1 field unchecked)
```

The unchecked residue (`UNCHECKED_IDS`) is const-derived from the same field set as the mask, is
assertable in const position (two `const _` assertions in the file hold), and costs zero
instructions in the emitted receipt. No feature gates.

## probe_3_fallback_slot_domain.rs (run)

```
rustc --edition 2021 -O probe_3_fallback_slot_domain.rs -o out/probe_3
./out/probe_3
```

```
clamp preset, 5/0: shadowing=7, completing=9223372036854775807, agree=false
hot preset,   5/0: shadowing=7, completing=7, agree=true
clamp preset, 0/0: shadowing=-1, completing=-1, agree=true
collapsed slots at hot: dir-cell=0, ind-cell=0
```

The two readings of the adopted arity-two fallback (slots always apply, against slots fill only
holes) diverge at a preset whose OverRange row answers the directional cell, and coincide at Hot,
the preset the fork spent its files on.

## probe_4_rmode_locus.rs (run + asm)

```
rustc --edition 2021 -O probe_4_rmode_locus.rs -o out/probe_4
./out/probe_4
rustc --edition 2021 --crate-type=lib -O --emit=asm -o out/probe_4_aarch64.s probe_4_rmode_locus.rs
```

```
fadd 1.0 + 1e-300 under RN: == 1.0 ? true
fadd 1.0 + 1e-300 under RP: >  1.0 ? true
fadd moved with the ambient state: true
cast 2.7 / -2.7 under RN: 2 / -2; under RP: 2 / -2
cast moved with the ambient state: false
fpcr restored: true
```

Emitted `cast_trunc` body: `fcvtzs x0, d0; ret`. On one target, one register write: the addition's
result moves with FPCR.RMode and the cast's does not, because `fcvtzs` encodes its rounding in the
opcode. Caveat as stated in the probe header: perturbing FPCR is outside LLVM's default-environment
assumption; `black_box` fences every operand, FPCR is restored, and the result is offered as a
silicon read of the ISA-level distinction. The lib-emit pass reports three dead-code warnings
(`main`, the FPCR helpers), expected for a bin-shaped file compiled as a lib.
