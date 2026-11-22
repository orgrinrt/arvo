# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (5.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 4.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 11% faster than the next best (quantiser-radix10)

quantiser-radix2 (4.76 us) leads quantiser-radix10 (5.29 us) by 11%, a clear separation rather than a photo finish. CV 68.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 23% (significant)

quantiser-radix2 is -1.19 us (23%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 68.8%)

quantiser-radix2 wins on median (4.76 us) yet has the highest variance (CV 68.8%), while quantiser-radix10 is the steadiest (CV 48.1%, 5.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (536 ns) is smaller than the fastest variant's own run-to-run std-dev (3.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### quantiser-radix2 is inconsistent: worst-20% is 2.7x its best-20%

quantiser-radix2's best 20% of batches run at 3.94 us but its worst 20% at 10.80 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 4756.9 ns median (-10.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.11x (fastest 4756.9 ns, slowest 5293.1 ns)

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
| quantiser-radix10 | 6367ns | 5348ns | 5227ns | 5598ns | 9816ns | base |
| quantiser-radix2 | 5764ns | 4825ns | 3991ns | 4598ns | 11034ns | -9.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 6286ns | 5173ns | 9638ns | base | 0.041 |
| quantiser-radix2 | 5667ns | 3935ns | 10798ns | -9.85% | 0.045 |

## Performance model

- Peak throughput: **0.065 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.048 | 74.3% |
| quantiser-radix2 | 0.054 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 6367ns | 6367ns | base |
| quantiser-radix2 | 5764ns | 5764ns | -9.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 5293ns | base | --- | [5226, 5839] | --- | --- | --- | --- |
| quantiser-radix2 | 4757ns | -1192.7ns (-22.5%) | [-1278, -732]ns | [4041, 4796] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|---|
| 1 | 1 | 6347ns | -24.8% |
| 1 | 2 | 5187ns | -22.8% |
| 1 | 3 | 5282ns | -25.2% |
| 1 | 4 | 5281ns | -8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.151 | ok |
| quantiser-radix2 | -0.252 | moderate- |

**Consistency summary:**

- **quantiser-radix2**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 23.3ns | 6286.1ns | 0.4% |  |
| quantiser-radix2 | 28.3ns | 5666.9ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 5173.0-9638.5 ns)
   5173.0 |########################################
   5396.3 |####
   5619.6 |
   5842.8 |
   6066.1 |##
   6289.4 |####################
   6512.6 |
   6735.9 |
   6959.2 |
   7182.5 |
   7405.7 |
   7629.0 |
   7852.3 |
   8075.5 |
   8298.8 |
   8522.1 |
   8745.4 |
   8968.6 |
   9191.9 |
   9415.2 |
  (4 below, 3 above range)

quantiser-radix2 (n=40, range 3935.3-10798.0 ns)
   3935.3 |################################
   4278.4 |
   4621.5 |########################################
   4964.7 |
   5307.8 |#####
   5651.0 |
   5994.1 |
   6337.2 |##
   6680.4 |
   7023.5 |
   7366.6 |
   7709.8 |
   8052.9 |##
   8396.1 |
   8739.2 |
   9082.3 |
   9425.5 |
   9768.6 |
  10111.8 |
  10454.9 |
  (3 below, 4 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=40.5% (high variance, measurements may be unstable)
- **quantiser-radix2**: CV=57.8% (high variance, measurements may be unstable)
