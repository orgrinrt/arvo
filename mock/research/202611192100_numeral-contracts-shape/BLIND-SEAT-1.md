# Blind seat 1: where the numeral value contracts are declared

Seat on opus, high effort, in a read-only worktree of arvo at 38d78396, cut before any earlier reading on this question was committed. Routed by stack-coord on 2026-11-20 after review pass 1 of `work/numeral-contracts` found that `proposal::a_value_contract_is_not_the_format_crates_subject` recorded two experts where no seat had ranked the two rules.

Verdict: (b), a crate of its own directly above `arvo-format`, depending on it (and notko) alone. The step that sets the bridge rule's "lowest" clause aside is the seat's own judgement: no source ranks the two rules.

## Corrections to the brief

- The tree at 38d78396 holds six crates; `arvo-digest` is a member (`mock/Cargo.toml`; `mock/agent/MAIN.md.tmpl:10-20`). `mock/PRINCIPLES.md.tmpl:5` still says five.
- The canon names no crate: `cargo mock query arvo-format` does not resolve.

## 1. arvo-bridge-home-rule, read as written

It places a trait in "the lowest crate in the substrate dependency graph that defines or reaches all those types" (`.shared/rules/arvo-bridge-home-rule.md.tmpl:10,14-16`), impls by the orphan rule (`:17-18`). `Bool` and `Width` are declared in `arvo-format` (`src/width.rs:35,102`, re-exported `src/lib.rs:143`); the ordering result is `core::cmp::Ordering` (`arvo-numeral/src/order.rs:22`). The lowest crate reaching all three is `arvo-format`, so the rule read literally says (a); a new crate is never what it picks.

Not applying to the current tree: the placement table (`:34-48`, round 202605021800) and examples (`:62-68`, `:74-75`) name the removed tree, a dead tier (`mock/agent/MAIN.md.tmpl:22-27`). Its scope is a const trait mirroring a std trait (`:28-32`, `:3`): sum, order and construction from a constant fit; identities, sign and magnitude have no std trait (the seat's reading; the dead table listed `Identity` and `Bounded`). What still applies is the inversion test (`:52-55`): any home must depend on `arvo-format`.

## 2. a-crate-holds-only-its-own-thing, and arvo-format's subject

"Each crate or module holds only the thing it is for" (`.shared/rules/a-crate-holds-only-its-own-thing.md.tmpl:9`); the tell is a trait about another crate's domain placed "because the place that needed it was here", the fix to give it to the owner "or, where no owner exists, to make one" (`:11-12`); contents by "what it is for, never by where the need arose" (`:26`).

The canon on the format concept: `ruling::the_format_spine_is_canon`, `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` (why `Bool` and `Width` sit there, as door primitives), `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point`, `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`. Design tier: `arvo-format/DESIGN.md.tmpl:31` "The declared signature, and nothing else"; `arvo-numeral/DESIGN.md.tmpl:43-45`.

Silent or open on the crate: `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`; `proposal::the_old_numeric_contracts_return_as_arvos_own_and_the_crate_does_not` (gap: the crate is open); `obligation::the_surface_expressible_as_contracts_before_anything_implements_it` (gap: nothing enforces where traits live).

## 3. Does a source rank them

No. No workspace rule states a precedence between the two; `every-rule-is-ops-words` gives none. `expert-dispatch-defends-the-canon.md.tmpl:21` gives a method, resolve by provenance: the crate rule carries op's dated words (`:20-24`), the bridge rule none beyond a round in the removed tree. Treating that as a ranking is the seat's reading.

## 4. Answer

1. The contracts are arvo's own and their crate is open (`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`): a design-tier choice, not a reserved open call.
2. Any home depends on `arvo-format` (inversion test, `:52-55`).
3. Not (a): the format concept is the declared signature; value contracts are another thing (crate rule `:9`, `:11`).
4. Not `arvo-numeral`: contracts stand before and apart from any implementation (`ruling::the_trait_contract_structure_is_a_primary_paradigm`, `obligation::the_surface_expressible_as_contracts_before_anything_implements_it`, `ruling::arvo_is_a_library_and_the_value_composes_on_top`, `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`). Judgement: that these require contract and implementation in separate crates is the seat's derivation.
5. No owner exists, so make one (`:11`), depending on `arvo-format` alone; `Identity<M>` (`arvo-numeral/src/identity.rs:18`) moves there.
6. Judgement: the crate rule wins over the bridge rule's "lowest" clause (`:15`), set aside because its worked content is the dead tier, part of the set lies outside its definition, and (b) passes its inversion test. A seat weighing the clause as binding gets (a).

Not settled: the home of a stack-owned ordering type if one is wanted; where `Bits<N>` implements the contracts (either home pulls `arvo-format` into `arvo-bits`, refused at `arvo-bits/DESIGN.md.tmpl:24-36`, a cost that does not separate (a) from (b)).

If (d): the missing decision is the precedence between the two workspace rules for arvo, op's to state or to have derived (`every-rule-is-ops-words`), not a canon question for the panel.
