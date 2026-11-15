# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 beats baseline by 96% (significant)

satfold-neon8 is -25.71 us (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 23.0x slower than the field

satfold-seq (26.93 us) is 23.0x the fastest (1.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon8, satfold-neon) are a dead heat (<1%)

satfold-neon8 (1.17 us) and satfold-neon (1.18 us) differ by 0.32%, inside the noise, even though the wider field spreads 2198.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (303% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 303% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 23.0x the fastest

Fastest satfold-neon8 (1.17 us) to slowest satfold-seq (26.93 us): 23.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.6x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.28 us but its worst 20% at 8.15 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 1171.7 ns median (-95.6% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 22.98x (fastest 1171.7 ns, slowest 26928.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 25735ns | 26956ns | 22049ns | 26549ns | 26977ns | base |
| satfold-lanes16 | 2492ns | 2598ns | 1926ns | 2504ns | 3024ns | -90.32% |
| satfold-lanes16-constl | 3366ns | 3744ns | 1420ns | 3576ns | 4685ns | -86.92% |
| satfold-lanes4-idx | 4692ns | 2374ns | 2341ns | 4216ns | 8472ns | -81.77% |
| satfold-lanes64 | 1540ns | 1493ns | 1286ns | 1540ns | 1793ns | -94.02% |
| satfold-neon | 1428ns | 1410ns | 1263ns | 1417ns | 1626ns | -94.45% |
| satfold-neon8 | 1394ns | 1371ns | 1254ns | 1386ns | 1560ns | -94.58% |
| satfold-nolaw | 14079ns | 14304ns | 11632ns | 14065ns | 16569ns | -45.29% |
| satfold-seq | 27024ns | 27004ns | 26978ns | 27007ns | 27120ns | +5.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 25666ns | 21995ns | 26894ns | base | 1.277 |
| satfold-lanes16 | 2259ns | 1752ns | 2710ns | -91.20% | 14.505 |
| satfold-lanes16-constl | 3136ns | 1307ns | 4296ns | -87.78% | 10.450 |
| satfold-lanes4-idx | 4545ns | 2276ns | 8154ns | -82.29% | 7.210 |
| satfold-lanes64 | 1302ns | 1097ns | 1480ns | -94.93% | 25.170 |
| satfold-neon | 1210ns | 1080ns | 1358ns | -95.29% | 27.086 |
| satfold-neon8 | 1181ns | 1071ns | 1308ns | -95.40% | 27.739 |
| satfold-nolaw | 14002ns | 11571ns | 16471ns | -45.45% | 2.340 |
| satfold-seq | 26944ns | 26904ns | 27027ns | +4.98% | 1.216 |

## Performance model

- Peak throughput: **30.592 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.219 | 4.0% |
| satfold-lanes16 | 13.937 | 45.6% |
| satfold-lanes16-constl | 9.282 | 30.3% |
| satfold-lanes4-idx | 14.200 | 46.4% |
| satfold-lanes64 | 25.672 | 83.9% |
| satfold-neon | 27.876 | 91.1% |
| satfold-neon8 | 27.966 | 91.4% |
| satfold-nolaw | 2.303 | 7.5% |
| satfold-seq | 1.217 | 4.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 25735ns | 25735ns | base |
| satfold-lanes16 | 2492ns | 2492ns | -90.32% |
| satfold-lanes16-constl | 3366ns | 3366ns | -86.92% |
| satfold-lanes4-idx | 4692ns | 4692ns | -81.77% |
| satfold-lanes64 | 1540ns | 1540ns | -94.02% |
| satfold-neon | 1428ns | 1428ns | -94.45% |
| satfold-neon8 | 1394ns | 1394ns | -94.58% |
| satfold-nolaw | 14079ns | 14079ns | -45.29% |
| satfold-seq | 27024ns | 27024ns | +5.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 26889ns | base | --- | [26881, 26891] | --- | --- | --- | --- |
| satfold-lanes16 | 2351ns | -24436.8ns (-90.9%) | [-24542, -24237]ns | [2110, 2442] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3530ns | -23027.0ns (-85.6%) | [-23360, -22569]ns | [3208, 3855] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2308ns | -20066.5ns (-74.6%) | [-24581, -19508]ns | [2295, 6709] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1276ns | -25498.7ns (-94.8%) | [-25512, -25420]ns | [1260, 1385] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1176ns | -25619.7ns (-95.3%) | [-25650, -25523]ns | [1156, 1260] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1172ns | -25709.3ns (-95.6%) | [-25736, -25627]ns | [1154, 1207] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 14231ns | -12640.2ns (-47.0%) | [-12649, -12504]ns | [14154, 14244] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 26929ns | +53.1ns (+0.2%) | [+37, +69]ns | [26922, 26943] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 26891ns | -90.9% | -86.4% | -90.4% | -95.3% | -95.3% | -95.9% | -47.4% | +0.2% |
| 1 | 2 | 21994ns | -90.0% | -84.6% | -77.4% | -95.1% | -94.7% | -94.3% | -47.4% | +22.5% |
| 1 | 3 | 26891ns | -93.3% | -92.7% | -84.6% | -95.1% | -95.7% | -95.3% | -47.0% | +0.1% |
| 1 | 4 | 26890ns | -90.4% | -85.7% | -91.4% | -94.8% | -95.2% | -96.0% | -47.0% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.417 | moderate- |
| satfold-lanes16 | -0.375 | moderate- |
| satfold-lanes16-constl | -0.427 | moderate- |
| satfold-lanes4-idx | -0.242 | moderate- |
| satfold-lanes64 | -0.059 | ok |
| satfold-neon | -0.243 | moderate- |
| satfold-neon8 | -0.246 | moderate- |
| satfold-nolaw | -0.391 | moderate- |
| satfold-seq | 0.236 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.6ns | 25666.2ns | 0.1% |  |
| satfold-lanes16 | 77.0ns | 2259.0ns | 3.4% |  |
| satfold-lanes16-constl | 73.3ns | 3135.7ns | 2.3% |  |
| satfold-lanes4-idx | 47.6ns | 4544.7ns | 1.0% |  |
| satfold-lanes64 | 80.9ns | 1301.9ns | 6.2% | HIGH |
| satfold-neon | 77.3ns | 1209.8ns | 6.4% | HIGH |
| satfold-neon8 | 73.4ns | 1181.3ns | 6.2% | HIGH |
| satfold-nolaw | 24.1ns | 14001.5ns | 0.2% |  |
| satfold-seq | 23.5ns | 26944.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 21995.1-26894.5 ns)
  21995.1 |######
  22240.1 |
  22485.0 |
  22730.0 |
  22975.0 |
  23219.9 |
  23464.9 |
  23709.9 |
  23954.8 |
  24199.8 |
  24444.8 |
  24689.7 |
  24934.7 |
  25179.7 |
  25424.6 |
  25669.6 |
  25914.6 |
  26159.5 |
  26404.5 |
  26649.5 |########################################
  (6 below, 4 above range)

satfold-lanes16 (n=40, range 1751.8-2710.0 ns)
   1751.8 |###########
   1799.7 |######################
   1847.6 |
   1895.6 |
   1943.5 |
   1991.4 |#####
   2039.3 |#################
   2087.2 |
   2135.1 |#####
   2183.0 |
   2230.9 |#################
   2278.8 |
   2326.7 |######################
   2374.6 |###########
   2422.6 |########################################
   2470.5 |###########
   2518.4 |#####
   2566.3 |
   2614.2 |#################
   2662.1 |#####
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 1307.2-4296.2 ns)
   1307.2 |################
   1456.7 |########
   1606.1 |
   1755.6 |
   1905.0 |
   2054.5 |####
   2203.9 |
   2353.4 |########
   2502.8 |
   2652.3 |
   2801.7 |
   2951.2 |####
   3100.6 |########
   3250.1 |########
   3399.5 |############
   3549.0 |########
   3698.4 |########
   3847.9 |########################################
   3997.3 |####
   4146.8 |####
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 2276.1-8153.9 ns)
   2276.1 |########################################
   2570.0 |##
   2863.8 |
   3157.7 |
   3451.6 |##
   3745.5 |
   4039.4 |
   4333.3 |
   4627.2 |
   4921.1 |
   5215.0 |
   5508.9 |
   5802.8 |##
   6096.6 |##
   6390.5 |
   6684.4 |#######
   6978.3 |#####
   7272.2 |#####
   7566.1 |#####
   7860.0 |##
  (5 below, 5 above range)

satfold-lanes64 (n=40, range 1097.1-1479.5 ns)
   1097.1 |
   1116.3 |#####
   1135.4 |
   1154.5 |#####
   1173.6 |#####
   1192.7 |
   1211.9 |
   1231.0 |#####
   1250.1 |########################################
   1269.2 |############################
   1288.3 |
   1307.5 |
   1326.6 |
   1345.7 |
   1364.8 |#################
   1383.9 |########################################
   1403.0 |###########
   1422.2 |#####
   1441.3 |#####
   1460.4 |#####
  (6 below, 3 above range)

satfold-neon (n=40, range 1080.5-1358.4 ns)
   1080.5 |#########################
   1094.4 |#####
   1108.3 |
   1122.2 |
   1136.1 |
   1150.0 |########################################
   1163.9 |####################
   1177.8 |#####
   1191.7 |
   1205.6 |#####
   1219.4 |##########
   1233.3 |
   1247.2 |###############
   1261.1 |##############################
   1275.0 |
   1288.9 |
   1302.8 |#####
   1316.7 |#####
   1330.6 |#####
   1344.5 |
  (2 below, 4 above range)

satfold-neon8 (n=40, range 1071.1-1308.2 ns)
   1071.1 |########################################
   1083.0 |
   1094.8 |########
   1106.7 |
   1118.5 |
   1130.4 |
   1142.2 |########################
   1154.1 |########################################
   1165.9 |########################################
   1177.8 |
   1189.6 |########
   1201.5 |################
   1213.3 |########
   1225.2 |
   1237.1 |########
   1248.9 |################
   1260.8 |################
   1272.6 |################################
   1284.5 |
   1296.3 |
  (5 below, 3 above range)

satfold-nolaw (n=40, range 11571.3-16471.2 ns)
  11571.3 |######
  11816.3 |
  12061.3 |
  12306.3 |
  12551.3 |
  12796.3 |
  13041.3 |
  13286.3 |
  13531.3 |
  13776.3 |
  14021.3 |########################################
  14266.2 |#
  14511.2 |#
  14756.2 |
  15001.2 |
  15246.2 |
  15491.2 |
  15736.2 |
  15981.2 |
  16226.2 |
  (6 below, 2 above range)

satfold-seq (n=40, range 26903.7-27026.6 ns)
  26903.7 |#################################
  26909.8 |####################
  26916.0 |#############
  26922.1 |########################################
  26928.3 |#################################
  26934.4 |
  26940.5 |####################
  26946.7 |#################################
  26952.8 |####################
  26959.0 |#############
  26965.1 |
  26971.3 |
  26977.4 |
  26983.6 |
  26989.7 |
  26995.8 |
  27002.0 |
  27008.1 |
  27014.3 |
  27020.4 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16-constl**: CV=35.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=56.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
