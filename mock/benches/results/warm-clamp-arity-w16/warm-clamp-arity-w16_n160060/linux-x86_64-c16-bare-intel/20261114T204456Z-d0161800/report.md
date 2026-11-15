# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 72% faster than the next best (warm-clamp-accfit)

warm-clamp-head (892 ns) leads warm-clamp-accfit (1.54 us) by 72%, a clear separation rather than a photo finish. CV 18.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 73% (significant)

warm-clamp-head is -2.54 us (73%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 5.5x slower than the field

warm-clamp-minimum (4.89 us) is 5.5x the fastest (892 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.64)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest warm-clamp-head (892 ns) to slowest warm-clamp-minimum (4.89 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.62 us but its worst 20% at 6.30 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 892.0 ns median (-74.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.48x (fastest 892.0 ns, slowest 4886.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3717ns | 3691ns | 3198ns | 3728ns | 4201ns | base |
| warm-clamp-accfit | 1814ns | 1745ns | 1588ns | 1755ns | 2218ns | -51.19% |
| warm-clamp-accfit-dyn | 2078ns | 1944ns | 1513ns | 2007ns | 2853ns | -44.09% |
| warm-clamp-head | 1203ns | 1099ns | 1036ns | 1136ns | 1568ns | -67.64% |
| warm-clamp-min-lanes | 1778ns | 1752ns | 1517ns | 1772ns | 2054ns | -52.17% |
| warm-clamp-minimum | 4117ns | 5061ns | 1678ns | 4112ns | 6572ns | +10.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3494ns | 3017ns | 3900ns | base | 2.344 |
| warm-clamp-accfit | 1586ns | 1400ns | 1938ns | -54.62% | 5.166 |
| warm-clamp-accfit-dyn | 1820ns | 1334ns | 2481ns | -47.93% | 4.502 |
| warm-clamp-head | 953ns | 832ns | 1244ns | -72.73% | 8.597 |
| warm-clamp-min-lanes | 1561ns | 1341ns | 1793ns | -55.33% | 5.249 |
| warm-clamp-minimum | 3967ns | 1620ns | 6297ns | +13.52% | 2.065 |

## Performance model

- Peak throughput: **9.850 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.353 | 23.9% |
| warm-clamp-accfit | 5.328 | 54.1% |
| warm-clamp-accfit-dyn | 4.772 | 48.4% |
| warm-clamp-head | 9.183 | 93.2% |
| warm-clamp-min-lanes | 5.300 | 53.8% |
| warm-clamp-minimum | 1.676 | 17.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3717ns | 3717ns | base |
| warm-clamp-accfit | 1814ns | 1814ns | -51.19% |
| warm-clamp-accfit-dyn | 2078ns | 2078ns | -44.09% |
| warm-clamp-head | 1203ns | 1203ns | -67.64% |
| warm-clamp-min-lanes | 1778ns | 1778ns | -52.17% |
| warm-clamp-minimum | 4117ns | 4117ns | +10.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3482ns | base | --- | [3350, 3779] | --- | --- | --- | --- |
| warm-clamp-accfit | 1537ns | -1897.3ns (-54.5%) | [-2016, -1740]ns | [1471, 1538] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1717ns | -1739.7ns (-50.0%) | [-1882, -1488]ns | [1687, 1859] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 892ns | -2543.8ns (-73.1%) | [-2632, -2361]ns | [834, 918] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1546ns | -1919.4ns (-55.1%) | [-2090, -1793]ns | [1449, 1657] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 4886ns | no significant difference | [-1129, +1691]ns | [2041, 5409] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3337ns | -53.9% | -27.8% | -72.5% | -55.6% | +49.0% |
| 1 | 2 | 3513ns | -58.9% | -52.0% | -76.3% | -54.8% | -52.9% |
| 1 | 3 | 3749ns | -61.2% | -64.3% | -67.8% | -64.5% | +30.5% |
| 1 | 4 | 3606ns | -47.9% | -48.4% | -76.9% | -52.2% | +48.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.127 | ok |
| warm-clamp-accfit | -0.120 | ok |
| warm-clamp-accfit-dyn | -0.052 | ok |
| warm-clamp-head | -0.592 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.638 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.330 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 78.0ns | 3494.4ns | 2.2% |  |
| warm-clamp-accfit | 81.6ns | 1585.6ns | 5.1% | HIGH |
| warm-clamp-accfit-dyn | 91.4ns | 1819.6ns | 5.0% | HIGH |
| warm-clamp-head | 88.6ns | 952.9ns | 9.3% | HIGH |
| warm-clamp-min-lanes | 79.1ns | 1560.8ns | 5.1% | HIGH |
| warm-clamp-minimum | 48.4ns | 3966.7ns | 1.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 3016.7-3900.4 ns)
   3016.7 |
   3060.8 |######
   3105.0 |######
   3149.2 |##########################
   3193.4 |####################
   3237.6 |######
   3281.8 |
   3326.0 |
   3370.2 |
   3414.3 |#############
   3458.5 |#################################
   3502.7 |######
   3546.9 |#############
   3591.1 |
   3635.3 |
   3679.5 |######
   3723.7 |######
   3767.8 |########################################
   3812.0 |#################################
   3856.2 |######
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 1400.4-1937.6 ns)
   1400.4 |###########################
   1427.3 |######
   1454.1 |######
   1481.0 |
   1507.9 |
   1534.7 |########################################
   1561.6 |###
   1588.4 |
   1615.3 |
   1642.1 |
   1669.0 |#########
   1695.9 |
   1722.7 |
   1749.6 |
   1776.4 |###
   1803.3 |###
   1830.1 |
   1857.0 |#########
   1883.9 |
   1910.7 |
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1334.4-2481.3 ns)
   1334.4 |########
   1391.7 |#############
   1449.1 |####
   1506.4 |
   1563.7 |
   1621.1 |####
   1678.4 |########################################
   1735.8 |#################
   1793.1 |########
   1850.5 |########
   1907.8 |########
   1965.2 |
   2022.5 |
   2079.8 |#############
   2137.2 |####
   2194.5 |
   2251.9 |
   2309.2 |
   2366.6 |########
   2423.9 |####
  (4 below, 3 above range)

warm-clamp-head (n=40, range 831.6-1243.8 ns)
    831.6 |########################################
    852.2 |##
    872.8 |##
    893.5 |#####
    914.1 |#####################
    934.7 |
    955.3 |
    975.9 |
    996.5 |
   1017.1 |
   1037.7 |
   1058.3 |
   1078.9 |
   1099.5 |
   1120.1 |##
   1140.7 |##########
   1161.3 |
   1181.9 |
   1202.5 |
   1223.1 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1340.7-1793.2 ns)
   1340.7 |#####
   1363.3 |
   1386.0 |
   1408.6 |########################################
   1431.2 |
   1453.8 |#####
   1476.5 |
   1499.1 |
   1521.7 |##########
   1544.3 |#########################
   1567.0 |#####
   1589.6 |##########
   1612.2 |#####
   1634.8 |
   1657.5 |
   1680.1 |###################################
   1702.7 |#####
   1725.3 |#####
   1748.0 |##########
   1770.6 |
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 1620.4-6296.8 ns)
   1620.4 |########################################
   1854.2 |######################
   2088.0 |
   2321.8 |#####
   2555.6 |
   2789.5 |
   3023.3 |
   3257.1 |
   3490.9 |
   3724.7 |
   3958.6 |
   4192.4 |
   4426.2 |#####
   4660.0 |###########
   4893.8 |######################
   5127.7 |#####
   5361.5 |############################
   5595.3 |###########
   5829.1 |######################
   6063.0 |###########
  (5 below, 2 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: CV=22.3% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=49.2% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
