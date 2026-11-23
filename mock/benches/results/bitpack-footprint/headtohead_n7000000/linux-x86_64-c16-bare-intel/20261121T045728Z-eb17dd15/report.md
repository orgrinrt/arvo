# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 25% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (958.52 us) leads bitpack-footprint-dense-alt (1.20 ms) by 25%, a clear separation rather than a photo finish. CV 11.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 24% (significant)

bitpack-footprint-packed is -296.06 us (24%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 4.7x slower than the field

bitpack-footprint-packed-naive (4.53 ms) is 4.7x the fastest (958.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 11.2%)

bitpack-footprint-packed wins on median (958.52 us) yet has the highest variance (CV 11.2%), while bitpack-footprint-packed-naive is the steadiest (CV 0.8%, 4.53 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-footprint-dense shows alternating (throttle bounce) (autocorr -0.73)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (266% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 266% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-footprint-packed (958.52 us) to slowest bitpack-footprint-packed-naive (4.53 ms): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 958519.2 ns median (-22.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.73x (fastest 958519.2 ns, slowest 4529871.5 ns)

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
| Counter frequency | 2112765842 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1329461ns | 1237686ns | 1215472ns | 1306334ns | 1512830ns | base |
| bitpack-footprint-dense-alt | 1238097ns | 1197615ns | 1185512ns | 1228327ns | 1319991ns | -6.87% |
| bitpack-footprint-packed | 1011579ns | 959053ns | 927426ns | 979198ns | 1192874ns | -23.91% |
| bitpack-footprint-packed-naive | 4544965ns | 4530299ns | 4514798ns | 4535287ns | 4604163ns | +241.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1328872ns | 1214897ns | 1512241ns | base | 5.268 |
| bitpack-footprint-dense-alt | 1237571ns | 1184962ns | 1319493ns | -6.87% | 5.656 |
| bitpack-footprint-packed | 1011123ns | 927002ns | 1192505ns | -23.91% | 6.923 |
| bitpack-footprint-packed-naive | 4544439ns | 4514322ns | 4603653ns | +241.98% | 1.540 |

## Performance model

- Peak throughput: **7.551 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.658 | 74.9% |
| bitpack-footprint-dense-alt | 5.847 | 77.4% |
| bitpack-footprint-packed | 7.303 | 96.7% |
| bitpack-footprint-packed-naive | 1.545 | 20.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1329461ns | 1329461ns | base |
| bitpack-footprint-dense-alt | 1238097ns | 1238097ns | -6.87% |
| bitpack-footprint-packed | 1011579ns | 1011579ns | -23.91% |
| bitpack-footprint-packed-naive | 4544965ns | 4544965ns | +241.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1237107ns | base | --- | [1227193, 1435941] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1197105ns | -45843.4ns (-3.7%) | [-138063, -36784]ns | [1192800, 1292193] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed | 958519ns | -296059.5ns (-23.9%) | [-455246, -285856]ns | [933475, 992196] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 4529871ns | +3280742.0ns (+265.2%) | [+3157327, +3296197]ns | [4524687, 4541677] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 1226530ns | -3.3% | -23.9% | +269.9% |
| 1 | 2 | 1431569ns | -16.6% | -35.2% | +216.1% |
| 1 | 3 | 1222329ns | +2.1% | -2.6% | +269.4% |
| 1 | 4 | 1450208ns | -9.6% | -31.6% | +216.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.731 | HIGH- (thermal bounce) |
| bitpack-footprint-dense-alt | 0.246 | moderate+ |
| bitpack-footprint-packed | -0.264 | moderate- |
| bitpack-footprint-packed-naive | -0.236 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 175.0ns | 1328872.2ns | 0.0% |  |
| bitpack-footprint-dense-alt | 115.7ns | 1237571.0ns | 0.0% |  |
| bitpack-footprint-packed | 107.0ns | 1011122.6ns | 0.0% |  |
| bitpack-footprint-packed-naive | 199.0ns | 4544438.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1214896.9-1512240.6 ns)
  1214896.9 |########################################
  1229764.1 |#################
  1244631.3 |
  1259498.5 |##
  1274365.7 |
  1289232.9 |##
  1304100.0 |
  1318967.2 |
  1333834.4 |
  1348701.6 |
  1363568.8 |
  1378436.0 |
  1393303.2 |
  1408170.3 |
  1423037.5 |#####
  1437904.7 |######################
  1452771.9 |#####
  1467639.1 |
  1482506.3 |
  1497373.5 |
  (2 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 1184962.4-1319493.4 ns)
  1184962.4 |################################
  1191689.0 |########################################
  1198415.5 |####
  1205142.1 |########
  1211868.6 |####
  1218595.2 |
  1225321.7 |
  1232048.3 |
  1238774.8 |
  1245501.4 |
  1252227.9 |
  1258954.5 |
  1265681.0 |
  1272407.6 |
  1279134.1 |
  1285860.7 |####
  1292587.2 |####
  1299313.8 |
  1306040.3 |################################
  1312766.9 |############
  (3 below, 2 above range)

bitpack-footprint-packed (n=40, range 927002.1-1192505.3 ns)
  927002.1 |########################################
  940277.2 |#####
  953552.4 |#####
  966827.5 |##
  980102.7 |####################
  993377.9 |##
  1006653.0 |
  1019928.2 |
  1033203.4 |
  1046478.5 |
  1059753.7 |
  1073028.8 |
  1086304.0 |
  1099579.2 |
  1112854.3 |
  1126129.5 |
  1139404.7 |
  1152679.8 |
  1165955.0 |
  1179230.1 |####################
  (3 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 4514321.5-4603652.6 ns)
  4514321.5 |########################################
  4518788.1 |##########################
  4523254.6 |#################################
  4527721.2 |####################
  4532187.7 |######
  4536654.3 |####################
  4541120.8 |####################
  4545587.4 |######
  4550053.9 |
  4554520.5 |
  4558987.1 |######
  4563453.6 |
  4567920.2 |#############
  4572386.7 |######
  4576853.3 |######
  4581319.8 |######
  4585786.4 |
  4590252.9 |#############
  4594719.5 |######
  4599186.0 |
  (3 below, 2 above range)

```
