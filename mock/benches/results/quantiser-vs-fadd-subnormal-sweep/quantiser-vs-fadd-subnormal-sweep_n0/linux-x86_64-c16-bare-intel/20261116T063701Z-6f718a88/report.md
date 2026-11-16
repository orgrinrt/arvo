# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 555% faster than the next best (quantiser-software)

quantiser-hardware (702 ns) leads quantiser-software (4.60 us) by 555%, a clear separation rather than a photo finish. CV 13.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows alternating (throttle bounce) (autocorr -0.80)

quantiser-hardware's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (702 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.6x the fastest

Fastest quantiser-hardware (702 ns) to slowest quantiser-software (4.60 us): 6.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.0x its best-20%

quantiser-software's best 20% of batches run at 3.75 us but its worst 20% at 11.35 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 701.8 ns median
- 1 variant significantly slower than baseline
- Spread: 6.55x (fastest 701.8 ns, slowest 4598.1 ns)

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
| quantiser-hardware | 1020ns | 985ns | 812ns | 987ns | 1325ns | base |
| quantiser-software | 5742ns | 4667ns | 3803ns | 4432ns | 11611ns | +463.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 696ns | 573ns | 822ns | base | 0.368 |
| quantiser-software | 5639ns | 3747ns | 11349ns | +710.80% | 0.045 |

## Performance model

- Peak throughput: **0.447 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.365 | 81.6% |
| quantiser-software | 0.056 | 12.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1020ns | 1020ns | base |
| quantiser-software | 5742ns | 5742ns | +463.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 702ns | base | --- | [625, 731] | --- | --- | --- | --- |
| quantiser-software | 4598ns | +3847.5ns (+548.3%) | [+3189, +4009]ns | [3775, 4634] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 625ns | +642.4% |
| 1 | 2 | 804ns | +368.2% |
| 1 | 3 | 564ns | +564.9% |
| 1 | 4 | 792ns | +481.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.798 | HIGH- (thermal bounce) |
| quantiser-software | -0.249 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 112.8ns | 695.5ns | 16.2% | HIGH |
| quantiser-software | 29.4ns | 5639.2ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 572.8-822.2 ns)
    572.8 |
    585.2 |
    597.7 |####
    610.2 |########
    622.7 |################################
    635.1 |
    647.6 |
    660.1 |####
    672.5 |
    685.0 |
    697.5 |################################
    710.0 |
    722.4 |
    734.9 |
    747.4 |####
    759.9 |
    772.3 |####
    784.8 |
    797.3 |########################################
    809.7 |
  (6 below, 2 above range)

quantiser-software (n=40, range 3746.6-11348.8 ns)
   3746.6 |########################
   4126.7 |
   4506.8 |########################################
   4886.9 |
   5267.0 |
   5647.1 |
   6027.2 |
   6407.3 |##
   6787.5 |
   7167.6 |
   7547.7 |
   7927.8 |
   8307.9 |##
   8688.0 |
   9068.1 |
   9448.2 |
   9828.4 |
  10208.5 |
  10588.6 |
  10968.7 |
  (4 below, 5 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=15.2% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=60.1% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.0x (possible bimodal distribution)
