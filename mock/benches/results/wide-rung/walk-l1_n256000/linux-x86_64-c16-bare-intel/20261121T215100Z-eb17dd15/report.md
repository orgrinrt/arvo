# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-align16) are a dead heat (<1%)

wide-rung-wordround-alias (1.77 us) and wide-rung-align16 (1.77 us) differ by 0.18%, inside the noise, even though the wider field spreads 22.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (397 ns) is smaller than the fastest variant's own run-to-run std-dev (1.80 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-ragged-overread is inconsistent: worst-20% is 4.0x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 1.56 us but its worst 20% at 6.29 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (4 ns, 0.25%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by 4 ns (0.25%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1767.2 ns median (-0.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 1767.2 ns, slowest 2163.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112830321 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3268ns | 1852ns | 1510ns | 2955ns | 5965ns | base |
| wide-rung-ragged | 3153ns | 1854ns | 1510ns | 2934ns | 5452ns | -3.52% |
| wide-rung-ragged-overread | 3326ns | 1854ns | 1636ns | 2792ns | 6617ns | +1.76% |
| wide-rung-wordround | 3409ns | 2262ns | 1511ns | 3213ns | 5898ns | +4.32% |
| wide-rung-wordround-alias | 3194ns | 1851ns | 1817ns | 2626ns | 6277ns | -2.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3113ns | 1444ns | 5658ns | base | 0.658 |
| wide-rung-ragged | 3004ns | 1444ns | 5162ns | -3.47% | 0.682 |
| wide-rung-ragged-overread | 3169ns | 1564ns | 6286ns | +1.80% | 0.646 |
| wide-rung-wordround | 3249ns | 1445ns | 5597ns | +4.37% | 0.630 |
| wide-rung-wordround-alias | 3041ns | 1735ns | 5939ns | -2.31% | 0.674 |

## Performance model

- Peak throughput: **1.419 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.157 | 81.5% |
| wide-rung-ragged | 1.155 | 81.5% |
| wide-rung-ragged-overread | 1.155 | 81.5% |
| wide-rung-wordround | 0.946 | 66.7% |
| wide-rung-wordround-alias | 1.159 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3268ns | 3268ns | base |
| wide-rung-ragged | 3153ns | 3153ns | -3.52% |
| wide-rung-ragged-overread | 3326ns | 3326ns | +1.76% |
| wide-rung-wordround | 3409ns | 3409ns | +4.32% |
| wide-rung-wordround-alias | 3194ns | 3194ns | -2.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1770ns | base | --- | [1762, 4348] | --- | --- | --- | --- |
| wide-rung-ragged | 1772ns | no significant difference | [-313, +2]ns | [1455, 4575] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-ragged-overread | 1772ns | +86.5ns (+4.9%) | [+3, +321]ns | [1766, 3539] | YES | 0.0089 | 0.0022 | 0 |
| wide-rung-wordround | 2164ns | no significant difference | [-202, +320]ns | [1765, 4590] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround-alias | 1767ns | +4.5ns (+0.3%) | [+1, +286]ns | [1766, 3018] | YES (adj: no) | 0.0770 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1765ns | +48.8% | +68.6% | -11.5% | +0.5% |
| 1 | 2 | 2596ns | -44.1% | -31.7% | -31.8% | +9.5% |
| 1 | 3 | 1450ns | +23.3% | +21.9% | +183.2% | +19.3% |
| 1 | 4 | 3802ns | -35.5% | -51.9% | +25.6% | -53.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.496 | moderate- |
| wide-rung-ragged | -0.294 | moderate- |
| wide-rung-ragged-overread | -0.091 | ok |
| wide-rung-wordround | 0.299 | moderate+ |
| wide-rung-wordround-alias | -0.422 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 43.1ns | 3112.6ns | 1.4% |  |
| wide-rung-ragged | 41.1ns | 3004.5ns | 1.4% |  |
| wide-rung-ragged-overread | 42.8ns | 3168.7ns | 1.4% |  |
| wide-rung-wordround | 44.0ns | 3248.7ns | 1.4% |  |
| wide-rung-wordround-alias | 41.3ns | 3040.7ns | 1.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1443.8-5657.6 ns)
   1443.8 |####################
   1654.5 |########################################
   1865.1 |
   2075.8 |
   2286.5 |
   2497.2 |
   2707.9 |
   2918.6 |
   3129.3 |
   3340.0 |######
   3550.7 |
   3761.4 |
   3972.1 |
   4182.8 |######
   4393.5 |######
   4604.1 |###
   4814.8 |##########
   5025.5 |
   5236.2 |#############
   5446.9 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1444.0-5162.4 ns)
   1444.0 |########################################
   1629.9 |###############
   1815.8 |
   2001.7 |###
   2187.7 |
   2373.6 |
   2559.5 |
   2745.4 |
   2931.3 |
   3117.3 |
   3303.2 |######
   3489.1 |
   3675.0 |
   3861.0 |
   4046.9 |
   4232.8 |
   4418.7 |#########
   4604.6 |###
   4790.6 |##################
   4976.5 |######
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1564.4-6286.0 ns)
   1564.4 |########################################
   1800.5 |
   2036.6 |##
   2272.6 |
   2508.7 |##
   2744.8 |##
   2980.9 |
   3217.0 |
   3453.0 |
   3689.1 |
   3925.2 |
   4161.3 |##
   4397.3 |
   4633.4 |##
   4869.5 |####
   5105.6 |##
   5341.7 |##
   5577.7 |
   5813.8 |####
   6049.9 |
  (5 below, 6 above range)

wide-rung-wordround (n=40, range 1444.8-5597.4 ns)
   1444.8 |######
   1652.5 |########################################
   1860.1 |
   2067.7 |
   2275.4 |
   2483.0 |###
   2690.6 |
   2898.3 |
   3105.9 |
   3313.5 |
   3521.1 |
   3728.8 |
   3936.4 |###
   4144.0 |######
   4351.7 |######
   4559.3 |##########
   4766.9 |#############
   4974.5 |###
   5182.2 |######
   5389.8 |
  (6 below, 4 above range)

wide-rung-wordround-alias (n=40, range 1734.6-5939.5 ns)
   1734.6 |########################################
   1944.8 |
   2155.1 |
   2365.3 |
   2575.6 |
   2785.8 |
   2996.0 |
   3206.3 |
   3416.5 |
   3626.8 |
   3837.0 |##
   4047.3 |##
   4257.5 |
   4467.8 |
   4678.0 |##
   4888.2 |##
   5098.5 |##
   5308.7 |######
   5519.0 |
   5729.2 |####
  (6 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=55.9% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **wide-rung-ragged**: CV=55.0% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=61.2% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wide-rung-wordround**: CV=53.1% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=59.2% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.4x (possible bimodal distribution)
