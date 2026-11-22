# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 beats baseline by 96% (significant)

satfold-lanes64 is -23.70 us (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 25.0x slower than the field

satfold-seq (27.28 us) is 25.0x the fastest (1.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-lanes64, satfold-neon) are a dead heat (<1%)

satfold-lanes64 (1.09 us) and satfold-neon (1.09 us) differ by 0.05%, inside the noise, even though the wider field spreads 2395.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.53)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (438% apart)

The field splits into a fast tier {satfold-lanes64, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 438% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 25.0x the fastest

Fastest satfold-lanes64 (1.09 us) to slowest satfold-seq (27.28 us): 25.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.1x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.87 us but its worst 20% at 7.59 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 1093.3 ns median (-95.6% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 24.95x (fastest 1093.3 ns, slowest 27281.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112779582 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 24874ns | 24889ns | 22374ns | 24873ns | 27378ns | base |
| satfold-lanes16 | 2408ns | 2497ns | 1771ns | 2429ns | 2984ns | -90.32% |
| satfold-lanes16-constl | 2351ns | 2355ns | 2033ns | 2348ns | 2677ns | -90.55% |
| satfold-lanes4-idx | 4137ns | 2367ns | 1928ns | 3631ns | 7865ns | -83.37% |
| satfold-lanes64 | 1320ns | 1307ns | 1215ns | 1313ns | 1448ns | -94.69% |
| satfold-neon | 1329ns | 1313ns | 1113ns | 1332ns | 1536ns | -94.66% |
| satfold-nolaw | 12457ns | 12445ns | 12418ns | 12453ns | 12508ns | -49.92% |
| satfold-seq | 26108ns | 27348ns | 22371ns | 26933ns | 27368ns | +4.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24803ns | 22315ns | 27292ns | base | 1.321 |
| satfold-lanes16 | 2161ns | 1602ns | 2650ns | -91.29% | 15.162 |
| satfold-lanes16-constl | 2121ns | 1846ns | 2375ns | -91.45% | 15.451 |
| satfold-lanes4-idx | 4010ns | 1873ns | 7590ns | -83.83% | 8.171 |
| satfold-lanes64 | 1095ns | 1012ns | 1175ns | -95.59% | 29.926 |
| satfold-neon | 1101ns | 928ns | 1262ns | -95.56% | 29.770 |
| satfold-nolaw | 12380ns | 12343ns | 12430ns | -50.09% | 2.647 |
| satfold-seq | 26040ns | 22315ns | 27286ns | +4.98% | 1.258 |

## Performance model

- Peak throughput: **35.321 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.321 | 3.7% |
| satfold-lanes16 | 14.520 | 41.1% |
| satfold-lanes16-constl | 15.345 | 43.4% |
| satfold-lanes4-idx | 14.239 | 40.3% |
| satfold-lanes64 | 29.972 | 84.9% |
| satfold-neon | 29.955 | 84.8% |
| satfold-nolaw | 2.649 | 7.5% |
| satfold-seq | 1.201 | 3.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24874ns | 24874ns | base |
| satfold-lanes16 | 2408ns | 2408ns | -90.32% |
| satfold-lanes16-constl | 2351ns | 2351ns | -90.55% |
| satfold-lanes4-idx | 4137ns | 4137ns | -83.37% |
| satfold-lanes64 | 1320ns | 1320ns | -94.69% |
| satfold-neon | 1329ns | 1329ns | -94.66% |
| satfold-nolaw | 12457ns | 12457ns | -49.92% |
| satfold-seq | 26108ns | 26108ns | +4.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 24803ns | base | --- | [22326, 27283] | --- | --- | --- | --- |
| satfold-lanes16 | 2257ns | -22571.7ns (-91.0%) | [-24773, -20688]ns | [2057, 2294] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2135ns | -22602.5ns (-91.1%) | [-24936, -20281]ns | [2017, 2191] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2301ns | -20443.9ns (-82.4%) | [-21404, -19861]ns | [2287, 5773] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1093ns | -23703.8ns (-95.6%) | [-26179, -21236]ns | [1091, 1104] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1094ns | -23670.2ns (-95.4%) | [-26138, -21225]ns | [1072, 1181] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12372ns | -12409.4ns (-50.0%) | [-14858, -9970]ns | [12365, 12379] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 27281ns | no significant difference | [-3, +3]ns | [27272, 27283] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 22324ns | -92.8% | -90.4% | -81.8% | -95.1% | -94.5% | -44.6% | +22.2% |
| 1 | 2 | 22318ns | -90.7% | -90.4% | -90.3% | -95.1% | -95.1% | -44.7% | -0.0% |
| 1 | 3 | 27285ns | -90.8% | -93.2% | -91.6% | -95.9% | -96.6% | -54.7% | -0.0% |
| 1 | 4 | 27285ns | -91.7% | -91.4% | -91.6% | -96.0% | -95.6% | -54.5% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.250 | moderate+ |
| satfold-lanes16 | 0.144 | ok |
| satfold-lanes16-constl | -0.527 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.135 | ok |
| satfold-lanes64 | -0.172 | ok |
| satfold-neon | -0.235 | moderate- |
| satfold-nolaw | 0.089 | ok |
| satfold-seq | -0.417 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.9ns | 24803.5ns | 0.1% |  |
| satfold-lanes16 | 81.6ns | 2161.2ns | 3.8% |  |
| satfold-lanes16-constl | 77.9ns | 2120.8ns | 3.7% |  |
| satfold-lanes4-idx | 42.3ns | 4010.3ns | 1.1% |  |
| satfold-lanes64 | 77.9ns | 1095.0ns | 7.1% | HIGH |
| satfold-neon | 77.5ns | 1100.7ns | 7.0% | HIGH |
| satfold-nolaw | 24.3ns | 12380.1ns | 0.2% |  |
| satfold-seq | 22.7ns | 26039.5ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22315.0-27292.1 ns)
  22315.0 |###################################
  22563.8 |
  22812.7 |
  23061.5 |
  23310.4 |
  23559.2 |
  23808.1 |
  24057.0 |
  24305.8 |
  24554.7 |
  24803.5 |
  25052.4 |
  25301.2 |
  25550.1 |
  25799.0 |
  26047.8 |
  26296.7 |
  26545.5 |
  26794.4 |
  27043.2 |########################################
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 1601.7-2650.0 ns)
   1601.7 |#########################
   1654.1 |#####
   1706.5 |#####
   1759.0 |#####
   1811.4 |
   1863.8 |##########
   1916.2 |
   1968.6 |
   2021.0 |###############
   2073.4 |#####
   2125.8 |#####
   2178.2 |#####
   2230.7 |########################################
   2283.1 |##########
   2335.5 |#####
   2387.9 |
   2440.3 |#####
   2492.7 |#########################
   2545.1 |##########
   2597.5 |
  (2 below, 3 above range)

satfold-lanes16-constl (n=40, range 1845.9-2374.7 ns)
   1845.9 |
   1872.3 |#####
   1898.7 |
   1925.2 |
   1951.6 |###################################
   1978.1 |
   2004.5 |##########
   2030.9 |##########
   2057.4 |#####
   2083.8 |
   2110.3 |###############
   2136.7 |###############
   2163.2 |##########
   2189.6 |#####
   2216.0 |
   2242.5 |
   2268.9 |#####
   2295.4 |
   2321.8 |##########
   2348.2 |########################################
  (5 below, 2 above range)

satfold-lanes4-idx (n=40, range 1873.4-7589.7 ns)
   1873.4 |#####
   2159.2 |########################################
   2445.0 |##
   2730.8 |
   3016.6 |
   3302.5 |
   3588.3 |
   3874.1 |
   4159.9 |
   4445.7 |
   4731.5 |
   5017.4 |
   5303.2 |##
   5589.0 |##
   5874.8 |########
   6160.6 |########
   6446.4 |##
   6732.3 |#####
   7018.1 |
   7303.9 |##
  (7 below, 4 above range)

satfold-lanes64 (n=40, range 1012.4-1175.4 ns)
   1012.4 |
   1020.6 |
   1028.7 |
   1036.9 |##
   1045.0 |
   1053.2 |
   1061.3 |
   1069.5 |##
   1077.6 |##
   1085.8 |########################################
   1093.9 |#####
   1102.1 |#################
   1110.2 |##
   1118.4 |
   1126.5 |##
   1134.7 |##
   1142.8 |
   1151.0 |
   1159.1 |
   1167.3 |########
  (6 below, 3 above range)

satfold-neon (n=40, range 927.7-1261.9 ns)
    927.7 |
    944.4 |#####
    961.1 |###########
    977.9 |
    994.6 |#################
   1011.3 |
   1028.0 |
   1044.7 |#####
   1061.4 |
   1078.1 |########################################
   1094.8 |###########
   1111.5 |#####
   1128.2 |
   1145.0 |
   1161.7 |###########
   1178.4 |#####
   1195.1 |######################
   1211.8 |###########
   1228.5 |#################
   1245.2 |#####
  (7 below, 3 above range)

satfold-nolaw (n=40, range 12342.9-12429.9 ns)
  12342.9 |####################
  12347.3 |#####
  12351.6 |##########
  12356.0 |
  12360.3 |###############
  12364.7 |###############
  12369.0 |########################################
  12373.4 |##########
  12377.7 |#####
  12382.1 |##########
  12386.4 |
  12390.8 |
  12395.1 |
  12399.5 |
  12403.8 |
  12408.2 |
  12412.5 |###############
  12416.9 |##########
  12421.2 |#####
  12425.6 |##########
  (3 below, 3 above range)

satfold-seq (n=40, range 22315.5-27285.6 ns)
  22315.5 |#####
  22564.0 |
  22812.5 |
  23061.0 |
  23309.5 |
  23558.0 |
  23806.5 |
  24055.0 |
  24303.5 |
  24552.0 |
  24800.5 |
  25049.1 |
  25297.6 |
  25546.1 |
  25794.6 |
  26043.1 |
  26291.6 |
  26540.1 |
  26788.6 |
  27037.1 |########################################
  (6 below, 3 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=59.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.1x (possible bimodal distribution)
