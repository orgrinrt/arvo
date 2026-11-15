# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (2.40 us) and warm-clamp-min-lanes (2.40 us) differ by 0.04%, inside the noise, even though the wider field spreads 85.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.55)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.06 us) is smaller than the fastest variant's own run-to-run std-dev (2.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-accfit} (42% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-accfit} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.35 us but its worst 20% at 5.65 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2401.3 ns median (-36.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.86x (fastest 2401.3 ns, slowest 4458.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3770ns | 3968ns | 1655ns | 4018ns | 5145ns | base |
| warm-clamp-accfit | 3812ns | 4644ns | 1575ns | 3806ns | 6067ns | +1.11% |
| warm-clamp-accfit-dyn | 3690ns | 4189ns | 1407ns | 3702ns | 5937ns | -2.13% |
| warm-clamp-head | 3577ns | 3595ns | 1988ns | 3588ns | 5132ns | -5.13% |
| warm-clamp-min-lanes | 4122ns | 2466ns | 2464ns | 2994ns | 9164ns | +9.32% |
| warm-clamp-minimum | 3741ns | 2464ns | 2015ns | 2943ns | 7862ns | -0.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3586ns | 1578ns | 4873ns | base | 2.284 |
| warm-clamp-accfit | 3645ns | 1513ns | 5751ns | +1.65% | 2.247 |
| warm-clamp-accfit-dyn | 3525ns | 1345ns | 5645ns | -1.72% | 2.324 |
| warm-clamp-head | 3374ns | 1880ns | 4827ns | -5.92% | 2.428 |
| warm-clamp-min-lanes | 4002ns | 2401ns | 8859ns | +11.60% | 2.047 |
| warm-clamp-minimum | 3634ns | 1964ns | 7601ns | +1.32% | 2.255 |

## Performance model

- Peak throughput: **6.089 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.164 | 35.5% |
| warm-clamp-accfit | 1.838 | 30.2% |
| warm-clamp-accfit-dyn | 2.038 | 33.5% |
| warm-clamp-head | 2.407 | 39.5% |
| warm-clamp-min-lanes | 3.410 | 56.0% |
| warm-clamp-minimum | 3.411 | 56.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3770ns | 3770ns | base |
| warm-clamp-accfit | 3812ns | 3812ns | +1.11% |
| warm-clamp-accfit-dyn | 3690ns | 3690ns | -2.13% |
| warm-clamp-head | 3577ns | 3577ns | -5.13% |
| warm-clamp-min-lanes | 4122ns | 4122ns | +9.32% |
| warm-clamp-minimum | 3741ns | 3741ns | -0.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3786ns | base | --- | [3467, 4198] | --- | --- | --- | --- |
| warm-clamp-accfit | 4458ns | +197.1ns (+5.2%) | [+125, +771]ns | [1543, 5092] | YES | 0.0415 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 4019ns | +359.7ns (+9.5%) | [+97, +555]ns | [2012, 4583] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-head | 3404ns | -303.8ns (-8.0%) | [-690, -89]ns | [3150, 3576] | YES | 0.0415 | 0.0166 | 0 |
| warm-clamp-min-lanes | 2402ns | no significant difference | [-1065, +1056]ns | [2402, 2414] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-minimum | 2401ns | no significant difference | [-1305, +618]ns | [2400, 2465] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3467ns | +24.0% | +9.5% | -7.2% | -30.7% | -35.3% |
| 1 | 2 | 4320ns | +3.2% | +15.8% | -24.0% | -44.1% | -54.4% |
| 1 | 3 | 4247ns | -26.3% | -65.7% | +4.4% | -43.3% | -43.4% |
| 1 | 4 | 3999ns | -0.3% | +2.0% | -19.0% | -39.9% | -40.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.216 | moderate- |
| warm-clamp-accfit | -0.252 | moderate- |
| warm-clamp-accfit-dyn | -0.552 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.395 | moderate- |
| warm-clamp-min-lanes | -0.306 | moderate- |
| warm-clamp-minimum | -0.142 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 65.1ns | 3586.3ns | 1.8% |  |
| warm-clamp-accfit | 58.5ns | 3645.3ns | 1.6% |  |
| warm-clamp-accfit-dyn | 57.9ns | 3524.5ns | 1.6% |  |
| warm-clamp-head | 70.8ns | 3373.9ns | 2.1% |  |
| warm-clamp-min-lanes | 40.1ns | 4002.4ns | 1.0% |  |
| warm-clamp-minimum | 36.0ns | 3633.6ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1577.9-4872.6 ns)
   1577.9 |
   1742.7 |
   1907.4 |
   2072.1 |######
   2236.9 |
   2401.6 |######
   2566.3 |
   2731.1 |
   2895.8 |######
   3060.5 |#############
   3225.3 |#############
   3390.0 |####################
   3554.7 |
   3719.5 |########################################
   3884.2 |
   4048.9 |##########################
   4213.6 |######
   4378.4 |#################################
   4543.1 |######
   4707.8 |######
  (6 below, 6 above range)

warm-clamp-accfit (n=40, range 1513.3-5750.9 ns)
   1513.3 |########################################
   1725.2 |
   1937.1 |
   2149.0 |
   2360.9 |##
   2572.7 |
   2784.6 |
   2996.5 |
   3208.4 |
   3420.2 |##
   3632.1 |##
   3844.0 |
   4055.9 |
   4267.8 |##
   4479.6 |##
   4691.5 |###########
   4903.4 |##
   5115.3 |##
   5327.2 |######################
   5539.0 |#####
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1345.3-5645.4 ns)
   1345.3 |########################################
   1560.3 |####
   1775.3 |
   1990.3 |
   2205.3 |########
   2420.3 |
   2635.4 |
   2850.4 |
   3065.4 |
   3280.4 |
   3495.4 |####
   3710.4 |############
   3925.4 |
   4140.4 |########
   4355.4 |################
   4570.4 |####
   4785.4 |########
   5000.4 |####
   5215.4 |########################
   5430.4 |####
  (3 below, 3 above range)

warm-clamp-head (n=40, range 1880.3-4826.8 ns)
   1880.3 |
   2027.6 |
   2174.9 |
   2322.3 |
   2469.6 |
   2616.9 |
   2764.2 |#####
   2911.6 |##########
   3058.9 |########################################
   3206.2 |###############
   3353.5 |###################################
   3500.9 |
   3648.2 |#########################
   3795.5 |#####
   3942.8 |#####
   4090.2 |#####
   4237.5 |#####
   4384.8 |
   4532.1 |#####
   4679.5 |
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2401.2-8858.5 ns)
   2401.2 |########################################
   2724.1 |
   3047.0 |
   3369.8 |
   3692.7 |
   4015.6 |
   4338.4 |##
   4661.3 |
   4984.1 |
   5307.0 |
   5629.9 |
   5952.7 |#
   6275.6 |
   6598.5 |#
   6921.3 |
   7244.2 |#
   7567.1 |#
   7889.9 |#
   8212.8 |
   8535.6 |####
  (1 below, 2 above range)

warm-clamp-minimum (n=40, range 1964.1-7601.4 ns)
   1964.1 |#################
   2245.9 |########################################
   2527.8 |
   2809.7 |
   3091.5 |##
   3373.4 |##
   3655.3 |
   3937.1 |
   4219.0 |
   4500.9 |
   4782.7 |
   5064.6 |##
   5346.5 |
   5628.3 |
   5910.2 |
   6192.1 |#######
   6474.0 |#####
   6755.8 |
   7037.7 |#####
   7319.6 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=32.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=50.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=49.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-head**: CV=31.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=67.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=62.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
