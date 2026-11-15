# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (27.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 1.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon beats baseline by 96% (significant)

satfold-neon is -26.21 us (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 27.1x slower than the field

satfold-iterfold (27.28 us) is 27.1x the fastest (1.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16, satfold-lanes4-idx, satfold-lanes16-constl} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (435% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16, satfold-lanes4-idx, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 435% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 27.1x the fastest

Fastest satfold-neon (1.01 us) to slowest satfold-iterfold (27.28 us): 27.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.2x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.87 us but its worst 20% at 7.78 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 1007.3 ns median (-96.3% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 27.09x (fastest 1007.3 ns, slowest 27283.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 26192ns | 27355ns | 22379ns | 27064ns | 27387ns | base |
| satfold-lanes16 | 2413ns | 2462ns | 2029ns | 2407ns | 2817ns | -90.79% |
| satfold-lanes16-constl | 2425ns | 2544ns | 1998ns | 2449ns | 2778ns | -90.74% |
| satfold-lanes4-idx | 4098ns | 2359ns | 1925ns | 3494ns | 8085ns | -84.35% |
| satfold-lanes64 | 1395ns | 1338ns | 1196ns | 1384ns | 1630ns | -94.67% |
| satfold-neon | 1266ns | 1206ns | 1116ns | 1246ns | 1473ns | -95.17% |
| satfold-nolaw | 12468ns | 12453ns | 12420ns | 12462ns | 12531ns | -52.40% |
| satfold-seq | 23621ns | 22370ns | 22367ns | 22795ns | 27357ns | -9.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 26117ns | 22319ns | 27297ns | base | 1.255 |
| satfold-lanes16 | 2172ns | 1838ns | 2515ns | -91.68% | 15.083 |
| satfold-lanes16-constl | 2191ns | 1814ns | 2482ns | -91.61% | 14.953 |
| satfold-lanes4-idx | 3965ns | 1871ns | 7778ns | -84.82% | 8.265 |
| satfold-lanes64 | 1156ns | 998ns | 1343ns | -95.57% | 28.339 |
| satfold-neon | 1043ns | 926ns | 1187ns | -96.01% | 31.417 |
| satfold-nolaw | 12388ns | 12346ns | 12443ns | -52.57% | 2.645 |
| satfold-seq | 23559ns | 22313ns | 27285ns | -9.79% | 1.391 |

## Performance model

- Peak throughput: **35.400 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.201 | 3.4% |
| satfold-lanes16 | 14.694 | 41.5% |
| satfold-lanes16-constl | 14.173 | 40.0% |
| satfold-lanes4-idx | 14.287 | 40.4% |
| satfold-lanes64 | 29.716 | 83.9% |
| satfold-neon | 32.531 | 91.9% |
| satfold-nolaw | 2.647 | 7.5% |
| satfold-seq | 1.468 | 4.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 26192ns | 26192ns | base |
| satfold-lanes16 | 2413ns | 2413ns | -90.79% |
| satfold-lanes16-constl | 2425ns | 2425ns | -90.74% |
| satfold-lanes4-idx | 4098ns | 4098ns | -84.35% |
| satfold-lanes64 | 1395ns | 1395ns | -94.67% |
| satfold-neon | 1266ns | 1266ns | -95.17% |
| satfold-nolaw | 12468ns | 12468ns | -52.40% |
| satfold-seq | 23621ns | 23621ns | -9.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 27283ns | base | --- | [27275, 27284] | --- | --- | --- | --- |
| satfold-lanes16 | 2230ns | -25029.8ns (-91.7%) | [-25109, -24776]ns | [2056, 2254] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2312ns | -24928.0ns (-91.4%) | [-24936, -24860]ns | [2125, 2353] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2294ns | -22297.9ns (-81.7%) | [-24989, -20033]ns | [2287, 4357] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1103ns | -26184.6ns (-96.0%) | [-26199, -26075]ns | [1088, 1214] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1007ns | -26205.8ns (-96.1%) | [-26284, -26084]ns | [999, 1095] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 12379ns | -14880.6ns (-54.5%) | [-14909, -14840]ns | [12375, 12388] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22315ns | -4963.2ns (-18.2%) | [-4970, -4956]ns | [22314, 22324] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 22320ns | -90.0% | -90.3% | -89.7% | -94.1% | -95.3% | -44.5% | +22.2% |
| 1 | 2 | 27285ns | -93.1% | -93.3% | -93.1% | -95.7% | -95.7% | -54.7% | -18.2% |
| 1 | 3 | 27285ns | -91.1% | -91.4% | -89.1% | -96.0% | -96.3% | -54.6% | -18.2% |
| 1 | 4 | 27284ns | -91.8% | -91.4% | -91.6% | -96.3% | -96.4% | -54.4% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.083 | ok |
| satfold-lanes16 | -0.482 | moderate- |
| satfold-lanes16-constl | -0.144 | ok |
| satfold-lanes4-idx | -0.498 | moderate- |
| satfold-lanes64 | 0.180 | ok |
| satfold-neon | -0.099 | ok |
| satfold-nolaw | 0.027 | ok |
| satfold-seq | -0.083 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.0ns | 26117.2ns | 0.1% |  |
| satfold-lanes16 | 83.1ns | 2172.5ns | 3.8% |  |
| satfold-lanes16-constl | 81.0ns | 2191.4ns | 3.7% |  |
| satfold-lanes4-idx | 41.6ns | 3964.9ns | 1.1% |  |
| satfold-lanes64 | 81.0ns | 1156.3ns | 7.0% | HIGH |
| satfold-neon | 74.1ns | 1043.0ns | 7.1% | HIGH |
| satfold-nolaw | 24.5ns | 12388.1ns | 0.2% |  |
| satfold-seq | 20.6ns | 23559.0ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 22318.8-27296.9 ns)
  22318.8 |#####
  22567.7 |
  22816.6 |
  23065.5 |
  23314.4 |
  23563.4 |
  23812.3 |
  24061.2 |
  24310.1 |
  24559.0 |
  24807.9 |
  25056.8 |#
  25305.7 |
  25554.6 |
  25803.5 |
  26052.4 |
  26301.3 |
  26550.2 |
  26799.1 |
  27048.0 |########################################
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 1838.0-2515.3 ns)
   1838.0 |
   1871.9 |#######
   1905.7 |#######
   1939.6 |###
   1973.5 |
   2007.3 |
   2041.2 |#########################
   2075.1 |###
   2108.9 |###
   2142.8 |
   2176.7 |#######
   2210.5 |
   2244.4 |########################################
   2278.3 |###
   2312.1 |##########
   2346.0 |
   2379.9 |
   2413.7 |
   2447.6 |
   2481.5 |###
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 1813.7-2481.9 ns)
   1813.7 |####
   1847.1 |
   1880.5 |
   1913.9 |####
   1947.3 |####################
   1980.7 |####
   2014.2 |
   2047.6 |
   2081.0 |####
   2114.4 |########
   2147.8 |
   2181.2 |############
   2214.6 |
   2248.0 |####
   2281.5 |
   2314.9 |############
   2348.3 |########################################
   2381.7 |####
   2415.1 |############
   2448.5 |
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 1871.0-7778.3 ns)
   1871.0 |#######################
   2166.4 |########################################
   2461.8 |
   2757.1 |
   3052.5 |###
   3347.8 |
   3643.2 |###
   3938.6 |###
   4233.9 |
   4529.3 |###
   4824.6 |
   5120.0 |
   5415.4 |
   5710.7 |###
   6006.1 |
   6301.5 |######
   6596.8 |###
   6892.2 |######
   7187.6 |######
   7482.9 |######
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 997.6-1342.8 ns)
    997.6 |########################################
   1014.9 |#####
   1032.1 |#####
   1049.4 |
   1066.7 |
   1083.9 |########################################
   1101.2 |##########
   1118.4 |
   1135.7 |#####
   1153.0 |
   1170.2 |##########
   1187.5 |
   1204.8 |####################
   1222.0 |#####
   1239.3 |
   1256.5 |#####
   1273.8 |
   1291.1 |
   1308.3 |####################
   1325.6 |##########
  (1 below, 4 above range)

satfold-neon (n=40, range 925.7-1186.7 ns)
    925.7 |####
    938.7 |
    951.8 |####
    964.8 |
    977.9 |
    990.9 |########################################
   1004.0 |####################
   1017.0 |####
   1030.1 |
   1043.1 |
   1056.2 |
   1069.2 |####
   1082.3 |####
   1095.3 |################################
   1108.4 |
   1121.4 |
   1134.5 |####
   1147.5 |
   1160.6 |
   1173.6 |
  (6 below, 5 above range)

satfold-nolaw (n=40, range 12346.2-12443.3 ns)
  12346.2 |#############
  12351.0 |####################
  12355.9 |######
  12360.7 |######
  12365.6 |
  12370.5 |#################################
  12375.3 |########################################
  12380.2 |####################
  12385.0 |##########################
  12389.9 |#############
  12394.7 |
  12399.6 |
  12404.5 |
  12409.3 |
  12414.2 |
  12419.0 |
  12423.9 |
  12428.7 |######
  12433.6 |##########################
  12438.5 |####################
  (3 below, 2 above range)

satfold-seq (n=40, range 22312.7-27285.4 ns)
  22312.7 |########################################
  22561.3 |
  22810.0 |
  23058.6 |
  23307.2 |
  23555.9 |
  23804.5 |
  24053.1 |
  24301.8 |
  24550.4 |
  24799.0 |
  25047.7 |
  25296.3 |
  25544.9 |
  25793.6 |
  26042.2 |
  26290.8 |
  26539.5 |
  26788.1 |
  27036.7 |##########
  (3 below, 3 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=61.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.2x (possible bimodal distribution)
