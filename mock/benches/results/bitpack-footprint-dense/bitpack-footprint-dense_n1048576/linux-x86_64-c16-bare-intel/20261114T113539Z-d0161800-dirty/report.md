# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (163.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 163.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 163.30 us to slowest 163.40 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense-alt is fastest but the noisiest (CV 14.0%)

bitpack-footprint-dense-alt wins on median (163.30 us) yet has the highest variance (CV 14.0%), while bitpack-footprint-dense is the steadiest (CV 1.2%, 163.40 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (98 ns) is smaller than the fastest variant's own run-to-run std-dev (22.88 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bitpack-footprint-dense-alt vs stability leader bitpack-footprint-dense (+0% speed for 11.7x steadier)

bitpack-footprint-dense-alt is fastest (163.30 us, CV 14.0%); bitpack-footprint-dense gives up 0.1% median for 11.7x lower variance (CV 1.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 163303.0 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 163303.0 ns, slowest 163401.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 162266ns | 163546ns | 159568ns | 162570ns | 164050ns | base |
| bitpack-footprint-dense-alt | 165944ns | 163447ns | 159584ns | 162757ns | 181866ns | +2.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 162076ns | 159421ns | 163781ns | base | 6.470 |
| bitpack-footprint-dense-alt | 165798ns | 159436ns | 181709ns | +2.30% | 6.324 |

## Performance model

- Peak throughput: **6.577 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.417 | 97.6% |
| bitpack-footprint-dense-alt | 6.421 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 162266ns | 162266ns | base |
| bitpack-footprint-dense-alt | 165944ns | 165944ns | +2.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 163401ns | base | --- | [160615, 163433] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 163303ns | no significant difference | [-142, +30]ns | [163275, 163369] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 159427ns | +0.0% |
| 1 | 2 | 163630ns | -0.2% |
| 1 | 3 | 163422ns | -0.1% |
| 1 | 4 | 162513ns | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.174 | ok |
| bitpack-footprint-dense-alt | -0.086 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 29.0ns | 162076.5ns | 0.0% |  |
| bitpack-footprint-dense-alt | 28.1ns | 165797.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 159420.5-163780.8 ns)
  159420.5 |####################################
  159638.5 |
  159856.5 |
  160074.5 |
  160292.6 |
  160510.6 |
  160728.6 |
  160946.6 |
  161164.6 |
  161382.6 |
  161600.6 |###
  161818.6 |
  162036.7 |
  162254.7 |
  162472.7 |
  162690.7 |
  162908.7 |
  163126.7 |
  163344.7 |########################################
  163562.7 |####################################
  (3 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 159435.6-181708.8 ns)
  159435.6 |##################
  160549.3 |
  161663.0 |
  162776.6 |########################################
  163890.3 |###
  165003.9 |
  166117.6 |
  167231.2 |
  168344.9 |
  169458.6 |
  170572.2 |
  171685.9 |
  172799.5 |
  173913.2 |
  175026.8 |
  176140.5 |
  177254.1 |
  178367.8 |
  179481.5 |
  180595.1 |
  (2 below, 1 above range)

```
