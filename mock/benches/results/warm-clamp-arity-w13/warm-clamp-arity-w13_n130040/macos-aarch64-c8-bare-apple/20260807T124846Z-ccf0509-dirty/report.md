# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 15% faster than the next best (warm-clamp-head)

warm-clamp-accfit (286 ns) leads warm-clamp-head (330 ns) by 15%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 48% (significant)

warm-clamp-accfit is -263 ns (48%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 3.8x slower than the field

warm-clamp-minimum (1.08 us) is 3.8x the fastest (286 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn shows warm-up / thermal drift (autocorr +0.88)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} (67% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-clamp-accfit (286 ns) to slowest warm-clamp-minimum (1.08 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 286.4 ns median (-47.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.79x (fastest 286.4 ns, slowest 1084.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 638ns | 611ns | 599ns | 617ns | 741ns | base |
| warm-clamp-accfit | 348ns | 349ns | 338ns | 348ns | 361ns | -45.42% |
| warm-clamp-accfit-dyn | 811ns | 821ns | 787ns | 812ns | 833ns | +27.09% |
| warm-clamp-head | 390ns | 395ns | 375ns | 391ns | 400ns | -38.93% |
| warm-clamp-min-lanes | 1132ns | 1117ns | 1104ns | 1124ns | 1182ns | +77.32% |
| warm-clamp-minimum | 1127ns | 1146ns | 1088ns | 1130ns | 1155ns | +76.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 571ns | 538ns | 656ns | base | 14.348 |
| warm-clamp-accfit | 285ns | 276ns | 297ns | -50.05% | 28.727 |
| warm-clamp-accfit-dyn | 747ns | 725ns | 765ns | +30.83% | 10.967 |
| warm-clamp-head | 326ns | 314ns | 333ns | -42.91% | 25.135 |
| warm-clamp-min-lanes | 1072ns | 1046ns | 1118ns | +87.74% | 7.643 |
| warm-clamp-minimum | 1066ns | 1030ns | 1093ns | +86.78% | 7.682 |

## Performance model

- Peak throughput: **29.734 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 14.905 | 50.1% |
| warm-clamp-accfit | 28.598 | 96.2% |
| warm-clamp-accfit-dyn | 10.844 | 36.5% |
| warm-clamp-head | 24.839 | 83.5% |
| warm-clamp-min-lanes | 7.745 | 26.0% |
| warm-clamp-minimum | 7.552 | 25.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 638ns | 638ns | base |
| warm-clamp-accfit | 348ns | 348ns | -45.42% |
| warm-clamp-accfit-dyn | 811ns | 811ns | +27.09% |
| warm-clamp-head | 390ns | 390ns | -38.93% |
| warm-clamp-min-lanes | 1132ns | 1132ns | +77.32% |
| warm-clamp-minimum | 1127ns | 1127ns | +76.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 550ns | base | --- | [547, 563] | --- | --- | --- | --- |
| warm-clamp-accfit | 286ns | -265.9ns (-48.4%) | [-278, -258]ns | [279, 289] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 755ns | +192.3ns (+35.0%) | [+169, +216]ns | [735, 761] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 330ns | -228.3ns (-41.5%) | [-235, -222]ns | [326, 331] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1058ns | +507.7ns (+92.4%) | [+493, +525]ns | [1056, 1064] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1085ns | +513.1ns (+93.4%) | [+496, +522]ns | [1052, 1087] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 567ns | -51.0% | +29.5% | -41.9% | +86.0% | +92.1% |
| 2 | 566ns | -51.5% | +29.7% | -41.6% | +86.7% | +92.0% |
| 3 | 567ns | -50.7% | +29.7% | -41.8% | +86.1% | +92.2% |
| 4 | 567ns | -51.8% | +29.4% | -41.4% | +87.0% | +93.2% |
| 5 | 561ns | -50.4% | +30.8% | -41.0% | +88.2% | +93.6% |
| 6 | 968ns | -71.5% | -24.3% | -65.6% | +10.0% | +12.1% |
| 7 | 564ns | -50.5% | +29.9% | -41.4% | +87.5% | +92.5% |
| 8 | 607ns | -53.9% | +21.1% | -45.4% | +74.0% | +79.1% |
| 9 | 689ns | -59.3% | +6.8% | -52.0% | +53.5% | +57.8% |
| 10 | 616ns | -55.1% | +18.5% | -46.5% | +71.3% | +76.7% |
| 11 | 562ns | -48.2% | +28.6% | -43.2% | +85.2% | +82.9% |
| 12 | 567ns | -49.2% | +27.3% | -42.8% | +85.9% | +93.3% |
| 13 | 565ns | -49.1% | +29.2% | -42.4% | +84.5% | +82.6% |
| 14 | 565ns | -49.1% | +29.3% | -41.4% | +84.1% | +82.7% |
| 15 | 550ns | -39.0% | +31.3% | -40.6% | +89.5% | +87.3% |
| 16 | 547ns | -47.2% | +32.4% | -41.3% | +90.8% | +88.7% |
| 17 | 632ns | -54.8% | +14.6% | -47.9% | +66.9% | +62.7% |
| 18 | 548ns | -47.6% | +32.0% | -39.7% | +92.7% | +88.3% |
| 19 | 544ns | -47.2% | +38.1% | -39.2% | +94.0% | +89.5% |
| 20 | 561ns | -48.3% | +31.0% | -41.9% | +88.5% | +83.6% |
| 21 | 599ns | -50.9% | +27.4% | -44.7% | +83.3% | +82.1% |
| 22 | 550ns | -47.3% | +39.4% | -39.7% | +99.9% | +98.5% |
| 23 | 547ns | -46.7% | +38.8% | -39.5% | +100.7% | +99.5% |
| 24 | 548ns | -47.1% | +39.3% | -39.4% | +99.3% | +99.7% |
| 25 | 546ns | -46.3% | +39.7% | -39.4% | +100.8% | +100.1% |
| 26 | 544ns | -46.8% | +40.5% | -39.1% | +101.6% | +101.4% |
| 27 | 546ns | -46.8% | +39.8% | -39.2% | +100.7% | +99.2% |
| 28 | 546ns | -47.1% | +39.5% | -39.5% | +101.4% | +98.6% |
| 29 | 547ns | -47.1% | +39.6% | -39.0% | +100.7% | +98.5% |
| 30 | 546ns | -47.2% | +39.8% | -38.5% | +101.5% | +98.8% |
| 31 | 535ns | -46.6% | +42.2% | -40.8% | +98.7% | +96.8% |
| 32 | 538ns | -46.1% | +42.2% | -41.8% | +97.6% | +94.7% |
| 33 | 547ns | -49.1% | +39.8% | -42.4% | +93.2% | +92.2% |
| 34 | 550ns | -49.2% | +39.3% | -42.5% | +92.4% | +91.6% |
| 35 | 552ns | -50.6% | +37.6% | -43.2% | +122.9% | +90.6% |
| 36 | 533ns | -46.7% | +43.4% | -41.0% | +100.7% | +96.8% |
| 37 | 540ns | -48.3% | +40.7% | -41.7% | +95.8% | +94.8% |
| 38 | 537ns | -48.2% | +42.8% | -41.7% | +97.9% | +95.9% |
| 39 | 540ns | -48.7% | +40.9% | -42.0% | +108.0% | +94.4% |
| 40 | 538ns | -48.6% | +41.2% | -41.2% | +97.5% | +95.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.069 | ok |
| warm-clamp-accfit | 0.342 | moderate+ |
| warm-clamp-accfit-dyn | 0.879 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.788 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.220 | moderate+ |
| warm-clamp-minimum | 0.642 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 1/40, lost 39/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.0ns | 570.9ns | 0.5% |  |
| warm-clamp-accfit | 3.0ns | 285.2ns | 1.0% |  |
| warm-clamp-accfit-dyn | 2.8ns | 746.9ns | 0.4% |  |
| warm-clamp-head | 2.9ns | 325.9ns | 0.9% |  |
| warm-clamp-min-lanes | 2.7ns | 1071.9ns | 0.2% |  |
| warm-clamp-minimum | 2.8ns | 1066.4ns | 0.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 537.9-655.6 ns)
    537.9 |########
    543.8 |########################################
    549.6 |#####
    555.5 |#####
    561.4 |#########################
    567.3 |
    573.2 |
    579.1 |
    585.0 |
    590.9 |
    596.8 |##
    602.6 |##
    608.5 |
    614.4 |##
    620.3 |
    626.2 |
    632.1 |##
    638.0 |
    643.9 |
    649.7 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 275.5-296.9 ns)
    275.5 |################
    276.6 |################
    277.6 |################################
    278.7 |########################################
    279.8 |########
    280.9 |
    281.9 |
    283.0 |
    284.1 |########
    285.1 |########
    286.2 |################
    287.3 |########################################
    288.3 |################################
    289.4 |########################################
    290.5 |########
    291.5 |########
    292.6 |########
    293.7 |########
    294.7 |
    295.8 |
  (3 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 724.7-765.1 ns)
    724.7 |#####
    726.7 |
    728.7 |#################
    730.8 |#####
    732.8 |############################
    734.8 |######################
    736.8 |
    738.8 |
    740.9 |
    742.9 |
    744.9 |
    746.9 |
    748.9 |
    751.0 |#####
    753.0 |
    755.0 |
    757.0 |
    759.0 |##################################
    761.1 |######################
    763.1 |########################################
  (5 below, 3 above range)

warm-clamp-head (n=40, range 314.2-332.6 ns)
    314.2 |#############
    315.1 |
    316.0 |#############
    317.0 |
    317.9 |
    318.8 |####
    319.7 |
    320.6 |####
    321.5 |
    322.5 |
    323.4 |####
    324.3 |
    325.2 |####
    326.1 |########
    327.0 |
    328.0 |
    328.9 |#################
    329.8 |#################
    330.7 |########################################
    331.6 |#################
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1046.5-1118.3 ns)
   1046.5 |
   1050.1 |
   1053.7 |########################################
   1057.2 |################
   1060.8 |#############
   1064.4 |###
   1068.0 |###
   1071.6 |
   1075.2 |
   1078.8 |
   1082.4 |
   1086.0 |
   1089.6 |###
   1093.1 |##########
   1096.7 |################
   1100.3 |###
   1103.9 |
   1107.5 |
   1111.1 |
   1114.7 |
  (5 below, 2 above range)

warm-clamp-minimum (n=40, range 1030.4-1093.2 ns)
   1030.4 |##################################
   1033.5 |
   1036.6 |
   1039.8 |
   1042.9 |
   1046.1 |#################
   1049.2 |##################################
   1052.4 |#####
   1055.5 |
   1058.7 |
   1061.8 |
   1064.9 |
   1068.1 |
   1071.2 |
   1074.4 |
   1077.5 |
   1080.7 |
   1083.8 |########################################
   1086.9 |########################################
   1090.1 |#################
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.64 (measurement drift or warm-up artifact)
