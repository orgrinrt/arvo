# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 22% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (2.32 us) leads warm-clamp-accfit-dyn (2.84 us) by 22%, a clear separation rather than a photo finish. CV 22.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.03 us but its worst 20% at 4.59 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2323.9 ns median (-18.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.63x (fastest 2323.9 ns, slowest 3798.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3041ns | 3104ns | 1728ns | 3147ns | 4033ns | base |
| warm-clamp-accfit | 3252ns | 3298ns | 2366ns | 3322ns | 3929ns | +6.95% |
| warm-clamp-accfit-dyn | 3089ns | 3075ns | 2459ns | 3096ns | 3698ns | +1.60% |
| warm-clamp-head | 2799ns | 2564ns | 2285ns | 2635ns | 3802ns | -7.96% |
| warm-clamp-min-lanes | 3368ns | 3823ns | 1084ns | 3617ns | 4903ns | +10.76% |
| warm-clamp-minimum | 3945ns | 3979ns | 2040ns | 4054ns | 5522ns | +29.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2827ns | 1615ns | 3714ns | base | 2.898 |
| warm-clamp-accfit | 3029ns | 2217ns | 3623ns | +7.15% | 2.705 |
| warm-clamp-accfit-dyn | 2876ns | 2294ns | 3435ns | +1.72% | 2.849 |
| warm-clamp-head | 2520ns | 2067ns | 3424ns | -10.85% | 3.251 |
| warm-clamp-min-lanes | 3174ns | 1027ns | 4586ns | +12.27% | 2.581 |
| warm-clamp-minimum | 3748ns | 1943ns | 5234ns | +32.59% | 2.186 |

## Performance model

- Peak throughput: **7.977 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.858 | 35.8% |
| warm-clamp-accfit | 2.650 | 33.2% |
| warm-clamp-accfit-dyn | 2.888 | 36.2% |
| warm-clamp-head | 3.525 | 44.2% |
| warm-clamp-min-lanes | 2.262 | 28.4% |
| warm-clamp-minimum | 2.157 | 27.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3041ns | 3041ns | base |
| warm-clamp-accfit | 3252ns | 3252ns | +6.95% |
| warm-clamp-accfit-dyn | 3089ns | 3089ns | +1.60% |
| warm-clamp-head | 2799ns | 2799ns | -7.96% |
| warm-clamp-min-lanes | 3368ns | 3368ns | +10.76% |
| warm-clamp-minimum | 3945ns | 3945ns | +29.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2866ns | base | --- | [2678, 3300] | --- | --- | --- | --- |
| warm-clamp-accfit | 3092ns | +61.5ns (+2.1%) | [+3, +213]ns | [2888, 3346] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-accfit-dyn | 2836ns | no significant difference | [-321, +109]ns | [2777, 3032] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-head | 2324ns | -516.5ns (-18.0%) | [-791, -156]ns | [2226, 2413] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3621ns | +704.9ns (+24.6%) | [+158, +904]ns | [3319, 3928] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-minimum | 3798ns | +1250.1ns (+43.6%) | [+455, +1317]ns | [3644, 4089] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2394ns | +3.6% | +2.7% | -4.5% | +51.3% | +55.4% |
| 1 | 2 | 3339ns | -3.5% | -16.8% | -32.1% | +18.7% | +6.4% |
| 1 | 3 | 3033ns | +5.1% | -0.1% | +7.4% | +13.9% | +20.9% |
| 1 | 4 | 2979ns | +13.3% | +8.6% | -26.7% | -46.3% | +42.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.375 | moderate- |
| warm-clamp-accfit | -0.075 | ok |
| warm-clamp-accfit-dyn | 0.241 | moderate+ |
| warm-clamp-head | -0.474 | moderate- |
| warm-clamp-min-lanes | 0.043 | ok |
| warm-clamp-minimum | -0.029 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 74.4ns | 2826.8ns | 2.6% |  |
| warm-clamp-accfit | 78.6ns | 3029.0ns | 2.6% |  |
| warm-clamp-accfit-dyn | 73.8ns | 2875.5ns | 2.6% |  |
| warm-clamp-head | 76.7ns | 2520.2ns | 3.0% |  |
| warm-clamp-min-lanes | 64.5ns | 3173.5ns | 2.0% |  |
| warm-clamp-minimum | 66.1ns | 3747.9ns | 1.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1615.3-3713.6 ns)
   1615.3 |
   1720.2 |
   1825.1 |
   1930.0 |
   2034.9 |
   2139.8 |###########
   2244.8 |
   2349.7 |######################
   2454.6 |
   2559.5 |######################
   2664.4 |#####
   2769.3 |############################
   2874.2 |#####
   2979.2 |###########
   3084.1 |###########
   3189.0 |#####
   3293.9 |########################################
   3398.8 |###########
   3503.7 |
   3608.6 |###########
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 2216.6-3623.4 ns)
   2216.6 |####
   2287.0 |
   2357.3 |############
   2427.6 |############
   2498.0 |
   2568.3 |########
   2638.7 |####
   2709.0 |
   2779.3 |########
   2849.7 |########
   2920.0 |
   2990.3 |############
   3060.7 |############
   3131.0 |
   3201.3 |
   3271.7 |############
   3342.0 |########################################
   3412.4 |########
   3482.7 |####
   3553.0 |
  (1 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2294.4-3434.7 ns)
   2294.4 |
   2351.4 |##########################
   2408.4 |######
   2465.4 |
   2522.4 |####################
   2579.5 |####################
   2636.5 |
   2693.5 |
   2750.5 |########################################
   2807.5 |#############
   2864.6 |######
   2921.6 |
   2978.6 |#################################
   3035.6 |#############
   3092.6 |######
   3149.6 |
   3206.7 |######
   3263.7 |
   3320.7 |#################################
   3377.7 |####################
  (2 below, 1 above range)

warm-clamp-head (n=40, range 2067.3-3424.3 ns)
   2067.3 |########
   2135.1 |########
   2203.0 |########################################
   2270.8 |########
   2338.7 |####
   2406.5 |###############################
   2474.4 |
   2542.2 |####
   2610.1 |########
   2677.9 |
   2745.8 |
   2813.6 |
   2881.5 |########
   2949.3 |####
   3017.2 |
   3085.0 |
   3152.9 |####
   3220.7 |
   3288.6 |####
   3356.4 |####
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1026.9-4586.0 ns)
   1026.9 |########################
   1204.9 |
   1382.8 |########
   1560.8 |
   1738.7 |
   1916.7 |
   2094.6 |########
   2272.6 |
   2450.5 |########
   2628.5 |
   2806.4 |
   2984.4 |########
   3162.4 |########
   3340.3 |################################
   3518.3 |########################
   3696.2 |########################################
   3874.2 |################
   4052.1 |################################
   4230.1 |########################
   4408.0 |################
  (6 below, 3 above range)

warm-clamp-minimum (n=40, range 1942.6-5233.9 ns)
   1942.6 |
   2107.2 |
   2271.8 |
   2436.3 |#####
   2600.9 |
   2765.4 |
   2930.0 |#####
   3094.6 |###########
   3259.1 |#####
   3423.7 |###########
   3588.2 |########################################
   3752.8 |#################
   3917.4 |######################
   4081.9 |#################
   4246.5 |#####
   4411.1 |#####
   4575.6 |#####
   4740.2 |###########
   4904.8 |###########
   5069.3 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=28.1% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=20.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=41.6% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=30.6% (high variance, measurements may be unstable)
