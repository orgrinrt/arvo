# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd shows alternating (throttle bounce) (autocorr -0.60)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 1152740.5 ns median (-19.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.43x (fastest 1152740.5 ns, slowest 1649816.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761890 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1481111ns | 1424953ns | 1418206ns | 1451049ns | 1634205ns | base |
| bitpack-contend-d16-padal | 1548213ns | 1428909ns | 1417354ns | 1480921ns | 1880950ns | +4.53% |
| bitpack-contend-d32 | 1692930ns | 1650928ns | 1634295ns | 1650659ns | 1878376ns | +14.30% |
| bitpack-contend-d32-padal | 1652308ns | 1643160ns | 1631510ns | 1645190ns | 1694462ns | +11.56% |
| bitpack-contend-packed-simd | 1199242ns | 1189841ns | 1146117ns | 1193852ns | 1268536ns | -19.03% |
| bitpack-contend-pipe4 | 1163052ns | 1153277ns | 1144764ns | 1156345ns | 1201463ns | -21.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 1480284ns | 1417382ns | 1633563ns | base | 5.667 |
| bitpack-contend-d16-padal | 1547513ns | 1416579ns | 1880210ns | +4.54% | 5.421 |
| bitpack-contend-d32 | 1691792ns | 1633107ns | 1877292ns | +14.29% | 4.958 |
| bitpack-contend-d32-padal | 1651111ns | 1630253ns | 1693299ns | +11.54% | 5.081 |
| bitpack-contend-packed-simd | 1198821ns | 1145671ns | 1268167ns | -19.01% | 6.997 |
| bitpack-contend-pipe4 | 1162509ns | 1144261ns | 1200826ns | -21.47% | 7.216 |

## Performance model

- Peak throughput: **7.331 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.891 | 80.4% |
| bitpack-contend-d16-padal | 5.873 | 80.1% |
| bitpack-contend-d32 | 5.085 | 69.4% |
| bitpack-contend-d32-padal | 5.108 | 69.7% |
| bitpack-contend-packed-simd | 7.053 | 96.2% |
| bitpack-contend-pipe4 | 7.277 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 1481111ns | 1481111ns | base |
| bitpack-contend-d16-padal | 1548213ns | 1548213ns | +4.53% |
| bitpack-contend-d32 | 1692930ns | 1692930ns | +14.30% |
| bitpack-contend-d32-padal | 1652308ns | 1652308ns | +11.56% |
| bitpack-contend-packed-simd | 1199242ns | 1199242ns | -19.03% |
| bitpack-contend-pipe4 | 1163052ns | 1163052ns | -21.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1424073ns | base | --- | [1422782, 1426930] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 1428368ns | no significant difference | [-6504, +133623]ns | [1421822, 1603423] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-d32 | 1649816ns | +223006.6ns (+15.7%) | [+214255, +228023]ns | [1641884, 1653399] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 1642162ns | +216315.8ns (+15.2%) | [+208133, +221551]ns | [1639656, 1649402] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 1189373ns | -244173.9ns (-17.1%) | [-260874, -220493]ns | [1167084, 1211410] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 1152741ns | -276491.7ns (-19.4%) | [-279567, -273936]ns | [1148878, 1155023] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1421802ns | +0.7% | +16.2% | +15.1% | -12.8% | -19.5% |
| 1 | 2 | 1631146ns | -12.8% | +4.6% | +1.2% | -29.7% | -29.3% |
| 1 | 3 | 1424706ns | +32.7% | +15.9% | +15.3% | -15.1% | -19.4% |
| 1 | 4 | 1419762ns | +0.0% | +15.1% | +16.2% | -15.7% | -16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.406 | moderate- |
| bitpack-contend-d16-padal | -0.434 | moderate- |
| bitpack-contend-d32 | -0.209 | moderate- |
| bitpack-contend-d32-padal | -0.570 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.605 | HIGH- (thermal bounce) |
| bitpack-contend-pipe4 | -0.138 | ok |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 1/4, lost 2/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 147.6ns | 1480283.6ns | 0.0% |  |
| bitpack-contend-d16-padal | 68.1ns | 1547512.8ns | 0.0% |  |
| bitpack-contend-d32 | 242.0ns | 1691791.6ns | 0.0% |  |
| bitpack-contend-d32-padal | 42.1ns | 1651111.5ns | 0.0% |  |
| bitpack-contend-packed-simd | 64.3ns | 1198820.9ns | 0.0% |  |
| bitpack-contend-pipe4 | 55.0ns | 1162509.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1417382.4-1633563.0 ns)
  1417382.4 |########################################
  1428191.4 |#
  1439000.4 |
  1449809.5 |
  1460618.5 |
  1471427.5 |
  1482236.6 |
  1493045.6 |
  1503854.6 |#
  1514663.6 |
  1525472.7 |
  1536281.7 |
  1547090.7 |#
  1557899.8 |
  1568708.8 |
  1579517.8 |
  1590326.9 |
  1601135.9 |
  1611944.9 |
  1622754.0 |###########
  (3 below, 3 above range)

bitpack-contend-d16-padal (n=40, range 1416578.7-1880210.0 ns)
  1416578.7 |########################################
  1439760.3 |##
  1462941.9 |
  1486123.4 |
  1509305.0 |
  1532486.6 |
  1555668.1 |
  1578849.7 |##
  1602031.3 |##########
  1625212.8 |##
  1648394.4 |
  1671576.0 |
  1694757.5 |
  1717939.1 |
  1741120.7 |
  1764302.2 |
  1787483.8 |##
  1810665.4 |
  1833846.9 |
  1857028.5 |
  (4 below, 7 above range)

bitpack-contend-d32 (n=40, range 1633107.2-1877291.7 ns)
  1633107.2 |#################################
  1645316.4 |########################################
  1657525.7 |################
  1669734.9 |###
  1681944.1 |
  1694153.3 |
  1706362.6 |
  1718571.8 |
  1730781.0 |
  1742990.2 |
  1755199.5 |###
  1767408.7 |
  1779617.9 |
  1791827.1 |
  1804036.4 |
  1816245.6 |
  1828454.8 |
  1840664.0 |
  1852873.3 |
  1865082.5 |
  (5 below, 6 above range)

bitpack-contend-d32-padal (n=40, range 1630253.3-1693298.6 ns)
  1630253.3 |########
  1633405.5 |#############
  1636557.8 |##########################
  1639710.1 |###############################
  1642862.3 |########
  1646014.6 |########
  1649166.9 |########################################
  1652319.1 |########
  1655471.4 |####
  1658623.7 |####
  1661775.9 |
  1664928.2 |####
  1668080.5 |
  1671232.7 |
  1674385.0 |
  1677537.3 |
  1680689.5 |
  1683841.8 |
  1686994.1 |
  1690146.3 |
  (3 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 1145671.0-1268166.6 ns)
  1145671.0 |######################
  1151795.8 |####
  1157920.5 |########
  1164045.3 |#############
  1170170.1 |####
  1176294.9 |####
  1182419.7 |########
  1188544.4 |#############
  1194669.2 |#############
  1200794.0 |
  1206918.8 |
  1213043.5 |####
  1219168.3 |####
  1225293.1 |
  1231417.9 |####
  1237542.7 |
  1243667.4 |
  1249792.2 |
  1255917.0 |
  1262041.8 |########################################
  (5 below, 2 above range)

bitpack-contend-pipe4 (n=40, range 1144260.7-1200826.2 ns)
  1144260.7 |####################
  1147089.0 |########################################
  1149917.2 |###############
  1152745.5 |###################################
  1155573.8 |
  1158402.1 |
  1161230.3 |
  1164058.6 |#####
  1166886.9 |
  1169715.2 |
  1172543.5 |##########
  1175371.7 |#####
  1178200.0 |#####
  1181028.3 |
  1183856.6 |#####
  1186684.9 |
  1189513.1 |#####
  1192341.4 |##########
  1195169.7 |##########
  1197998.0 |
  (5 below, 2 above range)

```
