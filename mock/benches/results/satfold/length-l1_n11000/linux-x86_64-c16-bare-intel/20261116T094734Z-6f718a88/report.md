# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 95% (significant)

satfold-neon is -20.82 us (95%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 18.2x slower than the field

satfold-seq (22.02 us) is 18.2x the fastest (1.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon8, satfold-neon) are a dead heat (<1%)

satfold-neon8 (1.21 us) and satfold-neon (1.21 us) differ by 0.54%, inside the noise, even though the wider field spreads 1723.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-nolaw shows alternating (throttle bounce) (autocorr -0.74)

satfold-nolaw's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (305% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 305% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.2x the fastest

Fastest satfold-neon8 (1.21 us) to slowest satfold-seq (22.02 us): 18.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.1x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.87 us but its worst 20% at 7.65 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (15 ns, 0.07%)

satfold-seq differs from baseline satfold-iterfold by 15 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 1207.8 ns median (-94.5% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 18.23x (fastest 1207.8 ns, slowest 22023.1 ns)

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
| satfold-iterfold | 23284ns | 22054ns | 22047ns | 22467ns | 26969ns | base |
| satfold-lanes16 | 2689ns | 2682ns | 2052ns | 2675ns | 3366ns | -88.45% |
| satfold-lanes16-constl | 3459ns | 3415ns | 2950ns | 3399ns | 4151ns | -85.14% |
| satfold-lanes4-idx | 4168ns | 2374ns | 1920ns | 3666ns | 7924ns | -82.10% |
| satfold-lanes64 | 1571ns | 1524ns | 1314ns | 1552ns | 1886ns | -93.25% |
| satfold-neon | 1452ns | 1411ns | 1298ns | 1427ns | 1680ns | -93.77% |
| satfold-neon8 | 1444ns | 1404ns | 1298ns | 1416ns | 1672ns | -93.80% |
| satfold-nolaw | 12976ns | 13112ns | 11586ns | 12977ns | 14364ns | -44.27% |
| satfold-seq | 23314ns | 22088ns | 22061ns | 22495ns | 27025ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 23220ns | 21990ns | 26897ns | base | 1.411 |
| satfold-lanes16 | 2438ns | 1865ns | 3056ns | -89.50% | 13.439 |
| satfold-lanes16-constl | 3245ns | 2778ns | 3862ns | -86.02% | 10.098 |
| satfold-lanes4-idx | 4039ns | 1866ns | 7653ns | -82.60% | 8.112 |
| satfold-lanes64 | 1332ns | 1122ns | 1587ns | -94.26% | 24.597 |
| satfold-neon | 1233ns | 1111ns | 1398ns | -94.69% | 26.570 |
| satfold-neon8 | 1221ns | 1108ns | 1358ns | -94.74% | 26.847 |
| satfold-nolaw | 12905ns | 11522ns | 14285ns | -44.42% | 2.539 |
| satfold-seq | 23247ns | 22001ns | 26947ns | +0.11% | 1.410 |

## Performance model

- Peak throughput: **29.566 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.490 | 5.0% |
| satfold-lanes16 | 13.394 | 45.3% |
| satfold-lanes16-constl | 10.185 | 34.4% |
| satfold-lanes4-idx | 14.200 | 48.0% |
| satfold-lanes64 | 25.274 | 85.5% |
| satfold-neon | 26.985 | 91.3% |
| satfold-neon8 | 27.131 | 91.8% |
| satfold-nolaw | 2.512 | 8.5% |
| satfold-seq | 1.488 | 5.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 23284ns | 23284ns | base |
| satfold-lanes16 | 2689ns | 2689ns | -88.45% |
| satfold-lanes16-constl | 3459ns | 3459ns | -85.14% |
| satfold-lanes4-idx | 4168ns | 4168ns | -82.10% |
| satfold-lanes64 | 1571ns | 1571ns | -93.25% |
| satfold-neon | 1452ns | 1452ns | -93.77% |
| satfold-neon8 | 1444ns | 1444ns | -93.80% |
| satfold-nolaw | 12976ns | 12976ns | -44.27% |
| satfold-seq | 23314ns | 23314ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 21996ns | base | --- | [21994, 22004] | --- | --- | --- | --- |
| satfold-lanes16 | 2446ns | -19553.3ns (-88.9%) | [-19677, -19477]ns | [2350, 2521] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3217ns | -19039.4ns (-86.6%) | [-19210, -18961]ns | [2989, 3383] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2308ns | -19698.4ns (-89.6%) | [-20116, -18193]ns | [2295, 5433] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1296ns | -20804.4ns (-94.6%) | [-20851, -20725]ns | [1261, 1397] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1214ns | -20822.1ns (-94.7%) | [-20837, -20734]ns | [1171, 1264] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1208ns | -20821.6ns (-94.7%) | [-20846, -20751]ns | [1171, 1250] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 13045ns | -10440.6ns (-47.5%) | [-10478, -10435]ns | [11558, 14230] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22023ns | +14.9ns (+0.1%) | [+6, +40]ns | [22010, 22037] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 21996ns | -88.0% | -85.4% | -91.4% | -94.7% | -94.7% | -94.3% | -47.6% | +0.1% |
| 1 | 2 | 26893ns | -93.1% | -85.9% | -91.4% | -94.2% | -95.7% | -95.6% | -47.1% | -18.2% |
| 1 | 3 | 21993ns | -88.0% | -86.4% | -81.0% | -94.1% | -94.3% | -94.4% | -47.5% | +22.4% |
| 1 | 4 | 21994ns | -88.8% | -86.5% | -85.3% | -94.1% | -93.8% | -94.7% | -35.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.416 | moderate- |
| satfold-lanes16 | -0.602 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.182 | ok |
| satfold-lanes4-idx | 0.086 | ok |
| satfold-lanes64 | -0.540 | HIGH- (thermal bounce) |
| satfold-neon | 0.252 | moderate+ |
| satfold-neon8 | -0.534 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.743 | HIGH- (thermal bounce) |
| satfold-seq | -0.415 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.4ns | 23220.3ns | 0.1% |  |
| satfold-lanes16 | 82.6ns | 2438.4ns | 3.4% |  |
| satfold-lanes16-constl | 71.5ns | 3245.1ns | 2.2% |  |
| satfold-lanes4-idx | 42.4ns | 4039.3ns | 1.0% |  |
| satfold-lanes64 | 84.1ns | 1332.2ns | 6.3% | HIGH |
| satfold-neon | 76.2ns | 1233.3ns | 6.2% | HIGH |
| satfold-neon8 | 74.5ns | 1220.5ns | 6.1% | HIGH |
| satfold-nolaw | 22.1ns | 12905.0ns | 0.2% |  |
| satfold-seq | 20.2ns | 23246.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 21990.4-26896.6 ns)
  21990.4 |########################################
  22235.8 |
  22481.1 |
  22726.4 |
  22971.7 |
  23217.0 |
  23462.3 |
  23707.6 |
  23952.9 |
  24198.2 |
  24443.5 |
  24688.8 |
  24934.1 |
  25179.4 |
  25424.7 |
  25670.1 |
  25915.4 |
  26160.7 |
  26406.0 |
  26651.3 |##########
  (2 below, 3 above range)

satfold-lanes16 (n=40, range 1865.2-3055.8 ns)
   1865.2 |########
   1924.7 |########
   1984.2 |
   2043.8 |################
   2103.3 |
   2162.8 |################
   2222.4 |########
   2281.9 |########
   2341.4 |########################################
   2401.0 |################################
   2460.5 |################################
   2520.0 |################
   2579.5 |################
   2639.1 |########
   2698.6 |########
   2758.1 |################
   2817.7 |
   2877.2 |
   2936.7 |
   2996.2 |################
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 2778.2-3861.7 ns)
   2778.2 |########################
   2832.4 |########################
   2886.6 |########
   2940.8 |########################################
   2994.9 |########################
   3049.1 |
   3103.3 |
   3157.5 |########
   3211.6 |########################################
   3265.8 |
   3320.0 |################
   3374.2 |################
   3428.3 |
   3482.5 |########################
   3536.7 |################
   3590.8 |
   3645.0 |
   3699.2 |################
   3753.4 |
   3807.5 |########
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1865.5-7652.6 ns)
   1865.5 |####################
   2154.9 |########################################
   2444.2 |
   2733.6 |###
   3022.9 |
   3312.3 |
   3601.7 |
   3891.0 |###
   4180.4 |
   4469.7 |
   4759.1 |###
   5048.4 |
   5337.8 |######
   5627.1 |###
   5916.5 |###
   6205.8 |######
   6495.2 |
   6784.5 |######
   7073.9 |######
   7363.2 |###
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 1122.1-1587.0 ns)
   1122.1 |
   1145.4 |########################################
   1168.6 |######
   1191.9 |######
   1215.1 |
   1238.3 |##########################
   1261.6 |##########################
   1284.8 |####################
   1308.1 |######
   1331.3 |
   1354.6 |
   1377.8 |####################
   1401.1 |##########################
   1424.3 |
   1447.5 |######
   1470.8 |#############
   1494.0 |
   1517.3 |
   1540.5 |##########################
   1563.8 |######
  (3 below, 2 above range)

satfold-neon (n=40, range 1111.1-1398.2 ns)
   1111.1 |
   1125.5 |###
   1139.8 |
   1154.2 |###
   1168.5 |########################################
   1182.9 |
   1197.2 |###
   1211.6 |###
   1225.9 |
   1240.3 |
   1254.7 |##########################
   1269.0 |######
   1283.4 |###
   1297.7 |
   1312.1 |######
   1326.4 |
   1340.8 |
   1355.1 |
   1369.5 |
   1383.8 |##########
  (5 below, 3 above range)

satfold-neon8 (n=40, range 1108.3-1357.5 ns)
   1108.3 |
   1120.8 |
   1133.2 |######
   1145.7 |##########################
   1158.1 |########################################
   1170.6 |#############
   1183.1 |#############
   1195.5 |######
   1208.0 |######
   1220.4 |
   1232.9 |#################################
   1245.4 |####################
   1257.8 |#############
   1270.3 |####################
   1282.7 |
   1295.2 |
   1307.7 |
   1320.1 |
   1332.6 |######
   1345.1 |#############
  (4 below, 3 above range)

satfold-nolaw (n=40, range 11521.7-14285.0 ns)
  11521.7 |##############################
  11659.9 |
  11798.0 |##
  11936.2 |
  12074.4 |
  12212.5 |
  12350.7 |
  12488.9 |
  12627.0 |
  12765.2 |
  12903.3 |
  13041.5 |
  13179.7 |
  13317.8 |
  13456.0 |
  13594.2 |
  13732.3 |
  13870.5 |
  14008.7 |
  14146.8 |########################################
  (6 below, 3 above range)

satfold-seq (n=40, range 22001.5-26946.8 ns)
  22001.5 |########################################
  22248.7 |
  22496.0 |
  22743.3 |
  22990.5 |
  23237.8 |
  23485.1 |
  23732.3 |
  23979.6 |
  24226.8 |
  24474.1 |
  24721.4 |
  24968.6 |
  25215.9 |
  25463.2 |
  25710.4 |
  25957.7 |
  26205.0 |
  26452.2 |
  26699.5 |#########
  (4 below, 4 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: CV=58.1% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.1x (possible bimodal distribution)
