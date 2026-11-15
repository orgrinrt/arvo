Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/arvo-stance.md, first posted 2026-11-12 21:14 UTC, read 2026-11-15.

# arvo: what the record already answers, the stances a first slice needs, and the slice

Author seat: Giesen. 12 November 2026. Read only; nothing was built, nothing was run, no tree was edited.

Three kinds of sentence are kept apart throughout:

- **CODE**: what a source file does, with `file:line`.
- **RECORD**: what a registry row, ruling, design or round says, with path and line.
- **MINE**: my judgement. Every MINE that says a thing fits the canon is a first read. A second, independent read is owed before anyone acts on it.

## 0. The two gates

**Canon gate: passed, with one thing flagged.** Checked against `/home/user/arvo/mockspace.toml:32` (`canon_paths = ["mock/registry/*.toml"]`) and the ruling rows quoted below. The ask (claim stances on open questions) is licensed three times over by rows that already exist, so it is not drift:

- RECORD `ruling.toml:1713-1719`, `the_panel_finishes_the_canon_without_him`, rung ratified: "Every remaining canon question is the panel's. [...] Nothing is parked awaiting him."
- RECORD `ruling.toml:1309-1313`, `steer_aggressively_to_convergence_without_rushing_it`, rung stated: "settling somewhere inside the constraints already formed is what is wanted."
- RECORD `ruling.toml:1392-1397`, two experts converging is a ratification, the coordinator holds the gate.

His words of 12 November say the same thing a fourth time. Flag: `mockspace.toml:24-28` says `mock check` refuses a change to the canon paths while a panel is open. I could not establish whether a panel is formally open at head. This file proposes rows and writes none.

**Test gate: not executed.** The brief forbids building, so the suite was not run and I do not know whether it is green. I lean on no suite and no bench for any claim below. Bodies I did read: `arvo-bits/src/tests.rs` (assertion lines; an independent folded-mask oracle with a negative control at `:111-133`, all 64 widths by macro at `:142-207`), the head of `arvo-format/tests/matlab_fi_parity.rs` (`:6-11` states its expected numbers come from MathWorks' page and not from arvo, with tie and overflow controls at `:58-106`), and the non-test source of all four crates. I found nothing tautological in what I read. I did not read the roughly 15,000 lines of `arvo-format` tests, so I make no claim about them.

One bench findings file was read in full (`mock/benches/bitpack-contention_n41943044_findings.md`). It is cited only for the fact that it exists and what its own table says.

## 1. The state of the record

### 1.1 Counts (counted here, not inherited)

RECORD `mock/registry/question.toml`:

| What | Count | How counted |
|---|---|---|
| Question rows | 110 | `[[question]]` headers |
| Rows carrying an `answered` field | 18 | `^answered = ` |
| Further rows answered by a ruling's `answers` edge, with no `answered` field | 6 | `ruling.toml:599, 839, 1056, 1196, 1386` (the last answers two) |
| **Answered by field or by a ruling edge** | **24** | union |
| Further rows that only a proposal names in `answers` | 26 | `proposal.toml`, `proposal-the-later-topics.toml` |
| Rows with no answer edge of any kind | 60 | 110 - 24 - 26 |
| Rows under a recorded deferral of his | 3 | Q6, Q7, Q8, see 1.3 |
| `decider`: op / panel / measurement / coordinator | 13 / 86 / 8 / 3 | `^decider = ` |

The panel's "18 answered, 92 unanswered" (`panel-consolidation-1.md:487`) and the two seats' "20 by F's grep, 18 by L's" (`stamps-1.md:174`) are all counts of one field. The file header says why that field undercounts, RECORD `question.toml:16-19`: "No answer is recorded here, including for the rows whose source records one. [...] What answers a question is a ruling or a proposal naming it, and the edge is drawn from that side." The honest figure is 24 answered, 26 with a proposal on file, 60 untouched. The census the panel asked for is therefore a classification of 86 rows, not 92.

Other namespaces, for scale: 102 rulings (38 ratified, 58 stated, 5 open, 1 in force; 6 of kind `deferral`), 127 proposals (92 at one expert, 23 at two, 6 at three or more, 6 cross topic), 24 laws, 16 obligations, 116 probes, 196 retirements.

### 1.2 Where the redesign stands

- CODE `mock/Cargo.toml:25-30`: four crates, `arvo-format`, `arvo-placement`, `arvo-strategy`, `arvo-bits`. `mock/Cargo.toml:3-12`: the old tree was deleted on 2026-08-08 and these four are the first source written from the canon.
- CODE: no crate enables any `#![feature]` gate (grep of the four `lib.rs`: none). Toolchain `nightly-2026-05-28` (`rust-toolchain.toml`).
- RECORD `README.md:23`: "there's no number to compute with yet. `UFixed` and the other points name a format and hold no value, what a value of one is hasn't been decided".
- CODE `arvo-format/src/apply.rs:250`: `adapt<S>(Exact, Dither) -> Slot` is the whole realised arithmetic. It is an interpreter over an `i128` slot index (`slots.rs:46`). The only admitted operation is addition (`addition.rs:230, 251`). There is no subtraction, negation, multiplication, division or conversion.
- The clone is shallow (`git rev-parse --is-shallow-repository` prints `true`, 31 commits, head `d841c5e` of 2026-10-23, only `dev`). `1b2796e9`, the rev hilavitkutin pins (`hilavitkutin/mock/Cargo.toml:39-48`), is **not reachable** here (`fatal: Not a valid object name`). What the ten pinned crates contained cannot be established from this clone. The `mock/research/arvo-*-history.md` files are 14 to 18 line stubs.

MINE: what is at head is a specification with an executable oracle and no product. That is a good thing to have under a product, and it is the reason a first slice is close: the hard semantic question, "what is the right answer", has a committed, tested, total function. What is missing is the thing that holds a number and computes that answer at machine speed.

### 1.3 The deferrals of his that touch the slice

Six rulings are of kind `deferral`. Three bear on questions a slice touches:

| Row | Line | Rung | Question |
|---|---|---|---|
| `the_carrier_question_waits_on_the_contention_measurement` | `ruling.toml:1170-1182` | open | Q7 |
| `wrap_or_clamp_stays_open_and_both_get_priced` | `ruling.toml:1038-1042` | open | Q6 |
| `the_family_question_wants_the_comparison_first` and `his_instinct_on_one_family_is_not_to_be_acted_on` | `ruling.toml:1122-1138` | open | Q8 |

The brief named only the first. The second matters as much and is handled in 2.4.

### 1.4 Which open questions a first slice depends on, and what the record says

"Depends on" means: a usable value type with arithmetic cannot be written without taking a position.

| # | Question | In the registry | Does the record already answer it? |
|---|---|---|---|
| D1 | What is a value of a format, and may a type hold one | Q4 `what_a_datum_stands_for`, `question.toml:93-106`, no answer edge | **Yes, in ratified rows.** See 1.5. |
| D2 | What a single value sits in | Q7 is adjacent but is about packing; `container_derivation_output_count` is answered | **Yes.** `ruling.toml:1531-1536` ratified: "one output at sole occupancy, the carrier". CODE `arvo-placement/src/lib.rs:205-230` implements it. |
| D3 | How the carrier is chosen at the type level without forbidden features | `ruling.toml:1151-1167`, "fresh eyes" | **Partly.** A committed spike shows a mechanism. See 1.6. |
| D4 | Which rounding and overflow a value has | Q6 open and deferred; `the_rounding_mode_vocabulary` answered | **Yes for the mechanism, no for the Warm preset.** Rounding and overflow are coordinates the consumer declares (`ruling.toml:1677-1697`, `adapt.rs:27-77`). Only the preset cell is open. |
| D5 | Which operations exist | `which_operation_set_the_design_ships`, answered | **Yes.** An admission rule, not a list: "an operation is admitted exactly when it is a function of the declared signature" (`ruling.toml:1536`). |
| D6 | Mixed format arithmetic | Q3 `mixed_numeral_addition`, `question.toml:57-90`, no answer edge | No. Stance in 2.5. |
| D7 | What a debug build does on overflow | `does_the_native_imitation_cover_the_debug_panic`, answered by `ruling.toml:1052-1058`, ratified | **Yes, and it bites.** See 2.6. |
| D8 | Which width pair a consumer writes | Q2, answered: "a projection choice, and it is not a canon question. [...] It belongs to the design round for the format" (`question.toml:40`) | Yes: design tier. Head chose total width and quantum exponent. A trap sits here, see 2.7. |
| D9 | Platform width | `what_a_platform_width_type_is`, answered, `ruling.toml:1750-1756` ratified | Yes. |
| D10 | Const generic positions | obligation `a_primitive_for_every_position_a_bare_number_would_take`, `obligation.toml:147-155` | **Yes, in his own words.** See 2.8. |
| D11 | Division where the divisor is zero | nothing | No. Stance in 2.9. |
| D12 | Capacity as a type | `design_rounds/202609190948/202609190946_topic...md:~30-37`: "whether a capacity position is such a case is not answered here" | No. Stance in 2.10. |

Questions a first slice does **not** depend on (it can ship without touching them, and nothing in it forecloses them): everything under `the_chain` (9 rows: folds, reduction order, chain carriers), `the_strategy_object` (5), most of `the_strategy_axis` (weightings, cost coordinates, selectors, exchange rates), `algebraic_laws` routes and naming (7), `the_predicate_notation` (5), `canon_form` (8), `panel_conduct` (3), stochastic rounding and dither (5 of the 9 rounding rows), set valued carriers, non magnitude number systems, crossing classes, the numeric door's type count (`ruling.toml:1740`, deliberately left open), Q8 one family or several (the slice instantiates points of the one ratified predicate and takes no position on whether a second family exists), and Q7 itself. That is roughly 95 of the 110 rows. MINE: the circling he describes is real and it is circling questions the base does not need.

### 1.5 D1: the record already says a datum is a point, and already licenses a value type

Q4's four options (`question.toml:96-101`): a point; an absorbing top; a constructor level clause; a set, admitted generally. The row carries no answer edge, and two agent written documents treat it as the blocker:

- RECORD `design_rounds/202609190825/202609190823_topic...md:127-128`: "What a value of a numeral is, and where it lives, is `question::what_a_datum_stands_for`, open with the panel as decider, and filling it inside this design would be filling an open question. It is handed back."
- RECORD `arvo-format/DESIGN.md.tmpl:243-244`: "a numeral value type carrying a stored slot, which no row licenses and which would be a second placement beside the one the derivation performs by occupancy".

Both are design tier, agent written, and by the provenance rule presumed wrong where they meet ratified rows. They meet four:

1. RECORD `ruling.toml:1436-1447`, `the_format_spine_is_canon`, ratified: "A format is identified by its ambient domain and its representable set [...] Arithmetic on a format is an exact operation in the ambient domain composed with a named total adaptation onto that set". A representable set is a set of points of the ambient domain, and the adaptation maps onto it. A member is a point. Option one.
2. RECORD `ruling.toml:1824-1829`, ratified: "Compositions over formats, stored-pair rationals, intervals, error-carrying values, are not format instances; they consume the format concept and owe their own laws." That is option four placed: set valued data exists, as a composition above the format, not as what a datum of a format is.
3. RECORD `ruling.toml:1786-1791`, ratified: "`top + x = top` for every representable `x`". The top is treated as an element with an algebraic property, which is option one's own wording ("saturation's absorbing behaviour is a documented restriction on where a fold is sound rather than a denotation"). Option two survives as a law about folds, not as a denotation.
4. RECORD `ruling.toml:1459-1470`, ratified: the footprint is observable "through exactly one observation the design does not own, the host language's layout observation on a sized type, and that observation exists only where the value is the sole logical occupant of its allocation." A sentence about `size_of` on the type that holds a value presupposes the type.

And the demand side, RECORD `obligation.toml:147-150`, quoting him: "We want arvo there. No bare usize other than in const generics for smoother and more ergonomic api, and even there, only when truly painful otherwise." A primitive for every position is a value type. `obligation.toml:82-85` asks for "an unsigned integer of the platform's size, usable in a public error type".

MINE, stated plainly: the sentence "no row licenses" a value type (`DESIGN.md.tmpl:243`) is false against four ratified rulings and two obligations, and the "second placement" objection is a category error. A value type whose representation **is** the sole occupancy output of the placement derivation is the first placement, used. The design mistook "not in this crate" (correct: `arvo-format/src/lib.rs:19-22` forbids a carrier there) for "not licensed anywhere". The cost of that mistake is everything since 2026-09-19: a numerics family with 22,000 lines and no number, and two consumers proxying around it. This is a first read. A second is owed, and it should be formed from the four rows before reading this section.

What stays honestly open inside Q4 is the sub fork its note names (`question.toml:104`), soundness against bestness of a derived numeral. The slice derives no numerals (2.5), so it does not touch it.

### 1.6 D3: the mechanism spike

RECORD `mock/research/sketches/202607282100_container-projection-without-gce/FINDINGS.md:4, 26-28, 43-64`: a const computed in type position is replaced by a width typestate and an associated type, so "`W::Bkt` is a path. Nothing computes in type position", and the sketch compiles with zero feature gates. Its cost section (`:77-80`): "Per-width impls, one row per supported width per family [...] the real crate expands its full range by macro."

What I cite it for: the existence claim that a width to carrier type map needs no gate. What I do not carry: its shape. It was written against the deleted tree and keys the carrier on the strategy (`:71-72`, Warm at 13 bits resolving to `u32` under a "2x-logical rule"), which the later ratified ruling replaced with occupancy and objective (`ruling.toml:1531-1536`, and CODE `arvo-placement/src/lib.rs:205-212`: at sole occupancy "both objectives give it"). I did not rebuild it. It is one instance; the first job of step 4 in section 4 is a second and third.

A tension to put in front of him rather than resolve: his recall (`ruling.toml:1157-1162`) is that the good attempt "did not contain any enumeration". The spike's own cost section is a per width impl table expanded by macro. Either that is not "enumeration" in his sense (no enumeration in a signature, one internal macro site), or the attempt he remembers is a different one that this shallow clone does not show me. Call 7 in section 5.

## 2. The stances

Each stance is sorted: **BITS** (changes a computed bit or a stored byte: right in the first release, or versioned) against **IMPROVABLE** (inventory, speed, ergonomics).

### 2.1 Stance A: a value is the slot index of a member, held at sole occupancy in the carrier the placement derives. BITS.

`Num<S>` for a declared signature `S` stores exactly one thing: the slot index of a member of `S`'s format, in the narrowest ladder carrier that covers the declared width (CODE `arvo-placement/src/lib.rs:194-203`), in canonical form: sign extended for the signed slot family, zero extended for the unsigned. One bit pattern per member.

- **Frozen by it:** the canonical encoding (so equality, order and hash on the carrier are equality, order and hash of the denotation, and `to_bits` output is stable), and the carrier rung for each declared width (it is observable through `size_of`).
- **Improvable later without breaking users:** every internal arm. Any wider register, any SIMD lane, any fused form, as long as the stored result is the same slot.
- **Compromise:** a 13 bit value costs 16 bits at sole occupancy. That is the ratified reading, not a new cost.
- **Set aside:** (i) one `u64` or `i64` carrier for everything. Simpler, no per width table, and wrong against the ratified sole occupancy derivation; it would also make a later narrowing a layout break. (ii) a value type for the integer points only (the third option at `202609190823_topic...md:131-132`). It would get hilavitkutin moving and leave kaski proxying, and the general form costs nothing extra once the carrier map exists. It comes back only if step 4's carrier map fails for fractional formats, which nothing suggests. (iii) the consumer carries values in `Bits<N>` (the first option there). CODE `arvo-bits/src/lib.rs:11-13`: "It is not a numeral: there is no sign, no ambient domain, no quantum, no rounding, no arithmetic". Refused by its own charter.

### 2.2 Stance B: the law of every operation is equality with the oracle. BITS, and it is the whole correctness contract.

For every admitted operation `op` and every pair of members:

```
slot(op_num(a, b)) = adapt::<S>(exact_op(slot(a), slot(b)), Dither::UNUSED)
```

The left side is whatever the machine does fastest. The right side is the committed interpreter (`apply.rs:250`, `addition.rs:251`). This is the canon's factoring (`ruling.toml:1447`: "an exact operation in the ambient domain composed with a named total adaptation") used as a test oracle.

- **Frozen:** the function. Nothing else.
- **Improvable:** all code on the left side, forever, under `ruling.toml:1479-1485` (arms over regions with const predicates) and `ruling.toml:1656-1662` (take every win, however small the region).
- **Compromise:** the oracle is `i128` and bounded at 64 bit slot ranges, so the slice is bounded at 64 declared bits. CODE `arvo-placement/src/lib.rs:80-83` already names that bound as real and held open by the canon (`arbitrary_width_demands_in_the_canon`, Q28).
- **Set aside:** stating laws per operation as hand written expected values. That is how kaski's `Fixed` and its canon came to disagree unnoticed (2.3).

### 2.3 Stance C: rounding and overflow are always written by whoever stores the bytes. BITS.

RECORD `ruling.toml:1677-1683` ratified: six rounding names. `ruling.toml:1691-1697` ratified: `half_up` is ties toward positive infinity. CODE `rounding.rs` ships all six; `overflow.rs` ships `Wrap`, `Saturate`, `Clamp`.

The value type is keyed on a declared signature, which is a format and an `Adapt<R, O>` (`adapt.rs:39-77`). So "both ways would have users, configurable via generics" is already the structure: `R` and `O` are type parameters with five deterministic modes and two usable policies.

For kaski this is not a preference. RECORD `kaski/mock/registry/tenet/engine.toml:540-542`, him, 1 November: "Round to nearest, half to even, in mul; every digest moves once, in F1b." CODE `kaski/mock/crates/kaski-engine-carrier/src/fixed.rs:173-176`: `let wide = (self.0 as i128) * (other.0 as i128); Self::saturate(wide >> FRAC)`. An arithmetic right shift is floor. **Kaski's code computes a different bit than kaski's canon rules, today.** The doc comment at `fixed.rs:32-35` says so itself ("rounds toward negative infinity"). The panel found this (D7, `panel-consolidation-1.md:483`, one seat); I confirm it independently from the two files. It is canon drift in kaski and it is the first thing to fix, because every differential test of arvo against the proxy is otherwise a test against a wrong reference.

In arvo's terms kaski's quantity is `Signature<Fi<64, 16>, Adapt<HalfEven, Saturate>>` (CODE `standards.rs:58-72`, `Fi<W, F>` is signed, fraction length `F`). Its wide form is `Fi<64, 30>`.

- **Frozen:** for each `(format, R, O)` the function of 2.2. `Clamp` is **not** shipped on values: CODE `apply.rs:224-229` says the declared signature "carries nowhere to put that bound", so today it silently equals saturation. Shipping a name whose meaning is "missing a coordinate" freezes a wrong bit. It returns when the coordinate exists.
- `Stochastic` is not shipped on values in the slice: it needs a dither source, and both rows about that (`does_a_consumer_supplied_seed_surface_exist`, `does_the_position_keyed_dither_arm_ship`, `question.toml:1392-1412`) were answered by a coordinator "in op's stead" with a note that the call "carries no authority". Not shipping it freezes nothing.
- **Test matrix cost:** 64 widths x 2 slot families x 5 modes x 2 policies = 1,280 signatures per fraction length. See 3.6.

### 2.4 Stance D: the presets ship as bindings with their cells marked unsettled; nothing that reaches a stored byte may be spelled with one. Honours the Q6 deferral.

His earlier ruling, RECORD `ruling.toml:1038-1042`: "Whether the intuitive-compromise strategy wraps or clamps is not settled. Both readings are explored and both are priced." His words of 12 November: "coldly claim a stance on something and state the compromise".

What the slice does that honours the earlier ruling: both `Wrap` and `Saturate` exist as explicit declarations and both get bench arms (3.7), which is "both get priced". `Warm` keeps the cell it has at head (CODE `arvo-strategy/src/lib.rs:139-145`, `Adapt<HalfEven, Wrap>`, under a `FIXME` naming Q6). No cell is declared settled.

What would need his word: closing Q6, that is, saying which way Warm goes.

A finding he should have before he does. RECORD `ruling.toml:575-580`, stated: Warm "should behave like native primitives in regular old Rust would". Head binds Warm to `HalfEven`. One adaptation serves every operation of a signature (`adapt.rs:63-69`, `:132-139`). The day integer division is admitted, `Warm` on an integer format computes `7 / 2 = 4` (3.5 is a tie, 4 is even), where native Rust gives 3. For add, sub and mul on integers the rounding never fires, so the defect is latent until division lands. MINE: the Warm rounding cell for integer formats is `TowardZero` if the stated intent holds; whether one preset may bind different rounding for integer and fractional formats is exactly the kind of thing `ruling.toml:591-595` ("the imitation is ergonomics, not an arithmetic boundary") leaves to design. Pin it as a red test now (3.6, T9).

- **BITS**, and deliberately **not frozen**: preset cells. README and rustdoc say so.
- **Compromise:** a type level guard is impossible, because `Uint<14, Warm>` and the explicit signature are one type. RECORD `ruling.toml:1470` ratified: "Identity saturates at the declared signature set." So the guard is a lint in the consumer over the spelling (kaski: no preset name in a type that derives a stored encoding or feeds a digest).
- **Set aside:** freezing the four cells now. That is option a of the panel's call 10 pattern applied to Q6, and it would supersede a deferral of his on an agent's word.

### 2.5 Stance E: one signature per operation; crossing formats is an explicit conversion. IMPROVABLE (additive).

Q3's third option (`question.toml:63`): "It exists only through an explicit conversion". CODE `DESIGN.md.tmpl:240`: "which is why addition takes one signature and not two".

A conversion `Num<S> -> Num<T>` is one adaptation of an exact position onto `T`, which is admitted by the rule as it stands. Chains of conversions are the open rows `does_narrowing_compose` and `what_the_double_rounding_mechanism_is`; the slice states the single step only and says nothing about two.

- **Frozen:** nothing beyond 2.2 for the conversion itself.
- **Compromise:** `a + b` across two fraction lengths does not compile; the consumer writes which side converts. Kaski already does this (`weight.rs`, `to_fixed::<24>()`).
- **What brings inference back:** the panel answering Q3 with option two and a derivation rule. Adding an inferred join later breaks no code that wrote the conversion.

### 2.6 Stance F: a debug build asserts on overflow per the ratified ruling, and a consumer for whom completion is the meaning says so in the type. Not bits. His call.

RECORD `ruling.toml:1052-1058`, ratified: "Panic is the go-to for what cannot be caught at compile time through the typestate, on debug builds, in every strategy [...] What stands in its place on release is the guarantee plus an explicit declaration of the mode, saturate or wrap or whatever applies". CODE `apply.rs:288-297`: `panic_on_overflow` returns a verdict and does not diverge.

Read literally, a debug build of kaski panics the first time a quantity saturates, and CODE `fixed.rs:32-35` says saturation is kaski's intended semantics, not its error path. Both ways have users. The parameter: the overflow marker carries a second associated const, whether completion is an error in debug (`Saturate`, `Wrap`: yes, per the ruling) or the declared meaning (a wrapper marker over the same policy: no). Same `POLICY`, same bits in release and in debug where nothing overflows.

- **Cost in the matrix:** none for value laws (the result slot is identical); one extra compile fail or should-panic test per policy.
- This changes whether a debug run aborts, so although it is not a bit it is not mine to pick. Call 4.

### 2.7 Stance G: consumer facing fixed point aliases take a fraction length, never a quantum exponent. BITS by accident.

CODE `arvo-format/src/lib.rs:144-156`: `UFixed<BITS, FRAC>` puts "the quantum at exponent `FRAC`", `Constant<FRAC>`. So sixteen fractional bits is `UFixed<32, -16>`. CODE `standards.rs:40-48, 58`: `Fi<W, F>` uses a fraction length, `BASE = Exponent::of(-F)`, so sixteen fractional bits is `Fi<32, 16>`. Two public spellings in one crate, opposite signs, both `i32`, both compile with either sign. A consumer who writes `UFixed<32, 16>` gets a format whose quantum is 65,536, silently.

Stance: the value crate's aliases are spelled with a fraction length and the parameter name of the `points` struct is changed from `FRAC` to `EXP` (it is already `EXP` on `Biased`, `lib.rs:178`). Q2's answer (`question.toml:40`) puts this in the design round, so it needs no canon row.

### 2.8 Stance H: const generic parameters are bare integers. IMPROVABLE.

RECORD `obligation.toml:148-150`, him: "No bare usize other than in const generics". CODE `arvo-format/tests/ui/an_arvo_type_as_a_const_parameter.rs` with its `.stderr`: the refusal is pinned as a compile fail test, and the comment says "every const generic parameter in the crate is spelled with a machine integer". RECORD `obligation.toml:174-178`: "A consumer naming arvo's types needs no unstable compiler features".

CODE hilavitkutin does the opposite today: `{ Cap(USize(8)) }` and `from_constant::<{ USize(1) }>` (seen in `hilavitkutin-api` and `hilavitkutin-str` sources), under `#![feature(adt_const_params)]` (`hilavitkutin-api/src/lib.rs:13`). That is the old tree's surface.

- **Compromise:** hilavitkutin's const positions are rewritten to bare integers. The gate leaves its crates.
- **What brings the newtype back:** `adt_const_params` stabilising. Adding `ConstParamTy` then is additive.

### 2.9 Stance I: division takes a divisor that is non zero by type. BITS.

Division by zero has no exact result, so there is nothing to adapt. RECORD `ruling.toml:949-954` ratified: "Never any runtime checks, ever", with a lead designer's blessing as the only exception. CODE kaski `fixed.rs:180-183`: "Zero when `other` is zero", a runtime branch inside the operation.

Stance: `div` takes a refined divisor; the refinement is a classification the consumer performs and branches on in its own code. Kaski's rule becomes two lines in kaski and computes the same bits.

- **Set aside:** a pole policy as a third adaptation coordinate. It is the reading the admission rule itself suggests ("where two realisations of one name disagree, the signature is missing a coordinate", `ruling.toml:1536`), and it would have users. It is additive later as a defaulted parameter. It doubles the division cells of the matrix.
- Call 5.

### 2.10 Stance J: capacity is its own small crate; masks, hash, graph, sparse, spectral and refit are slice two. IMPROVABLE.

CODE hilavitkutin's real imports (grep over `mock/crates/**/*.rs`): `USize` in about 250 `use` lines and 761 constructor sites `USize(..)`; `Bool`; `strategy::{Additive, Identity}` in about 75; `arvo_tensor::{Capacity, cap_size, Dim, ConstCapacity, cap}` in 56 `use` lines; `arvo_bitmask` in 21; `arvo_bits` in 9; `arvo_hash`, `arvo_sparse` in 4 each; `arvo_refit` in 2; `arvo_graph`, `arvo_spectral` in 2 each; `arvo-comb` in a manifest and no source line I found. `hilavitkutin-api/Cargo.toml:10-12` needs `arvo`, `arvo-bitmask`, `arvo-tensor`.

RECORD `ruling.toml:473-479` ratified: "The numeral base has to work so the bigger things can be built on it." RECORD `ruling.toml:441-445`: "The build order is strictly ground up".

Stance: slice one is the numeral base plus capacity, which is what lets `hilavitkutin-api` types be written. Slice two, immediately after and in this order by import weight: masks (`obligation.toml:24-33`, single instruction lowering is part of the need), the content hash, then graph, sparse, spectral.

- **Compromise:** hilavitkutin as a whole does not build on arvo `dev` at the end of slice one. Its api crate's numeric positions do.
- **No `arvo` facade crate in slice one.** Consumers name the crate that owns the thing. Adding a facade later breaks nobody; removing one does.

### 2.11 What a first slice does about "Explore, wait for Fog."

His earlier ruling, exactly, RECORD `ruling.toml:1170-1182`: quote "Explore, wait for Fog."; `says`: "On which carrier the packing claim is about, explore and wait for the contention measurement."; `because`: "Every measurement in hand at the time was single-threaded while arvo's own rules declare the workload concurrent".

His words of 12 November, exactly: "Perhaps the thing needed there is to just coldly claim a stance on something and state the compromise and have a usable crate and api that can then be perfected and improved over time."

What a first slice can do that honours the earlier ruling: ship **sole occupancy only**. The question is about which carrier the claim "packing pays" is measured against, which is a claim about shared placement. A value alone in its carrier is on the other side of the occupancy axis (`arvo-placement/src/lib.rs:40-46`). No packed or shared container appears in the slice's public surface, and no carrier type appears in any consumer facing signature except the two declared unwrap doors. That is the panel's option c (`stamps-1.md:30`); I reach it independently from the ruling's own `because`.

Two facts the panel did not report:

1. **The thing waited for is in the tree.** `mock/benches/` holds twelve `bitpack-contention_n*_findings.md` files (column split 1, 2 and 4 ways at four sizes) and `retirement.toml:2101` records "Two committed files are threaded benches on the mockspace harness [...] measuring one, two and four threads over exactly the packing trade". The ruling row is still rung `open` and nothing in the registry closes it against them.
2. RECORD `question.toml:179`: he "answered the regime half at `32` and the register records it as dissolving rather than selecting: the canon names no regime, so both the one-core and four-core break-even bands are correct as the two arms rather than as rivals."

MINE: so the deferral's stated condition has been met, and his own later answer points at Q7's third and fifth options (no fixed carrier; arms per regime). What remains is named in the same note: "writes are unpriced [...] random and strided access are unmeasured". That is measurement work for slice two's packed container and blocks nothing in slice one.

What would need his word: marking the deferral row superseded. Call 2.

## 3. The first slice, concretely

Shapes below are sketches of contracts. Names, arities and module paths are scaffolding, not design.

### 3.1 Crates

| Crate | State | Depends on | Holds |
|---|---|---|---|
| `arvo-format` | at head, extended | notko | signatures, the oracle; gains exact steps for negation, subtraction, multiplication, division, conversion |
| `arvo-placement` | at head, unchanged in meaning | `arvo-format` | sole and shared derivation |
| `arvo-bits` | at head, unchanged | nothing | `Bits<N>` |
| value crate (working name `arvo-numeral`) | **new** | `arvo-format`, `arvo-placement` | `Num<S>`, the carrier map, doors, operations, identities, integer and fixed aliases |
| `arvo-strategy` | at head, gains one edge | adds the value crate | preset keyed aliases |
| capacity crate (working name `arvo-capacity`) | **new** | the value crate | count typestate and its array projection |

All `#![no_std]`, no alloc, no `dyn`, no feature gate. MINE: the slice needs **none** of the vetted gates. `const fn` inherent methods and associated consts on stable machinery carry all of it, which is what head already demonstrates across four crates. `const_trait_impl` would let `a + b` work in const context; the slice leaves it out so that `obligation.toml:174-178` holds trivially, and offers inherent `const fn add` plus ordinary non const operator impls. Kaski's call sites are already `.add(..)`, `.mul(..)` (about 700 and 480 of them).

### 3.2 The value type

```
trait Sole: DeclaredSignature            // sole occupancy realisation of a signature
    type Repr: Copy + Eq + Ord + Hash    // the ladder carrier, by the width typestate
    // ADMITTED: () forces: slots admitted; Repr is narrowest_carrier(declared_width)

struct Num<S: Sole>(S::Repr)             // repr(transparent), field private

invariant  I(v):  slot(v) is a member of S's format, and v's carrier bits are the
                  canonical extension of slot(v) at the declared width
```

Observation perimeter, stated where the invariant is stated: the field is private; there is one way in that is not an operation (`adapted`), one compile time way in (`constant`), and two declared unwrap doors out (`slot`, `raw`). No `Deref`, no `From<host integer>`, no public field.

Doors:

| Door | Shape | Check |
|---|---|---|
| `Num::<S>::constant::<const V: i128>()` | member or the build fails | `const { assert!(..) }`, compile time only |
| `Num::<S>::adapted(Exact) -> Self` | total; the declared adaptation is the ingest rule | none beyond the declared semantics |
| `Num::<S>::from_raw(S::Repr) -> Self` | total; `adapted` of the on grid slot. Identity and branch free where the declared width fills the carrier | none |
| `v.slot() -> Slot`, `v.raw() -> S::Repr` | unwrap doors, declared as such, as `Bits::raw` is (`arvo-bits/src/lib.rs:107-115`) | |

`adapted` as the ingest door is the piece I think the record had not put together. RECORD `ruling.toml:985-989`: "arvo may ship casting and conversion helpers and may not use them in place of the consumer". RECORD `ruling.toml:949-954`: no runtime checks. A rejecting constructor is a runtime check; a masking one (`Bits`) has no numeric meaning. The adaptation is already a total map from every exact position onto the representable set, with the consumer's own declared rounding and overflow. Ingest **is** adaptation. No new mechanism.

### 3.3 Operations and their laws

Admitted in the slice, each a `const fn` and each a function of `S` alone:

| Operation | Exact step in the ambient domain | Admitted for |
|---|---|---|
| `add`, `sub`, `neg` | sum, difference, negation at a common scale | every non magnitude indexed format (as `is_addable` today, `addition.rs:260`) |
| `mul` | product of two members, repositioned by the quantum | constant quantum, radix two, zero phase |
| `div` | quotient by a non zero member | same |
| `convert::<T>` | the same point, positioned on `T`'s grid | constant quantum, same radix |
| `eq`, `cmp`, `min`, `max` | exact, no adaptation | every format with a monotone slot to value map |
| `Identity<Additive>::IDENTITY`, `Identity<Multiplicative>::IDENTITY` | the member denoting 0, or 1 | only where the format has one (`has_additive_identity`, `format.rs`) |

Laws, all as equalities against the oracle or exact rationals:

- **L1 realisation.** Section 2.2, for every operation.
- **L2 closure.** Every operation returns a value satisfying `I`.
- **L3 canonical form.** `a == b` on carriers iff `slot(a) == slot(b)`; `cmp` on carriers is `cmp` on slots.
- **L4 door round trip.** `from_raw(v.raw()) == v`; `adapted(Exact::on_grid(v.slot())) == v`; `constant::<V>().slot() == V`.
- **L5 identity.** `add(a, ZERO) == a` and `mul(a, ONE) == a` for every member, where the identity exists.
- **L6 commutativity** of `add` and `mul`, every pair.
- **L7 exactness.** Where the exact result is a member, the result is that member, under every mode and policy.
- **L8 the registry's own verdicts are reproduced on values**, not only on slots: `law.toml:31, 52` (additive associativity: holds under wrapping, 952 divergent triples at four bits signed under saturation), `law.toml:84, 114` (multiplicative), `law.toml:145` (distributivity), `law.toml:342` (multiplicative inverse round trip). Associativity is exposed as the const verdict (`addition_is_associative`, `addition.rs:351`), never as a declared marker trait: the registry carries a retirement whose slug is `a_declared_law_marker_compiles_clean_when_false`.
- **L9 carrier independence.** For a format `F` at width `N`, computing in the derived carrier and computing in the widest carrier give the same slot. This is `ruling.toml:1470` as a test.
- **L10 target independence.** For every signature not spelled with a platform width point, the result slot is identical on a 64 bit native target and on wasm32.

### 3.4 What hilavitkutin's imports map to

| hilavitkutin writes (old tree) | Count | Slice one |
|---|---|---|
| `arvo::USize`, `USize(n)`, `.0` | 761 constructor sites | value alias over the platform width point with `Adapt<TowardZero, Wrap>`; `USize::from_raw(n)`, `.raw()`. Mechanical rewrite |
| `arvo::Bool`, `Bool(b)`, `Bool::FALSE` | about 80 | `arvo_format::Bool` (CODE `width.rs:95-131`); `Bool::of(b)` |
| `<USize as Identity<Additive>>::IDENTITY` | about 75 | same shape, 3.3 |
| `Uint<14, Warm>`, `UFixed<{ibits(2)}, {fbits(0)}, Hot>` | few | preset keyed alias in `arvo-strategy`: total width and fraction length as bare consts, strategy as a type. `ibits`/`fbits` wrappers go (2.8) |
| `{ Cap(USize(8)) }`, `Capacity`, `cap_size`, `Dim`, `<C as Capacity>::Array<T>` | 56 `use` lines | capacity crate: a count as typestate with an associated array projection and a const count; const positions bare |
| `Bits<28, Hot>`, `narrow_to`, `Widen` | 9 `use` lines | `arvo_bits::Bits<28>`, `.cast::<M>()` (CODE `lib.rs:128`). The strategy parameter on a bit container is gone, correctly: a proposal at one expert says "An axis the realisation map does not read must not be a type parameter" (`proposal-the-later-topics.toml:101-106`) |
| `arvo_bitmask`, `arvo_hash`, `arvo_graph`, `arvo_sparse`, `arvo_spectral`, `arvo_refit` | 21, 4, 2, 4, 2, 2 | slice two |
| `arvo-comb` | 0 source lines found | nothing owed until a need is stated |

### 3.5 What kaski's lane maps to

CODE `fixed.rs:54`: `Fixed<const FRAC: u32>(i64)`, private field, and `:36-39`: "The field stays private so the swap is a change inside this file." Widths in use by grep: `Fixed::<16>` 53 sites, `FRAC` (16, `settings.toml:596`) 17, `48` 7, `30` 4 plus `FRAC_WIDE` (30, `settings.toml:594`) 3, `8`, `24`, `32` once or twice.

| kaski | Slice one |
|---|---|
| `Fixed<F>` | wrapper over `Num<Signature<Fi<64, F>, Adapt<HalfEven, Saturate>>>` with the "completion is the meaning" marker of 2.6 |
| `add`, `sub`, `min`, `max`, `gt` | direct |
| `mul` | direct, and now half to even, which is the canon row at `engine.toml:540-542` |
| `div`, `scale`, `from_ratio` | `div` through the non zero refinement; the zero case stays a branch in kaski. Rounding: kaski floors today (`divide_down`); the canon row speaks only of the product. **Whether `div`, `scale` and `from_ratio` also move to half to even is not stated anywhere I read.** Call 10 |
| `to_bits`, `from_bits` | `raw`, `from_raw`; identity at width 64 |
| `sqrt`, `cbrt`, `exp`, `ln`, `exp_neg`, `pow` | **stay in kaski in slice one**, written over `raw`. Their bit level results are kaski's own series (`fixed.rs:208-240`, "within one unit of the last place"), and moving them upstream freezes them for every consumer. Call 9 |
| `from_decimal(f64)` | stays in kaski; it parses text |
| `to_f32_lossy` | stays in kaski; drawing only |
| 16 and 32 bit lanes (the panel's call 2, stamped option b) | `Fi<16, F>` and `Fi<32, F>` with the same adaptation; the intermediate is the double width register, which is an internal arm and not a declared coordinate. Which channel takes which lane is kaski's, after its floor bench |

Native and wasm bit equality holds by construction for these: integer only, no float, no pointer width in any computed path. It is still tested (T7), because "by construction" is what the proxy's author believed about its rounding. One guard cannot be a type: `USize` is, within one compilation, the same type as the literal width point (`arvo-format/src/lib.rs:214-219`, and the ratified ruling at `ruling.toml:1756`). So "no platform width value in a stored byte" is a lint over the spelling in kaski, not a trait bound in arvo.

### 3.6 Tests and compile fail tests

The matrix: width 1..=64, slot family 2, rounding 5, overflow 2, which is 1,280 signatures; for `mul`, `div`, `convert` also fraction length in {0, 1, N/2, N-1} and kaski's {16, 30, 48} where they fit.

| Id | Proves | Shape |
|---|---|---|
| T1 | L1 exhaustively | every pair of members, every operation, every signature at widths 1..=8 (at most 65,536 pairs per cell) |
| T2 | L1 at every other width | widths 9..=64, **all of them, none sampled**: the full cross of an edge lattice (range ends, the neighbours of each end, zero and its neighbours, every single bit value, every tie position for the fraction length) plus seeded pseudo random pairs with the seed committed |
| T3 | L2, L3, L4 | same sweeps |
| T4 | L8 | the registry's counts reproduced through `Num` (952; 4,177,792 at eight bits; none unsigned; none under wrapping) |
| T5 | L9 | each cell computed in its derived carrier and in the 64 bit carrier |
| T6 | kaski differential | `Fixed<16>`, `<30>`, `<48>` against the wrapper over `Num`, every operation that maps, exhaustive at an 8 bit model width and lattice plus random at 64. Runs against the **corrected** proxy; against the floor proxy it must fail in `mul` at every tie and below, and that failure is committed as a negative control |
| T7 | L10 | one committed digest over a fixed vector set per kaski format, asserted on native and under a wasm32 runner |
| T8 | half to even drifts in neither direction | a long decay product chain: sum of signed rounding errors stays within a stated bound under `HalfEven` and leaves it under `Floor` (negative control) |
| T9 | red on purpose | `Warm` on an integer format: `7 / 2` against native `3`. Fails at head's binding once division exists. It marks Q6 and the rounding cell; it is not to be quieted |
| T10 | zero cost at full width | emitted assembly of `USize::add` equal to native `wrapping_add` in release, as a committed asm gate (hilavitkutin already keeps `mock/benches/asm_gate_fixtures`) |

Cost: about 1,280 signatures x 5 binary operations x about 5,000 pairs is 32 million oracle calls at widths above eight, plus the exhaustive band. Seconds of runtime. The real cost is monomorphisation: 1,280 instantiations of every generic test body. Split into one test binary per width band and one per operation. Each further const or generic axis multiplies this: the debug marker of 2.6 adds no value cells; a pole policy with two values would double the division cells only.

Compile fail, each with its diagnostic committed:

| Id | Refuses |
|---|---|
| C1 | `Num` over a declared width of zero or past sixty four |
| C2 | `constant::<V>()` where `V` is not a member |
| C3 | `Identity<Additive>` on a format with no additive identity (odd half step phase, `lib.rs:166-170`) |
| C4 | `mul` or `div` on a phased or magnitude indexed format |
| C5 | `add` across two signatures |
| C6 | `div` by an unrefined divisor |
| C7 | a host integer where a `Num` goes, and a `Num` where a host integer goes |
| C8 | `Clamp` or `Stochastic` as the policy or mode of a value signature |
| C9 | a `Num` field or carrier named from outside the crate |

### 3.7 Benches, on the harness in `mock/benches/`, arms named

Each family names its profile (release, opt level 3, the pinned toolchain) and runs natively; a wasm32 run is a second dimension, not a second family.

| Family | Arms | Decides |
|---|---|---|
| `fixed-mul-half-even` at `Fi<16,8>`, `Fi<32,16>`, `Fi<64,16>`, `Fi<64,30>`, over a column | `num` (the slice); `proxy-half-even` (kaski's corrected `Fixed`); `proxy-floor` (kaski's current, labelled as computing a different function); `inline-widening` (the obvious hand written widen, add half, fix the tie, shift); `fixed-crate` (the ecosystem's `fixed`, labelled floor); `oracle` (the `i128` interpreter, labelled reference) | what half to even costs over floor per lane, and whether `num` is the hand written loop |
| `int-add-full-width` at 8, 16, 32, 64 and platform width | `num-wrap`, `num-saturate`, `native-wrapping`, `native-saturating`, `native-checked` | prices Q6's two readings side by side, which is the pricing `ruling.toml:1042` asks for |
| `int-add-sub-width` at 13 in 16, 28 in 32, 47 in 64 | `num-wrap` (mask and extend after), `num-saturate` (compare and select), `widen-then-adapt`, `native-carrier` (labelled as computing the carrier's function, not the declared one) | the cost of honouring the declared width at sole occupancy |
| `fixed-div` at the same four formats | `num`, `proxy`, `inline-widening`, `oracle` | where the 128 by 64 divide lands, natively and on wasm32 |
| `convert` between `Fi<64,16>` and `Fi<64,30>`, and 64 to 32 | `num`, `inline-shift`, `oracle` | |

A slower arm starts work: name the instruction, remove it, measure again.

## 4. Steps from head to the slice

Each is one helper's brief and lands as one arvo design round (topic, doc changelist, lock, source changelist, lock, close) unless marked otherwise.

| # | Work | Touches | Proof | Needs first |
|---|---|---|---|---|
| 1 | **Canon rows, additive only.** Propose: a datum of a format is a point (answers Q4, citing the four rulings of 1.5); a value is the sole occupancy placement of a declared signature; ingest is adaptation. Add kaski as a consumer with two obligations (a deterministic fixed point lane with declared rounding; bit identical results across targets). Classify the 86 rows without a ruling edge into: depends, does not depend, answered by a proposal and owed a second read | `mock/registry/*.toml` | two independent reads per row before promotion; `mock check` | whether the panel is open (0); call 1 |
| 2 | **kaski: correct the proxy to half to even in `mul`.** In kaski, not arvo | `kaski-engine-carrier/src/fixed.rs:173-176`, its tests, every digest once | an exhaustive product test at an 8 bit model against a wide reference; T8 | nothing. Start now |
| 3 | **Oracle: negation and subtraction** | `arvo-format` addition module and design | the existing addition law set repeated; registry count for composed saturating add and subtract (`law.toml:196`) | nothing |
| 4 | **The carrier map.** Width typestate to ladder carrier, both slot families, all 64 widths, no gate. Rebuild the spike's mechanism against the current `Slots` families, twice, by two helpers independently | new value crate, design first | resolution asserted at every width; C1; a compile time note of impl count and check time | nothing. Parallel with 3 |
| 5 | **`Num<S>`, doors, `add`, `sub`, `neg`, compare, identities, integer aliases** | value crate | T1 to T5 for the three operations; C1 to C3, C5, C7 to C9; T10 | 3, 4 |
| 6 | **Oracle: multiplication** for constant quantum, radix two, zero phase | `arvo-format` | exact step equals the rational product at small widths; registry verdicts at `law.toml:84, 114, 145` | nothing. Parallel with 3 to 5 |
| 7 | **Oracle: widen the remainder coordinate, then division.** CODE `apply/position.rs:68`: `Fraction::of(num: i64, den: i64)`. A 64 bit divisor does not fit an `i64` denominator (unsigned above `2^63`, and signed `i64::MIN`), so the exact quotient has no position today. Same shape of round as `202609190825`, which widened the slot coordinate | `arvo-format` | every existing fraction test unchanged; quotient positions at the divisor's extremes | 6 |
| 8 | **`mul`, `div`, `convert` on values; fixed point aliases with fraction length; rename the `points` parameter (2.7)** | value crate, `arvo-format` points | T1 to T5 for the three; C4, C6; `law.toml:342` | 5, 6, 7; call 5 |
| 9 | **Capacity crate** | new | the array projection resolves at every count hilavitkutin names; a compile fail for a count of the wrong kind | 5. A sketch named `202607291500_capacity-without-supertrait` exists; I did not read it |
| 10 | **Preset keyed aliases; T9 committed red** | `arvo-strategy` | the alias and the explicit signature are one type (a type equality test); T9 | 8 |
| 11 | **Benches of 3.7** | `mock/benches/` | committed artifacts per family | 5 for the integer families, 8 for the rest. First confirm the harness runs at all at head: `mock/Cargo.toml:31-43` records that a deletion broke every bench and an `exclude` repaired it; nobody in what I read has run one since |
| 12 | **hilavitkutin-api on arvo `dev`** (in hilavitkutin) | its numeric positions | it builds natively and for wasm32 with no feature gate of arvo's making | 5, 9, 10; call 6 on const positions |
| 13 | **kaski: `Fixed` becomes a wrapper** (in kaski) | `fixed.rs` only, by its own design | T6, T7 | 2, 8; calls 4, 10 |
| 14 | Slice two: masks, hash, then graph, sparse, spectral, one round each | new crates | per `obligation.toml:24-110` | 5, 9 |

Steps 2, 3, 4 and 6 have no prerequisites and can start today.

## 5. What I could not establish, and the calls that are his

### 5.1 Not established

- Whether the suite at head passes. Not run.
- Whether any bench at head runs on the harness. Not run.
- The content of `1b2796e9` and the shapes of the six pinned crates the slice defers. The clone is shallow.
- Whether the carrier map of step 4 compiles gate free against the current slot families. One spike against the old tree says the mechanism does; I rebuilt nothing.
- Which attempt he means by "the latest, I think very good attempt" with no enumeration (`ruling.toml:1157-1162`), and whether the spike in 1.6 is it.
- Whether the capacity sketch proves what its name says.
- The bodies of `arvo-format`'s tests beyond the two files named in section 0.
- How much of kaski relies on saturation as meaning rather than as an error. I read the doc comment, not the 700 call sites.
- Whether kaski's `div`, `scale` and `from_ratio` are meant to follow the half to even row.
- Whether a wasm32 test runner is available to arvo's and kaski's gates.
- Whether a panel is formally open, which decides whether step 1 can land rows.
- My reading in 1.5 is one expert's. So is every MINE above.

### 5.2 Calls

**Call 1. Is a datum of a format a point?** The ratified rows say so by my reading (1.5); the question row has no answer edge.
- a. Yes: a member is one point of the ambient domain; intervals and error carrying values are compositions above it. (My stance.)
- b. The top of a saturating format denotes everything above it, and the value type's contracts say so.
- c. Leave Q4 open and ship a value type for integer formats only, where the two readings cannot differ under wrapping.

**Call 2. The carrier deferral ("Explore, wait for Fog.").** The contention benches are committed; the row is still open.
- a. Leave the row as it is. Slice one ships sole occupancy only and no packed container. (My stance; needs nothing from him.)
- b. Mark it superseded by his own later answer: no fixed carrier and no regime, one arm per regime, each with its own break even.
- c. Hold packed containers until writes and random access are also measured, and say so on the row.

**Call 3. Does Warm wrap or saturate, and what does it round integers with?** Deferred by him as "both get priced".
- a. Leave both cells unsettled; presets ship marked as such; nothing stored may be spelled with a preset. (My stance.)
- b. Warm is exactly native Rust: toward zero, wrap in release, assert in debug.
- c. Warm saturates.
- d. Warm binds differently for integer and fractional formats (toward zero and wrap for integers; half to even for fractions).

**Call 4. In a debug build, does an operation that saturates by declaration abort?** The ratified ruling says overflow panics on debug "in every strategy".
- a. Yes, always. Kaski then needs its quantities never to reach the range ends in debug runs.
- b. Yes by default, and a consumer may declare in the type that completion is the meaning, which turns the assertion off for that type. (My stance.)
- c. No: an explicitly declared policy is never an error; only presets that imitate native integers assert.

**Call 5. Division where the divisor may be zero.**
- a. The divisor is non zero by type; the consumer classifies and branches. (My stance.)
- b. A declared pole policy on the signature (result is zero, or the range end by sign), chosen by the consumer as a generic.
- c. Both: a non zero divisor is the base, and b is a convenience over it added later.

**Call 6. Const generic positions across the stack.**
- a. Bare integers, per his quoted words in `obligation.toml:150`; hilavitkutin's `{ Cap(USize(8)) }` forms are rewritten and its `adt_const_params` gate goes. (My stance.)
- b. arvo newtypes as const parameters; arvo and every consumer carry `adt_const_params`, and the "no unstable machinery reaches a consumer" obligation is amended.

**Call 7. Is one internal macro table of per width impls acceptable for the carrier map?**
- a. Yes: 128 impls at one private site, nothing enumerated in any signature. (My stance, on one spike.)
- b. No: find the attempt he remembers first, and hold step 4 for it.
- c. Four impls only (one per ladder rung), with the consumer naming the rung and a compile time assertion that it is the narrowest.

**Call 8. A facade crate named `arvo`.**
- a. None in slice one; consumers name the owning crate. (My stance.)
- b. A facade that re-exports the slice, so hilavitkutin's `use arvo::..` lines survive.

**Call 9. Kaski's elementary functions (`sqrt`, `cbrt`, `exp`, `ln`, `pow`).**
- a. Stay in kaski for slice one, written over the value's raw door; move upstream later as their own crate with their results versioned. (My stance.)
- b. Move upstream now, with kaski's current bit results frozen as arvo's.
- c. Only the exactly specifiable ones move now (`sqrt`, `cbrt`, as floor of the true root under a declared rounding); the series stay.

**Call 10. Do kaski's `div`, `scale` and `from_ratio` round half to even like `mul`?** The canon row names the product only; the code floors all four.
- a. All four move to half to even, digests move once.
- b. Only the product, as written.
- c. Each takes a declared mode in its type, and kaski's settings pick.
