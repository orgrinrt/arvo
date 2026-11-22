# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 4.8x slower than the field

satfold-lanes4-idx (4.13 us) is 4.8x the fastest (857 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.61)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (857 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest satfold-iterfold (857 ns) to slowest satfold-lanes4-idx (4.13 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.81 us but its worst 20% at 7.23 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 856.6 ns median
- 8 variants significantly slower than baseline
- Spread: 4.82x (fastest 856.6 ns, slowest 4131.8 ns)

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
| Counter frequency | 2112766038 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1090ns | 1073ns | 986ns | 1076ns | 1233ns | base |
| satfold-lanes16 | 2314ns | 2290ns | 2030ns | 2303ns | 2633ns | +112.40% |
| satfold-lanes16-constl | 2034ns | 2040ns | 1676ns | 2034ns | 2392ns | +86.66% |
| satfold-lanes4-idx | 4357ns | 4260ns | 1861ns | 4138ns | 7508ns | +299.84% |
| satfold-lanes64 | 1393ns | 1365ns | 1169ns | 1392ns | 1622ns | +27.88% |
| satfold-neon | 1261ns | 1261ns | 1098ns | 1265ns | 1409ns | +15.70% |
| satfold-neon8 | 1398ns | 1383ns | 1166ns | 1384ns | 1671ns | +28.26% |
| satfold-nolaw | 3547ns | 3692ns | 2315ns | 3656ns | 4449ns | +225.48% |
| satfold-seq | 1154ns | 1127ns | 1024ns | 1133ns | 1346ns | +5.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 861ns | 792ns | 940ns | base | 38.069 |
| satfold-lanes16 | 2073ns | 1830ns | 2314ns | +140.86% | 15.805 |
| satfold-lanes16-constl | 1809ns | 1497ns | 2113ns | +110.12% | 18.117 |
| satfold-lanes4-idx | 4217ns | 1807ns | 7228ns | +389.94% | 7.770 |
| satfold-lanes64 | 1134ns | 960ns | 1296ns | +31.76% | 28.892 |
| satfold-neon | 1029ns | 904ns | 1124ns | +19.54% | 31.847 |
| satfold-neon8 | 1143ns | 959ns | 1354ns | +32.77% | 28.672 |
| satfold-nolaw | 3330ns | 2168ns | 4173ns | +286.90% | 9.839 |
| satfold-seq | 901ns | 807ns | 1027ns | +4.65% | 36.377 |

## Performance model

- Peak throughput: **41.381 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 38.254 | 92.4% |
| satfold-lanes16 | 15.849 | 38.3% |
| satfold-lanes16-constl | 17.998 | 43.5% |
| satfold-lanes4-idx | 7.931 | 19.2% |
| satfold-lanes64 | 29.453 | 71.2% |
| satfold-neon | 32.067 | 77.5% |
| satfold-neon8 | 29.156 | 70.5% |
| satfold-nolaw | 9.443 | 22.8% |
| satfold-seq | 36.799 | 88.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1090ns | 1090ns | base |
| satfold-lanes16 | 2314ns | 2314ns | +112.40% |
| satfold-lanes16-constl | 2034ns | 2034ns | +86.66% |
| satfold-lanes4-idx | 4357ns | 4357ns | +299.84% |
| satfold-lanes64 | 1393ns | 1393ns | +27.88% |
| satfold-neon | 1261ns | 1261ns | +15.70% |
| satfold-neon8 | 1398ns | 1398ns | +28.26% |
| satfold-nolaw | 3547ns | 3547ns | +225.48% |
| satfold-seq | 1154ns | 1154ns | +5.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 857ns | base | --- | [847, 872] | --- | --- | --- | --- |
| satfold-lanes16 | 2067ns | +1217.9ns (+142.2%) | [+1122, +1297]ns | [1987, 2182] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1821ns | +957.8ns (+111.8%) | [+834, +1029]ns | [1696, 1904] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 4132ns | +3233.8ns (+377.5%) | [+1359, +4852]ns | [2208, 5679] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1113ns | +296.8ns (+34.6%) | [+247, +326]ns | [1093, 1220] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1022ns | +171.9ns (+20.1%) | [+127, +232]ns | [995, 1093] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1124ns | +275.1ns (+32.1%) | [+201, +358]ns | [1048, 1225] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 3470ns | +2613.0ns (+305.0%) | [+2458, +2704]ns | [3302, 3592] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 890ns | +24.9ns (+2.9%) | [+16, +52]ns | [881, 896] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 872ns | +128.0% | +103.0% | +243.8% | +41.4% | +14.4% | +44.9% | +299.9% | +2.1% |
| 1 | 2 | 885ns | +149.5% | +130.0% | +150.3% | +12.3% | +3.5% | +15.2% | +273.8% | +11.1% |
| 1 | 3 | 797ns | +158.8% | +92.0% | +563.7% | +37.4% | +37.2% | +25.4% | +318.9% | +1.6% |
| 1 | 4 | 861ns | +126.1% | +112.8% | +404.6% | +36.7% | +27.6% | +43.0% | +355.6% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.355 | moderate- |
| satfold-lanes16 | -0.237 | moderate- |
| satfold-lanes16-constl | -0.614 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.052 | ok |
| satfold-lanes64 | -0.365 | moderate- |
| satfold-neon | 0.028 | ok |
| satfold-neon8 | -0.243 | moderate- |
| satfold-nolaw | -0.121 | ok |
| satfold-seq | -0.517 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 77.3ns | 860.8ns | 9.0% | HIGH |
| satfold-lanes16 | 81.8ns | 2073.2ns | 3.9% |  |
| satfold-lanes16-constl | 76.5ns | 1808.7ns | 4.2% |  |
| satfold-lanes4-idx | 45.0ns | 4217.1ns | 1.1% |  |
| satfold-lanes64 | 90.9ns | 1134.2ns | 8.0% | HIGH |
| satfold-neon | 79.9ns | 1028.9ns | 7.8% | HIGH |
| satfold-neon8 | 86.9ns | 1142.8ns | 7.6% | HIGH |
| satfold-nolaw | 73.0ns | 3330.3ns | 2.2% |  |
| satfold-seq | 86.4ns | 900.8ns | 9.6% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 791.9-939.6 ns)
    791.9 |################
    799.2 |
    806.6 |########################
    814.0 |
    821.4 |
    828.8 |########
    836.2 |########
    843.6 |########################################
    851.0 |################################
    858.3 |################
    865.7 |################################
    873.1 |########################
    880.5 |
    887.9 |################
    895.3 |################
    902.7 |
    910.1 |
    917.4 |
    924.8 |########
    932.2 |########
  (5 below, 4 above range)

satfold-lanes16 (n=40, range 1829.7-2313.7 ns)
   1829.7 |####
   1853.9 |####
   1878.1 |####
   1902.3 |####
   1926.5 |
   1950.7 |
   1974.9 |############################
   1999.1 |
   2023.3 |############
   2047.5 |
   2071.7 |####
   2095.9 |####
   2120.1 |####
   2144.3 |
   2168.5 |########################################
   2192.7 |
   2216.9 |########
   2241.1 |####
   2265.3 |####
   2289.5 |
  (6 below, 3 above range)

satfold-lanes16-constl (n=40, range 1496.6-2113.0 ns)
   1496.6 |
   1527.4 |########################################
   1558.2 |##########
   1589.1 |
   1619.9 |##########
   1650.7 |##############################
   1681.5 |##############################
   1712.3 |####################
   1743.2 |
   1774.0 |
   1804.8 |##############################
   1835.6 |####################
   1866.4 |##############################
   1897.3 |##########
   1928.1 |
   1958.9 |####################
   1989.7 |########################################
   2020.6 |####################
   2051.4 |####################
   2082.2 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 1806.7-7227.8 ns)
   1806.7 |########################################
   2077.8 |########################################
   2348.8 |
   2619.9 |
   2890.9 |######
   3162.0 |######
   3433.1 |
   3704.1 |
   3975.2 |#############
   4246.2 |
   4517.3 |
   4788.3 |######
   5059.4 |######
   5330.4 |#############
   5601.5 |####################
   5872.5 |#############
   6143.6 |####################
   6414.7 |#############
   6685.7 |######
   6956.8 |######
  (5 below, 3 above range)

satfold-lanes64 (n=40, range 960.1-1296.2 ns)
    960.1 |
    976.9 |######
    993.7 |##########################
   1010.5 |
   1027.3 |######
   1044.1 |
   1060.9 |
   1077.7 |##########################
   1094.5 |########################################
   1111.3 |#############
   1128.1 |######
   1144.9 |######
   1161.7 |
   1178.5 |
   1195.3 |######
   1212.1 |####################
   1228.9 |#################################
   1245.8 |######
   1262.6 |######
   1279.4 |
  (4 below, 5 above range)

satfold-neon (n=40, range 904.3-1124.3 ns)
    904.3 |###############
    915.3 |###############
    926.3 |#####
    937.3 |
    948.3 |
    959.3 |
    970.3 |##########
    981.3 |#####
    992.3 |#########################
   1003.3 |##########
   1014.3 |#####
   1025.3 |#####
   1036.3 |
   1047.3 |#####
   1058.3 |
   1069.3 |
   1080.3 |##########
   1091.3 |########################################
   1102.3 |##########
   1113.3 |###############
  (2 below, 3 above range)

satfold-neon8 (n=40, range 958.7-1354.4 ns)
    958.7 |
    978.5 |######
    998.3 |#################################
   1018.1 |####################
   1037.8 |####################
   1057.6 |######
   1077.4 |######
   1097.2 |######
   1117.0 |####################
   1136.8 |
   1156.6 |
   1176.3 |######
   1196.1 |
   1215.9 |########################################
   1235.7 |######
   1255.5 |####################
   1275.3 |
   1295.1 |
   1314.9 |######
   1334.6 |#############
  (4 below, 4 above range)

satfold-nolaw (n=40, range 2167.9-4172.6 ns)
   2167.9 |
   2268.2 |
   2368.4 |################################
   2468.6 |########
   2568.9 |########
   2669.1 |
   2769.3 |
   2869.6 |########
   2969.8 |########
   3070.0 |########
   3170.3 |################
   3270.5 |################################
   3370.7 |########################
   3471.0 |################################
   3571.2 |########################################
   3671.4 |########
   3771.7 |########################
   3871.9 |################
   3972.1 |########
   4072.4 |
  (2 below, 4 above range)

satfold-seq (n=40, range 807.1-1026.8 ns)
    807.1 |#####################
    818.0 |#######
    829.0 |###
    840.0 |###
    851.0 |
    862.0 |
    873.0 |##########
    884.0 |########################################
    894.9 |#######
    905.9 |
    916.9 |
    927.9 |###
    938.9 |###
    949.9 |###
    960.9 |
    971.8 |#######
    982.8 |##########
    993.8 |
   1004.8 |
   1015.8 |
  (2 below, 4 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=52.1% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-nolaw**: CV=21.6% (high variance, measurements may be unstable)
