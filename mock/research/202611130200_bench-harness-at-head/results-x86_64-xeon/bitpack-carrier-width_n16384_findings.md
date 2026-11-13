# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 dominates: 31% faster than the next best (bitpack-carrier-packed)

bitpack-carrier-d64 (1.98 us) leads bitpack-carrier-packed (2.59 us) by 31%, a clear separation rather than a photo finish. CV 38.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d64 beats baseline by 43% (significant)

bitpack-carrier-d64 is -1.36 us (43%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d64 is fastest but the noisiest (CV 38.5%)

bitpack-carrier-d64 wins on median (1.98 us) yet has the highest variance (CV 38.5%), while bitpack-carrier-packed is the steadiest (CV 10.7%, 2.59 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-carrier-d64} vs {bitpack-carrier-packed, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} (31% apart)

The field splits into a fast tier {bitpack-carrier-d64} and a slow tier {bitpack-carrier-packed, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-carrier-d64 is inconsistent: worst-20% is 2.1x its best-20%

bitpack-carrier-d64's best 20% of batches run at 1.72 us but its worst 20% at 3.59 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-d64** at 1978.0 ns median (-38.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.69x (fastest 1978.0 ns, slowest 3351.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3364ns | 3313ns | 2752ns | 3342ns | 4041ns | base |
| bitpack-carrier-d16-control | 3661ns | 3476ns | 3249ns | 3519ns | 4498ns | +8.83% |
| bitpack-carrier-d32 | 3082ns | 3003ns | 2560ns | 2975ns | 3925ns | -8.37% |
| bitpack-carrier-d64 | 2543ns | 2138ns | 1854ns | 2362ns | 3773ns | -24.41% |
| bitpack-carrier-packed | 2799ns | 2705ns | 2577ns | 2712ns | 3282ns | -16.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3249ns | 2658ns | 3891ns | base | 5.043 |
| bitpack-carrier-d16-control | 3530ns | 3148ns | 4311ns | +8.65% | 4.642 |
| bitpack-carrier-d32 | 2919ns | 2432ns | 3716ns | -10.14% | 5.612 |
| bitpack-carrier-d64 | 2382ns | 1720ns | 3593ns | -26.69% | 6.879 |
| bitpack-carrier-packed | 2684ns | 2477ns | 3152ns | -17.39% | 6.104 |

## Performance model

- Peak throughput: **9.528 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 5.123 | 53.8% |
| bitpack-carrier-d16-control | 4.889 | 51.3% |
| bitpack-carrier-d32 | 5.859 | 61.5% |
| bitpack-carrier-d64 | 8.283 | 86.9% |
| bitpack-carrier-packed | 6.329 | 66.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 3364ns | 3364ns | base |
| bitpack-carrier-d16-control | 3661ns | 3661ns | +8.83% |
| bitpack-carrier-d32 | 3082ns | 3082ns | -8.37% |
| bitpack-carrier-d64 | 2543ns | 2543ns | -24.41% |
| bitpack-carrier-packed | 2799ns | 2799ns | -16.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3198ns | base | --- | [3165, 3334] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 3351ns | +190.5ns (+6.0%) | [+78, +333]ns | [3324, 3494] | YES | 0.0126 | 0.0095 | 1 |
| bitpack-carrier-d32 | 2796ns | -238.4ns (-7.5%) | [-540, -73]ns | [2628, 2980] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-carrier-d64 | 1978ns | -1361.1ns (-42.6%) | [-1421, -996]ns | [1819, 2408] | YES | 0.0126 | 0.0064 | 0 |
| bitpack-carrier-packed | 2589ns | -549.9ns (-17.2%) | [-682, -324]ns | [2552, 2632] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 3190ns | +5.6% | -2.7% | -43.8% | -18.8% |
| 1 | 2 | 3208ns | -0.1% | +2.6% | -35.0% | -10.3% |
| 1 | 3 | 3554ns | +19.6% | -27.2% | -44.7% | -29.8% |
| 1 | 4 | 2645ns | +28.3% | -4.1% | +28.1% | -5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.420 | moderate- |
| bitpack-carrier-d16-control | -0.435 | moderate- |
| bitpack-carrier-d32 | 0.171 | ok |
| bitpack-carrier-d64 | -0.112 | ok |
| bitpack-carrier-packed | -0.273 | moderate- |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 3/4
- **bitpack-carrier-d32**: won 3/4, lost 1/4
- **bitpack-carrier-d64**: won 3/4, lost 1/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 45.4ns | 3248.9ns | 1.4% |  |
| bitpack-carrier-d16-control | 46.1ns | 3529.9ns | 1.3% |  |
| bitpack-carrier-d32 | 57.2ns | 2919.4ns | 2.0% |  |
| bitpack-carrier-d64 | 80.6ns | 2381.9ns | 3.4% |  |
| bitpack-carrier-packed | 44.4ns | 2684.0ns | 1.7% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 2658.4-3890.7 ns)
   2658.4 |##########
   2720.0 |##########
   2781.6 |
   2843.2 |#####
   2904.8 |
   2966.5 |
   3028.1 |
   3089.7 |###############
   3151.3 |########################################
   3212.9 |####################
   3274.5 |#####
   3336.2 |##########
   3397.8 |
   3459.4 |###############
   3521.0 |###############
   3582.6 |##########
   3644.3 |
   3705.9 |#####
   3767.5 |#####
   3829.1 |
  (5 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 3148.0-4311.4 ns)
   3148.0 |############
   3206.2 |########
   3264.4 |############
   3322.5 |########################################
   3380.7 |
   3438.9 |############
   3497.0 |####
   3555.2 |####
   3613.4 |############
   3671.5 |####
   3729.7 |
   3787.9 |
   3846.0 |
   3904.2 |####
   3962.4 |
   4020.5 |
   4078.7 |
   4136.9 |
   4195.1 |####
   4253.2 |############
  (6 below, 2 above range)

bitpack-carrier-d32 (n=40, range 2432.3-3715.6 ns)
   2432.3 |##########################
   2496.5 |########################################
   2560.7 |######
   2624.8 |####################
   2689.0 |####################
   2753.1 |
   2817.3 |####################
   2881.5 |######
   2945.6 |#############
   3009.8 |#############
   3074.0 |######
   3138.1 |#############
   3202.3 |######
   3266.4 |######
   3330.6 |######
   3394.8 |
   3458.9 |#############
   3523.1 |######
   3587.2 |######
   3651.4 |
  (3 below, 2 above range)

bitpack-carrier-d64 (n=40, range 1719.5-3592.6 ns)
   1719.5 |########################################
   1813.2 |#######
   1906.8 |##################
   2000.5 |##############
   2094.1 |
   2187.8 |
   2281.4 |###
   2375.1 |###
   2468.8 |
   2562.4 |###
   2656.1 |
   2749.7 |
   2843.4 |
   2937.0 |###
   3030.7 |
   3124.3 |
   3218.0 |###
   3311.6 |##################
   3405.3 |#######
   3498.9 |###
  (3 below, 2 above range)

bitpack-carrier-packed (n=40, range 2477.3-3151.9 ns)
   2477.3 |#########################
   2511.0 |#####
   2544.8 |####################
   2578.5 |########################################
   2612.2 |####################
   2646.0 |#####
   2679.7 |##########
   2713.4 |
   2747.2 |#####
   2780.9 |
   2814.6 |#####
   2848.3 |
   2882.1 |
   2915.8 |##########
   2949.5 |
   2983.3 |
   3017.0 |
   3050.7 |#####
   3084.5 |
   3118.2 |
  (6 below, 4 above range)

```

## Diagnostics

- **bitpack-carrier-d64**: CV=32.0% (high variance, measurements may be unstable)
