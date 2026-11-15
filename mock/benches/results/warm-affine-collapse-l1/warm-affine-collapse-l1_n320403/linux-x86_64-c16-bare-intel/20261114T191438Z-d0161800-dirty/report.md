# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.71)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-headroom, warm-container-lanes-deferred, warm-container-minimum, warm-container-native} vs {warm-container-plusone} (43% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-headroom, warm-container-lanes-deferred, warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+6% speed for 1.2x steadier)

warm-container-kernel is fastest (3.05 us, CV 17.0%); warm-container-lanes-deferred gives up 5.7% median for 1.2x lower variance (CV 13.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.2x its best-20%

warm-container-plusone's best 20% of batches run at 1.61 us but its worst 20% at 6.84 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 3047.2 ns median (-2.6% vs baseline)
- Spread: 1.64x (fastest 3047.2 ns, slowest 5009.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3933ns | 3281ns | 1813ns | 3740ns | 6634ns | base |
| warm-container-kernel | 3231ns | 3279ns | 2412ns | 3296ns | 3854ns | -17.86% |
| warm-container-lanes-deferred | 3351ns | 3441ns | 2595ns | 3431ns | 3870ns | -14.80% |
| warm-container-minimum | 3557ns | 3502ns | 1748ns | 3516ns | 5489ns | -9.56% |
| warm-container-native | 3543ns | 3738ns | 2759ns | 3629ns | 4068ns | -9.94% |
| warm-container-plusone | 4755ns | 5251ns | 1692ns | 4957ns | 7212ns | +20.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3739ns | 1730ns | 6277ns | base | 8.763 |
| warm-container-kernel | 3025ns | 2266ns | 3579ns | -19.10% | 10.832 |
| warm-container-lanes-deferred | 3139ns | 2439ns | 3593ns | -16.05% | 10.439 |
| warm-container-minimum | 3325ns | 1643ns | 5111ns | -11.08% | 9.856 |
| warm-container-native | 3317ns | 2591ns | 3770ns | -11.28% | 9.878 |
| warm-container-plusone | 4526ns | 1614ns | 6836ns | +21.03% | 7.240 |

## Performance model

- Peak throughput: **20.300 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.468 | 51.6% |
| warm-container-kernel | 10.753 | 53.0% |
| warm-container-lanes-deferred | 10.178 | 50.1% |
| warm-container-minimum | 9.959 | 49.1% |
| warm-container-native | 9.339 | 46.0% |
| warm-container-plusone | 6.541 | 32.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3933ns | 3933ns | base |
| warm-container-kernel | 3231ns | 3231ns | -17.86% |
| warm-container-lanes-deferred | 3351ns | 3351ns | -14.80% |
| warm-container-minimum | 3557ns | 3557ns | -9.56% |
| warm-container-native | 3543ns | 3543ns | -9.94% |
| warm-container-plusone | 4755ns | 4755ns | +20.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3130ns | base | --- | [1743, 5449] | --- | --- | --- | --- |
| warm-container-kernel | 3047ns | no significant difference | [-1953, +771]ns | [2883, 3237] | no | 0.8746 | 0.6358 | 0 |
| warm-container-lanes-deferred | 3220ns | no significant difference | [-1955, +828]ns | [3009, 3384] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 3290ns | no significant difference | [-1154, +555]ns | [3027, 3543] | no | 0.8746 | 0.6358 | 0 |
| warm-container-native | 3509ns | no significant difference | [-1714, +1220]ns | [3252, 3545] | no | 0.8746 | 0.8746 | 0 |
| warm-container-plusone | 5010ns | no significant difference | [-121, +1501]ns | [4532, 5431] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2363ns | +9.1% | +16.9% | +36.3% | +40.2% | +54.6% |
| 1 | 2 | 4404ns | -31.6% | -24.9% | +2.2% | -19.9% | +14.7% |
| 1 | 3 | 2259ns | +49.5% | +48.2% | +37.1% | +55.4% | +93.9% |
| 1 | 4 | 4817ns | -30.3% | -29.6% | -26.7% | -28.2% | +29.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.706 | HIGH- (thermal bounce) |
| warm-container-kernel | 0.228 | moderate+ |
| warm-container-lanes-deferred | -0.015 | ok |
| warm-container-minimum | -0.623 | HIGH- (thermal bounce) |
| warm-container-native | -0.199 | ok |
| warm-container-plusone | -0.272 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 54.2ns | 3739.3ns | 1.4% |  |
| warm-container-kernel | 77.1ns | 3025.2ns | 2.5% |  |
| warm-container-lanes-deferred | 76.9ns | 3139.0ns | 2.4% |  |
| warm-container-minimum | 83.8ns | 3324.8ns | 2.5% |  |
| warm-container-native | 80.9ns | 3317.4ns | 2.4% |  |
| warm-container-plusone | 65.2ns | 4525.7ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1730.0-6277.5 ns)
   1730.0 |########################################
   1957.3 |
   2184.7 |
   2412.1 |
   2639.5 |##
   2866.8 |##
   3094.2 |##
   3321.6 |
   3549.0 |
   3776.3 |
   4003.7 |
   4231.1 |
   4458.5 |
   4685.9 |########
   4913.2 |
   5140.6 |##
   5368.0 |#####
   5595.4 |##############
   5822.7 |
   6050.1 |########
  (4 below, 5 above range)

warm-container-kernel (n=40, range 2266.4-3579.3 ns)
   2266.4 |
   2332.1 |##########
   2397.7 |
   2463.4 |
   2529.0 |####################
   2594.6 |
   2660.3 |
   2725.9 |####################
   2791.6 |##########
   2857.2 |
   2922.9 |###############
   2988.5 |###############
   3054.2 |#####
   3119.8 |#####
   3185.5 |#########################
   3251.1 |
   3316.8 |#####
   3382.4 |#####
   3448.0 |
   3513.7 |########################################
  (2 below, 3 above range)

warm-container-lanes-deferred (n=40, range 2439.0-3592.8 ns)
   2439.0 |
   2496.7 |##########
   2554.4 |#####
   2612.1 |
   2669.8 |
   2727.4 |###############
   2785.1 |
   2842.8 |
   2900.5 |
   2958.2 |##############################
   3015.9 |###############
   3073.6 |
   3131.3 |#####
   3189.0 |####################
   3246.7 |###############
   3304.4 |#####
   3362.0 |
   3419.7 |#####
   3477.4 |#####
   3535.1 |########################################
  (2 below, 4 above range)

warm-container-minimum (n=40, range 1642.8-5111.3 ns)
   1642.8 |
   1816.2 |######
   1989.6 |
   2163.1 |#############
   2336.5 |
   2509.9 |
   2683.3 |#############
   2856.8 |#################################
   3030.2 |####################
   3203.6 |#################################
   3377.0 |########################################
   3550.5 |####################
   3723.9 |######
   3897.3 |#############
   4070.7 |
   4244.2 |
   4417.6 |#############
   4591.0 |
   4764.4 |
   4937.9 |
  (4 below, 4 above range)

warm-container-native (n=40, range 2591.1-3769.8 ns)
   2591.1 |
   2650.0 |
   2709.0 |
   2767.9 |
   2826.8 |
   2885.8 |#####
   2944.7 |###############
   3003.6 |
   3062.6 |##########
   3121.5 |#####
   3180.4 |####################
   3239.4 |####################
   3298.3 |
   3357.2 |##########
   3416.2 |
   3475.1 |#########################
   3534.1 |########################################
   3593.0 |####################
   3651.9 |#####
   3710.9 |
  (2 below, 3 above range)

warm-container-plusone (n=40, range 1614.2-6836.3 ns)
   1614.2 |########################################
   1875.3 |
   2136.4 |#####
   2397.5 |
   2658.6 |
   2919.7 |
   3180.8 |
   3441.9 |
   3703.0 |
   3964.1 |#####
   4225.2 |###########
   4486.3 |###########
   4747.4 |######################
   5008.6 |######################
   5269.7 |#################
   5530.8 |#################
   5791.9 |#####
   6053.0 |############################
   6314.1 |#####
   6575.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=52.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=37.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-container-plusone**: CV=42.7% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.2x (possible bimodal distribution)
