# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-control shows warm-up / thermal drift (autocorr +0.61)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} vs {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d64} (33% apart)

The field splits into a fast tier {bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} and a slow tier {bitpack-contend-packed-simd, bitpack-contend-packed, bitpack-contend-d64} with a 33% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 365851.2 ns median (-1.5% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.64x (fastest 365851.2 ns, slowest 599350.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 374688ns | 371735ns | 367879ns | 372671ns | 387550ns | base |
| bitpack-contend-d16-control | 366267ns | 366503ns | 359264ns | 366272ns | 373253ns | -2.25% |
| bitpack-contend-d32 | 388744ns | 381725ns | 372075ns | 386378ns | 412510ns | +3.75% |
| bitpack-contend-d64 | 601787ns | 600809ns | 549669ns | 597396ns | 667076ns | +60.61% |
| bitpack-contend-packed | 541840ns | 540685ns | 535187ns | 540676ns | 551985ns | +44.61% |
| bitpack-contend-packed-simd | 512326ns | 507585ns | 503288ns | 508495ns | 532857ns | +36.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 374152ns | 366936ns | 387193ns | base | 11.210 |
| bitpack-contend-d16-control | 365762ns | 358781ns | 372813ns | -2.24% | 11.467 |
| bitpack-contend-d32 | 386976ns | 369882ns | 411085ns | +3.43% | 10.839 |
| bitpack-contend-d64 | 600057ns | 547329ns | 665629ns | +60.38% | 6.990 |
| bitpack-contend-packed | 541208ns | 534472ns | 551422ns | +44.65% | 7.750 |
| bitpack-contend-packed-simd | 511749ns | 502734ns | 532201ns | +36.78% | 8.196 |

## Performance model

- Peak throughput: **11.690 Gops/s** (bitpack-contend-d16-control; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.295 | 96.6% |
| bitpack-contend-d16-control | 11.465 | 98.1% |
| bitpack-contend-d32 | 11.038 | 94.4% |
| bitpack-contend-d64 | 6.998 | 59.9% |
| bitpack-contend-packed | 7.763 | 66.4% |
| bitpack-contend-packed-simd | 8.273 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 374688ns | 374688ns | base |
| bitpack-contend-d16-control | 366267ns | 366267ns | -2.25% |
| bitpack-contend-d32 | 388744ns | 388744ns | +3.75% |
| bitpack-contend-d64 | 601787ns | 601787ns | +60.61% |
| bitpack-contend-packed | 541840ns | 541840ns | +44.61% |
| bitpack-contend-packed-simd | 512326ns | 512326ns | +36.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 371348ns | base | --- | [370648, 374006] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 365851ns | -3760.1ns (-1.0%) | [-10310, -2095]ns | [363344, 368126] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d32 | 380001ns | +7649.3ns (+2.1%) | [+1385, +28044]ns | [371962, 398150] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-contend-d64 | 599351ns | +225346.4ns (+60.7%) | [+209124, +239253]ns | [583431, 611295] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 540275ns | +168181.2ns (+45.3%) | [+165954, +169797]ns | [537576, 542375] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 506986ns | +134638.1ns (+36.3%) | [+131842, +139872]ns | [505079, 509972] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 380280ns | -5.2% | +0.4% | +60.6% | +41.9% | +33.4% |
| 2 | 376494ns | -4.7% | -2.0% | +52.4% | +43.8% | +35.0% |
| 3 | 377348ns | -5.0% | -2.1% | +58.7% | +43.4% | +40.5% |
| 4 | 400535ns | -10.5% | -7.9% | +41.9% | +35.4% | +28.5% |
| 5 | 408540ns | -12.3% | -9.3% | +32.1% | +32.2% | +23.7% |
| 6 | 379408ns | -4.9% | -0.4% | +51.1% | +46.8% | +32.8% |
| 7 | 379700ns | -5.5% | -2.8% | +44.3% | +49.0% | +33.2% |
| 8 | 373480ns | -4.0% | +4.6% | +44.6% | +44.7% | +35.3% |
| 9 | 370879ns | -3.3% | +10.0% | +60.9% | +45.1% | +36.2% |
| 10 | 377000ns | -3.7% | +5.0% | +65.0% | +44.8% | +34.0% |
| 11 | 386195ns | -5.6% | -4.1% | +38.8% | +44.6% | +36.1% |
| 12 | 370929ns | -3.0% | +8.5% | +60.4% | +44.8% | +35.6% |
| 13 | 373173ns | +0.3% | -0.6% | +47.9% | +43.2% | +34.7% |
| 14 | 368488ns | +0.6% | +0.8% | +51.6% | +45.4% | +38.4% |
| 15 | 370676ns | +0.0% | +7.7% | +50.9% | +43.9% | +37.8% |
| 16 | 372401ns | +0.9% | -0.1% | +48.1% | +43.1% | +39.9% |
| 17 | 371347ns | -0.4% | -0.0% | +66.0% | +43.6% | +41.2% |
| 18 | 372198ns | -0.3% | +0.7% | +72.4% | +45.2% | +60.6% |
| 19 | 374531ns | -0.2% | -1.0% | +55.3% | +42.7% | +38.1% |
| 20 | 371024ns | -2.5% | +0.1% | +76.4% | +44.7% | +37.5% |
| 21 | 367158ns | -0.7% | +7.7% | +66.7% | +46.4% | +38.7% |
| 22 | 366748ns | -1.1% | +10.5% | +63.6% | +46.1% | +39.6% |
| 23 | 366521ns | +0.1% | +2.7% | +92.2% | +47.1% | +42.4% |
| 24 | 368237ns | -0.8% | +1.9% | +59.0% | +45.5% | +36.6% |
| 25 | 365694ns | +0.4% | +8.9% | +65.1% | +46.4% | +38.6% |
| 26 | 375160ns | -3.1% | +6.1% | +56.3% | +43.3% | +35.1% |
| 27 | 385539ns | -3.5% | -3.9% | +58.3% | +42.4% | +30.9% |
| 28 | 368134ns | +0.2% | +1.0% | +59.6% | +45.7% | +36.7% |
| 29 | 370422ns | -0.7% | +0.3% | +69.2% | +45.8% | +36.2% |
| 30 | 366414ns | -1.0% | +12.1% | +67.9% | +46.3% | +36.7% |
| 31 | 370619ns | -1.2% | +9.1% | +90.1% | +46.3% | +36.9% |
| 32 | 371612ns | -0.3% | +8.4% | +48.6% | +46.5% | +35.5% |
| 33 | 377053ns | -0.8% | +7.7% | +62.2% | +43.9% | +33.4% |
| 34 | 375858ns | -1.6% | +2.2% | +75.5% | +45.5% | +34.3% |
| 35 | 371349ns | -1.5% | +8.2% | +52.4% | +45.7% | +36.0% |
| 36 | 370467ns | -1.0% | +1.9% | +81.4% | +46.8% | +35.8% |
| 37 | 368900ns | -0.0% | +18.3% | +62.9% | +47.2% | +37.7% |
| 38 | 367711ns | -0.6% | +12.7% | +66.4% | +47.9% | +39.5% |
| 39 | 370746ns | -0.6% | +7.5% | +64.8% | +46.7% | +40.1% |
| 40 | 367104ns | -0.3% | +9.8% | +80.3% | +48.2% | +40.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.547 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.610 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.318 | moderate+ |
| bitpack-contend-d64 | 0.039 | ok |
| bitpack-contend-packed | 0.387 | moderate+ |
| bitpack-contend-packed-simd | 0.249 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 32/40, lost 5/40
- **bitpack-contend-d32**: won 10/40, lost 28/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 6.8ns | 374151.8ns | 0.0% |  |
| bitpack-contend-d16-control | 5.0ns | 365762.3ns | 0.0% |  |
| bitpack-contend-d32 | 5.5ns | 386975.8ns | 0.0% |  |
| bitpack-contend-d64 | 7.7ns | 600057.4ns | 0.0% |  |
| bitpack-contend-packed | 4.5ns | 541207.6ns | 0.0% |  |
| bitpack-contend-packed-simd | 9.5ns | 511749.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 366935.6-387193.3 ns)
  366935.6 |#################
  367948.4 |######################
  368961.3 |
  369974.2 |########################################
  370987.1 |######################
  372000.0 |###########
  373012.9 |###########
  374025.8 |#####
  375038.6 |###########
  376051.5 |#################
  377064.4 |#####
  378077.3 |
  379090.2 |###########
  380103.1 |#####
  381116.0 |
  382128.8 |
  383141.7 |
  384154.6 |
  385167.5 |#####
  386180.4 |#####
  (4 below, 2 above range)

bitpack-contend-d16-control (n=40, range 358781.2-372813.2 ns)
  358781.2 |#############
  359482.8 |#############
  360184.4 |##########################
  360886.0 |
  361587.6 |#############
  362289.2 |##########################
  362990.8 |##########################
  363692.4 |
  364394.0 |##########################
  365095.6 |########################################
  365797.2 |##########################
  366498.8 |########################################
  367200.4 |#############
  367902.0 |#############
  368603.6 |##########################
  369305.2 |#############
  370006.8 |########################################
  370708.4 |##########################
  371410.0 |#############
  372111.6 |
  (6 below, 4 above range)

bitpack-contend-d32 (n=40, range 369881.7-411085.1 ns)
  369881.7 |########################################
  371941.9 |####
  374002.1 |########
  376062.3 |############
  378122.4 |
  380182.6 |####
  382242.8 |####
  384302.9 |
  386363.1 |
  388423.3 |
  390483.4 |####
  392543.6 |
  394603.8 |########
  396663.9 |############
  398724.1 |####
  400784.3 |########
  402844.4 |############
  404904.6 |########
  406964.8 |####
  409024.9 |####
  (4 below, 2 above range)

bitpack-contend-d64 (n=40, range 547329.4-665629.5 ns)
  547329.4 |##########################
  553244.4 |######
  559159.4 |######
  565074.4 |#############
  570989.4 |#############
  576904.4 |######
  582819.4 |####################
  588734.4 |
  594649.4 |##########################
  600564.4 |#############
  606479.4 |########################################
  612394.5 |#############
  618309.5 |######
  624224.5 |######
  630139.5 |
  636054.5 |######
  641969.5 |
  647884.5 |
  653799.5 |######
  659714.5 |#############
  (3 below, 3 above range)

bitpack-contend-packed (n=40, range 534472.1-551422.4 ns)
  534472.1 |##########
  535319.7 |########################################
  536167.2 |##############################
  537014.7 |##############################
  537862.2 |##########
  538709.7 |####################
  539557.2 |##############################
  540404.7 |####################
  541252.3 |####################
  542099.8 |########################################
  542947.3 |##########
  543794.8 |########################################
  544642.3 |
  545489.8 |##########
  546337.3 |##########
  547184.9 |
  548032.4 |##########
  548879.9 |
  549727.4 |
  550574.9 |
  (4 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 502734.1-532201.0 ns)
  502734.1 |###################################
  504207.5 |########################################
  505680.8 |###############
  507154.2 |####################
  508627.5 |###############
  510100.8 |#####
  511574.2 |##########
  513047.5 |
  514520.9 |#####
  515994.2 |##########
  517467.6 |
  518940.9 |#####
  520414.3 |##########
  521887.6 |
  523361.0 |#####
  524834.3 |#####
  526307.7 |
  527781.0 |
  529254.4 |#####
  530727.7 |
  (2 below, 1 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.61 (measurement drift or warm-up artifact)
