# Review pass 3, arvo-bits-tier

Date: 2026-11-20
Reviewed: `work/arvo-bits-tier` at `1fd4d78e`, by a fresh `pr-reviewer-senior`, the third and last pass.

The reviewer's own file was written and committed on the branch, and was lost unpushed when the job's container was rebuilt at about 06:57 UTC, with the round that followed it (ticket `a_helper_container_rebuilt_mid_run_loses_every_unpushed_comm`). What stands here is what survived: the verdict, taken word for word from a read of that file before the loss, and the findings as the round's topic, `202611200755_topic.the-proof-is-invariant.md`, records them, which is the job's reading of them and not the reviewer's words.

## Verdict

MERGE AFTER FIXES. Findings 1 to 4 are to be fixed before landing; 5 to 10 can be follow-ups.

Canon gate passed, against the five rows the crate design declares and `obligation::the_unstable_machinery_does_not_reach_a_consumer` and `obligation::the_surface_expressible_as_contracts_before_anything_implements_it`, each queried with `cargo mock query`. Nothing contradicts a ratified row; one locus question is open (finding 3).

## Findings, as the topic records them

1. The root design says the contracts a numeral satisfies take `const_trait_impl` by default in `arvo-contracts` and `arvo-numeral` and that no other crate enables an unstable feature by default, while `arvo-bits` enables it for its const traits.
2. The proof `Within<B, I>` holds `PhantomData<B>` and so is covariant in its container: two containers related by subtyping let a proof made for a wide one coerce to a narrow one and index it past its width.
3. `dev` gained `arvo-contracts` under the branch, and whether the bit tier's contracts belong beside the numeral's there is a locus question two reads have to settle.
4. `mock/tools/the-positions` still files `Bits` and `BitAccess` as names arvo no longer exports, and a passing test asserts it.
5. A container of its own is held for access alone: no refusal case shifts one past its width, implements the caller's sequence by hand, or implements a caller trait without its supertrait.
6. The oracles of `tests/the_bit_access.rs` and `tests/the_bit_sequence.rs` restate the expressions `src/ops.rs` computes.
7. The constant test evaluates four of the eleven operations through `[const]` bounds.
8. The disassembly was taken before the operations gained their mutant branches.
9. The design names no owner for a set wider than sixty-four bits.
10. Three pushed subjects describe the process rather than the change.
