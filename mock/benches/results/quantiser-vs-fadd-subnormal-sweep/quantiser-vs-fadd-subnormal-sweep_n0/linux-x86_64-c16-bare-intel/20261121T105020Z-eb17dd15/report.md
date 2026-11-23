# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 268% faster than the next best (quantiser-software)

quantiser-hardware (1.06 us) leads quantiser-software (3.91 us) by 268%, a clear separation rather than a photo finish. CV 23.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (1.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest quantiser-hardware (1.06 us) to slowest quantiser-software (3.91 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.0x its best-20%

quantiser-software's best 20% of batches run at 3.89 us but its worst 20% at 11.69 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 1060.6 ns median
- 1 variant significantly slower than baseline
- Spread: 3.68x (fastest 1060.6 ns, slowest 3905.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Round-first software quantiser add vs native hardware fadd, subnormal fraction swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112817978 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1239ns | 1317ns | 920ns | 1249ns | 1530ns | base |
| quantiser-software | 5760ns | 3962ns | 3942ns | 4318ns | 11904ns | +364.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 961ns | 638ns | 1253ns | base | 0.266 |
| quantiser-software | 5668ns | 3886ns | 11685ns | +489.91% | 0.045 |

## Performance model

- Peak throughput: **0.401 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.241 | 60.2% |
| quantiser-software | 0.066 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1239ns | 1239ns | base |
| quantiser-software | 5760ns | 5760ns | +364.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 1061ns | base | --- | [767, 1143] | --- | --- | --- | --- |
| quantiser-software | 3905ns | +3197.6ns (+301.5%) | [+2847, +3566]ns | [3898, 4682] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 1133ns | +243.9% |
| 1 | 2 | 838ns | +460.1% |
| 1 | 3 | 738ns | +429.1% |
| 1 | 4 | 1111ns | +250.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.251 | moderate- |
| quantiser-software | -0.413 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 97.3ns | 960.8ns | 10.1% | HIGH |
| quantiser-software | 28.1ns | 5667.7ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 638.4-1252.7 ns)
    638.4 |
    669.1 |#####
    699.8 |#########################
    730.5 |#####
    761.3 |##########
    792.0 |##########
    822.7 |
    853.4 |#####
    884.1 |
    914.8 |
    945.5 |
    976.3 |
   1007.0 |#####
   1037.7 |#####
   1068.4 |#####
   1099.1 |##########
   1129.8 |########################################
   1160.6 |##########
   1191.3 |####################
   1222.0 |
  (6 below, 3 above range)

quantiser-software (n=40, range 3886.1-11685.1 ns)
   3886.1 |########################################
   4276.0 |
   4666.0 |################
   5055.9 |
   5445.9 |
   5835.8 |##
   6225.8 |
   6615.7 |##
   7005.7 |
   7395.6 |
   7785.6 |
   8175.5 |
   8565.5 |##
   8955.4 |
   9345.4 |
   9735.3 |
  10125.3 |
  10515.2 |
  10905.2 |
  11295.1 |####
  (4 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=25.4% (high variance, measurements may be unstable)
- **quantiser-software**: CV=57.5% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.0x (possible bimodal distribution)
