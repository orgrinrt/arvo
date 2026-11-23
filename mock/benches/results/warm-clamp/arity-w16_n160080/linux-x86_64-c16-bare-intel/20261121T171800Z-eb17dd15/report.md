# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 682 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 91% faster than the next best (warm-clamp-min-lanes)

warm-clamp-head (682 ns) leads warm-clamp-min-lanes (1.30 us) by 91%, a clear separation rather than a photo finish. CV 48.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 85% (significant)

warm-clamp-head is -3.20 us (85%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 5.6x slower than the field

warm-clamp-acc64 (3.79 us) is 5.6x the fastest (682 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.52)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.6x the fastest

Fastest warm-clamp-head (682 ns) to slowest warm-clamp-acc64 (3.79 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-head is inconsistent: worst-20% is 6.1x its best-20%

warm-clamp-head's best 20% of batches run at 186 ns but its worst 20% at 1.14 us (6.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 681.6 ns median (-82.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.56x (fastest 681.6 ns, slowest 3788.2 ns)

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
| warm-clamp-acc64 | 3360ns | 3981ns | 1320ns | 3400ns | 5277ns | base |
| warm-clamp-accfit | 2005ns | 1955ns | 1760ns | 1938ns | 2452ns | -40.31% |
| warm-clamp-accfit-dyn | 1972ns | 1908ns | 1343ns | 1921ns | 2751ns | -41.31% |
| warm-clamp-head | 850ns | 870ns | 237ns | 848ns | 1468ns | -74.71% |
| warm-clamp-min-lanes | 1576ns | 1561ns | 1351ns | 1553ns | 1871ns | -53.08% |
| warm-clamp-minimum | 3738ns | 2452ns | 2279ns | 2831ns | 7920ns | +11.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3183ns | 1255ns | 4962ns | base | 2.574 |
| warm-clamp-accfit | 1735ns | 1533ns | 2105ns | -45.49% | 4.722 |
| warm-clamp-accfit-dyn | 1706ns | 1171ns | 2361ns | -46.38% | 4.801 |
| warm-clamp-head | 659ns | 186ns | 1137ns | -79.30% | 12.434 |
| warm-clamp-min-lanes | 1315ns | 1136ns | 1537ns | -58.67% | 6.227 |
| warm-clamp-minimum | 3624ns | 2217ns | 7647ns | +13.88% | 2.260 |

## Performance model

- Peak throughput: **44.031 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.163 | 4.9% |
| warm-clamp-accfit | 4.807 | 10.9% |
| warm-clamp-accfit-dyn | 5.069 | 11.5% |
| warm-clamp-head | 12.019 | 27.3% |
| warm-clamp-min-lanes | 6.279 | 14.3% |
| warm-clamp-minimum | 3.431 | 7.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3360ns | 3360ns | base |
| warm-clamp-accfit | 2005ns | 2005ns | -40.31% |
| warm-clamp-accfit-dyn | 1972ns | 1972ns | -41.31% |
| warm-clamp-head | 850ns | 850ns | -74.71% |
| warm-clamp-min-lanes | 1576ns | 1576ns | -53.08% |
| warm-clamp-minimum | 3738ns | 3738ns | +11.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3788ns | base | --- | [1743, 4154] | --- | --- | --- | --- |
| warm-clamp-accfit | 1704ns | -2174.2ns (-57.4%) | [-2460, -203]ns | [1534, 1753] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-accfit-dyn | 1616ns | -1701.5ns (-44.9%) | [-1982, -438]ns | [1423, 1917] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-head | 682ns | -3204.0ns (-84.6%) | [-3514, -1328]ns | [631, 688] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1305ns | -2529.6ns (-66.8%) | [-2925, -666]ns | [1230, 1367] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2388ns | no significant difference | [-1036, +1076]ns | [2333, 3413] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3793ns | -55.1% | -42.3% | -82.0% | -69.6% | -36.2% |
| 1 | 2 | 1262ns | +52.0% | +1.3% | -49.3% | -2.5% | +84.8% |
| 1 | 3 | 3964ns | -57.0% | -62.3% | -71.2% | -62.7% | -12.5% |
| 1 | 4 | 3866ns | -60.3% | -50.4% | -95.2% | -64.7% | -39.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.408 | moderate- |
| warm-clamp-accfit | -0.034 | ok |
| warm-clamp-accfit-dyn | -0.301 | moderate- |
| warm-clamp-head | -0.524 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | 0.138 | ok |
| warm-clamp-minimum | -0.478 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 60.3ns | 3182.7ns | 1.9% |  |
| warm-clamp-accfit | 92.6ns | 1734.9ns | 5.3% | HIGH |
| warm-clamp-accfit-dyn | 90.4ns | 1706.5ns | 5.3% | HIGH |
| warm-clamp-head | 67.6ns | 658.8ns | 10.3% | HIGH |
| warm-clamp-min-lanes | 90.2ns | 1315.5ns | 6.9% | HIGH |
| warm-clamp-minimum | 36.5ns | 3624.5ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1255.2-4961.9 ns)
   1255.2 |########################################
   1440.5 |
   1625.8 |
   1811.2 |
   1996.5 |
   2181.9 |####
   2367.2 |
   2552.5 |
   2737.9 |####
   2923.2 |
   3108.5 |
   3293.9 |####
   3479.2 |
   3664.6 |######################
   3849.9 |####
   4035.2 |#################
   4220.6 |####
   4405.9 |###################################
   4591.2 |
   4776.6 |
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 1533.0-2104.7 ns)
   1533.0 |########################################
   1561.6 |###
   1590.2 |
   1618.8 |
   1647.4 |
   1676.0 |##################
   1704.5 |######
   1733.1 |#########
   1761.7 |######
   1790.3 |###
   1818.9 |
   1847.4 |
   1876.0 |###
   1904.6 |############
   1933.2 |
   1961.8 |###
   1990.3 |###
   2018.9 |
   2047.5 |
   2076.1 |###
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1170.5-2361.2 ns)
   1170.5 |#############
   1230.0 |#################################
   1289.6 |#############
   1349.1 |##########################
   1408.6 |#############
   1468.2 |
   1527.7 |####################
   1587.2 |#############
   1646.8 |####################
   1706.3 |
   1765.8 |
   1825.4 |
   1884.9 |#################################
   1944.5 |######
   2004.0 |
   2063.5 |
   2123.1 |
   2182.6 |########################################
   2242.1 |######
   2301.7 |
  (1 below, 3 above range)

warm-clamp-head (n=40, range 186.1-1137.0 ns)
    186.1 |########################
    233.6 |
    281.1 |
    328.7 |
    376.2 |
    423.8 |
    471.3 |
    518.9 |
    566.4 |
    614.0 |####################################
    661.5 |########################################
    709.1 |
    756.6 |####
    804.1 |
    851.7 |
    899.2 |
    946.8 |
    994.3 |################
   1041.9 |
   1089.4 |####
  (4 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1136.5-1537.0 ns)
   1136.5 |
   1156.5 |
   1176.5 |###
   1196.5 |###
   1216.6 |########################################
   1236.6 |
   1256.6 |
   1276.7 |###
   1296.7 |#######
   1316.7 |
   1336.8 |
   1356.8 |########################################
   1376.8 |
   1396.8 |
   1416.9 |###
   1436.9 |
   1456.9 |
   1477.0 |
   1497.0 |
   1517.0 |###
  (5 below, 6 above range)

warm-clamp-minimum (n=40, range 2217.1-7646.6 ns)
   2217.1 |########################################
   2488.5 |#
   2760.0 |
   3031.5 |
   3303.0 |#########
   3574.4 |#
   3845.9 |
   4117.4 |
   4388.9 |
   4660.4 |###
   4931.8 |
   5203.3 |
   5474.8 |
   5746.3 |
   6017.7 |
   6289.2 |
   6560.7 |#
   6832.2 |#
   7103.7 |
   7375.1 |
  (2 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=48.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=26.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=50.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 6.1x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
