# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.75)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-headroom, warm-container-lanes-deferred, warm-container-native} vs {warm-container-plusone} (68% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-headroom, warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-plusone} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+3% speed for 1.2x steadier)

warm-container-kernel is fastest (1.78 us, CV 15.3%); warm-container-lanes-deferred gives up 3.3% median for 1.2x lower variance (CV 13.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.5x its best-20%

warm-container-minimum's best 20% of batches run at 1.46 us but its worst 20% at 6.62 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-11 ns, 0.62%)

warm-container-plusone differs from baseline warm-container-headroom by -11 ns (0.62%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1779.5 ns median (-1.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.00x (fastest 1779.5 ns, slowest 3550.9 ns)

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
| warm-container-headroom | 3728ns | 1871ns | 1730ns | 3381ns | 6766ns | base |
| warm-container-kernel | 2042ns | 2031ns | 1702ns | 1988ns | 2544ns | -45.22% |
| warm-container-lanes-deferred | 2191ns | 2102ns | 1926ns | 2136ns | 2621ns | -41.22% |
| warm-container-minimum | 3583ns | 1865ns | 1514ns | 3161ns | 6918ns | -3.89% |
| warm-container-native | 2411ns | 2372ns | 1508ns | 2392ns | 3367ns | -35.34% |
| warm-container-plusone | 3571ns | 3677ns | 1723ns | 3400ns | 5931ns | -4.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3591ns | 1672ns | 6496ns | base | 11.406 |
| warm-container-kernel | 1804ns | 1512ns | 2251ns | -49.76% | 22.702 |
| warm-container-lanes-deferred | 1938ns | 1712ns | 2305ns | -46.02% | 21.131 |
| warm-container-minimum | 3449ns | 1462ns | 6623ns | -3.95% | 11.875 |
| warm-container-native | 2131ns | 1342ns | 2953ns | -40.66% | 19.220 |
| warm-container-plusone | 3440ns | 1664ns | 5679ns | -4.22% | 11.908 |

## Performance model

- Peak throughput: **30.528 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.657 | 74.2% |
| warm-container-kernel | 23.017 | 75.4% |
| warm-container-lanes-deferred | 22.281 | 73.0% |
| warm-container-minimum | 22.734 | 74.5% |
| warm-container-native | 19.382 | 63.5% |
| warm-container-plusone | 11.535 | 37.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3728ns | 3728ns | base |
| warm-container-kernel | 2042ns | 2042ns | -45.22% |
| warm-container-lanes-deferred | 2191ns | 2191ns | -41.22% |
| warm-container-minimum | 3583ns | 3583ns | -3.89% |
| warm-container-native | 2411ns | 2411ns | -35.34% |
| warm-container-plusone | 3571ns | 3571ns | -4.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1808ns | base | --- | [1796, 4998] | --- | --- | --- | --- |
| warm-container-kernel | 1780ns | -289.5ns (-16.0%) | [-3212, -120]ns | [1646, 1809] | YES | 0.0415 | 0.0166 | 0 |
| warm-container-lanes-deferred | 1838ns | no significant difference | [-3113, +12]ns | [1808, 2008] | no | 0.2682 | 0.2682 | 0 |
| warm-container-minimum | 1802ns | no significant difference | [-334, +24]ns | [1788, 4876] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 2113ns | no significant difference | [-2409, +97]ns | [2010, 2266] | no | 0.2682 | 0.2682 | 0 |
| warm-container-plusone | 3551ns | -11.1ns (-0.6%) | [-90, -5]ns | [1792, 4629] | YES | 0.0034 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1802ns | +0.3% | +0.3% | +173.9% | +43.9% | +109.3% |
| 1 | 2 | 1797ns | +20.0% | +18.2% | -18.7% | +11.9% | -0.6% |
| 1 | 3 | 4539ns | -64.8% | -60.2% | -60.4% | -50.1% | -1.7% |
| 1 | 4 | 3207ns | -48.8% | -38.8% | -3.0% | -56.7% | +43.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.012 | ok |
| warm-container-kernel | -0.196 | ok |
| warm-container-lanes-deferred | -0.748 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.237 | moderate- |
| warm-container-native | -0.222 | moderate- |
| warm-container-plusone | -0.188 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 47.8ns | 3591.2ns | 1.3% |  |
| warm-container-kernel | 83.4ns | 1804.3ns | 4.6% |  |
| warm-container-lanes-deferred | 90.0ns | 1938.4ns | 4.6% |  |
| warm-container-minimum | 45.7ns | 3449.3ns | 1.3% |  |
| warm-container-native | 98.2ns | 2131.1ns | 4.6% |  |
| warm-container-plusone | 47.2ns | 3439.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1671.5-6495.9 ns)
   1671.5 |########################################
   1912.7 |
   2154.0 |
   2395.2 |
   2636.4 |
   2877.6 |
   3118.8 |
   3360.1 |
   3601.3 |
   3842.5 |
   4083.7 |##
   4324.9 |##
   4566.2 |####
   4807.4 |####
   5048.6 |####
   5289.8 |####
   5531.0 |##
   5772.3 |##
   6013.5 |
   6254.7 |####
  (3 below, 5 above range)

warm-container-kernel (n=40, range 1511.8-2251.5 ns)
   1511.8 |
   1548.8 |###############
   1585.8 |
   1622.7 |########################################
   1659.7 |#####
   1696.7 |#####
   1733.7 |#####
   1770.7 |###############
   1807.7 |#########################
   1844.6 |#####
   1881.6 |#####
   1918.6 |
   1955.6 |#####
   1992.6 |###############
   2029.6 |#####
   2066.5 |
   2103.5 |
   2140.5 |
   2177.5 |
   2214.5 |
  (6 below, 5 above range)

warm-container-lanes-deferred (n=40, range 1711.8-2304.7 ns)
   1711.8 |
   1741.4 |######
   1771.1 |######
   1800.7 |########################################
   1830.4 |###
   1860.0 |
   1889.7 |###
   1919.3 |###
   1949.0 |###
   1978.6 |##########
   2008.2 |################
   2037.9 |###
   2067.5 |###
   2097.2 |
   2126.8 |
   2156.5 |###
   2186.1 |###
   2215.7 |
   2245.4 |######
   2275.0 |
  (4 below, 2 above range)

warm-container-minimum (n=40, range 1462.1-6623.1 ns)
   1462.1 |#####
   1720.1 |########################################
   1978.2 |
   2236.2 |
   2494.3 |
   2752.3 |
   3010.4 |##
   3268.4 |
   3526.5 |
   3784.5 |##
   4042.6 |
   4300.6 |##
   4558.7 |
   4816.7 |########
   5074.8 |
   5332.8 |########
   5590.9 |
   5848.9 |#####
   6107.0 |#####
   6365.0 |########
  (6 below, 2 above range)

warm-container-native (n=40, range 1341.7-2952.6 ns)
   1341.7 |#################################
   1422.3 |######
   1502.8 |
   1583.4 |
   1663.9 |
   1744.4 |######
   1825.0 |######
   1905.5 |######
   1986.1 |########################################
   2066.6 |#############
   2147.2 |######
   2227.7 |#################################
   2308.2 |######
   2388.8 |
   2469.3 |######
   2549.9 |#################################
   2630.4 |######
   2711.0 |######
   2791.5 |
   2872.0 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1664.1-5679.2 ns)
   1664.1 |########################################
   1864.8 |
   2065.6 |##
   2266.4 |
   2467.1 |
   2667.9 |##
   2868.6 |
   3069.4 |
   3270.1 |
   3470.9 |
   3671.7 |
   3872.4 |
   4073.2 |
   4273.9 |#####
   4474.7 |################
   4675.4 |##
   4876.2 |##########
   5077.0 |#####
   5277.7 |#####
   5478.5 |##
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.3% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=61.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-native**: CV=27.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=50.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.4x (possible bimodal distribution)
