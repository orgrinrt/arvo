# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged-overread, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-ragged-overread (906.84 us) and wide-rung-wordround-alias (915.80 us) differ by 0.99%, inside the noise, even though the wider field spreads 4.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole field within 4.5% of the fastest

All 5 variants sit between 906.84 us and 947.96 us - a 4.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 906838.2 ns median (-1.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 906838.2 ns, slowest 947957.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 924554ns | 923156ns | 916986ns | 923547ns | 935145ns | base |
| wide-rung-ragged | 952581ns | 949110ns | 945161ns | 949980ns | 967806ns | +3.03% |
| wide-rung-ragged-overread | 908004ns | 907968ns | 902343ns | 907721ns | 914518ns | -1.79% |
| wide-rung-wordround | 918428ns | 917653ns | 913598ns | 917686ns | 925484ns | -0.66% |
| wide-rung-wordround-alias | 919723ns | 916940ns | 913866ns | 917502ns | 932241ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 923555ns | 916074ns | 934186ns | base | 1.987 |
| wide-rung-ragged | 951594ns | 944227ns | 966878ns | +3.04% | 1.928 |
| wide-rung-ragged-overread | 906998ns | 901298ns | 913607ns | -1.79% | 2.023 |
| wide-rung-wordround | 917417ns | 912538ns | 924594ns | -0.66% | 2.000 |
| wide-rung-wordround-alias | 918718ns | 912875ns | 931415ns | -0.52% | 1.997 |

## Performance model

- Peak throughput: **2.036 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.990 | 97.7% |
| wide-rung-ragged | 1.936 | 95.1% |
| wide-rung-ragged-overread | 2.024 | 99.4% |
| wide-rung-wordround | 2.002 | 98.3% |
| wide-rung-wordround-alias | 2.004 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 924554ns | 924554ns | base |
| wide-rung-ragged | 952581ns | 952581ns | +3.03% |
| wide-rung-ragged-overread | 908004ns | 908004ns | -1.79% |
| wide-rung-wordround | 918428ns | 918428ns | -0.66% |
| wide-rung-wordround-alias | 919723ns | 919723ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 922069ns | base | --- | [920460, 923369] | --- | --- | --- | --- |
| wide-rung-ragged | 947957ns | +27450.8ns (+3.0%) | [+24194, +30604]ns | [946896, 951086] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 906838ns | -15597.7ns (-1.7%) | [-18742, -13470]ns | [905401, 907892] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 916572ns | -5951.5ns (-0.6%) | [-8062, -4202]ns | [915921, 917425] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround-alias | 915804ns | -5369.3ns (-0.6%) | [-7965, -2181]ns | [914891, 918281] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 914352ns | +3.7% | -0.5% | +0.4% | +0.5% |
| 2 | 918054ns | +3.5% | -1.4% | +0.1% | -0.6% |
| 3 | 916887ns | +3.3% | -2.3% | +0.3% | -0.2% |
| 4 | 919124ns | +7.1% | -2.7% | -0.6% | -0.0% |
| 5 | 919136ns | +3.2% | -1.5% | -1.0% | +0.0% |
| 6 | 917444ns | +3.2% | -1.4% | +2.8% | +0.3% |
| 7 | 923140ns | +2.7% | -2.1% | -0.8% | -0.2% |
| 8 | 922930ns | +2.7% | -1.7% | -0.8% | +0.2% |
| 9 | 909766ns | +4.5% | -0.4% | +0.8% | +2.1% |
| 10 | 916626ns | +3.8% | -0.8% | -1.0% | +0.2% |
| 11 | 920820ns | +3.6% | +0.0% | -0.6% | -1.2% |
| 12 | 920885ns | +3.3% | -1.5% | -0.7% | -0.6% |
| 13 | 920081ns | +3.0% | -1.3% | -0.4% | -0.3% |
| 14 | 918419ns | +4.2% | -1.4% | -0.3% | -0.4% |
| 15 | 920064ns | +3.6% | -1.3% | -0.2% | -0.1% |
| 16 | 922088ns | +3.3% | -1.9% | -0.5% | -0.7% |
| 17 | 928686ns | +2.5% | -2.6% | -1.4% | -1.8% |
| 18 | 921739ns | +5.2% | -1.2% | -0.6% | -0.8% |
| 19 | 922050ns | +3.1% | -1.5% | -0.2% | -0.6% |
| 20 | 949180ns | -0.3% | -4.5% | -3.5% | -3.5% |
| 21 | 925990ns | +1.9% | -2.1% | -1.2% | +3.3% |
| 22 | 920092ns | +3.6% | -1.9% | -0.4% | +0.7% |
| 23 | 921298ns | +4.0% | -1.3% | -0.4% | +2.0% |
| 24 | 923391ns | +2.1% | -1.5% | +0.4% | +0.5% |
| 25 | 921806ns | +2.4% | -1.7% | -0.6% | -0.6% |
| 26 | 926299ns | +2.1% | -2.4% | -1.2% | -1.3% |
| 27 | 926911ns | +6.2% | -2.8% | -1.6% | -1.4% |
| 28 | 922514ns | +2.6% | -2.1% | -1.0% | -0.9% |
| 29 | 920101ns | +2.8% | -1.7% | -0.6% | +0.6% |
| 30 | 917040ns | +3.0% | -1.3% | +0.0% | -0.5% |
| 31 | 922742ns | +5.5% | -1.6% | -0.6% | -0.8% |
| 32 | 926639ns | +2.1% | -2.1% | -1.0% | -1.3% |
| 33 | 926930ns | +2.0% | -2.0% | -0.7% | -1.0% |
| 34 | 928085ns | +1.9% | -2.3% | -0.9% | -1.5% |
| 35 | 929186ns | +1.6% | -1.6% | -1.3% | -1.4% |
| 36 | 929790ns | +1.6% | -2.5% | -1.2% | -1.6% |
| 37 | 930612ns | +1.8% | -2.4% | -1.4% | -1.8% |
| 38 | 928611ns | +2.7% | -1.9% | -0.1% | -1.5% |
| 39 | 923348ns | +2.8% | -0.2% | -0.6% | -0.7% |
| 40 | 949333ns | -0.2% | -4.3% | -3.5% | -3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.250 | moderate+ |
| wide-rung-ragged | -0.133 | ok |
| wide-rung-ragged-overread | 0.369 | moderate+ |
| wide-rung-wordround | -0.102 | ok |
| wide-rung-wordround-alias | 0.298 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 2/40, lost 38/40
- **wide-rung-ragged-overread**: won 39/40, lost 0/40
- **wide-rung-wordround**: won 33/40, lost 5/40
- **wide-rung-wordround-alias**: won 28/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 8.1ns | 923554.8ns | 0.0% |  |
| wide-rung-ragged | 8.7ns | 951593.9ns | 0.0% |  |
| wide-rung-ragged-overread | 6.7ns | 906998.2ns | 0.0% |  |
| wide-rung-wordround | 8.3ns | 917417.2ns | 0.0% |  |
| wide-rung-wordround-alias | 8.2ns | 918717.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 916073.5-934185.6 ns)
  916073.5 |####################
  916979.1 |####################
  917884.7 |####################
  918790.3 |####################
  919695.9 |########################################
  920601.6 |##############################
  921507.2 |########################################
  922412.8 |########################################
  923318.4 |####################
  924224.0 |
  925129.6 |##########
  926035.2 |########################################
  926940.8 |
  927846.4 |##############################
  928752.0 |##########
  929657.6 |##########
  930563.2 |##########
  931468.8 |
  932374.4 |
  933280.0 |
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 944227.1-966878.5 ns)
  944227.1 |####################
  945359.7 |########################################
  946492.3 |####################
  947624.8 |########################################
  948757.4 |######
  949890.0 |#############
  951022.5 |##########################
  952155.1 |#############
  953287.7 |####################
  954420.2 |
  955552.8 |
  956685.4 |######
  957817.9 |######
  958950.5 |
  960083.1 |
  961215.6 |
  962348.2 |
  963480.8 |
  964613.4 |
  965745.9 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 901297.9-913606.8 ns)
  901297.9 |########
  901913.4 |
  902528.8 |########
  903144.2 |########
  903759.7 |########
  904375.1 |########################################
  904990.6 |########################
  905606.0 |########################
  906221.5 |########################
  906836.9 |################################
  907452.3 |########################
  908067.8 |################################
  908683.2 |########
  909298.7 |########
  909914.1 |################
  910529.6 |################
  911145.0 |
  911760.4 |
  912375.9 |
  912991.3 |
  (2 below, 3 above range)

wide-rung-wordround (n=40, range 912537.9-924593.7 ns)
  912537.9 |
  913140.7 |######
  913743.4 |#############
  914346.2 |######
  914949.0 |#################################
  915551.8 |########################################
  916154.6 |#############
  916757.4 |#################################
  917360.2 |#################################
  917963.0 |#############
  918565.8 |######
  919168.6 |######
  919771.4 |#############
  920374.2 |######
  920976.9 |
  921579.7 |
  922182.5 |
  922785.3 |
  923388.1 |
  923990.9 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 912875.5-931415.4 ns)
  912875.5 |###########
  913802.5 |########################################
  914729.5 |##################################
  915656.5 |######################
  916583.5 |###########
  917510.5 |#####
  918437.5 |############################
  919364.5 |#####
  920291.4 |#####
  921218.4 |
  922145.4 |
  923072.4 |
  923999.4 |
  924926.4 |###########
  925853.4 |#####
  926780.4 |
  927707.4 |#####
  928634.4 |#####
  929561.4 |
  930488.4 |
  (4 below, 2 above range)

```
