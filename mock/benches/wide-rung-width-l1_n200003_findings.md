# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-align16, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-align16 (5.60 us) and wide-rung-wordround-alias (5.60 us) differ by 0.01%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.63)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.63, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (5.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 3.4% of the fastest

All 5 variants sit between 5.60 us and 5.79 us - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (12 ns, 0.22%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 12 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 5603.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.03x (fastest 5603.1 ns, slowest 5794.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5713ns | 5666ns | 5656ns | 5698ns | 5812ns | base |
| wide-rung-ragged | 5865ns | 5860ns | 5804ns | 5856ns | 5952ns | +2.67% |
| wide-rung-ragged-overread | 5700ns | 5675ns | 5663ns | 5682ns | 5792ns | -0.21% |
| wide-rung-wordround | 5785ns | 5783ns | 5656ns | 5792ns | 5890ns | +1.26% |
| wide-rung-wordround-alias | 5701ns | 5670ns | 5659ns | 5681ns | 5805ns | -0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5648ns | 5594ns | 5740ns | base | 1.450 |
| wide-rung-ragged | 5798ns | 5742ns | 5873ns | +2.66% | 1.413 |
| wide-rung-ragged-overread | 5636ns | 5602ns | 5730ns | -0.21% | 1.453 |
| wide-rung-wordround | 5721ns | 5597ns | 5826ns | +1.30% | 1.432 |
| wide-rung-wordround-alias | 5637ns | 5596ns | 5741ns | -0.19% | 1.453 |

## Performance model

- Peak throughput: **1.464 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.462 | 99.8% |
| wide-rung-ragged | 1.414 | 96.5% |
| wide-rung-ragged-overread | 1.461 | 99.8% |
| wide-rung-wordround | 1.432 | 97.8% |
| wide-rung-wordround-alias | 1.462 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5713ns | 5713ns | base |
| wide-rung-ragged | 5865ns | 5865ns | +2.67% |
| wide-rung-ragged-overread | 5700ns | 5700ns | -0.21% |
| wide-rung-wordround | 5785ns | 5785ns | +1.26% |
| wide-rung-wordround-alias | 5701ns | 5701ns | -0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5603ns | base | --- | [5599, 5699] | --- | --- | --- | --- |
| wide-rung-ragged | 5795ns | +149.0ns (+2.7%) | [+144, +168]ns | [5745, 5844] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 5608ns | no significant difference | [-2, +10]ns | [5605, 5620] | no | 0.2051 | 0.1539 | 0 |
| wide-rung-wordround | 5720ns | +65.6ns (+1.2%) | [+2, +158]ns | [5699, 5791] | YES | 0.0006 | 0.0003 | 1 |
| wide-rung-wordround-alias | 5604ns | no significant difference | [-68, +14]ns | [5600, 5626] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5792ns | -0.8% | -3.2% | -1.5% | -3.4% |
| 2 | 5798ns | -1.0% | -3.3% | -1.5% | +2.5% |
| 3 | 5702ns | +1.8% | -1.7% | +0.0% | -1.8% |
| 4 | 5707ns | +0.6% | -1.8% | -0.1% | -2.0% |
| 5 | 5705ns | +0.7% | -1.3% | +0.0% | -1.9% |
| 6 | 5705ns | +1.8% | -1.7% | -1.6% | -1.8% |
| 7 | 5704ns | +0.7% | -1.3% | +3.3% | -1.8% |
| 8 | 5705ns | +3.8% | -1.8% | +1.5% | -1.8% |
| 9 | 5662ns | +1.5% | -1.0% | +3.3% | -1.2% |
| 10 | 5594ns | +3.6% | +0.2% | +4.0% | +1.3% |
| 11 | 5797ns | -1.0% | -3.4% | -1.6% | -1.6% |
| 12 | 5600ns | +2.6% | +4.9% | +3.5% | +2.0% |
| 13 | 5595ns | +2.7% | +0.1% | +3.6% | +3.1% |
| 14 | 5603ns | +2.5% | +0.4% | +3.3% | +1.8% |
| 15 | 5595ns | +2.7% | +0.2% | +3.5% | +1.7% |
| 16 | 5601ns | +2.6% | +1.1% | +3.4% | -0.0% |
| 17 | 5618ns | +2.3% | -0.2% | +3.2% | -0.2% |
| 18 | 5595ns | +2.6% | +0.2% | +3.6% | +0.2% |
| 19 | 5595ns | +2.6% | +0.1% | +0.8% | +0.0% |
| 20 | 5598ns | +2.9% | +0.1% | +0.4% | +0.0% |
| 21 | 5593ns | +2.7% | +2.0% | +0.0% | +0.3% |
| 22 | 5599ns | +3.8% | +2.1% | +0.0% | +0.1% |
| 23 | 5706ns | +2.6% | +0.1% | -2.0% | -1.2% |
| 24 | 5625ns | +4.1% | +1.1% | -0.5% | +1.7% |
| 25 | 5603ns | +4.4% | -0.0% | +1.7% | +0.1% |
| 26 | 5635ns | +3.8% | -0.0% | +0.8% | +0.1% |
| 27 | 5600ns | +4.5% | +0.2% | -0.1% | +1.6% |
| 28 | 5595ns | +4.6% | +0.3% | +0.0% | +1.0% |
| 29 | 5594ns | +3.6% | +0.2% | +0.1% | +0.4% |
| 30 | 5594ns | +2.7% | +0.3% | +0.0% | +0.3% |
| 31 | 5707ns | +2.5% | +0.1% | +0.4% | -2.0% |
| 32 | 5708ns | +2.5% | +0.1% | +1.6% | -1.9% |
| 33 | 5704ns | +2.6% | +0.1% | +1.5% | -1.9% |
| 34 | 5706ns | +3.0% | -1.6% | +1.6% | -1.8% |
| 35 | 5695ns | +2.8% | -1.6% | +2.9% | -1.7% |
| 36 | 5599ns | +4.6% | +0.1% | +3.4% | -0.1% |
| 37 | 5595ns | +4.4% | +0.2% | +3.6% | +0.1% |
| 38 | 5602ns | +5.5% | +0.1% | +3.4% | +1.5% |
| 39 | 5595ns | +4.2% | +0.1% | +2.7% | +0.9% |
| 40 | 5596ns | +2.7% | +1.8% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.490 | moderate+ |
| wide-rung-ragged | 0.465 | moderate+ |
| wide-rung-ragged-overread | 0.133 | ok |
| wide-rung-wordround | 0.631 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.148 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 3/40, lost 37/40
- **wide-rung-ragged-overread**: won 13/40, lost 22/40
- **wide-rung-wordround**: won 7/40, lost 24/40
- **wide-rung-wordround-alias**: won 16/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.1ns | 5648.0ns | 0.0% |  |
| wide-rung-ragged | 1.9ns | 5798.0ns | 0.0% |  |
| wide-rung-ragged-overread | 1.8ns | 5636.3ns | 0.0% |  |
| wide-rung-wordround | 2.0ns | 5721.3ns | 0.0% |  |
| wide-rung-wordround-alias | 2.2ns | 5637.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5594.2-5740.1 ns)
   5594.2 |########################################
   5601.5 |########
   5608.8 |
   5616.1 |##
   5623.4 |##
   5630.7 |##
   5638.0 |
   5645.3 |
   5652.6 |
   5659.9 |##
   5667.1 |
   5674.4 |
   5681.7 |
   5689.0 |##
   5696.3 |##
   5703.6 |############################
   5710.9 |
   5718.2 |
   5725.5 |
   5732.8 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 5741.9-5872.6 ns)
   5741.9 |########################################
   5748.5 |
   5755.0 |
   5761.5 |##
   5768.1 |
   5774.6 |
   5781.1 |
   5787.7 |##
   5794.2 |##
   5800.8 |#####
   5807.3 |##
   5813.8 |
   5820.4 |
   5826.9 |##
   5833.4 |
   5840.0 |##
   5846.5 |##################
   5853.0 |##########
   5859.6 |
   5866.1 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 5601.7-5729.8 ns)
   5601.7 |########################################
   5608.1 |###########
   5614.5 |
   5620.9 |##
   5627.3 |######
   5633.7 |
   5640.1 |
   5646.5 |
   5652.9 |
   5659.4 |##
   5665.8 |
   5672.2 |
   5678.6 |
   5685.0 |##
   5691.4 |##
   5697.8 |
   5704.2 |####
   5710.6 |########
   5717.0 |
   5723.4 |
  (3 below, 1 above range)

wide-rung-wordround (n=40, range 5596.6-5825.7 ns)
   5596.6 |##########################
   5608.1 |####
   5619.5 |####
   5631.0 |####
   5642.4 |
   5653.9 |
   5665.3 |
   5676.8 |####
   5688.2 |####
   5699.7 |##########################
   5711.1 |
   5722.6 |####
   5734.0 |####
   5745.5 |
   5756.9 |
   5768.4 |
   5779.9 |######################
   5791.3 |########################################
   5802.8 |
   5814.2 |####
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 5595.8-5741.3 ns)
   5595.8 |########################################
   5603.1 |##########
   5610.4 |########
   5617.6 |
   5624.9 |
   5632.2 |#####
   5639.5 |##
   5646.7 |##
   5654.0 |
   5661.3 |##
   5668.5 |
   5675.8 |
   5683.1 |#####
   5690.4 |##
   5697.6 |#####
   5704.9 |##
   5712.2 |##
   5719.4 |
   5726.7 |
   5734.0 |
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-wordround**: autocorrelation=0.63 (measurement drift or warm-up artifact)
