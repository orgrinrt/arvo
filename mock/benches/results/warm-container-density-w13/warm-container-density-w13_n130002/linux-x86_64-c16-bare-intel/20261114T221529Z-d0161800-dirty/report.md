# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 68% faster than the next best (warm-container-kernel)

warm-container-native (1.80 us) leads warm-container-kernel (3.03 us) by 68%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 59% (significant)

warm-container-native is -2.56 us (59%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.9x slower than the field

warm-container-minimum (5.20 us) is 2.9x the fastest (1.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.60)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native} vs {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone, warm-container-headroom, warm-container-minimum} (68% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone, warm-container-headroom, warm-container-minimum} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 3.8x its best-20%

warm-container-minimum's best 20% of batches run at 1.78 us but its worst 20% at 6.70 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-40 ns, 0.92%)

warm-container-plusone differs from baseline warm-container-headroom by -40 ns (0.92%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1798.5 ns median (-58.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.89x (fastest 1798.5 ns, slowest 5195.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4124ns | 4539ns | 1858ns | 3958ns | 6887ns | base |
| warm-container-kernel | 3250ns | 3217ns | 2639ns | 3215ns | 3966ns | -21.19% |
| warm-container-lanes-deferred | 3561ns | 3544ns | 2714ns | 3565ns | 4394ns | -13.65% |
| warm-container-minimum | 4449ns | 5380ns | 1843ns | 4463ns | 7015ns | +7.90% |
| warm-container-native | 1976ns | 2030ns | 1717ns | 1999ns | 2163ns | -52.09% |
| warm-container-plusone | 3776ns | 4031ns | 1848ns | 3586ns | 6274ns | -8.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3969ns | 1794ns | 6600ns | base | 6.193 |
| warm-container-kernel | 3043ns | 2483ns | 3698ns | -23.32% | 8.076 |
| warm-container-lanes-deferred | 3341ns | 2555ns | 4117ns | -15.80% | 7.355 |
| warm-container-minimum | 4280ns | 1780ns | 6702ns | +7.85% | 5.742 |
| warm-container-native | 1741ns | 1527ns | 1872ns | -56.12% | 14.114 |
| warm-container-plusone | 3633ns | 1784ns | 6007ns | -8.44% | 6.764 |

## Performance model

- Peak throughput: **16.096 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.625 | 34.9% |
| warm-container-kernel | 8.118 | 50.4% |
| warm-container-lanes-deferred | 7.389 | 45.9% |
| warm-container-minimum | 4.730 | 29.4% |
| warm-container-native | 13.665 | 84.9% |
| warm-container-plusone | 6.310 | 39.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4124ns | 4124ns | base |
| warm-container-kernel | 3250ns | 3250ns | -21.19% |
| warm-container-lanes-deferred | 3561ns | 3561ns | -13.65% |
| warm-container-minimum | 4449ns | 4449ns | +7.90% |
| warm-container-native | 1976ns | 1976ns | -52.09% |
| warm-container-plusone | 3776ns | 3776ns | -8.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4369ns | base | --- | [1796, 5829] | --- | --- | --- | --- |
| warm-container-kernel | 3027ns | no significant difference | [-2309, +700]ns | [2792, 3293] | no | 0.8746 | 0.8746 | 0 |
| warm-container-lanes-deferred | 3326ns | no significant difference | [-2006, +991]ns | [3067, 3632] | no | 0.7948 | 0.6358 | 0 |
| warm-container-minimum | 5195ns | no significant difference | [-14, +35]ns | [1793, 5896] | no | 0.2564 | 0.1539 | 0 |
| warm-container-native | 1798ns | -2564.6ns (-58.7%) | [-3989, -230]ns | [1784, 1806] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3895ns | -40.3ns (-0.9%) | [-461, -10]ns | [1797, 4475] | YES | 0.0017 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3490ns | -22.0% | -18.9% | +63.2% | -56.5% | +41.5% |
| 1 | 2 | 4873ns | -30.8% | -31.5% | -42.8% | -62.9% | -17.9% |
| 1 | 3 | 1828ns | +94.7% | +121.7% | +147.3% | -1.2% | -2.4% |
| 1 | 4 | 3848ns | -33.3% | -10.9% | +35.0% | -53.3% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.601 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.273 | moderate- |
| warm-container-lanes-deferred | 0.009 | ok |
| warm-container-minimum | -0.411 | moderate- |
| warm-container-native | -0.096 | ok |
| warm-container-plusone | -0.178 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 55.4ns | 3968.6ns | 1.4% |  |
| warm-container-kernel | 74.8ns | 3043.1ns | 2.5% |  |
| warm-container-lanes-deferred | 80.5ns | 3341.5ns | 2.4% |  |
| warm-container-minimum | 60.7ns | 4280.0ns | 1.4% |  |
| warm-container-native | 84.0ns | 1741.3ns | 4.8% |  |
| warm-container-plusone | 49.7ns | 3633.4ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1794.2-6600.4 ns)
   1794.2 |########################################
   2034.5 |
   2274.8 |
   2515.1 |
   2755.5 |
   2995.8 |
   3236.1 |
   3476.4 |
   3716.7 |
   3957.0 |##
   4197.3 |
   4437.6 |
   4677.9 |##
   4918.2 |##
   5158.5 |#####
   5398.8 |##
   5639.1 |##
   5879.4 |############
   6119.7 |
   6360.1 |############
  (3 below, 4 above range)

warm-container-kernel (n=40, range 2483.2-3697.7 ns)
   2483.2 |########
   2544.0 |########################################
   2604.7 |########
   2665.4 |
   2726.1 |########
   2786.8 |########################################
   2847.6 |
   2908.3 |
   2969.0 |########################################
   3029.7 |
   3090.4 |################
   3151.2 |
   3211.9 |
   3272.6 |################################
   3333.3 |
   3394.1 |########
   3454.8 |########
   3515.5 |
   3576.2 |################################
   3636.9 |########################
  (5 below, 2 above range)

warm-container-lanes-deferred (n=40, range 2554.7-4117.2 ns)
   2554.7 |#############
   2632.9 |######
   2711.0 |######
   2789.1 |####################
   2867.2 |
   2945.3 |#############
   3023.5 |########################################
   3101.6 |######
   3179.7 |
   3257.8 |####################
   3336.0 |#############
   3414.1 |######
   3492.2 |######
   3570.3 |#################################
   3648.4 |##########################
   3726.6 |######
   3804.7 |
   3882.8 |
   3960.9 |######
   4039.0 |#############
  (1 below, 3 above range)

warm-container-minimum (n=40, range 1780.3-6701.5 ns)
   1780.3 |########################################
   2026.3 |
   2272.4 |###
   2518.4 |
   2764.5 |
   3010.6 |
   3256.6 |
   3502.7 |###
   3748.8 |###
   3994.8 |
   4240.9 |
   4486.9 |
   4733.0 |
   4979.1 |######
   5225.1 |######
   5471.2 |######
   5717.3 |#########
   5963.3 |
   6209.4 |#####################
   6455.4 |#########
  (2 below, 3 above range)

warm-container-native (n=40, range 1526.8-1871.5 ns)
   1526.8 |##
   1544.0 |##
   1561.3 |
   1578.5 |
   1595.8 |
   1613.0 |
   1630.2 |###########
   1647.5 |##
   1664.7 |
   1681.9 |##
   1699.2 |
   1716.4 |
   1733.7 |
   1750.9 |
   1768.1 |#####
   1785.4 |#################
   1802.6 |########################################
   1819.8 |
   1837.1 |#####
   1854.3 |##
  (5 below, 2 above range)

warm-container-plusone (n=40, range 1784.0-6007.0 ns)
   1784.0 |########################################
   1995.1 |
   2206.3 |
   2417.4 |
   2628.6 |
   2839.7 |###
   3050.9 |
   3262.0 |
   3473.2 |
   3684.3 |#########
   3895.5 |######
   4106.6 |############
   4317.8 |
   4528.9 |######
   4740.1 |###
   4951.2 |#########
   5162.4 |###
   5373.5 |#########
   5584.7 |
   5795.8 |######
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=49.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=47.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.4x (possible bimodal distribution)
