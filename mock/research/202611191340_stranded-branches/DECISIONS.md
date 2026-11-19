# What became of the stranded arvo branches

On 19 November 2026 every SOURCE and MIXED branch of arvo that clause-dev
`.shared/research/202611191130_stack-unmerged-branches/README.md` lists was decided against `dev` at
`3d95d776`. The branches had been rewritten before landing, so none is an ancestor of another or of
`dev`, and the commit counts ahead say nothing. The decision for each was taken path by path instead:
every path a branch changed since its merge base and still differing on `dev` was asked whether the
branch's blob ever existed in `dev`'s history since 25 October. A blob that did is a state `dev` passed
through and moved on from; one that did not is content `dev` never had. Then each branch's subjects were
compared against `dev`'s and against `work/arvo-values-next`, which `dev` took whole as `d0161800`.

The BENCH branches are left alone, as the brief says.

## The decisions

| Subject | Branch | Decision | Where `dev` has it |
|---|---|---|---|
| values | `work/arvo-values` | superseded | `d0161800`. Every subject but the three fresh-seat readings is in `work/arvo-values-next`, and those readings are on `dev` under `mock/research/202611132145_review-arvo-values/` |
| values | `work/arvo-values-fin-unpushed` | superseded | `d0161800`; its one later commit's red runs are on `dev` under `mock/research/sketches/202611140014_value-mutants/`, its round closed as `mock/design_rounds/202611140020/` |
| values | `work/arvo-values-next` | landed | `d0161800`, one path differing, a generated one |
| values | `work/numeral-integers` | superseded | `9dfd5339` |
| format | `work/arvo-format-base` | landed | `ccb4c239` |
| format | `work/arvo-format-slot` | landed | `280e3a2a` |
| format | `work/arvo-format-member` | landed | `b687fc01` |
| format | `work/arvo-standards` | landed | `085033a4` |
| format | `work/oracle-sub-neg` | landed | `7ec50792` |
| format | `work/oracle-fraction-wide` | landed | `2c8e5343` |
| format | `work/arvo-format` | record landed, source retired pending a question | the cleared rounds through `ccb4c239`; the five `land` rounds as record in `d1e2b9b7` |
| digest | `work/arvo-digest` | landed, consolidated | round `202611191340-whma` on `work/arvo-stranded` |
| registry | `work/arvo-registry`, `-registry-clean`, `-registry-next` | superseded | `9ba397c4` and the registry rows after it; every blob of `-clean` and `-next` is in `dev`'s history, and `work/arvo-registry`'s three differing files are its states before `-clean` undid the edits to ratified rows |
| registry | `salvage/arvo-registry-clean-20261114T093640Z` | superseded | a working-tree snapshot of `-clean`; against `dev` it only lacks rows `dev` added since |
| registry | `work/canon-double-check`, `work/registry-repairs`, `work/canon-value-stance`, `work/claim-the-value-rows` | superseded | every differing blob is in `dev`'s history |
| gate | `work/dev-gate-green`, `work/mockspace-lint-one-pass`, `work/principles-catch-up` | superseded | every differing blob is in `dev`'s history |
| gate | `work/commit-subject-config` | held on mockspace | `80e5afac` configures the `commit-subject` gate. The one differing file is the commit-style skill template, which reads `{{commit_rules}}`, `{{commit_types}}` and `{{commit_examples}}` from that config; the mockspace `dev` arvo runs renders none of them (tried on `work/arvo-stranded`, the placeholders came out verbatim), and they come from `hiisi-digital/mockspace` branch `work/commit-type-gates` (`397162b`), unlanded. The template lands after that branch does |

## The value tier and its two blockers

`job-arvo-values-fin.md` left two blockers on `work/arvo-values`. Both are answered on `dev` by
`d0161800`. The conversion builds the target's reduction, the option two seats picked one after the
other (`mock/research/202611140010_q36-two-readings/`), carried as a provisional stamp on
`proposal::a_conversion_is_one_placement_onto_the_target_grid` with question ticket
`does_the_target_s_reduction_govern_a_lossy_conversion_in_arv` to op; the seats' neutrality is
question `the_landing_oracle_shares_land_s_four_limb_limits`, held by coord. The quotient, product and
conversion run at every width 1 to 64 at radixes two, ten and three, and the review of that work
(`mock/research/202611140300_review-arvo-values-fin2/PASS_3.md`) ends at merge after fixes. Nothing
of the values chain is left off `dev`.

## The format branch and `land`

`work/arvo-format` is the one branch whose source stays off `dev` without being superseded.
`work/arvo-format-base` took every round its third review cleared (`li8r`, `ouou`, `aun1`, `jb7q`,
`c27b`, `c5qc`, the IEEE round redone as `202611140156`) and the wrapping round `202611140127` with
its fix in `202611140209`. What it left is `land`, `apply/landing.rs`, in rounds `sknb`, `wqfi`, `dp7c`,
`ibrf` and `202611140055`: the review reads round `202611140055` as do not merge, and reads `land`'s
run-time `Isnt` below the four-limb remainder as misaligned with
`ruling::the_format_spine_is_canon` and `ruling::never_a_runtime_check_and_one_lowered_path`, on one
seat. A blocker a review left is not shipped, so the source does not land, and the five rounds land as
the record. Whether `land` comes back, and in which form, is question ticket `does_arvo_format_s_land_come_back_and_in_which_form`, held by op,
beside tickets `land_lands_nothing_below_the_four_limb_remainder_ieee_underf`
and `a_helper_cannot_push_a_round_end_tag`, whose note is the C1 panic.

## The digest

`work/arvo-digest` was cut from `work/arvo-values` before that branch's last rounds, so it carried an
older value tier beside the digest. Its two rounds were on `dev` already as the record (`3d95d776`).
Round `202611191340-whma` takes the digest's own changes, everything the branch changed after it
merged `c97a1af5`, onto `dev`, adds `Fnv1a<64>` now that the 64-bit wrapping product is admitted, and
leaves the switched digest under a FIXME naming ticket
`the_switched_64_bit_digest_and_its_crossover_bench_in_the_ha`, since its crossover is taken from a
bench run that has not been made. The branch's three `digest-crossover` arms, in the retired
`variants/` layout, are ported into the harness as `mock/benches/digest-crossover/` in round
`202611191552-y2ye`. Op's xxhash call is clause-dev
`.data/op-responses/202611131704_op-on-option-1-i-love-zig-as-much-as.md`. `cargo test -p arvo-digest -p arvo-bits` passes on the branch,
the 64-bit reference values included; `cargo deny check licenses` could not run (ticket
`lazy_tool_cannot_install_cargo_deny_in_a_helper_container`).

Ticket `arvo_has_two_width_vocabularies_u16_for_a_digest_and_u32_for` is met by `e3202031`, where
the digest counts its widths in `u32`, and waits on the coordinator to close it. Ticket
`arvo_digest_s_digest_does_not_derive_hash_while_bits_does`, from the branch's review, stays open on
what landed.

## The reviews of the landing

Three fresh passes read `work/arvo-stranded`: `REVIEW_1.md` at `ec0dc80c`, `REVIEW_2.md` at `50ebd78a`
and `REVIEW_3.md` at `c7730375`, each merge after fixes. Rounds `202611191552-y2ye` and
`202611191608-le48` answer the first two. Of the third's two blockers, the XXH3 suite was run on the
two further SIMD paths this host has, on top of the SSE2 default, both green (6 tests each), under
`RUSTFLAGS="-C target-feature=+avx2"` and `+avx512f`, with
`cargo test --manifest-path mock/Cargo.toml -p arvo-digest --test xxh3`. Scalar, NEON and wasm are not
run, and the canon declaration of the design and a ruling row for op's xxhash call are left; those
and the third pass's other findings are follow-up tickets, after the third pass, as the landing rule
has it. Ticket `arvo_format_admits_the_wrapping_product_over_the_unsigned_64` is met on `dev` by
`d0161800` (closed ticket `value_s_product_admits_the_unsigned_64_bit_wrapping_product`), and nothing
cites it any more.
