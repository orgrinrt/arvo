# 60_probes

Probes for `60_stam_the_chain_derived_cold.md`, built and committed during phase one, cold, before
any panel file was read. Toolchain `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f)`), passed
explicitly. Each `.rs` states its hypothesis, its shortcuts, and its mutant check at the top. The
`.out` and `.stderr` files are the raw transcripts of the committed runs.

Build and run, from this directory:

```
rustc +nightly-2026-05-28 -O p_a_dot_schedules.rs -o p_a && ./p_a
rustc +nightly-2026-05-28 --edition 2021 -O p_b_order_dependence.rs -o p_b && ./p_b
rustc +nightly-2026-05-28 --edition 2021 -O p_c1_width_algebra.rs -o p_c1 && ./p_c1
rustc +nightly-2026-05-28 --edition 2021 p_c2_general_impl_refused.rs   # MUST fail; that is the result
```

`p_a` shows the rounding side of a schedule changes the computed function (exhaustive, 46,656
inputs; count = 3-element (a,b) raw-vector pairs over a 6-value set), with the wide single-narrow
arm correctly rounded on every input, property-checked rather than recomputed. `p_b` shows the
overflow side alone makes schedules semantically distinct (saturating fold order-dependent,
wrapping fold order-independent, wide-then-saturate order-independent), plus the f64 arm of the
same taxonomy. `p_c1` shows the exact-op width algebra and an explicit adaptation are statable as
trait contracts under the feature ban and that the solver chains them. `p_c2` is a deliberate
compile failure recording that the general one-impl-for-all-widths spelling is refused without
`generic_const_exprs`.

These are spikes. Shortcuts are taken everywhere and named in each file; none of them is a design
decision. No probe here is a bench and no magnitude claim is made anywhere in them.
