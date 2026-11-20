# Review of `work/arvo-numeral-exact-step`, pass 1

Branch HEAD `7df1e65e`, against `origin/dev` `3c78ee81`, merge base taken at `72c08840`. Read only; nothing built or run, as the brief ordered.

Canon gate: passed. Moving the exact step into `arvo-format` agrees with `ruling::the_format_spine_is_canon` (ratified, by both), which says arithmetic on a format is "an exact operation in the ambient domain composed with a named total adaptation", and with the workspace rule `a-crate-holds-only-its-own-thing`. `Member::landed` maps a run-time slot totally and classifies nothing, so it stays clear of `ruling::the_ingest_door_waits_on_a_known_use`. This is the first independent read of whether the stamp fits the canon. A second read is still owed under the two-expert rule.

## Verdict

MERGE AFTER FIXES. Findings 1 to 3 block.

## Findings

1. `mock/design_rounds/202611200739_changelist.src.md` [rules], blocks. The src changelist is not locked and round `202611200734` is not closed. Every earlier round on this branch was closed before the branch moved on (`2ec1faa0`, `365d6715`, `ad9ef225`). This branch would put an open round onto `dev`. Fix: `cargo mock lock`, then `cargo mock close`, before landing.

2. `mock/crates/arvo-format/src/member/step/tests.rs:204-263` [tests], blocks. The branch deleted the edge matrix that `8fd8fae8` added on this same branch: `room.rs`'s `bound::the_three_are_formed_here_to_the_rooms_edge_at_every_radix`. That test held `PRODUCT`, `QUOTIENT` and `HOLDS` at their edge and one past it, at widths 8, 16, 32 and 64, for six ambients, against independent arithmetic (`edges::product_room`, `quotient_room`, `host_bound`). The replacement holds the edge only at 64 bits, plus single points at `Fixed<8, 2>`, `UFixed<8, -100>`, `UFixed<16, -8>` and `UFixed<32/33, 0>`. The numeral's every-shape sweeps compare results with the oracle, and both arms give the same answer there, so a bound drifting inward at 8, 16 or 32 bits would turn no test red. That is a law sampled at one width of four. `tests/common/edges.rs:18-20` still says the format's "own tests hold its bound there", which the format's tests no longer do. The topic (section 1) says the bound tests "move with it", but most of the matrix was dropped instead. Fix: port the matrix into the format's tests, with the edge arithmetic re-derived inside `arvo-format`. The dependency direction forbids reusing the numeral's helper.

3. Commit messages [public-surface], blocks. `orgrinrt/arvo` is public. These messages describe how the work was produced rather than what changed:
   - `a2426475` body: "Two seats, the second blind on a worktree cut before the first, agree on...".
   - `39316635`: "land the first review of the numeral doors".
   - `c65658ff`: "open the topic on the second review and land its report".
   - `b3b011f4`: "...and reopen the doc cl".

   I read all 31 non-merge messages one at a time. None carries a byline, `Co-Authored-By` or a tool advert. Fix: reword these four before the merge into `dev`, or get the human to accept them in writing.

4. `mock/crates/arvo-numeral/src/num.rs:172`, `:408` and `:417` [drift]. `Num::member` now sends every operand through `Member::landed`. The range comparison in `complete_index` (`apply.rs:51`) cannot be dropped by the compiler wherever the range does not fill the carrier, for example `Fixed<6, _>` held in `i8`. `room.rs` read slots directly, so the product and the conversion now pay this cost too, not only the quotient. `ruling::never_a_runtime_check_and_one_lowered_path` asks that branching be erased "as much as possible". The topic (section 4) pushes the rerun onto the ticket `member_adapted_lands_an_on_grid_slot_through_the_four_limb_a`, yet this branch is that ticket's cure, and the ticket covers the quotient only. `7df1e65e` re-resolved the `-room` arms' lockfiles without a run, so the committed results now describe deleted code. `landed` also exists only on that unmeasured cost. The `complete_slot` fast path (`apply.rs:27-31`) may already make `adapted` at `Exact::on_grid` just as cheap. Fix: run `numeral-ratio` with a `landed` arm, an `adapted`-on-grid arm and the hand arm, plus `cargo mock bench disasm` of the product at a width that leaves part of the carrier unused. Close or narrow the ticket from those results.

5. `step/tests.rs:276-311` [tests]. After `apply.rs:27-31`, `adapted` at a position on the grid goes through the same `complete_index` that `landed` calls. The test cannot catch a wrong completion. It can only catch a wrong on-grid `round_slot` or a broken `checked_add` guard. The completion itself is covered by the older oracle tests (`apply/tests/the_oracle.rs`, `the_far_end_of_the_index.rs`). Fix: rename the test and rewrite its doc to say what it actually guards.

6. `step/tests.rs:150-158` [tests]. Every pair checked in the six-bit sweep falls in the 64-bit narrow arm. The 128-bit arm is held to the slot step only at the binary 64-bit corners. Radix ten and three never reach the 128-bit arm in a format test. Fix: add decimal and ternary corners next to the binary ones in `the_64_bit_corners_...`.

7. `mock/crates/arvo-format/src/member.rs:107` [naming]. A 114-column doc line, longer than any line on `dev`. In `member/step.rs:6-8` the module summary runs as one 3-line sentence; the summary should be under 80 columns.

## Verified

- `Ratio::exact` floor and residue for negative ratios, and the narrowing casts. Both are bounded by the constants.
- `PRODUCT`, `QUOTIENT` and `HOLDS` bound the numerator and the denominator from the range. `Divisor` doors keep `|b| >= 1` and the divisor inside the range at a zero phase (`division.rs`).
- `complete_index`'s wrap formula, with each term reduced separately.
- Each step forces its slot step's `ADMITTED` in both arms. The changed `.stderr` files follow from that.
- The mutant sketch is committed with its outputs. Each flag goes red.
- `docs/` was regenerated in the same commit as the templates. No em-dashes or bold in the added prose. No version or licence moved.

## Notes

I could not run `cargo mock` to confirm that the generated documents match their templates; whoever lands the branch should run it and read `git status`. Finding 2 removes coverage that a previous review on this branch had asked for.
