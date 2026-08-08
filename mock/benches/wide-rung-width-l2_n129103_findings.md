# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias shows warm-up / thermal drift (autocorr +0.53)

wide-rung-wordround-alias's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 896843.1 ns median (-2.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 896843.1 ns, slowest 953155.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 919412ns | 916527ns | 908922ns | 918047ns | 933995ns | base |
| wide-rung-ragged | 953076ns | 954060ns | 941286ns | 952414ns | 966852ns | +3.66% |
| wide-rung-ragged-overread | 898085ns | 897609ns | 885229ns | 896915ns | 914449ns | -2.32% |
| wide-rung-wordround | 913813ns | 911209ns | 903401ns | 912187ns | 929102ns | -0.61% |
| wide-rung-wordround-alias | 913582ns | 912585ns | 903440ns | 913110ns | 925138ns | -0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 918321ns | 907881ns | 932941ns | base | 1.998 |
| wide-rung-ragged | 952176ns | 940333ns | 965967ns | +3.69% | 1.927 |
| wide-rung-ragged-overread | 897216ns | 884430ns | 913650ns | -2.30% | 2.045 |
| wide-rung-wordround | 912550ns | 902179ns | 927923ns | -0.63% | 2.011 |
| wide-rung-wordround-alias | 912598ns | 902485ns | 924150ns | -0.62% | 2.011 |

## Performance model

- Peak throughput: **2.075 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.004 | 96.6% |
| wide-rung-ragged | 1.925 | 92.8% |
| wide-rung-ragged-overread | 2.046 | 98.6% |
| wide-rung-wordround | 2.016 | 97.2% |
| wide-rung-wordround-alias | 2.013 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 919412ns | 919412ns | base |
| wide-rung-ragged | 953076ns | 953076ns | +3.66% |
| wide-rung-ragged-overread | 898085ns | 898085ns | -2.32% |
| wide-rung-wordround | 913813ns | 913813ns | -0.61% |
| wide-rung-wordround-alias | 913582ns | 913582ns | -0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 915636ns | base | --- | [913415, 920232] | --- | --- | --- | --- |
| wide-rung-ragged | 953155ns | +33043.6ns (+3.6%) | [+30097, +36129]ns | [946745, 955288] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 896843ns | -23968.8ns (-2.6%) | [-27066, -13265]ns | [891260, 900713] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 910037ns | -6852.6ns (-0.7%) | [-10005, -2892]ns | [905372, 916977] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 911418ns | -5259.4ns (-0.6%) | [-6851, -3457]ns | [908261, 915939] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 913145ns | +3.0% | -3.2% | -1.1% | -0.6% |
| 2 | 914854ns | +3.2% | -3.5% | -1.3% | -0.9% |
| 3 | 911876ns | +3.0% | -2.7% | -1.1% | -0.4% |
| 4 | 913378ns | +4.2% | -0.2% | -1.2% | -0.8% |
| 5 | 911174ns | +3.5% | -1.4% | -1.1% | -0.6% |
| 6 | 941575ns | +0.2% | -5.2% | -3.4% | -3.9% |
| 7 | 911052ns | +3.4% | -2.3% | -0.6% | -0.6% |
| 8 | 924043ns | +2.0% | -4.2% | -2.1% | -2.1% |
| 9 | 915580ns | +2.8% | -3.3% | -0.9% | -0.2% |
| 10 | 913452ns | +2.7% | -3.4% | -0.6% | -0.6% |
| 11 | 907668ns | +3.9% | -0.2% | +1.4% | +1.0% |
| 12 | 911314ns | +3.4% | -2.9% | +1.0% | -0.7% |
| 13 | 904782ns | +5.5% | -0.9% | +0.7% | +0.7% |
| 14 | 916179ns | +4.1% | -2.9% | -0.3% | -0.6% |
| 15 | 920534ns | +3.5% | -3.0% | -1.8% | -0.6% |
| 16 | 909313ns | +6.6% | -1.2% | -0.4% | +0.1% |
| 17 | 905604ns | +5.5% | -0.3% | -0.3% | -0.6% |
| 18 | 927343ns | +4.0% | -3.0% | -2.5% | -3.0% |
| 19 | 910525ns | +4.6% | -2.7% | -0.8% | -1.1% |
| 20 | 905455ns | +8.6% | -2.3% | +4.6% | +1.4% |
| 21 | 914636ns | +4.9% | -0.3% | -1.2% | -0.4% |
| 22 | 914981ns | +3.8% | -1.3% | -1.4% | +0.4% |
| 23 | 918738ns | +3.5% | -1.8% | -0.7% | -0.4% |
| 24 | 915325ns | +3.1% | -1.5% | -0.5% | +0.1% |
| 25 | 952252ns | +0.7% | -4.6% | -5.4% | -3.7% |
| 26 | 916349ns | +4.8% | -1.4% | +1.0% | -0.7% |
| 27 | 915692ns | +4.8% | -1.0% | +0.5% | -1.7% |
| 28 | 925323ns | +3.7% | -0.3% | -1.0% | -2.1% |
| 29 | 926091ns | +3.7% | -4.1% | -1.2% | -0.9% |
| 30 | 911455ns | +5.5% | -2.6% | +0.7% | +0.9% |
| 31 | 908650ns | +5.0% | -0.7% | +1.1% | +0.3% |
| 32 | 935540ns | +2.1% | -0.4% | -1.7% | -1.5% |
| 33 | 920268ns | +3.5% | -2.7% | -0.3% | +0.1% |
| 34 | 926470ns | +3.1% | -2.9% | +1.7% | -0.6% |
| 35 | 922296ns | +3.6% | -2.7% | +0.5% | +2.4% |
| 36 | 920195ns | +1.8% | -0.9% | -0.0% | +0.2% |
| 37 | 928488ns | +1.4% | -4.3% | -0.9% | -0.9% |
| 38 | 920021ns | +3.6% | -2.5% | -0.2% | +0.2% |
| 39 | 925471ns | +4.7% | -3.2% | -1.9% | -1.4% |
| 40 | 925768ns | +3.0% | -3.5% | -1.8% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.013 | ok |
| wide-rung-ragged | 0.440 | moderate+ |
| wide-rung-ragged-overread | 0.152 | ok |
| wide-rung-wordround | 0.282 | moderate+ |
| wide-rung-wordround-alias | 0.533 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 40/40, lost 0/40
- **wide-rung-wordround**: won 29/40, lost 10/40
- **wide-rung-wordround-alias**: won 28/40, lost 11/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 8.6ns | 918321.3ns | 0.0% |  |
| wide-rung-ragged | 6.3ns | 952175.6ns | 0.0% |  |
| wide-rung-ragged-overread | 6.5ns | 897215.9ns | 0.0% |  |
| wide-rung-wordround | 8.7ns | 912549.9ns | 0.0% |  |
| wide-rung-wordround-alias | 7.3ns | 912598.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 907881.1-932940.8 ns)
  907881.1 |########
  909134.1 |########
  910387.1 |########################################
  911640.1 |########
  912893.0 |########################
  914146.0 |################################
  915399.0 |################################
  916652.0 |
  917905.0 |########
  919158.0 |########################
  920410.9 |########
  921663.9 |########
  922916.9 |########
  924169.9 |########
  925422.9 |################################
  926675.9 |########
  927928.8 |########
  929181.8 |
  930434.8 |
  931687.8 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 940332.5-965966.8 ns)
  940332.5 |################
  941614.3 |########################################
  942896.0 |################################
  944177.7 |
  945459.4 |
  946741.1 |
  948022.8 |
  949304.5 |########
  950586.3 |################
  951868.0 |########################
  953149.7 |################################
  954431.4 |################################
  955713.1 |########
  956994.8 |
  958276.5 |########################
  959558.3 |########################
  960840.0 |########
  962121.7 |
  963403.4 |########
  964685.1 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 884430.4-913649.7 ns)
  884430.4 |########################################
  885891.3 |##########################
  887352.3 |########################################
  888813.3 |##########################
  890274.2 |
  891735.2 |##########################
  893196.2 |#############
  894657.1 |##########################
  896118.1 |########################################
  897579.1 |##########################
  899040.0 |##########################
  900501.0 |########################################
  901962.0 |########################################
  903422.9 |
  904883.9 |#############
  906344.9 |#############
  907805.8 |#############
  909266.8 |
  910727.8 |########################################
  912188.8 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 902179.5-927923.0 ns)
  902179.5 |########################################
  903466.7 |#################
  904753.8 |###########
  906041.0 |
  907328.2 |#################
  908615.4 |###########
  909902.6 |###########
  911189.7 |#####
  912476.9 |#####
  913764.1 |
  915051.3 |#####
  916338.4 |#################
  917625.6 |###########
  918912.8 |######################
  920200.0 |###########
  921487.2 |
  922774.3 |
  924061.5 |
  925348.7 |#####
  926635.9 |#####
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 902484.8-924149.8 ns)
  902484.8 |
  903568.1 |##########
  904651.3 |########################################
  905734.6 |##############################
  906817.8 |
  907901.1 |##############################
  908984.3 |
  910067.6 |########################################
  911150.8 |####################
  912234.1 |##########
  913317.3 |##########
  914400.6 |##############################
  915483.8 |
  916567.1 |########################################
  917650.3 |##########
  918733.5 |####################
  919816.8 |##########
  920900.0 |########################################
  921983.3 |##########
  923066.5 |
  (4 below, 1 above range)

```

## Diagnostics

- **wide-rung-wordround-alias**: autocorrelation=0.53 (measurement drift or warm-up artifact)
