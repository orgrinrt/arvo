# Probe outcomes, file 47

All probes built against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, confirmed with `rustc --version` / `rustc -vV` from inside the repo. Every
figure below is a compile-time or diagnostic fact on that pin and host. Nothing here is a runtime
measurement and nothing here belongs in a bench harness.

`vu_nat.rs` and `vu_bias.rs` are `46_probes/vu_nat_sealed_adj.rs` and `vu_bias_sealed_adj.rs`
unmodified except for the `#[path]` retarget. `tower.rs` wraps them as a library crate so every
consumer probe compiles across a real crate boundary. Diff against `46_probes/` to audit.

Library builds first:

```
rustc --edition 2021 --crate-type lib --crate-name tower tower.rs
```

## Second reads owed to files 45 and 46

Run before anything else in this file, reading the probe source and forming a reading before reading
either file's conclusion.

| Owed | Command | Outcome |
|---|---|---|
| File 46 `probe_1b` | `rustc --edition 2021 --crate-type lib probe_1_tower_as_42_left_it_lib.rs` then `rustc --edition 2021 --crate-type lib --extern tower_as_42_left_it=libtower_as_42_left_it.rlib probe_1b_foreign_adjustment_still_lands.rs` | **COMPILES CLEAN (rc=0), confirming file 46's defect.** `vu_nat_sealed.rs:448` declares `pub trait Adjustment` with no supertrait, so a downstream local type implements it directly and reaches a fn-forced `A: Adjustment` position with fabricated `NUM = 6, DEN = 12`. Read independently before reading file 46's table. |
| File 46 `probe_3` | `rustc --edition 2021 --crate-type lib probe_2_vu_core_lib.rs` then `rustc --edition 2021 --crate-type lib --extern vu_core=libvu_core.rlib probe_3_direct_impls_refused.rs` | **FAILS x4, one E0277 per sealed trait**, `EvilPos: PosSealed`, `EvilNat: NatSealed`, `EvilAdj: AdjustmentSealed`, `EvilBias: BiasSealed`, matching file 46's reported error heads exactly. |
| File 45 finding 4.1 (`Int`) | `grep -rln "Zpos\|Zneg\|\bZ0\b" 36_probes 41_probes 42_probes 46_probes` | Hits **only** `36_probes` (`OUTCOMES.md`, `probe_6_signed_bias_is_the_same_construction.rs`). No occurrence in the 41, 42 or 46 towers, which build `Bias` as `BZero`/`BPos`/`BNeg` over `Pos` pairs (`41:101-131`). First read, agreeing with file 45. |

Test gate: `cargo test --workspace` from `mock/`, 654 passed, 0 failed, 9 ignored, summed from the
`test result:` lines rather than a headline. Matches files 41 through 46 exactly.

## This file's probes

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_declaration_as_it_stands.rs` | What does a consumer type to declare the numerals for one real column workload, in the ratified encoding, with nothing added? | COMPILES CLEAN, every value const-asserted. The finding is the text, not the outcome: Q0.15's quantum denominator is fifteen nested `O` constructors, precision 37 is `Pz<I<O<I<O<O<H>>>>>>`, and every one is a hand binary decomposition. |
| `probe_1b_a_wrong_digit_is_silent.rs` | Is a mistyped numeral a well-formed numeral? | **COMPILES CLEAN, which is the finding.** `Pz<I<O<I<O<O<H>>>>>>` is 37, `Pz<I<O<I<O<I<H>>>>>>` is 53, `Pz<I<O<I<O<H>>>>>` is 21. One changed or dropped constructor, all three legal, all three admitted at every `Nat`-bounded position, all const-asserted here. Value-uniqueness guarantees one type per value; it cannot guarantee the typed value is the meant one. |
| `probe_1c_the_diagnostic.rs` | What does the consumer read when a numeral mismatch does surface? | FAILS, E0308, verbatim below. The two lines of the message differ by one letter and the reader has to decode positional binary to learn that 37 was wanted and 53 supplied. |
| `probe_2_writing_a_number.rs` | Three spellings of 37: the encoding verbatim, a generated alias `N37`, and `nat!(37)` resolving through the table by `${concat(N, $v)}`. | COMPILES CLEAN. All three are the same type, proved by passing all three to one fn admitting exactly one type. The macro needs only `macro_metavar_expr_concat`, which arvo already enables (`crates/arvo/src/lib.rs:26`), no arithmetic, no recursion, no `generic_const_exprs`. |
| `probe_3_the_grade_is_projected.rs` | Can the published grade be projected rather than declared by the caller? | COMPILES CLEAN. `Folded<<K as FoldGrade>::Out>` in return position is an ordinary associated-type projection and needs no unstable feature. File 37's six positional const parameters become four type parameters the consumer already has, and the grade is gone from the call site. Interior safety is computed from the numerals through the tower's own `Cmp` (`vu_nat.rs:153`). |
| `probe_3b_no_law_at_any_view_refuses.rs` | The clamping composition, which has no law at any view. | FAILS, E0277, verbatim below. As a missing impl rather than a const-eval panic, so the message is a designed `#[diagnostic::on_unimplemented]` and the `help` enumerates every composition that does have a law. |
| `probe_3c_the_projection_is_checked.rs` | Negative control: is the annotated grade checked, or merely inferred? | FAILS x2, E0308, `expected Folded<Faithful>, found Folded<RefusalsTransferred>`. Both the understating combinator and the caller-contract mismatch (file 37's probe 4d in projected form). |
| `probe_4_the_siblings_report.rs` | Strict versus short circuit, checked over every four-leaf term. | COMPILES CLEAN. **CLAIM A refutes this probe's own first hypothesis**: the short-circuit report is grouping-invariant over all 3^4 = 81 terms, because every grouping visits leaves left to right. **CLAIM B** is what survives: the short-circuit report is not invariant under reordering, three orders of one channel multiset giving 0, 1 and 2 events, while strict gives 2 for all three. **CLAIM C**: the short-circuit report loses information monotonically as the data worsens (2, 1, 0 events against strict's 2, 2, 2). |
| `probe_5_precise_surface.rs` | `Precise`'s combinator surface, shapes A and B written as consumer code. | COMPILES CLEAN. Shape B's sequential constructor has to live inside the mechanism because `Folded`'s marker field is private, so a consumer cannot mint a faithful result from outside the perimeter. Shape B is something arvo ships or something nobody has. |
| `probe_5b_defaulted_grouping_refused.rs` | Shape C: one combinator with a defaulted grouping parameter. | FAILS x2, `error: defaults for generic parameters are not allowed here`, future-incompatible, issue #36887, for both a free fn and an inherent method. Shape C does not exist in this language, so the choice is between A and B. |
| `probe_6_the_caller_contract_diagnostic.rs` | Can the caller-contract mismatch name its own remedy? | Half compiles, half FAILS with E0277 carrying both remedies by name, verbatim below. Stating the contract as a bound (`G: Definite`) rather than an exact type (`Folded<Faithful>`) moves the refusal off E0308, which has no customisation surface, onto E0277, which has `#[diagnostic::on_unimplemented]`. Cost: one generic parameter in the consumer's own signature. |
| `probe_5c_shape_a_has_no_door.rs` | Under shape A, the consumer who cannot widen. | FAILS, E0425, `cannot find function fold_sequential in module mechanism`. |

## Verbatim diagnostics

Probe 1c, the numeral mismatch a consumer has to decode:

```
error[E0308]: mismatched types
  --> probe_1c_the_diagnostic.rs:30:14
   |
30 |     needs_37(acc);
   |     -------- ^^^ expected `Accumulator<Pz<I<O<I<O<O<H>>>>>>>`, found `Accumulator<Pz<I<O<I<O<I<H>>>>>>>`
   |
   = note: expected struct `Accumulator<Pz<I<O<I<O<O<H>>>>>>>`
              found struct `Accumulator<Pz<I<O<I<O<I<H>>>>>>>`
```

Probe 3b, the no-law-at-any-view refusal in its projected form:

```
error[E0277]: this composition's fold has no associativity law at any view
   --> probe_3b_no_law_at_any_view_refuses.rs:26:50
    |
 26 |     let f = regroup_fold::<Clamp, Clamp, Signed, H, I<H>>(xs);
    |                                                  ^ regrouping this fold changes the delivered value
    |
    = help: the trait `FoldGrade` is not implemented for `(Unsafe, mechanism::Clamp, mechanism::Clamp, Signed)`
    = note: no published grade makes the regrouping honest, because the values themselves diverge
    = note: widen the accumulator until the fold is interior-safe, or do not regroup
    = help: the following other types implement trait `FoldGrade`:
              (Safe, Top, Bot, Dom)
              (Unsafe, ReduceModulo, ReduceModulo, Signed)
              (Unsafe, ReduceModulo, ReduceModulo, Unsigned)
              (Unsafe, ReduceModulo, Refuse, Dom)
              (Unsafe, Refuse, ReduceModulo, Dom)
              (Unsafe, Refuse, Refuse, Dom)
note: required by a bound in `regroup_fold`
```

Probe 3c, the projection checked in both directions:

```
error[E0308]: mismatched types
  --> probe_3c_the_projection_is_checked.rs:29:5
   |
28 | pub fn understated(xs: &[i32]) -> Folded<Faithful> {
   |                                   ---------------- expected `Folded<Faithful>` because of return type
29 |     regroup_fold::<Refuse, Refuse, Signed, H, I<H>>(xs)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Folded<Faithful>`, found `Folded<RefusalsTransferred>`
```

Probe 5b, shape C:

```
error: defaults for generic parameters are not allowed here
  --> probe_5b_defaulted_grouping_refused.rs:23:13
   |
23 | pub fn fold<G = Regrouped>(xs: &[i32]) -> i32 {
   |             ^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #36887
```

Probe 6, the caller contract stated as a bound:

```
error[E0277]: this fold's definedness does not match the sequential fold's
   |     --------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ published grade `RefusalsTransferred`
   |     |
   |     required by a bound introduced by this call
   |
help: the trait `Definite` is not implemented for `RefusalsTransferred`
   = note: this combinator may refuse where a sequential fold returned, or return where it refused
   = note: to get a faithful fold: widen the accumulator numeral until the fold is interior-safe, or
           call `fold_sequential`, which does not regroup and pays for it
help: the trait `Definite` is implemented for `Faithful`
```

## Price (measured, `price/`)

`price/gen.py` emits four kinds; `price/sweep.sh` runs them; both sweeps are recorded in
`price/results.csv`. Same shape as files 36, 41, 42 and 46:
`--emit=metadata`, min of three, two counts (0 and 400), so every time figure is a difference
quotient rather than a fitted slope. Every emitted instantiation in the `alias_table` and
`grade_projected` kinds is const-asserted against a Python-computed value, so a wrong sweep fails to
compile rather than reporting a number. `grade_declared` is file 37's own mechanism, included by
`#[path]`, unmodified.

| kind | ms/item | metadata B/item |
|---|---|---|
| `alias_table_bare` (a decimal-literal table, no per-row check) | 0.031 | 165.4 |
| `alias_table` (the same table with a const assertion per row) | 0.083 | 668.1 |
| `grade_projected` (file 47 probe 3, const items) | 0.132 | 561.7 |
| `grade_declared` (file 37 probe 4, const items, unmodified) | 0.100 | 1493.7 |

**Scope, stated because two of these numbers do not say what they look like they say.** The metadata
figures are deterministic: the same byte counts came back on every run, and the projected form is
2.66x smaller than the declared form. The time figures are not: across two full sweeps
`grade_declared` came back at 0.139 and 0.100 ms per item and `grade_projected` at 0.127 and 0.132,
so **compile time does not separate the two mechanisms at this count on this harness** and no claim
is made that it does. `alias_table_bare` came back with a negative difference on one run, which is
the honest way of saying a 400-row alias table costs no measurable compile time; its metadata cost is
165 bytes per row and that figure is stable. Both grade kinds emit const items so the comparison is
matched on item kind; an earlier unmatched run with the projected form as fns is not reported.
