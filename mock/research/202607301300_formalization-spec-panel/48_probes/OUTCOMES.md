# Probe outcomes, file 48

All probes built against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, invoked
as `rustup run nightly-2026-05-28 rustc` (the default `rustc` on this machine resolves to stable
1.94.0 outside the repo directory; recorded so the next member does not lose ten minutes to the
same E0554 and temp-dir surprises). Host `aarch64-apple-darwin`, confirmed `rustc -vV`. Every
outcome below is a `pin + host` fact in file 45's sense.

No new tower copy exists in this directory, deliberately: probes 1 links against
`47_probes/tower.rs` built unmodified (`rustc --edition 2021 --crate-type lib --crate-name tower
tower.rs` from `47_probes/`), per file 42's own finding that a second unsynced copy of a sealed
tower is the hazard, not a convenience.

## Rebuilds of file 47's probes (read as source, rebuilt fresh)

| Probe | Recorded | Reproduced |
|---|---|---|
| `probe_2_writing_a_number` | clean | clean (rc=0) |
| `probe_3_the_grade_is_projected` | clean | clean (rc=0) |
| `probe_4_the_siblings_report` | clean | clean (rc=0); the 81-term exhaustive claims and the reordering table are const assertions and all hold |
| `probe_1b_a_wrong_digit_is_silent` | clean, the finding | clean (rc=0) |
| `probe_5b_defaulted_grouping_refused` | fails, no generic defaults | fails, same error, both sites |
| `probe_3b_no_law_at_any_view_refuses` | E0277, designed message | E0277, same message head |
| `probe_3c_the_projection_is_checked` | E0308 x2 | E0308 x2 |
| `probe_6_the_caller_contract_diagnostic` | E0277 with both remedies | E0277, same head |

Gate: `cargo test --workspace` from `mock/`, 654 passed, 0 failed, 9 ignored, summed per binary.
Matches files 41 through 47 exactly. Design-surface grep (file 45's corrected command):
`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo root, exit 1,
empty.

## This file's probes

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_wall_is_one_refactor_away.rs` | Does the grade projection dissolve the trait-solver wall or stand clear of it? | **FAILS, E0275**, `overflow evaluating the requirement Pz<O<_>>: ExactDivOdd<_>`, the exact signature of files 41/42/46. One where-clause (`Ratio<Hd, Am1>: Reduce`, a plausible "reduced headroom ratio" refactor) puts the divergence in the consumer-facing fold signature. The wall is avoided, not dissolved, and the avoidance rests on a structural property of the bound chain nothing pins. |
| `probe_2_grade_algebra_lib.rs` | The join half of the grade algebra, which 47's two proposals jointly need and neither compiled. | COMPILES CLEAN. Sixteen constructor-headed `Join` impls, no blanket; whole-matrix laws as const assertions and type-equality checks: join-BITS agreement (16), commutativity (16), associativity (64), identity and absorption (4+4), order-join compatibility positive half (9). `Grade` sealed per file 46's two-line checklist. `combine` returns the joined grade by projection: section 1.1's strict semantics carried in section 3.2's mechanism, one signature. |
| `probe_2b_downstream_cannot_mint_a_grade.rs` | Route (a): direct downstream impl of `Grade`. | **FAILS, E0277** on the private supertrait, with rustc's automatic sealed-trait note firing, same self-explaining diagnostic file 47 noted on the numeral seal. |
| `probe_2c_the_seal_is_unnameable.rs` | Route (b): implementing the seal supertrait itself. | **FAILS, E0603**, module `sealed` is private. Split from 2b because E0603 resolves before trait checking and would shadow 2b's refusal. |
| `negctl` (uncommitted control, command below) | Is the order-join gadget capable of failing? | **FAILS, E0277** on `weaker_joins_to::<RefusalsTransferred, Faithful>()`. The nine positive instantiations are not vacuous. The seven negative pairs belong in the shipped compile-fail suite, each forced through a call per file 46's 6.1. |
| `probe_3_the_out_of_range_diagnostic.rs` | What does `nat!` say outside the table? | **FAILS, E0425**, `cannot find type N48000 in module $crate::n`, pointing at the macro and the invocation. Loud and honest, and the number that triggers it (48000, a sample rate) is a constant file 43's exact-division subfamily makes ordinary, not exotic. |

## Verbatim error heads

Probe 1:

```
error[E0275]: overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`
  --> probe_1_the_wall_is_one_refactor_away.rs:45:21
   |
45 |     Ratio<Hd, Am1>: Reduce,
   |                     ^^^^^^
   = note: 126 redundant requirements hidden
   = note: required for `Ratio<Hd, Am1>` to implement `Reduce`
```

Probe 2b:

```
error[E0277]: the trait bound `TotallyFineGrade: grade_lib::sealed::GradeSealed` is not satisfied
   = note: `Grade` is a "sealed trait", because to implement it you also need to implement
     `grade_lib::sealed::GradeSealed`, which is not accessible
```

Probe 3:

```
error[E0425]: cannot find type `N48000` in module `$crate::n`
34 |     ($v:literal) => { $crate::n::${concat(N, $v)} };
   |                                   ^^^^^^^^^^^^^^^ not found in `$crate::n`
41 | pub type SampleRate = nat!(48000);
```

## Reproduction

From a scratch build directory:

```
cp 47_probes/tower.rs 47_probes/vu_nat.rs 47_probes/vu_bias.rs .
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib --crate-name tower tower.rs
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib \
  48_probes/probe_1_the_wall_is_one_refactor_away.rs        # expect E0275
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib --crate-name grade_lib \
  48_probes/probe_2_grade_algebra_lib.rs                     # expect clean
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib --extern grade_lib=libgrade_lib.rlib \
  48_probes/probe_2b_downstream_cannot_mint_a_grade.rs       # expect E0277
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib --extern grade_lib=libgrade_lib.rlib \
  48_probes/probe_2c_the_seal_is_unnameable.rs               # expect E0603
rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib \
  48_probes/probe_3_the_out_of_range_diagnostic.rs           # expect E0425
```

The negative control on the order gadget, not committed as a file because it is four lines and its
whole content is here:

```rust
use grade_lib::{WeakerThan, Join, Grade, RefusalsTransferred, Faithful};
const fn weaker_joins_to<A: WeakerThan<B> + Join<B, Out = B>, B: Grade>() {}
const _: () = weaker_joins_to::<RefusalsTransferred, Faithful>();   // E0277
```
