# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 446% faster than the next best (quantiser-software)

quantiser-hardware (627 ns) leads quantiser-software (3.42 us) by 446%, a clear separation rather than a photo finish. CV 33.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-software shows alternating (throttle bounce) (autocorr -0.55)

quantiser-software's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (627 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.5x the fastest

Fastest quantiser-hardware (627 ns) to slowest quantiser-software (3.42 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.9x its best-20%

quantiser-software's best 20% of batches run at 2.90 us but its worst 20% at 11.26 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 626.8 ns median
- 1 variant significantly slower than baseline
- Spread: 5.46x (fastest 626.8 ns, slowest 3420.2 ns)

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
| quantiser-hardware | 930ns | 878ns | 600ns | 897ns | 1358ns | base |
| quantiser-software | 5221ns | 3489ns | 2952ns | 3872ns | 11538ns | +461.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 662ns | 424ns | 985ns | base | 0.387 |
| quantiser-software | 5111ns | 2896ns | 11263ns | +671.73% | 0.050 |

## Performance model

- Peak throughput: **0.603 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.408 | 67.7% |
| quantiser-software | 0.075 | 12.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 930ns | 930ns | base |
| quantiser-software | 5221ns | 5221ns | +461.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 627ns | base | --- | [564, 693] | --- | --- | --- | --- |
| quantiser-software | 3420ns | +2850.0ns (+454.7%) | [+2606, +2926]ns | [3028, 3757] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 916ns | +310.6% |
| 1 | 2 | 627ns | +364.7% |
| 1 | 3 | 564ns | +505.9% |
| 1 | 4 | 434ns | +567.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | 0.101 | ok |
| quantiser-software | -0.547 | HIGH- (thermal bounce) |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 99.6ns | 662.3ns | 15.0% | HIGH |
| quantiser-software | 33.6ns | 5111.0ns | 0.7% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 424.3-984.9 ns)
    424.3 |###################################
    452.3 |
    480.3 |
    508.4 |
    536.4 |#########################
    564.4 |##########
    592.5 |
    620.5 |########################################
    648.5 |
    676.5 |#####
    704.6 |##########
    732.6 |
    760.6 |
    788.7 |###############
    816.7 |
    844.7 |##########
    872.7 |#####
    900.8 |
    928.8 |####################
    956.8 |
  (3 below, 2 above range)

quantiser-software (n=40, range 2896.1-11263.4 ns)
   2896.1 |########################################
   3314.4 |############################
   3732.8 |############################
   4151.2 |
   4569.5 |
   4987.9 |####
   5406.3 |
   5824.6 |####
   6243.0 |
   6661.4 |
   7079.7 |
   7498.1 |
   7916.5 |
   8334.8 |
   8753.2 |####
   9171.6 |########
   9590.0 |
  10008.3 |####
  10426.7 |####
  10845.1 |####
  (5 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=31.5% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=14.7% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=65.5% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.9x (possible bimodal distribution)
