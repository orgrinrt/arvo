# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-radix10 dominates: 64% faster than the next best (quantiser-radix2)

quantiser-radix10 (6.85 us) leads quantiser-radix2 (11.22 us) by 64%, a clear separation rather than a photo finish. CV 25.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix10 is fastest but the noisiest (CV 25.0%)

quantiser-radix10 wins on median (6.85 us) yet has the highest variance (CV 25.0%), while quantiser-radix2 is the steadiest (CV 9.4%, 11.22 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-radix10)

The baseline quantiser-radix10 is the fastest (6.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### quantiser-radix10 is inconsistent: worst-20% is 1.6x its best-20%

quantiser-radix10's best 20% of batches run at 5.62 us but its worst 20% at 8.77 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-radix10) is the fastest** at 6850.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 6850.6 ns, slowest 11220.0 ns)

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
| quantiser-radix10 | 7038ns | 6918ns | 5682ns | 6872ns | 8891ns | base |
| quantiser-radix2 | 11030ns | 11281ns | 9871ns | 11039ns | 12161ns | +56.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6960ns | 5624ns | 8766ns | base | 0.037 |
| quantiser-radix2 | 10963ns | 9805ns | 12087ns | +57.50% | 0.023 |

## Performance model

- Peak throughput: **0.046 Gops/s** (quantiser-radix10; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.037 | 82.1% |
| quantiser-radix2 | 0.023 | 50.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 7038ns | 7038ns | base |
| quantiser-radix2 | 11030ns | 11030ns | +56.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 6851ns | base | --- | [6839, 6858] | --- | --- | --- | --- |
| quantiser-radix2 | 11220ns | +3469.3ns (+50.6%) | [+3021, +5103]ns | [9904, 11960] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 6842ns | +44.2% |
| 1 | 2 | 5609ns | +114.7% |
| 1 | 3 | 6852ns | +74.6% |
| 1 | 4 | 6872ns | +43.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.400 | moderate- |
| quantiser-radix2 | -0.250 | moderate- |

**Consistency summary:**

- **quantiser-radix2**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 23.5ns | 6960.4ns | 0.3% |  |
| quantiser-radix2 | 22.0ns | 10962.6ns | 0.2% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 5624.2-8765.9 ns)
   5624.2 |####
   5781.3 |
   5938.3 |
   6095.4 |
   6252.5 |
   6409.6 |
   6566.7 |
   6723.8 |########################################
   6880.9 |####
   7038.0 |
   7195.0 |
   7352.1 |
   7509.2 |
   7666.3 |
   7823.4 |
   7980.5 |
   8137.6 |
   8294.7 |
   8451.7 |#
   8608.8 |
  (6 below, 2 above range)

quantiser-radix2 (n=40, range 9805.1-12087.0 ns)
   9805.1 |########################################
   9919.2 |##############
  10033.3 |
  10147.4 |
  10261.5 |###
  10375.6 |
  10489.7 |###
  10603.8 |
  10717.9 |
  10832.0 |
  10946.1 |
  11060.2 |
  11174.3 |
  11288.4 |
  11402.5 |
  11516.5 |
  11630.6 |
  11744.7 |
  11858.8 |#############################
  11972.9 |################################
  (3 below, 3 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=24.6% (high variance, measurements may be unstable)
