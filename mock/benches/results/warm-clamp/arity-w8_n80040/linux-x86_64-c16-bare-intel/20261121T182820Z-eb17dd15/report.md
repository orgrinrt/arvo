# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (2.92 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 1.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 12% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (1.50 us) leads warm-clamp-accfit-dyn (1.68 us) by 12%, a clear separation rather than a photo finish. CV 14.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 47% (significant)

warm-clamp-head is -1.38 us (47%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.59)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 461 ns but its worst 20% at 1.95 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 1498.3 ns median (-48.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.95x (fastest 1498.3 ns, slowest 2921.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755372 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3111ns | 3177ns | 2488ns | 3154ns | 3603ns | base |
| warm-clamp-accfit | 1997ns | 1980ns | 1658ns | 1968ns | 2423ns | -35.81% |
| warm-clamp-accfit-dyn | 1667ns | 1913ns | 527ns | 1840ns | 2290ns | -46.40% |
| warm-clamp-head | 1787ns | 1713ns | 1544ns | 1733ns | 2191ns | -42.56% |
| warm-clamp-min-lanes | 3745ns | 2244ns | 2014ns | 2778ns | 8379ns | +20.40% |
| warm-clamp-minimum | 3971ns | 2461ns | 2013ns | 3318ns | 7889ns | +27.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2892ns | 2322ns | 3331ns | base | 2.833 |
| warm-clamp-accfit | 1733ns | 1452ns | 2076ns | -40.07% | 4.727 |
| warm-clamp-accfit-dyn | 1446ns | 461ns | 1951ns | -49.98% | 5.664 |
| warm-clamp-head | 1550ns | 1349ns | 1896ns | -46.39% | 5.285 |
| warm-clamp-min-lanes | 3636ns | 1962ns | 8099ns | +25.75% | 2.253 |
| warm-clamp-minimum | 3856ns | 1960ns | 7627ns | +33.36% | 2.124 |

## Performance model

- Peak throughput: **17.759 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.804 | 15.8% |
| warm-clamp-accfit | 4.818 | 27.1% |
| warm-clamp-accfit-dyn | 4.877 | 27.5% |
| warm-clamp-head | 5.468 | 30.8% |
| warm-clamp-min-lanes | 3.747 | 21.1% |
| warm-clamp-minimum | 3.418 | 19.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3111ns | 3111ns | base |
| warm-clamp-accfit | 1997ns | 1997ns | -35.81% |
| warm-clamp-accfit-dyn | 1667ns | 1667ns | -46.40% |
| warm-clamp-head | 1787ns | 1787ns | -42.56% |
| warm-clamp-min-lanes | 3745ns | 3745ns | +20.40% |
| warm-clamp-minimum | 3971ns | 3971ns | +27.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2921ns | base | --- | [2749, 3132] | --- | --- | --- | --- |
| warm-clamp-accfit | 1700ns | -1318.5ns (-45.1%) | [-1414, -1216]ns | [1643, 1783] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1680ns | -1271.8ns (-43.5%) | [-1353, -1088]ns | [1618, 1796] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1498ns | -1379.7ns (-47.2%) | [-1429, -1290]ns | [1488, 1514] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2186ns | no significant difference | [-665, +308]ns | [1967, 2411] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-minimum | 2396ns | no significant difference | [-644, +2132]ns | [1970, 5003] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2634ns | -24.2% | -37.5% | -43.2% | -25.3% | +19.0% |
| 1 | 2 | 2869ns | -48.1% | -39.0% | -47.8% | -31.2% | +50.1% |
| 1 | 3 | 3133ns | -43.2% | -41.2% | -42.7% | -37.2% | -37.4% |
| 1 | 4 | 2968ns | -44.5% | -84.4% | -52.8% | -19.0% | -19.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.127 | ok |
| warm-clamp-accfit | -0.585 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.154 | ok |
| warm-clamp-head | -0.527 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.090 | ok |
| warm-clamp-minimum | -0.173 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 79.0ns | 2891.6ns | 2.7% |  |
| warm-clamp-accfit | 94.1ns | 1733.0ns | 5.4% | HIGH |
| warm-clamp-accfit-dyn | 76.5ns | 1446.4ns | 5.3% | HIGH |
| warm-clamp-head | 77.5ns | 1550.1ns | 5.0% | HIGH |
| warm-clamp-min-lanes | 35.5ns | 3636.2ns | 1.0% |  |
| warm-clamp-minimum | 37.3ns | 3856.1ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2321.5-3331.3 ns)
   2321.5 |
   2372.0 |####
   2422.5 |####
   2473.0 |
   2523.5 |
   2574.0 |########################
   2624.5 |########
   2675.0 |############
   2725.4 |
   2775.9 |####
   2826.4 |############
   2876.9 |########
   2927.4 |####
   2977.9 |####
   3028.4 |
   3078.9 |
   3129.3 |########################################
   3179.8 |################
   3230.3 |
   3280.8 |
  (1 below, 4 above range)

warm-clamp-accfit (n=40, range 1451.9-2076.4 ns)
   1451.9 |#####
   1483.1 |###########
   1514.4 |###########
   1545.6 |
   1576.8 |#################
   1608.0 |#####
   1639.3 |##################################
   1670.5 |###########
   1701.7 |###########
   1732.9 |
   1764.2 |########################################
   1795.4 |
   1826.6 |######################
   1857.9 |
   1889.1 |
   1920.3 |
   1951.5 |
   1982.8 |######################
   2014.0 |
   2045.2 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 461.3-1950.9 ns)
    461.3 |##############################
    535.8 |
    610.3 |
    684.7 |
    759.2 |
    833.7 |
    908.2 |
    982.6 |
   1057.1 |
   1131.6 |
   1206.1 |
   1280.6 |
   1355.0 |
   1429.5 |#####
   1504.0 |
   1578.5 |########################################
   1652.9 |##########
   1727.4 |###################################
   1801.9 |#########################
   1876.4 |###############
  (4 below, 4 above range)

warm-clamp-head (n=40, range 1349.1-1895.8 ns)
   1349.1 |####################################
   1376.5 |
   1403.8 |###
   1431.1 |
   1458.5 |###
   1485.8 |########################################
   1513.1 |###
   1540.5 |###
   1567.8 |
   1595.2 |###
   1622.5 |
   1649.8 |##########
   1677.2 |
   1704.5 |######
   1731.8 |###
   1759.2 |
   1786.5 |
   1813.8 |##########
   1841.2 |
   1868.5 |
  (1 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1962.2-8099.1 ns)
   1962.2 |########################################
   2269.1 |##################
   2575.9 |
   2882.8 |
   3189.6 |
   3496.5 |
   3803.3 |##
   4110.1 |##
   4417.0 |#####
   4723.8 |
   5030.7 |
   5337.5 |
   5644.4 |
   5951.2 |
   6258.1 |
   6564.9 |
   6871.8 |##
   7178.6 |#####
   7485.5 |
   7792.3 |########
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 1960.4-7626.8 ns)
   1960.4 |########################################
   2243.7 |#############################
   2527.0 |###
   2810.3 |
   3093.7 |
   3377.0 |
   3660.3 |
   3943.6 |
   4226.9 |###
   4510.3 |
   4793.6 |
   5076.9 |
   5360.2 |
   5643.6 |##########
   5926.9 |##########
   6210.2 |###
   6493.5 |###
   6776.8 |###
   7060.2 |#######
   7343.5 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: CV=40.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=67.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=61.7% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
