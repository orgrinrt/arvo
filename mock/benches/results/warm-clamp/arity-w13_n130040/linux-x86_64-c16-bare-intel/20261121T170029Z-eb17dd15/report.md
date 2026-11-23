# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 34% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (1.68 us) leads warm-clamp-accfit-dyn (2.25 us) by 34%, a clear separation rather than a photo finish. CV 17.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 51% (significant)

warm-clamp-head is -1.90 us (51%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.6x slower than the field

warm-clamp-min-lanes (4.44 us) is 2.6x the fastest (1.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.30 us but its worst 20% at 5.24 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 1684.2 ns median (-54.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.64x (fastest 1684.2 ns, slowest 4443.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761915 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3548ns | 3920ns | 1631ns | 3877ns | 4481ns | base |
| warm-clamp-accfit | 2535ns | 2481ns | 2277ns | 2513ns | 2861ns | -28.55% |
| warm-clamp-accfit-dyn | 2532ns | 2480ns | 2290ns | 2497ns | 2879ns | -28.65% |
| warm-clamp-head | 2003ns | 1953ns | 1579ns | 1976ns | 2508ns | -43.55% |
| warm-clamp-min-lanes | 3913ns | 4616ns | 1613ns | 4036ns | 5842ns | +10.27% |
| warm-clamp-minimum | 3303ns | 4096ns | 1352ns | 3223ns | 5492ns | -6.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3354ns | 1546ns | 4206ns | base | 2.443 |
| warm-clamp-accfit | 2292ns | 2068ns | 2570ns | -31.66% | 3.574 |
| warm-clamp-accfit-dyn | 2289ns | 2081ns | 2577ns | -31.75% | 3.579 |
| warm-clamp-head | 1739ns | 1369ns | 2180ns | -48.14% | 4.710 |
| warm-clamp-min-lanes | 3756ns | 1552ns | 5586ns | +12.00% | 2.181 |
| warm-clamp-minimum | 3168ns | 1300ns | 5238ns | -5.55% | 2.586 |

## Performance model

- Peak throughput: **6.300 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.204 | 35.0% |
| warm-clamp-accfit | 3.638 | 57.7% |
| warm-clamp-accfit-dyn | 3.638 | 57.7% |
| warm-clamp-head | 4.864 | 77.2% |
| warm-clamp-min-lanes | 1.844 | 29.3% |
| warm-clamp-minimum | 2.080 | 33.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3548ns | 3548ns | base |
| warm-clamp-accfit | 2535ns | 2535ns | -28.55% |
| warm-clamp-accfit-dyn | 2532ns | 2532ns | -28.65% |
| warm-clamp-head | 2003ns | 2003ns | -43.55% |
| warm-clamp-min-lanes | 3913ns | 3913ns | +10.27% |
| warm-clamp-minimum | 3303ns | 3303ns | -6.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3717ns | base | --- | [3428, 3970] | --- | --- | --- | --- |
| warm-clamp-accfit | 2252ns | -1391.9ns (-37.4%) | [-1599, -1085]ns | [2251, 2271] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2252ns | -1353.2ns (-36.4%) | [-1561, -1150]ns | [2251, 2260] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1684ns | -1896.5ns (-51.0%) | [-2034, -1769]ns | [1601, 1824] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 4444ns | +573.6ns (+15.4%) | [+366, +799]ns | [3170, 4875] | YES | 0.0008 | 0.0007 | 0 |
| warm-clamp-minimum | 3939ns | no significant difference | [-162, +419]ns | [1311, 4436] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3833ns | -43.3% | -41.2% | -47.7% | +18.2% | -57.6% |
| 1 | 2 | 3399ns | -31.9% | -33.8% | -52.9% | +45.1% | +25.5% |
| 1 | 3 | 3798ns | -40.7% | -34.8% | -54.0% | +9.6% | +8.0% |
| 1 | 4 | 3848ns | -40.7% | -41.5% | -62.0% | -6.0% | -33.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.373 | moderate- |
| warm-clamp-accfit | -0.477 | moderate- |
| warm-clamp-accfit-dyn | -0.419 | moderate- |
| warm-clamp-head | -0.283 | moderate- |
| warm-clamp-min-lanes | 0.157 | ok |
| warm-clamp-minimum | -0.244 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 69.5ns | 3353.9ns | 2.1% |  |
| warm-clamp-accfit | 87.7ns | 2292.2ns | 3.8% |  |
| warm-clamp-accfit-dyn | 86.4ns | 2289.1ns | 3.8% |  |
| warm-clamp-head | 142.2ns | 1739.2ns | 8.2% | HIGH |
| warm-clamp-min-lanes | 53.9ns | 3756.4ns | 1.4% |  |
| warm-clamp-minimum | 47.1ns | 3167.7ns | 1.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1546.1-4206.0 ns)
   1546.1 |###
   1679.1 |
   1812.1 |
   1945.1 |
   2078.1 |###
   2211.1 |
   2344.0 |
   2477.0 |
   2610.0 |
   2743.0 |
   2876.0 |###
   3009.0 |#######
   3142.0 |#######
   3275.0 |###
   3408.0 |##########
   3541.0 |##########
   3674.0 |##############
   3807.0 |#######
   3940.0 |##########
   4073.0 |########################################
  (5 below, 1 above range)

warm-clamp-accfit (n=40, range 2067.9-2569.6 ns)
   2067.9 |##
   2093.0 |####
   2118.1 |
   2143.2 |
   2168.3 |
   2193.3 |
   2218.4 |
   2243.5 |########################################
   2268.6 |##
   2293.7 |####
   2318.8 |##
   2343.8 |
   2368.9 |##
   2394.0 |
   2419.1 |##
   2444.2 |
   2469.2 |
   2494.3 |########
   2519.4 |##
   2544.5 |
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2081.2-2576.8 ns)
   2081.2 |########
   2106.0 |
   2130.8 |##
   2155.6 |
   2180.4 |
   2205.1 |
   2229.9 |########################################
   2254.7 |##
   2279.5 |######
   2304.3 |####
   2329.0 |
   2353.8 |
   2378.6 |
   2403.4 |
   2428.1 |####
   2452.9 |##
   2477.7 |##
   2502.5 |##
   2527.3 |
   2552.0 |##
  (3 below, 2 above range)

warm-clamp-head (n=40, range 1369.4-2179.7 ns)
   1369.4 |####
   1409.9 |
   1450.4 |########
   1490.9 |####
   1531.4 |####
   1571.9 |########################################
   1612.4 |####
   1653.0 |####
   1693.5 |####
   1734.0 |
   1774.5 |#############
   1815.0 |######################
   1855.5 |
   1896.0 |#############
   1936.6 |
   1977.1 |########
   2017.6 |
   2058.1 |####
   2098.6 |########
   2139.1 |
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1552.4-5586.2 ns)
   1552.4 |########################################
   1754.1 |
   1955.8 |####
   2157.5 |
   2359.2 |####
   2560.9 |
   2762.6 |
   2964.3 |
   3165.9 |
   3367.6 |
   3569.3 |
   3771.0 |########
   3972.7 |####
   4174.4 |########
   4376.1 |################
   4577.8 |########
   4779.5 |########################
   4981.2 |####
   5182.8 |############
   5384.5 |########
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 1300.3-5237.8 ns)
   1300.3 |########################################
   1497.2 |
   1694.0 |###
   1890.9 |###
   2087.8 |
   2284.7 |
   2481.5 |
   2678.4 |
   2875.3 |
   3072.2 |
   3269.0 |
   3465.9 |###
   3662.8 |###
   3859.7 |
   4056.5 |##########
   4253.4 |################
   4450.3 |###
   4647.2 |####################
   4844.0 |###
   5040.9 |###
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=29.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=42.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=53.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
