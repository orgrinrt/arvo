# Probe outcomes, file 27

All three compiled with the workspace rustc, `--edition 2021 --crate-type lib`, no feature gates,
`#![no_std]`. Full diagnostics quoted in the deliverable where load-bearing.

**`probe_a_finest_system_dispatch.rs`: WORKS.** The finest-system reading compiles whole: system
markers as ZSTs, the tower order as reflexive `ContainedIn` rows on the markers, upward membership as
one blanket impl over the `Numeral::System` projection, and consumer divergence as one impl per system
marker selected through the projection, resolving in const context
(`const _: () = assert!(pipeline_of::<ModelU5_3>() == 20)` passes). No specialization, no negative
bounds, no unstable feature.

**`probe_a2_downward_refusal.rs`: FAILS WITH E0277, which is the intended result.** A bound
`Inhabits<Zint>` refuses the fractional numeral: `` the trait bound `Dyadic: ContainedIn<Zint>` is not
satisfied ``, and the diagnostic volunteers `` but trait `ContainedIn<Dyadic>` is implemented for it ``,
naming the containment that does hold. The refusal quality is better than a hand-written message.

**`probe_b_marker_lattice_divergence.rs`: FAILS WITH E0119, which is the finding.** The naive
transcription of D38's own divergence example ("when a value is in R do this, when it is not R but Z do
that") as two blanket impls distinguished by `Numeral<System = _>` equality bounds is refused:
`` conflicting implementations of trait `Algo` ``. Associated-type-equality where-clauses do not
participate in coherence. `min_specialization` does not rescue it (the impls are incomparable), and
negative bounds are not in the permitted feature set. Divergence on membership must go through the
marker-dispatch shape of probe A; it cannot be written as competing bounds on the numeral.

Next step unblocked: the deliverable's proposal that membership ships as one finest-system projection
plus a tower order, with divergence on the marker, is compile-verified in both directions (admits what
it should, refuses what it should, and the forbidden shape is structurally unwritable rather than
merely discouraged).
