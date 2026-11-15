# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.76)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.78 us) is smaller than the fastest variant's own run-to-run std-dev (2.48 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-container-minimum's best 20% of batches run at 1.86 us but its worst 20% at 7.27 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2778.6 ns median (-25.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.64x (fastest 2778.6 ns, slowest 4561.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3708ns | 3903ns | 1627ns | 3519ns | 6357ns | base |
| warm-container-kernel | 4688ns | 3242ns | 2753ns | 3333ns | 10690ns | +26.43% |
| warm-container-lanes-deferred | 4073ns | 2871ns | 2563ns | 2984ns | 8847ns | +9.82% |
| warm-container-minimum | 4163ns | 3103ns | 1946ns | 3749ns | 7621ns | +12.26% |
| warm-container-native | 4698ns | 3819ns | 2583ns | 4068ns | 8702ns | +26.68% |
| warm-container-plusone | 4092ns | 4789ns | 1601ns | 4250ns | 6110ns | +10.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3514ns | 1547ns | 5982ns | base | 4.662 |
| warm-container-kernel | 4551ns | 2664ns | 10343ns | +29.49% | 3.600 |
| warm-container-lanes-deferred | 3927ns | 2475ns | 8494ns | +11.74% | 4.172 |
| warm-container-minimum | 3979ns | 1865ns | 7269ns | +13.23% | 4.117 |
| warm-container-native | 4528ns | 2493ns | 8362ns | +28.85% | 3.618 |
| warm-container-plusone | 3874ns | 1519ns | 5733ns | +10.24% | 4.229 |

## Performance model

- Peak throughput: **10.786 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.413 | 40.9% |
| warm-container-kernel | 5.189 | 48.1% |
| warm-container-lanes-deferred | 5.896 | 54.7% |
| warm-container-minimum | 5.499 | 51.0% |
| warm-container-native | 4.439 | 41.2% |
| warm-container-plusone | 3.592 | 33.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3708ns | 3708ns | base |
| warm-container-kernel | 4688ns | 4688ns | +26.43% |
| warm-container-lanes-deferred | 4073ns | 4073ns | +9.82% |
| warm-container-minimum | 4163ns | 4163ns | +12.26% |
| warm-container-native | 4698ns | 4698ns | +26.68% |
| warm-container-plusone | 4092ns | 4092ns | +10.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3713ns | base | --- | [1596, 4773] | --- | --- | --- | --- |
| warm-container-kernel | 3158ns | +1221.5ns (+32.9%) | [+238, +1541]ns | [3090, 3337] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-container-lanes-deferred | 2779ns | no significant difference | [-181, +1120]ns | [2699, 2837] | no | 0.1009 | 0.0807 | 0 |
| warm-container-minimum | 2979ns | no significant difference | [-15, +1143]ns | [2833, 4351] | no | 0.1009 | 0.0807 | 0 |
| warm-container-native | 3691ns | +1185.8ns (+31.9%) | [+799, +1996]ns | [2847, 4544] | YES | 0.0321 | 0.0064 | 0 |
| warm-container-plusone | 4562ns | no significant difference | [-111, +61]ns | [3737, 5138] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4620ns | -32.3% | -38.7% | -38.0% | -46.8% | -24.8% |
| 1 | 2 | 1583ns | +77.5% | +71.3% | +75.0% | +157.6% | +189.2% |
| 1 | 3 | 4446ns | -28.1% | -36.9% | +10.8% | -35.0% | +0.2% |
| 1 | 4 | 1993ns | +75.6% | +28.1% | +104.7% | +41.0% | +129.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.764 | HIGH- (thermal bounce) |
| warm-container-kernel | 0.045 | ok |
| warm-container-lanes-deferred | -0.344 | moderate- |
| warm-container-minimum | 0.036 | ok |
| warm-container-native | -0.498 | moderate- |
| warm-container-plusone | -0.155 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 54.8ns | 3514.4ns | 1.6% |  |
| warm-container-kernel | 34.3ns | 4550.8ns | 0.8% |  |
| warm-container-lanes-deferred | 38.7ns | 3927.1ns | 1.0% |  |
| warm-container-minimum | 49.1ns | 3979.4ns | 1.2% |  |
| warm-container-native | 44.7ns | 4528.3ns | 1.0% |  |
| warm-container-plusone | 61.3ns | 3874.0ns | 1.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1546.8-5981.8 ns)
   1546.8 |########################################
   1768.6 |
   1990.3 |
   2212.1 |##
   2433.8 |
   2655.6 |
   2877.3 |
   3099.1 |##
   3320.8 |##
   3542.6 |
   3764.3 |
   3986.0 |##
   4207.8 |#####
   4429.5 |#####
   4651.3 |#####
   4873.0 |#####
   5094.8 |#######
   5316.5 |##
   5538.3 |##########
   5760.0 |##
  (1 below, 2 above range)

warm-container-kernel (n=40, range 2664.1-10342.5 ns)
   2664.1 |################
   3048.0 |########################################
   3431.9 |#######
   3815.8 |
   4199.8 |##
   4583.7 |
   4967.6 |
   5351.5 |
   5735.5 |
   6119.4 |
   6503.3 |
   6887.2 |
   7271.1 |
   7655.1 |
   8039.0 |
   8422.9 |
   8806.8 |##
   9190.8 |
   9574.7 |##
   9958.6 |####
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2474.7-8494.0 ns)
   2474.7 |########################################
   2775.6 |#########################
   3076.6 |
   3377.6 |##
   3678.5 |
   3979.5 |
   4280.5 |
   4581.4 |
   4882.4 |##
   5183.3 |
   5484.3 |##
   5785.3 |
   6086.2 |##
   6387.2 |
   6688.2 |
   6989.1 |
   7290.1 |
   7591.1 |#####
   7892.0 |
   8193.0 |##
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1864.9-7268.8 ns)
   1864.9 |
   2135.1 |
   2405.3 |##############
   2675.5 |########################################
   2945.7 |###
   3215.9 |
   3486.1 |
   3756.2 |###
   4026.4 |##########
   4296.6 |#######
   4566.8 |#######
   4837.0 |#######
   5107.2 |#######
   5377.4 |
   5647.6 |##########
   5917.8 |
   6188.0 |
   6458.2 |
   6728.4 |
   6998.6 |
  (5 below, 4 above range)

warm-container-native (n=40, range 2492.5-8362.2 ns)
   2492.5 |############################
   2786.0 |########################################
   3079.5 |###########
   3373.0 |
   3666.5 |
   3960.0 |##################################
   4253.4 |
   4546.9 |#####
   4840.4 |#####
   5133.9 |#####
   5427.4 |
   5720.9 |
   6014.4 |#####
   6307.8 |
   6601.3 |#####
   6894.8 |
   7188.3 |###########
   7481.8 |
   7775.3 |###########
   8068.8 |#####
  (6 below, 4 above range)

warm-container-plusone (n=40, range 1519.0-5733.2 ns)
   1519.0 |########################################
   1729.7 |#####
   1940.4 |
   2151.1 |
   2361.8 |
   2572.6 |
   2783.3 |
   2994.0 |
   3204.7 |
   3415.4 |#####
   3626.1 |
   3836.8 |##########
   4047.5 |#####
   4258.2 |##########
   4468.9 |###############
   4679.6 |##########
   4890.4 |#####
   5101.1 |###################################
   5311.8 |###############
   5522.5 |##########
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=52.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=64.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=63.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=52.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=49.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-plusone**: CV=43.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
