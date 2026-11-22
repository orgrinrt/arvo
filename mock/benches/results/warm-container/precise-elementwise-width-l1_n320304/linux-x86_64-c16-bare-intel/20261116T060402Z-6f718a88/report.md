# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom dominates: 12% faster than the next best (warm-container-plusone)

warm-container-headroom (3.08 us) leads warm-container-plusone (3.44 us) by 12%, a clear separation rather than a photo finish. CV 86.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-headroom is fastest but the noisiest (CV 86.7%)

warm-container-headroom wins on median (3.08 us) yet has the highest variance (CV 86.7%), while warm-container-minimum is the steadiest (CV 30.9%, 3.46 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.61)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (511 ns) is smaller than the fastest variant's own run-to-run std-dev (2.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (3.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-kernel is inconsistent: worst-20% is 4.4x its best-20%

warm-container-kernel's best 20% of batches run at 1.10 us but its worst 20% at 4.85 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 3075.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 3075.4 ns, slowest 3585.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112840787 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4553ns | 3146ns | 2876ns | 3396ns | 9702ns | base |
| warm-container-kernel | 3284ns | 3753ns | 1153ns | 3383ns | 5122ns | -27.87% |
| warm-container-minimum | 3385ns | 3623ns | 1414ns | 3607ns | 4689ns | -25.66% |
| warm-container-native | 3534ns | 3683ns | 1379ns | 3491ns | 5821ns | -22.38% |
| warm-container-plusone | 4680ns | 3523ns | 3143ns | 3557ns | 9585ns | +2.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4440ns | 2808ns | 9429ns | base | 9.226 |
| warm-container-kernel | 3126ns | 1102ns | 4853ns | -29.60% | 13.105 |
| warm-container-minimum | 3225ns | 1351ns | 4447ns | -27.36% | 12.702 |
| warm-container-native | 3364ns | 1317ns | 5524ns | -24.22% | 12.176 |
| warm-container-plusone | 4565ns | 3076ns | 9313ns | +2.82% | 8.973 |

## Performance model

- Peak throughput: **37.185 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 13.319 | 35.8% |
| warm-container-kernel | 11.423 | 30.7% |
| warm-container-minimum | 11.836 | 31.8% |
| warm-container-native | 11.636 | 31.3% |
| warm-container-plusone | 11.907 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4553ns | 4553ns | base |
| warm-container-kernel | 3284ns | 3284ns | -27.87% |
| warm-container-minimum | 3385ns | 3385ns | -25.66% |
| warm-container-native | 3534ns | 3534ns | -22.38% |
| warm-container-plusone | 4680ns | 4680ns | +2.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3075ns | base | --- | [3071, 3090] | --- | --- | --- | --- |
| warm-container-kernel | 3586ns | no significant difference | [-1842, +419]ns | [3337, 3765] | no | 0.5728 | 0.4296 | 0 |
| warm-container-minimum | 3461ns | no significant difference | [-1716, +307]ns | [3212, 3727] | no | 1.0000 | 1.0000 | 0 |
| warm-container-native | 3520ns | no significant difference | [-1463, +179]ns | [2932, 3992] | no | 0.5364 | 0.2682 | 0 |
| warm-container-plusone | 3440ns | +322.9ns (+10.5%) | [+142, +373]ns | [3272, 3768] | YES | 0.0089 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3084ns | +20.9% | +12.5% | +11.0% | +0.1% |
| 1 | 2 | 3081ns | -8.7% | -39.8% | +27.0% | +11.5% |
| 1 | 3 | 2821ns | +30.1% | +14.7% | -50.8% | +9.6% |
| 1 | 4 | 2813ns | +23.6% | +34.3% | +19.8% | +34.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.259 | moderate+ |
| warm-container-kernel | -0.611 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.257 | moderate- |
| warm-container-native | -0.445 | moderate- |
| warm-container-plusone | -0.474 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 30.6ns | 4439.5ns | 0.7% |  |
| warm-container-kernel | 59.3ns | 3125.6ns | 1.9% |  |
| warm-container-minimum | 60.1ns | 3224.7ns | 1.9% |  |
| warm-container-native | 64.3ns | 3364.1ns | 1.9% |  |
| warm-container-plusone | 30.8ns | 4564.9ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2807.6-9428.7 ns)
   2807.6 |########################################
   3138.7 |
   3469.7 |#
   3800.8 |
   4131.8 |#
   4462.9 |
   4794.0 |
   5125.0 |
   5456.1 |
   5787.1 |
   6118.2 |
   6449.2 |
   6780.3 |
   7111.3 |
   7442.4 |
   7773.5 |
   8104.5 |#
   8435.6 |####
   8766.6 |
   9097.7 |###
  (4 below, 3 above range)

warm-container-kernel (n=40, range 1101.5-4853.0 ns)
   1101.5 |########################################
   1289.1 |
   1476.7 |
   1664.2 |#####
   1851.8 |
   2039.4 |
   2227.0 |#####
   2414.5 |
   2602.1 |
   2789.7 |
   2977.2 |
   3164.8 |#####
   3352.4 |##############################
   3540.0 |#####
   3727.5 |###################################
   3915.1 |####################
   4102.7 |#####
   4290.3 |#####
   4477.8 |##########
   4665.4 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 1350.6-4447.4 ns)
   1350.6 |
   1505.4 |
   1660.2 |
   1815.1 |
   1969.9 |
   2124.8 |
   2279.6 |
   2434.4 |
   2589.3 |#####
   2744.1 |
   2899.0 |#################
   3053.8 |#################
   3208.7 |######################
   3363.5 |######################
   3518.3 |###########
   3673.2 |########################################
   3828.0 |
   3982.9 |#################
   4137.7 |
   4292.5 |###########
  (7 below, 4 above range)

warm-container-native (n=40, range 1316.7-5523.9 ns)
   1316.7 |########################################
   1527.1 |
   1737.4 |####
   1947.8 |####
   2158.2 |
   2368.5 |
   2578.9 |####
   2789.2 |
   2999.6 |
   3209.9 |################
   3420.3 |############
   3630.7 |################
   3841.0 |############
   4051.4 |
   4261.7 |########
   4472.1 |
   4682.4 |################
   4892.8 |####
   5103.2 |
   5313.5 |########
  (1 below, 3 above range)

warm-container-plusone (n=40, range 3076.0-9313.0 ns)
   3076.0 |###################################
   3387.9 |########################################
   3699.7 |########################################
   4011.6 |
   4323.4 |
   4635.3 |
   4947.1 |
   5259.0 |
   5570.8 |
   5882.7 |
   6194.5 |
   6506.4 |
   6818.2 |
   7130.1 |
   7441.9 |####
   7753.8 |####
   8065.6 |####
   8377.5 |####
   8689.3 |####
   9001.2 |
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.0% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-kernel**: CV=45.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=33.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-native**: CV=46.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=55.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.0x (possible bimodal distribution)
