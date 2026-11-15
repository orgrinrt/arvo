# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 27% (significant)

warm-container-kernel is -561 ns (27%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.76)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-minimum is inconsistent: worst-20% is 4.7x its best-20%

warm-container-minimum's best 20% of batches run at 1.54 us but its worst 20% at 7.25 us (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-16 ns, 0.77%)

warm-container-plusone differs from baseline warm-container-headroom by -16 ns (0.77%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1696.2 ns median (-19.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.30x (fastest 1696.2 ns, slowest 2208.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3800ns | 2206ns | 1815ns | 3434ns | 6885ns | base |
| warm-container-kernel | 1917ns | 1924ns | 1701ns | 1906ns | 2169ns | -49.55% |
| warm-container-lanes-deferred | 2213ns | 2108ns | 1902ns | 2139ns | 2745ns | -41.78% |
| warm-container-minimum | 4092ns | 2287ns | 1593ns | 3767ns | 7568ns | +7.68% |
| warm-container-native | 2214ns | 2085ns | 1893ns | 2147ns | 2737ns | -41.74% |
| warm-container-plusone | 3821ns | 2252ns | 1842ns | 3486ns | 6807ns | +0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3635ns | 1753ns | 6529ns | base | 6.761 |
| warm-container-kernel | 1680ns | 1500ns | 1864ns | -53.78% | 14.627 |
| warm-container-lanes-deferred | 1956ns | 1689ns | 2413ns | -46.18% | 12.561 |
| warm-container-minimum | 3938ns | 1538ns | 7249ns | +8.32% | 6.241 |
| warm-container-native | 1953ns | 1683ns | 2400ns | -46.26% | 12.581 |
| warm-container-plusone | 3665ns | 1779ns | 6487ns | +0.82% | 6.705 |

## Performance model

- Peak throughput: **16.385 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.729 | 71.6% |
| warm-container-kernel | 14.489 | 88.4% |
| warm-container-lanes-deferred | 13.320 | 81.3% |
| warm-container-minimum | 11.128 | 67.9% |
| warm-container-native | 13.322 | 81.3% |
| warm-container-plusone | 11.391 | 69.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3800ns | 3800ns | base |
| warm-container-kernel | 1917ns | 1917ns | -49.55% |
| warm-container-lanes-deferred | 2213ns | 2213ns | -41.78% |
| warm-container-minimum | 4092ns | 4092ns | +7.68% |
| warm-container-native | 2214ns | 2214ns | -41.74% |
| warm-container-plusone | 3821ns | 3821ns | +0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2095ns | base | --- | [1862, 5236] | --- | --- | --- | --- |
| warm-container-kernel | 1696ns | -561.3ns (-26.8%) | [-3506, -259]ns | [1641, 1753] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1845ns | -296.0ns (-14.1%) | [-3448, -1]ns | [1809, 2004] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-container-minimum | 2208ns | no significant difference | [-18, +288]ns | [1784, 5995] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 1845ns | no significant difference | [-3078, +5]ns | [1807, 2008] | no | 0.1009 | 0.0807 | 0 |
| warm-container-plusone | 2157ns | -16.2ns (-0.8%) | [-29, -7]ns | [1987, 5092] | YES (adj: no) | 0.0962 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2006ns | -23.4% | +12.4% | -27.1% | -16.2% | -0.9% |
| 1 | 2 | 3636ns | -54.4% | -52.7% | -13.3% | -37.8% | +35.5% |
| 1 | 3 | 1802ns | -2.0% | +4.2% | +54.4% | +11.4% | +25.8% |
| 1 | 4 | 5207ns | -66.3% | -64.6% | -19.5% | -65.3% | -24.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.523 | HIGH- (thermal bounce) |
| warm-container-kernel | 0.225 | moderate+ |
| warm-container-lanes-deferred | -0.340 | moderate- |
| warm-container-minimum | -0.143 | ok |
| warm-container-native | -0.365 | moderate- |
| warm-container-plusone | -0.763 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 54.8ns | 3635.1ns | 1.5% |  |
| warm-container-kernel | 85.7ns | 1680.1ns | 5.1% | HIGH |
| warm-container-lanes-deferred | 94.2ns | 1956.5ns | 4.8% |  |
| warm-container-minimum | 53.4ns | 3937.7ns | 1.4% |  |
| warm-container-native | 93.6ns | 1953.4ns | 4.8% |  |
| warm-container-plusone | 53.4ns | 3665.1ns | 1.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1753.0-6529.4 ns)
   1753.0 |########################################
   1991.8 |#################
   2230.7 |
   2469.5 |
   2708.3 |##
   2947.1 |
   3185.9 |##
   3424.7 |
   3663.6 |
   3902.4 |
   4141.2 |##
   4380.0 |
   4618.8 |
   4857.6 |#####
   5096.5 |
   5335.3 |###########
   5574.1 |
   5812.9 |#####
   6051.7 |##
   6290.5 |##############
  (1 below, 2 above range)

warm-container-kernel (n=40, range 1499.9-1864.1 ns)
   1499.9 |######
   1518.1 |####################
   1536.3 |########################################
   1554.5 |
   1572.7 |
   1591.0 |######
   1609.2 |
   1627.4 |######
   1645.6 |######
   1663.8 |#############
   1682.0 |#################################
   1700.2 |#############
   1718.4 |######
   1736.6 |#################################
   1754.8 |##########################
   1773.0 |
   1791.3 |
   1809.5 |
   1827.7 |#############
   1845.9 |
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1689.3-2413.0 ns)
   1689.3 |
   1725.5 |
   1761.7 |######
   1797.9 |########################################
   1834.0 |###
   1870.2 |###
   1906.4 |###
   1942.6 |###
   1978.8 |##################
   2015.0 |######
   2051.2 |###
   2087.3 |
   2123.5 |
   2159.7 |
   2195.9 |
   2232.1 |############
   2268.3 |
   2304.4 |
   2340.6 |
   2376.8 |
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1538.1-7249.1 ns)
   1538.1 |########################################
   1823.7 |
   2109.2 |
   2394.8 |##
   2680.3 |
   2965.9 |
   3251.4 |
   3537.0 |##
   3822.5 |
   4108.1 |
   4393.6 |##
   4679.2 |##
   4964.7 |
   5250.3 |
   5535.8 |##
   5821.4 |#####
   6106.9 |
   6392.5 |##############
   6678.0 |##
   6963.6 |##############
  (6 below, 2 above range)

warm-container-native (n=40, range 1682.8-2400.2 ns)
   1682.8 |###
   1718.7 |###
   1754.6 |###
   1790.4 |########################################
   1826.3 |##############
   1862.2 |
   1898.0 |###
   1933.9 |
   1969.8 |
   2005.7 |#####################
   2041.5 |###
   2077.4 |
   2113.3 |###
   2149.1 |
   2185.0 |
   2220.9 |
   2256.8 |##########
   2292.6 |###
   2328.5 |###
   2364.4 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 1778.8-6486.7 ns)
   1778.8 |########################################
   2014.2 |
   2249.6 |##
   2485.0 |
   2720.4 |##
   2955.8 |
   3191.2 |
   3426.6 |
   3662.0 |
   3897.3 |##
   4132.7 |
   4368.1 |
   4603.5 |##
   4838.9 |#####
   5074.3 |##
   5309.7 |#####
   5545.1 |
   5780.5 |########
   6015.9 |##
   6251.3 |##############
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=55.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=61.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=54.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.6x (possible bimodal distribution)
