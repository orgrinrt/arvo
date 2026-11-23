# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 53% faster than the next best (warm-container-headroom)

warm-container-native (1.22 us) leads warm-container-headroom (1.87 us) by 53%, a clear separation rather than a photo finish. CV 14.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 40% (significant)

warm-container-native is -754 ns (40%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.5x slower than the field

warm-container-plusone (4.26 us) is 3.5x the fastest (1.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.80)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.5x the fastest

Fastest warm-container-native (1.22 us) to slowest warm-container-plusone (4.26 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.37 us but its worst 20% at 5.59 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-26 ns, 1.40%)

warm-container-minimum differs from baseline warm-container-headroom by -26 ns (1.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1221.4 ns median (-34.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.49x (fastest 1221.4 ns, slowest 4260.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 13 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112823892 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3805ns | 1930ns | 1725ns | 3452ns | 6940ns | base |
| warm-container-kernel | 3746ns | 4255ns | 1610ns | 3683ns | 6072ns | -1.53% |
| warm-container-lanes-deferred | 3589ns | 4244ns | 1426ns | 3557ns | 5848ns | -5.67% |
| warm-container-minimum | 3488ns | 3710ns | 1543ns | 3277ns | 6065ns | -8.33% |
| warm-container-native | 1532ns | 1472ns | 1321ns | 1478ns | 1908ns | -59.72% |
| warm-container-plusone | 4169ns | 4409ns | 1884ns | 3976ns | 7030ns | +9.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3666ns | 1668ns | 6657ns | base | 4.469 |
| warm-container-kernel | 3594ns | 1549ns | 5805ns | -1.96% | 4.558 |
| warm-container-lanes-deferred | 3443ns | 1369ns | 5590ns | -6.09% | 4.759 |
| warm-container-minimum | 3359ns | 1491ns | 5812ns | -8.38% | 4.878 |
| warm-container-native | 1263ns | 1096ns | 1555ns | -65.54% | 12.968 |
| warm-container-plusone | 4014ns | 1821ns | 6740ns | +9.49% | 4.082 |

## Performance model

- Peak throughput: **14.956 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.780 | 58.7% |
| warm-container-kernel | 4.002 | 26.8% |
| warm-container-lanes-deferred | 4.016 | 26.9% |
| warm-container-minimum | 4.569 | 30.5% |
| warm-container-native | 13.414 | 89.7% |
| warm-container-plusone | 3.846 | 25.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3805ns | 3805ns | base |
| warm-container-kernel | 3746ns | 3746ns | -1.53% |
| warm-container-lanes-deferred | 3589ns | 3589ns | -5.67% |
| warm-container-minimum | 3488ns | 3488ns | -8.33% |
| warm-container-native | 1532ns | 1532ns | -59.72% |
| warm-container-plusone | 4169ns | 4169ns | +9.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1866ns | base | --- | [1836, 5403] | --- | --- | --- | --- |
| warm-container-kernel | 4094ns | no significant difference | [-262, +58]ns | [1656, 4935] | no | 0.3352 | 0.2682 | 0 |
| warm-container-lanes-deferred | 4080ns | -272.6ns (-14.6%) | [-543, -249]ns | [1598, 4664] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-minimum | 3586ns | -26.1ns (-1.4%) | [-345, -9]ns | [1821, 4115] | YES | 0.0037 | 0.0022 | 0 |
| warm-container-native | 1221ns | -753.6ns (-40.4%) | [-3974, -718]ns | [1166, 1252] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4261ns | no significant difference | [-13, +351]ns | [1835, 5079] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1842ns | +22.1% | +79.0% | +133.9% | -33.6% | +82.4% |
| 1 | 2 | 3338ns | +0.1% | +22.2% | +4.7% | -65.0% | +13.4% |
| 1 | 3 | 3494ns | +21.4% | -13.9% | -47.9% | -56.5% | +29.8% |
| 1 | 4 | 1873ns | +82.2% | +110.2% | +117.9% | -40.0% | -2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.250 | moderate- |
| warm-container-kernel | 0.045 | ok |
| warm-container-lanes-deferred | -0.805 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.290 | moderate- |
| warm-container-native | -0.580 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.340 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 46.9ns | 3666.3ns | 1.3% |  |
| warm-container-kernel | 53.0ns | 3594.2ns | 1.5% |  |
| warm-container-lanes-deferred | 52.0ns | 3443.1ns | 1.5% |  |
| warm-container-minimum | 43.2ns | 3358.9ns | 1.3% |  |
| warm-container-native | 98.1ns | 1263.4ns | 7.8% | HIGH |
| warm-container-plusone | 53.0ns | 4014.2ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1667.9-6656.8 ns)
   1667.9 |########################################
   1917.4 |
   2166.8 |
   2416.3 |
   2665.7 |
   2915.1 |
   3164.6 |
   3414.0 |
   3663.5 |
   3912.9 |
   4162.4 |
   4411.8 |
   4661.2 |##
   4910.7 |####
   5160.1 |##
   5409.6 |###########
   5659.0 |
   5908.5 |######
   6157.9 |##
   6407.4 |
  (4 below, 5 above range)

warm-container-kernel (n=40, range 1549.2-5805.2 ns)
   1549.2 |########################################
   1762.0 |#####
   1974.8 |
   2187.6 |
   2400.4 |
   2613.2 |
   2826.0 |##
   3038.8 |
   3251.6 |
   3464.4 |
   3677.2 |
   3890.0 |#####
   4102.8 |##
   4315.6 |#####
   4528.4 |
   4741.2 |########
   4954.0 |#####
   5166.8 |#####
   5379.6 |#####
   5592.4 |####################
  (1 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1369.3-5590.4 ns)
   1369.3 |
   1580.3 |########################################
   1791.4 |####
   2002.4 |####
   2213.5 |
   2424.6 |
   2635.6 |
   2846.7 |####
   3057.7 |
   3268.8 |
   3479.8 |
   3690.9 |####
   3902.0 |########
   4113.0 |
   4324.1 |#################
   4535.1 |####
   4746.2 |#############
   4957.3 |####
   5168.3 |#############
   5379.4 |#############
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1491.2-5812.1 ns)
   1491.2 |##########
   1707.2 |########################################
   1923.2 |
   2139.3 |#####
   2355.4 |
   2571.4 |
   2787.5 |
   3003.5 |#####
   3219.6 |#####
   3435.6 |
   3651.7 |
   3867.7 |####################
   4083.8 |###############
   4299.8 |##########
   4515.9 |##########
   4731.9 |#####
   4948.0 |##########
   5164.0 |##########
   5380.1 |##########
   5596.1 |
  (7 below, 2 above range)

warm-container-native (n=40, range 1095.5-1555.3 ns)
   1095.5 |############################
   1118.5 |####
   1141.5 |####
   1164.5 |############
   1187.5 |
   1210.5 |########################################
   1233.4 |####
   1256.4 |
   1279.4 |
   1302.4 |####
   1325.4 |
   1348.4 |############
   1371.4 |############
   1394.4 |
   1417.4 |
   1440.4 |
   1463.3 |####
   1486.3 |
   1509.3 |
   1532.3 |
  (4 below, 5 above range)

warm-container-plusone (n=40, range 1821.1-6740.3 ns)
   1821.1 |########################################
   2067.0 |###
   2313.0 |###
   2559.0 |
   2804.9 |
   3050.9 |
   3296.8 |
   3542.8 |
   3788.7 |
   4034.7 |##########
   4280.7 |#######
   4526.6 |#######
   4772.6 |###
   5018.5 |#######
   5264.5 |###
   5510.5 |#######
   5756.4 |###
   6002.4 |
   6248.3 |###
   6494.3 |#####################
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=58.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-kernel**: CV=50.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=50.4% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=52.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=49.7% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
