# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (325.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 299.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-ragged, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-ragged (299.58 us) and wide-rung-ragged-overread (299.89 us) differ by 0.10%, inside the noise, even though the wider field spreads 8.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader wide-rung-ragged vs stability leader wide-rung-ragged-overread (+0% speed for 1.3x steadier)

wide-rung-ragged is fastest (299.58 us, CV 4.6%); wide-rung-ragged-overread gives up 0.1% median for 1.3x lower variance (CV 3.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged** at 299580.6 ns median (-8.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.09x (fastest 299580.6 ns, slowest 325507.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 336400ns | 326361ns | 306621ns | 329171ns | 387864ns | base |
| wide-rung-ragged | 302124ns | 300104ns | 287776ns | 300572ns | 321131ns | -10.19% |
| wide-rung-ragged-overread | 302313ns | 300773ns | 289887ns | 301043ns | 318549ns | -10.13% |
| wide-rung-wordround | 307513ns | 302949ns | 288677ns | 302017ns | 342834ns | -8.59% |
| wide-rung-wordround-alias | 306435ns | 305265ns | 293468ns | 305311ns | 322774ns | -8.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 335273ns | 305648ns | 386602ns | base | 1.368 |
| wide-rung-ragged | 301436ns | 286871ns | 320458ns | -10.09% | 1.522 |
| wide-rung-ragged-overread | 301469ns | 288959ns | 317859ns | -10.08% | 1.522 |
| wide-rung-wordround | 306643ns | 287198ns | 342060ns | -8.54% | 1.496 |
| wide-rung-wordround-alias | 305583ns | 292778ns | 321938ns | -8.86% | 1.501 |

## Performance model

- Peak throughput: **1.599 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.409 | 88.1% |
| wide-rung-ragged | 1.531 | 95.8% |
| wide-rung-ragged-overread | 1.530 | 95.7% |
| wide-rung-wordround | 1.517 | 94.9% |
| wide-rung-wordround-alias | 1.506 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 336400ns | 336400ns | base |
| wide-rung-ragged | 302124ns | 302124ns | -10.19% |
| wide-rung-ragged-overread | 302313ns | 302313ns | -10.13% |
| wide-rung-wordround | 307513ns | 307513ns | -8.59% |
| wide-rung-wordround-alias | 306435ns | 306435ns | -8.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 325507ns | base | --- | [318264, 335028] | --- | --- | --- | --- |
| wide-rung-ragged | 299581ns | -19771.1ns (-6.1%) | [-37693, -13292]ns | [293572, 306750] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 299886ns | -22741.5ns (-7.0%) | [-32371, -15758]ns | [295806, 304180] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 302325ns | -28014.4ns (-8.6%) | [-35098, -15972]ns | [291757, 310638] | YES | 0.0007 | 0.0007 | 0 |
| wide-rung-wordround-alias | 304580ns | -17876.9ns (-5.5%) | [-32175, -8863]ns | [302096, 307316] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 367472ns | -22.0% | -22.1% | -21.1% | -17.6% |
| 2 | 392036ns | -26.9% | -27.1% | -26.8% | -25.5% |
| 3 | 409091ns | -29.9% | -29.3% | -29.5% | -28.4% |
| 4 | 335491ns | -13.4% | -12.3% | -14.4% | -13.0% |
| 5 | 321680ns | -2.5% | -10.2% | -10.6% | -8.2% |
| 6 | 318693ns | +0.5% | +0.6% | -9.6% | +8.4% |
| 7 | 314933ns | -3.3% | +3.2% | -7.5% | -3.6% |
| 8 | 313724ns | -2.7% | -1.1% | -8.6% | -1.8% |
| 9 | 340521ns | -9.5% | -9.4% | -15.9% | -8.3% |
| 10 | 309820ns | -2.1% | -3.2% | -7.7% | -2.4% |
| 11 | 306517ns | -6.4% | -2.2% | -4.7% | -0.3% |
| 12 | 395214ns | -27.3% | -22.8% | -25.9% | -24.9% |
| 13 | 382525ns | -23.2% | -23.8% | -20.9% | -23.4% |
| 14 | 396091ns | -26.8% | -24.3% | -20.4% | -21.3% |
| 15 | 323450ns | -11.2% | -7.0% | -5.5% | -1.9% |
| 16 | 348701ns | -17.7% | -15.2% | -10.2% | -17.5% |
| 17 | 319018ns | -6.6% | -8.3% | -4.4% | -3.5% |
| 18 | 312738ns | -6.7% | -6.7% | -7.0% | +2.6% |
| 19 | 326209ns | -6.0% | -6.9% | -9.5% | -9.9% |
| 20 | 363612ns | -19.4% | -17.5% | -13.3% | -14.6% |
| 21 | 324805ns | -3.0% | -3.8% | -10.6% | -2.0% |
| 22 | 382794ns | -25.1% | -22.4% | -21.1% | -20.2% |
| 23 | 330707ns | -11.8% | -6.3% | -12.0% | -8.8% |
| 24 | 332070ns | -5.7% | -9.6% | -8.7% | -3.6% |
| 25 | 313746ns | -6.2% | -3.5% | +0.9% | -1.5% |
| 26 | 335412ns | -13.8% | -3.2% | -5.7% | -9.5% |
| 27 | 316613ns | -5.4% | -2.6% | -1.2% | -1.9% |
| 28 | 328497ns | -6.0% | -5.5% | -9.3% | -0.1% |
| 29 | 367592ns | -16.3% | -12.1% | -20.8% | -14.8% |
| 30 | 347285ns | -14.5% | -9.1% | -15.9% | -11.9% |
| 31 | 300418ns | -0.9% | -4.2% | +43.4% | -0.1% |
| 32 | 305012ns | +0.8% | -4.9% | +2.0% | -3.2% |
| 33 | 294567ns | +2.8% | +5.6% | +10.6% | +3.6% |
| 34 | 303048ns | +5.6% | -2.3% | +2.3% | -1.5% |
| 35 | 313062ns | -0.7% | -5.3% | -0.3% | -3.1% |
| 36 | 314818ns | -2.6% | -3.2% | +8.0% | -2.6% |
| 37 | 327078ns | -4.9% | -10.7% | +1.8% | -6.8% |
| 38 | 317835ns | -2.2% | -7.1% | -3.0% | -4.3% |
| 39 | 323378ns | +11.2% | -9.3% | +2.6% | -7.5% |
| 40 | 334643ns | -10.5% | -10.8% | +2.3% | -10.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.430 | moderate+ |
| wide-rung-ragged | 0.276 | moderate+ |
| wide-rung-ragged-overread | 0.389 | moderate+ |
| wide-rung-wordround | 0.207 | moderate+ |
| wide-rung-wordround-alias | 0.027 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 35/40, lost 5/40
- **wide-rung-ragged-overread**: won 37/40, lost 3/40
- **wide-rung-wordround**: won 31/40, lost 9/40
- **wide-rung-wordround-alias**: won 36/40, lost 3/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 11.0ns | 335272.9ns | 0.0% |  |
| wide-rung-ragged | 7.8ns | 301436.1ns | 0.0% |  |
| wide-rung-ragged-overread | 7.1ns | 301469.1ns | 0.0% |  |
| wide-rung-wordround | 14.5ns | 306642.9ns | 0.0% |  |
| wide-rung-wordround-alias | 7.8ns | 305582.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 305647.6-386601.9 ns)
  305647.6 |##########
  309695.3 |########################################
  313743.0 |########################################
  317790.7 |########################################
  321838.4 |##############################
  325886.1 |##############################
  329933.8 |####################
  333981.6 |##############################
  338029.3 |##########
  342077.0 |
  346124.7 |####################
  350172.4 |
  354220.1 |
  358267.8 |
  362315.6 |##########
  366363.3 |####################
  370411.0 |
  374458.7 |
  378506.4 |##########
  382554.1 |##########
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 286870.7-320458.4 ns)
  286870.7 |########################################
  288550.1 |##########################
  290229.5 |########################################
  291908.9 |#############
  293588.3 |##########################
  295267.7 |
  296947.0 |########################################
  298626.4 |##########################
  300305.8 |
  301985.2 |##########################
  303664.6 |##########################
  305343.9 |##########################
  307023.3 |########################################
  308702.7 |#############
  310382.1 |########################################
  312061.5 |##########################
  313740.9 |#############
  315420.2 |
  317099.6 |
  318779.0 |##########################
  (5 below, 1 above range)

wide-rung-ragged-overread (n=40, range 288959.0-317859.5 ns)
  288959.0 |########################
  290404.0 |########
  291849.1 |########################
  293294.1 |################
  294739.1 |########################
  296184.1 |################
  297629.2 |########
  299074.2 |########################################
  300519.2 |########
  301964.2 |########
  303409.2 |################
  304854.3 |########
  306299.3 |
  307744.3 |################
  309189.3 |########################
  310634.4 |########
  312079.4 |########
  313524.4 |
  314969.4 |########
  316414.5 |
  (3 below, 4 above range)

wide-rung-wordround (n=40, range 287198.3-342060.0 ns)
  287198.3 |############################
  289941.4 |########################################
  292684.5 |###########
  295427.6 |#####
  298170.7 |
  300913.8 |#################
  303656.8 |###########
  306399.9 |#####
  309143.0 |###########
  311886.1 |#################
  314629.2 |######################
  317372.3 |
  320115.4 |
  322858.4 |
  325601.5 |#####
  328344.6 |
  331087.7 |###########
  333830.8 |
  336573.9 |
  339317.0 |#####
  (4 below, 2 above range)

wide-rung-wordround-alias (n=40, range 292778.5-321937.6 ns)
  292778.5 |########################
  294236.5 |################
  295694.4 |########
  297152.4 |################
  298610.3 |################
  300068.3 |
  301526.2 |########################
  302984.2 |################################
  304442.1 |########################################
  305900.1 |########
  307358.0 |################
  308816.0 |########
  310274.0 |################
  311731.9 |########################
  313189.9 |
  314647.8 |
  316105.8 |########
  317563.7 |########
  319021.7 |########
  320479.6 |########
  (3 below, 2 above range)

```
