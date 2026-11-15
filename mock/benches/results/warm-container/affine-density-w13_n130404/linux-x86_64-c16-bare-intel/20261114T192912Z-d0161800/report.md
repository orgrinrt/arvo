# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 43% (significant)

warm-container-native is -1.23 us (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (4.47 us) is 2.5x the fastest (1.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-native (1.80 us) and warm-container-lanes-deferred (1.81 us) differ by 0.68%, inside the noise, even though the wider field spreads 149.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader warm-container-native vs stability leader warm-container-lanes-deferred (+1% speed for 1.4x steadier)

warm-container-native is fastest (1.80 us, CV 9.6%); warm-container-lanes-deferred gives up 0.7% median for 1.4x lower variance (CV 7.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 3.8x its best-20%

warm-container-plusone's best 20% of batches run at 1.74 us but its worst 20% at 6.62 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-24 ns, 0.86%)

warm-container-minimum differs from baseline warm-container-headroom by -24 ns (0.86%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1795.5 ns median (-37.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.49x (fastest 1795.5 ns, slowest 4473.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3950ns | 2966ns | 1865ns | 3686ns | 6826ns | base |
| warm-container-kernel | 2125ns | 2035ns | 1969ns | 2071ns | 2445ns | -46.19% |
| warm-container-lanes-deferred | 2072ns | 2034ns | 1856ns | 2062ns | 2317ns | -47.55% |
| warm-container-minimum | 3950ns | 2659ns | 1841ns | 3669ns | 6900ns | -0.01% |
| warm-container-native | 1972ns | 2024ns | 1631ns | 2004ns | 2218ns | -50.08% |
| warm-container-plusone | 4276ns | 4631ns | 1801ns | 4225ns | 6904ns | +8.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3804ns | 1801ns | 6539ns | base | 10.768 |
| warm-container-kernel | 1874ns | 1748ns | 2123ns | -50.73% | 21.856 |
| warm-container-lanes-deferred | 1832ns | 1650ns | 2033ns | -51.85% | 22.363 |
| warm-container-minimum | 3803ns | 1778ns | 6622ns | -0.03% | 10.771 |
| warm-container-native | 1742ns | 1448ns | 1936ns | -54.21% | 23.519 |
| warm-container-plusone | 4117ns | 1739ns | 6617ns | +8.24% | 9.949 |

## Performance model

- Peak throughput: **28.280 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 14.303 | 50.6% |
| warm-container-kernel | 22.637 | 80.0% |
| warm-container-lanes-deferred | 22.659 | 80.1% |
| warm-container-minimum | 15.962 | 56.4% |
| warm-container-native | 22.813 | 80.7% |
| warm-container-plusone | 9.157 | 32.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3950ns | 3950ns | base |
| warm-container-kernel | 2125ns | 2125ns | -46.19% |
| warm-container-lanes-deferred | 2072ns | 2072ns | -47.55% |
| warm-container-minimum | 3950ns | 3950ns | -0.01% |
| warm-container-native | 1972ns | 1972ns | -50.08% |
| warm-container-plusone | 4276ns | 4276ns | +8.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2864ns | base | --- | [1804, 5595] | --- | --- | --- | --- |
| warm-container-kernel | 1809ns | -1029.2ns (-35.9%) | [-3792, -14]ns | [1803, 1831] | YES | 0.0011 | 0.0007 | 0 |
| warm-container-lanes-deferred | 1808ns | -1138.4ns (-39.8%) | [-3787, -141]ns | [1803, 1845] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-minimum | 2566ns | -24.5ns (-0.9%) | [-26, -13]ns | [1779, 5363] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-native | 1796ns | -1233.1ns (-43.1%) | [-3682, -157]ns | [1785, 1820] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4473ns | no significant difference | [-193, +41]ns | [2585, 5366] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4334ns | -58.5% | -60.3% | -21.9% | -65.7% | -39.6% |
| 1 | 2 | 4614ns | -60.4% | -59.8% | -28.2% | -61.2% | -39.7% |
| 1 | 3 | 1808ns | +9.9% | +6.0% | -0.7% | +0.0% | +204.7% |
| 1 | 4 | 3875ns | -53.4% | -53.3% | +26.4% | -52.6% | +15.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.315 | moderate- |
| warm-container-kernel | -0.366 | moderate- |
| warm-container-lanes-deferred | -0.092 | ok |
| warm-container-minimum | -0.488 | moderate- |
| warm-container-native | -0.025 | ok |
| warm-container-plusone | 0.100 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 51.9ns | 3803.7ns | 1.4% |  |
| warm-container-kernel | 90.9ns | 1874.1ns | 4.9% |  |
| warm-container-lanes-deferred | 87.7ns | 1831.6ns | 4.8% |  |
| warm-container-minimum | 51.8ns | 3802.7ns | 1.4% |  |
| warm-container-native | 84.2ns | 1741.6ns | 4.8% |  |
| warm-container-plusone | 56.0ns | 4117.1ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1801.4-6538.9 ns)
   1801.4 |########################################
   2038.3 |
   2275.1 |
   2512.0 |
   2748.9 |#####
   2985.8 |
   3222.7 |
   3459.5 |##
   3696.4 |
   3933.3 |
   4170.2 |
   4407.0 |
   4643.9 |##
   4880.8 |##
   5117.7 |
   5354.5 |#####
   5591.4 |#####
   5828.3 |########
   6065.2 |
   6302.1 |##################
  (4 below, 2 above range)

warm-container-kernel (n=40, range 1747.7-2123.4 ns)
   1747.7 |###
   1766.5 |
   1785.3 |########################################
   1804.1 |########################################
   1822.9 |
   1841.6 |#######
   1860.4 |###
   1879.2 |
   1898.0 |
   1916.8 |
   1935.6 |
   1954.4 |###
   1973.1 |#######
   1991.9 |#######
   2010.7 |###
   2029.5 |###
   2048.3 |###
   2067.1 |
   2085.9 |###
   2104.6 |
  (3 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1649.6-2033.2 ns)
   1649.6 |
   1668.8 |#######
   1688.0 |
   1707.2 |
   1726.3 |
   1745.5 |
   1764.7 |
   1783.9 |#####################
   1803.1 |########################################
   1822.2 |###
   1841.4 |#######
   1860.6 |###
   1879.8 |#######
   1898.9 |
   1918.1 |
   1937.3 |###
   1956.5 |
   1975.6 |
   1994.8 |##############
   2014.0 |###
  (6 below, 3 above range)

warm-container-minimum (n=40, range 1777.5-6622.5 ns)
   1777.5 |########################################
   2019.8 |
   2262.0 |
   2504.3 |
   2746.5 |
   2988.8 |
   3231.0 |##
   3473.3 |
   3715.5 |
   3957.8 |
   4200.0 |
   4442.3 |##
   4684.5 |##
   4926.8 |##
   5169.0 |#######
   5411.2 |##
   5653.5 |##
   5895.7 |##
   6138.0 |#######
   6380.2 |############
  (4 below, 2 above range)

warm-container-native (n=40, range 1448.4-1935.8 ns)
   1448.4 |
   1472.8 |######################
   1497.1 |
   1521.5 |
   1545.9 |
   1570.2 |
   1594.6 |
   1619.0 |########
   1643.3 |####
   1667.7 |########
   1692.1 |
   1716.4 |
   1740.8 |
   1765.2 |#############
   1789.5 |########################################
   1813.9 |##########################
   1838.3 |#################
   1862.6 |
   1887.0 |
   1911.4 |####
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1739.3-6617.4 ns)
   1739.3 |########################################
   1983.2 |
   2227.1 |
   2471.0 |
   2714.9 |
   2958.8 |
   3202.7 |###
   3446.6 |###
   3690.5 |###
   3934.4 |
   4178.3 |#########
   4422.2 |###
   4666.1 |###
   4910.0 |######
   5154.0 |############
   5397.9 |######
   5641.8 |###
   5885.7 |###
   6129.6 |
   6373.5 |##################
  (1 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=54.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=55.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=46.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
