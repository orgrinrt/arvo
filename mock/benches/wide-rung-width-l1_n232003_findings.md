# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.77)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.77, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (4 ns, 0.08%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by 4 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 5325.2 ns median (-2.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 5325.2 ns, slowest 5806.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5506ns | 5522ns | 5369ns | 5480ns | 5720ns | base |
| wide-rung-ragged | 5893ns | 5867ns | 5760ns | 5872ns | 6091ns | +7.04% |
| wide-rung-ragged-overread | 5452ns | 5386ns | 5372ns | 5414ns | 5642ns | -0.99% |
| wide-rung-wordround | 5483ns | 5478ns | 5371ns | 5491ns | 5571ns | -0.42% |
| wide-rung-wordround-alias | 5561ns | 5560ns | 5371ns | 5552ns | 5780ns | +1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5440ns | 5307ns | 5650ns | base | 1.506 |
| wide-rung-ragged | 5828ns | 5698ns | 6021ns | +7.13% | 1.406 |
| wide-rung-ragged-overread | 5388ns | 5310ns | 5577ns | -0.96% | 1.520 |
| wide-rung-wordround | 5419ns | 5309ns | 5508ns | -0.38% | 1.512 |
| wide-rung-wordround-alias | 5495ns | 5309ns | 5706ns | +1.01% | 1.491 |

## Performance model

- Peak throughput: **1.544 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.501 | 97.2% |
| wide-rung-ragged | 1.411 | 91.4% |
| wide-rung-ragged-overread | 1.538 | 99.7% |
| wide-rung-wordround | 1.513 | 98.0% |
| wide-rung-wordround-alias | 1.490 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5506ns | 5506ns | base |
| wide-rung-ragged | 5893ns | 5893ns | +7.04% |
| wide-rung-ragged-overread | 5452ns | 5452ns | -0.99% |
| wide-rung-wordround | 5483ns | 5483ns | -0.42% |
| wide-rung-wordround-alias | 5561ns | 5561ns | +1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5458ns | base | --- | [5311, 5496] | --- | --- | --- | --- |
| wide-rung-ragged | 5807ns | +399.8ns (+7.3%) | [+382, +451]ns | [5747, 5885] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 5325ns | no significant difference | [-127, +5]ns | [5314, 5373] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround | 5414ns | no significant difference | [-84, +40]ns | [5409, 5493] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround-alias | 5498ns | +10.7ns (+0.2%) | [+1, +100]ns | [5449, 5533] | YES (adj: no) | 0.0770 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5310ns | +10.2% | +2.8% | +3.5% | +0.0% |
| 2 | 5544ns | +2.9% | -4.2% | -0.9% | -4.2% |
| 3 | 5426ns | +4.9% | -1.1% | +1.3% | -2.2% |
| 4 | 5315ns | +7.3% | +2.4% | +3.4% | -0.1% |
| 5 | 5311ns | +7.3% | +1.3% | +5.1% | -0.0% |
| 6 | 5308ns | +8.6% | +0.1% | +3.4% | +0.1% |
| 7 | 5308ns | +7.4% | +0.9% | +3.5% | +0.0% |
| 8 | 5312ns | +7.3% | +0.0% | +2.7% | -0.0% |
| 9 | 5310ns | +7.3% | -0.0% | +0.0% | +0.0% |
| 10 | 5306ns | +9.8% | +0.1% | +0.1% | +0.0% |
| 11 | 5495ns | +7.4% | -0.0% | -1.5% | +0.6% |
| 12 | 5494ns | +7.5% | +0.0% | -1.4% | +0.3% |
| 13 | 5493ns | +7.5% | +0.1% | -1.4% | +0.1% |
| 14 | 5491ns | +7.4% | +0.1% | -1.5% | +0.1% |
| 15 | 5499ns | +4.7% | -0.1% | -1.5% | +1.3% |
| 16 | 5519ns | +3.4% | -0.5% | -2.0% | -0.4% |
| 17 | 5653ns | +1.0% | -3.3% | -3.9% | -2.9% |
| 18 | 5388ns | +5.8% | -1.4% | +0.4% | +1.9% |
| 19 | 5501ns | +3.6% | -3.5% | -1.6% | -3.3% |
| 20 | 5600ns | +1.8% | -5.2% | -3.9% | -5.2% |
| 21 | 5308ns | +9.4% | +7.3% | +3.5% | +7.3% |
| 22 | 5306ns | +10.7% | +12.2% | +3.6% | +5.8% |
| 23 | 5310ns | +9.4% | +2.7% | +3.6% | +5.3% |
| 24 | 5310ns | +9.4% | +0.3% | +3.5% | +6.4% |
| 25 | 5305ns | +9.5% | +0.3% | +3.5% | +4.4% |
| 26 | 5308ns | +9.3% | +0.1% | +3.5% | +3.6% |
| 27 | 5309ns | +9.3% | +0.0% | +1.1% | +3.5% |
| 28 | 5311ns | +8.1% | +0.5% | -0.0% | +3.4% |
| 29 | 5308ns | +7.3% | +0.1% | -0.0% | +1.9% |
| 30 | 5310ns | +8.4% | +1.3% | -0.0% | +1.8% |
| 31 | 5496ns | +10.8% | -3.3% | +0.0% | +5.0% |
| 32 | 5750ns | +3.5% | -7.6% | -4.5% | -0.2% |
| 33 | 5655ns | +6.3% | -6.0% | -2.8% | +1.8% |
| 34 | 5724ns | +6.3% | -7.2% | -5.5% | -1.6% |
| 35 | 5494ns | +10.7% | -3.2% | -3.3% | +3.8% |
| 36 | 5496ns | +10.2% | -3.4% | -3.3% | +0.3% |
| 37 | 5620ns | +5.3% | -5.0% | -5.5% | -2.1% |
| 38 | 5507ns | +8.5% | -3.4% | -3.7% | +2.7% |
| 39 | 5538ns | +6.8% | -4.1% | -3.9% | +2.5% |
| 40 | 5654ns | +4.5% | -5.5% | -6.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.606 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.769 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.519 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.692 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.685 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 18/40, lost 14/40
- **wide-rung-wordround**: won 19/40, lost 15/40
- **wide-rung-wordround-alias**: won 11/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.2ns | 5440.0ns | 0.0% |  |
| wide-rung-ragged | 2.0ns | 5827.7ns | 0.0% |  |
| wide-rung-ragged-overread | 2.3ns | 5387.9ns | 0.0% |  |
| wide-rung-wordround | 2.2ns | 5419.5ns | 0.0% |  |
| wide-rung-wordround-alias | 2.5ns | 5494.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5307.0-5649.9 ns)
   5307.0 |########################################
   5324.1 |
   5341.3 |
   5358.4 |
   5375.5 |##
   5392.7 |
   5409.8 |##
   5427.0 |
   5444.1 |
   5461.3 |
   5478.4 |#############
   5495.6 |#############
   5512.7 |##
   5529.8 |#####
   5547.0 |
   5564.1 |
   5581.3 |
   5598.4 |##
   5615.6 |##
   5632.7 |
  (3 below, 5 above range)

wide-rung-ragged (n=40, range 5698.1-6020.6 ns)
   5698.1 |########################################
   5714.2 |
   5730.3 |####
   5746.5 |########
   5762.6 |####
   5778.7 |
   5794.8 |########################
   5811.0 |
   5827.1 |####
   5843.2 |####
   5859.4 |####
   5875.5 |
   5891.6 |################
   5907.7 |############
   5923.9 |
   5940.0 |####
   5956.1 |
   5972.2 |####
   5988.4 |
   6004.5 |####
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 5309.8-5577.1 ns)
   5309.8 |########################################
   5323.2 |#####
   5336.6 |#######
   5349.9 |##
   5363.3 |##
   5376.6 |#####
   5390.0 |
   5403.4 |
   5416.7 |
   5430.1 |##
   5443.5 |##
   5456.8 |#####
   5470.2 |
   5483.5 |############
   5496.9 |##
   5510.3 |
   5523.6 |
   5537.0 |
   5550.4 |
   5563.7 |
  (3 below, 2 above range)

wide-rung-wordround (n=40, range 5309.1-5507.7 ns)
   5309.1 |######################
   5319.0 |##
   5328.9 |
   5338.9 |
   5348.8 |
   5358.7 |##
   5368.7 |
   5378.6 |##
   5388.5 |
   5398.5 |
   5408.4 |#########################
   5418.3 |
   5428.3 |##
   5438.2 |
   5448.1 |##
   5458.1 |
   5468.0 |
   5477.9 |
   5487.8 |########################################
   5497.8 |##
  (2 below, 1 above range)

wide-rung-wordround-alias (n=40, range 5309.3-5705.6 ns)
   5309.3 |###################################
   5329.1 |
   5348.9 |
   5368.7 |
   5388.5 |########
   5408.4 |
   5428.2 |
   5448.0 |
   5467.8 |
   5487.6 |########################################
   5507.4 |########
   5527.3 |########
   5547.1 |
   5566.9 |####
   5586.7 |####
   5606.5 |####
   5626.3 |####
   5646.2 |#############
   5666.0 |####
   5685.8 |########
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.69 (measurement drift or warm-up artifact)
