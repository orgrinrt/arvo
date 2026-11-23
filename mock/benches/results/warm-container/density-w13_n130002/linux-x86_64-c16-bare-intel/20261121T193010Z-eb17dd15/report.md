# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 23% (significant)

warm-container-native is -406 ns (23%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.5x slower than the field

warm-container-minimum (4.47 us) is 2.5x the fastest (1.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone is fastest but the noisiest (CV 126.6%)

warm-container-plusone wins on median (1.78 us) yet has the highest variance (CV 126.6%), while warm-container-native is the steadiest (CV 22.3%, 1.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-plusone, warm-container-native) are a dead heat (<1%)

warm-container-plusone (1.78 us) and warm-container-native (1.80 us) differ by 0.68%, inside the noise, even though the wider field spreads 150.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader warm-container-plusone vs stability leader warm-container-native (+1% speed for 5.7x steadier)

warm-container-plusone is fastest (1.78 us, CV 126.6%); warm-container-native gives up 0.7% median for 5.7x lower variance (CV 22.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.8x its best-20%

warm-container-plusone's best 20% of batches run at 1.46 us but its worst 20% at 6.98 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 1784.2 ns median (-0.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.51x (fastest 1784.2 ns, slowest 4473.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 13 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112823892 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3459ns | 1862ns | 1519ns | 3005ns | 6760ns | base |
| warm-container-kernel | 3601ns | 2643ns | 1414ns | 3354ns | 6532ns | +4.13% |
| warm-container-lanes-deferred | 3517ns | 3831ns | 1378ns | 3362ns | 6121ns | +1.69% |
| warm-container-minimum | 3684ns | 4633ns | 1506ns | 3570ns | 6203ns | +6.51% |
| warm-container-native | 2078ns | 2021ns | 1569ns | 1985ns | 2863ns | -39.93% |
| warm-container-plusone | 3300ns | 1846ns | 1508ns | 2564ns | 7298ns | -4.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3328ns | 1467ns | 6466ns | base | 7.384 |
| warm-container-kernel | 3456ns | 1361ns | 6231ns | +3.84% | 7.111 |
| warm-container-lanes-deferred | 3377ns | 1326ns | 5856ns | +1.47% | 7.277 |
| warm-container-minimum | 3545ns | 1455ns | 5935ns | +6.53% | 6.932 |
| warm-container-native | 1832ns | 1394ns | 2522ns | -44.95% | 13.413 |
| warm-container-plusone | 3173ns | 1457ns | 6977ns | -4.66% | 7.745 |

## Performance model

- Peak throughput: **18.530 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 13.660 | 73.7% |
| warm-container-kernel | 9.657 | 52.1% |
| warm-container-lanes-deferred | 6.662 | 36.0% |
| warm-container-minimum | 5.494 | 29.6% |
| warm-container-native | 13.682 | 73.8% |
| warm-container-plusone | 13.775 | 74.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3459ns | 3459ns | base |
| warm-container-kernel | 3601ns | 3601ns | +4.13% |
| warm-container-lanes-deferred | 3517ns | 3517ns | +1.69% |
| warm-container-minimum | 3684ns | 3684ns | +6.51% |
| warm-container-native | 2078ns | 2078ns | -39.93% |
| warm-container-plusone | 3300ns | 3300ns | -4.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1799ns | base | --- | [1792, 4771] | --- | --- | --- | --- |
| warm-container-kernel | 2545ns | no significant difference | [-170, +82]ns | [1622, 5023] | no | 0.4470 | 0.2682 | 0 |
| warm-container-lanes-deferred | 3689ns | no significant difference | [-467, +150]ns | [1621, 4274] | no | 0.7948 | 0.6358 | 0 |
| warm-container-minimum | 4473ns | -158.4ns (-8.8%) | [-347, -7]ns | [1463, 5004] | YES | 0.0034 | 0.0007 | 0 |
| warm-container-native | 1796ns | -406.2ns (-22.6%) | [-3010, -48]ns | [1634, 1806] | YES (adj: no) | 0.2017 | 0.0807 | 0 |
| warm-container-plusone | 1784ns | no significant difference | [-336, +314]ns | [1778, 1978] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2036ns | +90.4% | -10.7% | +137.3% | -11.7% | -26.7% |
| 1 | 2 | 3383ns | +44.6% | +15.8% | +32.2% | -59.0% | +0.8% |
| 1 | 3 | 1467ns | -1.0% | +194.7% | +106.2% | +66.0% | +21.4% |
| 1 | 4 | 1470ns | +10.6% | +10.3% | -0.5% | +22.0% | +21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.200 | ok |
| warm-container-kernel | 0.101 | ok |
| warm-container-lanes-deferred | -0.259 | moderate- |
| warm-container-minimum | 0.258 | moderate+ |
| warm-container-native | -0.498 | moderate- |
| warm-container-plusone | -0.494 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 44.8ns | 3328.2ns | 1.3% |  |
| warm-container-kernel | 50.4ns | 3455.9ns | 1.5% |  |
| warm-container-lanes-deferred | 49.0ns | 3377.1ns | 1.5% |  |
| warm-container-minimum | 47.5ns | 3545.4ns | 1.3% |  |
| warm-container-native | 85.7ns | 1832.2ns | 4.7% |  |
| warm-container-plusone | 42.6ns | 3173.1ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1467.1-6466.0 ns)
   1467.1 |############################
   1717.0 |########################################
   1967.0 |
   2216.9 |####
   2466.9 |
   2716.8 |
   2966.8 |
   3216.7 |
   3466.6 |
   3716.6 |
   3966.5 |####
   4216.5 |
   4466.4 |####
   4716.4 |####
   4966.3 |
   5216.2 |####
   5466.2 |########
   5716.1 |############
   5966.1 |####
   6216.0 |####
  (6 below, 5 above range)

warm-container-kernel (n=40, range 1361.0-6230.9 ns)
   1361.0 |#######
   1604.5 |########################################
   1848.0 |
   2091.5 |
   2335.0 |###
   2578.5 |###
   2822.0 |
   3065.5 |
   3309.0 |
   3552.4 |###
   3795.9 |
   4039.4 |
   4282.9 |#######
   4526.4 |
   4769.9 |#######
   5013.4 |###
   5256.9 |##############
   5500.4 |###
   5743.9 |##############
   5987.4 |###
  (6 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1326.3-5856.4 ns)
   1326.3 |########################################
   1552.8 |########################################
   1779.3 |
   2005.8 |
   2232.3 |#####
   2458.8 |
   2685.3 |
   2911.9 |
   3138.4 |
   3364.9 |###########
   3591.4 |###########
   3817.9 |###########
   4044.4 |#################
   4270.9 |###########
   4497.4 |###########
   4723.9 |#####
   4950.4 |#####
   5176.9 |#################
   5403.4 |
   5629.9 |###########
  (2 below, 3 above range)

warm-container-minimum (n=40, range 1454.8-5935.0 ns)
   1454.8 |########################################
   1678.8 |
   1902.8 |
   2126.8 |
   2350.8 |
   2574.8 |#####
   2798.8 |
   3022.8 |
   3246.9 |
   3470.9 |
   3694.9 |
   3918.9 |
   4142.9 |
   4366.9 |########
   4590.9 |#####
   4814.9 |#####
   5038.9 |#####
   5263.0 |##############
   5487.0 |##
   5711.0 |########
  (3 below, 3 above range)

warm-container-native (n=40, range 1394.2-2522.0 ns)
   1394.2 |####
   1450.5 |########
   1506.9 |
   1563.3 |
   1619.7 |############################
   1676.1 |####
   1732.5 |
   1788.9 |########################################
   1845.3 |####
   1901.7 |####
   1958.1 |####
   2014.5 |####
   2070.9 |
   2127.3 |
   2183.7 |
   2240.1 |########
   2296.5 |####
   2352.9 |
   2409.2 |
   2465.6 |
  (7 below, 5 above range)

warm-container-plusone (n=40, range 1456.8-6976.8 ns)
   1456.8 |###############
   1732.8 |########################################
   2008.8 |##
   2284.8 |
   2560.8 |
   2836.8 |
   3112.8 |
   3388.8 |
   3664.8 |
   3940.8 |
   4216.8 |
   4492.8 |##
   4768.8 |##
   5044.8 |##
   5320.8 |##
   5596.8 |#####
   5872.8 |##
   6148.8 |##
   6424.8 |
   6700.8 |#####
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=64.0% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-kernel**: CV=58.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=52.5% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=54.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-native**: CV=21.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=71.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.8x (possible bimodal distribution)
