# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (27.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon8 at 1.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon8 dominates: 11% faster than the next best (satfold-lanes64)

satfold-neon8 (1.10 us) leads satfold-lanes64 (1.22 us) by 11%, a clear separation rather than a photo finish. CV 14.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon8 beats baseline by 96% (significant)

satfold-neon8 is -26.23 us (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 24.7x slower than the field

satfold-iterfold (27.29 us) is 24.7x the fastest (1.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.52)

satfold-neon's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-lanes64, satfold-neon, satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (440% apart)

The field splits into a fast tier {satfold-neon8, satfold-lanes64, satfold-neon, satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 440% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 24.7x the fastest

Fastest satfold-neon8 (1.10 us) to slowest satfold-iterfold (27.29 us): 24.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.5x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.28 us but its worst 20% at 7.98 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 1103.2 ns median (-96.0% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 24.73x (fastest 1103.2 ns, slowest 27287.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 27879ns | 27359ns | 27348ns | 27371ns | 29934ns | base |
| satfold-lanes16 | 2336ns | 2356ns | 1930ns | 2334ns | 2747ns | -91.62% |
| satfold-lanes16-constl | 2478ns | 2433ns | 2054ns | 2414ns | 3093ns | -91.11% |
| satfold-lanes4-idx | 4471ns | 2366ns | 2350ns | 3907ns | 8282ns | -83.96% |
| satfold-lanes64 | 1491ns | 1447ns | 1241ns | 1477ns | 1782ns | -94.65% |
| satfold-neon | 1651ns | 1600ns | 1354ns | 1632ns | 2003ns | -94.08% |
| satfold-neon8 | 1394ns | 1363ns | 1172ns | 1364ns | 1705ns | -95.00% |
| satfold-nolaw | 11939ns | 12510ns | 10217ns | 12303ns | 12572ns | -57.17% |
| satfold-seq | 27362ns | 27357ns | 27347ns | 27357ns | 27392ns | -1.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 27786ns | 27277ns | 29784ns | base | 1.179 |
| satfold-lanes16 | 2088ns | 1738ns | 2417ns | -92.49% | 15.696 |
| satfold-lanes16-constl | 2222ns | 1854ns | 2758ns | -92.00% | 14.744 |
| satfold-lanes4-idx | 4331ns | 2285ns | 7978ns | -84.41% | 7.565 |
| satfold-lanes64 | 1251ns | 1051ns | 1457ns | -95.50% | 26.189 |
| satfold-neon | 1380ns | 1133ns | 1658ns | -95.03% | 23.748 |
| satfold-neon8 | 1151ns | 974ns | 1392ns | -95.86% | 28.475 |
| satfold-nolaw | 11863ns | 10157ns | 12483ns | -57.31% | 2.762 |
| satfold-seq | 27286ns | 27277ns | 27292ns | -1.80% | 1.201 |

## Performance model

- Peak throughput: **33.641 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.201 | 3.6% |
| satfold-lanes16 | 15.394 | 45.8% |
| satfold-lanes16-constl | 14.924 | 44.4% |
| satfold-lanes4-idx | 14.250 | 42.4% |
| satfold-lanes64 | 26.771 | 79.6% |
| satfold-neon | 24.651 | 73.3% |
| satfold-neon8 | 29.703 | 88.3% |
| satfold-nolaw | 2.641 | 7.9% |
| satfold-seq | 1.201 | 3.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 27879ns | 27879ns | base |
| satfold-lanes16 | 2336ns | 2336ns | -91.62% |
| satfold-lanes16-constl | 2478ns | 2478ns | -91.11% |
| satfold-lanes4-idx | 4471ns | 4471ns | -83.96% |
| satfold-lanes64 | 1491ns | 1491ns | -94.65% |
| satfold-neon | 1651ns | 1651ns | -94.08% |
| satfold-neon8 | 1394ns | 1394ns | -95.00% |
| satfold-nolaw | 11939ns | 11939ns | -57.17% |
| satfold-seq | 27362ns | 27362ns | -1.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 27288ns | base | --- | [27286, 27291] | --- | --- | --- | --- |
| satfold-lanes16 | 2129ns | -25290.3ns (-92.7%) | [-25472, -25236]ns | [1999, 2203] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2196ns | -25268.7ns (-92.6%) | [-25315, -25095]ns | [2036, 2225] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2299ns | -24987.2ns (-91.6%) | [-24996, -21804]ns | [2286, 6148] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1224ns | -26084.0ns (-95.6%) | [-26100, -25979]ns | [1194, 1316] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1329ns | -25969.9ns (-95.2%) | [-26080, -25916]ns | [1311, 1457] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1103ns | -26228.1ns (-96.1%) | [-26307, -26210]ns | [1063, 1183] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12406ns | -14924.2ns (-54.7%) | [-17124, -14910]ns | [12363, 12453] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 27287ns | no significant difference | [-9, +2]ns | [27286, 27288] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30137ns | -92.5% | -91.6% | -92.4% | -96.5% | -95.6% | -95.7% | -58.7% | -9.5% |
| 1 | 2 | 27286ns | -92.3% | -91.9% | -80.7% | -95.6% | -95.2% | -96.1% | -54.7% | +0.0% |
| 1 | 3 | 27288ns | -93.5% | -92.4% | -79.7% | -95.1% | -94.2% | -95.8% | -62.8% | -0.0% |
| 1 | 4 | 27285ns | -92.2% | -93.2% | -91.6% | -95.6% | -95.5% | -96.4% | -54.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.084 | ok |
| satfold-lanes16 | -0.174 | ok |
| satfold-lanes16-constl | 0.163 | ok |
| satfold-lanes4-idx | -0.251 | moderate- |
| satfold-lanes64 | 0.002 | ok |
| satfold-neon | -0.525 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.329 | moderate- |
| satfold-nolaw | -0.410 | moderate- |
| satfold-seq | 0.102 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 27.6ns | 27786.3ns | 0.1% |  |
| satfold-lanes16 | 80.3ns | 2087.7ns | 3.8% |  |
| satfold-lanes16-constl | 84.8ns | 2222.5ns | 3.8% |  |
| satfold-lanes4-idx | 45.3ns | 4331.4ns | 1.0% |  |
| satfold-lanes64 | 82.5ns | 1251.2ns | 6.6% | HIGH |
| satfold-neon | 92.7ns | 1379.8ns | 6.7% | HIGH |
| satfold-neon8 | 82.5ns | 1150.8ns | 7.2% | HIGH |
| satfold-nolaw | 22.9ns | 11862.8ns | 0.2% |  |
| satfold-seq | 24.1ns | 27285.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 27276.6-29784.3 ns)
  27276.6 |########################################
  27402.0 |
  27527.4 |#
  27652.8 |
  27778.2 |
  27903.6 |
  28028.9 |
  28154.3 |
  28279.7 |
  28405.1 |
  28530.5 |
  28655.9 |
  28781.2 |
  28906.6 |
  29032.0 |
  29157.4 |
  29282.8 |
  29408.1 |
  29533.5 |
  29658.9 |
  (4 below, 7 above range)

satfold-lanes16 (n=40, range 1738.2-2417.4 ns)
   1738.2 |
   1772.1 |
   1806.1 |################
   1840.0 |
   1874.0 |####
   1908.0 |####
   1941.9 |
   1975.9 |########################
   2009.8 |########
   2043.8 |####
   2077.8 |
   2111.7 |
   2145.7 |
   2179.7 |########################################
   2213.6 |####
   2247.6 |################
   2281.5 |
   2315.5 |####
   2349.5 |
   2383.4 |
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 1853.5-2757.9 ns)
   1853.5 |#####
   1898.7 |
   1944.0 |
   1989.2 |########################################
   2034.4 |##########
   2079.6 |#####
   2124.8 |
   2170.0 |###################################
   2215.3 |####################
   2260.5 |##########
   2305.7 |
   2350.9 |
   2396.1 |#####
   2441.3 |###############
   2486.6 |
   2531.8 |#####
   2577.0 |
   2622.2 |#####
   2667.4 |
   2712.6 |##########
  (5 below, 2 above range)

satfold-lanes4-idx (n=40, range 2284.9-7978.0 ns)
   2284.9 |########################################
   2569.6 |
   2854.2 |
   3138.9 |
   3423.5 |##
   3708.2 |
   3992.9 |
   4277.5 |##
   4562.2 |
   4846.8 |##
   5131.5 |
   5416.1 |
   5700.8 |
   5985.4 |####
   6270.1 |####
   6554.8 |
   6839.4 |######
   7124.1 |##
   7408.7 |####
   7693.4 |##
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 1050.9-1457.3 ns)
   1050.9 |#####
   1071.2 |#####
   1091.5 |#####
   1111.8 |#####
   1132.2 |
   1152.5 |###########
   1172.8 |#################
   1193.1 |########################################
   1213.5 |
   1233.8 |###########
   1254.1 |
   1274.4 |
   1294.7 |######################
   1315.1 |######################
   1335.4 |###########
   1355.7 |#####
   1376.0 |#################
   1396.4 |
   1416.7 |#####
   1437.0 |
  (4 below, 3 above range)

satfold-neon (n=40, range 1132.6-1657.8 ns)
   1132.6 |
   1158.9 |####
   1185.1 |#############
   1211.4 |
   1237.7 |
   1263.9 |####
   1290.2 |########################################
   1316.4 |#############
   1342.7 |####
   1368.9 |####
   1395.2 |
   1421.4 |####
   1447.7 |###################################
   1474.0 |####
   1500.2 |
   1526.5 |
   1552.7 |####
   1579.0 |########
   1605.2 |
   1631.5 |########
  (4 below, 2 above range)

satfold-neon8 (n=40, range 974.0-1392.2 ns)
    974.0 |############################
    994.9 |
   1015.9 |###########
   1036.8 |#####
   1057.7 |########################################
   1078.6 |#####
   1099.5 |#####
   1120.4 |#####
   1141.3 |#####
   1162.2 |#################
   1183.1 |###########
   1204.0 |
   1225.0 |
   1245.9 |#####
   1266.8 |#####
   1287.7 |############################
   1308.6 |
   1329.5 |
   1350.4 |
   1371.3 |#####
  (4 below, 4 above range)

satfold-nolaw (n=40, range 10156.9-12482.9 ns)
  10156.9 |#############
  10273.2 |
  10389.5 |
  10505.8 |
  10622.1 |
  10738.4 |
  10854.7 |
  10971.0 |
  11087.3 |
  11203.6 |
  11319.9 |
  11436.2 |
  11552.5 |
  11668.8 |
  11785.1 |
  11901.4 |
  12017.7 |
  12134.0 |
  12250.3 |##########
  12366.6 |########################################
  (2 below, 1 above range)

satfold-seq (n=40, range 27276.5-27291.8 ns)
  27276.5 |################
  27277.3 |########
  27278.1 |
  27278.8 |
  27279.6 |
  27280.3 |
  27281.1 |
  27281.9 |################
  27282.6 |########
  27283.4 |
  27284.2 |################
  27284.9 |########################
  27285.7 |########################
  27286.4 |################################
  27287.2 |########################
  27288.0 |################
  27288.7 |########################
  27289.5 |########################################
  27290.3 |################
  27291.0 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=56.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.5x (possible bimodal distribution)
