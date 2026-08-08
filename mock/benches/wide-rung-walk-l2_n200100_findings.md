# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.53)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 1.9% of the fastest

All 5 variants sit between 487.15 us and 496.44 us - a 1.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 487151.8 ns median (-0.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 487151.8 ns, slowest 496440.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 491008ns | 488949ns | 485403ns | 488961ns | 502755ns | base |
| wide-rung-ragged | 498711ns | 497562ns | 491860ns | 497617ns | 508844ns | +1.57% |
| wide-rung-ragged-overread | 496824ns | 496146ns | 492841ns | 496143ns | 502850ns | +1.18% |
| wide-rung-wordround | 492378ns | 491056ns | 485752ns | 491142ns | 502714ns | +0.28% |
| wide-rung-wordround-alias | 490850ns | 488250ns | 484515ns | 488892ns | 503061ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 490175ns | 484556ns | 501962ns | base | 0.936 |
| wide-rung-ragged | 497870ns | 491202ns | 508029ns | +1.57% | 0.921 |
| wide-rung-ragged-overread | 495756ns | 491949ns | 501670ns | +1.14% | 0.925 |
| wide-rung-wordround | 491218ns | 484607ns | 501662ns | +0.21% | 0.934 |
| wide-rung-wordround-alias | 489890ns | 483621ns | 502241ns | -0.06% | 0.936 |

## Performance model

- Peak throughput: **0.949 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.940 | 99.1% |
| wide-rung-ragged | 0.924 | 97.4% |
| wide-rung-ragged-overread | 0.927 | 97.7% |
| wide-rung-wordround | 0.937 | 98.7% |
| wide-rung-wordround-alias | 0.942 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 491008ns | 491008ns | base |
| wide-rung-ragged | 498711ns | 498711ns | +1.57% |
| wide-rung-ragged-overread | 496824ns | 496824ns | +1.18% |
| wide-rung-wordround | 492378ns | 492378ns | +0.28% |
| wide-rung-wordround-alias | 490850ns | 490850ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 488056ns | base | --- | [487248, 488429] | --- | --- | --- | --- |
| wide-rung-ragged | 496440ns | +6655.6ns (+1.4%) | [+4538, +9582]ns | [494454, 498736] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 495124ns | +6817.1ns (+1.4%) | [+5759, +8523]ns | [494379, 495615] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 489784ns | no significant difference | [-2752, +5356]ns | [487304, 492648] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 487152ns | no significant difference | [-1295, +2348]ns | [486395, 489710] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 488560ns | +0.8% | +2.3% | -0.4% | -0.2% |
| 2 | 488005ns | +0.9% | +1.1% | -0.4% | +0.5% |
| 3 | 489441ns | +0.3% | +0.9% | -0.7% | -0.4% |
| 4 | 488317ns | +0.5% | +1.3% | -0.0% | -0.3% |
| 5 | 510665ns | -3.8% | -3.5% | -4.2% | -4.6% |
| 6 | 500799ns | -1.3% | -1.4% | -2.7% | -2.9% |
| 7 | 494001ns | +0.3% | +0.2% | -1.4% | -1.4% |
| 8 | 507262ns | -1.1% | -2.3% | -4.2% | -3.8% |
| 9 | 502710ns | -0.2% | -1.0% | -3.3% | -3.7% |
| 10 | 494476ns | -0.2% | +0.7% | -1.5% | -2.1% |
| 11 | 485631ns | +1.2% | +1.3% | +4.4% | +1.0% |
| 12 | 485515ns | +1.5% | +1.0% | +1.6% | +4.1% |
| 13 | 487042ns | +2.0% | +1.3% | +1.5% | +1.4% |
| 14 | 487940ns | +2.0% | +0.5% | +1.5% | -0.7% |
| 15 | 488287ns | +1.3% | +3.6% | +1.7% | +1.8% |
| 16 | 488107ns | +0.9% | +3.0% | +1.6% | -0.3% |
| 17 | 487454ns | +1.4% | +3.3% | +0.6% | +1.9% |
| 18 | 485858ns | +0.8% | +1.7% | +0.9% | +0.7% |
| 19 | 486395ns | +7.9% | +1.6% | +1.3% | -0.2% |
| 20 | 486373ns | +2.9% | +1.8% | +1.4% | -0.2% |
| 21 | 487870ns | +0.4% | +4.3% | -0.3% | -0.7% |
| 22 | 488423ns | +6.6% | +1.3% | -1.0% | -2.0% |
| 23 | 496014ns | +0.2% | -0.3% | -0.7% | -2.0% |
| 24 | 488328ns | +1.6% | +1.5% | +4.3% | -0.8% |
| 25 | 488435ns | +1.1% | +1.4% | +0.9% | +2.0% |
| 26 | 489008ns | +1.8% | +1.4% | +0.1% | +0.7% |
| 27 | 486804ns | +2.3% | +1.8% | +1.3% | +1.0% |
| 28 | 488261ns | +2.3% | +0.8% | -1.2% | +0.1% |
| 29 | 487880ns | +2.8% | +1.2% | -0.9% | +2.0% |
| 30 | 488944ns | +1.2% | +1.5% | -1.1% | +0.3% |
| 31 | 487638ns | +3.0% | +1.9% | -0.1% | +0.5% |
| 32 | 486687ns | +2.9% | +2.2% | +0.1% | -0.0% |
| 33 | 483846ns | +2.8% | +2.5% | +0.9% | +0.4% |
| 34 | 486676ns | +1.9% | +1.8% | +0.3% | -0.2% |
| 35 | 484416ns | +2.2% | +2.3% | +6.5% | +0.6% |
| 36 | 483781ns | +3.5% | +2.3% | +2.7% | +0.6% |
| 37 | 484234ns | +3.5% | +1.6% | +1.3% | +0.3% |
| 38 | 483163ns | +3.6% | +2.0% | +1.6% | +0.1% |
| 39 | 501379ns | +1.3% | -1.3% | -0.9% | +0.3% |
| 40 | 502392ns | +0.3% | -1.3% | -1.9% | +4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.526 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.027 | ok |
| wide-rung-ragged-overread | 0.214 | moderate+ |
| wide-rung-wordround | 0.270 | moderate+ |
| wide-rung-wordround-alias | 0.269 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 5/40, lost 35/40
- **wide-rung-ragged-overread**: won 7/40, lost 33/40
- **wide-rung-wordround**: won 18/40, lost 19/40
- **wide-rung-wordround-alias**: won 18/40, lost 20/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 5.9ns | 490175.5ns | 0.0% |  |
| wide-rung-ragged | 9.0ns | 497870.0ns | 0.0% |  |
| wide-rung-ragged-overread | 6.1ns | 495755.7ns | 0.0% |  |
| wide-rung-wordround | 5.6ns | 491217.7ns | 0.0% |  |
| wide-rung-wordround-alias | 7.7ns | 489890.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 484555.5-501962.2 ns)
  484555.5 |
  485425.9 |###############
  486296.2 |##############################
  487166.6 |##############################
  488036.9 |########################################
  488907.2 |###############
  489777.6 |
  490647.9 |
  491518.2 |
  492388.6 |
  493258.9 |#####
  494129.2 |#####
  494999.6 |
  495869.9 |#####
  496740.2 |
  497610.6 |
  498480.9 |
  499351.2 |
  500221.6 |#####
  501091.9 |#####
  (5 below, 4 above range)

wide-rung-ragged (n=40, range 491201.9-508028.8 ns)
  491201.9 |################
  492043.2 |################################
  492884.6 |########
  493725.9 |################################
  494567.3 |########################
  495408.6 |########
  496250.0 |################
  497091.3 |########################################
  497932.7 |
  498774.0 |########
  499615.4 |
  500456.7 |########################################
  501298.1 |########################
  502139.4 |########
  502980.7 |########
  503822.1 |
  504663.4 |
  505504.8 |
  506346.1 |
  507187.5 |########
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 491948.9-501670.3 ns)
  491948.9 |###########
  492435.0 |###########
  492921.1 |
  493407.1 |######################
  493893.2 |#################
  494379.3 |###########
  494865.3 |############################
  495351.4 |########################################
  495837.5 |#################
  496323.6 |#####
  496809.6 |
  497295.7 |#################
  497781.8 |
  498267.8 |
  498753.9 |
  499240.0 |#####
  499726.1 |
  500212.1 |
  500698.2 |
  501184.3 |
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 484607.3-501661.8 ns)
  484607.3 |
  485460.0 |##########################
  486312.7 |########################################
  487165.5 |######
  488018.2 |####################
  488870.9 |#############
  489723.6 |####################
  490576.4 |######
  491429.1 |
  492281.8 |#################################
  493134.6 |#############
  493987.3 |######
  494840.0 |######
  495692.7 |#############
  496545.5 |#############
  497398.2 |
  498250.9 |
  499103.6 |
  499956.4 |
  500809.1 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 483621.1-502240.9 ns)
  483621.1 |############################
  484552.1 |###########
  485483.1 |############################
  486414.1 |########################################
  487345.1 |#################
  488276.1 |#####
  489207.1 |###########
  490138.1 |#################
  491069.1 |#####
  492000.0 |#####
  492931.0 |
  493862.0 |#####
  494793.0 |
  495724.0 |
  496655.0 |#################
  497586.0 |#####
  498517.0 |
  499448.0 |
  500378.9 |
  501309.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.53 (measurement drift or warm-up artifact)
