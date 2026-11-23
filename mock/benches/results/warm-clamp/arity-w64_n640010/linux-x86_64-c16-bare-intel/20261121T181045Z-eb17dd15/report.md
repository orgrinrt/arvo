# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 2.0x slower than the field

warm-clamp-minimum (4.16 us) is 2.0x the fastest (2.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 105.2%)

warm-clamp-accfit-dyn wins on median (2.06 us) yet has the highest variance (CV 105.2%), while warm-clamp-minimum is the steadiest (CV 34.9%, 4.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-accfit-dyn, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-accfit-dyn (2.06 us) and warm-clamp-accfit (2.06 us) differ by 0.00%, inside the noise, even though the wider field spreads 101.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.79)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.10 us) is smaller than the fastest variant's own run-to-run std-dev (2.17 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.60 us but its worst 20% at 6.82 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2059.7 ns median (-48.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.02x (fastest 2059.7 ns, slowest 4157.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112579971 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3669ns | 4209ns | 1484ns | 3676ns | 5832ns | base |
| warm-clamp-accfit | 3613ns | 2143ns | 1751ns | 3099ns | 7017ns | -1.52% |
| warm-clamp-accfit-dyn | 3763ns | 2140ns | 1671ns | 3333ns | 7146ns | +2.58% |
| warm-clamp-head | 4082ns | 3136ns | 2566ns | 2952ns | 8990ns | +11.28% |
| warm-clamp-min-lanes | 3487ns | 4004ns | 1328ns | 3498ns | 5612ns | -4.96% |
| warm-clamp-minimum | 3916ns | 4376ns | 1517ns | 4120ns | 5703ns | +6.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3476ns | 1410ns | 5505ns | base | 2.357 |
| warm-clamp-accfit | 3462ns | 1684ns | 6700ns | -0.40% | 2.366 |
| warm-clamp-accfit-dyn | 3606ns | 1604ns | 6818ns | +3.74% | 2.272 |
| warm-clamp-head | 3966ns | 2499ns | 8704ns | +14.09% | 2.066 |
| warm-clamp-min-lanes | 3295ns | 1261ns | 5296ns | -5.20% | 2.486 |
| warm-clamp-minimum | 3711ns | 1441ns | 5390ns | +6.76% | 2.207 |

## Performance model

- Peak throughput: **6.499 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.050 | 31.5% |
| warm-clamp-accfit | 3.977 | 61.2% |
| warm-clamp-accfit-dyn | 3.977 | 61.2% |
| warm-clamp-head | 2.682 | 41.3% |
| warm-clamp-min-lanes | 2.161 | 33.2% |
| warm-clamp-minimum | 1.970 | 30.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3669ns | 3669ns | base |
| warm-clamp-accfit | 3613ns | 3613ns | -1.52% |
| warm-clamp-accfit-dyn | 3763ns | 3763ns | +2.58% |
| warm-clamp-head | 4082ns | 4082ns | +11.28% |
| warm-clamp-min-lanes | 3487ns | 3487ns | -4.96% |
| warm-clamp-minimum | 3916ns | 3916ns | +6.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3997ns | base | --- | [2136, 4489] | --- | --- | --- | --- |
| warm-clamp-accfit | 2060ns | +271.6ns (+6.8%) | [+46, +503]ns | [1692, 5189] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 2060ns | +131.9ns (+3.3%) | [+95, +790]ns | [1686, 4981] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-head | 3054ns | no significant difference | [-1177, +1501]ns | [2510, 3059] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-min-lanes | 3792ns | -183.8ns (-4.6%) | [-538, -2]ns | [2070, 4207] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-minimum | 4158ns | no significant difference | [-48, +26]ns | [3722, 4425] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4329ns | -60.9% | -52.3% | -29.3% | -12.4% | -7.8% |
| 1 | 2 | 1562ns | +124.0% | +104.1% | +95.9% | +154.0% | +196.5% |
| 1 | 3 | 4638ns | -55.4% | -63.6% | -46.1% | -38.8% | -29.8% |
| 1 | 4 | 3893ns | -56.6% | -18.2% | -35.7% | -64.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.563 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.365 | moderate- |
| warm-clamp-accfit-dyn | -0.790 | HIGH- (thermal bounce) |
| warm-clamp-head | 0.248 | moderate+ |
| warm-clamp-min-lanes | 0.207 | moderate+ |
| warm-clamp-minimum | -0.424 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 55.2ns | 3476.1ns | 1.6% |  |
| warm-clamp-accfit | 42.0ns | 3462.2ns | 1.2% |  |
| warm-clamp-accfit-dyn | 46.8ns | 3605.9ns | 1.3% |  |
| warm-clamp-head | 32.6ns | 3965.9ns | 0.8% |  |
| warm-clamp-min-lanes | 54.1ns | 3295.3ns | 1.6% |  |
| warm-clamp-minimum | 59.1ns | 3711.0ns | 1.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1409.6-5504.7 ns)
   1409.6 |########################################
   1614.3 |
   1819.1 |
   2023.9 |
   2228.6 |
   2433.4 |
   2638.1 |####
   2842.9 |####
   3047.6 |
   3252.4 |
   3457.1 |####
   3661.9 |########
   3866.6 |############
   4071.4 |####
   4276.2 |############
   4480.9 |################
   4685.7 |####
   4890.4 |########
   5095.2 |########
   5299.9 |
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 1684.4-6700.4 ns)
   1684.4 |########################################
   1935.2 |######################
   2186.0 |
   2436.8 |##
   2687.6 |
   2938.4 |
   3189.2 |
   3440.0 |
   3690.8 |
   3941.6 |
   4192.4 |
   4443.2 |
   4694.0 |
   4944.8 |##
   5195.6 |#####
   5446.4 |##
   5697.2 |#####
   5948.0 |##
   6198.8 |###########
   6449.6 |##
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1603.7-6817.9 ns)
   1603.7 |########################################
   1864.4 |#####################
   2125.1 |
   2385.8 |
   2646.5 |
   2907.2 |
   3167.9 |
   3428.7 |
   3689.4 |
   3950.1 |#######
   4210.8 |
   4471.5 |###
   4732.2 |###
   4992.9 |###
   5253.6 |###
   5514.3 |##############
   5775.0 |###
   6035.8 |#######
   6296.5 |
   6557.2 |###
  (5 below, 4 above range)

warm-clamp-head (n=40, range 2499.1-8704.4 ns)
   2499.1 |##############################
   2809.3 |########################################
   3119.6 |
   3429.9 |
   3740.1 |
   4050.4 |
   4360.7 |
   4670.9 |
   4981.2 |
   5291.5 |
   5601.8 |
   5912.0 |
   6222.3 |
   6532.6 |##
   6842.8 |
   7153.1 |##
   7463.4 |##
   7773.6 |##
   8083.9 |
   8394.2 |##
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1260.5-5295.8 ns)
   1260.5 |########################################
   1462.3 |##################################
   1664.1 |
   1865.8 |
   2067.6 |
   2269.4 |
   2471.1 |#####
   2672.9 |#####
   2874.6 |
   3076.4 |
   3278.2 |
   3479.9 |###########
   3681.7 |#################
   3883.5 |######################
   4085.2 |#################
   4287.0 |#################
   4488.7 |#####
   4690.5 |#####
   4892.3 |#####
   5094.0 |###########
  (1 below, 4 above range)

warm-clamp-minimum (n=40, range 1441.4-5389.9 ns)
   1441.4 |########################################
   1638.8 |######
   1836.2 |######
   2033.6 |
   2231.1 |
   2428.5 |
   2625.9 |
   2823.3 |
   3020.8 |
   3218.2 |
   3415.6 |######
   3613.0 |##########################
   3810.5 |#############
   4007.9 |#############
   4205.3 |########################################
   4402.7 |######
   4600.2 |##########################
   4797.6 |######
   4995.0 |#############
   5192.4 |#############
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=46.5% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=62.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-head**: CV=62.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=47.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=39.1% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
