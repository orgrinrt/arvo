# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bitpack-contend-d16, bitpack-contend-d16-control) are a dead heat (<1%)

bitpack-contend-d16 (733.75 us) and bitpack-contend-d16-control (737.66 us) differ by 0.53%, inside the noise, even though the wider field spreads 58.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-packed shows warm-up / thermal drift (autocorr +0.51)

bitpack-contend-packed's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-contend-d16)

The baseline bitpack-contend-d16 is the fastest (733.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} vs {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d64} (38% apart)

The field splits into a fast tier {bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} and a slow tier {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d64} with a 38% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (bitpack-contend-d16) is the fastest** at 733749.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.58x (fastest 733749.8 ns, slowest 1162106.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 747276ns | 735061ns | 726988ns | 740331ns | 788400ns | base |
| bitpack-contend-d16-control | 745895ns | 739059ns | 728817ns | 740670ns | 778651ns | -0.18% |
| bitpack-contend-d32 | 766114ns | 743744ns | 723278ns | 752066ns | 851092ns | +2.52% |
| bitpack-contend-d64 | 1224775ns | 1164368ns | 1082323ns | 1167292ns | 1539677ns | +63.90% |
| bitpack-contend-packed | 1100892ns | 1098730ns | 1095917ns | 1098943ns | 1111714ns | +47.32% |
| bitpack-contend-packed-simd | 1028877ns | 1027078ns | 1024007ns | 1026914ns | 1039637ns | +37.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 746058ns | 726064ns | 786897ns | base | 11.244 |
| bitpack-contend-d16-control | 744825ns | 727883ns | 777570ns | -0.17% | 11.263 |
| bitpack-contend-d32 | 764477ns | 721068ns | 849582ns | +2.47% | 10.973 |
| bitpack-contend-d64 | 1222499ns | 1079752ns | 1537563ns | +63.86% | 6.862 |
| bitpack-contend-packed | 1099664ns | 1094633ns | 1110605ns | +47.40% | 7.628 |
| bitpack-contend-packed-simd | 1027923ns | 1023278ns | 1038652ns | +37.78% | 8.161 |

## Performance model

- Peak throughput: **11.634 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.433 | 98.3% |
| bitpack-contend-d16-control | 11.372 | 97.8% |
| bitpack-contend-d32 | 11.298 | 97.1% |
| bitpack-contend-d64 | 7.218 | 62.0% |
| bitpack-contend-packed | 7.643 | 65.7% |
| bitpack-contend-packed-simd | 8.177 | 70.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 747276ns | 747276ns | base |
| bitpack-contend-d16-control | 745895ns | 745895ns | -0.18% |
| bitpack-contend-d32 | 766114ns | 766114ns | +2.52% |
| bitpack-contend-d64 | 1224775ns | 1224775ns | +63.90% |
| bitpack-contend-packed | 1100892ns | 1100892ns | +47.32% |
| bitpack-contend-packed-simd | 1028877ns | 1028877ns | +37.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 733750ns | base | --- | [732194, 743233] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 737656ns | no significant difference | [-4831, +3932]ns | [733257, 744670] | no | 0.3352 | 0.2682 | 0 |
| bitpack-contend-d32 | 742481ns | no significant difference | [-3416, +18664]ns | [730762, 763409] | no | 0.6358 | 0.6358 | 0 |
| bitpack-contend-d64 | 1162106ns | +386798.5ns (+52.7%) | [+374650, +445540]ns | [1112601, 1194539] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 1097554ns | +363232.3ns (+49.5%) | [+357015, +367042]ns | [1096565, 1098902] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 1025863ns | +290212.9ns (+39.6%) | [+282504, +293276]ns | [1025086, 1026534] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 770287ns | +0.4% | +5.8% | +50.2% | +42.9% | +34.2% |
| 2 | 783033ns | +1.5% | +6.9% | +49.4% | +41.5% | +31.1% |
| 3 | 786904ns | -2.3% | +6.6% | +43.0% | +41.2% | +34.2% |
| 4 | 782612ns | -0.3% | +15.1% | +50.7% | +41.9% | +34.5% |
| 5 | 728216ns | +4.0% | +0.3% | +76.5% | +51.3% | +41.1% |
| 6 | 725707ns | +0.9% | +0.9% | +83.8% | +51.6% | +41.2% |
| 7 | 730248ns | +0.1% | -0.3% | +59.1% | +50.6% | +40.1% |
| 8 | 736341ns | -1.2% | -0.8% | +60.0% | +49.3% | +39.4% |
| 9 | 745295ns | +0.3% | -1.4% | +144.4% | +48.2% | +37.7% |
| 10 | 764818ns | -4.7% | -3.9% | +210.5% | +44.0% | +34.3% |
| 11 | 733135ns | +0.6% | +3.8% | +51.2% | +49.7% | +39.9% |
| 12 | 733474ns | +0.3% | -2.4% | +60.4% | +49.7% | +39.6% |
| 13 | 732258ns | +0.1% | -2.2% | +52.4% | +49.6% | +40.0% |
| 14 | 732291ns | +2.7% | -0.6% | +51.4% | +49.5% | +40.3% |
| 15 | 743166ns | -1.5% | -3.5% | +49.3% | +47.2% | +38.1% |
| 16 | 727227ns | +0.5% | +0.1% | +66.4% | +51.1% | +40.9% |
| 17 | 728085ns | +0.3% | -0.3% | +49.8% | +50.7% | +41.0% |
| 18 | 727980ns | +0.5% | +2.8% | +50.2% | +50.5% | +41.0% |
| 19 | 729345ns | +0.5% | +10.2% | +48.3% | +50.6% | +40.4% |
| 20 | 728185ns | +0.7% | +4.6% | +77.0% | +50.3% | +40.9% |
| 21 | 724285ns | +0.0% | +0.4% | +49.7% | +51.2% | +41.6% |
| 22 | 724786ns | +4.6% | -0.8% | +48.7% | +51.1% | +42.1% |
| 23 | 724188ns | +2.8% | +1.7% | +49.1% | +51.2% | +44.5% |
| 24 | 726257ns | +8.2% | -0.2% | +48.5% | +51.3% | +41.9% |
| 25 | 734025ns | +1.4% | +11.8% | +46.3% | +49.5% | +39.4% |
| 26 | 767639ns | -3.7% | -4.7% | +39.7% | +42.9% | +33.4% |
| 27 | 735652ns | +0.3% | +8.2% | +51.8% | +48.9% | +39.4% |
| 28 | 774335ns | -5.7% | -1.1% | +57.9% | +41.4% | +32.4% |
| 29 | 732105ns | -0.8% | -1.3% | +51.1% | +49.5% | +40.2% |
| 30 | 732195ns | -1.0% | -0.6% | +50.9% | +49.7% | +40.1% |
| 31 | 743300ns | -1.3% | +14.9% | +96.2% | +51.1% | +38.2% |
| 32 | 739880ns | -0.9% | +17.6% | +80.8% | +52.1% | +38.3% |
| 33 | 760489ns | -3.7% | +1.8% | +60.4% | +44.3% | +35.1% |
| 34 | 742282ns | +7.8% | +1.7% | +81.2% | +47.8% | +38.0% |
| 35 | 733235ns | +1.7% | +2.3% | +74.7% | +49.7% | +39.2% |
| 36 | 748862ns | -0.5% | -0.0% | +71.8% | +46.6% | +36.9% |
| 37 | 732193ns | +2.5% | +10.6% | +58.7% | +50.3% | +40.0% |
| 38 | 832414ns | -10.8% | +3.4% | +34.4% | +31.5% | +23.1% |
| 39 | 787687ns | -5.5% | -2.8% | +47.8% | +39.3% | +31.1% |
| 40 | 777905ns | -2.3% | -5.8% | +72.3% | +40.9% | +31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.445 | moderate+ |
| bitpack-contend-d16-control | 0.381 | moderate+ |
| bitpack-contend-d32 | 0.419 | moderate+ |
| bitpack-contend-d64 | 0.351 | moderate+ |
| bitpack-contend-packed | 0.506 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.408 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 16/40, lost 23/40
- **bitpack-contend-d32**: won 17/40, lost 21/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 12.2ns | 746058.0ns | 0.0% |  |
| bitpack-contend-d16-control | 14.1ns | 744824.6ns | 0.0% |  |
| bitpack-contend-d32 | 9.4ns | 764477.2ns | 0.0% |  |
| bitpack-contend-d64 | 18.5ns | 1222498.9ns | 0.0% |  |
| bitpack-contend-packed | 8.2ns | 1099664.0ns | 0.0% |  |
| bitpack-contend-packed-simd | 7.7ns | 1027922.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 726064.3-786897.2 ns)
  726064.3 |##############################
  729105.9 |###############
  732147.6 |########################################
  735189.2 |##########
  738230.9 |#####
  741272.5 |###############
  744314.2 |#####
  747355.8 |#####
  750397.5 |
  753439.1 |
  756480.8 |
  759522.4 |#####
  762564.1 |#####
  765605.7 |#####
  768647.4 |#####
  771689.0 |#####
  774730.7 |
  777772.3 |#####
  780814.0 |##########
  783855.6 |
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 727883.4-777570.0 ns)
  727883.4 |###########
  730367.7 |########################################
  732852.0 |############################
  735336.4 |###########
  737820.7 |###########
  740305.0 |#####
  742789.4 |######################
  745273.7 |#####
  747758.0 |#####
  750242.3 |###########
  752726.7 |
  755211.0 |#####
  757695.3 |###########
  760179.7 |
  762664.0 |
  765148.3 |
  767632.6 |#####
  770117.0 |
  772601.3 |#####
  775085.6 |
  (4 below, 4 above range)

bitpack-contend-d32 (n=40, range 721068.2-849582.3 ns)
  721068.2 |#################
  727493.9 |########################################
  733919.6 |#############
  740345.3 |
  746771.0 |#############
  753196.8 |####
  759622.5 |#############
  766048.2 |####
  772473.9 |####
  778899.6 |
  785325.3 |
  791751.0 |####
  798176.7 |####
  804602.4 |####
  811028.1 |####
  817453.8 |####
  823879.5 |
  830305.2 |
  836730.9 |########
  843156.6 |
  (4 below, 4 above range)

bitpack-contend-d64 (n=40, range 1079751.6-1537562.5 ns)
  1079751.6 |#################
  1102642.2 |########################################
  1125532.7 |
  1148423.2 |######################
  1171313.8 |#############
  1194204.3 |####
  1217094.9 |########
  1239985.4 |
  1262876.0 |########
  1285766.5 |########
  1308657.1 |
  1331547.6 |#################
  1354438.2 |
  1377328.7 |
  1400219.2 |
  1423109.8 |
  1446000.3 |####
  1468890.9 |
  1491781.4 |
  1514672.0 |
  (5 below, 2 above range)

bitpack-contend-packed (n=40, range 1094633.2-1110605.0 ns)
  1094633.2 |######################
  1095431.8 |######################
  1096230.4 |####
  1097029.0 |########################################
  1097827.5 |####
  1098626.1 |#############
  1099424.7 |########
  1100223.3 |########
  1101021.9 |########
  1101820.5 |
  1102619.1 |
  1103417.7 |
  1104216.3 |####
  1105014.9 |
  1105813.5 |
  1106612.1 |
  1107410.6 |####
  1108209.2 |
  1109007.8 |
  1109806.4 |####
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 1023278.2-1038652.2 ns)
  1023278.2 |
  1024046.9 |########################################
  1024815.6 |##################################
  1025584.3 |########################################
  1026353.0 |############################
  1027121.7 |######################
  1027890.4 |
  1028659.1 |
  1029427.8 |#####
  1030196.5 |#####
  1030965.2 |
  1031733.9 |#####
  1032502.6 |
  1033271.3 |#####
  1034040.0 |
  1034808.7 |
  1035577.4 |
  1036346.1 |
  1037114.8 |
  1037883.5 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-packed**: autocorrelation=0.51 (measurement drift or warm-up artifact)
