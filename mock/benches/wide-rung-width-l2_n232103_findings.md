# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-wordround (1.25 ms) and wide-rung-wordround-alias (1.25 ms) differ by 0.02%, inside the noise, even though the wider field spreads 7.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: wide-rung-wordround** at 1253558.4 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.07x (fastest 1253558.4 ns, slowest 1345129.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1257628ns | 1256511ns | 1249771ns | 1255840ns | 1270848ns | base |
| wide-rung-ragged | 1347564ns | 1346308ns | 1344634ns | 1346318ns | 1354232ns | +7.15% |
| wide-rung-ragged-overread | 1261426ns | 1261134ns | 1258614ns | 1261395ns | 1264331ns | +0.30% |
| wide-rung-wordround | 1255713ns | 1254905ns | 1247167ns | 1254587ns | 1267635ns | -0.15% |
| wide-rung-wordround-alias | 1257084ns | 1254928ns | 1249137ns | 1254804ns | 1271869ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1256330ns | 1248464ns | 1269658ns | base | 1.461 |
| wide-rung-ragged | 1346388ns | 1343525ns | 1353066ns | +7.17% | 1.363 |
| wide-rung-ragged-overread | 1260189ns | 1257417ns | 1263171ns | +0.31% | 1.456 |
| wide-rung-wordround | 1254475ns | 1245946ns | 1266449ns | -0.15% | 1.463 |
| wide-rung-wordround-alias | 1255870ns | 1247925ns | 1270591ns | -0.04% | 1.461 |

## Performance model

- Peak throughput: **1.473 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.462 | 99.3% |
| wide-rung-ragged | 1.364 | 92.6% |
| wide-rung-ragged-overread | 1.457 | 98.9% |
| wide-rung-wordround | 1.464 | 99.4% |
| wide-rung-wordround-alias | 1.464 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1257628ns | 1257628ns | base |
| wide-rung-ragged | 1347564ns | 1347564ns | +7.15% |
| wide-rung-ragged-overread | 1261426ns | 1261426ns | +0.30% |
| wide-rung-wordround | 1255713ns | 1255713ns | -0.15% |
| wide-rung-wordround-alias | 1257084ns | 1257084ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1255216ns | base | --- | [1251540, 1257434] | --- | --- | --- | --- |
| wide-rung-ragged | 1345130ns | +90170.2ns (+7.2%) | [+88399, +94029]ns | [1344381, 1345734] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1259853ns | +5198.1ns (+0.4%) | [+3304, +10171]ns | [1259032, 1261322] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 1253558ns | -3500.0ns (-0.3%) | [-5070, -517]ns | [1251397, 1254942] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 1253812ns | no significant difference | [-4255, +1253]ns | [1250984, 1255491] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1255672ns | +7.1% | +0.4% | -0.5% | -0.6% |
| 2 | 1278795ns | +5.2% | -1.9% | -2.1% | -2.5% |
| 3 | 1254072ns | +7.2% | +0.4% | +0.0% | +0.2% |
| 4 | 1256671ns | +7.0% | +0.1% | -0.4% | -0.3% |
| 5 | 1257925ns | +6.9% | +0.1% | -0.4% | -0.6% |
| 6 | 1252524ns | +7.2% | +0.8% | +1.5% | +2.5% |
| 7 | 1254907ns | +7.1% | +0.4% | -0.4% | -0.2% |
| 8 | 1255141ns | +7.1% | +0.3% | -0.3% | -0.3% |
| 9 | 1257780ns | +6.8% | +0.1% | -0.4% | -0.7% |
| 10 | 1255358ns | +7.2% | +0.6% | -0.4% | -0.5% |
| 11 | 1248669ns | +8.1% | +0.8% | -0.2% | +0.0% |
| 12 | 1270023ns | +5.9% | -0.6% | -1.8% | -1.8% |
| 13 | 1257317ns | +6.9% | +0.2% | -1.0% | -0.8% |
| 14 | 1249185ns | +7.7% | +1.0% | -0.2% | -0.0% |
| 15 | 1249379ns | +7.8% | +0.8% | -0.3% | +0.1% |
| 16 | 1251063ns | +7.4% | +0.9% | -0.3% | -0.3% |
| 17 | 1247172ns | +7.9% | +1.0% | -0.1% | +1.0% |
| 18 | 1252272ns | +7.6% | +0.7% | -0.6% | -0.3% |
| 19 | 1250082ns | +7.6% | +0.9% | -0.2% | +0.1% |
| 20 | 1248448ns | +10.2% | +1.0% | +0.1% | +0.1% |
| 21 | 1250978ns | +7.5% | +0.9% | +0.3% | +0.6% |
| 22 | 1278466ns | +5.1% | -1.4% | -1.6% | -1.9% |
| 23 | 1250132ns | +7.5% | +1.0% | +0.5% | +0.3% |
| 24 | 1251410ns | +7.6% | +0.5% | +0.6% | +0.3% |
| 25 | 1249005ns | +7.7% | +0.8% | +0.9% | +0.7% |
| 26 | 1248275ns | +7.9% | +0.8% | +1.8% | +0.8% |
| 27 | 1247580ns | +7.8% | +0.9% | +1.2% | +5.3% |
| 28 | 1280844ns | +5.1% | -1.7% | -1.6% | -1.7% |
| 29 | 1249566ns | +7.6% | +1.0% | +0.6% | +0.5% |
| 30 | 1251671ns | +7.5% | +0.8% | +0.4% | +0.2% |
| 31 | 1258050ns | +6.7% | +0.1% | +2.4% | -0.1% |
| 32 | 1258213ns | +7.1% | +0.6% | +0.1% | +0.7% |
| 33 | 1259028ns | +7.7% | +0.3% | -0.4% | -0.4% |
| 34 | 1255572ns | +7.3% | +0.3% | -0.1% | -0.1% |
| 35 | 1257550ns | +6.8% | +0.3% | -0.3% | -0.4% |
| 36 | 1263412ns | +7.0% | -0.5% | -0.5% | -0.6% |
| 37 | 1262115ns | +6.5% | -0.3% | -0.6% | -0.3% |
| 38 | 1255292ns | +7.1% | +0.2% | -0.0% | +0.5% |
| 39 | 1260712ns | +6.6% | -0.2% | -0.5% | +0.1% |
| 40 | 1262892ns | +7.0% | +0.0% | -0.8% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.079 | ok |
| wide-rung-ragged | -0.019 | ok |
| wide-rung-ragged-overread | 0.067 | ok |
| wide-rung-wordround | 0.432 | moderate+ |
| wide-rung-wordround-alias | 0.103 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 7/40, lost 29/40
- **wide-rung-wordround**: won 25/40, lost 11/40
- **wide-rung-wordround-alias**: won 20/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 14.0ns | 1256330.5ns | 0.0% |  |
| wide-rung-ragged | 12.6ns | 1346387.9ns | 0.0% |  |
| wide-rung-ragged-overread | 10.8ns | 1260189.1ns | 0.0% |  |
| wide-rung-wordround | 11.8ns | 1254475.2ns | 0.0% |  |
| wide-rung-wordround-alias | 13.3ns | 1255869.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1248464.2-1269657.5 ns)
  1248464.2 |##########################
  1249523.8 |####################
  1250583.5 |####################
  1251643.2 |####################
  1252702.8 |
  1253762.5 |######
  1254822.2 |########################################
  1255881.8 |######
  1256941.5 |##########################
  1258001.2 |####################
  1259060.8 |
  1260120.5 |######
  1261180.2 |######
  1262239.8 |######
  1263299.5 |######
  1264359.2 |
  1265418.8 |
  1266478.5 |
  1267538.2 |
  1268597.8 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1343524.8-1353065.7 ns)
  1343524.8 |####
  1344001.8 |########################################
  1344478.9 |################
  1344955.9 |########################
  1345433.0 |################
  1345910.0 |####
  1346387.1 |####
  1346864.1 |################
  1347341.2 |
  1347818.2 |
  1348295.2 |
  1348772.3 |
  1349249.3 |####
  1349726.4 |
  1350203.4 |
  1350680.5 |####
  1351157.5 |####
  1351634.6 |
  1352111.6 |
  1352588.7 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 1257417.0-1263171.3 ns)
  1257417.0 |########
  1257704.7 |
  1257992.4 |################
  1258280.1 |########################
  1258567.9 |########################################
  1258855.6 |
  1259143.3 |################
  1259431.0 |################
  1259718.7 |################################
  1260006.4 |########
  1260294.2 |
  1260581.9 |
  1260869.6 |################
  1261157.3 |################
  1261445.0 |########################
  1261732.7 |################
  1262020.5 |################
  1262308.2 |
  1262595.9 |################
  1262883.6 |########
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 1245945.6-1266449.4 ns)
  1245945.6 |##############################
  1246970.8 |####################
  1247996.0 |
  1249021.2 |####################
  1250046.4 |####################
  1251071.6 |####################
  1252096.8 |##############################
  1253122.0 |########################################
  1254147.1 |########################################
  1255172.3 |####################
  1256197.5 |####################
  1257222.7 |####################
  1258247.9 |##########
  1259273.1 |##############################
  1260298.3 |
  1261323.4 |
  1262348.6 |##########
  1263373.8 |
  1264399.0 |
  1265424.2 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1247925.1-1270591.3 ns)
  1247925.1 |####################
  1249058.4 |##########################
  1250191.7 |####################
  1251325.0 |
  1252458.3 |####################
  1253591.6 |########################################
  1254724.9 |####################
  1255858.3 |#############
  1256991.6 |#############
  1258124.9 |##########################
  1259258.2 |
  1260391.5 |######
  1261524.8 |######
  1262658.1 |
  1263791.4 |
  1264924.7 |
  1266058.0 |
  1267191.3 |######
  1268324.6 |
  1269458.0 |
  (5 below, 2 above range)

```
