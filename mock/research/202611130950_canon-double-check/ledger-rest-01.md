### mock/research/202606231500_branchless-arith-review.md
- date: 202606231500 (name stamp); round 202606231229 referenced
- by: not stated
- holds: A deferred audit note on whether the arith op bodies (Precise logical-bound clamp, >64-bit fixed-point widen) should be branchless. It records the decision to keep the `if` form for now, the revisit conditions, and a TODO on per-target perf of the 256-bit widen.
- headline: "Decision for this round: keep the `if` form. Recorded here and as task #4 for a later audit pass." (line 8)
- watch: "The unsigned upper-clamp has a clean branchless form: `v - v.saturating_sub(hi)` (unsigned `saturating_sub` floors at 0, giving the lower bound for free). Zero readability cost; could adopt." (line 27)
- watch: "`<<` / `>>` by >= 128 (or the `128 - frac` underflow) is UB on `u128`" (line 22)
### mock/research/202607281616_prior_art/00_context.md
- date: 2026-07-28
- by: not stated
- holds: The shared baseline for a set of parallel prior-art research passes. It states what arvo shipped (sixteen crates and their export surfaces), the facade/dependency structural problem, the restructure prescribed in two named topic files, the standing constraints, and what a research deliverable may and may not do.
- headline: "The single most important line: **nothing in this directory decides anything.**" (line 11)
- watch: "`arvo-strategy` | `Strategy` and the markers `Hot` / `Warm` / `Cold` / `Precise`; `Signedness`, `Unsigned`, `Signed`; `Resolve`; `Identity<Op>`, `Additive`, `Multiplicative`, `Bounded`, `SignedIdentity`, `OneRepresentable`; `UArith`, `IArith`, `USaturating`, `ISaturating`;" (line 108)
- watch: "the axes family `Bitpacked`, `Dense`, `DoubleLogical`, `Min`, `OverflowPolicy`, `Saturating`, `Wrapping`, `StorageLayout`, `ContainerWidth`, `HasAxes`" (line 108)
- watch: "Seven crates carry a Cargo dependency on `arvo`: `arvo-bitmask`, `arvo-comb`, `arvo-graph`, `arvo-hash`, `arvo-sparse`, `arvo-spectral`, `arvo-tensor`." (line 128)
- watch: "The diagnosed cause: arvo's convention is a contracts crate plus a concrete sibling, as `arvo-bits-contracts` has `arvo-bits` and `arvo-mask-contracts` has `arvo-bitmask`." (line 141)
- watch: "**arvo ships tools and never polices its consumers**: it exposes the full lattice of choices with the tradeoffs documented, rather than picking a threshold on the consumer's behalf." (line 94)
- watch: "op has forbidden `generic_const_exprs` outright, alongside full `specialization`, restating a gate that predates this workspace" (line 71)
### mock/research/202607281616_prior_art/01_geometric_algebra_rotors_motors.md
- date: 2026-07-28
- by: not stated
- holds: A prior-art survey of rotors, motors and geometric algebra for rotation (plane-based PGA versus Lengyel's rigid GA, Dorst/Lasenby line, CGA, implementation practice, normalisation, interpolation). It reports a negative result on fixed-point geometric algebra and on type-level derivation of the bivector set from an axis list.
- headline: "This pass found no published work on general-purpose fixed-point or scaled-integer geometric algebra arithmetic, in the sense arvo means by fixed point (a `UFixed`/`IFixed`-style scaled integer as the primary representation for rotor and motor components, replacing float throughout)." (line 259)
- watch: "A design round building fixed-point rotors is building something without a literature to check its numerical claims against, not something with an established practice to follow." (line 299)
- watch: "Q-format quaternion libraries exist and are common in embedded 3D and orientation-filter work, though this pass did not find a peer-reviewed treatment of their error behaviour comparable to the float case" (line 295)
- watch: "Every rotor and motor implementation surveyed above, without exception, uses IEEE floating point as its component type." (line 261)
- watch: "This is the same choice arvo's own strategy-axis philosophy already names for unrelated reasons (arvo ships tools and does not pick the threshold on the consumer's behalf)" (line 166)
### mock/research/202607281616_prior_art/02_number_systems_and_fixed_point.md
- date: 2026-07-28
- by: not stated
- holds: A research survey of what the field does around fixed-width and fixed-point arithmetic: Q formats and saturation in DSP, determinism and constant-time arguments, posits/takums, block floating point, LNS, RNS, interval/affine/Taylor arithmetic, automatic word-length tools, bounded rationals, multi-limb integers, and iterative division/sqrt/transcendentals. It carries a correction note about `generic_const_exprs` and a closing list of what was not found.
- headline: "That two independently maintained, production-grade libraries in an adjacent field converge on \"const-generic-parameterised, stack-only, limb-array-backed\" as the right shape for arbitrary-but-fixed-width integer arithmetic is corroborating evidence that arvo's own `WideBits` design sits on well-trodden, actively-maintained ground, not on an idiosyncratic invention." (line 471)
- watch: "The DSP convention is the Q format, `Qm.n`, m integer bits and n fractional bits inside a fixed container, almost always saturating rather than wrapping on overflow because a wrapped audio or control sample is a much worse failure than a clipped one." (line 69)
- watch: "arvo's `OverflowPolicy` axis (`Saturating`, `Wrapping`) already covers this choice as a type-level parameter rather than a runtime flag, which is more expressive than the DSP convention (where the saturating behaviour is fixed per instruction, not chosen per value)" (line 74)
- watch: "the fixed-point literature treats saturating overflow as the default assumption in a way arvo's `Warm`-default framing does not." (line 79)
- watch: "a consumer picking arvo for cross-node determinism [...] is choosing it for a property (bit-exact reproducibility across compilers and targets) that the strategy-marker documentation does not currently name as a reason to reach for `Precise` over `FastFloat`." (line 107)
- watch: "arvo's `UArith` / `IArith` / `USaturating` / `ISaturating` families give no signal, in the surface enumerated by `00_context.md`, about whether a given implementation path is constant-time" (line 119)
- watch: "`bnum` [...] uses const generics to let a consumer pick any bit width from 2 up to `2^32 - 1`, signedness, and overflow behaviour (wrap, saturate, or panic) as compile-time parameters with zero runtime cost" (line 461)
### mock/research/202607281616_prior_art/03_type_level_shape_and_rank.md
- date: 2026-07-28
- by: not stated
- holds: A prior-art survey on type-level shape and rank: the hlist lineage and its compile-time costs (shapeless, frunk), shape-typed array languages (Accelerate, Dex, Futhark, Remora, Star), rank polymorphism, Rust feature status (`generic_const_exprs`, `adt_const_params`), typenum/generic-array precedent, and the cost of a per-width impl table. It was revised to correct a baseline error about `generic_const_exprs` and ends with a list of what was not found.
- headline: "**The honest state of this question, after this pass, is: the literature that exists supports treating a flat per-width impl table as a materially different, and plausibly cheaper, trait-solver workload than the recursive hlist question the rest of this pass covers in depth, but nothing found here measures that difference directly, for Rust, at the table sizes D2's crate split will actually generate.**" (line 404)
- watch: "`mock/crates/arvo-graph/src/lib.rs:17` states plainly that `generic_const_exprs` \"is not needed. Working arrays are the capacity's [own array]\"; and `mock/crates/arvo-spectral/tests/capacity_threading.rs:12` states its own \"GCE-escape proof is the absence of `#![feature(generic_const_exprs)]`.\"" (line 22)
- watch: "No source found a published account of fixed-point arithmetic combined with any of the shape-typing schemes surveyed here" (line 477)
- watch: "`WidthFor<F: Family>` maps a width, carried as typestate (`Wid<N>`, the direct analogue of `Dim<N>`), to a bucket, carried as an associated type, one impl per concrete width per strategy family." (line 360)
### mock/research/202607281616_prior_art/04_algebraic_structure_hierarchies.md
- date: 2026-07-28
- by: not stated
- holds: A prior-art survey of algebraic structure hierarchies (newtype-wrapper versus operation-marker designs, alga/simba/noether/num-traits/NumHask/Spire, monoid folding, CRDT semilattices, HKT cost). It works a counterexample showing two-sided saturating addition is not associative and states what was not found.
- headline: "Under `Saturating` with two-sided clamping (arvo's actual signed case) it is not associative at all, and a `Combine<Additive>` impl that claims otherwise would be claiming a false law." (line 233)
- watch: "with a four-bit signed range `MIN = -8`, `MAX = 7`, and `a = 7, b = -8, c = -8`, `(a ⊕ b) ⊕ c = f(f(7 + (-8)) + (-8)) = f(-1 + (-8)) = f(-9) = -8`, while `a ⊕ (b ⊕ c) = f(7 + f(-8 + (-8))) = f(7 + (-8)) = f(-1) = -1`. `-8 ≠ -1`: two-sided-clamped saturating addition is not associative." (line 225)
- watch: "Wrapping addition on a fixed width is addition in `Z / 2^N Z`: it is fully associative, has an identity, and every element has an inverse, so `UFixed<I, F, Hot>` under `Wrapping` addition is a genuine abelian group under addition, not merely a monoid." (line 217)
- watch: "arvo's own two overflow markers (`arvo-strategy/src/axes.rs:38`, `OverflowPolicy` sealed by `Wrapping` and `Saturating`)" (line 215)
- watch: "floating-point addition and multiplication are commutative but not associative because of rounding" (line 248)
- watch: "This pass found no published treatment, in Haskell, Scala, Rust, or a proof assistant, of exactly arvo's shape: one type, two overflow-policy instantiations, where the algebraic structure the type satisfies genuinely differs between the instantiations rather than merely the runtime behaviour differing." (line 208)
### mock/research/202607281616_prior_art/05_curve_representations.md
- date: 2026-07-28
- by: not stated
- holds: A prior-art survey of curve representations named in D11 (clothoids/Euler spirals, biarcs, implicit and signed-distance forms, interpolating splines, rational conics, Beziers), weighed against the rounded-corner, heavy-simulation and terrain-editor workloads. It reports that no fixed-point treatment of clothoid or Fresnel evaluation was found, and closes with the questions that separate the candidates.
- headline: "No paper or production codebase found in this pass evaluates clothoids, biarcs, or interpolating splines in fixed-point arithmetic as a primary target." (line 352)
- watch: "FreeType, the reference open-source rasteriser, represents every coordinate internally in F26Dot6 fixed point (26 integer bits, 6 fractional bits, so one unit is 1/64th of a pixel) throughout scan conversion" (line 364)
- watch: "This is decades of production use of fixed-point quadratic Bezier evaluation at a fixed, coarse precision (6 fractional bits) chosen because font rasterisation only ever needs sub-pixel accuracy relative to a target raster grid, not because 6 bits is a generally correct precision for fixed-point curve work." (line 368)
- watch: "each lerp introduces a rounding error, and a design that wants this exact should ask, and this pass could not answer from the literature, how that error compounds across a degree-3 or degree-5 evaluation chain at a given fractional-bit budget" (line 374)
- watch: "What fractional-bit budget and what composition depth (how many curve operations chain before a result is consumed) does the fixed-point substrate actually need to support, and does any of the four candidates' error behaviour under repeated fixed-point rounding actually get worse than the others at that budget. This question has no answer in the literature at all right now" (line 478)
### mock/research/202607281616_prior_art/06_nonlinear_algebra_and_low_rank.md
- date: 2026-07-28
- by: not stated
- holds: Two halves: an attempt to name the person behind op's recalled hunch about a bounded-optimal approximation win from the nonlinear-algebra side (ranked shortlist: Eckart, Farouki, Sturmfels, Hankel, Higham/Householder), and a survey of low-rank, tensor, structured-matrix, nonlinear-algebra, tropical and approximation-theory methods against a fixed-point, allocation-free, const-sized foundation. It reports no fixed-point implementation of any surveyed algorithm and no compile-time rank formulation.
- headline: "No fixed-point implementation of any low-rank, tensor, structured-matrix, or approximation-theory algorithm surveyed here was found anywhere in the literature searched." (line 463)
- watch: "D11 records op's hunch about an unconventional win in curve representation, coming from the nonlinear algebra side, attached to a name not yet recalled, and instructs research before any direction is committed." (line 9)
- watch: "This connects directly and concretely to `arvo-graph`'s existing `topo_sort`, `longest_path`, and `upward_rank`/`downward_rank` machinery: those are already tropical linear algebra" (line 376)
- watch: "**PALMA** (N'guessan, 2026, [arXiv:2601.17028](https://arxiv.org/html/2601.17028v1)), a C library implementing five tropical semirings (max-plus, min-plus, max-min, min-max, Boolean) for ARM-based embedded systems, deliberately using pure 32-bit signed integer arithmetic rather than floating point \"for determinism and predictability,\"" (line 382)
- watch: "Both formats sit directly above the sparse and spectral layers arvo already ships (`arvo-sparse`'s CSR and RCM reordering, `arvo-spectral`'s Laplacian and Fiedler-vector machinery)" (line 283)
- watch: "`arvo-numeric-contracts`' `Sqrt` and `Recip` traits already name" (line 429)
- watch: "No evidence was found of a compile-time (const generic, type-level) formulation of matrix or tensor rank anywhere, for any of the decomposition families surveyed." (line 476)
### mock/research/202607281616_prior_art/07_in_stack_facade_dependency_audit.md
- date: 2026-07-28
- by: not stated
- holds: An in-stack audit of the seven crates that depend on the `arvo` facade, tracing fifty-one `use arvo::` statements to the crate that declares each symbol. It concludes that fifty are import shortcuts, that `Cap` lives in `arvo-storage` rather than the facade, and that only the `Pred` aliases are facade-declared.
- headline: "Fifty of the fifty-one imports name a symbol that lives in a crate the importer is already permitted to depend on. The facade dependency is import convenience, not structural need." (line 50)
- watch: "**Not one of the seven imports `UFixed` or `IFixed`.** The algorithm crates are generic over trait bounds throughout" (line 47)
- watch: "`arvo-tensor`, `arvo-bitmask`, `arvo-hash`, `arvo-graph`, `arvo-sparse`, `arvo-comb` and `arvo-spectral` each carry a Cargo dependency on `arvo`." (line 22)
- watch: "**`Cap` does not live in the facade.** It is declared at `arvo-storage/src/platform.rs:73` as `pub struct Cap(pub USize)`, and re-exported from `arvo-storage/src/lib.rs:35`." (line 61)
- watch: "Creating `arvo-numeric` to complete the contracts-plus-sibling pattern rests on `arvo-numeric-contracts` having no sibling and its own README saying the default impl bodies live in the facade, which remains true and is the strongest observation in that round." (line 85)
- watch: "It does not change the restructure. D1 and D2 in `202607281220` are the lead designer's calls and they stand." (line 11)
### mock/research/202607281616_prior_art/08_in_stack_export_map_and_landing.md
- date: 2026-07-28
- by: not stated
- holds: An in-stack inventory of what the `arvo` facade declares versus re-exports, and where each declared group lands under the D2 restructure. It finds the `Pred` aliases unplaced, the euclidean family ambiguous between contracts and concretes, and `bitfield!` without an obvious home.
- headline: "Two items have no named destination and one group is ambiguous. Those are the finding; the rest is inventory." (line 14)
- watch: "`ScalarEuclid`, `EuclidDiv` and `EvenSplittable` are trait declarations, which by arvo's own contracts-plus-sibling convention belong with `Abs`, `Recip`, `Sqrt` and `TotalOrd` in `arvo-numeric-contracts` rather than with their impls in `arvo-numeric`." (line 83)
- watch: "`Pred`, `Pred2` and `Pred3` are not mentioned anywhere in the restructure." (line 71)
- watch: "`arvo-comb` imports `Pred2` at `greedy.rs:13` and `dp.rs:16`." (line 75)
- watch: "The strategy markers, `Identity`, `Bounded`, `SignedIdentity`, `Additive`, `Multiplicative`, `OneRepresentable`, `Picker`, `Ieee` and the widening family come from `arvo-strategy`." (line 37)
### mock/research/202607281616_prior_art/99_synthesis.md
- date: 2026-07-28
- by: not stated
- holds: A synthesis of the eight passes in the prior-art directory (00 through 08). It states the cross-pass finding that fixed point is alive in engineering and absent from the mathematics, what the field corroborates and challenges (including the non-associativity of two-sided saturating addition and the `Combine<Op>` dependence on the overflow-policy axis), what closes, and what the round should stop assuming.
- headline: "So the shape is not \"fixed point is unexplored\". It is that **fixed point is alive in engineering and absent from the mathematics**, and arvo sits precisely at the join." (line 41)
- watch: "signed `Saturating` addition is not associative under two-sided clamping, while `Wrapping` addition is a full abelian group. Both are `OverflowPolicy` instances sealed at `arvo-strategy/src/axes.rs:38`. So whether `Combine<Op>` satisfies its law depends on a **second** type-level axis, the strategy, not only on the operation marker that `Identity<Op>` is parameterised by." (line 151)
- watch: "ARM's CMSIS-DSP ships `q7_t` and `q15_t` and `q31_t` with saturating multiply-accumulate as a first-class instruction." (line 36)
- watch: "Uniswap V3 picked `Q64.96` specifically so `sqrt(price)` fits exactly." (line 38)
- watch: "Direct3D's UNORM convention divides by `2^n - 1` rather than `2^n`, so the all-ones bit pattern lands on exactly 1.0 [...] arvo's reasoning that `UFixed<0, F, S>` has no multiplicative identity is correct for arvo's encoding; UNORM is a different encoding at the same bit width where it does." (line 71)
- watch: "**Constant-time execution** is treated as a correctness property in cryptography, and it is in direct tension with the always-optimal-internals principle" (line 219)
- watch: "`arvo-comb`, `arvo-graph` and `arvo-spectral` each dropped their gate because of it." (line 118)
### mock/research/202607290030_predicates-arity-and-typestate-selection.md
- date: 2026-07-29
- by: not stated
- holds: A research summary pointing at two sketches on the `Pred`, `Pred2` and `Pred3` arity question and on typestate algorithm and microkernel selection. It records a supertrait drift between the locked changelist and the shipped file, the finding that a property in a predicate type selects an implementation before runtime, and the open questions (unpopulated properties, a soundness obligation, the fate of `Pred`). It carries no decision.
- headline: "**So the value is not performance. It is that the property cannot desync from the thing it describes.**" (line 87)
- watch: "**There is a soundness obligation.** An asserted property is an unchecked promise, and a false one produces a wrong answer rather than a slow one. That is a different risk class from `Strategy` and closer to an `unsafe` contract. It wants a stated contract per property and probably a debug-mode cross-check against the naive algorithm." (line 103)
- watch: "This is `Strategy` one level up: a property in the type, read by the trait solver, selecting the implementation before runtime, without the substrate hardcoding a threshold or policing the consumer." (line 92)
- watch: "The locked src changelist of round `202604470000` specifies **no supertrait**, blanket impls with real bodies [...] The shipped file has `: Fn(&A, &B) -> Bool` as a supertrait on every trait, empty blanket impls" (line 47)
- watch: "It sits inside `arvo-always-optimal-internals` (which licenses asm microkernels chosen by bench) by adding a per-shape axis alongside the per-target one, and inside `arvo-toolbox-not-policer` (which forbids hardcoded thresholds) because a derived property guesses nothing" (line 136)
- watch: "At **one** choice, typestate, const-generic `bool` and `const fn` are **byte-identical**; LLVM aliases them to a single symbol. \"Typestate gives better codegen\" is false." (line 79)
### mock/research/202609020400_the-admission-obligations-are-q1-and-the-open-question-they-answered-is-q31.md
- date: 202609020400 (name stamp); tree and registry revision `7fed7b5`
- by: Seat q29a (worktree `.worktrees/q29a`, branch `research/q29-q29a`)
- holds: A seat's reading of the nine `ADMITTED` assertions in `arvo-format`. It finds they are licensed by Q1 (op's Admissibility reading), are not an answer to Q29, and that the design instead used one word for what open question Q31 reserves to a panel. It also records a retraction on the ten-coordinate count, a false totality sentence, and an unguarded `Arity::of(0)`.
- headline: "**Both readings the brief offers are wrong, and the framing is wrong with them.**" (line 9)
- watch: "Usage, Admissibility, Self-validation, All that makes sense. Again, since this is the new panel, we learnt from last night, this isn't a strict call, so it can be challenged if truly not worth it, but my instinct is that the more robust it is, the better it'll serve us." (line 231)
- watch: "Admissibility: the typestate refuses declarations it cannot serve and does not refuse ones it could, which owes a two-directional sweep and for which the panel holds no evidence." (line 218)
- watch: "| 6 | `slots.rs:219` | `WIDTH.count() <= 62` | \"wider than a **slot index carries** ... `2^63` does not fit a **signed 64-bit integer**\" |" (line 104)
- watch: "A `Slots` impl with `WIDTH = 63` is a perfectly good slot range as a concept. This crate cannot carry it, because its own index is `i64`." (line 292)
- watch: "\"Where the canon explicitly holds a question open and reserves the call, that is not silence and it may not be filled inside a design.\"" (line 317)
- watch: "`Operation::ARITY` (`adapt.rs:126`) is an `Arity`, and `Arity::of` (`adapt.rs:97`) is `pub const fn of(operands: u32) -> Self { Self(operands) }` with no guard. **`Arity::of(0)` is constructible**" (line 384)
### mock/research/202609021238_probes/README.md
- date: 202609021238 (directory stamp); tree revision `7fed7b5932f31e77c8b288bd8aabbe93ec3b40aa`
- by: not stated
- holds: A mutation probe README asking whether anything goes red if four of the five assertions in `Slots::ADMITTED` are deleted. It reports the suite is identical (165 passed, 0 failed, 2 ignored) because the conditions are written twice and tests call the verdict copy.
- headline: "**Identical.** Not one test, doctest or `trybuild` case in the workspace changes colour when the obligation loses four of its five conditions, including both of the ones that are about what an `i64` holds." (line 31)
- watch: "The five conditions are written twice. Once in the const at `slots.rs:210` and again, independently, in `is_admissible` at `slots.rs:249`, which is what every test calls." (line 37)
### mock/research/202609021238_the-admitted-blocks-answer-q31-not-q29.md
- date: 202609021238 (name stamp); registry and tree at `7fed7b5932f31e77c8b288bd8aabbe93ec3b40aa`
- by: Seat q29b
- holds: A second seat's independent reading of the nine `ADMITTED` assertions across four contracts in `arvo-format`. It finds exposure is settled by a ratified door ruling, that three carrier-bound assertions fold hosting into admission (Q31 option one), that a topic seam in the stamped proposal is untracked, and (via a mutation probe) that four of five `Slots::ADMITTED` conditions are guarded by no test. A reconciliation section appended after reading prior panel files says which findings have prior instances.
- headline: "**Finding 3.** Rows 3, 6 and 7 of the table in section 3 answer `question::one_word_or_two_for_is_a_number_system` in favour of its first option, inside the design and code tiers, on a row that is open and whose `decider` is `panel`." (line 218)
- watch: "**Delete four of the five assertions in `Slots::ADMITTED` and the entire workspace stays green.**" (line 326)
- watch: "**Each of those three is a sentence about a container in this implementation.** `62` is `i64`. `i64::MAX` is `i64`. \"what an exponent carries\" is the `i32` behind `Exponent`." (line 158)
- watch: "The trait is open and `ADMITTED` closes it at 62. `mock/crates/arvo-format/DESIGN.md.tmpl:598` makes the openness claim in the design tier too" (line 198)
- watch: "\"There is no guard inside a function, because a guard is a thing that can be deleted and an absent impl is not.\"" (line 596)
- watch: "So the carrier bounds are indexed by this crate's chosen types and by no target." (line 430)
- watch: "`ruling::never_a_runtime_check_and_one_lowered_path` is what forces the check to be a const rather than a runtime guard" (line 262)
### mock/research/202609021400_q31_two_words_and_the_second_is_indexed_by_this_implementation.md
- date: 202609021400 (name stamp); tree at `26f65faf`
- by: Seat q31a
- holds: A blind seat's answer to open question Q31: option two (two words), because one word contradicts ratified and in-force canon sentences. It refutes option three by measurement (no hosting refusal is target-indexed), classifies the nine `ADMITTED` assertions six concept to three hosting, and reports that deleting the `WIDTH <= 62` clause leaves the suite green, with a closing construction.
- headline: "**Verdict: option two.** Two words, because one word contradicts three canon sentences that are already ratified or in force, and a vocabulary choice that falsifies a ratified row loses to it by provenance rather than by argument." (line 3)
- watch: "`Slots::ADMITTED` refuses a 63-bit range because `2^63` does not fit the `i64` **this crate** counts slots in. Widen `Slot` to `i128` and the refusal moves, on every target at once." (line 196)
- watch: "Every hosting bound is carried by `Radix(u32)`, `Exponent(i32)`, `MagnitudeCount(u32)`, `Slot(i64)` and `Width(u32)`, all fixed-width. With the declared coordinates held fixed, no assertion in any `ADMITTED` block can change truth value between two targets." (line 154)
- watch: "`SpanTooWide`'s own comment records that before the obligation was strengthened, `slot_count` panicked under `overflow-checks` and returned `-9223372036854775808` without it." (line 259)
- watch: "`WIDTH <= 62` is the crate's single clearest hosting condition. **Delete it and all 134 tests stay green.**" (line 265)
- watch: "in op's own words \"already in place, enforced by the mockspace lints and the workspace and repo rules, and they are **not to be questioned**.\"" (line 135)
- watch: "`Ambient::ADMITTED` carries **one** clause, radix at least two, and nothing about hosting at all, and the crate then instantiates it with `BinaryRationals`, `UnsignedBinaryRationals` and `DecimalRationals`, whose domain is the rationals." (line 244)
### mock/research/202609021401_phase_denominator_locus_b.md
- date: 202609021401 (name stamp)
- by: Seat B
- holds: An independent seat's answer on where a zero `Phase` denominator is refused and what the canon says. It finds the condition canon by entailment and the locus not canon, measures eighteen probe arms showing `Format::ADMITTED` is forced at only two sites and can be bypassed or disarmed, and finds the design contradicts itself about when the obligation fires. It composes four constructions rather than picking one and lists what is owed.
- headline: "**The canon requires the condition, requires that it be caught at compile time with no runtime check, and leaves the construction to the design tier under a functional test.**" (line 181)
- watch: "Runtime code can exist. Its branching should be done as much as possible with const-time ifs erased by monomorphisation, const solving and ultimately the backend. Never any runtime checks, ever: invalids are caught at compile time and unused paths are cleared when lowered. **Unless there is a justifiable reason that is blessed by a lead designer.**" (line 143)
- watch: "I do not think I get the framing. Monomorphisation and const solving should lead everything to go through one lowered path, that's it. So if a law is a law, it should be expressed so that it actually works, be it typestate or const expressions or whatever. We shouldn't police what kind of laws there are or what shapes they take." (line 122)
- watch: "A plain `const fn` carrying an `assert!` refuses at a `const` call site and **panics at run time**, because a `const fn` is also an ordinary function and `Phase::of` is `pub`." (line 150)
- watch: "Usage, Admissibility, Self-validation, All that makes sense." (line 159)
- watch: "Each obligation is a const evaluated at monomorphisation, so it fires during codegen. `cargo build` refuses. `cargo check` does not, because it skips codegen." (line 211)
- watch: "**\"`cargo check` does not\" is false.** Arms C and G refuse at `cargo check`, exit 101, because the call site is a `const` item and a `const` item is evaluated at check time." (line 225)
### mock/research/202609021406_phase_denominator_locus_a.md
- date: 202609021406 (name stamp); read at `67ce00d8`
- by: not stated (a seat answering independently of Seat B)
- holds: An independent answer on where the zero phase denominator is enforced. It finds the canon requires compile-time catching and forbids run-time catching but names no site, and measures eleven probe arms showing a non-denoting format reaches a running binary on four paths. It also checks the design against the chain rule and reports two stale registry claims and a rejected agenda-row framing.
- headline: "**The canon requires the condition to be caught at compile time, forbids it being caught at run time, and names no site.**" (line 28)
- watch: "Runtime code can exist. Its branching should be done as much as possible with const-time ifs erased by monomorphisation, const solving and ultimately the backend. Never any runtime checks, ever: invalids are caught at compile time and unused paths are cleared when lowered. **Unless there is a justifiable reason that is blessed by a lead designer.**" (line 47)
- watch: "Monomorphisation and const solving should lead everything to go through one lowered path, that's it. So if a law is a law, it should be expressed so that it actually works, be it typestate or const expressions or whatever. We shouldn't police what kind of laws there are or what shapes they take." (line 67)
- watch: "Again, we don't need to settle for one universal solution, it's the anti-pattern I've already named. Case by case. [...] Take the win where it applies, gate it out from where it does not. No single one-fits-all solutions, it's impossible" (line 79)
- watch: "if it's a rule that can't be avoided or designed away, a law, or something that constraints the work of the design, it's canon" (line 123)
- watch: "`adapt::<Signature<Broken, Adapt<HalfEven, Saturate>>>` is the applied map, `apply.rs:349`, the path a value actually takes. It returns slot 3." (line 203)
- watch: "A9 measures both false. At `Phase::of(1, 1)` with `Constant<0>` and `Signed<8>`, a nonzero phase, the shipped `has_additive_identity` returns `true`, which is the corrected claim." (line 388)
### mock/research/202609021500_probes/README.md
- date: 202609021500 (directory stamp)
- by: not stated
- holds: A README for the Q31 probe set: arms that build and refuse `Slots::ADMITTED` at 62 versus 63 bits, a carrier library that holds a declaration fixed while changing the machine type of a slot index, and cross-target invariance and mutant arms. It describes a classification instrument that separates assertions about the machine from assertions about the grid.
- headline: "**Hold the declaration fixed, change only the machine type a slot index is carried in. An assertion whose verdict moves is about the machine. An assertion whose verdict does not is about the grid.**" (line 38)
- watch: "Both operands are `Signed<32>`, which arvo admits. The set the exact product of two of them occupies is refused by arvo's own admission obligation, at `slots.rs:219`, on the ground that a slot index is a signed 64-bit integer." (line 22)
- watch: "`cross/invariance` | builds on every target | The admitted set, its exact boundary either side, and the endpoints of the widest admitted grid, asserted at check time and identical on four targets spanning two pointer widths, three operating-system values and two architecture families." (line 29)
- watch: "Everything here is a spike. It checks one thing each. None of the naming, none of the shapes, and none of the incidental choices is a design decision" (line 13)
### mock/research/202609021500_q31_one_word_or_two.md
- date: 202609021500 (name stamp); tree at `26f65faf`
- by: Seat q31b (branch `research/q31-q31b`)
- holds: A second blind seat's answer to Q31: two words. It derives this from the ratified format spine (the exact product of two admitted `Signed<32>` operands is refused by `Slots::ADMITTED`), shows by carrier mutation that the width refusal moves with the slot-index type while the mathematical refusal does not, and measures target invariance on five targets. It disagrees with the earlier reading that Q1 licenses folding, and lists registry acts owed.
- headline: "**Verdict, staked before I read anything but the registry and the shipped crate: two words. Not because the split is tidy, but because the ratified format spine is unstateable without it, and the shipped tree already welds the two into one const and is wrong to.**" (line 3)
- watch: "**Under one word, the set in which arvo's own arithmetic is defined to happen is not a number system.**" (line 179)
- watch: "**Q1 does not license folding, and read exactly it is evidence the other way.**" (line 353)
- watch: "`ruling::the_operating_constraints_are_intents_and_rules`, key `I14`, `rung = \"in_force\"`, `topic = \"operating_constraints\"`, whose `says` enumerates **\"no platform dependency\"** and whose `quote` is \"not to be questioned\"." (line 257)
- watch: "**The mathematical refusal is invariant under the carrier change and the width refusal is not.**" (line 225)
- watch: "It builds on **aarch64-apple-darwin, x86_64-unknown-linux-gnu, i686-unknown-linux-gnu, thumbv6m-none-eabi and msp430-none-elf**: pointer widths 16, 32 and 64" (line 240)
- watch: "*a residue clause is one whose verdict changes when a carrier type changes, and a concept clause is one whose verdict does not.*" (line 487)
### mock/research/202609021600_q31_attack_the_route_breaks_and_the_verdict_survives_on_one_leg.md
- date: 202609021600 (name stamp); tree and registry at `2a274940`
- by: Seat q31-attack
- holds: An attack on the two earlier Q31 seats. It argues neither central refutation of option one holds (an ambient domain is a component, not a candidate system), shows the carrier classifier measured the experimenter's edit, builds a derived-bound form that makes the classifier a measurement, and finds three of nine `ADMITTED` clauses neutralisable with the suite green. It keeps option two with the second predicate defined by a test, as cost rather than contradiction.
- headline: "**So the honest verdict is cost, not contradiction**, and the difference matters because \"one word contradicts the ratified spine\" is a sentence a consolidation would carry forward and it is false." (line 163)
- watch: "> A clause belongs to the second word exactly when its bound is a function of a carrier's range. Written as such a function, the clause moves when the carrier moves and the concept clauses do not." (line 308)
- watch: "`Signed<62>` is the widest range arvo admits and arvo admits it. The exact product of two operands at its top sits at slot `2^122` in the target's own units. `Slot` carries an `i64` (`slots.rs:41`), so: error: literal out of range for `i64`" (line 120)
- watch: "The rewrite is `pub type SlotIndex = i64;` named once, `Slot(SlotIndex)`, `Self::WIDTH.count() <= SlotIndex::BITS - 2` and `< SlotIndex::MAX as i128`." (line 224)
- watch: "Three of the nine `ADMITTED` clauses can be neutralised with the whole workspace suite staying green: `slots.rs:215`, `:219`, `:228`." (line 427)
- watch: "Op's `quote` on that row names the three readings in four words, \"Usage, Admissibility, Self-validation, All that makes sense\", and says nothing further about any of them" (line 375)
- watch: "`quantum.rs:186-194` names this exact hazard in its own doc comment, that \"widening any one of them would break the narrower form silently while every test still passed\"" (line 230)
### mock/research/202609030724_section_twelve_reconciliation_against_195.md
- date: 202609030724 (name stamp)
- by: not stated (a seat; the twelfth section of the composition-cluster file, reconciling against panel file `195`)
- holds: A reconciliation of the composition-cluster obligation file against panel file `195`. It records one agreement about the `obligation` schema having no way to say a half-discharged obligation, and one disagreement: two registry rows that `195` calls one convergence are adjacent but different claims, so pairing them would inflate a one-expert standing.
- headline: "I do not think they are the same claim, and I think recording them as one convergence would raise a one-expert row to two by pairing it with an arrival that is not an arrival at it." (line 45)
- watch: "unsigned saturating addition associates on every triple at eight bits and the composition of that addition with the matching subtraction fails on 82.7484 per cent of them." (line 52)
- watch: "`dl_interior_wrapping_with_a_reserved_absorbing_top` is named in the min-plus row's `gap` as a retired route into exactly this territory." (line 80)
### mock/research/202609030724_the_composition_cluster_is_four_obligations_and_two_rows_are_gateable.md
- date: 202609030724 (name stamp)
- by: not stated (a seat sent to judge promotability of ten proposals; it says it is not an independent instance)
- holds: A seat's review of the ten proposals reaching `obligation::composition_contracts_above_the_numeral`. It finds the obligation is four demands rather than one, that two rows are gateable after a mechanical repair, and that eight at one expert are not promotable. It rebuilds the min-plus DAG probe from its specification (headline reproduces, the max-plus control does not), reports a wording defect in the equality clause, and lists blockers met in the tree.
- headline: "The answer is that two of them are within one mechanical repair of being promotable and eight of them are not promotable at any level of argument, and that the more useful finding is the one underneath: the obligation is not one thing, the ten rows do not answer one demand" (line 6)
- watch: "unsigned saturating addition associates on every triple at eight bits and the composition of that addition with the matching subtraction fails on 82.7484 per cent of them" (line 192: \"13,882,880 of 16,777,216 triples fail, 82.7484 per cent, with the negative control under wrapping at zero as a group requires\")
- watch: "So the mechanism is mostly not absorption, and the row's own `gap` already names the other half without pricing it: absorption is one of the two properties a min-plus computation needs and monotonicity is the other" (line 410)
- watch: "What holds is that a min-plus reduction over a bounded numeral is correct when overflow maps in the direction the reduction discards, of which the top absorbing the sentinel is the special case applied to one value. Saturation gives both because it maps every overflow upward and minimum throws upward things away. Wrapping gives neither." (line 419)
- watch: "an ordinary candidate `dist[a] + w` can overflow and wrap small even when every exact intermediate is in range, because the exact intermediate is the minimum and the overflowing candidate is one that loses the minimum, so the in-range control never sees it. Under saturation the same candidate clamps to the top and the minimum discards it" (line 404)
- watch: "On the stored-pair rational the lifted equality is a congruence with zero violations of 57,600 ordered pairs, and it fails adequacy on 688 of the same 57,600, first witness `(0/1)` against `(0/2)`." (line 264)
- watch: "the workspace rule `.claude/rules/cl-claim-sketch-discipline.md` says the word `probe` is retired in favour of `sketch`, to keep the vocabulary monosemous. arvo's canon carries `probe` as a registry namespace key" (line 537)
### mock/research/202609191400_core_shadowing_probe/MANIFEST_RENAME_IS_UNGUARDED.md
- date: 202609191400 (directory stamp)
- by: not stated
- holds: A write-up of a fixture testing whether a `Cargo.toml` dependency renamed to the key `core` changes what the leading-`::` path `::core::primitive::usize::BITS` reads. Four of eight dependents build and read the stand-in's 8, and the lint, which reads `.rs` source only, does not refuse any of these renames. It lists what is not shown (other dependency tables, other editions).
- headline: "The lint reads `.rs` source only, through `ctx.all_sources`, and never opens a `Cargo.toml`, so none of these renames is refused by it." (line 92)
- watch: "So four of the eight dependents build, and in each of the four `::core` reads the stand-in's 8 rather than the pointer width." (line 75)

### mock/research/202609191400_core_shadowing_probe/arms.md
- date: none stated
- by: not stated
- holds: A generated table written by `run.sh` listing each probe arm (single-file shadowing arms, `manifest_rename` dependents, flip and premise variants, harness self-checks) with what it checks, what was expected and what was observed; it ends with zero unexpected outcomes.
- headline: "unexpected outcomes: 0" (line 86)
### mock/research/202611122300_value-stance-second-read.md
- date: 202611122300 (name stamp)
- by: not stated
- holds: A second read, made from the registry rows, of what the ratified rows say about the value a format holds. It answers four questions with rows quoted, sets the read against the stance file's ten stances, classifies the 92 questions with no ruling edge, and names the proposal rows filed beside it and the fate of each crate of the old tree.
- headline: "So an accumulator type is licensed, as a name the rows make necessary for a composite claim and nothing forbids, and it is not owed by them; its width algebra is a proposal's to state." (line 65)
- watch: "Panic is the go-to for what cannot be caught at compile time through the typestate, on debug builds, in every strategy rather than only where a native primitive is being imitated." (line 77)
- watch: "Never any runtime checks, ever: invalids are caught at compile time and unused paths are cleared when lowered. Unless there is a justifiable reason that is blessed by a lead designer." (line 73)
- watch: "Idea is to make things forced sound, which means usually panic over saturation or overflow" (line 79)
- watch: "This read takes (b), with the default asserting in every strategy as the row demands." (line 89)
- watch: "Exported `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, and the predicates `IsZero`, `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`." (line 261)
- watch: "Exported `Widen`, `Widened`, `Narrow`, `Narrowed`, re-exported from the bit contracts." (line 265)
### mock/research/202611130200_bench-harness-at-head/FINDINGS.md
- date: 202611130200 (directory stamp)
- by: not stated (names the job "arvo-bench-harness-alive, closing fault A6")
- holds: A report of running the mockspace bench harness at head on bitpack-carrier-width on an x86_64 Xeon host. It records where results were committed, the three runs, five observations O1 to O5 on what did not work as the brief said, and whether the harness can commit a disassembly of a named function.
- headline: "The two sets are not comparable: the host, and with it the cache sizes, differ, and so does the size at which the working set leaves L2." (line 7)
### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n1048576_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 1048576 ops per call on the x86_64 Xeon host: five variants (d16, d16-control, d32, d64, packed), 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-packed (178.01 us) leads bitpack-carrier-d16 (294.48 us) by 65%, a clear separation rather than a photo finish." (line 12)

### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n131072_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 131072 ops per call on the x86_64 Xeon host: five variants, 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-d64 (14.03 us) leads bitpack-carrier-packed (18.68 us) by 33%, a clear separation rather than a photo finish. CV 5.8%." (line 12)

### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n16384_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 16384 ops per call on the x86_64 Xeon host: five variants, 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-d64 (1.98 us) leads bitpack-carrier-packed (2.59 us) by 31%, a clear separation rather than a photo finish. CV 38.5%." (line 12)
### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n2097152_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 2097152 ops per call on the x86_64 Xeon host: five variants, 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-packed (498.20 us) leads bitpack-carrier-d32 (613.57 us) by 23%, a clear separation rather than a photo finish. CV 7.9%." (line 12)

### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n4194304_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 4194304 ops per call on the x86_64 Xeon host: five variants, 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-packed (1.05 ms) leads bitpack-carrier-d32 (1.19 ms) by 13%, a clear separation rather than a photo finish. CV 5.9%." (line 12)

### mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n8388608_findings.md
- date: none stated
- by: not stated (harness-generated findings file)
- holds: Generated findings for the bitpack-carrier-width bench at 8388608 ops per call on the x86_64 Xeon host: five variants, 40 samples each, d16 baseline, with highlights, tables, per-pass consistency and distributions.
- headline: "bitpack-carrier-packed (2.14 ms) leads bitpack-carrier-d32 (2.37 ms) by 11%, a clear separation rather than a photo finish. CV 8.2%." (line 12)
<!-- END -->
