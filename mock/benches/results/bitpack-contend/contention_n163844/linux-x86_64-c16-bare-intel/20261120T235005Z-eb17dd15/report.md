# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16 dominates: 64% faster than the next best (bitpack-contend-d16-control)

bitpack-contend-d16 (1.40 us) leads bitpack-contend-d16-control (2.29 us) by 64%, a clear separation rather than a photo finish. CV 100.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d64 beats baseline by 30% (significant)

bitpack-contend-d64 is -425 ns (30%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-packed-simd is an outlier: 2.1x slower than the field

bitpack-contend-packed-simd (2.96 us) is 2.1x the fastest (1.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16 is fastest but the noisiest (CV 100.2%)

bitpack-contend-d16 wins on median (1.40 us) yet has the highest variance (CV 100.2%), while bitpack-contend-packed-simd is the steadiest (CV 31.7%, 2.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16-control shows alternating (throttle bounce) (autocorr -0.69)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-contend-d16)

The baseline bitpack-contend-d16 is the fastest (1.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-contend-d16} vs {bitpack-contend-d16-control, bitpack-contend-d64, bitpack-contend-d32, bitpack-contend-packed, bitpack-contend-packed-simd} (64% apart)

The field splits into a fast tier {bitpack-contend-d16} and a slow tier {bitpack-contend-d16-control, bitpack-contend-d64, bitpack-contend-d32, bitpack-contend-packed, bitpack-contend-packed-simd} with a 64% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-d32 is inconsistent: worst-20% is 4.1x its best-20%

bitpack-contend-d32's best 20% of batches run at 1.05 us but its worst 20% at 4.27 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-contend-d16) is the fastest** at 1399.8 ns median
- 2 variants significantly faster than baseline
- Spread: 2.12x (fastest 1399.8 ns, slowest 2961.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 2634ns | 1463ns | 1265ns | 2375ns | 4777ns | base |
| bitpack-contend-d16-control | 2799ns | 2402ns | 1291ns | 2606ns | 4884ns | +6.27% |
| bitpack-contend-d32 | 2700ns | 2762ns | 1119ns | 2604ns | 4568ns | +2.50% |
| bitpack-contend-d64 | 2345ns | 2719ns | 1011ns | 2250ns | 3962ns | -10.97% |
| bitpack-contend-packed | 2649ns | 2880ns | 1077ns | 2647ns | 4228ns | +0.60% |
| bitpack-contend-packed-simd | 2815ns | 3139ns | 1096ns | 3032ns | 3884ns | +6.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 2501ns | 1206ns | 4510ns | base | 6.551 |
| bitpack-contend-d16-control | 2660ns | 1231ns | 4646ns | +6.37% | 6.158 |
| bitpack-contend-d32 | 2536ns | 1055ns | 4273ns | +1.38% | 6.462 |
| bitpack-contend-d64 | 2149ns | 946ns | 3615ns | -14.09% | 7.625 |
| bitpack-contend-packed | 2486ns | 1017ns | 3944ns | -0.60% | 6.590 |
| bitpack-contend-packed-simd | 2648ns | 1035ns | 3663ns | +5.87% | 6.188 |

## Performance model

- Peak throughput: **17.323 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.705 | 67.6% |
| bitpack-contend-d16-control | 7.143 | 41.2% |
| bitpack-contend-d32 | 6.301 | 36.4% |
| bitpack-contend-d64 | 6.638 | 38.3% |
| bitpack-contend-packed | 6.042 | 34.9% |
| bitpack-contend-packed-simd | 5.533 | 31.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 2634ns | 2634ns | base |
| bitpack-contend-d16-control | 2799ns | 2799ns | +6.27% |
| bitpack-contend-d32 | 2700ns | 2700ns | +2.50% |
| bitpack-contend-d64 | 2345ns | 2345ns | -10.97% |
| bitpack-contend-packed | 2649ns | 2649ns | +0.60% |
| bitpack-contend-packed-simd | 2815ns | 2815ns | +6.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1400ns | base | --- | [1287, 3354] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 2294ns | no significant difference | [-59, +148]ns | [1276, 3717] | no | 0.1923 | 0.1539 | 0 |
| bitpack-contend-d32 | 2600ns | no significant difference | [-144, +629]ns | [2400, 2721] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d64 | 2468ns | -425.2ns (-30.4%) | [-587, -226]ns | [1028, 2841] | YES | 0.0111 | 0.0022 | 0 |
| bitpack-contend-packed | 2712ns | no significant difference | [-529, +226]ns | [1405, 3430] | no | 0.1923 | 0.1539 | 0 |
| bitpack-contend-packed-simd | 2961ns | -216.3ns (-15.5%) | [-379, -9]ns | [2850, 3116] | YES (adj: no) | 0.1923 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2719ns | +0.4% | -6.7% | -65.1% | -0.3% | +9.5% |
| 1 | 2 | 1209ns | +1.9% | +42.5% | +155.6% | +153.5% | +135.7% |
| 1 | 3 | 1239ns | +179.6% | +119.5% | +42.9% | +177.9% | +131.8% |
| 1 | 4 | 3249ns | -31.7% | -67.4% | -18.2% | -68.1% | -8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.237 | moderate- |
| bitpack-contend-d16-control | -0.690 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.582 | HIGH- (thermal bounce) |
| bitpack-contend-d64 | -0.610 | HIGH- (thermal bounce) |
| bitpack-contend-packed | -0.245 | moderate- |
| bitpack-contend-packed-simd | -0.267 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 2/4, lost 2/4
- **bitpack-contend-d64**: won 2/4, lost 2/4
- **bitpack-contend-packed**: won 2/4, lost 2/4
- **bitpack-contend-packed-simd**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 46.4ns | 2501.0ns | 1.9% |  |
| bitpack-contend-d16-control | 46.9ns | 2660.4ns | 1.8% |  |
| bitpack-contend-d32 | 55.8ns | 2535.6ns | 2.2% |  |
| bitpack-contend-d64 | 63.4ns | 2148.6ns | 3.0% |  |
| bitpack-contend-packed | 56.1ns | 2486.0ns | 2.3% |  |
| bitpack-contend-packed-simd | 58.4ns | 2647.8ns | 2.2% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1206.2-4510.4 ns)
   1206.2 |########################################
   1371.4 |#############
   1536.6 |
   1701.8 |###
   1867.0 |
   2032.2 |
   2197.5 |
   2362.7 |
   2527.9 |
   2693.1 |
   2858.3 |
   3023.5 |
   3188.7 |#############
   3353.9 |######
   3519.1 |###
   3684.4 |
   3849.6 |###
   4014.8 |################
   4180.0 |###
   4345.2 |###
  (5 below, 3 above range)

bitpack-contend-d16-control (n=40, range 1231.1-4646.0 ns)
   1231.1 |########################################
   1401.9 |
   1572.6 |##
   1743.4 |
   1914.1 |
   2084.9 |
   2255.6 |
   2426.4 |
   2597.1 |
   2767.8 |
   2938.6 |#####
   3109.3 |
   3280.1 |##
   3450.8 |#####
   3621.6 |#####
   3792.3 |#####
   3963.1 |########
   4133.8 |########
   4304.5 |
   4475.3 |########
  (5 below, 2 above range)

bitpack-contend-d32 (n=40, range 1054.6-4272.6 ns)
   1054.6 |########################################
   1215.5 |
   1376.4 |
   1537.3 |
   1698.2 |
   1859.1 |######
   2020.0 |######
   2180.9 |
   2341.8 |#################################
   2502.7 |##########################
   2663.6 |########################################
   2824.5 |####################
   2985.4 |
   3146.3 |
   3307.2 |
   3468.1 |
   3629.0 |
   3789.9 |####################
   3950.8 |
   4111.7 |######
  (5 below, 5 above range)

bitpack-contend-d64 (n=40, range 945.8-3614.6 ns)
    945.8 |########################################
   1079.2 |
   1212.7 |
   1346.1 |##
   1479.6 |
   1613.0 |##
   1746.4 |
   1879.9 |##
   2013.3 |
   2146.8 |
   2280.2 |##
   2413.6 |##
   2547.1 |#####
   2680.5 |##
   2814.0 |###########
   2947.4 |########
   3080.8 |##############
   3214.3 |##
   3347.7 |
   3481.2 |##
  (2 below, 2 above range)

bitpack-contend-packed (n=40, range 1016.8-3943.8 ns)
   1016.8 |########################################
   1163.2 |####
   1309.5 |
   1455.9 |####
   1602.2 |
   1748.6 |
   1894.9 |
   2041.3 |
   2187.6 |
   2334.0 |
   2480.3 |#############
   2626.7 |#################
   2773.0 |####
   2919.4 |########
   3065.7 |
   3212.1 |
   3358.4 |###############################
   3504.8 |#################
   3651.1 |
   3797.5 |####
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 1034.7-3662.6 ns)
   1034.7 |###############
   1166.1 |
   1297.5 |#####
   1428.8 |
   1560.2 |
   1691.7 |#####
   1823.0 |
   1954.4 |
   2085.8 |#####
   2217.2 |
   2348.6 |
   2480.0 |
   2611.4 |#####
   2742.8 |####################
   2874.2 |########################################
   3005.6 |####################
   3137.0 |###############
   3268.4 |##########
   3399.8 |#####
   3531.2 |##########
  (5 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=56.1% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=55.1% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=44.8% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=51.8% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=47.8% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=35.5% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.5x (possible bimodal distribution)
