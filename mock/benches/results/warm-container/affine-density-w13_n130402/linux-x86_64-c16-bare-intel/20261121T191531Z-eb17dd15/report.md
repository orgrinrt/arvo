# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (4.19 us) is 2.5x the fastest (1.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.69)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-headroom, warm-container-minimum} vs {warm-container-plusone} (75% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-headroom, warm-container-minimum} and a slow tier {warm-container-plusone} with a 75% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+10% speed for 1.7x steadier)

warm-container-kernel is fastest (1.64 us, CV 14.7%); warm-container-lanes-deferred gives up 9.9% median for 1.7x lower variance (CV 8.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.4x its best-20%

warm-container-minimum's best 20% of batches run at 1.46 us but its worst 20% at 6.37 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-20 ns, 0.97%)

warm-container-plusone differs from baseline warm-container-headroom by -20 ns (0.97%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1644.1 ns median (-18.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.55x (fastest 1644.1 ns, slowest 4189.4 ns)

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
| Counter frequency | 2112850797 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3277ns | 2163ns | 1521ns | 2902ns | 6155ns | base |
| warm-container-kernel | 1908ns | 1850ns | 1590ns | 1863ns | 2359ns | -41.78% |
| warm-container-lanes-deferred | 2023ns | 2035ns | 1779ns | 2011ns | 2304ns | -38.25% |
| warm-container-minimum | 3672ns | 2473ns | 1507ns | 3401ns | 6650ns | +12.07% |
| warm-container-native | 2160ns | 2079ns | 1534ns | 2127ns | 2884ns | -34.08% |
| warm-container-plusone | 3611ns | 4336ns | 1510ns | 3469ns | 6138ns | +10.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3136ns | 1469ns | 5904ns | base | 7.837 |
| warm-container-kernel | 1687ns | 1414ns | 2077ns | -46.21% | 14.569 |
| warm-container-lanes-deferred | 1789ns | 1581ns | 2018ns | -42.97% | 13.740 |
| warm-container-minimum | 3536ns | 1456ns | 6368ns | +12.75% | 6.950 |
| warm-container-native | 1908ns | 1364ns | 2528ns | -39.17% | 12.882 |
| warm-container-plusone | 3477ns | 1458ns | 5887ns | +10.86% | 7.069 |

## Performance model

- Peak throughput: **18.020 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.194 | 67.7% |
| warm-container-kernel | 14.948 | 83.0% |
| warm-container-lanes-deferred | 13.605 | 75.5% |
| warm-container-minimum | 10.283 | 57.1% |
| warm-container-native | 13.263 | 73.6% |
| warm-container-plusone | 5.866 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3277ns | 3277ns | base |
| warm-container-kernel | 1908ns | 1908ns | -41.78% |
| warm-container-lanes-deferred | 2023ns | 2023ns | -38.25% |
| warm-container-minimum | 3672ns | 3672ns | +12.07% |
| warm-container-native | 2160ns | 2160ns | -34.08% |
| warm-container-plusone | 3611ns | 3611ns | +10.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2015ns | base | --- | [1925, 4169] | --- | --- | --- | --- |
| warm-container-kernel | 1644ns | -396.2ns (-19.7%) | [-2648, -346]ns | [1508, 1762] | YES | 0.0002 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1806ns | no significant difference | [-2362, +3]ns | [1793, 1809] | no | 0.1009 | 0.0807 | 0 |
| warm-container-minimum | 2390ns | no significant difference | [-371, +92]ns | [1460, 5265] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 1853ns | -204.1ns (-10.1%) | [-1804, -155]ns | [1807, 2006] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-plusone | 4189ns | -19.5ns (-1.0%) | [-103, -8]ns | [1785, 4462] | YES (adj: no) | 0.0641 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1472ns | +36.6% | +10.5% | +231.8% | -5.5% | +195.5% |
| 1 | 2 | 4318ns | -65.1% | -58.2% | -30.4% | -45.2% | -2.2% |
| 1 | 3 | 2015ns | -15.4% | -6.9% | -23.6% | -7.7% | -27.3% |
| 1 | 4 | 4210ns | -64.7% | -56.6% | -46.9% | -55.1% | -53.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.693 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.373 | moderate- |
| warm-container-lanes-deferred | 0.074 | ok |
| warm-container-minimum | 0.159 | ok |
| warm-container-native | -0.518 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.203 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 47.4ns | 3136.1ns | 1.5% |  |
| warm-container-kernel | 78.3ns | 1686.8ns | 4.6% |  |
| warm-container-lanes-deferred | 83.0ns | 1788.6ns | 4.6% |  |
| warm-container-minimum | 46.9ns | 3536.1ns | 1.3% |  |
| warm-container-native | 88.5ns | 1907.7ns | 4.6% |  |
| warm-container-plusone | 47.8ns | 3476.7ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1469.5-5904.2 ns)
   1469.5 |#############
   1691.2 |######################
   1912.9 |########################################
   2134.7 |
   2356.4 |
   2578.1 |####
   2799.9 |
   3021.6 |
   3243.3 |####
   3465.1 |
   3686.8 |
   3908.6 |####
   4130.3 |########
   4352.0 |####
   4573.8 |####
   4795.5 |#############
   5017.2 |
   5239.0 |########
   5460.7 |
   5682.4 |####
  (6 below, 4 above range)

warm-container-kernel (n=40, range 1413.5-2077.3 ns)
   1413.5 |#####
   1446.7 |###########
   1479.9 |########################################
   1513.1 |
   1546.3 |#####
   1579.5 |#####
   1612.6 |##################################
   1645.8 |#####
   1679.0 |#####
   1712.2 |#####
   1745.4 |
   1778.6 |############################
   1811.8 |
   1845.0 |
   1878.1 |###########
   1911.3 |
   1944.5 |
   1977.7 |###########
   2010.9 |###########
   2044.1 |
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1581.4-2017.5 ns)
   1581.4 |##
   1603.2 |##
   1625.0 |#################
   1646.8 |
   1668.6 |#####
   1690.4 |
   1712.2 |
   1734.0 |
   1755.8 |
   1777.7 |#####
   1799.5 |########################################
   1821.3 |
   1843.1 |#####
   1864.9 |##
   1886.7 |
   1908.5 |
   1930.3 |##
   1952.1 |##
   1973.9 |
   1995.7 |#####
  (3 below, 4 above range)

warm-container-minimum (n=40, range 1455.7-6368.4 ns)
   1455.7 |########################################
   1701.3 |##
   1946.9 |
   2192.6 |
   2438.2 |
   2683.8 |
   2929.5 |##
   3175.1 |
   3420.8 |
   3666.4 |
   3912.0 |
   4157.7 |
   4403.3 |##
   4649.0 |##
   4894.6 |#####
   5140.2 |##
   5385.9 |##########
   5631.5 |########
   5877.2 |##
   6122.8 |########
  (4 below, 3 above range)

warm-container-native (n=40, range 1363.8-2528.0 ns)
   1363.8 |###############
   1422.1 |##########
   1480.3 |#####
   1538.5 |#####
   1596.7 |#####
   1654.9 |
   1713.1 |
   1771.3 |########################################
   1829.5 |###############
   1887.7 |#####
   1945.9 |#####
   2004.1 |##############################
   2062.3 |
   2120.5 |#####
   2178.7 |##########
   2237.0 |##########
   2295.2 |
   2353.4 |
   2411.6 |
   2469.8 |#####
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1458.1-5887.2 ns)
   1458.1 |########################################
   1679.6 |#########################
   1901.0 |
   2122.5 |#####
   2343.9 |#####
   2565.4 |
   2786.8 |
   3008.3 |
   3229.7 |
   3451.2 |
   3672.6 |
   3894.1 |#####
   4115.6 |#########################
   4337.0 |###############
   4558.5 |###############
   4779.9 |###############
   5001.4 |
   5222.8 |
   5444.3 |##########
   5665.7 |#####
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=60.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-native**: CV=21.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=50.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
