# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (1.10 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 331.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 228% faster than the next best (warm-container-plusone)

warm-container-native (331.70 us) leads warm-container-plusone (1.09 ms) by 228%, a clear separation rather than a photo finish. CV 14.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 70% (significant)

warm-container-native is -773.28 us (70%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.3x slower than the field

warm-container-headroom (1.10 ms) is 3.3x the fastest (331.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 14.1%)

warm-container-native wins on median (331.70 us) yet has the highest variance (CV 14.1%), while warm-container-headroom is the steadiest (CV 1.0%, 1.10 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-native} vs {warm-container-plusone, warm-container-minimum, warm-container-headroom} (228% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 228% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest warm-container-native (331.70 us) to slowest warm-container-headroom (1.10 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 331699.2 ns median (-69.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.32x (fastest 331699.2 ns, slowest 1101297.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1103020ns | 1102622ns | 1087459ns | 1102645ns | 1119704ns | base |
| warm-container-minimum | 1094903ns | 1090263ns | 1082430ns | 1091646ns | 1117149ns | -0.74% |
| warm-container-native | 344872ns | 332074ns | 323602ns | 333421ns | 400494ns | -68.73% |
| warm-container-plusone | 1094023ns | 1088020ns | 1078313ns | 1088602ns | 1125994ns | -0.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1101732ns | 1086271ns | 1118461ns | base | 3.807 |
| warm-container-minimum | 1093894ns | 1081303ns | 1116187ns | -0.71% | 3.834 |
| warm-container-native | 344233ns | 323151ns | 399309ns | -68.76% | 12.185 |
| warm-container-plusone | 1093097ns | 1077506ns | 1125083ns | -0.78% | 3.837 |

## Performance model

- Peak throughput: **12.979 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.809 | 29.3% |
| warm-container-minimum | 3.851 | 29.7% |
| warm-container-native | 12.645 | 97.4% |
| warm-container-plusone | 3.859 | 29.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1103020ns | 1103020ns | base |
| warm-container-minimum | 1094903ns | 1094903ns | -0.74% |
| warm-container-native | 344872ns | 344872ns | -68.73% |
| warm-container-plusone | 1094023ns | 1094023ns | -0.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1101298ns | base | --- | [1096636, 1105379] | --- | --- | --- | --- |
| warm-container-minimum | 1089092ns | -10650.6ns (-1.0%) | [-16998, -3620]ns | [1086999, 1094575] | YES | 0.0166 | 0.0166 | 0 |
| warm-container-native | 331699ns | -765940.2ns (-69.5%) | [-771524, -759230]ns | [329916, 333962] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1086989ns | -12437.3ns (-1.1%) | [-18510, -7904]ns | [1084118, 1090743] | YES | 0.0003 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 1107135ns | -1.6% | -45.7% | -2.3% |
| 2 | 1117330ns | -2.6% | -58.9% | -0.8% |
| 3 | 1117229ns | -0.1% | -66.7% | -0.6% |
| 4 | 1095860ns | +0.4% | -68.6% | -1.8% |
| 5 | 1087525ns | +2.5% | -67.8% | -0.5% |
| 6 | 1090192ns | +0.4% | -66.1% | +0.1% |
| 7 | 1080021ns | +2.6% | -68.0% | +0.7% |
| 8 | 1101942ns | -0.6% | -68.5% | -1.2% |
| 9 | 1104070ns | -1.6% | -68.7% | -1.8% |
| 10 | 1103150ns | -2.4% | -68.3% | +16.3% |
| 11 | 1117062ns | -2.5% | -70.2% | -1.9% |
| 12 | 1100050ns | -1.0% | -69.6% | +0.2% |
| 13 | 1115157ns | -1.8% | -70.7% | -2.8% |
| 14 | 1088587ns | +0.1% | -69.6% | -0.2% |
| 15 | 1087627ns | +2.9% | -69.0% | -0.7% |
| 16 | 1089395ns | +4.3% | -69.8% | -0.4% |
| 17 | 1089329ns | +2.2% | -69.7% | +0.4% |
| 18 | 1096155ns | -0.8% | -70.5% | -0.7% |
| 19 | 1096326ns | -1.2% | -70.8% | +0.2% |
| 20 | 1121571ns | -3.0% | -70.8% | -2.8% |
| 21 | 1101005ns | -0.1% | -70.2% | -2.1% |
| 22 | 1096946ns | +1.7% | -69.8% | -1.0% |
| 23 | 1102648ns | -1.5% | -69.9% | -1.7% |
| 24 | 1090790ns | -0.6% | -69.6% | +0.5% |
| 25 | 1098200ns | -1.2% | -70.0% | -1.2% |
| 26 | 1095097ns | -0.9% | -70.0% | -1.0% |
| 27 | 1100263ns | -1.3% | -69.9% | -1.6% |
| 28 | 1083775ns | +0.5% | -70.2% | +1.0% |
| 29 | 1121312ns | -2.5% | -70.2% | -3.0% |
| 30 | 1101590ns | -0.8% | -70.0% | -1.6% |
| 31 | 1103922ns | -2.1% | -69.8% | -0.3% |
| 32 | 1110501ns | -2.2% | -70.6% | -0.8% |
| 33 | 1123967ns | -3.5% | -71.2% | -4.4% |
| 34 | 1114061ns | -3.3% | -70.7% | -2.4% |
| 35 | 1112352ns | -2.3% | -71.1% | -1.7% |
| 36 | 1083910ns | +0.2% | -70.4% | -1.1% |
| 37 | 1106687ns | +0.0% | -70.0% | -2.4% |
| 38 | 1107766ns | -0.9% | -70.0% | -1.3% |
| 39 | 1100090ns | -1.9% | -68.9% | -1.7% |
| 40 | 1108672ns | -1.2% | -69.6% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.217 | moderate+ |
| warm-container-minimum | 0.429 | moderate+ |
| warm-container-native | 0.451 | moderate+ |
| warm-container-plusone | -0.003 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 26/40, lost 11/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 32/40, lost 7/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.7ns | 1101731.7ns | 0.0% |  |
| warm-container-minimum | 37.2ns | 1093894.1ns | 0.0% |  |
| warm-container-native | 56.1ns | 344232.6ns | 0.0% |  |
| warm-container-plusone | 32.1ns | 1093096.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1086271.1-1118461.1 ns)
  1086271.1 |##########################
  1087880.6 |########################################
  1089490.1 |##########################
  1091099.6 |
  1092709.1 |
  1094318.6 |##########################
  1095928.1 |########################################
  1097537.6 |#############
  1099147.1 |########################################
  1100756.6 |########################################
  1102366.1 |########################################
  1103975.6 |#############
  1105585.1 |##########################
  1107194.6 |##########################
  1108804.1 |
  1110413.6 |#############
  1112023.1 |#############
  1113632.6 |##########################
  1115242.1 |
  1116851.6 |########################################
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1081302.6-1116186.5 ns)
  1081302.6 |
  1083046.7 |################################
  1084790.9 |########################################
  1086535.1 |########################################
  1088279.3 |########################################
  1090023.5 |
  1091767.7 |########
  1093511.9 |################################
  1095256.1 |########
  1097000.3 |########
  1098744.5 |################
  1100488.7 |
  1102232.9 |
  1103977.1 |
  1105721.3 |########
  1107465.5 |########
  1109209.7 |
  1110953.9 |
  1112698.1 |########
  1114442.3 |########################
  (4 below, 2 above range)

warm-container-native (n=40, range 323151.4-399308.6 ns)
  323151.4 |#####################
  326959.2 |#########################
  330767.1 |########################################
  334575.0 |#######
  338382.8 |###
  342190.7 |#######
  345998.6 |##########
  349806.4 |###
  353614.3 |
  357422.2 |
  361230.0 |
  365037.9 |
  368845.7 |#######
  372653.6 |
  376461.5 |
  380269.3 |
  384077.2 |
  387885.1 |
  391692.9 |
  395500.8 |
  (3 below, 2 above range)

warm-container-plusone (n=40, range 1077506.0-1125083.1 ns)
  1077506.0 |################################
  1079884.8 |########################
  1082263.7 |########################################
  1084642.5 |########################################
  1087021.4 |########################################
  1089400.2 |################
  1091779.1 |################
  1094158.0 |################################
  1096536.8 |
  1098915.7 |########################
  1101294.5 |########
  1103673.4 |
  1106052.2 |########
  1108431.1 |########
  1110809.9 |
  1113188.8 |
  1115567.7 |
  1117946.5 |
  1120325.4 |
  1122704.2 |
  (3 below, 1 above range)

```
