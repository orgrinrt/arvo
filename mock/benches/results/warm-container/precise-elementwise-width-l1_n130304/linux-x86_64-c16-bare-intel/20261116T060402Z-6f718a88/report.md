# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 11% faster than the next best (warm-container-native)

warm-container-minimum (1.85 us) leads warm-container-native (2.06 us) by 11%, a clear separation rather than a photo finish. CV 29.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-plusone beats baseline by 29% (significant)

warm-container-plusone is -682 ns (29%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is inconsistent: worst-20% is 3.8x its best-20%

warm-container-headroom's best 20% of batches run at 1.94 us but its worst 20% at 7.37 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1851.9 ns median (-22.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.30x (fastest 1851.9 ns, slowest 2398.6 ns)

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
| Counter frequency | 2112840787 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4083ns | 2444ns | 1989ns | 3599ns | 7627ns | base |
| warm-container-kernel | 2573ns | 2633ns | 2145ns | 2594ns | 2941ns | -36.97% |
| warm-container-minimum | 2269ns | 2083ns | 1640ns | 2168ns | 3200ns | -44.42% |
| warm-container-native | 2431ns | 2313ns | 2081ns | 2343ns | 3045ns | -40.46% |
| warm-container-plusone | 2352ns | 2313ns | 1957ns | 2284ns | 2952ns | -42.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3966ns | 1937ns | 7374ns | base | 10.329 |
| warm-container-kernel | 2339ns | 1966ns | 2626ns | -41.02% | 17.512 |
| warm-container-minimum | 2007ns | 1454ns | 2834ns | -49.39% | 20.409 |
| warm-container-native | 2151ns | 1852ns | 2673ns | -45.75% | 19.040 |
| warm-container-plusone | 2080ns | 1735ns | 2597ns | -47.55% | 19.693 |

## Performance model

- Peak throughput: **28.168 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.208 | 61.1% |
| warm-container-kernel | 17.076 | 60.6% |
| warm-container-minimum | 22.117 | 78.5% |
| warm-container-native | 19.901 | 70.7% |
| warm-container-plusone | 19.898 | 70.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4083ns | 4083ns | base |
| warm-container-kernel | 2573ns | 2573ns | -36.97% |
| warm-container-minimum | 2269ns | 2269ns | -44.42% |
| warm-container-native | 2431ns | 2431ns | -40.46% |
| warm-container-plusone | 2352ns | 2352ns | -42.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2380ns | base | --- | [2368, 5474] | --- | --- | --- | --- |
| warm-container-kernel | 2399ns | no significant difference | [-2987, +24]ns | [2261, 2475] | no | 0.0807 | 0.0807 | 0 |
| warm-container-minimum | 1852ns | -531.5ns (-22.3%) | [-3407, -337]ns | [1851, 1974] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2058ns | -519.3ns (-21.8%) | [-2909, -365]ns | [1940, 2191] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2058ns | -681.8ns (-28.6%) | [-3484, -179]ns | [1874, 2097] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3472ns | -28.6% | -45.3% | -28.5% | -46.0% |
| 1 | 2 | 2382ns | -16.8% | +11.5% | -10.5% | +2.9% |
| 1 | 3 | 2709ns | -8.4% | -31.7% | -29.5% | -24.0% |
| 1 | 4 | 2376ns | +3.9% | -21.2% | -20.9% | -25.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.304 | moderate- |
| warm-container-kernel | -0.432 | moderate- |
| warm-container-minimum | -0.401 | moderate- |
| warm-container-native | 0.209 | moderate+ |
| warm-container-plusone | -0.240 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.6ns | 3965.5ns | 1.0% |  |
| warm-container-kernel | 83.0ns | 2339.0ns | 3.5% |  |
| warm-container-minimum | 95.3ns | 2007.0ns | 4.8% |  |
| warm-container-native | 104.3ns | 2151.3ns | 4.8% |  |
| warm-container-plusone | 98.8ns | 2079.9ns | 4.8% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1937.3-7374.3 ns)
   1937.3 |###############
   2209.2 |########################################
   2481.0 |
   2752.9 |
   3024.7 |###
   3296.6 |###
   3568.4 |
   3840.3 |
   4112.1 |
   4384.0 |
   4655.8 |
   4927.7 |###
   5199.5 |
   5471.4 |
   5743.2 |###
   6015.1 |######
   6286.9 |######
   6558.8 |###
   6830.6 |#########
   7102.5 |######
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1966.0-2626.4 ns)
   1966.0 |
   1999.0 |###
   2032.0 |
   2065.0 |#######
   2098.1 |###
   2131.1 |
   2164.1 |
   2197.1 |
   2230.2 |#############################
   2263.2 |###
   2296.2 |###
   2329.2 |###
   2362.2 |
   2395.3 |
   2428.3 |
   2461.3 |########################################
   2494.3 |#######
   2527.4 |##############
   2560.4 |
   2593.4 |
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1454.2-2833.7 ns)
   1454.2 |
   1523.1 |
   1592.1 |
   1661.1 |#####
   1730.1 |##
   1799.0 |########################################
   1868.0 |#####
   1937.0 |##########
   2006.0 |############
   2074.9 |
   2143.9 |
   2212.9 |##
   2281.9 |
   2350.8 |
   2419.8 |
   2488.8 |##
   2557.8 |
   2626.7 |#####
   2695.7 |
   2764.7 |##
  (2 below, 3 above range)

warm-container-native (n=40, range 1852.2-2673.1 ns)
   1852.2 |########################################
   1893.3 |#################
   1934.3 |#####
   1975.4 |
   2016.4 |############################
   2057.5 |######################
   2098.5 |#####
   2139.5 |#####
   2180.6 |#####
   2221.6 |
   2262.7 |#####
   2303.7 |######################
   2344.8 |###########
   2385.8 |
   2426.8 |#####
   2467.9 |
   2508.9 |
   2550.0 |
   2591.0 |#####
   2632.1 |#####
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1735.4-2597.2 ns)
   1735.4 |
   1778.5 |
   1821.6 |########################################
   1864.7 |########
   1907.8 |########
   1950.9 |
   1994.0 |
   2037.0 |###############################
   2080.1 |#################
   2123.2 |
   2166.3 |
   2209.4 |
   2252.5 |
   2295.6 |#################
   2338.7 |####
   2381.7 |####
   2424.8 |
   2467.9 |
   2511.0 |####
   2554.1 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-minimum**: CV=27.2% (high variance, measurements may be unstable)
