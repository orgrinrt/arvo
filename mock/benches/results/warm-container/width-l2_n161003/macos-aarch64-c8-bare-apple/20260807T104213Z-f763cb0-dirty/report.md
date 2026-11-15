# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 95% (significant)

warm-container-minimum is -1.02 ms (95%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 20.5x slower than the field

warm-container-plusone (1.08 ms) is 20.5x the fastest (52.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows warm-up / thermal drift (autocorr +0.72)

warm-container-native's per-pass series has lag-1 autocorrelation +0.72, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (1914% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 1914% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 20.5x the fastest

Fastest warm-container-minimum (52.55 us) to slowest warm-container-plusone (1.08 ms): 20.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-minimum** at 52553.8 ns median (-95.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 20.46x (fastest 52553.8 ns, slowest 1075151.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1075991ns | 1072799ns | 1065844ns | 1073697ns | 1093020ns | base |
| warm-container-minimum | 53072ns | 52707ns | 52346ns | 52821ns | 54553ns | -95.07% |
| warm-container-native | 54264ns | 53359ns | 52517ns | 53507ns | 58278ns | -94.96% |
| warm-container-plusone | 1084716ns | 1076005ns | 1066965ns | 1076726ns | 1126438ns | +0.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1075153ns | 1064718ns | 1092502ns | base | 3.901 |
| warm-container-minimum | 52923ns | 52216ns | 54393ns | -95.08% | 79.253 |
| warm-container-native | 54117ns | 52364ns | 58113ns | -94.97% | 77.504 |
| warm-container-plusone | 1083843ns | 1065954ns | 1125439ns | +0.81% | 3.870 |

## Performance model

- Peak throughput: **80.326 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.913 | 4.9% |
| warm-container-minimum | 79.810 | 99.4% |
| warm-container-native | 78.809 | 98.1% |
| warm-container-plusone | 3.901 | 4.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1075991ns | 1075991ns | base |
| warm-container-minimum | 53072ns | 53072ns | -95.07% |
| warm-container-native | 54264ns | 54264ns | -94.96% |
| warm-container-plusone | 1084716ns | 1084716ns | +0.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1071854ns | base | --- | [1069078, 1076161] | --- | --- | --- | --- |
| warm-container-minimum | 52554ns | -1018929.4ns (-95.1%) | [-1023104, -1015870]ns | [52340, 53055] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 53221ns | -1018020.9ns (-95.0%) | [-1021667, -1015606]ns | [52993, 53385] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1075151ns | no significant difference | [-1218, +8861]ns | [1068919, 1081562] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 1076281ns | -95.0% | -95.1% | +0.7% |
| 2 | 1075262ns | -95.1% | -95.0% | +2.1% |
| 3 | 1074077ns | -95.1% | -95.1% | +7.4% |
| 4 | 1071222ns | -95.1% | -95.0% | +16.3% |
| 5 | 1083275ns | -95.1% | -95.1% | +5.0% |
| 6 | 1093579ns | -95.2% | -95.1% | +0.0% |
| 7 | 1078004ns | -95.1% | -95.1% | +0.3% |
| 8 | 1073731ns | -95.1% | -94.6% | -0.1% |
| 9 | 1079631ns | -95.1% | -94.5% | +0.4% |
| 10 | 1071565ns | -95.1% | -95.0% | +2.3% |
| 11 | 1070978ns | -94.9% | -95.0% | -0.2% |
| 12 | 1068937ns | -95.1% | -95.0% | -0.0% |
| 13 | 1067794ns | -95.1% | -94.9% | +0.2% |
| 14 | 1065412ns | -95.1% | -94.8% | +2.0% |
| 15 | 1067878ns | -95.1% | -94.7% | +1.0% |
| 16 | 1078404ns | -95.1% | -94.6% | -1.3% |
| 17 | 1120442ns | -95.3% | -94.9% | -4.7% |
| 18 | 1088852ns | -95.2% | -94.5% | -1.6% |
| 19 | 1067812ns | -95.1% | -94.4% | -0.0% |
| 20 | 1088711ns | -95.2% | -94.8% | -2.1% |
| 21 | 1062890ns | -94.8% | -94.9% | +0.3% |
| 22 | 1072144ns | -95.0% | -95.0% | -0.5% |
| 23 | 1076041ns | -95.1% | -95.1% | +1.5% |
| 24 | 1074336ns | -95.1% | -95.1% | -0.5% |
| 25 | 1086022ns | -95.2% | -95.1% | -1.7% |
| 26 | 1070322ns | -95.1% | -95.1% | +1.5% |
| 27 | 1063553ns | -95.1% | -95.1% | +1.3% |
| 28 | 1064492ns | -95.1% | -95.1% | +1.9% |
| 29 | 1091881ns | -95.2% | -95.2% | -1.1% |
| 30 | 1065890ns | -95.1% | -95.1% | +0.0% |
| 31 | 1070157ns | -95.1% | -94.9% | -0.5% |
| 32 | 1067450ns | -95.1% | -95.0% | -0.2% |
| 33 | 1086785ns | -95.1% | -95.2% | -0.6% |
| 34 | 1065072ns | -95.0% | -95.1% | +1.9% |
| 35 | 1065393ns | -95.0% | -95.0% | +1.5% |
| 36 | 1065039ns | -95.0% | -95.1% | +0.2% |
| 37 | 1076480ns | -95.1% | -95.0% | -0.3% |
| 38 | 1083746ns | -95.1% | -95.1% | -1.4% |
| 39 | 1067342ns | -94.8% | -95.1% | +0.4% |
| 40 | 1069219ns | -94.7% | -95.1% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.130 | ok |
| warm-container-minimum | 0.435 | moderate+ |
| warm-container-native | 0.717 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.620 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 14/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 13.2ns | 1075152.6ns | 0.0% |  |
| warm-container-minimum | 2.7ns | 52923.0ns | 0.0% |  |
| warm-container-native | 3.0ns | 54117.0ns | 0.0% |  |
| warm-container-plusone | 26.3ns | 1083843.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1064717.8-1092502.4 ns)
  1064717.8 |########################################
  1066107.0 |################
  1067496.2 |########################
  1068885.5 |########################
  1070274.7 |################################
  1071663.9 |########
  1073053.1 |########################
  1074442.4 |########
  1075831.6 |########################
  1077220.8 |################
  1078610.1 |########
  1079999.3 |
  1081388.5 |
  1082777.8 |################
  1084167.0 |
  1085556.2 |################
  1086945.5 |
  1088334.7 |################
  1089723.9 |
  1091113.2 |########
  (3 below, 2 above range)

warm-container-minimum (n=40, range 52215.9-54393.1 ns)
  52215.9 |########################################
  52324.7 |##############################
  52433.6 |##########
  52542.5 |###############
  52651.3 |#####
  52760.2 |#####
  52869.0 |#####
  52977.9 |
  53086.8 |###############
  53195.6 |###############
  53304.5 |###############
  53413.3 |
  53522.2 |#####
  53631.1 |
  53739.9 |
  53848.8 |
  53957.6 |
  54066.5 |
  54175.4 |#####
  54284.2 |
  (4 below, 3 above range)

warm-container-native (n=40, range 52364.3-58113.4 ns)
  52364.3 |#########################
  52651.8 |##########
  52939.2 |########################################
  53226.7 |###################################
  53514.1 |#####
  53801.6 |
  54089.0 |
  54376.5 |##########
  54664.0 |
  54951.4 |#####
  55238.9 |#####
  55526.3 |
  55813.8 |
  56101.2 |#####
  56388.7 |#####
  56676.2 |
  56963.6 |#####
  57251.1 |
  57538.5 |
  57826.0 |#####
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1065954.0-1125438.7 ns)
  1065954.0 |########################################
  1068928.2 |##########
  1071902.4 |##########
  1074876.7 |###
  1077850.9 |##########
  1080825.1 |##############
  1083799.4 |##############
  1086773.6 |###
  1089747.9 |###
  1092722.1 |###
  1095696.3 |#######
  1098670.6 |
  1101644.8 |
  1104619.0 |
  1107593.3 |
  1110567.5 |
  1113541.8 |
  1116516.0 |
  1119490.2 |
  1122464.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-native**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.62 (measurement drift or warm-up artifact)
