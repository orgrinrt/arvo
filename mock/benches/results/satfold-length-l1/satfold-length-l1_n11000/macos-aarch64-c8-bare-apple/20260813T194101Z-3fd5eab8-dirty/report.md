# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 dominates: 33% faster than the next best (satfold-neon)

satfold-neon8 (232 ns) leads satfold-neon (310 ns) by 33%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon8 beats baseline by 100% (significant)

satfold-neon8 is -41.18 us (100%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 177.8x slower than the field

satfold-seq (41.34 us) is 177.8x the fastest (232 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.80)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64} vs {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} (1091% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64} and a slow tier {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} with a 1091% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 177.8x the fastest

Fastest satfold-neon8 (232 ns) to slowest satfold-seq (41.34 us): 177.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-seq's edge over baseline is significant but tiny (-26 ns, 0.06%)

satfold-seq differs from baseline satfold-iterfold by -26 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 232.5 ns median (-99.4% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 177.81x (fastest 232.5 ns, slowest 41341.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 41635ns | 41327ns | 40988ns | 41474ns | 42765ns | base |
| satfold-lanes16 | 932ns | 925ns | 911ns | 929ns | 961ns | -97.76% |
| satfold-lanes16-constl | 950ns | 926ns | 914ns | 929ns | 1051ns | -97.72% |
| satfold-lanes4-idx | 13287ns | 13211ns | 13067ns | 13215ns | 13721ns | -68.09% |
| satfold-lanes64 | 1169ns | 1155ns | 1143ns | 1161ns | 1217ns | -97.19% |
| satfold-neon | 388ns | 369ns | 354ns | 370ns | 479ns | -99.07% |
| satfold-neon8 | 290ns | 290ns | 286ns | 290ns | 295ns | -99.30% |
| satfold-nolaw | 31829ns | 31697ns | 31439ns | 31756ns | 32437ns | -23.55% |
| satfold-seq | 41561ns | 41442ns | 41000ns | 41473ns | 42385ns | -0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 41513ns | 40877ns | 42616ns | base | 0.789 |
| satfold-lanes16 | 839ns | 820ns | 865ns | -97.98% | 39.040 |
| satfold-lanes16-constl | 857ns | 824ns | 951ns | -97.93% | 38.221 |
| satfold-lanes4-idx | 13157ns | 12943ns | 13578ns | -68.31% | 2.491 |
| satfold-lanes64 | 1110ns | 1085ns | 1156ns | -97.33% | 29.524 |
| satfold-neon | 328ns | 299ns | 407ns | -99.21% | 99.850 |
| satfold-neon8 | 232ns | 230ns | 235ns | -99.44% | 140.963 |
| satfold-nolaw | 31743ns | 31370ns | 32336ns | -23.53% | 1.032 |
| satfold-seq | 41449ns | 40893ns | 42274ns | -0.15% | 0.791 |

## Performance model

- Peak throughput: **142.508 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.796 | 0.6% |
| satfold-lanes16 | 39.290 | 27.6% |
| satfold-lanes16-constl | 39.187 | 27.5% |
| satfold-lanes4-idx | 2.506 | 1.8% |
| satfold-lanes64 | 29.857 | 21.0% |
| satfold-neon | 105.857 | 74.3% |
| satfold-neon8 | 140.938 | 98.9% |
| satfold-nolaw | 1.036 | 0.7% |
| satfold-seq | 0.793 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 41635ns | 41635ns | base |
| satfold-lanes16 | 932ns | 932ns | -97.76% |
| satfold-lanes16-constl | 950ns | 950ns | -97.72% |
| satfold-lanes4-idx | 13287ns | 13287ns | -68.09% |
| satfold-lanes64 | 1169ns | 1169ns | -97.19% |
| satfold-neon | 388ns | 388ns | -99.07% |
| satfold-neon8 | 290ns | 290ns | -99.30% |
| satfold-nolaw | 31829ns | 31829ns | -23.55% |
| satfold-seq | 41561ns | 41561ns | -0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 41170ns | base | --- | [41023, 41664] | --- | --- | --- | --- |
| satfold-lanes16 | 834ns | -40341.9ns (-98.0%) | [-40835, -40159]ns | [833, 835] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 836ns | -40346.7ns (-98.0%) | [-40777, -40186]ns | [827, 837] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 13075ns | -28115.2ns (-68.3%) | [-28514, -27972]ns | [13001, 13157] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1098ns | -40072.5ns (-97.3%) | [-40570, -39928]ns | [1096, 1102] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 310ns | -40795.0ns (-99.1%) | [-41362, -40681]ns | [301, 317] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 232ns | -40937.9ns (-99.4%) | [-41432, -40790]ns | [232, 233] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 31626ns | -9597.1ns (-23.3%) | [-10006, -9295]ns | [31456, 31877] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 41341ns | no significant difference | [-175, +112]ns | [41034, 41575] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 41089ns | -97.9% | -98.0% | -67.1% | -97.2% | -99.2% | -99.4% | -21.7% | +2.5% |
| 2 | 41062ns | -97.9% | -98.0% | -67.6% | -97.2% | -99.2% | -99.4% | -21.9% | +0.3% |
| 3 | 40885ns | -97.9% | -98.0% | -67.8% | -97.2% | -99.2% | -99.4% | -22.0% | +0.2% |
| 4 | 40990ns | -97.9% | -98.0% | -68.0% | -97.2% | -99.2% | -99.4% | -23.5% | +0.1% |
| 5 | 40950ns | -97.9% | -98.0% | -68.4% | -97.2% | -99.2% | -99.4% | -22.2% | +1.9% |
| 6 | 41029ns | -97.9% | -98.0% | -68.4% | -97.2% | -99.2% | -99.4% | -20.0% | +0.4% |
| 7 | 41017ns | -97.9% | -98.0% | -68.4% | -97.3% | -99.2% | -99.4% | -21.8% | +0.1% |
| 8 | 40885ns | -97.9% | -98.0% | -68.3% | -97.4% | -99.2% | -99.4% | -22.7% | -0.0% |
| 9 | 40837ns | -97.9% | -97.9% | -68.3% | -97.4% | -99.2% | -99.4% | -22.1% | +1.7% |
| 10 | 41015ns | -97.9% | -97.9% | -68.4% | -97.4% | -99.2% | -99.4% | -23.3% | +1.4% |
| 11 | 40876ns | -98.0% | -98.0% | -68.3% | -97.3% | -99.3% | -99.4% | -21.8% | +2.9% |
| 12 | 40838ns | -98.0% | -98.0% | -68.3% | -97.3% | -99.3% | -99.4% | -21.4% | +2.6% |
| 13 | 41101ns | -98.0% | -98.0% | -68.5% | -97.3% | -99.3% | -99.4% | -23.2% | -0.2% |
| 14 | 42432ns | -98.0% | -98.1% | -69.5% | -97.4% | -99.3% | -99.5% | -26.1% | -3.4% |
| 15 | 41438ns | -98.0% | -98.0% | -68.5% | -97.3% | -99.3% | -99.4% | -23.6% | -1.4% |
| 16 | 41087ns | -98.0% | -98.0% | -68.2% | -97.3% | -99.3% | -99.4% | -23.5% | -0.3% |
| 17 | 41078ns | -98.0% | -98.0% | -67.2% | -97.3% | -99.3% | -99.4% | -23.4% | -0.5% |
| 18 | 40884ns | -98.0% | -98.0% | -67.6% | -97.3% | -99.3% | -99.4% | -23.1% | +0.2% |
| 19 | 40958ns | -98.0% | -98.0% | -68.4% | -97.2% | -99.3% | -99.4% | -22.8% | -0.0% |
| 20 | 40970ns | -98.0% | -98.0% | -68.0% | -97.2% | -99.3% | -99.4% | -23.4% | +0.0% |
| 21 | 41886ns | -98.0% | -97.8% | -68.6% | -97.4% | -99.3% | -99.4% | -24.7% | +0.4% |
| 22 | 41573ns | -98.0% | -97.7% | -68.9% | -97.4% | -99.3% | -99.4% | -24.2% | +2.1% |
| 23 | 42981ns | -98.1% | -97.8% | -69.7% | -97.4% | -99.3% | -99.5% | -27.0% | -3.4% |
| 24 | 41755ns | -98.0% | -97.8% | -68.9% | -97.4% | -99.3% | -99.4% | -24.4% | -2.0% |
| 25 | 42045ns | -98.0% | -97.8% | -69.1% | -97.4% | -99.3% | -99.5% | -25.4% | -2.7% |
| 26 | 42673ns | -98.0% | -97.8% | -69.2% | -97.4% | -99.3% | -99.5% | -26.4% | -3.0% |
| 27 | 42417ns | -98.0% | -97.7% | -68.7% | -97.4% | -99.3% | -99.5% | -25.9% | -0.7% |
| 28 | 41316ns | -98.0% | -97.9% | -65.9% | -97.4% | -99.3% | -99.4% | -24.1% | +1.7% |
| 29 | 42128ns | -98.0% | -98.0% | -67.6% | -97.4% | -99.3% | -99.5% | -25.6% | -2.3% |
| 30 | 42162ns | -98.0% | -98.0% | -67.7% | -97.4% | -99.3% | -99.5% | -25.5% | +0.8% |
| 31 | 42061ns | -97.9% | -98.0% | -68.7% | -97.4% | -99.3% | -99.4% | -24.8% | -1.8% |
| 32 | 41001ns | -97.9% | -98.0% | -68.1% | -96.8% | -99.2% | -99.4% | -21.1% | +1.4% |
| 33 | 42138ns | -98.0% | -98.0% | -69.3% | -97.3% | -99.3% | -99.4% | -23.0% | -0.5% |
| 34 | 40857ns | -98.0% | -98.0% | -66.8% | -97.2% | -99.2% | -99.4% | -21.5% | +2.3% |
| 35 | 41239ns | -98.0% | -98.0% | -68.4% | -97.3% | -97.9% | -99.4% | -21.5% | -0.4% |
| 36 | 41370ns | -98.0% | -98.0% | -68.4% | -97.3% | -99.2% | -99.4% | -21.8% | -1.3% |
| 37 | 43083ns | -98.1% | -97.7% | -69.4% | -97.5% | -99.3% | -99.5% | -27.1% | -3.5% |
| 38 | 43038ns | -98.1% | -98.1% | -69.1% | -97.4% | -99.3% | -99.5% | -25.7% | -0.4% |
| 39 | 41558ns | -98.0% | -98.0% | -67.8% | -97.4% | -99.2% | -99.4% | -23.0% | -0.1% |
| 40 | 41804ns | -98.0% | -98.0% | -68.5% | -97.4% | -99.2% | -99.4% | -24.0% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.460 | moderate+ |
| satfold-lanes16 | 0.799 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.542 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.489 | moderate+ |
| satfold-lanes64 | 0.257 | moderate+ |
| satfold-neon | 0.023 | ok |
| satfold-neon8 | 0.397 | moderate+ |
| satfold-nolaw | 0.507 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.287 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 18/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 4.0ns | 41512.7ns | 0.0% |  |
| satfold-lanes16 | 3.0ns | 839.4ns | 0.4% |  |
| satfold-lanes16-constl | 2.6ns | 857.3ns | 0.3% |  |
| satfold-lanes4-idx | 3.2ns | 13157.0ns | 0.0% |  |
| satfold-lanes64 | 2.2ns | 1109.9ns | 0.2% |  |
| satfold-neon | 2.1ns | 328.2ns | 0.6% |  |
| satfold-neon8 | 1.7ns | 232.5ns | 0.7% |  |
| satfold-nolaw | 2.4ns | 31743.2ns | 0.0% |  |
| satfold-seq | 3.1ns | 41449.1ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 40876.5-42615.6 ns)
  40876.5 |#################################
  40963.5 |########################################
  41050.4 |#################################
  41137.4 |
  41224.3 |######
  41311.3 |#############
  41398.2 |######
  41485.2 |######
  41572.2 |######
  41659.1 |
  41746.1 |#############
  41833.0 |######
  41920.0 |
  42006.9 |#############
  42093.9 |####################
  42180.8 |
  42267.8 |
  42354.7 |#############
  42441.7 |
  42528.7 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 820.1-865.4 ns)
    820.1 |##############################
    822.3 |
    824.6 |#####
    826.9 |
    829.1 |##########
    831.4 |##############################
    833.7 |########################################
    835.9 |#####
    838.2 |
    840.5 |
    842.7 |
    845.0 |
    847.2 |
    849.5 |
    851.8 |
    854.0 |
    856.3 |
    858.6 |#####
    860.8 |##########
    863.1 |#########################
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 824.2-951.4 ns)
    824.2 |########################################
    830.6 |#################
    836.9 |#######
    843.3 |
    849.6 |
    856.0 |
    862.4 |##
    868.7 |
    875.1 |##
    881.4 |##
    887.8 |
    894.2 |
    900.5 |
    906.9 |
    913.2 |
    919.6 |
    926.0 |
    932.3 |############
    938.7 |##
    945.0 |
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 12942.9-13577.7 ns)
  12942.9 |########################################
  12974.6 |#############
  13006.4 |########
  13038.1 |#############
  13069.9 |####
  13101.6 |####
  13133.4 |#################
  13165.1 |########
  13196.8 |####
  13228.6 |####
  13260.3 |####
  13292.1 |########
  13323.8 |
  13355.5 |####
  13387.3 |
  13419.0 |
  13450.8 |
  13482.5 |####
  13514.3 |####
  13546.0 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 1085.5-1155.7 ns)
   1085.5 |####
   1089.0 |####
   1092.5 |########################################
   1096.0 |########################################
   1099.5 |########
   1103.0 |####
   1106.6 |####
   1110.1 |
   1113.6 |
   1117.1 |
   1120.6 |####
   1124.1 |####
   1127.6 |
   1131.1 |######################
   1134.7 |#################
   1138.2 |
   1141.7 |
   1145.2 |
   1148.7 |
   1152.2 |
  (4 below, 1 above range)

satfold-neon (n=40, range 298.9-406.6 ns)
    298.9 |########################################
    304.3 |######
    309.6 |############
    315.0 |###############
    320.4 |
    325.8 |
    331.2 |
    336.6 |############
    342.0 |##################
    347.4 |
    352.8 |
    358.1 |
    363.5 |
    368.9 |
    374.3 |
    379.7 |
    385.1 |
    390.5 |
    395.9 |
    401.2 |
  (5 below, 1 above range)

satfold-neon8 (n=40, range 229.9-235.3 ns)
    229.9 |################
    230.2 |################################
    230.5 |
    230.7 |################
    231.0 |################
    231.3 |
    231.6 |################################
    231.8 |
    232.1 |########################
    232.4 |########################################
    232.6 |
    232.9 |################################
    233.2 |########
    233.4 |
    233.7 |################
    234.0 |################
    234.2 |
    234.5 |########################
    234.8 |########
    235.0 |
  (2 below, 3 above range)

satfold-nolaw (n=40, range 31370.2-32335.7 ns)
  31370.2 |#################################
  31418.5 |########################################
  31466.8 |######
  31515.0 |######
  31563.3 |#############
  31611.6 |####################
  31659.9 |######
  31708.1 |
  31756.4 |#############
  31804.7 |
  31853.0 |#############
  31901.2 |
  31949.5 |####################
  31997.8 |
  32046.1 |##########################
  32094.3 |
  32142.6 |######
  32190.9 |
  32239.2 |
  32287.4 |######
  (4 below, 4 above range)

satfold-seq (n=40, range 40893.3-42274.2 ns)
  40893.3 |###########
  40962.3 |########################################
  41031.4 |###########
  41100.4 |
  41169.5 |#################
  41238.5 |
  41307.5 |###########
  41376.6 |
  41445.6 |
  41514.7 |############################
  41583.7 |#####
  41652.8 |###########
  41721.8 |#####
  41790.8 |
  41859.9 |#####
  41928.9 |#####
  41998.0 |###########
  42067.0 |#################
  42136.1 |
  42205.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **satfold-neon**: CV=26.0% (high variance, measurements may be unstable)
- **satfold-nolaw**: autocorrelation=0.51 (measurement drift or warm-up artifact)
