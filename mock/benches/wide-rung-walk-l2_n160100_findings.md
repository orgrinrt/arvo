# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (434.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 424.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.75)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 2.4% of the fastest

All 5 variants sit between 424.30 us and 434.65 us - a 2.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: wide-rung-ragged** at 424299.2 ns median (-2.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.02x (fastest 424299.2 ns, slowest 434652.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 443782ns | 435405ns | 428136ns | 438462ns | 475388ns | base |
| wide-rung-ragged | 426186ns | 425014ns | 417194ns | 425400ns | 437534ns | -3.96% |
| wide-rung-ragged-overread | 434941ns | 429105ns | 419487ns | 431209ns | 461593ns | -1.99% |
| wide-rung-wordround | 430624ns | 430274ns | 426572ns | 430130ns | 436156ns | -2.96% |
| wide-rung-wordround-alias | 434233ns | 429885ns | 425318ns | 429371ns | 457732ns | -2.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 442740ns | 426722ns | 474695ns | base | 1.036 |
| wide-rung-ragged | 425676ns | 416630ns | 437066ns | -3.85% | 1.078 |
| wide-rung-ragged-overread | 434312ns | 418929ns | 461043ns | -1.90% | 1.056 |
| wide-rung-wordround | 430084ns | 426130ns | 435736ns | -2.86% | 1.067 |
| wide-rung-wordround-alias | 433577ns | 424621ns | 457160ns | -2.07% | 1.058 |

## Performance model

- Peak throughput: **1.101 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.055 | 95.9% |
| wide-rung-ragged | 1.081 | 98.2% |
| wide-rung-ragged-overread | 1.070 | 97.2% |
| wide-rung-wordround | 1.068 | 97.0% |
| wide-rung-wordround-alias | 1.068 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 443782ns | 443782ns | base |
| wide-rung-ragged | 426186ns | 426186ns | -3.96% |
| wide-rung-ragged-overread | 434941ns | 434941ns | -1.99% |
| wide-rung-wordround | 430624ns | 430624ns | -2.96% |
| wide-rung-wordround-alias | 434233ns | 434233ns | -2.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 434652ns | base | --- | [430241, 439581] | --- | --- | --- | --- |
| wide-rung-ragged | 424299ns | -13373.8ns (-3.1%) | [-20127, -6303]ns | [423876, 426584] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 428570ns | -8139.4ns (-1.9%) | [-9497, -5809]ns | [425716, 431723] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-wordround | 429519ns | -5811.9ns (-1.3%) | [-12379, -2136]ns | [428699, 430279] | YES | 0.0007 | 0.0007 | 0 |
| wide-rung-wordround-alias | 429371ns | -5810.0ns (-1.3%) | [-9093, -2692]ns | [426115, 430917] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 430905ns | -1.4% | -0.2% | -1.2% | -0.4% |
| 2 | 425898ns | -0.5% | +0.6% | +0.4% | -0.1% |
| 3 | 434770ns | -2.5% | -1.4% | -1.4% | -2.2% |
| 4 | 430343ns | -1.2% | +0.4% | -0.8% | -0.3% |
| 5 | 438079ns | -3.1% | +2.8% | -2.8% | -2.7% |
| 6 | 440304ns | -3.9% | -1.3% | -2.9% | -3.2% |
| 7 | 439503ns | -3.9% | -2.2% | -3.1% | -3.1% |
| 8 | 436967ns | -2.9% | -1.4% | -2.6% | -2.1% |
| 9 | 426866ns | -0.7% | +1.1% | -0.3% | -0.2% |
| 10 | 429471ns | +0.1% | +0.5% | -0.8% | -0.8% |
| 11 | 476094ns | -10.0% | -3.7% | -7.3% | -6.2% |
| 12 | 503634ns | -9.1% | -9.7% | -14.3% | -1.3% |
| 13 | 478371ns | -10.4% | -1.1% | -10.0% | -2.1% |
| 14 | 463054ns | -7.6% | -2.0% | -5.8% | -1.9% |
| 15 | 478766ns | -9.7% | -5.8% | -8.7% | -6.2% |
| 16 | 461985ns | -6.4% | +1.9% | -5.9% | -6.0% |
| 17 | 464036ns | -7.8% | -1.8% | -7.1% | -5.0% |
| 18 | 463879ns | -5.4% | -4.0% | -6.4% | -7.0% |
| 19 | 469722ns | -8.4% | -2.4% | -7.1% | -8.2% |
| 20 | 458582ns | -6.9% | +1.0% | -6.1% | +1.9% |
| 21 | 438500ns | -5.2% | -2.9% | -2.0% | -1.5% |
| 22 | 434535ns | -4.5% | -2.2% | -0.8% | -0.8% |
| 23 | 460862ns | -3.9% | -7.6% | -6.7% | -6.8% |
| 24 | 434107ns | -3.7% | -1.7% | -1.3% | -0.6% |
| 25 | 443193ns | -6.0% | -3.6% | -3.0% | -2.9% |
| 26 | 429416ns | -0.2% | -0.2% | +0.2% | +0.6% |
| 27 | 438155ns | -4.8% | -2.8% | -1.8% | -1.8% |
| 28 | 439659ns | -5.2% | -3.2% | -2.1% | -2.1% |
| 29 | 429735ns | -3.0% | -0.7% | -0.1% | +0.1% |
| 30 | 429276ns | -2.9% | -0.9% | -0.3% | +0.2% |
| 31 | 426121ns | -0.5% | -1.7% | +0.8% | -0.5% |
| 32 | 430138ns | -0.9% | -2.7% | -0.3% | -1.3% |
| 33 | 431175ns | -1.5% | -2.0% | -0.7% | -1.4% |
| 34 | 430522ns | -1.5% | -2.8% | -0.2% | -1.3% |
| 35 | 427081ns | -1.1% | -1.4% | +0.3% | -0.4% |
| 36 | 427035ns | -1.2% | -1.1% | +0.4% | -0.6% |
| 37 | 427508ns | -0.8% | -1.9% | +0.4% | -0.8% |
| 38 | 428078ns | -0.9% | -2.2% | +0.4% | -0.6% |
| 39 | 426488ns | -1.3% | -1.9% | +0.7% | -0.4% |
| 40 | 426777ns | +1.2% | -2.1% | +1.8% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.739 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.182 | ok |
| wide-rung-ragged-overread | 0.753 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.443 | moderate+ |
| wide-rung-wordround-alias | 0.584 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 38/40, lost 1/40
- **wide-rung-ragged-overread**: won 33/40, lost 7/40
- **wide-rung-wordround**: won 30/40, lost 9/40
- **wide-rung-wordround-alias**: won 35/40, lost 3/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 6.2ns | 442739.7ns | 0.0% |  |
| wide-rung-ragged | 7.8ns | 425675.7ns | 0.0% |  |
| wide-rung-ragged-overread | 6.6ns | 434312.2ns | 0.0% |  |
| wide-rung-wordround | 6.3ns | 430084.4ns | 0.0% |  |
| wide-rung-wordround-alias | 5.8ns | 433577.1ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 426721.6-474694.6 ns)
  426721.6 |##########################
  429120.2 |########################################
  431518.9 |
  433917.5 |#############
  436316.2 |#################
  438714.8 |#############
  441113.5 |####
  443512.1 |
  445910.8 |
  448309.4 |
  450708.1 |
  453106.7 |
  455505.4 |
  457904.0 |####
  460302.7 |########
  462701.3 |#############
  465100.0 |
  467498.6 |####
  469897.3 |
  472295.9 |
  (3 below, 4 above range)

wide-rung-ragged (n=40, range 416629.6-437065.6 ns)
  416629.6 |##############
  417651.4 |###
  418673.2 |
  419695.0 |
  420716.8 |###
  421738.6 |##########
  422760.4 |###
  423782.2 |########################################
  424804.0 |###
  425825.8 |###
  426847.6 |###
  427869.4 |##################
  428891.2 |###
  429913.0 |###
  430934.8 |
  431956.6 |##########
  432978.4 |
  434000.2 |
  435022.0 |
  436043.8 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 418928.7-461043.1 ns)
  418928.7 |####################
  421034.4 |#############
  423140.1 |#############
  425245.9 |########################################
  427351.6 |##########################
  429457.3 |####################
  431563.0 |####################
  433668.7 |######
  435774.4 |
  437880.2 |
  439985.9 |
  442091.6 |
  444197.3 |######
  446303.0 |
  448408.8 |######
  450514.5 |######
  452620.2 |######
  454725.9 |#############
  456831.6 |#############
  458937.4 |
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 426130.1-435735.7 ns)
  426130.1 |
  426610.3 |##########
  427090.6 |##########
  427570.9 |##########
  428051.2 |##############################
  428531.5 |########################################
  429011.7 |########################################
  429492.0 |########################################
  429972.3 |########################################
  430452.6 |####################
  430932.9 |####################
  431413.1 |##########
  431893.4 |
  432373.7 |
  432854.0 |
  433334.3 |
  433814.6 |####################
  434294.8 |##########
  434775.1 |
  435255.4 |
  (6 below, 4 above range)

wide-rung-wordround-alias (n=40, range 424621.0-457160.2 ns)
  424621.0 |########################################
  426248.0 |###
  427875.0 |#########
  429501.9 |###############
  431128.9 |##################
  432755.8 |###
  434382.8 |
  436009.8 |
  437636.7 |
  439263.7 |
  440890.6 |###
  442517.6 |
  444144.5 |
  445771.5 |###
  447398.5 |
  449025.4 |###
  450652.4 |
  452279.3 |
  453906.3 |###
  455533.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.58 (measurement drift or warm-up artifact)
