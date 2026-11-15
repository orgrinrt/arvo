# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.82)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 1.1% of the fastest

All 5 variants sit between 11.83 us and 11.97 us - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (33 ns, 0.28%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 33 ns (0.28%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 11834.5 ns median (-0.4% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 11834.5 ns, slowest 11967.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 11967ns | 11953ns | 11763ns | 11939ns | 12255ns | base |
| wide-rung-ragged | 12172ns | 12037ns | 11963ns | 12084ns | 12646ns | +1.72% |
| wide-rung-ragged-overread | 11965ns | 11979ns | 11812ns | 11970ns | 12106ns | -0.01% |
| wide-rung-wordround | 11930ns | 11932ns | 11727ns | 11881ns | 12282ns | -0.30% |
| wide-rung-wordround-alias | 11871ns | 11901ns | 11726ns | 11880ns | 11987ns | -0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 11897ns | 11696ns | 12176ns | base | 1.549 |
| wide-rung-ragged | 12099ns | 11893ns | 12569ns | +1.70% | 1.523 |
| wide-rung-ragged-overread | 11894ns | 11740ns | 12031ns | -0.02% | 1.550 |
| wide-rung-wordround | 11859ns | 11657ns | 12203ns | -0.32% | 1.554 |
| wide-rung-wordround-alias | 11800ns | 11660ns | 11919ns | -0.81% | 1.562 |

## Performance model

- Peak throughput: **1.581 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 18432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.551 | 98.1% |
| wide-rung-ragged | 1.540 | 97.4% |
| wide-rung-ragged-overread | 1.547 | 97.9% |
| wide-rung-wordround | 1.555 | 98.3% |
| wide-rung-wordround-alias | 1.557 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 11967ns | 11967ns | base |
| wide-rung-ragged | 12172ns | 12172ns | +1.72% |
| wide-rung-ragged-overread | 11965ns | 11965ns | -0.01% |
| wide-rung-wordround | 11930ns | 11930ns | -0.30% |
| wide-rung-wordround-alias | 11871ns | 11871ns | -0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 11885ns | base | --- | [11863, 11901] | --- | --- | --- | --- |
| wide-rung-ragged | 11967ns | +201.6ns (+1.7%) | [+164, +235]ns | [11942, 12102] | YES | 0.0002 | 0.0000 | 0 |
| wide-rung-ragged-overread | 11912ns | +23.9ns (+0.2%) | [+14, +34]ns | [11902, 11925] | YES | 0.0166 | 0.0166 | 0 |
| wide-rung-wordround | 11856ns | -36.0ns (-0.3%) | [-48, -28]ns | [11720, 11866] | YES | 0.0004 | 0.0002 | 0 |
| wide-rung-wordround-alias | 11835ns | -51.9ns (-0.4%) | [-125, -22]ns | [11765, 11853] | YES | 0.0030 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 11905ns | +5.4% | +0.2% | +3.4% | -0.3% |
| 2 | 11941ns | +5.1% | -0.2% | +3.3% | +0.7% |
| 3 | 11865ns | +6.4% | +0.5% | +4.0% | +0.4% |
| 4 | 11861ns | +2.9% | +0.5% | +4.5% | +0.2% |
| 5 | 11895ns | +2.0% | +0.5% | +4.3% | -0.1% |
| 6 | 11908ns | +2.0% | +0.2% | +0.1% | -0.3% |
| 7 | 11892ns | +9.5% | +0.7% | -0.3% | -0.1% |
| 8 | 12117ns | +3.4% | -1.6% | -2.1% | -2.2% |
| 9 | 12318ns | +1.9% | -2.8% | -3.6% | -3.7% |
| 10 | 12344ns | +1.3% | -0.0% | -3.9% | -3.8% |
| 11 | 11880ns | +1.6% | +0.4% | -0.1% | -0.5% |
| 12 | 11898ns | +1.5% | +0.2% | -0.3% | -1.1% |
| 13 | 11900ns | +0.6% | +0.1% | -0.2% | -0.6% |
| 14 | 11885ns | +0.3% | +0.2% | +0.5% | -1.8% |
| 15 | 11990ns | -0.7% | -0.3% | -0.0% | -2.6% |
| 16 | 11884ns | +0.0% | +0.1% | +0.3% | -1.9% |
| 17 | 12035ns | -0.8% | -0.3% | -1.0% | -2.6% |
| 18 | 11901ns | +0.4% | +0.2% | -0.3% | -1.7% |
| 19 | 11884ns | +0.1% | +0.2% | -0.2% | -2.0% |
| 20 | 11901ns | +2.7% | +0.1% | -0.4% | -2.1% |
| 21 | 11871ns | +2.2% | +0.3% | -0.3% | -0.3% |
| 22 | 11878ns | +1.7% | +0.3% | -0.1% | -0.5% |
| 23 | 11767ns | +1.7% | +1.0% | -0.7% | -0.5% |
| 24 | 11690ns | +1.7% | +0.3% | -0.1% | -0.2% |
| 25 | 11736ns | +1.7% | +0.1% | -0.5% | +0.7% |
| 26 | 11789ns | +1.3% | -0.5% | -0.6% | -1.1% |
| 27 | 11723ns | +2.2% | +0.3% | -0.3% | -0.4% |
| 28 | 11693ns | +1.6% | +0.9% | -0.3% | +0.6% |
| 29 | 11681ns | +1.8% | +0.4% | -0.2% | -0.2% |
| 30 | 11692ns | +1.7% | +0.3% | -0.3% | -0.2% |
| 31 | 12048ns | -0.7% | -1.0% | -1.6% | -1.6% |
| 32 | 12029ns | -1.0% | -1.0% | -2.6% | -1.5% |
| 33 | 11991ns | -0.8% | -0.1% | -2.6% | -0.4% |
| 34 | 11986ns | -0.7% | -0.0% | -2.1% | -0.6% |
| 35 | 12528ns | -4.7% | -3.8% | -5.6% | -5.4% |
| 36 | 11708ns | +2.1% | +0.8% | -0.4% | +1.1% |
| 37 | 11760ns | +1.4% | +0.5% | -0.4% | +1.2% |
| 38 | 11708ns | +2.0% | +0.5% | -0.6% | +1.0% |
| 39 | 11696ns | +3.9% | +1.3% | -0.3% | +1.2% |
| 40 | 11701ns | +3.6% | +0.4% | -0.4% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.423 | moderate+ |
| wide-rung-ragged | 0.675 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.468 | moderate+ |
| wide-rung-wordround | 0.821 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.661 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 7/40, lost 31/40
- **wide-rung-ragged-overread**: won 10/40, lost 27/40
- **wide-rung-wordround**: won 31/40, lost 7/40
- **wide-rung-wordround-alias**: won 29/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 11897.0ns | 0.0% |  |
| wide-rung-ragged | 2.4ns | 12099.0ns | 0.0% |  |
| wide-rung-ragged-overread | 2.2ns | 11894.3ns | 0.0% |  |
| wide-rung-wordround | 2.3ns | 11858.5ns | 0.0% |  |
| wide-rung-wordround-alias | 2.5ns | 11800.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 11696.2-12176.4 ns)
  11696.2 |###############
  11720.2 |##########
  11744.2 |##########
  11768.3 |#####
  11792.3 |
  11816.3 |
  11840.3 |#####
  11864.3 |###################################
  11888.3 |########################################
  11912.3 |
  11936.3 |#####
  11960.3 |
  11984.3 |###############
  12008.3 |#####
  12032.3 |##########
  12056.4 |
  12080.4 |
  12104.4 |#####
  12128.4 |
  12152.4 |
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 11892.6-12568.5 ns)
  11892.6 |########################################
  11926.4 |###################################
  11960.2 |####################
  11994.0 |
  12027.8 |
  12061.6 |###############
  12095.4 |#####
  12129.2 |####################
  12163.0 |
  12196.8 |##########
  12230.6 |
  12264.4 |
  12298.2 |
  12332.0 |
  12365.8 |
  12399.6 |
  12433.3 |
  12467.1 |
  12500.9 |##########
  12534.7 |###############
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 11739.7-12031.1 ns)
  11739.7 |#############
  11754.3 |####
  11768.8 |
  11783.4 |
  11798.0 |########
  11812.6 |####
  11827.1 |
  11841.7 |####
  11856.3 |
  11870.8 |####
  11885.4 |
  11900.0 |###############################
  11914.6 |########################################
  11929.1 |########
  11943.7 |########
  11958.3 |####
  11972.9 |#############
  11987.4 |####
  12002.0 |
  12016.6 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 11656.9-12202.9 ns)
  11656.9 |#########################
  11684.2 |#######
  11711.5 |##############
  11738.8 |
  11766.1 |
  11793.4 |
  11820.7 |#######
  11848.0 |########################################
  11875.3 |###
  11902.6 |##########
  11929.9 |###
  11957.2 |
  11984.5 |###
  12011.8 |
  12039.1 |
  12066.4 |
  12093.7 |
  12121.0 |
  12148.3 |
  12175.6 |
  (3 below, 5 above range)

wide-rung-wordround-alias (n=40, range 11659.5-11918.6 ns)
  11659.5 |################################
  11672.5 |################
  11685.4 |
  11698.4 |################
  11711.3 |
  11724.3 |########
  11737.2 |
  11750.2 |########
  11763.1 |########
  11776.1 |
  11789.0 |
  11802.0 |
  11815.0 |################################
  11827.9 |########################################
  11840.9 |########################
  11853.8 |########
  11866.8 |########################################
  11879.7 |################
  11892.7 |########
  11905.6 |################
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.68 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.66 (measurement drift or warm-up artifact)
