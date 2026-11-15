# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.37 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 53% (significant)

warm-container-native is -1.25 us (53%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.1x slower than the field

warm-container-headroom (2.37 us) is 2.1x the fastest (1.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (1.13 us) and warm-container-native (1.13 us) differ by 0.16%, inside the noise, even though the wider field spreads 110.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.79)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (95% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 95% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 3.6x its best-20%

warm-container-plusone's best 20% of batches run at 2.14 us but its worst 20% at 7.70 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1125.8 ns median (-52.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.11x (fastest 1125.8 ns, slowest 2374.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4075ns | 2438ns | 2425ns | 3152ns | 8495ns | base |
| warm-container-kernel | 1438ns | 1355ns | 1351ns | 1405ns | 1623ns | -64.73% |
| warm-container-minimum | 1496ns | 1465ns | 1350ns | 1455ns | 1764ns | -63.30% |
| warm-container-native | 1465ns | 1375ns | 1344ns | 1425ns | 1708ns | -64.05% |
| warm-container-plusone | 3856ns | 2437ns | 2194ns | 3039ns | 7969ns | -5.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3958ns | 2362ns | 8220ns | base | 10.348 |
| warm-container-kernel | 1186ns | 1123ns | 1299ns | -70.04% | 34.543 |
| warm-container-minimum | 1233ns | 1122ns | 1430ns | -68.85% | 33.218 |
| warm-container-native | 1204ns | 1118ns | 1373ns | -69.59% | 34.027 |
| warm-container-plusone | 3744ns | 2136ns | 7704ns | -5.41% | 10.940 |

## Performance model

- Peak throughput: **36.649 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.253 | 47.1% |
| warm-container-kernel | 36.381 | 99.3% |
| warm-container-minimum | 33.680 | 91.9% |
| warm-container-native | 36.323 | 99.1% |
| warm-container-plusone | 17.256 | 47.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4075ns | 4075ns | base |
| warm-container-kernel | 1438ns | 1438ns | -64.73% |
| warm-container-minimum | 1496ns | 1496ns | -63.30% |
| warm-container-native | 1465ns | 1465ns | -64.05% |
| warm-container-plusone | 3856ns | 3856ns | -5.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2374ns | base | --- | [2372, 2461] | --- | --- | --- | --- |
| warm-container-kernel | 1126ns | -1247.4ns (-52.5%) | [-1271, -1244]ns | [1125, 1248] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1216ns | -1250.8ns (-52.7%) | [-1276, -1245]ns | [1123, 1248] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1128ns | -1250.9ns (-52.7%) | [-1338, -1243]ns | [1124, 1249] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2374ns | no significant difference | [-78, +2]ns | [2372, 2387] | no | 0.5224 | 0.5224 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2373ns | -52.6% | -52.7% | -44.9% | +0.2% |
| 1 | 2 | 2377ns | -47.5% | -51.9% | -52.7% | -0.0% |
| 1 | 3 | 2372ns | -52.6% | -40.9% | -47.5% | +0.1% |
| 1 | 4 | 2375ns | -48.7% | -49.0% | -52.7% | +59.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.791 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.788 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.170 | ok |
| warm-container-native | -0.595 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.085 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.5ns | 3958.2ns | 1.0% |  |
| warm-container-kernel | 92.9ns | 1185.8ns | 7.8% | HIGH |
| warm-container-minimum | 96.6ns | 1233.1ns | 7.8% | HIGH |
| warm-container-native | 96.0ns | 1203.8ns | 8.0% | HIGH |
| warm-container-plusone | 39.1ns | 3744.2ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2361.7-8219.5 ns)
   2361.7 |########################################
   2654.6 |
   2947.5 |
   3240.4 |
   3533.3 |
   3826.2 |
   4119.0 |###
   4411.9 |
   4704.8 |
   4997.7 |
   5290.6 |
   5583.5 |#
   5876.4 |
   6169.3 |
   6462.2 |
   6755.1 |#
   7048.0 |#
   7340.9 |#
   7633.8 |#
   7926.7 |###
  (2 below, 4 above range)

warm-container-kernel (n=40, range 1123.2-1299.4 ns)
   1123.2 |########################################
   1132.0 |
   1140.8 |
   1149.6 |
   1158.4 |####
   1167.2 |
   1176.0 |##
   1184.8 |##
   1193.7 |
   1202.5 |
   1211.3 |
   1220.1 |
   1228.9 |
   1237.7 |
   1246.5 |############################
   1255.3 |
   1264.1 |
   1273.0 |
   1281.8 |
   1290.6 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1121.6-1430.1 ns)
   1121.6 |########################################
   1137.0 |
   1152.4 |###
   1167.8 |###
   1183.3 |###
   1198.7 |#######
   1214.1 |###
   1229.6 |
   1245.0 |#############################
   1260.4 |###
   1275.8 |
   1291.3 |###
   1306.7 |###
   1322.1 |
   1337.5 |
   1353.0 |###
   1368.4 |
   1383.8 |###
   1399.2 |##############
   1414.7 |
  (4 below, 2 above range)

warm-container-native (n=40, range 1117.6-1373.5 ns)
   1117.6 |########################################
   1130.4 |##
   1143.2 |
   1156.0 |##
   1168.8 |
   1181.6 |
   1194.4 |##
   1207.2 |
   1220.0 |
   1232.8 |##
   1245.5 |################
   1258.3 |
   1271.1 |##
   1283.9 |
   1296.7 |
   1309.5 |
   1322.3 |
   1335.1 |##
   1347.9 |##
   1360.7 |
  (1 below, 5 above range)

warm-container-plusone (n=40, range 2136.1-7704.0 ns)
   2136.1 |########################################
   2414.5 |
   2692.9 |
   2971.3 |
   3249.7 |
   3528.1 |#
   3806.5 |
   4084.9 |
   4363.3 |
   4641.7 |
   4920.1 |
   5198.4 |#
   5476.8 |#####
   5755.2 |#
   6033.6 |#
   6312.0 |#
   6590.4 |
   6868.8 |
   7147.2 |
   7425.6 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=61.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=61.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.6x (possible bimodal distribution)
