# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred is fastest but the noisiest (CV 81.1%)

warm-container-lanes-deferred wins on median (2.54 us) yet has the highest variance (CV 81.1%), while warm-container-kernel is the steadiest (CV 38.0%, 4.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.54)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.55 us) is smaller than the fastest variant's own run-to-run std-dev (2.06 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-lanes-deferred} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom, warm-container-kernel} (43% apart)

The field splits into a fast tier {warm-container-lanes-deferred} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom, warm-container-kernel} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 5.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.23 us but its worst 20% at 6.22 us (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2537.7 ns median (-35.4% vs baseline)
- Spread: 1.61x (fastest 2537.7 ns, slowest 4084.6 ns)

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
| warm-container-headroom | 3643ns | 4129ns | 1441ns | 3564ns | 6079ns | base |
| warm-container-kernel | 3934ns | 4312ns | 1649ns | 4026ns | 5944ns | +8.01% |
| warm-container-lanes-deferred | 3570ns | 2665ns | 1294ns | 3313ns | 6619ns | -1.98% |
| warm-container-minimum | 3607ns | 3818ns | 1489ns | 3389ns | 6378ns | -0.97% |
| warm-container-native | 3688ns | 4060ns | 1646ns | 3594ns | 6012ns | +1.25% |
| warm-container-plusone | 3664ns | 4096ns | 1506ns | 3559ns | 6138ns | +0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3450ns | 1371ns | 5728ns | base | 4.749 |
| warm-container-kernel | 3724ns | 1568ns | 5600ns | +7.92% | 4.400 |
| warm-container-lanes-deferred | 3379ns | 1227ns | 6220ns | -2.07% | 4.849 |
| warm-container-minimum | 3411ns | 1412ns | 6012ns | -1.13% | 4.803 |
| warm-container-native | 3488ns | 1563ns | 5670ns | +1.09% | 4.697 |
| warm-container-plusone | 3473ns | 1432ns | 5806ns | +0.64% | 4.718 |

## Performance model

- Peak throughput: **13.349 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.174 | 31.3% |
| warm-container-kernel | 4.011 | 30.0% |
| warm-container-lanes-deferred | 6.456 | 48.4% |
| warm-container-minimum | 4.521 | 33.9% |
| warm-container-native | 4.266 | 32.0% |
| warm-container-plusone | 4.206 | 31.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3643ns | 3643ns | base |
| warm-container-kernel | 3934ns | 3934ns | +8.01% |
| warm-container-lanes-deferred | 3570ns | 3570ns | -1.98% |
| warm-container-minimum | 3607ns | 3607ns | -0.97% |
| warm-container-native | 3688ns | 3688ns | +1.25% |
| warm-container-plusone | 3664ns | 3664ns | +0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3926ns | base | --- | [1582, 4632] | --- | --- | --- | --- |
| warm-container-kernel | 4085ns | no significant difference | [-5, +109]ns | [3190, 4727] | no | 0.4296 | 0.4296 | 0 |
| warm-container-lanes-deferred | 2538ns | no significant difference | [-194, +61]ns | [1565, 4864] | no | 0.4296 | 0.2682 | 0 |
| warm-container-minimum | 3624ns | no significant difference | [-389, +130]ns | [1569, 4455] | no | 0.4296 | 0.4296 | 0 |
| warm-container-native | 3840ns | no significant difference | [-430, +164]ns | [1602, 4546] | no | 0.4296 | 0.1539 | 0 |
| warm-container-plusone | 3896ns | no significant difference | [-6, +36]ns | [1804, 4380] | no | 0.4296 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4789ns | +1.2% | -46.5% | -25.9% | -22.9% | -60.9% |
| 1 | 2 | 3926ns | -3.3% | -60.1% | -60.0% | -60.0% | -0.8% |
| 1 | 3 | 2348ns | +110.5% | +89.1% | +77.9% | +69.5% | +74.3% |
| 1 | 4 | 2947ns | -1.4% | -13.9% | +31.4% | +50.5% | +14.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.201 | moderate+ |
| warm-container-kernel | -0.540 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.496 | moderate- |
| warm-container-minimum | -0.350 | moderate- |
| warm-container-native | -0.201 | moderate- |
| warm-container-plusone | -0.109 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 55.7ns | 3450.3ns | 1.6% |  |
| warm-container-kernel | 59.4ns | 3723.6ns | 1.6% |  |
| warm-container-lanes-deferred | 54.4ns | 3379.0ns | 1.6% |  |
| warm-container-minimum | 55.7ns | 3411.3ns | 1.6% |  |
| warm-container-native | 55.9ns | 3487.9ns | 1.6% |  |
| warm-container-plusone | 54.3ns | 3472.5ns | 1.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1370.6-5727.8 ns)
   1370.6 |########################################
   1588.5 |
   1806.4 |########
   2024.2 |
   2242.1 |
   2459.9 |
   2677.8 |
   2895.7 |
   3113.5 |####
   3331.4 |
   3549.2 |
   3767.1 |########
   3984.9 |############
   4202.8 |########
   4420.7 |####
   4638.5 |########
   4856.4 |####
   5074.2 |############
   5292.1 |####
   5509.9 |################
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1567.9-5599.7 ns)
   1567.9 |########################################
   1769.4 |
   1971.0 |
   2172.6 |######
   2374.2 |
   2575.8 |######
   2777.4 |######
   2979.0 |
   3180.6 |
   3382.2 |#############
   3583.8 |#############
   3785.4 |######
   3986.9 |#############
   4188.5 |#############
   4390.1 |
   4591.7 |#################################
   4793.3 |######
   4994.9 |####################
   5196.5 |######
   5398.1 |#############
  (5 below, 5 above range)

warm-container-lanes-deferred (n=40, range 1227.3-6219.9 ns)
   1227.3 |##########
   1477.0 |########################################
   1726.6 |
   1976.2 |
   2225.8 |
   2475.5 |
   2725.1 |
   2974.7 |
   3224.3 |
   3474.0 |###
   3723.6 |###
   3973.2 |###
   4222.9 |###
   4472.5 |###
   4722.1 |###
   4971.7 |##########
   5221.4 |###
   5471.0 |#############
   5720.6 |###
   5970.2 |###
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1412.1-6011.7 ns)
   1412.1 |########################################
   1642.1 |
   1872.0 |
   2102.0 |
   2332.0 |##
   2562.0 |
   2792.0 |
   3021.9 |
   3251.9 |##
   3481.9 |##
   3711.9 |#####
   3941.9 |########
   4171.8 |##
   4401.8 |##
   4631.8 |##
   4861.8 |#####
   5091.8 |##########
   5321.7 |##
   5551.7 |#####
   5781.7 |
  (2 below, 3 above range)

warm-container-native (n=40, range 1562.9-5670.5 ns)
   1562.9 |########################################
   1768.2 |
   1973.6 |
   2179.0 |
   2384.4 |
   2589.8 |###
   2795.1 |
   3000.5 |###
   3205.9 |
   3411.3 |###
   3616.7 |######
   3822.1 |######
   4027.4 |######
   4232.8 |######
   4438.2 |
   4643.6 |#############
   4849.0 |###
   5054.3 |##########
   5259.7 |
   5465.1 |###
  (3 below, 5 above range)

warm-container-plusone (n=40, range 1431.5-5806.0 ns)
   1431.5 |########################################
   1650.3 |
   1869.0 |####
   2087.7 |####
   2306.4 |
   2525.2 |
   2743.9 |
   2962.6 |####
   3181.3 |
   3400.1 |
   3618.8 |############
   3837.5 |########
   4056.2 |############
   4275.0 |############
   4493.7 |####
   4712.4 |########
   4931.1 |
   5149.8 |####
   5368.6 |####
   5587.3 |################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-kernel**: CV=41.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=54.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-native**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=48.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
