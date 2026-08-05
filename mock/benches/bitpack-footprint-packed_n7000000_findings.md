# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 331% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (1.16 ms) leads bitpack-footprint-packed-naive (4.99 ms) by 331%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive shows warm-up / thermal drift (autocorr +0.86)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (1.16 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-footprint-packed (1.16 ms) to slowest bitpack-footprint-packed-naive (4.99 ms): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 1157448.1 ns median
- 1 variant significantly slower than baseline
- Spread: 4.31x (fastest 1157448.1 ns, slowest 4993673.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 1180086ns | 1158746ns | 1131070ns | 1165119ns | 1274002ns | base |
| bitpack-footprint-packed-naive | 5089767ns | 4995894ns | 4936449ns | 5027956ns | 5428521ns | +331.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 1178830ns | 1129676ns | 1272567ns | base | 5.938 |
| bitpack-footprint-packed-naive | 5087765ns | 4934803ns | 5426081ns | +331.59% | 1.376 |

## Performance model

- Peak throughput: **6.196 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 6.048 | 97.6% |
| bitpack-footprint-packed-naive | 1.402 | 22.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 1180086ns | 1180086ns | base |
| bitpack-footprint-packed-naive | 5089767ns | 5089767ns | +331.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 1157448ns | base | --- | [1146327, 1169791] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 4993673ns | +3844256.9ns (+332.1%) | [+3821393, +3881150]ns | [4972585, 5034442] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|
| 1 | 1255900ns | +334.9% |
| 2 | 1264865ns | +328.8% |
| 3 | 1231425ns | +348.3% |
| 4 | 1357823ns | +298.4% |
| 5 | 1219907ns | +344.4% |
| 6 | 1288210ns | +315.7% |
| 7 | 1331626ns | +297.6% |
| 8 | 1223780ns | +334.2% |
| 9 | 1223258ns | +345.1% |
| 10 | 1209225ns | +344.2% |
| 11 | 1158485ns | +338.1% |
| 12 | 1147708ns | +338.7% |
| 13 | 1226910ns | +305.3% |
| 14 | 1116187ns | +350.9% |
| 15 | 1168555ns | +329.1% |
| 16 | 1136279ns | +332.4% |
| 17 | 1139139ns | +335.0% |
| 18 | 1133088ns | +340.1% |
| 19 | 1177803ns | +326.2% |
| 20 | 1169038ns | +326.6% |
| 21 | 1188249ns | +322.2% |
| 22 | 1170544ns | +330.9% |
| 23 | 1143032ns | +335.4% |
| 24 | 1145117ns | +334.2% |
| 25 | 1143305ns | +333.7% |
| 26 | 1156411ns | +330.8% |
| 27 | 1144577ns | +338.4% |
| 28 | 1168321ns | +324.4% |
| 29 | 1132908ns | +350.2% |
| 30 | 1148617ns | +331.1% |
| 31 | 1126472ns | +341.6% |
| 32 | 1138952ns | +334.1% |
| 33 | 1149892ns | +327.5% |
| 34 | 1150595ns | +329.0% |
| 35 | 1137081ns | +335.0% |
| 36 | 1163268ns | +325.7% |
| 37 | 1129245ns | +339.8% |
| 38 | 1163714ns | +326.8% |
| 39 | 1126147ns | +344.0% |
| 40 | 1147538ns | +328.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.611 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed-naive | 0.859 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 62.6ns | 1178829.9ns | 0.0% |  |
| bitpack-footprint-packed-naive | 262.1ns | 5087764.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 1129675.8-1272567.5 ns)
  1129675.8 |#################
  1136820.4 |############################
  1143965.0 |########################################
  1151109.6 |#####
  1158254.2 |#################
  1165398.7 |######################
  1172543.3 |#####
  1179687.9 |
  1186832.5 |#####
  1193977.1 |
  1201121.7 |
  1208266.2 |#####
  1215410.8 |#####
  1222555.4 |#################
  1229700.0 |#####
  1236844.6 |
  1243989.2 |
  1251133.7 |#####
  1258278.3 |#####
  1265422.9 |
  (4 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 4934803.0-5426080.8 ns)
  4934803.0 |########################################
  4959366.9 |###################################
  4983930.8 |###############
  5008494.7 |####################
  5033058.6 |###############
  5057622.5 |#####
  5082186.3 |#####
  5106750.2 |
  5131314.1 |
  5155878.0 |
  5180441.9 |
  5205005.8 |
  5229569.7 |
  5254133.6 |
  5278697.5 |#####
  5303261.3 |#####
  5327825.2 |
  5352389.1 |##########
  5376953.0 |
  5401516.9 |###############
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **bitpack-footprint-packed-naive**: autocorrelation=0.86 (measurement drift or warm-up artifact)
