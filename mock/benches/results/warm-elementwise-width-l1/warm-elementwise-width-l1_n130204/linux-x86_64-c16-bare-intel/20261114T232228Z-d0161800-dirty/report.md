# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 2.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-plusone beats baseline by 43% (significant)

warm-container-plusone is -1.78 us (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-native, warm-container-plusone) are a dead heat (<1%)

warm-container-native (2.11 us) and warm-container-plusone (2.11 us) differ by 0.14%, inside the noise, even though the wider field spreads 94.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.58)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-plusone, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom} (76% apart)

The field splits into a fast tier {warm-container-native, warm-container-plusone, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom} with a 76% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-native vs stability leader warm-container-plusone (+0% speed for 1.3x steadier)

warm-container-native is fastest (2.11 us, CV 13.7%); warm-container-plusone gives up 0.1% median for 1.3x lower variance (CV 11.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.1x its best-20%

warm-container-headroom's best 20% of batches run at 1.16 us but its worst 20% at 4.76 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 2111.3 ns median (-48.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.94x (fastest 2111.3 ns, slowest 4100.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3486ns | 4322ns | 1219ns | 3699ns | 5115ns | base |
| warm-container-kernel | 2492ns | 2470ns | 1895ns | 2478ns | 3132ns | -28.51% |
| warm-container-minimum | 2523ns | 2595ns | 1736ns | 2575ns | 3150ns | -27.64% |
| warm-container-native | 2403ns | 2338ns | 2047ns | 2348ns | 2926ns | -31.06% |
| warm-container-plusone | 2461ns | 2353ns | 2188ns | 2410ns | 2887ns | -29.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3288ns | 1156ns | 4760ns | base | 12.456 |
| warm-container-kernel | 2236ns | 1711ns | 2777ns | -32.01% | 18.319 |
| warm-container-minimum | 2250ns | 1559ns | 2770ns | -31.57% | 18.202 |
| warm-container-native | 2158ns | 1848ns | 2633ns | -34.39% | 18.984 |
| warm-container-plusone | 2211ns | 1977ns | 2556ns | -32.77% | 18.527 |

## Performance model

- Peak throughput: **35.435 Gops/s** (warm-container-headroom; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.989 | 28.2% |
| warm-container-kernel | 18.732 | 52.9% |
| warm-container-minimum | 17.599 | 49.7% |
| warm-container-native | 19.400 | 54.7% |
| warm-container-plusone | 19.373 | 54.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3486ns | 3486ns | base |
| warm-container-kernel | 2492ns | 2492ns | -28.51% |
| warm-container-minimum | 2523ns | 2523ns | -27.64% |
| warm-container-native | 2403ns | 2403ns | -31.06% |
| warm-container-plusone | 2461ns | 2461ns | -29.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4100ns | base | --- | [3182, 4216] | --- | --- | --- | --- |
| warm-container-kernel | 2187ns | -1654.4ns (-40.3%) | [-1865, -1048]ns | [2113, 2352] | YES | 0.0129 | 0.0064 | 0 |
| warm-container-minimum | 2327ns | -1721.2ns (-42.0%) | [-1793, -998]ns | [2134, 2424] | YES | 0.0089 | 0.0022 | 0 |
| warm-container-native | 2111ns | -1670.8ns (-40.7%) | [-1978, -1124]ns | [2111, 2122] | YES | 0.0166 | 0.0166 | 0 |
| warm-container-plusone | 2114ns | -1778.4ns (-43.4%) | [-1957, -998]ns | [2111, 2221] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4184ns | -40.4% | -42.3% | -49.5% | -43.2% |
| 1 | 2 | 3808ns | -46.4% | -44.6% | -44.6% | -44.2% |
| 1 | 3 | 4193ns | -49.6% | -42.9% | -42.3% | -49.1% |
| 1 | 4 | 3573ns | -33.0% | -33.5% | -46.3% | -40.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.575 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.270 | moderate- |
| warm-container-minimum | -0.498 | moderate- |
| warm-container-native | -0.537 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.087 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 69.6ns | 3288.5ns | 2.1% |  |
| warm-container-kernel | 92.7ns | 2235.9ns | 4.1% |  |
| warm-container-minimum | 98.4ns | 2250.3ns | 4.4% |  |
| warm-container-native | 89.5ns | 2157.7ns | 4.1% |  |
| warm-container-plusone | 97.0ns | 2210.8ns | 4.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1155.9-4760.1 ns)
   1155.9 |############################
   1336.1 |
   1516.3 |
   1696.6 |#####
   1876.8 |
   2057.0 |
   2237.2 |
   2417.4 |
   2597.6 |#####
   2777.8 |
   2958.0 |#####
   3138.2 |
   3318.4 |###########
   3498.7 |
   3678.9 |###########
   3859.1 |#####
   4039.3 |########################################
   4219.5 |############################
   4399.7 |###########
   4579.9 |######################
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1711.4-2777.0 ns)
   1711.4 |
   1764.7 |
   1818.0 |
   1871.2 |############################
   1924.5 |######################
   1977.8 |
   2031.1 |
   2084.4 |########################################
   2137.6 |###########
   2190.9 |#####
   2244.2 |
   2297.5 |############################
   2350.8 |#################
   2404.1 |###########
   2457.3 |
   2510.6 |
   2563.9 |#####
   2617.2 |#################
   2670.5 |
   2723.7 |#################
  (2 below, 2 above range)

warm-container-minimum (n=40, range 1558.7-2769.5 ns)
   1558.7 |
   1619.2 |
   1679.8 |
   1740.3 |
   1800.8 |
   1861.4 |##########
   1921.9 |#####
   1982.5 |
   2043.0 |#####
   2103.6 |########################################
   2164.1 |##########
   2224.6 |##########
   2285.2 |####################
   2345.7 |##########
   2406.3 |###############
   2466.8 |#####
   2527.4 |#####
   2587.9 |#########################
   2648.5 |#####
   2709.0 |#####
  (3 below, 3 above range)

warm-container-native (n=40, range 1848.4-2633.0 ns)
   1848.4 |
   1887.6 |###########
   1926.8 |##
   1966.1 |##
   2005.3 |#####
   2044.5 |
   2083.8 |########################################
   2123.0 |########
   2162.2 |########
   2201.4 |
   2240.7 |
   2279.9 |
   2319.1 |##
   2358.4 |#####
   2397.6 |
   2436.8 |##
   2476.0 |
   2515.3 |
   2554.5 |
   2593.7 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1977.0-2556.4 ns)
   1977.0 |#######
   2006.0 |
   2035.0 |
   2063.9 |
   2092.9 |########################################
   2121.9 |#####
   2150.8 |##
   2179.8 |#####
   2208.8 |##
   2237.8 |
   2266.7 |##
   2295.7 |
   2324.7 |##########
   2353.6 |##
   2382.6 |#####
   2411.6 |##
   2440.6 |
   2469.5 |
   2498.5 |
   2527.5 |##
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=43.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=21.6% (high variance, measurements may be unstable)
