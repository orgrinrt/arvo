# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-minimum, warm-container-headroom) are a dead heat (<1%)

warm-container-minimum (2.62 us) and warm-container-headroom (2.62 us) differ by 0.27%, inside the noise, even though the wider field spreads 59.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.63)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.56 us) is smaller than the fastest variant's own run-to-run std-dev (2.37 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-headroom, warm-container-native, warm-container-plusone} vs {warm-container-kernel, warm-container-lanes-deferred} (36% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-headroom, warm-container-native, warm-container-plusone} and a slow tier {warm-container-kernel, warm-container-lanes-deferred} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.23 us but its worst 20% at 5.11 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2615.8 ns median (-0.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.60x (fastest 2615.8 ns, slowest 4173.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4128ns | 2686ns | 2244ns | 3227ns | 8717ns | base |
| warm-container-kernel | 4008ns | 4068ns | 2107ns | 4092ns | 5657ns | -2.91% |
| warm-container-lanes-deferred | 3543ns | 4384ns | 1295ns | 3669ns | 5412ns | -14.18% |
| warm-container-minimum | 4373ns | 2678ns | 2202ns | 3827ns | 8185ns | +5.93% |
| warm-container-native | 4309ns | 2823ns | 2805ns | 3038ns | 9628ns | +4.38% |
| warm-container-plusone | 4722ns | 2901ns | 2841ns | 3747ns | 9529ns | +14.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4020ns | 2191ns | 8455ns | base | 4.076 |
| warm-container-kernel | 3811ns | 2006ns | 5379ns | -5.19% | 4.299 |
| warm-container-lanes-deferred | 3362ns | 1232ns | 5113ns | -16.36% | 4.873 |
| warm-container-minimum | 4259ns | 2150ns | 7939ns | +5.97% | 3.846 |
| warm-container-native | 4202ns | 2742ns | 9359ns | +4.55% | 3.899 |
| warm-container-plusone | 4607ns | 2779ns | 9264ns | +14.62% | 3.556 |

## Performance model

- Peak throughput: **13.296 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.247 | 47.0% |
| warm-container-kernel | 4.232 | 31.8% |
| warm-container-lanes-deferred | 3.926 | 29.5% |
| warm-container-minimum | 6.263 | 47.1% |
| warm-container-native | 5.935 | 44.6% |
| warm-container-plusone | 5.773 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4128ns | 4128ns | base |
| warm-container-kernel | 4008ns | 4008ns | -2.91% |
| warm-container-lanes-deferred | 3543ns | 3543ns | -14.18% |
| warm-container-minimum | 4373ns | 4373ns | +5.93% |
| warm-container-native | 4309ns | 4309ns | +4.38% |
| warm-container-plusone | 4722ns | 4722ns | +14.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2623ns | base | --- | [2599, 2643] | --- | --- | --- | --- |
| warm-container-kernel | 3872ns | no significant difference | [-820, +1225]ns | [3618, 4117] | no | 0.3352 | 0.2682 | 0 |
| warm-container-lanes-deferred | 4173ns | no significant difference | [-1382, +703]ns | [2485, 4457] | no | 0.2564 | 0.1539 | 0 |
| warm-container-minimum | 2616ns | no significant difference | [-26, +13]ns | [2596, 5351] | no | 0.6358 | 0.6358 | 0 |
| warm-container-native | 2761ns | +161.3ns (+6.2%) | [+137, +379]ns | [2755, 2770] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2838ns | +246.6ns (+9.4%) | [+231, +619]ns | [2835, 3391] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2618ns | +22.7% | +66.1% | -0.8% | +5.4% | +43.9% |
| 1 | 2 | 2629ns | +62.2% | -46.6% | +30.0% | +4.9% | +8.0% |
| 1 | 3 | 2143ns | +67.5% | +85.1% | +111.7% | +29.1% | +29.4% |
| 1 | 4 | 2621ns | +49.2% | +70.3% | +0.1% | +5.1% | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.425 | moderate- |
| warm-container-kernel | -0.633 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.357 | moderate- |
| warm-container-minimum | -0.306 | moderate- |
| warm-container-native | -0.600 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.047 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 2/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.2ns | 4019.5ns | 0.9% |  |
| warm-container-kernel | 72.2ns | 3810.8ns | 1.9% |  |
| warm-container-lanes-deferred | 66.9ns | 3361.9ns | 2.0% |  |
| warm-container-minimum | 39.4ns | 4259.5ns | 0.9% |  |
| warm-container-native | 36.8ns | 4202.4ns | 0.9% |  |
| warm-container-plusone | 40.0ns | 4607.3ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2190.9-8455.4 ns)
   2190.9 |
   2504.1 |########################################
   2817.4 |#
   3130.6 |
   3443.8 |
   3757.1 |
   4070.3 |
   4383.5 |
   4696.7 |
   5010.0 |
   5323.2 |
   5636.4 |
   5949.6 |
   6262.9 |#
   6576.1 |#
   6889.3 |#
   7202.5 |#
   7515.8 |
   7829.0 |#####
   8142.2 |
  (7 below, 4 above range)

warm-container-kernel (n=40, range 2005.7-5379.0 ns)
   2005.7 |
   2174.4 |#####
   2343.1 |#####
   2511.7 |
   2680.4 |
   2849.1 |
   3017.7 |#####
   3186.4 |#####
   3355.1 |############################
   3523.7 |#####
   3692.4 |##################################
   3861.1 |#####
   4029.7 |########################################
   4198.4 |#####
   4367.0 |########################################
   4535.7 |#####
   4704.4 |###########
   4873.0 |
   5041.7 |
   5210.4 |
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1232.2-5113.1 ns)
   1232.2 |###################################
   1426.3 |#####
   1620.3 |
   1814.4 |#####
   2008.4 |
   2202.4 |
   2396.5 |
   2590.5 |
   2784.6 |
   2978.6 |##########
   3172.7 |
   3366.7 |
   3560.7 |#####
   3754.8 |#####
   3948.8 |##########
   4142.9 |##########
   4336.9 |########################################
   4530.9 |###############
   4725.0 |
   4919.0 |####################
  (5 below, 3 above range)

warm-container-minimum (n=40, range 2150.0-7938.9 ns)
   2150.0 |###########
   2439.5 |########################################
   2728.9 |
   3018.4 |##
   3307.8 |
   3597.2 |
   3886.7 |
   4176.1 |
   4465.6 |#####
   4755.0 |
   5044.5 |
   5333.9 |
   5623.4 |
   5912.8 |##
   6202.3 |#####
   6491.7 |#####
   6781.1 |#####
   7070.6 |#####
   7360.0 |##
   7649.5 |##
  (5 below, 3 above range)

warm-container-native (n=40, range 2741.9-9359.2 ns)
   2741.9 |########################################
   3072.8 |
   3403.7 |#
   3734.5 |
   4065.4 |
   4396.2 |
   4727.1 |
   5058.0 |
   5388.8 |
   5719.7 |
   6050.6 |
   6381.4 |
   6712.3 |#
   7043.1 |
   7374.0 |
   7704.9 |#
   8035.7 |
   8366.6 |
   8697.5 |#
   9028.3 |###
  (4 below, 4 above range)

warm-container-plusone (n=40, range 2778.5-9263.6 ns)
   2778.5 |########################################
   3102.8 |
   3427.1 |
   3751.3 |##
   4075.6 |
   4399.8 |##
   4724.1 |
   5048.3 |
   5372.6 |
   5696.8 |##
   6021.1 |
   6345.3 |
   6669.6 |
   6993.8 |##
   7318.1 |##
   7642.3 |##
   7966.6 |##
   8290.8 |##
   8615.1 |##
   8939.3 |####
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=39.8% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: CV=47.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=55.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=63.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-plusone**: CV=58.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.3x (possible bimodal distribution)
