# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (1.08 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 54.69 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 1874% faster than the next best (warm-container-minimum)

warm-container-native (54.69 us) leads warm-container-minimum (1.08 ms) by 1874%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 95% (significant)

warm-container-native is -1.03 ms (95%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 19.8x slower than the field

warm-container-headroom (1.08 ms) is 19.8x the fastest (54.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 5.7%)

warm-container-native wins on median (54.69 us) yet has the highest variance (CV 5.7%), while warm-container-headroom is the steadiest (CV 0.9%, 1.08 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows warm-up / thermal drift (autocorr +0.82)

warm-container-native's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (1874% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 1874% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 19.8x the fastest

Fastest warm-container-native (54.69 us) to slowest warm-container-headroom (1.08 ms): 19.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 54687.9 ns median (-94.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 19.76x (fastest 54687.9 ns, slowest 1080564.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1084188ns | 1081478ns | 1073192ns | 1082674ns | 1099727ns | base |
| warm-container-minimum | 1083004ns | 1080389ns | 1070822ns | 1081122ns | 1100834ns | -0.11% |
| warm-container-native | 55858ns | 54814ns | 52596ns | 55146ns | 61255ns | -94.85% |
| warm-container-plusone | 1086160ns | 1081358ns | 1073941ns | 1083758ns | 1105583ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1083380ns | 1072214ns | 1099184ns | base | 3.871 |
| warm-container-minimum | 1082177ns | 1069900ns | 1100218ns | -0.11% | 3.876 |
| warm-container-native | 55686ns | 52453ns | 61048ns | -94.86% | 75.321 |
| warm-container-plusone | 1085366ns | 1073055ns | 1104914ns | +0.18% | 3.864 |

## Performance model

- Peak throughput: **79.963 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.882 | 4.9% |
| warm-container-minimum | 3.886 | 4.9% |
| warm-container-native | 76.695 | 95.9% |
| warm-container-plusone | 3.882 | 4.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1084188ns | 1084188ns | base |
| warm-container-minimum | 1083004ns | 1083004ns | -0.11% |
| warm-container-native | 55858ns | 55858ns | -94.85% |
| warm-container-plusone | 1086160ns | 1086160ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1080565ns | base | --- | [1079132, 1084953] | --- | --- | --- | --- |
| warm-container-minimum | 1079468ns | no significant difference | [-7588, +1016]ns | [1076848, 1083155] | no | 0.1210 | 0.0807 | 0 |
| warm-container-native | 54688ns | -1025935.9ns (-94.9%) | [-1029041, -1023329]ns | [53477, 55973] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1080564ns | no significant difference | [-5759, +7110]ns | [1078776, 1085832] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 1094494ns | -1.8% | -95.2% | -0.9% |
| 2 | 1103038ns | -2.9% | -95.2% | -2.1% |
| 3 | 1088363ns | -1.7% | -95.2% | -0.9% |
| 4 | 1076847ns | -0.8% | -95.1% | +2.5% |
| 5 | 1077762ns | -0.9% | -95.2% | +1.1% |
| 6 | 1075308ns | -0.1% | -95.1% | +2.0% |
| 7 | 1072020ns | +1.4% | -95.1% | +0.5% |
| 8 | 1070877ns | +2.9% | -95.1% | +0.4% |
| 9 | 1079236ns | -0.9% | -95.1% | +0.7% |
| 10 | 1084631ns | -0.9% | -95.1% | -0.8% |
| 11 | 1085275ns | -0.3% | -95.0% | -0.1% |
| 12 | 1088205ns | -1.5% | -95.1% | -0.8% |
| 13 | 1080291ns | -0.2% | -94.9% | -0.5% |
| 14 | 1109354ns | -2.8% | -94.9% | -0.2% |
| 15 | 1094795ns | -0.2% | -94.7% | -1.3% |
| 16 | 1080280ns | +2.2% | -94.8% | -0.1% |
| 17 | 1080146ns | -0.3% | -94.6% | +0.5% |
| 18 | 1080838ns | +0.2% | -94.6% | +1.3% |
| 19 | 1080955ns | -0.1% | -95.0% | +1.8% |
| 20 | 1079592ns | +3.4% | -95.0% | -0.4% |
| 21 | 1079198ns | +0.5% | -95.0% | +3.0% |
| 22 | 1085890ns | -0.3% | -95.1% | -0.7% |
| 23 | 1103044ns | -2.4% | -95.0% | -2.0% |
| 24 | 1103978ns | -1.3% | -95.2% | -3.1% |
| 25 | 1077157ns | -0.0% | -94.9% | +0.8% |
| 26 | 1073478ns | +1.2% | -94.9% | +0.7% |
| 27 | 1069998ns | +1.7% | -94.9% | +0.6% |
| 28 | 1079066ns | +1.3% | -95.1% | +2.2% |
| 29 | 1089738ns | +1.3% | -95.0% | +0.9% |
| 30 | 1082062ns | +1.4% | -94.9% | +0.4% |
| 31 | 1091867ns | -1.4% | -94.9% | -1.7% |
| 32 | 1083528ns | -0.0% | -94.8% | -0.7% |
| 33 | 1074425ns | +1.8% | -94.3% | -0.1% |
| 34 | 1069845ns | +1.2% | -94.4% | +1.0% |
| 35 | 1075306ns | +0.7% | -94.4% | +1.8% |
| 36 | 1071762ns | -0.1% | -94.4% | +3.7% |
| 37 | 1083215ns | -0.6% | -94.4% | +1.8% |
| 38 | 1092905ns | -1.3% | -94.3% | -1.2% |
| 39 | 1088741ns | -1.8% | -94.4% | -1.6% |
| 40 | 1077711ns | -0.5% | -94.2% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.462 | moderate+ |
| warm-container-minimum | 0.293 | moderate+ |
| warm-container-native | 0.820 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.120 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 21/40, lost 14/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 18/40, lost 20/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.3ns | 1083380.4ns | 0.0% |  |
| warm-container-minimum | 25.9ns | 1082177.3ns | 0.0% |  |
| warm-container-native | 5.7ns | 55685.7ns | 0.0% |  |
| warm-container-plusone | 24.7ns | 1085366.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1072213.9-1099184.2 ns)
  1072213.9 |#####
  1073562.4 |#####
  1074910.9 |###########
  1076259.4 |###########
  1077607.9 |###########
  1078956.4 |########################################
  1080304.9 |###########
  1081653.5 |#####
  1083002.0 |###########
  1084350.5 |###########
  1085699.0 |#####
  1087047.5 |###########
  1088396.0 |###########
  1089744.6 |
  1091093.1 |#####
  1092441.6 |#####
  1093790.1 |###########
  1095138.6 |
  1096487.1 |
  1097835.7 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1069900.4-1100217.8 ns)
  1069900.4 |################
  1071416.2 |################
  1072932.1 |########
  1074448.0 |################
  1075963.9 |########################################
  1077479.7 |########################
  1078995.6 |########
  1080511.5 |########
  1082027.3 |########################################
  1083543.2 |########
  1085059.1 |################
  1086575.0 |########
  1088090.8 |
  1089606.7 |########
  1091122.6 |########
  1092638.5 |################
  1094154.3 |
  1095670.2 |########
  1097186.1 |
  1098702.0 |
  (5 below, 4 above range)

warm-container-native (n=40, range 52452.8-61047.8 ns)
  52452.8 |########################################
  52882.6 |##########################
  53312.3 |#############
  53742.1 |
  54171.8 |##########################
  54601.6 |####################
  55031.3 |####################
  55461.1 |######
  55890.8 |
  56320.6 |####################
  56750.3 |
  57180.0 |
  57609.8 |#############
  58039.5 |
  58469.3 |######
  58899.0 |
  59328.8 |######
  59758.5 |
  60188.3 |####################
  60618.0 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1073055.2-1104914.2 ns)
  1073055.2 |####################
  1074648.1 |##########################
  1076241.1 |#############
  1077834.0 |####################
  1079427.0 |########################################
  1081019.9 |######
  1082612.9 |#############
  1084205.8 |#############
  1085798.8 |#############
  1087391.7 |
  1088984.7 |######
  1090577.6 |
  1092170.6 |
  1093763.5 |#############
  1095356.5 |######
  1096949.4 |
  1098542.4 |######
  1100135.4 |######
  1101728.3 |#############
  1103321.3 |######
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-native**: autocorrelation=0.82 (measurement drift or warm-up artifact)
