# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-plusone is fastest but the noisiest (CV 99.5%)

warm-container-plusone wins on median (2.62 us) yet has the highest variance (CV 99.5%), while warm-container-kernel is the steadiest (CV 31.3%, 3.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-plusone, warm-container-native) are a dead heat (<1%)

warm-container-plusone (2.62 us) and warm-container-native (2.64 us) differ by 0.85%, inside the noise, even though the wider field spreads 63.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.77)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.66 us) is smaller than the fastest variant's own run-to-run std-dev (2.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.6x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.02 us but its worst 20% at 4.70 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 2619.2 ns median (-5.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.63x (fastest 2619.2 ns, slowest 4280.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4470ns | 2842ns | 2312ns | 3836ns | 8529ns | base |
| warm-container-kernel | 3220ns | 3369ns | 1475ns | 3371ns | 4511ns | -27.97% |
| warm-container-lanes-deferred | 3385ns | 3740ns | 1080ns | 3618ns | 4994ns | -24.26% |
| warm-container-minimum | 4622ns | 4381ns | 2493ns | 4024ns | 8543ns | +3.40% |
| warm-container-native | 4302ns | 2704ns | 2292ns | 3460ns | 8836ns | -3.76% |
| warm-container-plusone | 4165ns | 2682ns | 2221ns | 3238ns | 8891ns | -6.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4361ns | 2261ns | 8289ns | base | 3.757 |
| warm-container-kernel | 3037ns | 1397ns | 4230ns | -30.36% | 5.395 |
| warm-container-lanes-deferred | 3200ns | 1023ns | 4702ns | -26.61% | 5.119 |
| warm-container-minimum | 4504ns | 2438ns | 8300ns | +3.28% | 3.638 |
| warm-container-native | 4186ns | 2238ns | 8553ns | -4.01% | 3.914 |
| warm-container-plusone | 4053ns | 2170ns | 8613ns | -7.05% | 4.042 |

## Performance model

- Peak throughput: **16.008 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.894 | 36.8% |
| warm-container-kernel | 5.128 | 32.0% |
| warm-container-lanes-deferred | 4.614 | 28.8% |
| warm-container-minimum | 3.828 | 23.9% |
| warm-container-native | 6.202 | 38.7% |
| warm-container-plusone | 6.255 | 39.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4470ns | 4470ns | base |
| warm-container-kernel | 3220ns | 3220ns | -27.97% |
| warm-container-lanes-deferred | 3385ns | 3385ns | -24.26% |
| warm-container-minimum | 4622ns | 4622ns | +3.40% |
| warm-container-native | 4302ns | 4302ns | -3.76% |
| warm-container-plusone | 4165ns | 4165ns | -6.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2780ns | base | --- | [2770, 5513] | --- | --- | --- | --- |
| warm-container-kernel | 3195ns | no significant difference | [-1960, +179]ns | [2966, 3428] | no | 0.3352 | 0.2682 | 0 |
| warm-container-lanes-deferred | 3551ns | no significant difference | [-1849, +92]ns | [3074, 4153] | no | 0.1345 | 0.0807 | 0 |
| warm-container-minimum | 4280ns | +548.3ns (+19.7%) | [+56, +794]ns | [2833, 4393] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-native | 2642ns | no significant difference | [-225, +344]ns | [2621, 2651] | no | 0.4296 | 0.4296 | 0 |
| warm-container-plusone | 2619ns | -168.5ns (-6.1%) | [-246, -101]ns | [2592, 2660] | YES | 0.0017 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2825ns | +9.2% | -48.3% | +0.4% | -23.1% | -22.7% |
| 1 | 2 | 3960ns | -11.1% | +2.3% | -41.2% | -33.1% | -32.8% |
| 1 | 3 | 2290ns | +30.2% | +40.6% | +47.0% | +15.3% | -4.5% |
| 1 | 4 | 2780ns | +8.8% | +49.4% | +57.8% | -5.0% | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.457 | moderate- |
| warm-container-kernel | -0.354 | moderate- |
| warm-container-lanes-deferred | -0.315 | moderate- |
| warm-container-minimum | 0.168 | ok |
| warm-container-native | -0.099 | ok |
| warm-container-plusone | -0.772 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.8ns | 4360.9ns | 0.9% |  |
| warm-container-kernel | 64.7ns | 3036.7ns | 2.1% |  |
| warm-container-lanes-deferred | 67.9ns | 3200.5ns | 2.1% |  |
| warm-container-minimum | 38.9ns | 4503.8ns | 0.9% |  |
| warm-container-native | 38.6ns | 4186.1ns | 0.9% |  |
| warm-container-plusone | 37.0ns | 4053.2ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2260.7-8288.6 ns)
   2260.7 |######################
   2562.1 |########################################
   2863.5 |
   3164.9 |
   3466.2 |
   3767.6 |
   4069.0 |
   4370.4 |
   4671.8 |
   4973.2 |
   5274.6 |##
   5576.0 |##
   5877.4 |
   6178.8 |#####
   6480.2 |##
   6781.6 |##
   7083.0 |#####
   7384.4 |##
   7685.8 |#####
   7987.2 |##
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1397.0-4229.8 ns)
   1397.0 |
   1538.7 |
   1680.3 |
   1822.0 |
   1963.6 |
   2105.2 |########
   2246.9 |
   2388.5 |
   2530.2 |
   2671.8 |########################################
   2813.4 |########
   2955.1 |########################################
   3096.7 |########################################
   3238.3 |################
   3380.0 |################################
   3521.6 |########
   3663.3 |########################
   3804.9 |########
   3946.5 |########
   4088.2 |################
  (6 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1023.5-4702.3 ns)
   1023.5 |####################
   1207.4 |
   1391.3 |
   1575.3 |
   1759.2 |
   1943.2 |#####
   2127.1 |#####
   2311.1 |
   2495.0 |
   2678.9 |
   2862.9 |###############
   3046.8 |##########
   3230.8 |##########
   3414.7 |###############
   3598.6 |###############
   3782.6 |#####
   3966.5 |
   4150.5 |########################################
   4334.4 |##########
   4518.4 |##########
  (5 below, 3 above range)

warm-container-minimum (n=40, range 2437.8-8300.0 ns)
   2437.8 |
   2730.9 |########################################
   3024.1 |
   3317.2 |
   3610.3 |###
   3903.4 |###
   4196.5 |##############################
   4489.6 |
   4782.7 |
   5075.8 |###
   5368.9 |
   5662.0 |###
   5955.1 |
   6248.3 |
   6541.4 |
   6834.5 |
   7127.6 |##########
   7420.7 |
   7713.8 |######
   8006.9 |
  (6 below, 4 above range)

warm-container-native (n=40, range 2238.3-8553.2 ns)
   2238.3 |##
   2554.1 |########################################
   2869.8 |##
   3185.6 |
   3501.3 |
   3817.1 |
   4132.8 |
   4448.5 |
   4764.3 |
   5080.0 |
   5395.8 |
   5711.5 |
   6027.3 |
   6343.0 |
   6658.7 |####
   6974.5 |##
   7290.2 |##
   7606.0 |##
   7921.7 |####
   8237.5 |##
  (6 below, 4 above range)

warm-container-plusone (n=40, range 2169.8-8612.6 ns)
   2169.8 |##################################
   2491.9 |########################################
   2814.1 |##
   3136.2 |
   3458.3 |
   3780.5 |
   4102.6 |
   4424.8 |
   4746.9 |
   5069.1 |##
   5391.2 |
   5713.3 |
   6035.5 |##
   6357.6 |
   6679.8 |##
   7001.9 |##
   7324.0 |##
   7646.2 |##
   7968.3 |#####
   8290.5 |
  (1 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.3% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=32.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=41.9% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=48.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-native**: CV=61.7% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=64.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
