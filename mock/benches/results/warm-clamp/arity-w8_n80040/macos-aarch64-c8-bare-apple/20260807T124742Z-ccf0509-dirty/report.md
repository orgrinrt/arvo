# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 11% faster than the next best (warm-clamp-head)

warm-clamp-accfit (237 ns) leads warm-clamp-head (263 ns) by 11%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 63% (significant)

warm-clamp-accfit is -410 ns (63%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 4.3x slower than the field

warm-clamp-min-lanes (1.02 us) is 4.3x the fastest (237 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.89)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-minimum, warm-clamp-min-lanes} (148% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-minimum, warm-clamp-min-lanes} with a 148% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest warm-clamp-accfit (237 ns) to slowest warm-clamp-min-lanes (1.02 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 237.1 ns median (-63.7% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.31x (fastest 237.1 ns, slowest 1021.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 755ns | 717ns | 691ns | 729ns | 894ns | base |
| warm-clamp-accfit | 306ns | 299ns | 292ns | 302ns | 334ns | -59.40% |
| warm-clamp-accfit-dyn | 778ns | 744ns | 739ns | 757ns | 879ns | +3.06% |
| warm-clamp-head | 347ns | 325ns | 321ns | 333ns | 419ns | -53.96% |
| warm-clamp-min-lanes | 1102ns | 1085ns | 1043ns | 1094ns | 1183ns | +46.00% |
| warm-clamp-minimum | 1137ns | 1081ns | 1026ns | 1091ns | 1387ns | +50.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 687ns | 630ns | 816ns | base | 11.932 |
| warm-clamp-accfit | 243ns | 232ns | 266ns | -64.55% | 33.659 |
| warm-clamp-accfit-dyn | 714ns | 678ns | 807ns | +3.99% | 11.473 |
| warm-clamp-head | 281ns | 260ns | 339ns | -59.06% | 29.142 |
| warm-clamp-min-lanes | 1037ns | 984ns | 1111ns | +50.99% | 7.903 |
| warm-clamp-minimum | 1073ns | 969ns | 1309ns | +56.23% | 7.637 |

## Performance model

- Peak throughput: **35.352 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 12.555 | 35.5% |
| warm-clamp-accfit | 34.551 | 97.7% |
| warm-clamp-accfit-dyn | 11.980 | 33.9% |
| warm-clamp-head | 31.184 | 88.2% |
| warm-clamp-min-lanes | 8.016 | 22.7% |
| warm-clamp-minimum | 8.033 | 22.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 755ns | 755ns | base |
| warm-clamp-accfit | 306ns | 306ns | -59.40% |
| warm-clamp-accfit-dyn | 778ns | 778ns | +3.06% |
| warm-clamp-head | 347ns | 347ns | -53.96% |
| warm-clamp-min-lanes | 1102ns | 1102ns | +46.00% |
| warm-clamp-minimum | 1137ns | 1137ns | +50.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 652ns | base | --- | [647, 655] | --- | --- | --- | --- |
| warm-clamp-accfit | 237ns | -415.4ns (-63.7%) | [-421, -409]ns | [234, 240] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 684ns | +43.2ns (+6.6%) | [+26, +64]ns | [681, 716] | YES | 0.0022 | 0.0022 | 0 |
| warm-clamp-head | 263ns | -391.9ns (-60.1%) | [-395, -386]ns | [261, 265] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1022ns | +370.2ns (+56.7%) | [+366, +378]ns | [1021, 1026] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1020ns | +390.0ns (+59.8%) | [+333, +396]ns | [987, 1026] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 812ns | -67.6% | -16.0% | -58.7% | +19.4% | +57.3% |
| 2 | 808ns | -67.5% | -15.5% | -57.6% | +20.2% | +57.9% |
| 3 | 814ns | -67.4% | -16.1% | -58.3% | +19.3% | +64.6% |
| 4 | 817ns | -67.5% | -17.0% | -58.3% | +18.8% | +55.9% |
| 5 | 814ns | -67.2% | -16.1% | -58.6% | +21.7% | +56.3% |
| 6 | 818ns | -67.5% | -16.4% | -58.7% | +25.0% | +56.1% |
| 7 | 815ns | -67.4% | -16.6% | -58.7% | +25.3% | +56.7% |
| 8 | 816ns | -67.3% | -17.1% | -58.8% | +25.2% | +81.0% |
| 9 | 814ns | -67.3% | -16.0% | -58.1% | +23.9% | +56.6% |
| 10 | 816ns | -67.4% | -16.9% | -58.5% | +18.8% | +56.2% |
| 11 | 628ns | -61.9% | +8.0% | -58.5% | +75.2% | +62.4% |
| 12 | 632ns | -63.2% | +8.4% | -57.9% | +74.6% | +62.0% |
| 13 | 631ns | -63.0% | +7.3% | -58.5% | +75.1% | +62.7% |
| 14 | 631ns | -63.1% | +22.3% | -58.3% | +75.2% | +62.4% |
| 15 | 630ns | -63.0% | +31.6% | -58.2% | +85.0% | +63.1% |
| 16 | 629ns | -62.5% | +31.2% | -57.9% | +75.1% | +63.1% |
| 17 | 630ns | -62.9% | +31.3% | -58.1% | +75.1% | +61.8% |
| 18 | 632ns | -62.7% | +30.5% | -58.0% | +74.3% | +62.1% |
| 19 | 628ns | -62.9% | +31.3% | -58.1% | +76.0% | +63.7% |
| 20 | 630ns | -62.8% | +31.2% | -57.7% | +75.0% | +62.1% |
| 21 | 645ns | -63.7% | +5.7% | -59.7% | +58.1% | +50.1% |
| 22 | 654ns | -64.6% | +3.7% | -59.9% | +56.2% | +48.0% |
| 23 | 650ns | -64.2% | +4.7% | -59.6% | +57.0% | +49.3% |
| 24 | 652ns | -64.5% | +4.0% | -60.1% | +56.7% | +61.3% |
| 25 | 643ns | -63.9% | +5.7% | -59.6% | +58.7% | +50.3% |
| 26 | 656ns | -64.1% | +3.6% | -59.9% | +55.6% | +47.8% |
| 27 | 656ns | -64.3% | +3.9% | -60.3% | +55.5% | +47.7% |
| 28 | 653ns | -64.0% | +4.0% | -60.1% | +55.9% | +48.2% |
| 29 | 655ns | -64.7% | +3.7% | -60.1% | +55.8% | +48.0% |
| 30 | 642ns | -64.3% | +7.8% | -59.5% | +59.3% | +51.0% |
| 31 | 636ns | -62.2% | +10.2% | -58.7% | +61.2% | +57.6% |
| 32 | 656ns | -62.8% | +6.2% | -60.4% | +55.8% | +50.6% |
| 33 | 655ns | -62.2% | +10.7% | -60.2% | +56.8% | +50.6% |
| 34 | 650ns | -61.9% | +11.0% | -59.8% | +58.3% | +51.4% |
| 35 | 648ns | -63.0% | +12.3% | -59.5% | +57.8% | +52.6% |
| 36 | 648ns | -63.2% | +12.9% | -59.9% | +59.0% | +51.9% |
| 37 | 652ns | -63.6% | +9.8% | -59.2% | +56.5% | +51.0% |
| 38 | 652ns | -63.7% | +10.0% | -60.2% | +56.8% | +51.4% |
| 39 | 661ns | -63.7% | +8.3% | -60.7% | +55.4% | +48.8% |
| 40 | 655ns | -63.6% | +9.7% | -60.3% | +56.5% | +50.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.879 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.895 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.830 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.889 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.769 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.853 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 10/40, lost 30/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.1ns | 686.6ns | 0.5% |  |
| warm-clamp-accfit | 2.9ns | 243.4ns | 1.2% |  |
| warm-clamp-accfit-dyn | 3.0ns | 714.0ns | 0.4% |  |
| warm-clamp-head | 3.1ns | 281.1ns | 1.1% |  |
| warm-clamp-min-lanes | 2.9ns | 1036.6ns | 0.3% |  |
| warm-clamp-minimum | 2.8ns | 1072.7ns | 0.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 629.5-815.5 ns)
    629.5 |######################
    638.8 |###########
    648.1 |########################################
    657.4 |##
    666.7 |
    676.0 |
    685.3 |
    694.6 |
    703.9 |
    713.2 |
    722.5 |
    731.8 |
    741.1 |
    750.4 |
    759.7 |
    769.0 |
    778.3 |
    787.6 |
    796.9 |
    806.2 |#################
  (3 below, 4 above range)

warm-clamp-accfit (n=40, range 231.7-265.9 ns)
    231.7 |########################################
    233.4 |########################################
    235.1 |##########################
    236.9 |####################
    238.6 |####################
    240.3 |######
    242.0 |
    243.7 |######
    245.4 |
    247.1 |#############
    248.8 |
    250.5 |
    252.2 |
    254.0 |
    255.7 |
    257.4 |
    259.1 |
    260.8 |
    262.5 |#############
    264.2 |########################################
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 678.0-807.5 ns)
    678.0 |########################################
    684.5 |##
    691.0 |####
    697.5 |##
    703.9 |
    710.4 |####
    716.9 |######
    723.3 |####
    729.8 |##
    736.3 |
    742.8 |
    749.2 |
    755.7 |
    762.2 |
    768.6 |##
    775.1 |
    781.6 |
    788.1 |
    794.5 |
    801.0 |
  (2 below, 6 above range)

warm-clamp-head (n=40, range 259.9-339.2 ns)
    259.9 |########################################
    263.8 |#########
    267.8 |
    271.8 |
    275.7 |
    279.7 |
    283.7 |
    287.6 |
    291.6 |
    295.6 |
    299.5 |
    303.5 |
    307.5 |
    311.4 |
    315.4 |
    319.4 |
    323.3 |
    327.3 |
    331.3 |#
    335.2 |#########
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 983.7-1111.2 ns)
    983.7 |
    990.1 |###
    996.5 |
   1002.8 |###
   1009.2 |
   1015.6 |########################################
   1022.0 |###########################
   1028.3 |###
   1034.7 |
   1041.1 |
   1047.4 |
   1053.8 |
   1060.2 |
   1066.6 |
   1072.9 |
   1079.3 |
   1085.7 |
   1092.0 |
   1098.4 |#####################
   1104.8 |######
  (5 below, 1 above range)

warm-clamp-minimum (n=40, range 968.6-1309.1 ns)
    968.6 |########################################
    985.6 |######################
   1002.6 |########
   1019.6 |########################################
   1036.7 |####
   1053.7 |
   1070.7 |
   1087.7 |
   1104.8 |
   1121.8 |
   1138.8 |
   1155.8 |
   1172.9 |
   1189.9 |
   1206.9 |
   1223.9 |
   1241.0 |
   1258.0 |#################
   1275.0 |#################
   1292.0 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
