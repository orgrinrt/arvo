# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (5.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 3.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 66% faster than the next best (quantiser-radix10)

quantiser-radix2 (3.44 us) leads quantiser-radix10 (5.70 us) by 66%, a clear separation rather than a photo finish. CV 97.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 25% (significant)

quantiser-radix2 is -1.40 us (25%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 97.1%)

quantiser-radix2 wins on median (3.44 us) yet has the highest variance (CV 97.1%), while quantiser-radix10 is the steadiest (CV 50.1%, 5.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.26 us) is smaller than the fastest variant's own run-to-run std-dev (3.35 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### quantiser-radix2 is inconsistent: worst-20% is 3.3x its best-20%

quantiser-radix2's best 20% of batches run at 3.37 us but its worst 20% at 11.08 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 3444.9 ns median (-39.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.66x (fastest 3444.9 ns, slowest 5704.4 ns)

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
| quantiser-radix10 | 6418ns | 5771ns | 4817ns | 5459ns | 10899ns | base |
| quantiser-radix2 | 5212ns | 3500ns | 3423ns | 3766ns | 11341ns | -18.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6333ns | 4762ns | 10715ns | base | 0.040 |
| quantiser-radix2 | 5113ns | 3368ns | 11080ns | -19.27% | 0.050 |

## Performance model

- Peak throughput: **0.076 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.045 | 59.0% |
| quantiser-radix2 | 0.074 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 6418ns | 6418ns | base |
| quantiser-radix2 | 5212ns | 5212ns | -18.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 5704ns | base | --- | [4878, 5771] | --- | --- | --- | --- |
| quantiser-radix2 | 3445ns | -1399.2ns (-24.5%) | [-2319, -792]ns | [3401, 4149] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 4817ns | -13.6% |
| 1 | 2 | 4855ns | -30.2% |
| 1 | 3 | 5736ns | -40.8% |
| 1 | 4 | 5878ns | -41.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.286 | moderate+ |
| quantiser-radix2 | -0.104 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 26.6ns | 6333.4ns | 0.4% |  |
| quantiser-radix2 | 29.2ns | 5113.2ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 4762.3-10714.9 ns)
   4762.3 |###################################
   5059.9 |
   5357.5 |
   5655.2 |########################################
   5952.8 |
   6250.4 |
   6548.1 |
   6845.7 |
   7143.3 |
   7441.0 |
   7738.6 |
   8036.2 |
   8333.9 |
   8631.5 |
   8929.1 |
   9226.8 |
   9524.4 |
   9822.0 |
  10119.6 |
  10417.3 |##
  (3 below, 4 above range)

quantiser-radix2 (n=40, range 3368.4-11079.5 ns)
   3368.4 |########################################
   3754.0 |
   4139.5 |#################
   4525.1 |
   4910.6 |
   5296.2 |#
   5681.7 |
   6067.3 |#
   6452.9 |
   6838.4 |
   7224.0 |
   7609.5 |
   7995.1 |
   8380.6 |
   8766.2 |
   9151.8 |
   9537.3 |
   9922.9 |
  10308.4 |#
  10694.0 |
  (2 below, 5 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=45.1% (high variance, measurements may be unstable)
- **quantiser-radix2**: CV=65.4% (high variance, measurements may be unstable)
- **quantiser-radix2**: worst_20/best_20 = 3.3x (possible bimodal distribution)
