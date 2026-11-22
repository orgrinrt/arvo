# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 dominates: 15% faster than the next best (satfold-lanes64)

satfold-neon8 (1.17 us) leads satfold-lanes64 (1.34 us) by 15%, a clear separation rather than a photo finish. CV 9.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon8 beats baseline by 95% (significant)

satfold-neon8 is -23.56 us (95%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 21.3x slower than the field

satfold-seq (24.81 us) is 21.3x the fastest (1.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.65)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-lanes64, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (443% apart)

The field splits into a fast tier {satfold-neon8, satfold-lanes64, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 443% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 21.3x the fastest

Fastest satfold-neon8 (1.17 us) to slowest satfold-seq (24.81 us): 21.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.8x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.09 us but its worst 20% at 8.04 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 1166.0 ns median (-95.3% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 21.27x (fastest 1166.0 ns, slowest 24805.2 ns)

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
| Counter frequency | 2112716245 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 24875ns | 24894ns | 22375ns | 24873ns | 27382ns | base |
| satfold-lanes16 | 2519ns | 2439ns | 1856ns | 2434ns | 3436ns | -89.87% |
| satfold-lanes16-constl | 2458ns | 2438ns | 2077ns | 2432ns | 2919ns | -90.12% |
| satfold-lanes4-idx | 4071ns | 2352ns | 2151ns | 3294ns | 8324ns | -83.63% |
| satfold-lanes64 | 1662ns | 1580ns | 1389ns | 1631ns | 2030ns | -93.32% |
| satfold-neon | 1678ns | 1715ns | 1432ns | 1667ns | 1955ns | -93.26% |
| satfold-neon8 | 1405ns | 1402ns | 1255ns | 1384ns | 1617ns | -94.35% |
| satfold-nolaw | 11935ns | 12493ns | 10253ns | 12299ns | 12523ns | -52.02% |
| satfold-seq | 24873ns | 24885ns | 22373ns | 24871ns | 27379ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24803ns | 22315ns | 27293ns | base | 1.321 |
| satfold-lanes16 | 2260ns | 1674ns | 3068ns | -90.89% | 14.501 |
| satfold-lanes16-constl | 2207ns | 1874ns | 2610ns | -91.10% | 14.845 |
| satfold-lanes4-idx | 3945ns | 2091ns | 8040ns | -84.10% | 8.307 |
| satfold-lanes64 | 1394ns | 1173ns | 1686ns | -94.38% | 23.507 |
| satfold-neon | 1407ns | 1208ns | 1624ns | -94.33% | 23.286 |
| satfold-neon8 | 1167ns | 1044ns | 1338ns | -95.30% | 28.080 |
| satfold-nolaw | 11860ns | 10194ns | 12439ns | -52.18% | 2.763 |
| satfold-seq | 24803ns | 22316ns | 27293ns | +0.00% | 1.321 |

## Performance model

- Peak throughput: **31.372 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.321 | 4.2% |
| satfold-lanes16 | 14.896 | 47.5% |
| satfold-lanes16-constl | 14.909 | 47.5% |
| satfold-lanes4-idx | 14.333 | 45.7% |
| satfold-lanes64 | 24.529 | 78.2% |
| satfold-neon | 23.033 | 73.4% |
| satfold-neon8 | 28.103 | 89.6% |
| satfold-nolaw | 2.641 | 8.4% |
| satfold-seq | 1.321 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24875ns | 24875ns | base |
| satfold-lanes16 | 2519ns | 2519ns | -89.87% |
| satfold-lanes16-constl | 2458ns | 2458ns | -90.12% |
| satfold-lanes4-idx | 4071ns | 4071ns | -83.63% |
| satfold-lanes64 | 1662ns | 1662ns | -93.32% |
| satfold-neon | 1678ns | 1678ns | -93.26% |
| satfold-neon8 | 1405ns | 1405ns | -94.35% |
| satfold-nolaw | 11935ns | 11935ns | -52.02% |
| satfold-seq | 24873ns | 24873ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 24801ns | base | --- | [22323, 27285] | --- | --- | --- | --- |
| satfold-lanes16 | 2200ns | -23223.8ns (-93.6%) | [-25144, -20115]ns | [2050, 2244] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2198ns | -22540.4ns (-90.9%) | [-25086, -20124]ns | [2073, 2211] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2286ns | -20040.0ns (-80.8%) | [-21594, -20031]ns | [2285, 3640] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1336ns | -23397.6ns (-94.3%) | [-25796, -21102]ns | [1307, 1456] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1423ns | -23482.5ns (-94.7%) | [-25821, -21009]ns | [1320, 1458] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1166ns | -23564.4ns (-95.0%) | [-26092, -21227]ns | [1086, 1189] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12406ns | -12392.9ns (-50.0%) | [-14862, -9935]ns | [12380, 12426] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24805ns | no significant difference | [-1, +3]ns | [22320, 27285] | no | 0.7493 | 0.7493 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 22318ns | -87.5% | -89.1% | -89.7% | -94.2% | -93.3% | -94.8% | -44.3% | -0.0% |
| 1 | 2 | 22317ns | -90.0% | -90.2% | -89.8% | -93.4% | -94.1% | -94.7% | -44.5% | +0.0% |
| 1 | 3 | 27285ns | -92.6% | -91.9% | -91.6% | -95.1% | -95.1% | -96.1% | -54.5% | +0.0% |
| 1 | 4 | 27287ns | -92.7% | -93.0% | -85.8% | -94.6% | -94.7% | -95.7% | -62.6% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.250 | moderate+ |
| satfold-lanes16 | 0.121 | ok |
| satfold-lanes16-constl | -0.024 | ok |
| satfold-lanes4-idx | -0.081 | ok |
| satfold-lanes64 | -0.651 | HIGH- (thermal bounce) |
| satfold-neon | -0.271 | moderate- |
| satfold-neon8 | -0.586 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.094 | ok |
| satfold-seq | 0.250 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.9ns | 24803.1ns | 0.1% |  |
| satfold-lanes16 | 90.5ns | 2259.7ns | 4.0% |  |
| satfold-lanes16-constl | 84.6ns | 2207.4ns | 3.8% |  |
| satfold-lanes4-idx | 41.6ns | 3944.5ns | 1.1% |  |
| satfold-lanes64 | 94.2ns | 1393.9ns | 6.8% | HIGH |
| satfold-neon | 92.6ns | 1407.2ns | 6.6% | HIGH |
| satfold-neon8 | 82.2ns | 1166.9ns | 7.0% | HIGH |
| satfold-nolaw | 22.9ns | 11859.7ns | 0.2% |  |
| satfold-seq | 21.8ns | 24803.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22315.4-27293.0 ns)
  22315.4 |########################################
  22564.2 |
  22813.1 |
  23062.0 |
  23310.9 |
  23559.8 |
  23808.7 |
  24057.5 |
  24306.4 |
  24555.3 |
  24804.2 |
  25053.1 |
  25302.0 |
  25550.8 |
  25799.7 |
  26048.6 |
  26297.5 |
  26546.4 |
  26795.3 |
  27044.1 |########################################
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 1673.9-3068.1 ns)
   1673.9 |
   1743.6 |
   1813.3 |#################
   1883.0 |########
   1952.7 |######################
   2022.4 |#############
   2092.1 |
   2161.8 |########################################
   2231.5 |#################
   2301.2 |
   2371.0 |
   2440.7 |########
   2510.4 |########
   2580.1 |
   2649.8 |
   2719.5 |#############
   2789.2 |
   2858.9 |
   2928.6 |
   2998.4 |
  (2 below, 4 above range)

satfold-lanes16-constl (n=40, range 1874.2-2609.5 ns)
   1874.2 |#####
   1911.0 |#####
   1947.7 |
   1984.5 |############################
   2021.3 |###########
   2058.0 |###########
   2094.8 |###########
   2131.6 |
   2168.3 |########################################
   2205.1 |######################
   2241.8 |
   2278.6 |
   2315.4 |
   2352.1 |#####
   2388.9 |
   2425.7 |############################
   2462.4 |#####
   2499.2 |#####
   2536.0 |
   2572.7 |
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 2091.0-8039.6 ns)
   2091.0 |########################################
   2388.4 |
   2685.9 |
   2983.3 |
   3280.7 |
   3578.2 |
   3875.6 |
   4173.0 |
   4470.5 |
   4767.9 |#
   5065.3 |
   5362.8 |#
   5660.2 |#
   5957.6 |#
   6255.1 |#
   6552.5 |#
   6849.9 |#
   7147.4 |
   7444.8 |###
   7742.2 |
  (4 below, 5 above range)

satfold-lanes64 (n=40, range 1173.3-1685.8 ns)
   1173.3 |####################
   1198.9 |#####
   1224.5 |##########
   1250.1 |#####
   1275.8 |
   1301.4 |########################################
   1327.0 |###############
   1352.7 |#####
   1378.3 |
   1403.9 |#####
   1429.5 |#####
   1455.2 |####################
   1480.8 |#####
   1506.4 |
   1532.0 |##########
   1557.7 |
   1583.3 |
   1608.9 |#####
   1634.6 |##############################
   1660.2 |
  (3 below, 1 above range)

satfold-neon (n=40, range 1208.3-1624.2 ns)
   1208.3 |######
   1229.1 |
   1249.9 |
   1270.7 |
   1291.5 |#################################
   1312.3 |########################################
   1333.1 |#############
   1353.9 |#############
   1374.7 |######
   1395.5 |
   1416.3 |
   1437.1 |########################################
   1457.9 |#################################
   1478.7 |
   1499.4 |
   1520.2 |
   1541.0 |#############
   1561.8 |
   1582.6 |######
   1603.4 |
  (3 below, 6 above range)

satfold-neon8 (n=40, range 1044.5-1338.4 ns)
   1044.5 |#################
   1059.2 |########################################
   1073.9 |#################
   1088.6 |#####
   1103.3 |#####
   1118.0 |#####
   1132.7 |
   1147.3 |
   1162.0 |############################
   1176.7 |############################
   1191.4 |#####
   1206.1 |
   1220.8 |#####
   1235.5 |
   1250.2 |#####
   1264.9 |
   1279.6 |#####
   1294.3 |######################
   1309.0 |
   1323.7 |
  (2 below, 4 above range)

satfold-nolaw (n=40, range 10193.8-12438.5 ns)
  10193.8 |########
  10306.0 |
  10418.3 |
  10530.5 |
  10642.7 |
  10755.0 |
  10867.2 |
  10979.4 |
  11091.7 |
  11203.9 |
  11316.2 |
  11428.4 |
  11540.6 |
  11652.9 |
  11765.1 |
  11877.3 |
  11989.6 |
  12101.8 |
  12214.1 |
  12326.3 |########################################
  (4 below, 3 above range)

satfold-seq (n=40, range 22315.9-27292.8 ns)
  22315.9 |#####################################
  22564.7 |
  22813.6 |
  23062.4 |
  23311.3 |
  23560.1 |
  23809.0 |
  24057.8 |
  24306.7 |
  24555.5 |
  24804.3 |
  25053.2 |
  25302.0 |
  25550.9 |
  25799.7 |
  26048.6 |
  26297.4 |
  26546.3 |
  26795.1 |
  27044.0 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=24.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=62.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.8x (possible bimodal distribution)
