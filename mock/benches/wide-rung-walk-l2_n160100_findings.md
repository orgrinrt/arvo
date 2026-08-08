# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.50)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 1.9% of the fastest

All 5 variants sit between 423.15 us and 431.24 us - a 1.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 423150.4 ns median (-1.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.02x (fastest 423150.4 ns, slowest 431240.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 433895ns | 429597ns | 425697ns | 431128ns | 450396ns | base |
| wide-rung-ragged | 426204ns | 425267ns | 421502ns | 425371ns | 433407ns | -1.77% |
| wide-rung-ragged-overread | 423671ns | 423875ns | 419923ns | 423511ns | 427900ns | -2.36% |
| wide-rung-wordround | 432884ns | 432065ns | 429363ns | 432069ns | 438852ns | -0.23% |
| wide-rung-wordround-alias | 431282ns | 430827ns | 428234ns | 430683ns | 436126ns | -0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 432655ns | 424418ns | 449255ns | base | 1.060 |
| wide-rung-ragged | 425556ns | 420974ns | 432699ns | -1.64% | 1.078 |
| wide-rung-ragged-overread | 422947ns | 419000ns | 427194ns | -2.24% | 1.085 |
| wide-rung-wordround | 432055ns | 428475ns | 438219ns | -0.14% | 1.062 |
| wide-rung-wordround-alias | 430586ns | 427620ns | 435355ns | -0.48% | 1.065 |

## Performance model

- Peak throughput: **1.095 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.071 | 97.8% |
| wide-rung-ragged | 1.080 | 98.7% |
| wide-rung-ragged-overread | 1.084 | 99.0% |
| wide-rung-wordround | 1.064 | 97.2% |
| wide-rung-wordround-alias | 1.067 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 433895ns | 433895ns | base |
| wide-rung-ragged | 426204ns | 426204ns | -1.77% |
| wide-rung-ragged-overread | 423671ns | 423671ns | -2.36% |
| wide-rung-wordround | 432884ns | 432884ns | -0.23% |
| wide-rung-wordround-alias | 431282ns | 431282ns | -0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 428286ns | base | --- | [426869, 432098] | --- | --- | --- | --- |
| wide-rung-ragged | 424634ns | -3490.6ns (-0.8%) | [-5821, -1874]ns | [423447, 425506] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 423150ns | -5619.6ns (-1.3%) | [-8696, -4083]ns | [422323, 423573] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 431240ns | no significant difference | [-265, +4513]ns | [430312, 431541] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround-alias | 430135ns | no significant difference | [-3333, +3304]ns | [428945, 430764] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 437538ns | -4.1% | -3.9% | -1.4% | -1.3% |
| 2 | 456169ns | -8.0% | -7.6% | -6.1% | -5.1% |
| 3 | 424785ns | -1.2% | -1.0% | +1.0% | +1.6% |
| 4 | 443535ns | -5.0% | -4.3% | -3.3% | -3.0% |
| 5 | 427499ns | -1.4% | -1.2% | +0.9% | +0.7% |
| 6 | 428185ns | -1.7% | -2.0% | +0.3% | +0.5% |
| 7 | 424158ns | -0.0% | -1.4% | +1.1% | +1.8% |
| 8 | 471963ns | -9.9% | -10.6% | -8.6% | -8.7% |
| 9 | 437910ns | -2.8% | -4.4% | -1.9% | -1.5% |
| 10 | 437915ns | -2.8% | -4.3% | -1.5% | -0.3% |
| 11 | 428388ns | -0.8% | -0.7% | +0.1% | -0.3% |
| 12 | 426430ns | -0.2% | +0.1% | +0.5% | -0.1% |
| 13 | 427367ns | -0.1% | -0.6% | +0.2% | -0.2% |
| 14 | 427542ns | -0.6% | -0.9% | +0.2% | +1.4% |
| 15 | 438417ns | -3.1% | -3.5% | -1.6% | -2.3% |
| 16 | 432314ns | -0.6% | -1.4% | +1.2% | -0.9% |
| 17 | 431410ns | -0.3% | -2.1% | -0.5% | -0.9% |
| 18 | 457936ns | -6.9% | -6.7% | -6.7% | -6.4% |
| 19 | 446869ns | -5.0% | -5.0% | -2.9% | -4.0% |
| 20 | 431883ns | +3.8% | -2.0% | +2.3% | -0.8% |
| 21 | 426938ns | -0.8% | -1.0% | +3.0% | +3.8% |
| 22 | 426800ns | -0.8% | -0.3% | +2.4% | +1.5% |
| 23 | 425762ns | -0.4% | -0.6% | +4.0% | +1.2% |
| 24 | 428663ns | -0.9% | -1.2% | +0.7% | +0.0% |
| 25 | 425755ns | -0.1% | -0.7% | +2.3% | +0.7% |
| 26 | 425608ns | -0.5% | -0.5% | +1.9% | +1.1% |
| 27 | 426074ns | +2.2% | -0.6% | +1.2% | +0.7% |
| 28 | 428928ns | +0.0% | -1.1% | +0.8% | +0.3% |
| 29 | 425445ns | +1.0% | -0.4% | +1.2% | +1.1% |
| 30 | 434546ns | -1.3% | -2.4% | -1.1% | -0.8% |
| 31 | 427586ns | -1.1% | -2.0% | +0.9% | +2.5% |
| 32 | 423698ns | -0.2% | -0.8% | +3.1% | +1.5% |
| 33 | 426668ns | -1.0% | -0.1% | +1.8% | +0.8% |
| 34 | 437898ns | -3.4% | -0.7% | -1.4% | -2.0% |
| 35 | 426459ns | +0.7% | -1.5% | +0.8% | +1.5% |
| 36 | 431546ns | -1.8% | -1.9% | +0.7% | -0.4% |
| 37 | 441235ns | -3.7% | -4.1% | -2.3% | -2.8% |
| 38 | 432461ns | -0.6% | -3.0% | -0.3% | -0.9% |
| 39 | 422733ns | -0.0% | -1.0% | +1.9% | +1.5% |
| 40 | 423164ns | +0.0% | +0.0% | +1.8% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.113 | ok |
| wide-rung-ragged | 0.122 | ok |
| wide-rung-ragged-overread | 0.200 | moderate+ |
| wide-rung-wordround | 0.500 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.131 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 31/40, lost 4/40
- **wide-rung-ragged-overread**: won 37/40, lost 0/40
- **wide-rung-wordround**: won 14/40, lost 26/40
- **wide-rung-wordround-alias**: won 20/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 6.6ns | 432654.5ns | 0.0% |  |
| wide-rung-ragged | 7.2ns | 425555.6ns | 0.0% |  |
| wide-rung-ragged-overread | 13.3ns | 422947.3ns | 0.0% |  |
| wide-rung-wordround | 7.4ns | 432054.9ns | 0.0% |  |
| wide-rung-wordround-alias | 6.9ns | 430586.3ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 424418.2-449254.8 ns)
  424418.2 |#################
  425660.1 |########################################
  426901.9 |############################
  428143.7 |######################
  429385.6 |
  430627.4 |###########
  431869.2 |#################
  433111.0 |
  434352.9 |#####
  435594.7 |
  436836.5 |######################
  438078.4 |#####
  439320.2 |
  440562.0 |#####
  441803.9 |
  443045.7 |#####
  444287.5 |
  445529.4 |
  446771.2 |#####
  448013.0 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 420973.6-432698.9 ns)
  420973.6 |################
  421559.8 |
  422146.1 |################
  422732.4 |################################
  423318.6 |################################
  423904.9 |########################
  424491.2 |########################################
  425077.4 |########################
  425663.7 |########################
  426250.0 |
  426836.2 |########
  427422.5 |
  428008.8 |
  428595.0 |################
  429181.3 |########################
  429767.6 |################
  430353.8 |
  430940.1 |
  431526.4 |
  432112.6 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 419000.0-427193.8 ns)
  419000.0 |#############
  419409.7 |######
  419819.4 |#############
  420229.1 |#############
  420638.8 |
  421048.5 |
  421458.1 |######
  421867.8 |######
  422277.5 |##########################
  422687.2 |#############
  423096.9 |########################################
  423506.6 |####################
  423916.3 |#############
  424326.0 |#############
  424735.7 |######
  425145.4 |#############
  425555.1 |
  425964.8 |######
  426374.5 |######
  426784.2 |#############
  (4 below, 1 above range)

wide-rung-wordround (n=40, range 428475.3-438219.1 ns)
  428475.3 |######################
  428962.5 |#################
  429449.7 |#################
  429936.9 |
  430424.1 |#################
  430911.3 |########################################
  431398.4 |######################
  431885.6 |#####
  432372.8 |
  432860.0 |
  433347.2 |###########
  433834.4 |#####
  434321.6 |#####
  434808.8 |
  435296.0 |#####
  435783.2 |
  436270.3 |#####
  436757.5 |
  437244.7 |###########
  437731.9 |
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 427620.5-435355.4 ns)
  427620.5 |
  428007.2 |
  428394.0 |########################################
  428780.7 |############################
  429167.5 |#####
  429554.2 |#####
  429941.0 |##################################
  430327.7 |###########
  430714.4 |#####
  431101.2 |######################
  431487.9 |###########
  431874.7 |
  432261.4 |
  432648.2 |#####
  433034.9 |###########
  433421.7 |#####
  433808.4 |
  434195.1 |
  434581.9 |
  434968.6 |
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-wordround**: autocorrelation=0.50 (measurement drift or warm-up artifact)
