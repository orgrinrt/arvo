# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 26% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (955.11 us) leads bitpack-footprint-dense-alt (1.20 ms) by 26%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 22% (significant)

bitpack-footprint-packed is -268.46 us (22%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 4.7x slower than the field

bitpack-footprint-packed-naive (4.53 ms) is 4.7x the fastest (955.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (268% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 268% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-footprint-packed (955.11 us) to slowest bitpack-footprint-packed-naive (4.53 ms): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 955110.4 ns median (-22.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.74x (fastest 955110.4 ns, slowest 4531356.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1250452ns | 1231249ns | 1186783ns | 1248667ns | 1319476ns | base |
| bitpack-footprint-dense-alt | 1249806ns | 1201054ns | 1182420ns | 1219227ns | 1408930ns | -0.05% |
| bitpack-footprint-packed | 965057ns | 955608ns | 952628ns | 956021ns | 1004595ns | -22.82% |
| bitpack-footprint-packed-naive | 4571954ns | 4531923ns | 4516162ns | 4540825ns | 4721135ns | +265.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1249954ns | 1186307ns | 1319032ns | base | 5.600 |
| bitpack-footprint-dense-alt | 1249346ns | 1181977ns | 1408504ns | -0.05% | 5.603 |
| bitpack-footprint-packed | 964625ns | 952151ns | 1004129ns | -22.83% | 7.257 |
| bitpack-footprint-packed-naive | 4571392ns | 4515616ns | 4720560ns | +265.72% | 1.531 |

## Performance model

- Peak throughput: **7.352 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.688 | 77.4% |
| bitpack-footprint-dense-alt | 5.831 | 79.3% |
| bitpack-footprint-packed | 7.329 | 99.7% |
| bitpack-footprint-packed-naive | 1.545 | 21.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1250452ns | 1250452ns | base |
| bitpack-footprint-dense-alt | 1249806ns | 1249806ns | -0.05% |
| bitpack-footprint-packed | 965057ns | 965057ns | -22.82% |
| bitpack-footprint-packed-naive | 4571954ns | 4571954ns | +265.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1230694ns | base | --- | [1219568, 1304691] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1200522ns | no significant difference | [-38833, +9394]ns | [1183635, 1217595] | no | 0.6358 | 0.6358 | 0 |
| bitpack-footprint-packed | 955110ns | -268461.2ns (-21.8%) | [-339994, -251288]ns | [953608, 957626] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 4531356ns | +3307596.0ns (+268.8%) | [+3271889, +3340414]ns | [4520706, 4569843] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 1313900ns | -10.0% | -27.5% | +243.7% |
| 1 | 2 | 1228934ns | +6.4% | -22.0% | +272.1% |
| 1 | 3 | 1209085ns | -0.5% | -21.0% | +273.6% |
| 1 | 4 | 1185457ns | -0.2% | -19.6% | +293.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.100 | ok |
| bitpack-footprint-dense-alt | -0.380 | moderate- |
| bitpack-footprint-packed | -0.334 | moderate- |
| bitpack-footprint-packed-naive | -0.372 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 121.0ns | 1249953.7ns | 0.0% |  |
| bitpack-footprint-dense-alt | 77.0ns | 1249345.8ns | 0.0% |  |
| bitpack-footprint-packed | 115.0ns | 964624.6ns | 0.0% |  |
| bitpack-footprint-packed-naive | 160.2ns | 4571391.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1186306.5-1319031.7 ns)
  1186306.5 |#############
  1192942.8 |#################
  1199579.0 |
  1206215.3 |
  1212851.6 |####
  1219487.8 |#################
  1226124.1 |######################
  1232760.3 |####
  1239396.6 |
  1246032.8 |
  1252669.1 |
  1259305.3 |
  1265941.6 |
  1272577.9 |
  1279214.1 |
  1285850.4 |
  1292486.6 |
  1299122.9 |#############
  1305759.1 |####
  1312395.4 |########################################
  (6 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 1181976.6-1408504.0 ns)
  1181976.6 |########################################
  1193302.9 |#######
  1204629.3 |############
  1215955.7 |##
  1227282.1 |
  1238608.4 |
  1249934.8 |
  1261261.2 |
  1272587.6 |
  1283913.9 |
  1295240.3 |#####
  1306566.7 |#################
  1317893.0 |##
  1329219.4 |
  1340545.8 |
  1351872.2 |
  1363198.5 |
  1374524.9 |
  1385851.3 |
  1397177.6 |
  (2 below, 3 above range)

bitpack-footprint-packed (n=40, range 952150.5-1004128.8 ns)
  952150.5 |########################################
  954749.5 |#############
  957348.4 |###############
  959947.3 |##
  962546.2 |
  965145.1 |##
  967744.0 |
  970342.9 |
  972941.8 |
  975540.8 |
  978139.7 |##
  980738.6 |
  983337.5 |
  985936.4 |
  988535.3 |
  991134.2 |
  993733.1 |##
  996332.1 |
  998931.0 |
  1001529.9 |
  (2 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 4515615.8-4720560.2 ns)
  4515615.8 |########################################
  4525863.0 |
  4536110.2 |##########
  4546357.5 |##
  4556604.7 |
  4566851.9 |####################
  4577099.1 |#####
  4587346.4 |
  4597593.6 |
  4607840.8 |
  4618088.0 |
  4628335.2 |
  4638582.5 |
  4648829.7 |
  4659076.9 |
  4669324.1 |
  4679571.4 |
  4689818.6 |
  4700065.8 |
  4710313.0 |
  (4 below, 5 above range)

```
