# Review 1 of `work/arvo-stranded`

Range `origin/dev..ec0dc80c`, merge base `3d95d776`, nine commits, 67 files. Read against arvo's registry, the round `202611191340-whma`, the branches it names in `/home/user/arvo`, and the workspace rules. Nothing was committed and nothing was posted.

## Verdict

MERGE AFTER FIXES

## Gates

Canon gate: aligned for existence, and a second read is still owed. `obligation::a_content_hash` (registry, inside `canon_paths`) asks arvo for a content hash whose width the consumer names. Op's own words cover XXH3 through xxhash-rust: `.data/op-responses/202611131704_op-on-option-1-i-love-zig-as-much-as.md`, the 17:07 BSL-1.0 answer, "Then I think we start with xxhash and backlog for later benching our own impls". No ruling covers where the crate sits or what it is called, and `mock/crates/arvo-digest/DESIGN.md.tmpl:4-5` says so itself. The one row that speaks to it is `proposal::the_old_hash_crate_returns_changed_in_slice_two`, which has `one_expert` standing and is not canon. I am the first expert to read this. The two-expert rule needs a second independent read before the locus counts as agreed.

Test gate: `cargo test --manifest-path mock/Cargo.toml -p arvo-digest -p arvo-bits` is green, 18 + 4 tests in arvo-bits and 3 + 7 + 1 + 7 in arvo-digest. I read every body. Most of the suite is real: the FNV reference vectors, every prefix checked against host arithmetic, the xxHash sanity vectors at each class boundary, runtime against const at every length up to 1100, the planted wrong vectors, and the trybuild refusals. Findings 2, 3 and 4 fail the gate, and they block.

## Findings

1. Blocker, drift and rules. `mock/research/202611191340_stranded-branches/DECISIONS.md:70-71` and `mock/design_rounds/202611191340-whma/202611191340-whma_topic.the-digest-lands-on-dev.md:32-33` leave out the branch's digest bench arms (`mock/benches/variants/digest-fnv1a64`, `digest-xxh3-64`, `digest-xxh3-64-const`: 9 files, plus its `bench.toml` and `src/main.rs` edits). The reason given is "op's 14 November call ... no deprecated bloat pre release 1". That call is about something else. `.data/op-responses/202611141041_op-on-bench-run-smoke-on-macos-aarch64-apple-m1.md:100` reads "Wasn't a helper supposed to port all old bench ones to the new updated harness? ... And if they are no longer relevant, why not delete". `mock/research/202611151844_bench-conformance/report.md` scopes it to the four variants that depended on deleted crates (ticket `delete_the_four_arvo_bench_variants_that_depend_on_the_delet`). These arms depend on `arvo-digest`, which this branch lands. They are the `digest-crossover` arms that the lib.rs FIXME and `DESIGN.md.tmpl:83` say the switched digest is waiting on. So they are relevant, and op's first instruction about such arms is to port them. "Four" is also wrong: there are three. The round took op's words from another decision and applied them so the relevant work could be dropped, and `the-record-lands-from-every-branch` ("benches (variants, ...) land ... without exception") is broken as a result. Fix: port the three arms into `mock/benches/digest-crossover/arms/` with `cargo mock bench add`, or at least land them whole as the record. Then correct the topic and DECISIONS.md: the right quote with its scope, and the count.

2. Blocker, tests. A sampled law. `mock/crates/arvo-digest/tests/bits.rs:6-19` checks the narrowing law ("keeps the low `N` bits", `DESIGN.md.tmpl:30-33`) at N = 12, 32 for `Digest<32>` and N = 28, 64 for `Digest<64>`. That is 4 of the 96 admitted shapes. Fix: sweep every N from 1 to 32 and from 1 to 64 with a `macro_rules!` list, the same way `arvo-bits`' `every_width!` sweeps its widths, over several digests.

3. Blocker, tests. A redundant test with a mirrored oracle. `tests/xxh3.rs:118-125` `the_sanity_buffer_is_the_reference_one` recomputes `sanity_buffer`'s own formula (`(PRIME32 >> 56)`, `PRIME32.wrapping_mul(PRIME64) >> 56`) and compares the two. Its doc says it stops "a wrong buffer ... agreeing with a wrong expectation". It cannot do that, because the expectation is the same formula. The xxHash vectors are the independent check on the buffer. Delete it.

4. Blocker, tests. `tests/bits.rs:21-27` `the_high_bits_are_not_what_is_kept` is an `assert_ne!` about XXH3("foobar")'s data, not about the code. Any narrowing that keeps the low bits is already pinned by `the_low_bits_are_kept`. Calling it "the planted case" is wrong, because nothing planted is shown failing. Delete it, or replace it with a real negative: a narrowing law written as a function that returns, and shown rejecting a high-bits cast.

5. Blocker, public-surface and naming. `src/digest.rs:15,52`, `src/fnv1a.rs:29` and `src/xxh3.rs:18` spell digest widths `const BITS: u16`, while `src/digest.rs:78,90` take `const N: u32` on the same type, and `Bits`, `Uint` and `arvo_format::Width` all count bits in `u32`. On top of that, `pub struct Width` (`digest.rs:15`, re-exported at `lib.rs:29`) is a second public `Width` in a crate that depends on `arvo-format`, whose `Width` is the stack's width (`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`). `README.md.tmpl:11` ("no host integer is named here") is false because of this. Ticket `arvo_has_two_width_vocabularies_u16_for_a_digest_and_u32_for` exists. Pre-release with no consumers, it costs one design line and a rename now, and more with every consumer later. Fix: amend `DESIGN.md.tmpl:21`, take `u32` throughout, rename the marker (for example `DigestWidth`) so it does not collide with `arvo_format::Width`, and close the ticket.

6. Not a blocker, tests. `DESIGN.md.tmpl:106` says "The carriers are checked at the width each names". Nothing pins `Width<32>::Raw = Bits<32>` or `Width<64>::Raw = Bits<64>`. Every test would still pass if both were `Bits<64>`. Only the 128-bit carrier is pinned, by `joined`'s signature. Fix: add `let _: Bits<32> = Fnv1a::<32>::of(..).raw();` and the same at 64.

7. Not a blocker, drift. `Fnv1a<const BITS: u16>` and `Xxh3<const BITS: u16>` have no bound, so `Fnv1a<96>` and `Xxh3<32>` can be named as empty types. Fix: bound them on `Width<BITS>: Carrier` and add a trybuild case.

8. Not a blocker, tests. The xor laws (`arvo-bits/src/tests.rs:209-235`) leave out commutativity and associativity. Add them to the same every-width sweep, and add them to the law list in `arvo-bits/DESIGN.md.tmpl`.

9. Not a blocker, rules. The `mock/Cargo.toml:74` and `deny.toml:30` comments cite op's call with no path. Cite `.data/op-responses/202611131704_op-on-option-1-i-love-zig-as-much-as.md`. The generated `.claude/rules/cargo.md` says a non-notko crate goes behind a default-off feature. xxhash-rust is not behind one. Either gate `Xxh3`, or record op's call as a `ruling` row so the rule and the canon agree.

10. Not a blocker, rules. `ec0dc80c` uses type `research:`, which is not in `.claude/skills/commit-style/SKILL.md` (`feat`, `fix`, `refactor`, `docs`, `test`, `chore`). dev carries 440 of them. Reword to `docs:` before landing, or file the type into the commit-style config.

## Verified

1. The five `land` rounds are byte-identical to `origin/work/arvo-format`, and every record path that branch added is on this branch.
2. `arvo-bits/src` is identical to `3d1a3950`. `arvo-digest` differs only by `Fnv1a<64>`, the macro walk, the FIXME, and the matching design and test lines.
3. The FNV-1a 64 constants match the reference. Both widths are checked against host arithmetic over 513 prefixes.
4. The FIXME ticket and all six other tickets that DECISIONS.md names exist in `.shared/scripts/life/tickets-arvo.toml`.
5. The superseded claims hold by blob ancestry for `canon-double-check`, `registry-repairs`, `dev-gate-green`, `arvo-registry-clean` and `numeral-integers`: 0 novel blobs each. `commit-subject-config` has exactly the one novel file it names.
6. `cargo mock --lint-only` passes with no warnings for the touched crates. No em-dashes or en-dashes in any added or modified file (the check was positive-controlled). Bold appears only in round metadata.
7. `docs/DIGEST.md` and `docs/BITS.md` match their templates apart from the generated header. README and STRUCTURE name the new crate.
8. There is no `unsafe`, no `alloc`, no `dyn`, and no runtime check. `Bytes::of` and `Bits::halves` take host types inside the introducing crate's declared door (`mockspace.toml` `[primitive-introductions]`).

## Notes

The source is small and mostly sound. The blockers are about the record and the suite, not the hashing. Finding 1 matters most. Op's words were used outside their scope, which dropped the exact arms the crate's own FIXME is waiting on. Fixing it also unblocks the switched digest. Once the arms are ported and run, that work is a harness run rather than a new task.
