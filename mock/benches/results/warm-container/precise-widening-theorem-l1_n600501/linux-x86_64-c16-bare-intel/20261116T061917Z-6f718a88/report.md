# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.77)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.28 us) is smaller than the fastest variant's own run-to-run std-dev (2.02 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-plusone is inconsistent: worst-20% is 5.1x its best-20%

warm-container-plusone's best 20% of batches run at 1.29 us but its worst 20% at 6.55 us (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 2662.2 ns median (-1.4% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.48x (fastest 2662.2 ns, slowest 3939.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112735203 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3990ns | 2793ns | 1786ns | 3359ns | 8089ns | base |
| warm-container-kernel | 4307ns | 2994ns | 2215ns | 3375ns | 9196ns | +7.94% |
| warm-container-lanes-deferred | 4217ns | 3360ns | 1875ns | 3574ns | 8487ns | +5.68% |
| warm-container-minimum | 3508ns | 4153ns | 1296ns | 3598ns | 5450ns | -12.09% |
| warm-container-native | 4281ns | 3517ns | 2155ns | 3561ns | 8567ns | +7.29% |
| warm-container-plusone | 3529ns | 2779ns | 1358ns | 3133ns | 6886ns | -11.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3832ns | 1713ns | 7779ns | base | 4.276 |
| warm-container-kernel | 4172ns | 2141ns | 8887ns | +8.87% | 3.928 |
| warm-container-lanes-deferred | 4082ns | 1805ns | 8222ns | +6.53% | 4.014 |
| warm-container-minimum | 3319ns | 1228ns | 5128ns | -13.39% | 4.937 |
| warm-container-native | 4140ns | 2072ns | 8238ns | +8.04% | 3.958 |
| warm-container-plusone | 3360ns | 1292ns | 6549ns | -12.30% | 4.876 |

## Performance model

- Peak throughput: **13.341 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.065 | 45.5% |
| warm-container-kernel | 5.611 | 42.1% |
| warm-container-lanes-deferred | 5.000 | 37.5% |
| warm-container-minimum | 4.159 | 31.2% |
| warm-container-native | 4.782 | 35.8% |
| warm-container-plusone | 6.154 | 46.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3990ns | 3990ns | base |
| warm-container-kernel | 4307ns | 4307ns | +7.94% |
| warm-container-lanes-deferred | 4217ns | 4217ns | +5.68% |
| warm-container-minimum | 3508ns | 3508ns | -12.09% |
| warm-container-native | 4281ns | 4281ns | +7.29% |
| warm-container-plusone | 3529ns | 3529ns | -11.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2701ns | base | --- | [2397, 4188] | --- | --- | --- | --- |
| warm-container-kernel | 2920ns | no significant difference | [-54, +852]ns | [2560, 3160] | no | 0.1009 | 0.0807 | 0 |
| warm-container-lanes-deferred | 3277ns | +976.8ns (+36.2%) | [+448, +1158]ns | [3275, 3282] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-minimum | 3939ns | -107.1ns (-4.0%) | [-1030, -7]ns | [2344, 4360] | YES | 0.0415 | 0.0166 | 0 |
| warm-container-native | 3426ns | no significant difference | [-809, +1265]ns | [3407, 3436] | no | 0.6358 | 0.6358 | 0 |
| warm-container-plusone | 2662ns | -510.6ns (-18.9%) | [-816, -190]ns | [2175, 4079] | YES | 0.0111 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2420ns | +29.2% | +35.4% | +69.2% | +41.9% | +61.9% |
| 1 | 2 | 2720ns | -13.3% | -22.0% | +43.0% | -29.8% | -19.0% |
| 1 | 3 | 2214ns | +18.1% | +48.0% | +63.9% | +54.8% | +74.6% |
| 1 | 4 | 4188ns | -35.0% | -37.4% | -15.7% | -32.9% | -69.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.287 | moderate- |
| warm-container-kernel | -0.380 | moderate- |
| warm-container-lanes-deferred | -0.770 | HIGH- (thermal bounce) |
| warm-container-minimum | 0.285 | moderate+ |
| warm-container-native | -0.709 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.582 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 40.7ns | 3831.7ns | 1.1% |  |
| warm-container-kernel | 35.7ns | 4171.6ns | 0.9% |  |
| warm-container-lanes-deferred | 33.8ns | 4081.8ns | 0.8% |  |
| warm-container-minimum | 52.9ns | 3318.7ns | 1.6% |  |
| warm-container-native | 36.4ns | 4139.8ns | 0.9% |  |
| warm-container-plusone | 46.3ns | 3360.2ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1713.1-7778.6 ns)
   1713.1 |
   2016.3 |##################################
   2319.6 |##################################
   2622.9 |########################################
   2926.2 |
   3229.4 |
   3532.7 |#####
   3836.0 |###########
   4139.3 |#####
   4442.6 |###########
   4745.8 |###########
   5049.1 |#####
   5352.4 |
   5655.7 |
   5959.0 |
   6262.2 |
   6565.5 |#####
   6868.8 |#####
   7172.1 |#################
   7475.4 |#####
  (4 below, 2 above range)

warm-container-kernel (n=40, range 2140.7-8887.3 ns)
   2140.7 |##################
   2478.0 |########################################
   2815.4 |#########################
   3152.7 |###
   3490.0 |
   3827.4 |
   4164.7 |###
   4502.0 |
   4839.3 |###
   5176.7 |#######
   5514.0 |###
   5851.3 |
   6188.7 |###
   6526.0 |
   6863.3 |
   7200.7 |###
   7538.0 |
   7875.3 |###
   8212.7 |
   8550.0 |###
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1805.2-8221.7 ns)
   1805.2 |
   2126.1 |##
   2446.9 |############
   2767.7 |##
   3088.5 |########################################
   3409.4 |
   3730.2 |
   4051.0 |
   4371.8 |
   4692.7 |##
   5013.5 |#######
   5334.3 |
   5655.1 |
   5976.0 |##
   6296.8 |##
   6617.6 |##
   6938.4 |
   7259.3 |##
   7580.1 |
   7900.9 |##
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1228.1-5127.8 ns)
   1228.1 |########################################
   1423.1 |
   1618.1 |
   1813.1 |#####
   2008.1 |
   2203.0 |
   2398.0 |
   2593.0 |#####
   2788.0 |#####
   2983.0 |
   3178.0 |
   3372.9 |
   3567.9 |##########
   3762.9 |##########
   3957.9 |##########
   4152.9 |####################
   4347.9 |###############
   4542.9 |##########
   4737.8 |###############
   4932.8 |#####
  (5 below, 5 above range)

warm-container-native (n=40, range 2072.1-8237.9 ns)
   2072.1 |
   2380.4 |#####
   2688.7 |############
   2997.0 |
   3305.3 |########################################
   3613.5 |
   3921.8 |##
   4230.1 |
   4538.4 |##
   4846.7 |
   5155.0 |
   5463.3 |#####
   5771.6 |
   6079.9 |##
   6388.1 |
   6696.4 |#####
   7004.7 |
   7313.0 |
   7621.3 |
   7929.6 |
  (6 below, 4 above range)

warm-container-plusone (n=40, range 1292.0-6548.6 ns)
   1292.0 |###########
   1554.8 |######################
   1817.6 |
   2080.5 |########################################
   2343.3 |
   2606.1 |
   2869.0 |#####
   3131.8 |
   3394.6 |#####
   3657.4 |#################
   3920.3 |###########
   4183.1 |#################
   4445.9 |
   4708.8 |###########
   4971.6 |#####
   5234.4 |#####
   5497.2 |#####
   5760.1 |
   6022.9 |#####
   6285.7 |#####
  (7 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=58.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-kernel**: CV=62.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=58.0% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=47.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-native**: CV=57.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-plusone**: CV=60.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 5.1x (possible bimodal distribution)
