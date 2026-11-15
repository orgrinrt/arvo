# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 43% (significant)

warm-container-lanes-deferred is -996 ns (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-kernel, warm-container-headroom) are a dead heat (<1%)

warm-container-kernel (2.30 us) and warm-container-headroom (2.31 us) differ by 0.29%, inside the noise, even though the wider field spreads 23.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum is inconsistent: worst-20% is 3.7x its best-20%

warm-container-minimum's best 20% of batches run at 2.44 us but its worst 20% at 8.97 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 2300.8 ns median (-0.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.23x (fastest 2300.8 ns, slowest 2837.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4288ns | 2389ns | 2055ns | 3936ns | 7576ns | base |
| warm-container-kernel | 2557ns | 2476ns | 2195ns | 2520ns | 3032ns | -40.36% |
| warm-container-lanes-deferred | 2760ns | 2614ns | 1849ns | 2669ns | 3941ns | -35.64% |
| warm-container-minimum | 4521ns | 2900ns | 2494ns | 3625ns | 9235ns | +5.44% |
| warm-container-native | 4477ns | 2727ns | 2422ns | 3730ns | 8771ns | +4.41% |
| warm-container-plusone | 4008ns | 2391ns | 2217ns | 3402ns | 7616ns | -6.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4129ns | 1984ns | 7259ns | base | 3.968 |
| warm-container-kernel | 2351ns | 2037ns | 2720ns | -43.07% | 6.970 |
| warm-container-lanes-deferred | 2567ns | 1726ns | 3660ns | -37.83% | 6.383 |
| warm-container-minimum | 4410ns | 2440ns | 8972ns | +6.81% | 3.715 |
| warm-container-native | 4361ns | 2365ns | 8523ns | +5.63% | 3.757 |
| warm-container-plusone | 3856ns | 2140ns | 7290ns | -6.62% | 4.249 |

## Performance model

- Peak throughput: **9.493 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.101 | 74.8% |
| warm-container-kernel | 7.121 | 75.0% |
| warm-container-lanes-deferred | 6.706 | 70.6% |
| warm-container-minimum | 5.775 | 60.8% |
| warm-container-native | 6.150 | 64.8% |
| warm-container-plusone | 7.100 | 74.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4288ns | 4288ns | base |
| warm-container-kernel | 2557ns | 2557ns | -40.36% |
| warm-container-lanes-deferred | 2760ns | 2760ns | -35.64% |
| warm-container-minimum | 4521ns | 4521ns | +5.44% |
| warm-container-native | 4477ns | 4477ns | +4.41% |
| warm-container-plusone | 4008ns | 4008ns | -6.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2307ns | base | --- | [2296, 6053] | --- | --- | --- | --- |
| warm-container-kernel | 2301ns | -315.2ns (-13.7%) | [-3684, -147]ns | [2268, 2457] | YES | 0.0056 | 0.0034 | 1 |
| warm-container-lanes-deferred | 2443ns | -996.2ns (-43.2%) | [-3095, -47]ns | [2327, 2643] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-minimum | 2837ns | +498.7ns (+21.6%) | [+194, +946]ns | [2814, 2925] | YES | 0.0002 | 0.0000 | 0 |
| warm-container-native | 2664ns | +318.2ns (+13.8%) | [+292, +366]ns | [2619, 4731] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-plusone | 2308ns | no significant difference | [-6, +0]ns | [2296, 4903] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1884ns | +29.6% | +25.6% | +50.7% | +133.7% | +176.0% |
| 1 | 2 | 3294ns | -21.7% | +4.9% | -15.0% | -20.6% | -30.3% |
| 1 | 3 | 2306ns | -10.2% | +3.4% | +1.1% | +15.2% | -0.2% |
| 1 | 4 | 5060ns | -55.2% | -52.3% | -43.9% | -43.8% | -5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.322 | moderate- |
| warm-container-kernel | -0.145 | ok |
| warm-container-lanes-deferred | -0.445 | moderate- |
| warm-container-minimum | -0.402 | moderate- |
| warm-container-native | -0.127 | ok |
| warm-container-plusone | -0.247 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 43.4ns | 4128.7ns | 1.1% |  |
| warm-container-kernel | 71.6ns | 2350.7ns | 3.0% |  |
| warm-container-lanes-deferred | 72.8ns | 2567.0ns | 2.8% |  |
| warm-container-minimum | 37.7ns | 4410.1ns | 0.9% |  |
| warm-container-native | 39.9ns | 4361.0ns | 0.9% |  |
| warm-container-plusone | 40.4ns | 3855.6ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1983.9-7259.1 ns)
   1983.9 |
   2247.7 |########################################
   2511.4 |##
   2775.2 |
   3039.0 |
   3302.7 |
   3566.5 |
   3830.2 |
   4094.0 |#####
   4357.8 |
   4621.5 |
   4885.3 |
   5149.0 |
   5412.8 |
   5676.6 |##
   5940.3 |#####
   6204.1 |########
   6467.9 |#####
   6731.6 |########
   6995.4 |#####
  (6 below, 3 above range)

warm-container-kernel (n=40, range 2037.2-2720.2 ns)
   2037.2 |######
   2071.4 |##########################
   2105.5 |######
   2139.7 |
   2173.8 |#############
   2208.0 |
   2242.1 |########################################
   2276.3 |##########################
   2310.4 |######
   2344.6 |
   2378.7 |
   2412.9 |#############
   2447.0 |#################################
   2481.2 |######
   2515.3 |
   2549.5 |######
   2583.6 |#############
   2617.8 |
   2651.9 |##########################
   2686.1 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1726.0-3660.2 ns)
   1726.0 |
   1822.7 |
   1919.4 |
   2016.1 |
   2112.8 |##########################
   2209.5 |########################################
   2306.2 |########################################
   2403.0 |####################
   2499.7 |#############
   2596.4 |#################################
   2693.1 |#############
   2789.8 |######
   2886.5 |######
   2983.2 |######
   3079.9 |
   3176.6 |
   3273.3 |######
   3370.1 |
   3466.8 |
   3563.5 |######
  (3 below, 4 above range)

warm-container-minimum (n=40, range 2439.6-8972.0 ns)
   2439.6 |
   2766.2 |########################################
   3092.9 |
   3419.5 |
   3746.1 |
   4072.7 |
   4399.3 |###
   4725.9 |
   5052.6 |
   5379.2 |
   5705.8 |
   6032.4 |
   6359.0 |#
   6685.7 |
   7012.3 |
   7338.9 |
   7665.5 |###
   7992.1 |#
   8318.8 |#
   8645.4 |#######
  (6 below, 2 above range)

warm-container-native (n=40, range 2365.1-8523.4 ns)
   2365.1 |########################################
   2673.0 |####
   2980.9 |##
   3288.8 |##
   3596.7 |
   3904.6 |
   4212.6 |
   4520.5 |
   4828.4 |
   5136.3 |
   5444.2 |
   5752.1 |##
   6060.0 |######
   6368.0 |
   6675.9 |##
   6983.8 |##
   7291.7 |
   7599.6 |####
   7907.5 |##
   8215.5 |##
  (4 below, 4 above range)

warm-container-plusone (n=40, range 2139.8-7290.3 ns)
   2139.8 |########################################
   2397.4 |##
   2654.9 |
   2912.4 |
   3169.9 |##
   3427.5 |
   3685.0 |
   3942.5 |
   4200.0 |
   4457.5 |##
   4715.1 |
   4972.6 |##
   5230.1 |##
   5487.6 |####
   5745.2 |##
   6002.7 |####
   6260.2 |####
   6517.7 |##
   6775.2 |
   7032.8 |##
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=54.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=28.3% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=59.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=57.2% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=56.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.4x (possible bimodal distribution)
