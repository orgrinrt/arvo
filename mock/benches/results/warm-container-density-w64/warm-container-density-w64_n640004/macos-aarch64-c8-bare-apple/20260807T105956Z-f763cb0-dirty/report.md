# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 52% (significant)

warm-container-minimum is -3.21 us (52%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.1x slower than the field

warm-container-plusone (6.21 us) is 2.1x the fastest (2.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-minimum) are a dead heat (<1%)

warm-container-kernel (2.98 us) and warm-container-minimum (2.98 us) differ by 0.05%, inside the noise, even though the wider field spreads 108.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows warm-up / thermal drift (autocorr +0.78)

warm-container-native's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (107% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 107% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone's edge over baseline is significant but tiny (8 ns, 0.13%)

warm-container-plusone differs from baseline warm-container-headroom by 8 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 2981.4 ns median (-51.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.08x (fastest 2981.4 ns, slowest 6205.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6316ns | 6285ns | 6177ns | 6288ns | 6539ns | base |
| warm-container-kernel | 3056ns | 3041ns | 2983ns | 3046ns | 3159ns | -51.62% |
| warm-container-minimum | 3060ns | 3041ns | 3013ns | 3046ns | 3148ns | -51.56% |
| warm-container-native | 3064ns | 3050ns | 2983ns | 3063ns | 3150ns | -51.48% |
| warm-container-plusone | 6412ns | 6306ns | 6161ns | 6335ns | 6894ns | +1.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6220ns | 6084ns | 6437ns | base | 6.585 |
| warm-container-kernel | 2997ns | 2927ns | 3097ns | -51.82% | 13.668 |
| warm-container-minimum | 2999ns | 2952ns | 3084ns | -51.78% | 13.656 |
| warm-container-native | 3005ns | 2928ns | 3089ns | -51.69% | 13.631 |
| warm-container-plusone | 6312ns | 6072ns | 6787ns | +1.48% | 6.489 |

## Performance model

- Peak throughput: **13.995 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.614 | 47.3% |
| warm-container-kernel | 13.738 | 98.2% |
| warm-container-minimum | 13.732 | 98.1% |
| warm-container-native | 13.707 | 97.9% |
| warm-container-plusone | 6.601 | 47.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6316ns | 6316ns | base |
| warm-container-kernel | 3056ns | 3056ns | -51.62% |
| warm-container-minimum | 3060ns | 3060ns | -51.56% |
| warm-container-native | 3064ns | 3064ns | -51.48% |
| warm-container-plusone | 6412ns | 6412ns | +1.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6193ns | base | --- | [6181, 6212] | --- | --- | --- | --- |
| warm-container-kernel | 2981ns | -3199.4ns (-51.7%) | [-3225, -3147]ns | [2973, 2988] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2983ns | -3205.7ns (-51.8%) | [-3231, -3163]ns | [2978, 2990] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2988ns | -3189.8ns (-51.5%) | [-3206, -3148]ns | [2976, 3021] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6205ns | no significant difference | [-7, +48]ns | [6178, 6280] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 6258ns | -52.5% | -52.1% | -53.2% | -2.0% |
| 2 | 6219ns | -52.2% | -52.9% | -52.5% | -2.3% |
| 3 | 6079ns | -51.1% | -51.5% | -51.8% | +3.6% |
| 4 | 6089ns | -51.2% | -48.9% | -52.0% | +13.2% |
| 5 | 6078ns | -49.9% | -50.8% | -51.8% | +13.4% |
| 6 | 6133ns | -51.5% | -50.6% | -52.3% | +12.5% |
| 7 | 6231ns | -52.2% | -50.8% | -51.9% | +10.8% |
| 8 | 6203ns | -52.0% | -52.1% | -52.8% | +12.4% |
| 9 | 6537ns | -55.2% | -53.7% | -55.2% | -1.3% |
| 10 | 6788ns | -56.8% | -56.3% | -56.8% | -10.6% |
| 11 | 6180ns | -51.4% | -50.6% | -51.8% | +3.7% |
| 12 | 6183ns | -51.8% | -50.2% | -51.9% | +3.4% |
| 13 | 6177ns | -51.7% | -47.9% | -51.7% | +8.2% |
| 14 | 6181ns | -51.7% | -50.0% | -51.8% | +0.2% |
| 15 | 6187ns | -51.7% | -51.1% | -51.8% | +0.2% |
| 16 | 6185ns | -51.8% | -51.8% | -51.3% | -0.0% |
| 17 | 6208ns | -51.9% | -51.5% | -50.4% | -0.1% |
| 18 | 6222ns | -52.0% | -52.0% | -50.4% | +0.0% |
| 19 | 6412ns | -53.5% | -53.6% | -52.0% | -2.9% |
| 20 | 6192ns | -51.8% | -51.9% | -50.2% | +0.1% |
| 21 | 6399ns | -51.7% | -53.4% | -51.2% | +3.1% |
| 22 | 6411ns | -51.9% | -53.6% | -51.9% | -0.1% |
| 23 | 6237ns | -50.5% | -52.2% | -50.6% | +2.6% |
| 24 | 6198ns | -50.2% | -51.9% | -50.2% | +3.4% |
| 25 | 6189ns | -50.0% | -51.9% | -50.2% | +1.1% |
| 26 | 6181ns | -50.1% | -51.4% | -50.1% | -0.2% |
| 27 | 6193ns | -49.8% | -51.9% | -50.2% | -0.1% |
| 28 | 6216ns | -49.9% | -52.1% | -50.4% | -0.1% |
| 29 | 6157ns | -49.7% | -51.6% | -51.7% | +0.3% |
| 30 | 6204ns | -50.1% | -52.0% | -52.0% | +0.4% |
| 31 | 6321ns | -53.6% | -53.6% | -53.6% | -3.9% |
| 32 | 6075ns | -51.8% | -51.4% | -50.1% | +0.1% |
| 33 | 6145ns | -52.4% | -51.7% | -51.7% | +1.5% |
| 34 | 6225ns | -53.0% | -52.7% | -51.9% | -2.3% |
| 35 | 6076ns | -51.7% | -50.9% | -50.3% | -0.0% |
| 36 | 6371ns | -54.1% | -53.1% | -52.8% | -4.6% |
| 37 | 6194ns | -52.7% | -51.8% | -51.7% | -1.4% |
| 38 | 6115ns | -51.1% | -51.2% | -50.6% | -0.5% |
| 39 | 6067ns | -51.8% | -50.5% | -51.0% | -0.0% |
| 40 | 6090ns | -51.7% | -50.9% | -51.0% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.330 | moderate+ |
| warm-container-kernel | 0.776 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.443 | moderate+ |
| warm-container-native | 0.781 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.714 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 14/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 6220.1ns | 0.0% |  |
| warm-container-kernel | 2.5ns | 2996.7ns | 0.1% |  |
| warm-container-minimum | 2.9ns | 2999.4ns | 0.1% |  |
| warm-container-native | 2.9ns | 3004.9ns | 0.1% |  |
| warm-container-plusone | 6.2ns | 6312.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6083.7-6437.1 ns)
   6083.7 |##########
   6101.4 |#####
   6119.1 |#####
   6136.7 |#####
   6154.4 |#####
   6172.1 |########################################
   6189.7 |##############################
   6207.4 |####################
   6225.1 |###############
   6242.7 |#####
   6260.4 |
   6278.1 |
   6295.7 |
   6313.4 |#####
   6331.1 |
   6348.7 |
   6366.4 |#####
   6384.1 |#####
   6401.7 |##########
   6419.4 |
  (5 below, 2 above range)

warm-container-kernel (n=40, range 2926.8-3097.1 ns)
   2926.8 |##################################
   2935.3 |#####
   2943.8 |
   2952.3 |
   2960.8 |#####
   2969.4 |##################################
   2977.9 |########################################
   2986.4 |#################
   2994.9 |#####
   3003.4 |
   3011.9 |
   3020.5 |
   3029.0 |
   3037.5 |#####
   3046.0 |
   3054.5 |
   3063.0 |
   3071.5 |
   3080.1 |#################
   3088.6 |############################
  (4 below, 2 above range)

warm-container-minimum (n=40, range 2952.3-3084.0 ns)
   2952.3 |#####
   2958.9 |
   2965.5 |#################
   2972.1 |########################################
   2978.6 |########################################
   2985.2 |############################
   2991.8 |#####
   2998.4 |#####
   3005.0 |###########
   3011.6 |
   3018.2 |
   3024.7 |###########
   3031.3 |#####
   3037.9 |
   3044.5 |#####
   3051.1 |
   3057.7 |
   3064.3 |#####
   3070.8 |
   3077.4 |#####
  (4 below, 3 above range)

warm-container-native (n=40, range 2928.0-3089.0 ns)
   2928.0 |############################
   2936.1 |
   2944.1 |
   2952.2 |#####
   2960.2 |
   2968.3 |######################
   2976.3 |############################
   2984.4 |###########
   2992.4 |###########
   3000.5 |#####
   3008.5 |#####
   3016.6 |###########
   3024.6 |
   3032.7 |#####
   3040.7 |
   3048.8 |
   3056.8 |
   3064.9 |
   3072.9 |######################
   3081.0 |########################################
  (4 below, 1 above range)

warm-container-plusone (n=40, range 6071.7-6787.2 ns)
   6071.7 |########################################
   6107.5 |#####
   6143.2 |##########
   6179.0 |###################################
   6214.8 |####################
   6250.6 |#####
   6286.3 |#####
   6322.1 |
   6357.9 |
   6393.7 |#########################
   6429.4 |#####
   6465.2 |
   6501.0 |
   6536.8 |
   6572.5 |#####
   6608.3 |
   6644.1 |
   6679.9 |#####
   6715.6 |
   6751.4 |
  (3 below, 5 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.71 (measurement drift or warm-up artifact)
