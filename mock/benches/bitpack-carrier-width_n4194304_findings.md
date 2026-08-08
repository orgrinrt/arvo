# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d16-control shows warm-up / thermal drift (autocorr +0.61)

bitpack-carrier-d16-control's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-carrier-d16)

The baseline bitpack-carrier-d16 is the fastest (365.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bitpack-carrier-d16) is the fastest** at 365496.1 ns median
- 5 variants significantly slower than baseline
- Spread: 1.71x (fastest 365496.1 ns, slowest 626364.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 368292ns | 366754ns | 361571ns | 367154ns | 378428ns | base |
| bitpack-carrier-d16-control | 371307ns | 369746ns | 364555ns | 370078ns | 381745ns | +0.82% |
| bitpack-carrier-d32 | 412287ns | 406688ns | 376709ns | 404894ns | 470046ns | +11.95% |
| bitpack-carrier-d64 | 643312ns | 627930ns | 564910ns | 631884ns | 756000ns | +74.67% |
| bitpack-carrier-packed | 546419ns | 543205ns | 537021ns | 543980ns | 563135ns | +48.37% |
| bitpack-carrier-packed-simd | 507568ns | 506164ns | 502427ns | 506095ns | 517125ns | +37.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 367678ns | 361011ns | 377992ns | base | 11.408 |
| bitpack-carrier-d16-control | 370861ns | 364106ns | 381377ns | +0.87% | 11.310 |
| bitpack-carrier-d32 | 410984ns | 375251ns | 468764ns | +11.78% | 10.206 |
| bitpack-carrier-d64 | 641737ns | 562904ns | 754687ns | +74.54% | 6.536 |
| bitpack-carrier-packed | 545854ns | 536297ns | 562657ns | +48.46% | 7.684 |
| bitpack-carrier-packed-simd | 507039ns | 501870ns | 516687ns | +37.90% | 8.272 |

## Performance model

- Peak throughput: **11.618 Gops/s** (bitpack-carrier-d16; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.476 | 98.8% |
| bitpack-carrier-d16-control | 11.356 | 97.7% |
| bitpack-carrier-d32 | 10.351 | 89.1% |
| bitpack-carrier-d64 | 6.696 | 57.6% |
| bitpack-carrier-packed | 7.732 | 66.5% |
| bitpack-carrier-packed-simd | 8.295 | 71.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 368292ns | 368292ns | base |
| bitpack-carrier-d16-control | 371307ns | 371307ns | +0.82% |
| bitpack-carrier-d32 | 412287ns | 412287ns | +11.95% |
| bitpack-carrier-d64 | 643312ns | 643312ns | +74.67% |
| bitpack-carrier-packed | 546419ns | 546419ns | +48.37% |
| bitpack-carrier-packed-simd | 507568ns | 507568ns | +37.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 365496ns | base | --- | [363815, 369502] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 369356ns | +2444.7ns (+0.7%) | [+381, +5083]ns | [366242, 372867] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-carrier-d32 | 405220ns | +35100.0ns (+9.6%) | [+23613, +49300]ns | [394849, 412800] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 626364ns | +260316.2ns (+71.2%) | [+241165, +275056]ns | [612109, 644560] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 542471ns | +175190.2ns (+47.9%) | [+173519, +178789]ns | [540001, 546839] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 505631ns | +138731.2ns (+38.0%) | [+137666, +143414]ns | [503569, 507414] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 360387ns | +1.0% | +15.4% | +75.4% | +51.5% | +39.7% |
| 2 | 363155ns | +0.1% | +11.8% | +54.4% | +50.9% | +37.9% |
| 3 | 360488ns | +0.9% | +3.7% | +55.4% | +66.8% | +40.2% |
| 4 | 360852ns | +3.9% | +9.4% | +89.9% | +53.4% | +39.8% |
| 5 | 359868ns | +1.8% | +12.8% | +57.0% | +54.0% | +39.3% |
| 6 | 361980ns | +0.6% | +3.6% | +69.5% | +52.4% | +39.1% |
| 7 | 361785ns | +0.7% | +13.7% | +65.4% | +48.1% | +40.4% |
| 8 | 364827ns | -0.1% | +6.2% | +54.3% | +47.7% | +37.9% |
| 9 | 369214ns | -0.2% | +8.7% | +56.6% | +46.4% | +36.0% |
| 10 | 380905ns | -2.8% | +6.2% | +79.8% | +40.8% | +32.9% |
| 11 | 362031ns | +0.7% | +4.2% | +78.0% | +48.1% | +43.0% |
| 12 | 361744ns | +4.1% | +9.2% | +102.0% | +48.9% | +43.0% |
| 13 | 370772ns | +2.6% | +1.0% | +63.8% | +45.4% | +35.6% |
| 14 | 368218ns | +0.3% | +14.6% | +69.5% | +45.5% | +38.3% |
| 15 | 361804ns | +1.0% | +2.9% | +65.6% | +51.1% | +39.8% |
| 16 | 364762ns | +0.1% | +2.9% | +56.7% | +48.5% | +39.1% |
| 17 | 364422ns | +0.8% | +8.5% | +70.7% | +47.9% | +37.8% |
| 18 | 364996ns | +0.4% | +12.7% | +53.6% | +48.0% | +37.5% |
| 19 | 367677ns | -0.4% | +29.2% | +72.0% | +47.3% | +37.8% |
| 20 | 370947ns | +1.8% | +9.8% | +74.4% | +45.0% | +35.2% |
| 21 | 371909ns | +0.0% | +0.7% | +62.2% | +49.1% | +35.3% |
| 22 | 375660ns | -3.0% | +4.2% | +107.0% | +46.0% | +35.3% |
| 23 | 372738ns | -2.1% | +6.3% | +124.6% | +45.8% | +34.8% |
| 24 | 376075ns | -1.4% | +16.5% | +79.7% | +44.8% | +34.1% |
| 25 | 369790ns | -1.4% | +20.6% | +70.2% | +54.9% | +37.3% |
| 26 | 385966ns | -5.6% | -1.5% | +59.2% | +44.5% | +31.6% |
| 27 | 381834ns | -3.5% | +1.2% | +79.1% | +44.9% | +34.1% |
| 28 | 371305ns | -0.5% | +10.7% | +87.3% | +45.4% | +39.5% |
| 29 | 370519ns | +0.7% | +4.8% | +74.0% | +48.4% | +39.9% |
| 30 | 371324ns | +1.8% | +7.9% | +84.7% | +47.3% | +39.8% |
| 31 | 368330ns | +0.6% | +15.1% | +50.3% | +46.0% | +41.1% |
| 32 | 378848ns | -0.1% | +14.2% | +50.1% | +41.0% | +32.9% |
| 33 | 367324ns | +4.6% | +3.3% | +71.5% | +45.9% | +37.2% |
| 34 | 365645ns | +4.6% | +14.9% | +67.0% | +47.9% | +37.9% |
| 35 | 363207ns | +7.6% | +21.8% | +72.5% | +49.5% | +39.6% |
| 36 | 365347ns | +3.8% | +13.9% | +71.4% | +48.3% | +37.7% |
| 37 | 361184ns | +3.2% | +17.5% | +113.7% | +49.5% | +41.5% |
| 38 | 365160ns | +2.6% | +13.5% | +90.5% | +49.0% | +37.7% |
| 39 | 361779ns | +3.8% | +50.6% | +70.2% | +53.2% | +39.8% |
| 40 | 362344ns | +3.1% | +50.9% | +132.8% | +51.2% | +40.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.529 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d16-control | 0.609 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d32 | 0.398 | moderate+ |
| bitpack-carrier-d64 | 0.232 | moderate+ |
| bitpack-carrier-packed | 0.310 | moderate+ |
| bitpack-carrier-packed-simd | 0.512 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 12/40, lost 26/40
- **bitpack-carrier-d32**: won 1/40, lost 39/40
- **bitpack-carrier-d64**: won 0/40, lost 40/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 7.2ns | 367678.0ns | 0.0% |  |
| bitpack-carrier-d16-control | 9.2ns | 370860.7ns | 0.0% |  |
| bitpack-carrier-d32 | 16.7ns | 410984.2ns | 0.0% |  |
| bitpack-carrier-d64 | 11.6ns | 641736.6ns | 0.0% |  |
| bitpack-carrier-packed | 16.0ns | 545853.7ns | 0.0% |  |
| bitpack-carrier-packed-simd | 9.0ns | 507038.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 361010.8-377991.9 ns)
  361010.8 |########################################
  361859.9 |########################
  362708.9 |################
  363558.0 |
  364407.0 |########################################
  365256.1 |################
  366105.1 |
  366954.2 |################
  367803.2 |################
  368652.3 |########
  369501.4 |########
  370350.4 |########################
  371199.5 |########################
  372048.5 |########
  372897.6 |
  373746.6 |
  374595.7 |
  375444.7 |################
  376293.8 |
  377142.9 |
  (4 below, 4 above range)

bitpack-carrier-d16-control (n=40, range 364106.0-381377.1 ns)
  364106.0 |########################################
  364969.5 |##########
  365833.1 |###############
  366696.7 |#####
  367560.2 |#####
  368423.8 |##########
  369287.3 |#####
  370150.9 |###############
  371014.4 |
  371878.0 |##########
  372741.5 |##########
  373605.1 |
  374468.7 |##########
  375332.2 |#####
  376195.8 |#####
  377059.3 |#####
  377922.9 |##########
  378786.4 |#####
  379650.0 |#####
  380513.5 |
  (3 below, 3 above range)

bitpack-carrier-d32 (n=40, range 375251.3-468763.6 ns)
  375251.3 |##############################
  379926.9 |##########
  384602.5 |##############################
  389278.1 |##########
  393953.8 |########################################
  398629.4 |####################
  403305.0 |########################################
  407980.6 |##############################
  412656.2 |##############################
  417331.8 |####################
  422007.4 |####################
  426683.1 |
  431358.7 |##########
  436034.3 |##########
  440709.9 |##########
  445385.5 |##########
  450061.1 |
  454736.8 |
  459412.4 |
  464088.0 |
  (5 below, 3 above range)

bitpack-carrier-d64 (n=40, range 562903.6-754687.2 ns)
  562903.6 |####################
  572492.8 |######
  582082.0 |
  591671.1 |#############
  601260.3 |####################
  610849.5 |####################
  620438.7 |########################################
  630027.9 |#############
  639617.1 |####################
  649206.2 |
  658795.4 |
  668384.6 |######
  677973.8 |##########################
  687563.0 |#############
  697152.2 |
  706741.3 |
  716330.5 |
  725919.7 |######
  735508.9 |
  745098.1 |
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 536297.0-562657.2 ns)
  536297.0 |################
  537615.0 |################################
  538933.0 |########################################
  540251.0 |################
  541569.0 |########################
  542887.1 |########################
  544205.1 |########
  545523.1 |################
  546841.1 |########################
  548159.1 |########
  549477.1 |########
  550795.1 |########
  552113.1 |################
  553431.1 |########################
  554749.1 |
  556067.1 |
  557385.1 |########
  558703.2 |
  560021.2 |
  561339.2 |
  (4 below, 2 above range)

bitpack-carrier-packed-simd (n=40, range 501869.5-516686.5 ns)
  501869.5 |########################
  502610.4 |########################################
  503351.2 |########################################
  504092.1 |################
  504832.9 |########
  505573.8 |########################
  506314.6 |################
  507055.5 |########################
  507796.3 |########################
  508537.2 |
  509278.0 |########
  510018.9 |
  510759.7 |########
  511500.6 |########
  512241.4 |
  512982.3 |
  513723.1 |
  514464.0 |
  515204.8 |
  515945.7 |
  (4 below, 6 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **bitpack-carrier-d16-control**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed-simd**: autocorrelation=0.51 (measurement drift or warm-up artifact)
