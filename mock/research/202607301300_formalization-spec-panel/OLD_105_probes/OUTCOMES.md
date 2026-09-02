# 105_probes outcomes

This file's own contribution is a second read of five already-adjudicated items, not new mechanism,
so there is nothing new to compile. What is recorded here is the direct re-execution I ran myself,
per the method constraint (compile-time and emitted-code inspection are mine to do directly), rather
than trusting a predecessor's OUTCOMES.md by citation.

## Canon gate, re-run fresh

```
git log -1 --format="%H %ci"
grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"
grep -rln "FullRange\|UTerm\|AddWidth" mock/crates/ --include="*.rs"
```

```
364486583479045b8fba20f23ec5d2e8ad07fba8 2026-08-05 07:04:11 +0300
exit1: 1
exit2: 1
```

Both empty, both exit 1, at HEAD `3644865`.

## Test gate, re-run fresh

```
cd mock && cargo test --offline --workspace
```

Summed across every `test result:` line: **155 binaries, 672 passed, 0 failed, 9 ignored**, from a
clean tree. Matches `102`, `103`, and `104`'s reported counts exactly.

## Three known tautologies, re-read at source this session

```
mock/crates/arvo-tensor/tests/capacity.rs:14-18        -> assert_eq!(<Dim<3> as Capacity>::CAP, cap(3)) and siblings, substitutes to x==x against src/capacity.rs
mock/crates/arvo-tensor/tests/const_capacity.rs:49-53   -> assert_eq!(<Dim<3> as ConstCapacity>::CAP, cap(3)) and sibling, identical shape, second impl
mock/crates/arvo-hash/tests/aliases.rs:16-23            -> content_hash_roundtrip: ContentHash::from_raw(0x0123_4567_89AB_CDEF_u64) on both sides of one assert_eq!
```

All three present, unchanged, still counted in the green total above. Twenty-six files carried now.

## Existing probes re-read and re-verified against their claims (not re-executed independently;
## OUTCOMES.md content cross-checked against probe source and against the claims made in 97, 100, 103, 104)

- `97_probes/probe_1_foreclosed_region.rs` + `probe_1b_pairing_refused.rs`: read in full. Confirms the
  retirement claim in section 1 below: `embed` refuses `Bounded::<8192>` with `E0080` while
  `typed_mut` is never reached; the only declarable value-unique pairing sweeps all 65,535
  inhabitants with zero orphans.
- `92_spj...` section 1.3 and `95_pesce...` section 2.1: read in full at source (not just the
  consolidation's compression) to independently re-derive the retirement before reading file 97's
  conclusion.
- `100_probes/probe_1_shape_over_the_settled_capacity.rs` + `probe_2_the_agreement_door_is_not_
  one_door.rs`: OUTCOMES.md read in full; confirms `COUNT == 12`, `size_of(Store) == 21` disagree
  through the trait door and agree (refuse) through the inherent door, both compiled, claims A
  through D.
- `103_probes/p8_second_truth.rs`, `p2_codegen.s`: OUTCOMES.md read for the `_run_b1 = _run_a` symbol
  alias claim (byte-identical lowering) and the 22-vs-34-instruction inline cost.
- `104_probes/p1_overlap_shipped.rs`, `p5_occupancy_mask.rs`: source and OUTCOMES.md read in full;
  re-derived the overlap defect and the 65,536-pair prefix-mask separation independently from the
  probe source before trusting the prose summary.

## What this file adds that is not a re-verification

Section 4 below (the connective reading across 92, 100, 103, 104) is reasoned, not compiled: a fresh
corpus search establishing that no earlier file names the four instances as one pattern (dated in the
main document). The claim that a product of Boolean algebras is itself a Boolean algebra (used in
section 3 below) is standard universal algebra (Boolean algebras form a variety closed under direct
products; Birkhoff), external and cited, not something this toolchain can check and not offered as a
compiled result.
