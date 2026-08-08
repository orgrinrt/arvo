# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.22 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 1.21 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.69)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.7% of the fastest

All 5 variants sit between 1.21 ms and 1.22 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1212280.8 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 1212280.8 ns, slowest 1220266.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1234927ns | 1221426ns | 1210985ns | 1225025ns | 1288574ns | base |
| wide-rung-ragged | 1220630ns | 1217499ns | 1208995ns | 1217399ns | 1241958ns | -1.16% |
| wide-rung-ragged-overread | 1216504ns | 1213423ns | 1210933ns | 1214349ns | 1228540ns | -1.49% |
| wide-rung-wordround | 1230690ns | 1214951ns | 1209046ns | 1215025ns | 1299328ns | -0.34% |
| wide-rung-wordround-alias | 1233868ns | 1216767ns | 1207415ns | 1218388ns | 1306758ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1233621ns | 1209799ns | 1287025ns | base | 1.487 |
| wide-rung-ragged | 1219422ns | 1207741ns | 1240629ns | -1.15% | 1.505 |
| wide-rung-ragged-overread | 1215351ns | 1209777ns | 1227356ns | -1.48% | 1.510 |
| wide-rung-wordround | 1229372ns | 1207842ns | 1297731ns | -0.34% | 1.493 |
| wide-rung-wordround-alias | 1232590ns | 1206259ns | 1305184ns | -0.08% | 1.489 |

## Performance model

- Peak throughput: **1.521 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.504 | 98.9% |
| wide-rung-ragged | 1.508 | 99.2% |
| wide-rung-ragged-overread | 1.514 | 99.5% |
| wide-rung-wordround | 1.512 | 99.4% |
| wide-rung-wordround-alias | 1.509 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1234927ns | 1234927ns | base |
| wide-rung-ragged | 1220630ns | 1220630ns | -1.16% |
| wide-rung-ragged-overread | 1216504ns | 1216504ns | -1.49% |
| wide-rung-wordround | 1230690ns | 1230690ns | -0.34% |
| wide-rung-wordround-alias | 1233868ns | 1233868ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1220266ns | base | --- | [1215712, 1226633] | --- | --- | --- | --- |
| wide-rung-ragged | 1216489ns | no significant difference | [-10748, +1816]ns | [1214446, 1217731] | no | 0.2682 | 0.2682 | 0 |
| wide-rung-ragged-overread | 1212281ns | no significant difference | [-13792, +702]ns | [1211271, 1215061] | no | 0.2051 | 0.1539 | 0 |
| wide-rung-wordround | 1213799ns | -5088.5ns (-0.4%) | [-8377, -2027]ns | [1211743, 1215273] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 1215679ns | no significant difference | [-7780, +1185]ns | [1211461, 1220437] | no | 0.2051 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1242514ns | -2.8% | -2.6% | +41.1% | +2.6% |
| 2 | 1260451ns | -2.0% | -3.8% | +0.6% | +10.5% |
| 3 | 1246730ns | -2.9% | -2.9% | -2.3% | +2.7% |
| 4 | 1252890ns | -3.6% | -3.5% | -2.5% | -2.1% |
| 5 | 1263696ns | -4.5% | -4.1% | -3.3% | +2.5% |
| 6 | 1357435ns | -10.9% | -10.8% | -10.5% | +3.0% |
| 7 | 1317087ns | -8.5% | -8.2% | -8.1% | -2.8% |
| 8 | 1265660ns | -4.4% | -4.4% | -4.4% | -2.8% |
| 9 | 1323998ns | -8.6% | -8.6% | -8.7% | -4.6% |
| 10 | 1248256ns | -3.4% | -2.9% | -0.5% | +0.7% |
| 11 | 1212497ns | +2.7% | -0.1% | +0.2% | -0.6% |
| 12 | 1216418ns | -0.1% | +0.2% | -0.1% | -1.0% |
| 13 | 1222177ns | -0.5% | -0.9% | +0.2% | -1.4% |
| 14 | 1254983ns | -3.0% | -3.6% | -3.3% | -4.0% |
| 15 | 1241185ns | -1.9% | -2.5% | -2.1% | -2.6% |
| 16 | 1221022ns | -0.3% | -1.0% | -0.1% | -1.1% |
| 17 | 1214998ns | -0.1% | -0.2% | -0.0% | -0.7% |
| 18 | 1219388ns | +0.5% | -0.7% | -0.4% | +1.4% |
| 19 | 1216427ns | +0.0% | +0.3% | -0.2% | -0.7% |
| 20 | 1224379ns | -0.0% | -0.2% | -0.7% | -1.4% |
| 21 | 1228887ns | -1.2% | -1.3% | -1.8% | -1.6% |
| 22 | 1215006ns | -0.0% | -0.3% | -0.5% | -0.3% |
| 23 | 1210222ns | +0.5% | +0.1% | -0.1% | +0.2% |
| 24 | 1209321ns | +0.7% | +0.3% | -0.0% | +1.6% |
| 25 | 1208445ns | +0.8% | +0.3% | -0.1% | +0.5% |
| 26 | 1207056ns | +0.5% | +0.3% | +1.8% | +0.3% |
| 27 | 1211184ns | +0.1% | +0.5% | -0.2% | -0.0% |
| 28 | 1211456ns | +0.3% | +0.1% | -0.2% | -0.1% |
| 29 | 1209499ns | +0.4% | +2.9% | -0.3% | +0.1% |
| 30 | 1211211ns | +0.3% | +0.6% | -0.3% | +0.1% |
| 31 | 1214652ns | +1.6% | +1.0% | +0.7% | +0.2% |
| 32 | 1214813ns | +3.6% | +0.3% | -0.3% | -0.2% |
| 33 | 1214275ns | +0.8% | +0.1% | -0.1% | +0.9% |
| 34 | 1220466ns | -0.2% | -0.4% | -0.7% | -0.1% |
| 35 | 1222575ns | -0.5% | -0.5% | -0.7% | -0.1% |
| 36 | 1218178ns | +0.2% | -0.3% | -0.4% | -0.2% |
| 37 | 1234554ns | -1.2% | -1.5% | -1.4% | -1.5% |
| 38 | 1220066ns | -0.1% | -0.3% | -0.4% | -0.3% |
| 39 | 1218842ns | +4.9% | +1.5% | -0.5% | -0.2% |
| 40 | 1221941ns | -0.0% | +0.7% | -0.8% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.689 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.135 | ok |
| wide-rung-ragged-overread | 0.324 | moderate+ |
| wide-rung-wordround | 0.097 | ok |
| wide-rung-wordround-alias | 0.550 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 19/40, lost 14/40
- **wide-rung-ragged-overread**: won 25/40, lost 12/40
- **wide-rung-wordround**: won 30/40, lost 6/40
- **wide-rung-wordround-alias**: won 22/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 31.0ns | 1233621.0ns | 0.0% |  |
| wide-rung-ragged | 16.3ns | 1219422.3ns | 0.0% |  |
| wide-rung-ragged-overread | 14.1ns | 1215351.3ns | 0.0% |  |
| wide-rung-wordround | 27.4ns | 1229372.2ns | 0.0% |  |
| wide-rung-wordround-alias | 29.4ns | 1232590.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1209799.2-1287025.0 ns)
  1209799.2 |############################
  1213660.5 |########################################
  1217521.8 |##################################
  1221383.1 |######################
  1225244.4 |#####
  1229105.7 |
  1232966.9 |#####
  1236828.2 |
  1240689.5 |###########
  1244550.8 |###########
  1248412.1 |
  1252273.4 |###########
  1256134.7 |
  1259996.0 |###########
  1263857.3 |#####
  1267718.5 |
  1271579.8 |
  1275441.1 |
  1279302.4 |
  1283163.7 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 1207741.4-1240628.5 ns)
  1207741.4 |########
  1209385.7 |########################
  1211030.1 |########
  1212674.5 |################################
  1214318.8 |########################################
  1215963.2 |########################################
  1217607.5 |########################################
  1219251.9 |########
  1220896.3 |################
  1222540.6 |########
  1224185.0 |########
  1225829.3 |########
  1227473.7 |
  1229118.0 |
  1230762.4 |
  1232406.8 |########
  1234051.1 |########
  1235695.5 |
  1237339.8 |
  1238984.2 |
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1209777.4-1227355.7 ns)
  1209777.4 |######################
  1210656.3 |########################################
  1211535.2 |#################
  1212414.1 |#############
  1213293.1 |
  1214172.0 |####
  1215050.9 |########
  1215929.8 |#############
  1216808.7 |
  1217687.6 |########
  1218566.6 |########
  1219445.5 |####
  1220324.4 |
  1221203.3 |####
  1222082.2 |
  1222961.1 |
  1223840.1 |
  1224719.0 |
  1225597.9 |
  1226476.8 |####
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 1207842.1-1297731.0 ns)
  1207842.1 |####################################
  1212336.5 |########################################
  1216831.0 |############
  1221325.4 |#########
  1225819.9 |###
  1230314.3 |
  1234808.8 |
  1239303.2 |###
  1243797.7 |
  1248292.1 |
  1252786.6 |
  1257281.0 |
  1261775.5 |
  1266269.9 |###
  1270764.4 |
  1275258.8 |
  1279753.2 |
  1284247.7 |
  1288742.1 |
  1293236.6 |
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 1206259.0-1305183.8 ns)
  1206259.0 |########################################
  1211205.2 |####################################
  1216151.5 |############
  1221097.7 |########
  1226044.0 |############
  1230990.2 |
  1235936.4 |####
  1240882.7 |
  1245828.9 |
  1250775.2 |
  1255721.4 |####
  1260667.6 |####
  1265613.9 |
  1270560.1 |####
  1275506.4 |########
  1280452.6 |
  1285398.9 |
  1290345.1 |
  1295291.3 |####
  1300237.6 |
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.55 (measurement drift or warm-up artifact)
