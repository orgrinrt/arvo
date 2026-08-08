# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.74)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 2.2% of the fastest

All 5 variants sit between 11.53 us and 11.77 us - a 2.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-wordround's edge over baseline is significant but tiny (49 ns, 0.42%)

wide-rung-wordround differs from baseline wide-rung-align16 by 49 ns (0.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 11526.0 ns median (-0.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 11526.0 ns, slowest 11774.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 11767ns | 11644ns | 11530ns | 11683ns | 12256ns | base |
| wide-rung-ragged | 11927ns | 11838ns | 11728ns | 11869ns | 12301ns | +1.36% |
| wide-rung-ragged-overread | 11782ns | 11692ns | 11550ns | 11698ns | 12264ns | +0.12% |
| wide-rung-wordround | 11722ns | 11596ns | 11542ns | 11631ns | 12172ns | -0.39% |
| wide-rung-wordround-alias | 11656ns | 11589ns | 11536ns | 11615ns | 11897ns | -0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 11700ns | 11465ns | 12184ns | base | 1.575 |
| wide-rung-ragged | 11861ns | 11665ns | 12230ns | +1.38% | 1.554 |
| wide-rung-ragged-overread | 11715ns | 11488ns | 12192ns | +0.13% | 1.573 |
| wide-rung-wordround | 11655ns | 11476ns | 12105ns | -0.38% | 1.581 |
| wide-rung-wordround-alias | 11590ns | 11471ns | 11831ns | -0.94% | 1.590 |

## Performance model

- Peak throughput: **1.608 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 18432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.592 | 99.0% |
| wide-rung-ragged | 1.565 | 97.4% |
| wide-rung-ragged-overread | 1.585 | 98.6% |
| wide-rung-wordround | 1.599 | 99.4% |
| wide-rung-wordround-alias | 1.599 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 11767ns | 11767ns | base |
| wide-rung-ragged | 11927ns | 11927ns | +1.36% |
| wide-rung-ragged-overread | 11782ns | 11782ns | +0.12% |
| wide-rung-wordround | 11722ns | 11722ns | -0.39% |
| wide-rung-wordround-alias | 11656ns | 11656ns | -0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 11578ns | base | --- | [11507, 11704] | --- | --- | --- | --- |
| wide-rung-ragged | 11775ns | +194.0ns (+1.7%) | [+173, +223]ns | [11716, 11887] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 11626ns | no significant difference | [-14, +56]ns | [11549, 11706] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-wordround | 11531ns | no significant difference | [-125, +32]ns | [11511, 11606] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-wordround-alias | 11526ns | no significant difference | [-153, +7]ns | [11512, 11568] | no | 0.3077 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 12262ns | -0.4% | -1.0% | -6.3% | -4.5% |
| 2 | 12309ns | -1.9% | -4.8% | -6.1% | -6.2% |
| 3 | 12311ns | -3.2% | -4.9% | -6.8% | -7.0% |
| 4 | 11529ns | +3.1% | -0.2% | -0.5% | -0.4% |
| 5 | 11683ns | +0.8% | -1.2% | -1.5% | -1.1% |
| 6 | 11854ns | -1.4% | -3.1% | -2.9% | -2.9% |
| 7 | 11925ns | -2.1% | +0.5% | -3.4% | -3.4% |
| 8 | 11500ns | +1.5% | +3.5% | +0.3% | +0.0% |
| 9 | 11615ns | +0.9% | +4.5% | -0.6% | +0.2% |
| 10 | 11470ns | +1.7% | +8.3% | +0.1% | +0.1% |
| 11 | 11962ns | +4.5% | +4.1% | -2.2% | -2.4% |
| 12 | 11702ns | +6.9% | +3.9% | -1.9% | -0.0% |
| 13 | 11720ns | +2.6% | +0.5% | -2.2% | -0.4% |
| 14 | 11698ns | +1.6% | +0.2% | -1.9% | -1.3% |
| 15 | 11732ns | +1.5% | -0.5% | -1.3% | -1.7% |
| 16 | 11510ns | +3.4% | -0.1% | -0.2% | -0.2% |
| 17 | 11460ns | +1.8% | +0.2% | +0.5% | +0.5% |
| 18 | 11475ns | +2.3% | +0.1% | +0.3% | +0.3% |
| 19 | 11540ns | +1.5% | +1.0% | -0.1% | -0.3% |
| 20 | 11464ns | +2.0% | +4.1% | +0.2% | +0.9% |
| 21 | 12318ns | +2.0% | -6.7% | -4.5% | -5.4% |
| 22 | 12502ns | -4.3% | -7.0% | -8.1% | -7.7% |
| 23 | 11703ns | +0.1% | +0.2% | -1.9% | -1.9% |
| 24 | 11705ns | +1.5% | -0.7% | -0.0% | -2.2% |
| 25 | 11729ns | +2.1% | -0.9% | -0.8% | -1.3% |
| 26 | 11696ns | +1.6% | -1.6% | +0.2% | -1.1% |
| 27 | 11467ns | +2.4% | +0.2% | +0.5% | +0.3% |
| 28 | 11470ns | +4.2% | +1.8% | +0.4% | +0.4% |
| 29 | 11538ns | +1.8% | +0.6% | -0.0% | -0.2% |
| 30 | 11504ns | +1.4% | -0.1% | -0.0% | -0.2% |
| 31 | 11872ns | +0.1% | +3.8% | +3.6% | -3.4% |
| 32 | 11879ns | +0.1% | -1.4% | +3.6% | -1.5% |
| 33 | 11528ns | +3.0% | +0.7% | +6.6% | +4.1% |
| 34 | 11472ns | +2.6% | +0.2% | +7.2% | +4.6% |
| 35 | 11488ns | +2.2% | +1.2% | +7.3% | +3.7% |
| 36 | 11493ns | +1.5% | -0.0% | +2.7% | +3.4% |
| 37 | 11456ns | +1.9% | +1.4% | +2.5% | +2.5% |
| 38 | 11467ns | +1.7% | +0.2% | +2.2% | +0.4% |
| 39 | 11515ns | +1.5% | +0.4% | +1.7% | -0.1% |
| 40 | 11469ns | +1.7% | +0.2% | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.473 | moderate+ |
| wide-rung-ragged | 0.377 | moderate+ |
| wide-rung-ragged-overread | 0.533 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.744 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.726 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 6/40, lost 31/40
- **wide-rung-ragged-overread**: won 14/40, lost 24/40
- **wide-rung-wordround**: won 18/40, lost 18/40
- **wide-rung-wordround-alias**: won 23/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.0ns | 11699.9ns | 0.0% |  |
| wide-rung-ragged | 2.2ns | 11860.8ns | 0.0% |  |
| wide-rung-ragged-overread | 1.8ns | 11715.4ns | 0.0% |  |
| wide-rung-wordround | 1.9ns | 11654.9ns | 0.0% |  |
| wide-rung-wordround-alias | 1.9ns | 11589.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 11465.3-12183.8 ns)
  11465.3 |########################################
  11501.2 |####################
  11537.1 |########
  11573.0 |
  11609.0 |####
  11644.9 |
  11680.8 |########################
  11716.7 |############
  11752.7 |
  11788.6 |
  11824.5 |####
  11860.4 |########
  11896.4 |####
  11932.3 |####
  11968.2 |
  12004.1 |
  12040.1 |
  12076.0 |
  12111.9 |
  12147.8 |
  (3 below, 5 above range)

wide-rung-ragged (n=40, range 11665.4-12229.9 ns)
  11665.4 |############################
  11693.6 |############################
  11721.8 |######################
  11750.0 |###########
  11778.3 |
  11806.5 |
  11834.7 |
  11862.9 |########################################
  11891.2 |###########
  11919.4 |#####
  11947.6 |#################
  11975.9 |
  12004.1 |#####
  12032.3 |
  12060.5 |#####
  12088.8 |
  12117.0 |
  12145.2 |
  12173.4 |
  12201.7 |#####
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 11488.3-12192.1 ns)
  11488.3 |########################################
  11523.5 |########
  11558.7 |
  11593.9 |##########################
  11629.0 |########
  11664.2 |########
  11699.4 |######################
  11734.6 |
  11769.8 |####
  11805.0 |
  11840.2 |
  11875.4 |####
  11910.6 |####
  11945.8 |
  11981.0 |####
  12016.2 |
  12051.4 |
  12086.5 |
  12121.7 |########
  12156.9 |####
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 11476.2-12104.7 ns)
  11476.2 |########################################
  11507.6 |####################################
  11539.1 |#######
  11570.5 |###
  11601.9 |
  11633.3 |###
  11664.8 |
  11696.2 |##################
  11727.6 |###
  11759.0 |###
  11790.5 |###
  11821.9 |
  11853.3 |
  11884.7 |
  11916.1 |
  11947.6 |
  11979.0 |
  12010.4 |
  12041.8 |
  12073.3 |
  (2 below, 5 above range)

wide-rung-wordround-alias (n=40, range 11470.7-11831.1 ns)
  11470.7 |###################################
  11488.7 |###############
  11506.7 |########################################
  11524.7 |##########
  11542.8 |###############
  11560.8 |###############
  11578.8 |
  11596.8 |
  11614.8 |
  11632.9 |##########
  11650.9 |#####
  11668.9 |#####
  11686.9 |##########
  11704.9 |#####
  11723.0 |
  11741.0 |#####
  11759.0 |
  11777.0 |
  11795.0 |
  11813.1 |
  (2 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.73 (measurement drift or warm-up artifact)
