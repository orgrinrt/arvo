# Sketch: can a base primitive design be written invariant under the container premise

**Hypothesis.** A design for arvo's base numeric primitives can be written now, before
`question::the_container_premise` is ruled, by choosing a type shape that is invariant under both
branches. If that holds, the design round proceeds and the premise gets ruled later without
invalidating what was written. If it does not hold, writing the design picks a branch, and picking is
answering an open canon question inside a design where nothing records that a canon decision was
made.

Cited by
`mock/design_rounds/202608311729_topic.the-base-primitive-design-is-blocked-on-the-container-premise.md`.

## Outcome

**FAILS.** No invariant shape exists. Three files, one runnable and two that must not compile, and
the two refusals are the load-bearing results.

`p1_branch_arity.rs`, `WORKS` in the sense that both branches compile and the separation is real.
Output in `p1_output.txt`, six tests passing, one of which is the control that fails loudly if the two
carriers it separates are indistinguishable. Branch A, carrier not in the type, gives one footprint
per declared width. Branch B, carrier in the type, gives two footprints at one declared width. The
consumer signature takes two generic parameters under A and three under B. A default on the carrier
parameter does not dissolve the fork: it picks branch B and hides it, since two footprints stay
reachable at one declared width, and the default never reaches the polymorphic position, so every
generic consumer downstream threads the parameter regardless.

`p2_branch_a_cannot_express_two_carriers.rs`, `FAILS WITH error[E0080]`, which is the intended result.
Output in `p2_output.txt`. Under branch A the sentence "two carriers at one declared width" has no
expressible form. That reaches a ratified intent:
`ruling::cold_is_for_cold_paths_and_cold_storage` is about the footprint, and on branch A no
signature distinguishes the footprint.

`p3_branch_b_splits_a_homogeneous_container.rs`, `FAILS WITH error[E0308]`, also the intended result.
Output in `p3_output.txt`. Under branch B two selections at one declared width are not substitutable
inside a homogeneous container, which is the cost
`proposal::an_axis_the_realisation_map_does_not_read_is_not_a_type_parameter` names as "no repair at a
homogeneous container", landing on the storage path.

## What must fail, stated before the runs

Each file names its own. `p1` section 0 is the control: if `u16` and `u32` are indistinguishable by
the ambient layout observation, every separation in the file is vacuous and the control fails. `p2`
and `p3` are the two refusals, and a successful compile of either refutes the claim in its header
rather than confirming anything.

## Toolchain

`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2021, the repo's pinned nightly. `p1` built
with `-O --test`, the other two with `--crate-type lib`.

## What it unblocks, and what it blocks

**Blocks** the base primitive design round, which stays in TOPIC.

**Unblocks** nothing on its own. What it does is make the block checkable rather than a matter of
reading two canon rows and forming an impression, so the next seat can disagree with a compiler error
instead of with a paragraph. The three canon questions that would clear it are listed in the topic
file.
