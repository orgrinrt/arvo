# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-align16) are a dead heat (<1%)

wide-rung-wordround-alias (5.71 us) and wide-rung-align16 (5.72 us) differ by 0.08%, inside the noise, even though the wider field spreads 3.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.66)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.66, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.3% of the fastest

All 5 variants sit between 5.71 us and 5.90 us - a 3.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (43 ns, 0.74%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 43 ns (0.74%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 5714.6 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.03x (fastest 5714.6 ns, slowest 5902.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5842ns | 5782ns | 5766ns | 5805ns | 6030ns | base |
| wide-rung-ragged | 6063ns | 5967ns | 5914ns | 6003ns | 6391ns | +3.78% |
| wide-rung-ragged-overread | 5956ns | 5880ns | 5777ns | 5898ns | 6310ns | +1.95% |
| wide-rung-wordround | 5858ns | 5858ns | 5766ns | 5840ns | 6000ns | +0.27% |
| wide-rung-wordround-alias | 5825ns | 5785ns | 5768ns | 5806ns | 5937ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5776ns | 5703ns | 5958ns | base | 1.418 |
| wide-rung-ragged | 5993ns | 5852ns | 6311ns | +3.77% | 1.367 |
| wide-rung-ragged-overread | 5886ns | 5712ns | 6230ns | +1.92% | 1.392 |
| wide-rung-wordround | 5790ns | 5703ns | 5924ns | +0.24% | 1.415 |
| wide-rung-wordround-alias | 5758ns | 5702ns | 5871ns | -0.31% | 1.423 |

## Performance model

- Peak throughput: **1.437 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.432 | 99.7% |
| wide-rung-ragged | 1.388 | 96.6% |
| wide-rung-ragged-overread | 1.409 | 98.1% |
| wide-rung-wordround | 1.414 | 98.4% |
| wide-rung-wordround-alias | 1.434 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5842ns | 5842ns | base |
| wide-rung-ragged | 6063ns | 6063ns | +3.78% |
| wide-rung-ragged-overread | 5956ns | 5956ns | +1.95% |
| wide-rung-wordround | 5858ns | 5858ns | +0.27% |
| wide-rung-wordround-alias | 5825ns | 5825ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5719ns | base | --- | [5707, 5776] | --- | --- | --- | --- |
| wide-rung-ragged | 5902ns | +156.2ns (+2.7%) | [+148, +170]ns | [5868, 5963] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 5813ns | +37.5ns (+0.7%) | [+15, +127]ns | [5728, 5898] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 5793ns | no significant difference | [-2, +79]ns | [5734, 5797] | no | 0.2661 | 0.1996 | 1 |
| wide-rung-wordround-alias | 5715ns | no significant difference | [-8, +10]ns | [5709, 5775] | no | 0.7493 | 0.7493 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5707ns | +2.9% | +0.2% | +1.6% | -0.0% |
| 2 | 5705ns | +2.8% | +0.3% | +1.5% | +0.0% |
| 3 | 5704ns | +2.9% | +0.2% | +1.6% | -0.0% |
| 4 | 5723ns | +2.9% | -0.1% | +1.2% | -0.3% |
| 5 | 5705ns | +3.0% | +0.5% | +1.6% | -0.1% |
| 6 | 5700ns | +2.9% | +0.3% | +1.7% | +0.2% |
| 7 | 5710ns | +3.0% | +0.1% | +1.6% | +0.0% |
| 8 | 5704ns | +5.5% | +0.9% | +1.6% | +0.1% |
| 9 | 5703ns | +8.9% | +0.4% | +1.5% | +0.6% |
| 10 | 5705ns | +9.0% | +0.2% | +1.5% | +1.1% |
| 11 | 5704ns | +2.6% | +1.4% | +4.5% | +0.7% |
| 12 | 5801ns | +1.1% | -0.6% | +2.1% | -1.6% |
| 13 | 5718ns | +2.3% | -0.1% | +1.2% | -0.3% |
| 14 | 5857ns | +0.2% | -0.2% | -2.6% | -2.7% |
| 15 | 5705ns | +2.6% | +0.1% | -0.1% | +0.2% |
| 16 | 5705ns | +2.6% | +0.3% | +0.1% | +0.2% |
| 17 | 5777ns | +1.3% | -1.2% | -1.3% | -1.3% |
| 18 | 5707ns | +2.6% | +0.4% | -0.0% | +0.0% |
| 19 | 5720ns | +2.3% | +1.6% | -0.3% | -0.1% |
| 20 | 5703ns | +2.5% | +2.4% | +0.1% | +0.2% |
| 21 | 5777ns | +1.3% | +16.2% | +1.7% | +0.3% |
| 22 | 5726ns | +2.2% | +4.9% | +0.5% | -0.5% |
| 23 | 5710ns | +3.5% | +8.8% | +0.6% | +2.3% |
| 24 | 5771ns | +2.5% | +2.1% | -1.3% | -1.1% |
| 25 | 5707ns | +2.9% | +6.6% | +0.0% | +0.6% |
| 26 | 5774ns | +5.5% | +3.4% | -1.0% | +0.2% |
| 27 | 5701ns | +8.1% | +3.9% | +0.3% | +0.1% |
| 28 | 5707ns | +15.6% | +0.1% | -0.0% | +0.0% |
| 29 | 5703ns | +18.6% | +0.1% | +0.3% | +0.1% |
| 30 | 5740ns | +4.0% | +2.9% | -0.6% | +1.0% |
| 31 | 5870ns | +1.5% | +3.0% | +2.6% | -1.1% |
| 32 | 5874ns | +5.0% | -1.0% | +2.8% | -1.1% |
| 33 | 6020ns | +2.5% | +2.8% | -1.9% | -3.4% |
| 34 | 6131ns | +0.8% | +2.3% | -5.4% | -5.0% |
| 35 | 5910ns | +4.5% | +4.2% | -1.4% | +1.8% |
| 36 | 5814ns | +5.2% | +4.5% | +0.1% | +3.7% |
| 37 | 5810ns | +2.6% | +1.3% | +0.5% | +0.3% |
| 38 | 5812ns | +2.6% | +0.1% | -0.0% | -0.1% |
| 39 | 5993ns | -0.6% | +2.5% | -3.2% | -3.1% |
| 40 | 6011ns | -0.9% | +0.8% | -3.2% | -3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.663 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.632 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.442 | moderate+ |
| wide-rung-wordround | 0.593 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.660 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 2/40, lost 38/40
- **wide-rung-ragged-overread**: won 4/40, lost 32/40
- **wide-rung-wordround**: won 11/40, lost 21/40
- **wide-rung-wordround-alias**: won 15/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 5775.6ns | 0.0% |  |
| wide-rung-ragged | 2.6ns | 5993.2ns | 0.0% |  |
| wide-rung-ragged-overread | 2.3ns | 5886.3ns | 0.0% |  |
| wide-rung-wordround | 2.1ns | 5789.5ns | 0.0% |  |
| wide-rung-wordround-alias | 2.5ns | 5757.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5702.6-5958.1 ns)
   5702.6 |########################################
   5715.4 |#########
   5728.2 |##
   5741.0 |
   5753.7 |
   5766.5 |#########
   5779.3 |
   5792.0 |##
   5804.8 |#######
   5817.6 |
   5830.4 |
   5843.1 |
   5855.9 |##
   5868.7 |####
   5881.4 |
   5894.2 |
   5907.0 |##
   5919.8 |
   5932.5 |
   5945.3 |
  (2 below, 4 above range)

wide-rung-ragged (n=40, range 5851.9-6311.2 ns)
   5851.9 |########################################
   5874.8 |########
   5897.8 |#####
   5920.8 |
   5943.7 |#############
   5966.7 |##
   5989.7 |
   6012.6 |##
   6035.6 |
   6058.6 |
   6081.6 |##
   6104.5 |##
   6127.5 |
   6150.5 |########
   6173.4 |#####
   6196.4 |##
   6219.4 |##
   6242.3 |
   6265.3 |
   6288.3 |
  (2 below, 2 above range)

wide-rung-ragged-overread (n=40, range 5712.4-6230.5 ns)
   5712.4 |########################################
   5738.4 |###
   5764.3 |######
   5790.2 |###
   5816.1 |##########
   5842.0 |###
   5867.9 |######
   5893.8 |###
   5919.7 |###
   5945.6 |###
   5971.5 |
   5997.4 |###
   6023.3 |###
   6049.2 |######
   6075.1 |###
   6101.0 |
   6126.9 |###
   6152.8 |###
   6178.7 |###
   6204.6 |###
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 5703.2-5923.9 ns)
   5703.2 |###################################
   5714.2 |#############
   5725.2 |
   5736.3 |####
   5747.3 |####
   5758.4 |
   5769.4 |
   5780.4 |########
   5791.5 |########################################
   5802.5 |#############
   5813.5 |########
   5824.6 |####
   5835.6 |####
   5846.6 |
   5857.7 |
   5868.7 |####
   5879.8 |
   5890.8 |
   5901.8 |####
   5912.9 |
  (3 below, 4 above range)

wide-rung-wordround-alias (n=40, range 5702.4-5870.9 ns)
   5702.4 |########################################
   5710.8 |####################
   5719.3 |
   5727.7 |
   5736.1 |##########
   5744.5 |
   5753.0 |
   5761.4 |###
   5769.8 |
   5778.2 |###
   5786.7 |###
   5795.1 |###
   5803.5 |#############
   5811.9 |######
   5820.4 |######
   5828.8 |
   5837.2 |###
   5845.7 |
   5854.1 |
   5862.5 |
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.66 (measurement drift or warm-up artifact)
