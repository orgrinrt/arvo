# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 dominates: 18% faster than the next best (wide-rung-wordround)

wide-rung-align16 (3.65 us) leads wide-rung-wordround (4.31 us) by 18%, a clear separation rather than a photo finish. CV 45.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-align16 is fastest but the noisiest (CV 45.9%)

wide-rung-align16 wins on median (3.65 us) yet has the highest variance (CV 45.9%), while wide-rung-ragged is the steadiest (CV 19.1%, 4.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged-overread shows alternating (throttle bounce) (autocorr -0.73)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.07 us) is smaller than the fastest variant's own run-to-run std-dev (1.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (3.65 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### wide-rung-wordround is inconsistent: worst-20% is 1.9x its best-20%

wide-rung-wordround's best 20% of batches run at 3.66 us but its worst 20% at 6.85 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 3645.6 ns median
- 4 variants significantly slower than baseline
- Spread: 1.29x (fastest 3645.6 ns, slowest 4711.2 ns)

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
| wide-rung-align16 | 4381ns | 3713ns | 3598ns | 3850ns | 6760ns | base |
| wide-rung-ragged | 4784ns | 4792ns | 3926ns | 4759ns | 5717ns | +9.19% |
| wide-rung-ragged-overread | 4607ns | 4656ns | 3819ns | 4393ns | 6039ns | +5.16% |
| wide-rung-wordround | 4645ns | 4394ns | 3725ns | 4160ns | 7021ns | +6.02% |
| wide-rung-wordround-alias | 4848ns | 4555ns | 3731ns | 4558ns | 6836ns | +10.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4293ns | 3530ns | 6602ns | base | 1.431 |
| wide-rung-ragged | 4697ns | 3860ns | 5594ns | +9.40% | 1.308 |
| wide-rung-ragged-overread | 4519ns | 3752ns | 5900ns | +5.26% | 1.360 |
| wide-rung-wordround | 4552ns | 3659ns | 6850ns | +6.04% | 1.350 |
| wide-rung-wordround-alias | 4752ns | 3664ns | 6666ns | +10.69% | 1.293 |

## Performance model

- Peak throughput: **1.740 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.685 | 96.8% |
| wide-rung-ragged | 1.304 | 74.9% |
| wide-rung-ragged-overread | 1.343 | 77.2% |
| wide-rung-wordround | 1.425 | 81.9% |
| wide-rung-wordround-alias | 1.374 | 78.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4381ns | 4381ns | base |
| wide-rung-ragged | 4784ns | 4784ns | +9.19% |
| wide-rung-ragged-overread | 4607ns | 4607ns | +5.16% |
| wide-rung-wordround | 4645ns | 4645ns | +6.02% |
| wide-rung-wordround-alias | 4848ns | 4848ns | +10.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3646ns | base | --- | [3645, 3659] | --- | --- | --- | --- |
| wide-rung-ragged | 4711ns | +419.8ns (+11.5%) | [+389, +1059]ns | [4703, 4719] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4573ns | +252.5ns (+6.9%) | [+234, +269]ns | [3888, 4595] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 4312ns | +119.0ns (+3.3%) | [+10, +141]ns | [3670, 4397] | YES | 0.0064 | 0.0064 | 0 |
| wide-rung-wordround-alias | 4473ns | +162.1ns (+4.4%) | [+160, +827]ns | [4472, 4485] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 3650ns | +29.1% | +6.5% | +0.4% | +0.4% |
| 1 | 2 | 4318ns | +9.2% | +6.2% | -0.1% | +3.7% |
| 1 | 3 | 3529ns | +9.3% | +6.3% | +3.7% | +26.9% |
| 1 | 4 | 3646ns | +29.6% | +30.4% | +22.8% | +22.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.447 | moderate- |
| wide-rung-ragged | -0.426 | moderate- |
| wide-rung-ragged-overread | -0.726 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.678 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.085 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 0/4, lost 4/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 1/4, lost 3/4
- **wide-rung-wordround-alias**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 23.6ns | 4293.3ns | 0.5% |  |
| wide-rung-ragged | 24.0ns | 4697.0ns | 0.5% |  |
| wide-rung-ragged-overread | 23.4ns | 4519.2ns | 0.5% |  |
| wide-rung-wordround | 24.7ns | 4552.5ns | 0.5% |  |
| wide-rung-wordround-alias | 25.6ns | 4752.1ns | 0.5% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3530.5-6601.8 ns)
   3530.5 |########################################
   3684.0 |
   3837.6 |
   3991.2 |
   4144.7 |
   4298.3 |##################
   4451.9 |
   4605.4 |
   4759.0 |
   4912.6 |
   5066.1 |
   5219.7 |
   5373.3 |
   5526.9 |
   5680.4 |
   5834.0 |
   5987.6 |
   6141.1 |
   6294.7 |
   6448.3 |
  (7 below, 4 above range)

wide-rung-ragged (n=40, range 3860.1-5594.2 ns)
   3860.1 |#####
   3946.8 |
   4033.5 |
   4120.2 |
   4206.9 |
   4293.6 |
   4380.3 |
   4467.0 |
   4553.7 |
   4640.4 |########################################
   4727.1 |##########
   4813.8 |
   4900.6 |
   4987.3 |
   5074.0 |
   5160.7 |
   5247.4 |
   5334.1 |
   5420.8 |
   5507.5 |
  (6 below, 2 above range)

wide-rung-ragged-overread (n=40, range 3751.6-5900.2 ns)
   3751.6 |################
   3859.0 |####################################
   3966.4 |
   4073.9 |
   4181.3 |
   4288.7 |
   4396.2 |
   4503.6 |########################################
   4611.0 |
   4718.5 |####################################
   4825.9 |
   4933.3 |
   5040.8 |
   5148.2 |
   5255.6 |####
   5363.1 |
   5470.5 |
   5577.9 |
   5685.4 |
   5792.8 |
  (5 below, 2 above range)

wide-rung-wordround (n=40, range 3659.0-6849.8 ns)
   3659.0 |########################################
   3818.6 |
   3978.1 |
   4137.7 |
   4297.2 |#####################
   4456.7 |########################
   4616.3 |
   4775.8 |
   4935.3 |
   5094.9 |
   5254.4 |
   5414.0 |
   5573.5 |##
   5733.0 |
   5892.6 |
   6052.1 |
   6211.6 |
   6371.2 |##
   6530.7 |
   6690.3 |##
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 3663.9-6665.8 ns)
   3663.9 |####
   3814.0 |
   3964.1 |
   4114.2 |
   4264.2 |
   4414.3 |########################################
   4564.4 |
   4714.5 |
   4864.6 |
   5014.7 |
   5164.8 |
   5314.9 |#
   5465.0 |
   5615.1 |
   5765.2 |
   5915.3 |
   6065.4 |
   6215.5 |
   6365.6 |#
   6515.7 |
  (5 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=39.0% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=25.2% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=38.0% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=30.6% (high variance, measurements may be unstable)
