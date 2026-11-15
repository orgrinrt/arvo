# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.03 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 901.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Speed leader wide-rung-ragged vs stability leader wide-rung-wordround (+7% speed for 1.6x steadier)

wide-rung-ragged is fastest (901.74 us, CV 5.3%); wide-rung-wordround gives up 7.0% median for 1.6x lower variance (CV 3.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged** at 901742.0 ns median (-12.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.15x (fastest 901742.0 ns, slowest 1034170.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1047899ns | 1034624ns | 953446ns | 1043083ns | 1156797ns | base |
| wide-rung-ragged | 891275ns | 902097ns | 818230ns | 898855ns | 941577ns | -14.95% |
| wide-rung-ragged-overread | 978419ns | 939410ns | 929783ns | 957050ns | 1091163ns | -6.63% |
| wide-rung-wordround | 974748ns | 965008ns | 948571ns | 966461ns | 1025786ns | -6.98% |
| wide-rung-wordround-alias | 984886ns | 975123ns | 929197ns | 985187ns | 1039671ns | -6.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1047412ns | 952952ns | 1156367ns | base | 1.752 |
| wide-rung-ragged | 890845ns | 817890ns | 940987ns | -14.95% | 2.060 |
| wide-rung-ragged-overread | 978024ns | 929356ns | 1090703ns | -6.62% | 1.876 |
| wide-rung-wordround | 974365ns | 948166ns | 1025373ns | -6.97% | 1.883 |
| wide-rung-wordround-alias | 984448ns | 928728ns | 1039185ns | -6.01% | 1.864 |

## Performance model

- Peak throughput: **2.244 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.774 | 79.1% |
| wide-rung-ragged | 2.035 | 90.7% |
| wide-rung-ragged-overread | 1.954 | 87.1% |
| wide-rung-wordround | 1.902 | 84.8% |
| wide-rung-wordround-alias | 1.882 | 83.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1047899ns | 1047899ns | base |
| wide-rung-ragged | 891275ns | 891275ns | -14.95% |
| wide-rung-ragged-overread | 978419ns | 978419ns | -6.63% |
| wide-rung-wordround | 974748ns | 974748ns | -6.98% |
| wide-rung-wordround-alias | 984886ns | 984886ns | -6.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1034170ns | base | --- | [1033292, 1036174] | --- | --- | --- | --- |
| wide-rung-ragged | 901742ns | -148485.6ns (-14.4%) | [-175837, -140292]ns | [886763, 902406] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 939002ns | -79216.6ns (-7.7%) | [-93854, -55413]ns | [933796, 973044] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 964562ns | -79035.2ns (-7.6%) | [-85611, -68734]ns | [951568, 966709] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 974835ns | no significant difference | [-128429, +176]ns | [971880, 994990] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 953250ns | -12.5% | -2.4% | -0.2% | +2.5% |
| 1 | 2 | 1033601ns | -14.3% | -9.2% | -6.6% | -6.0% |
| 1 | 3 | 1034539ns | -12.8% | -9.9% | -8.2% | -6.2% |
| 1 | 4 | 1154902ns | -19.5% | -6.1% | -12.8% | -11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.000 | ok |
| wide-rung-ragged | 0.140 | ok |
| wide-rung-ragged-overread | -0.109 | ok |
| wide-rung-wordround | -0.287 | moderate- |
| wide-rung-wordround-alias | -0.118 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 75.3ns | 1047412.3ns | 0.0% |  |
| wide-rung-ragged | 104.4ns | 890845.2ns | 0.0% |  |
| wide-rung-ragged-overread | 78.6ns | 978024.2ns | 0.0% |  |
| wide-rung-wordround | 79.7ns | 974364.5ns | 0.0% |  |
| wide-rung-wordround-alias | 107.6ns | 984448.3ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 952952.3-1156367.1 ns)
  952952.3 |#####################
  963123.1 |
  973293.8 |
  983464.5 |
  993635.3 |
  1003806.0 |
  1013976.8 |
  1024147.5 |########################################
  1034318.2 |#############################
  1044489.0 |
  1054659.7 |
  1064830.5 |
  1075001.2 |
  1085171.9 |
  1095342.7 |
  1105513.4 |
  1115684.2 |
  1125854.9 |
  1136025.6 |###
  1146196.4 |#############################
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 817889.6-940987.2 ns)
  817889.6 |
  824044.5 |
  830199.4 |
  836354.3 |
  842509.1 |
  848664.0 |
  854818.9 |
  860973.8 |
  867128.7 |
  873283.5 |
  879438.4 |############
  885593.3 |########################################
  891748.2 |####
  897903.1 |########################################
  904057.9 |
  910212.8 |
  916367.7 |
  922522.6 |####################
  928677.5 |########
  934832.3 |########
  (5 below, 2 above range)

wide-rung-ragged-overread (n=40, range 929356.3-1090702.7 ns)
  929356.3 |########################################
  937423.6 |####################################
  945490.9 |
  953558.2 |
  961625.6 |###
  969692.9 |
  977760.2 |###
  985827.5 |##########
  993894.8 |
  1001962.2 |
  1010029.5 |
  1018096.8 |
  1026164.1 |
  1034231.4 |
  1042298.7 |
  1050366.1 |
  1058433.4 |#######
  1066500.7 |#######
  1074568.0 |###
  1082635.3 |
  (4 below, 5 above range)

wide-rung-wordround (n=40, range 948166.2-1025373.2 ns)
  948166.2 |########################################
  952026.5 |######
  955886.9 |
  959747.2 |######
  963607.6 |##########################
  967467.9 |###
  971328.3 |
  975188.6 |
  979049.0 |
  982909.3 |
  986769.7 |
  990630.0 |
  994490.4 |
  998350.7 |######
  1002211.1 |##########
  1006071.4 |#############
  1009931.8 |
  1013792.1 |
  1017652.5 |
  1021512.8 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 928727.6-1039185.2 ns)
  928727.6 |
  934250.5 |
  939773.4 |
  945296.3 |
  950819.1 |
  956342.0 |
  961864.9 |###
  967387.8 |########################################
  972910.7 |#############################
  978433.5 |###
  983956.4 |
  989479.3 |###
  995002.2 |
  1000525.0 |
  1006047.9 |
  1011570.8 |###
  1017093.7 |
  1022616.6 |#####################
  1028139.4 |###
  1033662.3 |##########
  (5 below, 2 above range)

```
