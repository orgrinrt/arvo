# Blind seat 2: where the numeral value contracts are declared

Seat on opus, high effort, in a read-only worktree of arvo at 38d78396, cut before any earlier reading on this question was committed, dispatched after blind seat 1 returned and without sight of it. Routed by stack-coord on 2026-11-20. Judgement steps are marked [J].

Verdict: (b), a crate of its own directly above `arvo-format`, depending on it (and notko) alone, by `a-crate-holds-only-its-own-thing` and the in_force bare-primitive constraint, with the taxonomy left open to the panel. No source ranks the two rules, so this binds once a second independent agreement stands under `ruling::the_panel_finishes_the_canon_without_him`.

## 1. arvo-bridge-home-rule, read as written

Lines 14-16: name every type in the surface, find the lowest crate that defines or reaches all of them, declare the trait there. `Bool` and `Width` are in `arvo-format` (`src/width.rs:35`, `:102`); the ordering result is `core::cmp::Ordering` (`arvo-numeral/src/order.rs:8`). `arvo-format` depends only on notko; notko defines neither type (the rule excludes it, 62-64); `arvo-bits` has no edge to `arvo-format`. Read literally: (a). Impls for `Num` go in `arvo-numeral` (17-18) under either home.

Not applying: the table (34-48) and examples (62-68, 74-75) name `arvo-storage` and `arvo-hash` and an old layering; neither crate is a member, and `arvo-strategy` now depends on `arvo-format` and `arvo-placement`. `mock/agent/MAIN.md.tmpl:25` calls the old architecture a dead tier. The trigger (3, 28-32) covers order, identities and construction from a constant; sign and magnitude mirror no std trait [J]. For a trait whose surface is only `Self` (sum, identities, construction) steps 2-3 pick no crate, and the rule lets those sit apart from the `Bool` family (65-68) [J]. Its bare-primitive impl clause may have nothing to act on, since `Num`'s machine types are sealed (`arvo-numeral/src/machine.rs:22`) [J]. It quotes none of op's words; its content comes from round 202605021800 of the removed tree.

## 2. a-crate-holds-only-its-own-thing, and arvo-format's subject

Line 9, each crate holds only the thing it is for; line 11, the tell is a trait placed "because the place that needed it was here", the fix to give it to the owner or, "where no owner exists, to make one"; line 12; line 26 (op, 30 Oct 2026, quoted at 20-24), "by what it is for, never by where the need arose". Here the need arose in `arvo-format` because `Bool` and `Width` sit there, which is the tell.

The canon: no ruling has the crate as subject; it is named only in notes of `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` and `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`. `topic::the_format` ("the widths it declares, its signedness, and how a declared format relates to the container underneath it"); `ruling::the_format_spine_is_canon`; `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` (the door carries the coordinate set; value contracts are not coordinates [J]). Design tier: `arvo-format/DESIGN.md.tmpl:31`; `width.rs:6`, `:21`. `proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature` (one expert) puts the value in a crate above it.

## 3. Does a source rank them

No. `ruling::the_operating_constraints_are_intents_and_rules` ranks none; `every-rule-is-ops-words` treats every rule as equal; `a-rule-is-loaded-every-session.md.tmpl:40` notes only that contradicting rules get picked arbitrarily; the ladder (`.shared/skills/dispatch-an-expert/reference/expert-dispatch-defends-the-canon.md.tmpl:12`) gives a method, resolve by provenance.

Set aside: the "lowest" clause of step 3 only [J], on three grounds. Provenance: the crate rule carries op's verbatim words, the bridge-home placement none, descending from the removed tree. Canon: `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (ratified, op verbatim) opens "the shape the contracts and traits and default impls etc take ... and also the taxonomy"; the bridge-home table is the old tree's taxonomy. Its own purpose: every example is an inversion, and a home directly above `arvo-format` causes none (65-68) [J].

## 4. Answer: (b)

1. The surface is `Bool`, `Width`, `Self` and an ordering result, reachable from `arvo-format` up.
2. The contracts are about values, not declared signatures [J], so by the crate rule (9, 11, 26) not in `arvo-format`.
3. A separate sourced ground: `arvo-format` is exempt crate-wide from the bare-primitive lints (`mockspace.toml:1500`), while the contracts are a public surface held to the in_force rule of the stack's own primitives (`ruling::the_operating_constraints_are_intents_and_rules`); declaring them there puts them where the lint cannot refuse anything, the defect the note on `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` records.
4. Not `arvo-numeral`, though it declares `Identity<M>` (`src/identity.rs:18`): it is one realisation, and contracts there bind every later value type to it, against `ruling::the_trait_contract_structure_is_a_primary_paradigm` and `obligation::the_surface_expressible_as_contracts_before_anything_implements_it` [J on the consequence].
5. No owner exists, so one is made, depending on `arvo-format` and not on `arvo-placement`, keeping the declared-signature property by the dependency edge (DESIGN :33-35) and the bridge-home floor (54-55).
6. `Num`'s impls in `arvo-numeral` (17-18).
7. The call is the panel's, not op's, and not (d): the taxonomy is held open by a ratified ruling, not a rung-open reservation; `ruling::the_panel_finishes_the_canon_without_him` makes it the panel's, by two independent agreements.

Not decided: `arvo-bits` depends on nothing in the workspace (`mockspace.toml:1502-1505`), so `Bits` implements these contracts only by a new edge or by impls in the contract crate, under (a) as under (b).
