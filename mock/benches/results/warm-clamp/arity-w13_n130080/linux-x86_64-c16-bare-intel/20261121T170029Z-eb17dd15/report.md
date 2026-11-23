# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 745 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 72% faster than the next best (warm-clamp-accfit)

warm-clamp-head (745 ns) leads warm-clamp-accfit (1.28 us) by 72%, a clear separation rather than a photo finish. CV 7.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 78% (significant)

warm-clamp-head is -2.57 us (78%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 4.5x slower than the field

warm-clamp-acc64 (3.32 us) is 4.5x the fastest (745 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.61)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.5x the fastest

Fastest warm-clamp-head (745 ns) to slowest warm-clamp-acc64 (3.32 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.23 us but its worst 20% at 5.12 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 745.0 ns median (-77.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 4.45x (fastest 745.0 ns, slowest 3315.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761915 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3297ns | 3485ns | 1291ns | 3254ns | 5433ns | base |
| warm-clamp-accfit | 1501ns | 1468ns | 1367ns | 1486ns | 1680ns | -54.48% |
| warm-clamp-accfit-dyn | 1734ns | 1760ns | 1482ns | 1731ns | 1993ns | -47.41% |
| warm-clamp-head | 988ns | 952ns | 901ns | 973ns | 1122ns | -70.02% |
| warm-clamp-min-lanes | 2397ns | 2255ns | 1729ns | 2316ns | 3308ns | -27.31% |
| warm-clamp-minimum | 4532ns | 3315ns | 3162ns | 3540ns | 8880ns | +37.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3124ns | 1228ns | 5122ns | base | 2.622 |
| warm-clamp-accfit | 1296ns | 1190ns | 1427ns | -58.51% | 6.320 |
| warm-clamp-accfit-dyn | 1502ns | 1291ns | 1704ns | -51.94% | 5.456 |
| warm-clamp-head | 768ns | 706ns | 853ns | -75.42% | 10.669 |
| warm-clamp-min-lanes | 2189ns | 1587ns | 3010ns | -29.93% | 3.742 |
| warm-clamp-minimum | 4448ns | 3109ns | 8685ns | +42.37% | 1.842 |

## Performance model

- Peak throughput: **11.606 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.471 | 21.3% |
| warm-clamp-accfit | 6.406 | 55.2% |
| warm-clamp-accfit-dyn | 5.342 | 46.0% |
| warm-clamp-head | 10.997 | 94.7% |
| warm-clamp-min-lanes | 3.956 | 34.1% |
| warm-clamp-minimum | 2.512 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3297ns | 3297ns | base |
| warm-clamp-accfit | 1501ns | 1501ns | -54.48% |
| warm-clamp-accfit-dyn | 1734ns | 1734ns | -47.41% |
| warm-clamp-head | 988ns | 988ns | -70.02% |
| warm-clamp-min-lanes | 2397ns | 2397ns | -27.31% |
| warm-clamp-minimum | 4532ns | 4532ns | +37.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3315ns | base | --- | [1866, 4222] | --- | --- | --- | --- |
| warm-clamp-accfit | 1279ns | -2035.8ns (-61.4%) | [-3041, -617]ns | [1278, 1279] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1533ns | -1749.0ns (-52.8%) | [-2665, -348]ns | [1449, 1534] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-head | 745ns | -2570.4ns (-77.5%) | [-3477, -1072]ns | [744, 771] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2071ns | -1056.5ns (-31.9%) | [-1294, -312]ns | [2018, 2233] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-minimum | 3261ns | no significant difference | [-127, +1994]ns | [3243, 3879] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3583ns | -64.3% | -64.3% | -79.0% | -44.6% | -9.1% |
| 1 | 2 | 1540ns | -15.4% | -4.7% | -53.3% | +32.0% | +110.7% |
| 1 | 3 | 4222ns | -71.6% | -63.1% | -81.9% | -30.1% | -7.9% |
| 1 | 4 | 3315ns | -60.7% | -52.9% | -77.5% | -33.2% | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.566 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.606 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.160 | ok |
| warm-clamp-head | -0.612 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.241 | moderate- |
| warm-clamp-minimum | -0.513 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 59.2ns | 3124.2ns | 1.9% |  |
| warm-clamp-accfit | 70.5ns | 1296.2ns | 5.4% | HIGH |
| warm-clamp-accfit-dyn | 79.3ns | 1501.6ns | 5.3% | HIGH |
| warm-clamp-head | 77.3ns | 767.9ns | 10.1% | HIGH |
| warm-clamp-min-lanes | 72.0ns | 2189.1ns | 3.3% |  |
| warm-clamp-minimum | 27.0ns | 4448.0ns | 0.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1227.9-5121.7 ns)
   1227.9 |########################################
   1422.6 |###
   1617.2 |
   1811.9 |##########
   2006.6 |
   2201.3 |
   2396.0 |
   2590.7 |###
   2785.4 |
   2980.1 |
   3174.8 |##########
   3369.5 |#######
   3564.2 |###
   3758.8 |###
   3953.5 |###
   4148.2 |#######
   4342.9 |###
   4537.6 |#####################
   4732.3 |###
   4927.0 |#######
  (1 below, 3 above range)

warm-clamp-accfit (n=40, range 1190.0-1426.7 ns)
   1190.0 |#####
   1201.9 |
   1213.7 |#####
   1225.5 |##
   1237.4 |
   1249.2 |
   1261.0 |##
   1272.9 |########################################
   1284.7 |
   1296.5 |
   1308.4 |
   1320.2 |#####
   1332.0 |##
   1343.9 |##
   1355.7 |
   1367.5 |
   1379.3 |
   1391.2 |#################
   1403.0 |
   1414.8 |
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1290.9-1704.0 ns)
   1290.9 |
   1311.5 |###
   1332.2 |
   1352.9 |###
   1373.5 |#########
   1394.2 |######
   1414.8 |###
   1435.5 |
   1456.1 |###
   1476.8 |
   1497.4 |###
   1518.1 |########################################
   1538.7 |
   1559.4 |###
   1580.1 |###
   1600.7 |
   1621.4 |###
   1642.0 |###
   1662.7 |###
   1683.3 |############
  (6 below, 2 above range)

warm-clamp-head (n=40, range 705.8-853.2 ns)
    705.8 |
    713.2 |
    720.6 |
    727.9 |
    735.3 |##########
    742.7 |########################################
    750.1 |
    757.4 |##
    764.8 |
    772.2 |
    779.5 |##
    786.9 |##
    794.3 |
    801.7 |##
    809.0 |##
    816.4 |###############
    823.8 |
    831.1 |
    838.5 |
    845.9 |
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1586.6-3009.7 ns)
   1586.6 |
   1657.8 |
   1728.9 |
   1800.1 |###############
   1871.3 |#########################
   1942.4 |####################
   2013.6 |########################################
   2084.7 |##########
   2155.9 |##########
   2227.0 |##########
   2298.2 |#####
   2369.3 |###############
   2440.5 |#####
   2511.7 |
   2582.8 |
   2654.0 |#####
   2725.1 |
   2796.3 |
   2867.4 |
   2938.6 |###############
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 3109.1-8685.5 ns)
   3109.1 |########################################
   3388.0 |##
   3666.8 |#######################
   3945.6 |
   4224.4 |
   4503.2 |
   4782.0 |
   5060.9 |
   5339.7 |
   5618.5 |
   5897.3 |##
   6176.1 |
   6455.0 |
   6733.8 |
   7012.6 |
   7291.4 |
   7570.2 |
   7849.0 |
   8127.9 |##
   8406.7 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=49.8% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-head**: bridge=10.3% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: CV=24.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=53.6% (high variance, measurements may be unstable)
