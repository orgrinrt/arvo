# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 87% faster than the next best (warm-container-kernel)

warm-container-native (752 ns) leads warm-container-kernel (1.41 us) by 87%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 93% (significant)

warm-container-native is -10.12 us (93%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 14.9x slower than the field

warm-container-plusone (11.24 us) is 14.9x the fastest (752 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.84)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone} (651% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone} with a 651% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 14.9x the fastest

Fastest warm-container-native (752 ns) to slowest warm-container-plusone (11.24 us): 14.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 752.5 ns median (-93.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 14.94x (fastest 752.5 ns, slowest 11240.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 11141ns | 10998ns | 10819ns | 11060ns | 11706ns | base |
| warm-container-kernel | 1517ns | 1471ns | 1437ns | 1480ns | 1706ns | -86.39% |
| warm-container-lanes-deferred | 1542ns | 1519ns | 1457ns | 1518ns | 1700ns | -86.16% |
| warm-container-minimum | 11316ns | 11125ns | 10839ns | 11134ns | 12339ns | +1.57% |
| warm-container-native | 815ns | 814ns | 801ns | 814ns | 832ns | -92.68% |
| warm-container-plusone | 11745ns | 11299ns | 10856ns | 11339ns | 13854ns | +5.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 11070ns | 10755ns | 11613ns | base | 12.581 |
| warm-container-kernel | 1455ns | 1380ns | 1635ns | -86.85% | 95.705 |
| warm-container-lanes-deferred | 1480ns | 1397ns | 1633ns | -86.63% | 94.128 |
| warm-container-minimum | 11249ns | 10780ns | 12269ns | +1.62% | 12.380 |
| warm-container-native | 753ns | 740ns | 767ns | -93.20% | 184.992 |
| warm-container-plusone | 11666ns | 10791ns | 13737ns | +5.39% | 11.938 |

## Performance model

- Peak throughput: **188.268 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.733 | 6.8% |
| warm-container-kernel | 98.727 | 52.4% |
| warm-container-lanes-deferred | 95.589 | 50.8% |
| warm-container-minimum | 12.595 | 6.7% |
| warm-container-native | 185.068 | 98.3% |
| warm-container-plusone | 12.390 | 6.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 11141ns | 11141ns | base |
| warm-container-kernel | 1517ns | 1517ns | -86.39% |
| warm-container-lanes-deferred | 1542ns | 1542ns | -86.16% |
| warm-container-minimum | 11316ns | 11316ns | +1.57% |
| warm-container-native | 815ns | 815ns | -92.68% |
| warm-container-plusone | 11745ns | 11745ns | +5.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 10937ns | base | --- | [10880, 11181] | --- | --- | --- | --- |
| warm-container-kernel | 1411ns | -9494.4ns (-86.8%) | [-9636, -9426]ns | [1384, 1458] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1457ns | -9485.2ns (-86.7%) | [-9684, -9415]ns | [1455, 1458] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 11057ns | no significant difference | [-21, +70]ns | [10931, 11218] | no | 0.2682 | 0.2682 | 0 |
| warm-container-native | 752ns | -10183.3ns (-93.1%) | [-10414, -10117]ns | [752, 753] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 11240ns | no significant difference | [-5, +415]ns | [11023, 11344] | no | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 10720ns | -87.1% | -86.3% | +0.7% | -93.0% | +2.1% |
| 2 | 10868ns | -87.3% | -86.6% | -0.7% | -92.9% | +0.3% |
| 3 | 10755ns | -87.2% | -86.5% | +0.3% | -93.0% | -0.4% |
| 4 | 10747ns | -87.1% | -86.4% | +0.3% | -93.0% | +0.5% |
| 5 | 10808ns | -87.2% | -86.9% | -0.3% | -93.0% | +2.8% |
| 6 | 10752ns | -87.1% | -86.9% | +3.5% | -93.0% | +25.4% |
| 7 | 12275ns | -88.6% | -88.6% | -12.1% | -93.9% | -12.1% |
| 8 | 11829ns | -88.0% | -88.1% | -7.5% | -93.6% | -8.3% |
| 9 | 11299ns | -87.1% | -87.6% | -3.0% | -93.3% | -3.3% |
| 10 | 11225ns | -87.0% | -87.3% | -1.7% | -93.3% | -1.2% |
| 11 | 10952ns | -87.2% | -87.4% | -1.5% | -93.1% | +13.7% |
| 12 | 10743ns | -87.1% | -87.1% | +0.5% | -93.0% | +3.1% |
| 13 | 10941ns | -87.4% | -87.4% | -1.5% | -93.1% | -2.0% |
| 14 | 11294ns | -87.8% | -86.6% | -1.9% | -93.3% | +1.8% |
| 15 | 11382ns | -87.9% | -85.3% | +1.9% | -93.4% | -5.2% |
| 16 | 11414ns | -87.9% | -85.3% | -0.6% | -93.4% | -5.6% |
| 17 | 11360ns | -87.9% | -85.0% | -0.1% | -93.4% | -0.8% |
| 18 | 10923ns | -87.3% | -84.6% | +0.4% | -93.1% | +12.0% |
| 19 | 10722ns | -87.1% | -84.4% | +0.6% | -93.0% | +13.6% |
| 20 | 10818ns | -86.7% | -84.5% | +0.4% | -93.0% | +12.8% |
| 21 | 10798ns | -84.4% | -86.5% | +19.5% | -93.1% | +12.8% |
| 22 | 10813ns | -84.5% | -86.5% | +17.5% | -93.2% | +12.8% |
| 23 | 10840ns | -84.5% | -86.5% | +12.5% | -93.2% | +16.9% |
| 24 | 10883ns | -84.6% | -86.6% | +11.3% | -93.2% | +1.3% |
| 25 | 11217ns | -85.0% | -87.0% | -2.8% | -93.3% | -1.7% |
| 26 | 11565ns | -84.8% | -87.4% | -6.8% | -93.5% | -2.8% |
| 27 | 11522ns | -87.9% | -87.4% | -2.2% | -93.6% | -2.5% |
| 28 | 11552ns | -88.0% | -87.4% | +5.9% | -93.6% | -4.7% |
| 29 | 10880ns | -87.3% | -86.6% | +12.3% | -93.2% | +4.1% |
| 30 | 10805ns | -87.2% | -86.5% | +12.7% | -93.2% | +1.1% |
| 31 | 10880ns | -86.6% | -86.6% | +2.2% | -92.8% | +4.3% |
| 32 | 11192ns | -87.0% | -87.0% | -1.0% | -93.0% | +2.0% |
| 33 | 11187ns | -87.0% | -86.9% | +2.8% | -93.2% | +1.3% |
| 34 | 10917ns | -86.6% | -86.6% | +2.6% | -93.1% | +5.1% |
| 35 | 11174ns | -86.9% | -87.0% | +0.6% | -93.3% | -0.4% |
| 36 | 10938ns | -86.6% | -86.7% | +3.2% | -93.1% | +0.4% |
| 37 | 11015ns | -86.7% | -86.7% | +0.6% | -92.9% | +45.5% |
| 38 | 10936ns | -86.7% | -86.8% | -0.1% | -93.1% | +70.4% |
| 39 | 10886ns | -86.6% | -86.5% | +0.4% | -93.1% | +4.1% |
| 40 | 10954ns | -86.7% | -86.8% | +0.2% | -93.1% | +3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.504 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.750 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.841 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.617 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.376 | moderate+ |
| warm-container-plusone | 0.366 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 14/40, lost 24/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 14/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 8.7ns | 11069.5ns | 0.1% |  |
| warm-container-kernel | 2.1ns | 1455.1ns | 0.1% |  |
| warm-container-lanes-deferred | 2.4ns | 1479.5ns | 0.2% |  |
| warm-container-minimum | 2.7ns | 11249.1ns | 0.0% |  |
| warm-container-native | 2.5ns | 752.8ns | 0.3% |  |
| warm-container-plusone | 3.8ns | 11665.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 10755.2-11612.5 ns)
  10755.2 |########
  10798.1 |########################################
  10840.9 |################################
  10883.8 |########################
  10926.7 |########################################
  10969.5 |
  11012.4 |########
  11055.3 |
  11098.1 |
  11141.0 |########
  11183.9 |################################
  11226.7 |
  11269.6 |################
  11312.5 |
  11355.3 |################
  11398.2 |########
  11441.1 |
  11483.9 |########
  11526.8 |################
  11569.6 |
  (6 below, 2 above range)

warm-container-kernel (n=40, range 1380.5-1635.2 ns)
   1380.5 |########################################
   1393.2 |#####
   1405.9 |
   1418.7 |##
   1431.4 |##
   1444.2 |########
   1456.9 |#########################
   1469.6 |
   1482.4 |
   1495.1 |
   1507.9 |
   1520.6 |
   1533.3 |
   1546.1 |
   1558.8 |
   1571.6 |
   1584.3 |
   1597.0 |
   1609.8 |
   1622.5 |
  (4 below, 6 above range)

warm-container-lanes-deferred (n=40, range 1397.3-1633.0 ns)
   1397.3 |#########
   1409.1 |##
   1420.8 |##
   1432.6 |
   1444.4 |################
   1456.2 |########################################
   1468.0 |
   1479.8 |
   1491.6 |
   1503.3 |##
   1515.1 |
   1526.9 |
   1538.7 |
   1550.5 |
   1562.3 |
   1574.0 |
   1585.8 |
   1597.6 |
   1609.4 |
   1621.2 |
  (3 below, 6 above range)

warm-container-minimum (n=40, range 10780.3-12268.6 ns)
  10780.3 |########################################
  10854.7 |#################
  10929.1 |############################
  11003.5 |#####
  11077.9 |############################
  11152.4 |#####
  11226.8 |#################
  11301.2 |###########
  11375.6 |
  11450.0 |#####
  11524.4 |#####
  11598.9 |
  11673.3 |
  11747.7 |
  11822.1 |
  11896.5 |
  11971.0 |
  12045.4 |#####
  12119.8 |#####
  12194.2 |#################
  (4 below, 2 above range)

warm-container-native (n=40, range 739.7-767.1 ns)
    739.7 |#####
    741.1 |##
    742.5 |##
    743.8 |
    745.2 |
    746.6 |##
    747.9 |
    749.3 |
    750.7 |##############
    752.1 |########################################
    753.4 |##############
    754.8 |
    756.2 |########
    757.5 |
    758.9 |
    760.3 |
    761.7 |
    763.0 |
    764.4 |
    765.8 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 10791.2-13737.1 ns)
  10791.2 |##############################
  10938.5 |###################################
  11085.8 |##########
  11233.1 |########################################
  11380.4 |###############
  11527.7 |
  11675.0 |
  11822.3 |
  11969.6 |
  12116.9 |#########################
  12264.2 |
  12411.5 |#####
  12558.8 |#####
  12706.0 |
  12853.3 |
  13000.6 |
  13147.9 |
  13295.2 |
  13442.5 |#####
  13589.8 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.62 (measurement drift or warm-up artifact)
