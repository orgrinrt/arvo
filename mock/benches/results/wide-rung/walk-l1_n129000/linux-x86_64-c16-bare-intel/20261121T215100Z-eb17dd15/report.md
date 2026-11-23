# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 1.83 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged beats baseline by 20% (significant)

wide-rung-ragged is -462 ns (20%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (460 ns) is smaller than the fastest variant's own run-to-run std-dev (1.74 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-ragged is inconsistent: worst-20% is 3.9x its best-20%

wide-rung-ragged's best 20% of batches run at 1.49 us but its worst 20% at 5.76 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-2 ns, 0.10%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -2 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 1831.4 ns median (-20.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.25x (fastest 1831.4 ns, slowest 2291.7 ns)

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
| wide-rung-align16 | 3714ns | 2375ns | 1941ns | 3074ns | 7407ns | base |
| wide-rung-ragged | 3199ns | 1894ns | 1546ns | 2814ns | 6009ns | -13.85% |
| wide-rung-ragged-overread | 3779ns | 2352ns | 1924ns | 3300ns | 7073ns | +1.77% |
| wide-rung-wordround | 3301ns | 1952ns | 1941ns | 2513ns | 7024ns | -11.12% |
| wide-rung-wordround-alias | 3143ns | 1952ns | 1940ns | 2280ns | 6938ns | -15.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3573ns | 1875ns | 7086ns | base | 0.573 |
| wide-rung-ragged | 3083ns | 1494ns | 5759ns | -13.73% | 0.664 |
| wide-rung-ragged-overread | 3669ns | 1873ns | 6847ns | +2.67% | 0.558 |
| wide-rung-wordround | 3176ns | 1875ns | 6729ns | -11.12% | 0.645 |
| wide-rung-wordround-alias | 3027ns | 1875ns | 6652ns | -15.29% | 0.677 |

## Performance model

- Peak throughput: **1.371 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.894 | 65.2% |
| wide-rung-ragged | 1.118 | 81.6% |
| wide-rung-ragged-overread | 0.895 | 65.3% |
| wide-rung-wordround | 1.086 | 79.2% |
| wide-rung-wordround-alias | 1.086 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3714ns | 3714ns | base |
| wide-rung-ragged | 3199ns | 3199ns | -13.85% |
| wide-rung-ragged-overread | 3779ns | 3779ns | +1.77% |
| wide-rung-wordround | 3301ns | 3301ns | -11.12% |
| wide-rung-wordround-alias | 3143ns | 3143ns | -15.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2292ns | base | --- | [2292, 2977] | --- | --- | --- | --- |
| wide-rung-ragged | 1831ns | -462.5ns (-20.2%) | [-796, -378]ns | [1821, 4092] | YES | 0.0007 | 0.0002 | 0 |
| wide-rung-ragged-overread | 2289ns | -2.2ns (-0.1%) | [-13, -2]ns | [1873, 5140] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-wordround | 1886ns | -416.5ns (-18.2%) | [-423, -0]ns | [1875, 2297] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround-alias | 1885ns | no significant difference | [-417, +96]ns | [1875, 2292] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4575ns | -60.2% | -58.9% | -49.8% | -59.0% |
| 1 | 2 | 2305ns | +1.4% | +173.9% | -18.7% | -18.7% |
| 1 | 3 | 2292ns | -19.9% | +43.9% | -17.9% | -18.2% |
| 1 | 4 | 1875ns | +112.0% | -0.1% | -0.0% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.044 | ok |
| wide-rung-ragged | -0.248 | moderate- |
| wide-rung-ragged-overread | -0.336 | moderate- |
| wide-rung-wordround | -0.091 | ok |
| wide-rung-wordround-alias | -0.083 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 3/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 36.0ns | 3573.3ns | 1.0% |  |
| wide-rung-ragged | 40.1ns | 3082.7ns | 1.3% |  |
| wide-rung-ragged-overread | 36.9ns | 3668.7ns | 1.0% |  |
| wide-rung-wordround | 32.0ns | 3176.0ns | 1.0% |  |
| wide-rung-wordround-alias | 30.5ns | 3026.8ns | 1.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1874.9-7086.5 ns)
   1874.9 |######################
   2135.5 |########################################
   2396.1 |
   2656.7 |
   2917.2 |
   3177.8 |
   3438.4 |##
   3699.0 |
   3959.5 |
   4220.1 |
   4480.7 |
   4741.3 |##
   5001.8 |
   5262.4 |##
   5523.0 |##
   5783.6 |#####
   6044.2 |
   6304.7 |########
   6565.3 |
   6825.9 |##
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1493.7-5758.7 ns)
   1493.7 |#################
   1706.9 |########################################
   1920.2 |
   2133.4 |
   2346.7 |
   2559.9 |
   2773.2 |
   2986.4 |##
   3199.7 |
   3412.9 |##
   3626.2 |##
   3839.4 |
   4052.7 |
   4265.9 |#####
   4479.2 |
   4692.4 |##############
   4905.7 |##
   5119.0 |#####
   5332.2 |
   5545.5 |
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 1873.0-6846.5 ns)
   1873.0 |########################################
   2121.6 |#############
   2370.3 |
   2619.0 |
   2867.7 |
   3116.4 |
   3365.0 |
   3613.7 |
   3862.4 |##
   4111.1 |
   4359.8 |##
   4608.4 |
   4857.1 |
   5105.8 |########
   5354.5 |##
   5603.1 |#####
   5851.8 |##
   6100.5 |########
   6349.2 |##
   6597.9 |
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 1874.8-6729.1 ns)
   1874.8 |########################################
   2117.5 |################
   2360.2 |
   2603.0 |
   2845.7 |
   3088.4 |##
   3331.1 |
   3573.8 |
   3816.5 |
   4059.2 |
   4301.9 |
   4544.7 |####
   4787.4 |##
   5030.1 |
   5272.8 |
   5515.5 |
   5758.2 |##
   6000.9 |##
   6243.6 |####
   6486.4 |##
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1874.8-6652.1 ns)
   1874.8 |########################################
   2113.7 |################
   2352.5 |
   2591.4 |
   2830.3 |##
   3069.1 |
   3308.0 |
   3546.8 |
   3785.7 |
   4024.6 |
   4263.4 |
   4502.3 |
   4741.2 |
   4980.0 |
   5218.9 |##
   5457.7 |
   5696.6 |##
   5935.5 |##
   6174.3 |######
   6413.2 |##
  (2 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=59.5% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-ragged**: CV=56.4% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=57.2% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **wide-rung-wordround**: CV=61.8% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=63.6% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.5x (possible bimodal distribution)
