# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 762 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 110% faster than the next best (warm-clamp-min-lanes)

warm-clamp-head (762 ns) leads warm-clamp-min-lanes (1.60 us) by 110%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 78% (significant)

warm-clamp-head is -2.54 us (78%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 4.3x slower than the field

warm-clamp-acc64 (3.25 us) is 4.3x the fastest (762 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.3x the fastest

Fastest warm-clamp-head (762 ns) to slowest warm-clamp-acc64 (3.25 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 2.21 us but its worst 20% at 8.52 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 762.0 ns median (-76.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 4.27x (fastest 762.0 ns, slowest 3251.9 ns)

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
| warm-clamp-acc64 | 3416ns | 3466ns | 2340ns | 3503ns | 4230ns | base |
| warm-clamp-accfit | 1886ns | 1917ns | 1655ns | 1889ns | 2109ns | -44.78% |
| warm-clamp-accfit-dyn | 2020ns | 1933ns | 1760ns | 1992ns | 2366ns | -40.86% |
| warm-clamp-head | 957ns | 951ns | 857ns | 952ns | 1075ns | -71.97% |
| warm-clamp-min-lanes | 1805ns | 1807ns | 1528ns | 1821ns | 2036ns | -47.15% |
| warm-clamp-minimum | 4228ns | 2852ns | 2269ns | 3361ns | 8791ns | +23.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3216ns | 2209ns | 3957ns | base | 2.547 |
| warm-clamp-accfit | 1653ns | 1461ns | 1812ns | -48.60% | 4.956 |
| warm-clamp-accfit-dyn | 1770ns | 1546ns | 2044ns | -44.96% | 4.628 |
| warm-clamp-head | 757ns | 687ns | 839ns | -76.45% | 10.815 |
| warm-clamp-min-lanes | 1583ns | 1341ns | 1756ns | -50.78% | 5.176 |
| warm-clamp-minimum | 4112ns | 2211ns | 8521ns | +27.85% | 1.992 |

## Performance model

- Peak throughput: **11.924 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.519 | 21.1% |
| warm-clamp-accfit | 4.843 | 40.6% |
| warm-clamp-accfit-dyn | 4.845 | 40.6% |
| warm-clamp-head | 10.751 | 90.2% |
| warm-clamp-min-lanes | 5.113 | 42.9% |
| warm-clamp-minimum | 2.948 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3416ns | 3416ns | base |
| warm-clamp-accfit | 1886ns | 1886ns | -44.78% |
| warm-clamp-accfit-dyn | 2020ns | 2020ns | -40.86% |
| warm-clamp-head | 957ns | 957ns | -71.97% |
| warm-clamp-min-lanes | 1805ns | 1805ns | -47.15% |
| warm-clamp-minimum | 4228ns | 4228ns | +23.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3252ns | base | --- | [3122, 3514] | --- | --- | --- | --- |
| warm-clamp-accfit | 1691ns | -1666.6ns (-51.2%) | [-1878, -1365]ns | [1654, 1694] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1691ns | -1532.4ns (-47.1%) | [-1620, -1447]ns | [1688, 1849] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 762ns | -2540.8ns (-78.1%) | [-2744, -2390]ns | [758, 763] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1602ns | -1707.8ns (-52.5%) | [-1984, -1408]ns | [1551, 1706] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2779ns | no significant difference | [-333, +1173]ns | [2604, 3060] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2932ns | -42.3% | -47.6% | -76.0% | -43.4% | -10.6% |
| 1 | 2 | 3303ns | -48.5% | -44.4% | -76.9% | -48.2% | -10.9% |
| 1 | 3 | 3811ns | -59.0% | -53.3% | -78.2% | -59.3% | -42.0% |
| 1 | 4 | 3420ns | -54.7% | -46.4% | -77.7% | -60.8% | -33.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.059 | ok |
| warm-clamp-accfit | 0.254 | moderate+ |
| warm-clamp-accfit-dyn | -0.215 | moderate- |
| warm-clamp-head | -0.032 | ok |
| warm-clamp-min-lanes | 0.182 | ok |
| warm-clamp-minimum | -0.020 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 71.7ns | 3216.0ns | 2.2% |  |
| warm-clamp-accfit | 82.3ns | 1652.9ns | 5.0% |  |
| warm-clamp-accfit-dyn | 87.1ns | 1770.2ns | 4.9% |  |
| warm-clamp-head | 71.4ns | 757.5ns | 9.4% | HIGH |
| warm-clamp-min-lanes | 79.3ns | 1582.8ns | 5.0% | HIGH |
| warm-clamp-minimum | 35.6ns | 4111.6ns | 0.9% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2209.4-3957.1 ns)
   2209.4 |
   2296.8 |
   2384.2 |
   2471.6 |
   2559.0 |######
   2646.4 |####################
   2733.7 |#############
   2821.1 |
   2908.5 |####################
   2995.9 |#############
   3083.3 |
   3170.7 |########################################
   3258.0 |#############
   3345.4 |######
   3432.8 |##########################
   3520.2 |#############
   3607.6 |
   3695.0 |######
   3782.3 |#################################
   3869.7 |####################
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 1461.3-1812.2 ns)
   1461.3 |##
   1478.8 |
   1496.4 |##
   1513.9 |
   1531.5 |########
   1549.0 |##
   1566.6 |##
   1584.1 |##
   1601.7 |##
   1619.2 |
   1636.7 |
   1654.3 |
   1671.8 |
   1689.4 |########################################
   1706.9 |
   1724.5 |##
   1742.0 |##
   1759.6 |##
   1777.1 |
   1794.7 |
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1545.7-2043.5 ns)
   1545.7 |
   1570.6 |
   1595.5 |##
   1620.4 |
   1645.3 |
   1670.2 |########################################
   1695.1 |
   1719.9 |##
   1744.8 |##
   1769.7 |
   1794.6 |#####
   1819.5 |##
   1844.4 |##
   1869.3 |#################
   1894.2 |
   1919.1 |#####
   1944.0 |
   1968.9 |
   1993.7 |
   2018.6 |#####
  (6 below, 3 above range)

warm-clamp-head (n=40, range 687.0-838.9 ns)
    687.0 |
    694.6 |
    702.2 |########################################
    709.8 |
    717.4 |
    725.0 |
    732.6 |
    740.2 |
    747.8 |####
    755.4 |####################################
    763.0 |############################
    770.5 |
    778.1 |
    785.7 |####
    793.3 |
    800.9 |
    808.5 |
    816.1 |
    823.7 |########
    831.3 |####################
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1340.6-1756.4 ns)
   1340.6 |
   1361.4 |##########
   1382.2 |
   1403.0 |#####
   1423.8 |##########
   1444.6 |
   1465.4 |
   1486.1 |
   1506.9 |##########
   1527.7 |
   1548.5 |###################################
   1569.3 |
   1590.1 |###############
   1610.9 |#####
   1631.7 |
   1652.5 |
   1673.3 |#####
   1694.0 |########################################
   1714.8 |#####
   1735.6 |###############
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 2211.0-8520.8 ns)
   2211.0 |########################################
   2526.5 |###################################
   2842.0 |##############################
   3157.5 |#####
   3472.9 |
   3788.4 |
   4103.9 |
   4419.4 |#####
   4734.9 |
   5050.4 |##########
   5365.9 |
   5681.4 |
   5996.9 |
   6312.3 |#####
   6627.8 |#####
   6943.3 |#####
   7258.8 |#####
   7574.3 |
   7889.8 |#####
   8205.3 |#####
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=21.2% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=60.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
