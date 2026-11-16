# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 2.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 43% (significant)

warm-container-kernel is -1.77 us (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.71)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-kernel, warm-container-plusone, warm-container-minimum, warm-container-native} vs {warm-container-headroom} (49% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-kernel, warm-container-plusone, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom} with a 49% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-lanes-deferred vs stability leader warm-container-kernel (+4% speed for 5.8x steadier)

warm-container-lanes-deferred is fastest (2.19 us, CV 54.2%); warm-container-kernel gives up 4.3% median for 5.8x lower variance (CV 9.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.1x its best-20%

warm-container-plusone's best 20% of batches run at 1.88 us but its worst 20% at 7.69 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2193.2 ns median (-47.0% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.89x (fastest 2193.2 ns, slowest 4136.2 ns)

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
| warm-container-headroom | 4124ns | 4282ns | 1945ns | 3874ns | 7055ns | base |
| warm-container-kernel | 2532ns | 2511ns | 2195ns | 2523ns | 2896ns | -38.61% |
| warm-container-lanes-deferred | 2628ns | 2419ns | 2120ns | 2400ns | 3817ns | -36.29% |
| warm-container-minimum | 4024ns | 2723ns | 2216ns | 3309ns | 7978ns | -2.43% |
| warm-container-native | 4460ns | 2845ns | 2437ns | 3625ns | 8988ns | +8.14% |
| warm-container-plusone | 3730ns | 2378ns | 1945ns | 2895ns | 8018ns | -9.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3971ns | 1878ns | 6761ns | base | 4.126 |
| warm-container-kernel | 2342ns | 2039ns | 2653ns | -41.01% | 6.994 |
| warm-container-lanes-deferred | 2436ns | 1969ns | 3579ns | -38.66% | 6.727 |
| warm-container-minimum | 3917ns | 2163ns | 7734ns | -1.35% | 4.182 |
| warm-container-native | 4350ns | 2381ns | 8737ns | +9.53% | 3.767 |
| warm-container-plusone | 3590ns | 1878ns | 7686ns | -9.58% | 4.563 |

## Performance model

- Peak throughput: **8.724 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.961 | 45.4% |
| warm-container-kernel | 7.161 | 82.1% |
| warm-container-lanes-deferred | 7.470 | 85.6% |
| warm-container-minimum | 6.166 | 70.7% |
| warm-container-native | 5.890 | 67.5% |
| warm-container-plusone | 7.137 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4124ns | 4124ns | base |
| warm-container-kernel | 2532ns | 2532ns | -38.61% |
| warm-container-lanes-deferred | 2628ns | 2628ns | -36.29% |
| warm-container-minimum | 4024ns | 4024ns | -2.43% |
| warm-container-native | 4460ns | 4460ns | +8.14% |
| warm-container-plusone | 3730ns | 3730ns | -9.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4136ns | base | --- | [2296, 5256] | --- | --- | --- | --- |
| warm-container-kernel | 2288ns | -1769.5ns (-42.8%) | [-2818, -195]ns | [2266, 2455] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-lanes-deferred | 2193ns | -915.9ns (-22.1%) | [-3050, -54]ns | [2103, 2296] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-container-minimum | 2657ns | +314.0ns (+7.6%) | [+292, +360]ns | [2613, 2669] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-native | 2782ns | +804.2ns (+19.4%) | [+472, +895]ns | [2769, 2840] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-plusone | 2296ns | no significant difference | [-417, +417]ns | [2295, 2423] | no | 0.5224 | 0.5224 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4988ns | -52.8% | -57.3% | -46.6% | -44.2% | -53.9% |
| 1 | 2 | 1884ns | +30.3% | +37.3% | +15.4% | +47.2% | +22.1% |
| 1 | 3 | 4393ns | -44.1% | -52.1% | -22.6% | -48.4% | -32.9% |
| 1 | 4 | 2309ns | -8.9% | -8.9% | +14.6% | +22.2% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.715 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.153 | ok |
| warm-container-lanes-deferred | -0.382 | moderate- |
| warm-container-minimum | -0.510 | HIGH- (thermal bounce) |
| warm-container-native | -0.453 | moderate- |
| warm-container-plusone | -0.535 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 43.1ns | 3971.0ns | 1.1% |  |
| warm-container-kernel | 70.4ns | 2342.5ns | 3.0% |  |
| warm-container-lanes-deferred | 67.4ns | 2435.7ns | 2.8% |  |
| warm-container-minimum | 36.0ns | 3917.3ns | 0.9% |  |
| warm-container-native | 37.8ns | 4349.6ns | 0.9% |  |
| warm-container-plusone | 39.4ns | 3590.4ns | 1.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1878.2-6761.3 ns)
   1878.2 |########################################
   2122.4 |########################
   2366.5 |
   2610.7 |
   2854.8 |
   3099.0 |
   3343.1 |
   3587.3 |####
   3831.5 |
   4075.6 |
   4319.8 |
   4563.9 |########
   4808.1 |####
   5052.2 |############
   5296.4 |################
   5540.5 |####
   5784.7 |########
   6028.8 |########
   6273.0 |####
   6517.1 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 2038.7-2653.1 ns)
   2038.7 |
   2069.4 |
   2100.1 |####################
   2130.8 |
   2161.5 |
   2192.3 |####
   2223.0 |
   2253.7 |########################################
   2284.4 |####
   2315.1 |
   2345.9 |
   2376.6 |########
   2407.3 |####
   2438.0 |############################
   2468.7 |
   2499.5 |########
   2530.2 |
   2560.9 |########
   2591.6 |
   2622.3 |
  (4 below, 5 above range)

warm-container-lanes-deferred (n=40, range 1968.7-3578.7 ns)
   1968.7 |######################
   2049.2 |########################################
   2129.7 |####
   2210.2 |##########################
   2290.7 |########
   2371.2 |####
   2451.7 |######################
   2532.2 |
   2612.7 |########
   2693.2 |####
   2773.7 |
   2854.2 |
   2934.7 |########
   3015.2 |
   3095.7 |
   3176.2 |
   3256.7 |
   3337.2 |
   3417.7 |
   3498.2 |
  (5 below, 1 above range)

warm-container-minimum (n=40, range 2163.4-7734.0 ns)
   2163.4 |#############################
   2441.9 |########################################
   2720.4 |
   2998.9 |
   3277.5 |
   3556.0 |
   3834.5 |
   4113.1 |
   4391.6 |##
   4670.1 |
   4948.7 |
   5227.2 |
   5505.7 |##
   5784.2 |##
   6062.8 |##
   6341.3 |#####
   6619.8 |#####
   6898.4 |
   7176.9 |##
   7455.4 |
  (1 below, 4 above range)

warm-container-native (n=40, range 2380.8-8736.8 ns)
   2380.8 |
   2698.6 |########################################
   3016.4 |
   3334.2 |
   3652.0 |
   3969.8 |
   4287.6 |
   4605.4 |#
   4923.2 |
   5241.0 |
   5558.8 |
   5876.6 |
   6194.4 |###
   6512.2 |
   6830.0 |#
   7147.8 |
   7465.6 |#
   7783.4 |
   8101.2 |#####
   8419.0 |###
  (6 below, 3 above range)

warm-container-plusone (n=40, range 1878.1-7685.6 ns)
   1878.1 |##############################
   2168.5 |########################################
   2458.9 |###
   2749.3 |
   3039.6 |
   3330.0 |
   3620.4 |
   3910.7 |######
   4201.1 |
   4491.5 |
   4781.9 |
   5072.2 |
   5362.6 |###
   5653.0 |######
   5943.4 |
   6233.7 |###
   6524.1 |###
   6814.5 |
   7104.8 |###
   7395.2 |###
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=50.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=48.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=57.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=58.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=64.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
