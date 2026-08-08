# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (307.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 288.67 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-ragged-overread, wide-rung-wordround) are a dead heat (<1%)

wide-rung-ragged-overread (288.67 us) and wide-rung-wordround (289.69 us) differ by 0.35%, inside the noise, even though the wider field spreads 6.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-ragged (+0% speed for 1.5x steadier)

wide-rung-ragged-overread is fastest (288.67 us, CV 3.4%); wide-rung-ragged gives up 0.4% median for 1.5x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 288672.1 ns median (-6.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.06x (fastest 288672.1 ns, slowest 307325.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 310296ns | 309294ns | 302581ns | 309396ns | 320711ns | base |
| wide-rung-ragged | 292907ns | 290550ns | 287600ns | 291187ns | 303373ns | -5.60% |
| wide-rung-ragged-overread | 292512ns | 289378ns | 285335ns | 290862ns | 304636ns | -5.73% |
| wide-rung-wordround | 293792ns | 290266ns | 287487ns | 291338ns | 307459ns | -5.32% |
| wide-rung-wordround-alias | 295963ns | 291700ns | 288414ns | 292490ns | 313931ns | -4.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 308811ns | 301356ns | 319455ns | base | 1.486 |
| wide-rung-ragged | 292311ns | 286927ns | 302867ns | -5.34% | 1.569 |
| wide-rung-ragged-overread | 291848ns | 284665ns | 304173ns | -5.49% | 1.572 |
| wide-rung-wordround | 292914ns | 286847ns | 306352ns | -5.15% | 1.566 |
| wide-rung-wordround-alias | 295347ns | 287856ns | 313189ns | -4.36% | 1.553 |

## Performance model

- Peak throughput: **1.612 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.493 | 92.6% |
| wide-rung-ragged | 1.583 | 98.2% |
| wide-rung-ragged-overread | 1.589 | 98.6% |
| wide-rung-wordround | 1.584 | 98.3% |
| wide-rung-wordround-alias | 1.575 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 310296ns | 310296ns | base |
| wide-rung-ragged | 292907ns | 292907ns | -5.60% |
| wide-rung-ragged-overread | 292512ns | 292512ns | -5.73% |
| wide-rung-wordround | 293792ns | 293792ns | -5.32% |
| wide-rung-wordround-alias | 295963ns | 295963ns | -4.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 307325ns | base | --- | [305638, 309260] | --- | --- | --- | --- |
| wide-rung-ragged | 289887ns | -18217.3ns (-5.9%) | [-20425, -12932]ns | [288751, 292124] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 288672ns | -17852.7ns (-5.8%) | [-22172, -13865]ns | [287593, 293618] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 289688ns | -17133.4ns (-5.6%) | [-18751, -12697]ns | [287533, 292420] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 291184ns | -17086.9ns (-5.6%) | [-18675, -12589]ns | [289157, 292683] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 319781ns | -9.5% | -7.8% | -9.2% | -10.1% |
| 2 | 338624ns | -9.4% | -11.8% | -14.1% | -10.0% |
| 3 | 307365ns | -2.2% | -5.8% | -5.9% | -3.3% |
| 4 | 298184ns | -1.6% | -0.3% | -2.9% | +6.0% |
| 5 | 298045ns | -2.2% | -2.1% | -3.5% | -2.7% |
| 6 | 315842ns | -6.5% | -7.0% | -7.0% | -8.5% |
| 7 | 305717ns | -3.4% | -4.0% | -4.4% | -4.6% |
| 8 | 302475ns | -3.6% | -2.3% | -3.0% | -4.2% |
| 9 | 300762ns | -2.3% | -4.0% | -2.3% | +3.7% |
| 10 | 301398ns | -2.6% | -1.8% | +2.1% | +9.5% |
| 11 | 311266ns | -6.7% | -8.4% | -7.3% | -6.4% |
| 12 | 317252ns | -8.6% | -9.1% | -9.4% | -8.9% |
| 13 | 309048ns | -6.6% | +11.0% | -5.6% | -6.7% |
| 14 | 305136ns | -5.4% | -2.9% | -5.6% | -5.9% |
| 15 | 304775ns | -3.6% | +1.3% | -5.7% | -5.7% |
| 16 | 304730ns | -0.7% | -3.5% | -4.3% | -5.4% |
| 17 | 305325ns | -4.3% | -3.7% | -5.9% | -5.7% |
| 18 | 306931ns | -4.8% | -4.3% | -6.4% | +3.8% |
| 19 | 310502ns | -6.2% | -4.7% | -7.5% | -0.3% |
| 20 | 305558ns | -0.5% | -4.4% | -6.1% | +2.4% |
| 21 | 305429ns | -6.1% | -5.9% | -6.0% | -2.5% |
| 22 | 308216ns | -6.8% | -5.4% | -6.8% | -5.6% |
| 23 | 311810ns | -7.7% | -7.9% | -7.0% | -4.0% |
| 24 | 324774ns | -11.7% | -11.6% | -11.5% | -10.3% |
| 25 | 303347ns | -5.1% | -5.4% | -5.3% | -3.9% |
| 26 | 301911ns | -4.1% | -4.7% | -4.9% | -2.7% |
| 27 | 311939ns | -7.4% | -7.5% | -8.1% | -5.1% |
| 28 | 306548ns | -4.1% | -6.1% | -6.2% | -2.2% |
| 29 | 305301ns | +3.3% | -6.1% | +1.6% | -1.2% |
| 30 | 309473ns | -6.4% | -3.8% | -5.5% | -6.2% |
| 31 | 312422ns | -7.7% | -9.0% | -8.5% | -7.5% |
| 32 | 315006ns | -9.0% | -9.7% | -9.0% | -7.4% |
| 33 | 307285ns | -7.0% | -7.3% | +4.0% | -5.9% |
| 34 | 305816ns | -5.2% | -5.8% | -0.2% | -5.8% |
| 35 | 311040ns | -7.6% | -8.5% | -3.6% | -7.3% |
| 36 | 311698ns | -8.0% | -8.7% | -3.8% | -7.4% |
| 37 | 308579ns | -1.9% | -7.8% | -4.0% | -6.5% |
| 38 | 307265ns | -5.8% | -7.3% | -2.2% | -5.6% |
| 39 | 307690ns | -6.4% | -6.4% | -0.9% | -5.3% |
| 40 | 308176ns | -6.4% | -7.2% | -1.7% | -6.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.242 | moderate+ |
| wide-rung-ragged | 0.128 | ok |
| wide-rung-ragged-overread | 0.146 | ok |
| wide-rung-wordround | 0.374 | moderate+ |
| wide-rung-wordround-alias | 0.334 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 39/40, lost 1/40
- **wide-rung-ragged-overread**: won 38/40, lost 2/40
- **wide-rung-wordround**: won 37/40, lost 3/40
- **wide-rung-wordround-alias**: won 35/40, lost 5/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 3.8ns | 308811.0ns | 0.0% |  |
| wide-rung-ragged | 5.8ns | 292311.2ns | 0.0% |  |
| wide-rung-ragged-overread | 4.3ns | 291848.3ns | 0.0% |  |
| wide-rung-wordround | 5.3ns | 292914.0ns | 0.0% |  |
| wide-rung-wordround-alias | 8.0ns | 295347.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 301356.4-319454.9 ns)
  301356.4 |###########
  302261.3 |#####
  303166.2 |#####
  304071.1 |###########
  304976.1 |########################################
  305881.0 |#####
  306785.9 |############################
  307690.9 |#################
  308595.8 |###########
  309500.7 |
  310405.6 |#################
  311310.6 |#################
  312215.5 |#####
  313120.4 |
  314025.3 |
  314930.3 |#####
  315835.2 |#####
  316740.1 |#####
  317645.1 |
  318550.0 |
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 286927.0-302866.7 ns)
  286927.0 |#############
  287723.9 |#################################
  288520.9 |####################
  289317.9 |########################################
  290114.9 |######
  290911.9 |####################
  291708.9 |#############
  292505.9 |######
  293302.8 |##########################
  294099.8 |
  294896.8 |#############
  295693.8 |
  296490.8 |
  297287.8 |
  298084.8 |
  298881.7 |
  299678.7 |
  300475.7 |######
  301272.7 |
  302069.7 |#############
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 284664.6-304173.1 ns)
  284664.6 |########################
  285640.1 |################
  286615.5 |################################
  287590.9 |########################################
  288566.3 |########################
  289541.7 |
  290517.2 |########
  291492.6 |################
  292468.0 |
  293443.4 |########################################
  294418.9 |################
  295394.3 |########################
  296369.7 |
  297345.1 |################
  298320.5 |########
  299296.0 |
  300271.4 |
  301246.8 |
  302222.2 |
  303197.7 |
  (5 below, 2 above range)

wide-rung-wordround (n=40, range 286846.7-306351.7 ns)
  286846.7 |########################################
  287821.9 |######
  288797.2 |######
  289772.4 |######
  290747.7 |###
  291722.9 |############
  292698.2 |######
  293673.4 |###
  294648.7 |
  295623.9 |###
  296599.2 |
  297574.4 |
  298549.7 |
  299525.0 |######
  300500.2 |###
  301475.5 |
  302450.7 |###
  303426.0 |
  304401.2 |######
  305376.5 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 287856.5-313189.5 ns)
  287856.5 |########################################
  289123.1 |##################
  290389.8 |#########################
  291656.4 |
  292923.1 |###
  294189.7 |
  295456.4 |###
  296723.0 |#######
  297989.7 |
  299256.3 |#######
  300523.0 |###
  301789.6 |
  303056.3 |
  304322.9 |###
  305589.6 |
  306856.2 |
  308122.9 |
  309389.5 |###
  310656.2 |###
  311922.8 |###
  (3 below, 3 above range)

```
