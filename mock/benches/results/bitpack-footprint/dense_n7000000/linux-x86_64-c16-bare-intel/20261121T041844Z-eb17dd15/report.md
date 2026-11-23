# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-dense is fastest but the noisiest (CV 10.4%)

bitpack-footprint-dense wins on median (1.19 ms) yet has the highest variance (CV 10.4%), while bitpack-footprint-dense-alt is the steadiest (CV 4.5%, 1.24 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-footprint-dense-alt shows alternating (throttle bounce) (autocorr -0.74)

bitpack-footprint-dense-alt's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (50.11 us) is smaller than the fastest variant's own run-to-run std-dev (123.06 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (1.19 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bitpack-footprint-dense vs stability leader bitpack-footprint-dense-alt (+4% speed for 2.3x steadier)

bitpack-footprint-dense is fastest (1.19 ms, CV 10.4%); bitpack-footprint-dense-alt gives up 4.2% median for 2.3x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 1185588.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1185588.5 ns, slowest 1235701.2 ns)

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
| bitpack-footprint-dense | 1233917ns | 1186021ns | 1175093ns | 1186341ns | 1435465ns | base |
| bitpack-footprint-dense-alt | 1253018ns | 1236187ns | 1191007ns | 1253454ns | 1313720ns | +1.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1233443ns | 1174599ns | 1434991ns | base | 5.675 |
| bitpack-footprint-dense-alt | 1252522ns | 1190472ns | 1313244ns | +1.55% | 5.589 |

## Performance model

- Peak throughput: **5.959 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.904 | 99.1% |
| bitpack-footprint-dense-alt | 5.665 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1233917ns | 1233917ns | base |
| bitpack-footprint-dense-alt | 1253018ns | 1253018ns | +1.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1185588ns | base | --- | [1184589, 1187006] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1235701ns | +43236.7ns (+3.6%) | [+13028, +120604]ns | [1199233, 1307050] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 1391478ns | -14.3% |
| 1 | 2 | 1187313ns | +10.1% |
| 1 | 3 | 1184608ns | +0.9% |
| 1 | 4 | 1185219ns | +10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.074 | ok |
| bitpack-footprint-dense-alt | -0.740 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 130.4ns | 1233442.7ns | 0.0% |  |
| bitpack-footprint-dense-alt | 138.2ns | 1252522.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1174598.5-1434991.5 ns)
  1174598.5 |########################################
  1187618.2 |#########
  1200637.8 |
  1213657.5 |
  1226677.1 |#
  1239696.8 |
  1252716.4 |
  1265736.1 |#
  1278755.7 |
  1291775.3 |
  1304795.0 |
  1317814.6 |
  1330834.3 |
  1343853.9 |
  1356873.6 |
  1369893.2 |
  1382912.9 |
  1395932.5 |
  1408952.2 |
  1421971.8 |
  (2 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 1190472.0-1313243.7 ns)
  1190472.0 |####################################
  1196610.6 |##########
  1202749.2 |
  1208887.7 |###
  1215026.3 |###
  1221164.9 |#######
  1227303.5 |
  1233442.1 |
  1239580.7 |###
  1245719.3 |
  1251857.9 |
  1257996.4 |
  1264135.0 |
  1270273.6 |
  1276412.2 |
  1282550.8 |
  1288689.4 |
  1294828.0 |
  1300966.5 |##################
  1307105.1 |########################################
  (3 below, 3 above range)

```
