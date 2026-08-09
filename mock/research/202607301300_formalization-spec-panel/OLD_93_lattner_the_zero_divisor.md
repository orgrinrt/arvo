# 93. The zero divisor: the Door decides cost, never content, and the fork's two alternatives were carrying one undefinable cell between them

Chris Lattner, file 93. I wrote file 12 (the fresh read), file 74 (the taxonomy rechecked), and
file 83 (the three width levels and the one width axis, where a fork another file opened dissolved
rather than resolved). This file is the second independent read the `90b` instinct named as its own
lock condition, and its assigned attack surface is the checkpoint's own sentence: whether the
`Door` placement genuinely absorbs `Hot` fixed-point's missing `x/0` cell, or smuggles a
resolution into a lowering.

**The verdict, stated first.** The `Door` placement, in the form the instinct states it ("the
target's own divide instruction defines the answer"), smuggles. It fails on four compiled or
silicon-read facts, and the sharpest of them is not about the Door at all: the fork's *other*
alternative carried the identical smuggle in its `Hot` cell, so the two alternatives were never two
answers. They were two addresses for one cell whose content neither can state, and a cell whose
content cannot be stated at the value layer is not absorbed by moving it. It is a consumer
parameter, and the design's own toolbox rule has said so about exactly this shape since before this
panel convened. The theorem half of alternative 1, which is the reason the instinct went that way,
survives every attack below untouched, and I strengthen its derivation on the way through, because
the limit argument as currently worded is false at the `0/0` clause and silently conventional at
its sign clause. Both are repairable, and the repair is cleaner than the original.

**What I read.** `91_consolidation_nine.md` in full, the only required reading, twice: once cold,
once against its sources. `89_orchard_the_two_held_calls.md` in full (the file that assembled this
call), `89_probes/probe_5_division_failure_and_the_far_point.rs` as source rather than trusted, and
`90b_persona_checkpoint_twentytwo.md` in full (the instinct, persona-tier, explicitly a line for op
to overrule). Behind the consolidation, with licence since the call sits directly on them and the
consolidation compresses past the clauses I needed: `84_leijen_failure_that_is_not_a_range_event.md`
lines 130 through 175 (the `At<N, Q>` precedent the instinct cites), and the `93_probes/` sources
of every compiled claim below. One `ls` of the panel directory, current through `92_probes`.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty,
matching `91:1091-1092`. This dispatch is the owed artifact the consolidation names verbatim at
`91:1000-1001`, so the work itself is licensed; gate passed. Test gate: `cargo test --offline
--workspace` from `mock/`, summed across every `test result:` line: **149 binaries, 666 passed, 0
failed, 9 ignored**, matching `91:43-44`. I read the bodies of the tests in my surface rather than
their names: `arvo/tests/fixed_point_div.rs` (thirteen real assertions with deliberate
container-overflow setups, one correctly formed catalogue red at line 111, and one convention test
at lines 68 to 72) and `arvo/tests/strategy_wrapping_div_zero.rs` (six real assertions pinning the
same convention across Hot, Warm, Cold, both signs). Both files are honest tests of the shipped
convention; the convention itself is why the redesign exists, and I use it as evidence in exactly
that role below. The standing tautology at `arvo-tensor/tests/capacity.rs:14-18` is unchanged,
already registered (`91:957-958`), outside my scope to touch. Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the tree.

**What is compiled, what is read from silicon, what is reasoned.** Sections 2 and 4 trace to
`93_probes/` (four probes, outcomes and exact commands in `93_probes/OUTCOMES.md`), written and run
fresh this session; the x86 result executes under Rosetta 2 on this host, so it is a silicon read,
not a citation. No timing claim appears anywhere; every count is from `objdump -d` or emitted
assembly, and the bench harness was not run, per the standing outage (`91:1104-1108`). Sections 1,
3, 5, and 6 are reasoned from ratified text and the compiled pieces, and say so per claim. The
external ISA facts I could not execute (RISC-V, and the manuals behind what I did execute) are
marked external. **The deletion test, applied**: the two shipped-tree citations in this file (the
two division test files) are tree-facts offered as evidence about why the redesign is happening,
and every design conclusion below survives their deletion.

---

## 1. Cheap factual claims checked before reasoning from them, per the standing discipline

Four, and each needed something. All four are about the consolidation's or file 89's own text; none
touches a ratified checkpoint.

**The limit argument's `0/0` clause is false as written, and the compiled probe behind it carries
the same false gloss.** `91:297-300`: "As the divisor is driven to zero with a nonzero dividend,
the exact quotient's magnitude exceeds every representable magnitude ... with a zero dividend the
limit does not exist." Fix the dividend at exactly zero, which is what an exact-value design means
by a zero dividend, and drive the divisor: `0/d = 0` for every nonzero `d`, so the one-sided limit
exists and is 0, a representable value. The claimed non-existence holds only under a two-variable
limit in which the dividend is *also* perturbed, and a design whose values are exact has no license
to perturb an operand that is exactly zero. `89_probes/probe_5:79-84` encodes the correct predicate
(`num != 0`) under the same wrong justification ("are 0/0, indeterminate, when num == 0"). So the
`invalid` half of the two-way split is, as currently derived, a citation wearing a theorem's
clothes: the predicate matches IEEE because it was written to, not because the stated argument
produces it. Section 5 repairs this with a derivation that needs no limits at the `0/0` clause at
all, and the repair is the reason this is a correction rather than an attack on the split itself.

**The sign clause is a convention for single-zero numerals, and the ratifying text should say so.**
`89:534` (carried into `91:298-299`): "the one-sided limit is the supremum in the dividend's sign
direction." A divisor driven to zero approaches from one of two sides, and the two one-sided limits
have opposite signs. IEEE derives the side from the signed-zero *datum* (`x / +0` and `x / -0`
differ), so for `IeeeSpecials` numerals the sign is a theorem of the operands. A two's-complement
fixed-point numeral has one zero carrying no side, so the dividend-sign answer is the `+0`
convention, chosen to agree with the standard, not derived. This is not vacuous bookkeeping: the
`Warm`/`Cold` clamp cell's *content* (which far point) depends on it. The definitional-completeness
line (`91:128-134`) wants that one sentence in the ratifying text.

**File 89's "exactly one cell has no answer" (`89:574`) counts the `OverRange` matrix and misses
the `0/0` row entirely.** Under alternative 1, `0/0` is "the only genuinely kind-2 division
failure" (`89:565-566`), and kind 2's resolution for a numeral with no NaN under a totalising
preset is stated nowhere in the record: not in file 84's sort as the consolidation carries it, not
in file 89's cost table, not in `91` section 1.13 or 1.16. `Hot`, `Warm`, and `Cold` fixed-point at
`0/0` are all empty cells under both alternatives. So the fork's accounting ("one stated exception"
against "three cells where one rule exists") undercounts both sides. Section 5 closes the row
derivably.

**The consolidation carries the kind sort and the fork's reclassification side by side without
naming the collision.** `91:425-426` states kind 2's canonical members as "division by zero,
`Recip` at zero, `Sqrt` of a negative", ratified since file 50. `91:305-311` adopts, as the
instinct, the reclassification of `x/0` (nonzero dividend) as a kind-1 range event. Both sentences
are in the same document. And the reclassification propagates: `Recip` at zero *is* `1/0` with a
nonzero dividend, so the same limit argument moves it to kind 1 too, leaving the ratified kind-2
example list with one member and a boundary case. This is exactly the two-organs-two-answers shape
the counting collision had one section over (`91` section 1.14), caught earlier this time because
the separation requirement now exists. Section 5's repair dissolves it rather than picking a side:
under the solution-set derivation the split is not "kind 1 versus kind 2" but "empty solution set
with a direction versus everything else", and both organs' sentences survive as views of that.

*Grounded on: ratified (`91:128-134` the definitional-completeness line, `74b` via `78:275-286`
the far point), settled shapes (`91:297-311`, `91:425-426`, `89:534`, `89:565-574`), verified at
source (`89_probes/probe_5:79-84`), reasoned (the one-variable limit computation, the sign-side
observation, the `Recip` propagation, mine).*

---

## 2. The target facts, read rather than assumed

The instinct's load-bearing sentence is "the target's own divide instruction defines the answer",
checked at one target (`89_probes/probe_5` CLAIM D, "on this host"). The brief licenses checking
the quantified version. Four facts, three executed on this machine, one external.

**aarch64 defines a value, and it is the same value for every divisor-zero cell**
(`93_probes/probe_1_what_the_isa_actually_defines.rs`, compiled and run). `sdiv`/`udiv` with a zero
divisor return 0 for every dividend: positive, negative, `MIN`, *and zero*. No trap. The last case
is the finding: the instruction does not observe the design's ratified `divideByZero`/`invalid`
distinction. A `Hot` cell defined as "what the instruction does" silently answers the `0/0` cell
too, with the same 0, which contradicts the very theorem the instinct adopted alternative 1 to keep
(the two-way split is *the* content of that theorem). To preserve the split, the lowering must
distinguish the cells, which means testing the dividend, which is the check the "free" argument
claimed to save. The same probe reads `sdiv MIN/-1` returning `MIN`: that cell the value layer
fully defines (`ReduceModulo` of the exact quotient `2^63` is `MIN`), and the ISA merely agrees. I
use this pair in section 3.

**x86-64 defines no value at all** (`93_probes/probe_2_x86_does_not_define_a_value.c`, compiled
for `x86_64-apple-darwin`, executed under Rosetta 2). Raw `idiv` with a zero divisor raises #DE and
the process dies with SIGFPE (shell status 136). A trap is not a value. On this target the Door has
nothing to point at, so a Door-defined cell must be *invented* there by whatever guard the lowering
emits, and choosing the guard's value is a resolution decision made per target, invisibly, on the
axis the design forbids to carry one. That is the smuggle in its plainest form, and it is not
hypothetical: it is the mandatory x86 code path.

**Targets that define values disagree with each other** (external: the RISC-V unprivileged ISA
defines `DIV`/`DIVU` by zero as all bits set, quotient minus one signed, and `REM` by zero as the
dividend; not executed here, primary read owed with the other ISA citations). So "the answer" the
Door would defer to is 0 on ARM, all-ones on RISC-V, and a trap on x86. A safe, total operation
delivering target-varying values from identical operands has no precedent anywhere in this design,
and section 3 shows the precedent the instinct cites does not supply one.

**The toolchain voids the route regardless of target**
(`93_probes/probe_3_the_toolchain_takes_it_back.c`, compiled at `-O2` for both architectures). LLVM
`udiv`/`sdiv` carry undefined behavior on a zero divisor (LLVM LangRef, external), and the
optimizer uses the licence: a zero-divisor check placed after a division is deleted on both
targets (`f` compiles to bare `sdiv` / bare `idivl`, no compare; control `g` keeps its check). So
the IR every arvo lowering actually passes through defines the cell as UB *on every target,
including the one whose silicon defines it as 0*. What the target gives away for free, the
toolchain takes back: reaching the ISA's answer deterministically requires an asm-opaque
instruction, which is a paid barrier, not a free cell. Applied to file 70's own method, which the
instinct inherits through `84:153-160` ("the row is whatever the target's own container gives
away"), the honest output at this cell is "nothing is free here", because the lowering pipeline
between the design and the container refuses to pass the question through.

*Grounded on: compiled and silicon-read (`93_probes/probe_1`, `probe_2`, `probe_3`, outcomes and
commands in `93_probes/OUTCOMES.md`), external (ARM DDI 0487 for the aarch64 behaviour the probe
confirms, Intel SDM #DE, the RISC-V unprivileged spec, LLVM LangRef `sdiv`/`udiv`; all secondary,
a position-cited read owed to the same bundle `78:934-941` carries), reasoned (the consequence
chains, mine).*

---

## 3. The sentence that separates absorbing from smuggling

The design already owns the mechanism for this distinction; nobody had moved it to this boundary.

**The `At<N, Q>` precedent the instinct cites does not do what the instinct does.** `84:153-160`
derives which ratified row governs `quantize` at a mixed target by reading the lowering: "the row
is whatever the target's own container gives away ... **the row is a consequence of the target's
lowering, not of a table lookup**." Read what the derivation *outputs*: "the far point is the
honest row" for one lowering, `ReduceModulo` for the other. Both outputs are members of the
design's own resolution vocabulary, fully stated at the value layer, target-independent once the
`Lowering` is fixed, governed by every law and boundary predicate that already governs those
resolutions (the 16,000,000-cell agreement at `91:437-443` quantifies over exactly them). The
lowering was an *input to a design-time derivation*; the derived cell's content survives with the
lowering deleted from its statement. The same holds for the second cell probe 1 read: `MIN/-1`
under `Hot` is `ReduceModulo`'s own answer, `MIN`, statable with no mention of any target; aarch64
happening to deliver it in one instruction is a fact about cost.

**The `x/0` cell fails the same test.** Delete the Door from its statement and nothing remains:
not a resolution-vocabulary member (`ReduceModulo` has no answer, which is the whole premise), not
a derived constant (section 2: three targets, three behaviours, one of them not a value), not a
consumer-named parameter (none exists in either alternative). The cell's entire content is
"whatever this axis's occupant does", and that is a value authored on the one axis the design
defines as authoring none.

So the sentence, offered in the consolidation's own provenance form as this file's candidate for
the spec text, because the brief is right that the design will face this shape again:

> **A `Lowering` may be an input to a derivation, and it may implement a stated value; it may
> never be the author of a value. The test is the review's own deletion test, moved to this
> boundary: state the cell with the `Lowering` deleted. If a value-layer sentence remains (a
> member of the resolution vocabulary, a constant derived from the numeral's parameters, a
> parameter the consumer names), the placement is legitimate, and the `Lowering`'s only remaining
> question is what the stated cell costs on each target. If nothing remains, the cell's content
> lives on the axis that is defined to carry none, and no precedent citation changes that.**

**Why the smuggle would have passed review, which is worth recording beside the sentence.** The
guard that patrols this boundary is "no law may read `Lowering`" (`91:158-159`), and the seventh
consolidation already found that rule doing double duty it was never given (`68:178-180`). Here it
fails in a new direction: it is satisfied *vacuously*. Every division law is conditioned on a
nonzero divisor, so no law quantifies over the smuggled cell, so no law reads the Door, so the one
mechanical check the design has never fires. A value with no law over it is exactly the value that
can be authored anywhere without tripping anything. The separation requirement (`91:136-143`)
catches it only when a model instantiates *two* targets, which is what sections 2's probes are: the
one-target model (`probe_5` CLAIM D, this host) is precisely the instantiation at which the wrong
subject (the ISA's constant) and the right one (a stated value) coincide, and the second target is
where they separate. One more instance of the pattern file 86 catalogued, on the axis nobody had
checked it.

**The steel-manned Door placement dies on the same facts, which is why this is a verdict and not a
taste.** The maximally honest version would lift the target's constant into declared type-level
content: a Door member carrying `const ZERO_DIVISOR_QUOTIENT`, named freely under the refined
naming principle (`91:845-851`, a name may denote type-level content), with the cross-target
surrender named once in trusted-base vocabulary per the general clause (`91:620-626`). Even that
version fails twice over: on x86 there is no constant to lift (probe 2), and on every target the
constant is unreachable through the IR without an asm barrier (probe 3), so the "declared" constant
is really a per-target promise the toolchain actively works against. The honest form of that
promise already exists in the design, and it is section 5's consumer parameter.

*Grounded on: ratified (`91:158-159`, `77b` via `78:409-441` the preset tables), settled shapes
(`84:153-160`, `68:178-180`, `91:136-143`, `91:437-443`, `91:846-851`, `91:620-626`), compiled
(`93_probes/probe_1` through `probe_3`), reasoned (the deletion-test transfer and the vacuity
observation, mine, and the sentence above is offered, not ruled).*

---

## 4. The price of stating the cell, measured

If the Door's cell is illegitimate, the honest alternative must be priced, because "free" was the
instinct's whole draw. Measured on the friendliest target the Door argument has
(`93_probes/probe_4_the_cell_priced.rs`, 64-element `i64` loop, counts from `objdump -d`, no
timing, neither loop unrolled by the compiler):

| body | instructions per element | data-dependent branches |
|---|---:|---:|
| consumer-stated, through IR: `if d == 0 { 0 } else { x.wrapping_div(d) }` | 11 | 2 |
| raw asm `sdiv` (the Door's cell) | 7 | 0 |

Three things the numbers say that the argument did not.

**The two bodies compute one function on this target.** Asserted at runtime over 64 elements with
14 scattered zero divisors and negative divisors: element-for-element identical, because the
consumer's fallback (0) was *chosen* to coincide with the ISA's constant, and `wrapping_div`
states the `MIN/-1` cell (`MIN`) that `sdiv` also delivers. Which means the 7-instruction body is
reachable with zero semantic delegation: a Kind-1 cfg-gated lowering
(`arvo-always-optimal-internals.md`, structural lowering) may emit bare `sdiv` *as an
implementation of the stated function*, on exactly the targets and exactly the fallback values
where the coincidence holds, with a guard elsewhere. Identical machine code, opposite provenance:
the Door decides cost, the design decides content. This is the entire legitimate residue of the
instinct, and it is not small; it is just not semantics.

**Two of the four extra IR-path instructions are not the consumer's zero test.** They are LLVM's
own inserted `MIN/-1` guard (`cmn`/`ccmp`) on `wrapping_div`, because LLVM `sdiv` is UB at that
cell too, the cell the value layer fully defines and the ISA delivers free. Probe 3's thesis,
confirmed from the other side: the toolchain guards even the stated cells, so the asm-microkernel
route is the design's already-licensed answer for *both* corner cells, and the Door placement was
never needed to reach the fast body.

**A consumer who names a different fallback pays one `csel`, and pays it knowingly.** Fallback 0
is free on ARM and costs a select on RISC-V; fallback all-ones is the reverse; x86 pays the guard
for any fallback. The cost table is per (fallback, target) and belongs in the Door's documentation,
which is exactly the toolbox posture: documented tradeoffs at the choice point, the consumer
decides.

*Grounded on: compiled and measured (`93_probes/probe_4`, counts and disassembly in
`93_probes/OUTCOMES.md`), settled shapes (`arvo-always-optimal-internals.md` Kind 1), reasoned (the
provenance inversion, mine). One honest limit: instruction counts are not throughput; the bench
harness stays unrun per the standing outage, and a latency claim for the branchy form under
mispredict pressure is named as owed to the harness once its overwrite defect is fixed.*

---

## 5. What the design is, in a form the next consolidation could take close to verbatim

Offered, not ruled, and owed a second read like everything one-pass in this stretch. It keeps
everything the instinct wanted (the theorem, zero new axis positions, one rule for three presets)
and drops only the authorship of the one cell.

**The split, re-derived so both halves are theorems.** Division is the inverse of multiplication:
the exact quotient of `x` by `d` is the solution of `q · d = x`. The solution set has three
shapes, and the design's failure vocabulary is a function of the shape.

1. *A singleton.* An ordinary quotient; the quantiser and the range machinery govern as ratified.
2. *Empty, with a determinate divergence direction.* `x/0` with `x != 0`: no value solves the
   equation, and the one-sided limit supplies a direction (this is the only clause where the limit
   argument is needed, and there it is sound). The operation's failure borrows the range event's
   own resolution row in that direction: clamp goes to the far point, `Specials` carrying infinity
   delivers it as the absorbing far point and the `divideByZero` generator fires, refuse refuses.
   This is alternative 1's content, kept, with the kind-sort collision dissolved: the ratified
   sentence "the result value does not exist mathematically" (`91:425`) stays true of this cell,
   and the reclassification sentence becomes "an empty solution set with a direction borrows the
   range resolution", so neither organ's text is wrong and `Recip` at zero inherits the same
   reading without a silent list edit.
3. *Everything, or empty with no direction.* `0/0` (every value solves `q · 0 = 0`) and `Sqrt` of
   a negative (nothing solves it, and there is no divergence to give a direction): no value is
   privileged, nothing exists to lie toward, and the event is `invalid`. This derivation needs no
   limit and no perturbation premise, which repairs section 1's first finding.

**The sign clause, stated as what it is.** Where the divisor's datum carries a side (`IeeeSpecials`
signed zero), the far point's sign is a theorem of the operands. Where it does not (single-zero
fixed point), the design states the `+0` convention once, as a citation-shaped clause, because
that is what it is.

**The `0/0` row, closed.** `invalid` resolves to NaN exactly where the numeral carries one
(`NanOnly`, `IeeeSpecials`); a numeral with no NaN has no honest total answer to a question whose
every answer is equally wrong, so the operation is partial at that input at every preset, through
the machinery the adopted surface already ships: refused at declaration where the divisor's domain
is a predicate, carried in a niche otherwise, at no layout cost (`91:284-285`). No third axis
position; the "third position" need shrinks to two existing mechanisms, NaN and partiality.

**`Hot`'s cell, resolved by ownership rather than by address.** `ReduceModulo` is a function of a
finite exact value; the empty-solution cell is outside its domain, and the design says so instead
of hunting a home for a value that does not exist. The record now contains three independent
inventions for this cell, which is the strongest evidence available that it is a preference and
not a fact: the shipped tree returns the numerator (`arvo/tests/fixed_point_div.rs:68-72` and
`strategy_wrapping_div_zero.rs`, a stated convention with no derivation, tree-fact, offered as
why-evidence), file 89's alternative-2 row returns the host's 0 (`89:586-588`), and `90b` delegates
to the Door (`90b:51-55`). Three authors, three answers, no derivation among them: by
`arvo-toolbox-not-policer.md`'s own decision test ("is the consumer the one who knows the
answer?"), the cell belongs to the consumer. Concretely: the partial forms are the default
(`div_floor`, `rem`, `div` as adopted, partial on the divisor); the total form over a possibly-zero
divisor takes the fallback as a parameter the consumer names, `div_or`-shaped, value-position or
type-position as the const machinery allows. The cell is then stated in the program,
target-independent, provable-tier, law-irrelevant by construction (it is the consumer's own
constant), and the aarch64 lowering of the fallback-0 instantiation is the bare `sdiv` body at 7
instructions per element (section 4). Every benefit the Door cell promised, none of the
authorship.

**The Door keeps exactly two roles, both legitimate under section 3's sentence.** Input to
design-time derivations whose outputs are value-layer sentences (the `At<N, Q>` precedent, intact
and now with its licence stated), and implementation selector for stated cells (the Kind-1
coincidence table: which fallback values are free on which targets). It authors nothing.

*Grounded on: ratified (`91:276-293` the adopted surface, `91:425-429`, `78:409-441`,
`arvo-toolbox-not-policer.md`), settled shapes (`84:386-397` the niche cost, `89:586-588`),
compiled (`93_probes/` in full), tree-fact (the two division test files, why-evidence only),
reasoned (the solution-set derivation and the ownership resolution, mine, owed a second read).*

---

## 6. What this file does not decide, and what it leaves owed

**The call.** It is op's, the instinct was persona-tier and said so, and this file is the stress
test it asked for, not a ruling. What I have tried to change is what the ruling is over: not "Door
or third position" but "is there a value to state", and the compiled answer is no.

**Whether `div_or`'s fallback is value-position or type-position, and its exact name.** A
type-position fallback is value-keyed and law-eligible by the same lift file 43 and file 84 both
made; a value-position one is the ergonomic default. Both fit the constraint set (no
`generic_const_exprs`; `adt_const_params` suffices for a fallback constant). That is a surface
question for the operation chapter, not for this fork.

**Owed, named with closing artifacts, all carried from `91` section 4 unchanged except the last
three, which are new.** The signed halves of file 43's probes 2, 4, 5 (a signed re-run of each;
probe 1 here touches `MIN/-1` but does not close them). The float-division compile against a
`Specials`-bearing numeral. The IEEE clause 7 primary-source reads, now joined by the ISA bundle
(ARM DDI 0487 `sdiv`/`udiv`, Intel SDM #DE, RISC-V M-extension divide-by-zero rows, LLVM LangRef
`sdiv`/`udiv`; artifact: verbatim position-cited quotations beside the executed probes that
confirm two of them). The `0/0` row and the sign-convention sentence, if section 5's shape is
adopted (artifact: the two sentences in the ratifying text). A throughput number for the guarded
against asm-implemented bodies under mispredict pressure (artifact: a `mock/benches` section,
after the orchestrator's overwrite defect is fixed upstream; not before).

**Standing.** Nothing here overturns a ratified call. The far-point rule, the adopted
three-operation surface, the preset tables, and the `At<N, Q>` derivation all stand exactly as
consolidated; the two-way split survives with a stronger derivation than it arrived with. What
does not survive is one persona-tier instinct's placement of one cell, attacked on the surface the
instinct itself named, with four compiled or silicon-read facts and a reused test doing the work.
My last file ended with a fork dissolving; this one ends the same way, and I do not think that is
a coincidence about me. It is what this review's method does to forks: stress the distinction
where it is nonvacuous, and the question that survives is usually smaller and better-posed than
the one that was asked.
