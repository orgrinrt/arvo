# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum is fastest but the noisiest (CV 113.7%)

warm-container-minimum wins on median (2.33 us) yet has the highest variance (CV 113.7%), while warm-container-lanes-deferred is the steadiest (CV 23.3%, 3.69 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.70)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.37 us) is smaller than the fastest variant's own run-to-run std-dev (2.65 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-plusone, warm-container-headroom, warm-container-native} vs {warm-container-kernel, warm-container-lanes-deferred} (26% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-plusone, warm-container-headroom, warm-container-native} and a slow tier {warm-container-kernel, warm-container-lanes-deferred} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-headroom is inconsistent: worst-20% is 4.1x its best-20%

warm-container-headroom's best 20% of batches run at 2.28 us but its worst 20% at 9.34 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2327.8 ns median (-15.9% vs baseline)
- Spread: 1.59x (fastest 2327.8 ns, slowest 3694.9 ns)

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
| Counter frequency | 2112763739 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4328ns | 2831ns | 2331ns | 3233ns | 9612ns | base |
| warm-container-kernel | 3499ns | 3722ns | 1390ns | 3712ns | 4973ns | -19.15% |
| warm-container-lanes-deferred | 3612ns | 3881ns | 2132ns | 3776ns | 4600ns | -16.55% |
| warm-container-minimum | 4205ns | 2379ns | 2360ns | 3203ns | 9057ns | -2.85% |
| warm-container-native | 4045ns | 2869ns | 2324ns | 2900ns | 9203ns | -6.54% |
| warm-container-plusone | 3996ns | 2690ns | 2658ns | 2993ns | 8345ns | -7.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4220ns | 2279ns | 9339ns | base | 3.882 |
| warm-container-kernel | 3320ns | 1322ns | 4708ns | -21.33% | 4.935 |
| warm-container-lanes-deferred | 3431ns | 2034ns | 4347ns | -18.70% | 4.775 |
| warm-container-minimum | 4102ns | 2308ns | 8808ns | -2.79% | 3.994 |
| warm-container-native | 3944ns | 2272ns | 8942ns | -6.55% | 4.154 |
| warm-container-plusone | 3892ns | 2595ns | 8111ns | -7.79% | 4.210 |

## Performance model

- Peak throughput: **12.390 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.919 | 47.8% |
| warm-container-kernel | 4.618 | 37.3% |
| warm-container-lanes-deferred | 4.434 | 35.8% |
| warm-container-minimum | 7.038 | 56.8% |
| warm-container-native | 5.840 | 47.1% |
| warm-container-plusone | 6.237 | 50.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4328ns | 4328ns | base |
| warm-container-kernel | 3499ns | 3499ns | -19.15% |
| warm-container-lanes-deferred | 3612ns | 3612ns | -16.55% |
| warm-container-minimum | 4205ns | 4205ns | -2.85% |
| warm-container-native | 4045ns | 4045ns | -6.54% |
| warm-container-plusone | 3996ns | 3996ns | -7.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2768ns | base | --- | [2321, 2842] | --- | --- | --- | --- |
| warm-container-kernel | 3548ns | no significant difference | [-1029, +540]ns | [3317, 3781] | no | 1.0000 | 1.0000 | 0 |
| warm-container-lanes-deferred | 3695ns | no significant difference | [-644, +814]ns | [3350, 3774] | no | 0.7160 | 0.4296 | 0 |
| warm-container-minimum | 2328ns | no significant difference | [-368, +15]ns | [2318, 2776] | no | 1.0000 | 1.0000 | 0 |
| warm-container-native | 2806ns | no significant difference | [-36, +71]ns | [2776, 2829] | no | 0.7160 | 0.4296 | 0 |
| warm-container-plusone | 2627ns | no significant difference | [-222, +303]ns | [2607, 2647] | no | 0.7160 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2309ns | -41.9% | +64.0% | +0.3% | -1.6% | +13.8% |
| 1 | 2 | 2768ns | +21.7% | +43.8% | +0.3% | +2.0% | -5.0% |
| 1 | 3 | 2310ns | +75.2% | +45.0% | +0.5% | +22.6% | +13.9% |
| 1 | 4 | 2842ns | +24.8% | +24.4% | -18.3% | -2.2% | -8.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.704 | HIGH- (thermal bounce) |
| warm-container-kernel | 0.055 | ok |
| warm-container-lanes-deferred | -0.090 | ok |
| warm-container-minimum | -0.427 | moderate- |
| warm-container-native | -0.095 | ok |
| warm-container-plusone | -0.196 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 35.4ns | 4220.2ns | 0.8% |  |
| warm-container-kernel | 64.4ns | 3320.2ns | 1.9% |  |
| warm-container-lanes-deferred | 66.2ns | 3431.1ns | 1.9% |  |
| warm-container-minimum | 34.2ns | 4102.3ns | 0.8% |  |
| warm-container-native | 33.1ns | 3943.9ns | 0.8% |  |
| warm-container-plusone | 34.8ns | 3891.6ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2279.0-9339.2 ns)
   2279.0 |####################################
   2632.0 |########################################
   2985.1 |###
   3338.1 |
   3691.1 |
   4044.1 |###
   4397.1 |
   4750.1 |
   5103.1 |###
   5456.1 |###
   5809.1 |
   6162.1 |
   6515.1 |
   6868.1 |
   7221.1 |
   7574.1 |###
   7927.2 |
   8280.2 |######
   8633.2 |###
   8986.2 |###
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1322.3-4708.2 ns)
   1322.3 |######
   1491.6 |
   1660.9 |
   1830.2 |######
   1999.5 |
   2168.8 |######
   2338.1 |
   2507.4 |
   2676.7 |
   2846.0 |#############
   3015.2 |
   3184.5 |##########################
   3353.8 |##########################
   3523.1 |####################
   3692.4 |########################################
   3861.7 |######
   4031.0 |##########################
   4200.3 |
   4369.6 |##########################
   4538.9 |
  (6 below, 3 above range)

warm-container-lanes-deferred (n=40, range 2034.2-4347.4 ns)
   2034.2 |
   2149.8 |
   2265.5 |
   2381.1 |
   2496.8 |###########
   2612.5 |
   2728.1 |#####
   2843.8 |#####
   2959.5 |#####
   3075.1 |#####
   3190.8 |######################
   3306.4 |
   3422.1 |######################
   3537.8 |#####
   3653.4 |########################################
   3769.1 |###########
   3884.8 |#####
   4000.4 |##################################
   4116.1 |
   4231.7 |
  (4 below, 5 above range)

warm-container-minimum (n=40, range 2308.5-8808.2 ns)
   2308.5 |########################################
   2633.5 |##############
   2958.5 |
   3283.4 |
   3608.4 |##
   3933.4 |
   4258.4 |
   4583.4 |##
   4908.4 |
   5233.4 |
   5558.4 |
   5883.3 |####
   6208.3 |
   6533.3 |
   6858.3 |
   7183.3 |
   7508.3 |##
   7833.3 |####
   8158.3 |##
   8483.2 |##
  (4 below, 4 above range)

warm-container-native (n=40, range 2272.0-8941.8 ns)
   2272.0 |#######
   2605.5 |########################################
   2938.9 |
   3272.4 |
   3605.9 |#
   3939.4 |
   4272.9 |
   4606.4 |
   4939.9 |
   5273.4 |
   5606.9 |
   5940.4 |#
   6273.9 |#
   6607.4 |
   6940.9 |
   7274.4 |
   7607.8 |
   7941.3 |
   8274.8 |
   8608.3 |
  (5 below, 6 above range)

warm-container-plusone (n=40, range 2595.4-8110.9 ns)
   2595.4 |########################################
   2871.1 |#
   3146.9 |
   3422.7 |
   3698.5 |
   3974.2 |
   4250.0 |
   4525.8 |
   4801.6 |
   5077.3 |
   5353.1 |#
   5628.9 |
   5904.7 |
   6180.5 |#
   6456.2 |
   6732.0 |
   7007.8 |
   7283.6 |#
   7559.3 |#
   7835.1 |####
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=66.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-kernel**: CV=35.1% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=25.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=64.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=66.7% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=57.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.1x (possible bimodal distribution)
