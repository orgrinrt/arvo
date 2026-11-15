# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 is an outlier: 2.6x slower than the field

bitpack-contend-d64 (1.15 ms) is 2.6x the fastest (444.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-packed shows warm-up / thermal drift (autocorr +0.86)

bitpack-contend-packed's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-packed-simd, bitpack-contend-d32, bitpack-contend-packed} vs {bitpack-contend-d64} (82% apart)

The field splits into a fast tier {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-packed-simd, bitpack-contend-d32, bitpack-contend-packed} and a slow tier {bitpack-contend-d64} with a 82% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-d32 is inconsistent: worst-20% is 1.5x its best-20%

bitpack-contend-d32's best 20% of batches run at 555.97 us but its worst 20% at 854.70 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 444903.1 ns median (-2.5% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.58x (fastest 444903.1 ns, slowest 1146756.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 454523ns | 457473ns | 403890ns | 457735ns | 495520ns | base |
| bitpack-contend-d16-control | 443733ns | 446272ns | 381821ns | 445055ns | 501678ns | -2.37% |
| bitpack-contend-d32 | 642573ns | 601644ns | 557923ns | 599592ns | 856165ns | +41.37% |
| bitpack-contend-d64 | 1186148ns | 1148455ns | 1090027ns | 1154069ns | 1378505ns | +160.97% |
| bitpack-contend-packed | 637402ns | 631116ns | 561223ns | 629625ns | 736914ns | +40.24% |
| bitpack-contend-packed-simd | 590557ns | 577667ns | 547104ns | 586581ns | 645941ns | +29.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 453332ns | 402458ns | 494448ns | base | 18.504 |
| bitpack-contend-d16-control | 442629ns | 380348ns | 500635ns | -2.36% | 18.952 |
| bitpack-contend-d32 | 641007ns | 555968ns | 854696ns | +41.40% | 13.087 |
| bitpack-contend-d64 | 1184354ns | 1087935ns | 1376833ns | +161.26% | 7.083 |
| bitpack-contend-packed | 636408ns | 560321ns | 735693ns | +40.38% | 13.181 |
| bitpack-contend-packed-simd | 589700ns | 546401ns | 645178ns | +30.08% | 14.225 |

## Performance model

- Peak throughput: **22.055 Gops/s** (bitpack-contend-d16-control; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 18.386 | 83.4% |
| bitpack-contend-d16-control | 18.855 | 85.5% |
| bitpack-contend-d32 | 13.971 | 63.3% |
| bitpack-contend-d64 | 7.315 | 33.2% |
| bitpack-contend-packed | 13.316 | 60.4% |
| bitpack-contend-packed-simd | 14.544 | 65.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 454523ns | 454523ns | base |
| bitpack-contend-d16-control | 443733ns | 443733ns | -2.37% |
| bitpack-contend-d32 | 642573ns | 642573ns | +41.37% |
| bitpack-contend-d64 | 1186148ns | 1186148ns | +160.97% |
| bitpack-contend-packed | 637402ns | 637402ns | +40.24% |
| bitpack-contend-packed-simd | 590557ns | 590557ns | +29.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 456248ns | base | --- | [443463, 473981] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 444903ns | -15870.2ns (-3.5%) | [-28692, -5029]ns | [439524, 462493] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-d32 | 600450ns | +151660.2ns (+33.2%) | [+133147, +170448]ns | [576957, 618578] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 1146756ns | +712080.8ns (+156.1%) | [+670250, +749183]ns | [1110629, 1185183] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 629967ns | +178878.2ns (+39.2%) | [+153700, +200952]ns | [608654, 651355] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 576773ns | +138795.6ns (+30.4%) | [+123252, +146460]ns | [569269, 601350] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 406550ns | -6.6% | +42.6% | +188.1% | +38.6% | +32.1% |
| 2 | 396915ns | -4.5% | +41.7% | +184.5% | +48.7% | +41.8% |
| 3 | 394087ns | -4.1% | +55.1% | +181.7% | +41.9% | +43.2% |
| 4 | 384602ns | +3.1% | +55.8% | +188.4% | +45.4% | +36.4% |
| 5 | 412757ns | -7.9% | +35.2% | +162.3% | +36.0% | +32.1% |
| 6 | 434498ns | -12.3% | +30.2% | +163.0% | +28.8% | +27.7% |
| 7 | 437022ns | -13.1% | +39.3% | +174.5% | +27.9% | +29.4% |
| 8 | 422420ns | -9.5% | +32.2% | +163.0% | +32.7% | +33.0% |
| 9 | 404330ns | -2.0% | +59.2% | +184.7% | +38.5% | +39.3% |
| 10 | 419474ns | -8.6% | +384.9% | +176.2% | +34.5% | +24.3% |
| 11 | 507978ns | -10.3% | +18.4% | +177.9% | +19.2% | +21.8% |
| 12 | 486959ns | -9.5% | +33.2% | +169.6% | +27.7% | +32.7% |
| 13 | 513835ns | -7.7% | +22.5% | +133.2% | +24.2% | +21.1% |
| 14 | 491482ns | -3.5% | +16.8% | +183.2% | +47.0% | +29.7% |
| 15 | 476972ns | +1.7% | +39.9% | +147.9% | +53.8% | +32.6% |
| 16 | 456026ns | +2.1% | +44.0% | +185.1% | +59.9% | +39.1% |
| 17 | 456019ns | +7.1% | +75.5% | +175.3% | +59.8% | +44.1% |
| 18 | 457285ns | +11.0% | +47.7% | +247.9% | +63.1% | +38.8% |
| 19 | 478914ns | +7.9% | +31.1% | +186.8% | +64.8% | +36.7% |
| 20 | 493531ns | +2.0% | +38.9% | +173.0% | +47.9% | +34.0% |
| 21 | 467324ns | -7.1% | +19.6% | +175.2% | +48.0% | +32.1% |
| 22 | 473456ns | +0.4% | +16.7% | +160.5% | +42.8% | +23.8% |
| 23 | 473263ns | -3.7% | +18.0% | +134.1% | +34.8% | +22.0% |
| 24 | 478332ns | -6.4% | +23.0% | +128.1% | +31.7% | +21.8% |
| 25 | 490666ns | -10.2% | +34.5% | +119.7% | +27.1% | +16.7% |
| 26 | 452820ns | -2.8% | +33.5% | +146.3% | +42.1% | +25.5% |
| 27 | 429922ns | +2.1% | +42.1% | +168.4% | +52.2% | +34.2% |
| 28 | 434486ns | +1.6% | +32.7% | +148.9% | +52.2% | +33.4% |
| 29 | 449152ns | -8.8% | +34.3% | +144.1% | +44.4% | +28.4% |
| 30 | 474506ns | -14.5% | +19.9% | +139.7% | +28.9% | +30.0% |
| 31 | 481477ns | -3.7% | +19.9% | +125.2% | +46.8% | +18.8% |
| 32 | 448488ns | -0.6% | +40.5% | +149.5% | +56.4% | +25.9% |
| 33 | 474568ns | -1.6% | +31.9% | +133.6% | +37.9% | +18.8% |
| 34 | 456470ns | -2.8% | +44.2% | +154.4% | +38.9% | +24.2% |
| 35 | 444159ns | +3.9% | +26.2% | +169.2% | +41.8% | +28.4% |
| 36 | 400948ns | +34.9% | +38.5% | +196.3% | +56.7% | +42.2% |
| 37 | 442767ns | +10.2% | +30.5% | +149.9% | +41.1% | +31.2% |
| 38 | 478779ns | -2.1% | +15.7% | +130.3% | +21.3% | +20.4% |
| 39 | 489657ns | -9.1% | +12.7% | +126.8% | +16.3% | +29.8% |
| 40 | 460365ns | -3.4% | +25.6% | +136.1% | +23.1% | +37.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.684 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.751 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.016 | ok |
| bitpack-contend-d64 | 0.583 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.857 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.708 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 27/40, lost 13/40
- **bitpack-contend-d32**: won 0/40, lost 40/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 7.5ns | 453331.5ns | 0.0% |  |
| bitpack-contend-d16-control | 7.6ns | 442628.8ns | 0.0% |  |
| bitpack-contend-d32 | 13.3ns | 641007.1ns | 0.0% |  |
| bitpack-contend-d64 | 17.3ns | 1184354.0ns | 0.0% |  |
| bitpack-contend-packed | 12.3ns | 636407.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 6.4ns | 589700.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 402457.9-494448.0 ns)
  402457.9 |####################
  407057.4 |
  411656.9 |##########
  416256.4 |##########
  420855.9 |##########
  425455.5 |##########
  430055.0 |####################
  434654.5 |##########
  439254.0 |##########
  443853.5 |##########
  448453.0 |##############################
  453052.5 |########################################
  457652.0 |##########
  462251.5 |
  466851.0 |##########
  471450.5 |########################################
  476050.0 |########################################
  480649.5 |##########
  485249.0 |####################
  489848.5 |##############################
  (4 below, 2 above range)

bitpack-contend-d16-control (n=40, range 380348.3-500635.1 ns)
  380348.3 |#################
  386362.6 |
  392377.0 |###########
  398391.3 |
  404405.6 |###########
  410420.0 |
  416434.3 |
  422448.7 |
  428463.0 |#####
  434477.3 |###########
  440491.7 |########################################
  446506.0 |#####
  452520.4 |###########
  458534.7 |###########
  464549.1 |#################
  470563.4 |#################
  476577.7 |
  482592.1 |#################
  488606.4 |
  494620.8 |
  (5 below, 4 above range)

bitpack-contend-d32 (n=40, range 555968.4-854695.7 ns)
  555968.4 |########################################
  570904.7 |##############################
  585841.1 |##########
  600777.5 |##############################
  615713.8 |####################
  630650.2 |#####
  645586.6 |####################
  660522.9 |##########
  675459.3 |#####
  690395.7 |
  705332.0 |
  720268.4 |
  735204.8 |
  750141.1 |
  765077.5 |
  780013.9 |
  794950.2 |#####
  809886.6 |
  824823.0 |
  839759.3 |
  (4 below, 1 above range)

bitpack-contend-d64 (n=40, range 1087935.4-1376832.9 ns)
  1087935.4 |########
  1102380.2 |########################################
  1116825.1 |########
  1131270.0 |########
  1145714.9 |#############
  1160159.7 |########
  1174604.6 |########
  1189049.5 |#############
  1203494.4 |
  1217939.2 |
  1232384.1 |####
  1246829.0 |####
  1261273.9 |
  1275718.7 |####
  1290163.6 |####
  1304608.5 |####
  1319053.4 |
  1333498.2 |####
  1347943.1 |
  1362388.0 |####
  (5 below, 3 above range)

bitpack-contend-packed (n=40, range 560321.4-735693.2 ns)
  560321.4 |#################################
  569090.0 |######
  577858.5 |######
  586627.1 |######
  595395.7 |
  604164.3 |#############
  612932.9 |
  621701.5 |########################################
  630470.1 |####################
  639238.7 |######
  648007.3 |####################
  656775.9 |######
  665544.5 |
  674313.1 |######
  683081.7 |######
  691850.3 |
  700618.9 |#############
  709387.5 |
  718156.0 |######
  726924.6 |##########################
  (5 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 546400.6-645178.3 ns)
  546400.6 |
  551339.5 |#####
  556278.3 |
  561217.2 |########################################
  566156.1 |######################
  571095.0 |###########
  576033.9 |##################################
  580972.8 |#####
  585911.7 |#####
  590850.5 |
  595789.4 |
  600728.3 |
  605667.2 |
  610606.1 |
  615545.0 |#################
  620483.9 |#####
  625422.7 |
  630361.6 |######################
  635300.5 |###########
  640239.4 |
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.68 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: CV=35.6% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.71 (measurement drift or warm-up artifact)
