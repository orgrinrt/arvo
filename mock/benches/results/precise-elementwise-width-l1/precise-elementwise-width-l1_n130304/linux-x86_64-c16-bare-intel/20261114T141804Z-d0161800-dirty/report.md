# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 22% (significant)

warm-container-native is -522 ns (22%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is inconsistent: worst-20% is 4.0x its best-20%

warm-container-headroom's best 20% of batches run at 1.94 us but its worst 20% at 7.77 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1851.3 ns median (-21.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.34x (fastest 1851.3 ns, slowest 2480.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3964ns | 2435ns | 1991ns | 3263ns | 8039ns | base |
| warm-container-kernel | 2700ns | 2712ns | 2372ns | 2666ns | 3133ns | -31.88% |
| warm-container-minimum | 2341ns | 2348ns | 1637ns | 2382ns | 2919ns | -40.95% |
| warm-container-native | 2178ns | 2090ns | 1806ns | 2132ns | 2688ns | -45.05% |
| warm-container-plusone | 2329ns | 2104ns | 1898ns | 2195ns | 3162ns | -41.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3848ns | 1939ns | 7765ns | base | 10.645 |
| warm-container-kernel | 2454ns | 2174ns | 2820ns | -36.22% | 16.691 |
| warm-container-minimum | 2049ns | 1444ns | 2500ns | -46.75% | 19.991 |
| warm-container-native | 1928ns | 1609ns | 2364ns | -49.89% | 21.242 |
| warm-container-plusone | 2062ns | 1692ns | 2780ns | -46.42% | 19.867 |

## Performance model

- Peak throughput: **28.365 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.272 | 60.9% |
| warm-container-kernel | 16.513 | 58.2% |
| warm-container-minimum | 19.742 | 69.6% |
| warm-container-native | 22.124 | 78.0% |
| warm-container-plusone | 21.828 | 77.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3964ns | 3964ns | base |
| warm-container-kernel | 2700ns | 2700ns | -31.88% |
| warm-container-minimum | 2341ns | 2341ns | -40.95% |
| warm-container-native | 2178ns | 2178ns | -45.05% |
| warm-container-plusone | 2329ns | 2329ns | -41.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2372ns | base | --- | [2367, 2886] | --- | --- | --- | --- |
| warm-container-kernel | 2480ns | no significant difference | [-399, +129]ns | [2346, 2487] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 2075ns | -465.0ns (-19.6%) | [-1764, -108]ns | [2054, 2224] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 1851ns | -521.7ns (-22.0%) | [-1071, -380]ns | [1832, 2056] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1876ns | -358.3ns (-15.1%) | [-1010, -254]ns | [1853, 2006] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4000ns | -38.4% | -48.6% | -54.2% | -52.4% |
| 1 | 2 | 2543ns | -2.2% | -26.4% | -34.3% | -30.2% |
| 1 | 3 | 2371ns | +10.7% | -8.3% | -13.3% | +8.8% |
| 1 | 4 | 1945ns | +16.3% | +19.0% | +10.7% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.043 | ok |
| warm-container-kernel | -0.420 | moderate- |
| warm-container-minimum | 0.091 | ok |
| warm-container-native | 0.144 | ok |
| warm-container-plusone | -0.493 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 40.2ns | 3847.8ns | 1.0% |  |
| warm-container-kernel | 88.4ns | 2454.0ns | 3.6% |  |
| warm-container-minimum | 104.5ns | 2048.9ns | 5.1% | HIGH |
| warm-container-native | 92.6ns | 1928.2ns | 4.8% |  |
| warm-container-plusone | 97.3ns | 2061.7ns | 4.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1939.1-7765.2 ns)
   1939.1 |###########################
   2230.4 |########################################
   2521.7 |###
   2813.0 |
   3104.3 |###
   3395.6 |
   3686.9 |
   3978.2 |
   4269.5 |
   4560.9 |
   4852.2 |
   5143.5 |
   5434.8 |###
   5726.1 |
   6017.4 |###
   6308.7 |###
   6600.0 |######
   6891.3 |###
   7182.6 |###
   7473.9 |#########
  (3 below, 3 above range)

warm-container-kernel (n=40, range 2173.6-2819.7 ns)
   2173.6 |#####
   2205.9 |
   2238.2 |###############
   2270.5 |##
   2302.8 |#####
   2335.1 |##
   2367.4 |
   2399.7 |##
   2432.0 |
   2464.3 |########################################
   2496.6 |
   2528.9 |##
   2561.2 |
   2593.5 |##
   2625.8 |
   2658.1 |
   2690.4 |
   2722.7 |
   2755.0 |#####
   2787.3 |##
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1444.0-2500.4 ns)
   1444.0 |
   1496.8 |
   1549.6 |
   1602.5 |
   1655.3 |
   1708.1 |
   1760.9 |
   1813.7 |########################################
   1866.5 |###########
   1919.4 |#####
   1972.2 |
   2025.0 |########################################
   2077.8 |###########
   2130.6 |#################
   2183.4 |###########
   2236.3 |#####
   2289.1 |##################################
   2341.9 |#####
   2394.7 |
   2447.5 |
  (3 below, 5 above range)

warm-container-native (n=40, range 1609.0-2364.3 ns)
   1609.0 |
   1646.8 |###################################
   1684.6 |##########
   1722.3 |
   1760.1 |
   1797.9 |#####
   1835.6 |########################################
   1873.4 |#####
   1911.2 |#####
   1948.9 |
   1986.7 |#####
   2024.4 |##############################
   2062.2 |
   2100.0 |#####
   2137.7 |
   2175.5 |#####
   2213.3 |
   2251.0 |
   2288.8 |####################
   2326.5 |
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1691.9-2780.0 ns)
   1691.9 |###
   1746.3 |#######
   1800.7 |########################################
   1855.1 |#######
   1909.5 |##############
   1963.9 |
   2018.3 |##########
   2072.7 |###
   2127.1 |
   2181.5 |
   2235.9 |
   2290.3 |##########
   2344.7 |###
   2399.1 |
   2453.6 |
   2508.0 |###
   2562.4 |
   2616.8 |###
   2671.2 |###
   2725.6 |
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=21.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=20.6% (high variance, measurements may be unstable)
