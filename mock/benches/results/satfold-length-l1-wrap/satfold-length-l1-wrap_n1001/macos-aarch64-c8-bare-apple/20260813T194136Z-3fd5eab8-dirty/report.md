# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-iterfold shows warm-up / thermal drift (autocorr +0.83)

satfold-iterfold's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-lanes4-idx's comparison is tie-heavy (10% tied pairs)

10% of paired samples for satfold-lanes4-idx are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Whole field within 1.6% of the fastest

All 9 variants sit between 2.98 us and 3.03 us - a 1.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### satfold-lanes16's edge over baseline is significant but tiny (39 ns, 1.30%)

satfold-lanes16 differs from baseline satfold-iterfold by 39 ns (1.30%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon** at 2978.3 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 2978.3 ns, slowest 3026.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 3095ns | 3058ns | 3053ns | 3069ns | 3213ns | base |
| satfold-lanes16 | 3097ns | 3099ns | 3057ns | 3087ns | 3163ns | +0.06% |
| satfold-lanes16-constl | 3099ns | 3079ns | 3054ns | 3084ns | 3189ns | +0.14% |
| satfold-lanes4-idx | 3172ns | 3067ns | 3053ns | 3093ns | 3530ns | +2.51% |
| satfold-lanes64 | 3089ns | 3100ns | 3049ns | 3082ns | 3150ns | -0.19% |
| satfold-neon | 3076ns | 3056ns | 3051ns | 3061ns | 3143ns | -0.61% |
| satfold-neon8 | 3088ns | 3064ns | 3054ns | 3077ns | 3153ns | -0.22% |
| satfold-nolaw | 3117ns | 3077ns | 3052ns | 3091ns | 3261ns | +0.73% |
| satfold-seq | 3156ns | 3108ns | 3054ns | 3113ns | 3387ns | +1.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3015ns | 2975ns | 3131ns | base | 10.869 |
| satfold-lanes16 | 3016ns | 2978ns | 3080ns | +0.05% | 10.864 |
| satfold-lanes16-constl | 3013ns | 2975ns | 3095ns | -0.06% | 10.875 |
| satfold-lanes4-idx | 3088ns | 2976ns | 3428ns | +2.44% | 10.611 |
| satfold-lanes64 | 3008ns | 2972ns | 3065ns | -0.24% | 10.895 |
| satfold-neon | 2997ns | 2974ns | 3060ns | -0.59% | 10.934 |
| satfold-neon8 | 3007ns | 2975ns | 3072ns | -0.24% | 10.896 |
| satfold-nolaw | 3035ns | 2974ns | 3175ns | +0.67% | 10.797 |
| satfold-seq | 3073ns | 2977ns | 3291ns | +1.94% | 10.663 |

## Performance model

- Peak throughput: **11.025 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 11.002 | 99.8% |
| satfold-lanes16 | 10.863 | 98.5% |
| satfold-lanes16-constl | 10.945 | 99.3% |
| satfold-lanes4-idx | 10.996 | 99.7% |
| satfold-lanes64 | 10.853 | 98.4% |
| satfold-neon | 11.002 | 99.8% |
| satfold-neon8 | 10.978 | 99.6% |
| satfold-nolaw | 10.958 | 99.4% |
| satfold-seq | 10.829 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3095ns | 3095ns | base |
| satfold-lanes16 | 3097ns | 3097ns | +0.06% |
| satfold-lanes16-constl | 3099ns | 3099ns | +0.14% |
| satfold-lanes4-idx | 3172ns | 3172ns | +2.51% |
| satfold-lanes64 | 3089ns | 3089ns | -0.19% |
| satfold-neon | 3076ns | 3076ns | -0.61% |
| satfold-neon8 | 3088ns | 3088ns | -0.22% |
| satfold-nolaw | 3117ns | 3117ns | +0.73% |
| satfold-seq | 3156ns | 3156ns | +1.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 2978ns | base | --- | [2977, 2981] | --- | --- | --- | --- |
| satfold-lanes16 | 3016ns | no significant difference | [-1, +44]ns | [2990, 3023] | no | 0.3228 | 0.0807 | 0 |
| satfold-lanes16-constl | 2994ns | no significant difference | [-2, +18]ns | [2980, 3023] | no | 0.8358 | 0.5224 | 1 |
| satfold-lanes4-idx | 2980ns | no significant difference | [-2, +47]ns | [2978, 3023] | no | 0.8358 | 0.3368 | 1 |
| satfold-lanes64 | 3019ns | no significant difference | [-6, +44]ns | [2975, 3021] | no | 1.0000 | 1.0000 | 0 |
| satfold-neon | 2978ns | no significant difference | [-4, +2]ns | [2977, 2986] | no | 0.8358 | 0.4296 | 0 |
| satfold-neon8 | 2985ns | no significant difference | [-5, +28]ns | [2979, 3024] | no | 0.9996 | 0.8746 | 0 |
| satfold-nolaw | 2990ns | no significant difference | [-4, +43]ns | [2978, 3022] | no | 0.8361 | 0.6271 | 2 |
| satfold-seq | 3026ns | +48.6ns (+1.6%) | [+41, +65]ns | [3022, 3048] | YES | 0.0178 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2975ns | +1.6% | +1.7% | +0.1% | +1.6% | +2.2% | +1.7% | +5.6% | +0.9% |
| 2 | 2978ns | +1.5% | +0.6% | -0.1% | +1.4% | -0.1% | +1.6% | +5.8% | +0.1% |
| 3 | 2975ns | +1.5% | +0.6% | +0.0% | +1.5% | -0.0% | +1.6% | +5.7% | +1.8% |
| 4 | 3040ns | -0.6% | -1.9% | -2.1% | -0.7% | -0.0% | -0.3% | +11.4% | -0.5% |
| 5 | 3043ns | -0.5% | -1.6% | -2.1% | -0.7% | -2.3% | -2.2% | +3.4% | -0.8% |
| 6 | 2974ns | +3.9% | +0.4% | +0.1% | +1.7% | +0.2% | +0.2% | +5.5% | +1.7% |
| 7 | 2978ns | +1.4% | -0.0% | +0.3% | +1.5% | -0.1% | -0.1% | +5.6% | +2.3% |
| 8 | 2977ns | +5.7% | -0.1% | +0.0% | +1.5% | -0.1% | +0.1% | +5.6% | +2.1% |
| 9 | 3011ns | +0.4% | -1.0% | -1.0% | +0.3% | -1.2% | -1.2% | +4.5% | +1.6% |
| 10 | 2975ns | +1.6% | +0.1% | +0.0% | +1.6% | +0.1% | +0.0% | +5.6% | +1.8% |
| 11 | 2980ns | +0.2% | -0.1% | +5.2% | +1.6% | +2.9% | +2.7% | -0.2% | +1.6% |
| 12 | 2976ns | +3.9% | -0.0% | +5.4% | +1.7% | +3.1% | +2.6% | +0.0% | +1.8% |
| 13 | 2982ns | +1.5% | +0.0% | +5.1% | +2.3% | -0.2% | +4.4% | -0.2% | +2.5% |
| 14 | 2976ns | +1.8% | +0.1% | +5.3% | +5.5% | +0.3% | +3.9% | +0.1% | +1.6% |
| 15 | 2982ns | +1.5% | -0.1% | +5.2% | +3.0% | +1.2% | +1.6% | +1.1% | +3.9% |
| 16 | 2977ns | +3.0% | +0.0% | +5.1% | +1.5% | +0.1% | +1.7% | -0.1% | +5.1% |
| 17 | 2981ns | +2.2% | -0.2% | +5.1% | +1.5% | -0.1% | +2.9% | -0.2% | +42.3% |
| 18 | 2977ns | +5.6% | +0.0% | +82.2% | +1.8% | +0.0% | +3.4% | +2.1% | +11.1% |
| 19 | 2977ns | +1.7% | +0.1% | +5.1% | +1.4% | +2.4% | +1.8% | -0.1% | +2.4% |
| 20 | 2975ns | +1.6% | +0.1% | +5.3% | +1.7% | +2.5% | +1.6% | -0.1% | +1.4% |
| 21 | 2975ns | +0.4% | +2.2% | +1.6% | +0.1% | +1.4% | +0.1% | +1.7% | +5.1% |
| 22 | 2979ns | -0.1% | +2.1% | +7.2% | -0.2% | +0.3% | -0.0% | +1.4% | +1.3% |
| 23 | 2980ns | +0.1% | +2.8% | +1.5% | -0.2% | -0.0% | -0.1% | +1.5% | +1.5% |
| 24 | 2975ns | +0.5% | +1.7% | +1.6% | -0.1% | -0.0% | +0.0% | +1.8% | +2.6% |
| 25 | 2979ns | +0.6% | +1.4% | +1.6% | +5.8% | -0.0% | -0.2% | +1.4% | +2.6% |
| 26 | 2976ns | +0.6% | +8.4% | +1.5% | -0.1% | +0.5% | -0.0% | +0.6% | +3.5% |
| 27 | 2978ns | +0.4% | +0.9% | +1.5% | -0.1% | +0.0% | +3.7% | +0.0% | +1.6% |
| 28 | 2978ns | +0.4% | +1.0% | -0.0% | -0.3% | +0.1% | -0.0% | +0.0% | +5.0% |
| 29 | 2979ns | -0.1% | +2.4% | -0.1% | -0.3% | -0.2% | -0.2% | -0.2% | +4.0% |
| 30 | 2977ns | +0.0% | +2.9% | -0.1% | -0.1% | -0.0% | +0.3% | +0.3% | +1.6% |
| 31 | 3135ns | -3.9% | -3.5% | -5.0% | -5.1% | -0.2% | -4.8% | -4.4% | -3.6% |
| 32 | 3128ns | -4.3% | +1.0% | -4.9% | -5.0% | -4.8% | -4.6% | -4.7% | +2.4% |
| 33 | 3130ns | -4.7% | -3.3% | -4.8% | -5.0% | -4.9% | -4.7% | -4.9% | -4.2% |
| 34 | 3132ns | -4.9% | -3.4% | -4.9% | -5.0% | -4.9% | -4.9% | -5.0% | -4.9% |
| 35 | 3132ns | -4.3% | -3.4% | -4.9% | -4.9% | -5.0% | -4.8% | -4.7% | -5.1% |
| 36 | 3130ns | -4.8% | -3.8% | -4.8% | -4.9% | -4.7% | -4.8% | -3.8% | -4.9% |
| 37 | 3130ns | -3.5% | -5.0% | -4.9% | -4.9% | -4.3% | -4.6% | -5.0% | -5.0% |
| 38 | 3133ns | -5.0% | -5.1% | -5.0% | -5.1% | -5.1% | -3.5% | -5.0% | -4.9% |
| 39 | 3078ns | -3.2% | +1.3% | -3.4% | -3.4% | -2.9% | -2.7% | -3.2% | -3.3% |
| 40 | 2978ns | -0.1% | -0.0% | +3.1% | -0.1% | +2.2% | +0.1% | -0.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.828 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.328 | moderate+ |
| satfold-lanes16-constl | 0.186 | ok |
| satfold-lanes4-idx | 0.080 | ok |
| satfold-lanes64 | 0.393 | moderate+ |
| satfold-neon | 0.063 | ok |
| satfold-neon8 | 0.561 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.706 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.260 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 12/40, lost 25/40
- **satfold-lanes16-constl**: won 12/40, lost 17/40
- **satfold-lanes4-idx**: won 12/40, lost 21/40
- **satfold-lanes64**: won 17/40, lost 19/40
- **satfold-neon**: won 15/40, lost 12/40
- **satfold-neon8**: won 15/40, lost 17/40
- **satfold-nolaw**: won 15/40, lost 20/40
- **satfold-seq**: won 10/40, lost 28/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.3ns | 3014.7ns | 0.1% |  |
| satfold-lanes16 | 2.5ns | 3016.2ns | 0.1% |  |
| satfold-lanes16-constl | 2.8ns | 3013.0ns | 0.1% |  |
| satfold-lanes4-idx | 3.4ns | 3088.2ns | 0.1% |  |
| satfold-lanes64 | 2.5ns | 3007.6ns | 0.1% |  |
| satfold-neon | 2.3ns | 2997.0ns | 0.1% |  |
| satfold-neon8 | 2.1ns | 3007.5ns | 0.1% |  |
| satfold-nolaw | 2.4ns | 3035.0ns | 0.1% |  |
| satfold-seq | 2.5ns | 3073.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 2975.0-3131.2 ns)
   2975.0 |########################################
   2982.8 |
   2990.6 |
   2998.4 |
   3006.2 |#
   3014.1 |
   3021.9 |
   3029.7 |
   3037.5 |###
   3045.3 |
   3053.1 |
   3060.9 |
   3068.7 |
   3076.5 |#
   3084.3 |
   3092.1 |
   3100.0 |
   3107.8 |
   3115.6 |
   3123.4 |######
  (2 below, 4 above range)

satfold-lanes16 (n=40, range 2977.7-3080.3 ns)
   2977.7 |#############
   2982.8 |#################
   2988.0 |#############
   2993.1 |#################
   2998.2 |
   3003.4 |
   3008.5 |####
   3013.6 |
   3018.7 |########################################
   3023.9 |#################
   3029.0 |####
   3034.1 |
   3039.3 |
   3044.4 |####
   3049.5 |
   3054.7 |
   3059.8 |
   3064.9 |####
   3070.0 |
   3075.2 |
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 2975.5-3094.9 ns)
   2975.5 |########################################
   2981.4 |##########
   2987.4 |
   2993.4 |##########
   2999.4 |###
   3005.3 |###
   3011.3 |###
   3017.3 |###
   3023.2 |#####################
   3029.2 |
   3035.2 |#######
   3041.2 |
   3047.1 |###
   3053.1 |
   3059.1 |#######
   3065.0 |
   3071.0 |
   3077.0 |
   3082.9 |
   3088.9 |
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 2975.5-3428.4 ns)
   2975.5 |########################################
   2998.1 |
   3020.8 |############
   3043.4 |
   3066.1 |##
   3088.7 |
   3111.4 |########
   3134.0 |##########
   3156.7 |
   3179.3 |##
   3202.0 |
   3224.6 |
   3247.3 |
   3269.9 |
   3292.6 |
   3315.2 |
   3337.8 |
   3360.5 |
   3383.1 |
   3405.8 |
  (3 below, 1 above range)

satfold-lanes64 (n=40, range 2972.2-3064.8 ns)
   2972.2 |########################################
   2976.8 |#####
   2981.4 |
   2986.1 |
   2990.7 |
   2995.3 |
   3000.0 |
   3004.6 |
   3009.2 |
   3013.9 |
   3018.5 |###############################
   3023.1 |##############
   3027.7 |##
   3032.4 |
   3037.0 |
   3041.6 |
   3046.3 |##
   3050.9 |
   3055.5 |
   3060.2 |
  (3 below, 3 above range)

satfold-neon (n=40, range 2973.9-3060.2 ns)
   2973.9 |########################################
   2978.2 |##########################
   2982.5 |###
   2986.9 |######
   2991.2 |######
   2995.5 |
   2999.8 |
   3004.1 |
   3008.4 |
   3012.7 |###
   3017.0 |###
   3021.4 |
   3025.7 |
   3030.0 |
   3034.3 |
   3038.6 |##########
   3042.9 |
   3047.2 |######
   3051.5 |
   3055.9 |
  (5 below, 3 above range)

satfold-neon8 (n=40, range 2975.0-3072.4 ns)
   2975.0 |########################################
   2979.9 |##################
   2984.7 |##########
   2989.6 |
   2994.5 |###
   2999.3 |
   3004.2 |
   3009.1 |
   3014.0 |
   3018.8 |#######
   3023.7 |##############
   3028.6 |##########
   3033.4 |
   3038.3 |
   3043.2 |
   3048.0 |
   3052.9 |###
   3057.8 |###
   3062.6 |###
   3067.5 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 2973.9-3175.3 ns)
   2973.9 |########################################
   2984.0 |########
   2994.0 |##
   3004.1 |##
   3014.2 |###########
   3024.2 |#####
   3034.3 |##
   3044.4 |
   3054.5 |
   3064.5 |
   3074.6 |
   3084.7 |
   3094.7 |
   3104.8 |
   3114.9 |
   3124.9 |
   3135.0 |#################
   3145.1 |########
   3155.1 |
   3165.2 |
  (4 below, 1 above range)

satfold-seq (n=40, range 2977.2-3291.1 ns)
   2977.2 |#################
   2992.9 |########
   3008.6 |##########################
   3024.3 |########################################
   3040.0 |#################
   3055.7 |########
   3071.3 |####
   3087.0 |########
   3102.7 |
   3118.4 |#############
   3134.1 |
   3149.8 |
   3165.5 |
   3181.2 |
   3196.9 |####
   3212.6 |
   3228.3 |
   3244.0 |
   3259.7 |
   3275.4 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.71 (measurement drift or warm-up artifact)
