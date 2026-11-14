# A second canon read on the slot coordinate tickets

**Date:** 2026-11-14
**Author:** an opus seat dispatched by job `arvo-format`, reading `f384696f` in its own worktree, blind to
the plan's verdicts in `mock/research/202611132345_slot-coordinate-plan/PLAN.md`.
**Verbatim:** the seat's report as returned, kept whole below.

## Where it agrees with the plan

Both seats read shape B (the exact position's floor in four limbs, `Slot` unchanged, the 64-bit range
bound kept) as permitted for the product and for the quotient and conversion positions, and as reaching
the width 64 cell of the inverse round trip law. Both read the width 65 and 128 cells as out of B's reach
and resting on an unsettled call. Both read the design's open item on widening the slot coordinate as the
text that disagrees with "Where the exact step happens". So round `202611140239` stands on two
independent agreements for what it builds. This seat reads shape A as forbidden and C as permitted on a
condition, where the plan only declined them; neither is built, so that difference decides nothing here.

## Report

I read the canon at f384696f in /home/user/.worktrees/canon-read-slot and wrote nothing. Two summary findings first. No `rung = "open"` row decides A against B against C, or M1 against M2. And the status-quo fourth condition is itself in tension with two ratified rows.

**How I searched.** `cargo mock canon` and `cargo mock already-said` are not subcommands in arvo; they exist only in kaski. `cargo mock query ruling` returns truncated tables. So I grepped `mock/registry/*.toml` directly and printed the rows in full. The four tickets are not in arvo's registry, so I could not read their text.

### The rows that decide

- **R1** `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` (ratified, experts): "an operation is admitted exactly when it is a function of the declared signature, and where two realisations of one name disagree, the signature is missing a coordinate."
- **R2** `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (ratified, experts): "every operation the design declares is a function of the declared width and never of the machine carrier".
- **R3** `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op): "Never any runtime checks, ever: invalids are caught at compile time and unused paths are cleared when lowered."
- **R4** `ruling::the_work_is_predicated_arms_composed` (ratified, op): "The work is not a generalisation but a set of arms with const predicates, each optimising one nameable region and applying nowhere else". Read with `ruling::arms_over_regions_are_the_fundamental_heart` (ratified, both): "rather than by one or a few general statements uniform across the dimensions". And with `ruling::accuracy_is_never_bought_with_performance` (ratified, op): "that is a gap in the solutions rather than a trade to be made, and the work is finding the arm that is both."
- **R5** `ruling::the_format_spine_is_canon` (ratified, both): "The concept is closed and the inventory of admitted instances is open."
- **R6** `ruling::the_operating_constraints_are_intents_and_rules` (in_force): "monomorphisation as the dispatch".

The fourth condition holds out the product over `UFixed<64,k>`, and some quotients and conversions, for a fact about the `i128` index. Those operations are functions of the declared signature, so R1 ("exactly when") and R2 both cut against the condition as it stands. All three shapes remove it.

### Ticket 1 (product over `UFixed<64,k>`, `k >= 0`) and ticket 2 (quotient and conversion positions past the index)

**A: forbidden, on my reading. It needs a second seat.**
- R4 is the ground. A puts every slot of every format, 8-bit included, on four limbs. That is one uniform statement across the width axis, paying wide arithmetic where only the 64-bit corner needs it.
- R3 adds a condition. If the partial `Slot::index` becomes a run-time "maybe", it is a runtime check. `proposal::a_quotient_s_divisor_is_non_zero_by_type` (one_expert, not canon) applies R3 the same way: "a constructor that classifies a slot and answers with a maybe, is a runtime check". A const-gated accessor would not be.
- What would flip this: a bench showing the wide slot lowers to narrow code at narrow formats. `ruling::do_not_escalate_a_measurement_dispute` (stated) says benches decide such disputes.

**B: permitted.**
- It removes the dependency R1 and R2 object to.
- Its wide floor is an intermediate, so the adapted answer still lands in a 64-bit range inside `Slot`.
- Nothing in it is a runtime check, provided the four-limb floor reduces or saturates totally.
- The 64-bit bound it keeps is reserved, not ratified. `proposal::the_platform_width_notes_region_predates_the_sixty_four_wide_ladder` (one_expert) says "`ADMITTED` still refuses unconditionally, now past 64, so what the diagnostic tells an outside implementor and what the assertion does are still to be squared". Against R5's open inventory, that bound is unsettled.

**C: permitted, on one condition.**
- The carrier must be a function of the declared signature (an associated type derived from `F`), never a free choice by the caller. A free choice breaks R1's clause that disagreeing realisations of one name mean the signature is missing a coordinate. R6 is satisfied.
- `ruling::the_carrier_question_waits_on_the_contention_measurement` (open) is not touched as long as "carrier" means the slot coordinate's integer. If C's carrier were the value's storage carrier, it would enter that reserved question and arvo-placement's domain (R1's placement half).
- `obligation::the_unstable_machinery_does_not_reach_a_consumer` binds if the per-format carrier needs a feature gate a consumer would have to carry.

### Ticket 3 (inverse round trip), per width cell

`law::the_multiplicative_inverse_round_trip` is a law row, which is evidence rather than a ruling. It records the law as holding at "W in 1..=65" and failing at "W = 128" with "Witness 1 at width 128". So the 128 cell owes a red.

| Cell | A | B | C |
|---|---|---|---|
| W=64 | permitted on width, forbidden on R4 as above | permitted and sufficient | permitted |
| W=65 | forbidden on R4 as above | does not reach it; reserved | permitted |
| W=128 | forbidden on R4 as above | cannot reach it | permitted, and the only shape that reaches it consistently with R4 |

- **W=64:** I could not see why it is unreachable today. The design's open item says "Under wrapping `mul` admits it", and this law is a wrapping law. Check the ticket before acting on this cell.
- **W=65:** this needs `Slots::ADMITTED` past 64. A 65-bit range already fits in an `i128` `Slot`; only the product position (about 2^130) passes the index. B as stated keeps the 64-bit bound, so it cannot reach the cell. Leaving 65 held out is not forbidden by any ruling sentence, but it conflicts with R5 as the stale-note proposal reads it, which makes it reserved.
- **W=128:** the range 0 to 2^128 - 1 does not fit in an `i128` slot, so B cannot reach it.

### Ticket 4 (runtime divisor over a whole-phase range without the zero)

**On R3, both M1 and M2 are permitted.** Neither door rejects at run time.
- `ruling::the_ingest_door_waits_on_a_known_use` (stated): "A runtime membership door at ingest is not blessed now ... the typestate is the default answer."
- `proposal::ingest_of_a_host_value_is_the_declared_adaptation` (one_expert): "a rejecting constructor is not an ingest".
- A range that leaves the zero out is zero-free by type, and the declared adaptation onto it is total.

**Whether a `Member<F>` type may exist in arvo-format at all is reserved.**
- `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` (ratified, experts) leaves it out of scope. Its `says` closes: "It does not decide ... how many value types there are or in which crate". Its `gap` names "the count and crate of value types" as "still the panel's".
- `proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature` (note, one_expert) says "the value type belongs to a crate above it".
- That value type admits only a constant quantum and a zero phase (`proposal::a_value_admits_a_constant_quantum_a_zero_phase_and_a_deterministic_mode`), so it cannot serve as a nonzero-phase divisor.

**M1: reserved, and it collides more.** It replaces the format crate's slot-taking surface with a member type on every operation. That is a second value type beside arvo-numeral's, and the count-and-crate question is unsettled.

**M2: reserved, but it is the narrower shape and the one the unratified rows already point at.**
- `proposal::a_quotient_s_divisor_is_non_zero_by_type` (one_expert): "a whole-phase format divides only by constants ... until a slot can be typed by membership".
- `proposal::the_old_numeric_contracts_return_as_arvos_own_and_the_crate_does_not` (one_expert): "division takes a refined divisor".

### The design text disagrees with itself

The section "Where the exact step happens, which is not in the set it lands in" says "The step's intermediate is not a slot index and is not required to be one; only the answer is". It also says the only reason a design would refuse or guard there "is that it chose a carrier too narrow for the intermediate". That sentence describes the fourth condition exactly.

The open item says instead that "A position past the index itself is a widening of the slot coordinate past `i128`, which every other computation over a slot would then owe too". That puts the intermediate in `Slot`, which is shape A's premise.

The exact-position section sides with the first reading. `Exact` already names positions past the index through the pinned slot and `past`, with no wider `Slot`. So the open item is the outlier, and the section is the text aligned with R1 and R2.

### Rows a reader would miss with the obvious terms

- **The note on `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`.** It says a slot-index ceiling "is a fact about one representation and not about what kind of thing a platform-width type is". It also records that `slots.rs`'s `E0277` diagnostic says the trait is open while `ADMITTED` refuses unconditionally, "so the crate contradicts itself in two sentences about one trait". The note is stale per the correcting proposal; its principle is not.
- **`proposal::the_exact_intermediate_is_named_beside_the_value`** (one_expert). It proposes a public exact accumulator, which contradicts the design's "giving it a name in the surface would put a quantity on the public API". Its gap says "a product or a quotient held exactly, is not stated", which is where B's wide floor lives.
- **`dimension::container`.** Its `moves` field is "Whether an intermediate fits, and therefore whether a law's proof over exact values survives at all."

### What I could not check

- The tickets' own text, which is not in arvo's registry.
- Why W=64 is unreachable, given the design says wrapping already admits it.
- Any performance measurement of A or C. No bench exists, and the A verdict rests on R4 without one.
- The research tree, which I did not read, per the brief's ladder.
- I ran no tests and no sketches.
- I am one seat: every forbidden or reserved call above needs an independent second agreement.
