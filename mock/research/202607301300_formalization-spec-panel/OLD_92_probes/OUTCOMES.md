# 92_probes/OUTCOMES.md

All files built (and where stated, run) fresh this session, inside the repo tree, on the
pinned toolchain (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`,
confirmed by `rustc --version` immediately before the first compile). Working directory:
`mock/research/202607301300_formalization-spec-panel/92_probes/`. No probe here is a
runtime performance claim; the bench harness was not needed and not run.

Re-verification of the file under second read, performed first: all five of `87_probes/`
rebuilt and (probes 2 and 3) re-run this session. Probe 1 reproduces the warn-level
`invalid_value` diagnostic verbatim ("this code causes undefined behavior when executed"),
exit 0. Probe 1b reproduces `error[E0004]: non-exhaustive patterns: `&Shrunk::C` not
covered`, refusing. Probe 2 reproduces `value-keyed read: 5000 ... 0xf388` against
`0x1388`. Probe 3 reproduces all three tiers verbatim. File 87's compiled claims stand
exactly as its OUTCOMES.md states them.

## probe_1: the seal, attacked by introduction route (six files)

`probe_1_tower.rs` builds as an rlib (`--crate-name tower --crate-type rlib`, exit 0):
the sealed `NicheCarrier` vocabulary at model scale, one explicit member
(`NonZeroU16`), one consuming blanket (`unsafe impl<C: NicheCarrier> Crosses for
ViaNiche<C>`), zero granting blankets. Each attack compiles against it with
`--extern tower`.

```
=== probe_1a_attack_direct_impl ===      (foreign impl of the sealed trait)
error[E0277]: the trait bound `Forged: tower::sealed::Sealed` is not satisfied
exit: 1
=== probe_1b_attack_supertrait ===       (foreign impl of the private supertrait)
error[E0603]: module `sealed` is private
exit: 1
=== probe_1c_attack_transparent_wrapper === (repr(transparent) wrapper over a member)
error[E0277]: the trait bound `Evil: tower::sealed::Sealed` is not satisfied
exit: 1
=== probe_1d_attack_reach_blanket ===    (instantiate the audited entry at a non-member)
error[E0277]: the trait bound `NotANiche: NicheCarrier` is not satisfied
exit: 1
=== probe_1e_controls ===                (honest member + foreign hand-laid unsafe impl)
exit: 0
```

The wrapper attack (1c) is the nonvacuous instantiation the separation requirement asks
for: `Evil` is layout-identical to an honest member (`repr(transparent)` over
`NonZeroU16`) and is still refused, so the seal separates membership-by-impl from
layout-identity, which is exactly the distinction file 87 needed it to hold. 1d refuses
at `ViaNiche`'s own struct bound, before `Crosses` is consulted. The second control in
1e compiles deliberately: a foreign hand-laid `unsafe impl Crosses` is the trusted-base
tier's own front door working as designed, and the seal does not (and should not) close
it; the seal's guarantee is only that the ONE audited niche entry covers no unaudited
type. The re-impl (E0117) and downstream-blanket (E0210) routes from file 46's table are
closed by the orphan rules with no probe needed, same as there.

## probe_2_vocabulary_shape.rs

```
$ rustc --edition 2021 --crate-type lib --emit=metadata probe_2_vocabulary_shape.rs
exit: 0
```

All const assertions hold: char's validity set has a non-inhabitant (0xD800) strictly
between two inhabitants (0xD7FF, 0xE000), so it is not one excluded run at zero and no
bias-by-k debias maps it to a contiguous domain; bool's 2 inhabitants equal 2^1, so
field-shrinking already expresses it; no power of two equals 65535 or 4095 (the
collision, re-confirmed at both widths file 87 used); a 2^13-value bounded domain in
NonZeroU16 leaves 57343 inhabitants with no decode.

## probe_3_width_claim_const_checkable.rs

```
$ rustc --edition 2021 --crate-type lib --emit=metadata probe_3_width_claim_const_checkable.rs
exit: 0
```

All const assertions hold on the pin: `size_of::<Option<NonZeroU16>>() == 2`
(documented NPO guarantee), `size_of::<MaybeModel<NonZeroU16>>() == 2` for the
Maybe-shaped local enum (a per-pin fact, NOT a documented guarantee for a non-Option
enum; the assertion is itself the pinning mechanism, which is notko's own `MaybeNull`
discipline, notko/src/maybe.rs:40-45), and the negative control
`size_of::<MaybeModel<u16>>() == 4`, which makes the second assertion nonvacuous.

## probe_4_combined_case.rs

```
$ rustc --edition 2021 -O -o /tmp/92/p4 probe_4_combined_case.rs
compile exit: 0        (zero diagnostics; nothing fires on the `*b.to_raw_mut() = 0` line)
$ /tmp/92/p4
combined case: typed door, 10 safe mutations, value = 30, excluded pattern unreachable
throughout; integer door compiled with zero diagnostics and stays unexecuted
run exit: 0
```

The combined case consolidation nine's open list owes (91:997-999). Compiled facts: the
integer-typed raw door (`&mut u16`, file 87 probe 3's shape unchanged) transplanted onto
a niche carrier admits the niche-violating write `*door = 0` with **no diagnostic of any
kind**, quieter than 87 probe 1's value-transmute (which at least drew the warn-level
`invalid_value` lint); the violating body is compiled and never executed, because
executing it is undefined behaviour, not decorrelation. The typed door
(`&mut NonZeroU16`) is safe, and ten arbitrary mutations through it cannot reach the
excluded pattern, structurally, because no safely-constructed `NonZeroU16` is zero. The
padding obligation is const-confirmed vacuous at every NonZero member width (whole-byte
carriers, whole-byte bitpack groups, zero pad bits).
