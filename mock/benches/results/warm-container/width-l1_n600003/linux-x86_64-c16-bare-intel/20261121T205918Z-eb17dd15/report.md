# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 57% faster than the next best (warm-container-minimum)

warm-container-native (1.62 us) leads warm-container-minimum (2.55 us) by 57%, a clear separation rather than a photo finish. CV 115.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 44% (significant)

warm-container-native is -1.24 us (44%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (4.00 us) is 2.5x the fastest (1.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 115.6%)

warm-container-native wins on median (1.62 us) yet has the highest variance (CV 115.6%), while warm-container-kernel is the steadiest (CV 43.0%, 4.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.74)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.5x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.27 us but its worst 20% at 5.75 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1620.3 ns median (-42.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.47x (fastest 1620.3 ns, slowest 3997.8 ns)

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
| warm-container-headroom | 4166ns | 2887ns | 2877ns | 3096ns | 8664ns | base |
| warm-container-kernel | 3595ns | 4208ns | 1362ns | 3596ns | 5828ns | -13.69% |
| warm-container-lanes-deferred | 3440ns | 3930ns | 1338ns | 3255ns | 6095ns | -17.43% |
| warm-container-minimum | 3676ns | 2658ns | 1599ns | 3446ns | 6444ns | -11.75% |
| warm-container-native | 3238ns | 1702ns | 1400ns | 2911ns | 6058ns | -22.26% |
| warm-container-plusone | 3745ns | 4178ns | 1554ns | 3616ns | 6320ns | -10.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4058ns | 2810ns | 8411ns | base | 8.074 |
| warm-container-kernel | 3408ns | 1294ns | 5505ns | -16.02% | 9.614 |
| warm-container-lanes-deferred | 3257ns | 1271ns | 5748ns | -19.75% | 10.060 |
| warm-container-minimum | 3507ns | 1532ns | 6102ns | -13.59% | 9.343 |
| warm-container-native | 3058ns | 1328ns | 5693ns | -24.66% | 10.717 |
| warm-container-plusone | 3574ns | 1487ns | 6019ns | -11.95% | 9.170 |

## Performance model

- Peak throughput: **25.788 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.618 | 45.1% |
| warm-container-kernel | 8.197 | 31.8% |
| warm-container-lanes-deferred | 8.775 | 34.0% |
| warm-container-minimum | 12.871 | 49.9% |
| warm-container-native | 20.223 | 78.4% |
| warm-container-plusone | 8.197 | 31.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4166ns | 4166ns | base |
| warm-container-kernel | 3595ns | 3595ns | -13.69% |
| warm-container-lanes-deferred | 3440ns | 3440ns | -17.43% |
| warm-container-minimum | 3676ns | 3676ns | -11.75% |
| warm-container-native | 3238ns | 3238ns | -22.26% |
| warm-container-plusone | 3745ns | 3745ns | -10.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2820ns | base | --- | [2811, 3435] | --- | --- | --- | --- |
| warm-container-kernel | 3998ns | no significant difference | [-1510, +413]ns | [1582, 4641] | no | 0.1009 | 0.0807 | 0 |
| warm-container-lanes-deferred | 3734ns | no significant difference | [-1543, +149]ns | [1549, 4293] | no | 0.1009 | 0.0807 | 0 |
| warm-container-minimum | 2546ns | -939.2ns (-33.3%) | [-1277, -234]ns | [1870, 4844] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-container-native | 1620ns | -1235.6ns (-43.8%) | [-1462, -1059]ns | [1579, 4520] | YES | 0.0321 | 0.0064 | 0 |
| warm-container-plusone | 3998ns | no significant difference | [-1325, +1035]ns | [1841, 4477] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2811ns | -53.6% | -54.9% | +43.6% | -47.6% | -44.7% |
| 1 | 2 | 2816ns | +42.0% | +36.3% | -33.1% | +7.7% | +47.0% |
| 1 | 3 | 2811ns | +58.9% | +0.2% | +61.9% | -43.8% | +47.2% |
| 1 | 4 | 3441ns | +20.1% | +30.2% | -9.0% | +26.5% | -36.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.087 | ok |
| warm-container-kernel | 0.006 | ok |
| warm-container-lanes-deferred | -0.331 | moderate- |
| warm-container-minimum | -0.736 | HIGH- (thermal bounce) |
| warm-container-native | -0.487 | moderate- |
| warm-container-plusone | -0.241 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.8ns | 4058.5ns | 0.7% |  |
| warm-container-kernel | 52.5ns | 3408.2ns | 1.5% |  |
| warm-container-lanes-deferred | 51.2ns | 3257.1ns | 1.6% |  |
| warm-container-minimum | 46.9ns | 3507.1ns | 1.3% |  |
| warm-container-native | 50.2ns | 3057.7ns | 1.6% |  |
| warm-container-plusone | 47.0ns | 3573.5ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2810.4-8410.8 ns)
   2810.4 |########################################
   3090.4 |
   3370.4 |###############
   3650.4 |
   3930.5 |
   4210.5 |
   4490.5 |
   4770.5 |
   5050.5 |###
   5330.6 |
   5610.6 |
   5890.6 |#
   6170.6 |
   6450.7 |
   6730.7 |
   7010.7 |
   7290.7 |
   7570.7 |
   7850.8 |
   8130.8 |
  (3 below, 5 above range)

warm-container-kernel (n=40, range 1294.1-5504.7 ns)
   1294.1 |##################################
   1504.6 |########################################
   1715.1 |
   1925.7 |#####
   2136.2 |
   2346.7 |#####
   2557.3 |
   2767.8 |
   2978.3 |
   3188.8 |#####
   3399.4 |
   3609.9 |#####
   3820.4 |###########
   4031.0 |#####
   4241.5 |###########
   4452.0 |#################
   4662.5 |#####
   4873.1 |#################
   5083.6 |#################
   5294.1 |###########
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1270.7-5747.6 ns)
   1270.7 |###############
   1494.5 |########################################
   1718.4 |#####
   1942.2 |
   2166.1 |
   2389.9 |
   2613.8 |#####
   2837.6 |
   3061.5 |
   3285.3 |
   3509.2 |#####
   3733.0 |##########
   3956.9 |##########
   4180.7 |###############
   4404.6 |##########
   4628.4 |##########
   4852.2 |
   5076.1 |##########
   5299.9 |#####
   5523.8 |####################
  (6 below, 2 above range)

warm-container-minimum (n=40, range 1532.2-6102.5 ns)
   1532.2 |#############
   1760.7 |########################################
   1989.2 |
   2217.7 |
   2446.3 |
   2674.8 |
   2903.3 |
   3131.8 |###
   3360.3 |
   3588.8 |
   3817.4 |
   4045.9 |
   4274.4 |##########
   4502.9 |###
   4731.4 |##########
   4959.9 |######
   5188.4 |###
   5417.0 |#############
   5645.5 |###
   5874.0 |
  (4 below, 4 above range)

warm-container-native (n=40, range 1327.8-5692.5 ns)
   1327.8 |########################################
   1546.1 |########################################
   1764.3 |#####
   1982.5 |
   2200.8 |
   2419.0 |
   2637.2 |
   2855.5 |
   3073.7 |#####
   3291.9 |
   3510.2 |
   3728.4 |
   3946.6 |#####
   4164.9 |#####
   4383.1 |##########
   4601.3 |###############
   4819.6 |##########
   5037.8 |#########################
   5256.0 |
   5474.3 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 1486.9-6018.6 ns)
   1486.9 |#################################
   1713.4 |########################################
   1940.0 |
   2166.6 |
   2393.2 |######
   2619.8 |######
   2846.4 |
   3073.0 |
   3299.5 |
   3526.1 |
   3752.7 |####################
   3979.3 |##########################
   4205.9 |#############
   4432.5 |#############
   4659.1 |##########################
   4885.7 |
   5112.2 |######
   5338.8 |######
   5565.4 |
   5792.0 |#############
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=50.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=55.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=53.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=61.2% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=48.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
