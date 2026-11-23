# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 1.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 44% (significant)

warm-container-minimum is -1.41 us (44%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is fastest but the noisiest (CV 167.7%)

warm-container-plusone wins on median (1.92 us) yet has the highest variance (CV 167.7%), while warm-container-minimum is the steadiest (CV 10.7%, 1.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-plusone, warm-container-minimum) are a dead heat (<1%)

warm-container-plusone (1.92 us) and warm-container-minimum (1.94 us) differ by 0.66%, inside the noise, even though the wider field spreads 67.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.72)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.30 us) is smaller than the fastest variant's own run-to-run std-dev (3.23 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-plusone, warm-container-minimum, warm-container-kernel, warm-container-native} vs {warm-container-headroom} (52% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-minimum, warm-container-kernel, warm-container-native} and a slow tier {warm-container-headroom} with a 52% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-plusone vs stability leader warm-container-minimum (+1% speed for 15.6x steadier)

warm-container-plusone is fastest (1.92 us, CV 167.7%); warm-container-minimum gives up 0.7% median for 15.6x lower variance (CV 10.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.0x its best-20%

warm-container-headroom's best 20% of batches run at 1.16 us but its worst 20% at 4.58 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 1923.0 ns median (-40.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.68x (fastest 1923.0 ns, slowest 3222.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112697097 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3107ns | 3399ns | 1219ns | 3146ns | 4875ns | base |
| warm-container-kernel | 2164ns | 2188ns | 1730ns | 2129ns | 2702ns | -30.34% |
| warm-container-minimum | 2147ns | 2140ns | 1802ns | 2164ns | 2440ns | -30.89% |
| warm-container-native | 2474ns | 2359ns | 1887ns | 2432ns | 3189ns | -20.35% |
| warm-container-plusone | 2724ns | 2129ns | 1842ns | 2187ns | 5216ns | -12.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 2934ns | 1157ns | 4579ns | base | 13.962 |
| warm-container-kernel | 1945ns | 1564ns | 2410ns | -33.71% | 21.062 |
| warm-container-minimum | 1931ns | 1630ns | 2176ns | -34.17% | 21.209 |
| warm-container-native | 2215ns | 1706ns | 2829ns | -24.50% | 18.493 |
| warm-container-plusone | 2504ns | 1670ns | 4935ns | -14.64% | 16.357 |

## Performance model

- Peak throughput: **35.403 Gops/s** (warm-container-headroom; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.712 | 35.9% |
| warm-container-kernel | 20.739 | 58.6% |
| warm-container-minimum | 21.160 | 59.8% |
| warm-container-native | 19.353 | 54.7% |
| warm-container-plusone | 21.300 | 60.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3107ns | 3107ns | base |
| warm-container-kernel | 2164ns | 2164ns | -30.34% |
| warm-container-minimum | 2147ns | 2147ns | -30.89% |
| warm-container-native | 2474ns | 2474ns | -20.35% |
| warm-container-plusone | 2724ns | 2724ns | -12.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3222ns | base | --- | [2168, 3903] | --- | --- | --- | --- |
| warm-container-kernel | 1975ns | -1050.9ns (-32.6%) | [-1514, -436]ns | [1712, 2116] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| warm-container-minimum | 1936ns | -1407.1ns (-43.7%) | [-1905, -298]ns | [1787, 2112] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| warm-container-native | 2116ns | no significant difference | [-1407, +223]ns | [2080, 2351] | no | 0.1539 | 0.1539 | 0 |
| warm-container-plusone | 1923ns | -1404.0ns (-43.6%) | [-1861, -2]ns | [1912, 2114] | YES (adj: no) | 0.1076 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 1273ns | +68.0% | +66.3% | +94.8% | +66.4% |
| 1 | 2 | 4154ns | -60.1% | -49.1% | -57.6% | -49.1% |
| 1 | 3 | 3502ns | -39.5% | -47.5% | -32.1% | -45.4% |
| 1 | 4 | 3119ns | -47.7% | -47.5% | -32.2% | -43.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.300 | moderate- |
| warm-container-kernel | -0.724 | HIGH- (thermal bounce) |
| warm-container-minimum | 0.275 | moderate+ |
| warm-container-native | -0.715 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.275 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 60.9ns | 2933.7ns | 2.1% |  |
| warm-container-kernel | 79.5ns | 1944.7ns | 4.1% |  |
| warm-container-minimum | 79.9ns | 1931.2ns | 4.1% |  |
| warm-container-native | 93.2ns | 2214.8ns | 4.2% |  |
| warm-container-plusone | 81.1ns | 2504.2ns | 3.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1157.0-4579.0 ns)
   1157.0 |########################################
   1328.1 |#####
   1499.2 |
   1670.3 |
   1841.4 |#####
   2012.5 |
   2183.6 |#####
   2354.7 |
   2525.8 |
   2696.9 |
   2868.0 |#####
   3039.1 |####################
   3210.2 |###############
   3381.3 |##########
   3552.4 |#####
   3723.5 |
   3894.6 |
   4065.7 |###################################
   4236.8 |##########
   4407.9 |#####
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1563.7-2409.8 ns)
   1563.7 |
   1606.0 |#####################
   1648.3 |#######
   1690.6 |#######
   1732.9 |##############
   1775.2 |
   1817.5 |###
   1859.9 |
   1902.2 |
   1944.5 |
   1986.8 |
   2029.1 |
   2071.4 |
   2113.7 |########################################
   2156.0 |###
   2198.3 |
   2240.6 |
   2282.9 |
   2325.2 |##############
   2367.5 |###
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1629.5-2176.0 ns)
   1629.5 |###
   1656.8 |#######
   1684.2 |
   1711.5 |
   1738.8 |#####################
   1766.1 |
   1793.5 |###
   1820.8 |
   1848.1 |###
   1875.4 |
   1902.8 |##########
   1930.1 |#######
   1957.4 |###
   1984.7 |
   2012.1 |
   2039.4 |##########
   2066.7 |
   2094.0 |########################################
   2121.4 |
   2148.7 |###
  (5 below, 3 above range)

warm-container-native (n=40, range 1706.0-2829.0 ns)
   1706.0 |#################
   1762.2 |#####
   1818.3 |
   1874.5 |#################
   1930.6 |#####
   1986.8 |#####
   2042.9 |###########
   2099.1 |########################################
   2155.2 |###########
   2211.4 |
   2267.5 |
   2323.6 |######################
   2379.8 |#################
   2435.9 |
   2492.1 |
   2548.2 |
   2604.4 |##################################
   2660.5 |
   2716.7 |
   2772.8 |#####
  (4 below, 2 above range)

warm-container-plusone (n=40, range 1670.0-4934.6 ns)
   1670.0 |##########################
   1833.3 |##############################
   1996.5 |########################################
   2159.7 |######
   2323.0 |##########
   2486.2 |
   2649.4 |
   2812.6 |
   2975.9 |
   3139.1 |
   3302.3 |###
   3465.6 |
   3628.8 |
   3792.0 |
   3955.2 |
   4118.5 |
   4281.7 |
   4444.9 |
   4608.2 |
   4771.4 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=46.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-plusone**: CV=128.8% (high variance, measurements may be unstable)
