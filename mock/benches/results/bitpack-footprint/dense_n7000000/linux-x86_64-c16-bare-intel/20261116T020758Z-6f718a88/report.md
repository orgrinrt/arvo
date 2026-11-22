# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 1.18 ms to slowest 1.19 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bitpack-footprint-dense is fastest but the noisiest (CV 5.1%)

bitpack-footprint-dense wins on median (1.18 ms) yet has the highest variance (CV 5.1%), while bitpack-footprint-dense-alt is the steadiest (CV 4.6%, 1.19 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.24 us) is smaller than the fastest variant's own run-to-run std-dev (60.80 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (1.18 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bitpack-footprint-dense vs stability leader bitpack-footprint-dense-alt (+0% speed for 1.1x steadier)

bitpack-footprint-dense is fastest (1.18 ms, CV 5.1%); bitpack-footprint-dense-alt gives up 0.4% median for 1.1x lower variance (CV 4.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 1183599.9 ns median
- Spread: 1.00x (fastest 1183599.9 ns, slowest 1187839.5 ns)

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
| bitpack-footprint-dense | 1228824ns | 1184114ns | 1180951ns | 1216695ns | 1313085ns | base |
| bitpack-footprint-dense-alt | 1217641ns | 1188436ns | 1181384ns | 1198419ns | 1311562ns | -0.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1228295ns | 1180414ns | 1312550ns | base | 5.699 |
| bitpack-footprint-dense-alt | 1217141ns | 1180884ns | 1311184ns | -0.91% | 5.751 |

## Performance model

- Peak throughput: **5.930 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.914 | 99.7% |
| bitpack-footprint-dense-alt | 5.893 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1228824ns | 1228824ns | base |
| bitpack-footprint-dense-alt | 1217641ns | 1217641ns | -0.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1183600ns | base | --- | [1182794, 1261684] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1187840ns | no significant difference | [-671, +2268]ns | [1183726, 1189098] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 1183600ns | -0.2% |
| 1 | 2 | 1198558ns | -0.8% |
| 1 | 3 | 1308587ns | +0.1% |
| 1 | 4 | 1180895ns | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.399 | moderate- |
| bitpack-footprint-dense-alt | -0.398 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 104.1ns | 1228294.9ns | 0.0% |  |
| bitpack-footprint-dense-alt | 96.8ns | 1217141.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1180413.9-1312549.6 ns)
  1180413.9 |########################################
  1187020.7 |
  1193627.5 |##
  1200234.3 |
  1206841.1 |
  1213447.9 |##
  1220054.7 |
  1226661.4 |
  1233268.2 |
  1239875.0 |
  1246481.8 |
  1253088.6 |
  1259695.4 |
  1266302.1 |
  1272908.9 |
  1279515.7 |
  1286122.5 |
  1292729.3 |
  1299336.1 |
  1305942.9 |#####################
  (5 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 1180883.6-1311184.4 ns)
  1180883.6 |########################################
  1187398.7 |#########################
  1193913.7 |##
  1200428.8 |#####
  1206943.8 |
  1213458.8 |
  1219973.9 |
  1226488.9 |
  1233003.9 |
  1239519.0 |
  1246034.0 |
  1252549.1 |
  1259064.1 |
  1265579.1 |
  1272094.2 |
  1278609.2 |
  1285124.2 |
  1291639.3 |
  1298154.3 |
  1304669.4 |######################
  (4 below, 2 above range)

```
