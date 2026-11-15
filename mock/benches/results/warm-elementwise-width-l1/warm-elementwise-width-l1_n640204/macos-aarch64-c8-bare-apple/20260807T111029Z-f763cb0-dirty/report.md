# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 49% (significant)

warm-container-native is -2.76 us (49%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.89)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (86% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 86% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone's edge over baseline is significant but tiny (38 ns, 0.66%)

warm-container-plusone differs from baseline warm-container-headroom by 38 ns (0.66%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 2956.4 ns median (-48.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.96x (fastest 2956.4 ns, slowest 5796.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5917ns | 5798ns | 5655ns | 5841ns | 6406ns | base |
| warm-container-kernel | 3158ns | 3097ns | 2968ns | 3122ns | 3457ns | -46.62% |
| warm-container-minimum | 3151ns | 3130ns | 2976ns | 3125ns | 3405ns | -46.74% |
| warm-container-native | 3039ns | 3036ns | 2991ns | 3032ns | 3108ns | -48.64% |
| warm-container-plusone | 6138ns | 5892ns | 5629ns | 5893ns | 7384ns | +3.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5818ns | 5555ns | 6306ns | base | 7.040 |
| warm-container-kernel | 3090ns | 2902ns | 3384ns | -46.90% | 13.257 |
| warm-container-minimum | 3083ns | 2920ns | 3339ns | -47.01% | 13.284 |
| warm-container-native | 2967ns | 2926ns | 3028ns | -49.01% | 13.806 |
| warm-container-plusone | 6037ns | 5538ns | 7258ns | +3.76% | 6.785 |

## Performance model

- Peak throughput: **14.113 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.193 | 51.0% |
| warm-container-kernel | 13.535 | 95.9% |
| warm-container-minimum | 13.392 | 94.9% |
| warm-container-native | 13.854 | 98.2% |
| warm-container-plusone | 7.066 | 50.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5917ns | 5917ns | base |
| warm-container-kernel | 3158ns | 3158ns | -46.62% |
| warm-container-minimum | 3151ns | 3151ns | -46.74% |
| warm-container-native | 3039ns | 3039ns | -48.64% |
| warm-container-plusone | 6138ns | 6138ns | +3.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5695ns | base | --- | [5625, 5846] | --- | --- | --- | --- |
| warm-container-kernel | 3026ns | -2701.2ns (-47.4%) | [-2790, -2566]ns | [2954, 3070] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3058ns | -2712.3ns (-47.6%) | [-2804, -2695]ns | [2973, 3065] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2956ns | -2734.6ns (-48.0%) | [-2878, -2688]ns | [2949, 2972] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5796ns | no significant difference | [-25, +166]ns | [5641, 5877] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 5530ns | -47.5% | -39.8% | -45.6% | +0.2% |
| 2 | 6182ns | -53.1% | -46.2% | -52.1% | -10.3% |
| 3 | 6415ns | -54.7% | -47.2% | -53.1% | -13.6% |
| 4 | 6290ns | -52.8% | -47.1% | -52.4% | -10.2% |
| 5 | 6286ns | -53.8% | -47.0% | -52.1% | -11.7% |
| 6 | 6341ns | -53.9% | -47.4% | -52.3% | -12.6% |
| 7 | 6285ns | -53.8% | -46.8% | -51.4% | -11.9% |
| 8 | 6283ns | -53.7% | -46.8% | -51.9% | -11.8% |
| 9 | 6274ns | -53.7% | -47.0% | -53.5% | -11.8% |
| 10 | 6277ns | -53.8% | -50.4% | -52.6% | -11.5% |
| 11 | 5668ns | -37.9% | -46.1% | -48.5% | -2.3% |
| 12 | 5532ns | -36.3% | -43.4% | -47.2% | +0.1% |
| 13 | 5539ns | -36.8% | -44.9% | -47.4% | +2.2% |
| 14 | 5546ns | -47.7% | -44.9% | -46.4% | +1.5% |
| 15 | 5681ns | -48.6% | -44.4% | -47.7% | +0.3% |
| 16 | 5852ns | -50.4% | -47.7% | -48.9% | -2.7% |
| 17 | 5624ns | -47.4% | -45.5% | -47.1% | +0.7% |
| 18 | 5832ns | -49.4% | -47.6% | -48.9% | -0.9% |
| 19 | 5530ns | -46.6% | -44.8% | -44.2% | +5.2% |
| 20 | 5525ns | -46.2% | -44.6% | -46.4% | +5.3% |
| 21 | 5709ns | -47.2% | -46.3% | -47.4% | +2.9% |
| 22 | 5712ns | -48.1% | -45.1% | -48.3% | +2.9% |
| 23 | 5862ns | -49.4% | -47.8% | -49.7% | +0.2% |
| 24 | 5869ns | -48.2% | -47.8% | -49.6% | +0.3% |
| 25 | 5846ns | -47.5% | -49.1% | -49.4% | +0.6% |
| 26 | 5866ns | -47.8% | -49.4% | -49.5% | +0.1% |
| 27 | 5818ns | -47.3% | -49.0% | -49.3% | +0.8% |
| 28 | 5668ns | -45.8% | -47.8% | -47.8% | +51.4% |
| 29 | 5774ns | -46.9% | -48.5% | -49.0% | +59.4% |
| 30 | 5845ns | -47.5% | -48.7% | -49.5% | +10.5% |
| 31 | 5625ns | -41.2% | -48.2% | -47.8% | +19.4% |
| 32 | 5625ns | -41.0% | -47.3% | -47.8% | +19.4% |
| 33 | 5625ns | -41.5% | -48.2% | -47.8% | +19.4% |
| 34 | 5661ns | -41.8% | -48.5% | -48.0% | +20.0% |
| 35 | 5624ns | -41.5% | -47.9% | -47.7% | +19.7% |
| 36 | 5623ns | -41.4% | -47.9% | -47.7% | +17.5% |
| 37 | 5624ns | -41.3% | -47.8% | -47.6% | +11.7% |
| 38 | 5623ns | -41.4% | -48.0% | -47.8% | +8.6% |
| 39 | 5619ns | -41.5% | -48.1% | -47.8% | -1.4% |
| 40 | 5617ns | -41.3% | -48.0% | -47.3% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.770 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.701 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.888 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.431 | moderate+ |
| warm-container-plusone | 0.656 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 14/40, lost 24/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.3ns | 5818.2ns | 0.0% |  |
| warm-container-kernel | 2.7ns | 3089.6ns | 0.1% |  |
| warm-container-minimum | 2.8ns | 3083.3ns | 0.1% |  |
| warm-container-native | 2.3ns | 2966.7ns | 0.1% |  |
| warm-container-plusone | 2.9ns | 6037.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5554.7-6306.4 ns)
   5554.7 |
   5592.3 |########################################
   5629.9 |####
   5667.5 |############
   5705.1 |########
   5742.7 |####
   5780.2 |
   5817.8 |####################
   5855.4 |############
   5893.0 |
   5930.6 |
   5968.2 |
   6005.7 |
   6043.3 |
   6080.9 |
   6118.5 |
   6156.1 |####
   6193.7 |
   6231.2 |
   6268.8 |########################
  (6 below, 2 above range)

warm-container-kernel (n=40, range 2902.2-3383.6 ns)
   2902.2 |###################################
   2926.3 |
   2950.4 |###############################
   2974.4 |
   2998.5 |####
   3022.6 |####
   3046.6 |##########################
   3070.7 |
   3094.8 |
   3118.8 |
   3142.9 |
   3167.0 |
   3191.0 |
   3215.1 |
   3239.2 |
   3263.2 |
   3287.3 |########################################
   3311.4 |####
   3335.4 |
   3359.5 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 2919.9-3338.8 ns)
   2919.9 |################
   2940.8 |####
   2961.8 |####################
   2982.7 |####
   3003.7 |
   3024.6 |
   3045.6 |########################################
   3066.5 |####
   3087.5 |
   3108.4 |####
   3129.4 |########
   3150.3 |####
   3171.3 |
   3192.2 |
   3213.2 |
   3234.1 |
   3255.1 |
   3276.0 |
   3297.0 |
   3317.9 |########################
  (5 below, 3 above range)

warm-container-native (n=40, range 2926.1-3028.4 ns)
   2926.1 |########
   2931.2 |########
   2936.4 |########################################
   2941.5 |################
   2946.6 |########################
   2951.7 |########################################
   2956.8 |################
   2961.9 |################
   2967.0 |########
   2972.1 |################
   2977.3 |################
   2982.4 |
   2987.5 |################
   2992.6 |
   2997.7 |
   3002.8 |########
   3007.9 |########################
   3013.1 |
   3018.2 |
   3023.3 |################
  (4 below, 2 above range)

warm-container-plusone (n=40, range 5538.5-7257.9 ns)
   5538.5 |########################################
   5624.4 |##########################
   5710.4 |####
   5796.4 |###################################
   5882.4 |####
   5968.3 |
   6054.3 |####
   6140.3 |
   6226.3 |####
   6312.2 |
   6398.2 |####
   6484.2 |
   6570.1 |####
   6656.1 |#################
   6742.1 |####
   6828.1 |
   6914.0 |
   7000.0 |
   7086.0 |
   7172.0 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.66 (measurement drift or warm-up artifact)
