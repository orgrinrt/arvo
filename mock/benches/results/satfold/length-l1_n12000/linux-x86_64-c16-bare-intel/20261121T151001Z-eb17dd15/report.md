# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 dominates: 13% faster than the next best (satfold-neon8)

satfold-lanes64 (1.00 us) leads satfold-neon8 (1.14 us) by 13%, a clear separation rather than a photo finish. CV 10.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes64 beats baseline by 96% (significant)

satfold-lanes64 is -26.27 us (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 27.2x slower than the field

satfold-seq (27.29 us) is 27.2x the fastest (1.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.75)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-neon8, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (430% apart)

The field splits into a fast tier {satfold-lanes64, satfold-neon8, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 430% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 27.2x the fastest

Fastest satfold-lanes64 (1.00 us) to slowest satfold-seq (27.29 us): 27.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.9x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.12 us but its worst 20% at 8.30 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 1002.3 ns median (-96.3% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 27.22x (fastest 1002.3 ns, slowest 27286.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 26117ns | 27352ns | 22376ns | 26940ns | 27388ns | base |
| satfold-lanes16 | 2424ns | 2358ns | 2165ns | 2409ns | 2726ns | -90.72% |
| satfold-lanes16-constl | 2257ns | 2247ns | 1825ns | 2244ns | 2730ns | -91.36% |
| satfold-lanes4-idx | 4079ns | 2412ns | 2181ns | 3208ns | 8591ns | -84.38% |
| satfold-lanes64 | 1239ns | 1205ns | 1095ns | 1215ns | 1455ns | -95.26% |
| satfold-neon | 1392ns | 1353ns | 1284ns | 1374ns | 1553ns | -94.67% |
| satfold-neon8 | 1387ns | 1363ns | 1216ns | 1380ns | 1576ns | -94.69% |
| satfold-nolaw | 11937ns | 12511ns | 10189ns | 12318ns | 12543ns | -54.29% |
| satfold-seq | 26134ns | 27354ns | 22375ns | 26945ns | 27462ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 26049ns | 22322ns | 27303ns | base | 1.258 |
| satfold-lanes16 | 2188ns | 1966ns | 2419ns | -91.60% | 14.973 |
| satfold-lanes16-constl | 2029ns | 1649ns | 2432ns | -92.21% | 16.153 |
| satfold-lanes4-idx | 3956ns | 2122ns | 8296ns | -84.81% | 8.282 |
| satfold-lanes64 | 1019ns | 914ns | 1163ns | -96.09% | 32.152 |
| satfold-neon | 1159ns | 1079ns | 1256ns | -95.55% | 28.274 |
| satfold-neon8 | 1156ns | 1021ns | 1303ns | -95.56% | 28.339 |
| satfold-nolaw | 11865ns | 10130ns | 12463ns | -54.45% | 2.762 |
| satfold-seq | 26060ns | 22321ns | 27359ns | +0.04% | 1.257 |

## Performance model

- Peak throughput: **35.861 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.201 | 3.3% |
| satfold-lanes16 | 15.300 | 42.7% |
| satfold-lanes16-constl | 16.251 | 45.3% |
| satfold-lanes4-idx | 13.963 | 38.9% |
| satfold-lanes64 | 32.693 | 91.2% |
| satfold-neon | 28.650 | 79.9% |
| satfold-neon8 | 28.824 | 80.4% |
| satfold-nolaw | 2.635 | 7.3% |
| satfold-seq | 1.201 | 3.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 26117ns | 26117ns | base |
| satfold-lanes16 | 2424ns | 2424ns | -90.72% |
| satfold-lanes16-constl | 2257ns | 2257ns | -91.36% |
| satfold-lanes4-idx | 4079ns | 4079ns | -84.38% |
| satfold-lanes64 | 1239ns | 1239ns | -95.26% |
| satfold-neon | 1392ns | 1392ns | -94.67% |
| satfold-neon8 | 1387ns | 1387ns | -94.69% |
| satfold-nolaw | 11937ns | 11937ns | -54.29% |
| satfold-seq | 26134ns | 26134ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 27286ns | base | --- | [27281, 27290] | --- | --- | --- | --- |
| satfold-lanes16 | 2142ns | -24988.1ns (-91.6%) | [-25138, -24916]ns | [2090, 2299] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2016ns | -25074.4ns (-91.9%) | [-25102, -24876]ns | [1850, 2200] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2347ns | -22721.8ns (-83.3%) | [-24943, -20356]ns | [2333, 3307] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1002ns | -26265.4ns (-96.3%) | [-26317, -26191]ns | [977, 1053] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1144ns | -26099.8ns (-95.7%) | [-26154, -26049]ns | [1128, 1164] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1137ns | -26047.2ns (-95.5%) | [-26118, -25999]ns | [1117, 1198] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12433ns | -14838.2ns (-54.4%) | [-14849, -14825]ns | [12420, 12443] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 27287ns | no significant difference | [-9, +8]ns | [27280, 27289] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 27292ns | -91.6% | -91.9% | -91.4% | -96.0% | -95.5% | -95.4% | -54.4% | +0.0% |
| 1 | 2 | 22321ns | -90.5% | -92.4% | -89.5% | -95.2% | -94.9% | -95.3% | -54.6% | +22.3% |
| 1 | 3 | 27286ns | -92.2% | -93.3% | -91.4% | -96.4% | -95.8% | -95.5% | -54.5% | +0.0% |
| 1 | 4 | 27289ns | -91.9% | -91.7% | -82.3% | -96.6% | -95.9% | -95.9% | -54.3% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.417 | moderate- |
| satfold-lanes16 | -0.210 | moderate- |
| satfold-lanes16-constl | -0.247 | moderate- |
| satfold-lanes4-idx | -0.088 | ok |
| satfold-lanes64 | 0.293 | moderate+ |
| satfold-neon | 0.039 | ok |
| satfold-neon8 | -0.752 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.409 | moderate- |
| satfold-seq | -0.083 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.1ns | 26048.8ns | 0.1% |  |
| satfold-lanes16 | 78.7ns | 2188.4ns | 3.6% |  |
| satfold-lanes16-constl | 76.8ns | 2028.5ns | 3.8% |  |
| satfold-lanes4-idx | 40.0ns | 3956.4ns | 1.0% |  |
| satfold-lanes64 | 71.5ns | 1019.1ns | 7.0% | HIGH |
| satfold-neon | 79.4ns | 1158.9ns | 6.8% | HIGH |
| satfold-neon8 | 79.6ns | 1156.3ns | 6.9% | HIGH |
| satfold-nolaw | 22.7ns | 11864.6ns | 0.2% |  |
| satfold-seq | 22.2ns | 26059.5ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22322.0-27302.5 ns)
  22322.0 |#####
  22571.0 |
  22820.0 |
  23069.0 |
  23318.1 |
  23567.1 |
  23816.1 |
  24065.1 |
  24314.2 |
  24563.2 |
  24812.2 |
  25061.3 |
  25310.3 |
  25559.3 |
  25808.3 |
  26057.4 |
  26306.4 |
  26555.4 |
  26804.5 |
  27053.5 |########################################
  (6 below, 2 above range)

satfold-lanes16 (n=40, range 1965.9-2419.0 ns)
   1965.9 |
   1988.6 |###########
   2011.2 |
   2033.9 |
   2056.5 |###########
   2079.2 |########################################
   2101.8 |
   2124.5 |##################################
   2147.2 |
   2169.8 |
   2192.5 |
   2215.1 |#####
   2237.8 |#####
   2260.4 |
   2283.1 |########################################
   2305.8 |###########
   2328.4 |#####
   2351.1 |#####
   2373.7 |#################
   2396.4 |
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 1648.7-2431.6 ns)
   1648.7 |####
   1687.9 |######################
   1727.0 |####
   1766.2 |
   1805.3 |#############
   1844.5 |#############
   1883.6 |
   1922.7 |####
   1961.9 |
   2001.0 |#################
   2040.2 |####
   2079.3 |####
   2118.5 |
   2157.6 |####
   2196.8 |########################################
   2235.9 |
   2275.0 |
   2314.2 |########
   2353.3 |
   2392.5 |####
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 2121.7-8296.2 ns)
   2121.7 |########################################
   2430.4 |
   2739.1 |
   3047.9 |
   3356.6 |
   3665.3 |
   3974.0 |#
   4282.8 |
   4591.5 |
   4900.2 |#
   5208.9 |#
   5517.7 |
   5826.4 |###
   6135.1 |#
   6443.8 |#
   6752.6 |
   7061.3 |
   7370.0 |###
   7678.7 |#
   7987.5 |#
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 913.8-1163.2 ns)
    913.8 |########################################
    926.2 |#############
    938.7 |
    951.2 |
    963.6 |####
    976.1 |########
    988.6 |#################
   1001.1 |########
   1013.5 |########
   1026.0 |####
   1038.5 |
   1050.9 |########
   1063.4 |
   1075.9 |######################
   1088.4 |#################
   1100.8 |####
   1113.3 |####
   1125.8 |
   1138.2 |
   1150.7 |
  (1 below, 2 above range)

satfold-neon (n=40, range 1079.3-1256.4 ns)
   1079.3 |######
   1088.2 |######
   1097.0 |
   1105.9 |####################
   1114.7 |#################################
   1123.6 |#############
   1132.4 |##########################
   1141.3 |########################################
   1150.1 |
   1159.0 |#############
   1167.8 |
   1176.7 |######
   1185.5 |
   1194.4 |
   1203.3 |
   1212.1 |#############
   1221.0 |######
   1229.8 |####################
   1238.7 |######
   1247.5 |#############
  (3 below, 3 above range)

satfold-neon8 (n=40, range 1021.3-1302.6 ns)
   1021.3 |#############
   1035.4 |####################
   1049.4 |######
   1063.5 |
   1077.6 |######
   1091.6 |
   1105.7 |########################################
   1119.8 |#############
   1133.8 |#############
   1147.9 |#############
   1161.9 |
   1176.0 |#############
   1190.1 |#############
   1204.1 |
   1218.2 |#############
   1232.3 |#############
   1246.3 |####################
   1260.4 |######
   1274.5 |######
   1288.5 |######
  (4 below, 3 above range)

satfold-nolaw (n=40, range 10130.2-12463.3 ns)
  10130.2 |#########
  10246.8 |
  10363.5 |
  10480.2 |
  10596.8 |
  10713.5 |
  10830.1 |
  10946.8 |
  11063.4 |
  11180.1 |
  11296.8 |
  11413.4 |
  11530.1 |
  11646.7 |
  11763.4 |
  11880.0 |
  11996.7 |
  12113.4 |
  12230.0 |
  12346.7 |########################################
  (4 below, 4 above range)

satfold-seq (n=40, range 22320.9-27359.3 ns)
  22320.9 |######
  22572.8 |
  22824.7 |
  23076.6 |
  23328.5 |
  23580.5 |
  23832.4 |
  24084.3 |
  24336.2 |
  24588.1 |
  24840.1 |
  25092.0 |
  25343.9 |
  25595.8 |
  25847.8 |
  26099.7 |
  26351.6 |
  26603.5 |
  26855.4 |
  27107.4 |########################################
  (5 below, 1 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=64.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
