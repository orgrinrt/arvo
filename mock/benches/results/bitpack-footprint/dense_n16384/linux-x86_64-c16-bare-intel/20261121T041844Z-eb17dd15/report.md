# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (2.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.51 us to slowest 2.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2.31 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-footprint-dense is inconsistent: worst-20% is 3.4x its best-20%

bitpack-footprint-dense's best 20% of batches run at 2.50 us but its worst 20% at 8.53 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 2506.8 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2506.8 ns, slowest 2506.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Dense footprint: sequential sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112804717 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 4319ns | 2559ns | 2547ns | 3430ns | 8761ns | base |
| bitpack-footprint-dense-alt | 3894ns | 2559ns | 2547ns | 2828ns | 8439ns | -9.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 4222ns | 2496ns | 8534ns | base | 3.881 |
| bitpack-footprint-dense-alt | 3805ns | 2496ns | 8215ns | -9.87% | 4.306 |

## Performance model

- Peak throughput: **6.564 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.535 | 99.6% |
| bitpack-footprint-dense-alt | 6.536 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 4319ns | 4319ns | base |
| bitpack-footprint-dense-alt | 3894ns | 3894ns | -9.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 2507ns | base | --- | [2497, 3197] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 2507ns | no significant difference | [-60, +1]ns | [2497, 3051] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 2501ns | -0.2% |
| 1 | 2 | 4059ns | -38.5% |
| 1 | 3 | 2502ns | -0.0% |
| 1 | 4 | 2501ns | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.417 | moderate- |
| bitpack-footprint-dense-alt | -0.077 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 32.4ns | 4221.7ns | 0.8% |  |
| bitpack-footprint-dense-alt | 29.3ns | 3805.0ns | 0.8% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2496.1-8534.1 ns)
   2496.1 |########################################
   2798.0 |###########
   3099.9 |##
   3401.8 |
   3703.7 |
   4005.6 |
   4307.5 |
   4609.4 |##
   4911.3 |####
   5213.2 |
   5515.1 |
   5817.0 |
   6118.9 |
   6420.8 |
   6722.7 |
   7024.6 |##
   7326.5 |####
   7628.4 |####
   7930.3 |
   8232.2 |##
  (4 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 2496.1-8215.4 ns)
   2496.1 |########################################
   2782.1 |##############
   3068.1 |
   3354.0 |
   3640.0 |
   3925.9 |
   4211.9 |
   4497.9 |
   4783.8 |##
   5069.8 |
   5355.8 |
   5641.7 |
   5927.7 |
   6213.6 |##
   6499.6 |##
   6785.6 |
   7071.5 |
   7357.5 |
   7643.5 |##
   7929.4 |##
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=58.7% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=60.7% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.3x (possible bimodal distribution)
