# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (11.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 11.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 11.08 us to slowest 11.09 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense shows alternating (throttle bounce) (autocorr -0.75)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (1.11 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 11082.8 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 11082.8 ns, slowest 11087.8 ns)

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
| bitpack-footprint-dense | 11166ns | 11190ns | 10042ns | 11164ns | 12298ns | base |
| bitpack-footprint-dense-alt | 11163ns | 11176ns | 10042ns | 11160ns | 12293ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 11087ns | 9975ns | 12205ns | base | 5.911 |
| bitpack-footprint-dense-alt | 11083ns | 9975ns | 12198ns | -0.03% | 5.913 |

## Performance model

- Peak throughput: **6.570 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.911 | 90.0% |
| bitpack-footprint-dense-alt | 5.913 | 90.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 11166ns | 11166ns | base |
| bitpack-footprint-dense-alt | 11163ns | 11163ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11088ns | base | --- | [9980, 12185] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 11083ns | no significant difference | [-10, +2]ns | [9977, 12184] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 9977ns | -0.0% |
| 1 | 2 | 12185ns | -18.1% |
| 1 | 3 | 9977ns | +22.2% |
| 1 | 4 | 12194ns | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.749 | HIGH- (thermal bounce) |
| bitpack-footprint-dense-alt | 0.249 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 21.4ns | 11086.6ns | 0.2% |  |
| bitpack-footprint-dense-alt | 21.4ns | 11083.4ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 9974.6-12205.2 ns)
   9974.6 |########################################
  10086.2 |
  10197.7 |
  10309.2 |
  10420.7 |
  10532.3 |
  10643.8 |
  10755.3 |
  10866.9 |
  10978.4 |
  11089.9 |
  11201.4 |
  11313.0 |
  11424.5 |
  11536.0 |
  11647.5 |
  11759.1 |
  11870.6 |
  11982.1 |
  12093.7 |#####################################
  (1 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 9975.4-12198.4 ns)
   9975.4 |###################################
  10086.6 |
  10197.7 |
  10308.9 |
  10420.0 |
  10531.2 |
  10642.3 |
  10753.5 |
  10864.6 |
  10975.8 |
  11086.9 |
  11198.1 |
  11309.2 |
  11420.4 |
  11531.5 |
  11642.7 |
  11753.8 |
  11865.0 |
  11976.1 |
  12087.3 |########################################
  (4 below, 2 above range)

```
