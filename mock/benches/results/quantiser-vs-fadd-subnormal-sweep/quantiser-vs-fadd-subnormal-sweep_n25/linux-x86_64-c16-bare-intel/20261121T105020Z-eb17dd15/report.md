# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 595% faster than the next best (quantiser-software)

quantiser-hardware (622 ns) leads quantiser-software (4.32 us) by 595%, a clear separation rather than a photo finish. CV 14.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (622 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.0x the fastest

Fastest quantiser-hardware (622 ns) to slowest quantiser-software (4.32 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.1x its best-20%

quantiser-software's best 20% of batches run at 3.66 us but its worst 20% at 11.27 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 621.8 ns median
- 1 variant significantly slower than baseline
- Spread: 6.95x (fastest 621.8 ns, slowest 4322.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Round-first software quantiser add vs native hardware fadd, subnormal fraction swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112817978 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 918ns | 876ns | 800ns | 892ns | 1116ns | base |
| quantiser-software | 5688ns | 4394ns | 3721ns | 4406ns | 11499ns | +519.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 647ns | 568ns | 775ns | base | 0.396 |
| quantiser-software | 5589ns | 3660ns | 11271ns | +763.84% | 0.046 |

## Performance model

- Peak throughput: **0.450 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.412 | 91.4% |
| quantiser-software | 0.059 | 13.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 918ns | 918ns | base |
| quantiser-software | 5688ns | 5688ns | +519.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 622ns | base | --- | [622, 622] | --- | --- | --- | --- |
| quantiser-software | 4323ns | +3692.8ns (+593.9%) | [+3684, +3708]ns | [4308, 4384] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 699ns | +528.3% |
| 1 | 2 | 622ns | +593.0% |
| 1 | 3 | 622ns | +596.2% |
| 1 | 4 | 609ns | +487.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.053 | ok |
| quantiser-software | -0.084 | ok |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 93.8ns | 647.0ns | 14.5% | HIGH |
| quantiser-software | 30.5ns | 5589.1ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 568.4-775.2 ns)
    568.4 |
    578.7 |
    589.0 |####
    599.4 |
    609.7 |
    620.1 |########################################
    630.4 |
    640.8 |##
    651.1 |
    661.4 |##
    671.8 |
    682.1 |##
    692.5 |############
    702.8 |
    713.1 |
    723.5 |
    733.8 |
    744.2 |
    754.5 |
    764.9 |
  (6 below, 3 above range)

quantiser-software (n=40, range 3659.7-11271.0 ns)
   3659.7 |
   4040.3 |########################################
   4420.8 |
   4801.4 |
   5182.0 |
   5562.5 |
   5943.1 |
   6323.6 |
   6704.2 |
   7084.8 |#
   7465.3 |
   7845.9 |
   8226.5 |
   8607.0 |
   8987.6 |#
   9368.1 |
   9748.7 |
  10129.3 |
  10509.8 |
  10890.4 |
  (7 below, 5 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=14.9% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=59.2% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.1x (possible bimodal distribution)
