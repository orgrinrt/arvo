# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 63% (significant)

warm-container-minimum is -2.71 us (63%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.9x slower than the field

warm-container-headroom (4.33 us) is 2.9x the fastest (1.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.75)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-plusone, warm-container-lanes-deferred} vs {warm-container-headroom} (139% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-plusone, warm-container-lanes-deferred} and a slow tier {warm-container-headroom} with a 139% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+9% speed for 1.8x steadier)

warm-container-minimum is fastest (1.51 us, CV 15.9%); warm-container-native gives up 9.4% median for 1.8x lower variance (CV 8.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 3.9x its best-20%

warm-container-headroom's best 20% of batches run at 1.55 us but its worst 20% at 6.02 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1507.0 ns median (-65.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.87x (fastest 1507.0 ns, slowest 4325.0 ns)

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
| Counter frequency | 2112787121 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3776ns | 4476ns | 1605ns | 3666ns | 6276ns | base |
| warm-container-kernel | 1909ns | 1915ns | 1547ns | 1916ns | 2253ns | -49.43% |
| warm-container-lanes-deferred | 2145ns | 2038ns | 1689ns | 2085ns | 2784ns | -43.18% |
| warm-container-minimum | 1863ns | 1726ns | 1565ns | 1810ns | 2322ns | -50.65% |
| warm-container-native | 1897ns | 1855ns | 1662ns | 1897ns | 2131ns | -49.76% |
| warm-container-plusone | 3357ns | 1871ns | 1855ns | 2680ns | 6888ns | -11.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3637ns | 1551ns | 6024ns | base | 9.010 |
| warm-container-kernel | 1686ns | 1368ns | 1966ns | -53.64% | 19.436 |
| warm-container-lanes-deferred | 1896ns | 1501ns | 2453ns | -47.86% | 17.280 |
| warm-container-minimum | 1640ns | 1390ns | 2040ns | -54.91% | 19.981 |
| warm-container-native | 1675ns | 1475ns | 1857ns | -53.94% | 19.560 |
| warm-container-plusone | 3234ns | 1792ns | 6615ns | -11.07% | 10.131 |

## Performance model

- Peak throughput: **23.950 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.576 | 31.6% |
| warm-container-kernel | 19.225 | 80.3% |
| warm-container-lanes-deferred | 18.085 | 75.5% |
| warm-container-minimum | 21.744 | 90.8% |
| warm-container-native | 19.874 | 83.0% |
| warm-container-plusone | 18.152 | 75.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3776ns | 3776ns | base |
| warm-container-kernel | 1909ns | 1909ns | -49.43% |
| warm-container-lanes-deferred | 2145ns | 2145ns | -43.18% |
| warm-container-minimum | 1863ns | 1863ns | -50.65% |
| warm-container-native | 1897ns | 1897ns | -49.76% |
| warm-container-plusone | 3357ns | 3357ns | -11.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4325ns | base | --- | [1801, 4742] | --- | --- | --- | --- |
| warm-container-kernel | 1704ns | -2475.8ns (-57.2%) | [-3033, -215]ns | [1642, 1808] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1812ns | -2559.1ns (-59.2%) | [-2917, -22]ns | [1805, 1855] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-minimum | 1507ns | -2713.4ns (-62.7%) | [-3152, -290]ns | [1502, 1638] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1649ns | -2523.0ns (-58.3%) | [-2996, -309]ns | [1644, 1797] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1805ns | no significant difference | [-8, +378]ns | [1793, 3152] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3303ns | -50.2% | -31.6% | -39.4% | -54.6% | -45.7% |
| 1 | 2 | 4325ns | -57.0% | -65.0% | -65.4% | -58.2% | +18.2% |
| 1 | 3 | 2352ns | -23.6% | -23.1% | -34.6% | -28.6% | +22.3% |
| 1 | 4 | 4505ns | -69.1% | -59.7% | -66.6% | -63.5% | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.749 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.132 | ok |
| warm-container-lanes-deferred | -0.433 | moderate- |
| warm-container-minimum | -0.136 | ok |
| warm-container-native | -0.430 | moderate- |
| warm-container-plusone | -0.335 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 48.8ns | 3636.8ns | 1.3% |  |
| warm-container-kernel | 77.9ns | 1685.9ns | 4.6% |  |
| warm-container-lanes-deferred | 92.3ns | 1896.3ns | 4.9% |  |
| warm-container-minimum | 102.4ns | 1639.9ns | 6.2% | HIGH |
| warm-container-native | 78.1ns | 1675.2ns | 4.7% |  |
| warm-container-plusone | 43.2ns | 3234.3ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1551.2-6023.6 ns)
   1551.2 |
   1774.8 |########################################
   1998.4 |
   2222.1 |
   2445.7 |
   2669.3 |
   2892.9 |###
   3116.5 |
   3340.2 |
   3563.8 |
   3787.4 |
   4011.0 |###
   4234.7 |##########
   4458.3 |##########
   4681.9 |#######
   4905.5 |##########
   5129.1 |###
   5352.8 |###
   5576.4 |##########
   5800.0 |#######
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1368.2-1966.2 ns)
   1368.2 |######################
   1398.1 |
   1428.0 |
   1457.9 |#####
   1487.8 |#################
   1517.7 |#####
   1547.6 |
   1577.5 |
   1607.4 |
   1637.3 |########################################
   1667.2 |
   1697.1 |#################
   1727.0 |
   1756.9 |#####
   1786.8 |########################################
   1816.7 |#####
   1846.6 |######################
   1876.5 |
   1906.4 |
   1936.3 |
  (3 below, 5 above range)

warm-container-lanes-deferred (n=40, range 1501.5-2452.6 ns)
   1501.5 |##############
   1549.0 |##
   1596.6 |
   1644.1 |#####
   1691.7 |
   1739.2 |
   1786.8 |########################################
   1834.4 |########
   1881.9 |
   1929.5 |
   1977.0 |###########
   2024.6 |#####
   2072.1 |
   2119.7 |
   2167.3 |
   2214.8 |########
   2262.4 |
   2309.9 |
   2357.5 |
   2405.1 |
  (2 below, 4 above range)

warm-container-minimum (n=40, range 1389.8-2040.4 ns)
   1389.8 |######
   1422.4 |
   1454.9 |
   1487.4 |########################################
   1519.9 |
   1552.5 |######
   1585.0 |
   1617.5 |############
   1650.1 |
   1682.6 |###
   1715.1 |
   1747.6 |###
   1780.2 |######
   1812.7 |
   1845.2 |######
   1877.7 |###
   1910.3 |
   1942.8 |
   1975.3 |#########
   2007.8 |
  (6 below, 3 above range)

warm-container-native (n=40, range 1475.4-1856.8 ns)
   1475.4 |####
   1494.5 |######################
   1513.5 |####
   1532.6 |####
   1551.7 |
   1570.8 |
   1589.8 |####
   1608.9 |####
   1628.0 |###############################
   1647.0 |######################
   1666.1 |
   1685.2 |
   1704.2 |####
   1723.3 |
   1742.4 |
   1761.5 |
   1780.5 |########
   1799.6 |########################################
   1818.7 |
   1837.7 |
  (2 below, 4 above range)

warm-container-plusone (n=40, range 1791.9-6615.3 ns)
   1791.9 |########################################
   2033.1 |
   2274.2 |
   2515.4 |
   2756.6 |
   2997.8 |###
   3238.9 |
   3480.1 |
   3721.3 |#
   3962.4 |
   4203.6 |
   4444.8 |
   4685.9 |
   4927.1 |#
   5168.3 |###
   5409.4 |###
   5650.6 |
   5891.8 |
   6132.9 |#
   6374.1 |#####
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=49.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=62.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
