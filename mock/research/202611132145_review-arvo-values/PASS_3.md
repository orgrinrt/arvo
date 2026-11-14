# Third reading of `work/arvo-values`, at `e4e5b456`

A fresh `pr-reviewer-senior` seat on opus, in a detached worktree at `e4e5b456`, with a neutral brief
that named the two earlier readings and `RESPONSE.md` as claims to test. The report is below as it
was returned. It is the last of the three readings a branch gets.

## Verdict

DO NOT MERGE.

The canon gate does not pass for one surface. `Num::convert` and `fits_convert` build one option of a canon question that is still open, and that is the same defect this branch already accepted as grounds to remove `Accumulator`. The rest of the gate passes, checked against `ruling::the_debug_abort_is_configured_by_the_consumer`, `ruling::the_ingest_door_waits_on_a_known_use` and `ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`. The test gate also fails: three laws are claimed over every member of every admitted signature and are tested on a sample of widths and on one radix. I am the first reader to raise finding 1, so a second independent read is owed.

## Findings

1. `mock/crates/arvo-numeral/src/ratio.rs:83-109`, and the design at `DESIGN.md.tmpl:332-336` [canon, blocks; the call is yours]
   - `question::whose_reduction_governs_a_lossy_crossing` (Q36) is open. Its decider is `panel` and it has no `answered` field.
   - Its second option is "The target's: the crossing is the target's business." `convert` does exactly that: "placed on `T`'s grid by `T`'s rounding and completed by `T`'s policy".
   - The only row supporting that choice is `proposal::a_conversion_is_one_placement_onto_the_target_grid`. It sits at `standing = "one_expert"` and does not declare `answers` for Q36.
   - The numeral design never cites Q36. It cites the other open question it touches, so this one was missed rather than weighed.
   - `RESPONSE.md` second reading, item 1, removed `Accumulator` because it was "the first option of `question::which_chain_carrier_ships`, which is open with the panel as its decider". The same reasoning applies here word for word, and the branch did not apply it to itself.
   - What makes this ambiguous rather than plainly misaligned: `arvo_format::convert`, which takes a target signature, already ships on `origin/dev` (`conversion.rs:233`). The branch puts that unratified choice onto the value's public surface; it did not originate it.
   - Two consistent routes: take `convert` and `fits_convert` off the branch as `Accumulator` was, or have the panel answer Q36. Nothing else on the branch depends on `convert`.

2. The laws are tested on a sample of widths [tests, blocks]
   - The design says `div` is "equal to the format's `div` on every member and every divisor" (`DESIGN.md.tmpl:322`) and makes the same claim for `convert` (`:334`).
   - The bands sweep `add`, `sub`, `neg` and the integer `mul` at every width from 1 to 64 (`band_17_to_32.rs` and the others).
   - `the_quotient.rs` takes integer widths 1 to 8, 13, 32 and 64 only. There is no width in 9 to 12, 14 to 31 or 33 to 63, so no 32-bit or 64-bit carrier below its full width is ever divided.
   - `the_conversion.rs:157-179` and `fixed_point_product.rs:63-77` likewise pick individual formats.
   - `PASS_2` finding 3 asked for the bands' machinery to be used for all three. The response widened the sample but did not finish the sweep.

3. The radix axis is absent from the suite [tests, blocks]
   - `Sole` (`admit.rs`) does not refuse a decimal format, and `Format` is open to other crates. So a value over `DecimalRationals` with exponent `-2` is admitted.
   - The oracle cannot express that case: `tests/common/oracle.rs:42` asserts the radix is 2. No numeral test names a decimal format.
   - I compiled and ran a scratch crate (uncommitted, so not evidence) and got six right answers. I am not claiming a defect, only that the suite has nothing to say about this axis.

4. `src/host.rs:130-140`, `holds_index` [tests, blocks with 2]
   - This private verdict decides where `from_index` aborts, and it is observed only at `Int<4>` and `Uint<4>` (`the_debug_check.rs:142-174`).
   - Its arithmetic, `shift = 64 - WIDTH`, has edge cases at widths 1 and 64 that no test instantiates.
   - The design claims the abort happens "exactly where" `adapted` would (`DESIGN.md.tmpl:258`). That is checked at one width.

5. `Cargo.toml:37-48`, with `cfg!(feature = "mutant-*")` in `num.rs:194`, `ratio.rs:50,92` and `host.rs:115` [drift, follow-up]
   - The branch grows the test-mutant features from 4 to 12. Each makes the published library compute wrong answers, and feature unification lets any crate in a build turn one on.
   - The crate's own design rejects a cargo feature as the vehicle for the debug check for exactly that reason (`DESIGN.md.tmpl:210-216`).
   - `cargo test --all-features` turns this crate's suite red.
   - Ticket `arvo_numeral_carries_its_test_mutants_as_cargo_features_in_s` covers it. It should not keep growing.

6. `RESPONSE.md:50` [drift, minor]
   - It says the release twin runs six tests under `--release`. It runs 5 at the tip.

7. `a_consumer_crate.rs:53-75` [tests, minor]
   - The test re-execs itself to show that `CARGO_MANIFEST_DIR` is read at runtime. It does assert something, but what it protects is test plumbing rather than the crate.

8. `arvo-format/src/adapt.rs:9-11,49-55` [drift, minor]
   - Associated items in another lane's crate are reordered with no change in behaviour.

9. Commits [rules, does not block]
   - `9a3d5149` and `ae4efecb` keep git's default merge subject. They are already pushed.
   - The other 55 subjects pass: lowercase, under 72 characters, no parentheses, no bylines.

## Verified

- The `PASS_2` response holds on its other points: `Accumulator` is fully gone, `holds_index` is private, and the "asserts nothing" test is folded into the real one.
- The oracle in `oracle.rs` is independent of the format. I read its rounding, `HalfUp` included, which matches `rounding.rs:51` (ties go toward positive infinity).
- `from_index` handles wrap and saturate correctly for the two cases where the host's sign and the range's sign differ.
- `under` hands over the carrier soundly: the format's equality forces the carrier types equal.
- The committed `mutant-div` and `mutant-convert` runs are red, with their toolchain recorded.
- Added lines contain no em-dashes or en-dashes, checked against a positive control. No bold outside round metadata.
- `docs/NUMERAL.md` was regenerated after the last design change. The README is current.
- The branch is 57 ahead and 0 behind `origin/dev` and merges cleanly.

## What I ran (all from `mock/`)

| Command | Result |
|---|---|
| `cargo test -q -p arvo-numeral` | green |
| `--features hlist-const --test the_count` | 4 passed |
| `--features hlist --test the_count_without_const` | 3 passed |
| `--release --test the_debug_check_in_release` | 5 passed |
| `cargo test -q -p arvo-format` | 301 passed, 1 failed |
| `cargo mock --lint-only --strict` | all lints pass, 46 warnings; worktree clean |

The one `arvo-format` failure is `the_inverse_of_three_round_trips…`, in a file this branch does not touch. It is ticket `arvo_format_inverse_round_trip_underflows_at_width_1`.

## Not checked

- Whether the tickets named in `RESPONSE.md` exist.
- The `.stderr` contents of the ui tests.
- The committed mutant runs from earlier rounds.
- Whether `notko` moving from `0a13d99` to `8a74dbc` in `Cargo.lock` changes anything for `arvo-format`.
- `notko`'s visibility, so whether its https git dependency hangs: `gh repo view` was refused because GraphQL is not reachable from here.

The repo's `.claude/rules/` has no `the-public-surface-is-checked-on-every-pr.md`, so I used the workspace copy.
