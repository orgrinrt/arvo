# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 24% (significant)

warm-container-minimum is -570 ns (24%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.50)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-headroom is inconsistent: worst-20% is 3.7x its best-20%

warm-container-headroom's best 20% of batches run at 1.94 us but its worst 20% at 7.09 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 1854.8 ns median (-21.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.34x (fastest 1854.8 ns, slowest 2487.7 ns)

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
| warm-container-headroom | 3783ns | 2437ns | 1993ns | 3197ns | 7331ns | base |
| warm-container-kernel | 2680ns | 2709ns | 2409ns | 2678ns | 2957ns | -29.16% |
| warm-container-minimum | 2260ns | 2245ns | 1778ns | 2197ns | 2930ns | -40.27% |
| warm-container-native | 2243ns | 2212ns | 1984ns | 2212ns | 2596ns | -40.71% |
| warm-container-plusone | 2317ns | 2089ns | 1712ns | 2183ns | 3322ns | -38.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3676ns | 1942ns | 7093ns | base | 11.143 |
| warm-container-kernel | 2449ns | 2208ns | 2667ns | -33.38% | 16.727 |
| warm-container-minimum | 2002ns | 1585ns | 2580ns | -45.54% | 20.460 |
| warm-container-native | 1986ns | 1769ns | 2263ns | -45.97% | 20.623 |
| warm-container-plusone | 2055ns | 1527ns | 2944ns | -44.10% | 19.933 |

## Performance model

- Peak throughput: **26.827 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.250 | 64.3% |
| warm-container-kernel | 16.465 | 61.4% |
| warm-container-minimum | 20.747 | 77.3% |
| warm-container-native | 20.807 | 77.6% |
| warm-container-plusone | 22.083 | 82.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3783ns | 3783ns | base |
| warm-container-kernel | 2680ns | 2680ns | -29.16% |
| warm-container-minimum | 2260ns | 2260ns | -40.27% |
| warm-container-native | 2243ns | 2243ns | -40.71% |
| warm-container-plusone | 2317ns | 2317ns | -38.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2375ns | base | --- | [1948, 4871] | --- | --- | --- | --- |
| warm-container-kernel | 2488ns | no significant difference | [-2414, +271]ns | [2462, 2489] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 1974ns | -569.6ns (-24.0%) | [-2704, -400]ns | [1811, 2055] | YES | 0.0221 | 0.0166 | 0 |
| warm-container-native | 1969ns | -324.1ns (-13.6%) | [-2873, -216]ns | [1873, 2057] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1855ns | -519.7ns (-21.9%) | [-2056, -403]ns | [1854, 2060] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3360ns | -32.4% | -53.1% | -38.9% | -44.8% |
| 1 | 2 | 5160ns | -52.0% | -60.2% | -63.7% | -48.1% |
| 1 | 3 | 1943ns | +28.1% | +25.7% | -4.7% | -20.5% |
| 1 | 4 | 2375ns | +4.4% | -22.1% | -13.0% | -20.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.183 | ok |
| warm-container-kernel | -0.048 | ok |
| warm-container-minimum | -0.141 | ok |
| warm-container-native | -0.254 | moderate- |
| warm-container-plusone | -0.500 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.3ns | 3675.9ns | 1.0% |  |
| warm-container-kernel | 86.8ns | 2448.7ns | 3.5% |  |
| warm-container-minimum | 97.2ns | 2001.9ns | 4.9% |  |
| warm-container-native | 94.2ns | 1986.1ns | 4.7% |  |
| warm-container-plusone | 96.3ns | 2054.9ns | 4.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1941.8-7093.3 ns)
   1941.8 |########################################
   2199.4 |##################
   2457.0 |
   2714.5 |
   2972.1 |
   3229.7 |
   3487.3 |
   3744.8 |
   4002.4 |##
   4260.0 |
   4517.6 |##
   4775.1 |##
   5032.7 |##
   5290.3 |##########
   5547.9 |#####
   5805.4 |##
   6063.0 |##
   6320.6 |##
   6578.2 |
   6835.7 |
  (2 below, 3 above range)

warm-container-kernel (n=40, range 2208.3-2667.2 ns)
   2208.3 |
   2231.2 |########
   2254.2 |############
   2277.1 |########
   2300.1 |####
   2323.0 |
   2346.0 |########
   2368.9 |
   2391.9 |
   2414.8 |
   2437.8 |####
   2460.7 |################
   2483.6 |########################################
   2506.6 |
   2529.5 |########################
   2552.5 |####
   2575.4 |####
   2598.4 |
   2621.3 |
   2644.3 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 1584.6-2579.6 ns)
   1584.6 |
   1634.3 |#########################
   1684.1 |##########
   1733.8 |#####
   1783.6 |
   1833.3 |#########################
   1883.1 |#####
   1932.8 |
   1982.6 |
   2032.3 |########################################
   2082.1 |#####
   2131.8 |
   2181.6 |
   2231.3 |#####
   2281.1 |####################
   2330.8 |#####
   2380.6 |
   2430.3 |
   2480.1 |#####
   2529.8 |
  (6 below, 4 above range)

warm-container-native (n=40, range 1768.7-2262.8 ns)
   1768.7 |####
   1793.4 |
   1818.1 |
   1842.8 |####################################
   1867.5 |
   1892.2 |################
   1916.9 |####
   1941.6 |####
   1966.3 |####
   1991.0 |
   2015.8 |
   2040.5 |########################################
   2065.2 |####
   2089.9 |####
   2114.6 |####
   2139.3 |
   2164.0 |
   2188.7 |
   2213.4 |####
   2238.1 |
  (4 below, 5 above range)

warm-container-plusone (n=40, range 1526.8-2944.4 ns)
   1526.8 |##############
   1597.7 |###
   1668.6 |###
   1739.5 |###
   1810.4 |########################################
   1881.2 |##############
   1952.1 |
   2023.0 |##################
   2093.9 |
   2164.7 |
   2235.6 |
   2306.5 |#######
   2377.4 |###
   2448.3 |
   2519.1 |###
   2590.0 |###
   2660.9 |###
   2731.8 |
   2802.6 |
   2873.5 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=59.3% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=25.5% (high variance, measurements may be unstable)
