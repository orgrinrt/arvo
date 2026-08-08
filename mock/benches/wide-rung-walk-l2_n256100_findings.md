# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.63)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.63, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.13 us) is smaller than the fastest variant's own run-to-run std-dev (10.03 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 5 variants sit between 490.56 us and 493.69 us - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged** at 490560.0 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 490560.0 ns, slowest 493685.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 495773ns | 494117ns | 489008ns | 495180ns | 504316ns | base |
| wide-rung-ragged | 495663ns | 491779ns | 487908ns | 493014ns | 511365ns | -0.02% |
| wide-rung-ragged-overread | 501910ns | 493144ns | 484035ns | 493112ns | 546180ns | +1.24% |
| wide-rung-wordround | 500319ns | 494559ns | 490184ns | 495639ns | 524493ns | +0.92% |
| wide-rung-wordround-alias | 496010ns | 493475ns | 488981ns | 494168ns | 508564ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 494852ns | 488091ns | 503467ns | base | 0.927 |
| wide-rung-ragged | 494663ns | 487081ns | 510444ns | -0.04% | 0.927 |
| wide-rung-ragged-overread | 500855ns | 483293ns | 544939ns | +1.21% | 0.916 |
| wide-rung-wordround | 499294ns | 489322ns | 523441ns | +0.90% | 0.919 |
| wide-rung-wordround-alias | 494826ns | 487492ns | 507407ns | -0.01% | 0.927 |

## Performance model

- Peak throughput: **0.949 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.930 | 98.0% |
| wide-rung-ragged | 0.935 | 98.5% |
| wide-rung-ragged-overread | 0.933 | 98.3% |
| wide-rung-wordround | 0.929 | 97.9% |
| wide-rung-wordround-alias | 0.931 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 495773ns | 495773ns | base |
| wide-rung-ragged | 495663ns | 495663ns | -0.02% |
| wide-rung-ragged-overread | 501910ns | 501910ns | +1.24% |
| wide-rung-wordround | 500319ns | 500319ns | +0.92% |
| wide-rung-wordround-alias | 496010ns | 496010ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 493059ns | base | --- | [491560, 497317] | --- | --- | --- | --- |
| wide-rung-ragged | 490560ns | -4109.0ns (-0.8%) | [-6720, -204]ns | [489657, 494215] | YES (adj: no) | 0.3228 | 0.0807 | 0 |
| wide-rung-ragged-overread | 491683ns | no significant difference | [-5938, +1990]ns | [489465, 493488] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround | 493685ns | no significant difference | [-2064, +3087]ns | [491283, 496649] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround-alias | 492639ns | no significant difference | [-4443, +2259]ns | [490376, 495125] | no | 0.8478 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 492321ns | +9.3% | +15.2% | +0.3% | +2.9% |
| 2 | 499842ns | -1.5% | +39.4% | -1.2% | +1.9% |
| 3 | 498897ns | -1.9% | +12.5% | +1.8% | +2.4% |
| 4 | 490602ns | -0.2% | +2.3% | +1.0% | +1.4% |
| 5 | 495522ns | -1.1% | +0.7% | -0.1% | -0.5% |
| 6 | 497088ns | -2.0% | -0.8% | +10.3% | -1.5% |
| 7 | 497116ns | -1.3% | +0.2% | +12.7% | -1.3% |
| 8 | 492772ns | +1.1% | +0.7% | +8.1% | +4.0% |
| 9 | 489907ns | -0.8% | +6.8% | +4.9% | +0.2% |
| 10 | 491656ns | -0.9% | +0.4% | +3.1% | -0.3% |
| 11 | 489779ns | -0.2% | +0.4% | +0.1% | +1.0% |
| 12 | 488896ns | +2.9% | +0.8% | +0.5% | +2.4% |
| 13 | 498107ns | -0.3% | +0.2% | -1.6% | -0.7% |
| 14 | 507968ns | -3.7% | -2.8% | -3.1% | -2.6% |
| 15 | 489266ns | +0.1% | +0.7% | +0.1% | +0.6% |
| 16 | 491712ns | +1.1% | +0.6% | -0.2% | +0.5% |
| 17 | 501767ns | -2.4% | -1.7% | -0.7% | -1.1% |
| 18 | 496438ns | -1.5% | -1.1% | -1.4% | -1.0% |
| 19 | 488028ns | +0.3% | +0.4% | +4.0% | +1.5% |
| 20 | 506087ns | -2.8% | -2.6% | -3.0% | -1.5% |
| 21 | 492258ns | -0.5% | -0.5% | +0.9% | -1.1% |
| 22 | 491465ns | +1.2% | -1.7% | -0.0% | -1.0% |
| 23 | 487496ns | +3.4% | -0.9% | +3.2% | +0.6% |
| 24 | 491424ns | -1.0% | +1.4% | -0.5% | -0.9% |
| 25 | 486690ns | +4.7% | +0.5% | +2.0% | +0.1% |
| 26 | 485836ns | +3.1% | -0.4% | +0.8% | +0.9% |
| 27 | 489870ns | +4.1% | -1.3% | +0.1% | +4.5% |
| 28 | 488734ns | +0.6% | -0.9% | +3.6% | -0.3% |
| 29 | 490142ns | +1.3% | -1.4% | +0.4% | -0.5% |
| 30 | 500761ns | -1.8% | -3.5% | -1.7% | -0.9% |
| 31 | 503820ns | -1.4% | -2.9% | -0.7% | -3.1% |
| 32 | 501239ns | -2.0% | -2.4% | +1.8% | -2.5% |
| 33 | 498404ns | -1.6% | -1.8% | -0.2% | -1.6% |
| 34 | 497518ns | -0.3% | -1.7% | -0.3% | -1.6% |
| 35 | 500194ns | -2.3% | -2.1% | -2.3% | -1.7% |
| 36 | 505902ns | -3.2% | -3.2% | -3.4% | +0.4% |
| 37 | 493346ns | -1.0% | -2.3% | -0.8% | -0.1% |
| 38 | 493778ns | -0.8% | -0.9% | -0.0% | -0.7% |
| 39 | 498778ns | -2.9% | +2.5% | -1.3% | +0.0% |
| 40 | 492645ns | +5.4% | -0.2% | -0.5% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.253 | moderate+ |
| wide-rung-ragged | -0.010 | ok |
| wide-rung-ragged-overread | 0.552 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.632 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.191 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 26/40, lost 13/40
- **wide-rung-ragged-overread**: won 22/40, lost 18/40
- **wide-rung-wordround**: won 17/40, lost 18/40
- **wide-rung-wordround-alias**: won 21/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.0ns | 494851.7ns | 0.0% |  |
| wide-rung-ragged | 6.6ns | 494663.5ns | 0.0% |  |
| wide-rung-ragged-overread | 17.2ns | 500854.7ns | 0.0% |  |
| wide-rung-wordround | 9.2ns | 499294.5ns | 0.0% |  |
| wide-rung-wordround-alias | 7.1ns | 494826.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 488090.5-503467.2 ns)
  488090.5 |##########
  488859.3 |####################
  489628.2 |########################################
  490397.0 |##########
  491165.8 |########################################
  491934.7 |##############################
  492703.5 |####################
  493472.3 |##########
  494241.2 |
  495010.0 |##########
  495778.8 |##########
  496547.7 |####################
  497316.5 |##########
  498085.4 |##############################
  498854.2 |##########
  499623.0 |####################
  500391.9 |##########
  501160.7 |####################
  501929.5 |
  502698.4 |
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 487080.6-510443.6 ns)
  487080.6 |############
  488248.8 |################
  489416.9 |########################################
  490585.1 |####################
  491753.2 |####
  492921.4 |
  494089.5 |
  495257.7 |####
  496425.8 |####################
  497594.0 |####
  498762.1 |
  499930.2 |
  501098.4 |####
  502266.5 |####
  503434.7 |####
  504602.8 |
  505771.0 |
  506939.1 |
  508107.3 |
  509275.4 |########
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 483293.4-544938.8 ns)
  483293.4 |####################
  486375.7 |##############################
  489458.0 |###################################
  492540.2 |########################################
  495622.5 |###############
  498704.8 |###############
  501787.0 |
  504869.3 |
  507951.6 |
  511033.8 |#####
  514116.1 |
  517198.4 |
  520280.6 |#####
  523362.9 |
  526445.2 |
  529527.5 |
  532609.7 |
  535692.0 |
  538774.3 |
  541856.5 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 489322.4-523441.2 ns)
  489322.4 |########################################
  491028.3 |##############################
  492734.3 |###############
  494440.2 |###############
  496146.2 |###############
  497852.1 |#####
  499558.0 |#####
  501264.0 |#####
  502969.9 |
  504675.9 |
  506381.8 |####################
  508087.7 |
  509793.7 |#####
  511499.6 |
  513205.6 |#####
  514911.5 |
  516617.4 |
  518323.4 |
  520029.3 |
  521735.3 |
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 487491.9-507407.3 ns)
  487491.9 |################
  488487.7 |########################
  489483.5 |########################################
  490479.2 |########################
  491475.0 |################
  492470.8 |################
  493466.5 |########
  494462.3 |################################
  495458.1 |################
  496453.9 |########
  497449.6 |########
  498445.4 |########
  499441.2 |########
  500436.9 |########
  501432.7 |
  502428.5 |
  503424.3 |
  504420.0 |
  505415.8 |
  506411.6 |########
  (5 below, 5 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.63 (measurement drift or warm-up artifact)
