# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 2.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head beats baseline by 33% (significant)

warm-clamp-head is -1.25 us (33%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is fastest but the noisiest (CV 106.3%)

warm-clamp-minimum wins on median (2.34 us) yet has the highest variance (CV 106.3%), while warm-clamp-head is the steadiest (CV 14.3%, 2.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-minimum, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-minimum (2.34 us) and warm-clamp-accfit (2.34 us) differ by 0.14%, inside the noise, even though the wider field spreads 59.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.40 us) is smaller than the fastest variant's own run-to-run std-dev (2.48 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes} vs {warm-clamp-acc64} (46% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes} and a slow tier {warm-clamp-acc64} with a 46% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-clamp-minimum vs stability leader warm-clamp-head (+9% speed for 7.4x steadier)

warm-clamp-minimum is fastest (2.34 us, CV 106.3%); warm-clamp-head gives up 9.1% median for 7.4x lower variance (CV 14.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-minimum is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.91 us but its worst 20% at 8.26 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2335.4 ns median (-37.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.60x (fastest 2335.4 ns, slowest 3734.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3507ns | 3923ns | 1321ns | 3533ns | 5615ns | base |
| warm-clamp-accfit | 4116ns | 2402ns | 1960ns | 3557ns | 7949ns | +17.38% |
| warm-clamp-accfit-dyn | 3935ns | 2511ns | 1962ns | 3350ns | 7660ns | +12.20% |
| warm-clamp-head | 2912ns | 2787ns | 2457ns | 2844ns | 3570ns | -16.97% |
| warm-clamp-min-lanes | 4143ns | 2669ns | 2203ns | 3578ns | 7778ns | +18.15% |
| warm-clamp-minimum | 3670ns | 2398ns | 1966ns | 2609ns | 8558ns | +4.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3325ns | 1256ns | 5296ns | base | 2.464 |
| warm-clamp-accfit | 3995ns | 1909ns | 7677ns | +20.15% | 2.051 |
| warm-clamp-accfit-dyn | 3805ns | 1911ns | 7357ns | +14.45% | 2.153 |
| warm-clamp-head | 2656ns | 2246ns | 3238ns | -20.14% | 3.085 |
| warm-clamp-min-lanes | 4012ns | 2146ns | 7506ns | +20.64% | 2.042 |
| warm-clamp-minimum | 3559ns | 1914ns | 8260ns | +7.04% | 2.302 |

## Performance model

- Peak throughput: **6.525 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.194 | 33.6% |
| warm-clamp-accfit | 3.503 | 53.7% |
| warm-clamp-accfit-dyn | 3.374 | 51.7% |
| warm-clamp-head | 3.215 | 49.3% |
| warm-clamp-min-lanes | 3.207 | 49.2% |
| warm-clamp-minimum | 3.508 | 53.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3507ns | 3507ns | base |
| warm-clamp-accfit | 4116ns | 4116ns | +17.38% |
| warm-clamp-accfit-dyn | 3935ns | 3935ns | +12.20% |
| warm-clamp-head | 2912ns | 2912ns | -16.97% |
| warm-clamp-min-lanes | 4143ns | 4143ns | +18.15% |
| warm-clamp-minimum | 3670ns | 3670ns | +4.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3734ns | base | --- | [2928, 4264] | --- | --- | --- | --- |
| warm-clamp-accfit | 2339ns | +1070.3ns (+28.7%) | [+646, +1213]ns | [2332, 4958] | YES (adj: no) | 0.0829 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 2428ns | +654.9ns (+17.5%) | [+286, +1137]ns | [2335, 3496] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-clamp-head | 2548ns | -1246.3ns (-33.4%) | [-1672, -186]ns | [2542, 2632] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-min-lanes | 2554ns | no significant difference | [-48, +1295]ns | [2372, 4935] | no | 0.1009 | 0.0807 | 0 |
| warm-clamp-minimum | 2335ns | no significant difference | [-1024, +1077]ns | [2334, 2346] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1268ns | +50.9% | +51.1% | +134.1% | +234.6% | +84.2% |
| 1 | 2 | 4111ns | +31.6% | -53.3% | -38.2% | -5.9% | -53.5% |
| 1 | 3 | 4442ns | -47.3% | -43.3% | -40.5% | -42.5% | -47.4% |
| 1 | 4 | 3646ns | -47.5% | -35.8% | -32.0% | -35.1% | -35.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.075 | ok |
| warm-clamp-accfit | -0.388 | moderate- |
| warm-clamp-accfit-dyn | 0.129 | ok |
| warm-clamp-head | -0.223 | moderate- |
| warm-clamp-min-lanes | 0.304 | moderate+ |
| warm-clamp-minimum | -0.358 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 63.2ns | 3325.2ns | 1.9% |  |
| warm-clamp-accfit | 40.4ns | 3995.1ns | 1.0% |  |
| warm-clamp-accfit-dyn | 44.4ns | 3805.5ns | 1.2% |  |
| warm-clamp-head | 90.1ns | 2655.5ns | 3.4% |  |
| warm-clamp-min-lanes | 44.3ns | 4011.5ns | 1.1% |  |
| warm-clamp-minimum | 35.9ns | 3559.1ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1255.5-5296.1 ns)
   1255.5 |########################################
   1457.5 |
   1659.6 |
   1861.6 |
   2063.6 |
   2265.7 |
   2467.7 |#####
   2669.7 |
   2871.7 |
   3073.8 |#####
   3275.8 |#####
   3477.8 |#################
   3679.8 |#################
   3881.9 |#####
   4083.9 |#################
   4285.9 |#################
   4488.0 |############################
   4690.0 |
   4892.0 |#####
   5094.0 |#####
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 1909.0-7676.7 ns)
   1909.0 |#########################
   2197.4 |########################################
   2485.7 |
   2774.1 |
   3062.5 |
   3350.9 |###
   3639.3 |
   3927.7 |
   4216.1 |###
   4504.4 |###
   4792.8 |
   5081.2 |###
   5369.6 |
   5658.0 |
   5946.4 |#######
   6234.8 |#######
   6523.1 |###
   6811.5 |#######
   7099.9 |#######
   7388.3 |
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1910.8-7356.6 ns)
   1910.8 |########################################
   2183.1 |###############################
   2455.4 |##########################
   2727.7 |
   3000.0 |
   3272.3 |
   3544.6 |
   3816.8 |
   4089.1 |
   4361.4 |####
   4633.7 |
   4906.0 |
   5178.3 |
   5450.6 |
   5722.9 |
   5995.1 |
   6267.4 |#################
   6539.7 |####
   6812.0 |#################
   7084.3 |####
  (4 below, 3 above range)

warm-clamp-head (n=40, range 2245.6-3237.9 ns)
   2245.6 |
   2295.2 |##############################
   2344.8 |
   2394.5 |##########
   2444.1 |
   2493.7 |##############################
   2543.3 |########################################
   2592.9 |##########
   2642.5 |##########
   2692.1 |
   2741.7 |
   2791.4 |#########################
   2841.0 |
   2890.6 |#####
   2940.2 |
   2989.8 |
   3039.4 |
   3089.0 |#####
   3138.6 |#####
   3188.3 |#####
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2145.7-7505.8 ns)
   2145.7 |########################################
   2413.7 |#############################
   2681.7 |
   2949.7 |###
   3217.7 |
   3485.7 |###
   3753.7 |
   4021.7 |
   4289.7 |###
   4557.7 |
   4825.7 |
   5093.7 |
   5361.7 |#######
   5629.7 |###
   5897.7 |###
   6165.7 |###
   6433.7 |#######
   6701.7 |
   6969.7 |#######
   7237.7 |#######
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 1914.2-8259.9 ns)
   1914.2 |###
   2231.5 |########################################
   2548.7 |
   2866.0 |
   3183.3 |
   3500.6 |#
   3817.9 |
   4135.2 |
   4452.5 |
   4769.7 |
   5087.0 |
   5404.3 |
   5721.6 |#
   6038.9 |#
   6356.2 |
   6673.4 |
   6990.7 |#
   7308.0 |
   7625.3 |###
   7942.6 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=47.8% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=59.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=60.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=54.0% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=69.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
