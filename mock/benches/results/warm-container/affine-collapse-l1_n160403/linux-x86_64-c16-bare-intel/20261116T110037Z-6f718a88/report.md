# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 1.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 69% (significant)

warm-container-kernel is -3.57 us (69%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.4x slower than the field

warm-container-headroom (5.20 us) is 3.4x the fastest (1.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.51)

warm-container-native's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (140% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 140% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.4x the fastest

Fastest warm-container-lanes-deferred (1.54 us) to slowest warm-container-headroom (5.20 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-lanes-deferred vs stability leader warm-container-minimum (+9% speed for 1.5x steadier)

warm-container-lanes-deferred is fastest (1.54 us, CV 11.6%); warm-container-minimum gives up 9.3% median for 1.5x lower variance (CV 7.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.5x its best-20%

warm-container-headroom's best 20% of batches run at 1.59 us but its worst 20% at 7.21 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1541.0 ns median (-70.4% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 3.38x (fastest 1541.0 ns, slowest 5203.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112700793 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4382ns | 5388ns | 1647ns | 4239ns | 7544ns | base |
| warm-container-kernel | 1944ns | 2033ns | 1643ns | 1963ns | 2189ns | -55.63% |
| warm-container-lanes-deferred | 1782ns | 1746ns | 1510ns | 1771ns | 2086ns | -59.34% |
| warm-container-minimum | 1925ns | 1892ns | 1700ns | 1925ns | 2149ns | -56.07% |
| warm-container-native | 2089ns | 2033ns | 1847ns | 2065ns | 2403ns | -52.32% |
| warm-container-plusone | 3844ns | 4498ns | 1541ns | 3803ns | 6269ns | -12.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4214ns | 1591ns | 7205ns | base | 7.776 |
| warm-container-kernel | 1715ns | 1458ns | 1905ns | -59.30% | 19.103 |
| warm-container-lanes-deferred | 1571ns | 1335ns | 1837ns | -62.72% | 20.858 |
| warm-container-minimum | 1698ns | 1511ns | 1860ns | -59.71% | 19.300 |
| warm-container-native | 1825ns | 1587ns | 2109ns | -56.68% | 17.950 |
| warm-container-plusone | 3700ns | 1488ns | 5995ns | -12.19% | 8.855 |

## Performance model

- Peak throughput: **24.554 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.298 | 25.6% |
| warm-container-kernel | 18.140 | 73.9% |
| warm-container-lanes-deferred | 21.263 | 86.6% |
| warm-container-minimum | 19.459 | 79.3% |
| warm-container-native | 18.179 | 74.0% |
| warm-container-plusone | 7.547 | 30.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4382ns | 4382ns | base |
| warm-container-kernel | 1944ns | 1944ns | -55.63% |
| warm-container-lanes-deferred | 1782ns | 1782ns | -59.34% |
| warm-container-minimum | 1925ns | 1925ns | -56.07% |
| warm-container-native | 2089ns | 2089ns | -52.32% |
| warm-container-plusone | 3844ns | 3844ns | -12.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5203ns | base | --- | [1795, 5896] | --- | --- | --- | --- |
| warm-container-kernel | 1806ns | -3565.1ns (-68.5%) | [-4239, -199]ns | [1646, 1821] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-lanes-deferred | 1541ns | -3446.9ns (-66.2%) | [-4309, -497]ns | [1504, 1646] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1684ns | -3544.4ns (-68.1%) | [-4240, -287]ns | [1638, 1805] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1802ns | -3247.2ns (-62.4%) | [-4094, -240]ns | [1800, 1827] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 4342ns | -409.8ns (-7.9%) | [-909, -326]ns | [1793, 4965] | YES | 0.0007 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1793ns | +1.6% | -24.5% | -6.7% | -8.4% | +159.5% |
| 1 | 2 | 6476ns | -71.8% | -72.6% | -72.1% | -69.8% | -55.7% |
| 1 | 3 | 5205ns | -68.5% | -68.8% | -68.6% | -65.4% | -16.6% |
| 1 | 4 | 1904ns | -20.6% | -20.2% | -14.0% | -0.6% | +129.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.266 | moderate- |
| warm-container-kernel | 0.274 | moderate+ |
| warm-container-lanes-deferred | -0.371 | moderate- |
| warm-container-minimum | -0.280 | moderate- |
| warm-container-native | -0.511 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.495 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 57.1ns | 4214.1ns | 1.4% |  |
| warm-container-kernel | 81.2ns | 1715.3ns | 4.7% |  |
| warm-container-lanes-deferred | 74.3ns | 1571.0ns | 4.7% |  |
| warm-container-minimum | 81.6ns | 1697.8ns | 4.8% |  |
| warm-container-native | 94.8ns | 1825.5ns | 5.2% | HIGH |
| warm-container-plusone | 50.7ns | 3700.3ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1590.8-7205.4 ns)
   1590.8 |########################################
   1871.5 |
   2152.2 |###
   2433.0 |
   2713.7 |
   2994.4 |###
   3275.1 |
   3555.9 |
   3836.6 |
   4117.3 |
   4398.1 |
   4678.8 |
   4959.5 |###
   5240.2 |######
   5521.0 |######
   5801.7 |##########
   6082.4 |######
   6363.2 |####################
   6643.9 |###
   6924.6 |###
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1458.2-1904.8 ns)
   1458.2 |
   1480.6 |########
   1502.9 |################
   1525.2 |
   1547.5 |####
   1569.9 |
   1592.2 |
   1614.5 |########
   1636.9 |############
   1659.2 |
   1681.5 |
   1703.9 |####
   1726.2 |
   1748.5 |####
   1770.9 |
   1793.2 |####################
   1815.5 |########################################
   1837.8 |####
   1860.2 |########
   1882.5 |############
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1334.6-1837.1 ns)
   1334.6 |
   1359.7 |
   1384.8 |##################################
   1409.9 |
   1435.1 |#####
   1460.2 |
   1485.3 |########################################
   1510.5 |#####
   1535.6 |#####
   1560.7 |
   1585.8 |#####
   1611.0 |
   1636.1 |########################################
   1661.2 |
   1686.4 |#################
   1711.5 |#####
   1736.6 |
   1761.8 |
   1786.9 |#################
   1812.0 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1511.3-1860.2 ns)
   1511.3 |
   1528.7 |
   1546.2 |#####
   1563.6 |#####
   1581.0 |#####
   1598.5 |#####
   1615.9 |#####
   1633.4 |###################################
   1650.8 |
   1668.3 |#####
   1685.7 |#####
   1703.2 |#####
   1720.6 |
   1738.1 |#####
   1755.5 |
   1773.0 |
   1790.4 |####################
   1807.9 |########################################
   1825.3 |
   1842.8 |#####
  (7 below, 4 above range)

warm-container-native (n=40, range 1587.5-2109.5 ns)
   1587.5 |###
   1613.6 |###
   1639.7 |###############
   1665.8 |###
   1691.9 |###
   1718.0 |
   1744.1 |
   1770.2 |###
   1796.3 |########################################
   1822.4 |###
   1848.5 |###
   1874.6 |
   1900.7 |###
   1926.8 |
   1952.9 |###
   1979.0 |############
   2005.1 |
   2031.2 |
   2057.3 |######
   2083.4 |###
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1487.9-5995.3 ns)
   1487.9 |####
   1713.3 |########################################
   1938.6 |
   2164.0 |
   2389.4 |
   2614.7 |
   2840.1 |
   3065.5 |
   3290.9 |
   3516.2 |
   3741.6 |########
   3967.0 |####
   4192.3 |
   4417.7 |########
   4643.1 |####
   4868.4 |######################
   5093.8 |
   5319.2 |######################
   5544.5 |####
   5769.9 |########
  (7 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=50.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
