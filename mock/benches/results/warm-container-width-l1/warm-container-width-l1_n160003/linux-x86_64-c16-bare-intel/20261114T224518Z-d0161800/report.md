# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 68% (significant)

warm-container-minimum is -3.42 us (68%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.0x slower than the field

warm-container-headroom (5.06 us) is 3.0x the fastest (1.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.76)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+6% speed for 3.5x steadier)

warm-container-minimum is fastest (1.70 us, CV 16.5%); warm-container-native gives up 6.0% median for 3.5x lower variance (CV 4.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 3.7x its best-20%

warm-container-plusone's best 20% of batches run at 1.67 us but its worst 20% at 6.11 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-24 ns, 0.48%)

warm-container-plusone differs from baseline warm-container-headroom by -24 ns (0.48%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 1704.7 ns median (-66.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.97x (fastest 1704.7 ns, slowest 5062.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4429ns | 5243ns | 1857ns | 4562ns | 6606ns | base |
| warm-container-kernel | 3331ns | 3357ns | 2246ns | 3401ns | 4208ns | -24.79% |
| warm-container-lanes-deferred | 3319ns | 3257ns | 2729ns | 3306ns | 3949ns | -25.07% |
| warm-container-minimum | 1905ns | 1928ns | 1528ns | 1870ns | 2389ns | -56.99% |
| warm-container-native | 2014ns | 2033ns | 1847ns | 2020ns | 2163ns | -54.53% |
| warm-container-plusone | 4080ns | 4476ns | 1730ns | 4101ns | 6368ns | -7.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4266ns | 1793ns | 6321ns | base | 7.681 |
| warm-container-kernel | 3123ns | 2113ns | 3921ns | -26.80% | 10.493 |
| warm-container-lanes-deferred | 3109ns | 2566ns | 3691ns | -27.14% | 10.541 |
| warm-container-minimum | 1684ns | 1358ns | 2094ns | -60.53% | 19.460 |
| warm-container-native | 1780ns | 1642ns | 1885ns | -58.28% | 18.410 |
| warm-container-plusone | 3930ns | 1671ns | 6109ns | -7.89% | 8.339 |

## Performance model

- Peak throughput: **24.137 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.472 | 26.8% |
| warm-container-kernel | 10.411 | 43.1% |
| warm-container-lanes-deferred | 10.691 | 44.3% |
| warm-container-minimum | 19.222 | 79.6% |
| warm-container-native | 18.135 | 75.1% |
| warm-container-plusone | 7.584 | 31.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4429ns | 4429ns | base |
| warm-container-kernel | 3331ns | 3331ns | -24.79% |
| warm-container-lanes-deferred | 3319ns | 3319ns | -25.07% |
| warm-container-minimum | 1905ns | 1905ns | -56.99% |
| warm-container-native | 2014ns | 2014ns | -54.53% |
| warm-container-plusone | 4080ns | 4080ns | -7.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5063ns | base | --- | [2640, 5470] | --- | --- | --- | --- |
| warm-container-kernel | 3147ns | -1775.3ns (-35.1%) | [-1972, -783]ns | [2972, 3401] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-lanes-deferred | 3065ns | no significant difference | [-2246, +205]ns | [2958, 3303] | no | 0.1539 | 0.1539 | 0 |
| warm-container-minimum | 1705ns | -3423.5ns (-67.6%) | [-3877, -1266]ns | [1484, 1808] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1807ns | -3280.2ns (-64.8%) | [-3655, -974]ns | [1804, 1808] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4321ns | -24.1ns (-0.5%) | [-393, -2]ns | [2504, 5279] | YES | 0.0296 | 0.0237 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 5183ns | -33.1% | -43.8% | -61.4% | -65.1% | -17.9% |
| 1 | 2 | 4897ns | -36.4% | -27.8% | -71.7% | -63.4% | +3.1% |
| 1 | 3 | 5248ns | -46.8% | -46.8% | -72.3% | -66.0% | -4.9% |
| 1 | 4 | 5235ns | -30.3% | -38.7% | -65.5% | -65.5% | -23.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.322 | moderate- |
| warm-container-kernel | -0.337 | moderate- |
| warm-container-lanes-deferred | -0.758 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.261 | moderate- |
| warm-container-native | -0.282 | moderate- |
| warm-container-plusone | -0.230 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 57.8ns | 4266.3ns | 1.4% |  |
| warm-container-kernel | 75.0ns | 3122.8ns | 2.4% |  |
| warm-container-lanes-deferred | 75.4ns | 3108.6ns | 2.4% |  |
| warm-container-minimum | 80.5ns | 1683.9ns | 4.8% |  |
| warm-container-native | 85.0ns | 1779.9ns | 4.8% |  |
| warm-container-plusone | 54.7ns | 3929.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1793.3-6320.5 ns)
   1793.3 |########################################
   2019.7 |#####
   2246.0 |###########
   2472.4 |
   2698.8 |
   2925.1 |#####
   3151.5 |
   3377.8 |
   3604.2 |
   3830.6 |
   4056.9 |
   4283.3 |
   4509.6 |#####
   4736.0 |#####
   4962.4 |######################
   5188.7 |###########
   5415.1 |##################################
   5641.4 |#####
   5867.8 |#################
   6094.2 |###########
  (4 below, 5 above range)

warm-container-kernel (n=40, range 2113.4-3921.4 ns)
   2113.4 |
   2203.8 |
   2294.2 |
   2384.6 |######
   2475.0 |
   2565.4 |####################
   2655.8 |
   2746.2 |#################################
   2836.6 |#############
   2927.0 |
   3017.4 |########################################
   3107.8 |
   3198.2 |#############
   3288.6 |####################
   3379.0 |#############
   3469.4 |######
   3559.8 |####################
   3650.2 |#################################
   3740.6 |
   3831.0 |
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2566.2-3691.5 ns)
   2566.2 |####################
   2622.4 |######
   2678.7 |#############
   2735.0 |######
   2791.2 |####################
   2847.5 |#############
   2903.8 |######
   2960.0 |######
   3016.3 |########################################
   3072.5 |#############
   3128.8 |
   3185.1 |#############
   3241.3 |
   3297.6 |#################################
   3353.9 |######
   3410.1 |######
   3466.4 |
   3522.7 |
   3578.9 |#################################
   3635.2 |#############
  (1 below, 1 above range)

warm-container-minimum (n=40, range 1357.6-2093.7 ns)
   1357.6 |###############################
   1394.4 |#############
   1431.2 |
   1468.0 |#############
   1504.8 |########
   1541.6 |
   1578.4 |########
   1615.2 |
   1652.0 |
   1688.8 |
   1725.6 |
   1762.4 |
   1799.3 |########################################
   1836.1 |####
   1872.9 |
   1909.7 |####
   1946.5 |
   1983.3 |#################
   2020.1 |########
   2056.9 |
  (3 below, 3 above range)

warm-container-native (n=40, range 1642.0-1885.5 ns)
   1642.0 |##########
   1654.2 |
   1666.4 |
   1678.5 |
   1690.7 |####
   1702.9 |##
   1715.1 |
   1727.2 |
   1739.4 |
   1751.6 |##
   1763.7 |
   1775.9 |##
   1788.1 |
   1800.3 |########################################
   1812.4 |
   1824.6 |
   1836.8 |####
   1849.0 |######
   1861.1 |
   1873.3 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1671.0-6108.5 ns)
   1671.0 |########################################
   1892.9 |
   2114.8 |
   2336.6 |
   2558.5 |
   2780.4 |
   3002.3 |###
   3224.1 |###
   3446.0 |
   3667.9 |
   3889.8 |#######
   4111.6 |#######
   4333.5 |###
   4555.4 |#######
   4777.3 |###
   4999.1 |#######
   5221.0 |##################
   5442.9 |#######
   5664.8 |###
   5886.6 |#######
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=43.3% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-kernel**: CV=22.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=45.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
