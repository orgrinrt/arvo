# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 457% faster than the next best (quantiser-software)

quantiser-hardware (706 ns) leads quantiser-software (3.94 us) by 457%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (706 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.6x the fastest

Fastest quantiser-hardware (706 ns) to slowest quantiser-software (3.94 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.7x its best-20%

quantiser-software's best 20% of batches run at 3.27 us but its worst 20% at 12.16 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 706.0 ns median
- 1 variant significantly slower than baseline
- Spread: 5.57x (fastest 706.0 ns, slowest 3935.6 ns)

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
| quantiser-hardware | 1040ns | 991ns | 878ns | 1033ns | 1223ns | base |
| quantiser-software | 5658ns | 4003ns | 3330ns | 4183ns | 12413ns | +443.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 730ns | 627ns | 822ns | base | 0.350 |
| quantiser-software | 5554ns | 3274ns | 12156ns | +660.36% | 0.046 |

## Performance model

- Peak throughput: **0.408 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.363 | 88.8% |
| quantiser-software | 0.065 | 15.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1040ns | 1040ns | base |
| quantiser-software | 5658ns | 5658ns | +443.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 706ns | base | --- | [705, 806] | --- | --- | --- | --- |
| quantiser-software | 3936ns | +3129.3ns (+443.2%) | [+2899, +3277]ns | [3616, 3975] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 706ns | +463.1% |
| 1 | 2 | 807ns | +387.6% |
| 1 | 3 | 706ns | +363.9% |
| 1 | 4 | 627ns | +427.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.037 | ok |
| quantiser-software | 0.251 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 116.7ns | 730.4ns | 16.0% | HIGH |
| quantiser-software | 32.7ns | 5553.7ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 626.8-821.8 ns)
    626.8 |##############
    636.5 |
    646.3 |
    656.0 |##
    665.8 |
    675.5 |
    685.3 |
    695.0 |##
    704.8 |##################################
    714.5 |
    724.3 |##
    734.0 |
    743.8 |
    753.5 |
    763.3 |
    773.0 |
    782.8 |##
    792.5 |
    802.3 |########################################
    812.0 |
  (4 below, 1 above range)

quantiser-software (n=40, range 3274.2-12156.0 ns)
   3274.2 |########################
   3718.3 |########################################
   4162.4 |
   4606.5 |
   5050.5 |
   5494.6 |##
   5938.7 |
   6382.8 |##
   6826.9 |
   7271.0 |##
   7715.1 |
   8159.2 |
   8603.3 |
   9047.4 |
   9491.5 |
   9935.6 |
  10379.7 |##
  10823.7 |
  11267.8 |#####
  11711.9 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=14.7% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=61.8% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.7x (possible bimodal distribution)
