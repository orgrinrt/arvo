# Review 3 of `work/arvo-stranded`

Range `origin/dev..c7730375`, merge base `3d95d776`, 18 commits, 95 files. Pass 3, the last. Nothing was committed and nothing was posted.

## Canon reading, formed before opening `REVIEW_1.md` or `REVIEW_2.md`

The canon is `mock/registry/*.toml` (`canon_paths` in `mockspace.toml`). Rows that bear on whether `arvo-digest` exists and where it sits:

1. `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`, ratified, `ratified_by = "op"` by default from his written history: "Every family the old arvo carried returns in function and in feature. The shape the contracts, the traits and the default implementations take is open to a whole rewrite, and so is the taxonomy." Its note: "the function returns, the crate name and the home do not survive by default". The old tree carried a hash family, so a digest in arvo is licensed by a ratified ruling, and its crate name and home are design choices, not a reserved question (no `rung = "open"` row or `question` row covers them).
2. `ruling::arvo_is_a_library_and_the_value_composes_on_top`, ratified by op: algorithm crates are arvo's "main selling point". An algorithm crate above the value tier is in arvo's own ground.
3. `obligation::a_content_hash`: stable across sessions and machines, width named by the consumer. `Digest<BITS>` with `bits::<N>()` serves it.
4. Op's 13 November call, clause-dev `.data/op-responses/202611131704_op-on-option-1-i-love-zig-as-much-as.md`, the second BSL-1.0 question, options [Allow BSL-1.0: arvo takes xxhash-rust] [No, use twox-hash] [No, arvo or zig writes its own]: "Then I think we start with xxhash and backlog for later benching our own impls". This is his call in the written history, so it is ratified by default under `ruling::what_the_written_history_shows_to_be_his_is_ratified_by_default`, but no registry row carries it.
5. `proposal::the_old_hash_crate_returns_changed_in_slice_two` (`one_expert`) and `proposal::the_first_slice_answers_to_hilavitkutin_and_kaski` (`one_expert`, content hash "is slice two's") are proposals. They bind nothing. Landing the digest now contradicts no ratified row.
6. The `in_force` constraints (no_std, no alloc, no dyn, const sizes, no runtime checks, the stack's own primitives) hold in the crate as written. `Bytes::of(&[u8])`, `Bytes::host` and `Bits::halves(u128)` sit inside `arvo-bits`' declared numeric door (`mockspace.toml` `[primitive-introductions]`).

Canon gate: aligned. The crate should exist, and a separate crate above `arvo-bits` and `arvo-numeral` is a design choice the ratified families ruling leaves open. On reconciling after this reading: pass 1 reached the same verdict on existence independently, and pass 2 read pass 1 first. So pass 1 and this pass are the two independent reads, and existence is agreed by two. Both earlier passes say no ruling covers the crate. That is wrong, and the design says the same wrong thing (finding 2).

Test gate: `cargo test --manifest-path mock/Cargo.toml -p arvo-digest -p arvo-bits` is green: arvo-bits 18 + 4 trybuild, arvo-digest 3 + 7 + 1 + 6. `cargo check ... -p arvo-digest --no-default-features` is green. I read every test body in `arvo-digest/tests/`, `arvo-bits/src/bytes/tests.rs` and the added part of `arvo-bits/src/tests.rs`. FNV-1a reference values are correct, the prefix sweep against host arithmetic covers every byte value (stride 7 is coprime to 256), the narrowing sweep has an independent oracle and a real planted counter, and the ui cases fail for the stated reason. One sampled law remains (finding 1).

## Verdict

MERGE AFTER FIXES

## Findings

1. Blocker, tests, sampled law. `mock/crates/arvo-digest/tests/xxh3.rs:137-151` and `:108-116` check that runtime and const XXH3 agree, and that both match xxHash's vectors, in one target-feature configuration only. xxhash-rust 0.8.21 (`src/xxh3.rs:16-55`) picks one of five accumulate paths by `cfg(target_feature)`: scalar, SSE2/NEON/wasm simd128, AVX2, AVX-512. The suite runs the x86_64 baseline, which is SSE2. The design states stability "on every machine" (`DESIGN.md.tmpl:9-11`) and that runtime equals const (`:66-70`). That cross-machine stability is the fundamental `obligation::a_content_hash` asks for. This host has `avx2` and `avx512f`, so two more paths need only `RUSTFLAGS="-C target-feature=+avx2"` and `+avx512f`. Fix: run the XXH3 suite under each of those and record them in `mock/test-history/` (or as a `cargo mock test` configuration). Until NEON, wasm simd128 and scalar have been run, state them in the design as untested.
2. Blocker, spec, canon declaration. `mock/crates/arvo-digest/DESIGN.md.tmpl:3-5` says "No ruling places a digest crate in arvo, names it or picks its algorithms." `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (ratified) places the hash family in arvo and leaves the name and home open. Op's 13 November call picks xxhash-rust. The design declares only the obligation, and reaches op's call through a `.data` path in `mock/Cargo.toml:55-61` and `deny.toml:29-33` instead of the canon. Fix: declare the families ruling in the design's opening line and correct the sentence. Append a `ruling` row carrying op's quote and the three options, ratified by default. This is additive, so it nukes nothing. Then cite that row from the design and the two comments.
3. Tests, non-blocker. `DESIGN.md.tmpl:115` says "`Xxh3<32>` is a type ... and has no `of`". Nothing pins that, and nothing pins `Fnv1a<128>` having no `of` either. Fix: add ui cases calling `Xxh3::<32>::of` and `Fnv1a::<128>::of`. The better shape is a sealed width trait per algorithm, so these inert types cannot be named at all. That is a design append.
4. Tests and drift, non-blocker. The dev-dependency on itself with `xxh3` (`arvo-digest/Cargo.toml:23-25`) means no test ever builds the crate without the feature. So the feature-off build and the absence of `Xxh3` are shown by nothing in the suite. Pass 2 asked for "a trybuild or cfg test for the absent case". Fix: put `cargo check -p arvo-digest --no-default-features` into the gate. Separately, `DESIGN.md.tmpl:78-81` makes the switched digest "the stack's digest", and it needs XXH3, which `:72-73` puts behind an off-by-default feature. The design has to say which of the two gives way before that digest is built. Add it to the switched digest's FIXME ticket.
5. Tests, non-blocker. `arvo-bits/src/tests.rs:220` (`x.xor(y).raw() == x.raw() ^ y.raw()`) restates `lib.rs:141`'s body, and `arvo-bits/src/bytes/tests.rs:63-64` restate `halves` at `lib.rs:155`. Neither line can fail independently of its implementation. The laws beside them are real. Fix: delete these lines, or replace each with a per-bit oracle that loops over the positions.
6. Unsafe, non-blocker. Pass-2 finding 8 is unchanged. `mock/benches/digest-crossover/arms/*/src/lib.rs:20-21` carry no `// SAFETY:` naming the harness contract (readable `n` bytes in, `bench_output_size` bytes out) and no null check. Fix: add the line to these three, and file a ticket for the rest of the class.
7. Drift, non-blocker. `DECISIONS.md:77-78` says ticket `arvo_has_two_width_vocabularies_u16_for_a_digest_and_u32_for` "stays open on what landed". `e3202031` did that work, and the ticket is still open (no file under `.shared/tickets/closed/`). Fix: close it with `e3202031` and append a correction to DECISIONS.md.
8. Public surface, non-blocker. `README.md:10` and the `mock/DESIGN.md.tmpl:51` table present XXH3 and `xxhash-rust` without saying they are behind the default-off `xxh3` feature. `deny.toml:29-33`, a public root file, names a clause-dev path. Fix: say "behind the `xxh3` feature" in both places. In `deny.toml`, point at the ruling row from finding 2 instead of the path.
9. Public surface, non-blocker, generator. `docs/STRUCTURE.md` lists only `Bits<const N: u32>` as `arvo-bits`' public API and nothing for `arvo-digest`, so `Bytes`, `Digest`, `Fnv1a` and `Xxh3`, all re-exported through `pub use`, are missing. `docs/DIGEST.md` and `docs/BITS.md` equal their templates apart from the header. Fix: file a mockspace ticket against the structure generator. This does not hold the branch.

## Earlier findings

Pass 1:

1. Bench arms dropped on a misapplied quote: addressed (three arms ported, DECISIONS corrected).
2. Narrowing sampled: addressed.
3. Mirrored sanity-buffer test: addressed, deleted.
4. Data `assert_ne!` as a planted case: addressed in le48 (the test is gone, and the sweep's counter stands).
5. `u16` widths and a second `Width`: addressed in code. The ticket is open (finding 7).
6. Carriers unpinned: addressed.
7. Unbounded `Fnv1a`/`Xxh3`: partly (the bound and the `<96>` cases are in, and the inert `Xxh3<32>`/`Fnv1a<128>` are unpinned, finding 3).
8. Xor laws: addressed, in the tests and in `arvo-bits/DESIGN.md.tmpl`.
9. Provenance and feature gate: addressed (feature in, paths cited). The citation should move to a registry row (finding 2).
10. `research:` type on `ec0dc80c`: not addressed, declined in le48. Dev already carries the type, so I do not hold the merge on it. File it into the commit-style config.

Pass 2:

1. High-bits data test: addressed.
2. Tautological carrier assertions: addressed.
3. Algorithm-width compile-fail cases: partly (finding 3).
4. Default-off feature and comment paths: partly. The gate and the paths are in, and the absent-case test is not (finding 4).
5. Xor laws in the design: addressed.
6. Width ticket: not addressed (finding 7).
7. Commit type: not addressed (as pass 1, 10).
8. SAFETY in the arms: not addressed (finding 6).
9. `Output` name: not addressed, declined as taste. I agree it does not hold the merge.
10. Fidelity validation across arms of different outputs: not addressed, nothing in the tree changed. Still owed before the bench's first run.

## Verified

1. The five `land` rounds (`sknb`, `wqfi`, `dp7c`, `ibrf`, `202611140055`) are byte-identical to `origin/work/arvo-format`.
2. Every commit DECISIONS.md names is on `origin/dev`. For `arvo-values-next`, `arvo-format-base`, `arvo-format-slot`, `oracle-sub-neg` and `arvo-standards`, no path the branch changed differs between its tip and the `dev` commit named for it (positive control: 69 paths differ against the merge base).
3. Commit subjects are under 72 characters and lowercase, with no bylines in the bodies. No em-dash or en-dash appears on any added line (the check was positive-controlled).
4. The code says nothing the design does not: the macro walk over `Num::mul` and `Bits::xor`, the `const` assert in `Digest<32>::bits`, and the feature-gated module all match their design lines.

## Notes

The source is small and correct. Finding 1 is the one that matters: the stability across machines is the property the crate exists for, and it is checked on one of five SIMD paths when two more are a flag away on this host. Finding 2 is a doc append and one registry row. This is the last pass, so whatever is left at landing goes on as follow-up tickets.
