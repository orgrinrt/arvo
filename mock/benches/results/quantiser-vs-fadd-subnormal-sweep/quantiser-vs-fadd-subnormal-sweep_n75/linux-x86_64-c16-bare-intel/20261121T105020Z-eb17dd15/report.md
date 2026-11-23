# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 244% faster than the next best (quantiser-software)

quantiser-hardware (1.09 us) leads quantiser-software (3.75 us) by 244%, a clear separation rather than a photo finish. CV 28.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (1.09 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest quantiser-hardware (1.09 us) to slowest quantiser-software (3.75 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.3x its best-20%

quantiser-software's best 20% of batches run at 3.68 us but its worst 20% at 11.97 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 1089.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.44x (fastest 1089.7 ns, slowest 3748.6 ns)

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
| quantiser-hardware | 1430ns | 1537ns | 941ns | 1413ns | 1972ns | base |
| quantiser-software | 6646ns | 3816ns | 3746ns | 5757ns | 12211ns | +364.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 1036ns | 672ns | 1379ns | base | 0.247 |
| quantiser-software | 6535ns | 3678ns | 11971ns | +530.62% | 0.039 |

## Performance model

- Peak throughput: **0.381 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.235 | 61.6% |
| quantiser-software | 0.068 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1430ns | 1430ns | base |
| quantiser-software | 6646ns | 6646ns | +364.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 1090ns | base | --- | [748, 1323] | --- | --- | --- | --- |
| quantiser-software | 3749ns | +2992.5ns (+274.6%) | [+2958, +8682]ns | [3718, 10031] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 1346ns | +645.3% |
| 1 | 2 | 748ns | +396.7% |
| 1 | 3 | 695ns | +429.9% |
| 1 | 4 | 1328ns | +182.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.250 | moderate- |
| quantiser-software | -0.082 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 135.3ns | 1036.3ns | 13.1% | HIGH |
| quantiser-software | 34.5ns | 6535.2ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 671.8-1379.5 ns)
    671.8 |###
    707.1 |#######
    742.5 |########################################
    777.9 |
    813.3 |
    848.7 |
    884.1 |
    919.5 |
    954.9 |
    990.2 |
   1025.6 |###
   1061.0 |
   1096.4 |
   1131.8 |###
   1167.2 |
   1202.6 |
   1237.9 |
   1273.3 |
   1308.7 |####################################
   1344.1 |#####################
  (5 below, 3 above range)

quantiser-software (n=40, range 3678.0-11970.8 ns)
   3678.0 |########################################
   4092.7 |
   4507.3 |
   4921.9 |
   5336.6 |
   5751.2 |
   6165.9 |##
   6580.5 |
   6995.1 |
   7409.8 |
   7824.4 |
   8239.0 |
   8653.7 |
   9068.3 |
   9482.9 |
   9897.6 |##################
  10312.2 |
  10726.8 |####
  11141.5 |
  11556.1 |
  (4 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=30.1% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=15.0% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=55.8% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.3x (possible bimodal distribution)
