# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.55)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.34 us) is smaller than the fastest variant's own run-to-run std-dev (6.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 5 variants sit between 1.21 ms and 1.21 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-17 ns, 0.00%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -17 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1210327.1 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 1210327.1 ns, slowest 1213670.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1214246ns | 1213880ns | 1209759ns | 1213829ns | 1219985ns | base |
| wide-rung-ragged | 1215768ns | 1214298ns | 1209603ns | 1214477ns | 1225809ns | +0.13% |
| wide-rung-ragged-overread | 1214155ns | 1211663ns | 1208549ns | 1212218ns | 1225570ns | -0.01% |
| wide-rung-wordround | 1218536ns | 1215016ns | 1210864ns | 1214876ns | 1237188ns | +0.35% |
| wide-rung-wordround-alias | 1215896ns | 1211978ns | 1208838ns | 1212576ns | 1232913ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1213059ns | 1208506ns | 1218819ns | base | 1.513 |
| wide-rung-ragged | 1214478ns | 1208408ns | 1224338ns | +0.12% | 1.511 |
| wide-rung-ragged-overread | 1212923ns | 1207288ns | 1224307ns | -0.01% | 1.513 |
| wide-rung-wordround | 1217273ns | 1209482ns | 1236014ns | +0.35% | 1.507 |
| wide-rung-wordround-alias | 1214702ns | 1207656ns | 1231446ns | +0.14% | 1.511 |

## Performance model

- Peak throughput: **1.520 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.513 | 99.6% |
| wide-rung-ragged | 1.513 | 99.5% |
| wide-rung-ragged-overread | 1.516 | 99.7% |
| wide-rung-wordround | 1.512 | 99.5% |
| wide-rung-wordround-alias | 1.515 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1214246ns | 1214246ns | base |
| wide-rung-ragged | 1215768ns | 1215768ns | +0.13% |
| wide-rung-ragged-overread | 1214155ns | 1214155ns | -0.01% |
| wide-rung-wordround | 1218536ns | 1218536ns | +0.35% |
| wide-rung-wordround-alias | 1215896ns | 1215896ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1212711ns | base | --- | [1211719, 1213848] | --- | --- | --- | --- |
| wide-rung-ragged | 1213080ns | no significant difference | [-1757, +3133]ns | [1212416, 1213844] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-ragged-overread | 1210327ns | no significant difference | [-2959, +847]ns | [1209639, 1212610] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround | 1213671ns | no significant difference | [-1613, +2385]ns | [1212036, 1214989] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround-alias | 1210952ns | no significant difference | [-3885, +839]ns | [1210450, 1212428] | no | 0.8478 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1210631ns | +0.4% | +0.2% | +0.3% | +0.1% |
| 2 | 1210252ns | -0.1% | +0.2% | +5.8% | -0.3% |
| 3 | 1212548ns | -0.2% | +0.0% | +4.7% | -0.7% |
| 4 | 1214070ns | -0.4% | +0.2% | -0.2% | -0.3% |
| 5 | 1218272ns | -0.9% | +1.2% | -0.5% | -0.4% |
| 6 | 1214749ns | +0.9% | -0.1% | -0.0% | +2.6% |
| 7 | 1213284ns | -0.5% | +0.9% | -0.3% | +1.8% |
| 8 | 1210902ns | -0.3% | +1.1% | +0.2% | -0.1% |
| 9 | 1211500ns | -0.2% | +1.1% | +0.4% | +0.0% |
| 10 | 1212730ns | -0.2% | +0.0% | +0.0% | +1.9% |
| 11 | 1205605ns | +0.5% | +0.9% | +0.5% | +0.4% |
| 12 | 1206782ns | +0.5% | +0.3% | +0.2% | +2.0% |
| 13 | 1213132ns | -0.1% | -0.2% | -0.2% | -0.3% |
| 14 | 1210762ns | +0.8% | -0.1% | +0.2% | -0.0% |
| 15 | 1209834ns | +0.4% | -0.0% | +1.5% | -0.0% |
| 16 | 1210497ns | +0.1% | -0.2% | +0.0% | +0.0% |
| 17 | 1212215ns | +0.8% | +0.0% | -0.2% | -0.1% |
| 18 | 1211938ns | +0.2% | +0.1% | +0.1% | +0.1% |
| 19 | 1209843ns | +0.3% | +1.0% | -0.1% | +0.2% |
| 20 | 1212046ns | +0.1% | -0.1% | -0.3% | -0.1% |
| 21 | 1212361ns | +0.6% | -0.2% | +0.4% | +3.7% |
| 22 | 1207588ns | +0.6% | +0.3% | +0.6% | +0.7% |
| 23 | 1209624ns | +0.3% | +0.1% | +1.7% | +0.3% |
| 24 | 1208989ns | +0.5% | +0.0% | +0.5% | +0.5% |
| 25 | 1218651ns | -0.4% | -0.3% | +0.3% | -0.6% |
| 26 | 1212693ns | -0.2% | +1.8% | -0.2% | +0.3% |
| 27 | 1213928ns | -0.1% | -0.3% | -0.3% | -0.4% |
| 28 | 1213440ns | -0.1% | -0.2% | -0.2% | -0.2% |
| 29 | 1214168ns | -0.1% | -0.3% | -0.2% | +0.0% |
| 30 | 1209779ns | +1.8% | -0.1% | +0.7% | +0.4% |
| 31 | 1214653ns | +0.1% | -0.6% | -0.1% | -0.3% |
| 32 | 1213455ns | +2.4% | -0.3% | +0.1% | -0.3% |
| 33 | 1216164ns | -0.2% | -0.7% | -0.1% | -0.5% |
| 34 | 1213768ns | -0.0% | -0.4% | +0.1% | -0.5% |
| 35 | 1215135ns | -0.1% | -0.7% | -0.3% | -0.7% |
| 36 | 1222162ns | -0.9% | -1.1% | -0.7% | -0.9% |
| 37 | 1226836ns | -1.1% | -1.7% | -0.8% | -1.2% |
| 38 | 1217892ns | -0.5% | -0.9% | -0.1% | -0.5% |
| 39 | 1215439ns | -0.2% | -0.6% | +0.6% | -0.4% |
| 40 | 1214024ns | +0.2% | -0.6% | +0.1% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.552 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.043 | ok |
| wide-rung-ragged-overread | 0.371 | moderate+ |
| wide-rung-wordround | 0.405 | moderate+ |
| wide-rung-wordround-alias | 0.119 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 18/40, lost 18/40
- **wide-rung-ragged-overread**: won 18/40, lost 14/40
- **wide-rung-wordround**: won 15/40, lost 18/40
- **wide-rung-wordround-alias**: won 20/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 10.6ns | 1213058.5ns | 0.0% |  |
| wide-rung-ragged | 11.1ns | 1214477.8ns | 0.0% |  |
| wide-rung-ragged-overread | 15.4ns | 1212922.6ns | 0.0% |  |
| wide-rung-wordround | 12.5ns | 1217273.4ns | 0.0% |  |
| wide-rung-wordround-alias | 18.6ns | 1214702.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1208505.6-1218818.8 ns)
  1208505.6 |########
  1209021.2 |
  1209536.9 |################################
  1210052.6 |################
  1210568.2 |########################
  1211083.9 |########
  1211599.6 |################
  1212115.2 |########################
  1212630.9 |########################
  1213146.5 |########################
  1213662.2 |########################################
  1214177.9 |########
  1214693.5 |################
  1215209.2 |########
  1215724.8 |########
  1216240.5 |
  1216756.2 |
  1217271.8 |
  1217787.5 |################
  1218303.1 |########
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 1208408.1-1224337.8 ns)
  1208408.1 |#################
  1209204.6 |#####
  1210001.1 |#####
  1210797.6 |###########
  1211594.1 |######################
  1212390.5 |########################################
  1213187.0 |##################################
  1213983.5 |#####
  1214780.0 |#################
  1215576.5 |#################
  1216372.9 |
  1217169.4 |
  1217965.9 |
  1218762.4 |
  1219558.9 |###########
  1220355.4 |
  1221151.8 |#####
  1221948.3 |
  1222744.8 |
  1223541.3 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1207287.8-1224307.4 ns)
  1207287.8 |######################
  1208138.8 |######################
  1208989.8 |#################
  1209840.8 |########################################
  1210691.7 |#################
  1211542.7 |
  1212393.7 |############################
  1213244.7 |###########
  1214095.6 |
  1214946.6 |#####
  1215797.6 |###########
  1216648.6 |
  1217499.6 |
  1218350.5 |
  1219201.5 |
  1220052.5 |
  1220903.5 |
  1221754.5 |#####
  1222605.4 |
  1223456.4 |#####
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 1209481.9-1236014.0 ns)
  1209481.9 |########################################
  1210808.5 |#################################
  1212135.1 |#################################
  1213461.7 |########################################
  1214788.3 |##########################
  1216114.9 |##########################
  1217441.5 |######
  1218768.1 |
  1220094.7 |
  1221421.3 |######
  1222747.9 |######
  1224074.5 |
  1225401.1 |
  1226727.7 |######
  1228054.3 |
  1229380.9 |######
  1230707.5 |
  1232034.1 |
  1233360.8 |
  1234687.4 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1207656.1-1231445.7 ns)
  1207656.1 |########
  1208845.5 |###############################
  1210035.0 |########################################
  1211224.5 |######################
  1212414.0 |#############
  1213603.5 |#############
  1214793.0 |########
  1215982.4 |####
  1217171.9 |
  1218361.4 |
  1219550.9 |
  1220740.4 |
  1221929.9 |
  1223119.3 |
  1224308.8 |
  1225498.3 |
  1226687.8 |
  1227877.3 |
  1229066.8 |
  1230256.3 |####
  (3 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.55 (measurement drift or warm-up artifact)
