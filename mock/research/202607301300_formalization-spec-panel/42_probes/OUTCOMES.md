# Probe outcomes, file 42

All probes compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
confirmed with `rustc --version` from inside the repo. `vu_nat.rs` and `vu_bias.rs` in this directory
are copies of `41_probes/vu_nat.rs` / `41_probes/vu_bias.rs`, unmodified, kept as the unsealed base
state the attacks run against. `vu_nat_sealed.rs` and `vu_bias_sealed.rs` are the fix (probe 3).

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_widen_adjustment_via_fabricated_pos_lib.rs` + `probe_1b_widen_adjustment_via_fabricated_pos.rs` | Does a genuinely separate downstream crate widen `Adjustment` without implementing `Adjustment` at all, by fabricating a foreign `Pos` type and feeding it to `Adjustment`'s already-exported blanket impl? | WORKS (the defect: this should have been refused). `Fabricated: Pos` with `VAL = 4` and `Fabricated: Gcd<D, Out = H>` unconditionally (no real coprimality check), then `Ratio<Fabricated, D4>` satisfies `Adjustment` with `NUM = 4, DEN = 4`, an unreduced fraction, through the ALREADY-shipped blanket impl. Neither `Fabricated`'s `Pos` impl nor its `Gcd` impl needs a seal, because `Pos`/`Gcd` have none in `vu_nat.rs`, the module `Adjustment` actually composes with. |
| `probe_2_widen_bias_via_fabricated_pos_lib.rs` + `probe_2b_widen_bias_via_fabricated_pos.rs` | Does the identical attack succeed against `Bias`, whose OWN trait file 41 sealed and verified closed (`41_probes/probe_5`/`probe_5b`)? | WORKS (the headline defect). `Bias`'s own seal (`bias_sealed::BiasSealed`) is never touched: the attack supplies `BPos<Fabricated, D4>` directly to `Bias`'s upstream blanket impl (`impl<N: Pos + Gcd<D, Out=H>, D: Pos> Bias for BPos<N,D>`), which is a normal generic impl over the LOCAL type `BPos`, satisfiable by any caller-supplied `N`, `D` regardless of orphan rules. File 41's own two closed routes (implement `BiasSealed` directly; feed `BPos` an unreduced pair of `H`/`O`/`I`) never covered this third route, one layer below both. |
| `probe_3_sealed_tower_refuses_both_lib.rs` + `probe_3b_sealed_tower_refuses_both.rs` | Does sealing `Pos`/`Nat` (the private-supertrait pattern `36_probes/probe_5` already demonstrated in isolation, applied here to the module everything actually composes with) close BOTH attacks, with zero change to `Adjustment`, `Bias`, `Gcd`, `ExactDivOdd`, `Strip2` or `Reduce`? | FAILS on both, at the shared root (`impl Pos for Fabricated`), one E0277: "the trait bound `Fabricated: PosSealed` is not satisfied ... `Pos` is a sealed trait". Confirms sealing `Pos` alone (never `Gcd` separately) is sufficient: a foreign type can no longer become a `Pos` at all, so it never reaches the point of satisfying `Gcd<D, Out=H>`. |
| `probe_4_reduce_chain_as_bare_bounds.rs` | Does `Reduce`'s own where-clause chain (`Strip2`, `Gcd`, `ExactDivOdd`, `AsPos`), copied verbatim onto an unrelated function's signature as ordinary bounds, compose generically? | WORKS. The chain itself does not diverge as a set of deferred assumptions. |
| `probe_4b_bare_reduce_bound_diverges.rs` | Does naming `Reduce` ITSELF as a bound (`T: Reduce`), unmodified from file 41's own probe 2(b), still diverge, confirming the trigger is impl-selection-and-confirmation rather than the underlying machinery probe 4 shows composes fine? | FAILS WITH E0275, "overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`", verbatim below. Matches file 41's own probe 2(b) error text exactly, independently re-derived. |
| (negative control, not committed as a file; recorded here) | Does dropping the `: Pos` bound on `Reduce`'s own associated types (eager well-formedness checking of the declared bound, an alternative explanation) change the outcome? | FAILS IDENTICALLY. Ruled out: the trigger is naming `Reduce` as a bound at all, independent of what its associated types are declared to satisfy. |
| (two negative controls, not committed; recorded here) | Does an abstract `Wrap<P>`-wrapped recursive trait, over (a) a bare rigid type parameter and (b) an unresolved associated-type projection, diverge as an unused generic bound the way `ExactDivOdd`'s own `Pz<P>` wrapper was hypothesised to? | Both COMPILE CLEAN. Falsifies "wrapper position alone" as a sufficient explanation; corroborates that the trigger is specifically naming `Reduce`, not a wrapper shape in the abstract. |
| `probe_4c_recursion_limit_crashes_rustc.rs` | Does raising `#![recursion_limit]` on probe 4b's identical bare-`Reduce`-bound file (per rustc's own suggested remediation) produce a clean, deeper answer, or does it confirm the divergence is genuinely unbounded? | rustc CRASHES, SIGBUS, inside `rustc_trait_selection`'s `OpportunisticVarResolver`, reproduced twice independently on the pinned nightly. Stronger evidence than a bare overflow diagnostic that this is a genuine unbounded search, not a shallow default-limit artifact. NOT part of the standard build sweep; do not add to an unattended build script. |
| `probe_5_generic_biasmul.rs` | Does file 41's own conclusion ("the design cannot have a generic `BiasMul` trait") hold, or does spelling the chain directly (per probe 4's finding) let a fully generic `BiasMulGeneric` trait exist? | WORKS, contradicting file 41's stated conclusion. Correct against both of file 41's own probe-3 witnesses: 1/2 * 5/2 = 5/4 (all sign combinations file 41 checked reduce to the same magnitude construction here), and 2/3 * 3/4's raw componentwise product 6/12 correctly renormalising to 1/2. One repair beyond mechanical unbundling: `Reduce`'s own declared associated-type bound (`: Pos` alone) does not carry coprimality, an axiom the design already relies on informally; a generic caller producing a `BPos` output has to assert it explicitly (`FinalN: Pos + Gcd<FinalD, Out = H>`). |
| `probe_6_adjustment_half.rs` | Is the adjustment half of the closure formula (`31:397-400`, `gcd(A1*A2, A1*B2, A2*B1)`), generalised over rational A1, A2, B1, B2, buildable from existing machinery (no new arithmetic primitive)? | WORKS, both witnesses. Witness 1 (A1=3/4, A2=1/2, B1=1/2, B2=1/3 -> 1/8) and witness 2 (A1=2/3, A2=3/5, B1=1/4, B2=5/6 -> 1/180, cross-denominator lcm(5,9,20)=180), both cross-checked against Python's `fractions.Fraction` before being spelled at the type level (methodology below), every intermediate value asserted, not only the final answer. |
| `probe_7_unified_biasproduct.rs` | Does file 41's own open item ("whether the sign-and-magnitude split ... is the shape the eventual shipped crate should carry") close in favour of a single unified trait covering all nine sign combinations (four new, plus file 41's own three zero-handling impls, unchanged)? | WORKS. No coherence overlap against file 41's own three impls. Four correctness witnesses: file 41's own 1/2 * 5/2 = 5/4, a sign-mixing case (1/2 * -5/2 = -5/4) neither file 41 nor probe 5 needed alone, and a zero-absorption case confirming coexistence with the unchanged existing impls. |
| `probe_7b_shared_helper_trait_also_diverges.rs` | Does factoring the magnitude computation into a shared helper trait (rather than inlining it four times, once per sign combination) survive, the way probe 4 shows the constituent facts survive as bare bounds? | FAILS WITH E0275, `Ratio<O<_>, O<_>>: Strip2` overflow, verbatim below, once the helper trait's associated type is projected and re-bounded (`MagN<..>: Pos`), the shape any real caller needs to name the computed type. Corrected from an initial, too-strong attempt: the SAME bound alone, unused, with nothing projected from it, does NOT diverge, unlike probe 4b's bare `Ratio<N, D>: Reduce`. Recorded as the narrower, testing-corrected finding rather than smoothed into probe 4's own broader claim. |

## Probe 4b, exact error (rustc, verbatim)

```
error[E0275]: overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`
   --> probe_4b_bare_reduce_bound_diverges.rs:30:18
    |
 30 |     Ratio<N, D>: Reduce,
    |                  ^^^^^^
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`probe_4b_bare_reduce_bound_diverges`)
note: required for `Pz<O<O<_>>>` to implement `ExactDivOdd<_>`
   --> vu_nat.rs:312:22
    |
312 | impl<P: Pos, D: Pos> ExactDivOdd<D> for Pz<O<P>>
    |                      ^^^^^^^^^^^^^^     ^^^^^^^^
...
315 |     <Pz<P> as ExactDivOdd<D>>::Out: Dbl,
    |                                     --- unsatisfied trait bound introduced here
    = note: 126 redundant requirements hidden
    = note: required for `Pz<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<...>>>>>>>>>>>>>>>>>>>>>>` to implement `ExactDivOdd<_>`
note: required for `Ratio<N, D>` to implement `Reduce`
   --> vu_nat.rs:396:22
    |
396 | impl<N: Pos, D: Pos> Reduce for Ratio<N, D>
    |                      ^^^^^^     ^^^^^^^^^^^
...
406 |     >>::Out: AsPos,
    |              ----- unsatisfied trait bound introduced here

error: aborting due to 1 previous error
```

## Probe 3b, exact error (rustc, verbatim)

```
error[E0277]: the trait bound `Fabricated: nat::sealed::PosSealed` is not satisfied
  --> probe_3b_sealed_tower_refuses_both.rs:36:14
   |
36 | impl Pos for Fabricated {
   |              ^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `nat::sealed::PosSealed` is not implemented for `Fabricated`
  --> probe_3b_sealed_tower_refuses_both.rs:35:1
   |
35 | pub struct Fabricated;
   | ^^^^^^^^^^^^^^^^^^^^^
help: the following other types implement trait `nat::sealed::PosSealed`
  --> vu_nat_sealed.rs:47:1
   |
47 | impl sealed::PosSealed for H {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `H`
48 | impl<P: Pos> sealed::PosSealed for O<P> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `O<P>`
49 | impl<P: Pos> sealed::PosSealed for I<P> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `I<P>`
note: required by a bound in `Pos`
  --> vu_nat_sealed.rs:40:16
   |
40 | pub trait Pos: sealed::PosSealed {
   |                ^^^^^^^^^^^^^^^^^ required by this bound in `Pos`
   = note: `Pos` is a "sealed trait", because to implement it you also need to implement
     `vu_sealed_tower::bias::nat::sealed::PosSealed`, which is not accessible

error: aborting due to 1 previous error
```

## Probe 7b, exact error (rustc, verbatim)

```
error[E0275]: overflow evaluating the requirement `Ratio<O<_>, O<_>>: Strip2`
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate
note: required for `Ratio<O<O<_>>, O<O<_>>>` to implement `Strip2`
   --> vu_nat.rs:351:22
    |
351 | impl<A: Pos, B: Pos> Strip2 for Ratio<O<A>, O<B>>
    |                      ^^^^^^     ^^^^^^^^^^^^^^^^^
352 | where
353 |     Ratio<A, B>: Strip2,
    |                  ------ unsatisfied trait bound introduced here
    = note: 125 redundant requirements hidden
    = note: required for `Ratio<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<...>>>>>>>>>>>>>>>>>>, ...>` to implement `Strip2`
note: required for `Mag` to implement `Magnitude<(_, _, _, _)>`
   --> probe_7b_shared_helper_trait_also_diverges.rs:52:5
    |
 52 |     Magnitude<(N1, D1, N2, D2)> for Mag
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
 60 |     Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    |                               ---------- unsatisfied trait bound introduced here

error: aborting due to 1 previous error
```

Note the failing requirement is `Strip2`, not `ExactDivOdd`: naming a DIFFERENT fresh trait as a bound
diverges at a DIFFERENT point in the same underlying chain (`Strip2`'s own recursive impl over
`Ratio<O<A>, O<B>>`, rather than `ExactDivOdd`'s over `Pz<O<P>>`), consistent with the mechanism being
about impl-selection-and-confirmation in general rather than about any one link in the chain
specifically.

## Probe 4c, crash signature (verbatim head)

```
error: rustc interrupted by SIGBUS, printing backtrace

0   librustc_driver-... 0x... _RNvNtCs..._17rustc_driver_impl14signal_handler17print_stack_trace + 140
1   libsystem_platform.dylib 0x... _sigtramp + 56
2   librustc_driver-... 0x... _RINvXsl_...12rustc_middle2ty16structural_implsNtB8_2TyINtNtCs...
    13rustc_type_ir4fold17TypeSuperFoldableNtNtB8_7context6TyCtxtE15super_fold_withNtNtNtCs...
    11rustc_infer5infer7resolve24OpportunisticVarResolverECs..._21rustc_trait_selection + 340

note: we would appreciate a report at https://github.com/rust-lang/rust
```

Reproduced twice independently, in two separate invocations, both times inside the same
`rustc_infer::infer::resolve::OpportunisticVarResolver` frame. The two runs' symbol hashes differ
(rebuild-to-rebuild ASLR/hash-salt variance is expected), the crash site does not.

## Price (measured, `price/`)

`price/gen.py` + `price/sweep.sh`, the same methodology as `36_probes/price/` and `41_probes/price/`
(a seeded generator, `rustc --edition 2021 --crate-type lib --emit=metadata`, min-of-1 wall time,
baseline at count 0 subtracted, every instantiation forced by a const assertion against a
Python-computed value). Three kinds swept, all at 8-bit operands (the width comparable to file 36's
own 12.07 ms/composition `Reduce` headline and file 41's own comparable point), counts 0, 25, 50, 100,
200, 400, least-squares slope over the non-zero counts:

| kind | what it measures | ms/composition (least-squares slope) |
|---|---|---|
| `alias` | `BiasMulPP`, file 41's bare-alias mechanism, unsealed tower | 15.407 |
| `generic` | `BiasMulGeneric`, probe 5's generic trait, unsealed tower | 15.738 |
| `alias_sealed` | `BiasMulPP` against the sealed tower (probe 3's fix) | 15.486 |

All three within roughly 2% of each other. The generic trait's compile cost is not distinguishable
from the bare alias's at this scope; sealing `Pos`/`Nat` adds no measurable cost either. Metadata size
at 400 compositions: `alias` 2242.3 bytes/composition, `generic` 2227.6 bytes/composition (marginally
smaller, not larger), `alias_sealed` 2247.4 bytes/composition. Zero symbols emitted in every case
(`PhantomData`-only types throughout; not independently re-checked with `nm -g` in this file since
file 41 and file 36 already established this class of type never emits a symbol and nothing here
changes that class).

**Scope stated honestly**, matching file 41's own discipline: min-of-1 rather than min-of-3, 8-bit
only rather than the full 8/16-bit cross, three kinds rather than a wider matrix. These are
single-dispatch wall-clock decisions, not claims about what the mechanisms cost beyond what is
reported. The absolute numbers here (15.4-15.7 ms/composition for `BiasMulPP`, the same mechanism file
41 measured at 19.10 ms/composition at 8-bit) differ from file 41's own recorded figure by roughly 20
percent; this is read as machine-load and min-of-1 noise between independent dispatch sessions, not as
a correction to file 41's number, and is exactly why the RELATIVE comparison (all three kinds run in
the same session, same machine, same load) rather than the absolute figure is what this file's
recommendation rests on.

## Witness cross-checks (Python, independent of the type-level construction)

```python
from fractions import Fraction as F
from math import gcd

def rat_gcd(terms):
    dens = [t.denominator for t in terms]
    D = 1
    for d in dens:
        D = D * d // gcd(D, d)
    nums = [abs(t.numerator) * (D // t.denominator) for t in terms]
    g = 0
    for n in nums:
        g = gcd(g, n)
    return F(g, D)

# witness 1
A1, A2, B1, B2 = F(3, 4), F(1, 2), F(1, 2), F(1, 3)
assert rat_gcd([A1 * A2, A1 * B2, A2 * B1]) == F(1, 8)

# witness 2
A1, A2, B1, B2 = F(2, 3), F(3, 5), F(1, 4), F(5, 6)
assert rat_gcd([A1 * A2, A1 * B2, A2 * B1]) == F(1, 180)
```

Both assertions pass. The type-level construction in `probe_6_adjustment_half.rs` was written to match
these results, not the reverse: the Python computation ran first, independently, before a single type
alias was spelled.
