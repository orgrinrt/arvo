# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.62)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.62, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.12 us) is smaller than the fastest variant's own run-to-run std-dev (17.71 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 2.0% of the fastest

All 5 variants sit between 500.96 us and 511.07 us - a 2.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 500955.2 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.02x (fastest 500955.2 ns, slowest 511070.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 513983ns | 505509ns | 492556ns | 508071ns | 553147ns | base |
| wide-rung-ragged | 516992ns | 511985ns | 491132ns | 512804ns | 555419ns | +0.59% |
| wide-rung-ragged-overread | 506927ns | 501978ns | 492784ns | 501890ns | 536183ns | -1.37% |
| wide-rung-wordround | 512257ns | 504465ns | 490283ns | 503882ns | 559358ns | -0.34% |
| wide-rung-wordround-alias | 514187ns | 507435ns | 494730ns | 508021ns | 552143ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 512928ns | 491655ns | 551934ns | base | 0.894 |
| wide-rung-ragged | 515923ns | 490221ns | 554156ns | +0.58% | 0.889 |
| wide-rung-ragged-overread | 505891ns | 491562ns | 535150ns | -1.37% | 0.907 |
| wide-rung-wordround | 511179ns | 489235ns | 558164ns | -0.34% | 0.897 |
| wide-rung-wordround-alias | 513182ns | 493865ns | 550915ns | +0.05% | 0.894 |

## Performance model

- Peak throughput: **0.938 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.909 | 97.0% |
| wide-rung-ragged | 0.898 | 95.7% |
| wide-rung-ragged-overread | 0.916 | 97.7% |
| wide-rung-wordround | 0.911 | 97.2% |
| wide-rung-wordround-alias | 0.906 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 513983ns | 513983ns | base |
| wide-rung-ragged | 516992ns | 516992ns | +0.59% |
| wide-rung-ragged-overread | 506927ns | 506927ns | -1.37% |
| wide-rung-wordround | 512257ns | 512257ns | -0.34% |
| wide-rung-wordround-alias | 514187ns | 514187ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 504500ns | base | --- | [499436, 514350] | --- | --- | --- | --- |
| wide-rung-ragged | 511070ns | no significant difference | [-6043, +11230]ns | [502623, 520617] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-ragged-overread | 500955ns | -9492.1ns (-1.9%) | [-15999, -2528]ns | [494467, 504949] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| wide-rung-wordround | 503465ns | no significant difference | [-11456, +1429]ns | [498448, 506192] | no | 0.3077 | 0.1539 | 0 |
| wide-rung-wordround-alias | 506423ns | no significant difference | [-10843, +4247]ns | [501861, 510430] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 531965ns | -8.3% | -7.3% | -4.5% | -4.7% |
| 2 | 548449ns | -11.2% | -8.5% | -4.4% | -7.3% |
| 3 | 532061ns | -8.4% | -4.8% | -7.8% | -6.2% |
| 4 | 520913ns | -6.8% | -5.6% | -4.0% | -0.9% |
| 5 | 505100ns | +9.2% | -2.8% | -1.0% | -1.0% |
| 6 | 508939ns | -1.3% | -1.8% | -0.9% | +3.8% |
| 7 | 499422ns | +1.2% | -0.9% | -2.2% | +3.7% |
| 8 | 522060ns | -4.9% | -4.2% | -3.4% | -5.9% |
| 9 | 498415ns | -1.7% | -1.6% | +7.4% | +2.4% |
| 10 | 510829ns | +0.4% | -3.7% | +0.7% | -3.7% |
| 11 | 532803ns | -3.5% | -4.0% | -2.3% | -3.2% |
| 12 | 523873ns | +6.3% | -1.0% | -4.7% | +0.7% |
| 13 | 515854ns | -0.5% | -4.8% | -2.2% | -2.7% |
| 14 | 499450ns | +2.2% | +0.8% | +1.7% | +1.3% |
| 15 | 544507ns | -4.5% | -8.4% | -6.8% | -4.0% |
| 16 | 526896ns | -6.0% | -6.4% | -6.8% | -3.7% |
| 17 | 494946ns | +1.1% | +4.3% | -0.5% | -0.2% |
| 18 | 507278ns | -1.1% | -2.0% | +2.3% | -0.9% |
| 19 | 502266ns | +9.9% | +4.6% | -1.9% | +0.1% |
| 20 | 493532ns | +2.4% | +2.1% | +2.0% | +3.8% |
| 21 | 486349ns | +8.1% | +5.2% | -0.1% | +5.0% |
| 22 | 504792ns | +1.4% | -2.4% | -3.5% | -2.4% |
| 23 | 494795ns | +6.8% | -0.3% | +2.0% | +0.9% |
| 24 | 521780ns | -2.6% | -3.4% | -4.4% | -5.9% |
| 25 | 504208ns | +5.0% | -2.4% | -2.3% | -1.9% |
| 26 | 516449ns | +2.0% | -2.2% | -5.3% | -2.6% |
| 27 | 492014ns | +2.2% | +2.6% | +1.2% | +3.5% |
| 28 | 488628ns | +2.3% | +3.6% | +1.0% | +3.9% |
| 29 | 494182ns | -0.2% | +5.0% | -0.4% | +0.4% |
| 30 | 488878ns | +1.8% | +8.8% | +0.1% | +3.5% |
| 31 | 597166ns | -5.9% | -4.8% | +1.0% | -11.8% |
| 32 | 601626ns | -1.9% | -8.4% | -3.2% | -10.7% |
| 33 | 504110ns | +5.1% | +9.3% | -0.1% | +16.3% |
| 34 | 502078ns | +4.8% | +0.6% | +0.5% | +17.8% |
| 35 | 512846ns | -1.9% | -3.6% | +2.2% | +6.7% |
| 36 | 501593ns | -0.2% | -1.9% | -0.2% | -0.1% |
| 37 | 498441ns | +3.3% | -1.3% | -1.1% | +0.6% |
| 38 | 497004ns | +8.1% | -0.7% | +8.5% | +0.4% |
| 39 | 495762ns | +5.2% | +0.5% | +15.2% | +0.8% |
| 40 | 494861ns | +11.9% | +2.9% | +18.3% | +13.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.373 | moderate+ |
| wide-rung-ragged | 0.270 | moderate+ |
| wide-rung-ragged-overread | 0.624 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.425 | moderate+ |
| wide-rung-wordround-alias | 0.540 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 18/40, lost 22/40
- **wide-rung-ragged-overread**: won 27/40, lost 13/40
- **wide-rung-wordround**: won 24/40, lost 15/40
- **wide-rung-wordround-alias**: won 19/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 21.0ns | 512928.0ns | 0.0% |  |
| wide-rung-ragged | 34.5ns | 515923.0ns | 0.0% |  |
| wide-rung-ragged-overread | 15.6ns | 505890.9ns | 0.0% |  |
| wide-rung-wordround | 17.8ns | 511179.4ns | 0.0% |  |
| wide-rung-wordround-alias | 18.6ns | 513181.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 491654.8-551934.1 ns)
  491654.8 |########################
  494668.8 |########################################
  497682.7 |################################
  500696.7 |########################
  503710.7 |################################
  506724.6 |################
  509738.6 |########
  512752.6 |########
  515766.5 |################
  518780.5 |################
  521794.4 |################
  524808.4 |########
  527822.4 |
  530836.3 |########################
  533850.3 |
  536864.3 |
  539878.2 |
  542892.2 |########
  545906.2 |########
  548920.1 |
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 490220.6-554156.4 ns)
  490220.6 |######
  493417.4 |#############
  496614.1 |######
  499810.9 |########################################
  503007.7 |####################
  506204.5 |######
  509401.3 |#############
  512598.1 |##########################
  515794.9 |
  518991.7 |#############
  522188.5 |
  525385.3 |##########################
  528582.1 |#############
  531778.9 |
  534975.6 |######
  538172.4 |
  541369.2 |
  544566.0 |
  547762.8 |
  550959.6 |####################
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 491562.2-535150.4 ns)
  491562.2 |########################################
  493741.6 |########
  495921.0 |####
  498100.4 |################
  500279.8 |####
  502459.2 |################
  504638.6 |################
  506818.0 |
  508997.5 |####
  511176.9 |########
  513356.3 |
  515535.7 |####
  517715.1 |########
  519894.5 |
  522073.9 |
  524253.3 |####
  526432.7 |
  528612.1 |
  530791.5 |####
  532971.0 |
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 489234.6-558164.3 ns)
  489234.6 |########################################
  492681.1 |###########
  496127.6 |#################
  499574.1 |#################
  503020.6 |########################################
  506467.0 |#################
  509913.5 |
  513360.0 |#####
  516806.5 |#####
  520253.0 |#####
  523699.5 |###########
  527146.0 |
  530592.4 |
  534038.9 |#####
  537485.4 |#####
  540931.9 |
  544378.4 |
  547824.9 |
  551271.4 |
  554717.8 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 493864.5-550915.0 ns)
  493864.5 |########################
  496717.0 |########################
  499569.6 |########################################
  502422.1 |########################
  505274.6 |########################################
  508127.1 |################################
  510979.7 |########
  513832.2 |################
  516684.7 |########
  519537.2 |
  522389.8 |########
  525242.3 |################
  528094.8 |########
  530947.3 |
  533799.9 |
  536652.4 |########
  539504.9 |
  542357.4 |
  545210.0 |########
  548062.5 |
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.54 (measurement drift or warm-up artifact)
