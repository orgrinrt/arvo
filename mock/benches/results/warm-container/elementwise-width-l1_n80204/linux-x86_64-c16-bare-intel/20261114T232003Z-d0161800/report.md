# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 915 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 57% (significant)

warm-container-kernel is -1.20 us (57%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.3x slower than the field

warm-container-headroom (2.11 us) is 2.3x the fastest (915 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-kernel) are a dead heat (<1%)

warm-container-native (915 ns) and warm-container-kernel (916 ns) differ by 0.10%, inside the noise, even though the wider field spreads 130.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.50)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (107% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 107% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 1.8x its best-20%

warm-container-kernel's best 20% of batches run at 795 ns but its worst 20% at 1.39 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 915.4 ns median (-56.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.31x (fastest 915.4 ns, slowest 2114.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 2550ns | 2342ns | 2225ns | 2409ns | 3300ns | base |
| warm-container-kernel | 1279ns | 1146ns | 989ns | 1216ns | 1758ns | -49.85% |
| warm-container-minimum | 1397ns | 1301ns | 1126ns | 1352ns | 1806ns | -45.21% |
| warm-container-native | 1223ns | 1145ns | 991ns | 1180ns | 1586ns | -52.03% |
| warm-container-plusone | 2261ns | 2338ns | 1694ns | 2312ns | 2674ns | -11.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 2291ns | 2009ns | 2951ns | base | 17.876 |
| warm-container-kernel | 1015ns | 795ns | 1392ns | -55.70% | 40.350 |
| warm-container-minimum | 1093ns | 902ns | 1367ns | -52.30% | 37.477 |
| warm-container-native | 966ns | 785ns | 1253ns | -57.86% | 42.422 |
| warm-container-plusone | 2032ns | 1529ns | 2375ns | -11.31% | 20.154 |

## Performance model

- Peak throughput: **52.171 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 19.375 | 37.1% |
| warm-container-kernel | 44.699 | 85.7% |
| warm-container-minimum | 40.244 | 77.1% |
| warm-container-native | 44.745 | 85.8% |
| warm-container-plusone | 19.404 | 37.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 2550ns | 2550ns | base |
| warm-container-kernel | 1279ns | 1279ns | -49.85% |
| warm-container-minimum | 1397ns | 1397ns | -45.21% |
| warm-container-native | 1223ns | 1223ns | -52.03% |
| warm-container-plusone | 2261ns | 2261ns | -11.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2114ns | base | --- | [2111, 2176] | --- | --- | --- | --- |
| warm-container-kernel | 916ns | -1198.0ns (-56.7%) | [-1278, -1195]ns | [916, 992] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1018ns | -1131.4ns (-53.5%) | [-1197, -1096]ns | [1016, 1161] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 915ns | -1195.6ns (-56.6%) | [-1260, -1131]ns | [914, 917] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2111ns | -94.2ns (-4.5%) | [-534, -1]ns | [2111, 2133] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2113ns | -56.6% | -51.9% | -42.4% | +0.9% |
| 1 | 2 | 2713ns | -51.8% | -50.8% | -71.1% | -21.4% |
| 1 | 3 | 2111ns | -61.2% | -50.4% | -56.6% | -0.0% |
| 1 | 4 | 2112ns | -55.6% | -56.7% | -56.7% | -28.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.416 | moderate- |
| warm-container-kernel | -0.503 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.194 | ok |
| warm-container-native | -0.356 | moderate- |
| warm-container-plusone | -0.056 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 97.0ns | 2291.4ns | 4.2% |  |
| warm-container-kernel | 102.6ns | 1015.1ns | 10.1% | HIGH |
| warm-container-minimum | 111.2ns | 1092.9ns | 10.2% | HIGH |
| warm-container-native | 94.9ns | 965.5ns | 9.8% | HIGH |
| warm-container-plusone | 88.5ns | 2032.3ns | 4.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2008.7-2950.8 ns)
   2008.7 |##
   2055.8 |
   2102.9 |########################################
   2150.0 |########
   2197.1 |##
   2244.2 |
   2291.3 |
   2338.4 |########
   2385.6 |
   2432.7 |##
   2479.8 |
   2526.9 |
   2574.0 |
   2621.1 |##
   2668.2 |##
   2715.3 |##
   2762.4 |
   2809.5 |
   2856.6 |
   2903.7 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 794.7-1392.1 ns)
    794.7 |###
    824.6 |################
    854.4 |
    884.3 |
    914.2 |########################################
    944.1 |#############
    973.9 |
   1003.8 |#############
   1033.7 |
   1063.5 |
   1093.4 |
   1123.3 |
   1153.1 |
   1183.0 |###
   1212.9 |
   1242.7 |
   1272.6 |
   1302.5 |################
   1332.3 |###
   1362.2 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 902.3-1367.4 ns)
    902.3 |########################################
    925.5 |
    948.8 |
    972.1 |####
    995.3 |########################################
   1018.6 |########
   1041.8 |
   1065.1 |####
   1088.3 |
   1111.6 |
   1134.9 |########
   1158.1 |########
   1181.4 |
   1204.6 |####
   1227.9 |#############
   1251.1 |
   1274.4 |########
   1297.7 |####
   1320.9 |
   1344.2 |
  (2 below, 5 above range)

warm-container-native (n=40, range 785.1-1252.7 ns)
    785.1 |##
    808.5 |##
    831.9 |##########
    855.3 |#####
    878.6 |
    902.0 |########################################
    925.4 |
    948.8 |##
    972.1 |
    995.5 |
   1018.9 |
   1042.3 |
   1065.7 |##
   1089.0 |
   1112.4 |
   1135.8 |#############
   1159.2 |
   1182.6 |
   1205.9 |
   1229.3 |
  (5 below, 5 above range)

warm-container-plusone (n=40, range 1529.1-2375.4 ns)
   1529.1 |##
   1571.5 |
   1613.8 |########
   1656.1 |##
   1698.4 |
   1740.7 |
   1783.0 |
   1825.3 |
   1867.6 |
   1910.0 |
   1952.3 |
   1994.6 |
   2036.9 |##
   2079.2 |########################################
   2121.5 |#####
   2163.8 |########
   2206.1 |##
   2248.5 |
   2290.8 |##
   2333.1 |########
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=21.6% (high variance, measurements may be unstable)
