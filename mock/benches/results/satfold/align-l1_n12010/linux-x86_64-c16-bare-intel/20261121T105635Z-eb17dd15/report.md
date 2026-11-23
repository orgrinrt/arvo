# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (24.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 1.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon beats baseline by 95% (significant)

satfold-neon is -23.70 us (95%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 22.5x slower than the field

satfold-iterfold (24.88 us) is 22.5x the fastest (1.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (426% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 426% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.5x the fastest

Fastest satfold-neon (1.11 us) to slowest satfold-iterfold (24.88 us): 22.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.92 us but its worst 20% at 7.62 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 1105.4 ns median (-95.6% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 22.51x (fastest 1105.4 ns, slowest 24881.6 ns)

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
| Counter frequency | 2112835759 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 24880ns | 24948ns | 22374ns | 24879ns | 27392ns | base |
| satfold-lanes16 | 2446ns | 2491ns | 1983ns | 2436ns | 2939ns | -90.17% |
| satfold-lanes16-constl | 2494ns | 2488ns | 2068ns | 2473ns | 2982ns | -89.98% |
| satfold-lanes4-idx | 4221ns | 2421ns | 1969ns | 3756ns | 7869ns | -83.03% |
| satfold-lanes64 | 1485ns | 1384ns | 1167ns | 1407ns | 2041ns | -94.03% |
| satfold-neon | 1411ns | 1318ns | 1219ns | 1365ns | 1739ns | -94.33% |
| satfold-nolaw | 11934ns | 12483ns | 10253ns | 12296ns | 12528ns | -52.04% |
| satfold-seq | 23628ns | 22382ns | 22373ns | 22801ns | 27366ns | -5.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24810ns | 22316ns | 27308ns | base | 1.321 |
| satfold-lanes16 | 2202ns | 1791ns | 2627ns | -91.12% | 14.881 |
| satfold-lanes16-constl | 2247ns | 1873ns | 2672ns | -90.94% | 14.584 |
| satfold-lanes4-idx | 4097ns | 1916ns | 7619ns | -83.49% | 7.998 |
| satfold-lanes64 | 1234ns | 974ns | 1686ns | -95.02% | 26.545 |
| satfold-neon | 1190ns | 1037ns | 1465ns | -95.20% | 27.530 |
| satfold-nolaw | 11859ns | 10193ns | 12441ns | -52.20% | 2.763 |
| satfold-seq | 23561ns | 22314ns | 27288ns | -5.03% | 1.391 |

## Performance model

- Peak throughput: **33.628 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.317 | 3.9% |
| satfold-lanes16 | 14.508 | 43.1% |
| satfold-lanes16-constl | 14.556 | 43.3% |
| satfold-lanes4-idx | 13.908 | 41.4% |
| satfold-lanes64 | 28.246 | 84.0% |
| satfold-neon | 29.642 | 88.1% |
| satfold-nolaw | 2.642 | 7.9% |
| satfold-seq | 1.468 | 4.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24880ns | 24880ns | base |
| satfold-lanes16 | 2446ns | 2446ns | -90.17% |
| satfold-lanes16-constl | 2494ns | 2494ns | -89.98% |
| satfold-lanes4-idx | 4221ns | 4221ns | -83.03% |
| satfold-lanes64 | 1485ns | 1485ns | -94.03% |
| satfold-neon | 1411ns | 1411ns | -94.33% |
| satfold-nolaw | 11934ns | 11934ns | -52.04% |
| satfold-seq | 23628ns | 23628ns | -5.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 24882ns | base | --- | [22324, 27285] | --- | --- | --- | --- |
| satfold-lanes16 | 2259ns | -22543.1ns (-90.6%) | [-24985, -20514]ns | [2079, 2266] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2251ns | -22518.6ns (-90.5%) | [-24937, -20411]ns | [2117, 2272] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2356ns | -20401.0ns (-82.0%) | [-21437, -19968]ns | [2341, 5617] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1160ns | -23660.6ns (-95.1%) | [-26189, -21109]ns | [1093, 1221] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1105ns | -23702.7ns (-95.3%) | [-26175, -21197]ns | [1103, 1189] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12404ns | -12458.5ns (-50.1%) | [-14850, -9933]ns | [12384, 12416] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22322ns | no significant difference | [-9, +0]ns | [22318, 22326] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 27286ns | -91.6% | -92.0% | -91.4% | -96.0% | -96.0% | -54.4% | +0.0% |
| 1 | 2 | 22318ns | -89.4% | -91.5% | -91.4% | -92.8% | -93.5% | -44.5% | -0.0% |
| 1 | 3 | 22318ns | -91.9% | -89.1% | -86.6% | -94.5% | -95.0% | -44.4% | +0.0% |
| 1 | 4 | 27286ns | -91.7% | -91.4% | -85.3% | -96.3% | -96.0% | -62.6% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.250 | moderate- |
| satfold-lanes16 | -0.419 | moderate- |
| satfold-lanes16-constl | -0.156 | ok |
| satfold-lanes4-idx | 0.189 | ok |
| satfold-lanes64 | -0.243 | moderate- |
| satfold-neon | -0.418 | moderate- |
| satfold-nolaw | -0.088 | ok |
| satfold-seq | -0.084 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.3ns | 24810.2ns | 0.1% |  |
| satfold-lanes16 | 80.8ns | 2202.0ns | 3.7% |  |
| satfold-lanes16-constl | 84.7ns | 2246.8ns | 3.8% |  |
| satfold-lanes4-idx | 42.1ns | 4097.3ns | 1.0% |  |
| satfold-lanes64 | 84.7ns | 1234.4ns | 6.9% | HIGH |
| satfold-neon | 75.8ns | 1190.3ns | 6.4% | HIGH |
| satfold-nolaw | 22.5ns | 11859.1ns | 0.2% |  |
| satfold-seq | 20.2ns | 23561.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22315.7-27307.8 ns)
  22315.7 |###############################
  22565.3 |
  22814.9 |
  23064.5 |
  23314.1 |
  23563.7 |
  23813.3 |
  24062.9 |
  24312.5 |
  24562.1 |
  24811.7 |
  25061.3 |
  25310.9 |
  25560.5 |
  25810.1 |
  26059.7 |
  26309.3 |
  26558.9 |
  26808.5 |
  27058.1 |########################################
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 1791.3-2627.0 ns)
   1791.3 |####
   1833.1 |
   1874.9 |########
   1916.7 |####
   1958.5 |####
   2000.2 |
   2042.0 |#################
   2083.8 |########
   2125.6 |####
   2167.4 |
   2209.1 |####
   2250.9 |########################################
   2292.7 |#################
   2334.5 |
   2376.3 |####
   2418.1 |
   2459.8 |####
   2501.6 |#############
   2543.4 |
   2585.2 |####
  (5 below, 3 above range)

satfold-lanes16-constl (n=40, range 1873.2-2671.5 ns)
   1873.2 |##############
   1913.1 |###
   1953.1 |
   1993.0 |###
   2032.9 |##############
   2072.8 |#######
   2112.7 |###
   2152.6 |
   2192.5 |
   2232.4 |########################################
   2272.4 |###
   2312.3 |##########
   2352.2 |###
   2392.1 |
   2432.0 |
   2471.9 |
   2511.8 |##########
   2551.8 |#######
   2591.7 |
   2631.6 |###
  (2 below, 3 above range)

satfold-lanes4-idx (n=40, range 1915.5-7618.8 ns)
   1915.5 |####################
   2200.7 |########################################
   2485.9 |
   2771.0 |
   3056.2 |
   3341.3 |###
   3626.5 |
   3911.7 |
   4196.8 |###
   4482.0 |###
   4767.1 |
   5052.3 |
   5337.5 |###
   5622.6 |###
   5907.8 |######
   6193.0 |###
   6478.1 |######
   6763.3 |###
   7048.5 |######
   7333.6 |######
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 974.4-1686.4 ns)
    974.4 |########################################
   1010.0 |#####
   1045.6 |#####
   1081.2 |##############################
   1116.8 |
   1152.4 |#########################
   1188.0 |####################
   1223.6 |
   1259.2 |#####
   1294.8 |
   1330.4 |#####
   1366.0 |
   1401.6 |
   1437.2 |
   1472.8 |
   1508.4 |#########################
   1544.0 |
   1579.6 |
   1615.2 |
   1650.8 |#####
  (3 below, 4 above range)

satfold-neon (n=40, range 1036.6-1465.5 ns)
   1036.6 |
   1058.1 |######
   1079.5 |######
   1101.0 |########################################
   1122.4 |###
   1143.8 |######
   1165.3 |###
   1186.7 |#########
   1208.2 |
   1229.6 |
   1251.0 |###
   1272.5 |
   1293.9 |
   1315.4 |#########
   1336.8 |
   1358.2 |
   1379.7 |###
   1401.1 |
   1422.6 |
   1444.0 |###
  (5 below, 5 above range)

satfold-nolaw (n=40, range 10193.1-12440.5 ns)
  10193.1 |########
  10305.4 |
  10417.8 |
  10530.2 |
  10642.6 |
  10754.9 |
  10867.3 |
  10979.7 |
  11092.1 |
  11204.4 |
  11316.8 |
  11429.2 |
  11541.6 |
  11653.9 |
  11766.3 |
  11878.7 |
  11991.1 |
  12103.4 |
  12215.8 |
  12328.2 |########################################
  (4 below, 3 above range)

satfold-seq (n=40, range 22314.2-27288.4 ns)
  22314.2 |########################################
  22562.9 |
  22811.6 |
  23060.3 |
  23309.0 |
  23557.7 |
  23806.4 |
  24055.1 |
  24303.9 |
  24552.6 |
  24801.3 |
  25050.0 |
  25298.7 |
  25547.4 |
  25796.1 |
  26044.8 |
  26293.5 |
  26542.2 |
  26790.9 |
  27039.7 |#######
  (3 below, 5 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=56.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=21.8% (high variance, measurements may be unstable)
