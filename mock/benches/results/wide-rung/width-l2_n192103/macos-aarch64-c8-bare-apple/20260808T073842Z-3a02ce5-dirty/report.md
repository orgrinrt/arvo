# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (882.02 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 871.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.94 us) is smaller than the fastest variant's own run-to-run std-dev (15.66 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.3% of the fastest

All 5 variants sit between 871.08 us and 882.02 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 871082.1 ns median (-1.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.01x (fastest 871082.1 ns, slowest 882021.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 890210ns | 883261ns | 874776ns | 884307ns | 923350ns | base |
| wide-rung-ragged | 873603ns | 872880ns | 868208ns | 872669ns | 881800ns | -1.87% |
| wide-rung-ragged-overread | 876398ns | 872141ns | 866518ns | 873187ns | 895911ns | -1.55% |
| wide-rung-wordround | 897950ns | 873259ns | 867685ns | 873821ns | 1000604ns | +0.87% |
| wide-rung-wordround-alias | 881993ns | 877245ns | 868694ns | 877779ns | 907935ns | -0.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 889099ns | 873809ns | 922067ns | base | 2.064 |
| wide-rung-ragged | 872707ns | 867384ns | 880849ns | -1.84% | 2.103 |
| wide-rung-ragged-overread | 875475ns | 865709ns | 894887ns | -1.53% | 2.096 |
| wide-rung-wordround | 897032ns | 866859ns | 999360ns | +0.89% | 2.046 |
| wide-rung-wordround-alias | 881066ns | 867936ns | 906886ns | -0.90% | 2.083 |

## Performance model

- Peak throughput: **2.120 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.080 | 98.2% |
| wide-rung-ragged | 2.105 | 99.3% |
| wide-rung-ragged-overread | 2.107 | 99.4% |
| wide-rung-wordround | 2.104 | 99.2% |
| wide-rung-wordround-alias | 2.094 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 890210ns | 890210ns | base |
| wide-rung-ragged | 873603ns | 873603ns | -1.87% |
| wide-rung-ragged-overread | 876398ns | 876398ns | -1.55% |
| wide-rung-wordround | 897950ns | 897950ns | +0.87% |
| wide-rung-wordround-alias | 881993ns | 881993ns | -0.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 882022ns | base | --- | [878472, 887408] | --- | --- | --- | --- |
| wide-rung-ragged | 871810ns | -8945.2ns (-1.0%) | [-15913, -7296]ns | [869795, 873656] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 871082ns | -12064.8ns (-1.4%) | [-14889, -5811]ns | [869786, 874194] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 872304ns | -7771.2ns (-0.9%) | [-14244, -5652]ns | [869716, 876095] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 876164ns | -6745.0ns (-0.8%) | [-10330, -2724]ns | [872080, 880989] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 889903ns | -2.0% | -3.0% | -1.4% | -1.5% |
| 2 | 911687ns | -4.3% | -4.2% | +2.3% | -5.1% |
| 3 | 893546ns | -2.3% | -2.6% | -1.0% | -1.0% |
| 4 | 888314ns | +0.1% | -2.1% | -1.4% | -2.2% |
| 5 | 919064ns | -4.6% | -5.5% | -4.3% | -4.0% |
| 6 | 879226ns | -0.8% | +0.1% | +36.7% | +2.2% |
| 7 | 873830ns | -0.2% | -0.7% | +62.8% | +6.6% |
| 8 | 878727ns | -0.5% | -1.0% | +1.1% | +0.4% |
| 9 | 914889ns | -5.1% | -4.9% | -3.9% | -3.3% |
| 10 | 874986ns | -0.7% | -0.0% | -0.1% | +2.5% |
| 11 | 872965ns | -0.6% | -0.4% | -0.6% | -0.4% |
| 12 | 869972ns | +0.1% | +0.3% | -0.2% | +0.1% |
| 13 | 883789ns | -1.8% | -1.7% | -0.6% | -1.7% |
| 14 | 879923ns | -1.4% | -0.7% | -0.7% | +1.6% |
| 15 | 876590ns | -1.0% | +0.2% | +0.6% | -0.6% |
| 16 | 875001ns | -0.8% | +10.0% | -0.8% | -0.8% |
| 17 | 876381ns | +0.0% | -0.7% | -0.8% | -0.8% |
| 18 | 875785ns | -0.2% | -0.5% | -0.9% | +1.0% |
| 19 | 885532ns | -1.9% | -1.7% | -1.9% | -2.2% |
| 20 | 878606ns | -0.8% | -0.1% | -0.7% | -1.2% |
| 21 | 892422ns | -2.3% | -1.7% | -0.5% | -2.5% |
| 22 | 904381ns | -2.7% | -1.5% | -3.8% | -3.5% |
| 23 | 880133ns | +0.0% | +1.8% | -0.4% | -0.0% |
| 24 | 898345ns | -1.2% | -2.0% | -2.9% | -3.1% |
| 25 | 965820ns | -9.2% | -9.0% | -9.2% | -8.1% |
| 26 | 893587ns | -2.9% | -0.2% | -2.7% | -0.7% |
| 27 | 872563ns | -0.5% | +0.1% | -0.1% | -0.1% |
| 28 | 901206ns | -3.9% | -3.4% | -3.3% | -3.2% |
| 29 | 877725ns | +0.1% | -0.9% | -0.5% | -0.1% |
| 30 | 875368ns | -1.1% | -0.1% | +2.2% | +0.3% |
| 31 | 961142ns | -9.0% | -9.8% | -9.1% | -7.8% |
| 32 | 890140ns | -1.9% | -2.6% | -2.4% | +5.8% |
| 33 | 882641ns | -1.0% | -2.2% | -1.5% | +3.8% |
| 34 | 876001ns | -0.9% | -1.3% | -0.7% | -0.3% |
| 35 | 881402ns | -1.6% | -1.4% | -1.7% | -0.3% |
| 36 | 884547ns | -1.3% | -1.8% | -2.2% | -1.0% |
| 37 | 878339ns | -0.8% | -1.6% | -1.5% | -0.5% |
| 38 | 886502ns | -1.8% | -1.6% | -1.9% | -0.9% |
| 39 | 877572ns | -0.2% | +0.1% | -0.9% | -0.6% |
| 40 | 885412ns | -1.0% | -0.7% | -2.1% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.054 | ok |
| wide-rung-ragged | 0.349 | moderate+ |
| wide-rung-ragged-overread | 0.109 | ok |
| wide-rung-wordround | 0.431 | moderate+ |
| wide-rung-wordround-alias | 0.350 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 35/40, lost 1/40
- **wide-rung-ragged-overread**: won 31/40, lost 5/40
- **wide-rung-wordround**: won 33/40, lost 6/40
- **wide-rung-wordround-alias**: won 28/40, lost 9/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 35.2ns | 889099.1ns | 0.0% |  |
| wide-rung-ragged | 20.5ns | 872706.9ns | 0.0% |  |
| wide-rung-ragged-overread | 23.3ns | 875475.0ns | 0.0% |  |
| wide-rung-wordround | 34.9ns | 897032.5ns | 0.0% |  |
| wide-rung-wordround-alias | 31.4ns | 881065.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 873808.9-922066.5 ns)
  873808.9 |########################################
  876221.8 |########################################
  878634.7 |##########################
  881047.5 |#############
  883460.4 |##########################
  885873.3 |######
  888286.2 |####################
  890699.1 |######
  893111.9 |#############
  895524.8 |
  897937.7 |######
  900350.6 |######
  902763.5 |######
  905176.3 |
  907589.2 |
  910002.1 |######
  912415.0 |
  914827.9 |######
  917240.8 |######
  919653.6 |
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 867383.5-880848.7 ns)
  867383.5 |########################################
  868056.8 |######################
  868730.0 |#####
  869403.3 |
  870076.6 |###########
  870749.8 |###########
  871423.1 |#################
  872096.3 |###########
  872769.6 |#################
  873442.9 |#####
  874116.1 |#################
  874789.4 |
  875462.6 |#####
  876135.9 |###########
  876809.2 |###########
  877482.4 |
  878155.7 |#####
  878828.9 |
  879502.2 |#####
  880175.5 |#####
  (2 below, 2 above range)

wide-rung-ragged-overread (n=40, range 865709.5-894887.0 ns)
  865709.5 |######
  867168.4 |##########################
  868627.2 |########################################
  870086.1 |########################################
  871545.0 |#############
  873003.9 |##########################
  874462.7 |######
  875921.6 |
  877380.5 |##########################
  878839.4 |####################
  880298.2 |######
  881757.1 |
  883216.0 |
  884674.9 |
  886133.7 |
  887592.6 |
  889051.5 |
  890510.4 |#############
  891969.2 |
  893428.1 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 866858.9-999360.4 ns)
  866858.9 |########################################
  873484.0 |########################
  880109.0 |####
  886734.1 |####
  893359.2 |##
  899984.3 |
  906609.3 |
  913234.4 |
  919859.5 |
  926484.6 |##
  933109.6 |
  939734.7 |
  946359.8 |
  952984.8 |
  959609.9 |
  966235.0 |
  972860.1 |
  979485.1 |
  986110.2 |
  992735.3 |
  (3 below, 2 above range)

wide-rung-wordround-alias (n=40, range 867935.9-906886.3 ns)
  867935.9 |########################################
  869883.4 |########################################
  871830.9 |################################
  873778.4 |################
  875726.0 |########################
  877673.5 |########################
  879621.0 |########
  881568.5 |################
  883516.1 |########################
  885463.6 |################
  887411.1 |########
  889358.6 |
  891306.2 |
  893253.7 |########
  895201.2 |
  897148.7 |################
  899096.2 |
  901043.8 |
  902991.3 |
  904938.8 |
  (3 below, 3 above range)

```
