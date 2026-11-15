# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 222% faster than the next best (quantiser-software)

quantiser-hardware (1.15 us) leads quantiser-software (3.72 us) by 222%, a clear separation rather than a photo finish. CV 31.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest quantiser-hardware (1.15 us) to slowest quantiser-software (3.72 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.3x its best-20%

quantiser-software's best 20% of batches run at 3.66 us but its worst 20% at 11.97 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 1154.6 ns median
- 1 variant significantly slower than baseline
- Spread: 3.22x (fastest 1154.6 ns, slowest 3716.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1666ns | 1432ns | 1256ns | 1525ns | 2497ns | base |
| quantiser-software | 5754ns | 3786ns | 3727ns | 4257ns | 12275ns | +245.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 1321ns | 998ns | 1983ns | base | 0.194 |
| quantiser-software | 5633ns | 3658ns | 11971ns | +326.36% | 0.045 |

## Performance model

- Peak throughput: **0.257 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.222 | 86.4% |
| quantiser-software | 0.069 | 26.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1666ns | 1666ns | base |
| quantiser-software | 5754ns | 5754ns | +245.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 1155ns | base | --- | [1130, 1275] | --- | --- | --- | --- |
| quantiser-software | 3716ns | +2611.6ns (+226.2%) | [+2555, +2714]ns | [3692, 3745] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 1903ns | +92.3% |
| 1 | 2 | 1130ns | +228.2% |
| 1 | 3 | 1164ns | +217.2% |
| 1 | 4 | 1061ns | +252.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.091 | ok |
| quantiser-software | -0.219 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 123.0ns | 1321.1ns | 9.3% | HIGH |
| quantiser-software | 36.7ns | 5632.7ns | 0.7% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 997.5-1982.8 ns)
    997.5 |##############################
   1046.8 |###############
   1096.0 |########################################
   1145.3 |###################################
   1194.6 |
   1243.8 |###############
   1293.1 |##########
   1342.4 |
   1391.6 |
   1440.9 |#####
   1490.2 |
   1539.4 |
   1588.7 |
   1637.9 |
   1687.2 |
   1736.5 |
   1785.7 |##########
   1835.0 |##########
   1884.3 |
   1933.5 |#####
  (1 below, 4 above range)

quantiser-software (n=40, range 3657.7-11970.8 ns)
   3657.7 |########################################
   4073.3 |
   4489.0 |
   4904.6 |
   5320.3 |#
   5736.0 |
   6151.6 |
   6567.3 |
   6982.9 |#
   7398.6 |
   7814.2 |
   8229.9 |
   8645.5 |
   9061.2 |
   9476.9 |###
   9892.5 |
  10308.2 |
  10723.8 |#
  11139.5 |#
  11555.1 |#
  (5 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=27.8% (high variance, measurements may be unstable)
- **quantiser-software**: CV=60.1% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.3x (possible bimodal distribution)
