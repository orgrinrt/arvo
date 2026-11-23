# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 40% (significant)

warm-container-native is -1.12 us (40%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 2.3x slower than the field

warm-container-kernel (3.98 us) is 2.3x the fastest (1.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.56)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-plusone is inconsistent: worst-20% is 4.2x its best-20%

warm-container-plusone's best 20% of batches run at 1.55 us but its worst 20% at 6.56 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1733.4 ns median (-38.5% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.30x (fastest 1733.4 ns, slowest 3979.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112855779 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4113ns | 2915ns | 1856ns | 3933ns | 6911ns | base |
| warm-container-kernel | 3711ns | 4136ns | 1605ns | 3594ns | 6166ns | -9.79% |
| warm-container-lanes-deferred | 3757ns | 3846ns | 1682ns | 3650ns | 6152ns | -8.66% |
| warm-container-minimum | 2129ns | 2032ns | 1697ns | 2075ns | 2720ns | -48.25% |
| warm-container-native | 1940ns | 1980ns | 1658ns | 1928ns | 2258ns | -52.84% |
| warm-container-plusone | 3641ns | 1869ns | 1609ns | 3252ns | 6837ns | -11.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3963ns | 1793ns | 6621ns | base | 8.268 |
| warm-container-kernel | 3562ns | 1545ns | 5900ns | -10.12% | 9.199 |
| warm-container-lanes-deferred | 3606ns | 1620ns | 5879ns | -9.00% | 9.086 |
| warm-container-minimum | 1881ns | 1507ns | 2395ns | -52.53% | 17.419 |
| warm-container-native | 1713ns | 1473ns | 1971ns | -56.77% | 19.126 |
| warm-container-plusone | 3506ns | 1554ns | 6563ns | -11.53% | 9.346 |

## Performance model

- Peak throughput: **22.250 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.631 | 52.3% |
| warm-container-kernel | 8.234 | 37.0% |
| warm-container-lanes-deferred | 8.848 | 39.8% |
| warm-container-minimum | 18.169 | 81.7% |
| warm-container-native | 18.903 | 85.0% |
| warm-container-plusone | 18.145 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4113ns | 4113ns | base |
| warm-container-kernel | 3711ns | 3711ns | -9.79% |
| warm-container-lanes-deferred | 3757ns | 3757ns | -8.66% |
| warm-container-minimum | 2129ns | 2129ns | -48.25% |
| warm-container-native | 1940ns | 1940ns | -52.84% |
| warm-container-plusone | 3641ns | 3641ns | -11.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2817ns | base | --- | [1796, 5969] | --- | --- | --- | --- |
| warm-container-kernel | 3980ns | -469.5ns (-16.7%) | [-702, -177]ns | [1631, 4822] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-lanes-deferred | 3703ns | -263.0ns (-9.3%) | [-692, -174]ns | [1669, 4472] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-minimum | 1804ns | -914.8ns (-32.5%) | [-3801, -266]ns | [1793, 1955] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-native | 1733ns | -1119.6ns (-39.7%) | [-4165, -289]ns | [1643, 1809] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 1806ns | -151.7ns (-5.4%) | [-327, -1]ns | [1795, 4630] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1968ns | +43.5% | +29.9% | -23.6% | -23.4% | -8.6% |
| 1 | 2 | 4162ns | +0.1% | -26.2% | -56.7% | -61.7% | -56.7% |
| 1 | 3 | 1801ns | +151.3% | +112.4% | +2.2% | +0.6% | -17.2% |
| 1 | 4 | 5769ns | -65.7% | -42.4% | -61.0% | -67.4% | -20.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.563 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.269 | moderate- |
| warm-container-lanes-deferred | 0.097 | ok |
| warm-container-minimum | 0.046 | ok |
| warm-container-native | 0.308 | moderate+ |
| warm-container-plusone | -0.166 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 53.1ns | 3963.1ns | 1.3% |  |
| warm-container-kernel | 51.8ns | 3562.1ns | 1.5% |  |
| warm-container-lanes-deferred | 53.5ns | 3606.5ns | 1.5% |  |
| warm-container-minimum | 87.2ns | 1881.1ns | 4.6% |  |
| warm-container-native | 81.3ns | 1713.3ns | 4.7% |  |
| warm-container-plusone | 46.8ns | 3506.0ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1792.8-6620.9 ns)
   1792.8 |########################################
   2034.2 |##
   2275.6 |
   2517.1 |##
   2758.5 |##
   2999.9 |
   3241.3 |
   3482.7 |
   3724.1 |
   3965.5 |
   4206.9 |
   4448.3 |
   4689.7 |
   4931.1 |##
   5172.5 |##
   5413.9 |##
   5655.3 |##
   5896.7 |########
   6138.1 |##
   6379.5 |######################
  (4 below, 3 above range)

warm-container-kernel (n=40, range 1544.6-5900.4 ns)
   1544.6 |########################################
   1762.4 |##
   1980.2 |
   2198.0 |##
   2415.8 |
   2633.6 |
   2851.4 |
   3069.2 |
   3286.9 |
   3504.7 |
   3722.5 |##
   3940.3 |#####
   4158.1 |########
   4375.9 |##
   4593.7 |##
   4811.5 |##
   5029.2 |##
   5247.0 |#####
   5464.8 |##
   5682.6 |######################
  (2 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1619.5-5878.9 ns)
   1619.5 |########################################
   1832.5 |
   2045.5 |
   2258.5 |
   2471.4 |
   2684.4 |####
   2897.4 |
   3110.3 |
   3323.3 |########
   3536.3 |########
   3749.2 |############
   3962.2 |####
   4175.2 |####
   4388.1 |########
   4601.1 |
   4814.1 |####
   5027.0 |
   5240.0 |########
   5453.0 |####
   5666.0 |############################
  (5 below, 2 above range)

warm-container-minimum (n=40, range 1506.7-2394.7 ns)
   1506.7 |
   1551.1 |####
   1595.5 |#############
   1639.9 |####
   1684.3 |########
   1728.7 |
   1773.1 |########################################
   1817.5 |#############
   1861.9 |
   1906.3 |####
   1950.7 |####
   1995.1 |#################
   2039.5 |####
   2083.9 |
   2128.3 |####
   2172.7 |
   2217.1 |########
   2261.5 |
   2305.9 |
   2350.3 |####
  (6 below, 4 above range)

warm-container-native (n=40, range 1472.7-1971.3 ns)
   1472.7 |####
   1497.6 |################################
   1522.5 |
   1547.5 |####
   1572.4 |
   1597.3 |
   1622.3 |################
   1647.2 |############
   1672.1 |
   1697.0 |####
   1722.0 |
   1746.9 |####
   1771.8 |####
   1796.8 |########################################
   1821.7 |########
   1846.6 |####
   1871.5 |
   1896.5 |####
   1921.4 |
   1946.3 |
  (2 below, 4 above range)

warm-container-plusone (n=40, range 1553.9-6562.5 ns)
   1553.9 |########################################
   1804.3 |##########
   2054.7 |
   2305.2 |
   2555.6 |
   2806.0 |
   3056.5 |
   3306.9 |
   3557.3 |
   3807.8 |######
   4058.2 |###
   4308.6 |
   4559.1 |#############
   4809.5 |######
   5059.9 |
   5310.4 |###
   5560.8 |
   5811.2 |###
   6061.7 |###
   6312.1 |################
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=55.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=51.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=48.0% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=57.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.2x (possible bimodal distribution)
