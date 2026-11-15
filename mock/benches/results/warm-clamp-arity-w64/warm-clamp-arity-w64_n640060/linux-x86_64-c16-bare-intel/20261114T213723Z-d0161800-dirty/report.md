# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 23% (significant)

warm-clamp-min-lanes is -484 ns (23%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.0x slower than the field

warm-clamp-min-lanes (3.84 us) is 2.0x the fastest (1.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-accfit-dyn, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-accfit-dyn (1.88 us) and warm-clamp-accfit (1.89 us) differ by 0.50%, inside the noise, even though the wider field spreads 104.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.76)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.96 us) is smaller than the fastest variant's own run-to-run std-dev (2.18 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.40 us but its worst 20% at 5.79 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 1877.4 ns median (-9.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.04x (fastest 1877.4 ns, slowest 3839.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4063ns | 2161ns | 1925ns | 3496ns | 7900ns | base |
| warm-clamp-accfit | 4135ns | 1971ns | 1951ns | 3790ns | 7355ns | +1.78% |
| warm-clamp-accfit-dyn | 3760ns | 1961ns | 1948ns | 3192ns | 7272ns | -7.46% |
| warm-clamp-head | 4532ns | 3125ns | 2561ns | 3558ns | 9427ns | +11.56% |
| warm-clamp-min-lanes | 3758ns | 4062ns | 1479ns | 3725ns | 6139ns | -7.48% |
| warm-clamp-minimum | 4352ns | 2713ns | 1859ns | 4060ns | 7722ns | +7.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3877ns | 1842ns | 7510ns | base | 2.113 |
| warm-clamp-accfit | 3946ns | 1867ns | 6979ns | +1.78% | 2.076 |
| warm-clamp-accfit-dyn | 3587ns | 1865ns | 6903ns | -7.49% | 2.284 |
| warm-clamp-head | 4399ns | 2493ns | 9116ns | +13.46% | 1.862 |
| warm-clamp-min-lanes | 3553ns | 1398ns | 5792ns | -8.38% | 2.306 |
| warm-clamp-minimum | 4146ns | 1778ns | 7313ns | +6.93% | 1.976 |

## Performance model

- Peak throughput: **5.861 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.963 | 67.6% |
| warm-clamp-accfit | 4.342 | 74.1% |
| warm-clamp-accfit-dyn | 4.363 | 74.4% |
| warm-clamp-head | 2.693 | 46.0% |
| warm-clamp-min-lanes | 2.134 | 36.4% |
| warm-clamp-minimum | 3.154 | 53.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4063ns | 4063ns | base |
| warm-clamp-accfit | 4135ns | 4135ns | +1.78% |
| warm-clamp-accfit-dyn | 3760ns | 3760ns | -7.46% |
| warm-clamp-head | 4532ns | 4532ns | +11.56% |
| warm-clamp-min-lanes | 3758ns | 3758ns | -7.48% |
| warm-clamp-minimum | 4352ns | 4352ns | +7.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2067ns | base | --- | [2015, 5607] | --- | --- | --- | --- |
| warm-clamp-accfit | 1887ns | no significant difference | [-158, +77]ns | [1877, 6033] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-accfit-dyn | 1877ns | -147.9ns (-7.2%) | [-196, -135]ns | [1867, 4496] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-head | 3042ns | +1014.1ns (+49.1%) | [+979, +1030]ns | [3033, 3057] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3839ns | -484.0ns (-23.4%) | [-713, -288]ns | [1667, 4765] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-minimum | 2597ns | no significant difference | [-45, +5]ns | [2014, 6153] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3516ns | -23.8% | +15.7% | -29.1% | +41.3% | -50.8% |
| 1 | 2 | 2015ns | -6.9% | -7.1% | +51.5% | +96.8% | +210.1% |
| 1 | 3 | 3108ns | +68.7% | -13.1% | -2.4% | +3.0% | -35.1% |
| 1 | 4 | 2110ns | -10.8% | -11.5% | +45.1% | +77.4% | +110.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.656 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.602 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.372 | moderate- |
| warm-clamp-head | -0.099 | ok |
| warm-clamp-min-lanes | 0.105 | ok |
| warm-clamp-minimum | -0.764 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 45.5ns | 3877.4ns | 1.2% |  |
| warm-clamp-accfit | 53.9ns | 3946.3ns | 1.4% |  |
| warm-clamp-accfit-dyn | 46.7ns | 3587.0ns | 1.3% |  |
| warm-clamp-head | 35.5ns | 4399.4ns | 0.8% |  |
| warm-clamp-min-lanes | 55.8ns | 3552.6ns | 1.6% |  |
| warm-clamp-minimum | 49.9ns | 4146.2ns | 1.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1842.1-7509.9 ns)
   1842.1 |########################################
   2125.5 |##
   2408.9 |##
   2692.3 |
   2975.7 |
   3259.0 |
   3542.4 |
   3825.8 |
   4109.2 |##
   4392.6 |
   4676.0 |
   4959.4 |##
   5242.8 |
   5526.1 |
   5809.5 |####
   6092.9 |##
   6376.3 |####
   6659.7 |#######
   6943.1 |
   7226.5 |#######
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 1867.3-6979.4 ns)
   1867.3 |########################################
   2122.9 |
   2378.5 |
   2634.1 |
   2889.7 |
   3145.3 |
   3400.9 |##
   3656.5 |
   3912.1 |
   4167.7 |##
   4423.3 |##
   4678.9 |
   4934.5 |##
   5190.1 |
   5445.7 |
   5701.4 |##
   5957.0 |##
   6212.6 |##
   6468.2 |####
   6723.8 |################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1864.8-6902.6 ns)
   1864.8 |########################################
   2116.7 |
   2368.6 |
   2620.5 |
   2872.4 |##
   3124.3 |
   3376.2 |##
   3628.1 |##
   3879.9 |
   4131.8 |
   4383.7 |
   4635.6 |
   4887.5 |
   5139.4 |##
   5391.3 |##
   5643.1 |
   5895.0 |####
   6146.9 |####
   6398.8 |##
   6650.7 |##########
  (4 below, 2 above range)

warm-clamp-head (n=40, range 2492.6-9116.4 ns)
   2492.6 |#########
   2823.8 |########################################
   3155.0 |
   3486.2 |
   3817.4 |
   4148.5 |
   4479.7 |
   4810.9 |
   5142.1 |#
   5473.3 |
   5804.5 |#
   6135.7 |
   6466.9 |
   6798.1 |
   7129.3 |
   7460.4 |
   7791.6 |#
   8122.8 |#
   8454.0 |#
   8785.2 |###
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1397.7-5792.5 ns)
   1397.7 |####################
   1617.4 |########################################
   1837.2 |
   2056.9 |#####
   2276.6 |
   2496.4 |
   2716.1 |
   2935.9 |
   3155.6 |
   3375.3 |##########
   3595.1 |##########
   3814.8 |#####
   4034.5 |
   4254.3 |###############
   4474.0 |
   4693.8 |####################
   4913.5 |###############
   5133.2 |
   5353.0 |##########
   5572.7 |###############
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 1777.9-7313.2 ns)
   1777.9 |########################################
   2054.7 |################
   2331.4 |
   2608.2 |
   2885.0 |####
   3161.7 |
   3438.5 |
   3715.3 |
   3992.0 |########
   4268.8 |
   4545.6 |
   4822.3 |
   5099.1 |
   5375.9 |####
   5652.6 |####
   5929.4 |####
   6206.2 |########
   6482.9 |########
   6759.7 |####
   7036.4 |####################
  (6 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=62.0% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=58.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=60.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-head**: CV=58.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=49.2% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=57.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
