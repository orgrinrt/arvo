# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed-naive is an outlier: 6.6x slower than the field

bitpack-footprint-packed-naive (9.47 us) is 6.6x the fastest (1.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-footprint-dense, bitpack-footprint-dense-alt) are a dead heat (<1%)

bitpack-footprint-dense (1.44 us) and bitpack-footprint-dense-alt (1.44 us) differ by 0.03%, inside the noise, even though the wider field spreads 556.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-footprint-dense-alt shows warm-up / thermal drift (autocorr +0.82)

bitpack-footprint-dense-alt's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (1.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-footprint-dense, bitpack-footprint-dense-alt, bitpack-footprint-packed} vs {bitpack-footprint-packed-naive} (338% apart)

The field splits into a fast tier {bitpack-footprint-dense, bitpack-footprint-dense-alt, bitpack-footprint-packed} and a slow tier {bitpack-footprint-packed-naive} with a 338% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.6x the fastest

Fastest bitpack-footprint-dense (1.44 us) to slowest bitpack-footprint-packed-naive (9.47 us): 6.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-dense-alt's edge over baseline is significant but tiny (4 ns, 0.29%)

bitpack-footprint-dense-alt differs from baseline bitpack-footprint-dense by 4 ns (0.29%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 1442.5 ns median
- 2 variants significantly slower than baseline
- Spread: 6.56x (fastest 1442.5 ns, slowest 9469.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1481ns | 1506ns | 1406ns | 1492ns | 1520ns | base |
| bitpack-footprint-dense-alt | 1501ns | 1506ns | 1405ns | 1492ns | 1626ns | +1.40% |
| bitpack-footprint-packed | 2203ns | 2234ns | 2099ns | 2185ns | 2360ns | +48.79% |
| bitpack-footprint-packed-naive | 9569ns | 9540ns | 9215ns | 9534ns | 10031ns | +546.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1418ns | 1348ns | 1457ns | base | 11.551 |
| bitpack-footprint-dense-alt | 1437ns | 1347ns | 1557ns | +1.32% | 11.401 |
| bitpack-footprint-packed | 2138ns | 2042ns | 2283ns | +50.72% | 7.664 |
| bitpack-footprint-packed-naive | 9507ns | 9160ns | 9970ns | +570.29% | 1.723 |

## Performance model

- Peak throughput: **12.168 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 11.358 | 93.3% |
| bitpack-footprint-dense-alt | 11.355 | 93.3% |
| bitpack-footprint-packed | 7.571 | 62.2% |
| bitpack-footprint-packed-naive | 1.730 | 14.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1481ns | 1481ns | base |
| bitpack-footprint-dense-alt | 1501ns | 1501ns | +1.40% |
| bitpack-footprint-packed | 2203ns | 2203ns | +48.79% |
| bitpack-footprint-packed-naive | 9569ns | 9569ns | +546.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1442ns | base | --- | [1417, 1445] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1443ns | no significant difference | [-2, +50]ns | [1387, 1447] | no | 1.0000 | 1.0000 | 2 |
| bitpack-footprint-packed | 2164ns | +720.6ns (+50.0%) | [+693, +747]ns | [2043, 2194] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 9469ns | +8050.1ns (+558.1%) | [+7940, +8111]ns | [9336, 9561] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|
| 1 | 1360ns | -0.8% | +50.2% | +615.2% |
| 2 | 1348ns | -0.0% | +51.5% | +623.7% |
| 3 | 1349ns | +0.0% | +51.4% | +623.6% |
| 4 | 1349ns | -0.1% | +51.4% | +622.9% |
| 5 | 1348ns | -0.0% | +51.6% | +619.6% |
| 6 | 1349ns | -0.1% | +51.4% | +587.0% |
| 7 | 1348ns | -0.3% | +51.5% | +590.2% |
| 8 | 1475ns | -8.7% | +38.5% | +528.6% |
| 9 | 1349ns | -0.2% | +51.4% | +587.3% |
| 10 | 1345ns | +4.1% | +51.8% | +587.3% |
| 11 | 1448ns | +7.3% | +54.8% | +576.0% |
| 12 | 1449ns | +7.7% | +51.6% | +577.2% |
| 13 | 1442ns | +7.9% | +52.1% | +555.2% |
| 14 | 1448ns | +7.3% | +51.5% | +541.7% |
| 15 | 1440ns | +8.0% | +52.4% | +544.1% |
| 16 | 1442ns | +7.8% | +51.8% | +544.3% |
| 17 | 1446ns | +7.8% | +51.7% | +540.5% |
| 18 | 1445ns | +7.6% | +52.1% | +551.8% |
| 19 | 1442ns | +7.9% | +52.4% | +554.9% |
| 20 | 1444ns | +7.9% | +51.6% | +548.5% |
| 21 | 1443ns | +0.5% | +41.5% | +530.0% |
| 22 | 1449ns | -0.1% | +41.0% | +617.4% |
| 23 | 1446ns | -0.1% | +41.2% | +587.7% |
| 24 | 1442ns | +0.3% | +53.2% | +530.7% |
| 25 | 1446ns | +7.4% | +41.2% | +528.8% |
| 26 | 1450ns | -4.2% | +40.9% | +559.9% |
| 27 | 1444ns | -6.8% | +41.4% | +631.4% |
| 28 | 1450ns | -7.0% | +40.9% | +569.2% |
| 29 | 1443ns | -6.6% | +41.5% | +529.8% |
| 30 | 1445ns | -6.6% | +41.4% | +532.0% |
| 31 | 1378ns | +5.0% | +59.4% | +588.6% |
| 32 | 1379ns | +4.7% | +62.7% | +588.1% |
| 33 | 1436ns | +0.7% | +51.9% | +559.2% |
| 34 | 1445ns | -0.2% | +92.3% | +552.1% |
| 35 | 1447ns | +0.0% | +50.9% | +560.6% |
| 36 | 1403ns | +3.2% | +53.0% | +577.4% |
| 37 | 1422ns | -1.6% | +54.3% | +569.2% |
| 38 | 1487ns | -6.8% | +47.4% | +544.6% |
| 39 | 1402ns | +0.0% | +56.8% | +575.5% |
| 40 | 1412ns | -0.1% | +55.7% | +584.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.522 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.816 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed | 0.335 | moderate+ |
| bitpack-footprint-packed-naive | 0.246 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 15/40, lost 18/40
- **bitpack-footprint-packed**: won 0/40, lost 40/40
- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 1.7ns | 1418.4ns | 0.1% |  |
| bitpack-footprint-dense-alt | 1.7ns | 1437.1ns | 0.1% |  |
| bitpack-footprint-packed | 1.5ns | 2137.8ns | 0.1% |  |
| bitpack-footprint-packed-naive | 1.8ns | 9507.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1348.0-1456.8 ns)
   1348.0 |################
   1353.4 |
   1358.8 |###
   1364.3 |
   1369.7 |
   1375.2 |######
   1380.6 |
   1386.0 |
   1391.5 |
   1396.9 |
   1402.4 |######
   1407.8 |###
   1413.3 |
   1418.7 |###
   1424.1 |
   1429.6 |
   1435.0 |######
   1440.5 |########################################
   1445.9 |##############################
   1451.3 |
  (3 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 1346.5-1557.2 ns)
   1346.5 |########################################
   1357.0 |
   1367.6 |
   1378.1 |########
   1388.6 |
   1399.2 |############
   1409.7 |####
   1420.2 |
   1430.8 |
   1441.3 |########################################
   1451.8 |
   1462.4 |
   1472.9 |
   1483.4 |
   1494.0 |
   1504.5 |
   1515.0 |
   1525.6 |
   1536.1 |
   1546.6 |################################
  (3 below, 3 above range)

bitpack-footprint-packed (n=40, range 2041.7-2283.2 ns)
   2041.7 |########################################
   2053.8 |
   2065.9 |
   2078.0 |
   2090.0 |
   2102.1 |
   2114.2 |
   2126.2 |
   2138.3 |###
   2150.4 |
   2162.5 |
   2174.5 |######
   2186.6 |####################################
   2198.7 |#########
   2210.7 |
   2222.8 |
   2234.9 |######
   2247.0 |
   2259.0 |
   2271.1 |
  (6 below, 1 above range)

bitpack-footprint-packed-naive (n=40, range 9159.7-9970.5 ns)
   9159.7 |
   9200.2 |
   9240.8 |########################################
   9281.3 |####################
   9321.8 |
   9362.4 |######
   9402.9 |#############
   9443.5 |##########################
   9484.0 |##########################
   9524.5 |######
   9565.1 |#############
   9605.6 |
   9646.2 |######
   9686.7 |#############
   9727.2 |##########################
   9767.8 |######
   9808.3 |######
   9848.9 |
   9889.4 |
   9929.9 |######
  (5 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **bitpack-footprint-dense-alt**: autocorrelation=0.82 (measurement drift or warm-up artifact)
