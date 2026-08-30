# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-control dominates: 10% faster than the next best (bitpack-contend-d16)

bitpack-contend-d16-control (195.38 us) leads bitpack-contend-d16 (215.71 us) by 10%, a clear separation rather than a photo finish. CV 6.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d64 is an outlier: 2.9x slower than the field

bitpack-contend-d64 (564.18 us) is 2.9x the fastest (195.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.92)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.92, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd, bitpack-contend-packed} vs {bitpack-contend-d64} (76% apart)

The field splits into a fast tier {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd, bitpack-contend-packed} and a slow tier {bitpack-contend-d64} with a 76% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 195382.3 ns median (-9.4% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 2.89x (fastest 195382.3 ns, slowest 564176.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 210976ns | 216277ns | 182720ns | 214720ns | 227998ns | base |
| bitpack-contend-d16-control | 200204ns | 195800ns | 186235ns | 197257ns | 223016ns | -5.11% |
| bitpack-contend-d32 | 270550ns | 268599ns | 256997ns | 268319ns | 290795ns | +28.24% |
| bitpack-contend-d64 | 582806ns | 565605ns | 542528ns | 568433ns | 666202ns | +176.24% |
| bitpack-contend-packed | 309037ns | 321313ns | 273325ns | 312470ns | 334449ns | +46.48% |
| bitpack-contend-packed-simd | 284997ns | 288257ns | 259368ns | 285485ns | 309163ns | +35.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 210597ns | 182582ns | 227514ns | base | 19.916 |
| bitpack-contend-d16-control | 199815ns | 185952ns | 222572ns | -5.12% | 20.991 |
| bitpack-contend-d32 | 268932ns | 255158ns | 289446ns | +27.70% | 15.596 |
| bitpack-contend-d64 | 581189ns | 540212ns | 664794ns | +175.97% | 7.217 |
| bitpack-contend-packed | 308546ns | 273064ns | 333829ns | +46.51% | 13.594 |
| bitpack-contend-packed-simd | 284519ns | 258894ns | 308566ns | +35.10% | 14.742 |

## Performance model

- Peak throughput: **22.972 Gops/s** (bitpack-contend-d16; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 19.444 | 84.6% |
| bitpack-contend-d16-control | 21.467 | 93.4% |
| bitpack-contend-d32 | 15.692 | 68.3% |
| bitpack-contend-d64 | 7.434 | 32.4% |
| bitpack-contend-packed | 13.065 | 56.9% |
| bitpack-contend-packed-simd | 14.575 | 63.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 210976ns | 210976ns | base |
| bitpack-contend-d16-control | 200204ns | 200204ns | -5.11% |
| bitpack-contend-d32 | 270550ns | 270550ns | +28.24% |
| bitpack-contend-d64 | 582806ns | 582806ns | +176.24% |
| bitpack-contend-packed | 309037ns | 309037ns | +46.48% |
| bitpack-contend-packed-simd | 284997ns | 284997ns | +35.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 215715ns | base | --- | [211999, 220201] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 195382ns | no significant difference | [-18125, +581]ns | [193458, 200164] | no | 0.0807 | 0.0807 | 0 |
| bitpack-contend-d32 | 267292ns | +53546.2ns (+24.8%) | [+48349, +61562]ns | [263539, 269553] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 564176ns | +359613.3ns (+166.7%) | [+334538, +380712]ns | [547246, 587180] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 321044ns | +99767.9ns (+46.2%) | [+93968, +106245]ns | [305379, 324670] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 287772ns | +78761.1ns (+36.5%) | [+74765, +83330]ns | [267645, 302566] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 195760ns | -3.5% | +41.7% | +202.9% | +39.7% | +36.8% |
| 2 | 201103ns | -6.3% | +32.4% | +175.9% | +35.5% | +33.1% |
| 3 | 183006ns | +2.0% | +53.8% | +305.4% | +50.1% | +45.9% |
| 4 | 182464ns | +3.0% | +46.5% | +261.5% | +50.3% | +45.8% |
| 5 | 182485ns | +7.2% | +43.2% | +214.4% | +51.6% | +45.9% |
| 6 | 182550ns | +6.4% | +45.8% | +214.4% | +51.5% | +47.8% |
| 7 | 182665ns | +5.9% | +56.1% | +221.6% | +49.7% | +41.7% |
| 8 | 182586ns | +2.1% | +41.6% | +221.5% | +49.6% | +43.8% |
| 9 | 182439ns | +2.2% | +47.3% | +228.5% | +51.3% | +41.3% |
| 10 | 182461ns | +2.1% | +62.9% | +198.5% | +49.6% | +41.5% |
| 11 | 199317ns | -7.6% | +29.1% | +173.8% | +55.7% | +34.8% |
| 12 | 206078ns | -10.7% | +28.1% | +170.2% | +56.0% | +28.9% |
| 13 | 214702ns | -13.2% | +23.5% | +168.1% | +47.9% | +27.1% |
| 14 | 218031ns | -14.1% | +22.1% | +151.7% | +47.2% | +29.1% |
| 15 | 224520ns | -11.2% | +19.1% | +140.6% | +44.6% | +18.1% |
| 16 | 228141ns | -14.7% | +30.1% | +137.7% | +42.2% | +18.2% |
| 17 | 227214ns | +1.8% | +20.3% | +139.4% | +41.4% | +13.3% |
| 18 | 224140ns | -3.4% | +19.3% | +140.6% | +30.4% | +14.7% |
| 19 | 223473ns | -0.2% | +17.7% | +141.4% | +22.0% | +18.0% |
| 20 | 222407ns | +0.9% | +35.8% | +146.5% | +22.3% | +14.9% |
| 21 | 221263ns | -10.8% | +13.3% | +173.5% | +44.9% | +34.2% |
| 22 | 224468ns | -15.7% | +12.7% | +142.4% | +45.2% | +34.7% |
| 23 | 228735ns | -15.5% | +10.6% | +139.4% | +44.0% | +32.4% |
| 24 | 232155ns | -15.9% | +9.5% | +134.2% | +42.0% | +31.8% |
| 25 | 230735ns | -15.2% | +16.3% | +133.6% | +46.3% | +33.9% |
| 26 | 223972ns | -14.2% | +24.1% | +140.9% | +47.2% | +36.8% |
| 27 | 219138ns | -9.6% | +23.6% | +147.2% | +53.6% | +39.6% |
| 28 | 208610ns | -6.8% | +24.2% | +162.1% | +58.4% | +46.9% |
| 29 | 221795ns | -11.5% | +16.6% | +168.9% | +50.3% | +38.0% |
| 30 | 223244ns | -13.3% | +15.9% | +199.3% | +49.4% | +37.6% |
| 31 | 211969ns | +4.8% | +37.8% | +160.6% | +43.6% | +38.8% |
| 32 | 217175ns | +2.2% | +24.5% | +173.2% | +49.5% | +36.5% |
| 33 | 216185ns | +4.8% | +23.6% | +149.8% | +50.3% | +37.5% |
| 34 | 213084ns | +0.7% | +26.1% | +168.2% | +55.8% | +45.6% |
| 35 | 213574ns | -0.9% | +27.0% | +174.2% | +58.0% | +45.1% |
| 36 | 215451ns | -3.3% | +25.5% | +175.3% | +50.5% | +43.4% |
| 37 | 216426ns | -3.6% | +30.8% | +192.8% | +43.8% | +41.3% |
| 38 | 215978ns | -5.4% | +20.8% | +190.3% | +41.8% | +43.8% |
| 39 | 212341ns | -4.6% | +22.9% | +253.1% | +52.2% | +39.1% |
| 40 | 212028ns | -5.2% | +20.7% | +198.7% | +53.4% | +43.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.920 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.722 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | -0.058 | ok |
| bitpack-contend-d64 | 0.413 | moderate+ |
| bitpack-contend-packed | 0.809 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.878 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 26/40, lost 14/40
- **bitpack-contend-d32**: won 0/40, lost 40/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 3.8ns | 210596.8ns | 0.0% |  |
| bitpack-contend-d16-control | 3.7ns | 199815.2ns | 0.0% |  |
| bitpack-contend-d32 | 3.6ns | 268932.4ns | 0.0% |  |
| bitpack-contend-d64 | 10.7ns | 581189.1ns | 0.0% |  |
| bitpack-contend-packed | 3.5ns | 308546.2ns | 0.0% |  |
| bitpack-contend-packed-simd | 4.3ns | 284519.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 182582.2-227513.7 ns)
  182582.2 |####################
  184828.8 |
  187075.3 |
  189321.9 |
  191568.5 |
  193815.1 |######
  196061.6 |
  198308.2 |######
  200554.8 |######
  202801.4 |
  205048.0 |######
  207294.5 |######
  209541.1 |
  211787.7 |#################################
  214034.3 |##########################
  216280.8 |####################
  218527.4 |######
  220774.0 |####################
  223020.6 |########################################
  225267.2 |######
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 185952.1-222572.0 ns)
  185952.1 |########################################
  187783.1 |##########################
  189614.1 |
  191445.1 |######
  193276.1 |########################################
  195107.1 |##########################
  196938.1 |#############
  198769.1 |######
  200600.1 |######
  202431.1 |######
  204262.1 |######
  206093.1 |
  207924.0 |#############
  209755.0 |
  211586.0 |######
  213417.0 |######
  215248.0 |######
  217079.0 |
  218910.0 |
  220741.0 |#############
  (2 below, 4 above range)

bitpack-contend-d32 (n=40, range 255158.1-289445.5 ns)
  255158.1 |#####
  256872.4 |###########
  258586.8 |#################
  260301.2 |#################
  262015.6 |#####
  263729.9 |###########
  265444.3 |#################
  267158.7 |########################################
  268873.0 |###########
  270587.4 |###########
  272301.8 |#####
  274016.2 |
  275730.5 |#####
  277444.9 |#####
  279159.3 |
  280873.6 |#####
  282588.0 |#####
  284302.4 |#####
  286016.8 |
  287731.1 |
  (4 below, 4 above range)

bitpack-contend-d64 (n=40, range 540212.0-664793.6 ns)
  540212.0 |########################################
  546441.1 |############################
  552670.2 |###########
  558899.3 |
  565128.3 |
  571357.4 |######################
  577586.5 |
  583815.6 |#################
  590044.7 |#################
  596273.7 |###########
  602502.8 |#####
  608731.9 |
  614961.0 |
  621190.0 |#####
  627419.1 |###########
  633648.2 |
  639877.3 |
  646106.3 |
  652335.4 |
  658564.5 |#####
  (6 below, 3 above range)

bitpack-contend-packed (n=40, range 273063.6-333828.5 ns)
  273063.6 |########################################
  276101.9 |#############
  279140.1 |
  282178.4 |
  285216.6 |
  288254.9 |
  291293.1 |######
  294331.3 |
  297369.6 |
  300407.8 |
  303446.1 |#############
  306484.3 |
  309522.6 |#############
  312560.8 |
  315599.0 |######
  318637.3 |##########################
  321675.5 |##########################
  324713.8 |##########################
  327752.0 |##########################
  330790.3 |####################
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 258894.5-308566.5 ns)
  258894.5 |
  261378.1 |################
  263861.7 |################################
  266345.3 |################################
  268828.9 |################
  271312.5 |########
  273796.1 |
  276279.7 |
  278763.3 |
  281246.9 |########
  283730.5 |
  286214.1 |
  288697.7 |
  291181.3 |
  293664.9 |################
  296148.5 |########################
  298632.1 |
  301115.7 |################
  303599.3 |########################################
  306082.9 |########################
  (6 below, 5 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.92 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.88 (measurement drift or warm-up artifact)
