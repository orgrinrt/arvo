# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe2 beats baseline by 25% (significant)

bitpack-contend-pipe2 is -81.91 us (25%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-pipe2 is fastest but the noisiest (CV 38.9%)

bitpack-contend-pipe2 wins on median (263.96 us) yet has the highest variance (CV 38.9%), while bitpack-contend-packed-simd is the steadiest (CV 10.0%, 372.54 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.56)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-pipe2 is inconsistent: worst-20% is 2.1x its best-20%

bitpack-contend-pipe2's best 20% of batches run at 221.89 us but its worst 20% at 458.79 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 263964.0 ns median (-19.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.41x (fastest 263964.0 ns, slowest 372536.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 349181ns | 328176ns | 279588ns | 339671ns | 447305ns | base |
| bitpack-contend-d16-control | 333344ns | 308004ns | 279680ns | 324313ns | 414099ns | -4.54% |
| bitpack-contend-packed-simd | 380587ns | 373671ns | 340790ns | 375705ns | 435028ns | +8.99% |
| bitpack-contend-pipe2 | 302947ns | 264538ns | 223407ns | 276985ns | 460374ns | -13.24% |
| bitpack-contend-pipe4 | 296687ns | 288264ns | 229877ns | 288697ns | 387468ns | -15.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 348080ns | 278846ns | 445771ns | base | 24.100 |
| bitpack-contend-d16-control | 332151ns | 278902ns | 412127ns | -4.58% | 25.255 |
| bitpack-contend-packed-simd | 379803ns | 340138ns | 434137ns | +9.11% | 22.087 |
| bitpack-contend-pipe2 | 301708ns | 221885ns | 458789ns | -13.32% | 27.804 |
| bitpack-contend-pipe4 | 295846ns | 228927ns | 386517ns | -15.01% | 28.355 |

## Performance model

- Peak throughput: **37.806 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 25.636 | 67.8% |
| bitpack-contend-d16-control | 27.311 | 72.2% |
| bitpack-contend-packed-simd | 22.518 | 59.6% |
| bitpack-contend-pipe2 | 31.779 | 84.1% |
| bitpack-contend-pipe4 | 29.172 | 77.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 349181ns | 349181ns | base |
| bitpack-contend-d16-control | 333344ns | 333344ns | -4.54% |
| bitpack-contend-packed-simd | 380587ns | 380587ns | +8.99% |
| bitpack-contend-pipe2 | 302947ns | 302947ns | -13.24% |
| bitpack-contend-pipe4 | 296687ns | 296687ns | -15.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 327224ns | base | --- | [300340, 387502] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 307146ns | no significant difference | [-47429, +9443]ns | [292075, 361103] | no | 0.2682 | 0.2682 | 0 |
| bitpack-contend-packed-simd | 372536ns | no significant difference | [-9973, +76313]ns | [368958, 385590] | no | 0.2051 | 0.1539 | 0 |
| bitpack-contend-pipe2 | 263964ns | -51699.2ns (-15.8%) | [-87954, -14647]ns | [251635, 304872] | YES | 0.0332 | 0.0166 | 0 |
| bitpack-contend-pipe4 | 287555ns | -41708.6ns (-12.7%) | [-92295, -1986]ns | [256785, 312570] | YES | 0.0332 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 386033ns | -5.5% | -8.7% | -45.9% | -43.9% |
| 2 | 389119ns | -24.8% | -14.4% | -43.4% | -42.6% |
| 3 | 436280ns | -33.6% | -10.0% | -49.5% | -48.6% |
| 4 | 448505ns | -36.5% | -25.6% | -49.7% | -39.7% |
| 5 | 431793ns | -35.0% | -21.9% | -46.3% | -45.2% |
| 6 | 458869ns | -38.2% | -26.5% | -46.8% | -45.5% |
| 7 | 327862ns | -14.0% | +32.4% | -27.0% | -11.7% |
| 8 | 352908ns | -20.5% | +7.6% | -33.2% | -27.6% |
| 9 | 394558ns | -3.2% | -10.8% | -17.5% | -36.8% |
| 10 | 355697ns | +16.2% | -4.4% | -29.3% | -35.4% |
| 11 | 273689ns | +42.7% | +24.0% | +21.2% | +52.7% |
| 12 | 274520ns | +55.8% | +26.8% | +13.9% | +25.3% |
| 13 | 277395ns | +59.0% | +34.4% | -4.9% | +18.3% |
| 14 | 302127ns | +37.2% | +34.4% | -18.6% | +0.6% |
| 15 | 300470ns | +32.8% | +24.7% | -15.7% | +11.9% |
| 16 | 345252ns | +17.1% | +11.6% | -24.4% | -25.2% |
| 17 | 278043ns | +28.5% | +38.8% | +18.4% | -2.4% |
| 18 | 287333ns | -4.4% | +29.5% | -3.3% | -0.6% |
| 19 | 288023ns | -4.3% | +26.4% | -8.3% | -9.6% |
| 20 | 296282ns | -6.2% | +34.6% | +6.9% | -2.2% |
| 21 | 281521ns | -1.1% | +34.2% | +17.0% | +33.4% |
| 22 | 284608ns | -1.2% | +29.3% | +27.8% | -0.1% |
| 23 | 292727ns | -2.9% | +26.2% | +31.6% | -15.4% |
| 24 | 401115ns | -12.2% | -7.9% | -8.1% | -5.3% |
| 25 | 350451ns | -19.5% | +21.6% | +42.0% | -0.7% |
| 26 | 326587ns | +15.5% | +22.8% | +125.8% | +6.2% |
| 27 | 280747ns | +3.8% | +31.9% | +26.7% | +3.2% |
| 28 | 300210ns | +0.8% | +23.5% | -23.9% | +39.6% |
| 29 | 432731ns | -7.5% | -4.9% | -47.1% | -42.1% |
| 30 | 388972ns | -20.5% | +41.2% | -35.3% | -19.7% |
| 31 | 428667ns | -14.4% | -16.5% | -48.8% | -31.2% |
| 32 | 310836ns | +2.7% | +20.7% | -28.1% | +14.0% |
| 33 | 410632ns | -20.3% | -12.6% | -31.8% | -20.1% |
| 34 | 307857ns | +28.8% | +19.8% | -8.8% | +29.7% |
| 35 | 282688ns | +8.0% | +37.7% | +5.1% | +40.7% |
| 36 | 282160ns | +31.8% | +36.7% | -8.1% | -19.8% |
| 37 | 301482ns | +5.0% | +28.5% | -5.2% | -24.5% |
| 38 | 449080ns | -32.6% | -17.6% | -14.8% | -45.0% |
| 39 | 480246ns | -37.0% | -11.7% | +20.6% | -47.8% |
| 40 | 425119ns | -29.0% | -1.0% | -40.2% | -26.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.562 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.532 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.159 | ok |
| bitpack-contend-pipe2 | 0.474 | moderate+ |
| bitpack-contend-pipe4 | 0.267 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 24/40, lost 16/40
- **bitpack-contend-packed-simd**: won 15/40, lost 25/40
- **bitpack-contend-pipe2**: won 28/40, lost 12/40
- **bitpack-contend-pipe4**: won 28/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 112.3ns | 348079.9ns | 0.0% |  |
| bitpack-contend-d16-control | 41.3ns | 332151.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 16.4ns | 379802.7ns | 0.0% |  |
| bitpack-contend-pipe2 | 29.4ns | 301708.0ns | 0.0% |  |
| bitpack-contend-pipe4 | 15.8ns | 295846.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 278845.5-445771.5 ns)
  278845.5 |########################################
  287191.8 |########################
  295538.1 |########################################
  303884.4 |################
  312230.7 |
  320577.0 |################
  328923.3 |
  337269.6 |########
  345615.9 |################
  353962.2 |########
  362308.5 |
  370654.8 |
  379001.1 |########
  387347.4 |########################
  395693.7 |########
  404040.0 |########
  412386.3 |
  420732.6 |################
  429078.9 |########################
  437425.2 |
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 278902.3-412127.4 ns)
  278902.3 |########################################
  285563.6 |##########
  292224.8 |#####
  298886.1 |#########################
  305547.3 |#####
  312208.6 |#####
  318869.8 |#####
  325531.1 |#####
  332192.3 |
  338853.6 |
  345514.9 |
  352176.1 |##########
  358837.4 |#####
  365498.6 |##########
  372159.9 |#####
  378821.1 |#####
  385482.4 |#####
  392143.6 |#####
  398804.9 |###############
  405466.1 |
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 340137.6-434137.0 ns)
  340137.6 |
  344837.6 |#####
  349537.5 |##########
  354237.5 |##########
  358937.5 |
  363637.5 |##########
  368337.4 |########################################
  373037.4 |##########
  377737.4 |##########
  382437.3 |###############
  387137.3 |##########
  391837.3 |#####
  396537.2 |##########
  401237.2 |
  405937.2 |#####
  410637.1 |#####
  415337.1 |
  420037.1 |##########
  424737.0 |#####
  429437.0 |
  (6 below, 2 above range)

bitpack-contend-pipe2 (n=40, range 221885.0-458788.9 ns)
  221885.0 |########################################
  233730.2 |########################
  245575.4 |########################################
  257420.6 |################################
  269265.8 |########################
  281111.0 |########
  292956.2 |########
  304801.4 |########
  316646.6 |################
  328491.8 |########################
  340337.0 |
  352182.2 |################
  364027.4 |########
  375872.6 |################
  387717.8 |
  399562.9 |
  411408.1 |
  423253.3 |
  435098.5 |
  446943.7 |
  (4 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 228926.8-386517.0 ns)
  228926.8 |#############
  236806.3 |
  244685.8 |########################################
  252565.3 |####################
  260444.8 |
  268324.3 |#############
  276203.8 |
  284083.4 |#################################
  291962.9 |######
  299842.4 |######
  307721.9 |#############
  315601.4 |
  323480.9 |#############
  331360.4 |######
  339240.0 |#############
  347119.5 |#############
  354999.0 |
  362878.5 |
  370758.0 |######
  378637.5 |######
  (5 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe2**: CV=34.0% (high variance, measurements may be unstable)
