# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom dominates: 16% faster than the next best (warm-container-minimum)

warm-container-headroom (3.32 us) leads warm-container-minimum (3.84 us) by 16%, a clear separation rather than a photo finish. CV 62.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-headroom is fastest but the noisiest (CV 62.2%)

warm-container-headroom wins on median (3.32 us) yet has the highest variance (CV 62.2%), while warm-container-kernel is the steadiest (CV 38.4%, 4.21 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.79)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (882 ns) is smaller than the fastest variant's own run-to-run std-dev (2.07 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (3.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-minimum is inconsistent: worst-20% is 4.3x its best-20%

warm-container-minimum's best 20% of batches run at 1.32 us but its worst 20% at 5.65 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 3323.9 ns median
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.27x (fastest 3323.9 ns, slowest 4205.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112787121 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4582ns | 3391ns | 3380ns | 3713ns | 8392ns | base |
| warm-container-kernel | 3669ns | 4417ns | 1547ns | 3687ns | 5737ns | -19.93% |
| warm-container-lanes-deferred | 3563ns | 4400ns | 1393ns | 3620ns | 5560ns | -22.25% |
| warm-container-minimum | 3442ns | 4036ns | 1384ns | 3283ns | 5979ns | -24.88% |
| warm-container-native | 4034ns | 4279ns | 1697ns | 4111ns | 6140ns | -11.96% |
| warm-container-plusone | 4843ns | 4133ns | 3385ns | 3928ns | 9048ns | +5.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4484ns | 3314ns | 8193ns | base | 7.308 |
| warm-container-kernel | 3484ns | 1472ns | 5421ns | -22.30% | 9.405 |
| warm-container-lanes-deferred | 3377ns | 1326ns | 5239ns | -24.68% | 9.703 |
| warm-container-minimum | 3266ns | 1318ns | 5650ns | -27.17% | 10.034 |
| warm-container-native | 3830ns | 1616ns | 5805ns | -14.57% | 8.555 |
| warm-container-plusone | 4733ns | 3318ns | 8812ns | +5.56% | 6.923 |

## Performance model

- Peak throughput: **24.868 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.858 | 39.6% |
| warm-container-kernel | 7.791 | 31.3% |
| warm-container-lanes-deferred | 7.846 | 31.5% |
| warm-container-minimum | 8.534 | 34.3% |
| warm-container-native | 8.047 | 32.4% |
| warm-container-plusone | 8.089 | 32.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4582ns | 4582ns | base |
| warm-container-kernel | 3669ns | 3669ns | -19.93% |
| warm-container-lanes-deferred | 3563ns | 3563ns | -22.25% |
| warm-container-minimum | 3442ns | 3442ns | -24.88% |
| warm-container-native | 4034ns | 4034ns | -11.96% |
| warm-container-plusone | 4843ns | 4843ns | +5.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3324ns | base | --- | [3315, 3689] | --- | --- | --- | --- |
| warm-container-kernel | 4206ns | -1459.9ns (-43.9%) | [-1706, -43]ns | [1860, 4502] | YES (adj: no) | 0.1345 | 0.0807 | 0 |
| warm-container-lanes-deferred | 4176ns | no significant difference | [-1987, +111]ns | [1427, 4652] | no | 0.1345 | 0.0807 | 0 |
| warm-container-minimum | 3840ns | no significant difference | [-1998, +206]ns | [1366, 4475] | no | 0.1923 | 0.1539 | 0 |
| warm-container-native | 4072ns | no significant difference | [-1699, +541]ns | [3618, 4844] | no | 0.8746 | 0.8746 | 0 |
| warm-container-plusone | 4051ns | +191.1ns (+5.8%) | [+1, +470]ns | [3516, 4052] | YES (adj: no) | 0.0829 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3316ns | +35.0% | +25.5% | -60.2% | +22.8% | +6.0% |
| 1 | 2 | 3316ns | +25.9% | -4.7% | +17.3% | -20.9% | +22.2% |
| 1 | 3 | 3315ns | -50.9% | +27.0% | -58.8% | +37.8% | +0.1% |
| 1 | 4 | 5191ns | -16.6% | -16.4% | -7.1% | -3.3% | -21.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.084 | ok |
| warm-container-kernel | -0.363 | moderate- |
| warm-container-lanes-deferred | -0.294 | moderate- |
| warm-container-minimum | -0.637 | HIGH- (thermal bounce) |
| warm-container-native | -0.077 | ok |
| warm-container-plusone | -0.788 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 26.5ns | 4483.7ns | 0.6% |  |
| warm-container-kernel | 52.1ns | 3484.0ns | 1.5% |  |
| warm-container-lanes-deferred | 52.1ns | 3377.2ns | 1.5% |  |
| warm-container-minimum | 49.0ns | 3265.6ns | 1.5% |  |
| warm-container-native | 57.3ns | 3830.3ns | 1.5% |  |
| warm-container-plusone | 29.2ns | 4733.0ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3314.1-8193.3 ns)
   3314.1 |########################################
   3558.0 |
   3802.0 |
   4046.0 |#####
   4289.9 |
   4533.9 |
   4777.8 |#
   5021.8 |###
   5265.8 |#
   5509.7 |#
   5753.7 |
   5997.7 |
   6241.6 |
   6485.6 |
   6729.6 |
   6973.5 |
   7217.5 |
   7461.4 |#
   7705.4 |
   7949.4 |
  (3 below, 5 above range)

warm-container-kernel (n=40, range 1471.9-5421.1 ns)
   1471.9 |########################################
   1669.4 |
   1866.8 |
   2064.3 |####
   2261.8 |####
   2459.2 |
   2656.7 |
   2854.1 |
   3051.6 |####
   3249.0 |
   3446.5 |
   3644.0 |
   3841.4 |
   4038.9 |################
   4236.3 |####
   4433.8 |############################
   4631.2 |########
   4828.7 |################
   5026.1 |####
   5223.6 |####
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1326.0-5238.6 ns)
   1326.0 |########################################
   1521.6 |
   1717.3 |
   1912.9 |
   2108.5 |
   2304.1 |
   2499.8 |
   2695.4 |
   2891.0 |
   3086.7 |
   3282.3 |
   3477.9 |##
   3673.6 |
   3869.2 |##
   4064.8 |##########
   4260.4 |
   4456.1 |##########
   4651.7 |#####
   4847.3 |################
   5043.0 |########
  (1 below, 3 above range)

warm-container-minimum (n=40, range 1317.7-5649.9 ns)
   1317.7 |########################################
   1534.3 |##########
   1750.9 |
   1967.5 |
   2184.1 |
   2400.7 |
   2617.3 |
   2833.9 |
   3050.5 |
   3267.2 |###
   3483.8 |
   3700.4 |#############
   3917.0 |
   4133.6 |######
   4350.2 |######
   4566.8 |
   4783.4 |#############
   5000.0 |###
   5216.6 |##########
   5433.3 |
  (3 below, 5 above range)

warm-container-native (n=40, range 1616.2-5804.9 ns)
   1616.2 |########################################
   1825.7 |
   2035.1 |
   2244.6 |
   2454.0 |
   2663.4 |
   2872.9 |#####
   3082.3 |
   3291.7 |
   3501.2 |###############
   3710.6 |###############
   3920.0 |#####
   4129.5 |##########
   4338.9 |##########
   4548.3 |
   4757.8 |####################
   4967.2 |
   5176.6 |###############
   5386.1 |###############
   5595.5 |
  (4 below, 6 above range)

warm-container-plusone (n=40, range 3318.2-8812.3 ns)
   3318.2 |############################
   3592.9 |
   3867.6 |########################################
   4142.3 |
   4417.0 |
   4691.7 |
   4966.4 |
   5241.1 |
   5515.8 |
   5790.5 |
   6065.2 |
   6340.0 |
   6614.7 |
   6889.4 |
   7164.1 |##
   7438.8 |
   7713.5 |
   7988.2 |
   8262.9 |
   8537.6 |##
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=46.1% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=46.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=50.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=55.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-native**: CV=42.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=52.6% (high variance, measurements may be unstable)
