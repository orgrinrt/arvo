# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-radix10 dominates: 119% faster than the next best (quantiser-radix2)

quantiser-radix10 (4.24 us) leads quantiser-radix2 (9.28 us) by 119%, a clear separation rather than a photo finish. CV 70.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix10 is fastest but the noisiest (CV 70.3%)

quantiser-radix10 wins on median (4.24 us) yet has the highest variance (CV 70.3%), while quantiser-radix2 is the steadiest (CV 9.7%, 9.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-radix10)

The baseline quantiser-radix10 is the fastest (4.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### quantiser-radix10 is inconsistent: worst-20% is 2.7x its best-20%

quantiser-radix10's best 20% of batches run at 4.09 us but its worst 20% at 10.89 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-radix10) is the fastest** at 4237.1 ns median
- 1 variant significantly slower than baseline
- Spread: 2.19x (fastest 4237.1 ns, slowest 9279.8 ns)

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
| quantiser-radix10 | 5799ns | 4291ns | 4142ns | 4588ns | 11092ns | base |
| quantiser-radix2 | 9849ns | 9341ns | 9269ns | 9514ns | 11434ns | +69.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 5714ns | 4087ns | 10895ns | base | 0.045 |
| quantiser-radix2 | 9784ns | 9203ns | 11356ns | +71.23% | 0.026 |

## Performance model

- Peak throughput: **0.063 Gops/s** (quantiser-radix10; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.060 | 96.5% |
| quantiser-radix2 | 0.028 | 44.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 5799ns | 5799ns | base |
| quantiser-radix2 | 9849ns | 9849ns | +69.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 4237ns | base | --- | [4141, 5207] | --- | --- | --- | --- |
| quantiser-radix2 | 9280ns | +5044.5ns (+119.1%) | [+4105, +5090]ns | [9268, 9326] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 4240ns | +118.8% |
| 1 | 2 | 4129ns | +123.6% |
| 1 | 3 | 4091ns | +177.2% |
| 1 | 4 | 5214ns | +78.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.133 | ok |
| quantiser-radix2 | -0.420 | moderate- |

**Consistency summary:**

- **quantiser-radix2**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 25.9ns | 5713.8ns | 0.5% |  |
| quantiser-radix2 | 20.0ns | 9783.5ns | 0.2% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 4087.0-10894.7 ns)
   4087.0 |########################################
   4427.4 |
   4767.7 |
   5108.1 |##################
   5448.5 |
   5788.9 |
   6129.3 |
   6469.7 |
   6810.1 |
   7150.5 |
   7490.8 |
   7831.2 |##
   8171.6 |
   8512.0 |##
   8852.4 |
   9192.8 |
   9533.2 |
   9873.6 |
  10213.9 |##
  10554.3 |
  (5 below, 4 above range)

quantiser-radix2 (n=40, range 9203.3-11356.3 ns)
   9203.3 |########################################
   9311.0 |#########
   9418.6 |
   9526.3 |
   9633.9 |
   9741.6 |
   9849.2 |
   9956.9 |
  10064.5 |
  10172.2 |
  10279.8 |
  10387.5 |
  10495.1 |
  10602.8 |
  10710.4 |
  10818.1 |
  10925.7 |
  11033.4 |
  11141.0 |
  11248.7 |##########
  (3 below, 4 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=52.1% (high variance, measurements may be unstable)
