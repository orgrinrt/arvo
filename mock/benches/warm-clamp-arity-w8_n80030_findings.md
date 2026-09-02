# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 10% faster than the next best (warm-clamp-head)

warm-clamp-accfit (468 ns) leads warm-clamp-head (515 ns) by 10%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 31% (significant)

warm-clamp-accfit is -207 ns (31%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 2.4x slower than the field

warm-clamp-accfit-dyn (1.12 us) is 2.4x the fastest (468 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.87)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit-dyn} (48% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit-dyn} with a 48% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-accfit** at 467.7 ns median (-30.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.40x (fastest 467.7 ns, slowest 1124.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 732ns | 729ns | 725ns | 730ns | 743ns | base |
| warm-clamp-accfit | 559ns | 529ns | 519ns | 545ns | 641ns | -23.57% |
| warm-clamp-accfit-dyn | 1182ns | 1189ns | 1127ns | 1176ns | 1253ns | +61.47% |
| warm-clamp-head | 578ns | 575ns | 572ns | 576ns | 590ns | -21.03% |
| warm-clamp-min-lanes | 1079ns | 1087ns | 1046ns | 1076ns | 1119ns | +47.43% |
| warm-clamp-minimum | 1121ns | 1050ns | 1031ns | 1072ns | 1359ns | +53.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 670ns | 664ns | 679ns | base | 12.226 |
| warm-clamp-accfit | 494ns | 458ns | 567ns | -26.25% | 16.577 |
| warm-clamp-accfit-dyn | 1118ns | 1067ns | 1187ns | +66.87% | 7.326 |
| warm-clamp-head | 517ns | 512ns | 529ns | -22.78% | 15.831 |
| warm-clamp-min-lanes | 1018ns | 987ns | 1057ns | +51.99% | 8.044 |
| warm-clamp-minimum | 1057ns | 972ns | 1282ns | +57.79% | 7.748 |

## Performance model

- Peak throughput: **17.874 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 12.265 | 68.6% |
| warm-clamp-accfit | 17.516 | 98.0% |
| warm-clamp-accfit-dyn | 7.287 | 40.8% |
| warm-clamp-head | 15.907 | 89.0% |
| warm-clamp-min-lanes | 7.992 | 44.7% |
| warm-clamp-minimum | 8.273 | 46.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 732ns | 732ns | base |
| warm-clamp-accfit | 559ns | 559ns | -23.57% |
| warm-clamp-accfit-dyn | 1182ns | 1182ns | +61.47% |
| warm-clamp-head | 578ns | 578ns | -21.03% |
| warm-clamp-min-lanes | 1079ns | 1079ns | +47.43% |
| warm-clamp-minimum | 1121ns | 1121ns | +53.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 668ns | base | --- | [667, 670] | --- | --- | --- | --- |
| warm-clamp-accfit | 468ns | -206.3ns (-30.9%) | [-208, -150]ns | [465, 517] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1124ns | +447.9ns (+67.1%) | [+420, +454]ns | [1091, 1125] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 515ns | -151.7ns (-22.7%) | [-153, -151]ns | [515, 516] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1025ns | +350.2ns (+52.4%) | [+329, +361]ns | [1000, 1030] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 990ns | +313.3ns (+46.9%) | [+309, +319]ns | [978, 995] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 677ns | -32.4% | +66.1% | -21.0% | +52.4% | +47.2% |
| 2 | 682ns | -30.8% | +64.8% | -21.7% | +51.3% | +46.2% |
| 3 | 671ns | -31.4% | +68.4% | -21.2% | +53.9% | +46.0% |
| 4 | 670ns | -31.3% | +68.2% | -22.4% | +54.2% | +46.1% |
| 5 | 668ns | -30.1% | +68.5% | -21.9% | +54.1% | +45.9% |
| 6 | 667ns | -29.5% | +69.1% | -21.0% | +55.7% | +46.4% |
| 7 | 666ns | -31.1% | +69.2% | -20.2% | +50.3% | +46.1% |
| 8 | 665ns | -31.1% | +69.5% | -20.3% | +47.0% | +46.1% |
| 9 | 669ns | -30.5% | +68.5% | -23.7% | +46.3% | +45.2% |
| 10 | 667ns | -31.2% | +68.8% | -21.7% | +47.0% | +46.4% |
| 11 | 667ns | -9.2% | +61.8% | -22.6% | +49.3% | +47.7% |
| 12 | 666ns | -9.0% | +63.0% | -22.6% | +49.8% | +46.1% |
| 13 | 669ns | -9.8% | +62.0% | -22.9% | +48.8% | +45.9% |
| 14 | 668ns | -9.4% | +62.4% | -23.3% | +53.3% | +46.1% |
| 15 | 667ns | -17.1% | +62.7% | -22.8% | +65.9% | +45.9% |
| 16 | 668ns | -31.0% | +67.9% | -22.3% | +49.3% | +45.7% |
| 17 | 660ns | -30.5% | +70.1% | -22.0% | +50.6% | +47.2% |
| 18 | 667ns | -31.3% | +68.6% | -22.8% | +48.6% | +46.2% |
| 19 | 668ns | -31.7% | +68.2% | -22.8% | +46.9% | +45.9% |
| 20 | 670ns | -31.4% | +68.3% | -23.3% | +61.3% | +47.7% |
| 21 | 666ns | -21.6% | +60.7% | -22.9% | +55.0% | +92.8% |
| 22 | 664ns | -21.7% | +60.8% | -22.4% | +55.0% | +93.1% |
| 23 | 665ns | -21.8% | +60.4% | -22.9% | +55.0% | +92.9% |
| 24 | 668ns | -22.2% | +59.9% | -22.6% | +54.1% | +91.8% |
| 25 | 665ns | -21.4% | +60.5% | -22.6% | +55.3% | +92.3% |
| 26 | 668ns | -22.2% | +59.4% | -22.7% | +54.1% | +91.6% |
| 27 | 666ns | -22.7% | +60.2% | -22.8% | +55.0% | +92.3% |
| 28 | 667ns | -22.0% | +60.9% | -22.3% | +54.1% | +92.5% |
| 29 | 664ns | -21.4% | +66.3% | -22.9% | +55.3% | +92.9% |
| 30 | 666ns | -21.6% | +60.5% | -22.6% | +54.8% | +91.2% |
| 31 | 674ns | -30.6% | +66.9% | -23.2% | +63.2% | +47.4% |
| 32 | 676ns | -31.1% | +66.7% | -23.9% | +47.9% | +47.1% |
| 33 | 678ns | -30.5% | +66.6% | -24.4% | +47.3% | +46.1% |
| 34 | 681ns | -31.3% | +65.1% | -24.5% | +47.2% | +46.2% |
| 35 | 678ns | -31.0% | +66.1% | -24.1% | +47.0% | +46.6% |
| 36 | 681ns | -31.8% | +114.9% | -24.2% | +47.0% | +45.4% |
| 37 | 675ns | -30.7% | +62.4% | -23.7% | +48.0% | +46.8% |
| 38 | 675ns | -30.8% | +87.4% | -24.2% | +51.9% | +47.4% |
| 39 | 680ns | -31.7% | +65.6% | -24.4% | +50.1% | +45.9% |
| 40 | 679ns | -31.4% | +66.2% | -24.7% | +47.0% | +46.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.733 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.744 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.099 | ok |
| warm-clamp-head | 0.591 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.239 | moderate+ |
| warm-clamp-minimum | 0.869 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 670.1ns | 0.4% |  |
| warm-clamp-accfit | 2.6ns | 494.2ns | 0.5% |  |
| warm-clamp-accfit-dyn | 3.0ns | 1118.1ns | 0.3% |  |
| warm-clamp-head | 2.8ns | 517.5ns | 0.5% |  |
| warm-clamp-min-lanes | 2.4ns | 1018.4ns | 0.2% |  |
| warm-clamp-minimum | 2.7ns | 1057.3ns | 0.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 664.2-679.4 ns)
    664.2 |#############
    664.9 |######
    665.7 |#################################
    666.5 |########################################
    667.2 |########################################
    668.0 |
    668.7 |#############
    669.5 |#############
    670.2 |######
    671.0 |
    671.8 |
    672.5 |
    673.3 |
    674.0 |######
    674.8 |#############
    675.6 |######
    676.3 |
    677.1 |####################
    677.8 |
    678.6 |######
  (3 below, 4 above range)

warm-clamp-accfit (n=40, range 458.3-567.4 ns)
    458.3 |#####################
    463.8 |########################################
    469.2 |##########
    474.7 |
    480.1 |
    485.6 |
    491.0 |
    496.5 |
    502.0 |
    507.4 |
    512.9 |###
    518.3 |################################
    523.8 |
    529.2 |
    534.7 |
    540.2 |
    545.6 |
    551.1 |###
    556.5 |
    562.0 |
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1067.2-1187.3 ns)
   1067.2 |#############
   1073.2 |
   1079.2 |##########
   1085.2 |##
   1091.2 |##
   1097.2 |
   1103.2 |##
   1109.2 |
   1115.2 |##
   1121.3 |########################################
   1127.3 |################
   1133.3 |
   1139.3 |
   1145.3 |
   1151.3 |
   1157.3 |
   1163.3 |
   1169.3 |
   1175.3 |
   1181.3 |
  (4 below, 2 above range)

warm-clamp-head (n=40, range 511.9-528.7 ns)
    511.9 |################
    512.7 |####
    513.6 |####################
    514.4 |########################################
    515.2 |############
    516.1 |########
    516.9 |
    517.8 |########
    518.6 |####
    519.5 |####
    520.3 |
    521.1 |########
    522.0 |
    522.8 |
    523.7 |
    524.5 |
    525.4 |
    526.2 |####
    527.0 |
    527.9 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 986.5-1056.9 ns)
    986.5 |
    990.1 |########
    993.6 |############
    997.1 |########################
   1000.6 |############
   1004.1 |
   1007.7 |
   1011.2 |
   1014.7 |
   1018.2 |####
   1021.7 |####
   1025.3 |########
   1028.8 |########################################
   1032.3 |################
   1035.8 |####
   1039.3 |
   1042.8 |
   1046.4 |
   1049.9 |
   1053.4 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 972.4-1281.7 ns)
    972.4 |########################################
    987.9 |#####################################
   1003.4 |
   1018.8 |
   1034.3 |
   1049.8 |
   1065.2 |
   1080.7 |
   1096.1 |
   1111.6 |
   1127.1 |
   1142.5 |
   1158.0 |
   1173.5 |
   1188.9 |
   1204.4 |
   1219.8 |
   1235.3 |
   1250.8 |
   1266.2 |#################
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.87 (measurement drift or warm-up artifact)
