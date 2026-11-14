# work/arvo-values, first reading after round 202611140020

Read at `e19aa3dd` against `origin/dev` at `3865b39a`, by a fresh `pr-reviewer-senior` seat on a
neutral brief, read-only. Kept as the seat returned it.

**Verdict: DO NOT MERGE**

Canon gate: passes provisionally, checked against `ruling::the_panel_finishes_the_canon_without_him` (ratified, op: "Every remaining canon question is the panel's ... put through two independent agreements"), `question::whose_reduction_governs_a_lossy_crossing` (`decider = "panel"`, open) and the `stamped_by = "experts"` stamp on `proposal::a_conversion_is_one_placement_onto_the_target_grid`. The design states the conversion as provisional and not settled (`mock/crates/arvo-numeral/DESIGN.md.tmpl:30-33, 342-351`). I am handing one ambiguity back (N1). I am the first reader of the stamped state, so a second read is still owed.

Test gate: fails, on B1 and B2. Three tests claim to see a property that they cannot see.

## Blockers

**B1. The conversion suite cannot tell the stamped option apart from a source-policy reading.** [tests]
- **Where:**
  - `tests/the_conversion_under_every_source_signature.rs:6-8` says "a conversion that did [read the source's mode or policy] would disagree here".
  - `DESIGN.md.tmpl:651-653` says the same.
- **Why that claim is false for the policy:** the sweep only crosses exponent -1 onto 0 at one width and one range (`tests/common/exponents.rs:167-168`). There, |round(a/r)| ≤ |a| for every member, so no member ever leaves the target's range and the source's policy is never consulted.
- **Every other sweep fixes the source policy:** the width, exponent and radix sweeps and the named cases all fix the source at `Adapt<TowardZero, Wrap>` (`common/widths.rs:24`, `common/exponents.rs:60-63, 203`, `the_conversion.rs:111`). So a saturating source crossing a lossy edge is tested nowhere.
- **Reproduction (scratch copy, not committed, this seat is read-only):**
  - Mutant A: replace the body at `src/ratio.rs:100` with `convert_slots::<S::Format, WithOverflow<T, <S's Overflow>>>`. All three `no_source_signature_moves_a_*_conversion` tests pass. Only the width matrix goes red, because its sources wrap.
  - Mutant B: "saturate wherever the source saturates, else T's policy", which is option 1's policy half. It passes all 18 binaries: every `the_conversion*` binary, `the_crossing` and `the_debug_check`.
  - So the exact distinction the stamp decides has no failing test.
- **Fix:**
  - Sweep the source signatures across a step that loses range, such as a narrower width or a coarser exponent near the ends.
  - Commit mutant B as `mutant-…` with its red run.

**B2. The exponent families stop at +1, while the format admits much more.** [tests]
- **Where:** the product's `top` is capped at one (`tests/every_shape/product.rs:20-28`). The quotient and conversion families are `{0, 1, -1, half, deep}`.
- **What the format admits past +1:** `is_multipliable` holds at `Fixed<63, 2>`, `Fixed<8, 40>` and `UFixed<16, 20>`. `is_divisible` holds at `Fixed<8, 40>` and `Fixed<64, 5>`. `is_convertible` holds for `Fixed<8,1>→Fixed<8,20>` and back. These come from a scratch probe, and the first one also follows from the test's own `HELD` predicate: 2^124·4 ≤ i128::MAX.
- **The false claims:** `the_product_at_every_shape_binary.rs` "The family's ends are the obligation's own: one past either is refused" is false for every width under 64. `DESIGN.md.tmpl:620-622` ("last exponent the format admits") is false at the positive end.
- **Fix:** compute the true positive edge per width and radix, and assert admitted at it and refused one past it.

**B3. Sabotage features ship in the published manifest.** [public-surface]
- **Where:** `crates/arvo-numeral/Cargo.toml` features. This branch adds eight to the four on dev: `mutant-sub` to `mutant-count`. They branch inside shipped `pub const fn` bodies (`src/ratio.rs:50,92`, `src/host.rs:115`, `src/num.rs:194`, `src/rung.rs:147,170,210`, `src/count_*_path.rs`).
- **Why it blocks:**
  - Cargo unifies features, so any crate in a build that enables one silently corrupts every consumer's arithmetic.
  - The design's own argument against a feature for the debug check (`DESIGN.md.tmpl:214-221`) applies word for word.
  - `a-hand-check-becomes-a-test-every-time.md` says "declare the wrong construction inside the test module".
  - The checklist blocks on "Cargo.toml carries something that should not ship".
- **Fix:** move the mutants out of the library. The four inherited ones are follow-ups.

## Non-blocking

**N1. Handed back: is the stamp independent?** [canon]
- Both seats derived inside a tree whose design, code and oracle already shipped option 2:
  - `SEAT_1.md:105` says so ("four copies of one author's stance").
  - `SEAT_2.md:73` cites `ratio.rs`.
- The standing vocabulary requires "each deriving before reading the other". The seats' brief is not committed, so their neutrality cannot be checked.
- This call belongs to the coordinator or panel, not to me.

**N2. Registry rows not brought in line with the stamp.** [canon]
- `proposal::a_conversion_is_one_placement_onto_the_target_grid` declares no `answers = ["whose_reduction_governs_a_lossy_crossing"]`.
- `proposal::an_order_is_named_exactly_where_a_crossing_is_lossy` still says the call "is op's" (note, gap). Its `says`, "An order must be named", contradicts the stamped shape. Both seats flagged this and it is still unfixed.

**N3. Seat 1's instrument was Python and is gone.** [evidence]
- Seat 1's model script was not kept. `NOTES.md:19` correctly calls its counts unproven.
- So the stamp's instrumented region (≤8 bits, radix 2) is one instance, seat 2's.

**N4. The committed test record drops a red that is still red.** [tests]
- `1c691760` dropped the `red` record for `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63` from `test-history/default/arvo-format.json`.
- At HEAD, `cargo test -p arvo-format --lib the_inverse_of_three` FAILED: it panics at `the_inverse_round_trip.rs:37:48`. Ticket: `arvo_format_inverse_round_trip_underflows_at_width_1`.
- My `cargo mock test -p arvo-format --cheap` reported FAILED and also wrote no `red`, so this looks like a tool defect. Worth a tooling ticket.

**N5. File size.** [rules] `src/num.rs` is 507 lines (306 on dev). The file-size lint did not fire.

**N6. Commit text.** [rules]
- `2b413b50` body: "The third fresh seat refuses the branch". `8e46316d` and `ff8aeffa` subjects end "by a fresh seat". These are workflow leakage in a public repo.
- `e33c0c13`, `9a3d5149` and `ae4efecb` keep git's default merge subject.

**N7. Declared but unswept matrices.** [tests] The conversion sweeps never cross width × exponent under the modes other than `HalfEven`, or width × radix at all. `DESIGN.md.tmpl:655-656` says so openly, but it is still a sample.

**N8. Pre-existing drift (not this branch).** `DESIGN.md.tmpl:737-746` ("The old families...") reasons from the removed tree, which `CLAUDE.md` forbids.

**N9. I left the worktree dirty.** My `cargo mock test` run rewrote `mock/test-history/default/arvo-format.json` in the review worktree. The `git checkout --` to restore it was refused. Restore it before reusing the worktree.

## Verified

- **arvo-numeral suite:** `cargo test -p arvo-numeral --no-fail-fast` is green, every binary. `hlist-const` the_count passes 4, `hlist` the_count_without_const passes 3, and the release debug-check twin passes 5.
- **`from_index`:** sign-crossing cases under wrap and saturate are correct, and `holds_index` is right at width 64.
- **Rung arithmetic:** `sub`, `neg` and `mul` are exact below carrier width. Signed and unsigned saturation at carrier width are correct.
- **`under`:** the carrier hand-over is sound because the format equality is in the bound.
- **Oracle (`common/oracle.rs`):** it computes from declarations only. Rounding of ties and the wide path are correct.
- **`cargo mock --lint-only`:** all lints pass, and none of the 46 warnings falls on lines this branch added.
- **Added lines:** zero em-dashes (grep checked against a positive control), no new `allow` attribute outside tests, no `dyn`, no `alloc`, and `#![no_std]` is intact.
- **Prose:** the README and the crate readme are current with the surface, and the design is honest that Q36 is open.
- **Open PRs:** none on `orgrinrt/arvo`.
