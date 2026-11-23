# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (163.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 162.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.9% (fastest 162.11 us to slowest 163.62 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense-alt shows alternating (throttle bounce) (autocorr -0.74)

bitpack-footprint-dense-alt's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.51 us) is smaller than the fastest variant's own run-to-run std-dev (2.14 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 162110.0 ns median (-0.9% vs baseline)
- Spread: 1.01x (fastest 162110.0 ns, slowest 163624.5 ns)

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
| bitpack-footprint-dense | 170682ns | 163830ns | 159660ns | 166087ns | 195488ns | base |
| bitpack-footprint-dense-alt | 161905ns | 162347ns | 159589ns | 161893ns | 164254ns | -5.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 170492ns | 159504ns | 195311ns | base | 6.150 |
| bitpack-footprint-dense-alt | 161679ns | 159425ns | 163992ns | -5.17% | 6.486 |

## Performance model

- Peak throughput: **6.577 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.408 | 97.4% |
| bitpack-footprint-dense-alt | 6.468 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 170682ns | 170682ns | base |
| bitpack-footprint-dense-alt | 161905ns | 161905ns | -5.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 163625ns | base | --- | [163558, 163717] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 162110ns | no significant difference | [-163, +21]ns | [159514, 163640] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 163564ns | +0.2% |
| 1 | 2 | 194982ns | -18.2% |
| 1 | 3 | 163679ns | -0.0% |
| 1 | 4 | 159531ns | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.320 | moderate- |
| bitpack-footprint-dense-alt | -0.742 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 27.3ns | 170491.5ns | 0.0% |  |
| bitpack-footprint-dense-alt | 27.1ns | 161678.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159504.3-195311.1 ns)
  159504.3 |###############
  161294.7 |
  163085.0 |########################################
  164875.4 |##
  166665.7 |
  168456.0 |
  170246.4 |
  172036.7 |
  173827.1 |
  175617.4 |
  177407.7 |
  179198.1 |
  180988.4 |
  182778.7 |
  184569.1 |
  186359.4 |
  188149.8 |
  189940.1 |
  191730.4 |
  193520.8 |#################
  (4 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 159424.9-163991.9 ns)
  159424.9 |########################################
  159653.2 |
  159881.6 |##
  160109.9 |
  160338.3 |
  160566.6 |##
  160795.0 |
  161023.3 |
  161251.7 |
  161480.0 |
  161708.4 |
  161936.7 |
  162165.1 |
  162393.5 |
  162621.8 |
  162850.2 |
  163078.5 |
  163306.9 |
  163535.2 |########################
  163763.6 |##########################
  (3 below, 1 above range)

```
