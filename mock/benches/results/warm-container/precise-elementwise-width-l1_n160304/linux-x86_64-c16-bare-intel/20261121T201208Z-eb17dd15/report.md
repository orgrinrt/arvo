# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.37 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 48% (significant)

warm-container-kernel is -1.14 us (48%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {warm-container-minimum, warm-container-kernel, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (69% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 69% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-kernel (+5% speed for 2.0x steadier)

warm-container-minimum is fastest (1.19 us, CV 18.3%); warm-container-kernel gives up 5.3% median for 2.0x lower variance (CV 9.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.1x its best-20%

warm-container-plusone's best 20% of batches run at 2.10 us but its worst 20% at 8.58 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1188.5 ns median (-50.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.00x (fastest 1188.5 ns, slowest 2374.9 ns)

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
| Counter frequency | 2112749630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3959ns | 2437ns | 1994ns | 3375ns | 7676ns | base |
| warm-container-kernel | 1529ns | 1504ns | 1350ns | 1512ns | 1760ns | -61.36% |
| warm-container-minimum | 1381ns | 1473ns | 1072ns | 1354ns | 1769ns | -65.12% |
| warm-container-native | 1769ns | 1688ns | 1412ns | 1732ns | 2240ns | -55.30% |
| warm-container-plusone | 4087ns | 2437ns | 2158ns | 3133ns | 8875ns | +3.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3847ns | 1943ns | 7428ns | base | 10.647 |
| warm-container-kernel | 1265ns | 1125ns | 1432ns | -67.12% | 32.385 |
| warm-container-minimum | 1141ns | 892ns | 1446ns | -70.35% | 35.907 |
| warm-container-native | 1464ns | 1184ns | 1836ns | -61.96% | 27.987 |
| warm-container-plusone | 3968ns | 2102ns | 8581ns | +3.15% | 10.322 |

## Performance model

- Peak throughput: **45.899 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.247 | 37.6% |
| warm-container-kernel | 32.742 | 71.3% |
| warm-container-minimum | 34.465 | 75.1% |
| warm-container-native | 29.179 | 63.6% |
| warm-container-plusone | 17.257 | 37.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3959ns | 3959ns | base |
| warm-container-kernel | 1529ns | 1529ns | -61.36% |
| warm-container-minimum | 1381ns | 1381ns | -65.12% |
| warm-container-native | 1769ns | 1769ns | -55.30% |
| warm-container-plusone | 4087ns | 4087ns | +3.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2375ns | base | --- | [1953, 4818] | --- | --- | --- | --- |
| warm-container-kernel | 1251ns | -1138.9ns (-48.0%) | [-3551, -817]ns | [1250, 1260] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1188ns | -1130.8ns (-47.6%) | [-3639, -1038]ns | [937, 1247] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1404ns | -971.6ns (-40.9%) | [-3302, -745]ns | [1357, 1542] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2374ns | no significant difference | [-2, +429]ns | [2372, 2386] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2669ns | -53.1% | -47.4% | -46.0% | -10.9% |
| 1 | 2 | 2839ns | -60.3% | -56.0% | -58.3% | -16.4% |
| 1 | 3 | 1948ns | -31.3% | -52.9% | -28.0% | +21.7% |
| 1 | 4 | 2138ns | -37.2% | -56.2% | -21.1% | +14.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.071 | ok |
| warm-container-kernel | -0.084 | ok |
| warm-container-minimum | 0.280 | moderate+ |
| warm-container-native | -0.025 | ok |
| warm-container-plusone | -0.134 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 39.1ns | 3847.0ns | 1.0% |  |
| warm-container-kernel | 97.4ns | 1264.8ns | 7.7% | HIGH |
| warm-container-minimum | 89.0ns | 1140.7ns | 7.8% | HIGH |
| warm-container-native | 112.0ns | 1463.5ns | 7.7% | HIGH |
| warm-container-plusone | 40.4ns | 3968.1ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1942.6-7428.0 ns)
   1942.6 |########################################
   2216.9 |#################
   2491.1 |
   2765.4 |##
   3039.7 |##
   3314.0 |
   3588.2 |##
   3862.5 |
   4136.8 |
   4411.0 |
   4685.3 |
   4959.6 |
   5233.8 |
   5508.1 |
   5782.4 |#####
   6056.7 |#####
   6330.9 |#####
   6605.2 |##
   6879.5 |#####
   7153.7 |#####
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1125.2-1431.8 ns)
   1125.2 |#####################
   1140.5 |
   1155.8 |
   1171.1 |
   1186.5 |
   1201.8 |
   1217.1 |###
   1232.5 |###
   1247.8 |########################################
   1263.1 |###
   1278.5 |
   1293.8 |###
   1309.1 |###
   1324.5 |
   1339.8 |
   1355.1 |###
   1370.5 |###
   1385.8 |
   1401.1 |#####################
   1416.5 |
  (4 below, 2 above range)

warm-container-minimum (n=40, range 892.4-1445.9 ns)
    892.4 |########
    920.1 |########################################
    947.7 |
    975.4 |########
   1003.1 |########
   1030.8 |
   1058.4 |
   1086.1 |
   1113.8 |####
   1141.5 |
   1169.1 |
   1196.8 |
   1224.5 |########################################
   1252.2 |
   1279.8 |
   1307.5 |####
   1335.2 |########
   1362.9 |
   1390.5 |##########################
   1418.2 |
  (4 below, 2 above range)

warm-container-native (n=40, range 1183.7-1836.0 ns)
   1183.7 |#####
   1216.3 |##################################
   1248.9 |#####
   1281.5 |
   1314.1 |#####
   1346.8 |#####
   1379.4 |########################################
   1412.0 |
   1444.6 |###########
   1477.2 |
   1509.8 |###########
   1542.4 |#####
   1575.1 |######################
   1607.7 |#####
   1640.3 |
   1672.9 |######################
   1705.5 |
   1738.1 |
   1770.7 |
   1803.3 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 2101.9-8580.7 ns)
   2101.9 |########################################
   2425.9 |
   2749.8 |#
   3073.7 |
   3397.7 |
   3721.6 |
   4045.5 |
   4369.5 |
   4693.4 |
   5017.4 |
   5341.3 |
   5665.2 |#
   5989.2 |#
   6313.1 |###
   6637.0 |#
   6961.0 |
   7284.9 |#
   7608.8 |#
   7932.8 |
   8256.7 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=66.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
