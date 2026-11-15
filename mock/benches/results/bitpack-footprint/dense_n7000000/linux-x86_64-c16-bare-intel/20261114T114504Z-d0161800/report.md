# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (1.31 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 1.20 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-footprint-dense shows alternating (throttle bounce) (autocorr -0.73)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-footprint-dense-alt vs stability leader bitpack-footprint-dense (+9% speed for 1.0x steadier)

bitpack-footprint-dense-alt is fastest (1.20 ms, CV 4.2%); bitpack-footprint-dense gives up 8.6% median for 1.0x lower variance (CV 4.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 1202210.9 ns median (-7.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.09x (fastest 1202210.9 ns, slowest 1305485.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1263602ns | 1305861ns | 1183809ns | 1273472ns | 1313785ns | base |
| bitpack-footprint-dense-alt | 1225089ns | 1202862ns | 1181435ns | 1211751ns | 1308758ns | -3.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1263107ns | 1183262ns | 1313295ns | base | 5.542 |
| bitpack-footprint-dense-alt | 1224578ns | 1181002ns | 1308324ns | -3.05% | 5.716 |

## Performance model

- Peak throughput: **5.927 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.362 | 90.5% |
| bitpack-footprint-dense-alt | 5.823 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1263602ns | 1263602ns | base |
| bitpack-footprint-dense-alt | 1225089ns | 1225089ns | -3.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1305485ns | base | --- | [1225015, 1308882] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1202211ns | -4818.5ns (-0.4%) | [-36506, -2899]ns | [1185069, 1223201] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 1215191ns | -2.5% |
| 1 | 2 | 1305633ns | -9.5% |
| 1 | 3 | 1216822ns | -0.0% |
| 1 | 4 | 1312182ns | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.726 | HIGH- (thermal bounce) |
| bitpack-footprint-dense-alt | 0.125 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 88.7ns | 1263107.2ns | 0.0% |  |
| bitpack-footprint-dense-alt | 69.0ns | 1224577.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1183261.9-1313294.9 ns)
  1183261.9 |######
  1189763.5 |######
  1196265.2 |
  1202766.8 |######
  1209268.5 |######
  1215770.1 |######
  1222271.8 |###
  1228773.4 |###
  1235275.1 |
  1241776.7 |###
  1248278.4 |###
  1254780.0 |
  1261281.7 |
  1267783.3 |
  1274285.0 |
  1280786.6 |
  1287288.3 |
  1293789.9 |
  1300291.6 |#####################
  1306793.2 |########################################
  (4 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 1181002.4-1308324.5 ns)
  1181002.4 |########################################
  1187368.5 |
  1193734.6 |##
  1200100.7 |
  1206466.8 |#####
  1212832.9 |##########
  1219199.0 |
  1225565.1 |#######
  1231931.2 |
  1238297.3 |
  1244663.4 |##
  1251029.5 |
  1257395.7 |
  1263761.8 |
  1270127.9 |
  1276494.0 |
  1282860.1 |
  1289226.2 |
  1295592.3 |
  1301958.4 |#################
  (3 below, 3 above range)

```
