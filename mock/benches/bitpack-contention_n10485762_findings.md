# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd shows warm-up / thermal drift (autocorr +0.75)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} (29% apart)

The field splits into a fast tier {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-d32** at 44720.6 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.51x (fastest 44720.6 ns, slowest 67736.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 45616ns | 45447ns | 45322ns | 45439ns | 46439ns | base |
| bitpack-contend-d16-control | 45445ns | 45399ns | 45329ns | 45399ns | 45698ns | -0.38% |
| bitpack-contend-d32 | 45187ns | 44888ns | 44719ns | 44941ns | 46392ns | -0.94% |
| bitpack-contend-d64 | 59377ns | 58593ns | 55743ns | 59133ns | 63742ns | +30.17% |
| bitpack-contend-packed | 68131ns | 67920ns | 67775ns | 67952ns | 69026ns | +49.36% |
| bitpack-contend-packed-simd | 64409ns | 64024ns | 63812ns | 64061ns | 66046ns | +41.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 45482ns | 45212ns | 46263ns | base | 23.055 |
| bitpack-contend-d16-control | 45313ns | 45201ns | 45566ns | -0.37% | 23.141 |
| bitpack-contend-d32 | 45048ns | 44607ns | 46252ns | -0.95% | 23.277 |
| bitpack-contend-d64 | 59133ns | 55540ns | 63464ns | +30.01% | 17.733 |
| bitpack-contend-packed | 67951ns | 67645ns | 68773ns | +49.40% | 15.431 |
| bitpack-contend-packed-simd | 64252ns | 63683ns | 65881ns | +41.27% | 16.320 |

## Performance model

- Peak throughput: **23.507 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 23.146 | 98.5% |
| bitpack-contend-d16-control | 23.170 | 98.6% |
| bitpack-contend-d32 | 23.447 | 99.7% |
| bitpack-contend-d64 | 18.001 | 76.6% |
| bitpack-contend-packed | 15.480 | 65.9% |
| bitpack-contend-packed-simd | 16.419 | 69.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 45616ns | 45616ns | base |
| bitpack-contend-d16-control | 45445ns | 45445ns | -0.38% |
| bitpack-contend-d32 | 45187ns | 45187ns | -0.94% |
| bitpack-contend-d64 | 59377ns | 59377ns | +30.17% |
| bitpack-contend-packed | 68131ns | 68131ns | +49.36% |
| bitpack-contend-packed-simd | 64409ns | 64409ns | +41.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 45302ns | base | --- | [45256, 45361] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 45256ns | no significant difference | [-90, +2]ns | [45235, 45295] | no | 0.1539 | 0.1539 | 0 |
| bitpack-contend-d32 | 44721ns | -617.3ns (-1.4%) | [-644, -569]ns | [44675, 44884] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d64 | 58252ns | +12965.2ns (+28.6%) | [+11941, +14396]ns | [57364, 59668] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 67737ns | +22449.6ns (+49.6%) | [+22377, +22499]ns | [67698, 67829] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 63865ns | +18514.6ns (+40.9%) | [+18433, +18714]ns | [63757, 63991] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 45320ns | -0.2% | -1.5% | +28.2% | +49.4% | +40.5% |
| 2 | 45285ns | -0.1% | -1.5% | +27.0% | +49.4% | +40.7% |
| 3 | 45314ns | +0.3% | -1.4% | +26.2% | +49.4% | +41.0% |
| 4 | 45288ns | -0.0% | -1.4% | +26.4% | +49.4% | +41.4% |
| 5 | 45632ns | -0.6% | -2.1% | +23.3% | +48.7% | +39.6% |
| 6 | 47438ns | -4.7% | -5.7% | +21.6% | +42.7% | +34.2% |
| 7 | 45233ns | +0.0% | -1.3% | +27.3% | +49.6% | +40.8% |
| 8 | 45248ns | +0.0% | -1.4% | +27.6% | +49.6% | +40.8% |
| 9 | 45292ns | -0.2% | -1.6% | +26.3% | +49.6% | +41.3% |
| 10 | 45385ns | -0.3% | -1.4% | +24.8% | +49.4% | +40.4% |
| 11 | 45422ns | -0.4% | -1.3% | +21.8% | +48.8% | +40.5% |
| 12 | 45189ns | +0.1% | -0.2% | +22.2% | +49.8% | +40.9% |
| 13 | 45223ns | +0.1% | -0.8% | +25.5% | +54.3% | +40.9% |
| 14 | 45389ns | +1.7% | -1.6% | +21.4% | +49.9% | +40.3% |
| 15 | 45360ns | +0.2% | -1.4% | +23.7% | +51.9% | +43.2% |
| 16 | 45226ns | +0.2% | -1.2% | +32.1% | +49.7% | +45.7% |
| 17 | 45263ns | -0.0% | -1.4% | +23.6% | +51.1% | +45.6% |
| 18 | 45207ns | -0.0% | -1.2% | +35.5% | +49.7% | +47.3% |
| 19 | 45315ns | +1.1% | -1.4% | +34.2% | +52.5% | +47.7% |
| 20 | 45313ns | -0.2% | -1.3% | +27.8% | +50.2% | +46.6% |
| 21 | 45228ns | +0.4% | -1.3% | +39.4% | +54.4% | +41.4% |
| 22 | 46872ns | -3.6% | -3.9% | +43.2% | +45.1% | +36.6% |
| 23 | 47541ns | -4.7% | -1.9% | +32.0% | +42.6% | +34.1% |
| 24 | 45621ns | -0.2% | -1.6% | +36.8% | +48.7% | +39.8% |
| 25 | 45362ns | -0.3% | -1.4% | +39.3% | +50.0% | +40.8% |
| 26 | 45210ns | +0.1% | -1.2% | +38.2% | +50.0% | +40.8% |
| 27 | 46018ns | -1.7% | -1.6% | +36.0% | +47.9% | +38.7% |
| 28 | 45434ns | -0.4% | +0.8% | +38.8% | +49.7% | +40.4% |
| 29 | 45435ns | -0.3% | -0.9% | +38.9% | +49.2% | +40.3% |
| 30 | 45288ns | -0.1% | +2.9% | +38.7% | +49.4% | +41.0% |
| 31 | 45544ns | -0.7% | -2.0% | +18.3% | +48.5% | +41.3% |
| 32 | 45248ns | -0.1% | -1.3% | +24.8% | +50.0% | +45.7% |
| 33 | 45233ns | -0.0% | -1.4% | +25.7% | +49.6% | +42.3% |
| 34 | 45255ns | +0.1% | -0.6% | +29.0% | +49.6% | +42.4% |
| 35 | 45397ns | -0.2% | -1.8% | +31.3% | +49.0% | +40.7% |
| 36 | 45229ns | +0.0% | +1.5% | +31.3% | +49.7% | +41.6% |
| 37 | 45258ns | +0.1% | +2.3% | +31.2% | +49.5% | +41.0% |
| 38 | 45232ns | -0.1% | +2.6% | +31.2% | +49.8% | +41.3% |
| 39 | 45182ns | +0.5% | +0.7% | +31.6% | +49.8% | +41.6% |
| 40 | 45347ns | -0.1% | +3.4% | +38.0% | +49.2% | +40.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.291 | moderate+ |
| bitpack-contend-d16-control | 0.004 | ok |
| bitpack-contend-d32 | 0.354 | moderate+ |
| bitpack-contend-d64 | 0.645 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.080 | ok |
| bitpack-contend-packed-simd | 0.747 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 19/40, lost 11/40
- **bitpack-contend-d32**: won 33/40, lost 7/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.6ns | 45481.9ns | 0.0% |  |
| bitpack-contend-d16-control | 2.7ns | 45312.7ns | 0.0% |  |
| bitpack-contend-d32 | 2.4ns | 45048.4ns | 0.0% |  |
| bitpack-contend-d64 | 3.1ns | 59132.7ns | 0.0% |  |
| bitpack-contend-packed | 2.6ns | 67950.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 3.0ns | 64251.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 45211.7-46262.7 ns)
  45211.7 |########################################
  45264.3 |#######################
  45316.8 |#############
  45369.4 |#############
  45421.9 |######
  45474.5 |
  45527.0 |###
  45579.6 |######
  45632.1 |
  45684.7 |
  45737.2 |
  45789.8 |
  45842.3 |
  45894.9 |
  45947.4 |
  46000.0 |###
  46052.5 |
  46105.1 |
  46157.6 |
  46210.2 |
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 45201.3-45565.7 ns)
  45201.3 |
  45219.5 |########################################
  45237.7 |################
  45255.9 |################
  45274.2 |########
  45292.4 |########
  45310.6 |################
  45328.8 |
  45347.1 |####
  45365.3 |
  45383.5 |####
  45401.7 |####
  45419.9 |
  45438.2 |########
  45456.4 |
  45474.6 |
  45492.8 |
  45511.1 |####
  45529.3 |
  45547.5 |
  (6 below, 2 above range)

bitpack-contend-d32 (n=40, range 44606.7-46251.8 ns)
  44606.7 |########################################
  44689.0 |##########################
  44771.2 |###
  44853.5 |######
  44935.7 |###
  45018.0 |##########
  45100.3 |
  45182.5 |
  45264.8 |###
  45347.0 |
  45429.3 |
  45511.5 |###
  45593.8 |
  45676.1 |
  45758.3 |###
  45840.6 |
  45922.8 |###
  46005.1 |
  46087.3 |
  46169.6 |
  (4 below, 5 above range)

bitpack-contend-d64 (n=40, range 55540.4-63464.4 ns)
  55540.4 |
  55936.6 |##############################
  56332.8 |####################
  56729.0 |####################
  57125.2 |########################################
  57521.4 |##############################
  57917.6 |####################
  58313.8 |##########
  58710.0 |
  59106.2 |########################################
  59502.4 |####################
  59898.6 |
  60294.8 |
  60691.0 |##########
  61087.2 |##########
  61483.4 |
  61879.6 |
  62275.8 |########################################
  62672.0 |########################################
  63068.2 |####################
  (4 below, 1 above range)

bitpack-contend-packed (n=40, range 67644.9-68773.5 ns)
  67644.9 |########################################
  67701.3 |########################
  67757.7 |################
  67814.2 |################
  67870.6 |
  67927.0 |
  67983.5 |################
  68039.9 |########
  68096.3 |
  68152.8 |
  68209.2 |
  68265.6 |
  68322.1 |
  68378.5 |####
  68434.9 |
  68491.3 |
  68547.8 |
  68604.2 |
  68660.6 |
  68717.1 |
  (5 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 63683.0-65880.5 ns)
  63683.0 |########################################
  63792.8 |#####################
  63902.7 |############
  64012.6 |#########
  64122.5 |
  64232.4 |
  64342.2 |#########
  64452.1 |
  64562.0 |
  64671.9 |
  64781.7 |
  64891.6 |###
  65001.5 |
  65111.4 |
  65221.3 |
  65331.1 |
  65441.0 |
  65550.9 |
  65660.8 |
  65770.6 |
  (3 below, 6 above range)

```

## Diagnostics

- **bitpack-contend-d64**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.75 (measurement drift or warm-up artifact)
