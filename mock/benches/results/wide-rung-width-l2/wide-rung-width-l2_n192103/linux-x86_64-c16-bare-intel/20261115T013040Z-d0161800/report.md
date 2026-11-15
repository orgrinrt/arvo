# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 916.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (916.08 us) and wide-rung-wordround (921.10 us) differ by 0.55%, inside the noise, even though the wider field spreads 9.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 916078.4 ns median (-8.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.09x (fastest 916078.4 ns, slowest 1001024.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1024398ns | 1001704ns | 921291ns | 1020983ns | 1137749ns | base |
| wide-rung-ragged | 953077ns | 943082ns | 865712ns | 948930ns | 1052885ns | -6.96% |
| wide-rung-ragged-overread | 922668ns | 965478ns | 807665ns | 942132ns | 979279ns | -9.93% |
| wide-rung-wordround | 923552ns | 921477ns | 899622ns | 922189ns | 951572ns | -9.84% |
| wide-rung-wordround-alias | 927577ns | 916474ns | 902014ns | 920886ns | 973217ns | -9.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1023821ns | 920778ns | 1137195ns | base | 1.792 |
| wide-rung-ragged | 952678ns | 865309ns | 1052491ns | -6.95% | 1.926 |
| wide-rung-ragged-overread | 922207ns | 807318ns | 978884ns | -9.92% | 1.990 |
| wide-rung-wordround | 923137ns | 899126ns | 951172ns | -9.83% | 1.988 |
| wide-rung-wordround-alias | 927196ns | 901691ns | 972862ns | -9.44% | 1.979 |

## Performance model

- Peak throughput: **2.273 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.833 | 80.6% |
| wide-rung-ragged | 1.946 | 85.6% |
| wide-rung-ragged-overread | 1.902 | 83.7% |
| wide-rung-wordround | 1.992 | 87.6% |
| wide-rung-wordround-alias | 2.003 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1024398ns | 1024398ns | base |
| wide-rung-ragged | 953077ns | 953077ns | -6.96% |
| wide-rung-ragged-overread | 922668ns | 922668ns | -9.93% |
| wide-rung-wordround | 923552ns | 923552ns | -9.84% |
| wide-rung-wordround-alias | 927577ns | 927577ns | -9.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1001024ns | base | --- | [998340, 1102676] | --- | --- | --- | --- |
| wide-rung-ragged | 942724ns | -85574.7ns (-8.5%) | [-88233, -34795]ns | [913823, 948438] | YES | 0.0166 | 0.0166 | 0 |
| wide-rung-ragged-overread | 964968ns | -120032.6ns (-12.0%) | [-139280, -94531]ns | [906469, 966476] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 921099ns | -97694.8ns (-9.8%) | [-185355, -54073]ns | [917918, 923066] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 916078ns | -88181.1ns (-8.8%) | [-128009, -26510]ns | [914296, 917163] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1001393ns | +5.0% | -3.5% | -10.1% | -3.0% |
| 1 | 2 | 1136034ns | -17.0% | -13.9% | -18.9% | -19.3% |
| 1 | 3 | 1001024ns | -8.8% | -9.5% | -5.5% | -9.9% |
| 1 | 4 | 926993ns | -6.5% | -12.9% | -1.0% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.099 | ok |
| wide-rung-ragged | 0.126 | ok |
| wide-rung-ragged-overread | 0.200 | moderate+ |
| wide-rung-wordround | -0.072 | ok |
| wide-rung-wordround-alias | 0.026 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 86.2ns | 1023820.6ns | 0.0% |  |
| wide-rung-ragged | 85.0ns | 952677.6ns | 0.0% |  |
| wide-rung-ragged-overread | 72.4ns | 922207.2ns | 0.0% |  |
| wide-rung-wordround | 140.6ns | 923136.6ns | 0.0% |  |
| wide-rung-wordround-alias | 110.3ns | 927195.9ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 920777.9-1137195.3 ns)
  920777.9 |##############################
  931598.7 |###
  942419.6 |
  953240.5 |
  964061.4 |
  974882.2 |
  985703.1 |
  996524.0 |########################################
  1007344.8 |
  1018165.7 |
  1028986.6 |
  1039807.4 |
  1050628.3 |
  1061449.2 |
  1072270.1 |
  1083090.9 |###
  1093911.8 |
  1104732.7 |
  1115553.5 |##########
  1126374.4 |##############################
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 865309.4-1052491.5 ns)
  865309.4 |
  874668.5 |
  884027.6 |###
  893386.7 |
  902745.8 |##########
  912104.9 |#########################
  921464.0 |
  930823.1 |
  940182.2 |########################################
  949541.3 |###
  958900.4 |###
  968259.5 |
  977618.6 |
  986977.7 |
  996336.8 |
  1005695.9 |
  1015055.1 |
  1024414.2 |
  1033773.3 |
  1043132.4 |################################
  (5 below, 2 above range)

wide-rung-ragged-overread (n=40, range 807318.2-978884.3 ns)
  807318.2 |##########
  815896.5 |
  824474.8 |
  833053.1 |
  841631.4 |
  850209.7 |
  858788.0 |
  867366.3 |
  875944.6 |
  884523.0 |
  893101.3 |
  901679.6 |####################################
  910257.9 |
  918836.2 |
  927414.5 |
  935992.8 |
  944571.1 |
  953149.4 |###
  961727.7 |########################################
  970306.0 |#########################
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 899125.9-951172.4 ns)
  899125.9 |###############################
  901728.3 |
  904330.6 |
  906932.9 |
  909535.2 |
  912137.5 |
  914739.9 |####
  917342.2 |##########################
  919944.5 |########################################
  922546.8 |#############
  925149.1 |
  927751.5 |
  930353.8 |
  932956.1 |
  935558.4 |
  938160.8 |
  940763.1 |####
  943365.4 |#################
  945967.7 |#################
  948570.0 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 901690.8-972862.4 ns)
  901690.8 |##########################
  905249.4 |
  908808.0 |####
  912366.6 |########################################
  915925.2 |###################################
  919483.7 |########
  923042.3 |
  926600.9 |
  930159.5 |####
  933718.0 |
  937276.6 |
  940835.2 |
  944393.8 |
  947952.3 |
  951510.9 |
  955069.5 |
  958628.1 |
  962186.6 |
  965745.2 |
  969303.8 |###############################
  (3 below, 3 above range)

```
