# Review pass 4, the first over work/arvo-bits-third-review

Date: 2026-11-20

Reviewed: `work/arvo-bits-third-review` at `ce4be9f1`, against `origin/dev` at `3c78ee81`. Read only; nothing was built or run, as the brief asked.

Canon gate: passed. The design change about the unstable gate (`mock/DESIGN.md.tmpl:29-37`) was checked against `obligation::the_unstable_machinery_does_not_reach_a_consumer`, queried with `cargo mock query`. That row asks that a crate *naming* arvo's types need no gate. The new sentence only states that a consumer who writes `[const]` bounds of its own needs the feature for that, and no design can avoid that. The registry holds no ruling on unstable features. The locus question (topic decision 3) goes back to the coordinator as a ticket instead of being decided here, which is correct. I am the first expert to read this, so a second read is owed. I did not verify that the question ticket exists.

Test gate: no tautology in the bit tier's tests, but two gaps (findings 4 and 6). The new lint's fixtures help the implementation, and that is how finding 1 got through.

## Verdict

**MERGE AFTER FIXES.** Findings 1 to 5 block. 6 to 8 are cheap and belong in this branch, but they do not block.

## Findings

1. `mock/lints/a_name_filed_as_gone_is_exported_by_no_crate.rs:86-105` [correctness]: the parser reads past `ARVO_GONE` to the end of `supply.rs`. It stops only on a line that starts with `];`, and the rustfmt'd list in `mock/tools/the-positions/src/supply.rs:109-110` closes inline as `...];`. No line after 109 starts with `];` (the only one is line 93). So `gone()` returns 16 names: the 7 real ones, plus `Just`, `Maybe`, `Outcome`, `Boundable`, `NonZeroable`, `Lent` from `NOTKO` (line 113), plus `arvo`, `gone`, `notko` from `supplier`'s literals (lines 124-128). Any arvo crate that exports `Outcome` would then be refused as "gone". The fixture `supply()` in `tests.rs:185-195` always writes one name per line and closes with `];` on its own line, so the real shape is never tested. The control `the_reading_finds_the_list_it_judges` (`tests.rs:250-263`) only checks that `Cap` is in the list, so the extra names pass it. Fix: read the `ARVO_GONE` item from the syntax tree `view.rust()` already gives, and stop at its own `;`. Add a fixture in the real shape: one-line array, `NOTKO` after it, `Some("...")` literals. Change the control to assert that the parsed list *equals* `ARVO_GONE`.

2. `mock/lints/a_name_filed_as_gone_is_exported_by_no_crate.rs:126-132` [correctness]: `exports` only looks at lines that start with `pub`. `rustfmt.toml:31` sets `imports_layout = "HorizontalVertical"`, so a long `pub use` is split one name per line. Every root re-export of `arvo-format` is already written that way (`crates/arvo-format/src/lib.rs:63,77,93,104`). `Adapt`, `Arity`, `BinaryRationals`, `Phase` and `Neg` appear on no `pub` line, so if they were filed as gone the lint would stay silent. `arvo-bits`'s own `pub use tier::{...}` (`lib.rs:170`) is 95 columns wide. One more name and it wraps, and then the lint stops seeing the tier it was written for. That makes the claim in `supply.rs:105-108` false: it says "the list cannot keep a name after it ships again". Fix: walk the `use_declaration` and item nodes that carry a `visibility_modifier`, from the tree. Add a fixture with a vertical `pub use`.

3. `mock/tools/the-positions/src/supply.rs:84-85`, `src/tests/controls.rs:420` [correctness]: `Unsigned` was moved to `ARVO_SHIPPED`, and the comment says "arvo-numeral, and arvo-format's `slots`". `arvo-numeral` exports no `Unsigned` (`lib.rs:54`). The only one is `arvo_format::slots::Unsigned<const BITS>` (`slots.rs:279`), a slot marker that no crate root re-exports. The lint's own doc (`:26-29`) says a module path "is not one" of the names the list is about. So the branch breaks its own rule, and a new assertion pins the mistake. More generally, nothing checks `ARVO_SHIPPED` against the tree, which is the same drift as finding 4 of PASS_3 in the other direction. `the_control_the_names_arvo_does_export_are_supply` says "read off the three shipped `lib.rs` files" but reads nothing; it only compares the hand-written list with itself. Fix: put `Unsigned` back on `ARVO_GONE`, or justify it by a root export. Make the lint check both directions: every shipped name is exported by some root, and no gone name is.

4. `mock/crates/arvo-bits/tests/ui/` [tests]: the seal has four cases (two caller traits, each implemented by hand beside or without its implementor trait), and only three are pinned. `BitSequence` without `BitSequenceBy` has no case. `tier.rs:12` (`BitSequence: BitSequenceBy`) is the only thing stopping a downstream type from implementing `BitSequence` with no proof at all. If that bound were dropped, no test would go red. Fix: add `an_impl_of_the_callers_sequence_without_its_supertrait_is_refused` (`E0277`), plus the matching sentence at `crates/arvo-bits/DESIGN.md.tmpl:373`.

5. `mock/design_rounds/202611200755_*`, `202611200806_*`, `202611200811_changelist.src.md` [rules]: the round is open. The src changelist is not locked and the round is not closed. `origin/dev` holds only closed round directories and no flat files. Fix: `cargo mock lock`, then `cargo mock close`, before landing.

6. `mock/crates/arvo-bits/tests/common/mod.rs:166,179` [tests]: `rotated_up` and `written` still read a bit as `(v >> j) & 1 == 1`, the same expression as `src/ops.rs:31`. Topic decision 6 rests on the claim that these two "already" avoid it, and that claim is false. Fix the whole class: have both read through `bit_of`.

7. `mock/crates/arvo-bits/tests/the_bit_tier_in_a_constant.rs:223-226` [tests]: all eleven operations are now covered, but only at `Bits<8>` and amount 3. The `N == 64` branch of `rotate_up` (`ops.rs:142`) is never evaluated in a constant. Fix: also evaluate `every` at `Bits<1>` and `Bits<64>`.

8. `mock/crates/arvo-bits/src/tier.rs:45-46` [tests]: the doc says the proof is `Send` and `Sync` whatever `B` is, and the design repeats it (`DESIGN.md.tmpl:76`), but no test asserts it. Fix: a static assertion over a `!Send` container.

## Verified

- `Within`'s field `PhantomData<fn(B) -> B>` is invariant. The covariant field did allow wide-to-narrow. The new ui case would compile under the old field, so it can tell the two apart. A contravariant field would also refuse this direction, but narrow-to-wide is harmless.
- The sketch shows both arms. The covariant one panics at the slice index, the invariant one gives `E0308`. Its sources, its toolchain pin and its outputs are committed.
- `shifted` and `bit_of` are correct for both directions and every `j`, `k`, `n`. The eleven literals in `every_operation_of_the_tier_evaluates_in_a_constant` check out by hand.
- The new ui cases test the boundary (`128` of `128`). `a_proof_built_by_hand_is_refused.stderr` was regenerated for the new field.
- `docs/DESIGN.md` and `docs/BITS.md` move together with their templates, at `d978c989`. The root README's arvo-bits paragraph and its feature-gate claim are still accurate. No `Cargo.toml` change, no version move.
- All five commit messages are authored by op, carry no byline or advert, are under 72 characters and are lowercase imperative. `docs: land the third bits review and open its topic` adds the review file and names the artefact, so it does not narrate that a review produced the change.

## Notes

Findings 1 to 3 are one defect class. A lint was added to stop a hand-written list drifting from the tree, but it reads the list wrongly, reads the tree partly, checks one direction only, and its fixtures avoid every one of those paths. Its green test `the_shipped_tree_files_no_shipped_name_as_gone` passes today only because none of the nine names it should not have read is exported. PASS_3.md labels its findings as the job's reading, not the reviewer's words. That is honest, but it means this branch was measured against a paraphrase.
