# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-radix10 dominates: 76% faster than the next best (quantiser-radix2)

quantiser-radix10 (5.59 us) leads quantiser-radix2 (9.84 us) by 76%, a clear separation rather than a photo finish. CV 41.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix10 is fastest but the noisiest (CV 41.8%)

quantiser-radix10 wins on median (5.59 us) yet has the highest variance (CV 41.8%), while quantiser-radix2 is the steadiest (CV 8.5%, 9.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (quantiser-radix10)

The baseline quantiser-radix10 is the fastest (5.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### quantiser-radix10 is inconsistent: worst-20% is 1.7x its best-20%

quantiser-radix10's best 20% of batches run at 5.52 us but its worst 20% at 9.49 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-radix10) is the fastest** at 5585.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.76x (fastest 5585.4 ns, slowest 9842.8 ns)

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
| Counter frequency | 2112854571 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 6434ns | 5641ns | 5582ns | 5643ns | 9660ns | base |
| quantiser-radix2 | 10369ns | 9910ns | 9798ns | 10071ns | 11830ns | +61.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6356ns | 5524ns | 9492ns | base | 0.040 |
| quantiser-radix2 | 10301ns | 9741ns | 11763ns | +62.08% | 0.025 |

## Performance model

- Peak throughput: **0.046 Gops/s** (quantiser-radix10; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.046 | 98.9% |
| quantiser-radix2 | 0.026 | 56.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 6434ns | 6434ns | base |
| quantiser-radix2 | 10369ns | 10369ns | +61.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 5585ns | base | --- | [5545, 5630] | --- | --- | --- | --- |
| quantiser-radix2 | 9843ns | +4271.2ns (+76.5%) | [+4184, +4318]ns | [9815, 9901] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 5544ns | +78.0% |
| 1 | 2 | 5535ns | +76.6% |
| 1 | 3 | 5647ns | +73.7% |
| 1 | 4 | 5630ns | +108.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.166 | ok |
| quantiser-radix2 | -0.082 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 23.4ns | 6355.7ns | 0.4% |  |
| quantiser-radix2 | 21.6ns | 10301.1ns | 0.2% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 5524.0-9492.4 ns)
   5524.0 |########################################
   5722.4 |####
   5920.8 |
   6119.2 |
   6317.7 |
   6516.1 |
   6714.5 |
   6912.9 |
   7111.3 |
   7309.8 |
   7508.2 |
   7706.6 |
   7905.0 |
   8103.4 |
   8301.8 |
   8500.3 |
   8698.7 |
   8897.1 |
   9095.5 |
   9293.9 |
  (4 below, 4 above range)

quantiser-radix2 (n=40, range 9740.7-11762.5 ns)
   9740.7 |########################################
   9841.8 |####################
   9942.9 |#####
  10044.0 |
  10145.0 |
  10246.1 |
  10347.2 |
  10448.3 |
  10549.4 |
  10650.5 |
  10751.6 |
  10852.7 |
  10953.8 |
  11054.9 |
  11156.0 |
  11257.1 |
  11358.2 |
  11459.3 |
  11560.4 |#####
  11661.4 |#######
  (4 below, 5 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=36.7% (high variance, measurements may be unstable)
