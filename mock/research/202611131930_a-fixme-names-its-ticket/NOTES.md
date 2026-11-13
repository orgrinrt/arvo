# A lint refusing a `FIXME` marker that names no ticket

The lint half of ticket `arvo_fixmes_carry_no_ticket_id_and_nothing_refuses_them`. It was first written and tested here without being landed, because landing it turns the gate red until six markers name their tickets, and editing those crates needs a source changelist. The round `202611132036` opened that window, and the lint now sits at `mock/lints/a_fixme_names_its_ticket.rs`, with `comment_lines` beside `code_only` in `mock/lints/code_only_source.rs`. The copies that stood in this directory were removed in the same change.

The six markers and the tickets filed for them:

| site | ticket |
| --- | --- |
| `arvo-strategy/src/lib.rs`, the four preset impls | `arvo_s_four_preset_adaptation_defaults_are_carried_rather_th` |
| `arvo-format/src/symmetry.rs`, the parity region | `arvo_s_even_translation_parity_region_is_named_and_expressed` |
| `arvo-format/src/symmetry.rs`, the reflection predicate | `arvo_s_reflection_predicate_takes_no_reach_and_answers_no_in` |

Each marker is now `// FIXME(<ticket>): <the existing reason>`.

The id check is a shape check (snake_case, at least twelve characters, an underscore), because the tracker lives outside this repository. Whether an id resolves is a different check.

The rule binds every repository, so the lint belongs in `mockspace-extra-lints`; that move is the ticket `a_fixme_names_its_ticket_is_workspace_general_and_has_no_hom`.
