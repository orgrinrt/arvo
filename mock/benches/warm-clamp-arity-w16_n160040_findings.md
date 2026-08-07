# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 18% faster than the next best (warm-clamp-head)

warm-clamp-accfit (282 ns) leads warm-clamp-head (332 ns) by 18%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 49% (significant)

warm-clamp-accfit is -278 ns (49%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 3.6x slower than the field

warm-clamp-minimum (1.03 us) is 3.6x the fastest (282 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.84)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} (71% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} with a 71% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-clamp-accfit (282 ns) to slowest warm-clamp-minimum (1.03 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 281.9 ns median (-50.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.64x (fastest 281.9 ns, slowest 1026.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 628ns | 632ns | 611ns | 630ns | 638ns | base |
| warm-clamp-accfit | 350ns | 345ns | 340ns | 346ns | 371ns | -44.28% |
| warm-clamp-accfit-dyn | 883ns | 876ns | 856ns | 877ns | 928ns | +40.60% |
| warm-clamp-head | 394ns | 395ns | 381ns | 395ns | 400ns | -37.34% |
| warm-clamp-min-lanes | 1057ns | 1051ns | 1026ns | 1052ns | 1102ns | +68.30% |
| warm-clamp-minimum | 1084ns | 1090ns | 1057ns | 1082ns | 1117ns | +72.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 564ns | 549ns | 572ns | base | 14.531 |
| warm-clamp-accfit | 285ns | 277ns | 297ns | -49.47% | 28.760 |
| warm-clamp-accfit-dyn | 817ns | 795ns | 856ns | +44.99% | 10.022 |
| warm-clamp-head | 330ns | 321ns | 334ns | -41.55% | 24.860 |
| warm-clamp-min-lanes | 995ns | 968ns | 1037ns | +76.53% | 8.231 |
| warm-clamp-minimum | 1023ns | 998ns | 1053ns | +81.39% | 8.011 |

## Performance model

- Peak throughput: **29.523 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 14.457 | 49.0% |
| warm-clamp-accfit | 29.060 | 98.4% |
| warm-clamp-accfit-dyn | 10.087 | 34.2% |
| warm-clamp-head | 24.682 | 83.6% |
| warm-clamp-min-lanes | 8.266 | 28.0% |
| warm-clamp-minimum | 7.978 | 27.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 628ns | 628ns | base |
| warm-clamp-accfit | 350ns | 350ns | -44.28% |
| warm-clamp-accfit-dyn | 883ns | 883ns | +40.60% |
| warm-clamp-head | 394ns | 394ns | -37.34% |
| warm-clamp-min-lanes | 1057ns | 1057ns | +68.30% |
| warm-clamp-minimum | 1084ns | 1084ns | +72.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 567ns | base | --- | [565, 568] | --- | --- | --- | --- |
| warm-clamp-accfit | 282ns | -278.3ns (-49.1%) | [-283, -276]ns | [280, 288] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 812ns | +253.6ns (+44.7%) | [+244, +258]ns | [797, 826] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 332ns | -234.6ns (-41.4%) | [-236, -233]ns | [331, 332] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 991ns | +425.4ns (+75.1%) | [+418, +441]ns | [987, 993] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1027ns | +460.0ns (+81.2%) | [+450, +466]ns | [1003, 1032] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 565ns | -48.9% | +56.1% | -41.0% | +80.5% | +82.9% |
| 2 | 562ns | -48.7% | +47.1% | -40.5% | +81.0% | +82.6% |
| 3 | 567ns | -49.0% | +54.4% | -41.9% | +79.9% | +82.2% |
| 4 | 565ns | -48.9% | +47.8% | -41.1% | +76.3% | +83.8% |
| 5 | 569ns | -49.2% | +44.9% | -41.6% | +72.7% | +81.2% |
| 6 | 572ns | -39.0% | +44.9% | -41.6% | +73.3% | +80.4% |
| 7 | 565ns | -49.3% | +46.0% | -41.5% | +80.3% | +82.4% |
| 8 | 566ns | -49.0% | +45.8% | -40.9% | +80.0% | +94.6% |
| 9 | 570ns | -49.4% | +44.6% | -41.6% | +79.1% | +80.7% |
| 10 | 567ns | -49.0% | +46.6% | -41.2% | +75.2% | +81.3% |
| 11 | 567ns | -50.1% | +46.8% | -41.7% | +91.9% | +76.0% |
| 12 | 568ns | -51.0% | +45.4% | -41.8% | +74.2% | +80.1% |
| 13 | 566ns | -51.2% | +46.6% | -41.5% | +75.2% | +81.6% |
| 14 | 571ns | -51.1% | +45.1% | -41.9% | +72.7% | +76.2% |
| 15 | 569ns | -50.4% | +44.8% | -41.4% | +73.8% | +74.9% |
| 16 | 567ns | -50.0% | +46.2% | -41.4% | +77.4% | +76.7% |
| 17 | 571ns | -50.7% | +44.9% | -41.6% | +72.6% | +75.7% |
| 18 | 568ns | -51.1% | +45.5% | -41.7% | +75.0% | +75.9% |
| 19 | 568ns | -49.7% | +64.8% | -41.6% | +73.2% | +76.2% |
| 20 | 564ns | -48.4% | +47.2% | -41.4% | +75.3% | +81.1% |
| 21 | 569ns | -49.2% | +40.0% | -41.9% | +70.2% | +84.2% |
| 22 | 565ns | -49.2% | +41.0% | -41.5% | +71.5% | +88.2% |
| 23 | 568ns | -49.3% | +40.4% | -41.5% | +70.5% | +84.7% |
| 24 | 566ns | -48.9% | +40.6% | -41.3% | +70.6% | +84.8% |
| 25 | 571ns | -48.5% | +39.6% | -41.5% | +69.9% | +82.6% |
| 26 | 567ns | -50.9% | +40.3% | -41.1% | +70.7% | +82.8% |
| 27 | 582ns | -51.9% | +36.9% | -42.9% | +66.7% | +77.9% |
| 28 | 568ns | -50.9% | +39.9% | -41.2% | +70.5% | +82.6% |
| 29 | 565ns | -50.5% | +40.8% | -41.1% | +71.6% | +83.6% |
| 30 | 562ns | -50.6% | +41.9% | -40.7% | +72.1% | +84.6% |
| 31 | 546ns | -49.0% | +45.6% | -41.1% | +81.5% | +83.3% |
| 32 | 551ns | -49.9% | +44.4% | -41.9% | +79.4% | +81.7% |
| 33 | 545ns | -48.9% | +46.4% | -41.4% | +82.4% | +82.3% |
| 34 | 552ns | -49.4% | +43.8% | -42.0% | +80.4% | +81.0% |
| 35 | 551ns | -49.0% | +44.3% | -41.6% | +95.6% | +81.6% |
| 36 | 557ns | -49.6% | +43.7% | -42.4% | +78.2% | +79.4% |
| 37 | 570ns | -51.2% | +39.2% | -43.7% | +73.7% | +75.8% |
| 38 | 552ns | -49.4% | +44.5% | -41.6% | +87.1% | +80.9% |
| 39 | 549ns | -49.5% | +45.4% | -41.4% | +80.6% | +82.0% |
| 40 | 548ns | -48.9% | +45.5% | -41.3% | +80.9% | +85.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.639 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.223 | moderate+ |
| warm-clamp-accfit-dyn | 0.432 | moderate+ |
| warm-clamp-head | 0.837 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.278 | moderate+ |
| warm-clamp-minimum | 0.607 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.6ns | 563.8ns | 0.5% |  |
| warm-clamp-accfit | 2.8ns | 284.8ns | 1.0% |  |
| warm-clamp-accfit-dyn | 3.0ns | 817.4ns | 0.4% |  |
| warm-clamp-head | 2.7ns | 329.5ns | 0.8% |  |
| warm-clamp-min-lanes | 2.8ns | 995.2ns | 0.3% |  |
| warm-clamp-minimum | 2.1ns | 1022.6ns | 0.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 549.3-572.1 ns)
    549.3 |
    550.4 |###########
    551.5 |###########
    552.7 |
    553.8 |
    555.0 |
    556.1 |#####
    557.2 |
    558.4 |
    559.5 |
    560.7 |#####
    561.8 |#####
    562.9 |#####
    564.1 |#################
    565.2 |############################
    566.4 |########################################
    567.5 |#################
    568.7 |#################
    569.8 |#################
    570.9 |###########
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 277.5-297.4 ns)
    277.5 |########################################
    278.5 |######################
    279.5 |######################
    280.5 |###########
    281.5 |###########
    282.5 |###########
    283.5 |
    284.5 |
    285.5 |#####
    286.4 |###########
    287.4 |############################
    288.4 |##################################
    289.4 |
    290.4 |#####
    291.4 |
    292.4 |
    293.4 |#####
    294.4 |
    295.4 |
    296.4 |
  (2 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 794.9-856.3 ns)
    794.9 |########################################
    798.0 |########
    801.1 |
    804.2 |
    807.2 |
    810.3 |
    813.4 |
    816.4 |
    819.5 |
    822.6 |##############
    825.6 |###########
    828.7 |#################
    831.8 |#####
    834.9 |
    837.9 |
    841.0 |
    844.1 |
    847.1 |
    850.2 |
    853.3 |
  (3 below, 3 above range)

warm-clamp-head (n=40, range 320.8-334.0 ns)
    320.8 |###########
    321.5 |#################
    322.1 |#####
    322.8 |
    323.5 |
    324.1 |
    324.8 |
    325.4 |
    326.1 |
    326.7 |
    327.4 |
    328.1 |
    328.7 |
    329.4 |#####
    330.0 |#################
    330.7 |############################
    331.4 |#####
    332.0 |########################################
    332.7 |##################################
    333.3 |#################
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 968.0-1037.0 ns)
    968.0 |###############################
    971.4 |
    974.9 |
    978.3 |
    981.8 |########
    985.2 |#################
    988.7 |########################################
    992.1 |#################
    995.6 |####
    999.0 |
   1002.5 |
   1005.9 |####
   1009.4 |
   1012.8 |
   1016.3 |#################
   1019.7 |########
   1023.2 |
   1026.6 |
   1030.1 |####
   1033.5 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 997.8-1052.9 ns)
    997.8 |########################################
   1000.6 |############################
   1003.3 |#####
   1006.1 |#####
   1008.8 |
   1011.6 |
   1014.3 |
   1017.1 |#####
   1019.8 |#####
   1022.6 |#####
   1025.3 |###########
   1028.1 |#################
   1030.9 |######################
   1033.6 |#####
   1036.4 |############################
   1039.1 |
   1041.9 |#####
   1044.6 |#####
   1047.4 |###########
   1050.1 |
  (2 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.61 (measurement drift or warm-up artifact)
