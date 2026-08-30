# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 509 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 82% (significant)

warm-container-kernel is -2.54 us (82%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 6.1x slower than the field

warm-container-headroom (3.09 us) is 6.1x the fastest (509 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (509 ns) and warm-container-native (510 ns) differ by 0.16%, inside the noise, even though the wider field spreads 508.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.81)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (486% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 486% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.1x the fastest

Fastest warm-container-kernel (509 ns) to slowest warm-container-headroom (3.09 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (6 ns, 0.20%)

warm-container-plusone differs from baseline warm-container-headroom by 6 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 508.8 ns median (-83.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 6.08x (fastest 508.8 ns, slowest 3094.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3177ns | 3155ns | 3009ns | 3127ns | 3494ns | base |
| warm-container-kernel | 578ns | 572ns | 568ns | 575ns | 599ns | -81.80% |
| warm-container-minimum | 581ns | 589ns | 559ns | 584ns | 594ns | -81.70% |
| warm-container-native | 578ns | 573ns | 562ns | 575ns | 604ns | -81.80% |
| warm-container-plusone | 3116ns | 3136ns | 3034ns | 3117ns | 3192ns | -1.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3113ns | 2951ns | 3416ns | base | 13.157 |
| warm-container-kernel | 515ns | 507ns | 535ns | -83.45% | 79.519 |
| warm-container-minimum | 517ns | 499ns | 528ns | -83.38% | 79.155 |
| warm-container-native | 514ns | 500ns | 537ns | -83.48% | 79.658 |
| warm-container-plusone | 3054ns | 2978ns | 3124ns | -1.91% | 13.413 |

## Performance model

- Peak throughput: **82.131 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 13.236 | 16.1% |
| warm-container-kernel | 80.503 | 98.0% |
| warm-container-minimum | 78.079 | 95.1% |
| warm-container-native | 80.377 | 97.9% |
| warm-container-plusone | 13.323 | 16.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3177ns | 3177ns | base |
| warm-container-kernel | 578ns | 578ns | -81.80% |
| warm-container-minimum | 581ns | 581ns | -81.70% |
| warm-container-native | 578ns | 578ns | -81.80% |
| warm-container-plusone | 3116ns | 3116ns | -1.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3095ns | base | --- | [3005, 3102] | --- | --- | --- | --- |
| warm-container-kernel | 509ns | -2570.8ns (-83.1%) | [-2583, -2498]ns | [508, 512] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 525ns | -2570.7ns (-83.1%) | [-2584, -2493]ns | [515, 526] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 510ns | -2569.8ns (-83.0%) | [-2582, -2502]ns | [507, 511] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3074ns | no significant difference | [-77, +55]ns | [3002, 3097] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 2986ns | -82.8% | -82.4% | -82.9% | +3.7% |
| 2 | 2981ns | -83.0% | -82.3% | -83.0% | +3.9% |
| 3 | 3082ns | -83.4% | -83.0% | -83.6% | +0.3% |
| 4 | 3128ns | -83.8% | -83.3% | -83.7% | -0.9% |
| 5 | 4464ns | -88.6% | -88.2% | -88.6% | -32.8% |
| 6 | 3095ns | -83.6% | -83.0% | -83.4% | -3.5% |
| 7 | 3108ns | -83.7% | -83.1% | -83.6% | -3.4% |
| 8 | 3100ns | -83.5% | -83.1% | -83.7% | -3.5% |
| 9 | 3091ns | -83.6% | -83.0% | -83.6% | -3.5% |
| 10 | 3091ns | -83.5% | -82.9% | -83.6% | -3.4% |
| 11 | 2974ns | -82.9% | -82.2% | -83.0% | +1.2% |
| 12 | 3067ns | -83.3% | -82.8% | -83.5% | -2.4% |
| 13 | 2983ns | -83.0% | -82.3% | -82.9% | +2.3% |
| 14 | 2975ns | -82.9% | -82.3% | -82.8% | +3.7% |
| 15 | 2983ns | -83.0% | -82.3% | -82.9% | +4.0% |
| 16 | 2982ns | -82.8% | -82.3% | -82.9% | +4.0% |
| 17 | 2985ns | -83.0% | -82.4% | -83.0% | +4.1% |
| 18 | 3114ns | -83.7% | -83.0% | -83.7% | -0.2% |
| 19 | 2981ns | -82.9% | -82.2% | -82.8% | +3.9% |
| 20 | 2983ns | -82.8% | -82.4% | -82.9% | +4.0% |
| 21 | 3317ns | -84.7% | -84.9% | -84.9% | -10.2% |
| 22 | 3313ns | -84.7% | -84.9% | -84.9% | -11.3% |
| 23 | 3317ns | -82.8% | -85.0% | -85.0% | -10.4% |
| 24 | 3313ns | -84.7% | -84.9% | -84.6% | -9.4% |
| 25 | 3250ns | -84.4% | -84.7% | -84.4% | -8.2% |
| 26 | 2924ns | -82.6% | -83.0% | -82.8% | +2.3% |
| 27 | 2929ns | -82.6% | -83.0% | -83.0% | +5.4% |
| 28 | 2922ns | -82.6% | -82.9% | -81.6% | +6.2% |
| 29 | 2920ns | -82.6% | -82.9% | -83.0% | +5.8% |
| 30 | 3024ns | -83.3% | -83.5% | -83.6% | +4.2% |
| 31 | 3195ns | -83.5% | -83.8% | -83.3% | -1.3% |
| 32 | 3106ns | -83.1% | -83.5% | -83.0% | -0.4% |
| 33 | 3101ns | -82.9% | -83.1% | -82.9% | -1.2% |
| 34 | 3095ns | -82.9% | -83.6% | -82.5% | +0.5% |
| 35 | 3098ns | -83.0% | -83.5% | -81.9% | +0.5% |
| 36 | 3095ns | -83.0% | -83.2% | -83.0% | -1.5% |
| 37 | 3102ns | -82.9% | -83.1% | -82.9% | -2.5% |
| 38 | 3156ns | -83.2% | -83.2% | -83.0% | -4.9% |
| 39 | 3095ns | -82.8% | -83.1% | -82.9% | -3.1% |
| 40 | 3103ns | -83.0% | -83.6% | -82.9% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.142 | ok |
| warm-container-kernel | 0.280 | moderate+ |
| warm-container-minimum | 0.814 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.566 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.658 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 21/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.0ns | 3113.2ns | 0.1% |  |
| warm-container-kernel | 2.3ns | 515.1ns | 0.5% |  |
| warm-container-minimum | 2.4ns | 517.5ns | 0.5% |  |
| warm-container-native | 2.6ns | 514.2ns | 0.5% |  |
| warm-container-plusone | 2.7ns | 3053.7ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2950.7-3415.5 ns)
   2950.7 |###
   2974.0 |###########################
   2997.2 |
   3020.5 |###
   3043.7 |###
   3066.9 |###
   3090.2 |########################################
   3113.4 |######
   3136.7 |###
   3159.9 |
   3183.1 |###
   3206.4 |
   3229.6 |###
   3252.8 |
   3276.1 |
   3299.3 |############
   3322.6 |
   3345.8 |
   3369.0 |
   3392.3 |
  (4 below, 1 above range)

warm-container-kernel (n=40, range 506.5-535.1 ns)
    506.5 |########################################
    507.9 |#######################
    509.4 |
    510.8 |####################
    512.2 |###
    513.6 |
    515.1 |
    516.5 |
    517.9 |
    519.4 |
    520.8 |
    522.2 |
    523.6 |###
    525.1 |###
    526.5 |######
    527.9 |######
    529.4 |##########
    530.8 |
    532.2 |###
    533.6 |
  (3 below, 1 above range)

warm-container-minimum (n=40, range 498.7-528.4 ns)
    498.7 |##############################
    500.2 |#####
    501.7 |
    503.2 |
    504.6 |
    506.1 |#####
    507.6 |#####
    509.1 |
    510.6 |#####
    512.1 |#####
    513.6 |
    515.0 |
    516.5 |#####
    518.0 |
    519.5 |#####
    521.0 |
    522.5 |##########
    523.9 |########################################
    525.4 |#########################
    526.9 |#########################
  (3 below, 4 above range)

warm-container-native (n=40, range 499.8-536.9 ns)
    499.8 |
    501.7 |#####
    503.5 |
    505.4 |########################################
    507.2 |############################
    509.1 |##################################
    510.9 |#################
    512.8 |
    514.7 |#####
    516.5 |
    518.4 |
    520.2 |
    522.1 |
    523.9 |
    525.8 |#####
    527.6 |#################
    529.5 |###########
    531.4 |#####
    533.2 |
    535.1 |#####
  (6 below, 3 above range)

warm-container-plusone (n=40, range 2977.6-3123.9 ns)
   2977.6 |####################
   2984.9 |##########################
   2992.2 |#############
   2999.5 |##########################
   3006.8 |######
   3014.1 |
   3021.4 |######
   3028.8 |
   3036.1 |
   3043.4 |######
   3050.7 |######
   3058.0 |######
   3065.3 |
   3072.7 |
   3080.0 |######
   3087.3 |####################
   3094.6 |########################################
   3101.9 |########################################
   3109.2 |######
   3116.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.66 (measurement drift or warm-up artifact)
