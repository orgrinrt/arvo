# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 15% faster than the next best (warm-clamp-acc64)

warm-clamp-accfit (526 ns) leads warm-clamp-acc64 (608 ns) by 15%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn is an outlier: 2.6x slower than the field

warm-clamp-accfit-dyn (1.37 us) is 2.6x the fastest (526 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit-dyn} (56% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit-dyn} with a 56% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-accfit** at 526.2 ns median (-13.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.61x (fastest 526.2 ns, slowest 1372.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 702ns | 673ns | 639ns | 676ns | 845ns | base |
| warm-clamp-accfit | 604ns | 585ns | 576ns | 594ns | 664ns | -13.96% |
| warm-clamp-accfit-dyn | 1443ns | 1434ns | 1413ns | 1433ns | 1502ns | +105.42% |
| warm-clamp-head | 651ns | 686ns | 569ns | 654ns | 724ns | -7.32% |
| warm-clamp-min-lanes | 1053ns | 1014ns | 1008ns | 1016ns | 1207ns | +49.87% |
| warm-clamp-minimum | 1096ns | 1072ns | 1052ns | 1080ns | 1187ns | +56.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 636ns | 579ns | 766ns | base | 12.883 |
| warm-clamp-accfit | 541ns | 516ns | 595ns | -14.87% | 15.134 |
| warm-clamp-accfit-dyn | 1380ns | 1354ns | 1436ns | +117.05% | 5.935 |
| warm-clamp-head | 581ns | 508ns | 646ns | -8.59% | 14.094 |
| warm-clamp-min-lanes | 991ns | 948ns | 1138ns | +55.80% | 8.269 |
| warm-clamp-minimum | 1034ns | 994ns | 1118ns | +62.65% | 7.921 |

## Performance model

- Peak throughput: **16.116 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 13.480 | 83.6% |
| warm-clamp-accfit | 15.567 | 96.6% |
| warm-clamp-accfit-dyn | 5.968 | 37.0% |
| warm-clamp-head | 13.383 | 83.0% |
| warm-clamp-min-lanes | 8.604 | 53.4% |
| warm-clamp-minimum | 8.101 | 50.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 702ns | 702ns | base |
| warm-clamp-accfit | 604ns | 604ns | -13.96% |
| warm-clamp-accfit-dyn | 1443ns | 1443ns | +105.42% |
| warm-clamp-head | 651ns | 651ns | -7.32% |
| warm-clamp-min-lanes | 1053ns | 1053ns | +49.87% |
| warm-clamp-minimum | 1096ns | 1096ns | +56.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 608ns | base | --- | [582, 614] | --- | --- | --- | --- |
| warm-clamp-accfit | 526ns | -73.8ns (-12.1%) | [-88, -64]ns | [521, 531] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1373ns | +779.2ns (+128.2%) | [+774, +792]ns | [1356, 1378] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 612ns | no significant difference | [-97, +5]ns | [516, 615] | no | 0.0807 | 0.0807 | 0 |
| warm-clamp-min-lanes | 952ns | +368.1ns (+60.6%) | [+349, +371]ns | [951, 959] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1011ns | +418.1ns (+68.8%) | [+414, +435]ns | [997, 1049] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 760ns | -22.6% | +78.0% | -18.8% | +25.5% | +31.0% |
| 2 | 762ns | -23.3% | +77.5% | -19.7% | +24.6% | +40.3% |
| 3 | 765ns | -23.0% | +81.2% | -19.2% | +24.6% | +30.4% |
| 4 | 765ns | -23.5% | +77.0% | -19.7% | +24.5% | +30.3% |
| 5 | 768ns | -23.4% | +76.7% | -19.8% | +27.5% | +29.4% |
| 6 | 771ns | -24.1% | +75.5% | -16.5% | +25.4% | +28.9% |
| 7 | 765ns | -14.7% | +76.9% | -20.0% | +26.2% | +42.6% |
| 8 | 767ns | -24.0% | +76.4% | -20.0% | +25.9% | +44.3% |
| 9 | 764ns | -23.6% | +77.5% | -19.0% | +26.4% | +30.3% |
| 10 | 760ns | -22.7% | +82.5% | -18.9% | +35.5% | +31.0% |
| 11 | 588ns | -5.9% | +133.5% | -13.4% | +61.8% | +78.4% |
| 12 | 595ns | -8.5% | +131.1% | -14.4% | +59.9% | +76.0% |
| 13 | 609ns | -10.4% | +144.3% | -15.3% | +56.1% | +68.7% |
| 14 | 580ns | -9.0% | +135.5% | -11.9% | +66.1% | +74.2% |
| 15 | 581ns | -10.5% | +150.0% | -11.7% | +63.6% | +77.3% |
| 16 | 582ns | -11.2% | +136.4% | -12.3% | +63.6% | +74.5% |
| 17 | 580ns | -10.0% | +133.5% | -12.7% | +63.7% | +74.2% |
| 18 | 582ns | -10.6% | +135.9% | -11.4% | +63.1% | +73.7% |
| 19 | 578ns | -10.5% | +137.8% | -11.0% | +64.5% | +74.8% |
| 20 | 579ns | -9.0% | +137.9% | -12.7% | +64.0% | +78.0% |
| 21 | 579ns | -10.7% | +134.3% | +6.5% | +64.0% | +72.1% |
| 22 | 578ns | -10.6% | +134.8% | +6.1% | +64.3% | +72.3% |
| 23 | 580ns | -10.6% | +133.4% | +6.3% | +63.9% | +72.0% |
| 24 | 579ns | -10.6% | +133.9% | +7.3% | +64.3% | +71.8% |
| 25 | 582ns | -11.3% | +138.7% | +4.8% | +63.1% | +70.8% |
| 26 | 582ns | -11.4% | +135.9% | +5.3% | +62.3% | +70.9% |
| 27 | 580ns | -11.2% | +136.8% | +5.4% | +63.7% | +71.3% |
| 28 | 581ns | -10.7% | +134.0% | +5.4% | +63.9% | +71.5% |
| 29 | 582ns | -11.5% | +133.9% | +5.9% | +63.8% | +70.9% |
| 30 | 582ns | -11.2% | +133.0% | +5.1% | +62.7% | +71.1% |
| 31 | 607ns | -13.7% | +123.3% | -15.9% | +90.9% | +74.4% |
| 32 | 612ns | -14.6% | +121.5% | -15.7% | +88.3% | +72.6% |
| 33 | 615ns | -14.2% | +124.3% | -16.2% | +88.9% | +71.6% |
| 34 | 611ns | -13.9% | +132.4% | -16.8% | +89.8% | +73.5% |
| 35 | 612ns | -13.9% | +132.8% | -17.4% | +88.3% | +137.1% |
| 36 | 615ns | -13.9% | +131.4% | -2.8% | +87.9% | +71.1% |
| 37 | 614ns | -14.7% | +132.2% | +7.9% | +84.9% | +71.0% |
| 38 | 614ns | -13.1% | +132.3% | +8.7% | +55.2% | +71.1% |
| 39 | 614ns | -15.0% | +132.1% | +9.0% | +54.8% | +71.4% |
| 40 | 615ns | -14.4% | +131.5% | +8.0% | +54.7% | +71.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.897 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.831 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.394 | moderate+ |
| warm-clamp-head | 0.776 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.800 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.175 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 26/40, lost 14/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.6ns | 635.9ns | 0.4% |  |
| warm-clamp-accfit | 2.4ns | 541.3ns | 0.4% |  |
| warm-clamp-accfit-dyn | 2.1ns | 1380.2ns | 0.2% |  |
| warm-clamp-head | 2.8ns | 581.2ns | 0.5% |  |
| warm-clamp-min-lanes | 2.5ns | 990.7ns | 0.3% |  |
| warm-clamp-minimum | 2.3ns | 1034.3ns | 0.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 579.1-765.9 ns)
    579.1 |########################################
    588.5 |##
    597.8 |##
    607.1 |##########################
    616.5 |
    625.8 |
    635.1 |
    644.5 |
    653.8 |
    663.2 |
    672.5 |
    681.8 |
    691.2 |
    700.5 |
    709.9 |
    719.2 |
    728.5 |
    737.9 |
    747.2 |
    756.5 |##################
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 516.1-595.0 ns)
    516.1 |########################################
    520.0 |##########################
    524.0 |######################
    527.9 |########
    531.9 |####
    535.8 |
    539.8 |
    543.7 |########
    547.7 |
    551.6 |####
    555.6 |
    559.5 |
    563.5 |
    567.4 |
    571.4 |
    575.3 |
    579.3 |
    583.2 |##########################
    587.2 |#############
    591.1 |
  (4 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1353.7-1436.1 ns)
   1353.7 |########################################
   1357.8 |######
   1361.9 |
   1366.0 |###
   1370.1 |###############
   1374.3 |############
   1378.4 |
   1382.5 |######
   1386.6 |###
   1390.8 |
   1394.9 |
   1399.0 |
   1403.1 |
   1407.2 |
   1411.4 |
   1415.5 |###
   1419.6 |###
   1423.7 |###############
   1427.8 |
   1432.0 |
  (3 below, 2 above range)

warm-clamp-head (n=40, range 508.3-645.6 ns)
    508.3 |#####################
    515.2 |########
    522.0 |
    528.9 |
    535.8 |
    542.6 |
    549.5 |
    556.4 |
    563.2 |
    570.1 |
    576.9 |
    583.8 |
    590.7 |##
    597.5 |
    604.4 |#####
    611.3 |########################################
    618.1 |#####
    625.0 |
    631.8 |
    638.7 |##
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 948.5-1137.9 ns)
    948.5 |########################################
    958.0 |########
    967.4 |
    976.9 |#
    986.4 |
    995.8 |
   1005.3 |
   1014.8 |
   1024.2 |#
   1033.7 |
   1043.2 |
   1052.6 |
   1062.1 |
   1071.6 |
   1081.0 |
   1090.5 |
   1100.0 |
   1109.5 |
   1118.9 |
   1128.4 |#
  (2 below, 6 above range)

warm-clamp-minimum (n=40, range 994.0-1118.5 ns)
    994.0 |########################################
   1000.2 |
   1006.5 |############
   1012.7 |###
   1018.9 |
   1025.1 |#########
   1031.4 |
   1037.6 |
   1043.8 |############
   1050.0 |###############
   1056.3 |######
   1062.5 |
   1068.7 |###
   1074.9 |
   1081.1 |
   1087.4 |###
   1093.6 |
   1099.8 |
   1106.0 |###
   1112.3 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.80 (measurement drift or warm-up artifact)
