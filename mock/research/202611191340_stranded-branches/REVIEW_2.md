# Review 2 of `work/arvo-stranded`

Range `origin/dev..50ebd78a`, merge base `3d95d776`, 21 commits, 84 files, read with pass 1 (`REVIEW_1.md`, at `ec0dc80c`) and round `202611191552-y2ye`. Nothing was committed and nothing was posted.

## Verdict

MERGE AFTER FIXES

## Gates

Canon gate: aligned on existence. `obligation::a_content_hash` (registry, inside `canon_paths = ["mock/registry/*.toml"]`) needs "a hash of content that is stable across sessions and across machines ... and whose width the consumer names rather than inherits". `Digest` with `bits::<N>()` delivers that need. No ruling names the crate, its locus or its algorithms. The only row on it is `proposal::the_old_hash_crate_returns_changed_in_slice_two`, which has `one_expert` standing and is not canon. I read the crate name and boundary as design-tier choices under that obligation. This read is not blind: the brief had me read `REVIEW_1.md` first, and it reached the same conclusion. So this is confirmation, not the second independent read. That read is still owed, on a worktree without `REVIEW_1.md` or this file. The coordinator has to arrange it, and the locus does not count as agreed until it is done.

Test gate: `cargo test --manifest-path mock/Cargo.toml -p arvo-digest -p arvo-bits` is green: arvo-bits 18 + 4 (trybuild), arvo-digest 6 + 7 + 1 + 6. I read every body in `arvo-digest/tests/` and the changed `arvo-bits/src/tests.rs`. Two things in `tests/bits.rs` fail the gate (blockers 1 and 2). One of them is pass-1 finding 4 back in a different form. The other is a pair of assertions that cannot fail.

## Pass-1 findings

1. Bench arms dropped on a misapplied quote. Addressed. Three arms are in `mock/benches/digest-crossover/arms/`, they compile (`cargo check --offline --locked` on `digest-xxh3-64-const`), and DECISIONS.md and the y2ye topic are corrected. The closed whma topic still says "four", which is acceptable because the record is immutable and y2ye corrects it.
2. Narrowing law sampled at 4 of 96 shapes. Addressed. Every N from 1 to 32 and from 1 to 64, over four inputs, with both 64-bit algorithms.
3. Mirrored-oracle sanity-buffer test. Addressed, deleted.
4. Data `assert_ne!` dressed as a planted case. Not addressed. It was deleted and then reintroduced as a `should_panic` (blocker 1).
5. `u16` widths and a second public `Width`. Addressed in code and design. The ticket `arvo_has_two_width_vocabularies_u16_for_a_digest_and_u32_for` is still open (non-blocker 6).
6. Carriers at 32 and 64 not pinned. Addressed by the type annotations. The test that does it also carries two tautologies (blocker 2).
7. `Fnv1a`/`Xxh3` unbounded, plus a trybuild case. Partly. The bound is in, but no trybuild case refuses `Fnv1a<96>` or `Xxh3<32>` (blocker 3).
8. Xor commutativity and associativity, in tests and the law list. Partly. The tests are in, and `arvo-bits/DESIGN.md.tmpl` is not touched (non-blocker 5).
9. Provenance comments and the feature gate on xxhash-rust. Partly. The path went into DECISIONS.md only. The comments at `mock/Cargo.toml:74` and `deny.toml:30` still carry no path. The gate half is dropped without a word, and the y2ye topic says finding 9 "is the note's" (blocker 4).
10. `research:` commit type. Not addressed. `ec0dc80c` is unchanged (non-blocker 7).

## Findings

1. Blocker, tests. `mock/crates/arvo-digest/tests/bits.rs:361-376` `a_narrowing_that_kept_the_high_bits_is_refused` is the test pass 1 told you to delete, turned inside out. `assert_eq!(high(whole, 16, 64), low(whole, 16), "the low 16 bits of a 64 bit digest")` under `#[should_panic(expected = "the low 16 bits")]` is `assert_ne!(high, low)` on XXH3("foobar") with extra steps. It never calls `bits`. It passes only because its own message string copies the sweep's message, so the `expected` match proves the test author can type the same words twice, not that the sweep catches anything. Its doc says "had `bits` kept the high bits, the assertion the sweep makes would have failed", and nothing in the body runs that assertion. The `planted_caught > 0` counter in `narrows_at_every_width!` (lines 280-288, 331-334, 353-356) is the real planted check, and it already covers this. Round y2ye's topic (line 22) and src changelist (lines 50-53) say finding 4's test "is deleted". That claim is true of the name and false of the test. Fix: delete lines 361-376.

2. Blocker, tests, tautological. `tests/bits.rs:400` `assert_eq!(at_64.raw(), Xxh3::<64>::of(Bytes::of(b"a")).raw().raw())` and `:401` `assert_eq!(at_128, Xxh3::<128>::of(Bytes::of(b"a")).raw())` compare a value with the same computation passed through an identity function (`raw_of`, lines 388-395). They cannot fail. They are the assertion-shaped filler this workspace deletes on sight, sitting in a test whose doc says the type annotations are the pin, which they are. Fix: delete both lines. Drop `raw_of`, since `.raw()` already returns `<Output<BITS> as Carrier>::Raw`, so `let at_64: Bits<64> = Xxh3::<64>::of(..).raw();` pins the carrier on its own. Line 399 (`0xE40C_292C`) is a real reference value and can stay.

3. Blocker, tests. `DESIGN.md.tmpl:111-112` states that `Fnv1a<BITS>` and `Xxh3<BITS>` "are bounded on `Output<BITS>: Carrier`, so neither names a width the digest has no container for". `tests/refused.rs:132-133` covers only `Digest<96>` and an over-wide `bits`. Removing the `where` clauses at `src/fnv1a.rs:30-31` and `src/xxh3.rs:243-244` would keep every test green. Pass 1 asked for this case and y2ye's topic (line 16) claims finding 7 answered. Fix: add `tests/ui/an_algorithm_at_a_width_with_no_carrier.rs` naming `Fnv1a<96>` and `Xxh3<32>`, record its stderr, and list it in `refused.rs`.

4. Blocker, rules. `.claude/rules/cargo.md:58` makes `notko` "the single permitted workspace-foundation dep", and `:68-70` say any other crate "must be behind a default-off feature gate and listed explicitly in a design round". `xxhash-rust` is unconditional at `mock/Cargo.toml:79` and in `arvo-digest/Cargo.toml`. Op's words ("start with xxhash", `.data/op-responses/202611131704_op-on-option-1-i-love-zig-as-much-as.md`) choose the dependency. They do not exempt it from the gate, so the two can both hold. Pass 1 raised this. Round y2ye took the easy half of finding 9, a path in a research note, and recorded the finding as handled (topic lines 32-33). Fix: either put `Xxh3` behind a default-off `xxh3` feature through a doc changelist, with a trybuild or cfg test for the absent case, or append a `ruling` row carrying op's quote that says xxhash-rust is a permitted unconditional dependency, so that the rule and the canon agree. Also put the op-responses path into the comments at `mock/Cargo.toml:74-78` and `deny.toml:30-32`.

5. Not a blocker, design drift. Finding 8 asked for commutativity and associativity in the law list in `mock/crates/arvo-bits/DESIGN.md.tmpl`. The range `ec0dc80c..HEAD` does not touch that file. Fix: add both laws to the list. This is a doc-only append.

6. Not a blocker, process. Ticket `arvo_has_two_width_vocabularies_u16_for_a_digest_and_u32_for` is open (`.shared/scripts/life/tickets-arvo.toml:1453`, no closed file). The work it names is done. Fix: close it with the sha `e3202031`.

7. Not a blocker, rules. The type in `ec0dc80c` `research: record what became of each stranded arvo branch` is not in the commit-style set (`writing-style.md`, Commit messages). Fix: reword it to `docs:` before the branch lands.

8. Not a blocker, unsafe. In `mock/benches/digest-crossover/arms/*/src/lib.rs:20-21`, `from_raw_parts(input_ptr, n)` and `&mut *(output_ptr as *mut [u8; 8])` carry no `// SAFETY:` comment and do no null check. Each relies on the harness passing a readable `n`-byte input and an 8-byte writable output (`bench_output_size`). 54 of the 80 arms in `mock/benches/` share the gap, so this is a class, not three files. Fix: add a SAFETY line naming the harness contract to these three, and file a ticket for the other 54.

9. Not a blocker, naming. `Output` collides with `std::process::Output`, which is why rustc now prints `arvo_digest::Output<96>` in `tests/ui/a_width_with_no_carrier.stderr:1`. Any consumer glob-importing beside std gets the same ambiguity. A name like `DigestWidth` or `OutputWidth` does not collide, and the sealed trait is already called `OutputWidth`. A rename is cheap now. It is a taste call, and I do not hold the merge on it.

10. Not a blocker, drift, uncertain. `probe.toml` (the digest-fidelity probe) records that the harness's digest column has never been populated. When fidelity validation is wired in, `digest-crossover` puts arms with different outputs (FNV-1a and XXH3) side by side against a baseline, and the validation will flag them. Check how `validate` compares arms before the bench's first run. If it compares outputs across arms, this bench needs a per-arm reference rather than the baseline's output.

## Verified

1. `Output<const BITS: u32>` runs through `digest.rs`, `fnv1a.rs`, `xxh3.rs`, `lib.rs`, the design, `mock/DESIGN.md.tmpl` and `docs/`. No `u16` or `Width<` is left in `docs/` or the digest crate.
2. The narrowing sweep covers every admitted N with real low-bit oracles. The planted high-bits counter is a genuine check.
3. The xor associativity loop runs 8^3 triples at every width of `every_width!`.
4. The bench arms differ only in algorithm, entry path and name. They take a `black_box` slice, their lockfiles follow the arm convention, and `xxh3-64-const` checks offline and locked. The root `[dispatch] points` match the bench's own, and its comment is accurate.
5. `cargo mock --lint-only` passes. With `MOCKSPACE_LINT_WARNINGS=all` there are no warnings for arvo-digest, arvo-bits or the benches.
6. There are no em-dashes or en-dashes on any added line. The check was positive-controlled. The fix range adds no bold or italic outside round metadata.
7. `docs/DIGEST.md` equals its template apart from the generated header.
8. Commit subjects are all under 72 characters, lowercase and free of parentheses, with no bylines or tool adverts in any body. The one exception is the type in `ec0dc80c`.

## Notes

The source fixes are right. Everything blocking is in the suite or the record, and three of the four blockers are cases where the round claimed a pass-1 finding was closed when it was not. Blockers 1 and 2 are deletions. Blocker 3 is one ui file. Blocker 4 is one feature or one ruling row. The second blind read of the canon locus is owed separately, and it does not block landing on the gate as written.
