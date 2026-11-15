# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bitpack-carrier-d16-control, bitpack-carrier-d16) are a dead heat (<1%)

bitpack-carrier-d16-control (735.30 us) and bitpack-carrier-d16 (737.73 us) differ by 0.33%, inside the noise, even though the wider field spreads 66.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-carrier-d16-control shows warm-up / thermal drift (autocorr +0.81)

bitpack-carrier-d16-control's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} vs {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d64} (39% apart)

The field splits into a fast tier {bitpack-carrier-d16-control, bitpack-carrier-d16, bitpack-carrier-d32} and a slow tier {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d64} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-d16-control** at 735295.2 ns median (-0.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.66x (fastest 735295.2 ns, slowest 1222556.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 747672ns | 739340ns | 731466ns | 742502ns | 779390ns | base |
| bitpack-carrier-d16-control | 745776ns | 736663ns | 729880ns | 737507ns | 786478ns | -0.25% |
| bitpack-carrier-d32 | 767714ns | 741527ns | 717329ns | 753360ns | 861159ns | +2.68% |
| bitpack-carrier-d64 | 1244090ns | 1224153ns | 1095053ns | 1231644ns | 1430468ns | +66.40% |
| bitpack-carrier-packed | 1107250ns | 1102721ns | 1097249ns | 1103108ns | 1129678ns | +48.09% |
| bitpack-carrier-packed-simd | 1029541ns | 1027725ns | 1022985ns | 1028201ns | 1040114ns | +37.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 746274ns | 730140ns | 778368ns | base | 11.241 |
| bitpack-carrier-d16-control | 744399ns | 728763ns | 785416ns | -0.25% | 11.269 |
| bitpack-carrier-d32 | 766061ns | 715172ns | 859828ns | +2.65% | 10.950 |
| bitpack-carrier-d64 | 1241938ns | 1092429ns | 1428576ns | +66.42% | 6.754 |
| bitpack-carrier-packed | 1106071ns | 1095932ns | 1128583ns | +48.21% | 7.584 |
| bitpack-carrier-packed-simd | 1028422ns | 1021750ns | 1039096ns | +37.81% | 8.157 |

## Performance model

- Peak throughput: **11.730 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.371 | 96.9% |
| bitpack-carrier-d16-control | 11.408 | 97.3% |
| bitpack-carrier-d32 | 11.329 | 96.6% |
| bitpack-carrier-d64 | 6.862 | 58.5% |
| bitpack-carrier-packed | 7.616 | 64.9% |
| bitpack-carrier-packed-simd | 8.169 | 69.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 747672ns | 747672ns | base |
| bitpack-carrier-d16-control | 745776ns | 745776ns | -0.25% |
| bitpack-carrier-d32 | 767714ns | 767714ns | +2.68% |
| bitpack-carrier-d64 | 1244090ns | 1244090ns | +66.40% |
| bitpack-carrier-packed | 1107250ns | 1107250ns | +48.09% |
| bitpack-carrier-packed-simd | 1029541ns | 1029541ns | +37.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 737729ns | base | --- | [735939, 746211] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 735295ns | no significant difference | [-8531, +880]ns | [732412, 736832] | no | 0.1009 | 0.0807 | 0 |
| bitpack-carrier-d32 | 740435ns | no significant difference | [-9643, +16545]ns | [729149, 764366] | no | 0.6358 | 0.6358 | 0 |
| bitpack-carrier-d64 | 1222556ns | +468119.1ns (+63.5%) | [+402150, +581305]ns | [1132543, 1320086] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 1101400ns | +362317.1ns (+49.1%) | [+359774, +366426]ns | [1100262, 1103444] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 1026903ns | +286873.5ns (+38.9%) | [+282038, +291453]ns | [1025458, 1028116] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 757582ns | -2.7% | +11.2% | +83.0% | +45.9% | +36.6% |
| 2 | 771745ns | -4.2% | +6.5% | +76.8% | +46.9% | +34.3% |
| 3 | 807321ns | -6.3% | +7.9% | +77.9% | +42.6% | +29.6% |
| 4 | 759937ns | +4.4% | +19.5% | +79.8% | +49.5% | +35.5% |
| 5 | 775720ns | -2.6% | +3.6% | +57.4% | +44.6% | +32.9% |
| 6 | 756170ns | +4.1% | +7.2% | +61.9% | +48.6% | +35.9% |
| 7 | 770169ns | +4.2% | +10.6% | +68.1% | +44.0% | +34.2% |
| 8 | 749973ns | +8.0% | +20.3% | +83.6% | +47.1% | +37.8% |
| 9 | 752735ns | +7.1% | +8.1% | +99.6% | +50.2% | +40.1% |
| 10 | 811591ns | -4.9% | -0.3% | +76.4% | +36.5% | +27.6% |
| 11 | 728264ns | +0.5% | +1.7% | +51.9% | +51.2% | +41.1% |
| 12 | 728260ns | +0.4% | +4.9% | +54.4% | +51.2% | +41.1% |
| 13 | 729140ns | +0.4% | +1.5% | +95.3% | +51.5% | +40.6% |
| 14 | 728739ns | +0.3% | +17.7% | +84.1% | +51.1% | +40.1% |
| 15 | 772882ns | -4.9% | +0.1% | +82.8% | +42.6% | +32.1% |
| 16 | 753044ns | -2.6% | -2.6% | +87.4% | +49.0% | +35.6% |
| 17 | 740181ns | -1.2% | -1.2% | +76.0% | +48.6% | +38.2% |
| 18 | 740120ns | -1.2% | -2.0% | +77.6% | +48.7% | +38.1% |
| 19 | 732602ns | -0.2% | +0.8% | +74.5% | +50.3% | +40.8% |
| 20 | 732955ns | -0.3% | +0.9% | +47.9% | +50.3% | +39.9% |
| 21 | 735876ns | -0.0% | -2.5% | +56.6% | +49.5% | +39.0% |
| 22 | 737722ns | -1.0% | +3.7% | +49.5% | +49.1% | +39.1% |
| 23 | 737735ns | -1.3% | -1.7% | +50.1% | +49.3% | +38.7% |
| 24 | 737522ns | -1.7% | +1.8% | +50.7% | +49.4% | +38.4% |
| 25 | 737443ns | -1.4% | -0.9% | +79.8% | +48.9% | +38.9% |
| 26 | 733840ns | +0.3% | +1.5% | +50.6% | +50.5% | +39.8% |
| 27 | 731582ns | -0.4% | -1.8% | +50.3% | +50.8% | +41.1% |
| 28 | 747897ns | -1.4% | +0.1% | +54.9% | +47.2% | +37.3% |
| 29 | 732008ns | -0.4% | -1.1% | +51.5% | +50.8% | +40.5% |
| 30 | 733745ns | +0.4% | -1.4% | +49.8% | +49.7% | +40.1% |
| 31 | 740186ns | -0.5% | +2.7% | +83.2% | +48.0% | +38.9% |
| 32 | 733555ns | -0.1% | -2.1% | +58.4% | +49.4% | +39.8% |
| 33 | 730529ns | +0.3% | -2.2% | +55.8% | +50.0% | +40.4% |
| 34 | 734546ns | -0.3% | -2.8% | +49.4% | +50.7% | +39.7% |
| 35 | 736235ns | -0.5% | -2.9% | +53.0% | +48.9% | +39.5% |
| 36 | 736668ns | +1.5% | -3.0% | +46.0% | +48.8% | +39.2% |
| 37 | 736002ns | +2.3% | -3.0% | +46.1% | +48.6% | +41.1% |
| 38 | 752182ns | -1.3% | -5.1% | +89.4% | +46.1% | +36.7% |
| 39 | 742012ns | -0.8% | +11.0% | +63.1% | +48.0% | +38.0% |
| 40 | 744526ns | -1.1% | -2.3% | +66.1% | +47.2% | +37.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.367 | moderate+ |
| bitpack-carrier-d16-control | 0.807 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d32 | 0.617 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d64 | 0.472 | moderate+ |
| bitpack-carrier-packed | 0.655 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed-simd | 0.475 | moderate+ |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 24/40, lost 14/40
- **bitpack-carrier-d32**: won 18/40, lost 20/40
- **bitpack-carrier-d64**: won 0/40, lost 40/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 15.7ns | 746273.5ns | 0.0% |  |
| bitpack-carrier-d16-control | 12.4ns | 744399.1ns | 0.0% |  |
| bitpack-carrier-d32 | 14.8ns | 766060.6ns | 0.0% |  |
| bitpack-carrier-d64 | 35.6ns | 1241938.0ns | 0.0% |  |
| bitpack-carrier-packed | 20.6ns | 1106071.2ns | 0.0% |  |
| bitpack-carrier-packed-simd | 13.4ns | 1028421.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 730140.4-778368.5 ns)
  730140.4 |####################
  732551.8 |########################################
  734963.2 |##########################
  737374.6 |##########################
  739786.0 |##########################
  742197.4 |######
  744608.8 |
  747020.2 |######
  749431.7 |######
  751843.1 |####################
  754254.5 |######
  756665.9 |######
  759077.3 |######
  761488.7 |
  763900.1 |
  766311.5 |
  768722.9 |######
  771134.3 |#############
  773545.7 |######
  775957.1 |
  (4 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 728763.3-785416.2 ns)
  728763.3 |########################################
  731596.0 |###################################
  734428.6 |########################################
  737261.3 |##########
  740093.9 |#####
  742926.6 |
  745759.2 |#####
  748591.8 |
  751424.5 |#####
  754257.1 |##########
  757089.8 |
  759922.4 |
  762755.1 |
  765587.7 |
  768420.3 |
  771253.0 |#####
  774085.6 |
  776918.3 |
  779750.9 |
  782583.5 |
  (4 below, 5 above range)

bitpack-carrier-d32 (n=40, range 715171.8-859828.4 ns)
  715171.8 |########################
  722404.6 |########################################
  729637.5 |########################
  736870.3 |################################
  744103.1 |########################
  751336.0 |
  758568.8 |########################
  765801.6 |
  773034.5 |########
  780267.3 |
  787500.1 |
  794733.0 |
  801965.8 |########
  809198.6 |########################
  816431.4 |########
  823664.3 |########
  830897.1 |
  838129.9 |########
  845362.8 |########
  852595.6 |########
  (6 below, 3 above range)

bitpack-carrier-d64 (n=40, range 1092428.9-1428576.4 ns)
  1092428.9 |########################################
  1109236.3 |#################
  1126043.6 |###########
  1142851.0 |###########
  1159658.4 |#####
  1176465.8 |
  1193273.1 |#####
  1210080.5 |###########
  1226887.9 |#####
  1243695.3 |
  1260502.6 |
  1277310.0 |#####
  1294117.4 |###########
  1310924.8 |###########
  1327732.1 |#####
  1344539.5 |#####
  1361346.9 |#################
  1378154.3 |#####
  1394961.6 |#####
  1411769.0 |#################
  (3 below, 3 above range)

bitpack-carrier-packed (n=40, range 1095931.7-1128582.6 ns)
  1095931.7 |#################
  1097564.3 |######################
  1099196.8 |########################################
  1100829.3 |##################################
  1102461.9 |######################
  1104094.4 |###########
  1105727.0 |#####
  1107359.5 |#####
  1108992.0 |#####
  1110624.6 |
  1112257.1 |
  1113889.7 |
  1115522.2 |
  1117154.8 |
  1118787.3 |
  1120419.8 |#####
  1122052.4 |#####
  1123684.9 |#####
  1125317.5 |
  1126950.0 |
  (4 below, 4 above range)

bitpack-carrier-packed-simd (n=40, range 1021750.2-1039095.6 ns)
  1021750.2 |####################
  1022617.4 |####################
  1023484.7 |####################
  1024352.0 |########################################
  1025219.3 |########################################
  1026086.5 |####################
  1026953.8 |########################################
  1027821.1 |########################################
  1028688.3 |
  1029555.6 |##########
  1030422.9 |##########
  1031290.2 |####################
  1032157.4 |
  1033024.7 |####################
  1033892.0 |
  1034759.2 |####################
  1035626.5 |##########
  1036493.8 |
  1037361.1 |
  1038228.3 |##########
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-carrier-d16-control**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **bitpack-carrier-d32**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed**: autocorrelation=0.66 (measurement drift or warm-up artifact)
