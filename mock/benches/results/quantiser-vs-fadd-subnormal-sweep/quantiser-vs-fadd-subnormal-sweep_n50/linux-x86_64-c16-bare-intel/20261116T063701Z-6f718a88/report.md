# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 379% faster than the next best (quantiser-software)

quantiser-hardware (819 ns) leads quantiser-software (3.93 us) by 379%, a clear separation rather than a photo finish. CV 20.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (819 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest quantiser-hardware (819 ns) to slowest quantiser-software (3.93 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.5x its best-20%

quantiser-software's best 20% of batches run at 3.30 us but its worst 20% at 11.70 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 819.1 ns median
- 1 variant significantly slower than baseline
- Spread: 4.79x (fastest 819.1 ns, slowest 3925.6 ns)

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
| Counter frequency | 2112891188 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1139ns | 1248ns | 717ns | 1179ns | 1439ns | base |
| quantiser-software | 5682ns | 3994ns | 3358ns | 4345ns | 12020ns | +398.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 786ns | 511ns | 947ns | base | 0.326 |
| quantiser-software | 5562ns | 3300ns | 11702ns | +607.69% | 0.046 |

## Performance model

- Peak throughput: **0.501 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.313 | 62.4% |
| quantiser-software | 0.065 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1139ns | 1139ns | base |
| quantiser-software | 5682ns | 5682ns | +398.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 819ns | base | --- | [775, 933] | --- | --- | --- | --- |
| quantiser-software | 3926ns | +3189.3ns (+389.3%) | [+3002, +3423]ns | [3910, 3936] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 509ns | +673.3% |
| 1 | 2 | 801ns | +386.8% |
| 1 | 3 | 934ns | +320.3% |
| 1 | 4 | 933ns | +244.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.153 | ok |
| quantiser-software | -0.103 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 118.2ns | 786.0ns | 15.0% | HIGH |
| quantiser-software | 33.8ns | 5562.3ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 511.1-947.0 ns)
    511.1 |##
    532.9 |
    554.7 |#####
    576.5 |
    598.3 |
    620.1 |
    641.9 |
    663.7 |
    685.5 |#####
    707.3 |##
    729.0 |##
    750.8 |
    772.6 |
    794.4 |################
    816.2 |##
    838.0 |
    859.8 |
    881.6 |########
    903.4 |
    925.2 |########################################
  (7 below, 1 above range)

quantiser-software (n=40, range 3299.8-11702.4 ns)
   3299.8 |
   3720.0 |########################################
   4140.1 |
   4560.2 |
   4980.4 |
   5400.5 |
   5820.6 |
   6240.7 |
   6660.9 |
   7081.0 |#
   7501.1 |
   7921.3 |
   8341.4 |
   8761.5 |#
   9181.7 |###
   9601.8 |
  10021.9 |#
  10442.0 |
  10862.2 |
  11282.3 |###
  (7 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=21.6% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=15.0% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=59.9% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.5x (possible bimodal distribution)
