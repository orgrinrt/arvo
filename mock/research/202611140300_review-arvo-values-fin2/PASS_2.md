# work/arvo-values, second reading after round 202611140253

Read at `a24b331b` against `dev` at `3865b39a`, read-only, in `/home/user/.worktrees/arvo-values-review2`. The worktree was clean when I finished.

## Canon gate

It passes provisionally, and the same condition as in the first pass still holds. I checked it against these rows:

- `ruling::the_panel_finishes_the_canon_without_him` (ratified by op). It gives every remaining question to the panel, "put through two independent agreements".
- `question::whose_reduction_governs_a_lossy_crossing`. Its decider is the panel, it has no `answered` field, and it is open.
- `proposal::a_conversion_is_one_placement_onto_the_target_grid`, which carries the experts' stamp and now `answers` Q36.
- `ruling::the_operating_constraints_are_intents_and_rules` (in force).

I formed my reading from the rows before opening PASS_1, so I am the second reader of this gate. The design does not fill Q36 itself. It builds the target's option on the stamp and says so (`DESIGN.md.tmpl:30-33`, `:344-351`). Whether that stamp holds is ticket `6a490295809d`, and that call is not mine. What I see for that ticket:

- The convert code landed at `275b7038` (2026-11-13 20:54). The readings landed at `a8c65b64` (2026-11-14 01:14). The stamp therefore ratified an answer the tree had already shipped.
- `SEAT_1.md:109` says so itself: "A design answered an open canon question by shipping".
- `SEAT_2.md:73,99` reason partly from `ratio.rs` and from `p2_shipped`.
- The row's own `standing` is still `one_expert`, while its `stamp` says "Two experts".
- The stamp says "Their probes cover...". Seat 1's model is uncommitted Python, so under the evidence rule its counts are void. Only seat 2's probes are evidence.

If the stamp fails, the conversion's place on this branch fails with it. Nothing else in the diff decides a canon question.

## Suite audit

It passes for this pass's scope.

- I ran `cargo test -p arvo-numeral --no-fail-fast` over all 44 targets: 286 tests passed, 0 failed.
- I read the bodies in `common/edges.rs`, `common/exponents.rs`, `common/crossings.rs` and `common/shapes.rs`, the product and quotient families, the edge tests, and the two source-signature and three positive-edge binaries.
- The edges are computed in the tests' own arithmetic and held against the format's verdict on both sides of each edge. This happens at every width, both ranges and three radixes.
- `source_signatures!` asserts `unfit > 0 == leaves`, so the sweep proves that it consults the policy. I found no tautologies.
- The suite is still sampled where `DESIGN.md.tmpl:677-678` says it is (N7, ticketed).
- `arvo-format`'s `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63` is still red at `a24b331b`. It panics at `the_inverse_round_trip.rs:37:48`.

## Verdict

Merge after fixes.

## Blockers

1. `docs/NUMERAL.md:66`, `:578` [public-surface]
   - The generated documents were last rendered at `fc087e6c`. `DESIGN.md.tmpl` has changed twice since, at `a8c65b64` and `d61e2b2a`.
   - The published page still says the mutants are cargo features. It also lacks the disclosure that the conversion rests on open Q36 (template `:30`, `:344`) and the five-sweep paragraph.
   - This was already stale at `e19aa3dd`, and PASS_1's "prose is current" missed it.
   - Fix: run `cargo mock`, then commit `docs/`.

2. Commit text [rules]
   - These commits leak workflow into a public repository:
     - `ff8aeffa` and `8e46316d`: "by a fresh seat".
     - `2b413b50`: "The third fresh seat refuses the branch".
     - `1f27fce0`: "each point of the first reading".
   - `e33c0c13`, `9a3d5149` and `ae4efecb` keep git's default merge subject.
   - These become permanent once the branch lands. Fix: reword them before landing. Ticket `ffa4ce633701`.

## Non-blocking

1. `DESIGN.md.tmpl:675` [tests]: the line "it passes every other sweep" goes beyond the committed run, whose predicate covers three binaries. I ran `arvo_mutant_convert_policy` against `the_conversion`, `the_crossing` and `the_conversion_at_every_width_binary`, and all three pass. That is a reviewer's check, not a record. Fix: add those binaries to `run.sh`'s policy line and commit the output.
2. `the_conversion_under_every_source_signature.rs:5-8` [tests]: the claim that reading the source's mode would disagree has no mutant behind it. By the arithmetic it holds, since radix two at minus one onto zero has ties and the 10 by 10 pairs include Floor against Ceil, but no mutant shows it failing. Fix: commit a mode mutant and its red run.
3. `DESIGN.md.tmpl:672` [naming]: "on either end" is false for an unsigned range, where only the top end leaves. Fix the wording.
4. `src/machine.rs:24` [rules]: `const SIGNED: bool` is a bare `bool`, and it can be read through any `H: HostInteger`. This goes against `ruling::the_operating_constraints_are_intents_and_rules`. Fix: use `Bool`.
5. `DESIGN.md.tmpl` is 785 lines; it was 365 on dev [rules]. The proof section is the seam to split along, into a `DEEPDIVE_*.md.tmpl`.
6. `registry/proposal.toml` [canon]: `an_order_is_named_exactly_where_a_crossing_is_lossy` still `says` "An order must be named". Only the `note` reconciles that with the stamp, under which the target signature fixes the order everywhere.

## PASS_1 findings

- B1: resolved. The sweep now crosses zero onto minus one, asserts that members leave the range, and its mutant red run is committed. Residue is in non-blocking 1 and 2.
- B2: resolved. The edges are computed and held at both ends for all three operations.
- B3: resolved. The mutants are `arvo_mutant_*` cfg flags declared to `check-cfg`, and no features are left.
- N1: left, ticket `6a490295809d`. Observations are in the canon gate above.
- N2: partly resolved. `answers` is added and the note and gap are reworded. The `says` remains (non-blocking 6).
- N3: left. The stamp text still leans on seat 1.
- N4: left. Reproduced red; the record is still dropped. Ticket `7c3655630b91`.
- N5: resolved. `num.rs` is 443 lines after the move to `order.rs`.
- N6: left, and raised here to blocker 2. Ticket `ffa4ce633701`.
- N7: left, ticket `1d26aa3e8658`.
- N8: left, at `DESIGN.md.tmpl:760`. Ticket `5cdda7b81725`.
- N9: not applicable. This was a fresh worktree, left clean.

## Verified

- `cargo mock --lint-only --strict`: all lints pass.
- No em-dashes or en-dashes on added lines, grep checked against a positive control.
- `copy` carries a SAFETY comment and its invariants hold for every `Machine` pair.
- No open PRs on `orgrinrt/arvo`, and no version or toolchain moved.
- The rest of the `Cargo.toml` changes are in line: the cfg declarations and the new `notko-hlist` dependency, which matches the existing `notko` spelling.
