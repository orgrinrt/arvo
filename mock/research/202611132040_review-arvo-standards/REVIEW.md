# Review of `work/arvo-standards` at `f0eeb2e3`

Base `faec24c6` (`origin/dev`). Ten commits, 27 files, about 1980 lines added.

Canon gate passed. I checked it against `ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (ratified, op): MATLAB `fi`/`fimath` and IEEE 754 are built here and SystemC, Vitis and Algorithmic C are left reserved. That matches. I also checked it against `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`. I am the first reader on canon fit, so a second independent read is still owed.

The test gate failed, and so does the verdict. The parameterisation is right. The branch turns eleven tests red in a crate its brief never ran, and it skips parity arms that its own canon row claims exist.

## Verdict

DO NOT MERGE.

## Findings

1. `mock/crates/arvo-format/src/standards/ieee754.rs:136-141` [correctness, blocking]. The `pub use ... as RoundTowardZero` re-export changes how rustc prints `arvo_format::rounding::TowardZero` in every downstream diagnostic. Because of that, `arvo-numeral`'s `tests/compile_fail.rs` goes from 18 of 18 passing on `faec24c6` to 7 passing and 11 failing on `f0eeb2e3`. Here is the diff for `clamp_is_refused`: expected `Adapt<arvo_format::rounding::TowardZero, ...>`, actual `Adapt<RoundTowardZero, ...>`. Across all eleven, `RoundTowardZero` appears 18 times. The probes are `probes/clamp_is_refused.expected.txt` and `.actual.txt`. Re-blessing the snapshots is not the fix, because then a user who never touched IEEE 754 sees an IEEE name in an `arvo-numeral` error. The fix is to name the directions in a way that does not take over the visible path, and then show the 18 still pass. The brief scoped the checks to `-p arvo-format`, so nobody ran the crate that consumes this one. Running the whole suite once would have caught it.
2. `mock/registry/obligation.toml:201` [spec, blocking]. The new `gap` text says the binary suite asserts "onto each ... under every rounding direction but ties away". That is false. Binary16 (`tests/ieee754_binary_parity.rs:296-304`) and binary64 (`:257-265`) run only ties to even, and binary16 has ten hand-picked inputs. The src changelist is accurate ("every direction on binary32"), so the error came in when it was copied into the canon. Either write the missing arms or reword the row. The arms work: `probes/spike_ieee_formats.rs` `binary64_directed` gives 0 misses under all four directions and 9 under planted `HalfUp`.
3. `tests/ieee754_decimal_parity.rs` [tests, blocking]. There is no `roundTiesToAway` arm. IEEE 754 requires a decimal implementation to provide it, and `src/standards/ieee754.rs:132-134` points at the alias. That alias can be written from the public surface: `probes/spike_ties_away_decimal.rs` gives 0 misses on decimal32 and decimal64 against an integer digit rule, and its planted control gives 1474 and 1559. Calling a direction "but ties away" without trying it is a gap left unbuilt and presented as if it were a limit.
4. `tests/ieee754_decimal_parity.rs:9` and `DESIGN.md.tmpl:1371` [tests, blocking]. Both say the expected side "reads no arvo code". In fact `rule` takes `qmin` and `qmax` from `range::<F>()` (`:153-159`, `:197`), which is arvo's own quantum law. Only the separate pin at `:216-217` would catch a shifted exponent range. Hard-code (-101, 90) and (-398, 369) in the oracle, or correct the claim in both places.
5. `src/standards/ieee754.rs:75-79` [correctness, blocking]. `Significand<A, P>` reads `A::RADIX` and ignores `A::SIGNED`. That lets `Interchange<UnsignedBinaryRationals, 24, 127>` build and pass `is_admissible_format`, and `contains` then admits slot -1 in an unsigned domain. `Binary<0, 1>` is also admitted. The probe is `probes/spike_unsigned_ambient.rs`. Constrain `A` to a signed ambient and reject `P = 0`, with a compile-fail test for each.
6. `README.md:21` [public-surface, blocking]. It still says "MATLAB is the only convention named so far", but this branch adds IEEE 754.
7. `tests/matlab_fimath_parity.rs:32-36` [tests, blocking]. The sweep is a law stated over every `W` and `F`, but it runs only at `Fi<8, 0>`, and `UfiMath` is never run through it. `probes/spike_fimath_shapes.rs` runs the same rules at 12 shapes and they all pass. Commit that matrix in place of the single sample.
8. `tests/ieee754_binary_parity.rs:318-349` [tests, non-blocking]. The test asserts that `Binary32`'s coordinates equal `Floating`'s. That is one declaration checked against another, so if both drift together it still passes. Only the `-2^128` half at `:350-355` checks anything against the host. Replace the equalities with a sweep of `Floating` through `land` against `host32`, so that exactly the `-2^128` miss shows up (the probe shows 1 miss).
9. `src/standards/ieee754.rs:41,57-73` [rules, non-blocking]. This adds more casts and bare `i128`/`u32` locals, which `type-surface.md:56-61` forbids. The lint does not see them, and trunk already does the same, but this is new code adding to it.
10. `src/standards/ieee754.rs:29-30,47-48,81-82` [naming, non-blocking]. These summary sentences run past 80 characters.
11. `202611131910_topic.the-standard-targets.md:76` [drift, non-blocking]. It says "a provisional stamp for op on a question ticket" but gives no id, so I could not check the ticket exists.

## Verified

- `p`, `emax`, `emin = 1 - emax`, base `2 - emax - p`, `2 emax` magnitudes and significand `radix^p - 1` are correct for all five aliases and for the 128-bit refusals.
- MATLAB and IEEE direction mappings are correct.
- The `land` finest-magnitude rule matches IEEE binade rounding, including carries and overflow thresholds.
- The planted arms are real. The binary32 sweep also separates wrong `P` and `EMAX` (probe: 1883, 5313, 329 and 323 misses).
- No tautological tests.
- Integration suite green. The lib red at `the_inverse_round_trip.rs:37` is known. The `arvo-numeral` consumer red also fails at the base, so it is not this branch.
- Doctests green. `cargo mock --lint-only` green.
- Regenerated docs are current: `cargo mock` in a detached worktree at `f0eeb2e3` left no diff.
- The sketch exists, runs (2 tests green) and records WORKS.
- No em-dashes and no bold were added. Commit subjects are well formed with no bylines. `registry:` is a type the rule does not list, but trunk uses it.

## Notes

The spikes are ad-hoc, in `probes/`, with their raw output in `probes/README.md`. Findings 1 to 3 are the expensive ones. Finding 1 is a defect that only shows up when the whole workspace runs, so any re-review brief needs to run the whole workspace and not only `arvo-format`.
