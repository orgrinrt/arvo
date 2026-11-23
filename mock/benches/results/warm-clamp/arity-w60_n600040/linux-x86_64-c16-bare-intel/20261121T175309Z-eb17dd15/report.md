# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader warm-clamp-accfit-dyn vs stability leader warm-clamp-minimum (+8% speed for 2.0x steadier)

warm-clamp-accfit-dyn is fastest (2.23 us, CV 21.3%); warm-clamp-minimum gives up 8.1% median for 2.0x lower variance (CV 10.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.6x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 2.36 us but its worst 20% at 8.47 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2225.6 ns median (-7.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.37x (fastest 2225.6 ns, slowest 3049.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891686 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2605ns | 2654ns | 1925ns | 2629ns | 3214ns | base |
| warm-clamp-accfit | 2717ns | 2578ns | 2243ns | 2634ns | 3437ns | +4.29% |
| warm-clamp-accfit-dyn | 2308ns | 2452ns | 1451ns | 2391ns | 2918ns | -11.39% |
| warm-clamp-head | 3777ns | 3131ns | 2564ns | 2930ns | 7532ns | +44.99% |
| warm-clamp-min-lanes | 4253ns | 2532ns | 2435ns | 3345ns | 8792ns | +63.26% |
| warm-clamp-minimum | 2664ns | 2655ns | 2272ns | 2639ns | 3132ns | +2.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2354ns | 1746ns | 2874ns | base | 3.480 |
| warm-clamp-accfit | 2441ns | 2033ns | 3033ns | +3.68% | 3.357 |
| warm-clamp-accfit-dyn | 2083ns | 1305ns | 2598ns | -11.53% | 3.934 |
| warm-clamp-head | 3670ns | 2497ns | 7295ns | +55.89% | 2.232 |
| warm-clamp-min-lanes | 4109ns | 2361ns | 8466ns | +74.56% | 1.994 |
| warm-clamp-minimum | 2404ns | 2064ns | 2808ns | +2.14% | 3.407 |

## Performance model

- Peak throughput: **6.276 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.399 | 54.2% |
| warm-clamp-accfit | 3.495 | 55.7% |
| warm-clamp-accfit-dyn | 3.681 | 58.6% |
| warm-clamp-head | 2.687 | 42.8% |
| warm-clamp-min-lanes | 3.335 | 53.1% |
| warm-clamp-minimum | 3.405 | 54.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2605ns | 2605ns | base |
| warm-clamp-accfit | 2717ns | 2717ns | +4.29% |
| warm-clamp-accfit-dyn | 2308ns | 2308ns | -11.39% |
| warm-clamp-head | 3777ns | 3777ns | +44.99% |
| warm-clamp-min-lanes | 4253ns | 4253ns | +63.26% |
| warm-clamp-minimum | 2664ns | 2664ns | +2.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2410ns | base | --- | [2227, 2468] | --- | --- | --- | --- |
| warm-clamp-accfit | 2344ns | no significant difference | [-101, +162]ns | [2241, 2413] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 2226ns | -172.1ns (-7.1%) | [-280, -105]ns | [2069, 2315] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-head | 3049ns | +789.3ns (+32.7%) | [+277, +955]ns | [2510, 3060] | YES | 0.0034 | 0.0007 | 0 |
| warm-clamp-min-lanes | 2456ns | +436.6ns (+18.1%) | [+298, +1017]ns | [2415, 2794] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-minimum | 2406ns | no significant difference | [-64, +23]ns | [2309, 2432] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2660ns | +8.3% | -9.4% | -6.0% | -11.2% | -6.6% |
| 1 | 2 | 2555ns | -5.6% | -48.9% | -1.6% | -5.5% | +0.5% |
| 1 | 3 | 2336ns | -7.7% | -8.1% | +31.0% | +19.3% | -3.9% |
| 1 | 4 | 2162ns | +4.0% | +11.5% | +41.7% | +13.2% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.283 | moderate+ |
| warm-clamp-accfit | 0.144 | ok |
| warm-clamp-accfit-dyn | -0.358 | moderate- |
| warm-clamp-head | 0.257 | moderate+ |
| warm-clamp-min-lanes | -0.255 | moderate- |
| warm-clamp-minimum | 0.174 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 72.1ns | 2353.9ns | 3.1% |  |
| warm-clamp-accfit | 81.0ns | 2440.6ns | 3.3% |  |
| warm-clamp-accfit-dyn | 64.9ns | 2082.5ns | 3.1% |  |
| warm-clamp-head | 28.9ns | 3669.5ns | 0.8% |  |
| warm-clamp-min-lanes | 35.3ns | 4109.0ns | 0.9% |  |
| warm-clamp-minimum | 75.5ns | 2404.3ns | 3.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1746.3-2873.9 ns)
   1746.3 |
   1802.6 |
   1859.0 |
   1915.4 |#####
   1971.8 |
   2028.2 |######################
   2084.6 |###########
   2140.9 |#####
   2197.3 |############################
   2253.7 |###########
   2310.1 |
   2366.5 |########################################
   2422.8 |###########
   2479.2 |###########
   2535.6 |
   2592.0 |#################
   2648.4 |###########
   2704.7 |#####
   2761.1 |
   2817.5 |#####
  (2 below, 5 above range)

warm-clamp-accfit (n=40, range 2033.3-3033.2 ns)
   2033.3 |####################
   2083.3 |#####
   2133.3 |
   2183.3 |########################################
   2233.2 |####################
   2283.2 |##########
   2333.2 |
   2383.2 |###################################
   2433.2 |#####
   2483.2 |
   2533.2 |
   2583.2 |#####
   2633.2 |##########
   2683.2 |
   2733.2 |
   2783.2 |###############
   2833.2 |#####
   2883.2 |#####
   2933.2 |
   2983.2 |
  (1 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1305.3-2598.5 ns)
   1305.3 |#############
   1369.9 |####
   1434.6 |
   1499.2 |
   1563.9 |
   1628.6 |
   1693.2 |
   1757.9 |
   1822.5 |
   1887.2 |####
   1951.9 |
   2016.5 |#################
   2081.2 |
   2145.8 |
   2210.5 |########################################
   2275.2 |#############
   2339.8 |
   2404.5 |##########################
   2469.1 |####
   2533.8 |
  (6 below, 6 above range)

warm-clamp-head (n=40, range 2496.8-7295.4 ns)
   2496.8 |###################################
   2736.8 |
   2976.7 |########################################
   3216.6 |
   3456.5 |
   3696.5 |
   3936.4 |
   4176.3 |
   4416.2 |##
   4656.2 |
   4896.1 |
   5136.0 |
   5375.9 |
   5615.9 |
   5855.8 |
   6095.7 |
   6335.7 |
   6575.6 |
   6815.5 |##
   7055.4 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 2360.6-8465.6 ns)
   2360.6 |########################################
   2665.9 |################
   2971.1 |
   3276.4 |
   3581.6 |
   3886.9 |
   4192.1 |
   4497.3 |
   4802.6 |
   5107.8 |
   5413.1 |
   5718.3 |
   6023.6 |####
   6328.8 |
   6634.1 |
   6939.3 |##
   7244.6 |#######
   7549.8 |
   7855.1 |####
   8160.3 |##
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 2064.5-2807.7 ns)
   2064.5 |######################
   2101.6 |#####
   2138.8 |###########
   2175.9 |
   2213.1 |######################
   2250.3 |#####
   2287.4 |######################
   2324.6 |
   2361.8 |###########
   2398.9 |########################################
   2436.1 |#####
   2473.2 |
   2510.4 |######################
   2547.6 |#####
   2584.7 |#####
   2621.9 |#####
   2659.0 |
   2696.2 |
   2733.4 |
   2770.5 |#################
  (1 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: CV=22.8% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=57.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=62.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
