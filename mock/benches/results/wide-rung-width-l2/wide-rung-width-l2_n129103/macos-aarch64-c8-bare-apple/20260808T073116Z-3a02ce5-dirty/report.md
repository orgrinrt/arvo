# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 907049.6 ns median (-2.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 907049.6 ns, slowest 961537.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 954415ns | 934596ns | 922264ns | 934300ns | 1046910ns | base |
| wide-rung-ragged | 964828ns | 962352ns | 954803ns | 963186ns | 979778ns | +1.09% |
| wide-rung-ragged-overread | 910437ns | 907854ns | 900238ns | 908418ns | 926692ns | -4.61% |
| wide-rung-wordround | 938605ns | 922064ns | 919210ns | 922658ns | 1005842ns | -1.66% |
| wide-rung-wordround-alias | 932147ns | 926095ns | 920918ns | 927354ns | 957752ns | -2.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 953379ns | 921594ns | 1045380ns | base | 1.925 |
| wide-rung-ragged | 964042ns | 953825ns | 979254ns | +1.12% | 1.903 |
| wide-rung-ragged-overread | 909733ns | 899436ns | 926239ns | -4.58% | 2.017 |
| wide-rung-wordround | 937680ns | 918260ns | 1004945ns | -1.65% | 1.957 |
| wide-rung-wordround-alias | 931187ns | 919798ns | 956829ns | -2.33% | 1.971 |

## Performance model

- Peak throughput: **2.040 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.966 | 96.3% |
| wide-rung-ragged | 1.908 | 93.5% |
| wide-rung-ragged-overread | 2.023 | 99.2% |
| wide-rung-wordround | 1.992 | 97.6% |
| wide-rung-wordround-alias | 1.983 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 954415ns | 954415ns | base |
| wide-rung-ragged | 964828ns | 964828ns | +1.09% |
| wide-rung-ragged-overread | 910437ns | 910437ns | -4.61% |
| wide-rung-wordround | 938605ns | 938605ns | -1.66% |
| wide-rung-wordround-alias | 932147ns | 932147ns | -2.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 933531ns | base | --- | [931862, 935905] | --- | --- | --- | --- |
| wide-rung-ragged | 961538ns | +30501.7ns (+3.3%) | [+26955, +35682]ns | [957336, 967708] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 907050ns | -24407.9ns (-2.6%) | [-27079, -20411]ns | [905273, 910716] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 921099ns | -10430.8ns (-1.1%) | [-12392, -8009]ns | [920029, 924117] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 925180ns | no significant difference | [-9713, +337]ns | [923682, 928605] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 921779ns | +4.1% | -2.2% | -0.1% | +0.5% |
| 2 | 928467ns | +5.8% | -2.7% | -0.9% | +0.0% |
| 3 | 933512ns | +2.9% | -3.4% | -1.2% | +0.1% |
| 4 | 938268ns | +3.8% | -4.3% | -1.4% | -0.9% |
| 5 | 939689ns | +3.1% | -4.4% | +60.4% | -2.4% |
| 6 | 1376655ns | -29.7% | -33.1% | -33.2% | -33.2% |
| 7 | 1326598ns | -28.0% | -30.9% | -30.1% | -30.8% |
| 8 | 936625ns | +1.9% | -2.6% | -0.4% | +0.3% |
| 9 | 936562ns | +2.5% | -3.3% | -0.9% | -1.9% |
| 10 | 923120ns | +4.0% | -2.7% | +0.5% | -0.4% |
| 11 | 932306ns | +2.6% | -2.3% | -1.2% | -0.1% |
| 12 | 935755ns | +2.8% | -3.8% | -1.2% | -1.3% |
| 13 | 927421ns | +4.6% | -2.2% | -0.3% | +0.8% |
| 14 | 932119ns | +4.4% | -2.8% | -1.1% | -0.8% |
| 15 | 932120ns | +3.8% | +0.5% | -1.2% | -0.5% |
| 16 | 932042ns | +2.4% | -2.6% | -1.4% | -1.0% |
| 17 | 921779ns | +3.6% | -2.5% | -0.5% | +0.3% |
| 18 | 916998ns | +4.3% | -1.5% | +0.4% | +0.6% |
| 19 | 922557ns | +3.6% | +0.2% | -0.5% | +0.1% |
| 20 | 932821ns | +3.3% | -1.1% | -0.9% | -1.0% |
| 21 | 933550ns | +3.5% | -2.8% | -0.9% | -0.5% |
| 22 | 940517ns | +5.9% | -3.5% | -1.6% | -1.1% |
| 23 | 928270ns | +3.0% | -2.0% | -1.0% | +2.0% |
| 24 | 918680ns | +5.4% | -1.8% | +0.0% | +9.0% |
| 25 | 923824ns | +5.6% | -1.9% | -0.6% | +2.7% |
| 26 | 931242ns | +4.8% | -2.7% | -1.4% | +6.5% |
| 27 | 934316ns | +3.3% | -1.6% | -1.4% | -1.5% |
| 28 | 936056ns | +2.7% | -1.8% | -1.6% | -1.4% |
| 29 | 935308ns | +2.0% | -2.5% | -1.6% | +0.6% |
| 30 | 931683ns | +3.2% | -3.4% | -1.3% | +1.7% |
| 31 | 939753ns | +3.0% | -0.4% | -2.2% | -1.6% |
| 32 | 936383ns | +2.3% | -0.7% | -1.8% | -1.4% |
| 33 | 927569ns | +3.2% | -1.9% | -0.8% | -0.2% |
| 34 | 924017ns | +5.0% | -1.8% | -0.6% | +0.1% |
| 35 | 934142ns | +4.7% | -3.1% | -1.7% | -0.9% |
| 36 | 940628ns | +4.0% | -4.0% | -2.2% | -0.3% |
| 37 | 938005ns | +1.7% | -3.7% | +1.3% | -1.6% |
| 38 | 939348ns | +1.5% | -3.0% | -0.4% | -1.6% |
| 39 | 934826ns | +1.8% | -2.4% | -1.2% | -1.3% |
| 40 | 959852ns | -0.7% | -5.5% | -3.1% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.483 | moderate+ |
| wide-rung-ragged | 0.210 | moderate+ |
| wide-rung-ragged-overread | 0.206 | moderate+ |
| wide-rung-wordround | -0.026 | ok |
| wide-rung-wordround-alias | 0.310 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 3/40, lost 37/40
- **wide-rung-ragged-overread**: won 38/40, lost 2/40
- **wide-rung-wordround**: won 34/40, lost 4/40
- **wide-rung-wordround-alias**: won 25/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 23.5ns | 953379.1ns | 0.0% |  |
| wide-rung-ragged | 9.0ns | 964041.5ns | 0.0% |  |
| wide-rung-ragged-overread | 8.5ns | 909732.9ns | 0.0% |  |
| wide-rung-wordround | 12.4ns | 937680.3ns | 0.0% |  |
| wide-rung-wordround-alias | 12.8ns | 931187.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 921594.2-1045379.9 ns)
  921594.2 |######################
  927783.5 |###############################
  933972.8 |########################################
  940162.1 |#####
  946351.4 |
  952540.7 |
  958729.9 |##
  964919.2 |
  971108.5 |
  977297.8 |
  983487.1 |
  989676.4 |
  995865.7 |
  1002054.9 |
  1008244.2 |
  1014433.5 |
  1020622.8 |
  1026812.1 |
  1033001.4 |
  1039190.7 |
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 953825.0-979253.7 ns)
  953825.0 |########################################
  955096.4 |########################################
  956367.8 |########
  957639.3 |########
  958910.7 |################
  960182.1 |########################
  961453.6 |################
  962725.0 |########
  963996.4 |
  965267.9 |################
  966539.3 |########
  967810.7 |################################
  969082.2 |########
  970353.6 |########
  971625.0 |
  972896.5 |########
  974167.9 |################
  975439.3 |########
  976710.8 |
  977982.2 |################
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 899436.0-926238.5 ns)
  899436.0 |#############
  900776.2 |#############
  902116.3 |####################
  903456.4 |#############
  904796.5 |####################
  906136.7 |########################################
  907476.8 |#############
  908816.9 |#############
  910157.0 |######
  911497.2 |##########################
  912837.3 |
  914177.4 |
  915517.5 |######
  916857.7 |
  918197.8 |######
  919537.9 |######
  920878.0 |######
  922218.2 |######
  923558.3 |######
  924898.4 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 918260.2-1004945.2 ns)
  918260.2 |########################################
  922594.5 |############
  926928.7 |#######
  931263.0 |###
  935597.2 |
  939931.5 |
  944265.7 |
  948600.0 |#
  952934.2 |
  957268.5 |
  961602.7 |
  965937.0 |
  970271.2 |
  974605.5 |
  978939.7 |
  983274.0 |
  987608.2 |
  991942.5 |
  996276.7 |
  1000611.0 |
  (3 below, 1 above range)

wide-rung-wordround-alias (n=40, range 919798.2-956828.7 ns)
  919798.2 |########
  921649.7 |###############################
  923501.3 |########################################
  925352.8 |#############
  927204.3 |########
  929055.8 |########
  930907.4 |####
  932758.9 |
  934610.4 |########
  936461.9 |####
  938313.5 |####
  940165.0 |####
  942016.5 |
  943868.0 |
  945719.6 |########
  947571.1 |####
  949422.6 |
  951274.1 |
  953125.7 |
  954977.2 |
  (4 below, 2 above range)

```
