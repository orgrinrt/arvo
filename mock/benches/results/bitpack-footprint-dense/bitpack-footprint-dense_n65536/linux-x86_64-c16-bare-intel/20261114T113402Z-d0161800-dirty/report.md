# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (12.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 11.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.02 us) is smaller than the fastest variant's own run-to-run std-dev (1.10 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-footprint-dense-alt's edge over baseline is significant but tiny (-3 ns, 0.02%)

bitpack-footprint-dense-alt differs from baseline bitpack-footprint-dense by -3 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 11181.1 ns median (-8.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.09x (fastest 11181.1 ns, slowest 12198.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11980ns | 12283ns | 10037ns | 12104ns | 13551ns | base |
| bitpack-footprint-dense-alt | 11171ns | 11254ns | 10046ns | 11172ns | 12294ns | -6.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 11888ns | 9969ns | 13425ns | base | 5.513 |
| bitpack-footprint-dense-alt | 11093ns | 9980ns | 12202ns | -6.69% | 5.908 |

## Performance model

- Peak throughput: **6.574 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.372 | 81.7% |
| bitpack-footprint-dense-alt | 5.861 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 11980ns | 11980ns | base |
| bitpack-footprint-dense-alt | 11171ns | 11171ns | -6.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 12199ns | base | --- | [12198, 12200] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 11181ns | -2.7ns (-0.0%) | [-14, -0]ns | [9981, 12190] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 12199ns | -0.1% |
| 1 | 2 | 9969ns | +0.1% |
| 1 | 3 | 12866ns | -22.4% |
| 1 | 4 | 12199ns | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.468 | moderate- |
| bitpack-footprint-dense-alt | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 24.0ns | 11887.8ns | 0.2% |  |
| bitpack-footprint-dense-alt | 21.4ns | 11092.8ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 9969.0-13425.4 ns)
   9969.0 |#######
  10141.9 |
  10314.7 |
  10487.5 |
  10660.3 |
  10833.1 |
  11005.9 |
  11178.8 |
  11351.6 |
  11524.4 |
  11697.2 |
  11870.0 |
  12042.8 |########################################
  12215.6 |#####
  12388.5 |
  12561.3 |
  12734.1 |
  12906.9 |
  13079.7 |
  13252.5 |#
  (6 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 9980.1-12201.7 ns)
   9980.1 |###################################
  10091.2 |##
  10202.3 |
  10313.3 |
  10424.4 |
  10535.5 |
  10646.6 |
  10757.7 |
  10868.7 |
  10979.8 |
  11090.9 |
  11202.0 |
  11313.0 |
  11424.1 |
  11535.2 |
  11646.3 |
  11757.4 |
  11868.4 |
  11979.5 |
  12090.6 |########################################
  (3 below, 2 above range)

```
