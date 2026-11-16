# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 378% faster than the next best (quantiser-software)

quantiser-hardware (751 ns) leads quantiser-software (3.59 us) by 378%, a clear separation rather than a photo finish. CV 31.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows alternating (throttle bounce) (autocorr -0.72)

quantiser-hardware's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (751 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest quantiser-hardware (751 ns) to slowest quantiser-software (3.59 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.7x its best-20%

quantiser-software's best 20% of batches run at 2.81 us but its worst 20% at 10.49 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 750.8 ns median
- 1 variant significantly slower than baseline
- Spread: 4.78x (fastest 750.8 ns, slowest 3587.9 ns)

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
| quantiser-hardware | 1119ns | 1119ns | 786ns | 1068ns | 1602ns | base |
| quantiser-software | 5498ns | 3656ns | 2863ns | 4614ns | 10785ns | +391.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 785ns | 561ns | 1120ns | base | 0.326 |
| quantiser-software | 5374ns | 2807ns | 10493ns | +584.64% | 0.048 |

## Performance model

- Peak throughput: **0.457 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.341 | 74.7% |
| quantiser-software | 0.071 | 15.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1119ns | 1119ns | base |
| quantiser-software | 5498ns | 5498ns | +391.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 751ns | base | --- | [623, 853] | --- | --- | --- | --- |
| quantiser-software | 3588ns | +2825.9ns (+376.4%) | [+2787, +4024]ns | [3388, 4803] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 623ns | +465.5% |
| 1 | 2 | 865ns | +315.7% |
| 1 | 3 | 561ns | +505.7% |
| 1 | 4 | 934ns | +213.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.724 | HIGH- (thermal bounce) |
| quantiser-software | 0.114 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 133.3ns | 785.0ns | 17.0% | HIGH |
| quantiser-software | 37.8ns | 5374.4ns | 0.7% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 560.6-1120.2 ns)
    560.6 |#################
    588.6 |########
    616.6 |###################################
    644.6 |
    672.6 |####
    700.5 |####
    728.5 |
    756.5 |
    784.5 |##########################
    812.5 |
    840.4 |
    868.4 |
    896.4 |####
    924.4 |########################################
    952.3 |
    980.3 |
   1008.3 |
   1036.3 |
   1064.3 |
   1092.2 |####
  (4 below, 3 above range)

quantiser-software (n=40, range 2806.9-10493.2 ns)
   2806.9 |##################################
   3191.2 |########################################
   3575.6 |########################################
   3959.9 |#####
   4344.2 |
   4728.5 |
   5112.8 |#####
   5497.1 |
   5881.4 |
   6265.7 |
   6650.1 |
   7034.4 |
   7418.7 |
   7803.0 |###########
   8187.3 |###########
   8571.6 |#####
   8955.9 |#####
   9340.3 |###########
   9724.6 |
  10108.9 |#####
  (5 below, 4 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=29.7% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=16.2% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=57.7% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.7x (possible bimodal distribution)
