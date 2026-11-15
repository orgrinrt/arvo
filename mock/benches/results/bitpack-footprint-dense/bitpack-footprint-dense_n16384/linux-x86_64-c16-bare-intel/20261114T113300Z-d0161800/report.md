# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (3.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-footprint-dense-alt dominates: 22% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-dense-alt (2.51 us) leads bitpack-footprint-dense (3.05 us) by 22%, a clear separation rather than a photo finish. CV 115.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-dense-alt is fastest but the noisiest (CV 115.1%)

bitpack-footprint-dense-alt wins on median (2.51 us) yet has the highest variance (CV 115.1%), while bitpack-footprint-dense is the steadiest (CV 88.5%, 3.05 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (544 ns) is smaller than the fastest variant's own run-to-run std-dev (2.89 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-footprint-dense-alt is inconsistent: worst-20% is 3.7x its best-20%

bitpack-footprint-dense-alt's best 20% of batches run at 2.50 us but its worst 20% at 9.34 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 2506.4 ns median (-17.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 2506.4 ns, slowest 3050.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 4551ns | 3117ns | 2692ns | 3485ns | 9607ns | base |
| bitpack-footprint-dense-alt | 4665ns | 2560ns | 2549ns | 3727ns | 9596ns | +2.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 4443ns | 2636ns | 9344ns | base | 3.688 |
| bitpack-footprint-dense-alt | 4551ns | 2496ns | 9336ns | +2.44% | 3.600 |

## Performance model

- Peak throughput: **6.565 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.370 | 81.8% |
| bitpack-footprint-dense-alt | 6.537 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 4551ns | 4551ns | base |
| bitpack-footprint-dense-alt | 4665ns | 4665ns | +2.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3051ns | base | --- | [3050, 3062] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 2506ns | -543.7ns (-17.8%) | [-555, -0]ns | [2496, 4015] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 3051ns | -18.0% |
| 1 | 2 | 3056ns | -18.2% |
| 1 | 3 | 2501ns | -0.2% |
| 1 | 4 | 3056ns | +154.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.424 | moderate- |
| bitpack-footprint-dense-alt | -0.084 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 34.5ns | 4442.6ns | 0.8% |  |
| bitpack-footprint-dense-alt | 36.3ns | 4551.2ns | 0.8% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2635.7-9343.5 ns)
   2635.7 |
   2971.1 |########################################
   3306.5 |
   3641.9 |
   3977.3 |
   4312.7 |
   4648.1 |
   4983.5 |
   5318.9 |
   5654.2 |
   5989.6 |
   6325.0 |
   6660.4 |
   6995.8 |#
   7331.2 |#
   7666.6 |#
   8002.0 |###
   8337.4 |#
   8672.8 |
   9008.1 |
  (6 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 2495.7-9336.0 ns)
   2495.7 |########################################
   2837.7 |#######
   3179.7 |
   3521.8 |
   3863.8 |
   4205.8 |
   4547.8 |
   4889.8 |##
   5231.8 |
   5573.9 |
   5915.9 |
   6257.9 |##
   6599.9 |##
   6941.9 |
   7283.9 |####
   7626.0 |#######
   7968.0 |
   8310.0 |##
   8652.0 |
   8994.0 |####
  (6 below, 3 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=60.8% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=63.4% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.7x (possible bimodal distribution)
