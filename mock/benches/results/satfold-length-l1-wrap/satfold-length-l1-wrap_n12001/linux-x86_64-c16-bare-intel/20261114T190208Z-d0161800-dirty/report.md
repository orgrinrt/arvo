# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is an outlier: 4.6x slower than the field

satfold-nolaw (3.80 us) is 4.6x the fastest (835 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-iterfold shows alternating (throttle bounce) (autocorr -0.62)

satfold-iterfold's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.6x the fastest

Fastest satfold-seq (835 ns) to slowest satfold-nolaw (3.80 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.6x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.11 us but its worst 20% at 7.64 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-seq** at 834.6 ns median (-8.2% vs baseline)
- 6 variants significantly slower than baseline
- Spread: 4.56x (fastest 834.6 ns, slowest 3803.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1228ns | 1168ns | 998ns | 1169ns | 1636ns | base |
| satfold-lanes16 | 2370ns | 2422ns | 1940ns | 2389ns | 2744ns | +93.01% |
| satfold-lanes16-constl | 2183ns | 2227ns | 1762ns | 2205ns | 2541ns | +77.81% |
| satfold-lanes4-idx | 4055ns | 2287ns | 2170ns | 3393ns | 7925ns | +230.19% |
| satfold-lanes64 | 1434ns | 1377ns | 978ns | 1404ns | 1980ns | +16.76% |
| satfold-neon | 1352ns | 1328ns | 1116ns | 1331ns | 1650ns | +10.08% |
| satfold-neon8 | 1418ns | 1354ns | 1331ns | 1375ns | 1634ns | +15.50% |
| satfold-nolaw | 3791ns | 4065ns | 2169ns | 4011ns | 4752ns | +208.72% |
| satfold-seq | 1234ns | 1085ns | 1015ns | 1141ns | 1733ns | +0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 973ns | 795ns | 1303ns | base | 33.665 |
| satfold-lanes16 | 2122ns | 1750ns | 2419ns | +118.01% | 15.442 |
| satfold-lanes16-constl | 1934ns | 1573ns | 2233ns | +98.74% | 16.940 |
| satfold-lanes4-idx | 3923ns | 2108ns | 7644ns | +303.07% | 8.352 |
| satfold-lanes64 | 1172ns | 806ns | 1610ns | +20.37% | 27.968 |
| satfold-neon | 1105ns | 918ns | 1349ns | +13.49% | 29.665 |
| satfold-neon8 | 1150ns | 1095ns | 1273ns | +18.10% | 28.506 |
| satfold-nolaw | 3559ns | 2032ns | 4465ns | +265.68% | 9.206 |
| satfold-seq | 952ns | 802ns | 1295ns | -2.15% | 34.404 |

## Performance model

- Peak throughput: **41.221 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 36.037 | 87.4% |
| satfold-lanes16 | 14.997 | 36.4% |
| satfold-lanes16-constl | 16.468 | 39.9% |
| satfold-lanes4-idx | 14.755 | 35.8% |
| satfold-lanes64 | 28.391 | 68.9% |
| satfold-neon | 29.995 | 72.8% |
| satfold-neon8 | 29.509 | 71.6% |
| satfold-nolaw | 8.616 | 20.9% |
| satfold-seq | 39.260 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1228ns | 1228ns | base |
| satfold-lanes16 | 2370ns | 2370ns | +93.01% |
| satfold-lanes16-constl | 2183ns | 2183ns | +77.81% |
| satfold-lanes4-idx | 4055ns | 4055ns | +230.19% |
| satfold-lanes64 | 1434ns | 1434ns | +16.76% |
| satfold-neon | 1352ns | 1352ns | +10.08% |
| satfold-neon8 | 1418ns | 1418ns | +15.50% |
| satfold-nolaw | 3791ns | 3791ns | +208.72% |
| satfold-seq | 1234ns | 1234ns | +0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 909ns | base | --- | [871, 961] | --- | --- | --- | --- |
| satfold-lanes16 | 2185ns | +1161.8ns (+127.8%) | [+1113, +1228]ns | [2099, 2194] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1990ns | +1075.9ns (+118.3%) | [+955, +1182]ns | [1988, 1992] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2221ns | +1387.8ns (+152.6%) | [+1284, +4402]ns | [2209, 5217] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1154ns | no significant difference | [-89, +345]ns | [1111, 1225] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon | 1092ns | +173.9ns (+19.1%) | [+146, +224]ns | [1085, 1112] | YES | 0.0030 | 0.0022 | 0 |
| satfold-neon8 | 1110ns | +231.9ns (+25.5%) | [+177, +287]ns | [1101, 1141] | YES | 0.0003 | 0.0002 | 0 |
| satfold-nolaw | 3803ns | +2832.5ns (+311.5%) | [+2529, +3007]ns | [3445, 4014] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 835ns | no significant difference | [-135, +41]ns | [829, 916] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 943ns | +131.8% | +111.0% | +134.8% | -13.7% | +25.9% | +18.3% | +311.9% | -12.0% |
| 1 | 2 | 835ns | +161.6% | +140.3% | +165.3% | +78.1% | +19.9% | +34.3% | +387.1% | +52.6% |
| 1 | 3 | 1197ns | +85.5% | +34.7% | +85.5% | -6.8% | -9.2% | -6.7% | +212.1% | -32.0% |
| 1 | 4 | 861ns | +111.0% | +139.2% | +517.1% | +42.2% | +28.7% | +27.7% | +294.2% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.620 | HIGH- (thermal bounce) |
| satfold-lanes16 | -0.157 | ok |
| satfold-lanes16-constl | -0.508 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.082 | ok |
| satfold-lanes64 | -0.560 | HIGH- (thermal bounce) |
| satfold-neon | -0.450 | moderate- |
| satfold-neon8 | 0.004 | ok |
| satfold-nolaw | 0.150 | ok |
| satfold-seq | -0.537 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 86.1ns | 973.3ns | 8.8% | HIGH |
| satfold-lanes16 | 81.9ns | 2122.0ns | 3.9% |  |
| satfold-lanes16-constl | 82.0ns | 1934.4ns | 4.2% |  |
| satfold-lanes4-idx | 42.0ns | 3923.3ns | 1.1% |  |
| satfold-lanes64 | 89.8ns | 1171.6ns | 7.7% | HIGH |
| satfold-neon | 85.3ns | 1104.6ns | 7.7% | HIGH |
| satfold-neon8 | 97.9ns | 1149.5ns | 8.5% | HIGH |
| satfold-nolaw | 79.2ns | 3559.3ns | 2.2% |  |
| satfold-seq | 89.9ns | 952.4ns | 9.4% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 794.9-1303.1 ns)
    794.9 |######
    820.3 |#################################
    845.8 |####################
    871.2 |########################################
    896.6 |
    922.0 |#################################
    947.4 |#############
    972.8 |
    998.2 |#############
   1023.6 |#############
   1049.0 |
   1074.4 |######
   1099.8 |
   1125.2 |
   1150.6 |######
   1176.0 |#############
   1201.4 |
   1226.9 |
   1252.3 |######
   1277.7 |
  (5 below, 4 above range)

satfold-lanes16 (n=40, range 1750.1-2418.6 ns)
   1750.1 |
   1783.6 |
   1817.0 |#################
   1850.4 |
   1883.8 |
   1917.3 |
   1950.7 |####
   1984.1 |#############
   2017.5 |
   2051.0 |########
   2084.4 |####
   2117.8 |
   2151.2 |######################
   2184.7 |########################################
   2218.1 |#################
   2251.5 |
   2284.9 |
   2318.4 |
   2351.8 |
   2385.2 |
  (4 below, 7 above range)

satfold-lanes16-constl (n=40, range 1572.6-2232.6 ns)
   1572.6 |##
   1605.6 |##
   1638.6 |##
   1671.6 |##
   1704.6 |##
   1737.6 |
   1770.6 |##
   1803.6 |########
   1836.6 |
   1869.6 |
   1902.6 |
   1935.6 |
   1968.6 |########################################
   2001.6 |###########
   2034.6 |##
   2067.6 |
   2100.6 |##
   2133.6 |##
   2166.6 |
   2199.6 |########
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 2107.7-7643.9 ns)
   2107.7 |########################################
   2384.5 |
   2661.3 |
   2938.1 |#
   3214.9 |
   3491.7 |
   3768.6 |#
   4045.4 |
   4322.2 |
   4599.0 |
   4875.8 |#
   5152.6 |#####
   5429.4 |#
   5706.2 |#
   5983.0 |#
   6259.8 |
   6536.6 |#
   6813.5 |
   7090.3 |###
   7367.1 |
  (2 below, 5 above range)

satfold-lanes64 (n=40, range 805.7-1609.9 ns)
    805.7 |#####
    845.9 |###############
    886.1 |
    926.3 |
    966.5 |
   1006.8 |##########
   1047.0 |
   1087.2 |########################################
   1127.4 |###############
   1167.6 |#####
   1207.8 |########################################
   1248.0 |#####
   1288.2 |
   1328.4 |#####
   1368.6 |#####
   1408.8 |#####
   1449.0 |
   1489.2 |
   1529.4 |#####
   1569.7 |##########
  (5 below, 2 above range)

satfold-neon (n=40, range 917.8-1348.6 ns)
    917.8 |
    939.3 |
    960.8 |#####
    982.4 |##################################
   1003.9 |#####
   1025.5 |
   1047.0 |#####
   1068.6 |##################################
   1090.1 |########################################
   1111.6 |######################
   1133.2 |#####
   1154.7 |###########
   1176.3 |
   1197.8 |###########
   1219.3 |
   1240.9 |
   1262.4 |
   1284.0 |
   1305.5 |
   1327.1 |#####
  (4 below, 4 above range)

satfold-neon8 (n=40, range 1094.7-1272.5 ns)
   1094.7 |########################################
   1103.6 |########
   1112.5 |########
   1121.4 |##
   1130.3 |#####
   1139.2 |##
   1148.1 |
   1156.9 |##
   1165.8 |##
   1174.7 |##
   1183.6 |
   1192.5 |
   1201.4 |##
   1210.3 |#####
   1219.2 |##
   1228.1 |##
   1237.0 |#####
   1245.9 |
   1254.8 |##
   1263.7 |
  (3 below, 2 above range)

satfold-nolaw (n=40, range 2032.5-4465.3 ns)
   2032.5 |
   2154.1 |#####
   2275.8 |
   2397.4 |#####
   2519.1 |
   2640.7 |
   2762.3 |###########
   2884.0 |
   3005.6 |#####
   3127.3 |###########
   3248.9 |
   3370.5 |############################
   3492.2 |###########
   3613.8 |###########
   3735.5 |#################
   3857.1 |######################
   3978.7 |#####
   4100.4 |#####
   4222.0 |########################################
   4343.6 |#####
  (3 below, 4 above range)

satfold-seq (n=40, range 802.0-1294.9 ns)
    802.0 |########################################
    826.6 |#############################
    851.3 |###
    875.9 |###
    900.6 |#####################
    925.2 |
    949.9 |
    974.5 |###
    999.2 |
   1023.8 |
   1048.5 |
   1073.1 |
   1097.8 |
   1122.4 |
   1147.0 |#######
   1171.7 |
   1196.3 |###
   1221.0 |###
   1245.6 |
   1270.3 |##############
  (2 below, 2 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=57.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes64**: CV=24.4% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=26.4% (high variance, measurements may be unstable)
- **satfold-seq**: CV=20.2% (high variance, measurements may be unstable)
