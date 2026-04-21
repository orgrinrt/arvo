# #129 — arvo ecosystem lint:allow audit

**Date:** 2026-04-21
**Scope:** arvo workspace source files (8 crates)
**Verdict:** all 5 unique source-file sites are structural or externally blocked; zero fixable-now replacements.

## Background

Task #129 asks whether any `lint:allow(arvo-types-only)` or
`lint:allow(no-bare-numeric)` escape in the arvo source tree can
be removed by replacing the underlying construct with an arvo
primitive. Directive carried from the 2026-04-21 exact-widths
round: "even where lints technically allow, prefer arvo primitives
when one fits."

The allow annotations coexist with the `[primitive-introductions]`
category allowlist in `mockspace.toml`, which declares arvo,
arvo-bits, and arvo-hash as sites where raw numerics may enter the
tree without annotation. Explicit annotations therefore mark
narrower boundaries the author wanted to document inline even
when the category would already permit the line.

## Inventory

Grep pattern: `lint:allow\((arvo-types-only|no-bare-numeric|no-bare-static-str)` restricted to `mock/crates/**/src/**/*.rs`.

| File | Sites | Tracked | Class |
|------|-------|---------|-------|
| arvo-bits/src/bits.rs | 9 | #127 | structural substrate |
| arvo-bits/src/bitfield.rs | 4 | #127 | structural macro-expansion |
| arvo-tensor/src/cap.rs | 1 | #121 | external blocker (rustc) |
| arvo-spectral/src/fiedler.rs | 1 | #123 | deferred retrofit |

Total: 15 annotations across 5 source sites.

## Classification

### arvo-bits/src/bits.rs (9 sites, #127)

Lines 119, 125: per-N narrow from u64 to the dispatched container
(`Self((raw & Self::MASK_U64) as $ty)`) and widen back (`self.0 as
u64`). The macro-expanded `$ty` is `u8`/`u16`/`u32`/`u64`. arvo-bits
is the substrate that introduces `Bits<N>` as the opaque-bit
primitive; its internal bridge to the native container is the
canonical floor of the type stack. Nothing shorter can be
expressed using only arvo primitives.

Lines 141, 143, 145, 149: the `impl_bits_u64!` dispatch table
(`impl_bits_u64!(u8, 1..=8)`, etc). Names the native storage
primitives that `UContainerFor<N>` picks. Structural by definition
of the substrate.

Lines 174, 178, 182, 186: `BitAccess` bridge to the sealed
`BitPrim` trait (`self.0.get_bit(idx.0 as u32)`). `BitPrim` is an
arvo-bits-internal helper whose signature takes u32 indices because
it is implemented on the native u8/u16/u32/u64 types. Changing
`BitPrim` to take `USize`/`Bool` would not reduce annotation count:
the bare-numeric sites would migrate from the bridge call-sites
(29 total across bits.rs + ufixed_impl.rs + ifixed_impl.rs) into
the macro-generated impl bodies (9 methods × 4 primitive types =
36 internal sites). Net loss. The current boundary is the cleanest
layering achievable.

Lines 197, 201, 208: BitSequence bridge (`USize(self.0.count_ones()
as usize)`). Same reasoning as the BitAccess bridge — maps
BitPrim's u32 counts into arvo-bits' USize contract.

### arvo-bits/src/bitfield.rs (4 sites, #127)

Lines 93, 126, 135, 148: macro-expansion sites inside `bitfield!`.
The macro performs compile-time arithmetic on declared `$n`,
`$field_bits`, `$lo` literals (`($lo as u16) + ($field_bits as u16)
<= ($n as u16)`) and generates per-field shift/mask expressions
(`1u64 << $field_bits - 1`). All arithmetic is on macro-input
literals at const-eval time; the declarative-macro grammar has no
surface for passing arvo primitives. Structural.

### arvo-tensor/src/cap.rs (1 site, #121)

Line 16: `pub const fn cap_size(c: Cap) -> usize`. Rust's array
`[T; N]` grammar requires `N: usize` at the language level;
`generic_const_exprs` rejects inline newtype destructuring in
const-generic position. The named `const fn` returning `usize` is
the single idiom that compiles under current nightly. Blocked on
rustc stabilisation of const-eval over newtypes (#121). External.

### arvo-spectral/src/fiedler.rs (1 site, #123)

Line 92: `let n_as_u8 = n as u8`. `arvo::traits::FromConstant::from_constant`
takes `u8`. The `as u8` boundary conversion resolves once #123
retrofits FromConstant to take USize. Deferred to that task. No
local fix.

## Outcome

All 15 annotations across 5 sites are genuinely necessary under the
current language and crate-layering constraints. No bare primitives
in arvo source are replaceable by arvo primitives today; the
boundary lives at language-grammar (#121), internal-trait
contract (#123), or substrate-floor (#127) walls.

#129 closes with no code change. When #121 and #123 resolve, the
annotations on cap.rs and fiedler.rs lift. The #127 annotations
are permanent — arvo-bits is the boundary where the type stack
meets the host's native integers.

## Collateral findings (outside #129 scope)

hilavitkutin and clause source trees carry unrelated format drift
in `lint:allow` comments (pre-#80 `// lint:allow(X) -- reason`
form instead of the current `// lint:allow(X) reason: ...; tracked:
#N` form). These are tracked under their respective sweeps:

- hilavitkutin-persistence/manifest.rs:17,23 — tracked: #72
- hilavitkutin-api/src/lib.rs, hint.rs; hilavitkutin-str/src/*.rs —
  tracked: #72 residual
- clause-syntax, clause-typecheck, clause-codegen, clause-runtime-*
  — `lint:allow(bare_collection)` escapes to remove per #76 when
  the clause sweep (#73) lands.

None of these are arvo sites. They close as part of their respective
sweep rounds.
