# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.56 us) is smaller than the fastest variant's own run-to-run std-dev (7.59 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (488.07 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.5% of the fastest

All 5 variants sit between 488.07 us and 495.63 us - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 488070.7 ns median
- 4 variants significantly slower than baseline
- Spread: 1.02x (fastest 488070.7 ns, slowest 495628.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 490120ns | 488984ns | 481036ns | 489286ns | 501705ns | base |
| wide-rung-ragged | 499104ns | 496029ns | 490180ns | 496698ns | 515246ns | +1.83% |
| wide-rung-ragged-overread | 496713ns | 496462ns | 493581ns | 496485ns | 500531ns | +1.35% |
| wide-rung-wordround | 493395ns | 493619ns | 486041ns | 493271ns | 501120ns | +0.67% |
| wide-rung-wordround-alias | 493117ns | 490999ns | 485640ns | 491781ns | 504603ns | +0.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 489270ns | 480214ns | 500871ns | base | 0.938 |
| wide-rung-ragged | 498401ns | 489526ns | 514432ns | +1.87% | 0.920 |
| wide-rung-ragged-overread | 495613ns | 492623ns | 499484ns | +1.30% | 0.926 |
| wide-rung-wordround | 492372ns | 485100ns | 500194ns | +0.63% | 0.932 |
| wide-rung-wordround-alias | 492214ns | 484837ns | 503644ns | +0.60% | 0.932 |

## Performance model

- Peak throughput: **0.955 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.940 | 98.4% |
| wide-rung-ragged | 0.926 | 96.9% |
| wide-rung-ragged-overread | 0.927 | 97.0% |
| wide-rung-wordround | 0.932 | 97.5% |
| wide-rung-wordround-alias | 0.936 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 490120ns | 490120ns | base |
| wide-rung-ragged | 499104ns | 499104ns | +1.83% |
| wide-rung-ragged-overread | 496713ns | 496713ns | +1.35% |
| wide-rung-wordround | 493395ns | 493395ns | +0.67% |
| wide-rung-wordround-alias | 493117ns | 493117ns | +0.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 488071ns | base | --- | [486972, 489620] | --- | --- | --- | --- |
| wide-rung-ragged | 495629ns | +8798.5ns (+1.8%) | [+2668, +12939]ns | [493395, 496428] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-ragged-overread | 495129ns | +7028.5ns (+1.4%) | [+3906, +9907]ns | [494598, 496114] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 492482ns | +3199.4ns (+0.7%) | [+450, +5157]ns | [489582, 494891] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 489989ns | +3376.7ns (+0.7%) | [+118, +6480]ns | [488919, 493090] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 504117ns | -2.3% | -1.3% | -3.0% | +2.5% |
| 2 | 495383ns | -0.8% | -0.2% | -0.6% | +0.4% |
| 3 | 488990ns | +1.4% | +0.6% | +0.0% | -0.3% |
| 4 | 487695ns | +6.6% | +3.7% | -0.2% | -0.1% |
| 5 | 486769ns | +4.0% | +2.7% | -0.3% | +1.4% |
| 6 | 491881ns | +0.8% | -0.0% | -0.9% | +0.4% |
| 7 | 484022ns | +3.2% | +1.8% | +0.8% | -0.7% |
| 8 | 486369ns | +1.9% | +1.9% | +0.2% | -0.6% |
| 9 | 515537ns | -4.2% | -4.0% | -3.9% | -6.0% |
| 10 | 492328ns | +0.2% | +0.5% | +1.0% | -1.5% |
| 11 | 482029ns | +2.8% | +3.1% | +0.8% | +4.2% |
| 12 | 480716ns | +2.7% | +2.6% | +0.5% | +2.2% |
| 13 | 478804ns | +3.0% | +3.4% | +3.5% | +3.6% |
| 14 | 478633ns | +3.6% | +4.7% | +4.7% | +2.2% |
| 15 | 483657ns | +1.9% | +2.4% | +1.0% | +1.4% |
| 16 | 488060ns | +0.2% | +2.2% | -0.2% | +1.8% |
| 17 | 478822ns | +6.1% | +3.9% | +1.4% | +0.9% |
| 18 | 485269ns | +6.9% | +2.3% | -0.1% | +2.5% |
| 19 | 481225ns | +5.2% | +2.9% | +0.2% | +1.3% |
| 20 | 479060ns | +6.9% | +3.1% | +2.4% | +1.9% |
| 21 | 504527ns | -2.0% | -2.2% | -1.9% | -3.1% |
| 22 | 489535ns | +2.2% | +0.2% | +1.7% | +1.4% |
| 23 | 493905ns | +0.3% | +0.5% | +0.2% | -1.2% |
| 24 | 487804ns | +1.8% | +1.7% | +1.4% | -0.2% |
| 25 | 494794ns | +0.2% | -0.4% | -0.3% | -1.2% |
| 26 | 489768ns | +0.6% | +1.0% | +1.1% | +1.1% |
| 27 | 488082ns | +8.4% | +1.3% | +1.3% | +0.9% |
| 28 | 489625ns | +3.6% | +1.1% | +1.4% | +2.9% |
| 29 | 489614ns | +2.7% | +1.2% | +3.6% | +1.0% |
| 30 | 503221ns | -1.9% | -1.4% | -2.3% | -2.0% |
| 31 | 485625ns | +2.1% | +2.3% | +0.9% | +1.5% |
| 32 | 489446ns | +0.4% | +1.5% | +1.9% | +0.3% |
| 33 | 489218ns | +0.5% | +1.4% | +0.6% | +5.9% |
| 34 | 487260ns | +0.6% | +1.4% | +3.6% | +0.4% |
| 35 | 487176ns | +0.4% | +1.5% | +1.0% | +0.1% |
| 36 | 493514ns | -1.6% | +0.2% | -0.2% | -0.8% |
| 37 | 487814ns | +5.3% | +1.4% | +1.3% | +0.4% |
| 38 | 492620ns | +1.7% | +0.4% | +0.5% | -0.3% |
| 39 | 495481ns | -1.2% | +0.6% | -0.1% | -1.2% |
| 40 | 482420ns | +1.5% | +2.7% | +3.4% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.139 | ok |
| wide-rung-ragged | 0.281 | moderate+ |
| wide-rung-ragged-overread | 0.072 | ok |
| wide-rung-wordround | 0.358 | moderate+ |
| wide-rung-wordround-alias | 0.048 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 7/40, lost 33/40
- **wide-rung-ragged-overread**: won 6/40, lost 33/40
- **wide-rung-wordround**: won 12/40, lost 26/40
- **wide-rung-wordround-alias**: won 13/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 5.7ns | 489270.4ns | 0.0% |  |
| wide-rung-ragged | 11.4ns | 498401.2ns | 0.0% |  |
| wide-rung-ragged-overread | 5.3ns | 495612.7ns | 0.0% |  |
| wide-rung-wordround | 5.9ns | 492371.6ns | 0.0% |  |
| wide-rung-wordround-alias | 5.5ns | 492213.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 480213.9-500870.7 ns)
  480213.9 |################
  481246.7 |########
  482279.6 |########
  483312.4 |################
  484345.3 |########
  485378.1 |################
  486410.9 |########################
  487443.8 |########################################
  488476.6 |########################
  489509.5 |################################
  490542.3 |
  491575.1 |################
  492608.0 |################
  493640.8 |########
  494673.7 |########################
  495706.5 |
  496739.4 |
  497772.2 |
  498805.0 |
  499837.9 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 489526.4-514431.5 ns)
  489526.4 |###########
  490771.7 |#################
  492016.9 |############################
  493262.2 |######################
  494507.4 |########################################
  495752.7 |###########
  496997.9 |
  498243.2 |
  499488.4 |###########
  500733.7 |#####
  501979.0 |#####
  503224.2 |
  504469.5 |
  505714.7 |###########
  506960.0 |#####
  508205.2 |#####
  509450.5 |
  510695.7 |
  511941.0 |#####
  513186.3 |#####
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 492622.7-499484.5 ns)
  492622.7 |################
  492965.7 |########
  493308.8 |########
  493651.9 |
  493995.0 |################################
  494338.1 |########################################
  494681.2 |########################
  495024.3 |########################
  495367.4 |########################
  495710.5 |########
  496053.6 |################
  496396.6 |################
  496739.7 |################
  497082.8 |################
  497425.9 |########
  497769.0 |
  498112.1 |########
  498455.2 |########
  498798.3 |
  499141.4 |
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 485099.6-500194.4 ns)
  485099.6 |########################
  485854.3 |
  486609.1 |########################
  487363.8 |################
  488118.5 |########
  488873.3 |################
  489628.0 |########
  490382.8 |########
  491137.5 |########
  491892.3 |################################
  492647.0 |########
  493401.7 |########
  494156.5 |################################
  494911.2 |########################################
  495666.0 |########
  496420.7 |
  497175.4 |################
  497930.2 |########
  498684.9 |########
  499439.7 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 484836.9-503644.4 ns)
  484836.9 |########
  485777.3 |########
  486717.7 |########################
  487658.0 |################################
  488598.4 |########################################
  489538.8 |########################
  490479.2 |########################
  491419.5 |
  492359.9 |################
  493300.3 |########################
  494240.7 |########
  495181.0 |########
  496121.4 |########################
  497061.8 |################
  498002.2 |
  498942.6 |
  499882.9 |
  500823.3 |
  501763.7 |########
  502704.1 |
  (4 below, 3 above range)

```
