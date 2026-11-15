# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (3.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 2.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (692 ns) is smaller than the fastest variant's own run-to-run std-dev (988 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-wordround (+6% speed for 1.4x steadier)

wide-rung-wordround-alias is fastest (2.75 us, CV 35.9%); wide-rung-wordround gives up 6.4% median for 1.4x lower variance (CV 26.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged-overread is inconsistent: worst-20% is 5.7x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 827 ns but its worst 20% at 4.68 us (5.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 2751.1 ns median (-20.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.25x (fastest 2751.1 ns, slowest 3442.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3186ns | 3686ns | 1052ns | 3478ns | 4445ns | base |
| wide-rung-ragged | 3196ns | 3659ns | 1477ns | 3405ns | 4291ns | +0.33% |
| wide-rung-ragged-overread | 2726ns | 2982ns | 884ns | 2567ns | 5048ns | -14.42% |
| wide-rung-wordround | 3103ns | 3196ns | 1734ns | 3291ns | 3909ns | -2.59% |
| wide-rung-wordround-alias | 2837ns | 2983ns | 971ns | 3025ns | 4141ns | -10.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2964ns | 983ns | 4092ns | base | 0.691 |
| wide-rung-ragged | 2971ns | 1363ns | 3945ns | +0.23% | 0.689 |
| wide-rung-ragged-overread | 2543ns | 827ns | 4682ns | -14.20% | 0.805 |
| wide-rung-wordround | 2846ns | 1597ns | 3544ns | -3.99% | 0.720 |
| wide-rung-wordround-alias | 2607ns | 895ns | 3775ns | -12.03% | 0.786 |

## Performance model

- Peak throughput: **2.476 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.595 | 24.0% |
| wide-rung-ragged | 0.596 | 24.1% |
| wide-rung-ragged-overread | 0.732 | 29.6% |
| wide-rung-wordround | 0.700 | 28.3% |
| wide-rung-wordround-alias | 0.744 | 30.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3186ns | 3186ns | base |
| wide-rung-ragged | 3196ns | 3196ns | +0.33% |
| wide-rung-ragged-overread | 2726ns | 2726ns | -14.42% |
| wide-rung-wordround | 3103ns | 3103ns | -2.59% |
| wide-rung-wordround-alias | 2837ns | 2837ns | -10.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3443ns | base | --- | [3173, 3527] | --- | --- | --- | --- |
| wide-rung-ragged | 3434ns | -88.1ns (-2.6%) | [-363, -24]ns | [3131, 3451] | YES | 0.0385 | 0.0385 | 0 |
| wide-rung-ragged-overread | 2799ns | -428.3ns (-12.4%) | [-624, -193]ns | [1141, 3141] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround | 2927ns | -307.2ns (-8.9%) | [-463, -90]ns | [2882, 3188] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-wordround-alias | 2751ns | -510.0ns (-14.8%) | [-683, -171]ns | [2472, 3102] | YES | 0.0089 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1645ns | +109.6% | +90.9% | +84.5% | +95.4% |
| 1 | 2 | 3505ns | -1.6% | -77.6% | -45.5% | -29.5% |
| 1 | 3 | 3459ns | -0.7% | -72.2% | -15.6% | -15.8% |
| 1 | 4 | 3399ns | -52.9% | -16.3% | -7.6% | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.110 | ok |
| wide-rung-ragged | -0.077 | ok |
| wide-rung-ragged-overread | -0.254 | moderate- |
| wide-rung-wordround | -0.326 | moderate- |
| wide-rung-wordround-alias | -0.337 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 61.1ns | 2963.8ns | 2.1% |  |
| wide-rung-ragged | 74.7ns | 2970.6ns | 2.5% |  |
| wide-rung-ragged-overread | 64.5ns | 2542.8ns | 2.5% |  |
| wide-rung-wordround | 70.2ns | 2845.6ns | 2.5% |  |
| wide-rung-wordround-alias | 64.2ns | 2607.1ns | 2.5% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 982.8-4091.6 ns)
    982.8 |
   1138.2 |########
   1293.7 |
   1449.1 |
   1604.5 |
   1760.0 |
   1915.4 |
   2070.9 |
   2226.3 |####
   2381.8 |####
   2537.2 |
   2692.6 |
   2848.1 |
   3003.5 |############
   3159.0 |############
   3314.4 |############
   3469.9 |########################################
   3625.3 |
   3780.8 |########################
   3936.2 |
  (7 below, 4 above range)

wide-rung-ragged (n=40, range 1362.7-3945.2 ns)
   1362.7 |
   1491.8 |#################
   1620.9 |
   1750.0 |
   1879.2 |
   2008.3 |
   2137.4 |
   2266.5 |
   2395.7 |
   2524.8 |
   2653.9 |
   2783.0 |##
   2912.2 |
   3041.3 |############
   3170.4 |#####
   3299.6 |
   3428.7 |########################################
   3557.8 |##
   3686.9 |#####
   3816.1 |##
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 827.0-4682.3 ns)
    827.0 |########################################
   1019.8 |
   1212.5 |#####
   1405.3 |
   1598.1 |
   1790.8 |
   1983.6 |
   2176.4 |
   2369.1 |
   2561.9 |####################
   2754.6 |#########################
   2947.4 |##########
   3140.2 |#########################
   3332.9 |##########
   3525.7 |#####
   3718.5 |##########
   3911.2 |
   4104.0 |#####
   4296.8 |
   4489.5 |
  (6 below, 3 above range)

wide-rung-wordround (n=40, range 1597.0-3544.4 ns)
   1597.0 |
   1694.4 |
   1791.7 |
   1889.1 |
   1986.5 |
   2083.8 |
   2181.2 |
   2278.6 |
   2375.9 |
   2473.3 |
   2570.7 |###############
   2668.1 |##########
   2765.4 |###############
   2862.8 |########################################
   2960.2 |
   3057.5 |####################
   3154.9 |##########
   3252.3 |###############
   3349.6 |#####
   3447.0 |###################################
  (5 below, 2 above range)

wide-rung-wordround-alias (n=40, range 895.2-3774.7 ns)
    895.2 |########################################
   1039.2 |
   1183.2 |
   1327.2 |
   1471.1 |
   1615.1 |
   1759.1 |
   1903.0 |
   2047.0 |
   2191.0 |########################
   2334.9 |################################
   2478.9 |################
   2622.9 |################################
   2766.9 |################
   2910.8 |################
   3054.8 |########################
   3198.8 |################
   3342.7 |################################
   3486.7 |########
   3630.7 |########
  (3 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=38.2% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **wide-rung-ragged**: CV=32.9% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=60.0% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 5.7x (possible bimodal distribution)
- **wide-rung-wordround**: CV=27.1% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=37.9% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 4.2x (possible bimodal distribution)
