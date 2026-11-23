# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 1.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred dominates: 30% faster than the next best (warm-container-native)

warm-container-lanes-deferred (1.68 us) leads warm-container-native (2.18 us) by 30%, a clear separation rather than a photo finish. CV 124.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 45% (significant)

warm-container-native is -1.84 us (45%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.4x slower than the field

warm-container-headroom (4.05 us) is 2.4x the fastest (1.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred is fastest but the noisiest (CV 124.3%)

warm-container-lanes-deferred wins on median (1.68 us) yet has the highest variance (CV 124.3%), while warm-container-minimum is the steadiest (CV 48.5%, 3.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.75)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-native} vs {warm-container-kernel, warm-container-minimum, warm-container-plusone, warm-container-headroom} (62% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-kernel, warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 62% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.5x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.34 us but its worst 20% at 6.01 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1680.5 ns median (-58.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.41x (fastest 1680.5 ns, slowest 4049.1 ns)

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
| warm-container-headroom | 4536ns | 4131ns | 3380ns | 3887ns | 7641ns | base |
| warm-container-kernel | 3261ns | 3719ns | 1329ns | 3055ns | 5809ns | -28.12% |
| warm-container-lanes-deferred | 3526ns | 1769ns | 1410ns | 3281ns | 6379ns | -22.27% |
| warm-container-minimum | 3479ns | 3838ns | 1377ns | 3309ns | 6092ns | -23.31% |
| warm-container-native | 3585ns | 2287ns | 1545ns | 3260ns | 6600ns | -20.97% |
| warm-container-plusone | 4745ns | 4129ns | 3383ns | 3931ns | 8548ns | +4.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4438ns | 3313ns | 7446ns | base | 7.384 |
| warm-container-kernel | 3091ns | 1263ns | 5487ns | -30.35% | 10.602 |
| warm-container-lanes-deferred | 3339ns | 1339ns | 6007ns | -24.75% | 9.812 |
| warm-container-minimum | 3296ns | 1310ns | 5741ns | -25.74% | 9.943 |
| warm-container-native | 3400ns | 1471ns | 6237ns | -23.37% | 9.636 |
| warm-container-plusone | 4637ns | 3316ns | 8318ns | +4.48% | 7.067 |

## Performance model

- Peak throughput: **25.935 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.093 | 31.2% |
| warm-container-kernel | 9.272 | 35.8% |
| warm-container-lanes-deferred | 19.500 | 75.2% |
| warm-container-minimum | 9.048 | 34.9% |
| warm-container-native | 15.044 | 58.0% |
| warm-container-plusone | 8.096 | 31.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4536ns | 4536ns | base |
| warm-container-kernel | 3261ns | 3261ns | -28.12% |
| warm-container-lanes-deferred | 3526ns | 3526ns | -22.27% |
| warm-container-minimum | 3479ns | 3479ns | -23.31% |
| warm-container-native | 3585ns | 3585ns | -20.97% |
| warm-container-plusone | 4745ns | 4745ns | +4.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4049ns | base | --- | [3326, 4052] | --- | --- | --- | --- |
| warm-container-kernel | 3534ns | no significant difference | [-2064, +215]ns | [1277, 4172] | no | 0.2564 | 0.1539 | 0 |
| warm-container-lanes-deferred | 1680ns | -1754.1ns (-43.3%) | [-2463, -1069]ns | [1584, 5478] | YES (adj: no) | 0.0829 | 0.0166 | 0 |
| warm-container-minimum | 3622ns | no significant difference | [-2426, +452]ns | [1624, 4382] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 2178ns | -1840.6ns (-45.5%) | [-2439, -62]ns | [1607, 4828] | YES (adj: no) | 0.2017 | 0.0807 | 0 |
| warm-container-plusone | 4047ns | no significant difference | [-550, +733]ns | [3511, 4055] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4056ns | -3.3% | -68.6% | -67.0% | -60.2% | -18.2% |
| 1 | 2 | 3319ns | +12.5% | -33.3% | +21.4% | +18.7% | +22.1% |
| 1 | 3 | 4052ns | -58.9% | -57.7% | -59.9% | +3.1% | -13.5% |
| 1 | 4 | 3316ns | -61.7% | +11.5% | +21.7% | +27.2% | +22.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.748 | HIGH- (thermal bounce) |
| warm-container-kernel | 0.295 | moderate+ |
| warm-container-lanes-deferred | -0.220 | moderate- |
| warm-container-minimum | -0.717 | HIGH- (thermal bounce) |
| warm-container-native | -0.007 | ok |
| warm-container-plusone | -0.646 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.8ns | 4437.5ns | 0.6% |  |
| warm-container-kernel | 50.0ns | 3090.9ns | 1.6% |  |
| warm-container-lanes-deferred | 53.9ns | 3339.4ns | 1.6% |  |
| warm-container-minimum | 53.6ns | 3295.5ns | 1.6% |  |
| warm-container-native | 54.6ns | 3400.4ns | 1.6% |  |
| warm-container-plusone | 30.1ns | 4636.5ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3312.6-7446.4 ns)
   3312.6 |#########################
   3519.3 |
   3726.0 |
   3932.7 |########################################
   4139.4 |
   4346.1 |
   4552.8 |
   4759.4 |
   4966.1 |
   5172.8 |
   5379.5 |
   5586.2 |##
   5792.9 |
   5999.6 |
   6206.3 |
   6413.0 |
   6619.7 |
   6826.3 |
   7033.0 |
   7239.7 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1263.5-5486.8 ns)
   1263.5 |########################################
   1474.6 |
   1685.8 |
   1897.0 |###
   2108.1 |
   2319.3 |
   2530.5 |
   2741.6 |###
   2952.8 |
   3164.0 |
   3375.1 |######
   3586.3 |##########
   3797.5 |###
   4008.6 |###
   4219.8 |##########
   4431.0 |######
   4642.1 |######
   4853.3 |
   5064.5 |######
   5275.6 |
  (5 below, 5 above range)

warm-container-lanes-deferred (n=40, range 1339.0-6006.9 ns)
   1339.0 |##############
   1572.4 |########################################
   1805.8 |###
   2039.2 |
   2272.6 |
   2506.0 |
   2739.4 |###
   2972.8 |
   3206.2 |
   3439.6 |
   3673.0 |
   3906.4 |
   4139.8 |
   4373.2 |
   4606.6 |
   4840.0 |###
   5073.4 |###
   5306.8 |###
   5540.2 |################################
   5773.6 |###
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1310.1-5741.4 ns)
   1310.1 |########################################
   1531.7 |###################################
   1753.3 |#####
   1974.8 |#####
   2196.4 |
   2418.0 |
   2639.5 |
   2861.1 |
   3082.7 |#####
   3304.2 |
   3525.8 |##########
   3747.3 |##########
   3968.9 |#####
   4190.5 |###############
   4412.0 |###############
   4633.6 |##########
   4855.2 |#####
   5076.7 |##########
   5298.3 |#####
   5519.9 |
  (1 below, 4 above range)

warm-container-native (n=40, range 1470.7-6236.8 ns)
   1470.7 |########################################
   1709.0 |##
   1947.3 |
   2185.6 |
   2423.9 |##
   2662.2 |
   2900.5 |##
   3138.8 |
   3377.1 |
   3615.4 |
   3853.7 |##
   4092.0 |
   4330.3 |#####
   4568.6 |
   4806.9 |########
   5045.3 |#####
   5283.6 |
   5521.9 |##
   5760.2 |##################
   5998.5 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 3316.2-8318.5 ns)
   3316.2 |##############################
   3566.3 |
   3816.5 |########################################
   4066.6 |
   4316.7 |
   4566.8 |##
   4816.9 |##
   5067.0 |
   5317.1 |##
   5567.2 |
   5817.4 |
   6067.5 |
   6317.6 |
   6567.7 |
   6817.8 |
   7067.9 |
   7318.0 |
   7568.1 |
   7818.3 |
   8068.4 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=46.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=55.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=62.6% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=53.3% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-native**: CV=59.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.3% (high variance, measurements may be unstable)
