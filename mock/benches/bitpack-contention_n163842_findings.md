# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 dominates: 11% faster than the next best (bitpack-contend-d16)

bitpack-contend-d64 (770 ns) leads bitpack-contend-d16 (858 ns) by 11%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d32 shows warm-up / thermal drift (autocorr +0.87)

bitpack-contend-d32's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d64, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} vs {bitpack-contend-packed-simd, bitpack-contend-packed} (31% apart)

The field splits into a fast tier {bitpack-contend-d64, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} and a slow tier {bitpack-contend-packed-simd, bitpack-contend-packed} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-d16-control's edge over baseline is significant but tiny (7 ns, 0.78%)

bitpack-contend-d16-control differs from baseline bitpack-contend-d16 by 7 ns (0.78%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-contend-d64** at 770.4 ns median (-10.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.67x (fastest 770.4 ns, slowest 1288.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 966ns | 924ns | 887ns | 939ns | 1127ns | base |
| bitpack-contend-d16-control | 933ns | 956ns | 888ns | 931ns | 986ns | -3.42% |
| bitpack-contend-d32 | 960ns | 938ns | 854ns | 946ns | 1108ns | -0.60% |
| bitpack-contend-d64 | 878ns | 833ns | 827ns | 855ns | 999ns | -9.07% |
| bitpack-contend-packed | 1331ns | 1357ns | 1217ns | 1326ns | 1460ns | +37.80% |
| bitpack-contend-packed-simd | 1194ns | 1203ns | 1164ns | 1189ns | 1238ns | +23.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 897ns | 824ns | 1046ns | base | 18.265 |
| bitpack-contend-d16-control | 862ns | 825ns | 906ns | -3.88% | 19.002 |
| bitpack-contend-d32 | 891ns | 791ns | 1028ns | -0.68% | 18.390 |
| bitpack-contend-d64 | 813ns | 764ns | 928ns | -9.31% | 20.141 |
| bitpack-contend-packed | 1264ns | 1156ns | 1386ns | +40.86% | 12.966 |
| bitpack-contend-packed-simd | 1131ns | 1103ns | 1172ns | +26.10% | 14.484 |

## Performance model

- Peak throughput: **21.433 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 19.107 | 89.1% |
| bitpack-contend-d16-control | 19.015 | 88.7% |
| bitpack-contend-d32 | 18.855 | 88.0% |
| bitpack-contend-d64 | 21.267 | 99.2% |
| bitpack-contend-packed | 12.713 | 59.3% |
| bitpack-contend-packed-simd | 14.369 | 67.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 966ns | 966ns | base |
| bitpack-contend-d16-control | 933ns | 933ns | -3.42% |
| bitpack-contend-d32 | 960ns | 960ns | -0.60% |
| bitpack-contend-d64 | 878ns | 878ns | -9.07% |
| bitpack-contend-packed | 1331ns | 1331ns | +37.80% |
| bitpack-contend-packed-simd | 1194ns | 1194ns | +23.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 858ns | base | --- | [829, 891] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 862ns | no significant difference | [-64, +61]ns | [830, 889] | no | 1.0000 | 1.0000 | 0 |
| bitpack-contend-d32 | 869ns | no significant difference | [-33, +122]ns | [798, 949] | no | 1.0000 | 1.0000 | 0 |
| bitpack-contend-d64 | 770ns | -92.3ns (-10.8%) | [-121, -57]ns | [768, 777] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 1289ns | +413.3ns (+48.2%) | [+329, +465]ns | [1209, 1291] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 1140ns | +265.2ns (+30.9%) | [+215, +278]ns | [1106, 1146] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 823ns | +8.7% | -3.6% | -6.4% | +40.6% | +38.5% |
| 2 | 832ns | +7.8% | -4.0% | -6.6% | +39.3% | +38.2% |
| 3 | 829ns | +19.1% | -3.5% | -6.7% | +39.5% | +38.5% |
| 4 | 830ns | +7.4% | -3.8% | -7.6% | +39.4% | +38.0% |
| 5 | 830ns | +7.3% | -5.2% | -7.2% | +39.5% | +46.7% |
| 6 | 825ns | +8.1% | -4.3% | -6.6% | +40.0% | +39.1% |
| 7 | 825ns | +7.8% | -3.8% | -6.4% | +40.3% | +38.2% |
| 8 | 824ns | +7.9% | -4.1% | -6.8% | +40.6% | +52.6% |
| 9 | 824ns | +7.9% | -3.7% | -6.9% | +40.8% | +38.5% |
| 10 | 826ns | +7.8% | -4.1% | -6.9% | +39.5% | +38.5% |
| 11 | 892ns | -6.1% | +15.5% | -13.3% | +55.1% | +24.2% |
| 12 | 890ns | -7.2% | +15.0% | -13.7% | +55.2% | +24.0% |
| 13 | 896ns | -7.4% | +13.9% | -14.7% | +54.1% | +23.5% |
| 14 | 889ns | -6.7% | +15.8% | -13.3% | +55.0% | +24.7% |
| 15 | 887ns | -6.6% | +16.4% | -13.5% | +55.7% | +24.4% |
| 16 | 889ns | -7.3% | +15.7% | -13.8% | +55.2% | +24.2% |
| 17 | 894ns | -7.7% | +13.7% | -14.4% | +54.7% | +23.5% |
| 18 | 892ns | -7.8% | +14.7% | -14.3% | +54.5% | +24.1% |
| 19 | 883ns | -7.1% | +16.1% | -13.1% | +56.5% | +29.3% |
| 20 | 885ns | -6.9% | +16.7% | -13.3% | +56.0% | +25.2% |
| 21 | 823ns | +8.4% | +16.7% | -6.6% | +56.7% | +33.9% |
| 22 | 824ns | +8.0% | +16.7% | -6.6% | +56.6% | +33.9% |
| 23 | 830ns | +7.2% | +14.8% | -8.3% | +55.3% | +33.2% |
| 24 | 829ns | +8.2% | +13.0% | +14.3% | +55.9% | +32.9% |
| 25 | 830ns | +7.4% | +13.8% | -6.5% | +55.8% | +33.3% |
| 26 | 822ns | +8.5% | +14.8% | +9.0% | +56.8% | +34.3% |
| 27 | 830ns | +7.2% | +13.7% | -7.4% | +55.6% | +33.0% |
| 28 | 827ns | +7.1% | +15.2% | +11.9% | +55.9% | +33.5% |
| 29 | 828ns | +7.3% | +12.9% | -7.9% | +55.9% | +32.9% |
| 30 | 825ns | +8.2% | +13.9% | +15.5% | +56.5% | +33.8% |
| 31 | 1048ns | -20.6% | -24.0% | -24.2% | +16.1% | +9.7% |
| 32 | 1043ns | -20.2% | -23.7% | -26.7% | +14.0% | +10.1% |
| 33 | 1046ns | -20.7% | -23.8% | -19.2% | +12.2% | +10.3% |
| 34 | 1045ns | -20.4% | -23.6% | -11.5% | +15.7% | +9.7% |
| 35 | 1048ns | -20.7% | -24.0% | -12.2% | +15.5% | +10.0% |
| 36 | 1040ns | -20.1% | -23.9% | -12.1% | +16.2% | +10.3% |
| 37 | 1046ns | -20.7% | -23.6% | -12.7% | +15.5% | +9.7% |
| 38 | 1048ns | -21.1% | -23.5% | -12.4% | +15.6% | +9.4% |
| 39 | 1040ns | -20.2% | -22.8% | -11.7% | +17.1% | +10.4% |
| 40 | 1045ns | -20.5% | -24.5% | -11.9% | +35.2% | +10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.864 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.716 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.870 | HIGH+ (drift/warm-up) |
| bitpack-contend-d64 | 0.382 | moderate+ |
| bitpack-contend-packed | 0.775 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.468 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 20/40, lost 20/40
- **bitpack-contend-d32**: won 20/40, lost 20/40
- **bitpack-contend-d64**: won 36/40, lost 4/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.7ns | 897.0ns | 0.3% |  |
| bitpack-contend-d16-control | 2.9ns | 862.2ns | 0.3% |  |
| bitpack-contend-d32 | 2.7ns | 890.9ns | 0.3% |  |
| bitpack-contend-d64 | 2.4ns | 813.5ns | 0.3% |  |
| bitpack-contend-packed | 2.5ns | 1263.6ns | 0.2% |  |
| bitpack-contend-packed-simd | 2.5ns | 1131.2ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 823.7-1046.1 ns)
    823.7 |########################################
    834.9 |
    846.0 |
    857.1 |
    868.2 |
    879.3 |################
    890.4 |##########
    901.6 |
    912.7 |
    923.8 |
    934.9 |
    946.0 |
    957.1 |
    968.3 |
    979.4 |
    990.5 |
   1001.6 |
   1012.7 |
   1023.8 |
   1035.0 |################
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 824.6-905.6 ns)
    824.6 |############
    828.7 |########################################
    832.7 |####
    836.8 |####
    840.8 |
    844.9 |
    848.9 |
    853.0 |
    857.0 |
    861.1 |
    865.1 |
    869.2 |
    873.2 |
    877.3 |
    881.3 |
    885.4 |########################
    889.4 |########################################
    893.4 |############
    897.5 |
    901.5 |
  (5 below, 1 above range)

bitpack-contend-d32 (n=40, range 790.7-1028.5 ns)
    790.7 |########################################
    802.6 |##
    814.4 |
    826.3 |
    838.2 |
    850.1 |
    862.0 |
    873.9 |
    885.8 |
    897.7 |
    909.6 |
    921.4 |
    933.3 |###############
    945.2 |#####
    957.1 |#####
    969.0 |
    980.9 |
    992.8 |
   1004.7 |##
   1016.6 |##########
  (3 below, 5 above range)

bitpack-contend-d64 (n=40, range 764.4-928.2 ns)
    764.4 |########################################
    772.6 |########
    780.8 |
    789.0 |##
    797.2 |
    805.4 |
    813.5 |
    821.7 |
    829.9 |
    838.1 |##
    846.3 |
    854.5 |
    862.7 |
    870.9 |
    879.0 |
    887.2 |
    895.4 |##
    903.6 |
    911.8 |########
    920.0 |########
  (3 below, 2 above range)

bitpack-contend-packed (n=40, range 1156.5-1385.5 ns)
   1156.5 |############################
   1168.0 |####
   1179.4 |####
   1190.9 |
   1202.3 |####################
   1213.8 |########
   1225.2 |
   1236.7 |
   1248.1 |
   1259.6 |
   1271.0 |
   1282.5 |########################################
   1293.9 |
   1305.4 |
   1316.8 |
   1328.3 |
   1339.7 |
   1351.2 |
   1362.6 |
   1374.1 |########################################
  (3 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 1102.7-1172.2 ns)
   1102.7 |########################################
   1106.2 |########################
   1109.6 |
   1113.1 |
   1116.6 |
   1120.1 |
   1123.5 |
   1127.0 |
   1130.5 |
   1134.0 |
   1137.4 |############
   1140.9 |########
   1144.4 |############################
   1147.9 |####################
   1151.3 |########
   1154.8 |
   1158.3 |
   1161.8 |
   1165.2 |
   1168.7 |
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.77 (measurement drift or warm-up artifact)
