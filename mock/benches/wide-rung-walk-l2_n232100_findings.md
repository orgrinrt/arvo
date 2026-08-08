# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 2.3% of the fastest

All 5 variants sit between 486.15 us and 497.43 us - a 2.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-wordround** at 486145.4 ns median (-1.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 486145.4 ns, slowest 497430.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 497377ns | 495983ns | 489275ns | 496386ns | 508453ns | base |
| wide-rung-ragged | 500465ns | 498365ns | 494858ns | 498396ns | 512277ns | +0.62% |
| wide-rung-ragged-overread | 497817ns | 497678ns | 493196ns | 497853ns | 502330ns | +0.09% |
| wide-rung-wordround | 488442ns | 486775ns | 479109ns | 486845ns | 502565ns | -1.80% |
| wide-rung-wordround-alias | 491083ns | 488954ns | 482478ns | 488840ns | 506417ns | -1.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 496239ns | 488169ns | 507241ns | base | 0.924 |
| wide-rung-ragged | 499544ns | 493910ns | 511432ns | +0.67% | 0.918 |
| wide-rung-ragged-overread | 497029ns | 492503ns | 501481ns | +0.16% | 0.923 |
| wide-rung-wordround | 487628ns | 478321ns | 501674ns | -1.74% | 0.941 |
| wide-rung-wordround-alias | 490097ns | 481511ns | 505420ns | -1.24% | 0.936 |

## Performance model

- Peak throughput: **0.959 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.927 | 96.7% |
| wide-rung-ragged | 0.922 | 96.2% |
| wide-rung-ragged-overread | 0.923 | 96.2% |
| wide-rung-wordround | 0.944 | 98.4% |
| wide-rung-wordround-alias | 0.940 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 497377ns | 497377ns | base |
| wide-rung-ragged | 500465ns | 500465ns | +0.62% |
| wide-rung-ragged-overread | 497817ns | 497817ns | +0.09% |
| wide-rung-wordround | 488442ns | 488442ns | -1.80% |
| wide-rung-wordround-alias | 491083ns | 491083ns | -1.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 494821ns | base | --- | [492495, 497244] | --- | --- | --- | --- |
| wide-rung-ragged | 497430ns | +4327.1ns (+0.9%) | [+538, +5861]ns | [495866, 498412] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-ragged-overread | 497009ns | no significant difference | [-2023, +5570]ns | [495757, 498207] | no | 0.2682 | 0.2682 | 0 |
| wide-rung-wordround | 486145ns | -8368.1ns (-1.7%) | [-12543, -3721]ns | [482818, 489226] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 488148ns | -6819.2ns (-1.4%) | [-9431, -5076]ns | [486805, 489059] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 502959ns | +0.9% | -0.7% | -2.5% | -3.1% |
| 2 | 495929ns | +1.4% | +0.5% | -0.6% | -1.2% |
| 3 | 493820ns | +1.9% | +1.2% | -0.8% | -0.6% |
| 4 | 500302ns | +6.3% | -0.0% | -2.1% | -2.6% |
| 5 | 493340ns | +1.0% | +1.1% | +1.1% | -1.0% |
| 6 | 492351ns | +1.3% | +2.0% | +1.7% | -1.1% |
| 7 | 490036ns | +1.7% | +2.0% | -0.3% | -0.3% |
| 8 | 493742ns | +0.9% | +0.9% | -0.5% | -0.9% |
| 9 | 492189ns | +1.1% | +1.5% | -1.2% | -0.7% |
| 10 | 492542ns | +0.9% | +1.4% | -1.0% | +0.8% |
| 11 | 504942ns | -0.9% | -1.9% | -4.4% | -4.5% |
| 12 | 500896ns | -1.1% | -1.2% | -3.9% | -2.8% |
| 13 | 501713ns | -1.5% | -1.3% | +0.1% | -2.9% |
| 14 | 500482ns | -0.1% | -0.9% | +4.1% | -3.0% |
| 15 | 494887ns | +0.0% | +0.5% | -2.3% | -2.4% |
| 16 | 496359ns | -0.2% | +0.3% | -2.8% | +5.9% |
| 17 | 495400ns | +0.2% | +0.4% | -0.4% | +1.0% |
| 18 | 494543ns | +0.6% | +0.2% | -2.9% | -1.2% |
| 19 | 502397ns | -1.2% | -1.7% | -3.8% | -2.7% |
| 20 | 491105ns | +0.9% | +3.1% | -0.5% | +0.5% |
| 21 | 515735ns | -3.9% | -4.0% | -6.8% | -4.4% |
| 22 | 511055ns | -3.2% | -3.2% | -5.5% | -5.2% |
| 23 | 492448ns | +0.6% | +0.6% | -2.3% | -2.0% |
| 24 | 495785ns | -0.4% | -0.4% | -2.7% | -1.9% |
| 25 | 486165ns | +1.5% | +1.1% | -0.9% | -0.9% |
| 26 | 484444ns | +1.5% | +2.1% | -0.3% | -0.7% |
| 27 | 491416ns | +0.5% | +1.5% | -1.8% | -1.9% |
| 28 | 489415ns | +1.0% | +2.3% | -2.6% | -1.2% |
| 29 | 488049ns | +1.9% | +2.9% | -0.4% | -1.6% |
| 30 | 486989ns | +11.3% | +1.6% | +0.7% | -1.3% |
| 31 | 516548ns | -4.0% | -3.0% | -5.6% | -5.2% |
| 32 | 500965ns | -1.1% | -0.9% | -2.2% | -2.6% |
| 33 | 498129ns | +0.6% | -0.5% | -0.7% | -1.8% |
| 34 | 489695ns | +2.0% | +1.5% | -1.6% | -1.7% |
| 35 | 494754ns | +1.3% | +0.4% | -4.0% | -1.6% |
| 36 | 495085ns | +1.3% | +0.5% | +2.9% | -1.0% |
| 37 | 490898ns | +1.9% | -1.3% | -2.7% | +4.5% |
| 38 | 502580ns | -1.0% | -1.2% | -5.8% | -1.5% |
| 39 | 498899ns | -0.1% | -1.4% | -2.5% | +0.2% |
| 40 | 490557ns | +1.6% | +1.2% | -1.1% | +5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.211 | moderate+ |
| wide-rung-ragged | 0.063 | ok |
| wide-rung-ragged-overread | 0.125 | ok |
| wide-rung-wordround | 0.132 | ok |
| wide-rung-wordround-alias | 0.288 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 13/40, lost 26/40
- **wide-rung-ragged-overread**: won 15/40, lost 24/40
- **wide-rung-wordround**: won 34/40, lost 6/40
- **wide-rung-wordround-alias**: won 33/40, lost 7/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.6ns | 496238.7ns | 0.0% |  |
| wide-rung-ragged | 8.1ns | 499544.3ns | 0.0% |  |
| wide-rung-ragged-overread | 5.4ns | 497028.5ns | 0.0% |  |
| wide-rung-wordround | 5.3ns | 487627.9ns | 0.0% |  |
| wide-rung-wordround-alias | 6.5ns | 490096.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 488168.7-507241.2 ns)
  488168.7 |
  489122.3 |##############################
  490076.0 |####################
  491029.6 |####################
  491983.2 |########################################
  492936.8 |##############################
  493890.5 |####################
  494844.1 |########################################
  495797.7 |####################
  496751.3 |
  497705.0 |##########
  498658.6 |##########
  499612.2 |####################
  500565.8 |####################
  501519.5 |####################
  502473.1 |####################
  503426.7 |
  504380.3 |##########
  505334.0 |
  506287.6 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 493909.7-511431.6 ns)
  493909.7 |################################
  494785.8 |########################################
  495661.9 |################################
  496538.0 |################################
  497414.1 |################################
  498290.2 |################################
  499166.3 |################
  500042.4 |################
  500918.4 |########################
  501794.5 |
  502670.6 |################
  503546.7 |
  504422.8 |
  505298.9 |
  506175.0 |
  507051.1 |########
  507927.2 |
  508803.3 |
  509679.4 |
  510555.5 |
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 492502.8-501480.8 ns)
  492502.8 |
  492951.7 |
  493400.6 |
  493849.5 |####################
  494298.4 |####################
  494747.3 |##############################
  495196.2 |##############################
  495645.1 |##############################
  496094.0 |####################
  496542.9 |####################
  496991.8 |##############################
  497440.7 |####################
  497889.6 |####################
  498338.5 |
  498787.4 |####################
  499236.3 |########################################
  499685.2 |##########
  500134.1 |####################
  500583.0 |
  501031.9 |##########
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 478320.6-501673.5 ns)
  478320.6 |
  479488.2 |######
  480655.9 |##########################
  481823.5 |########################################
  482991.2 |####################
  484158.8 |######
  485326.5 |#############
  486494.1 |######
  487661.8 |##########################
  488829.4 |#############
  489997.0 |####################
  491164.7 |######
  492332.3 |######
  493500.0 |#############
  494667.6 |
  495835.3 |
  497002.9 |
  498170.6 |######
  499338.2 |
  500505.9 |######
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 481510.7-505419.6 ns)
  481510.7 |##########################
  482706.1 |#############
  483901.6 |######
  485097.0 |######
  486292.4 |########################################
  487487.9 |#################################
  488683.3 |########################################
  489878.8 |#############
  491074.2 |
  492269.7 |######
  493465.1 |######
  494660.6 |######
  495856.0 |######
  497051.4 |
  498246.9 |
  499442.3 |#############
  500637.8 |
  501833.2 |
  503028.7 |
  504224.1 |
  (4 below, 3 above range)

```
