# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged dominates: 19% faster than the next best (wide-rung-align16)

wide-rung-ragged (3.36 us) leads wide-rung-align16 (4.00 us) by 19%, a clear separation rather than a photo finish. CV 45.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (864 ns) is smaller than the fastest variant's own run-to-run std-dev (1.52 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround is inconsistent: worst-20% is 2.8x its best-20%

wide-rung-wordround's best 20% of batches run at 3.27 us but its worst 20% at 9.22 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (27 ns, 0.67%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by 27 ns (0.67%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 3358.4 ns median (-16.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 3358.4 ns, slowest 4222.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112722093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4090ns | 4084ns | 3341ns | 3816ns | 5663ns | base |
| wide-rung-ragged | 4135ns | 3427ns | 3414ns | 3615ns | 6417ns | +1.09% |
| wide-rung-ragged-overread | 4418ns | 4304ns | 3530ns | 4060ns | 6379ns | +8.01% |
| wide-rung-wordround | 5392ns | 4096ns | 3335ns | 4723ns | 9454ns | +31.82% |
| wide-rung-wordround-alias | 4516ns | 4110ns | 3361ns | 4114ns | 6877ns | +10.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4000ns | 3274ns | 5511ns | base | 1.024 |
| wide-rung-ragged | 4045ns | 3347ns | 6250ns | +1.12% | 1.013 |
| wide-rung-ragged-overread | 4329ns | 3463ns | 6234ns | +8.21% | 0.946 |
| wide-rung-wordround | 5269ns | 3268ns | 9222ns | +31.72% | 0.777 |
| wide-rung-wordround-alias | 4418ns | 3295ns | 6699ns | +10.45% | 0.927 |

## Performance model

- Peak throughput: **1.253 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.024 | 81.7% |
| wide-rung-ragged | 1.220 | 97.3% |
| wide-rung-ragged-overread | 0.970 | 77.4% |
| wide-rung-wordround | 1.021 | 81.4% |
| wide-rung-wordround-alias | 1.017 | 81.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4090ns | 4090ns | base |
| wide-rung-ragged | 4135ns | 4135ns | +1.09% |
| wide-rung-ragged-overread | 4418ns | 4418ns | +8.01% |
| wide-rung-wordround | 5392ns | 5392ns | +31.82% |
| wide-rung-wordround-alias | 4516ns | 4516ns | +10.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4002ns | base | --- | [3291, 4009] | --- | --- | --- | --- |
| wide-rung-ragged | 3358ns | no significant difference | [-649, +80]ns | [3353, 3739] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-ragged-overread | 4222ns | no significant difference | [-447, +965]ns | [3574, 4247] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround | 4014ns | no significant difference | [-3, +2430]ns | [3324, 6447] | no | 0.1614 | 0.0807 | 0 |
| wide-rung-wordround-alias | 4029ns | +27.0ns (+0.7%) | [+18, +39]ns | [4027, 4039] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4012ns | -16.4% | -13.6% | +22.8% | +0.7% |
| 1 | 2 | 3282ns | +2.1% | +29.1% | +22.3% | +0.4% |
| 1 | 3 | 3285ns | +24.7% | +32.8% | -0.5% | +22.6% |
| 1 | 4 | 4010ns | -16.4% | -11.0% | +61.2% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.250 | moderate- |
| wide-rung-ragged | -0.416 | moderate- |
| wide-rung-ragged-overread | -0.240 | moderate- |
| wide-rung-wordround | -0.311 | moderate- |
| wide-rung-wordround-alias | -0.418 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 1/4, lost 3/4
- **wide-rung-wordround-alias**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 24.1ns | 4000.0ns | 0.6% |  |
| wide-rung-ragged | 23.9ns | 4044.8ns | 0.6% |  |
| wide-rung-ragged-overread | 24.3ns | 4328.5ns | 0.6% |  |
| wide-rung-wordround | 33.5ns | 5268.8ns | 0.6% |  |
| wide-rung-wordround-alias | 26.8ns | 4418.0ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3274.3-5511.2 ns)
   3274.3 |###########################
   3386.2 |
   3498.0 |
   3609.9 |
   3721.7 |
   3833.6 |
   3945.4 |########################################
   4057.2 |
   4169.1 |
   4280.9 |
   4392.8 |
   4504.6 |
   4616.4 |
   4728.3 |
   4840.1 |##
   4952.0 |
   5063.8 |
   5175.7 |
   5287.5 |
   5399.3 |##
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 3346.7-6250.4 ns)
   3346.7 |########################################
   3491.8 |
   3637.0 |
   3782.2 |
   3927.4 |
   4072.6 |###############
   4217.8 |
   4363.0 |
   4508.2 |
   4653.4 |
   4798.5 |
   4943.7 |
   5088.9 |
   5234.1 |
   5379.3 |#
   5524.5 |#
   5669.7 |
   5814.9 |
   5960.1 |
   6105.2 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 3463.0-6234.3 ns)
   3463.0 |########################################
   3601.6 |
   3740.2 |
   3878.7 |
   4017.3 |
   4155.8 |#########################
   4294.4 |############################
   4433.0 |
   4571.5 |
   4710.1 |
   4848.7 |
   4987.2 |
   5125.8 |
   5264.4 |
   5402.9 |
   5541.5 |
   5680.1 |
   5818.6 |
   5957.2 |
   6095.8 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 3268.0-9221.6 ns)
   3268.0 |########################################
   3565.6 |
   3863.3 |################################
   4161.0 |
   4458.7 |
   4756.4 |
   5054.0 |
   5351.7 |
   5649.4 |
   5947.1 |
   6244.8 |####################################
   6542.4 |
   6840.1 |
   7137.8 |
   7435.5 |###
   7733.1 |
   8030.8 |
   8328.5 |
   8626.2 |
   8923.9 |
  (4 below, 5 above range)

wide-rung-wordround-alias (n=40, range 3295.3-6698.9 ns)
   3295.3 |####
   3465.5 |
   3635.6 |
   3805.8 |
   3976.0 |########################################
   4146.2 |
   4316.4 |
   4486.5 |
   4656.7 |
   4826.9 |#
   4997.1 |
   5167.3 |
   5337.4 |
   5507.6 |#
   5677.8 |
   5848.0 |
   6018.2 |
   6188.3 |
   6358.5 |
   6528.7 |
  (5 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=31.1% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=37.6% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=35.3% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=46.9% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=39.1% (high variance, measurements may be unstable)
