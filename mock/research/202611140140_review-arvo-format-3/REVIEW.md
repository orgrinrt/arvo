# Review of work/arvo-format, pass 3

**Date:** 2026-11-14
**Reviewer:** a fresh `pr-reviewer-senior` on opus, worktree detached at `91a2604e`.
**Range:** `3865b39a..91a2604e`, the merge base with `origin/dev`. Round `202611140127`, closed after the
review started, is not in it.
**Verbatim:** the reviewer's report as returned, kept whole below.

## Report

# Review of `work/arvo-format`, pass 3 (`3865b39a..91a2604e`)

**Canon gate: the call is handed back on one mechanism.** I checked the branch against `cargo mock query` on the four `ruling` rows below. On the one mechanism that matters, `land` returning nothing at run time, my reading is that it is misaligned. That reading is not independent: the brief pointed me at pass 2 and I read its conclusion before forming mine. Under the two-expert rule my read is confirmation, not corroboration. The verdict below does not depend on that call. Findings C1, T1, T2 and P1 stand on the branch's own design text and on reproductions.

## 1. Canon gate

| Row | Rung | Outcome |
|---|---|---|
| `ruling::the_format_spine_is_canon` | ratified | **Handed back.** The row says arithmetic is "an exact operation ... composed with a named total adaptation". `land` calls itself "the completion of `adapt` past the range" (`landing.rs:138-140`). It returns `Isnt` on admitted signatures (see C2), so it is not total. My read: misaligned. It is not independent, so one blind read is still owed. |
| `ruling::never_a_runtime_check_and_one_lowered_path` | ratified | **Zero-denominator phase:** aligned for impls that keep the default `ADMITTED`, held by the compile-fail pair. **The `Isnt` on four-limb overflow:** a run-time check on the point's value, with no lead-designer blessing cited. Ticket `arvo_s_exact_position_carries_no_remainder_denominator_past` is an agent-filed task. My read: misaligned, same caveat as above. |
| `ruling::the_operating_constraints_are_intents_and_rules` | in_force | No heap, `dyn` or `std` added. The bare-primitive dispute (pass-1 finding 3) is still open. |
| `ruling::wrap_or_clamp_stays_open_and_both_get_priced` | open | Not filled. |
| `proposal::wrapping_is_admitted_exactly_where_the_representable_set_is_one_progression` | proposal | Still used as provisional. The exhaustive instrument (`990f0e11c61c`) is still owed. |

## 2. Verdict per closed round

| Round | Verdict |
|---|---|
| 1910 (IEEE 754) | Merge after fixes. The parity suites never reach the region where `land` returns nothing (C2). |
| li8r, ouou, c27b, c5qc, wqfi, dp7c | Merge. Carried from pass 2; I did not re-derive these. |
| aun1, jb7q | Merge after fixes: the `perf:` commit types, unchanged. |
| sknb, ibrf | Superseded by 055 for `land`. |
| **202611140055** | **Do not merge.** C1 is a regression it introduced, and T1 and T2 are setup that helps. |

**The equivalent-mutant claim** (src changelist, "Passing an unreadable magnitude ..."):
- **Sound within its premises.** On the walk `m` only falls. For `m >= 0` the divisor is `D0 < 2^191`, and a range holding zero over several magnitudes has `reach + 1 <= 2^64`, so an unformed numerator is always provably past. For `m < 0` both `Isnt` routes (divisor overflow, and a `K` past the limbs failing `past_the_range`) are absorbing. Slope-zero and single-magnitude formats are trivially equal.
- **But both premises are trait defaults that can be overridden.** "At most `2^64`" comes from `Slots::ADMITTED`. `format.rs:80-90` says outright that an implementor can write an empty `ADMITTED`, and that this is why the predicates "stay total". So the claim holds only for formats that keep the default `Format::ADMITTED` and `Slots::ADMITTED`, and it must say so. Outside that, M1 shows the walk breaking.
- **One slip that changes nothing.** "the reduced phase's under `2^63`" should be "at most `2^63`": `Phase::of(1, i64::MIN)` reduces to `q = 2^63`. The conclusion still holds, because `(2^128-1)·2^63·2^64 < 2^255`.
- I checked this by reasoning. I did not build the mutant.

## 3. Earlier findings

**Pass 2:**
1. **Coarser-magnitude landing: addressed.** `the_limits.rs` pins magnitude 135, slot 127. The oracle part is only partly addressed: every point is now counted, but its "nothing" region is `land`'s own arithmetic (T1).
2. **`ADMITTED` not forced: addressed**, for default-`ADMITTED` impls, with the compile-fail pair. It regressed for overriding impls: what used to be an `Isnt` is now a panic (C1).
3. **`land` partial at run time: not addressed in substance.** A test now pins the `Isnt` as correct (T3). The second canon read is in section 1.
4. **Tests:**
   - Falling clamp: addressed (`past_the_ends.rs`).
   - All six modes plus three dithers: addressed (`the_members.rs`).
   - An `Isnt` path: addressed, but it pins a placeholder (T3).
   - `is_one_progression`: partly. There are three more hand-picked laws; it is still a sample, not the exhaustive instrument.
5. **Set test only picks the message: addressed in the text.**
6. **Leakage: partly.** The reference to the review is gone. "Three points the walk once missed" (`DESIGN.md.tmpl:2342`, `docs/FORMAT.md:2351`) still tells defect history in a published document.
7. **"50 magnitudes": the comment is fixed, the substance is not.** With the cancellation fixed, the format was not widened back (T2).
8. **Second landing in the IEEE harness: partly.** It no longer passes silently, but the hand-rolled walk at `tests/ieee754/mod.rs:83` remains a second landing.
9. **README and `Scaled` equality: partly.** The `PartialEq` derive is dropped. `README.md.tmpl` still does not say `land` can return nothing (P1).

**Pass 1, still open:**
- **3, bare primitives: not addressed.** It went to question `388c088fe`, held by coord. Two reads disagreeing should go to the human, and this one has not reached him.
- **4, `Remainder::of` / `Fraction::of` reading zero as zero: unchanged.** `position.rs` is untouched in the range.
- **7, `perf:` commits: not addressed.**
- **8, `Exact` growth unpriced: not addressed.** Task `da5cc43e8`.

## 4. New findings

**C1. A format overriding `ADMITTED` now panics in `land`.** High, correctness, introduced by 055.
- `placing.rs:51-55` (`admitted_phase`) assumes forcing `Format::ADMITTED` rules out a zero denominator. `format.rs:84-90` says an implementor may override it with an empty body.
- The previous behaviour was `Isnt`; it is now a panic.
- `DESIGN.md.tmpl:2234` ("never reached at run time") is false for such a format.
- **Repro:** a `Format` with `PHASE = Phase::of(1, 0)` and `const ADMITTED: () = ();` panics with "attempt to divide by zero" at `wide.rs:318`, in debug and in release.
- **Same class:** a `Slots` impl overriding `ADMITTED` with `MIN`/`MAX` at `i128::MIN`/`i128::MAX` overflows at `landing.rs:395` under `Wrap`.
- **Fix:** force a constant the implementor cannot override, as the crate-private `Landable` already does (`landing.rs:91-129`), for the phase and for the 64-bit width cap.

**C2. `land` returns nothing on ordinary IEEE underflow.** Blocker, correctness and canon.
- Reproduced with the shipped aliases:
  - `Attributed<Binary64, RoundTowardPositive, Saturate>` at `2^-1400` returns `Isnt`.
  - `Decimal64` at `10^-480` returns `Isnt`.
  - The standard's answer is the smallest subnormal under toward-positive and zero under ties-to-even. Binary64 fails for any value below about `2^-1329`, which includes the exact product of `2^-700 · 2^-700`.
- **Why the suites miss it:** the harness says itself that the parity inputs never reach this region (`tests/ieee754/mod.rs:81`). The README's parity claim (`README.md.tmpl:47`) reads as covering it.
- **It is not a wall.** At `landing.rs:358-362` the numerator `K` is formed and the divisor overflows, so `|position| < 1`. A sticky remainder representative keeps every comparison `adapt` makes, against a half and against a dither of bounded denominator. Where `K` is past the limbs, the common radix powers can be cancelled before anything is formed. This part is my reasoning, not a committed probe.
- **No FIXME** sits at any of the `Isnt` returns (`landing.rs:160-162`, `358-385`).

**T1. The oracle counts its own copy of `land`'s limits as agreement.** Blocker, tests.
- `placed` (`tests.rs:176-246`) restates `distance` and `position_at`: the same `e`, the same terms, the same `past_the_range` threshold.
- `(Maybe::Isnt, Want::Nothing) => true` (`tests.rs:383`) then counts each shared limb overflow as a pass.
- **Measured:** on `ZeroDecimal` with the `Far` sweep, `land` returns `Isnt` on 480 of 2880 points per mode. All of them are at `k <= -159`, where the answer is slot 0 or ±1 at the finest magnitude.
- `the_walk_starts_no_later_than_the_finest_magnitude_in_range` reports those points as correct. The `nowhere * 2 <= all` guard tolerates 50%.
- The commit body of `ddc6460f`, "The oracle covers that region", is false.
- **Fix:** an oracle with more limbs than `land`, test-local, so that `Nothing` is never expected where the answer is decidable.

**T2. The phased decimal sweep is cut to where `land` answers.** Tests.
- At its pre-narrowing width, `Indexed<-100, 200>` with the `Far` sweep, `land` returns `Isnt` on 2152 of 2880 points.
- The 30-magnitude `PhasedDecimal` and the ±60 `Near` sweep are what keep the guard under 50% (`tests.rs:352-361`, `448`).

**T3. `past_the_remainder_s_limit_nothing_lands` passes on an answer the design has not met.** Tests, `the_limits.rs:51-61`.
- `at(-256)` should land on slot 0 with a sticky remainder.
- It should be a red test with a FIXME naming the ticket, not a green one that asserts `Isnt`.

**P1. The published documents claim what C1 and C2 refute.** Public surface, blocker.
- `DESIGN.md.tmpl:2234` and `docs/FORMAT.md:2243`: "never reached at run time".
- `README.md.tmpl:47`: the parity claim. The README's sentence on `land` also never says it can return nothing.

**M1. The mutant-equivalence claim leaves its premises unstated.** Drift: see section 2.

**D1. Unreachable branches in `distance`.** Drift, low.
- The `_ => Isnt` arm (`landing.rs:254`) and both `under_half` guards (`252-253`) can never fire. One term always carries `radix^0` and is under `2^191`.
- State that by construction rather than as a dead check at run time.

**R1. Commit messages.** Rules.
- `f0eeb2e3` uses the type `registry:`, which is not in arvo's type list.
- `ef0053dd` has git's default merge subject.
- `ddc6460f` and `95eedd34` are not imperative ("land forms").
- `ddc6460f`'s body leaves `land` and `Format::ADMITTED` unbackticked.

**R2. Tickets cited by merge sha.** Naming, low. The changelists cite tickets by clause-dev merge sha (`388c088fee6c`, `da5cc43e8757`, `bb82327ee374`) rather than by ticket key.

**Where the reproductions are.** They are uncommitted spikes in `/tmp/claude-0/-home-user/7c35e2ad-fb65-5898-963d-96f2bad980fd/scratchpad/r3/src/main.rs`, run with `cargo run --release -- hollow|whole|sweep|ieee` (no argument for the `2^-256` and wrap cases). That scratchpad is shared with the author's session. Commit them as red tests in the fix.

## 5. Verified, and the commands run

- `CARGO_INCREMENTAL=0 cargo test -p arvo-format --no-fail-fast`: green. lib 335, compile_fail 32, ieee754 binary 14 and decimal 5, matlab fi 15 and fimath 268, ties_away 10, plus 4 and 5.
- `cargo clippy -p arvo-format --all-targets -- -D warnings`: clean.
- `cargo mock --lint-only`: all passed (warnings only).
- The `pr-review` `surface --range` and `attribution` sweeps: clean. Neither "ruled unrepairable" hit is in the range.
- All 72 commit messages read. No em-dashes in the diff.
- Checked by hand:
  - `distance`, `first_candidate` (the floor and ceiling of `L`, `least`, `room`) and the wrap residue are correct over admitted slots.
  - The topic's `2^99`, `2^130` and `2^128 + 4` cases are present and correct.
  - `written_out` is an independent oracle.
- The worktree is clean and still at `91a2604e`.

## Verdict: DO NOT MERGE

This is the third pass, so under the local flow the branch goes back to the coordinator. The rounds pass 2 cleared still stand, and splitting them off from `land` is the coordinator's call. One independent read of the `land`-totality question, blind to passes 2 and 3, is still owed.

## The author's reading

C1 and C2 were rerun from the reviewer's spike on the repo's toolchain and print what the report says: a
format with an empty `ADMITTED` and a zero-denominator phase panics dividing by zero in
`Wide::floor_divided`, and `binary64` at `2^-1400` and `decimal64` at `1e-480` under toward-positive land
nothing, where `2^-1080` and `binary32` at `2^-400` land on slot 1. The other findings were not rerun.

C2 holds against the design as written. The design declares the four-limb remainder as a limit, but
the reviewer's sticky-remainder reading says no comparison `adapt` makes needs more, and that is not a
limit the IEEE parity claim mentions. So the `Isnt` is the defect, and T3's green test pins it. T1 and T2
are the same defect seen from the tests: the oracle and the narrowed sweeps stop where `land` stops.

This is the third pass, so the branch goes back to coord instead of into a fourth fix loop. The blockers
are tickets: C1 is `land_panics_on_a_format_that_overrides_admitted`; C2 with T2, T3 and P1 is
`land_lands_nothing_below_the_four_limb_remainder_ieee_underf`; T1 is
`the_landing_oracle_shares_land_s_four_limb_limits`. M1's premise and the `2^63` slip, D1, R1 and R2 are
left for whoever takes `land` back up. Round `202611140127`, the wrapping product, closed after the
review started and has had no review.
