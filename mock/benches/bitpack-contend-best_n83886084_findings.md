# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-padal beats baseline by 21% (significant)

bitpack-contend-d16-padal is -61.38 us (21%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d32 is an outlier: 2.4x slower than the field

bitpack-contend-d32 (582.73 us) is 2.4x the fastest (242.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-contend-d16-padal, bitpack-contend-pipe4, bitpack-contend-d16, bitpack-contend-packed-simd} vs {bitpack-contend-d32-padal, bitpack-contend-d32} (56% apart)

The field splits into a fast tier {bitpack-contend-d16-padal, bitpack-contend-pipe4, bitpack-contend-d16, bitpack-contend-packed-simd} and a slow tier {bitpack-contend-d32-padal, bitpack-contend-d32} with a 56% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 1.6x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 218.49 us but its worst 20% at 351.15 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-padal** at 242375.2 ns median (-16.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.40x (fastest 242375.2 ns, slowest 582731.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 310312ns | 289240ns | 271974ns | 296198ns | 390993ns | base |
| bitpack-contend-d16-padal | 243155ns | 242961ns | 227649ns | 241411ns | 263894ns | -21.64% |
| bitpack-contend-d32 | 597335ns | 583627ns | 557680ns | 587271ns | 667183ns | +92.49% |
| bitpack-contend-d32-padal | 597289ns | 583189ns | 561949ns | 585004ns | 669485ns | +92.48% |
| bitpack-contend-packed-simd | 384175ns | 375082ns | 350948ns | 377497ns | 437435ns | +23.80% |
| bitpack-contend-pipe4 | 263988ns | 252085ns | 219237ns | 249591ns | 351929ns | -14.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 309442ns | 271235ns | 389943ns | base | 27.109 |
| bitpack-contend-d16-padal | 242423ns | 226929ns | 263117ns | -21.66% | 34.603 |
| bitpack-contend-d32 | 596039ns | 556233ns | 665892ns | +92.62% | 14.074 |
| bitpack-contend-d32-padal | 596158ns | 560642ns | 668278ns | +92.66% | 14.071 |
| bitpack-contend-packed-simd | 383418ns | 350346ns | 436194ns | +23.91% | 21.878 |
| bitpack-contend-pipe4 | 263271ns | 218492ns | 351148ns | -14.92% | 31.863 |

## Performance model

- Peak throughput: **38.393 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 29.084 | 75.8% |
| bitpack-contend-d16-padal | 34.610 | 90.1% |
| bitpack-contend-d32 | 14.395 | 37.5% |
| bitpack-contend-d32-padal | 14.407 | 37.5% |
| bitpack-contend-packed-simd | 22.421 | 58.4% |
| bitpack-contend-pipe4 | 33.347 | 86.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 310312ns | 310312ns | base |
| bitpack-contend-d16-padal | 243155ns | 243155ns | -21.64% |
| bitpack-contend-d32 | 597335ns | 597335ns | +92.49% |
| bitpack-contend-d32-padal | 597289ns | 597289ns | +92.48% |
| bitpack-contend-packed-simd | 384175ns | 384175ns | +23.80% |
| bitpack-contend-pipe4 | 263988ns | 263988ns | -14.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 288431ns | base | --- | [281461, 305838] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 242375ns | -45476.4ns (-15.8%) | [-57813, -39572]ns | [233512, 245482] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 582731ns | +292228.3ns (+101.3%) | [+268824, +309577]ns | [571978, 602670] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 582264ns | +290786.1ns (+100.8%) | [+282782, +299869]ns | [570167, 591716] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 374144ns | +81992.9ns (+28.4%) | [+71800, +91107]ns | [368337, 380323] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 251558ns | -46889.4ns (-16.3%) | [-62974, -27265]ns | [228652, 264869] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 328553ns | -22.6% | +80.3% | +69.0% | +1.7% | -35.4% |
| 2 | 280137ns | -3.3% | +100.1% | +101.8% | +45.5% | -13.9% |
| 3 | 282381ns | -6.0% | +133.1% | +102.6% | +19.1% | -25.2% |
| 4 | 284469ns | -14.1% | +130.7% | +99.8% | +46.2% | -23.1% |
| 5 | 286229ns | -14.1% | +137.4% | +100.0% | +74.0% | -9.5% |
| 6 | 360032ns | -35.4% | +71.3% | +69.9% | +20.7% | -38.0% |
| 7 | 284683ns | -13.6% | +109.1% | +106.8% | +45.4% | +8.8% |
| 8 | 349187ns | -30.4% | +63.0% | +60.9% | +24.8% | -36.6% |
| 9 | 263956ns | -8.5% | +122.1% | +164.2% | +57.1% | -14.6% |
| 10 | 302641ns | -19.8% | +93.8% | +104.9% | +36.9% | -25.0% |
| 11 | 352142ns | -34.5% | +56.9% | +67.0% | +2.7% | -36.3% |
| 12 | 272360ns | -13.0% | +109.1% | +117.8% | +30.4% | -17.2% |
| 13 | 288196ns | -11.2% | +93.8% | +103.0% | +29.5% | -16.2% |
| 14 | 270190ns | -14.3% | +113.6% | +117.2% | +40.6% | -7.6% |
| 15 | 272365ns | -16.7% | +104.4% | +115.1% | +37.0% | -7.1% |
| 16 | 288667ns | -22.9% | +92.6% | +93.4% | +25.8% | -8.0% |
| 17 | 302318ns | -15.9% | +88.1% | +87.4% | +20.2% | -12.6% |
| 18 | 273329ns | -10.1% | +120.9% | +134.0% | +29.7% | +7.0% |
| 19 | 281945ns | -11.8% | +100.8% | +105.5% | +27.4% | -9.7% |
| 20 | 307433ns | -14.3% | +100.3% | +96.7% | +25.3% | +7.9% |
| 21 | 290249ns | -15.6% | +148.2% | +132.9% | +16.7% | -23.4% |
| 22 | 316472ns | -14.5% | +78.6% | +104.2% | +17.5% | -28.6% |
| 23 | 276702ns | -12.0% | +117.4% | +107.9% | +33.0% | -16.8% |
| 24 | 268477ns | -11.8% | +138.2% | +112.2% | +37.0% | -9.0% |
| 25 | 277196ns | -18.4% | +107.3% | +117.5% | +32.8% | -9.8% |
| 26 | 341040ns | -33.7% | +69.3% | +65.7% | +8.1% | -25.2% |
| 27 | 346715ns | -34.5% | +66.2% | +63.5% | +9.7% | -20.0% |
| 28 | 603788ns | -62.2% | -7.0% | -5.5% | -37.0% | +15.1% |
| 29 | 274201ns | -16.6% | +104.8% | +104.7% | +37.0% | +9.0% |
| 30 | 302579ns | -24.2% | +104.0% | +85.6% | +24.1% | -15.8% |
| 31 | 274998ns | -10.8% | +113.1% | +102.5% | +36.9% | -21.9% |
| 32 | 277451ns | -3.7% | +117.7% | +106.4% | +35.2% | -18.4% |
| 33 | 280977ns | -13.9% | +106.2% | +135.4% | +30.8% | -19.3% |
| 34 | 343701ns | -32.4% | +68.3% | +76.0% | +12.8% | -21.5% |
| 35 | 277735ns | -16.4% | +126.7% | +105.1% | +32.2% | -3.5% |
| 36 | 318512ns | -26.8% | +72.7% | +128.0% | +22.2% | -15.3% |
| 37 | 408827ns | -42.8% | +35.6% | +39.2% | +13.1% | -33.7% |
| 38 | 355152ns | -32.9% | +70.8% | +78.5% | +4.8% | -22.6% |
| 39 | 306599ns | -18.2% | +136.9% | +117.1% | +21.3% | -6.7% |
| 40 | 305077ns | -15.6% | +98.7% | +93.5% | +30.6% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.065 | ok |
| bitpack-contend-d16-padal | 0.446 | moderate+ |
| bitpack-contend-d32 | 0.223 | moderate+ |
| bitpack-contend-d32-padal | 0.065 | ok |
| bitpack-contend-packed-simd | 0.348 | moderate+ |
| bitpack-contend-pipe4 | 0.152 | ok |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 40/40, lost 0/40
- **bitpack-contend-d32**: won 1/40, lost 39/40
- **bitpack-contend-d32-padal**: won 1/40, lost 39/40
- **bitpack-contend-packed-simd**: won 1/40, lost 39/40
- **bitpack-contend-pipe4**: won 34/40, lost 6/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 13.3ns | 309441.5ns | 0.0% |  |
| bitpack-contend-d16-padal | 7.2ns | 242423.0ns | 0.0% |  |
| bitpack-contend-d32 | 15.5ns | 596038.5ns | 0.0% |  |
| bitpack-contend-d32-padal | 9.4ns | 596158.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 32.3ns | 383418.3ns | 0.0% |  |
| bitpack-contend-pipe4 | 7.8ns | 263271.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 271234.5-389942.9 ns)
  271234.5 |##################################
  277169.9 |########################################
  283105.4 |############################
  289040.8 |#####
  294976.2 |
  300911.6 |############################
  306847.0 |#####
  312782.4 |###########
  318717.9 |
  324653.3 |#####
  330588.7 |
  336524.1 |#####
  342459.5 |###########
  348395.0 |###########
  354330.4 |###########
  360265.8 |
  366201.2 |
  372136.6 |
  378072.0 |
  384007.5 |
  (3 below, 2 above range)

bitpack-contend-d16-padal (n=40, range 226929.4-263117.4 ns)
  226929.4 |################################
  228738.8 |################
  230548.2 |################
  232357.6 |################################
  234167.0 |
  235976.4 |################
  237785.8 |########
  239595.2 |
  241404.6 |################################
  243214.0 |################
  245023.4 |########################################
  246832.8 |########
  248642.2 |
  250451.6 |########
  252261.0 |
  254070.4 |################
  255879.8 |################
  257689.2 |
  259498.6 |
  261308.0 |
  (3 below, 5 above range)

bitpack-contend-d32 (n=40, range 556232.8-665891.5 ns)
  556232.8 |########################################
  561715.7 |################
  567198.6 |########################
  572681.6 |################################
  578164.5 |################
  583647.5 |########################
  589130.4 |########
  594613.3 |########
  600096.3 |########################
  605579.2 |################
  611062.1 |########
  616545.1 |################
  622028.0 |
  627511.0 |########
  632993.9 |
  638476.8 |########
  643959.8 |
  649442.7 |
  654925.6 |################
  660408.6 |
  (4 below, 3 above range)

bitpack-contend-d32-padal (n=40, range 560642.3-668278.2 ns)
  560642.3 |############################
  566024.1 |########################################
  571405.9 |######################
  576787.7 |#####
  582169.5 |#################
  587551.3 |#################
  592933.1 |#####
  598314.9 |#####
  603696.7 |###########
  609078.5 |#####
  614460.3 |
  619842.1 |#####
  625223.9 |
  630605.7 |#####
  635987.4 |#####
  641369.2 |#####
  646751.0 |
  652132.8 |
  657514.6 |#####
  662896.4 |#####
  (3 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 350345.8-436194.5 ns)
  350345.8 |#####
  354638.2 |#####
  358930.7 |###############
  363223.1 |##########
  367515.5 |#########################
  371808.0 |########################################
  376100.4 |####################
  380392.8 |
  384685.3 |##########
  388977.7 |#####
  393270.1 |
  397562.6 |#####
  401855.0 |
  406147.4 |#####
  410439.9 |###############
  414732.3 |#####
  419024.7 |
  423317.2 |
  427609.6 |
  431902.1 |##########
  (3 below, 2 above range)

bitpack-contend-pipe4 (n=40, range 218491.7-351148.1 ns)
  218491.7 |############################
  225124.5 |########################################
  231757.4 |
  238390.2 |#################
  245023.0 |###########
  251655.8 |######################
  258288.6 |###########
  264921.4 |############################
  271554.3 |###########
  278187.1 |
  284819.9 |#####
  291452.7 |#####
  298085.5 |#####
  304718.4 |#####
  311351.2 |
  317984.0 |#####
  324616.8 |
  331249.6 |#####
  337882.4 |
  344515.3 |
  (3 below, 1 above range)

```

## Diagnostics

- **bitpack-contend-pipe4**: CV=28.6% (high variance, measurements may be unstable)
