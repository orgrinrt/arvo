# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is an outlier: 4.0x slower than the field

satfold-nolaw (3.46 us) is 4.0x the fastest (856 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.78)

satfold-neon's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (856 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.0x the fastest

Fastest satfold-iterfold (856 ns) to slowest satfold-nolaw (3.46 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.2x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.80 us but its worst 20% at 7.60 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 856.4 ns median
- 7 variants significantly slower than baseline
- Spread: 4.04x (fastest 856.4 ns, slowest 3462.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112745822 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1108ns | 1132ns | 938ns | 1095ns | 1314ns | base |
| satfold-lanes16 | 2079ns | 2099ns | 1753ns | 2078ns | 2408ns | +87.68% |
| satfold-lanes16-constl | 2023ns | 2068ns | 1642ns | 2051ns | 2320ns | +82.58% |
| satfold-lanes4-idx | 4166ns | 2378ns | 1856ns | 3700ns | 7872ns | +276.02% |
| satfold-lanes64 | 1487ns | 1450ns | 1263ns | 1494ns | 1692ns | +34.24% |
| satfold-neon | 1441ns | 1479ns | 1292ns | 1448ns | 1572ns | +30.10% |
| satfold-neon8 | 1396ns | 1405ns | 1224ns | 1391ns | 1582ns | +26.01% |
| satfold-nolaw | 3471ns | 3696ns | 2062ns | 3651ns | 4341ns | +213.35% |
| satfold-seq | 1160ns | 1141ns | 896ns | 1128ns | 1520ns | +4.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 869ns | 744ns | 1031ns | base | 37.720 |
| satfold-lanes16 | 1852ns | 1568ns | 2131ns | +113.13% | 17.698 |
| satfold-lanes16-constl | 1800ns | 1469ns | 2052ns | +107.23% | 18.202 |
| satfold-lanes4-idx | 4036ns | 1803ns | 7596ns | +364.56% | 8.120 |
| satfold-lanes64 | 1235ns | 1059ns | 1377ns | +42.13% | 26.540 |
| satfold-neon | 1198ns | 1085ns | 1278ns | +37.94% | 27.346 |
| satfold-neon8 | 1150ns | 1015ns | 1290ns | +32.43% | 28.484 |
| satfold-nolaw | 3241ns | 1930ns | 4022ns | +273.10% | 10.110 |
| satfold-seq | 910ns | 709ns | 1183ns | +4.77% | 36.004 |

## Performance model

- Peak throughput: **46.211 Gops/s** (satfold-seq; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 38.262 | 82.8% |
| satfold-lanes16 | 17.871 | 38.7% |
| satfold-lanes16-constl | 17.936 | 38.8% |
| satfold-lanes4-idx | 14.206 | 30.7% |
| satfold-lanes64 | 27.016 | 58.5% |
| satfold-neon | 26.374 | 57.1% |
| satfold-neon8 | 28.184 | 61.0% |
| satfold-nolaw | 9.464 | 20.5% |
| satfold-seq | 36.362 | 78.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1108ns | 1108ns | base |
| satfold-lanes16 | 2079ns | 2079ns | +87.68% |
| satfold-lanes16-constl | 2023ns | 2023ns | +82.58% |
| satfold-lanes4-idx | 4166ns | 4166ns | +276.02% |
| satfold-lanes64 | 1487ns | 1487ns | +34.24% |
| satfold-neon | 1441ns | 1441ns | +30.10% |
| satfold-neon8 | 1396ns | 1396ns | +26.01% |
| satfold-nolaw | 3471ns | 3471ns | +213.35% |
| satfold-seq | 1160ns | 1160ns | +4.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 856ns | base | --- | [820, 902] | --- | --- | --- | --- |
| satfold-lanes16 | 1834ns | +944.5ns (+110.3%) | [+838, +1100]ns | [1723, 1989] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1827ns | +982.8ns (+114.8%) | [+761, +1167]ns | [1695, 1988] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2307ns | +1490.4ns (+174.0%) | [+1331, +5023]ns | [2207, 5927] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1213ns | +371.7ns (+43.4%) | [+280, +445]ns | [1212, 1322] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1242ns | +367.7ns (+42.9%) | [+331, +409]ns | [1161, 1245] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1163ns | +293.0ns (+34.2%) | [+244, +348]ns | [1099, 1183] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 3462ns | +2602.1ns (+303.8%) | [+2434, +2692]ns | [3374, 3515] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 901ns | no significant difference | [-5, +80]ns | [793, 992] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 754ns | +166.0% | +166.5% | +215.9% | +65.1% | +54.1% | +66.0% | +363.3% | -6.8% |
| 1 | 2 | 902ns | +121.1% | +65.5% | +163.1% | +49.4% | +38.0% | +33.3% | +234.7% | +10.1% |
| 1 | 3 | 988ns | +73.0% | +75.8% | +138.1% | +22.8% | +13.1% | +10.2% | +252.8% | +13.5% |
| 1 | 4 | 820ns | +97.2% | +142.7% | +121.1% | +32.8% | +52.0% | +27.9% | +322.4% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.170 | ok |
| satfold-lanes16 | 0.294 | moderate+ |
| satfold-lanes16-constl | -0.305 | moderate- |
| satfold-lanes4-idx | -0.051 | ok |
| satfold-lanes64 | 0.081 | ok |
| satfold-neon | -0.785 | HIGH- (thermal bounce) |
| satfold-neon8 | 0.307 | moderate+ |
| satfold-nolaw | -0.462 | moderate- |
| satfold-seq | -0.190 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 78.5ns | 868.7ns | 9.0% | HIGH |
| satfold-lanes16 | 76.1ns | 1851.5ns | 4.1% |  |
| satfold-lanes16-constl | 74.8ns | 1800.2ns | 4.2% |  |
| satfold-lanes4-idx | 43.3ns | 4035.6ns | 1.1% |  |
| satfold-lanes64 | 87.0ns | 1234.7ns | 7.1% | HIGH |
| satfold-neon | 82.9ns | 1198.3ns | 6.9% | HIGH |
| satfold-neon8 | 85.0ns | 1150.4ns | 7.4% | HIGH |
| satfold-nolaw | 81.3ns | 3241.2ns | 2.5% |  |
| satfold-seq | 82.8ns | 910.1ns | 9.1% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 744.3-1031.3 ns)
    744.3 |########################################
    758.7 |#####
    773.0 |#####
    787.4 |#####
    801.7 |#####
    816.1 |############################
    830.4 |#####
    844.7 |#####
    859.1 |#####
    873.4 |
    887.8 |############################
    902.1 |############################
    916.5 |#####
    930.8 |
    945.2 |
    959.5 |#####
    973.9 |#################
    988.2 |#####
   1002.6 |#####
   1016.9 |
  (2 below, 2 above range)

satfold-lanes16 (n=40, range 1567.6-2131.3 ns)
   1567.6 |#####
   1595.8 |
   1624.0 |#####
   1652.2 |############################
   1680.4 |#####
   1708.6 |###########
   1736.7 |
   1764.9 |
   1793.1 |#####
   1821.3 |############################
   1849.5 |
   1877.7 |###########
   1905.8 |
   1934.0 |
   1962.2 |#################
   1990.4 |########################################
   2018.6 |#####
   2046.8 |###########
   2075.0 |#####
   2103.1 |#####
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 1468.9-2051.9 ns)
   1468.9 |
   1498.0 |######
   1527.2 |####################
   1556.3 |######
   1585.5 |
   1614.6 |
   1643.8 |####################
   1672.9 |######
   1702.1 |######
   1731.2 |
   1760.4 |######
   1789.5 |##########################
   1818.7 |######
   1847.8 |######
   1877.0 |#############
   1906.1 |######
   1935.3 |
   1964.4 |########################################
   1993.6 |######
   2022.7 |########################################
  (5 below, 2 above range)

satfold-lanes4-idx (n=40, range 1802.8-7595.7 ns)
   1802.8 |#########################
   2092.5 |########################################
   2382.1 |###
   2671.8 |###
   2961.4 |
   3251.1 |
   3540.7 |
   3830.3 |
   4120.0 |
   4409.6 |
   4699.3 |
   4988.9 |
   5278.6 |###
   5568.2 |###
   5857.9 |###
   6147.5 |###
   6437.2 |#######
   6726.8 |#######
   7016.4 |#######
   7306.1 |#######
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 1059.2-1376.7 ns)
   1059.2 |###
   1075.1 |
   1090.9 |###############
   1106.8 |
   1122.7 |
   1138.6 |
   1154.4 |
   1170.3 |
   1186.2 |
   1202.1 |########################################
   1217.9 |
   1233.8 |
   1249.7 |
   1265.6 |######
   1281.4 |
   1297.3 |###
   1313.2 |
   1329.1 |
   1344.9 |##############################
   1360.8 |###
  (4 below, 3 above range)

satfold-neon (n=40, range 1085.3-1277.5 ns)
   1085.3 |
   1095.0 |
   1104.6 |#####
   1114.2 |
   1123.8 |########
   1133.4 |#####
   1143.0 |###########
   1152.6 |
   1162.2 |#####
   1171.8 |
   1181.4 |
   1191.1 |
   1200.7 |##
   1210.3 |##
   1219.9 |##
   1229.5 |
   1239.1 |########################################
   1248.7 |###########
   1258.3 |
   1267.9 |
  (3 below, 3 above range)

satfold-neon8 (n=40, range 1014.8-1289.6 ns)
   1014.8 |################################
   1028.5 |################
   1042.2 |
   1056.0 |########################
   1069.7 |########
   1083.5 |########
   1097.2 |########################
   1111.0 |########
   1124.7 |########
   1138.4 |########
   1152.2 |################
   1165.9 |########
   1179.7 |########################################
   1193.4 |########
   1207.2 |########
   1220.9 |########
   1234.6 |################
   1248.4 |
   1262.1 |########################################
   1275.9 |
  (2 below, 3 above range)

satfold-nolaw (n=40, range 1929.8-4021.8 ns)
   1929.8 |
   2034.4 |
   2139.0 |
   2243.6 |
   2348.2 |
   2452.8 |
   2557.4 |
   2662.0 |########
   2766.6 |
   2871.2 |############
   2975.8 |####
   3080.4 |########
   3185.0 |####
   3289.6 |####
   3394.2 |########################################
   3498.8 |########################
   3603.4 |####
   3708.0 |
   3812.6 |####################
   3917.2 |####
  (4 below, 3 above range)

satfold-seq (n=40, range 709.1-1182.8 ns)
    709.1 |
    732.8 |########################################
    756.5 |########################################
    780.2 |
    803.8 |########################################
    827.5 |##########
    851.2 |
    874.9 |
    898.6 |########################################
    922.3 |
    945.9 |##########
    969.6 |##############################
    993.3 |########################################
   1017.0 |
   1040.7 |##########
   1064.4 |##########
   1088.1 |
   1111.7 |##############################
   1135.4 |##########
   1159.1 |
  (6 below, 3 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=60.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-nolaw**: CV=25.2% (high variance, measurements may be unstable)
