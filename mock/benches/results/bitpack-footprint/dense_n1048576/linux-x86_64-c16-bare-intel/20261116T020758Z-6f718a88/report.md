# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.1% (fastest 161.66 us to slowest 163.37 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.71 us) is smaller than the fastest variant's own run-to-run std-dev (2.12 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (161.66 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 161656.0 ns median
- Spread: 1.01x (fastest 161656.0 ns, slowest 163368.9 ns)

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
| bitpack-footprint-dense | 161749ns | 161808ns | 159593ns | 161745ns | 163920ns | base |
| bitpack-footprint-dense-alt | 162541ns | 163529ns | 159643ns | 163135ns | 163654ns | +0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 161589ns | 159437ns | 163748ns | base | 6.489 |
| bitpack-footprint-dense-alt | 162374ns | 159486ns | 163477ns | +0.49% | 6.458 |

## Performance model

- Peak throughput: **6.577 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.486 | 98.6% |
| bitpack-footprint-dense-alt | 6.418 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 161749ns | 161749ns | base |
| bitpack-footprint-dense-alt | 162541ns | 162541ns | +0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 161656ns | base | --- | [159459, 163678] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 163369ns | no significant difference | [-320, +3898]ns | [163348, 163387] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 159440ns | +2.5% |
| 1 | 2 | 163690ns | -0.2% |
| 1 | 3 | 163689ns | -2.6% |
| 1 | 4 | 159459ns | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.250 | moderate- |
| bitpack-footprint-dense-alt | -0.428 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 28.6ns | 161588.6ns | 0.0% |  |
| bitpack-footprint-dense-alt | 28.4ns | 162374.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159436.6-163748.1 ns)
  159436.6 |###################################
  159652.2 |
  159867.8 |
  160083.3 |
  160298.9 |
  160514.5 |
  160730.1 |
  160945.6 |
  161161.2 |
  161376.8 |
  161592.4 |
  161807.9 |
  162023.5 |
  162239.1 |
  162454.6 |
  162670.2 |
  162885.8 |
  163101.4 |
  163316.9 |
  163532.5 |########################################
  (4 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 159486.2-163477.0 ns)
  159486.2 |########
  159685.7 |
  159885.2 |
  160084.8 |
  160284.3 |
  160483.9 |
  160683.4 |
  160883.0 |
  161082.5 |#
  161282.1 |
  161481.6 |
  161681.1 |
  161880.7 |
  162080.2 |
  162279.8 |
  162479.3 |
  162678.9 |
  162878.4 |
  163077.9 |
  163277.5 |########################################
  (4 below, 2 above range)

```
