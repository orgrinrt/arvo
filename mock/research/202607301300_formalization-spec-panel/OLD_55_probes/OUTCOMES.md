# Probe outcomes, file 55

Every probe here built against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, resolved from `rust-toolchain.toml` (`channel = "nightly-2026-05-28"`) by
running from inside the repo tree. File 52's warning is real and was re-confirmed here: `rustc
--version` run from `/tmp` on this machine reports `rustc 1.94.0 (4a4ef493e 2026-03-02)`, a stable
toolchain, so no probe in this directory is compiled from outside `~/Dev/clause-dev/arvo`.

Nothing here is a timing measurement. Every figure is a compile outcome, a diagnostic, or a value
printed by a test binary; none belongs in a bench harness.

Test gate: `cargo test --workspace` from `mock/`, summed from the per-binary `test result:` lines
rather than a headline: **654 passed, 0 failed, 9 ignored**. Identical to every file from 41 through
54. Re-run after the two temporary tests below were removed: same figures, working tree clean apart
from this directory.

## Library builds first

```
rustc --edition 2021 --crate-type lib --crate-name tower tower.rs
rustc --edition 2021 --crate-type lib --crate-name grade_lib grade_lib.rs
```

`vu_nat.rs`, `vu_bias.rs` and `tower.rs` are `47_probes/`'s copies unmodified (themselves
`46_probes/`'s sealed tower). `grade_lib.rs` is `48_probes/probe_2_grade_algebra_lib.rs` unmodified.
Diff against those directories to audit. Nothing in this file re-derives either.

## Full rebuild from clean, verified

```
rustc --edition 2021 --crate-type lib --crate-name tower tower.rs
rustc --edition 2021 --crate-type lib --crate-name grade_lib grade_lib.rs
rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib --extern grade_lib=libgrade_lib.rlib probe_1_the_arity_of_a_rank.rs                 # rc=0
rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib --extern grade_lib=libgrade_lib.rlib probe_2_the_grade_of_a_fixpoint.rs             # rc=0
rustc --edition 2021 --crate-type lib --crate-name p2b --extern tower=libtower.rlib --extern grade_lib=libgrade_lib.rlib probe_2b_the_arity_of_an_unbounded_loop.rs  # rc=0
rustc --edition 2021 --crate-type lib --crate-name p4 --extern tower=libtower.rlib probe_4_the_result_numeral_and_the_constant.rs                      # rc=0
rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib probe_1b_the_const_capacity_cannot_state_it.rs                                      # rc=1, intended
rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib probe_1c_the_two_spellings_cannot_drift.rs                                          # rc=1, intended
rustc --edition 2021 --crate-type lib -L . --extern tower=libtower.rlib --extern grade_lib=libgrade_lib.rlib --extern p2b=./libp2b.rlib probe_2c_the_refusals_behind_the_fixpoint.rs  # rc=1, intended
rustc --edition 2021 --crate-type lib -L . --extern tower=libtower.rlib --extern p4=./libp4.rlib probe_4b_the_constant_and_the_numeral_refuse.rs       # rc=1, intended
```

`-L .` is required on the two cross-crate refusals: without it rustc reports `E0463: can't find
crate` for an `--extern` naming an rlib in the working directory when other rlibs are present, which
cost ten minutes and is recorded so it costs nobody else any.

## Second read owed to file 48

Run before this file's own probes, reading the source and forming a reading before reading file 48's
table.

| Owed | Command | Outcome |
|---|---|---|
| File 48's `Same` gadget is a real check, not a tautology | `rustc --edition 2021 --crate-type lib --extern grade_lib=libgrade_lib.rlib` on a file asserting `same::<J<Faithful, RefusalsTransferred>, Faithful>()` and `same::<J<J<RefusalsTransferred, EventsTransferred>, Faithful>, RefusalsTransferred>()` | **FAILS x2, `E0277`**, `RefusalsTransferred: Same<Faithful>` and `BothTransferred: Same<RefusalsTransferred>` not satisfied. The whole-matrix laws in `48_probes/probe_2` are therefore load-bearing rather than vacuous. First read, agreeing with file 48. |
| File 48's join matrix omits idempotence | read `48_probes/probe_2_grade_algebra_lib.rs` law section | Confirmed: commutativity, associativity, identity, absorption and order-join compatibility are present; `J<G, G> = G` is not asserted anywhere. It is the one law a fixpoint needs, and probe 2 adds it over the whole carrier. |

## This file's probes

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_arity_of_a_rank.rs` | Can `upward_rank` state the interior-safety obligation the design's fold surface requires? | COMPILES CLEAN once `Capacity` carries a `Pos` face. The arity is the node capacity, which the signature already has; it has it as a const, and the obligation needs a type. |
| `probe_1b_the_const_capacity_cannot_state_it.rs` | The same obligation from today's `const CAP`. | **FAILS x3.** rustc names the forbidden feature itself: "add `#![feature(generic_const_exprs)]`". |
| `probe_1c_the_two_spellings_cannot_drift.rs` | Does the forced agreement between the const and the type actually fire? | **FAILS, `E0080`**, `DimBoth<63, P64>` refused at monomorphisation with the assertion named. The check is real, and it fires at USE rather than at declaration, which is stated as a limit rather than hidden. |
| `probe_2_the_grade_of_a_fixpoint.rs` | What does a published grade mean across an unbounded number of iterations? | COMPILES CLEAN. Join is idempotent over the whole four-point carrier at widths one through four and in both associations, plus the sixteen seed-then-four-steps cells. So the grade of an iteration is trip-count independent. |
| `probe_2b_the_arity_of_an_unbounded_loop.rs` | The three shapes for an iteration's arity, and whether the resulting grade is actionable. | COMPILES CLEAN. Lifting the trip count to a type works and costs the trip count becoming compile-time knowledge; the `Unbounded` arity marker is two lines and coexists with the `Pos` blanket without `min_specialization`; the top-of-lattice grade is actionable, with the sign-reading consumer compiling and the magnitude-reading one refused. |
| `probe_2c_the_refusals_behind_the_fixpoint.rs` | Probe 2b's negative controls. | **FAILS x4.** `TRIPS * STEP` in a bound is `generic_const_exprs`; the magnitude consumer is refused against both a wrapping and a refusing solver. |
| `probe_3a_the_rank_has_no_expected_value.rs.txt` | Write down the correct answer for `upward_rank` on a four-node chain of weight-100 nodes. | **FAILS TO COMPILE, and that is the result.** `error: literal out of range for u8`, twice: the correct ranks, 300 and 400, have no spelling in the type `upward_rank` returns. There is no correct expected value to write, which is a statement about the signature rather than about the test. |
| `probe_3b_the_rank_is_wrong_at_both_presets.rs.txt` | The same input with the comparison widened enough to be expressible. | **FAILS x2 at runtime, and the printed values are the finding.** `Hot` returns `[144, 44, 200, 100]` where the answer is `[400, 300, 200, 100]`, and on two independent chains it ranks the weight-400 path at 144 below the weight-210 path at 210: the ordering inverts. `Precise` returns `[255, 255, 200, 100]`: two nodes tied that the true ranks separate. Neither preset reports anything. |
| `probe_4_the_result_numeral_and_the_constant.rs` | The signature that fixes probe 3, and the representability obligation nobody states. | COMPILES CLEAN. `foldnum(W, C::Dim)` is spelled entirely from what the signature already carries; `FromConstantKeyed<C>` puts the literal in a bound. |
| `probe_4b_the_constant_and_the_numeral_refuse.rs` | Probe 4's negative controls. | **FAILS x3.** The widened return type is checked rather than inferred; `U8Num` has no 300; `Q0_15` has no 2, and the diagnostic says which constants it does have and which bound wanted the missing one. |

## The two temporary tests, and how to reproduce them

Neither is committed to the tree. Both were written into the mock workspace, run, and removed; the
suite was re-run clean afterward. Their sources are the two `.rs.txt` files in this directory, and the
`zz_` prefix keeps them out of any alphabetical-first position if anyone restores them.

**The `FromConstant` representability measurement.** Copy the file below to
`mock/crates/arvo/tests/zz_fc_check.rs` and run `cargo test -p arvo --test zz_fc_check`.

```rust
use arvo::traits::FromConstant;
use arvo::{Transparent, UFixed, USize};
use arvo::strategy::Hot;
use arvo::{ibits, fbits};

type Q8_16 = UFixed<{ ibits(8) }, { fbits(16) }, Hot>;

#[test]
fn from_constant_admits_a_value_the_numeral_cannot_hold() {
    let v = <Q8_16 as FromConstant>::from_constant::<{ USize(300) }>();
    let bits = Transparent::raw(v);
    let raw: u32 = Transparent::raw(bits);
    let max_raw: u32 = (1u32 << 24) - 1;
    println!("raw = {raw}, max raw for 24 logical bits = {max_raw}");
    assert!(raw <= max_raw, "raw {raw} exceeds the numeral's own bit width {max_raw}");
}
```

Output, verbatim:

```
raw = 19660800, max raw for 24 logical bits = 16777215

thread 'from_constant_admits_a_value_the_numeral_cannot_hold' panicked at crates/arvo/tests/zz_fc_check.rs:15:5:
raw 19660800 exceeds the numeral's own bit width 16777215
```

**The rank demonstration.** Copy `probe_3b_the_rank_is_wrong_at_both_presets.rs.txt` to
`mock/crates/arvo-graph/tests/zz_rank_overflow.rs` and run
`cargo test -p arvo-graph --test zz_rank_overflow -- --nocapture`. Output, verbatim:

```
running 3 tests
rank = [144, 44, 200, 100]
rank A(node 0) = 144, rank B(node 2) = 210
precise rank = [255, 255, 200, 100]
test precise_rank_on_the_same_input ... ok
test hot_rank_ordering_inverts ... FAILED
test hot_rank_on_a_four_chain_of_hundreds ... FAILED

---- hot_rank_ordering_inverts stdout ----
thread 'hot_rank_ordering_inverts' panicked at crates/arvo-graph/tests/zz_rank_overflow.rs:52:5:
the longer path ranked below the shorter one: 144 vs 210

---- hot_rank_on_a_four_chain_of_hundreds stdout ----
assertion `left == right` failed: rank[1] should be 300
  left: 44
 right: 300
```

`probe_3a_the_rank_has_no_expected_value.rs.txt` is the same file before the two comparisons were
widened to `u32`. It does not compile, which is its result:

```
error: literal out of range for `u8`
  --> crates/arvo-graph/tests/zz_rank_overflow.rs:32:31
   |
32 |     assert_eq!(r[1].to_raw(), 300, "rank[1] should be 300");
   |                               ^^^
   |
   = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
   = note: `#[deny(overflowing_literals)]` on by default

error: literal out of range for `u8`
  --> crates/arvo-graph/tests/zz_rank_overflow.rs:33:31
   |
33 |     assert_eq!(r[0].to_raw(), 400, "rank[0] should be 400");
   |                               ^^^
   |
   = note: the literal `400` does not fit into the type `u8` whose range is `0..=255`
```

## One hypothesis this file formed and refuted itself

`arvo-spectral`'s bound is `Add + Sub + Mul + Sqrt + Recip + TotalOrd + Copy + FromConstant`
(`fiedler.rs:61-68`). `Recip` is implemented on exactly four shipped types, all floats
(`arvo/src/traits/recip.rs:12,20,28,36`), and `Sqrt` on those plus integer-only `UFixed`
(`arvo/src/traits/sqrt.rs:24,109,117,125,133`). The hypothesis was that no shipped arvo type
satisfies the whole conjunction, which would make the crate generic over an uninhabited contract.

**Refuted by compiling it.** `FastFloat<f32>` satisfies the whole bound and both `fiedler_vector` and
`power_iteration` run on it, checked with a temporary test at
`mock/crates/arvo-spectral/tests/zz_shipped_numeral.rs` (2 passed) and removed. What survives is the
weaker and still real finding: the crate has two shipped inhabitant families, and every one of its own
ten test files uses neither, running instead on `TF`, a test-local newtype over bare `f32`
(`arvo-spectral/tests/common/mod.rs:21`).

## Verbatim diagnostics

### probe_1b, the const capacity cannot state the obligation

```
error: generic parameters may not be used in const operations
  --> probe_1b_the_const_capacity_cannot_state_it.rs:57:26
   |
57 |     Hd: InteriorSafety<{ C::CAP - 1 }>,
   |                          ^ cannot perform const operation using `C`
   |
   = note: type parameters may not be used in const expressions
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0575]: expected associated type, found associated constant `CapacityAsShipped::CAP`
  --> probe_1b_the_const_capacity_cannot_state_it.rs:64:24
   |
48 |     type Array<T>;
   |     -------------- associated type `Array` defined here
...
64 |     Hd: InteriorSafety<<C as CapacityAsShipped>::CAP>,
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a associated type
   |
help: maybe you meant this associated type
   |
64 -     Hd: InteriorSafety<<C as CapacityAsShipped>::CAP>,
64 +     Hd: InteriorSafety<<C as CapacityAsShipped>::Array>,
   |

error[E0747]: constant provided when a type was expected
  --> probe_1b_the_const_capacity_cannot_state_it.rs:57:24
   |
57 |     Hd: InteriorSafety<{ C::CAP - 1 }>,
   |                        ^^^^^^^^^^^^^^

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0575, E0747.
For more information about an error, try `rustc --explain E0575`.
```

### probe_1c, the two spellings cannot drift

```
error[E0080]: evaluation panicked: assertion failed: N as u64 == <Pz<P> as Nat>::VAL
  --> probe_1c_the_two_spellings_cannot_drift.rs:48:24
   |
48 |     const AGREES: () = assert!(N as u64 == <Pz<P> as Nat>::VAL);
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `<DimBoth<63, tower::nat::O<tower::nat::O<tower::nat::O<tower::nat::O<tower::nat::O<tower::nat::O<tower::nat::H>>>>>>> as DimAgrees>::AGREES` failed here

note: erroneous constant encountered
  --> probe_1c_the_two_spellings_cannot_drift.rs:43:18
   |
43 |         let () = Self::AGREES;
   |                  ^^^^^^^^^^^^

note: the above error was encountered while instantiating `fn <DimBoth<63, O<O<O<O<O<O<H>>>>>>> as DimAgrees>::witness`
  --> probe_1c_the_two_spellings_cannot_drift.rs:60:5
   |
60 |     <DimBoth<63, P64> as DimAgrees>::witness();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0080`.
```

### probe_2c, the refusals behind the fixpoint

```
error: generic parameters may not be used in const operations
  --> probe_2c_the_refusals_behind_the_fixpoint.rs:25:31
   |
25 |     Hd: p2b::InteriorSafety<{ TRIPS * STEP }>,
   |                               ^^^^^ cannot perform const operation using `TRIPS`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `TRIPS`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
  --> probe_2c_the_refusals_behind_the_fixpoint.rs:25:39
   |
25 |     Hd: p2b::InteriorSafety<{ TRIPS * STEP }>,
   |                                       ^^^^ cannot perform const operation using `STEP`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `STEP`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0747]: constant provided when a type was expected
  --> probe_2c_the_refusals_behind_the_fixpoint.rs:25:29
   |
25 |     Hd: p2b::InteriorSafety<{ TRIPS * STEP }>,
   |                             ^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> probe_2c_the_refusals_behind_the_fixpoint.rs:31:20
    |
 31 |     needs_faithful(fiedler_hot(t))
    |     -------------- ^^^^^^^^^^^^^^ expected `Iterated<Faithful>`, found `Iterated<EventsTransferred>`
    |     |
    |     arguments to this function are incorrect
    |
    = note: expected struct `Iterated<grade_lib::Faithful>`
               found struct `Iterated<grade_lib::EventsTransferred>`
note: function defined here
   --> probe_2b_the_arity_of_an_unbounded_loop.rs:207:8
    |
207 | pub fn needs_faithful(_v: Iterated<Faithful>) -> u8 {
    |        ^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> probe_2c_the_refusals_behind_the_fixpoint.rs:37:20
    |
 37 |     needs_faithful(p2b::fiedler_precise(t))
    |     -------------- ^^^^^^^^^^^^^^^^^^^^^^^ expected `Iterated<Faithful>`, found `Iterated<RefusalsTransferred>`
    |     |
    |     arguments to this function are incorrect
    |
    = note: expected struct `Iterated<grade_lib::Faithful>`
               found struct `Iterated<grade_lib::RefusalsTransferred>`
note: function defined here
   --> probe_2b_the_arity_of_an_unbounded_loop.rs:207:8
    |
207 | pub fn needs_faithful(_v: Iterated<Faithful>) -> u8 {
    |        ^^^^^^^^^^^^^^

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0308, E0747.
For more information about an error, try `rustc --explain E0308`.
```

### probe_4b, the constant and the numeral refuse

```
error[E0308]: mismatched types
  --> probe_4b_the_constant_and_the_numeral_refuse.rs:28:5
   |
27 | pub fn wrong_result_numeral() -> Ranks<Cap64, Num<p4::P10Public>> {
   |                                  -------------------------------- expected `Ranks<Cap64, Num<O<I<O<H>>>>>` because of return type
28 |     upward_rank_widening::<Cap64, Num<p4::P8Public>>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Ranks<Cap64, Num<O<I<O<H>>>>>`, found `Ranks<Cap64, Num<O<I<I<H>>>>>`
   |
   = note: expected struct `Ranks<_, Num<tower::bias::nat::O<tower::bias::nat::I<tower::bias::nat::O<tower::bias::nat::H>>>>>`
              found struct `Ranks<_, Num<tower::bias::nat::O<tower::bias::nat::I<tower::bias::nat::I<tower::bias::nat::H>>>>>`
   = note: the full name for the type has been written to 'probe_4b_the_constant_and_the_numeral_refuse.long-type-17470172810765657645.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0277]: the trait bound `U8Num: FromConstantKeyed<300>` is not satisfied
   --> probe_4b_the_constant_and_the_numeral_refuse.rs:33:6
    |
 33 |     <U8Num as p4::FromConstantKeyed<300>>::get()
    |      ^^^^^ the trait `FromConstantKeyed<300>` is not implemented for `U8Num`
    |
help: `U8Num` implements trait `FromConstantKeyed<C>`
   --> probe_4_the_result_numeral_and_the_constant.rs:226:1
    |
226 | impl FromConstantKeyed<0> for U8Num {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `FromConstantKeyed<0>`
...
231 | impl FromConstantKeyed<1> for U8Num {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `FromConstantKeyed<1>`
...
236 | impl FromConstantKeyed<2> for U8Num {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `FromConstantKeyed<2>`

error[E0277]: the trait bound `Q0_15: FromConstantKeyed<2>` is not satisfied
   --> probe_4b_the_constant_and_the_numeral_refuse.rs:38:24
    |
 38 |     lambda_max_bound::<Q0_15>()
    |                        ^^^^^ the trait `FromConstantKeyed<2>` is not implemented for `Q0_15`
    |
help: the trait `FromConstantKeyed<2>` is not implemented for `Q0_15`
      but trait `FromConstantKeyed<0>` is implemented for it
   --> probe_4_the_result_numeral_and_the_constant.rs:243:1
    |
243 | impl FromConstantKeyed<0> for Q0_15 {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `lambda_max_bound`
   --> probe_4_the_result_numeral_and_the_constant.rs:257:31
    |
255 | pub fn lambda_max_bound<F>() -> F
    |        ---------------- required by a bound in this function
256 | where
257 |     F: FromConstantKeyed<0> + FromConstantKeyed<2>,
    |                               ^^^^^^^^^^^^^^^^^^^^ required by this bound in `lambda_max_bound`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

