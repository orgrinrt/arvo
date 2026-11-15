# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 933.67 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 933665.8 ns median (-12.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.14x (fastest 933665.8 ns, slowest 1063851.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1077074ns | 1064649ns | 1060482ns | 1069112ns | 1117550ns | base |
| wide-rung-ragged | 929257ns | 945684ns | 828587ns | 921608ns | 1052872ns | -13.72% |
| wide-rung-ragged-overread | 945327ns | 933875ns | 888054ns | 941987ns | 1012619ns | -12.23% |
| wide-rung-wordround | 1045346ns | 1022236ns | 979094ns | 1029158ns | 1160159ns | -2.95% |
| wide-rung-wordround-alias | 1033915ns | 989151ns | 970981ns | 1020180ns | 1138053ns | -4.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1076474ns | 1059935ns | 1117007ns | base | 1.705 |
| wide-rung-ragged | 929012ns | 828415ns | 1052565ns | -13.70% | 1.975 |
| wide-rung-ragged-overread | 945071ns | 887859ns | 1012302ns | -12.21% | 1.942 |
| wide-rung-wordround | 1044900ns | 978630ns | 1159739ns | -2.93% | 1.756 |
| wide-rung-wordround-alias | 1033478ns | 970506ns | 1137682ns | -3.99% | 1.776 |

## Performance model

- Peak throughput: **2.215 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.725 | 77.9% |
| wide-rung-ragged | 1.941 | 87.6% |
| wide-rung-ragged-overread | 1.965 | 88.7% |
| wide-rung-wordround | 1.796 | 81.1% |
| wide-rung-wordround-alias | 1.856 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1077074ns | 1077074ns | base |
| wide-rung-ragged | 929257ns | 929257ns | -13.72% |
| wide-rung-ragged-overread | 945327ns | 945327ns | -12.23% |
| wide-rung-wordround | 1045346ns | 1045346ns | -2.95% |
| wide-rung-wordround-alias | 1033915ns | 1033915ns | -4.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1063852ns | base | --- | [1061547, 1065293] | --- | --- | --- | --- |
| wide-rung-ragged | 945488ns | -170862.4ns (-16.1%) | [-211735, -164846]ns | [850370, 950852] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 933666ns | -129432.6ns (-12.2%) | [-131571, -127905]ns | [932909, 941970] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 1021814ns | -44211.2ns (-4.2%) | [-49084, -38463]ns | [1013409, 1025973] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 988595ns | -77918.2ns (-7.3%) | [-83469, -31683]ns | [984288, 1061264] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1060577ns | -21.9% | -10.6% | +9.3% | -8.6% |
| 1 | 2 | 1062121ns | -19.9% | -12.2% | -4.6% | -7.4% |
| 1 | 3 | 1064405ns | -2.2% | -12.3% | -3.7% | +6.8% |
| 1 | 4 | 1116271ns | -15.1% | -12.0% | -12.3% | -11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.044 | ok |
| wide-rung-ragged | 0.052 | ok |
| wide-rung-ragged-overread | -0.157 | ok |
| wide-rung-wordround | -0.092 | ok |
| wide-rung-wordround-alias | -0.323 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 128.3ns | 1076473.9ns | 0.0% |  |
| wide-rung-ragged | 79.4ns | 929012.2ns | 0.0% |  |
| wide-rung-ragged-overread | 53.5ns | 945070.7ns | 0.0% |  |
| wide-rung-wordround | 78.6ns | 1044900.0ns | 0.0% |  |
| wide-rung-wordround-alias | 98.2ns | 1033477.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1059935.3-1117007.0 ns)
  1059935.3 |########################################
  1062788.9 |########################
  1065642.5 |#####
  1068496.1 |##
  1071349.6 |
  1074203.2 |
  1077056.8 |
  1079910.4 |##
  1082764.0 |
  1085617.6 |
  1088471.2 |
  1091324.8 |
  1094178.3 |
  1097031.9 |
  1099885.5 |
  1102739.1 |
  1105592.7 |
  1108446.3 |
  1111299.9 |
  1114153.5 |#####################
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 828415.2-1052565.2 ns)
  828415.2 |##########################
  839622.7 |##########################
  850830.2 |########
  862037.7 |
  873245.2 |
  884452.7 |
  895660.2 |
  906867.7 |
  918075.2 |
  929282.7 |
  940490.2 |########################################
  951697.7 |####
  962905.2 |
  974112.7 |
  985320.2 |
  996527.7 |
  1007735.2 |
  1018942.7 |
  1030150.2 |###############################
  1041357.7 |#################
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 887858.5-1012302.3 ns)
  887858.5 |
  894080.7 |
  900302.9 |
  906525.1 |
  912747.3 |
  918969.5 |
  925191.7 |
  931413.9 |########################################
  937636.0 |##########
  943858.2 |##
  950080.4 |
  956302.6 |##
  962524.8 |
  968747.0 |
  974969.2 |
  981191.4 |##
  987413.5 |
  993635.7 |
  999857.9 |########
  1006080.1 |
  (3 below, 5 above range)

wide-rung-wordround (n=40, range 978630.1-1159738.5 ns)
  978630.1 |####################
  987685.5 |
  996741.0 |
  1005796.4 |################################
  1014851.8 |########
  1023907.2 |########################################
  1032962.6 |
  1042018.1 |
  1051073.5 |
  1060128.9 |
  1069184.3 |
  1078239.8 |
  1087295.2 |
  1096350.6 |
  1105406.0 |
  1114461.4 |
  1123516.9 |
  1132572.3 |
  1141627.7 |
  1150683.1 |####################################
  (5 below, 1 above range)

wide-rung-wordround-alias (n=40, range 970506.4-1137682.3 ns)
  970506.4 |###
  978865.2 |########################################
  987224.0 |##############
  995582.7 |
  1003941.5 |
  1012300.3 |
  1020659.1 |#######
  1029017.9 |
  1037376.7 |
  1045735.5 |
  1054094.3 |##########
  1062453.1 |###
  1070811.9 |
  1079170.7 |##########
  1087529.5 |
  1095888.3 |
  1104247.1 |
  1112605.9 |
  1120964.7 |
  1129323.5 |#########################
  (6 below, 2 above range)

```
