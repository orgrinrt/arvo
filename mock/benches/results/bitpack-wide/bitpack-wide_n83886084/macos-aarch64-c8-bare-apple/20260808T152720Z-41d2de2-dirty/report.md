# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-d16-padal dominates: 21% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-d16-padal (250.71 us) leads bitpack-wide-d16-control (303.85 us) by 21%, a clear separation rather than a photo finish. CV 14.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 shows warm-up / thermal drift (autocorr +0.70)

bitpack-wide-pipe4's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-wide-pipe4 is inconsistent: worst-20% is 2.2x its best-20%

bitpack-wide-pipe4's best 20% of batches run at 241.64 us but its worst 20% at 528.17 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-wide-d16-padal** at 250712.5 ns median (-19.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.32x (fastest 250712.5 ns, slowest 331712.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 347581ns | 310930ns | 268590ns | 330373ns | 478199ns | base |
| bitpack-wide-d16-control | 318943ns | 304764ns | 261105ns | 311394ns | 399428ns | -8.24% |
| bitpack-wide-d16-padal | 263992ns | 251729ns | 235096ns | 254631ns | 320972ns | -24.05% |
| bitpack-wide-pipe4 | 364661ns | 333198ns | 242321ns | 350377ns | 529852ns | +4.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 346387ns | 267679ns | 476554ns | base | 24.217 |
| bitpack-wide-d16-control | 317761ns | 259806ns | 398002ns | -8.26% | 26.399 |
| bitpack-wide-d16-padal | 263020ns | 234405ns | 319808ns | -24.07% | 31.893 |
| bitpack-wide-pipe4 | 363336ns | 241638ns | 528174ns | +4.89% | 23.088 |

## Performance model

- Peak throughput: **35.787 Gops/s** (bitpack-wide-d16-padal; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 27.069 | 75.6% |
| bitpack-wide-d16-control | 27.608 | 77.1% |
| bitpack-wide-d16-padal | 33.459 | 93.5% |
| bitpack-wide-pipe4 | 25.289 | 70.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 347581ns | 347581ns | base |
| bitpack-wide-d16-control | 318943ns | 318943ns | -8.24% |
| bitpack-wide-d16-padal | 263992ns | 263992ns | -24.05% |
| bitpack-wide-pipe4 | 364661ns | 364661ns | +4.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 309892ns | base | --- | [295405, 373637] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 303850ns | -12775.4ns (-4.1%) | [-44458, -1917]ns | [285863, 336463] | YES (adj: no) | 0.1210 | 0.0807 | 0 |
| bitpack-wide-d16-padal | 250712ns | -54397.9ns (-17.6%) | [-85898, -32930]ns | [248625, 255917] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 331713ns | no significant difference | [-13682, +49747]ns | [290876, 413764] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 265951ns | -4.6% | -11.8% | +42.2% |
| 2 | 265201ns | -3.5% | -11.6% | +69.8% |
| 3 | 269858ns | -4.9% | -11.8% | +31.6% |
| 4 | 268926ns | -3.2% | +0.8% | +12.4% |
| 5 | 280909ns | +3.8% | -11.5% | +18.5% |
| 6 | 308757ns | +4.2% | -29.4% | +7.1% |
| 7 | 267755ns | +2.5% | -6.5% | +20.0% |
| 8 | 278378ns | -5.3% | -16.3% | +14.9% |
| 9 | 299558ns | -14.9% | -23.4% | +1.3% |
| 10 | 273726ns | +18.4% | -12.2% | +4.9% |
| 11 | 372541ns | -4.2% | -30.4% | -37.2% |
| 12 | 323022ns | -13.7% | -21.8% | -27.0% |
| 13 | 307579ns | -13.1% | -15.9% | -21.8% |
| 14 | 332782ns | -20.1% | -24.3% | -24.7% |
| 15 | 266897ns | +3.6% | -4.8% | +9.3% |
| 16 | 266621ns | +11.0% | -4.9% | +6.0% |
| 17 | 270225ns | +64.0% | -7.1% | -6.4% |
| 18 | 279334ns | +4.0% | -10.7% | -17.9% |
| 19 | 288394ns | +27.8% | -13.2% | -19.6% |
| 20 | 302414ns | +0.3% | -14.8% | -8.7% |
| 21 | 446798ns | -39.8% | -44.5% | -20.5% |
| 22 | 483916ns | -43.5% | -48.6% | +24.6% |
| 23 | 511388ns | -25.1% | -51.5% | -4.2% |
| 24 | 451796ns | -24.3% | -44.6% | -1.3% |
| 25 | 441638ns | -31.1% | -42.7% | +18.5% |
| 26 | 584122ns | -51.9% | -57.6% | +3.0% |
| 27 | 432345ns | -23.0% | -42.7% | +33.8% |
| 28 | 308056ns | -3.9% | -18.7% | +58.4% |
| 29 | 460426ns | -12.8% | -46.0% | -43.8% |
| 30 | 401824ns | -16.8% | -38.3% | +4.5% |
| 31 | 291252ns | +50.0% | -0.1% | -7.5% |
| 32 | 309445ns | +20.4% | -0.4% | -6.2% |
| 33 | 397335ns | -11.6% | -28.8% | -32.5% |
| 34 | 380956ns | -21.3% | -20.5% | +14.2% |
| 35 | 376508ns | -1.3% | +13.5% | +22.6% |
| 36 | 378563ns | -10.6% | -14.2% | +26.7% |
| 37 | 364950ns | +11.5% | -15.1% | +25.5% |
| 38 | 360275ns | -3.3% | -27.3% | +13.2% |
| 39 | 374733ns | -17.2% | -26.8% | -2.7% |
| 40 | 310339ns | +13.6% | +0.3% | +38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.668 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-control | 0.289 | moderate+ |
| bitpack-wide-d16-padal | 0.593 | HIGH+ (drift/warm-up) |
| bitpack-wide-pipe4 | 0.701 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 26/40, lost 14/40
- **bitpack-wide-d16-padal**: won 37/40, lost 3/40
- **bitpack-wide-pipe4**: won 16/40, lost 24/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 25.3ns | 346387.3ns | 0.0% |  |
| bitpack-wide-d16-control | 19.9ns | 317761.0ns | 0.0% |  |
| bitpack-wide-d16-padal | 11.8ns | 263020.0ns | 0.0% |  |
| bitpack-wide-pipe4 | 58.0ns | 363336.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 267679.1-476553.5 ns)
  267679.1 |#################################
  278122.8 |##########################
  288566.5 |######
  299010.3 |########################################
  309454.0 |######
  319897.7 |######
  330341.4 |######
  340785.1 |
  351228.9 |######
  361672.6 |######
  372116.3 |#################################
  382560.0 |
  393003.8 |#############
  403447.5 |
  413891.2 |
  424334.9 |######
  434778.6 |######
  445222.4 |#############
  455666.1 |######
  466109.8 |
  (4 below, 3 above range)

bitpack-wide-d16-control (n=40, range 259805.7-398002.4 ns)
  259805.7 |########################################
  266715.5 |########################################
  273625.3 |########################################
  280535.2 |#############
  287445.0 |##########################
  294354.9 |########################################
  301264.7 |##########################
  308174.5 |#############
  315084.4 |#############
  321994.2 |#############
  328904.0 |##########################
  335813.9 |##########################
  342723.7 |#############
  349633.6 |##########################
  356543.4 |#############
  363453.2 |#############
  370363.1 |##########################
  377272.9 |#############
  384182.7 |
  391092.6 |
  (4 below, 4 above range)

bitpack-wide-d16-padal (n=40, range 234405.4-319808.2 ns)
  234405.4 |########
  238675.6 |##
  242945.7 |
  247215.8 |########################################
  251486.0 |##############
  255756.1 |########
  260026.2 |##
  264296.4 |
  268566.5 |##
  272836.7 |##
  277106.8 |
  281376.9 |##
  285647.1 |
  289917.2 |##
  294187.4 |
  298457.5 |
  302727.6 |##
  306997.8 |#####
  311267.9 |##
  315538.0 |
  (3 below, 2 above range)

bitpack-wide-pipe4 (n=40, range 241638.1-528174.2 ns)
  241638.1 |##########################
  255964.9 |########################################
  270291.7 |##########################
  284618.5 |########################################
  298945.3 |##########################
  313272.1 |##########################
  327598.9 |##########################
  341925.7 |##########################
  356252.5 |#############
  370579.3 |#############
  384906.1 |
  399232.9 |#############
  413559.7 |#############
  427886.5 |##########################
  442213.3 |##########################
  456540.1 |##########################
  470866.9 |#############
  485193.8 |##########################
  499520.6 |
  513847.4 |#############
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-wide-d16**: CV=22.9% (high variance, measurements may be unstable)
- **bitpack-wide-d16**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-padal**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **bitpack-wide-pipe4**: CV=29.5% (high variance, measurements may be unstable)
- **bitpack-wide-pipe4**: autocorrelation=0.70 (measurement drift or warm-up artifact)
