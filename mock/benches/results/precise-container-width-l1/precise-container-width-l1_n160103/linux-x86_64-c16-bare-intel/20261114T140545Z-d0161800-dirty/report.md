# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 4901% faster than the next best (warm-container-minimum)

warm-container-kernel (58 ns) leads warm-container-minimum (2.88 us) by 4901%, a clear separation rather than a photo finish. CV 57.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -3.96 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 69.8x slower than the field

warm-container-plusone (4.02 us) is 69.8x the fastest (58 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-headroom, warm-container-plusone} (4901% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-headroom, warm-container-plusone} with a 4901% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 69.8x the fastest

Fastest warm-container-kernel (58 ns) to slowest warm-container-plusone (4.02 us): 69.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.8x its best-20%

warm-container-native's best 20% of batches run at 2.48 us but its worst 20% at 9.42 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 57.5 ns median (-98.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 69.81x (fastest 57.5 ns, slowest 4017.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4927ns | 4076ns | 3429ns | 4081ns | 8961ns | base |
| warm-container-kernel | 346ns | 324ns | 277ns | 328ns | 468ns | -92.98% |
| warm-container-minimum | 4621ns | 2942ns | 2934ns | 3381ns | 10027ns | -6.21% |
| warm-container-native | 4670ns | 2945ns | 2537ns | 3708ns | 9688ns | -5.22% |
| warm-container-plusone | 5203ns | 4082ns | 4075ns | 4082ns | 9692ns | +5.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4840ns | 3377ns | 8771ns | base | 6.770 |
| warm-container-kernel | 67ns | 52ns | 108ns | -98.61% | 486.262 |
| warm-container-minimum | 4510ns | 2871ns | 9757ns | -6.82% | 7.265 |
| warm-container-native | 4557ns | 2482ns | 9420ns | -5.85% | 7.191 |
| warm-container-plusone | 5111ns | 4011ns | 9487ns | +5.58% | 6.412 |

## Performance model

- Peak throughput: **634.117 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.166 | 1.3% |
| warm-container-kernel | 569.383 | 89.8% |
| warm-container-minimum | 11.385 | 1.8% |
| warm-container-native | 11.375 | 1.8% |
| warm-container-plusone | 8.156 | 1.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4927ns | 4927ns | base |
| warm-container-kernel | 346ns | 346ns | -92.98% |
| warm-container-minimum | 4621ns | 4621ns | -6.21% |
| warm-container-native | 4670ns | 4670ns | -5.22% |
| warm-container-plusone | 5203ns | 5203ns | +5.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4013ns | base | --- | [4012, 4025] | --- | --- | --- | --- |
| warm-container-kernel | 58ns | -3960.5ns (-98.7%) | [-3968, -3955]ns | [57, 64] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2878ns | -1133.9ns (-28.3%) | [-1141, -422]ns | [2872, 2884] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 2881ns | -1127.3ns (-28.1%) | [-1141, -411]ns | [2872, 3119] | YES | 0.0009 | 0.0007 | 0 |
| warm-container-plusone | 4018ns | no significant difference | [-1, +96]ns | [4012, 4024] | no | 0.5224 | 0.5224 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4020ns | -98.7% | -28.4% | -41.3% | -0.1% |
| 1 | 2 | 3288ns | -98.0% | -12.6% | -12.4% | +22.2% |
| 1 | 3 | 4018ns | -98.4% | -28.4% | -28.2% | +0.0% |
| 1 | 4 | 4019ns | -98.6% | -28.2% | -28.4% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.417 | moderate- |
| warm-container-kernel | -0.266 | moderate- |
| warm-container-minimum | 0.020 | ok |
| warm-container-native | -0.080 | ok |
| warm-container-plusone | -0.379 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.9ns | 4840.2ns | 0.6% |  |
| warm-container-kernel | 100.6ns | 67.4ns | 149.3% | HIGH |
| warm-container-minimum | 38.4ns | 4510.3ns | 0.9% |  |
| warm-container-native | 39.0ns | 4556.9ns | 0.9% |  |
| warm-container-plusone | 31.5ns | 5110.5ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3376.5-8770.9 ns)
   3376.5 |
   3646.3 |
   3916.0 |########################################
   4185.7 |#
   4455.4 |
   4725.1 |
   4994.9 |
   5264.6 |
   5534.3 |
   5804.0 |
   6073.7 |
   6343.4 |
   6613.2 |#
   6882.9 |
   7152.6 |
   7422.3 |
   7692.0 |
   7961.7 |
   8231.5 |
   8501.2 |
  (7 below, 5 above range)

warm-container-kernel (n=40, range 51.7-108.0 ns)
     51.7 |#######################
     54.5 |#######################
     57.3 |#######################
     60.1 |###
     62.9 |########################################
     65.8 |
     68.6 |
     71.4 |
     74.2 |
     77.0 |
     79.8 |
     82.7 |
     85.5 |
     88.3 |
     91.1 |
     93.9 |
     96.8 |
     99.6 |
    102.4 |
    105.2 |
  (3 below, 3 above range)

warm-container-minimum (n=40, range 2871.4-9756.9 ns)
   2871.4 |########################################
   3215.7 |
   3559.9 |
   3904.2 |#
   4248.5 |
   4592.8 |
   4937.1 |
   5281.3 |
   5625.6 |
   5969.9 |
   6314.2 |
   6658.4 |#
   7002.7 |
   7347.0 |
   7691.3 |#
   8035.6 |
   8379.8 |#
   8724.1 |
   9068.4 |#
   9412.7 |###
  (3 below, 4 above range)

warm-container-native (n=40, range 2482.4-9419.6 ns)
   2482.4 |
   2829.3 |########################################
   3176.1 |##
   3523.0 |
   3869.8 |##
   4216.7 |
   4563.5 |
   4910.4 |
   5257.3 |
   5604.1 |
   5951.0 |##
   6297.8 |
   6644.7 |##
   6991.6 |##
   7338.4 |####
   7685.3 |
   8032.1 |##
   8379.0 |
   8725.8 |##
   9072.7 |
  (6 below, 5 above range)

warm-container-plusone (n=40, range 4011.5-9487.0 ns)
   4011.5 |########################################
   4285.2 |#
   4559.0 |
   4832.8 |
   5106.6 |
   5380.4 |##
   5654.1 |
   5927.9 |
   6201.7 |
   6475.5 |
   6749.2 |
   7023.0 |#
   7296.8 |
   7570.6 |
   7844.4 |
   8118.1 |
   8391.9 |
   8665.7 |
   8939.5 |
   9213.3 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=50.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=49.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=168.6% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=62.3% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-native**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=54.9% (high variance, measurements may be unstable)
