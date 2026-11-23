# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 1.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 34% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (1.44 us) leads warm-clamp-accfit-dyn (1.93 us) by 34%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 60% (significant)

warm-clamp-head is -1.95 us (60%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.2x slower than the field

warm-clamp-acc64 (3.22 us) is 2.2x the fastest (1.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.71)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.90 us but its worst 20% at 7.50 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 1436.6 ns median (-55.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.24x (fastest 1436.6 ns, slowest 3221.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112729874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3171ns | 3398ns | 1175ns | 3366ns | 4584ns | base |
| warm-clamp-accfit | 2171ns | 2155ns | 1707ns | 2168ns | 2643ns | -31.54% |
| warm-clamp-accfit-dyn | 2004ns | 2127ns | 1136ns | 2106ns | 2564ns | -36.82% |
| warm-clamp-head | 1636ns | 1652ns | 1423ns | 1621ns | 1893ns | -48.41% |
| warm-clamp-min-lanes | 3852ns | 2401ns | 1962ns | 3353ns | 7240ns | +21.48% |
| warm-clamp-minimum | 3952ns | 2386ns | 1951ns | 3354ns | 7748ns | +24.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2997ns | 1115ns | 4298ns | base | 2.734 |
| warm-clamp-accfit | 1962ns | 1538ns | 2375ns | -34.52% | 4.175 |
| warm-clamp-accfit-dyn | 1812ns | 1026ns | 2302ns | -39.55% | 4.522 |
| warm-clamp-head | 1424ns | 1245ns | 1631ns | -52.49% | 5.754 |
| warm-clamp-min-lanes | 3741ns | 1911ns | 6998ns | +24.83% | 2.190 |
| warm-clamp-minimum | 3837ns | 1899ns | 7499ns | +28.04% | 2.135 |

## Performance model

- Peak throughput: **7.986 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.543 | 31.8% |
| warm-clamp-accfit | 4.177 | 52.3% |
| warm-clamp-accfit-dyn | 4.250 | 53.2% |
| warm-clamp-head | 5.702 | 71.4% |
| warm-clamp-min-lanes | 3.505 | 43.9% |
| warm-clamp-minimum | 3.528 | 44.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3171ns | 3171ns | base |
| warm-clamp-accfit | 2171ns | 2171ns | -31.54% |
| warm-clamp-accfit-dyn | 2004ns | 2004ns | -36.82% |
| warm-clamp-head | 1636ns | 1636ns | -48.41% |
| warm-clamp-min-lanes | 3852ns | 3852ns | +21.48% |
| warm-clamp-minimum | 3952ns | 3952ns | +24.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3222ns | base | --- | [3061, 3472] | --- | --- | --- | --- |
| warm-clamp-accfit | 1961ns | -1448.9ns (-45.0%) | [-1628, -1222]ns | [1877, 2047] | YES | 0.0011 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 1928ns | -1277.2ns (-39.6%) | [-1409, -986]ns | [1878, 2048] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1437ns | -1946.7ns (-60.4%) | [-2094, -1708]ns | [1336, 1458] | YES | 0.0011 | 0.0007 | 0 |
| warm-clamp-min-lanes | 2337ns | no significant difference | [-345, +1286]ns | [1923, 5311] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-minimum | 2322ns | no significant difference | [-498, +1226]ns | [2319, 4329] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3222ns | -52.1% | -40.2% | -54.8% | -19.4% | +5.9% |
| 1 | 2 | 3452ns | -43.0% | -42.8% | -61.3% | -31.6% | -24.3% |
| 1 | 3 | 2101ns | +8.2% | -50.5% | -23.7% | +64.3% | +10.8% |
| 1 | 4 | 3181ns | -38.9% | -29.2% | -60.1% | -26.5% | -40.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.433 | moderate- |
| warm-clamp-accfit | 0.007 | ok |
| warm-clamp-accfit-dyn | -0.549 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.708 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.593 | HIGH- (thermal bounce) |
| warm-clamp-minimum | 0.153 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 62.4ns | 2996.6ns | 2.1% |  |
| warm-clamp-accfit | 74.2ns | 1962.3ns | 3.8% |  |
| warm-clamp-accfit-dyn | 68.1ns | 1811.6ns | 3.8% |  |
| warm-clamp-head | 75.6ns | 1423.7ns | 5.3% | HIGH |
| warm-clamp-min-lanes | 37.6ns | 3740.6ns | 1.0% |  |
| warm-clamp-minimum | 38.2ns | 3836.8ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1114.5-4298.0 ns)
   1114.5 |########################################
   1273.7 |
   1432.9 |
   1592.1 |
   1751.2 |
   1910.4 |
   2069.6 |
   2228.8 |
   2387.9 |#####
   2547.1 |
   2706.3 |
   2865.5 |###########
   3024.6 |##################################
   3183.8 |######################
   3343.0 |##################################
   3502.2 |###########
   3661.3 |######################
   3820.5 |#####
   3979.7 |#####
   4138.9 |#####
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 1538.1-2374.5 ns)
   1538.1 |
   1579.9 |####################
   1621.8 |######
   1663.6 |
   1705.4 |#############
   1747.2 |
   1789.0 |######
   1830.9 |######
   1872.7 |########################################
   1914.5 |######
   1956.3 |
   1998.1 |#############
   2040.0 |########################################
   2081.8 |
   2123.6 |######
   2165.4 |
   2207.2 |
   2249.1 |########################################
   2290.9 |######
   2332.7 |######
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1025.8-2301.6 ns)
   1025.8 |#########################
   1089.5 |
   1153.3 |
   1217.1 |
   1280.9 |
   1344.7 |
   1408.5 |
   1472.3 |
   1536.1 |
   1599.9 |
   1663.7 |
   1727.5 |
   1791.3 |#####
   1855.0 |########################################
   1918.8 |##########
   1982.6 |#####
   2046.4 |########################################
   2110.2 |#####
   2174.0 |
   2237.8 |###################################
  (5 below, 2 above range)

warm-clamp-head (n=40, range 1245.0-1631.0 ns)
   1245.0 |####
   1264.3 |
   1283.6 |####
   1302.9 |
   1322.2 |########################################
   1341.5 |
   1360.8 |
   1380.1 |####
   1399.4 |
   1418.7 |########
   1438.0 |##########################
   1457.3 |#############
   1476.6 |
   1495.9 |########
   1515.2 |
   1534.5 |
   1553.8 |########
   1573.1 |
   1592.4 |#################
   1611.7 |####
  (6 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1910.6-6997.8 ns)
   1910.6 |########################################
   2164.9 |#############################
   2419.3 |
   2673.6 |###
   2928.0 |
   3182.4 |###
   3436.7 |
   3691.1 |
   3945.4 |
   4199.8 |
   4454.1 |
   4708.5 |
   4962.9 |###
   5217.2 |
   5471.6 |###
   5725.9 |##########
   5980.3 |##########
   6234.7 |###
   6489.0 |##########
   6743.4 |###
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 1899.5-7498.7 ns)
   1899.5 |####################################
   2179.4 |########################################
   2459.4 |
   2739.3 |
   3019.3 |###
   3299.3 |
   3579.2 |
   3859.2 |
   4139.1 |###
   4419.1 |###
   4699.1 |
   4979.0 |
   5259.0 |
   5538.9 |###
   5818.9 |###
   6098.9 |###
   6378.8 |#######
   6658.8 |#######
   6938.8 |#######
   7218.7 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=37.7% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=26.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=58.2% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=60.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
