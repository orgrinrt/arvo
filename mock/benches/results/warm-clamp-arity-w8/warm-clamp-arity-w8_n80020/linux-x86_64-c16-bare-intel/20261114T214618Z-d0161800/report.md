# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.60)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.82 us) is smaller than the fastest variant's own run-to-run std-dev (2.29 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-minimum is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.96 us but its worst 20% at 7.97 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2339.4 ns median (-26.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.78x (fastest 2339.4 ns, slowest 4159.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3717ns | 3294ns | 1732ns | 3523ns | 6285ns | base |
| warm-clamp-accfit | 3711ns | 4111ns | 1410ns | 3883ns | 5494ns | -0.16% |
| warm-clamp-accfit-dyn | 3952ns | 4351ns | 1464ns | 4118ns | 5940ns | +6.32% |
| warm-clamp-head | 3874ns | 2403ns | 2073ns | 3142ns | 7871ns | +4.22% |
| warm-clamp-min-lanes | 4397ns | 2472ns | 2071ns | 3939ns | 8096ns | +18.29% |
| warm-clamp-minimum | 3940ns | 2462ns | 2015ns | 3150ns | 8236ns | +6.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3554ns | 1660ns | 5984ns | base | 2.305 |
| warm-clamp-accfit | 3545ns | 1352ns | 5223ns | -0.25% | 2.311 |
| warm-clamp-accfit-dyn | 3772ns | 1402ns | 5638ns | +6.14% | 2.172 |
| warm-clamp-head | 3761ns | 2019ns | 7612ns | +5.82% | 2.178 |
| warm-clamp-min-lanes | 4272ns | 2019ns | 7837ns | +20.21% | 1.918 |
| warm-clamp-minimum | 3827ns | 1964ns | 7968ns | +7.69% | 2.141 |

## Performance model

- Peak throughput: **6.060 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.590 | 42.7% |
| warm-clamp-accfit | 2.079 | 34.3% |
| warm-clamp-accfit-dyn | 1.970 | 32.5% |
| warm-clamp-head | 3.502 | 57.8% |
| warm-clamp-min-lanes | 3.401 | 56.1% |
| warm-clamp-minimum | 3.414 | 56.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3717ns | 3717ns | base |
| warm-clamp-accfit | 3711ns | 3711ns | -0.16% |
| warm-clamp-accfit-dyn | 3952ns | 3952ns | +6.32% |
| warm-clamp-head | 3874ns | 3874ns | +4.22% |
| warm-clamp-min-lanes | 4397ns | 4397ns | +18.29% |
| warm-clamp-minimum | 3940ns | 3940ns | +6.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3163ns | base | --- | [1670, 5159] | --- | --- | --- | --- |
| warm-clamp-accfit | 3941ns | -220.9ns (-7.0%) | [-674, -192]ns | [3505, 4374] | YES | 0.0207 | 0.0166 | 0 |
| warm-clamp-accfit-dyn | 4159ns | no significant difference | [-415, +106]ns | [3935, 4492] | no | 0.0807 | 0.0807 | 0 |
| warm-clamp-head | 2339ns | +669.5ns (+21.2%) | [+453, +678]ns | [2338, 2352] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-min-lanes | 2409ns | +737.6ns (+23.3%) | [+727, +1182]ns | [2397, 6361] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2399ns | +732.5ns (+23.2%) | [+303, +748]ns | [2399, 2546] | YES | 0.0037 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3895ns | +16.1% | +4.6% | -39.9% | -49.5% | -38.3% |
| 1 | 2 | 4162ns | -11.0% | +0.1% | -53.8% | +2.0% | -42.2% |
| 1 | 3 | 2696ns | +49.9% | +0.2% | -13.2% | -10.7% | -27.2% |
| 1 | 4 | 1855ns | +128.9% | +144.7% | +26.4% | +160.6% | +61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.254 | moderate+ |
| warm-clamp-accfit | -0.395 | moderate- |
| warm-clamp-accfit-dyn | -0.549 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.407 | moderate- |
| warm-clamp-min-lanes | -0.600 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.456 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 0/4, lost 3/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 51.2ns | 3553.7ns | 1.4% |  |
| warm-clamp-accfit | 58.7ns | 3545.0ns | 1.7% |  |
| warm-clamp-accfit-dyn | 61.3ns | 3771.8ns | 1.6% |  |
| warm-clamp-head | 39.1ns | 3760.7ns | 1.0% |  |
| warm-clamp-min-lanes | 42.7ns | 4272.0ns | 1.0% |  |
| warm-clamp-minimum | 37.7ns | 3826.9ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1660.5-5984.4 ns)
   1660.5 |########################################
   1876.7 |###
   2092.9 |
   2309.1 |
   2525.3 |###
   2741.4 |
   2957.6 |
   3173.8 |
   3390.0 |
   3606.2 |######
   3822.4 |
   4038.6 |
   4254.8 |
   4471.0 |###
   4687.2 |
   4903.4 |######
   5119.6 |######
   5335.8 |################
   5552.0 |###
   5768.2 |##########
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 1351.8-5222.9 ns)
   1351.8 |########################################
   1545.3 |
   1738.9 |
   1932.4 |#####
   2126.0 |
   2319.6 |
   2513.1 |
   2706.7 |
   2900.2 |#####
   3093.8 |#####
   3287.3 |
   3480.9 |######################
   3674.5 |#################
   3868.0 |#################
   4061.6 |###########
   4255.1 |###########
   4448.7 |###########
   4642.3 |######################
   4835.8 |
   5029.4 |
  (3 below, 7 above range)

warm-clamp-accfit-dyn (n=40, range 1401.7-5638.1 ns)
   1401.7 |########################################
   1613.5 |
   1825.4 |
   2037.2 |
   2249.0 |
   2460.8 |
   2672.6 |
   2884.5 |
   3096.3 |####
   3308.1 |
   3519.9 |
   3731.7 |########
   3943.6 |##########################
   4155.4 |########
   4367.2 |######################
   4579.0 |########
   4790.8 |#############
   5002.7 |########
   5214.5 |#############
   5426.3 |
  (2 below, 3 above range)

warm-clamp-head (n=40, range 2019.0-7612.1 ns)
   2019.0 |
   2298.7 |########################################
   2578.3 |
   2858.0 |
   3137.6 |
   3417.3 |
   3696.9 |
   3976.6 |#
   4256.2 |
   4535.9 |
   4815.5 |
   5095.2 |
   5374.8 |
   5654.5 |#
   5934.1 |#
   6213.8 |#
   6493.4 |###
   6773.1 |#
   7052.8 |###
   7332.4 |
  (6 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2018.7-7837.2 ns)
   2018.7 |
   2309.6 |########################################
   2600.5 |
   2891.5 |##
   3182.4 |
   3473.3 |
   3764.2 |
   4055.2 |
   4346.1 |
   4637.0 |
   4928.0 |
   5218.9 |
   5509.8 |
   5800.7 |#####
   6091.7 |
   6382.6 |#######
   6673.5 |##
   6964.4 |#####
   7255.4 |#######
   7546.3 |##
  (7 below, 4 above range)

warm-clamp-minimum (n=40, range 1963.9-7968.0 ns)
   1963.9 |##
   2264.1 |########################################
   2564.3 |##
   2864.5 |
   3164.7 |
   3464.9 |##
   3765.1 |
   4065.3 |
   4365.5 |##
   4665.7 |
   4966.0 |
   5266.2 |
   5566.4 |
   5866.6 |
   6166.8 |##
   6467.0 |##
   6767.2 |##
   7067.4 |####
   7367.6 |##
   7667.8 |##
  (7 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=53.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=40.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=42.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-head**: CV=61.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=57.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=62.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
