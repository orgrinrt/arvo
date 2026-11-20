# Review pass 2, arvo-numeral-doors

**Date:** 2026-11-20
**Reviewed:** `work/arvo-numeral-doors` at `7afe7350`, by a fresh `pr-reviewer-senior`.

**Canon gate: partly passed, and one call is handed back to you.** I checked the branch against `ruling::the_format_spine_is_canon` (an exact step composed with a named adaptation, which `room` keeps), `ruling::arms_over_regions_are_the_fundamental_heart` and `ruling::never_a_runtime_check_and_one_lowered_path` (each arm is picked by `if const`), and `ruling::arvo_is_a_library_and_the_value_composes_on_top`. The branch fits all four. Where the exact step lives (finding 2) is not settled by the registry. Pass 1 was the first read. I am the second and read the canon on my own: it is silent on this. Silence is not permission, and a workspace rule does speak to it, so I hand the call back rather than settle it.

## Verdict

**DO NOT MERGE as it stands.**
- Finding 2 needs the handed-back call first. Landing before it is made decides it by silence.
- Finding 1 blocks on its own, because a commit body leaks workflow on a public repo.
- Once both are settled, findings 3 to 5 bring this to MERGE AFTER FIXES.

## Findings

1. **Commit `51ef3a67` body [public-surface, blocker].** It reads "Two opus seats, the second blind on a worktree cut before the first". `gh api repos/orgrinrt/arvo` says the repo is `public`. This names the model and describes how the change was produced, which `public-surface.md` lists as leakage that blocks. `git log dev` has no body naming a model. Pass 1 only checked subjects. Fix: reword the body on the feature branch before landing. Minor, same sweep: the `18b923ae` body leaves `Member::adapted`, `Divisor::member` and `adapt` without backticks.

2. **`DESIGN.md.tmpl:321-324` and `src/room.rs` [drift/locus, high; canon call handed back].**
   - `room.rs` is a second exact-position step inside the value crate, next to the format's `product_position`, `quotient_position` and `converted_position`.
   - The design's justification is false as written: "The format cannot take that arm itself ... an arm there would be a test of the operands at run time". The format already ships `Member<F>` (`arvo-format/src/member.rs:34`, built by `adapted` at `:108`). A step over `Member` operands would carry the same constant bound with no operand test.
   - The round's own topic admits this: "holds only because the format has no position step over `Member<F>` operands" (`202611200013_topic...md:75-77`).
   - `arvo-format/DESIGN.md.tmpl:1201` puts the exact step in the format's map.
   - The workspace rules `a-crate-holds-only-its-own-thing` ("no private second version of it ... give the owner what it needs") and `extract-the-shared-logic-never-mirror-it` point the arm at `arvo-format`. The old design text, removed by this branch, said the same: "a second arithmetic kept equal to the first by tests alone".
   - Filing ticket `arvo_format_has_no_exact_position_step_over_member_operands` is a deferral, not a fix.
   - At minimum, the public sentence must stop calling a gap in the format's current surface an impossibility.

3. **`DEEPDIVE_PROOF.md.tmpl:192-194` (rendered at `docs/NUMERAL_PROOF.md:202`) [correctness/spec, medium].** It says "every width up to 8 ... on the first [host] arm". That is false at the highest and deepest exponents of every width.
   - Product, highest exponent: `product_highest(1,false,2)` is 255. `power(2,255)` is `Isnt`, so `PRODUCT` is false.
   - Product, deepest exponent: -255, same result.
   - Quotient at 8 bits: the highest exponent is 248, and `times(REACH, POWERS.0)` is `Isnt`.
   - Both ends of every family run on the four limbs. This is the same kind of error as pass-1 finding 2, and it was not fixed.
   - Related, low: `DESIGN.md.tmpl:318` says "one 128-bit division", but the narrow terms take the 64-bit one (`room.rs:223-225`).

4. **The new tests have no red run, and the edge tests sample the law [tests, medium].**
   - The only red run on record, `202611192130-room_value-mutant/outputs/`, comes from `6a1b8e58`. That is before `531269a3` added the edge tests, and its `run.sh` test list leaves out `the_conversion_at_the_room_edge`.
   - I ran `RUSTFLAGS="--cfg arvo_mutant_room" cargo test -p arvo-numeral` on the new and changed binaries. This was a quick check by hand and is not committed. `the_conversion_at_the_room_edge` stayed green on all 6 tests: a conversion from exponent 0 to a deeper one is always on the grid, so the mutant never reaches it.
   - Separately, the sixth exponent (`common::edges::{product_room, quotient_room}`) is tied to `Sides::PRODUCT`/`QUOTIENT` and `Crossing::HOLDS` only through `room.rs` `bound`, at radix 2 and 8 and 64 bits. Both arms equal the oracle, so a predicate stricter than the room at radix 10 or 3 leaves every suite green. The claim that the room arm is tested at its edge is therefore unverified away from radix 2.
   - Fix: in `bound`, assert each predicate at the room exponent and one past it, over every width, both ranges and radixes 2, 10 and 3. Add a mutant that reaches the conversion edge, for example `holds` admitting one exponent too far. Commit the red run.

5. **`docs/NUMERAL.md:355-356` [public-surface, low].** `{{ reg::question::... }}` renders as bare snake_case with no backticks and no `question::` prefix. The same document writes `` `question::...` `` everywhere else (`:37`, `:39`, `:365`).

6. **`mock/test-history/default/arvo-numeral.json` [tests, low].** My `cargo mock test` said "arvo-numeral: its own inputs moved", although no file under the three crates changed after `531269a3`. It then rewrote this file, which is now **modified and uncommitted in `/home/user/arvo`**, and I left it for you. The committed record therefore does not match the head's inputs. Find out which outside path moved, and commit the new record.

7. **Notes outside the branch.**
   - `tickets-arvo.toml:2018` quotes run `20261119T223043Z` (1.25 us, 151 ns), which is not committed. By the evidence rule those numbers are void.
   - `donald-knuth.md:456` says a reconciliation is owed before any proposal is filed. None exists.

## Pass-1 findings

| # | Status | Evidence |
|---|---|---|
| 1 docs stale | resolved | `68054405` regenerated `docs/`, and no template or row changed after it. The stale sentences are gone. |
| 2 product overclaim | resolved, with a low residue | `DESIGN:318-320`. The test was renamed and asserts -127 true and -128 false at 64 bits, both ranges, and 8-bit signed. Finding 3 is the same kind of error and is still open. |
| 3 remainder citation | resolved | Registry references at `DESIGN:346-347`, ticket name gone. New rendering issue in finding 5. |
| 4 void factors | resolved | Recorded in a new file, round-2 topic §"The first run's factors are void", which names the claim and what depended on it. |
| 5 by-value sweep | resolved | Divides by 1-255 and 1-127, nothing skipped, `landed == 0` asserted. It failed under the mutant in my run. `passed` only checks the sweep's own size. |
| 6 suites at the edge | partly | Room exponent added and a conversion-edge file written, but see finding 4. |
| 7 bench | not resolved, ticketed | `numeral_ratio_bench_prices_the_64_bit_point_and_the_quotient`. The quotient is still 9.0x the hand version and unexplained. |
| 8 registry rows | mostly resolved | `cf14848b`: the sign clause is gone, 306/720 is now attributed to the first seat, the question note and option 2 are fixed. The reconciliation is still missing. |
| 9 locus | not resolved | Ticket only. I rate it higher than pass 1 did; see finding 2. |

## What I ran

- `CARGO_INCREMENTAL=0 cargo mock test` at `7afe7350`: "10 tree(s) green", exit 0.
  - 103 arvo-numeral units were run fresh, including all of `the_conversion_at_the_room_edge` and `the_quotient_by_a_value`.
  - 32 binaries were reused from the branch's own record and not re-run, among them the `every_shape` suites.
- The mutant run above, which also re-ran those suites. Results:
  - The product and quotient shape bands, binary and decimal: red, for example "`mul` disagrees with the oracle on 59489 of 1133760".
  - `the_quotient_by_a_value`: red.
  - `the_conversion_at_the_room_edge`: green.
- I read the body of every test added or changed: `room.rs` `bound`, `edges.rs`, the shape families, the conversion edge and the quotient by a value. None is tautological.
- I checked the arithmetic: the 2^127 bound, -127, -38 and -80, and the hand-written room numbers (-63, -19, -18, -40, -39, -95).
- No em-dashes or en-dashes in added lines; checked with a positive control.
- Every subject line is in the required form.
- `Cargo.toml` adds only a cfg name, and the READMEs are not stale.

