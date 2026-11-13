# Review r3: orgrinrt/arvo `work/arvo-registry-clean`, 8945eee3..30ec7a08

Canon gate passed. I checked the range against the registry (`canon_paths`) and against `ruling::what_the_written_history_shows_to_be_his_is_ratified_by_default`. The `maintainer` route has a basis in that ruling's quote: "The rest can be on provisional stamps to begin with". I am the first reader on whether that route fits the canon, so a second read is still owed. The public-surface checklist is not in arvo's `.claude/rules/`. I used the workspace copy and `pr-review/reference/public-surface.md`.

## Verdict

MERGE AFTER FIXES

## Findings

1. `mockspace.toml:1191` and `:1456` [correctness]: both descriptions name `maintainer`, but `values = ["coordinators", "experts"]` was not changed. I set `stamped_by = "maintainer"` on a copy of the tree and the schema refused it: `"maintainer" is not one of ["coordinators","experts"]`. So the field advertises a value no row can carry. Fix: add `"maintainer"` to both `values` lists.

2. `mock/registry/law.toml:396`, `law::coherence_of_unsigned_saturating_addition_holds_where_the_ambient_domain_is_non_negative` [correctness]: the statement is false as written. It says coherence holds "exactly where the exact values ... are non-negative", and that "once negative values reach it ... coherence fails". On a non-positive domain the clamp sends every value to 0, so `clamp(a+b) = 0 = clamp(clamp a + clamp b)` and coherence holds. Fix: say it holds on the non-negative domain and fails where both signs reach it. The u3 row in the same commit already words this correctly and lists the non-positive domain as not run.

3. `mock/registry/proposal.toml:2443`, `wrapping_is_admitted_exactly_where_the_representable_set_is_one_progression` [correctness]: the second sentence of `says` contradicts the first.
   - It refuses every "union of magnitude ranges at more than one quantum".
   - The row's own `note`, and ticket `arvo_format_refuses_one_progression_quantum_laws_by_slope_al`, name formats at more than one quantum whose set is one progression ({0,1,2}).
   - The proof in `because` only covers a set with "a coarser range holding two members".

   As written, the stamped row backs the over-refusing slope test that the ticket calls a defect. Fix: reword as "a format whose set is not one progression".

4. Commit messages [rules; public repo, so a blocker on the checklist]:
   - `db398794` says "as the lint pack now reads it". That is false. `6eddac7` exists only on `mockspace-extra-lints` `origin/work/provisional-stamp-lint`, and the pack revision arvo builds against (`18ae1e99`) does not have this lint at all.
   - `f59ad2f5` opens "A second reading of the new rows found ...". That is the "a reviewer found" kind of workflow leakage.
   - `30ec7a08` opens "Two experts ... answered" and says "question ticket ... asks him".
   - The types `registry:` and `config:` are not in arvo's `writing-style.md` list (feat, fix, refactor, docs, test, chore).

   Fix: reword before landing.

5. `proposal::the_written_history_notes_schema_now_has_a_stamp_field` (`proposal.toml:2435`) [drift]: its `says` still lists only the coordinators and the experts as stamp routes. The rule is to fix the whole class, so update this row on the same branch.

6. Vocabulary [naming]: the new rows add spellings the rest of the registry does not use.
   - `away from zero`, where the canon spells it `away_from_zero`.
   - `saturating`, where the axis grammar uses `saturate`.
   - `nearest with ties away from zero`, which is outside the six rounding modes.

   `cargo mock rounding-vocabulary` lists these under "Outside the six". Respell the first two. The third is honest about what the instrument ran, but it is a canon question and needs a ticket.

## Verified

- `cargo mock --lint-only --commit` passes, 799 rows, schema check passed.
- I opened the probe outputs each item cites and the figures match:
  - `147/r1` and `151/v1`: half-even saturating rates 0.93 to 2.08.
  - `97/p2`: 184 cells and 0 mismatches at W 4, 5 and 6.
  - `149/y4`: the 3728, 760 and 476 figures, plus BADFOLD.
  - `122/u3`: the P1 table, and 656/968 and 4416/5840.
- The u3 mislabel defect is real: the code computes `0..=32`, and 33² = 1089.
- The y4 control C2 range of 1712 to 3280 is the `saturated` column, which is correct.
- I checked the wrapping proof (cosets, then H = nℤu, then the two gap cases) and it holds for n ≥ 3. The worked instance {0,1,2,3,4,6,8,12} is also correct.
- The canon is silent on wrapping over a union of ranges. The nearby open rows (`where_wrapping_lives`, `does_warm_wrap_or_clamp`) ask different questions.
- Tickets `29bd8e5f1b6c` and `d4e329a0446b` exist on clause-dev `main`.
- No em-dash, en-dash or bold in any added line or commit message.
- `docs/` cites these rows only by bare slug, so nothing needs regenerating.
- No open PRs on arvo.

## Notes

Running the whole suite gives two reds in crates this range does not touch, so they are on `dev` too:
- `arvo-format` `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`.
- `arvo-numeral` `a_consumer_with_no_feature_line_builds_and_runs`, which fails because `Cargo.lock` cannot be updated under `--locked`.

Ticket both if they are not ticketed already. The merge commit `3adfbbef`, which pulls the branch's own remote back into itself, adds noise; drop it if the branch gets reworded anyway.

## What was done

Findings 1, 2, 3 and 5 are fixed in `fix: let a row carry the maintainer route and narrow two claims`: `maintainer` is in both `values` lists, the coherence law says it fails where both signs reach the clamp and that the non-positive domain was not run, the wrapping refusal names a set that is not one progression, and the schema-stamp proposal names the third route. Finding 4: the five commits were replayed onto `work/arvo-registry-next` from `origin/dev` with clean messages and types from the list; the tree is identical to `work/arvo-registry-clean`'s, and `db398794`'s claim about the pack is gone. Finding 6: the overflow axis is spelled `saturate` throughout; `away from zero` is left as written, since `question::is_the_rounding_vocabulary_complete_at_six` holds that spelling open, and `nearest with ties away from zero` is ticket `f4392c6b8e76`. The second read of the `maintainer` route is with the hilavitkutin lane, which owns the pack field.
