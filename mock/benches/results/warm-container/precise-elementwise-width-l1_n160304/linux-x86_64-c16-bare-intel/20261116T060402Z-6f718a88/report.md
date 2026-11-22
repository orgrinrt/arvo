# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 57% (significant)

warm-container-minimum is -1.35 us (57%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.1x slower than the field

warm-container-plusone (2.38 us) is 2.1x the fastest (1.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-minimum) are a dead heat (<1%)

warm-container-kernel (1.12 us) and warm-container-minimum (1.13 us) differ by 0.12%, inside the noise, even though the wider field spreads 111.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.80)

warm-container-native's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (94% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 94% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-headroom is inconsistent: worst-20% is 3.9x its best-20%

warm-container-headroom's best 20% of batches run at 1.99 us but its worst 20% at 7.77 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (5 ns, 0.22%)

warm-container-plusone differs from baseline warm-container-headroom by 5 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1124.6 ns median (-52.5% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.11x (fastest 1124.6 ns, slowest 2378.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112840787 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4024ns | 2432ns | 2046ns | 3344ns | 8041ns | base |
| warm-container-kernel | 1348ns | 1352ns | 1160ns | 1341ns | 1556ns | -66.50% |
| warm-container-minimum | 1435ns | 1381ns | 1228ns | 1416ns | 1699ns | -64.34% |
| warm-container-native | 1510ns | 1499ns | 1255ns | 1490ns | 1827ns | -62.46% |
| warm-container-plusone | 4254ns | 2442ns | 2105ns | 3655ns | 8198ns | +5.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3907ns | 1993ns | 7771ns | base | 10.485 |
| warm-container-kernel | 1111ns | 966ns | 1257ns | -71.57% | 36.879 |
| warm-container-minimum | 1187ns | 1022ns | 1413ns | -69.61% | 34.504 |
| warm-container-native | 1243ns | 1041ns | 1488ns | -68.18% | 32.956 |
| warm-container-plusone | 4131ns | 2050ns | 7932ns | +5.75% | 9.915 |

## Performance model

- Peak throughput: **42.385 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.291 | 40.8% |
| warm-container-kernel | 36.422 | 85.9% |
| warm-container-minimum | 36.380 | 85.8% |
| warm-container-native | 33.600 | 79.3% |
| warm-container-plusone | 17.222 | 40.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4024ns | 4024ns | base |
| warm-container-kernel | 1348ns | 1348ns | -66.50% |
| warm-container-minimum | 1435ns | 1435ns | -64.34% |
| warm-container-native | 1510ns | 1510ns | -62.46% |
| warm-container-plusone | 4254ns | 4254ns | +5.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2369ns | base | --- | [2368, 3180] | --- | --- | --- | --- |
| warm-container-kernel | 1125ns | -1324.5ns (-55.9%) | [-2140, -1243]ns | [1124, 1126] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1126ns | -1345.2ns (-56.8%) | [-2125, -1244]ns | [1124, 1245] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1219ns | -1274.8ns (-53.8%) | [-1959, -1236]ns | [1123, 1321] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2378ns | +5.2ns (+0.2%) | [+3, +17]ns | [2372, 4970] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 1943ns | -42.1% | -36.7% | -28.5% | +0.3% |
| 1 | 2 | 2374ns | -59.2% | -42.0% | -57.0% | +0.2% |
| 1 | 3 | 2377ns | -52.7% | -56.9% | -40.9% | +0.0% |
| 1 | 4 | 2371ns | -52.5% | -52.6% | -52.6% | +10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.080 | ok |
| warm-container-kernel | -0.408 | moderate- |
| warm-container-minimum | -0.180 | ok |
| warm-container-native | -0.796 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.009 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.2ns | 3906.5ns | 1.1% |  |
| warm-container-kernel | 88.6ns | 1110.6ns | 8.0% | HIGH |
| warm-container-minimum | 93.0ns | 1187.1ns | 7.8% | HIGH |
| warm-container-native | 98.1ns | 1242.9ns | 7.9% | HIGH |
| warm-container-plusone | 43.1ns | 4131.3ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1993.3-7771.0 ns)
   1993.3 |
   2282.2 |########################################
   2571.1 |
   2860.0 |
   3148.9 |
   3437.8 |
   3726.6 |##
   4015.5 |
   4304.4 |##
   4593.3 |
   4882.2 |
   5171.1 |
   5459.9 |
   5748.8 |
   6037.7 |##
   6326.6 |
   6615.5 |
   6904.4 |########
   7193.2 |##
   7482.1 |##
  (7 below, 5 above range)

warm-container-kernel (n=40, range 966.4-1257.2 ns)
    966.4 |
    980.9 |##
    995.5 |
   1010.0 |##############
   1024.5 |
   1039.1 |
   1053.6 |
   1068.2 |
   1082.7 |
   1097.2 |
   1111.8 |########################################
   1126.3 |####
   1140.9 |
   1155.4 |##
   1169.9 |
   1184.5 |##
   1199.0 |
   1213.5 |
   1228.1 |
   1242.6 |##############
  (5 below, 1 above range)

warm-container-minimum (n=40, range 1022.0-1413.5 ns)
   1022.0 |########
   1041.5 |####
   1061.1 |####
   1080.7 |####
   1100.3 |####
   1119.8 |########################################
   1139.4 |
   1159.0 |####
   1178.6 |########
   1198.1 |
   1217.7 |####
   1237.3 |######################
   1256.9 |####
   1276.4 |####
   1296.0 |
   1315.6 |
   1335.2 |
   1354.7 |########
   1374.3 |#############
   1393.9 |########
  (6 below, 1 above range)

warm-container-native (n=40, range 1040.5-1488.4 ns)
   1040.5 |
   1062.9 |
   1085.3 |###
   1107.7 |########################################
   1130.1 |###
   1152.5 |
   1174.9 |###
   1197.3 |
   1219.7 |
   1242.1 |##############
   1264.5 |
   1286.9 |#######
   1309.3 |
   1331.7 |###
   1354.1 |###
   1376.4 |
   1398.8 |#########################
   1421.2 |
   1443.6 |###
   1466.0 |###
  (6 below, 3 above range)

warm-container-plusone (n=40, range 2050.4-7932.5 ns)
   2050.4 |
   2344.5 |########################################
   2638.6 |##
   2932.7 |
   3226.8 |
   3520.9 |
   3815.0 |
   4109.1 |##
   4403.2 |
   4697.3 |
   4991.4 |
   5285.5 |
   5579.6 |##
   5873.7 |
   6167.8 |
   6461.9 |##
   6756.0 |####
   7050.1 |######
   7344.3 |####
   7638.4 |####
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=59.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
