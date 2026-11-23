# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-plusone dominates: 12% faster than the next best (warm-container-native)

warm-container-plusone (3.37 us) leads warm-container-native (3.77 us) by 12%, a clear separation rather than a photo finish. CV 54.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-plusone is fastest but the noisiest (CV 54.7%)

warm-container-plusone wins on median (3.37 us) yet has the highest variance (CV 54.7%), while warm-container-kernel is the steadiest (CV 31.4%, 3.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.58)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (705 ns) is smaller than the fastest variant's own run-to-run std-dev (1.84 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-plusone is inconsistent: worst-20% is 4.5x its best-20%

warm-container-plusone's best 20% of batches run at 1.29 us but its worst 20% at 5.81 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-native's edge over baseline is significant but tiny (-11 ns, 0.26%)

warm-container-native differs from baseline warm-container-headroom by -11 ns (0.26%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-plusone** at 3365.3 ns median (-16.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.21x (fastest 3365.3 ns, slowest 4070.5 ns)

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
| Counter frequency | 2112763739 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3648ns | 4244ns | 1552ns | 3538ns | 6073ns | base |
| warm-container-kernel | 3762ns | 4024ns | 1586ns | 3987ns | 5265ns | +3.13% |
| warm-container-lanes-deferred | 3818ns | 4278ns | 1573ns | 3822ns | 6052ns | +4.67% |
| warm-container-minimum | 3382ns | 4116ns | 1346ns | 3352ns | 5508ns | -7.29% |
| warm-container-native | 3721ns | 3969ns | 1607ns | 3578ns | 6264ns | +2.00% |
| warm-container-plusone | 3541ns | 3537ns | 1357ns | 3401ns | 6147ns | -2.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3458ns | 1475ns | 5733ns | base | 4.738 |
| warm-container-kernel | 3570ns | 1507ns | 4987ns | +3.25% | 4.589 |
| warm-container-lanes-deferred | 3623ns | 1495ns | 5719ns | +4.76% | 4.523 |
| warm-container-minimum | 3206ns | 1280ns | 5183ns | -7.30% | 5.111 |
| warm-container-native | 3527ns | 1528ns | 5913ns | +2.00% | 4.645 |
| warm-container-plusone | 3358ns | 1289ns | 5805ns | -2.90% | 4.880 |

## Performance model

- Peak throughput: **12.800 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.057 | 31.7% |
| warm-container-kernel | 4.282 | 33.5% |
| warm-container-lanes-deferred | 4.025 | 31.4% |
| warm-container-minimum | 4.186 | 32.7% |
| warm-container-native | 4.341 | 33.9% |
| warm-container-plusone | 4.869 | 38.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3648ns | 3648ns | base |
| warm-container-kernel | 3762ns | 3762ns | +3.13% |
| warm-container-lanes-deferred | 3818ns | 3818ns | +4.67% |
| warm-container-minimum | 3382ns | 3382ns | -7.29% |
| warm-container-native | 3721ns | 3721ns | +2.00% |
| warm-container-plusone | 3541ns | 3541ns | -2.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4038ns | base | --- | [1576, 4748] | --- | --- | --- | --- |
| warm-container-kernel | 3826ns | no significant difference | [-564, +11]ns | [3586, 4054] | no | 0.3352 | 0.2682 | 0 |
| warm-container-lanes-deferred | 4070ns | no significant difference | [-38, +54]ns | [2691, 4427] | no | 0.3352 | 0.2682 | 0 |
| warm-container-minimum | 3914ns | -293.3ns (-7.3%) | [-304, -267]ns | [1566, 4569] | YES | 0.0034 | 0.0007 | 0 |
| warm-container-native | 3775ns | -10.7ns (-0.3%) | [-41, -5]ns | [1924, 4373] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-container-plusone | 3365ns | no significant difference | [-287, +255]ns | [1571, 4654] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4247ns | -14.4% | -10.1% | +2.5% | -14.1% | -49.2% |
| 1 | 2 | 2784ns | +51.3% | +59.0% | -13.7% | -43.6% | +71.3% |
| 1 | 3 | 3008ns | +26.9% | +14.4% | +10.2% | +51.1% | +35.3% |
| 1 | 4 | 4157ns | +3.1% | -62.1% | -6.7% | +4.7% | -68.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.259 | moderate- |
| warm-container-kernel | -0.576 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | 0.106 | ok |
| warm-container-minimum | -0.388 | moderate- |
| warm-container-native | -0.250 | moderate- |
| warm-container-plusone | -0.208 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 53.0ns | 3457.9ns | 1.5% |  |
| warm-container-kernel | 54.7ns | 3570.3ns | 1.5% |  |
| warm-container-lanes-deferred | 55.7ns | 3622.6ns | 1.5% |  |
| warm-container-minimum | 49.5ns | 3205.6ns | 1.5% |  |
| warm-container-native | 54.2ns | 3527.0ns | 1.5% |  |
| warm-container-plusone | 51.2ns | 3357.6ns | 1.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1475.1-5733.4 ns)
   1475.1 |########################################
   1688.0 |##
   1900.9 |
   2113.8 |
   2326.7 |
   2539.7 |
   2752.6 |
   2965.5 |
   3178.4 |
   3391.3 |
   3604.2 |##
   3817.2 |##
   4030.1 |##
   4243.0 |########
   4455.9 |##
   4668.8 |########
   4881.8 |
   5094.7 |###########
   5307.6 |##
   5520.5 |##############
  (3 below, 2 above range)

warm-container-kernel (n=40, range 1507.2-4987.2 ns)
   1507.2 |##########################
   1681.2 |
   1855.2 |#############
   2029.2 |
   2203.2 |
   2377.2 |
   2551.2 |
   2725.2 |
   2899.2 |
   3073.2 |
   3247.2 |######
   3421.2 |##########################
   3595.2 |####################
   3769.2 |########################################
   3943.2 |########################################
   4117.2 |
   4291.2 |####################
   4465.2 |
   4639.2 |##########################
   4813.2 |
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1495.4-5719.4 ns)
   1495.4 |########################################
   1706.6 |
   1917.8 |###
   2129.0 |
   2340.2 |
   2551.4 |
   2762.6 |
   2973.8 |
   3185.0 |###
   3396.2 |
   3607.4 |##########
   3818.6 |###
   4029.8 |##########
   4241.0 |##############
   4452.2 |###
   4663.4 |#######
   4874.6 |###
   5085.8 |#######
   5297.0 |#######
   5508.2 |##############
  (2 below, 2 above range)

warm-container-minimum (n=40, range 1280.0-5183.3 ns)
   1280.0 |########################################
   1475.1 |################
   1670.3 |
   1865.5 |
   2060.6 |####
   2255.8 |
   2451.0 |
   2646.1 |
   2841.3 |
   3036.4 |
   3231.6 |####
   3426.8 |####
   3621.9 |
   3817.1 |
   4012.3 |
   4207.4 |################
   4402.6 |############
   4597.8 |####################
   4792.9 |####
   4988.1 |############
  (3 below, 4 above range)

warm-container-native (n=40, range 1527.7-5912.9 ns)
   1527.7 |########################################
   1746.9 |
   1966.2 |
   2185.5 |###
   2404.7 |
   2624.0 |
   2843.3 |
   3062.5 |###
   3281.8 |###
   3501.0 |######
   3720.3 |######
   3939.6 |#########
   4158.8 |#########
   4378.1 |######
   4597.3 |######
   4816.6 |###
   5035.9 |######
   5255.1 |###
   5474.4 |
   5693.6 |######
  (1 below, 3 above range)

warm-container-plusone (n=40, range 1289.1-5805.5 ns)
   1289.1 |####################
   1514.9 |########################################
   1740.7 |
   1966.5 |
   2192.4 |
   2418.2 |#####
   2644.0 |#####
   2869.8 |#####
   3095.7 |
   3321.5 |
   3547.3 |
   3773.1 |#####
   3998.9 |###############
   4224.8 |#####
   4450.6 |#####
   4676.4 |###############
   4902.2 |
   5128.0 |#####
   5353.9 |###############
   5579.7 |####################
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=52.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=33.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=45.5% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-minimum**: CV=52.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=48.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=54.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.5x (possible bimodal distribution)
