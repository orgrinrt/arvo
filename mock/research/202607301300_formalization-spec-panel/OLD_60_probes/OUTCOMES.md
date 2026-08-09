# Probe outcomes, file 60

Every probe built fresh for this dispatch on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, run from the repo root so a bare `rustc` resolves `rust-toolchain.toml`'s
pin (checked: `rustc --version` from the repo root reports the pin). Command shape throughout, no
other codegen flags:

```
rustc --edition 2024 <probe>.rs -o /tmp/<name>
```

No probe here depends on the `arvo` crate. Where a probe reproduces a shipped mechanism it says so
inline and cites the exact `file:line` it copied, per the same discipline files 58 and 59 already
use for reduced-to-essentials probes.

## probe_1_shipped_total_cmp_is_datum_level.rs, WORKS

Reproduces `total_cmp_f32` verbatim from `arvo/src/traits/total_ord.rs:29-41` (read fresh for this
dispatch). Compiles clean, runs, prints:

```
total_cmp_f32(-0.0, 0.0)  = Less (shipped: NOT Equal)
total_cmp_f32(nan1, nan2) = Less (shipped: NOT Equal)
probe_1 WORKS: shipped total_cmp is datum-level (distinguishes -0.0 from 0.0 and NaN payloads)
```

`-0.0` sorts strictly before `0.0` and two differently-payloaded quiet NaNs sort against each
other, both by direct compiled evidence, not by reading the algorithm and reasoning about it.

## probe_2_value_level_reading_compiles.rs, WORKS

Canonicalise-then-compare: fold the `-0.0`/`0.0` cohort and every NaN payload to one representative
bit pattern, then run the identical datum comparator. `const fn` throughout, no forbidden feature.
Compiles clean, prints:

```
value_total_cmp_f32(-0.0, 0.0)  = Equal (value-level: Equal)
value_total_cmp_f32(nan1, nan2) = Equal (value-level: Equal)
value_total_cmp_f32(1.0, 2.0)   = Less (still a real order)
probe_2 WORKS: the value-level reading compiles, const, no forbidden feature
```

Both readings are constructible. The fork is not about which one CAN be built.

## probe_3a_the_law_the_shipped_order_fails.rs, FAILS TO COMPILE (E0080), as predicted

The law "two data that denote the same value compare Equal" stated as a `const` assertion over the
shipped datum order, applied to the `-0.0`/`0.0` cohort. Verbatim:

```
error[E0080]: evaluation panicked: the shipped order does not respect the value-equality of -0.0 and 0.0
  --> probe_3a_the_law_the_shipped_order_fails.rs:44:5
   |
44 | /     assert!(
45 | |         matches!(ord, Ordering::Equal),
46 | |         "the shipped order does not respect the value-equality of -0.0 and 0.0"
47 | |     );
   | |_____^ evaluation of `LAW_HOLDS_FOR_SHIPPED_ORDER` failed here

note: erroneous constant encountered
  --> probe_3a_the_law_the_shipped_order_fails.rs:52:13
   |
52 |     let _ = LAW_HOLDS_FOR_SHIPPED_ORDER;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error
```

This is the decisive result, not a supporting one. Per `a-test-that-cannot-compile-is-the-finding`,
a contract test that will not compile forecloses more than a failing assertion would: there is no
value the law could accept here, at any input, because the shipped order's own definition places
the cohort apart. No sampling, no threshold, no weaker reading of the law survives it.

## probe_3b_the_law_the_value_order_holds.rs, WORKS

The identical assertion text, against the value-level reading. Compiles clean:

```
probe_3b WORKS: the identical law holds under the value-level reading
```

3a and 3b differ in exactly one thing: which comparator the law is checked against. The compiler's
answer flips from refusal to acceptance with no other change. That is "let the compiles decide" in
its sharpest form available in this language: the same claim is unstatable against one candidate and
provable against the other.

## probe_4_spectral_bisection_nan_instability.rs, WORKS

Reproduces `arvo-spectral/src/partition.rs:59`'s classification line verbatim in shape ("class 0
iff `total_cmp(x, zero) == Greater`, else class 1", matching the pattern at `partition.rs:59,156,181`
exactly), against a degenerate (NaN) Fiedler component differing only in its sign bit. Compiles,
runs:

```
datum order:  class(nan, sign=0) = 0, class(nan, sign=1) = 1
value order:  class(nan, sign=0) = 0, class(nan, sign=1) = 0
probe_4 WORKS: under the shipped datum order, spectral_bisection's class assignment for a degenerate
(NaN) Fiedler component depends on the NaN's sign bit alone. Under the value order it does not.
```

The two NaN inputs represent the identical "not a value" condition (per the crossing contract,
`Specials::NAN` is not itself a rational, so no VALUE distinguishes them at all); the shipped order
still splits them into opposite classes, and the split tracks nothing but which operand order a
prior arithmetic step happened to divide in. This is not hypothetical: it is the exact classification
line hilavitkutin's `spectral_partition` step consumes to seed a `FiberGrouping`
(`~/Dev/clause-dev/hilavitkutin/mock/design_rounds/202605300120/202605300120_topic.plan-spectral-fibers.md`),
run against the exact function it calls into.

## probe_5_fixed_point_fork_is_moot.rs, WORKS

Spot-checks (three 512-wide windows at the bottom, middle, and top of the range, for both an
unsigned and a two's-complement signed stand-in) that a bit-comparator over an injective encoding is
automatically value-respecting. Compiles, runs:

```
probe_5 WORKS: 788482 unsigned pairs, 786432 signed pairs, zero forks
for an injective encoding (UFixed/IFixed today), datum_cmp == Equal iff value_cmp == Equal always;
the fork this file exists to resolve does not apply to arvo-graph/arvo-comb's shipped weight types
at all.
```

Not exhaustive over the full `u16`/`i16` range (that would be a different, much larger sweep this
file does not need to make its point; the claim being checked is a structural fact about injectivity
that does not depend on which window is sampled, and three windows at the range's extremes plus its
centre is enough to falsify it if it were false anywhere). The structural argument (`arvo/src/
traits/total_ord.rs:60-79` routes `UFixed`/`IFixed` through `arvo_storage::ConstOrd` over a
container with one datum per value) is stated in the probe's own header and is not, on its own, this
file's evidence; the compiled spot check is.
