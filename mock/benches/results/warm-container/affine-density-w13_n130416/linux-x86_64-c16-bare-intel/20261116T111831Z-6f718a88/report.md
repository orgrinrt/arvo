# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 39% (significant)

warm-container-native is -914 ns (39%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.9x slower than the field

warm-container-minimum (4.45 us) is 2.9x the fastest (1.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.56)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-container-native vs stability leader warm-container-kernel (+1% speed for 1.4x steadier)

warm-container-native is fastest (1.54 us, CV 14.4%); warm-container-kernel gives up 1.2% median for 1.4x lower variance (CV 10.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.3x its best-20%

warm-container-plusone's best 20% of batches run at 1.46 us but its worst 20% at 6.28 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1541.7 ns median (-34.5% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.88x (fastest 1541.7 ns, slowest 4445.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112561731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4093ns | 2416ns | 1975ns | 3499ns | 7991ns | base |
| warm-container-kernel | 1783ns | 1787ns | 1513ns | 1783ns | 2054ns | -56.43% |
| warm-container-lanes-deferred | 1848ns | 1809ns | 1502ns | 1833ns | 2239ns | -54.85% |
| warm-container-minimum | 4008ns | 4606ns | 1758ns | 3967ns | 6384ns | -2.07% |
| warm-container-native | 1801ns | 1789ns | 1500ns | 1761ns | 2225ns | -55.99% |
| warm-container-plusone | 3686ns | 3754ns | 1517ns | 3455ns | 6548ns | -9.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3976ns | 1924ns | 7743ns | base | 35.025 |
| warm-container-kernel | 1547ns | 1320ns | 1766ns | -61.09% | 90.017 |
| warm-container-lanes-deferred | 1604ns | 1313ns | 1934ns | -59.66% | 86.823 |
| warm-container-minimum | 3860ns | 1698ns | 6123ns | -2.92% | 36.080 |
| warm-container-native | 1566ns | 1312ns | 1928ns | -60.61% | 88.927 |
| warm-container-plusone | 3548ns | 1465ns | 6280ns | -10.76% | 39.248 |

## Performance model

- Peak throughput: **106.148 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 59.198 | 55.8% |
| warm-container-kernel | 89.240 | 84.1% |
| warm-container-lanes-deferred | 88.136 | 83.0% |
| warm-container-minimum | 31.324 | 29.5% |
| warm-container-native | 90.331 | 85.1% |
| warm-container-plusone | 38.410 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4093ns | 4093ns | base |
| warm-container-kernel | 1783ns | 1783ns | -56.43% |
| warm-container-lanes-deferred | 1848ns | 1848ns | -54.85% |
| warm-container-minimum | 4008ns | 4008ns | -2.07% |
| warm-container-native | 1801ns | 1801ns | -55.99% |
| warm-container-plusone | 3686ns | 3686ns | -9.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2352ns | base | --- | [2351, 4535] | --- | --- | --- | --- |
| warm-container-kernel | 1561ns | -790.7ns (-33.6%) | [-3034, -770]ns | [1556, 1563] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1580ns | -795.6ns (-33.8%) | [-3021, -632]ns | [1554, 1618] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4446ns | -574.0ns (-24.4%) | [-954, -305]ns | [1792, 5076] | YES | 0.0022 | 0.0022 | 0 |
| warm-container-native | 1542ns | -913.9ns (-38.8%) | [-3155, -768]ns | [1439, 1584] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3626ns | -562.5ns (-23.9%) | [-892, -290]ns | [1791, 4690] | YES | 0.0008 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2374ns | -44.4% | -44.5% | +80.8% | -44.4% | -24.3% |
| 1 | 2 | 1930ns | -19.3% | -18.0% | +178.7% | -2.6% | +121.1% |
| 1 | 3 | 2360ns | -31.8% | -33.9% | +86.2% | -32.9% | +15.2% |
| 1 | 4 | 2787ns | -43.9% | -34.8% | -35.8% | -48.4% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.014 | ok |
| warm-container-kernel | 0.013 | ok |
| warm-container-lanes-deferred | -0.048 | ok |
| warm-container-minimum | 0.020 | ok |
| warm-container-native | -0.403 | moderate- |
| warm-container-plusone | -0.555 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.8ns | 3976.1ns | 1.1% |  |
| warm-container-kernel | 85.1ns | 1547.1ns | 5.5% | HIGH |
| warm-container-lanes-deferred | 115.3ns | 1604.0ns | 7.2% | HIGH |
| warm-container-minimum | 52.4ns | 3859.9ns | 1.4% |  |
| warm-container-native | 85.9ns | 1566.0ns | 5.5% | HIGH |
| warm-container-plusone | 48.1ns | 3548.3ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1924.2-7743.4 ns)
   1924.2 |#########################
   2215.1 |########################################
   2506.1 |
   2797.0 |#######
   3088.0 |###
   3379.0 |
   3669.9 |
   3960.9 |
   4251.9 |
   4542.8 |
   4833.8 |
   5124.7 |
   5415.7 |
   5706.7 |###
   5997.6 |###
   6288.6 |###
   6579.6 |#######
   6870.5 |#######
   7161.5 |#######
   7452.4 |#######
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1320.4-1766.4 ns)
   1320.4 |##
   1342.7 |
   1365.0 |##
   1387.3 |
   1409.6 |######
   1431.9 |
   1454.2 |
   1476.5 |##
   1498.8 |
   1521.1 |
   1543.4 |########################################
   1565.7 |####
   1588.0 |##
   1610.3 |##
   1632.6 |##
   1654.9 |##
   1677.2 |
   1699.5 |
   1721.8 |######
   1744.1 |
  (5 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1312.6-1934.1 ns)
   1312.6 |#########################
   1343.7 |
   1374.8 |
   1405.8 |##########
   1436.9 |
   1468.0 |#####
   1499.1 |
   1530.1 |########################################
   1561.2 |##############################
   1592.3 |####################
   1623.3 |
   1654.4 |#####
   1685.5 |
   1716.6 |
   1747.6 |#########################
   1778.7 |
   1809.8 |
   1840.9 |
   1871.9 |#####
   1903.0 |
  (2 below, 5 above range)

warm-container-minimum (n=40, range 1697.6-6123.0 ns)
   1697.6 |########################################
   1918.9 |
   2140.1 |
   2361.4 |
   2582.7 |
   2803.9 |
   3025.2 |
   3246.5 |
   3467.7 |
   3689.0 |##
   3910.3 |##
   4131.6 |#####
   4352.8 |
   4574.1 |#####
   4795.4 |###########
   5016.6 |##
   5237.9 |##############
   5459.2 |##
   5680.4 |########
   5901.7 |
  (2 below, 4 above range)

warm-container-native (n=40, range 1312.0-1928.5 ns)
   1312.0 |########################################
   1342.8 |#####
   1373.6 |#####
   1404.4 |
   1435.3 |########################################
   1466.1 |###########
   1496.9 |#####
   1527.7 |
   1558.6 |########################################
   1589.4 |
   1620.2 |###########
   1651.0 |#####
   1681.9 |#####
   1712.7 |#####
   1743.5 |###########
   1774.3 |
   1805.2 |#####
   1836.0 |
   1866.8 |
   1897.6 |
  (1 below, 5 above range)

warm-container-plusone (n=40, range 1464.9-6280.3 ns)
   1464.9 |#######
   1705.7 |########################################
   1946.5 |
   2187.2 |
   2428.0 |
   2668.8 |
   2909.5 |
   3150.3 |
   3391.1 |#######
   3631.8 |
   3872.6 |
   4113.4 |##########
   4354.2 |###
   4594.9 |#######
   4835.7 |###
   5076.5 |###
   5317.2 |#######
   5558.0 |###
   5798.8 |#######
   6039.6 |#######
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=61.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=47.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=54.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.3x (possible bimodal distribution)
