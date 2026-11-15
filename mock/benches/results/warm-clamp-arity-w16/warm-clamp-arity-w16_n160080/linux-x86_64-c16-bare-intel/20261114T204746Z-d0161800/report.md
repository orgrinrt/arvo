# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.76 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 745 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 106% faster than the next best (warm-clamp-accfit)

warm-clamp-head (745 ns) leads warm-clamp-accfit (1.53 us) by 106%, a clear separation rather than a photo finish. CV 28.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 76% (significant)

warm-clamp-head is -2.87 us (76%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 5.1x slower than the field

warm-clamp-acc64 (3.76 us) is 5.1x the fastest (745 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.1x the fastest

Fastest warm-clamp-head (745 ns) to slowest warm-clamp-acc64 (3.76 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 3.6x its best-20%

warm-clamp-minimum's best 20% of batches run at 2.34 us but its worst 20% at 8.45 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 745.1 ns median (-80.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.05x (fastest 745.1 ns, slowest 3764.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3611ns | 3971ns | 1608ns | 3883ns | 4798ns | base |
| warm-clamp-accfit | 1752ns | 1761ns | 1462ns | 1753ns | 2042ns | -51.47% |
| warm-clamp-accfit-dyn | 2092ns | 1992ns | 1746ns | 2053ns | 2555ns | -42.07% |
| warm-clamp-head | 1064ns | 951ns | 849ns | 973ns | 1553ns | -70.53% |
| warm-clamp-min-lanes | 2596ns | 2632ns | 2147ns | 2599ns | 3036ns | -28.11% |
| warm-clamp-minimum | 4473ns | 2486ns | 2404ns | 3738ns | 8744ns | +23.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3400ns | 1521ns | 4518ns | base | 2.409 |
| warm-clamp-accfit | 1516ns | 1273ns | 1753ns | -55.43% | 5.405 |
| warm-clamp-accfit-dyn | 1807ns | 1521ns | 2166ns | -46.86% | 4.534 |
| warm-clamp-head | 824ns | 665ns | 1199ns | -75.77% | 9.945 |
| warm-clamp-min-lanes | 2360ns | 1958ns | 2734ns | -30.60% | 3.471 |
| warm-clamp-minimum | 4340ns | 2339ns | 8455ns | +27.64% | 1.888 |

## Performance model

- Peak throughput: **12.324 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.176 | 17.7% |
| warm-clamp-accfit | 5.341 | 43.3% |
| warm-clamp-accfit-dyn | 4.764 | 38.7% |
| warm-clamp-head | 10.994 | 89.2% |
| warm-clamp-min-lanes | 3.405 | 27.6% |
| warm-clamp-minimum | 3.385 | 27.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3611ns | 3611ns | base |
| warm-clamp-accfit | 1752ns | 1752ns | -51.47% |
| warm-clamp-accfit-dyn | 2092ns | 2092ns | -42.07% |
| warm-clamp-head | 1064ns | 1064ns | -70.53% |
| warm-clamp-min-lanes | 2596ns | 2596ns | -28.11% |
| warm-clamp-minimum | 4473ns | 4473ns | +23.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3764ns | base | --- | [3456, 3953] | --- | --- | --- | --- |
| warm-clamp-accfit | 1534ns | -2215.0ns (-58.8%) | [-2378, -2039]ns | [1468, 1535] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1720ns | -1812.0ns (-48.1%) | [-2111, -1540]ns | [1703, 1916] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 745ns | -2868.3ns (-76.2%) | [-3089, -2561]ns | [713, 746] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2406ns | -1381.1ns (-36.7%) | [-1471, -1186]ns | [2255, 2460] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 2420ns | no significant difference | [-517, +1562]ns | [2363, 5097] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3334ns | -61.7% | -44.2% | -78.6% | -39.2% | -27.4% |
| 1 | 2 | 4202ns | -64.0% | -59.5% | -82.2% | -43.4% | +10.4% |
| 1 | 3 | 3783ns | -58.8% | -59.5% | -69.0% | -35.7% | -19.3% |
| 1 | 4 | 3803ns | -55.8% | -46.7% | -82.1% | -32.1% | -37.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.496 | moderate- |
| warm-clamp-accfit | 0.092 | ok |
| warm-clamp-accfit-dyn | -0.358 | moderate- |
| warm-clamp-head | -0.432 | moderate- |
| warm-clamp-min-lanes | 0.070 | ok |
| warm-clamp-minimum | -0.329 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 71.1ns | 3400.3ns | 2.1% |  |
| warm-clamp-accfit | 84.0ns | 1515.5ns | 5.5% | HIGH |
| warm-clamp-accfit-dyn | 98.4ns | 1806.9ns | 5.4% | HIGH |
| warm-clamp-head | 86.0ns | 823.7ns | 10.4% | HIGH |
| warm-clamp-min-lanes | 83.5ns | 2359.9ns | 3.5% |  |
| warm-clamp-minimum | 44.7ns | 4340.0ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1520.7-4517.9 ns)
   1520.7 |
   1670.5 |
   1820.4 |########
   1970.3 |
   2120.1 |
   2270.0 |
   2419.8 |
   2569.7 |################
   2719.6 |########
   2869.4 |
   3019.3 |########
   3169.1 |################
   3319.0 |########
   3468.9 |################################
   3618.7 |################
   3768.6 |########################################
   3918.4 |################################
   4068.3 |################
   4218.2 |################
   4368.0 |################################
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 1273.5-1752.9 ns)
   1273.5 |################
   1297.4 |######
   1321.4 |
   1345.4 |
   1369.4 |###
   1393.3 |#############
   1417.3 |
   1441.3 |###
   1465.3 |
   1489.2 |###
   1513.2 |########################################
   1537.2 |
   1561.1 |##########
   1585.1 |
   1609.1 |
   1633.1 |###
   1657.0 |
   1681.0 |################
   1705.0 |
   1729.0 |###
  (1 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1521.4-2165.9 ns)
   1521.4 |###################################
   1553.6 |
   1585.8 |####
   1618.0 |####
   1650.3 |
   1682.5 |########################################
   1714.7 |####
   1746.9 |
   1779.2 |####
   1811.4 |
   1843.6 |####
   1875.9 |####
   1908.1 |###############################
   1940.3 |
   1972.5 |########
   2004.8 |
   2037.0 |########
   2069.2 |
   2101.4 |
   2133.7 |
  (1 below, 5 above range)

warm-clamp-head (n=40, range 664.7-1198.9 ns)
    664.7 |########################################
    691.4 |
    718.1 |##################
    744.8 |#############################
    771.6 |###
    798.3 |#######
    825.0 |
    851.7 |
    878.4 |###
    905.1 |
    931.8 |###
    958.5 |#######
    985.2 |
   1012.0 |
   1038.7 |
   1065.4 |
   1092.1 |
   1118.8 |###
   1145.5 |
   1172.2 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1957.9-2733.5 ns)
   1957.9 |
   1996.7 |###############
   2035.5 |#####
   2074.2 |
   2113.0 |
   2151.8 |
   2190.6 |###############
   2229.4 |#########################
   2268.2 |
   2306.9 |#####
   2345.7 |##########
   2384.5 |##########
   2423.3 |#########################
   2462.1 |########################################
   2500.8 |#####
   2539.6 |
   2578.4 |
   2617.2 |
   2656.0 |#####
   2694.8 |#####
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 2338.5-8454.9 ns)
   2338.5 |########################################
   2644.4 |##
   2950.2 |
   3256.0 |
   3561.8 |####
   3867.6 |
   4173.4 |
   4479.3 |
   4785.1 |
   5090.9 |
   5396.7 |
   5702.5 |
   6008.3 |
   6314.1 |######
   6620.0 |
   6925.8 |####
   7231.6 |####
   7537.4 |
   7843.2 |####
   8149.0 |##
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=31.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=25.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=10.4% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: CV=59.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
