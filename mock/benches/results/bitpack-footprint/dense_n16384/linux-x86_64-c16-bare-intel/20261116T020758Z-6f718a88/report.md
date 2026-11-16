# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (3.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 3.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 3.05 us to slowest 3.05 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense-alt is fastest but the noisiest (CV 86.9%)

bitpack-footprint-dense-alt wins on median (3.05 us) yet has the highest variance (CV 86.9%), while bitpack-footprint-dense is the steadiest (CV 75.4%, 3.05 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2.65 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bitpack-footprint-dense-alt vs stability leader bitpack-footprint-dense (+0% speed for 1.2x steadier)

bitpack-footprint-dense-alt is fastest (3.05 us, CV 86.9%); bitpack-footprint-dense gives up 0.0% median for 1.2x lower variance (CV 75.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-footprint-dense-alt is inconsistent: worst-20% is 3.8x its best-20%

bitpack-footprint-dense-alt's best 20% of batches run at 2.50 us but its worst 20% at 9.43 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 3050.6 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 3050.6 ns, slowest 3050.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Dense footprint: sequential sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112547631 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3922ns | 3116ns | 2550ns | 2956ns | 8192ns | base |
| bitpack-footprint-dense-alt | 4367ns | 3117ns | 2552ns | 3198ns | 9688ns | +11.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 3829ns | 2496ns | 7966ns | base | 4.279 |
| bitpack-footprint-dense-alt | 4264ns | 2498ns | 9430ns | +11.38% | 3.842 |

## Performance model

- Peak throughput: **6.565 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.371 | 81.8% |
| bitpack-footprint-dense-alt | 5.371 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 3922ns | 3922ns | base |
| bitpack-footprint-dense-alt | 4367ns | 4367ns | +11.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3051ns | base | --- | [2508, 3051] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 3051ns | no significant difference | [-0, +554]ns | [3050, 3063] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 2501ns | +22.6% |
| 1 | 2 | 3051ns | -18.2% |
| 1 | 3 | 3051ns | -0.0% |
| 1 | 4 | 2496ns | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.250 | moderate- |
| bitpack-footprint-dense-alt | -0.431 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 29.8ns | 3828.6ns | 0.8% |  |
| bitpack-footprint-dense-alt | 33.2ns | 4264.3ns | 0.8% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2495.8-7966.1 ns)
   2495.8 |######################
   2769.4 |
   3042.9 |########################################
   3316.4 |
   3589.9 |
   3863.4 |
   4136.9 |
   4410.4 |
   4683.9 |
   4957.5 |
   5231.0 |
   5504.5 |
   5778.0 |
   6051.5 |
   6325.0 |##
   6598.5 |
   6872.0 |
   7145.6 |
   7419.1 |##
   7692.6 |
  (5 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 2498.4-9430.3 ns)
   2498.4 |###
   2845.0 |########################################
   3191.6 |
   3538.2 |
   3884.8 |
   4231.4 |
   4577.9 |#
   4924.5 |
   5271.1 |
   5617.7 |
   5964.3 |
   6310.9 |
   6657.5 |
   7004.1 |#
   7350.7 |
   7697.3 |
   8043.9 |
   8390.5 |
   8737.1 |###
   9083.7 |#
  (6 below, 4 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=60.0% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=62.2% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.8x (possible bimodal distribution)
