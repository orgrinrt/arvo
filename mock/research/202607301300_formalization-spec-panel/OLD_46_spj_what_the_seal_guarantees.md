# 46. What the seal guarantees

**Member:** Simon Peyton Jones. I wrote file 07, on whether the type story was sound, thirty-nine
files back, and per this dispatch's own instruction I carry none of it forward: I did not re-read it,
the third consolidation supersedes whatever it settled, and anything below that happens to agree with
it agrees by re-derivation, not by inheritance. The habit of mind this dispatch wants is one I have
spent a career on from the other side: in a language where "sealed" is not a primitive but a
discipline assembled from visibility and coherence, a claim that a perimeter is closed is a theorem,
and a theorem is not proved by checking the case the last person thought of. The lovely thing, and
the reason this file exists, is that the proof obligation here is dischargeable by a compiler: the
adversary is a crate, and either it compiles or it does not.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed,
9 ignored, summed per binary from the `test result:` lines rather than trusted from a headline,
matching files 41 through 45 exactly. The test surface this dispatch touches is empty in the shipped
tree: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo root
returns nothing (exit 1), the corrected command and expected result file 45 established
(`45:456-475`); nothing shipped names this surface, so the bodies to read are the review's own probe
files, which I read as source and rebuilt rather than trusted (below). Test bodies in the nearest
shipped surface were read in full by file 45 two files ago (`45:14-22`) and nothing here touches
them. Canon gate: `40_consolidation_three.md` and `44b_op_checkpoint_ten.md` in full before a line of
code. The work itself is canon-licensed twice over: op ratified the encoding and named the seal "an
implementation task, owed" (`44b:17-19`), and this dispatch is the panel's execution of that owed
task at the design level, with the soundness question attached. Nothing below overturns a ratified
call; the one sentence of the consolidation this file corrects (`40:446`) was already found false by
file 42 and is here found false a second way.

**What I read:** `40_consolidation_three.md` in full, twice. `41_chlipala_the_rational_bias.md`,
`42_arntzen_the_observation_surface.md`, `43_smith_division.md`, `44_ringer_what_the_overturn_left_
behind.md`, `45_leroy_what_each_claim_rests_on.md`, and `44b`, in full, the deliverables since the
consolidation per the dispatch. As source rather than through any file's paraphrase:
`42_probes/vu_nat_sealed.rs` and `42_probes/vu_bias_sealed.rs` in full (the tower everything since
file 42 treats as the fixed state), `42_probes/OUTCOMES.md` in full (the build method and the
verbatim errors my probes had to reproduce or contradict), and the head of `41_chlipala` section 0
through 1 for the exact shape of `Bias`'s seal and of the attack file 41 found first. `ls` of the
review directory once: 45 numbered deliverables plus probe directories before this one.

**What I compiled or measured, separated from what I reasoned.** Everything load-bearing is
compiled, against the pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with
`rustc --version` inside the repo; host `aarch64-apple-darwin`, confirmed with `rustc -vV`, named
because file 45 established nobody had been labelling target-specific facts). `46_probes/` holds
twenty Rust files plus the outcome record and `price/`: the two unmodified copies of file 42's tower, the two-file completed tower
(`vu_nat_sealed_adj.rs`, one audited diff; `vu_bias_sealed_adj.rs`, path retarget only), two library
wrappers, fourteen numbered probes, and `price/` with the generator, the sweep and a committed CSV.
`46_probes/OUTCOMES.md` carries the full outcome table with verbatim error heads and the build
commands that reproduce every line. The price figures are min-of-1 difference quotients at one width,
`--emit=metadata`, and are `pin + host` facts, not benches; the scope is stated where they appear.
Reasoned rather than compiled, and marked in place: the completeness argument in section 3 (why the
enumeration of introduction routes is exhaustive), the quantification block in section 4, and every
recommendation.

## 0. The verdict, stated first

**The perimeter was open a third time, on the first hole this review ever found, and it is now
closed and priced.** The tower as file 42 left it, the one every file since composes with, seals
`Pos`, `Nat` and `Bias` and leaves `Adjustment` a bare `pub trait` with no supertrait
(`42_probes/vu_nat_sealed.rs:448-455`). File 41's original attack, a genuinely separate downstream
crate writing `impl Adjustment for EvilAdjustment { const NUM: u64 = 6; const DEN: u64 = 12; }`, no
`Pos`, no `Ratio`, no coprimality anywhere, **compiles clean against that tower and reaches an
`A: Adjustment`-bounded position** (`46_probes/probe_1b`, forced through a fn signature). That is the
identity contract's own front door: `Implicit<E, A: Adjustment, B: Bias>` (`40:70`) admits it. File
41 found this hole and recommended the seal (`41:62-70`); file 42 correctly showed that seal was
insufficient for the deeper fabricated-`Pos` hole and built the `Pos`/`Nat` seal instead
(`42:74-79`), and its own enumeration then recorded `Adjustment` as "OPEN before the fix, CLOSED
after" (`42:139`), which is true of the route file 42 attacked and false of the route file 41 had
already published. Nobody re-ran the first attack against the second fix. Three passes, three
different perimeters, and after each pass the record said closed.

**The fix is one private trait and one blanket impl, and it costs nothing.**
`46_probes/vu_nat_sealed_adj.rs` adds `AdjustmentSealed` to the existing private `sealed` module,
gives `Adjustment` the supertrait, and adds one `AdjustmentSealed` blanket impl carrying the
identical bound the `Adjustment` impl already carries. Measured against file 42's tower at the same
methodology (`46_probes/price/`): 16.00 against 15.91 ms/composition, metadata 2036.7 against 2036.3
bytes/composition, both inside noise. The same attack against the completed tower fails with
`E0277: EvilAdj: nat::sealed::AdjustmentSealed is not satisfied` (`46_probes/probe_3`).

**"Closed rather than closed where the last person looked" is discharged by enumeration over error
classes, not over attacks anyone thought of.** In this language there are exactly four routes by
which a downstream crate can introduce a new obligation `T: Tr` for a foreign trait `Tr`, and the
adversary compiles all four against every sealed trait, plus the observation, erasure and
extension surfaces: direct impl (refused E0277 on the seal, `probe_3`), the supertrait itself
(unnameable, E0603, `probe_3b`), re-impl on an existing inhabitant (refused E0117 before any seal is
consulted, `probe_3e`), and a downstream blanket over a type parameter (refused E0210, `probe_3f`).
Fabricated-`Pos` laundering through upstream blankets is refused at the root (`probe_3c`, replaying
file 42's attack as a regression check on my own diff), and malformed genuine types are refused at
the bound (`probe_3d`, two E0271 carrying the actual gcd in the type, one E0277). Section 3 is the
table; section 1 is why enumerating by route, rather than by attack, is what ends the
hole-per-pass pattern.

**File 42's one argued-not-compiled residual is now compiled, in both halves, and the answer is more
interesting than the argument.** Coherence **admits** `impl Gcd<LocalRhs> for H` in a downstream
crate (`probe_4`, compiles clean against the upstream blanket `impl<B: Pos> Gcd<B> for H`): a
downstream can genuinely hold a lying `Gcd` fact about a genuine inhabitant. And it cannot spend it:
the other operand position's own `Pos` bound refuses (`probe_4b`, E0277) before the fabricated fact
is consulted, exactly as file 42 argued (`42:146-165`). So the completeness argument the whole
perimeter rests on is no longer an argument; both of its halves are compiler output.

**The seal costs legitimate extension nothing, and this is compiled, not asserted.** A downstream
crate builds a new operation over the sealed encoding by structural recursion over the public
constructors, spells MATLAB's slope 1, bias 1/2 from sealed parts, and declares a convention
contract whose associated types are bounded on the sealed traits, all const-asserted, all clean
(`probe_6`). The design's own layered-crate shape crosses the seal for its good reason: the bias
layer compiles as a genuinely separate crate over the sealed nat crate, declaring its own sealed
carrier on top (`probe_7`). The seal quantifies over inhabitant introduction, never over
composition, observation, or the contract layer, and section 4 states that sentence in the form the
next consolidation can take verbatim.

## 1. Why three passes closed three different perimeters

The diagnosis matters more than the third hole, because a fourth pass under the same method would
find a fourth. Each pass asked "is trait X sealed?" for the X it was looking at: file 36 asked it of
`Pos`/`Nat` and answered in an orphaned demonstration; file 41 asked it of `Bias` and answered
correctly for `Bias` while discovering `Adjustment` was open; file 42 asked it of the composing
copy's `Pos`/`Nat`, closed the deepest hole, and never carried file 41's answer for `Adjustment`
into the copy it fixed. The question was per-trait; the property is not. The property is
**per-obligation-introduction**: value-uniqueness is quantified over every place a `T: Pos`,
`T: Nat`, `T: Adjustment` or `T: Bias` obligation can come to hold, and a pass that checks one
trait's own seal has checked one introduction route of one trait.

So the repair is not a fourth attack hunt; it is a checklist small enough to run mechanically, per
trait, in two lines. A trait whose inhabitant set a guarantee quantifies over (call it a **carrier**)
owes exactly two things:

1. **Sealed at the trait.** A private supertrait, so the direct-impl route (E0277) and the
   supertrait route (E0603) are closed, and the orphan rules close the re-impl (E0117) and
   downstream-blanket (E0210) routes for free.
2. **Carrier-bounded through the parameters.** Every impl of the trait, including the seal's own
   blanket, either sits on a closed constructor whose every type argument is re-bounded on carrier
   traits, or re-establishes the defining property directly in its where-clauses. This is what
   closes the laundering route: an upstream blanket is a machine for minting obligations from
   caller-chosen parameters, and it is exactly as strong as the bounds on those parameters.

Run the checklist over the history and each pass's finding falls out as the line it missed. File
41's `Bias` satisfied both (its blanket bounds on `N: Pos + Gcd<D, Out = H>, D: Pos`, deliberately
not on the abstract `Adjustment`, `vu_bias_sealed.rs:168-175`); the `Pos` it bounded on satisfied
neither in the composing copy, which is file 42's finding. File 42's `Pos`/`Nat` satisfied both;
`Adjustment` satisfied the second (its blanket is right) and not the first, which is this file's
finding. The checklist is two lines per carrier and there are four carriers; the audit is eight
lines, and section 3's adversary is those eight lines executed by rustc.

One more thing follows from the diagnosis, and it is the difference between this file and a fourth
instance of the pattern: **the perimeter must live as a compiled artifact, not as prose that says
"sealed".** The consolidation's sentence "`Pos`, `Nat` and `Int` are sealed" (`40:446`) has now been
false in two different ways in three weeks (file 42: true only of an orphaned demo; this file: the
fixed copy left a carrier out), and no reader of either sentence could tell. The adversary crate is
the honest form of the sentence: when the tower ships as source, `46_probes`' attack files ship as
compile-fail tests beside it, in the shape `mock/crates/arvo/tests/ui/` already uses (nine
compile-fail pairs live there today, counted by file 45's gate, `45:18-21`), so a later loosening of
any bound turns a green suite red instead of turning a prose sentence quietly false. This is
`harness-the-type-system.md`'s own rule, "a refusal that nothing pins can be deleted by accident,"
applied to the refusal the whole uniqueness result rests on.

## 2. The seal, completed

The diff, in full, against `42_probes/vu_nat_sealed.rs` (everything else in
`46_probes/vu_nat_sealed_adj.rs` is byte-identical; diff the files to audit):

```rust
mod sealed {
    pub trait PosSealed {}
    pub trait NatSealed {}
    pub trait AdjustmentSealed {}   // added
}

pub trait Adjustment: sealed::AdjustmentSealed {   // supertrait added
    const NUM: u64;
    const DEN: u64;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> sealed::AdjustmentSealed for Ratio<N, D> {}   // added
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for Ratio<N, D> { /* unchanged */ }
```

Note the seal impl's bound duplicates the trait impl's bound. That is checklist line 2 applied to
the seal itself: an unconditional `impl<N, D> AdjustmentSealed for Ratio<N, D>` would still close
every route in section 3 (the `Adjustment` impl's own bound re-checks the condition), but the
duplicated bound keeps the invariant local to each impl rather than distributed across two, which is
the shape a reader can verify without a global argument. It costs nothing measurable: the solver
memoises the condition, and the sweep confirms it (16.00 against 15.91 ms/composition, metadata
identical to half a byte, `46_probes/price/results.csv`, min-of-1, 8-bit, `--emit=metadata`,
`aarch64-apple-darwin`).

`Bias`'s seal carries over from file 41 unchanged. `Nat` and `Pos` carry over from file 42
unchanged. `Int` has no construction in any composing probe and, per file 45's finding 4.1
(`45:403-425`), no grounding; if op keeps it, it gets the same two-line treatment in the same
module, and if op drops it, nothing here changes.

## 3. The adversary, enumerated by route

Every row compiled, `46_probes/OUTCOMES.md` has the verbatim error heads and the exact build
commands. "Refused" means the attack does not compile against the completed tower; "admitted" means
it compiles and the following column says why that is not a breach.

| Route | Probe | Outcome |
|---|---|---|
| Direct impl of each sealed trait on a local type | `probe_3` | Refused, E0277 on the private supertrait, all four carriers. The `Adjustment` row is the one that compiles clean against file 42's tower (`probe_1b`). |
| Implementing the private supertrait itself | `probe_3b` | Refused, E0603 at the path, before trait solving. |
| Re-impl on a genuine inhabitant with lying consts or a lying gcd | `probe_3e` | Refused, E0117, orphan rule; also disposes of the `min_specialization` worry, since the upstream impls are not `default` and overlap is refused at coherence regardless of feature gates. |
| Downstream blanket over a type parameter | `probe_3f` | Refused, E0210, uncovered parameter. |
| Fabricated `Pos` with a lying `Gcd`, laundered through the upstream `Adjustment`/`Bias` blankets (file 42's attack) | `probe_3c` | Refused at the shared root, E0277 on `PosSealed`, matching `42_probes`' own error shape: regression check that this file's diff loosened nothing. |
| Malformed genuine types at bounded positions: unreduced `Ratio<P6, P12>`, unreduced `BPos<P6, P12>`, padded `O<Evil>` | `probe_3d` | Refused, E0271 twice (the error carries the actual gcd as a type: `<O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H` expected `H`, found `O<I<H>>`) and E0277 once. |
| Foreign-parameter impl on a genuine inhabitant: `impl Gcd<LocalRhs> for H` (file 42's argued residual, first half) | `probe_4` | **Admitted by coherence.** The residual is real: a downstream can hold a lying `Gcd` fact about `H`. |
| Spending that fact: `Ratio<H, LocalRhs>` at an `Adjustment` bound (second half) | `probe_4b` | Refused, E0277 on `LocalRhs: Pos` at the other operand position, before the fabricated fact is consulted. File 42's completeness argument (`42:146-165`), both halves now compiler output. |
| Helper-trait impls on local types (`Dbl` for `LocalNat`), then entering the tower through `Reduce` | `probe_5` | Impl admitted (open by design, overlaps nothing); entry refused. The refusal's diagnostic is E0275, not E0277: see section 6.2. |
| Type erasure (`&dyn Pos`) | `probe_8` | Refused, E0038, not dyn-compatible; the route does not exist before the workspace's no-`dyn` rule applies. |
| New operation, new numeral, new convention contract (the legitimate crossings) | `probe_6` | Clean, const-asserted. Section 5. |
| The design's own layering: bias as a separate crate over the sealed nat crate, declaring its own sealed carrier | `probe_7` | Clean. Section 5. |

**Why this enumeration is exhaustive** (reasoned, and the one place the argument is mine rather than
rustc's): an obligation `T: Tr` for a foreign `Tr` holds only if some impl applies to `T`. Impls are
either downstream (routes one through four: on a local type, on the supertrait, on a foreign type,
or blanket over a parameter, which is every header shape the orphan rules distinguish) or upstream
(the sealed set plus the blankets, whose applicability is exactly their parameter bounds, which is
checklist line 2 and probes 3c/3d/4b). There is no fifth place an impl can live, `specialization`
and `TypeId` are forbidden by `unstable-features.md` (and file 10 already established those bans are
load-bearing for exactly this kind of closed-world claim), and erasure is probe 8. What the
enumeration does not cover is a future rustc changing coherence behaviour, which is why every
refusal above is a `pin`-grounded fact until it ships as a compile-fail test, per section 1.

## 4. What the guarantee is quantified over

Written for the next consolidation to take nearly verbatim, because file 45's backfill found the
tower's by-construction claims "conditional on `seal-owed`" with no statement anywhere of what the
discharged condition would even assert (`45:427-441`). This is that statement.

> **The uniqueness guarantee and its perimeter.** In any crate graph containing the sealing
> crate(s), compiled on the pinned toolchain with the forbidden-feature bans in force
> (`specialization`, `TypeId`), the inhabitants of the carrier traits are exactly:
> `Pos` = `{H, O<P>, I<P> | P: Pos}`; `Nat` = `{Z, Pz<P> | P: Pos}`; `Adjustment` =
> `{Ratio<N, D> | N: Pos + Gcd<D, Out = H>, D: Pos}`; `Bias` = `{BZero} ∪ {BPos<N, D>, BNeg<N, D>}`
> under the same bound. Consequently, two types inhabiting one carrier denote the same value if and
> only if they are the same type: uniqueness by file 36's induction, whose closed-world hypothesis
> is the inhabitant set above, supplied by the seal.
>
> The guarantee quantifies over **inhabitant introduction only**. It does not quantify over, and is
> not threatened by: helper-trait impls on downstream-local types (admitted, unreachable);
> foreign-parameter impls of unsealed traits on genuine inhabitants (admitted by coherence,
> unreachable, because every consuming position re-bounds both operands on carriers); observation
> (any downstream may read `VAL`/`NUM`/`DEN` and recurse structurally over the public
> constructors, which is how derived facts and convention crates are built).
>
> It rests on, and goes stale with: the seal modules staying private; every impl of a carrier trait
> keeping the two-obligation discipline (sealed at the trait, carrier-bounded through the
> parameters); the `specialization`/`TypeId` bans; and the pinned solver's coherence behaviour,
> which the compile-fail suite pins the day the tower ships.

Per the grounding discipline op adopted at `44b` and file 45's registry: the tower's bin-1 claims,
currently `conditional on seal-owed` (`45:386`), discharge to `grounded on: enc, vu, pin, ffl` when
this completed seal lands in the shipped tree with the adversary as its compile-fail suite. The
condition is discharged in the review's artifact as of this file (the composing copy is sealed on
all four carriers and attacked on every route); it is not discharged in the tree, where nothing
exists yet, and the grounding field is where that distinction stays visible, exactly as file 45
said it should (`45:427-441`).

## 5. What the seal costs, compiled

The dispatch named three crossings that must survive, and all three are positive controls in the
adversary rather than assertions.

**A consumer bringing a new numeral** composes existing inhabitants; it never needs a new one,
because every positive integer already has exactly one spelling, which is the entire point of the
encoding. `probe_6` builds MATLAB's witness (slope 1, bias 1/2, file 39's own defect case) from
sealed parts, through the normalising aliases, const-asserted to hold `NUM = 1, DEN = 2`.

**A convention crate** needs two things: derived facts over the encoding, and a contract naming
carrier-typed members. Both compile: a `BitLen` derived by structural recursion over the public
constructors (a local trait implemented for `H`, `O<P>`, `I<P>` is orphan-legal, and this is the
shape every finest-inhabited-system style derivation in section 1.6 of the consolidation takes),
and a local `Convention` trait with `Slope: Adjustment, Offset: Bias` associated types. The contract
layer stays open while the carrier layer is closed, and that split is the design principle this
file proposes naming in the spec: **seal the carriers, open the contracts.** `Numeral`, `Policy`,
`Lowering`, the convention traits, the algebra ladder: open, they are what a downstream implements.
`Pos`, `Nat`, `Adjustment`, `Bias`, and any future closed-vocabulary axis a guarantee quantifies
over (`SignDomain`'s three instances, `SignIndexing`'s four, when they are built): sealed, they are
what a downstream instantiates.

**The design's own layered crates**: `probe_7` compiles the bias layer as a genuinely separate
crate against the sealed nat crate's rlib, declaring its own sealed carrier (`Bias`) whose blanket
bounds on the upstream carriers. So the six-crate packaging is unconstrained by the seal beyond one
rule: a carrier's seal lives in the crate that declares the carrier.

**The one real cost, stated plainly**: the constructor set of each carrier is fixed at its
declaring crate. A downstream cannot add an inhabitant, and that is the guarantee itself, so the
cost and the product are the same fact. Its practical consequence is about locus, and it settles
the locus of two open items without settling their content: whether `FullRange` survives as its own
named `Adjustment` constructor (`40:688-689`), and whether `Int` stays (`45:403-425`), are both
edits to the sealing crate whichever way op calls them, never additions a consumer could make. A
convention crate that genuinely needed a new exponent-form branch (the BFP-shaped composite of
`40:96-104`) goes through a design round upstream, which is `use-the-stack-not-reinvent.md`'s
existing rule arriving at the type level.

## 6. Smaller findings made along the way

### 6.1 A refusal probe that cannot fail is not a probe: type aliases defer their bounds

My first draft of three refusal probes pushed malformed types through bare type aliases
(`pub type Attack = AdjPos<Ratio<P6, P12>>;`) and every one **compiled clean while testing
nothing**: a type alias's bounds are not checked at the alias. The review knew the lazy-alias fact
in the positive direction (it is why `Reduce` composes as an alias at all, `vu_bias_sealed.rs`
header), and I still walked into its negative face. The committed probes force well-formedness
through fn signatures or a projecting const, and OUTCOMES.md keeps the tautology on the record
rather than smoothing it over, because the lesson generalises: **when this perimeter ships as a
compile-fail suite, every case must force, or the suite is green while asserting nothing**, which
is precisely the fake-green class the workspace's test gate exists to catch.

### 6.2 The composition wall fires on concrete non-inhabitants, not only abstract parameters

`probe_5`'s entry attempt (`<Ratio<LocalNat, H> as Reduce>::N` with `LocalNat` a rigid local
non-`Pos` type) is refused, but with `E0275: overflow evaluating the requirement
Pz<O<_>>: ExactDivOdd<_>`, the same divergence signature files 41 and 42 measured for fully
abstract `N, D`, not the crisp `E0277` I predicted. The refusal is correct and the guarantee is
untouched; the new boundary fact is that the eager-confirmation divergence file 42 isolated
(`42:185-227`) does not need two abstract operands, one rigid non-inhabitant suffices. Recorded for
whoever next touches the wall's residual (`42:380-385`): it slightly worsens the diagnostic a
consumer sees on a wrong type reaching `Reduce`, and slightly strengthens the case for the
spelled-out-chain discipline at consumer-facing surfaces, where the chain's own bounds fail fast
with named types (`probe_3d`'s E0271 carries the offending gcd in the error) while the trait bound
diverges anonymously.

### 6.3 One operand position in the tower is unbounded, and the coherence door probe 4 opened reaches it

`BiasProduct<Rhs>` declares no bound on `Rhs` (`vu_bias_sealed_adj.rs`), so a downstream
`impl BiasProduct<NotABias> for BZero` is admitted (`probe_4c`, compiles clean). Not a breach: the
declared `Out: Bias` means only genuine inhabitants come out, and the impl fires only on the
downstream's own query. But it is the single public trait in the tower whose harmlessness argument
rests on the output bound rather than the input bounds, and the fix is one token:
`trait BiasProduct<Rhs: Bias>`. Then the reachability argument is uniform across the whole tower,
every parameter of every public trait carrier-bounded, which is the shape section 3's completeness
argument wants to quantify over without a special case.

### 6.4 The consolidation sentence to correct

`40:446` ("`Pos`, `Nat` and `Int` are sealed") should become, in the next consolidation: **"`Pos`,
`Nat`, `Adjustment` and `Bias` are sealed, in the copy everything composes with, attacked on every
introduction route by a committed downstream adversary (`46_probes`), at zero measured cost; `Int`
has no construction and no grounding (45's 4.1) and is not sealed because it is not built."** And
the claim's grounding gains `pin` per file 45's rule 3, until the compile-fail suite pins it.

## 7. What this file does not decide

**Whether the completed seal is ratified as the shape.** Op ratified the encoding with the seal
owed as an implementation task (`44b:17-19`); this file completes the seal's design content
(which traits, which routes, what it guarantees, what it costs) and is the first read on the
two-expert ladder for the `Adjustment` half specifically. A second member should re-run
`probe_1b` and `probe_3` from the committed files (two commands, both in OUTCOMES.md) before the
next consolidation absorbs section 4's block.

**`Int`** stays exactly where file 45 left it, op's call between drop and labelled
forward-provision; this file adds only that either answer is an edit to the sealing crate, and if
kept it owes the same two lines every carrier owes.

**Whether `BiasProduct<Rhs: Bias>` lands** (6.3) is a one-token spec edit, suggested not ruled.

**The packaging of the adversary as shipped compile-fail tests** (trybuild-shaped or the existing
`tests/ui/` pair shape) is an implementation choice for whoever lands the tower in the tree; the
design content is only that the perimeter ships as compiled refusals, per section 1.

**The E0275-diagnostic finding** (6.2) is recorded, not resolved; it belongs to the composition
wall's residual, which file 42 owns and explicitly left open.

**The sibling-evaluation sentence, tick 3, `Precise`'s combinator surface, the float model, and the
codegen regression tests** stand exactly as open as `44b` left them; nothing here touches the axes
they depend on.

## 8. Standing

The dispatch asked for the seal closed, and for "closed" to mean something stronger than the last
three times the review said it. The closure is a one-trait diff on the copy everything composes
with, priced at nothing; the "something stronger" is the shift from per-trait checking to
per-introduction-route enumeration, executed as a committed adversary whose every row is rustc
output, including both halves of the one residual the previous pass could only argue, and including
the two positive controls that prove the seal does not close the design against its own consumers.
The guarantee now has a stated quantification (section 4) sitting where the guarantee is stated,
which is what `what-you-can-observe-is-what-you-guaranteed.md` requires and what no prior file
wrote down; and the two-line-per-carrier checklist plus the ship-it-as-compile-fail rule are the
mechanism by which the fourth pass finds nothing, rather than the fourth hole. I hold the seal
question closed on this evidence, pending the second read section 7 names, and I would ask the next
consolidation to carry exactly three things from here: the corrected sentence (6.4), the
quantification block (4), and the principle that carriers are sealed and contracts are open (5),
because those three are the parts a later reader checking the guarantee will need where they are
looking, rather than reconstructed from five files of history.
