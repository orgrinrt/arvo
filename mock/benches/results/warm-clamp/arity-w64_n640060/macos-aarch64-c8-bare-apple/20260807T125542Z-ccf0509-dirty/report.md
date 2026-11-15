# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 36% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (1.79 us) leads warm-clamp-accfit (2.44 us) by 36%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 51% (significant)

warm-clamp-min-lanes is -1.88 us (51%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.1x slower than the field

warm-clamp-minimum (3.78 us) is 2.1x the fastest (1.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.89)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes} vs {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum} (36% apart)

The field splits into a fast tier {warm-clamp-min-lanes} and a slow tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1792.9 ns median (-51.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.11x (fastest 1792.9 ns, slowest 3782.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3769ns | 3754ns | 3728ns | 3760ns | 3837ns | base |
| warm-clamp-accfit | 2600ns | 2505ns | 2500ns | 2516ns | 2953ns | -31.01% |
| warm-clamp-accfit-dyn | 3041ns | 2912ns | 2844ns | 2971ns | 3448ns | -19.32% |
| warm-clamp-head | 2696ns | 2565ns | 2550ns | 2612ns | 3096ns | -28.46% |
| warm-clamp-min-lanes | 1952ns | 1858ns | 1808ns | 1916ns | 2202ns | -48.21% |
| warm-clamp-minimum | 3918ns | 3852ns | 3731ns | 3866ns | 4258ns | +3.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3705ns | 3665ns | 3771ns | base | 2.211 |
| warm-clamp-accfit | 2535ns | 2437ns | 2879ns | -31.58% | 3.232 |
| warm-clamp-accfit-dyn | 2980ns | 2788ns | 3381ns | -19.56% | 2.749 |
| warm-clamp-head | 2602ns | 2463ns | 2990ns | -29.76% | 3.148 |
| warm-clamp-min-lanes | 1883ns | 1744ns | 2125ns | -49.18% | 4.351 |
| warm-clamp-minimum | 3846ns | 3669ns | 4171ns | +3.82% | 2.130 |

## Performance model

- Peak throughput: **4.697 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.222 | 47.3% |
| warm-clamp-accfit | 3.353 | 71.4% |
| warm-clamp-accfit-dyn | 2.877 | 61.2% |
| warm-clamp-head | 3.314 | 70.6% |
| warm-clamp-min-lanes | 4.569 | 97.3% |
| warm-clamp-minimum | 2.166 | 46.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3769ns | 3769ns | base |
| warm-clamp-accfit | 2600ns | 2600ns | -31.01% |
| warm-clamp-accfit-dyn | 3041ns | 3041ns | -19.32% |
| warm-clamp-head | 2696ns | 2696ns | -28.46% |
| warm-clamp-min-lanes | 1952ns | 1952ns | -48.21% |
| warm-clamp-minimum | 3918ns | 3918ns | +3.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3687ns | base | --- | [3671, 3725] | --- | --- | --- | --- |
| warm-clamp-accfit | 2443ns | -1232.5ns (-33.4%) | [-1245, -1226]ns | [2441, 2467] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2848ns | -862.1ns (-23.4%) | [-879, -774]ns | [2837, 2891] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2472ns | -1205.4ns (-32.7%) | [-1236, -1199]ns | [2468, 2482] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1793ns | -1888.3ns (-51.2%) | [-1900, -1850]ns | [1774, 1846] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 3782ns | +92.3ns (+2.5%) | [+60, +149]ns | [3736, 3874] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 3729ns | -33.4% | -23.5% | -33.7% | -53.1% | +4.1% |
| 2 | 3722ns | -33.4% | -22.8% | -33.7% | -53.2% | +15.2% |
| 3 | 3725ns | -33.3% | -24.0% | -33.4% | -53.2% | +4.2% |
| 4 | 3729ns | -33.4% | -24.0% | -31.7% | -53.2% | +5.1% |
| 5 | 3728ns | -33.5% | -23.2% | -33.7% | -53.3% | +11.7% |
| 6 | 3815ns | -34.5% | -25.6% | -35.2% | -54.0% | +2.3% |
| 7 | 3732ns | -34.5% | -24.9% | -33.8% | -53.2% | +4.1% |
| 8 | 3725ns | -34.1% | -23.4% | -33.7% | -53.2% | +4.1% |
| 9 | 3728ns | -34.4% | -23.8% | -33.7% | -51.3% | +3.8% |
| 10 | 3725ns | -34.5% | -24.7% | -33.3% | -52.8% | +4.6% |
| 11 | 3663ns | -33.4% | -21.4% | -32.5% | -42.4% | +5.4% |
| 12 | 3670ns | -33.4% | -24.0% | -32.8% | -41.9% | +13.3% |
| 13 | 3667ns | -33.4% | -24.0% | -32.8% | -51.0% | +2.6% |
| 14 | 3667ns | -33.6% | -24.0% | -32.9% | -51.9% | +2.8% |
| 15 | 3671ns | -33.5% | -24.1% | -32.8% | -50.7% | +1.3% |
| 16 | 3665ns | -33.5% | -23.6% | -32.8% | -51.6% | +1.7% |
| 17 | 3670ns | -33.2% | -24.0% | -33.0% | -51.2% | +2.0% |
| 18 | 3671ns | -33.5% | -24.1% | -32.9% | -51.6% | +1.8% |
| 19 | 3671ns | -19.7% | -24.0% | -32.9% | -51.6% | -0.1% |
| 20 | 3668ns | -19.4% | -23.9% | -32.8% | -51.7% | -0.0% |
| 21 | 3885ns | -37.4% | -13.0% | -36.6% | -45.5% | +14.6% |
| 22 | 3693ns | -33.9% | -8.4% | -33.2% | -42.5% | +6.6% |
| 23 | 3713ns | -34.1% | -8.9% | -33.5% | -42.9% | -1.1% |
| 24 | 3667ns | -33.5% | -7.8% | -32.4% | -42.1% | +0.1% |
| 25 | 3681ns | -33.8% | -8.1% | -32.6% | -42.3% | -0.3% |
| 26 | 3666ns | -23.5% | -7.7% | -32.7% | -42.1% | +0.1% |
| 27 | 3725ns | -20.6% | -9.3% | -33.4% | -43.0% | -1.5% |
| 28 | 3725ns | -20.7% | -9.3% | -33.0% | -43.0% | -1.5% |
| 29 | 3760ns | -21.4% | -10.2% | -34.3% | -43.6% | +1.6% |
| 30 | 3740ns | -21.0% | -9.6% | -34.0% | -43.1% | +19.0% |
| 31 | 3681ns | -33.7% | -22.2% | -18.7% | -52.6% | +1.2% |
| 32 | 3665ns | -33.4% | -22.4% | -18.4% | -51.2% | +9.0% |
| 33 | 3732ns | -34.6% | -24.0% | -20.0% | -50.9% | +2.4% |
| 34 | 3773ns | -35.3% | -24.7% | -20.8% | -50.5% | -0.1% |
| 35 | 3664ns | -33.4% | -22.5% | -18.3% | -51.0% | +2.6% |
| 36 | 3672ns | -33.6% | -19.6% | -18.6% | -51.7% | +3.3% |
| 37 | 3669ns | -33.6% | -20.7% | -18.5% | -51.4% | -0.0% |
| 38 | 3698ns | -34.1% | -23.2% | -19.2% | -49.7% | +0.3% |
| 39 | 3667ns | -33.5% | -20.9% | -18.5% | -51.9% | +1.9% |
| 40 | 3670ns | -32.4% | -12.9% | -18.7% | -50.1% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.204 | moderate+ |
| warm-clamp-accfit | 0.650 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.824 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.888 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.721 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.133 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 6/40, lost 30/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 3704.7ns | 0.1% |  |
| warm-clamp-accfit | 2.6ns | 2534.7ns | 0.1% |  |
| warm-clamp-accfit-dyn | 2.9ns | 2980.0ns | 0.1% |  |
| warm-clamp-head | 2.7ns | 2602.0ns | 0.1% |  |
| warm-clamp-min-lanes | 2.4ns | 1882.7ns | 0.1% |  |
| warm-clamp-minimum | 3.0ns | 3846.3ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 3665.4-3770.7 ns)
   3665.4 |########################################
   3670.6 |################
   3675.9 |########
   3681.2 |
   3686.4 |
   3691.7 |####
   3697.0 |####
   3702.3 |
   3707.5 |
   3712.8 |####
   3718.1 |####
   3723.3 |############################
   3728.6 |################
   3733.9 |
   3739.1 |####
   3744.4 |
   3749.7 |
   3754.9 |
   3760.2 |####
   3765.5 |
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 2436.7-2878.9 ns)
   2436.7 |########################################
   2458.8 |###
   2480.9 |#########
   2503.0 |
   2525.2 |
   2547.3 |
   2569.4 |
   2591.5 |
   2613.6 |
   2635.7 |
   2657.8 |
   2679.9 |
   2702.0 |
   2724.1 |
   2746.2 |
   2768.4 |
   2790.5 |#
   2812.6 |
   2834.7 |
   2856.8 |
  (4 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 2787.5-3381.2 ns)
   2787.5 |###################################
   2817.2 |########################################
   2846.9 |######################
   2876.6 |########
   2906.2 |####
   2935.9 |####
   2965.6 |
   2995.3 |
   3025.0 |
   3054.7 |
   3084.3 |
   3114.0 |
   3143.7 |
   3173.4 |####
   3203.1 |
   3232.8 |
   3262.4 |
   3292.1 |
   3321.8 |
   3351.5 |######################
  (3 below, 5 above range)

warm-clamp-head (n=40, range 2462.8-2990.1 ns)
   2462.8 |########################################
   2489.2 |#
   2515.5 |
   2541.9 |#
   2568.3 |
   2594.6 |
   2621.0 |
   2647.4 |
   2673.7 |
   2700.1 |
   2726.5 |
   2752.8 |
   2779.2 |
   2805.5 |
   2831.9 |
   2858.3 |
   2884.6 |
   2911.0 |
   2937.4 |
   2963.7 |##########
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1744.1-2124.9 ns)
   1744.1 |########################
   1763.1 |############################
   1782.2 |####################
   1801.2 |########
   1820.2 |########
   1839.3 |
   1858.3 |########
   1877.4 |
   1896.4 |
   1915.4 |
   1934.5 |
   1953.5 |
   1972.6 |
   1991.6 |
   2010.6 |
   2029.7 |
   2048.7 |
   2067.8 |
   2086.8 |
   2105.8 |########################################
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 3668.8-4171.0 ns)
   3668.8 |##########################
   3693.9 |######
   3719.0 |########################################
   3744.1 |####################
   3769.2 |######
   3794.3 |#############
   3819.4 |#############
   3844.5 |######
   3869.7 |#################################
   3894.8 |####################
   3919.9 |######
   3945.0 |
   3970.1 |
   3995.2 |######
   4020.3 |
   4045.4 |
   4070.5 |
   4095.6 |
   4120.7 |
   4145.9 |#############
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.72 (measurement drift or warm-up artifact)
