# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (4.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 715 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 115% faster than the next best (warm-clamp-accfit)

warm-clamp-head (715 ns) leads warm-clamp-accfit (1.53 us) by 115%, a clear separation rather than a photo finish. CV 10.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 81% (significant)

warm-clamp-head is -3.27 us (81%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 5.7x slower than the field

warm-clamp-acc64 (4.06 us) is 5.7x the fastest (715 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.61)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.7x the fastest

Fastest warm-clamp-head (715 ns) to slowest warm-clamp-acc64 (4.06 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 2.8x its best-20%

warm-clamp-minimum's best 20% of batches run at 3.88 us but its worst 20% at 10.97 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 714.8 ns median (-82.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.69x (fastest 714.8 ns, slowest 4064.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3928ns | 4279ns | 1899ns | 4245ns | 5005ns | base |
| warm-clamp-accfit | 1874ns | 1761ns | 1628ns | 1815ns | 2296ns | -52.29% |
| warm-clamp-accfit-dyn | 1787ns | 1761ns | 1552ns | 1759ns | 2105ns | -54.50% |
| warm-clamp-head | 931ns | 946ns | 798ns | 923ns | 1086ns | -76.30% |
| warm-clamp-min-lanes | 2967ns | 2876ns | 2756ns | 2913ns | 3341ns | -24.46% |
| warm-clamp-minimum | 5475ns | 3995ns | 3945ns | 4074ns | 11209ns | +39.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3720ns | 1803ns | 4702ns | base | 2.202 |
| warm-clamp-accfit | 1616ns | 1420ns | 1984ns | -56.56% | 5.069 |
| warm-clamp-accfit-dyn | 1547ns | 1353ns | 1802ns | -58.43% | 5.297 |
| warm-clamp-head | 719ns | 625ns | 827ns | -80.67% | 11.392 |
| warm-clamp-min-lanes | 2707ns | 2525ns | 3003ns | -27.24% | 3.026 |
| warm-clamp-minimum | 5374ns | 3879ns | 10969ns | +44.44% | 1.524 |

## Performance model

- Peak throughput: **13.113 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.016 | 15.4% |
| warm-clamp-accfit | 5.341 | 40.7% |
| warm-clamp-accfit-dyn | 5.340 | 40.7% |
| warm-clamp-head | 11.460 | 87.4% |
| warm-clamp-min-lanes | 3.113 | 23.7% |
| warm-clamp-minimum | 2.085 | 15.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3928ns | 3928ns | base |
| warm-clamp-accfit | 1874ns | 1874ns | -52.29% |
| warm-clamp-accfit-dyn | 1787ns | 1787ns | -54.50% |
| warm-clamp-head | 931ns | 931ns | -76.30% |
| warm-clamp-min-lanes | 2967ns | 2967ns | -24.46% |
| warm-clamp-minimum | 5475ns | 5475ns | +39.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4064ns | base | --- | [3780, 4291] | --- | --- | --- | --- |
| warm-clamp-accfit | 1534ns | -2404.9ns (-59.2%) | [-2606, -2243]ns | [1534, 1535] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1534ns | -2447.6ns (-60.2%) | [-2636, -2315]ns | [1501, 1545] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 715ns | -3273.1ns (-80.5%) | [-3569, -3074]ns | [679, 743] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2632ns | -1344.3ns (-33.1%) | [-1463, -1151]ns | [2617, 2704] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 3929ns | +193.3ns (+4.8%) | [+75, +1805]ns | [3914, 3941] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3852ns | -63.4% | -62.0% | -82.4% | -31.5% | +1.8% |
| 1 | 2 | 4434ns | -56.8% | -65.2% | -83.2% | -41.3% | -11.1% |
| 1 | 3 | 3950ns | -61.2% | -56.1% | -84.1% | -33.5% | -0.6% |
| 1 | 4 | 3938ns | -61.1% | -64.6% | -79.3% | -31.1% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.485 | moderate- |
| warm-clamp-accfit | -0.526 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.410 | moderate- |
| warm-clamp-head | -0.610 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.029 | ok |
| warm-clamp-minimum | 0.010 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 70.8ns | 3720.4ns | 1.9% |  |
| warm-clamp-accfit | 89.0ns | 1616.0ns | 5.5% | HIGH |
| warm-clamp-accfit-dyn | 84.5ns | 1546.6ns | 5.5% | HIGH |
| warm-clamp-head | 74.0ns | 719.1ns | 10.3% | HIGH |
| warm-clamp-min-lanes | 88.5ns | 2706.8ns | 3.3% |  |
| warm-clamp-minimum | 33.0ns | 5373.8ns | 0.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1802.9-4701.7 ns)
   1802.9 |#####
   1947.8 |
   2092.7 |
   2237.7 |#####
   2382.6 |
   2527.6 |
   2672.5 |
   2817.5 |
   2962.4 |
   3107.3 |
   3252.3 |
   3397.2 |###########
   3542.2 |#####
   3687.1 |########################################
   3832.1 |###########
   3977.0 |############################
   4121.9 |###########
   4266.9 |###########
   4411.8 |##################################
   4556.8 |#####
  (5 below, 5 above range)

warm-clamp-accfit (n=40, range 1419.7-1983.9 ns)
   1419.7 |##
   1447.9 |##
   1476.1 |
   1504.3 |##
   1532.5 |########################################
   1560.7 |
   1588.9 |
   1617.2 |##
   1645.4 |
   1673.6 |
   1701.8 |######
   1730.0 |
   1758.2 |
   1786.4 |##
   1814.6 |
   1842.8 |
   1871.0 |##
   1899.2 |######
   1927.4 |##
   1955.7 |
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1352.6-1802.2 ns)
   1352.6 |
   1375.1 |##########################
   1397.6 |
   1420.1 |######
   1442.6 |
   1465.0 |###
   1487.5 |
   1510.0 |###
   1532.5 |########################################
   1554.9 |
   1577.4 |######
   1599.9 |###
   1622.4 |
   1644.9 |###
   1667.3 |
   1689.8 |#############
   1712.3 |
   1734.8 |
   1757.3 |###
   1779.7 |###
  (3 below, 3 above range)

warm-clamp-head (n=40, range 624.7-826.6 ns)
    624.7 |########################################
    634.8 |
    644.9 |
    655.0 |
    665.1 |
    675.2 |############################
    685.3 |########
    695.4 |
    705.5 |
    715.6 |
    725.7 |
    735.7 |################################
    745.8 |####
    755.9 |
    766.0 |
    776.1 |
    786.2 |
    796.3 |####
    806.4 |############
    816.5 |####################
  (1 below, 2 above range)

warm-clamp-min-lanes (n=40, range 2524.8-3002.9 ns)
   2524.8 |###
   2548.7 |
   2572.6 |
   2596.5 |########################################
   2620.4 |################
   2644.3 |##########
   2668.2 |######
   2692.1 |###
   2716.0 |###
   2739.9 |###
   2763.8 |
   2787.7 |###
   2811.6 |
   2835.5 |###
   2859.4 |
   2883.3 |##########
   2907.2 |
   2931.1 |######
   2955.0 |######
   2978.9 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 3878.9-10969.2 ns)
   3878.9 |########################################
   4233.4 |
   4588.0 |
   4942.5 |
   5297.0 |#
   5651.5 |
   6006.0 |
   6360.5 |
   6715.0 |#
   7069.5 |
   7424.1 |
   7778.6 |
   8133.1 |
   8487.6 |
   8842.1 |
   9196.6 |#
   9551.1 |
   9905.6 |
  10260.2 |#
  10614.7 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=28.8% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=10.4% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: CV=54.6% (high variance, measurements may be unstable)
