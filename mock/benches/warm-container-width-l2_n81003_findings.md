# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 99% (significant)

warm-container-minimum is -1.07 ms (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 40.9x slower than the field

warm-container-plusone (1.08 ms) is 40.9x the fastest (26.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (26.44 us) and warm-container-minimum (26.50 us) differ by 0.21%, inside the noise, even though the wider field spreads 3994.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.60)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (3969% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 3969% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 40.9x the fastest

Fastest warm-container-native (26.44 us) to slowest warm-container-plusone (1.08 ms): 40.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 26440.2 ns median (-97.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 40.94x (fastest 26440.2 ns, slowest 1082499.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1083734ns | 1078864ns | 1069827ns | 1080244ns | 1108112ns | base |
| warm-container-minimum | 27021ns | 26616ns | 26256ns | 26651ns | 28894ns | -97.51% |
| warm-container-native | 27062ns | 26579ns | 26251ns | 26861ns | 28477ns | -97.50% |
| warm-container-plusone | 1084977ns | 1083191ns | 1068800ns | 1082541ns | 1108462ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1082881ns | 1068848ns | 1107301ns | base | 3.873 |
| warm-container-minimum | 26890ns | 26148ns | 28691ns | -97.52% | 155.980 |
| warm-container-native | 26935ns | 26136ns | 28321ns | -97.51% | 155.721 |
| warm-container-plusone | 1084145ns | 1067884ns | 1107596ns | +0.12% | 3.869 |

## Performance model

- Peak throughput: **160.482 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.890 | 2.4% |
| warm-container-minimum | 158.297 | 98.6% |
| warm-container-native | 158.634 | 98.8% |
| warm-container-plusone | 3.875 | 2.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1083734ns | 1083734ns | base |
| warm-container-minimum | 27021ns | 27021ns | -97.51% |
| warm-container-native | 27062ns | 27062ns | -97.50% |
| warm-container-plusone | 1084977ns | 1084977ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1078137ns | base | --- | [1075428, 1084712] | --- | --- | --- | --- |
| warm-container-minimum | 26496ns | -1050631.9ns (-97.4%) | [-1056617, -1049065]ns | [26258, 26709] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 26440ns | -1050833.9ns (-97.5%) | [-1057220, -1048743]ns | [26276, 27310] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1082499ns | no significant difference | [-1983, +5286]ns | [1074835, 1087746] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 1088246ns | -97.6% | -97.4% | +0.1% |
| 2 | 1078055ns | -97.5% | -97.5% | +2.3% |
| 3 | 1079844ns | -97.4% | -97.4% | +1.9% |
| 4 | 1079542ns | -97.4% | -97.4% | +0.3% |
| 5 | 1082625ns | -97.2% | -97.4% | +0.6% |
| 6 | 1097694ns | -97.5% | -97.5% | -1.4% |
| 7 | 1091254ns | -97.1% | -97.4% | +1.1% |
| 8 | 1078812ns | -97.4% | -97.4% | +0.3% |
| 9 | 1158515ns | -97.6% | -97.7% | -2.2% |
| 10 | 1111214ns | -97.5% | -97.6% | -1.8% |
| 11 | 1100736ns | -97.6% | -97.6% | -0.8% |
| 12 | 1086850ns | -97.6% | -97.6% | +1.1% |
| 13 | 1094866ns | -97.6% | -97.6% | -1.1% |
| 14 | 1075892ns | -97.6% | -97.6% | +0.6% |
| 15 | 1068695ns | -97.5% | -97.6% | +1.6% |
| 16 | 1075586ns | -97.6% | -97.5% | +1.4% |
| 17 | 1075269ns | -97.6% | -97.6% | +1.0% |
| 18 | 1091182ns | -97.6% | -97.5% | +1.5% |
| 19 | 1089268ns | -97.6% | -97.5% | -0.2% |
| 20 | 1093705ns | -97.6% | -97.5% | -1.2% |
| 21 | 1075679ns | -97.5% | -97.6% | +0.1% |
| 22 | 1072722ns | -97.5% | -97.6% | +0.3% |
| 23 | 1069006ns | -97.6% | -97.5% | +4.6% |
| 24 | 1077272ns | -97.5% | -97.6% | +1.8% |
| 25 | 1068740ns | -97.5% | -97.5% | +2.6% |
| 26 | 1071027ns | -97.5% | -97.4% | -0.3% |
| 27 | 1078220ns | -97.4% | -97.2% | -0.9% |
| 28 | 1086800ns | -97.5% | -97.5% | -1.8% |
| 29 | 1080490ns | -97.5% | -97.6% | -1.2% |
| 30 | 1070534ns | -97.5% | -97.6% | -0.1% |
| 31 | 1070168ns | -97.5% | -97.4% | +0.8% |
| 32 | 1069722ns | -97.6% | -97.2% | +0.3% |
| 33 | 1071275ns | -97.5% | -97.4% | -0.1% |
| 34 | 1068138ns | -97.5% | -97.5% | +0.0% |
| 35 | 1067558ns | -97.5% | -97.5% | +0.0% |
| 36 | 1104089ns | -97.6% | -97.6% | -3.1% |
| 37 | 1073268ns | -97.6% | -97.6% | +0.0% |
| 38 | 1068756ns | -97.6% | -97.5% | +0.4% |
| 39 | 1076332ns | -97.6% | -97.6% | -1.0% |
| 40 | 1097590ns | -97.6% | -97.6% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.349 | moderate+ |
| warm-container-minimum | 0.605 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.498 | moderate+ |
| warm-container-plusone | 0.416 | moderate+ |

**Consistency summary:**

- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 16/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 28.3ns | 1082880.9ns | 0.0% |  |
| warm-container-minimum | 7.7ns | 26890.0ns | 0.0% |  |
| warm-container-native | 3.8ns | 26934.8ns | 0.0% |  |
| warm-container-plusone | 26.0ns | 1084144.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1068847.9-1107301.1 ns)
  1068847.9 |################################
  1070770.6 |################
  1072693.3 |################
  1074615.9 |########################################
  1076538.6 |########################
  1078461.2 |########################
  1080383.9 |########
  1082306.6 |########
  1084229.2 |
  1086151.9 |################
  1088074.5 |################
  1089997.2 |################
  1091919.8 |########
  1093842.5 |########
  1095765.2 |########
  1097687.8 |########
  1099610.5 |########
  1101533.1 |
  1103455.8 |########
  1105378.4 |
  (5 below, 2 above range)

warm-container-minimum (n=40, range 26148.1-28690.9 ns)
  26148.1 |########################################
  26275.3 |######################
  26402.4 |#################
  26529.6 |#############
  26656.7 |####
  26783.8 |########
  26911.0 |####
  27038.1 |####
  27165.3 |
  27292.4 |
  27419.5 |
  27546.7 |########
  27673.8 |########
  27801.0 |
  27928.1 |
  28055.2 |
  28182.4 |####
  28309.5 |####
  28436.7 |
  28563.8 |####
  (5 below, 2 above range)

warm-container-native (n=40, range 26135.6-28321.2 ns)
  26135.6 |########################################
  26244.9 |########################
  26354.2 |####
  26463.5 |####
  26572.7 |
  26682.0 |####
  26791.3 |
  26900.6 |
  27009.9 |########
  27119.1 |####
  27228.4 |############
  27337.7 |####
  27447.0 |########
  27556.2 |############
  27665.5 |####
  27774.8 |########
  27884.1 |
  27993.4 |
  28102.6 |
  28211.9 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1067884.0-1107596.3 ns)
  1067884.0 |########################################
  1069869.6 |#############
  1071855.2 |####################
  1073840.9 |
  1075826.5 |#############
  1077812.1 |######
  1079797.7 |######
  1081783.3 |#################################
  1083768.9 |######
  1085754.5 |#############
  1087740.2 |#############
  1089725.8 |#############
  1091711.4 |######
  1093697.0 |
  1095682.6 |#############
  1097668.2 |######
  1099653.8 |######
  1101639.5 |#############
  1103625.1 |
  1105610.7 |######
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-minimum**: autocorrelation=0.60 (measurement drift or warm-up artifact)
