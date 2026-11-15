# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-ragged) are a dead heat (<1%)

wide-rung-wordround-alias (1.77 us) and wide-rung-ragged (1.77 us) differ by 0.02%, inside the noise, even though the wider field spreads 46.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged-overread shows alternating (throttle bounce) (autocorr -0.79)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (825 ns) is smaller than the fastest variant's own run-to-run std-dev (1.53 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {wide-rung-wordround-alias, wide-rung-ragged, wide-rung-wordround, wide-rung-align16} vs {wide-rung-ragged-overread} (46% apart)

The field splits into a fast tier {wide-rung-wordround-alias, wide-rung-ragged, wide-rung-wordround, wide-rung-align16} and a slow tier {wide-rung-ragged-overread} with a 46% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged is inconsistent: worst-20% is 4.1x its best-20%

wide-rung-ragged's best 20% of batches run at 1.53 us but its worst 20% at 6.31 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1773.7 ns median (-0.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.47x (fastest 1773.7 ns, slowest 2599.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3254ns | 1869ns | 1854ns | 2730ns | 6227ns | base |
| wide-rung-ragged | 3132ns | 1859ns | 1603ns | 2461ns | 6673ns | -3.75% |
| wide-rung-ragged-overread | 2837ns | 2738ns | 1484ns | 2391ns | 5528ns | -12.82% |
| wide-rung-wordround | 3209ns | 1860ns | 1855ns | 2576ns | 6463ns | -1.37% |
| wide-rung-wordround-alias | 2783ns | 1856ns | 1501ns | 2312ns | 5476ns | -14.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3094ns | 1770ns | 5878ns | base | 0.662 |
| wide-rung-ragged | 2978ns | 1531ns | 6307ns | -3.76% | 0.688 |
| wide-rung-ragged-overread | 2692ns | 1416ns | 5234ns | -12.99% | 0.761 |
| wide-rung-wordround | 3052ns | 1771ns | 6115ns | -1.37% | 0.671 |
| wide-rung-wordround-alias | 2648ns | 1432ns | 5178ns | -14.43% | 0.774 |

## Performance model

- Peak throughput: **1.447 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.148 | 79.3% |
| wide-rung-ragged | 1.154 | 79.8% |
| wide-rung-ragged-overread | 0.788 | 54.5% |
| wide-rung-wordround | 1.154 | 79.8% |
| wide-rung-wordround-alias | 1.155 | 79.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3254ns | 3254ns | base |
| wide-rung-ragged | 3132ns | 3132ns | -3.75% |
| wide-rung-ragged-overread | 2837ns | 2837ns | -12.82% |
| wide-rung-wordround | 3209ns | 3209ns | -1.37% |
| wide-rung-wordround-alias | 2783ns | 2783ns | -14.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1785ns | base | --- | [1774, 3628] | --- | --- | --- | --- |
| wide-rung-ragged | 1774ns | no significant difference | [-9, +6]ns | [1773, 2131] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-ragged-overread | 2599ns | no significant difference | [-359, +63]ns | [1774, 2603] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-wordround | 1774ns | no significant difference | [-1, +9]ns | [1774, 2292] | no | 0.5728 | 0.4296 | 0 |
| wide-rung-wordround-alias | 1774ns | -171.9ns (-9.6%) | [-360, -1]ns | [1772, 1921] | YES | 0.0257 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4875ns | -70.1% | -46.6% | -63.4% | -44.8% |
| 1 | 2 | 1780ns | -0.2% | -20.5% | +0.0% | +84.8% |
| 1 | 3 | 1774ns | +0.8% | +46.7% | +0.4% | +0.0% |
| 1 | 4 | 1774ns | -0.1% | +0.3% | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.082 | ok |
| wide-rung-ragged | -0.068 | ok |
| wide-rung-ragged-overread | -0.789 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.168 | ok |
| wide-rung-wordround-alias | 0.058 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 1/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 1/4, lost 1/4
- **wide-rung-wordround-alias**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 42.4ns | 3094.1ns | 1.4% |  |
| wide-rung-ragged | 41.0ns | 2977.7ns | 1.4% |  |
| wide-rung-ragged-overread | 39.9ns | 2692.2ns | 1.5% |  |
| wide-rung-wordround | 42.7ns | 3051.8ns | 1.4% |  |
| wide-rung-wordround-alias | 36.4ns | 2647.7ns | 1.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1770.3-5878.5 ns)
   1770.3 |########################################
   1975.7 |
   2181.1 |
   2386.5 |#
   2591.9 |
   2797.3 |
   3002.8 |#
   3208.2 |
   3413.6 |
   3619.0 |
   3824.4 |
   4029.8 |#
   4235.2 |
   4440.6 |#
   4646.0 |#
   4851.4 |###
   5056.8 |###
   5262.2 |###
   5467.7 |#
   5673.1 |#
  (2 below, 3 above range)

wide-rung-ragged (n=40, range 1530.6-6307.1 ns)
   1530.6 |####
   1769.5 |########################################
   2008.3 |
   2247.1 |##
   2485.9 |
   2724.8 |
   2963.6 |####
   3202.4 |
   3441.2 |
   3680.1 |
   3918.9 |
   4157.7 |
   4396.5 |
   4635.4 |##
   4874.2 |
   5113.0 |
   5351.8 |##
   5590.6 |######
   5829.5 |
   6068.3 |##
  (6 below, 5 above range)

wide-rung-ragged-overread (n=40, range 1415.5-5233.9 ns)
   1415.5 |########
   1606.5 |#################
   1797.4 |##
   1988.3 |
   2179.2 |
   2370.1 |##
   2561.1 |########################################
   2752.0 |
   2942.9 |##
   3133.8 |
   3324.7 |
   3515.7 |
   3706.6 |##
   3897.5 |##
   4088.4 |
   4279.3 |
   4470.3 |##
   4661.2 |
   4852.1 |
   5043.0 |
  (7 below, 4 above range)

wide-rung-wordround (n=40, range 1771.3-6115.1 ns)
   1771.3 |########################################
   1988.5 |
   2205.7 |
   2422.9 |
   2640.1 |#
   2857.3 |
   3074.5 |
   3291.6 |
   3508.8 |
   3726.0 |#
   3943.2 |
   4160.4 |
   4377.6 |#
   4594.8 |
   4811.9 |
   5029.1 |#
   5246.3 |###
   5463.5 |#
   5680.7 |####
   5897.9 |
  (1 below, 4 above range)

wide-rung-wordround-alias (n=40, range 1432.2-5178.1 ns)
   1432.2 |###########
   1619.5 |########################################
   1806.8 |##
   1994.1 |
   2181.4 |##
   2368.7 |
   2556.0 |
   2743.3 |
   2930.6 |
   3117.9 |
   3305.2 |
   3492.5 |
   3679.8 |
   3867.1 |##
   4054.4 |
   4241.7 |##
   4429.0 |#########
   4616.3 |
   4803.6 |####
   4990.9 |####
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=57.0% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **wide-rung-ragged**: CV=65.0% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=55.8% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **wide-rung-wordround**: CV=60.5% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=57.8% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.6x (possible bimodal distribution)
