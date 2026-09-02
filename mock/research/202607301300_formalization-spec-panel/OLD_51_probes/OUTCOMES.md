# Probe outcomes, file 51

All probes built and run against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, confirmed with `rustc -vV` from inside the repo (matches file 50's own pin).
Probes 4 and 5 need `+nightly-2026-05-28` explicitly when invoked from outside a directory carrying the
repo's `rust-toolchain.toml`, because bare `rustc` outside the repo resolves to the `stable` default and
`#![feature(...)]` is refused with `E0554`. Inside the repo tree the pin resolves automatically.

Build and run:

```
rustc --edition 2021 -O probe_1_growth_surface_enumeration.rs          -o /tmp/p1 && /tmp/p1
rustc --edition 2021 -O probe_2_policy_threaded_is_inert.rs            -o /tmp/p2 && /tmp/p2
rustc --edition 2021 probe_3_policy_dependent_growth_refused.rs                    # expected to FAIL, E0119
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -C opt-level=3 \
  --emit asm,llvm-ir probe_4_licence_reassoc_vectorizes.rs -o /tmp/p4
rustc +nightly-2026-05-28 --edition 2021 -O -C opt-level=3 \
  -o /tmp/p5 probe_5_licence_contract_overgrants.rs && /tmp/p5
rustc +nightly-2026-05-28 --edition 2021 -O -C opt-level=3 \
  -o /tmp/p6 probe_6_licence_destroys_compensation.rs && /tmp/p6
```

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_growth_surface_enumeration.rs` | Enumerate the design's operation surface through file 50 (in-numeral arithmetic, `mul_full`, `mulnum` over `Ranged`, `div_exact`, `div_floor`/`rem`, `fold`/`fold_sequential`/`fold_compensated`, `quantize`) and check each operation's result numeral is computed from (Op, operand numeral(s)) with no `Policy` parameter in any of the eleven growth traits. | **WORKS.** Eleven operations, eleven growth traits, zero `Policy` tokens. Reproduces file 50's own numbers where they overlap: `mul_full(p4,p3) -> p7`, `mulnum_ranged(bin32,bin32) -> p48`. |
| `probe_2_policy_threaded_is_inert.rs` | If `Policy` is threaded as an extra generic parameter of `mul_full`'s growth trait, does the projected `Out` type actually depend on which `Policy` is named? | **WORKS as a negative result.** `Out` under `PolicyA` and `Out` under `PolicyB` are forced through a function-signature unification and a `const` equality assertion; both hold. `Policy` compiles as a parameter and computes nothing: legal, inert. |
| `probe_3_policy_dependent_growth_refused.rs` | Write two implementations of the same growth trait for the same `(MulFull, N1, N2)` domain, disagreeing on `Out`, with no `Policy` parameter to disambiguate them (the only way growth could vary "by policy" without becoming Shape A). | **FAILS, `E0119: conflicting implementations of trait MulFullGrowth<_, _> for type MulFull`.** Refused by coherence before any question about which alternative is correct is even reached. |
| `probe_4_licence_reassoc_vectorizes.rs` | Does `f32::algebraic_add` (`float_algebraic`, tracking issue rust-lang/rust#136469) reproduce file 50's own reassociation-refusal measurement (`50:439-453`) and grant the vectoriser the permission it otherwise lacks? | **WORKS.** `sum_plain`: 8x scalar `fadd`, no vector instruction, matching file 50's `5 scalar fadd, 0 vector fadds` for its own reduction. `sum_algebraic`: `fadd.4s` + `faddp` pairwise reduction, the identical two-instruction shape `sum_int`'s `add.4s` + `addv` already gets for free. LLVM IR carries `reassoc nsz arcp contract` on the reduction call, **not** `nnan`/`ninf`. |
| `probe_5_licence_contract_overgrants.rs` | Does the `algebraic_mul`/`algebraic_add` bundle's `contract` flag silently fuse an adjacent multiply and add into one rounding step, the same value `f32::mul_add` computes, on a witness where fused and separately-rounded disagree? | **CONFIRMED.** `mac_algebraic` (chained `.algebraic_mul().algebraic_add()`) and `mac_fma` (`.mul_add()`) both deliver `0x28800000`; `mac_plain` (`c + a * b`) delivers `0x00000000`. `mac_algebraic` compiles to one `fmadd`; `mac_plain` compiles to `fmul` then `fadd`. The design's own droplist entry (`49:921-923`) already names this substitution "a different operation, not a permission"; this is the same substitution arriving silently through a route not spelled `mul_add`. |
| `probe_6_licence_destroys_compensation.rs` | Does the licence, applied to `fold_compensated`'s Kahan-shaped error-feedback step, reassociate the compensation term away? | **CONFIRMED, the textbook failure, compiled.** Strict: `(sum + y) - sum - y` recovers `-9.313226e-10`, the exact lost bits, via three real instructions. Algebraic: the identical expression compiles to one instruction, `fsub s0, s1, s1`, always zero. `reassoc` licenses treating the two as interchangeable because they are algebraically identical; `fold_compensated`'s entire mechanism depends on them not being treated that way. |

Nothing above needed the sealed value-unique tower (`vu_nat_sealed_adj.rs` / `vu_bias_sealed_adj.rs`,
composed by files 46 through 50). Both questions this file answers are about which INPUTS a growth or a
licence decision reads, not about how a numeral encodes its own value; a lightweight marker numeral
(a bare `const P: u32`) carries the question without dragging in encoding machinery that has nothing to
add to it. Reusing the tower here would be composing for its own sake, not for rigour.
