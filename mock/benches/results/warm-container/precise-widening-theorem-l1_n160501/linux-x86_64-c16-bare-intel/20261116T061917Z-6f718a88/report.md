# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom is fastest but the noisiest (CV 108.3%)

warm-container-headroom wins on median (2.32 us) yet has the highest variance (CV 108.3%), while warm-container-lanes-deferred is the steadiest (CV 30.2%, 4.04 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.72 us) is smaller than the fastest variant's own run-to-run std-dev (2.51 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (2.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-container-headroom, warm-container-plusone, warm-container-minimum, warm-container-native} vs {warm-container-kernel, warm-container-lanes-deferred} (34% apart)

The field splits into a fast tier {warm-container-headroom, warm-container-plusone, warm-container-minimum, warm-container-native} and a slow tier {warm-container-kernel, warm-container-lanes-deferred} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 4.2x its best-20%

warm-container-kernel's best 20% of batches run at 1.13 us but its worst 20% at 4.69 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 2321.4 ns median
- 1 variant significantly faster than baseline
- Spread: 1.74x (fastest 2321.4 ns, slowest 4036.5 ns)

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
| warm-container-headroom | 3877ns | 2374ns | 2350ns | 2718ns | 8877ns | base |
| warm-container-kernel | 3544ns | 3911ns | 1184ns | 3850ns | 4983ns | -8.58% |
| warm-container-lanes-deferred | 3880ns | 4242ns | 1542ns | 4266ns | 5059ns | +0.08% |
| warm-container-minimum | 4190ns | 2679ns | 2189ns | 3484ns | 8306ns | +8.08% |
| warm-container-native | 4086ns | 2851ns | 2323ns | 3057ns | 8936ns | +5.41% |
| warm-container-plusone | 3945ns | 2655ns | 2173ns | 3084ns | 8300ns | +1.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3777ns | 2297ns | 8615ns | base | 4.338 |
| warm-container-kernel | 3359ns | 1127ns | 4695ns | -11.06% | 4.877 |
| warm-container-lanes-deferred | 3676ns | 1467ns | 4739ns | -2.67% | 4.457 |
| warm-container-minimum | 4081ns | 2137ns | 8059ns | +8.04% | 4.015 |
| warm-container-native | 3984ns | 2272ns | 8678ns | +5.48% | 4.113 |
| warm-container-plusone | 3841ns | 2121ns | 8048ns | +1.69% | 4.266 |

## Performance model

- Peak throughput: **14.538 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.058 | 48.5% |
| warm-container-kernel | 4.400 | 30.3% |
| warm-container-lanes-deferred | 4.059 | 27.9% |
| warm-container-minimum | 6.261 | 43.1% |
| warm-container-native | 5.878 | 40.4% |
| warm-container-plusone | 6.321 | 43.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3877ns | 3877ns | base |
| warm-container-kernel | 3544ns | 3544ns | -8.58% |
| warm-container-lanes-deferred | 3880ns | 3880ns | +0.08% |
| warm-container-minimum | 4190ns | 4190ns | +8.08% |
| warm-container-native | 4086ns | 4086ns | +5.41% |
| warm-container-plusone | 3945ns | 3945ns | +1.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2321ns | base | --- | [2313, 2809] | --- | --- | --- | --- |
| warm-container-kernel | 3724ns | no significant difference | [-1074, +897]ns | [3439, 4056] | no | 0.8746 | 0.8746 | 0 |
| warm-container-lanes-deferred | 4036ns | no significant difference | [-985, +1418]ns | [3724, 4432] | no | 0.4470 | 0.2682 | 0 |
| warm-container-minimum | 2617ns | no significant difference | [-176, +294]ns | [2145, 4434] | no | 0.3846 | 0.1539 | 0 |
| warm-container-native | 2788ns | no significant difference | [-38, +13]ns | [2741, 2816] | no | 0.5370 | 0.4296 | 0 |
| warm-container-plusone | 2592ns | -176.3ns (-7.6%) | [-198, -46]ns | [2136, 3092] | YES (adj: no) | 0.3846 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2320ns | +44.0% | +2.1% | -7.5% | -1.2% | -7.4% |
| 1 | 2 | 2825ns | +26.2% | +54.5% | -14.4% | -0.5% | -24.1% |
| 1 | 3 | 2299ns | +70.7% | +76.4% | +13.9% | +21.8% | -7.5% |
| 1 | 4 | 2316ns | +75.1% | +71.9% | +79.7% | -0.6% | +12.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.420 | moderate- |
| warm-container-kernel | 0.297 | moderate+ |
| warm-container-lanes-deferred | -0.223 | moderate- |
| warm-container-minimum | 0.037 | ok |
| warm-container-native | -0.250 | moderate- |
| warm-container-plusone | -0.112 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 33.9ns | 3777.0ns | 0.9% |  |
| warm-container-kernel | 66.8ns | 3359.2ns | 2.0% |  |
| warm-container-lanes-deferred | 73.1ns | 3676.1ns | 2.0% |  |
| warm-container-minimum | 37.8ns | 4080.8ns | 0.9% |  |
| warm-container-native | 34.6ns | 3983.8ns | 0.9% |  |
| warm-container-plusone | 36.6ns | 3840.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2297.5-8615.4 ns)
   2297.5 |########################################
   2613.4 |#################
   2929.3 |
   3245.2 |
   3561.1 |##
   3876.9 |
   4192.8 |
   4508.7 |
   4824.6 |##
   5140.5 |
   5456.4 |
   5772.3 |
   6088.2 |
   6404.1 |
   6720.0 |
   7035.9 |##
   7351.8 |##
   7667.7 |##
   7983.6 |##
   8299.5 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1127.0-4695.0 ns)
   1127.0 |###########
   1305.4 |
   1483.8 |#####
   1662.2 |
   1840.6 |#####
   2019.0 |#####
   2197.4 |
   2375.8 |#####
   2554.2 |
   2732.6 |
   2911.0 |
   3089.4 |#####
   3267.8 |###########
   3446.2 |###########
   3624.6 |############################
   3803.0 |#####
   3981.4 |############################
   4159.8 |###########
   4338.2 |########################################
   4516.6 |
  (5 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1467.0-4738.6 ns)
   1467.0 |
   1630.6 |
   1794.2 |
   1957.8 |###
   2121.4 |
   2284.9 |
   2448.5 |
   2612.1 |
   2775.7 |
   2939.2 |
   3102.8 |
   3266.4 |###
   3430.0 |###
   3593.6 |#####################
   3757.1 |###
   3920.7 |#####################
   4084.3 |###
   4247.9 |###
   4411.4 |########################################
   4575.0 |###
  (6 below, 4 above range)

warm-container-minimum (n=40, range 2137.4-8059.1 ns)
   2137.4 |########################################
   2433.5 |########################
   2729.6 |
   3025.7 |
   3321.8 |
   3617.9 |
   3913.9 |
   4210.0 |
   4506.1 |
   4802.2 |
   5098.3 |
   5394.3 |
   5690.4 |
   5986.5 |#####
   6282.6 |##
   6578.7 |########
   6874.7 |#####
   7170.8 |#####
   7466.9 |
   7763.0 |
  (2 below, 4 above range)

warm-container-native (n=40, range 2271.5-8678.4 ns)
   2271.5 |#########################
   2591.8 |########################################
   2912.2 |##
   3232.5 |
   3552.9 |
   3873.2 |
   4193.6 |
   4513.9 |
   4834.3 |
   5154.6 |
   5474.9 |##
   5795.3 |
   6115.6 |
   6436.0 |
   6756.3 |##
   7076.7 |##
   7397.0 |##
   7717.4 |##
   8037.7 |##
   8358.1 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 2121.4-8047.6 ns)
   2121.4 |########################################
   2417.7 |##################
   2714.0 |
   3010.3 |
   3306.6 |##
   3602.9 |##
   3899.3 |##
   4195.6 |
   4491.9 |
   4788.2 |
   5084.5 |##
   5380.8 |
   5677.1 |
   5973.4 |
   6269.8 |
   6566.1 |##
   6862.4 |##
   7158.7 |#####
   7455.0 |##
   7751.3 |#####
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=66.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=38.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=33.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=61.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=63.7% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=62.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
