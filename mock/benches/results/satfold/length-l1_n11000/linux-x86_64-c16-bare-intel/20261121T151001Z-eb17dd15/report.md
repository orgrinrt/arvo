# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (24.48 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 1.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon beats baseline by 95% (significant)

satfold-neon is -23.31 us (95%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 20.7x slower than the field

satfold-iterfold (24.48 us) is 20.7x the fastest (1.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-nolaw shows alternating (throttle bounce) (autocorr -0.72)

satfold-nolaw's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-neon8, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (358% apart)

The field splits into a fast tier {satfold-neon, satfold-neon8, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 358% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 20.7x the fastest

Fastest satfold-neon (1.18 us) to slowest satfold-iterfold (24.48 us): 20.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader satfold-neon vs stability leader satfold-lanes64 (+8% speed for 1.2x steadier)

satfold-neon is fastest (1.18 us, CV 11.4%); satfold-lanes64 gives up 8.2% median for 1.2x lower variance (CV 9.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.9x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.86 us but its worst 20% at 7.17 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 1180.4 ns median (-95.2% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 20.74x (fastest 1180.4 ns, slowest 24476.2 ns)

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
| satfold-iterfold | 24561ns | 24558ns | 22087ns | 24552ns | 27060ns | base |
| satfold-lanes16 | 2917ns | 2932ns | 2409ns | 2924ns | 3406ns | -88.12% |
| satfold-lanes16-constl | 2833ns | 2712ns | 2256ns | 2783ns | 3559ns | -88.47% |
| satfold-lanes4-idx | 4099ns | 3020ns | 1914ns | 3718ns | 7427ns | -83.31% |
| satfold-lanes64 | 1539ns | 1487ns | 1362ns | 1519ns | 1774ns | -93.73% |
| satfold-neon | 1413ns | 1382ns | 1181ns | 1410ns | 1654ns | -94.25% |
| satfold-neon8 | 1488ns | 1471ns | 1322ns | 1469ns | 1714ns | -93.94% |
| satfold-nolaw | 13075ns | 13509ns | 11723ns | 13089ns | 14388ns | -46.76% |
| satfold-seq | 23323ns | 22108ns | 22058ns | 22516ns | 27012ns | -5.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24496ns | 22033ns | 26981ns | base | 1.338 |
| satfold-lanes16 | 2689ns | 2215ns | 3128ns | -89.02% | 12.187 |
| satfold-lanes16-constl | 2612ns | 2083ns | 3272ns | -89.34% | 12.545 |
| satfold-lanes4-idx | 3975ns | 1861ns | 7168ns | -83.77% | 8.245 |
| satfold-lanes64 | 1306ns | 1162ns | 1485ns | -94.67% | 25.082 |
| satfold-neon | 1200ns | 1012ns | 1374ns | -95.10% | 27.303 |
| satfold-neon8 | 1267ns | 1132ns | 1449ns | -94.83% | 25.861 |
| satfold-nolaw | 13005ns | 11656ns | 14309ns | -46.91% | 2.520 |
| satfold-seq | 23261ns | 22004ns | 26937ns | -5.04% | 1.409 |

## Performance model

- Peak throughput: **32.391 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.339 | 4.1% |
| satfold-lanes16 | 12.143 | 37.5% |
| satfold-lanes16-constl | 13.223 | 40.8% |
| satfold-lanes4-idx | 11.157 | 34.4% |
| satfold-lanes64 | 25.661 | 79.2% |
| satfold-neon | 27.760 | 85.7% |
| satfold-neon8 | 26.013 | 80.3% |
| satfold-nolaw | 2.438 | 7.5% |
| satfold-seq | 1.486 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24561ns | 24561ns | base |
| satfold-lanes16 | 2917ns | 2917ns | -88.12% |
| satfold-lanes16-constl | 2833ns | 2833ns | -88.47% |
| satfold-lanes4-idx | 4099ns | 4099ns | -83.31% |
| satfold-lanes64 | 1539ns | 1539ns | -93.73% |
| satfold-neon | 1413ns | 1413ns | -94.25% |
| satfold-neon8 | 1488ns | 1488ns | -93.94% |
| satfold-nolaw | 13075ns | 13075ns | -46.76% |
| satfold-seq | 23323ns | 23323ns | -5.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 24476ns | base | --- | [22050, 26919] | --- | --- | --- | --- |
| satfold-lanes16 | 2698ns | -21851.9ns (-89.3%) | [-24043, -19708]ns | [2620, 2799] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2478ns | -21748.7ns (-88.9%) | [-23826, -19881]ns | [2303, 2818] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2937ns | -20194.2ns (-82.5%) | [-20988, -20186]ns | [1867, 5512] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1277ns | -23238.4ns (-94.9%) | [-25656, -20759]ns | [1260, 1330] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1180ns | -23312.2ns (-95.2%) | [-25643, -21015]ns | [1167, 1268] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1260ns | -23030.1ns (-94.1%) | [-25649, -20823]ns | [1196, 1267] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 13439ns | -11515.9ns (-47.0%) | [-12671, -10361]ns | [11770, 14161] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22052ns | no significant difference | [-37, +5]ns | [22050, 22061] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 22046ns | -89.6% | -89.6% | -91.5% | -94.3% | -94.2% | -94.0% | -47.1% | +0.0% |
| 1 | 2 | 22049ns | -87.3% | -90.1% | -86.5% | -93.5% | -95.4% | -94.6% | -35.8% | +0.0% |
| 1 | 3 | 26904ns | -90.0% | -89.7% | -84.9% | -95.5% | -95.3% | -95.7% | -56.3% | +0.1% |
| 1 | 4 | 26959ns | -89.9% | -88.2% | -80.3% | -95.3% | -95.6% | -95.0% | -47.0% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.253 | moderate+ |
| satfold-lanes16 | -0.265 | moderate- |
| satfold-lanes16-constl | 0.246 | moderate+ |
| satfold-lanes4-idx | 0.239 | moderate+ |
| satfold-lanes64 | -0.542 | HIGH- (thermal bounce) |
| satfold-neon | -0.670 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.270 | moderate- |
| satfold-nolaw | -0.724 | HIGH- (thermal bounce) |
| satfold-seq | -0.421 | moderate- |

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
| satfold-iterfold | 21.1ns | 24495.7ns | 0.1% |  |
| satfold-lanes16 | 76.8ns | 2688.8ns | 2.9% |  |
| satfold-lanes16-constl | 74.0ns | 2612.1ns | 2.8% |  |
| satfold-lanes4-idx | 41.3ns | 3974.5ns | 1.0% |  |
| satfold-lanes64 | 81.9ns | 1306.4ns | 6.3% | HIGH |
| satfold-neon | 72.6ns | 1200.2ns | 6.0% | HIGH |
| satfold-neon8 | 76.6ns | 1267.1ns | 6.0% | HIGH |
| satfold-nolaw | 22.3ns | 13004.7ns | 0.2% |  |
| satfold-seq | 20.0ns | 23260.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22033.0-26980.7 ns)
  22033.0 |#####################################
  22280.3 |
  22527.7 |
  22775.1 |
  23022.5 |
  23269.9 |
  23517.3 |
  23764.7 |
  24012.1 |
  24259.4 |
  24506.8 |
  24754.2 |
  25001.6 |
  25249.0 |
  25496.4 |
  25743.8 |
  25991.2 |
  26238.6 |
  26485.9 |
  26733.3 |########################################
  (2 below, 1 above range)

satfold-lanes16 (n=40, range 2215.4-3128.2 ns)
   2215.4 |######
   2261.1 |####################
   2306.7 |
   2352.3 |######
   2398.0 |#############
   2443.6 |
   2489.3 |######
   2534.9 |
   2580.5 |##########################
   2626.2 |######
   2671.8 |########################################
   2717.5 |#############
   2763.1 |##########################
   2808.7 |######
   2854.4 |#############
   2900.0 |######
   2945.7 |
   2991.3 |
   3037.0 |####################
   3082.6 |#############
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 2083.0-3271.9 ns)
   2083.0 |#############
   2142.5 |
   2201.9 |####################
   2261.4 |########################################
   2320.8 |
   2380.3 |#############
   2439.7 |##########################
   2499.1 |######
   2558.6 |######
   2618.0 |
   2677.5 |######
   2736.9 |######
   2796.4 |#############
   2855.8 |######
   2915.3 |####################
   2974.7 |######
   3034.1 |#############
   3093.6 |
   3153.0 |#############
   3212.5 |######
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 1860.6-7168.1 ns)
   1860.6 |########################################
   2126.0 |
   2391.4 |##
   2656.8 |
   2922.1 |
   3187.5 |##
   3452.9 |
   3718.2 |
   3983.6 |##
   4249.0 |##
   4514.4 |
   4779.7 |
   5045.1 |##
   5310.5 |#######
   5575.8 |
   5841.2 |##########
   6106.6 |#####
   6371.9 |#####
   6637.3 |
   6902.7 |#####
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 1161.9-1484.9 ns)
   1161.9 |###########
   1178.1 |#####
   1194.2 |
   1210.4 |#####
   1226.5 |
   1242.7 |#################
   1258.8 |########################################
   1275.0 |##################################
   1291.1 |
   1307.3 |#####
   1323.4 |#####
   1339.6 |#####
   1355.7 |
   1371.9 |#####
   1388.0 |#####
   1404.2 |############################
   1420.3 |
   1436.5 |
   1452.6 |#####
   1468.8 |#####
  (5 below, 3 above range)

satfold-neon (n=40, range 1011.6-1373.8 ns)
   1011.6 |
   1029.7 |##########
   1047.9 |
   1066.0 |##########
   1084.1 |#####
   1102.2 |
   1120.3 |
   1138.4 |###############
   1156.5 |###################################
   1174.6 |#####
   1192.7 |
   1210.8 |#####
   1228.9 |
   1247.0 |####################
   1265.1 |########################################
   1283.2 |
   1301.3 |#####
   1319.5 |#####
   1337.6 |#####
   1355.7 |
  (5 below, 3 above range)

satfold-neon8 (n=40, range 1132.3-1449.3 ns)
   1132.3 |
   1148.2 |########
   1164.0 |#################
   1179.9 |########
   1195.7 |######################
   1211.6 |####
   1227.4 |
   1243.3 |####
   1259.1 |########################################
   1275.0 |########
   1290.8 |
   1306.7 |
   1322.5 |
   1338.4 |####
   1354.2 |####
   1370.1 |########
   1385.9 |#############
   1401.8 |########
   1417.6 |
   1433.5 |
  (3 below, 2 above range)

satfold-nolaw (n=40, range 11655.6-14309.2 ns)
  11655.6 |########################################
  11788.3 |################
  11921.0 |
  12053.6 |
  12186.3 |
  12319.0 |
  12451.7 |
  12584.4 |
  12717.0 |####
  12849.7 |
  12982.4 |
  13115.1 |
  13247.8 |
  13380.4 |
  13513.1 |
  13645.8 |
  13778.5 |
  13911.1 |
  14043.8 |########################################
  14176.5 |########################
  (5 below, 4 above range)

satfold-seq (n=40, range 22003.9-26937.3 ns)
  22003.9 |########################################
  22250.5 |
  22497.2 |
  22743.9 |
  22990.6 |
  23237.2 |
  23483.9 |
  23730.6 |
  23977.2 |
  24223.9 |
  24470.6 |
  24717.3 |
  24963.9 |
  25210.6 |
  25457.3 |
  25704.0 |
  25950.6 |
  26197.3 |
  26444.0 |
  26690.7 |#########
  (4 below, 4 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=56.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
