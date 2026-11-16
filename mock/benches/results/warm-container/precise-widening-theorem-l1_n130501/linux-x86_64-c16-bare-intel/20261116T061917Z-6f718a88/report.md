# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (919 ns) is smaller than the fastest variant's own run-to-run std-dev (2.36 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.0x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.21 us but its worst 20% at 4.86 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-31 ns, 1.18%)

warm-container-plusone differs from baseline warm-container-headroom by -31 ns (1.18%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-plusone** at 2592.7 ns median (-1.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.35x (fastest 2592.7 ns, slowest 3512.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112735203 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4050ns | 2688ns | 2200ns | 3156ns | 8584ns | base |
| warm-container-kernel | 3400ns | 3475ns | 1262ns | 3554ns | 5077ns | -16.06% |
| warm-container-lanes-deferred | 3388ns | 3691ns | 1267ns | 3503ns | 5161ns | -16.36% |
| warm-container-minimum | 3806ns | 2825ns | 2296ns | 2788ns | 8372ns | -6.02% |
| warm-container-native | 3905ns | 2682ns | 2361ns | 2910ns | 8433ns | -3.59% |
| warm-container-plusone | 3739ns | 2656ns | 2173ns | 2746ns | 8283ns | -7.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3943ns | 2149ns | 8324ns | base | 4.155 |
| warm-container-kernel | 3224ns | 1200ns | 4796ns | -18.25% | 5.082 |
| warm-container-lanes-deferred | 3210ns | 1205ns | 4858ns | -18.61% | 5.105 |
| warm-container-minimum | 3709ns | 2244ns | 8124ns | -5.94% | 4.417 |
| warm-container-native | 3801ns | 2305ns | 8177ns | -3.61% | 4.310 |
| warm-container-plusone | 3637ns | 2121ns | 8022ns | -7.77% | 4.505 |

## Performance model

- Peak throughput: **13.649 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.241 | 45.7% |
| warm-container-kernel | 4.955 | 36.3% |
| warm-container-lanes-deferred | 4.665 | 34.2% |
| warm-container-minimum | 5.932 | 43.5% |
| warm-container-native | 6.255 | 45.8% |
| warm-container-plusone | 6.319 | 46.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4050ns | 4050ns | base |
| warm-container-kernel | 3400ns | 3400ns | -16.06% |
| warm-container-lanes-deferred | 3388ns | 3388ns | -16.36% |
| warm-container-minimum | 3806ns | 3806ns | -6.02% |
| warm-container-native | 3905ns | 3905ns | -3.59% |
| warm-container-plusone | 3739ns | 3739ns | -7.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2625ns | base | --- | [2593, 2634] | --- | --- | --- | --- |
| warm-container-kernel | 3306ns | no significant difference | [-1583, +550]ns | [3074, 3649] | no | 1.0000 | 1.0000 | 0 |
| warm-container-lanes-deferred | 3512ns | no significant difference | [-1394, +854]ns | [3013, 3976] | no | 0.7160 | 0.4296 | 0 |
| warm-container-minimum | 2762ns | no significant difference | [-379, +604]ns | [2744, 2769] | no | 0.7948 | 0.6358 | 0 |
| warm-container-native | 2619ns | no significant difference | [-19, +463]ns | [2611, 2635] | no | 0.6705 | 0.2682 | 0 |
| warm-container-plusone | 2593ns | -31.0ns (-1.2%) | [-42, -25]ns | [2364, 2604] | YES (adj: no) | 0.0829 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2160ns | +48.4% | +49.1% | +28.1% | +21.5% | +20.3% |
| 1 | 2 | 2154ns | +97.8% | +78.9% | +28.2% | +21.7% | -1.5% |
| 1 | 3 | 2627ns | +24.9% | +46.7% | -13.8% | -0.4% | -18.8% |
| 1 | 4 | 2626ns | +20.9% | -24.8% | -14.3% | +17.2% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.246 | moderate+ |
| warm-container-kernel | -0.379 | moderate- |
| warm-container-lanes-deferred | -0.168 | ok |
| warm-container-minimum | 0.260 | moderate+ |
| warm-container-native | -0.092 | ok |
| warm-container-plusone | -0.250 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 36.3ns | 3943.4ns | 0.9% |  |
| warm-container-kernel | 64.4ns | 3223.8ns | 2.0% |  |
| warm-container-lanes-deferred | 63.8ns | 3209.5ns | 2.0% |  |
| warm-container-minimum | 32.6ns | 3709.2ns | 0.9% |  |
| warm-container-native | 35.3ns | 3801.2ns | 0.9% |  |
| warm-container-plusone | 33.9ns | 3637.1ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2148.5-8324.3 ns)
   2148.5 |#####################
   2457.3 |########################################
   2766.1 |
   3074.9 |
   3383.7 |
   3692.5 |
   4001.3 |
   4310.1 |
   4618.9 |
   4927.6 |
   5236.4 |#####
   5545.2 |
   5854.0 |
   6162.8 |##
   6471.6 |##
   6780.4 |##
   7089.2 |##
   7398.0 |
   7706.8 |##
   8015.5 |
  (5 below, 5 above range)

warm-container-kernel (n=40, range 1200.4-4796.5 ns)
   1200.4 |
   1380.2 |
   1560.0 |
   1739.8 |
   1919.6 |
   2099.4 |
   2279.2 |
   2459.0 |#############
   2638.8 |
   2818.6 |####################
   2998.4 |########################################
   3178.2 |#############
   3358.0 |#################################
   3537.9 |####################
   3717.7 |
   3897.5 |####################
   4077.3 |######
   4257.1 |######
   4436.9 |####################
   4616.7 |
  (7 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1205.3-4858.2 ns)
   1205.3 |########################################
   1388.0 |
   1570.6 |
   1753.2 |
   1935.9 |####
   2118.5 |
   2301.2 |
   2483.8 |
   2666.5 |####
   2849.1 |#############
   3031.8 |########
   3214.4 |####
   3397.0 |########
   3579.7 |######################
   3762.3 |
   3945.0 |######################
   4127.6 |
   4310.3 |#################
   4492.9 |####
   4675.5 |####
  (1 below, 4 above range)

warm-container-minimum (n=40, range 2244.0-8124.2 ns)
   2244.0 |#######################
   2538.0 |########################################
   2832.0 |
   3126.0 |##
   3420.1 |
   3714.1 |##
   4008.1 |
   4302.1 |
   4596.1 |
   4890.1 |
   5184.1 |
   5478.1 |
   5772.1 |
   6066.1 |##
   6360.2 |
   6654.2 |
   6948.2 |##
   7242.2 |##
   7536.2 |####
   7830.2 |
  (3 below, 3 above range)

warm-container-native (n=40, range 2305.3-8177.3 ns)
   2305.3 |###
   2598.9 |########################################
   2892.5 |
   3186.1 |
   3479.7 |
   3773.3 |#
   4066.9 |#
   4360.5 |
   4654.1 |#
   4947.7 |
   5241.3 |
   5534.9 |
   5828.5 |
   6122.1 |
   6415.7 |#
   6709.3 |#
   7002.9 |
   7296.5 |#
   7590.1 |#
   7883.7 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 2121.1-8022.5 ns)
   2121.1 |##########################
   2416.1 |########################################
   2711.2 |##
   3006.3 |
   3301.3 |
   3596.4 |
   3891.5 |
   4186.6 |
   4481.6 |##
   4776.7 |
   5071.8 |##
   5366.8 |
   5661.9 |##
   5957.0 |
   6252.1 |
   6547.1 |##
   6842.2 |##
   7137.3 |##
   7432.3 |
   7727.4 |
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=38.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=41.8% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=62.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=60.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=64.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
