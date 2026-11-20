# Review pass 2, arvo-bits-tier

Date: 2026-11-20
Reviewed: `work/arvo-bits-tier` at `586bc48d` (and `74a7bd9b`, a test-history record only) against `origin/dev` at `d15dd473`, by a fresh `pr-reviewer-senior`, briefed to answer the placement question before reading pass 1 or the topic's placement section. The findings are taken up by the round that follows this file on the branch.

## Verdict

MERGE AFTER FIXES. The operations are correct at every admitted width. Findings 1 to 3 are to be fixed before landing.

Canon gate passed, against the five rows the crate design declares and `obligation::a_primitive_for_every_position_a_bare_number_would_take`, with `proposal::the_old_bit_contracts_crate_is_gone`, `proposal::the_old_bitmask_crate_returns_changed_in_slice_two` and `proposal::the_old_storage_crate_is_gone` read and none binding. Test gate fails on one sampled law (finding 1). `cargo mock test`: 10 trees green, `arvo-bits` reran 6 of 6.

## Placement, answered before reading pass 1

Consistent. `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (ratified by op) returns the bit contracts in function and leaves their shape and taxonomy to the design; no `open` row or question reserves where they live. `Bool` and `Width` are forced by `obligation::a_primitive_for_every_position_a_bare_number_would_take`, and their one owner is `arvo-format` (`crates/arvo-format/src/width.rs:35,102`); a second copy in `arvo-bits` would be a mirror. `arvo-format` takes only notko, so there is no cycle. The traits are the bit container's contract and `arvo-format` is not a container, so `arvo-bits` is the crate.

Two things point elsewhere and neither binds: the proposal's `because` says the traits "return with the masks over `arvo-bits`", and `proposal::the_old_storage_crate_is_gone` makes `Bool` a value alias, which if stamped would move `Bool` into `arvo-numeral` and pull `arvo-bits` onto the value crate.

This agrees with pass 1 and the topic on placement, so it is the second independent read. It disagrees with the premise of topic decision 7 (finding 2).

## Findings

1. `tests/compile_fail.rs:20-46`, `tests/ui/*`, tests, high, blocking. All 19 index and amount refusals are at `Bits<8>`, index 8. Nothing pins `N = 1` at 1, `N = 64` at 64 (where the shift overflow lives), or an index past the width; a mutant `I != N` passes the whole suite, and `with_bit::<9>` on `Bits<8>` would then set a bit above the width. The shipped code already refuses all nine edge cases with E0080: `Bits<1>` `bit` and `with_bit` at 1, `Bits<64>` at 64 for `bit`, `with_bit`, both shifts and both rotations, `bit::<{u32::MAX}>`, `shift_down::<9>`. Fix: add them as cases.

2. `src/tier.rs:15-17,34-36,70-71`, `DESIGN.md.tmpl:141-145`, topic decision 7, drift and correctness, high. "No trait can enforce that" is false. A spike enforces the refusal at the trait: the required method takes a token `At<Self, I>` whose `NEW` asserts `I < B::WIDTH.count()`. A 256-bit container with no assertion of its own is refused at index 256 at build, through a `[const]` bound in a constant too, and the token cannot be had past the width. As shipped every implementor writes its own guard, a copy per container, and one that forgets turns `self.0[I / 64]` into a run-time bounds check, against `ruling::never_a_runtime_check_and_one_lowered_path`; the 256-bit set of `arvo_bits_sets_wider_than_sixty_four_bits` is the next implementor. Fix: a round where the trait owns the guard once, the token plus a blanket-implemented trait so the guard cannot be overridden; commit the spike first. The spike is one instance and establishes only that the shape exists.

3. `src/ops.rs:24-25`, `DESIGN.md.tmpl:263-268`, tests, medium. The comment says the compile_fail cases "have to catch" `arvo_mutant_bits_index`. trybuild removes `RUSTFLAGS` (`trybuild-1.0.120/src/cargo.rs:47`), so they never can; only the sketch's example builds see it. Fix: say so in both places and cite `trybuild_drops_rustflags_so_a_cfg_mutant_never_reaches_a_com`.

4. Tests, medium. No mutant is shown failing for `shift_down`, `rotate_down`, `bit`, `and`, `or`, `xor` or `is_empty`. Fix: add those mutants to the red run.

5. Tests, low. `the_bit_logic.rs:26,28,35-49,62-84`, `the_bit_access.rs:46,49-53`, `the_bit_sequence.rs:61,71-85`: over the same inputs each follows from the equality against the oracle and none can fail first. Fix: delete them, or have the design say these laws follow from the oracle checks.

6. `DESIGN.md.tmpl:147-150`, spec, low. "On the 64-bit carrier ... one instruction each" was measured at `N = 64` only, and union is `mov; or`. Fix: the note's predicate, with the toolchain pin and the target features. The disassembly was taken at `a69939c8`; the changelist declares it, and the arm has changed since only in a comment.

7. `docs/STRUCTURE.md:66`, public surface, medium. Lists only `Bits`; the four traits and `Bytes` are missing because the generator does not list re-exports. Ticketed as `arvo_docs_structure_md_lists_none_of_a_crate_s_re_exported_p` and not fixable on this branch. Whether it blocks is the lander's call.

8. `README.md:10`, `mock/DESIGN.md.tmpl:39`, drift, low. Both say `arvo-bits` sits beside `arvo-format` while depending on it, and `STRUCTURE.md` puts it at depth 1. Fix: above `arvo-format`, beside `arvo-placement`.

9. Style, low. Bold at `mock/DESIGN.md.tmpl:121-122`, a file this branch edits. `crates/arvo-bits/README.md.tmpl` is 20 lines against the rule's 3 to 10.

10. `202611200329_changelist.src.lock.md:4`, drift, low. Names "job `arvo-numeral-doors`" for a round that overhauls `arvo-bits`.

11. Public surface, low. 45453c9d, 70e68022 and 237baaff name "the first review", which is how the change was made rather than what changed. No bylines or tool adverts in any of the 15 commits.

## Pass 1, finding by finding

1 to 6, 8 and 9 addressed (3 with the cases at one shape, finding 1; 8 apart from finding 3). 7 addressed as asked, but its premise is false (finding 2). 10 addressed except the README length.

## Verified

Every operation correct at `N = 1` and `N = 64`, at index or amount 0 and `N - 1`. The design matches the source otherwise, and the source has nothing the design does not say. The mutant outputs match `FINDINGS.md`. `Cargo.toml` and the lockfiles are consistent; the digest baseline rename is declared in the first round's changelist. `docs/BITS.md` matches its template. `const_trait_impl` is on the watch tier, so vetted.
