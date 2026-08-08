# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 2.8% of the fastest

All 5 variants sit between 422.03 us and 433.85 us - a 2.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 422029.6 ns median (-1.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 422029.6 ns, slowest 433851.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 434164ns | 431285ns | 422813ns | 431442ns | 453680ns | base |
| wide-rung-ragged | 428009ns | 427291ns | 421892ns | 427781ns | 434810ns | -1.42% |
| wide-rung-ragged-overread | 424214ns | 422458ns | 417642ns | 422427ns | 436149ns | -2.29% |
| wide-rung-wordround | 436145ns | 434478ns | 430770ns | 435034ns | 444853ns | +0.46% |
| wide-rung-wordround-alias | 434353ns | 432489ns | 428740ns | 433285ns | 443171ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 433071ns | 421848ns | 452440ns | base | 1.059 |
| wide-rung-ragged | 427316ns | 421154ns | 434136ns | -1.33% | 1.074 |
| wide-rung-ragged-overread | 423680ns | 417195ns | 435484ns | -2.17% | 1.083 |
| wide-rung-wordround | 435330ns | 429879ns | 444178ns | +0.52% | 1.054 |
| wide-rung-wordround-alias | 433639ns | 428035ns | 442525ns | +0.13% | 1.058 |

## Performance model

- Peak throughput: **1.100 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.067 | 97.0% |
| wide-rung-ragged | 1.075 | 97.8% |
| wide-rung-ragged-overread | 1.087 | 98.9% |
| wide-rung-wordround | 1.057 | 96.2% |
| wide-rung-wordround-alias | 1.062 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 434164ns | 434164ns | base |
| wide-rung-ragged | 428009ns | 428009ns | -1.42% |
| wide-rung-ragged-overread | 424214ns | 424214ns | -2.29% |
| wide-rung-wordround | 436145ns | 436145ns | +0.46% |
| wide-rung-wordround-alias | 434353ns | 434353ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 430054ns | base | --- | [426056, 433576] | --- | --- | --- | --- |
| wide-rung-ragged | 426676ns | no significant difference | [-6817, +335]ns | [426020, 428421] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-ragged-overread | 422030ns | -6585.2ns (-1.5%) | [-13314, -3378]ns | [419689, 423691] | YES | 0.0027 | 0.0007 | 0 |
| wide-rung-wordround | 433851ns | +5157.0ns (+1.2%) | [+846, +7632]ns | [432371, 435971] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| wide-rung-wordround-alias | 431797ns | no significant difference | [-1700, +5627]ns | [430108, 434866] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 433173ns | -4.0% | -3.2% | -0.8% | -1.2% |
| 2 | 437453ns | -5.0% | -4.3% | -1.5% | -2.1% |
| 3 | 423370ns | +1.2% | -1.0% | +1.7% | +1.3% |
| 4 | 422101ns | +0.9% | -0.7% | +2.0% | +1.5% |
| 5 | 421592ns | -0.4% | -0.5% | +1.7% | +2.0% |
| 6 | 424768ns | +0.5% | -1.5% | +1.6% | +0.8% |
| 7 | 422680ns | +0.2% | -0.9% | +3.0% | +1.1% |
| 8 | 422605ns | +2.1% | +3.1% | +3.6% | +1.1% |
| 9 | 424147ns | +0.5% | -0.2% | +1.9% | +1.8% |
| 10 | 427370ns | -1.4% | -1.5% | +0.7% | +0.7% |
| 11 | 420362ns | +1.2% | +0.5% | +3.8% | +2.6% |
| 12 | 419989ns | +2.2% | +0.5% | +2.6% | +7.1% |
| 13 | 427977ns | +0.4% | +5.2% | +1.3% | +1.4% |
| 14 | 426145ns | +0.4% | +0.4% | +2.1% | +1.8% |
| 15 | 459333ns | -7.2% | -6.8% | -5.6% | -3.5% |
| 16 | 430745ns | -1.1% | -1.6% | +1.3% | +3.3% |
| 17 | 441195ns | -2.5% | -3.9% | -2.1% | -1.8% |
| 18 | 433962ns | -1.7% | -2.3% | +0.0% | +0.4% |
| 19 | 437448ns | -2.0% | -3.2% | -1.6% | -0.7% |
| 20 | 425966ns | +0.2% | -0.7% | +1.7% | +3.0% |
| 21 | 431378ns | -0.2% | -3.0% | +0.4% | +0.1% |
| 22 | 435472ns | -2.1% | -4.2% | +0.9% | -0.7% |
| 23 | 440354ns | -0.0% | -4.9% | -1.7% | -2.2% |
| 24 | 457250ns | -4.7% | -7.3% | -2.4% | -6.4% |
| 25 | 432413ns | -0.3% | -3.9% | +2.3% | +1.7% |
| 26 | 448510ns | -5.1% | -7.5% | -4.3% | -2.7% |
| 27 | 447275ns | -4.9% | -6.8% | -4.1% | -3.9% |
| 28 | 449980ns | -4.8% | -4.3% | -4.1% | -2.2% |
| 29 | 435882ns | -1.9% | -4.3% | +0.4% | -1.5% |
| 30 | 450368ns | -5.4% | +0.6% | -1.3% | -4.6% |
| 31 | 465606ns | -7.0% | -9.8% | -2.3% | -6.1% |
| 32 | 433191ns | -1.5% | -1.2% | -0.2% | +1.0% |
| 33 | 429362ns | -0.6% | -1.7% | +1.1% | +3.6% |
| 34 | 426401ns | +2.3% | -1.2% | +2.5% | +1.0% |
| 35 | 425201ns | -0.2% | -0.3% | +2.2% | +0.8% |
| 36 | 424125ns | -0.2% | +1.4% | +3.6% | +2.9% |
| 37 | 429302ns | -1.0% | -2.0% | +3.4% | +1.4% |
| 38 | 425284ns | -0.0% | +0.6% | +3.8% | +1.1% |
| 39 | 431022ns | +0.8% | -2.1% | +1.0% | -0.1% |
| 40 | 422085ns | +1.5% | +0.1% | +4.3% | +4.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.434 | moderate+ |
| wide-rung-ragged | 0.245 | moderate+ |
| wide-rung-ragged-overread | -0.042 | ok |
| wide-rung-wordround | 0.360 | moderate+ |
| wide-rung-wordround-alias | 0.213 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 24/40, lost 14/40
- **wide-rung-ragged-overread**: won 31/40, lost 8/40
- **wide-rung-wordround**: won 13/40, lost 26/40
- **wide-rung-wordround-alias**: won 14/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 7.4ns | 433071.0ns | 0.0% |  |
| wide-rung-ragged | 9.1ns | 427316.1ns | 0.0% |  |
| wide-rung-ragged-overread | 8.3ns | 423680.5ns | 0.0% |  |
| wide-rung-wordround | 7.0ns | 435329.8ns | 0.0% |  |
| wide-rung-wordround-alias | 9.5ns | 433639.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 421848.0-452439.5 ns)
  421848.0 |########################################
  423377.6 |########################
  424907.2 |########################################
  426436.7 |########
  427966.3 |########################
  429495.9 |################
  431025.5 |################
  432555.0 |########################
  434084.6 |########
  435614.2 |########
  437143.8 |################
  438673.3 |
  440202.9 |################
  441732.5 |
  443262.1 |
  444791.6 |
  446321.2 |########
  447850.8 |########
  449380.4 |################
  450909.9 |
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 421154.5-434135.6 ns)
  421154.5 |######
  421803.5 |
  422452.6 |
  423101.6 |#############
  423750.7 |######
  424399.8 |######
  425048.8 |##########################
  425697.9 |#################################
  426346.9 |########################################
  426996.0 |######
  427645.0 |######
  428294.1 |##########################
  428943.2 |#############
  429592.2 |######
  430241.3 |######
  430890.3 |######
  431539.4 |######
  432188.5 |
  432837.5 |######
  433486.6 |
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 417194.7-435484.0 ns)
  417194.7 |######
  418109.2 |#################################
  419023.7 |#################################
  419938.1 |######
  420852.6 |#############
  421767.1 |#################################
  422681.5 |#############
  423596.0 |########################################
  424510.4 |
  425424.9 |
  426339.4 |
  427253.8 |##########################
  428168.3 |
  429082.8 |
  429997.2 |#############
  430911.7 |
  431826.2 |
  432740.6 |
  433655.1 |
  434569.5 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 429878.7-444178.3 ns)
  429878.7 |########################################
  430593.7 |####################
  431308.7 |##############################
  432023.7 |####################
  432738.7 |##############################
  433453.6 |##############################
  434168.6 |####################
  434883.6 |##############################
  435598.6 |
  436313.5 |####################
  437028.5 |##############################
  437743.5 |
  438458.5 |
  439173.4 |####################
  439888.4 |##########
  440603.4 |
  441318.4 |##########
  442033.3 |##########
  442748.3 |
  443463.3 |##########
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 428034.6-442525.1 ns)
  428034.6 |########################################
  428759.1 |################
  429483.6 |################################
  430208.2 |################################
  430932.7 |################
  431657.2 |################
  432381.7 |
  433106.3 |################
  433830.8 |################
  434555.3 |
  435279.8 |################
  436004.4 |################
  436728.9 |################
  437453.4 |
  438177.9 |########
  438902.5 |########
  439627.0 |################
  440351.5 |
  441076.0 |
  441800.6 |
  (3 below, 4 above range)

```
