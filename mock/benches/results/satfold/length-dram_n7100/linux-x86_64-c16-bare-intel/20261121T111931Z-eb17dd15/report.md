# The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 29% faster than the next best (satfold-lanes16-constl)

satfold-lanes4-idx (1.11 ms) leads satfold-lanes16-constl (1.43 ms) by 29%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 85% (significant)

satfold-lanes4-idx is -6.30 ms (85%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 6.7x slower than the field

satfold-seq (7.44 ms) is 6.7x the fastest (1.11 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {satfold-lanes4-idx, satfold-lanes16-constl, satfold-neon, satfold-lanes64, satfold-lanes16} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (306% apart)

The field splits into a fast tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-neon, satfold-lanes64, satfold-lanes16} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 306% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest satfold-lanes4-idx (1.11 ms) to slowest satfold-seq (7.44 ms): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-lanes4-idx** at 1112478.5 ns median (-85.0% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.68x (fastest 1112478.5 ns, slowest 7435903.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112552305 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 7424242ns | 7416500ns | 7396402ns | 7416819ns | 7474351ns | base |
| satfold-lanes16 | 1591261ns | 1662845ns | 1433695ns | 1605486ns | 1706151ns | -78.57% |
| satfold-lanes16-constl | 1503257ns | 1433077ns | 1412963ns | 1466469ns | 1703915ns | -79.75% |
| satfold-lanes4-idx | 1139735ns | 1113195ns | 1090089ns | 1112714ns | 1270443ns | -84.65% |
| satfold-lanes64 | 1669227ns | 1630592ns | 1614524ns | 1644846ns | 1797071ns | -77.52% |
| satfold-neon | 1672318ns | 1622888ns | 1599836ns | 1657439ns | 1789439ns | -77.47% |
| satfold-nolaw | 6780501ns | 6750877ns | 6666311ns | 6762477ns | 6948762ns | -8.67% |
| satfold-seq | 7477984ns | 7436736ns | 7393479ns | 7449141ns | 7649018ns | +0.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 7423363ns | 7395603ns | 7473419ns | base | 2.260 |
| satfold-lanes16 | 1590693ns | 1433074ns | 1705657ns | -78.57% | 10.547 |
| satfold-lanes16-constl | 1502604ns | 1412238ns | 1703319ns | -79.76% | 11.165 |
| satfold-lanes4-idx | 1139069ns | 1089454ns | 1269763ns | -84.66% | 14.729 |
| satfold-lanes64 | 1668611ns | 1613867ns | 1796413ns | -77.52% | 10.055 |
| satfold-neon | 1671562ns | 1599027ns | 1788655ns | -77.48% | 10.037 |
| satfold-nolaw | 6779481ns | 6665200ns | 6947729ns | -8.67% | 2.475 |
| satfold-seq | 7477237ns | 7392695ns | 7648312ns | +0.73% | 2.244 |

## Performance model

- Peak throughput: **15.400 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.262 | 14.7% |
| satfold-lanes16 | 10.093 | 65.5% |
| satfold-lanes16-constl | 11.712 | 76.1% |
| satfold-lanes4-idx | 15.081 | 97.9% |
| satfold-lanes64 | 10.293 | 66.8% |
| satfold-neon | 10.342 | 67.2% |
| satfold-nolaw | 2.486 | 16.1% |
| satfold-seq | 2.256 | 14.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 7424242ns | 7424242ns | base |
| satfold-lanes16 | 1591261ns | 1591261ns | -78.57% |
| satfold-lanes16-constl | 1503257ns | 1503257ns | -79.75% |
| satfold-lanes4-idx | 1139735ns | 1139735ns | -84.65% |
| satfold-lanes64 | 1669227ns | 1669227ns | -77.52% |
| satfold-neon | 1672318ns | 1672318ns | -77.47% |
| satfold-nolaw | 6780501ns | 6780501ns | -8.67% |
| satfold-seq | 7477984ns | 7477984ns | +0.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 7415568ns | base | --- | [7411634, 7420857] | --- | --- | --- | --- |
| satfold-lanes16 | 1662314ns | -5765244.9ns (-77.7%) | [-5927277, -5756717]ns | [1477980, 1673814] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1432466ns | -5970823.3ns (-80.5%) | [-5986789, -5960594]ns | [1429920, 1438783] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1112478ns | -6303943.7ns (-85.0%) | [-6306806, -6298765]ns | [1103956, 1118614] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1630012ns | -5774611.1ns (-77.9%) | [-5789595, -5763776]ns | [1622602, 1649445] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1622206ns | -5787822.9ns (-78.0%) | [-5797996, -5741395]ns | [1609476, 1710331] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 6749703ns | -668566.2ns (-9.0%) | [-688667, -648603]ns | [6723737, 6792235] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 7435904ns | +11324.5ns (+0.2%) | [+1810, +34017]ns | [7417480, 7444571] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7414502ns | -77.2% | -77.1% | -85.0% | -78.1% | -78.1% | -9.0% | +0.1% |
| 1 | 2 | 7397062ns | -80.5% | -80.6% | -85.2% | -78.0% | -78.3% | -8.9% | +0.3% |
| 1 | 3 | 7413606ns | -80.6% | -80.7% | -85.0% | -77.9% | -78.3% | -8.4% | -0.2% |
| 1 | 4 | 7426163ns | -77.6% | -80.9% | -83.9% | -75.9% | -76.0% | -9.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.066 | ok |
| satfold-lanes16 | -0.249 | moderate- |
| satfold-lanes16-constl | -0.059 | ok |
| satfold-lanes4-idx | -0.007 | ok |
| satfold-lanes64 | -0.012 | ok |
| satfold-neon | -0.092 | ok |
| satfold-nolaw | -0.321 | moderate- |
| satfold-seq | -0.496 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 327.4ns | 7423363.3ns | 0.0% |  |
| satfold-lanes16 | 91.1ns | 1590692.5ns | 0.0% |  |
| satfold-lanes16-constl | 162.7ns | 1502603.8ns | 0.0% |  |
| satfold-lanes4-idx | 154.7ns | 1139068.8ns | 0.0% |  |
| satfold-lanes64 | 98.8ns | 1668610.9ns | 0.0% |  |
| satfold-neon | 124.3ns | 1671561.9ns | 0.0% |  |
| satfold-nolaw | 226.9ns | 6779480.6ns | 0.0% |  |
| satfold-seq | 318.6ns | 7477236.6ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 7395603.3-7473418.9 ns)
  7395603.3 |####################
  7399494.1 |#############
  7403384.8 |
  7407275.6 |##########################
  7411166.4 |########################################
  7415057.2 |########################################
  7418948.0 |#############
  7422838.8 |########################################
  7426729.5 |#############
  7430620.3 |######
  7434511.1 |######
  7438401.9 |
  7442292.7 |
  7446183.4 |
  7450074.2 |
  7453965.0 |
  7457855.8 |
  7461746.6 |
  7465637.4 |
  7469528.1 |
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1433074.3-1705656.7 ns)
  1433074.3 |########################################
  1446703.5 |
  1460332.6 |
  1473961.7 |
  1487590.8 |
  1501219.9 |####
  1514849.0 |
  1528478.2 |
  1542107.3 |
  1555736.4 |
  1569365.5 |
  1582994.6 |
  1596623.7 |
  1610252.9 |
  1623882.0 |####
  1637511.1 |
  1651140.2 |####################################
  1664769.3 |####
  1678398.4 |########################################
  1692027.6 |########
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 1412237.9-1703319.2 ns)
  1412237.9 |####################
  1426792.0 |########################################
  1441346.0 |
  1455900.1 |
  1470454.2 |
  1485008.2 |##
  1499562.3 |##
  1514116.4 |
  1528670.4 |
  1543224.5 |
  1557778.6 |
  1572332.6 |
  1586886.7 |##
  1601440.8 |
  1615994.8 |
  1630548.9 |
  1645103.0 |
  1659657.0 |
  1674211.1 |
  1688765.1 |####################
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 1089453.8-1269762.9 ns)
  1089453.8 |########################################
  1098469.3 |#########################
  1107484.7 |###################################
  1116500.2 |##############################
  1125515.6 |#####
  1134531.1 |
  1143546.6 |#####
  1152562.0 |
  1161577.5 |
  1170592.9 |
  1179608.4 |#####
  1188623.8 |
  1197639.3 |
  1206654.7 |
  1215670.2 |##########
  1224685.7 |
  1233701.1 |
  1242716.6 |
  1251732.0 |
  1260747.5 |
  (4 below, 5 above range)

satfold-lanes64 (n=40, range 1613866.9-1796413.3 ns)
  1613866.9 |########################################
  1622994.2 |#####################
  1632121.5 |##########
  1641248.8 |##########
  1650376.2 |###
  1659503.5 |###
  1668630.8 |
  1677758.1 |###
  1686885.4 |
  1696012.8 |
  1705140.1 |###
  1714267.4 |
  1723394.7 |
  1732522.0 |
  1741649.4 |
  1750776.7 |
  1759904.0 |
  1769031.3 |
  1778158.6 |
  1787286.0 |#####################
  (4 below, 3 above range)

satfold-neon (n=40, range 1599027.0-1788655.3 ns)
  1599027.0 |########################################
  1608508.4 |##################
  1617989.8 |##########
  1627471.2 |###
  1636952.6 |#######
  1646434.0 |
  1655915.5 |
  1665396.9 |###
  1674878.3 |
  1684359.7 |
  1693841.1 |
  1703322.5 |
  1712803.9 |
  1722285.4 |
  1731766.8 |
  1741248.2 |###
  1750729.6 |#######
  1760211.0 |
  1769692.4 |
  1779173.8 |#############################
  (3 below, 3 above range)

satfold-nolaw (n=40, range 6665200.2-6947728.8 ns)
  6665200.2 |########################################
  6679326.6 |##############################
  6693453.0 |##########
  6707579.5 |####################
  6721705.9 |##############################
  6735832.3 |########################################
  6749958.8 |####################
  6764085.2 |
  6778211.6 |##############################
  6792338.1 |####################
  6806464.5 |
  6820590.9 |
  6834717.4 |####################
  6848843.8 |##########
  6862970.2 |##########
  6877096.7 |##############################
  6891223.1 |
  6905349.5 |
  6919476.0 |##########
  6933602.4 |
  (4 below, 4 above range)

satfold-seq (n=40, range 7392695.0-7648311.7 ns)
  7392695.0 |########################################
  7405475.9 |######################
  7418256.7 |######################
  7431037.5 |########################################
  7443818.4 |#####
  7456599.2 |
  7469380.0 |
  7482160.9 |
  7494941.7 |
  7507722.5 |
  7520503.4 |###########
  7533284.2 |#####
  7546065.0 |#################
  7558845.8 |###########
  7571626.7 |
  7584407.5 |
  7597188.3 |
  7609969.2 |
  7622750.0 |
  7635530.8 |
  (4 below, 5 above range)

```
