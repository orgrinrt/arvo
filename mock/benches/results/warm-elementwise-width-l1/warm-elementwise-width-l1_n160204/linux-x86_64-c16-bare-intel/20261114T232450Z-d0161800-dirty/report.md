# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 49% (significant)

warm-container-minimum is -1.66 us (49%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.68)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-headroom, warm-container-plusone} (87% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone} with a 87% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+3% speed for 1.1x steadier)

warm-container-minimum is fastest (1.77 us, CV 11.0%); warm-container-native gives up 2.7% median for 1.1x lower variance (CV 9.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.3x its best-20%

warm-container-plusone's best 20% of batches run at 1.03 us but its worst 20% at 4.43 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1771.0 ns median (-47.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.97x (fastest 1771.0 ns, slowest 3484.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3349ns | 3584ns | 1872ns | 3493ns | 4396ns | base |
| warm-container-kernel | 2099ns | 2054ns | 1694ns | 2095ns | 2516ns | -37.31% |
| warm-container-minimum | 2073ns | 1999ns | 1801ns | 2035ns | 2457ns | -38.11% |
| warm-container-native | 2133ns | 2048ns | 1895ns | 2098ns | 2476ns | -36.31% |
| warm-container-plusone | 3332ns | 3678ns | 1081ns | 3614ns | 4735ns | -0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3158ns | 1775ns | 4124ns | base | 12.969 |
| warm-container-kernel | 1846ns | 1502ns | 2182ns | -41.56% | 22.192 |
| warm-container-minimum | 1826ns | 1596ns | 2142ns | -42.19% | 22.434 |
| warm-container-native | 1881ns | 1682ns | 2161ns | -40.43% | 21.772 |
| warm-container-plusone | 3146ns | 1025ns | 4430ns | -0.40% | 13.021 |

## Performance model

- Peak throughput: **39.959 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.045 | 30.1% |
| warm-container-kernel | 22.515 | 56.3% |
| warm-container-minimum | 23.128 | 57.9% |
| warm-container-native | 22.531 | 56.4% |
| warm-container-plusone | 11.757 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3349ns | 3349ns | base |
| warm-container-kernel | 2099ns | 2099ns | -37.31% |
| warm-container-minimum | 2073ns | 2073ns | -38.11% |
| warm-container-native | 2133ns | 2133ns | -36.31% |
| warm-container-plusone | 3332ns | 3332ns | -0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3400ns | base | --- | [2942, 3572] | --- | --- | --- | --- |
| warm-container-kernel | 1819ns | -1361.1ns (-40.0%) | [-1806, -964]ns | [1772, 1968] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1771ns | -1663.4ns (-48.9%) | [-1882, -727]ns | [1770, 1795] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1818ns | -1509.9ns (-44.4%) | [-1684, -899]ns | [1773, 1967] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3484ns | no significant difference | [-116, +659]ns | [3227, 3903] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3354ns | -39.2% | -52.0% | -45.8% | -37.0% |
| 1 | 2 | 3783ns | -60.3% | -53.2% | -46.8% | +8.1% |
| 1 | 3 | 2668ns | -33.4% | -26.2% | -31.9% | +31.9% |
| 1 | 4 | 3690ns | -47.0% | -52.0% | -53.1% | -5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.679 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.368 | moderate- |
| warm-container-minimum | -0.029 | ok |
| warm-container-native | -0.137 | ok |
| warm-container-plusone | -0.343 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 67.2ns | 3158.3ns | 2.1% |  |
| warm-container-kernel | 92.3ns | 1845.7ns | 5.0% | HIGH |
| warm-container-minimum | 92.3ns | 1825.8ns | 5.1% | HIGH |
| warm-container-native | 92.9ns | 1881.3ns | 4.9% |  |
| warm-container-plusone | 70.1ns | 3145.7ns | 2.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1775.0-4123.8 ns)
   1775.0 |
   1892.4 |
   2009.9 |######
   2127.3 |
   2244.7 |
   2362.2 |#############
   2479.6 |######
   2597.1 |####################
   2714.5 |######
   2831.9 |#############
   2949.4 |######
   3066.8 |######
   3184.3 |####################
   3301.7 |######
   3419.1 |#################################
   3536.6 |#############
   3654.0 |
   3771.5 |########################################
   3888.9 |
   4006.3 |######
  (4 below, 6 above range)

warm-container-kernel (n=40, range 1502.3-2181.9 ns)
   1502.3 |#####
   1536.2 |
   1570.2 |
   1604.2 |###############
   1638.2 |
   1672.2 |#####
   1706.2 |
   1740.1 |########################################
   1774.1 |#####
   1808.1 |###############
   1842.1 |
   1876.1 |
   1910.1 |###############
   1944.0 |####################
   1978.0 |###############
   2012.0 |#####
   2046.0 |
   2080.0 |#####
   2114.0 |
   2147.9 |
  (5 below, 6 above range)

warm-container-minimum (n=40, range 1595.7-2142.0 ns)
   1595.7 |###########
   1623.0 |
   1650.3 |##
   1677.7 |
   1705.0 |
   1732.3 |##
   1759.6 |########################################
   1786.9 |
   1814.2 |####
   1841.5 |####
   1868.9 |
   1896.2 |
   1923.5 |
   1950.8 |#########
   1978.1 |
   2005.4 |
   2032.7 |##
   2060.1 |##
   2087.4 |
   2114.7 |
  (2 below, 4 above range)

warm-container-native (n=40, range 1681.9-2161.1 ns)
   1681.9 |
   1705.9 |###
   1729.8 |###
   1753.8 |########################################
   1777.7 |
   1801.7 |##############
   1825.7 |#######
   1849.6 |###
   1873.6 |
   1897.6 |###
   1921.5 |
   1945.5 |##########
   1969.4 |##########
   1993.4 |
   2017.4 |
   2041.3 |##########
   2065.3 |
   2089.3 |
   2113.2 |###
   2137.2 |###
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1025.1-4429.5 ns)
   1025.1 |############################
   1195.3 |
   1365.5 |
   1535.7 |
   1705.9 |
   1876.2 |
   2046.4 |
   2216.6 |
   2386.8 |
   2557.1 |
   2727.3 |
   2897.5 |
   3067.7 |############################
   3238.0 |###########
   3408.2 |############################
   3578.4 |#####
   3748.6 |#################
   3918.9 |#################
   4089.1 |########################################
   4259.3 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=27.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=40.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.3x (possible bimodal distribution)
