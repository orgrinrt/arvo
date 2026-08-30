# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged-overread, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-ragged-overread (911.59 us) and wide-rung-wordround-alias (912.37 us) differ by 0.09%, inside the noise, even though the wider field spreads 5.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 911590.4 ns median (-0.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 911590.4 ns, slowest 958218.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 920907ns | 919061ns | 912470ns | 918947ns | 935225ns | base |
| wide-rung-ragged | 961300ns | 959079ns | 951066ns | 959698ns | 976340ns | +4.39% |
| wide-rung-ragged-overread | 914064ns | 912279ns | 900434ns | 912657ns | 931914ns | -0.74% |
| wide-rung-wordround | 949123ns | 919393ns | 909697ns | 920274ns | 1075097ns | +3.06% |
| wide-rung-wordround-alias | 915032ns | 913757ns | 910977ns | 914162ns | 921697ns | -0.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 919858ns | 911669ns | 933952ns | base | 1.995 |
| wide-rung-ragged | 960469ns | 950058ns | 975681ns | +4.41% | 1.911 |
| wide-rung-ragged-overread | 913276ns | 899398ns | 931118ns | -0.72% | 2.009 |
| wide-rung-wordround | 947994ns | 908778ns | 1073055ns | +3.06% | 1.936 |
| wide-rung-wordround-alias | 914117ns | 909916ns | 920871ns | -0.62% | 2.007 |

## Performance model

- Peak throughput: **2.040 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.999 | 98.0% |
| wide-rung-ragged | 1.915 | 93.9% |
| wide-rung-ragged-overread | 2.013 | 98.7% |
| wide-rung-wordround | 1.997 | 97.9% |
| wide-rung-wordround-alias | 2.011 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 920907ns | 920907ns | base |
| wide-rung-ragged | 961300ns | 961300ns | +4.39% |
| wide-rung-ragged-overread | 914064ns | 914064ns | -0.74% |
| wide-rung-wordround | 949123ns | 949123ns | +3.06% |
| wide-rung-wordround-alias | 915032ns | 915032ns | -0.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 917954ns | base | --- | [916256, 919750] | --- | --- | --- | --- |
| wide-rung-ragged | 958219ns | +39881.2ns (+4.3%) | [+36717, +43840]ns | [955829, 960856] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 911590ns | -7918.2ns (-0.9%) | [-10294, -448]ns | [907770, 916616] | YES (adj: no) | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 918790ns | no significant difference | [-5085, +8523]ns | [913104, 923408] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 912369ns | -4376.7ns (-0.5%) | [-6832, -1660]ns | [911609, 914385] | YES | 0.0332 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 913106ns | +4.1% | -1.1% | +0.8% | +0.6% |
| 2 | 915351ns | +3.9% | -1.8% | -0.4% | +0.2% |
| 3 | 911804ns | +3.9% | -1.1% | +2.0% | +0.3% |
| 4 | 919976ns | +3.1% | -2.2% | -1.2% | -0.6% |
| 5 | 915618ns | +4.3% | -1.9% | -0.8% | +0.3% |
| 6 | 916449ns | +5.6% | +2.3% | -0.9% | -0.7% |
| 7 | 915119ns | +11.4% | +1.1% | -0.9% | -0.4% |
| 8 | 916135ns | +4.6% | +0.3% | -0.7% | -0.4% |
| 9 | 918502ns | +3.4% | +0.8% | +1.5% | -0.8% |
| 10 | 919780ns | +3.5% | -1.0% | +1.2% | -1.2% |
| 11 | 918146ns | +4.2% | +0.2% | +0.7% | -0.6% |
| 12 | 921050ns | +4.3% | -1.1% | -0.5% | -0.9% |
| 13 | 916377ns | +5.7% | -0.9% | -0.4% | -0.5% |
| 14 | 916908ns | +5.4% | -2.1% | -0.4% | +0.1% |
| 15 | 917797ns | +4.4% | -2.2% | -0.8% | +0.0% |
| 16 | 924332ns | +3.8% | -2.8% | -1.5% | -1.0% |
| 17 | 913907ns | +6.6% | -1.2% | -0.3% | +1.5% |
| 18 | 913160ns | +4.7% | +0.3% | -0.6% | +1.0% |
| 19 | 919810ns | +5.0% | -0.5% | -0.8% | -0.9% |
| 20 | 997891ns | -4.0% | -8.4% | -8.8% | -8.4% |
| 21 | 920107ns | +4.0% | -0.9% | +0.4% | -0.4% |
| 22 | 917004ns | +4.0% | -1.0% | +2.9% | +0.3% |
| 23 | 922514ns | +3.8% | +2.2% | -0.4% | -1.1% |
| 24 | 912605ns | +6.1% | +0.5% | +1.1% | +0.9% |
| 25 | 910498ns | +4.8% | +0.3% | +1.9% | -0.0% |
| 26 | 903990ns | +6.0% | +0.8% | +1.5% | +0.8% |
| 27 | 925082ns | +3.9% | -0.6% | -0.9% | -1.8% |
| 28 | 921181ns | +5.1% | -0.3% | -0.9% | -1.2% |
| 29 | 925304ns | +3.2% | -0.5% | -0.7% | -1.6% |
| 30 | 926367ns | +2.7% | +0.5% | +1.8% | -1.6% |
| 31 | 919721ns | +4.7% | -1.9% | +96.6% | -0.1% |
| 32 | 915945ns | +4.8% | -0.8% | +16.0% | -0.2% |
| 33 | 918112ns | +3.5% | +2.2% | +4.6% | -0.8% |
| 34 | 914283ns | +4.5% | -0.3% | +3.6% | -0.5% |
| 35 | 917305ns | +5.8% | +0.2% | +3.6% | -0.6% |
| 36 | 918807ns | +4.9% | +0.9% | +5.5% | -0.7% |
| 37 | 928948ns | +2.4% | -1.6% | -0.6% | -2.0% |
| 38 | 919560ns | +5.2% | -1.8% | +0.2% | -0.8% |
| 39 | 915663ns | +5.8% | -1.1% | +0.4% | -0.3% |
| 40 | 920103ns | +4.2% | -1.5% | -0.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.046 | ok |
| wide-rung-ragged | 0.142 | ok |
| wide-rung-ragged-overread | 0.262 | moderate+ |
| wide-rung-wordround | 0.159 | ok |
| wide-rung-wordround-alias | 0.313 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 1/40, lost 39/40
- **wide-rung-ragged-overread**: won 26/40, lost 14/40
- **wide-rung-wordround**: won 21/40, lost 19/40
- **wide-rung-wordround-alias**: won 27/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 12.5ns | 919857.9ns | 0.0% |  |
| wide-rung-ragged | 15.9ns | 960468.5ns | 0.0% |  |
| wide-rung-ragged-overread | 17.1ns | 913276.4ns | 0.0% |  |
| wide-rung-wordround | 55.2ns | 947993.7ns | 0.0% |  |
| wide-rung-wordround-alias | 11.4ns | 914116.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 911669.1-933952.4 ns)
  911669.1 |###########
  912783.2 |###########
  913897.4 |###########
  915011.6 |############################
  916125.7 |############################
  917239.9 |######################
  918354.1 |###########
  919468.2 |########################################
  920582.4 |###########
  921696.6 |#####
  922810.8 |
  923924.9 |#####
  925039.1 |###########
  926153.3 |#####
  927267.4 |
  928381.6 |#####
  929495.8 |
  930609.9 |
  931724.1 |
  932838.3 |
  (2 below, 1 above range)

wide-rung-ragged (n=40, range 950058.2-975680.5 ns)
  950058.2 |##############################
  951339.3 |##############################
  952620.5 |##########
  953901.6 |##############################
  955182.7 |##############################
  956463.8 |####################
  957744.9 |########################################
  959026.0 |########################################
  960307.1 |##########
  961588.3 |
  962869.4 |####################
  964150.5 |
  965431.6 |####################
  966712.7 |##############################
  967993.8 |##############################
  969274.9 |##########
  970556.1 |
  971837.2 |
  973118.3 |##########
  974399.4 |
  (3 below, 1 above range)

wide-rung-ragged-overread (n=40, range 899397.5-931117.7 ns)
  899397.5 |########
  900983.5 |################
  902569.5 |########################
  904155.5 |########
  905741.5 |########
  907327.5 |########################
  908913.6 |
  910499.6 |########################################
  912085.6 |########
  913671.6 |################
  915257.6 |################
  916843.6 |########
  918429.6 |################################
  920015.6 |################
  921601.6 |
  923187.6 |
  924773.6 |################
  926359.6 |########
  927945.6 |
  929531.7 |
  (5 below, 4 above range)

wide-rung-wordround (n=40, range 908777.6-1073055.0 ns)
  908777.6 |########################################
  916991.5 |#################################
  925205.4 |############
  933419.2 |
  941633.1 |#########
  949847.0 |###
  958060.8 |###
  966274.7 |###
  974488.6 |
  982702.4 |
  990916.3 |
  999130.2 |
  1007344.0 |
  1015557.9 |
  1023771.8 |
  1031985.6 |
  1040199.5 |
  1048413.4 |
  1056627.2 |###
  1064841.1 |
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 909915.6-920870.5 ns)
  909915.6 |########################
  910463.4 |################################
  911011.1 |################################
  911558.8 |################
  912106.6 |########################################
  912654.3 |################
  913202.1 |########
  913749.8 |########
  914297.6 |########################
  914845.3 |
  915393.1 |
  915940.8 |########
  916488.6 |
  917036.3 |########
  917584.0 |################
  918131.8 |########################
  918679.5 |
  919227.3 |
  919775.0 |########
  920322.8 |########
  (3 below, 3 above range)

```
