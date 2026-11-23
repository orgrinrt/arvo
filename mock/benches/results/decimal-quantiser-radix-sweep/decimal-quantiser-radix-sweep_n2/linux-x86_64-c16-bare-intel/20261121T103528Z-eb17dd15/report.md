# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-radix10 dominates: 52% faster than the next best (quantiser-radix2)

quantiser-radix10 (6.23 us) leads quantiser-radix2 (9.48 us) by 52%, a clear separation rather than a photo finish. CV 32.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix10 is fastest but the noisiest (CV 32.5%)

quantiser-radix10 wins on median (6.23 us) yet has the highest variance (CV 32.5%), while quantiser-radix2 is the steadiest (CV 9.7%, 9.48 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix10 shows alternating (throttle bounce) (autocorr -0.69)

quantiser-radix10's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-radix10)

The baseline quantiser-radix10 is the fastest (6.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### quantiser-radix10 is inconsistent: worst-20% is 1.7x its best-20%

quantiser-radix10's best 20% of batches run at 5.15 us but its worst 20% at 8.84 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-radix10) is the fastest** at 6233.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.52x (fastest 6233.4 ns, slowest 9477.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Quantiser cost by radix: decimal32 against binary32, exponent spread swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112757813 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 6392ns | 6301ns | 5202ns | 5932ns | 8963ns | base |
| quantiser-radix2 | 10070ns | 9542ns | 9482ns | 9726ns | 11692ns | +57.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6318ns | 5146ns | 8843ns | base | 0.041 |
| quantiser-radix2 | 10007ns | 9426ns | 11615ns | +58.40% | 0.026 |

## Performance model

- Peak throughput: **0.050 Gops/s** (quantiser-radix10; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.041 | 82.6% |
| quantiser-radix2 | 0.027 | 54.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 6392ns | 6392ns | base |
| quantiser-radix2 | 10070ns | 10070ns | +57.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 6233ns | base | --- | [5297, 6302] | --- | --- | --- | --- |
| quantiser-radix2 | 9478ns | +3240.9ns (+52.0%) | [+3197, +4314]ns | [9465, 9505] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 6337ns | +49.5% |
| 1 | 2 | 5283ns | +119.2% |
| 1 | 3 | 6262ns | +51.2% |
| 1 | 4 | 5154ns | +83.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.694 | HIGH- (thermal bounce) |
| quantiser-radix2 | -0.412 | moderate- |

**Consistency summary:**

- **quantiser-radix2**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 23.6ns | 6317.7ns | 0.4% |  |
| quantiser-radix2 | 20.0ns | 10007.1ns | 0.2% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 5146.1-8843.3 ns)
   5146.1 |####################################
   5331.0 |######
   5515.9 |
   5700.7 |
   5885.6 |
   6070.4 |############
   6255.3 |########################################
   6440.1 |######
   6625.0 |
   6809.9 |
   6994.7 |
   7179.6 |
   7364.4 |
   7549.3 |
   7734.2 |
   7919.0 |
   8103.9 |
   8288.7 |
   8473.6 |
   8658.4 |
  (4 below, 3 above range)

quantiser-radix2 (n=40, range 9426.5-11615.2 ns)
   9426.5 |########################################
   9535.9 |#
   9645.4 |#
   9754.8 |
   9864.2 |
   9973.7 |
  10083.1 |
  10192.5 |
  10302.0 |
  10411.4 |
  10520.9 |
  10630.3 |
  10739.7 |
  10849.2 |
  10958.6 |
  11068.0 |
  11177.5 |
  11286.9 |
  11396.3 |
  11505.8 |###########
  (4 below, 3 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=32.0% (high variance, measurements may be unstable)
