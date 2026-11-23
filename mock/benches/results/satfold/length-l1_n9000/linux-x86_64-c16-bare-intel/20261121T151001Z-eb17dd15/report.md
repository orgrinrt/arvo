# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon dominates: 16% faster than the next best (satfold-lanes64)

satfold-neon (2.75 us) leads satfold-lanes64 (3.20 us) by 16%, a clear separation rather than a photo finish. CV 31.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 85% (significant)

satfold-neon is -15.50 us (85%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 8.1x slower than the field

satfold-seq (22.27 us) is 8.1x the fastest (2.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.73)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.1x the fastest

Fastest satfold-neon (2.75 us) to slowest satfold-seq (22.27 us): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-constl is inconsistent: worst-20% is 4.0x its best-20%

satfold-lanes16-constl's best 20% of batches run at 1.56 us but its worst 20% at 6.18 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 2745.6 ns median (-84.9% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 8.11x (fastest 2745.6 ns, slowest 22266.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 19323ns | 18292ns | 18282ns | 18657ns | 22365ns | base |
| satfold-lanes16 | 4006ns | 4252ns | 1613ns | 4136ns | 6010ns | -79.27% |
| satfold-lanes16-constl | 3805ns | 4106ns | 1620ns | 3650ns | 6456ns | -80.31% |
| satfold-lanes4-idx | 5396ns | 4502ns | 4374ns | 4707ns | 8481ns | -72.08% |
| satfold-lanes64 | 3317ns | 3393ns | 1694ns | 3441ns | 4568ns | -82.83% |
| satfold-neon | 2673ns | 2914ns | 928ns | 2941ns | 3612ns | -86.17% |
| satfold-neon8 | 3350ns | 3419ns | 2357ns | 3446ns | 4052ns | -82.67% |
| satfold-nolaw | 8070ns | 8088ns | 7255ns | 8071ns | 8883ns | -58.23% |
| satfold-seq | 21381ns | 22338ns | 18276ns | 22009ns | 22602ns | +10.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 19256ns | 18224ns | 22282ns | base | 1.702 |
| satfold-lanes16 | 3848ns | 1554ns | 5759ns | -80.01% | 8.515 |
| satfold-lanes16-constl | 3652ns | 1561ns | 6180ns | -81.04% | 8.973 |
| satfold-lanes4-idx | 5311ns | 4313ns | 8316ns | -72.42% | 6.170 |
| satfold-lanes64 | 3116ns | 1596ns | 4266ns | -83.82% | 10.517 |
| satfold-neon | 2506ns | 875ns | 3339ns | -86.99% | 13.078 |
| satfold-neon8 | 3147ns | 2221ns | 3774ns | -83.66% | 10.414 |
| satfold-nolaw | 8000ns | 7193ns | 8800ns | -58.46% | 4.096 |
| satfold-seq | 21306ns | 18216ns | 22508ns | +10.65% | 1.538 |

## Performance model

- Peak throughput: **37.450 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.797 | 4.8% |
| satfold-lanes16 | 7.998 | 21.4% |
| satfold-lanes16-constl | 8.287 | 22.1% |
| satfold-lanes4-idx | 7.377 | 19.7% |
| satfold-lanes64 | 10.250 | 27.4% |
| satfold-neon | 11.935 | 31.9% |
| satfold-neon8 | 10.176 | 27.2% |
| satfold-nolaw | 4.096 | 10.9% |
| satfold-seq | 1.472 | 3.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 19323ns | 19323ns | base |
| satfold-lanes16 | 4006ns | 4006ns | -79.27% |
| satfold-lanes16-constl | 3805ns | 3805ns | -80.31% |
| satfold-lanes4-idx | 5396ns | 5396ns | -72.08% |
| satfold-lanes64 | 3317ns | 3317ns | -82.83% |
| satfold-neon | 2673ns | 2673ns | -86.17% |
| satfold-neon8 | 3350ns | 3350ns | -82.67% |
| satfold-nolaw | 8070ns | 8070ns | -58.23% |
| satfold-seq | 21381ns | 21381ns | +10.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 18232ns | base | --- | [18230, 18241] | --- | --- | --- | --- |
| satfold-lanes16 | 4097ns | -14249.0ns (-78.2%) | [-15540, -13943]ns | [3789, 4341] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3954ns | -15192.0ns (-83.3%) | [-16567, -14189]ns | [2592, 4286] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 4442ns | -13827.8ns (-75.8%) | [-13917, -13688]ns | [4406, 4785] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 3197ns | -15172.8ns (-83.2%) | [-15721, -15016]ns | [3143, 3411] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2746ns | -15499.9ns (-85.0%) | [-15681, -15362]ns | [2725, 2935] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 3220ns | -15268.0ns (-83.7%) | [-15501, -15028]ns | [3100, 3468] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8000ns | -10231.0ns (-56.1%) | [-11019, -9438]ns | [7216, 8791] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22267ns | +4035.2ns (+22.1%) | [+4030, +4037]ns | [22264, 22269] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 18235ns | -79.2% | -77.9% | -76.1% | -83.2% | -84.9% | -82.6% | -60.4% | +22.1% |
| 1 | 2 | 22279ns | -92.5% | -88.4% | -80.2% | -85.6% | -96.1% | -84.8% | -67.7% | -18.2% |
| 1 | 3 | 18230ns | -73.7% | -87.8% | -75.7% | -80.7% | -84.9% | -83.8% | -51.7% | +22.2% |
| 1 | 4 | 18230ns | -77.5% | -77.7% | -70.5% | -82.7% | -83.9% | -81.0% | -51.8% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.416 | moderate- |
| satfold-lanes16 | -0.384 | moderate- |
| satfold-lanes16-constl | -0.258 | moderate- |
| satfold-lanes4-idx | -0.062 | ok |
| satfold-lanes64 | -0.218 | moderate- |
| satfold-neon | -0.341 | moderate- |
| satfold-neon8 | -0.732 | HIGH- (thermal bounce) |
| satfold-nolaw | 0.246 | moderate+ |
| satfold-seq | -0.417 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.1ns | 19255.8ns | 0.1% |  |
| satfold-lanes16 | 53.4ns | 3848.4ns | 1.4% |  |
| satfold-lanes16-constl | 51.9ns | 3651.8ns | 1.4% |  |
| satfold-lanes4-idx | 24.2ns | 5311.1ns | 0.5% |  |
| satfold-lanes64 | 68.1ns | 3115.8ns | 2.2% |  |
| satfold-neon | 61.1ns | 2505.6ns | 2.4% |  |
| satfold-neon8 | 68.2ns | 3146.6ns | 2.2% |  |
| satfold-nolaw | 21.5ns | 7999.6ns | 0.3% |  |
| satfold-seq | 22.2ns | 21306.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 18223.6-22281.6 ns)
  18223.6 |########################################
  18426.5 |
  18629.4 |#
  18832.3 |
  19035.2 |
  19238.1 |
  19441.0 |
  19643.9 |
  19846.8 |
  20049.7 |
  20252.6 |
  20455.5 |
  20658.4 |
  20861.3 |
  21064.2 |
  21267.1 |
  21470.0 |
  21672.9 |
  21875.8 |
  22078.7 |###########
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1553.6-5759.0 ns)
   1553.6 |########################################
   1763.9 |
   1974.1 |
   2184.4 |
   2394.7 |
   2605.0 |######
   2815.2 |
   3025.5 |######
   3235.8 |
   3446.0 |
   3656.3 |##########################
   3866.6 |#################################
   4076.9 |##########################
   4287.1 |##########################
   4497.4 |####################
   4707.7 |######
   4917.9 |#############
   5128.2 |
   5338.5 |######
   5548.8 |######
  (3 below, 4 above range)

satfold-lanes16-constl (n=40, range 1560.6-6180.2 ns)
   1560.6 |########################################
   1791.6 |
   2022.6 |
   2253.5 |###
   2484.5 |
   2715.5 |###
   2946.5 |
   3177.5 |
   3408.4 |###
   3639.4 |#######
   3870.4 |##################
   4101.4 |##############
   4332.4 |###
   4563.3 |##############
   4794.3 |
   5025.3 |
   5256.3 |
   5487.2 |###
   5718.2 |###
   5949.2 |##########
  (2 below, 3 above range)

satfold-lanes4-idx (n=40, range 4313.2-8315.9 ns)
   4313.2 |########################################
   4513.3 |#
   4713.4 |
   4913.6 |#
   5113.7 |#####
   5313.8 |###########
   5514.0 |
   5714.1 |#
   5914.3 |
   6114.4 |
   6314.5 |
   6514.7 |
   6714.8 |
   6914.9 |
   7115.1 |
   7315.2 |
   7515.4 |
   7715.5 |
   7915.6 |
   8115.8 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 1596.1-4266.1 ns)
   1596.1 |
   1729.6 |
   1863.1 |
   1996.6 |
   2130.1 |
   2263.6 |
   2397.1 |
   2530.6 |#####
   2664.1 |#####
   2797.6 |###########
   2931.1 |######################
   3064.6 |########################################
   3198.1 |############################
   3331.6 |###########
   3465.1 |######################
   3598.6 |
   3732.1 |######################
   3865.6 |#####
   3999.1 |
   4132.6 |
  (5 below, 4 above range)

satfold-neon (n=40, range 875.0-3338.7 ns)
    875.0 |##########
    998.2 |
   1121.3 |
   1244.5 |
   1367.7 |
   1490.9 |
   1614.1 |
   1737.3 |
   1860.4 |
   1983.6 |
   2106.8 |
   2230.0 |
   2353.2 |
   2476.4 |##################
   2599.6 |
   2722.7 |########################################
   2845.9 |#########################
   2969.1 |#######
   3092.3 |##############
   3215.5 |###
  (5 below, 2 above range)

satfold-neon8 (n=40, range 2220.9-3773.6 ns)
   2220.9 |
   2298.6 |
   2376.2 |
   2453.8 |
   2531.5 |
   2609.1 |
   2686.7 |#################
   2764.4 |
   2842.0 |###########
   2919.6 |############################
   2997.3 |#####
   3074.9 |
   3152.5 |########################################
   3230.2 |#####
   3307.8 |#####
   3385.4 |###########
   3463.1 |##################################
   3540.7 |###########
   3618.3 |
   3696.0 |#################
  (3 below, 4 above range)

satfold-nolaw (n=40, range 7193.2-8800.4 ns)
   7193.2 |#####################################
   7273.6 |
   7353.9 |
   7434.3 |
   7514.6 |
   7595.0 |
   7675.4 |
   7755.7 |
   7836.1 |
   7916.4 |
   7996.8 |
   8077.2 |
   8157.5 |
   8237.9 |
   8318.3 |
   8398.6 |
   8479.0 |
   8559.4 |
   8639.7 |
   8720.1 |########################################
  (3 below, 2 above range)

satfold-seq (n=40, range 18216.4-22508.0 ns)
  18216.4 |##########
  18431.0 |
  18645.6 |
  18860.2 |
  19074.8 |
  19289.3 |
  19503.9 |
  19718.5 |
  19933.1 |
  20147.7 |
  20362.2 |
  20576.8 |
  20791.4 |
  21006.0 |
  21220.6 |
  21435.1 |
  21649.7 |
  21864.3 |
  22078.9 |########################################
  22293.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=37.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=46.8% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=42.1% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=31.4% (high variance, measurements may be unstable)
- **satfold-neon**: CV=34.6% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-neon8**: CV=20.6% (high variance, measurements may be unstable)
