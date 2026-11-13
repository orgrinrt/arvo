# A lint refusing a `FIXME` marker that names no ticket

The lint half of ticket `arvo_fixmes_carry_no_ticket_id_and_nothing_refuses_them`, written and tested but not landed, because landing it turns the gate red until six markers name their tickets and editing those crates needs a source changelist.

`a_fixme_names_its_ticket.rs.txt` is the lint, to be placed at `mock/lints/a_fixme_names_its_ticket.rs`. `code_only_source.patch` adds `comment_lines` beside `code_only` in `mock/lints/code_only_source.rs` and applies with `git apply`. With both in place the lint's 18 tests pass, and `cargo mock --lint-only` reports exactly the six markers below, as errors.

The six markers and the tickets filed for them:

| site | ticket |
| --- | --- |
| `arvo-strategy/src/lib.rs` lines 87, 105, 122, 139 | `arvo_s_four_preset_adaptation_defaults_are_carried_rather_th` |
| `arvo-format/src/symmetry.rs` line 64 | `arvo_s_even_translation_parity_region_is_named_and_expressed` |
| `arvo-format/src/symmetry.rs` line 221 | `arvo_s_reflection_predicate_takes_no_reach_and_answers_no_in` |

Each marker becomes `// FIXME(<ticket>): <the existing reason>`. That edit is a source change in two crates, so the gate refuses it outside a source changelist (`changelist-required`). It lands with the lint in one change once a round for those crates is open.

The id check is a shape check (snake_case, at least twelve characters, an underscore), because the tracker lives outside this repository. Whether an id resolves is a different check.
