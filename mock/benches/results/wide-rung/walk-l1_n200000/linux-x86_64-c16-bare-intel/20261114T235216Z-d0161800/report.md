# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-wordround-alias (1.79 us) and wide-rung-ragged-overread (1.80 us) differ by 0.65%, inside the noise, even though the wider field spreads 86.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.55 us) is smaller than the fastest variant's own run-to-run std-dev (1.78 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {wide-rung-wordround-alias, wide-rung-ragged-overread, wide-rung-ragged} vs {wide-rung-align16, wide-rung-wordround} (71% apart)

The field splits into a fast tier {wide-rung-wordround-alias, wide-rung-ragged-overread, wide-rung-ragged} and a slow tier {wide-rung-align16, wide-rung-wordround} with a 71% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-wordround is inconsistent: worst-20% is 3.8x its best-20%

wide-rung-wordround's best 20% of batches run at 1.61 us but its worst 20% at 6.14 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1788.3 ns median (-42.1% vs baseline)
- Spread: 1.87x (fastest 1788.3 ns, slowest 3338.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3804ns | 3232ns | 1847ns | 3586ns | 6415ns | base |
| wide-rung-ragged | 3548ns | 1884ns | 1710ns | 3181ns | 6489ns | -6.73% |
| wide-rung-ragged-overread | 3103ns | 1883ns | 1859ns | 2374ns | 6531ns | -18.44% |
| wide-rung-wordround | 3847ns | 3496ns | 1687ns | 3684ns | 6496ns | +1.13% |
| wide-rung-wordround-alias | 3425ns | 1872ns | 1759ns | 2961ns | 6482ns | -9.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3622ns | 1764ns | 6073ns | base | 0.565 |
| wide-rung-ragged | 3383ns | 1634ns | 6160ns | -6.60% | 0.605 |
| wide-rung-ragged-overread | 2952ns | 1778ns | 6170ns | -18.51% | 0.694 |
| wide-rung-wordround | 3656ns | 1611ns | 6142ns | +0.94% | 0.560 |
| wide-rung-wordround-alias | 3258ns | 1682ns | 6142ns | -10.06% | 0.629 |

## Performance model

- Peak throughput: **1.272 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.663 | 52.2% |
| wide-rung-ragged | 1.136 | 89.3% |
| wide-rung-ragged-overread | 1.138 | 89.5% |
| wide-rung-wordround | 0.613 | 48.2% |
| wide-rung-wordround-alias | 1.145 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3804ns | 3804ns | base |
| wide-rung-ragged | 3548ns | 3548ns | -6.73% |
| wide-rung-ragged-overread | 3103ns | 3103ns | -18.44% |
| wide-rung-wordround | 3847ns | 3847ns | +1.13% |
| wide-rung-wordround-alias | 3425ns | 3425ns | -9.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3088ns | base | --- | [1780, 5250] | --- | --- | --- | --- |
| wide-rung-ragged | 1803ns | no significant difference | [-263, +12]ns | [1791, 4609] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-ragged-overread | 1800ns | no significant difference | [-13, +19]ns | [1795, 2035] | no | 0.8478 | 0.4296 | 0 |
| wide-rung-wordround | 3338ns | no significant difference | [-190, +197]ns | [1996, 4913] | no | 1.0000 | 1.0000 | 1 |
| wide-rung-wordround-alias | 1788ns | no significant difference | [-334, +1]ns | [1780, 4114] | no | 0.6154 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4346ns | +3.5% | -58.6% | -29.9% | -5.0% |
| 1 | 2 | 3884ns | -53.8% | -53.6% | -32.8% | -54.1% |
| 1 | 3 | 3182ns | -43.5% | -43.5% | +64.8% | -43.9% |
| 1 | 4 | 1786ns | +36.4% | +0.7% | +109.3% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.193 | ok |
| wide-rung-ragged | -0.142 | ok |
| wide-rung-ragged-overread | -0.121 | ok |
| wide-rung-wordround | -0.223 | moderate- |
| wide-rung-wordround-alias | -0.083 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 51.3ns | 3622.4ns | 1.4% |  |
| wide-rung-ragged | 47.9ns | 3383.2ns | 1.4% |  |
| wide-rung-ragged-overread | 41.2ns | 2951.8ns | 1.4% |  |
| wide-rung-wordround | 54.1ns | 3656.4ns | 1.5% |  |
| wide-rung-wordround-alias | 46.2ns | 3257.9ns | 1.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1764.2-6073.3 ns)
   1764.2 |########################################
   1979.7 |
   2195.1 |##
   2410.6 |
   2626.0 |
   2841.5 |
   3056.9 |
   3272.4 |
   3487.8 |
   3703.3 |##
   3918.8 |##
   4134.2 |
   4349.7 |
   4565.1 |##
   4780.6 |#####
   4996.0 |##
   5211.5 |########
   5426.9 |########
   5642.4 |##
   5857.8 |###########
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 1634.0-6159.6 ns)
   1634.0 |########################################
   1860.3 |
   2086.6 |
   2312.8 |##
   2539.1 |
   2765.4 |
   2991.7 |##
   3218.0 |
   3444.3 |
   3670.5 |
   3896.8 |##
   4123.1 |
   4349.4 |##
   4575.7 |####
   4802.0 |
   5028.2 |####
   5254.5 |####
   5480.8 |####
   5707.1 |##
   5933.4 |####
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1777.7-6169.8 ns)
   1777.7 |########################################
   1997.3 |
   2216.9 |#
   2436.5 |#
   2656.1 |#
   2875.7 |
   3095.3 |
   3315.0 |
   3534.6 |#
   3754.2 |
   3973.8 |
   4193.4 |
   4413.0 |
   4632.6 |
   4852.2 |
   5071.8 |
   5291.4 |#
   5511.0 |###
   5730.6 |#####
   5950.2 |
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 1610.6-6141.6 ns)
   1610.6 |########################################
   1837.2 |#################
   2063.7 |#####
   2290.3 |#####
   2516.8 |######################
   2743.4 |
   2969.9 |
   3196.5 |
   3423.0 |
   3649.6 |
   3876.1 |#####
   4102.7 |#####
   4329.2 |#####
   4555.7 |###########
   4782.3 |#################
   5008.8 |#####
   5235.4 |###########
   5461.9 |#####
   5688.5 |######################
   5915.0 |#####
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1681.6-6142.1 ns)
   1681.6 |########################################
   1904.6 |
   2127.7 |
   2350.7 |
   2573.7 |
   2796.7 |##
   3019.7 |##
   3242.8 |
   3465.8 |
   3688.8 |##
   3911.8 |####
   4134.9 |######
   4357.9 |##
   4580.9 |##
   4803.9 |##
   5026.9 |
   5250.0 |
   5473.0 |##
   5696.0 |####
   5919.0 |
  (2 below, 5 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=52.0% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wide-rung-ragged**: CV=56.1% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=61.8% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wide-rung-wordround**: CV=49.9% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=54.6% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.7x (possible bimodal distribution)
