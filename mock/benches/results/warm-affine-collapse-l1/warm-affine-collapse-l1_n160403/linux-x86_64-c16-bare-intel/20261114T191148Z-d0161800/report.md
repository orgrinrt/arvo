# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 1.80 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 62% (significant)

warm-container-lanes-deferred is -3.09 us (62%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.8x slower than the field

warm-container-headroom (4.98 us) is 2.8x the fastest (1.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.75)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-plusone is inconsistent: worst-20% is 4.0x its best-20%

warm-container-plusone's best 20% of batches run at 1.63 us but its worst 20% at 6.52 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1800.7 ns median (-63.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.77x (fastest 1800.7 ns, slowest 4983.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4124ns | 5160ns | 1858ns | 4090ns | 6491ns | base |
| warm-container-kernel | 2121ns | 2050ns | 1757ns | 2075ns | 2622ns | -48.58% |
| warm-container-lanes-deferred | 2159ns | 2088ns | 1878ns | 2112ns | 2584ns | -47.64% |
| warm-container-minimum | 2425ns | 2086ns | 1778ns | 2182ns | 3801ns | -41.19% |
| warm-container-native | 2137ns | 2029ns | 2014ns | 2078ns | 2436ns | -48.19% |
| warm-container-plusone | 4018ns | 3237ns | 1691ns | 3863ns | 6806ns | -2.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3971ns | 1794ns | 6209ns | base | 8.252 |
| warm-container-kernel | 1873ns | 1563ns | 2305ns | -52.83% | 17.494 |
| warm-container-lanes-deferred | 1907ns | 1669ns | 2276ns | -51.97% | 17.180 |
| warm-container-minimum | 2143ns | 1579ns | 3356ns | -46.02% | 15.287 |
| warm-container-native | 1885ns | 1787ns | 2105ns | -52.53% | 17.381 |
| warm-container-plusone | 3869ns | 1634ns | 6517ns | -2.56% | 8.469 |

## Performance model

- Peak throughput: **20.960 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.576 | 31.4% |
| warm-container-kernel | 18.001 | 85.9% |
| warm-container-lanes-deferred | 17.963 | 85.7% |
| warm-container-minimum | 17.882 | 85.3% |
| warm-container-native | 18.198 | 86.8% |
| warm-container-plusone | 10.478 | 50.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4124ns | 4124ns | base |
| warm-container-kernel | 2121ns | 2121ns | -48.58% |
| warm-container-lanes-deferred | 2159ns | 2159ns | -47.64% |
| warm-container-minimum | 2425ns | 2425ns | -41.19% |
| warm-container-native | 2137ns | 2137ns | -48.19% |
| warm-container-plusone | 4018ns | 4018ns | -2.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4983ns | base | --- | [1807, 5380] | --- | --- | --- | --- |
| warm-container-kernel | 1820ns | -2883.6ns (-57.9%) | [-3429, -279]ns | [1800, 1867] | YES | 0.0003 | 0.0002 | 0 |
| warm-container-lanes-deferred | 1824ns | -3086.1ns (-61.9%) | [-3405, -172]ns | [1795, 1938] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1832ns | -2216.3ns (-44.5%) | [-3153, -358]ns | [1809, 2009] | YES | 0.0028 | 0.0022 | 0 |
| warm-container-native | 1801ns | -3084.8ns (-61.9%) | [-3419, -14]ns | [1795, 1892] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 3127ns | no significant difference | [-0, +121]ns | [1797, 5754] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4996ns | -68.6% | -64.1% | -63.8% | -64.0% | -47.1% |
| 1 | 2 | 2970ns | -38.3% | -27.6% | +7.3% | -39.4% | +86.5% |
| 1 | 3 | 3935ns | -53.8% | -54.2% | -54.0% | -54.3% | -50.2% |
| 1 | 4 | 5069ns | -56.9% | -63.8% | -60.3% | -60.9% | -21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.277 | moderate- |
| warm-container-kernel | -0.026 | ok |
| warm-container-lanes-deferred | -0.500 | moderate- |
| warm-container-minimum | -0.533 | HIGH- (thermal bounce) |
| warm-container-native | -0.088 | ok |
| warm-container-plusone | -0.754 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 53.8ns | 3971.0ns | 1.4% |  |
| warm-container-kernel | 89.4ns | 1873.0ns | 4.8% |  |
| warm-container-lanes-deferred | 93.0ns | 1907.4ns | 4.9% |  |
| warm-container-minimum | 101.9ns | 2143.5ns | 4.8% |  |
| warm-container-native | 91.1ns | 1885.2ns | 4.8% |  |
| warm-container-plusone | 52.5ns | 3869.3ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1794.1-6208.7 ns)
   1794.1 |########################################
   2014.9 |
   2235.6 |###
   2456.3 |###
   2677.0 |
   2897.8 |
   3118.5 |
   3339.2 |
   3559.9 |###
   3780.7 |
   4001.4 |###
   4222.1 |
   4442.9 |
   4663.6 |
   4884.3 |#############
   5105.0 |######
   5325.8 |################
   5546.5 |
   5767.2 |################
   5987.9 |###
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1563.4-2305.5 ns)
   1563.4 |
   1600.5 |####
   1637.6 |##########################
   1674.7 |
   1711.8 |
   1748.9 |
   1786.0 |########################################
   1823.1 |#############
   1860.2 |#############
   1897.3 |####
   1934.4 |
   1971.5 |#############
   2008.6 |
   2045.7 |####
   2082.8 |#############
   2119.9 |
   2157.0 |
   2194.1 |
   2231.3 |####
   2268.4 |####
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1668.9-2276.1 ns)
   1668.9 |#################
   1699.3 |
   1729.6 |####
   1760.0 |#############
   1790.3 |########################################
   1820.7 |####
   1851.1 |#################
   1881.4 |####
   1911.8 |
   1942.2 |
   1972.5 |#################
   2002.9 |########
   2033.2 |####
   2063.6 |########
   2094.0 |
   2124.3 |
   2154.7 |
   2185.0 |
   2215.4 |####
   2245.8 |####
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1578.8-3356.2 ns)
   1578.8 |#######
   1667.7 |
   1756.6 |########################################
   1845.4 |############
   1934.3 |#######
   2023.2 |
   2112.1 |
   2200.9 |#######
   2289.8 |##
   2378.7 |
   2467.5 |
   2556.4 |
   2645.3 |##
   2734.2 |
   2823.0 |
   2911.9 |
   3000.8 |#####
   3089.6 |
   3178.5 |
   3267.4 |##
  (1 below, 4 above range)

warm-container-native (n=40, range 1786.6-2105.3 ns)
   1786.6 |########################################
   1802.6 |##
   1818.5 |
   1834.4 |######
   1850.4 |##
   1866.3 |
   1882.2 |##
   1898.2 |
   1914.1 |
   1930.0 |##
   1946.0 |##
   1961.9 |##
   1977.8 |######
   1993.8 |##
   2009.7 |
   2025.7 |####
   2041.6 |
   2057.5 |##
   2073.5 |
   2089.4 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1633.6-6516.8 ns)
   1633.6 |########################################
   1877.8 |##
   2121.9 |
   2366.1 |
   2610.3 |##
   2854.4 |
   3098.6 |
   3342.7 |##
   3586.9 |
   3831.1 |
   4075.2 |
   4319.4 |
   4563.5 |
   4807.7 |
   5051.9 |##
   5296.0 |#####
   5540.2 |########
   5784.3 |########
   6028.5 |#####
   6272.7 |###########
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=31.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=55.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
