# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 52% (significant)

warm-container-native is -1.73 us (52%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.2x slower than the field

warm-container-plusone (3.79 us) is 2.2x the fastest (1.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-kernel} vs {warm-container-headroom, warm-container-plusone} (83% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-native vs stability leader warm-container-minimum (+2% speed for 2.1x steadier)

warm-container-native is fastest (1.75 us, CV 14.7%); warm-container-minimum gives up 2.4% median for 2.1x lower variance (CV 6.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.8x its best-20%

warm-container-headroom's best 20% of batches run at 1.03 us but its worst 20% at 4.96 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1749.2 ns median (-46.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.16x (fastest 1749.2 ns, slowest 3786.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112697097 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3406ns | 3470ns | 1084ns | 3561ns | 5262ns | base |
| warm-container-kernel | 2154ns | 2046ns | 1817ns | 2128ns | 2567ns | -36.76% |
| warm-container-minimum | 2105ns | 2020ns | 1970ns | 2058ns | 2381ns | -38.19% |
| warm-container-native | 2008ns | 1981ns | 1693ns | 1957ns | 2479ns | -41.02% |
| warm-container-plusone | 3790ns | 3989ns | 2111ns | 3966ns | 4943ns | +11.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3219ns | 1028ns | 4960ns | base | 12.723 |
| warm-container-kernel | 1902ns | 1612ns | 2260ns | -40.91% | 21.533 |
| warm-container-minimum | 1858ns | 1750ns | 2058ns | -42.30% | 22.050 |
| warm-container-native | 1778ns | 1505ns | 2187ns | -44.77% | 23.038 |
| warm-container-plusone | 3582ns | 2002ns | 4623ns | +11.25% | 11.436 |

## Performance model

- Peak throughput: **39.845 Gops/s** (warm-container-headroom; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.442 | 31.2% |
| warm-container-kernel | 22.827 | 57.3% |
| warm-container-minimum | 22.863 | 57.4% |
| warm-container-native | 23.416 | 58.8% |
| warm-container-plusone | 10.819 | 27.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3406ns | 3406ns | base |
| warm-container-kernel | 2154ns | 2154ns | -36.76% |
| warm-container-minimum | 2105ns | 2105ns | -38.19% |
| warm-container-native | 2008ns | 2008ns | -41.02% |
| warm-container-plusone | 3790ns | 3790ns | +11.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3292ns | base | --- | [3180, 3808] | --- | --- | --- | --- |
| warm-container-kernel | 1794ns | -1286.7ns (-39.1%) | [-2104, -1181]ns | [1793, 1993] | YES | 0.0009 | 0.0007 | 0 |
| warm-container-minimum | 1792ns | -1478.8ns (-44.9%) | [-1819, -1365]ns | [1791, 1837] | YES | 0.0009 | 0.0007 | 0 |
| warm-container-native | 1749ns | -1726.9ns (-52.5%) | [-1984, -1526]ns | [1628, 1810] | YES | 0.0009 | 0.0007 | 0 |
| warm-container-plusone | 3786ns | +284.8ns (+8.7%) | [+30, +441]ns | [3475, 4071] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4315ns | -58.4% | -58.0% | -55.0% | -31.3% |
| 1 | 2 | 3655ns | -55.4% | -48.7% | -55.5% | +8.2% |
| 1 | 3 | 3203ns | -36.8% | -44.1% | -50.4% | +12.3% |
| 1 | 4 | 1347ns | +50.3% | +33.0% | +35.5% | +176.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.109 | ok |
| warm-container-kernel | 0.045 | ok |
| warm-container-minimum | -0.217 | moderate- |
| warm-container-native | -0.205 | moderate- |
| warm-container-plusone | -0.400 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 67.3ns | 3219.4ns | 2.1% |  |
| warm-container-kernel | 91.6ns | 1902.2ns | 4.8% |  |
| warm-container-minimum | 92.8ns | 1857.6ns | 5.0% |  |
| warm-container-native | 86.1ns | 1778.0ns | 4.8% |  |
| warm-container-plusone | 74.6ns | 3581.7ns | 2.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1028.0-4959.9 ns)
   1028.0 |#################
   1224.6 |
   1421.2 |
   1617.8 |#####
   1814.4 |#####
   2011.0 |
   2207.6 |
   2404.2 |
   2600.8 |
   2797.4 |###########
   2994.0 |###########
   3190.6 |########################################
   3387.2 |######################
   3583.8 |
   3780.4 |###########
   3977.0 |######################
   4173.6 |#####
   4370.2 |#####
   4566.8 |#################
   4763.4 |#####
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1612.4-2260.1 ns)
   1612.4 |####################
   1644.8 |
   1677.2 |###
   1709.5 |
   1741.9 |
   1774.3 |########################################
   1806.7 |###
   1839.1 |
   1871.5 |
   1903.9 |
   1936.2 |###
   1968.6 |####################
   2001.0 |
   2033.4 |##########
   2065.8 |
   2098.2 |###
   2130.6 |
   2162.9 |
   2195.3 |
   2227.7 |##########
  (2 below, 4 above range)

warm-container-minimum (n=40, range 1749.6-2058.0 ns)
   1749.6 |
   1765.0 |
   1780.4 |########################################
   1795.9 |
   1811.3 |
   1826.7 |############
   1842.1 |
   1857.5 |
   1873.0 |
   1888.4 |##
   1903.8 |
   1919.2 |##
   1934.7 |##
   1950.1 |
   1965.5 |
   1980.9 |##########
   1996.3 |
   2011.8 |
   2027.2 |
   2042.6 |##
  (2 below, 3 above range)

warm-container-native (n=40, range 1504.6-2186.7 ns)
   1504.6 |#####
   1538.7 |##########
   1572.8 |
   1606.9 |########################################
   1641.0 |#####
   1675.1 |##########
   1709.2 |
   1743.3 |
   1777.4 |##############################
   1811.5 |##########
   1845.6 |##########
   1879.7 |#####
   1913.8 |#####
   1948.0 |#####
   1982.1 |#####
   2016.2 |#####
   2050.3 |#####
   2084.4 |
   2118.5 |#####
   2152.6 |
  (6 below, 3 above range)

warm-container-plusone (n=40, range 2002.3-4623.1 ns)
   2002.3 |
   2133.3 |
   2264.4 |
   2395.4 |
   2526.5 |
   2657.5 |
   2788.6 |
   2919.6 |
   3050.6 |
   3181.7 |#################################
   3312.7 |######
   3443.8 |#################################
   3574.8 |#############
   3705.8 |#################################
   3836.9 |#############
   3967.9 |#############
   4099.0 |########################################
   4230.0 |######
   4361.1 |######
   4492.1 |######
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=42.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=27.8% (high variance, measurements may be unstable)
