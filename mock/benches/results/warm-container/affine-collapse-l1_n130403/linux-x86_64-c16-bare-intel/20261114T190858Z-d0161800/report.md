# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 34% (significant)

warm-container-lanes-deferred is -927 ns (34%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.9x slower than the field

warm-container-minimum (5.21 us) is 2.9x the fastest (1.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-native) are a dead heat (<1%)

warm-container-lanes-deferred (1.81 us) and warm-container-native (1.81 us) differ by 0.12%, inside the noise, even though the wider field spreads 187.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.75)

warm-container-native's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-plusone is inconsistent: worst-20% is 3.9x its best-20%

warm-container-plusone's best 20% of batches run at 1.62 us but its worst 20% at 6.33 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1809.5 ns median (-34.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.88x (fastest 1809.5 ns, slowest 5209.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3942ns | 2854ns | 1855ns | 3626ns | 6979ns | base |
| warm-container-kernel | 2187ns | 2083ns | 1739ns | 2133ns | 2800ns | -44.52% |
| warm-container-lanes-deferred | 2046ns | 2036ns | 1854ns | 2033ns | 2275ns | -48.11% |
| warm-container-minimum | 4482ns | 5396ns | 1844ns | 4537ns | 6957ns | +13.69% |
| warm-container-native | 2109ns | 2040ns | 1873ns | 2072ns | 2455ns | -46.51% |
| warm-container-plusone | 3901ns | 3946ns | 1679ns | 3741ns | 6604ns | -1.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3795ns | 1792ns | 6676ns | base | 8.635 |
| warm-container-kernel | 1930ns | 1546ns | 2445ns | -49.14% | 16.977 |
| warm-container-lanes-deferred | 1808ns | 1648ns | 1991ns | -52.35% | 18.120 |
| warm-container-minimum | 4316ns | 1780ns | 6660ns | +13.74% | 7.592 |
| warm-container-native | 1860ns | 1664ns | 2132ns | -51.00% | 17.622 |
| warm-container-plusone | 3755ns | 1621ns | 6327ns | -1.06% | 8.727 |

## Performance model

- Peak throughput: **21.193 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.889 | 56.1% |
| warm-container-kernel | 17.874 | 84.3% |
| warm-container-lanes-deferred | 18.108 | 85.4% |
| warm-container-minimum | 6.291 | 29.7% |
| warm-container-native | 18.087 | 85.3% |
| warm-container-plusone | 8.600 | 40.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3942ns | 3942ns | base |
| warm-container-kernel | 2187ns | 2187ns | -44.52% |
| warm-container-lanes-deferred | 2046ns | 2046ns | -48.11% |
| warm-container-minimum | 4482ns | 4482ns | +13.69% |
| warm-container-native | 2109ns | 2109ns | -46.51% |
| warm-container-plusone | 3901ns | 3901ns | -1.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2756ns | base | --- | [1793, 5403] | --- | --- | --- | --- |
| warm-container-kernel | 1833ns | no significant difference | [-3508, +6]ns | [1807, 2005] | no | 0.1923 | 0.1539 | 0 |
| warm-container-lanes-deferred | 1810ns | -927.2ns (-33.6%) | [-3594, -147]ns | [1808, 1811] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-minimum | 5209ns | no significant difference | [-15, +611]ns | [1804, 6125] | no | 0.8746 | 0.8746 | 0 |
| warm-container-native | 1812ns | -927.1ns (-33.6%) | [-3417, -150]ns | [1807, 1850] | YES | 0.0037 | 0.0022 | 0 |
| warm-container-plusone | 3810ns | -131.0ns (-4.8%) | [-310, -12]ns | [1786, 5256] | YES | 0.0005 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3840ns | -40.3% | -52.9% | +35.6% | -53.0% | -53.5% |
| 1 | 2 | 5418ns | -66.6% | -66.6% | -15.6% | -65.9% | -5.5% |
| 1 | 3 | 1793ns | +8.7% | +1.9% | +216.3% | +0.8% | +64.5% |
| 1 | 4 | 4986ns | -69.3% | -64.3% | -64.2% | -63.3% | -26.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.701 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.191 | ok |
| warm-container-lanes-deferred | -0.433 | moderate- |
| warm-container-minimum | -0.312 | moderate- |
| warm-container-native | -0.747 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.627 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 51.7ns | 3794.9ns | 1.4% |  |
| warm-container-kernel | 92.3ns | 1930.2ns | 4.8% |  |
| warm-container-lanes-deferred | 86.4ns | 1808.3ns | 4.8% |  |
| warm-container-minimum | 58.8ns | 4316.2ns | 1.4% |  |
| warm-container-native | 88.4ns | 1859.5ns | 4.8% |  |
| warm-container-plusone | 52.2ns | 3754.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1792.1-6675.8 ns)
   1792.1 |########################################
   2036.3 |##
   2280.5 |
   2524.6 |##
   2768.8 |##
   3013.0 |
   3257.2 |
   3501.4 |
   3745.6 |
   3989.8 |##
   4233.9 |
   4478.1 |
   4722.3 |##
   4966.5 |#####
   5210.7 |##
   5454.9 |##########
   5699.0 |#####
   5943.2 |#####
   6187.4 |##
   6431.6 |##########
  (2 below, 1 above range)

warm-container-kernel (n=40, range 1546.2-2444.7 ns)
   1546.2 |####
   1591.1 |
   1636.0 |############
   1680.9 |####
   1725.9 |
   1770.8 |########################################
   1815.7 |####
   1860.7 |########
   1905.6 |
   1950.5 |########
   1995.4 |####################
   2040.4 |
   2085.3 |####
   2130.2 |########
   2175.1 |
   2220.1 |########
   2265.0 |
   2309.9 |####
   2354.8 |
   2399.8 |####
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1648.0-1990.9 ns)
   1648.0 |
   1665.2 |##
   1682.3 |##
   1699.4 |
   1716.6 |##
   1733.7 |
   1750.9 |##
   1768.0 |
   1785.2 |##
   1802.3 |########################################
   1819.4 |
   1836.6 |####
   1853.7 |
   1870.9 |
   1888.0 |
   1905.1 |##
   1922.3 |##
   1939.4 |
   1956.6 |
   1973.7 |##
  (7 below, 5 above range)

warm-container-minimum (n=40, range 1779.6-6659.7 ns)
   1779.6 |########################################
   2023.6 |
   2267.6 |
   2511.6 |########
   2755.6 |
   2999.6 |
   3243.6 |
   3487.6 |
   3731.7 |
   3975.7 |
   4219.7 |
   4463.7 |####
   4707.7 |
   4951.7 |########
   5195.7 |####
   5439.7 |########
   5683.7 |####
   5927.7 |########
   6171.7 |#############
   6415.7 |###################################
  (6 below, 3 above range)

warm-container-native (n=40, range 1663.7-2131.9 ns)
   1663.7 |
   1687.1 |##
   1710.6 |
   1734.0 |
   1757.4 |##
   1780.8 |#####
   1804.2 |########################################
   1827.6 |########
   1851.0 |#####
   1874.4 |##
   1897.8 |
   1921.2 |##
   1944.6 |
   1968.0 |
   1991.4 |#################
   2014.9 |
   2038.3 |
   2061.7 |
   2085.1 |
   2108.5 |
  (6 below, 3 above range)

warm-container-plusone (n=40, range 1620.9-6326.7 ns)
   1620.9 |########################################
   1856.2 |
   2091.5 |
   2326.8 |
   2562.1 |##
   2797.4 |
   3032.6 |
   3267.9 |
   3503.2 |##
   3738.5 |
   3973.8 |##
   4209.1 |
   4444.4 |
   4679.6 |##
   4914.9 |########
   5150.2 |#####
   5385.5 |#####
   5620.8 |###########
   5856.1 |
   6091.4 |#####
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=55.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=49.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
