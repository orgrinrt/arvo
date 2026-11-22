# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 15% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (1.04 ms) leads bitpack-footprint-dense-alt (1.19 ms) by 15%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 4.4x slower than the field

bitpack-footprint-packed-naive (4.58 ms) is 4.4x the fastest (1.04 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 5.3%)

bitpack-footprint-packed wins on median (1.04 ms) yet has the highest variance (CV 5.3%), while bitpack-footprint-packed-naive is the steadiest (CV 1.0%, 4.58 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (285% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 285% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-footprint-packed (1.04 ms) to slowest bitpack-footprint-packed-naive (4.58 ms): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 1038554.1 ns median (-12.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.41x (fastest 1038554.1 ns, slowest 4579041.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112837581 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1234434ns | 1190401ns | 1183262ns | 1224979ns | 1313973ns | base |
| bitpack-footprint-dense-alt | 1197109ns | 1189866ns | 1185172ns | 1191443ns | 1226047ns | -3.02% |
| bitpack-footprint-packed | 1020896ns | 1038888ns | 956227ns | 1021854ns | 1082694ns | -17.30% |
| bitpack-footprint-packed-naive | 4590028ns | 4579626ns | 4569142ns | 4579499ns | 4642502ns | +271.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1233920ns | 1182822ns | 1313434ns | base | 5.673 |
| bitpack-footprint-dense-alt | 1196579ns | 1184702ns | 1225417ns | -3.03% | 5.850 |
| bitpack-footprint-packed | 1020435ns | 955816ns | 1082012ns | -17.30% | 6.860 |
| bitpack-footprint-packed-naive | 4589482ns | 4568679ns | 4641863ns | +271.94% | 1.525 |

## Performance model

- Peak throughput: **7.324 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.883 | 80.3% |
| bitpack-footprint-dense-alt | 5.885 | 80.4% |
| bitpack-footprint-packed | 6.740 | 92.0% |
| bitpack-footprint-packed-naive | 1.529 | 20.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1234434ns | 1234434ns | base |
| bitpack-footprint-dense-alt | 1197109ns | 1197109ns | -3.02% |
| bitpack-footprint-packed | 1020896ns | 1020896ns | -17.30% |
| bitpack-footprint-packed-naive | 4590028ns | 4590028ns | +271.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1189855ns | base | --- | [1188139, 1297567] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1189372ns | no significant difference | [-79413, +2370]ns | [1186779, 1192031] | no | 0.6358 | 0.6358 | 0 |
| bitpack-footprint-packed | 1038554ns | -216003.3ns (-18.2%) | [-234627, -132879]ns | [965966, 1064400] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 4579041ns | +3391258.5ns (+285.0%) | [+3276932, +3397896]ns | [4574885, 4581616] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 1182977ns | +0.3% | -18.2% | +287.2% |
| 1 | 2 | 1188279ns | +0.2% | -9.0% | +286.5% |
| 1 | 3 | 1246900ns | -5.0% | -14.7% | +266.4% |
| 1 | 4 | 1309836ns | -6.5% | -27.0% | +249.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.250 | moderate+ |
| bitpack-footprint-dense-alt | -0.177 | ok |
| bitpack-footprint-packed | -0.244 | moderate- |
| bitpack-footprint-packed-naive | -0.144 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 2/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 97.9ns | 1233919.7ns | 0.0% |  |
| bitpack-footprint-dense-alt | 102.7ns | 1196579.0ns | 0.0% |  |
| bitpack-footprint-packed | 95.7ns | 1020435.5ns | 0.0% |  |
| bitpack-footprint-packed-naive | 254.9ns | 4589482.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1182821.8-1313433.7 ns)
  1182821.8 |########################################
  1189352.4 |##############
  1195882.9 |
  1202413.6 |#####
  1208944.1 |
  1215474.8 |
  1222005.3 |
  1228535.9 |
  1235066.5 |
  1241597.1 |
  1248127.8 |
  1254658.3 |
  1261188.9 |
  1267719.5 |
  1274250.1 |
  1280780.7 |##
  1287311.3 |
  1293841.9 |
  1300372.5 |
  1306903.1 |############################
  (4 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 1184701.8-1225417.0 ns)
  1184701.8 |########################################
  1186737.6 |##################
  1188773.3 |#####################
  1190809.1 |#######
  1192844.9 |###
  1194880.6 |###
  1196916.4 |###
  1198952.1 |
  1200987.9 |
  1203023.6 |###
  1205059.4 |
  1207095.1 |
  1209130.9 |
  1211166.7 |###
  1213202.4 |
  1215238.2 |
  1217273.9 |#######
  1219309.7 |
  1221345.4 |
  1223381.2 |#######
  (3 below, 4 above range)

bitpack-footprint-packed (n=40, range 955815.8-1082012.0 ns)
  955815.8 |##########################
  962125.6 |##########################
  968435.4 |####
  974745.3 |
  981055.1 |
  987364.9 |
  993674.7 |####
  999984.5 |
  1006294.3 |####
  1012604.1 |####
  1018913.9 |
  1025223.7 |
  1031533.5 |
  1037843.3 |
  1044153.2 |
  1050463.0 |
  1056772.8 |
  1063082.6 |########################################
  1069392.4 |####
  1075702.2 |########################################
  (4 below, 1 above range)

bitpack-footprint-packed-naive (n=40, range 4568678.6-4641862.6 ns)
  4568678.6 |########################################
  4572337.8 |####################
  4575997.0 |#########################
  4579656.2 |###################################
  4583315.4 |#####
  4586974.6 |#########################
  4590633.8 |#####
  4594293.0 |##########
  4597952.2 |
  4601611.4 |
  4605270.6 |
  4608929.8 |
  4612589.0 |
  4616248.2 |
  4619907.4 |
  4623566.6 |
  4627225.8 |#####
  4630885.0 |
  4634544.2 |
  4638203.4 |
  (4 below, 2 above range)

```
