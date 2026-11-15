# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 39% (significant)

warm-container-lanes-deferred is -926 ns (39%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.56)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-container-lanes-deferred vs stability leader warm-container-kernel (+2% speed for 1.6x steadier)

warm-container-lanes-deferred is fastest (1.56 us, CV 19.4%); warm-container-kernel gives up 1.5% median for 1.6x lower variance (CV 12.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.0x its best-20%

warm-container-headroom's best 20% of batches run at 1.98 us but its worst 20% at 7.91 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1560.8 ns median (-33.7% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.52x (fastest 1560.8 ns, slowest 2374.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4035ns | 2417ns | 2033ns | 3319ns | 8185ns | base |
| warm-container-kernel | 1791ns | 1812ns | 1439ns | 1798ns | 2123ns | -55.60% |
| warm-container-lanes-deferred | 1919ns | 1809ns | 1639ns | 1814ns | 2514ns | -52.44% |
| warm-container-minimum | 3592ns | 2087ns | 1848ns | 3059ns | 6935ns | -10.97% |
| warm-container-native | 2059ns | 1965ns | 1783ns | 1968ns | 2609ns | -48.97% |
| warm-container-plusone | 4076ns | 2459ns | 1841ns | 3769ns | 7230ns | +1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3917ns | 1980ns | 7908ns | base | 18.823 |
| warm-container-kernel | 1555ns | 1258ns | 1829ns | -60.30% | 47.416 |
| warm-container-lanes-deferred | 1644ns | 1377ns | 2185ns | -58.02% | 44.835 |
| warm-container-minimum | 3444ns | 1785ns | 6604ns | -12.07% | 21.408 |
| warm-container-native | 1790ns | 1556ns | 2265ns | -54.31% | 41.197 |
| warm-container-plusone | 3924ns | 1778ns | 6923ns | +0.18% | 18.789 |

## Performance model

- Peak throughput: **58.626 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 31.321 | 53.4% |
| warm-container-kernel | 46.522 | 79.4% |
| warm-container-lanes-deferred | 47.239 | 80.6% |
| warm-container-minimum | 37.226 | 63.5% |
| warm-container-native | 42.950 | 73.3% |
| warm-container-plusone | 31.045 | 53.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4035ns | 4035ns | base |
| warm-container-kernel | 1791ns | 1791ns | -55.60% |
| warm-container-lanes-deferred | 1919ns | 1919ns | -52.44% |
| warm-container-minimum | 3592ns | 3592ns | -10.97% |
| warm-container-native | 2059ns | 2059ns | -48.97% |
| warm-container-plusone | 4076ns | 4076ns | +1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2354ns | base | --- | [2353, 3504] | --- | --- | --- | --- |
| warm-container-kernel | 1585ns | -919.7ns (-39.1%) | [-2017, -775]ns | [1572, 1591] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1561ns | -926.3ns (-39.4%) | [-2041, -540]ns | [1475, 1591] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1981ns | -558.8ns (-23.7%) | [-581, -373]ns | [1803, 3650] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1717ns | -772.4ns (-32.8%) | [-1882, -567]ns | [1608, 1766] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2375ns | -558.1ns (-23.7%) | [-571, -206]ns | [1792, 6156] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1926ns | -10.1% | -20.4% | +55.3% | -18.1% | +44.1% |
| 1 | 2 | 2354ns | -32.9% | -12.1% | -15.9% | -11.4% | +16.1% |
| 1 | 3 | 2366ns | -32.6% | -40.2% | -24.3% | -29.8% | +49.0% |
| 1 | 4 | 3503ns | -63.7% | -56.2% | -24.6% | -53.3% | -32.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.016 | ok |
| warm-container-kernel | -0.048 | ok |
| warm-container-lanes-deferred | -0.459 | moderate- |
| warm-container-minimum | -0.204 | moderate- |
| warm-container-native | -0.463 | moderate- |
| warm-container-plusone | -0.560 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.7ns | 3916.9ns | 1.1% |  |
| warm-container-kernel | 85.6ns | 1554.9ns | 5.5% | HIGH |
| warm-container-lanes-deferred | 97.7ns | 1644.4ns | 5.9% | HIGH |
| warm-container-minimum | 50.8ns | 3443.9ns | 1.5% |  |
| warm-container-native | 97.6ns | 1789.6ns | 5.5% | HIGH |
| warm-container-plusone | 53.5ns | 3924.0ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1980.1-7908.5 ns)
   1980.1 |
   2276.5 |########################################
   2572.9 |
   2869.3 |
   3165.8 |
   3462.2 |
   3758.6 |
   4055.0 |
   4351.4 |##
   4647.9 |##
   4944.3 |
   5240.7 |
   5537.1 |##
   5833.5 |
   6130.0 |##
   6426.4 |##
   6722.8 |
   7019.2 |####
   7315.6 |##
   7612.0 |######
  (7 below, 3 above range)

warm-container-kernel (n=40, range 1257.6-1828.7 ns)
   1257.6 |
   1286.2 |###
   1314.7 |#########
   1343.3 |###
   1371.8 |
   1400.4 |
   1428.9 |#########
   1457.5 |
   1486.1 |
   1514.6 |
   1543.2 |###
   1571.7 |########################################
   1600.3 |###
   1628.8 |######
   1657.4 |
   1686.0 |###
   1714.5 |
   1743.1 |############
   1771.6 |
   1800.2 |######
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1377.1-2185.4 ns)
   1377.1 |#############
   1417.6 |#################################
   1458.0 |#################################
   1498.4 |#############
   1538.8 |####################
   1579.2 |########################################
   1619.6 |#############
   1660.0 |
   1700.4 |
   1740.9 |
   1781.3 |######
   1821.7 |######
   1862.1 |
   1902.5 |
   1942.9 |######
   1983.3 |#############
   2023.7 |
   2064.2 |
   2104.6 |
   2145.0 |######
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1784.6-6604.1 ns)
   1784.6 |########################################
   2025.6 |
   2266.6 |####
   2507.5 |
   2748.5 |##
   2989.5 |##
   3230.4 |
   3471.4 |
   3712.4 |
   3953.4 |##
   4194.3 |
   4435.3 |
   4676.3 |
   4917.2 |
   5158.2 |
   5399.2 |##
   5640.2 |
   5881.1 |#########
   6122.1 |
   6363.1 |##############
  (5 below, 2 above range)

warm-container-native (n=40, range 1556.2-2265.0 ns)
   1556.2 |########################################
   1591.7 |###############
   1627.1 |##########
   1662.6 |##########
   1698.0 |
   1733.4 |###################################
   1768.9 |##########
   1804.3 |#####
   1839.7 |
   1875.2 |
   1910.6 |#####
   1946.0 |
   1981.5 |##########
   2016.9 |#####
   2052.3 |
   2087.8 |
   2123.2 |#####
   2158.7 |
   2194.1 |
   2229.5 |
  (5 below, 5 above range)

warm-container-plusone (n=40, range 1777.9-6922.8 ns)
   1777.9 |########################################
   2035.1 |
   2292.4 |
   2549.6 |
   2806.9 |##
   3064.1 |
   3321.4 |
   3578.6 |#####
   3835.9 |
   4093.1 |
   4350.4 |
   4607.6 |
   4864.8 |
   5122.1 |##
   5379.3 |
   5636.6 |
   5893.8 |#####
   6151.1 |#######
   6408.3 |#################
   6665.6 |##
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=61.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=59.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=58.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
