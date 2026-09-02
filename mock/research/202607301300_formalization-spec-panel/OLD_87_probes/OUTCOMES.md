# 87_probes/OUTCOMES.md

All four files built and run fresh this session, inside the repo tree, on the pinned
toolchain (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`,
confirmed by `rustc --version` immediately before each compile). Working directory:
`mock/research/202607301300_formalization-spec-panel/87_probes/`.

## probe_1_niche_domain_and_reachability.rs

```
$ rustc --edition 2021 --crate-type=lib --emit=metadata -o /tmp/probe1.rmeta probe_1_niche_domain_and_reachability.rs
warning: the type `NonZero<u16>` does not permit zero-initialization
  --> probe_1_niche_domain_and_reachability.rs:58:14
   |
58 |     unsafe { transmute::<u16, NonZeroU16>(0) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this code causes undefined behavior when executed
   |
   = note: `core::num::NonZero<u16>` must be non-null
   = note: because `core::num::niche_types::NonZeroU16Inner` must be non-null
   = note: `#[warn(invalid_value)]` on by default

warning: 1 warning emitted
exit: 0
```

Confirms, independently of `86_probes/probe_1`, at a different width (a 12-bit biased
domain, 4095 members, alongside the full 16-bit case, 65535 members): no `k` in `0..=16`
gives `2^k` equal to either domain size, checked exhaustively in const position, not
argued from parity alone. Confirms `NonZeroU16::new(0)` refuses at the safe constructor
(`is_none()` asserted in const position). Confirms the excluded pattern is reachable
through unsafe code: the compiler accepts `transmute::<u16, NonZeroU16>(0)` as a
compiling function (never called anywhere in the file, grep-confirmed zero call sites),
and its own diagnostic is a **lint, `invalid_value`, warn by default**, not a hard
error, with the exact wording "this code causes undefined behavior when executed": the
compiler is telling the reader the code compiles and only fails at execution. This is
the compiled distinction between "refused" (probe_1b, below) and "reachable but
undefined."

## probe_1b_exhaustiveness_negative_control.rs

```
$ rustc --edition 2021 --crate-type=lib --emit=metadata -o /tmp/probe1b.rmeta probe_1b_exhaustiveness_negative_control.rs
error[E0004]: non-exhaustive patterns: `&Shrunk::C` not covered
 --> probe_1b_exhaustiveness_negative_control.rs:11:11
...
error: aborting due to 1 previous error
exit: 1
```

Negative control for probe 1's exhaustiveness claim: a field-shrunk closed enum with a
third, unhandled variant fails to compile, hard, `E0004`. Contrasted with probe 1's
niche-transmute case (a warn-level lint, not a compile refusal), this is the compiled
shape of the provable-versus-trusted distinction section 1 of the deliverable rests on.

## probe_1c_custom_niche_unavailable.rs

Two attempts, both refused, corroborating file 84's finding (`84_probes/probe_5b`)
independently rather than re-reading it:

```
$ rustc --edition 2021 --crate-type=bin -o /tmp/probe1c probe_1c_custom_niche_unavailable.rs
error[E0658]: use of unstable library feature `pattern_type_macro`
error[E0432]: unresolved import `core::pat::pattern_type`
```

(inside the tree, with `#![feature(pattern_types, core_pattern_type)]` already enabled;
a second, undocumented gate `pattern_type_macro` is needed and is not in
`unstable-features.md`'s tables at all).

```
$ rustc --edition 2021 --crate-type=bin -o /tmp/probe1c_outside probe1c_outside.rs   # outside the tree
error[E0554]: `#![feature]` may not be used on the stable release channel
```

No claim of exhaustiveness beyond what these two attempts show: a downstream author
cannot, on the permitted or even the pinned nightly's feature set, declare an arbitrary
custom validity range in one step. This corroborates, it does not independently
re-derive, file 84's own more thorough vetting.

## probe_2_mutation_decorrelates_byte_image.rs

```
$ rustc --edition 2021 -O -o /tmp/probe2 probe_2_mutation_decorrelates_byte_image.rs
compile exit: 0
$ /tmp/probe2
value-keyed read: 5000 (correct) | byte image at birth: 0x1388 | byte image after padding-only mutation: 0xf388 | fresh construction: 0x1388
run exit: 0
```

All four assertions in the file passed. Live, executed demonstration (not merely
argued) that a raw mutable door into a padding zone, present even without any niche
mechanism (an ordinary `u16` carrier has no validity range of its own; this is the
weakest form of the attack, requiring nothing beyond `repr(transparent)` plus an
ordinary `&mut`), leaves every value-keyed read correct (`5000`) while the raw byte
image decorrelates from what a fresh, canonical construction of the identical value
produces (`0xf388` against `0x1388`), and that the fresh construction reproduces the
pre-mutation image exactly (`0x1388` both times), confirming the divergence is the
mutation's doing and nothing else.

## probe_3_two_tier_repair.rs

```
$ rustc --edition 2021 -O -o /tmp/probe3 probe_3_two_tier_repair.rs
compile exit: 0
$ /tmp/probe3
tier 1: 10 safe mutations, padding canonical throughout
tier 2 (contract honoured): padding canonical, byte image = 0x01f4
tier 2 (contract violated): value-keyed read still correct (500), byte image decorrelated (0xe1f4 != 0x01f4), exactly as the door's own safety contract said would happen
run exit: 0
```

All assertions passed. Confirms the two-tier repair is buildable at the model's scale:
a safe surface (`set`, `add_wrapping`) that never exposes a raw accessor into the
padding zone keeps every one of ten arbitrary mutations canonical, unconditionally
(structural, not disciplinary); an unsafe escape hatch (`to_raw_mut`) carries a
documented, trusted-base postcondition exactly parallel to `Crosses`'s own statement
0/P obligations, and violating it produces exactly the decorrelation probe 2
demonstrated, nothing worse and nothing hidden.
