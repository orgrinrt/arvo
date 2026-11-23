# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.95 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.64 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 60% (significant)

warm-container-native is -2.38 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.4x slower than the field

warm-container-headroom (3.95 us) is 2.4x the fastest (1.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-kernel (1.64 us) and warm-container-lanes-deferred (1.65 us) differ by 0.04%, inside the noise, even though the wider field spreads 140.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader warm-container-kernel vs stability leader warm-container-native (+7% speed for 1.0x steadier)

warm-container-kernel is fastest (1.64 us, CV 19.9%); warm-container-native gives up 6.5% median for 1.0x lower variance (CV 19.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.3x its best-20%

warm-container-minimum's best 20% of batches run at 1.46 us but its worst 20% at 6.22 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-32 ns, 0.81%)

warm-container-minimum differs from baseline warm-container-headroom by -32 ns (0.81%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1645.0 ns median (-58.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.40x (fastest 1645.0 ns, slowest 3952.6 ns)

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
| warm-container-headroom | 3669ns | 4091ns | 1521ns | 3522ns | 6256ns | base |
| warm-container-kernel | 2047ns | 1884ns | 1709ns | 1981ns | 2585ns | -44.20% |
| warm-container-lanes-deferred | 1990ns | 1862ns | 1582ns | 1916ns | 2620ns | -45.76% |
| warm-container-minimum | 3381ns | 1977ns | 1508ns | 2969ns | 6492ns | -7.83% |
| warm-container-native | 1996ns | 1973ns | 1502ns | 1958ns | 2604ns | -45.59% |
| warm-container-plusone | 3658ns | 2683ns | 1759ns | 3478ns | 6096ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3533ns | 1470ns | 5997ns | base | 9.275 |
| warm-container-kernel | 1798ns | 1519ns | 2247ns | -49.11% | 18.227 |
| warm-container-lanes-deferred | 1758ns | 1399ns | 2314ns | -50.23% | 18.634 |
| warm-container-minimum | 3254ns | 1456ns | 6224ns | -7.90% | 10.071 |
| warm-container-native | 1764ns | 1335ns | 2303ns | -50.08% | 18.580 |
| warm-container-plusone | 3523ns | 1699ns | 5848ns | -0.27% | 9.301 |

## Performance model

- Peak throughput: **24.543 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.290 | 33.8% |
| warm-container-kernel | 19.920 | 81.2% |
| warm-container-lanes-deferred | 19.912 | 81.1% |
| warm-container-minimum | 17.170 | 70.0% |
| warm-container-native | 18.703 | 76.2% |
| warm-container-plusone | 12.647 | 51.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3669ns | 3669ns | base |
| warm-container-kernel | 2047ns | 2047ns | -44.20% |
| warm-container-lanes-deferred | 1990ns | 1990ns | -45.76% |
| warm-container-minimum | 3381ns | 3381ns | -7.83% |
| warm-container-native | 1996ns | 1996ns | -45.59% |
| warm-container-plusone | 3658ns | 3658ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3953ns | base | --- | [1796, 4693] | --- | --- | --- | --- |
| warm-container-kernel | 1645ns | -2228.1ns (-56.4%) | [-2974, -227]ns | [1644, 1813] | YES | 0.0080 | 0.0064 | 0 |
| warm-container-lanes-deferred | 1646ns | -2332.8ns (-59.0%) | [-2876, -429]ns | [1593, 1797] | YES | 0.0080 | 0.0064 | 0 |
| warm-container-minimum | 1908ns | -32.2ns (-0.8%) | [-237, -1]ns | [1468, 4394] | YES | 0.0385 | 0.0385 | 0 |
| warm-container-native | 1752ns | -2382.4ns (-60.3%) | [-3030, -176]ns | [1643, 1808] | YES | 0.0056 | 0.0022 | 0 |
| warm-container-plusone | 2591ns | +312.9ns (+7.9%) | [+307, +631]ns | [2378, 4817] | YES | 0.0056 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2227ns | -9.3% | +1.2% | -34.3% | -1.2% | +73.5% |
| 1 | 2 | 4354ns | -63.3% | -60.4% | -1.5% | -68.8% | +6.3% |
| 1 | 3 | 4362ns | -58.7% | -62.7% | -11.1% | -62.3% | -7.3% |
| 1 | 4 | 1815ns | -9.4% | -22.8% | -19.5% | -0.4% | +35.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.246 | moderate- |
| warm-container-kernel | -0.482 | moderate- |
| warm-container-lanes-deferred | 0.084 | ok |
| warm-container-minimum | -0.253 | moderate- |
| warm-container-native | -0.377 | moderate- |
| warm-container-plusone | -0.008 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 47.3ns | 3532.8ns | 1.3% |  |
| warm-container-kernel | 88.1ns | 1797.8ns | 4.9% |  |
| warm-container-lanes-deferred | 82.0ns | 1758.5ns | 4.7% |  |
| warm-container-minimum | 43.3ns | 3253.7ns | 1.3% |  |
| warm-container-native | 81.8ns | 1763.6ns | 4.6% |  |
| warm-container-plusone | 48.4ns | 3523.1ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1469.6-5996.9 ns)
   1469.6 |########################################
   1696.0 |##############################
   1922.4 |
   2148.7 |
   2375.1 |
   2601.5 |
   2827.8 |#####
   3054.2 |
   3280.6 |#####
   3506.9 |#####
   3733.3 |
   3959.7 |
   4186.0 |####################
   4412.4 |
   4638.8 |####################
   4865.1 |###############
   5091.5 |##########
   5317.9 |##########
   5544.2 |
   5770.6 |#####
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1518.8-2247.1 ns)
   1518.8 |######
   1555.2 |
   1591.6 |
   1628.0 |########################################
   1664.4 |
   1700.8 |
   1737.2 |###
   1773.7 |#########
   1810.1 |######
   1846.5 |###
   1882.9 |###
   1919.3 |
   1955.7 |#########
   1992.2 |######
   2028.6 |###
   2065.0 |###
   2101.4 |######
   2137.8 |
   2174.2 |###
   2210.6 |
  (6 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1399.5-2314.1 ns)
   1399.5 |######
   1445.2 |
   1490.9 |########################################
   1536.7 |#############
   1582.4 |######
   1628.1 |########################################
   1673.9 |######
   1719.6 |#############
   1765.3 |####################
   1811.1 |#############
   1856.8 |######
   1902.5 |
   1948.3 |#############
   1994.0 |#############
   2039.7 |
   2085.5 |
   2131.2 |
   2176.9 |
   2222.7 |#############
   2268.4 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1456.3-6224.4 ns)
   1456.3 |########################################
   1694.7 |##############
   1933.1 |###
   2171.5 |
   2410.0 |###
   2648.4 |
   2886.8 |
   3125.2 |
   3363.6 |
   3602.0 |
   3840.4 |#######
   4078.8 |#######
   4317.2 |###
   4555.6 |#######
   4794.0 |#######
   5032.4 |###
   5270.8 |###
   5509.2 |#######
   5747.6 |###
   5986.0 |
  (5 below, 4 above range)

warm-container-native (n=40, range 1335.1-2303.1 ns)
   1335.1 |
   1383.5 |#################
   1431.9 |####
   1480.3 |####
   1528.7 |
   1577.1 |
   1625.5 |###############################
   1673.9 |########
   1722.3 |
   1770.7 |########################################
   1819.1 |
   1867.5 |####
   1915.9 |
   1964.3 |########
   2012.7 |####
   2061.1 |
   2109.5 |########
   2157.9 |
   2206.3 |
   2254.7 |####
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1699.4-5847.8 ns)
   1699.4 |########################################
   1906.8 |#####
   2114.2 |#####
   2321.6 |##############################
   2529.1 |####################
   2736.5 |
   2943.9 |#####
   3151.3 |
   3358.7 |
   3566.2 |
   3773.6 |
   3981.0 |
   4188.4 |#####
   4395.8 |#####
   4603.3 |#####
   4810.7 |#####
   5018.1 |###############
   5225.5 |###############
   5432.9 |##########
   5640.4 |##########
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=60.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.4x (possible bimodal distribution)
