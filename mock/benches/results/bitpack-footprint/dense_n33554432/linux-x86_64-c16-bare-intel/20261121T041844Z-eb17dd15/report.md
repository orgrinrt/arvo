# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 5.91 ms to slowest 5.92 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense is fastest but the noisiest (CV 11.6%)

bitpack-footprint-dense wins on median (5.91 ms) yet has the highest variance (CV 11.6%), while bitpack-footprint-dense-alt is the steadiest (CV 7.3%, 5.92 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8.13 us) is smaller than the fastest variant's own run-to-run std-dev (687.23 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (5.91 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bitpack-footprint-dense vs stability leader bitpack-footprint-dense-alt (+0% speed for 1.6x steadier)

bitpack-footprint-dense is fastest (5.91 ms, CV 11.6%); bitpack-footprint-dense-alt gives up 0.1% median for 1.6x lower variance (CV 7.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 5908773.2 ns median
- Spread: 1.00x (fastest 5908773.2 ns, slowest 5916903.3 ns)

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
| Counter frequency | 2112804717 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 6264527ns | 5909576ns | 5894070ns | 5963204ns | 7538954ns | base |
| bitpack-footprint-dense-alt | 6162369ns | 5917839ns | 5895525ns | 6000082ns | 6916073ns | -1.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 6263622ns | 5893213ns | 7538097ns | base | 5.357 |
| bitpack-footprint-dense-alt | 6161455ns | 5894601ns | 6915177ns | -1.63% | 5.446 |

## Performance model

- Peak throughput: **5.694 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.679 | 99.7% |
| bitpack-footprint-dense-alt | 5.671 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 6264527ns | 6264527ns | base |
| bitpack-footprint-dense-alt | 6162369ns | 6162369ns | -1.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 5908773ns | base | --- | [5903991, 5980608] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 5916903ns | no significant difference | [-24198, +8807]ns | [5909639, 5927260] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 5902061ns | +0.1% |
| 1 | 2 | 5980608ns | -1.1% |
| 1 | 3 | 7202704ns | -4.0% |
| 1 | 4 | 5896282ns | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.405 | moderate- |
| bitpack-footprint-dense-alt | -0.421 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 163.1ns | 6263621.6ns | 0.0% |  |
| bitpack-footprint-dense-alt | 200.1ns | 6161454.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 5893212.6-7538097.0 ns)
  5893212.6 |########################################
  5975456.8 |############
  6057701.0 |
  6139945.2 |
  6222189.5 |
  6304433.7 |
  6386677.9 |
  6468922.1 |
  6551166.3 |##
  6633410.6 |
  6715654.8 |
  6797899.0 |
  6880143.2 |######
  6962387.4 |
  7044631.7 |
  7126875.9 |
  7209120.1 |
  7291364.3 |
  7373608.5 |
  7455852.8 |##
  (5 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 5894601.4-6915176.9 ns)
  5894601.4 |########################################
  5945630.2 |#
  5996659.0 |
  6047687.7 |
  6098716.5 |
  6149745.3 |
  6200774.1 |
  6251802.8 |
  6302831.6 |
  6353860.4 |
  6404889.2 |
  6455917.9 |
  6506946.7 |
  6557975.5 |
  6609004.3 |
  6660033.0 |
  6711061.8 |
  6762090.6 |
  6813119.4 |
  6864148.1 |###########
  (4 below, 3 above range)

```
