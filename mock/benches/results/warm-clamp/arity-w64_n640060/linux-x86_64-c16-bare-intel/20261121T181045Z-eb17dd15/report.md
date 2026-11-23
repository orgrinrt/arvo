# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 21% (significant)

warm-clamp-min-lanes is -451 ns (21%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.2x slower than the field

warm-clamp-minimum (4.12 us) is 2.2x the fastest (1.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-accfit, warm-clamp-accfit-dyn) are a dead heat (<1%)

warm-clamp-accfit (1.88 us) and warm-clamp-accfit-dyn (1.89 us) differ by 0.53%, inside the noise, even though the wider field spreads 119.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.60)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64} vs {warm-clamp-head, warm-clamp-min-lanes, warm-clamp-minimum} (42% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64} and a slow tier {warm-clamp-head, warm-clamp-min-lanes, warm-clamp-minimum} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.35 us but its worst 20% at 5.64 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 1875.5 ns median (-12.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.20x (fastest 1875.5 ns, slowest 4120.9 ns)

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
| warm-clamp-acc64 | 3700ns | 2234ns | 1828ns | 3085ns | 7417ns | base |
| warm-clamp-accfit | 3345ns | 1962ns | 1949ns | 2555ns | 7109ns | -9.61% |
| warm-clamp-accfit-dyn | 3774ns | 1967ns | 1725ns | 3454ns | 6785ns | +2.00% |
| warm-clamp-head | 4389ns | 3118ns | 2621ns | 3312ns | 9387ns | +18.61% |
| warm-clamp-min-lanes | 3479ns | 3643ns | 1422ns | 3331ns | 5981ns | -5.96% |
| warm-clamp-minimum | 4144ns | 4289ns | 1886ns | 3926ns | 7057ns | +12.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3542ns | 1756ns | 7078ns | base | 2.313 |
| warm-clamp-accfit | 3193ns | 1869ns | 6766ns | -9.87% | 2.566 |
| warm-clamp-accfit-dyn | 3606ns | 1654ns | 6448ns | +1.80% | 2.272 |
| warm-clamp-head | 4262ns | 2552ns | 9087ns | +20.32% | 1.922 |
| warm-clamp-min-lanes | 3300ns | 1354ns | 5638ns | -6.84% | 2.482 |
| warm-clamp-minimum | 3969ns | 1811ns | 6739ns | +12.03% | 2.064 |

## Performance model

- Peak throughput: **6.051 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.823 | 63.2% |
| warm-clamp-accfit | 4.368 | 72.2% |
| warm-clamp-accfit-dyn | 4.345 | 71.8% |
| warm-clamp-head | 2.699 | 44.6% |
| warm-clamp-min-lanes | 2.362 | 39.0% |
| warm-clamp-minimum | 1.988 | 32.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3700ns | 3700ns | base |
| warm-clamp-accfit | 3345ns | 3345ns | -9.61% |
| warm-clamp-accfit-dyn | 3774ns | 3774ns | +2.00% |
| warm-clamp-head | 4389ns | 4389ns | +18.61% |
| warm-clamp-min-lanes | 3479ns | 3479ns | -5.96% |
| warm-clamp-minimum | 4144ns | 4144ns | +12.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2143ns | base | --- | [2108, 4166] | --- | --- | --- | --- |
| warm-clamp-accfit | 1875ns | no significant difference | [-248, +106]ns | [1871, 2093] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-accfit-dyn | 1885ns | no significant difference | [-246, +57]ns | [1865, 5474] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-head | 3035ns | +924.3ns (+43.1%) | [+866, +1270]ns | [3027, 3041] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3468ns | -450.5ns (-21.0%) | [-475, -401]ns | [1655, 4681] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 4121ns | +407.7ns (+19.0%) | [+254, +737]ns | [2195, 5062] | YES (adj: no) | 0.0641 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1755ns | +7.0% | +91.1% | +72.9% | -22.6% | +25.3% |
| 1 | 2 | 2118ns | -11.4% | -11.2% | +43.8% | +52.8% | +128.2% |
| 1 | 3 | 2170ns | +66.1% | +27.3% | +39.7% | +109.8% | +109.7% |
| 1 | 4 | 3208ns | -41.6% | -31.3% | -22.5% | +21.5% | +70.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.008 | ok |
| warm-clamp-accfit | -0.418 | moderate- |
| warm-clamp-accfit-dyn | -0.599 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.067 | ok |
| warm-clamp-min-lanes | 0.147 | ok |
| warm-clamp-minimum | -0.109 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 40.1ns | 3542.4ns | 1.1% |  |
| warm-clamp-accfit | 41.2ns | 3192.8ns | 1.3% |  |
| warm-clamp-accfit-dyn | 47.2ns | 3606.2ns | 1.3% |  |
| warm-clamp-head | 34.4ns | 4262.3ns | 0.8% |  |
| warm-clamp-min-lanes | 48.4ns | 3300.2ns | 1.5% |  |
| warm-clamp-minimum | 45.0ns | 3968.8ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1756.5-7077.6 ns)
   1756.5 |##################
   2022.5 |########################################
   2288.6 |
   2554.6 |
   2820.7 |
   3086.7 |
   3352.8 |
   3618.9 |###
   3884.9 |
   4151.0 |
   4417.0 |###
   4683.1 |
   4949.1 |###
   5215.2 |###
   5481.2 |###
   5747.3 |###
   6013.3 |######
   6279.4 |###
   6545.5 |
   6811.5 |######
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 1868.7-6766.0 ns)
   1868.7 |########################################
   2113.6 |#
   2358.4 |
   2603.3 |
   2848.2 |
   3093.0 |#
   3337.9 |
   3582.8 |#
   3827.6 |
   4072.5 |
   4317.3 |
   4562.2 |
   4807.1 |#
   5051.9 |#
   5296.8 |#
   5541.7 |#
   5786.5 |#
   6031.4 |
   6276.2 |
   6521.1 |#####
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1653.8-6448.4 ns)
   1653.8 |########################################
   1893.5 |
   2133.2 |
   2373.0 |##
   2612.7 |
   2852.4 |
   3092.1 |
   3331.9 |
   3571.6 |##
   3811.3 |##
   4051.1 |
   4290.8 |
   4530.5 |
   4770.3 |
   5010.0 |##
   5249.7 |##
   5489.5 |##########
   5729.2 |##
   5968.9 |##########
   6208.7 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 2551.6-9087.1 ns)
   2551.6 |
   2878.4 |########################################
   3205.1 |
   3531.9 |
   3858.7 |
   4185.5 |
   4512.2 |
   4839.0 |#
   5165.8 |
   5492.6 |#
   5819.4 |
   6146.1 |
   6472.9 |
   6799.7 |
   7126.5 |
   7453.2 |
   7780.0 |
   8106.8 |###
   8433.6 |#
   8760.3 |#
  (7 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1353.8-5637.6 ns)
   1353.8 |########################################
   1567.9 |########################################
   1782.1 |
   1996.3 |
   2210.5 |
   2424.7 |
   2638.9 |
   2853.1 |
   3067.3 |
   3281.5 |#####
   3495.7 |#####
   3709.8 |
   3924.0 |
   4138.2 |
   4352.4 |###############
   4566.6 |####################
   4780.8 |###############
   4995.0 |#####
   5209.2 |####################
   5423.4 |#####
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 1810.8-6739.0 ns)
   1810.8 |
   2057.2 |########################################
   2303.6 |
   2550.0 |
   2796.4 |
   3042.8 |
   3289.2 |
   3535.6 |###
   3782.1 |###
   4028.5 |###
   4274.9 |
   4521.3 |
   4767.7 |##################
   5014.1 |###
   5260.5 |##############
   5506.9 |###
   5753.4 |###
   5999.8 |#######
   6246.2 |###
   6492.6 |###
  (7 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=61.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=63.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=57.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=58.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=54.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=49.1% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
