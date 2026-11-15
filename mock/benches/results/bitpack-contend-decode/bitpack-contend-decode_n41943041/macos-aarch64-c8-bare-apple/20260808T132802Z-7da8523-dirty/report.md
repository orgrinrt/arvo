# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe4 is fastest but the noisiest (CV 5.9%)

bitpack-contend-pipe4 wins on median (327.84 us) yet has the highest variance (CV 5.9%), while bitpack-contend-d16 is the steadiest (CV 3.1%, 375.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-packed-simd shows warm-up / thermal drift (autocorr +0.74)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-packed-simd} (38% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-packed-simd} with a 38% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 327840.4 ns median (-12.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 327840.4 ns, slowest 517872.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 380765ns | 376104ns | 370013ns | 377696ns | 400722ns | base |
| bitpack-contend-d16-control | 377132ns | 370452ns | 366319ns | 371920ns | 403580ns | -0.95% |
| bitpack-contend-packed-simd | 529621ns | 518121ns | 504586ns | 522377ns | 576389ns | +39.09% |
| bitpack-contend-pipe2 | 338961ns | 337912ns | 313267ns | 337441ns | 369215ns | -10.98% |
| bitpack-contend-pipe4 | 328957ns | 328194ns | 302126ns | 328128ns | 358277ns | -13.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 380282ns | 369643ns | 400126ns | base | 11.029 |
| bitpack-contend-d16-control | 376489ns | 365788ns | 402897ns | -1.00% | 11.141 |
| bitpack-contend-packed-simd | 529177ns | 503928ns | 576054ns | +39.15% | 7.926 |
| bitpack-contend-pipe2 | 338410ns | 312794ns | 368841ns | -11.01% | 12.394 |
| bitpack-contend-pipe4 | 328610ns | 301760ns | 357898ns | -13.59% | 12.764 |

## Performance model

- Peak throughput: **13.899 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.165 | 80.3% |
| bitpack-contend-d16-control | 11.347 | 81.6% |
| bitpack-contend-packed-simd | 8.099 | 58.3% |
| bitpack-contend-pipe2 | 12.439 | 89.5% |
| bitpack-contend-pipe4 | 12.794 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 380765ns | 380765ns | base |
| bitpack-contend-d16-control | 377132ns | 377132ns | -0.95% |
| bitpack-contend-packed-simd | 529621ns | 529621ns | +39.09% |
| bitpack-contend-pipe2 | 338961ns | 338961ns | -10.98% |
| bitpack-contend-pipe4 | 328957ns | 328957ns | -13.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 375657ns | base | --- | [373808, 380781] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 369650ns | -5902.7ns (-1.6%) | [-7697, -4714]ns | [368466, 373542] | YES | 0.0007 | 0.0007 | 0 |
| bitpack-contend-packed-simd | 517872ns | +139535.4ns (+37.1%) | [+135844, +148235]ns | [513510, 530506] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 337179ns | -41759.8ns (-11.1%) | [-48486, -36820]ns | [332102, 340549] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 327840ns | -51596.9ns (-13.7%) | [-58906, -46842]ns | [322306, 335449] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 368408ns | +9.2% | +37.0% | -15.2% | -7.8% |
| 2 | 368901ns | +0.2% | +36.7% | -10.5% | -2.1% |
| 3 | 374594ns | -2.0% | +34.7% | -11.3% | -4.3% |
| 4 | 371530ns | -1.4% | +37.5% | -17.6% | -11.2% |
| 5 | 375095ns | -1.8% | +37.4% | -14.9% | -10.2% |
| 6 | 375739ns | -1.9% | +38.8% | -13.1% | -13.5% |
| 7 | 374570ns | -2.0% | +35.8% | -18.9% | -12.2% |
| 8 | 374174ns | -1.8% | +37.3% | -18.5% | -14.0% |
| 9 | 371915ns | -1.3% | +44.9% | -15.5% | -19.8% |
| 10 | 371725ns | -0.5% | +46.1% | -8.8% | -19.7% |
| 11 | 370955ns | -1.3% | +36.0% | -6.7% | -18.6% |
| 12 | 372538ns | -1.4% | +36.8% | -8.7% | -17.2% |
| 13 | 377752ns | -2.4% | +35.9% | -11.1% | -20.5% |
| 14 | 373441ns | -1.0% | +38.5% | -10.8% | -18.7% |
| 15 | 382687ns | -3.3% | +35.9% | -11.1% | -20.6% |
| 16 | 379830ns | -2.4% | +36.3% | -10.5% | -14.8% |
| 17 | 387274ns | -3.5% | +32.7% | -13.1% | -14.5% |
| 18 | 369461ns | +1.1% | +39.6% | -8.8% | -16.4% |
| 19 | 383025ns | -3.7% | +32.0% | -13.7% | -20.9% |
| 20 | 378335ns | -2.6% | +33.4% | -12.3% | -19.6% |
| 21 | 383210ns | +4.4% | +31.0% | -16.5% | -6.5% |
| 22 | 377780ns | -0.2% | +35.8% | -11.0% | -4.7% |
| 23 | 373133ns | -2.1% | +38.8% | -6.1% | -12.4% |
| 24 | 375575ns | -1.9% | +44.3% | -12.7% | -13.8% |
| 25 | 369034ns | -1.3% | +51.0% | -7.6% | -12.8% |
| 26 | 377224ns | -3.4% | +52.2% | -13.2% | -12.6% |
| 27 | 372863ns | -1.3% | +45.6% | -13.8% | -14.2% |
| 28 | 369590ns | -0.8% | +40.6% | -4.0% | -11.9% |
| 29 | 389031ns | -5.9% | +34.0% | -11.6% | -18.2% |
| 30 | 369264ns | +1.6% | +46.8% | -6.9% | -12.6% |
| 31 | 416460ns | -3.5% | +26.4% | -19.0% | -14.5% |
| 32 | 395682ns | +3.2% | +41.6% | -13.1% | -9.0% |
| 33 | 374947ns | +10.9% | +47.7% | -9.6% | -11.1% |
| 34 | 385426ns | +5.4% | +44.4% | -10.8% | -13.3% |
| 35 | 400783ns | -7.1% | +46.6% | -9.4% | -14.6% |
| 36 | 405332ns | -7.4% | +51.0% | -7.7% | -16.6% |
| 37 | 385803ns | +1.7% | +57.3% | -2.9% | -12.3% |
| 38 | 397728ns | -2.6% | +34.4% | -4.6% | -14.9% |
| 39 | 381731ns | -1.3% | +33.0% | -1.0% | -9.0% |
| 40 | 408721ns | -3.1% | +22.6% | -7.9% | -11.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.290 | moderate+ |
| bitpack-contend-d16-control | 0.575 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.738 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe2 | 0.723 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe4 | 0.629 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 31/40, lost 9/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe2**: won 40/40, lost 0/40
- **bitpack-contend-pipe4**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 7.1ns | 380281.7ns | 0.0% |  |
| bitpack-contend-d16-control | 5.1ns | 376489.5ns | 0.0% |  |
| bitpack-contend-packed-simd | 6.0ns | 529176.6ns | 0.0% |  |
| bitpack-contend-pipe2 | 8.1ns | 338410.0ns | 0.0% |  |
| bitpack-contend-pipe4 | 5.3ns | 328610.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 369642.8-400126.3 ns)
  369642.8 |######
  371167.0 |##########################
  372691.2 |##########################
  374215.3 |########################################
  375739.5 |######
  377263.7 |####################
  378787.9 |######
  380312.0 |######
  381836.2 |####################
  383360.4 |
  384884.6 |#############
  386408.8 |######
  387932.9 |######
  389457.1 |
  390981.3 |
  392505.5 |
  394029.6 |
  395553.8 |######
  397078.0 |######
  398602.2 |
  (6 below, 4 above range)

bitpack-contend-d16-control (n=40, range 365788.1-402897.1 ns)
  365788.1 |########################################
  367643.6 |###############################
  369499.0 |######################
  371354.5 |####
  373209.9 |#############
  375065.4 |########
  376920.8 |####
  378776.3 |
  380631.7 |
  382487.2 |
  384342.6 |
  386198.1 |####
  388053.5 |
  389909.0 |
  391764.4 |####
  393619.9 |
  395475.3 |####
  397330.8 |
  399186.2 |####
  401041.7 |########
  (3 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 503927.8-576053.5 ns)
  503927.8 |########################################
  507534.1 |##########################
  511140.4 |##########################
  514746.7 |#################################
  518353.0 |##########################
  521959.2 |
  525565.5 |######
  529171.8 |
  532778.1 |######
  536384.4 |######
  539990.7 |##########################
  543597.0 |
  547203.3 |
  550809.5 |######
  554415.8 |#############
  558022.1 |######
  561628.4 |
  565234.7 |
  568841.0 |
  572447.3 |######
  (2 below, 3 above range)

bitpack-contend-pipe2 (n=40, range 312794.0-368840.7 ns)
  312794.0 |########
  315596.4 |
  318398.7 |################
  321201.0 |########
  324003.4 |########
  326805.7 |################
  329608.0 |################################
  332410.4 |########
  335212.7 |########################################
  338015.0 |########################################
  340817.3 |########
  343619.7 |########################################
  346422.0 |
  349224.3 |########
  352026.7 |
  354829.0 |########
  357631.3 |
  360433.7 |########
  363236.0 |
  366038.3 |
  (4 below, 5 above range)

bitpack-contend-pipe4 (n=40, range 301759.9-357898.1 ns)
  301759.9 |########################################
  304566.8 |
  307373.8 |################
  310180.7 |
  312987.6 |
  315794.5 |########
  318601.4 |########
  321408.3 |########################################
  324215.2 |########################
  327022.1 |################
  329829.0 |################
  332635.9 |################
  335442.8 |################
  338249.8 |########################
  341056.7 |########
  343863.6 |
  346670.5 |########
  349477.4 |
  352284.3 |
  355091.2 |########
  (3 below, 6 above range)

```

## Diagnostics

- **bitpack-contend-d16-control**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe2**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: autocorrelation=0.63 (measurement drift or warm-up artifact)
