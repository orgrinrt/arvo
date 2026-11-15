# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-align16, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-align16 (6.16 us) and wide-rung-wordround-alias (6.17 us) differ by 0.23%, inside the noise, even though the wider field spreads 4.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (245 ns) is smaller than the fastest variant's own run-to-run std-dev (754 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (6.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader wide-rung-align16 vs stability leader wide-rung-ragged (+4% speed for 1.5x steadier)

wide-rung-align16 is fastest (6.16 us, CV 12.2%); wide-rung-ragged gives up 3.9% median for 1.5x lower variance (CV 8.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged-overread is inconsistent: worst-20% is 2.2x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 5.25 us but its worst 20% at 11.49 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.0% of the fastest

All 5 variants sit between 6.16 us and 6.40 us - a 4.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 6157.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.04x (fastest 6157.2 ns, slowest 6402.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 6375ns | 6245ns | 6172ns | 6248ns | 6959ns | base |
| wide-rung-ragged | 6178ns | 6482ns | 5235ns | 6380ns | 6517ns | -3.09% |
| wide-rung-ragged-overread | 7479ns | 6484ns | 5327ns | 6811ns | 11635ns | +17.32% |
| wide-rung-wordround | 7619ns | 6396ns | 6384ns | 6807ns | 11287ns | +19.51% |
| wide-rung-wordround-alias | 6021ns | 6266ns | 5074ns | 6194ns | 6446ns | -5.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 6283ns | 6088ns | 6851ns | base | 1.304 |
| wide-rung-ragged | 6094ns | 5166ns | 6413ns | -3.01% | 1.344 |
| wide-rung-ragged-overread | 7381ns | 5255ns | 11490ns | +17.48% | 1.110 |
| wide-rung-wordround | 7516ns | 6302ns | 11138ns | +19.63% | 1.090 |
| wide-rung-wordround-alias | 5932ns | 4990ns | 6346ns | -5.59% | 1.381 |

## Performance model

- Peak throughput: **1.642 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.330 | 81.0% |
| wide-rung-ragged | 1.280 | 78.0% |
| wide-rung-ragged-overread | 1.279 | 77.9% |
| wide-rung-wordround | 1.298 | 79.0% |
| wide-rung-wordround-alias | 1.327 | 80.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 6375ns | 6375ns | base |
| wide-rung-ragged | 6178ns | 6178ns | -3.09% |
| wide-rung-ragged-overread | 7479ns | 7479ns | +17.32% |
| wide-rung-wordround | 7619ns | 7619ns | +19.51% |
| wide-rung-wordround-alias | 6021ns | 6021ns | -5.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 6157ns | base | --- | [6138, 6184] | --- | --- | --- | --- |
| wide-rung-ragged | 6399ns | +227.4ns (+3.7%) | [+170, +241]ns | [6392, 6403] | YES | 0.0086 | 0.0064 | 0 |
| wide-rung-ragged-overread | 6403ns | +234.8ns (+3.8%) | [+158, +307]ns | [6383, 6413] | YES | 0.0086 | 0.0064 | 0 |
| wide-rung-wordround | 6313ns | +191.5ns (+3.1%) | [+146, +224]ns | [6311, 6316] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 6171ns | no significant difference | [-98, +80]ns | [6137, 6245] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 6096ns | +5.1% | +5.0% | +3.5% | +2.7% |
| 1 | 2 | 6170ns | +3.8% | -14.8% | +2.3% | -19.0% |
| 1 | 3 | 6166ns | +3.8% | +3.8% | +2.4% | +0.2% |
| 1 | 4 | 6142ns | -15.9% | +87.0% | +80.9% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.196 | ok |
| wide-rung-ragged | -0.082 | ok |
| wide-rung-ragged-overread | 0.007 | ok |
| wide-rung-wordround | -0.083 | ok |
| wide-rung-wordround-alias | -0.376 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 1/4, lost 3/4
- **wide-rung-wordround**: won 0/4, lost 4/4
- **wide-rung-wordround-alias**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 25.3ns | 6282.9ns | 0.4% |  |
| wide-rung-ragged | 23.7ns | 6093.7ns | 0.4% |  |
| wide-rung-ragged-overread | 29.5ns | 7381.0ns | 0.4% |  |
| wide-rung-wordround | 30.7ns | 7515.9ns | 0.4% |  |
| wide-rung-wordround-alias | 24.2ns | 5931.9ns | 0.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 6088.1-6851.0 ns)
   6088.1 |############################
   6126.2 |########################################
   6164.4 |########################################
   6202.5 |############
   6240.7 |########
   6278.8 |############
   6317.0 |
   6355.1 |
   6393.3 |
   6431.4 |
   6469.5 |
   6507.7 |
   6545.8 |
   6584.0 |
   6622.1 |
   6660.3 |
   6698.4 |
   6736.6 |
   6774.7 |
   6812.9 |
  (4 below, 1 above range)

wide-rung-ragged (n=40, range 5165.5-6413.3 ns)
   5165.5 |#######
   5227.9 |
   5290.3 |
   5352.7 |
   5415.1 |
   5477.5 |
   5539.9 |
   5602.3 |
   5664.6 |
   5727.0 |
   5789.4 |
   5851.8 |
   5914.2 |
   5976.6 |
   6039.0 |
   6101.4 |
   6163.8 |
   6226.2 |
   6288.6 |
   6350.9 |########################################
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 5254.8-11490.3 ns)
   5254.8 |##############
   5566.5 |
   5878.3 |
   6190.1 |########################################
   6501.9 |
   6813.6 |
   7125.4 |
   7437.2 |
   7749.0 |
   8060.7 |
   8372.5 |
   8684.3 |
   8996.1 |
   9307.8 |
   9619.6 |
   9931.4 |
  10243.2 |
  10554.9 |
  10866.7 |
  11178.5 |################
  (3 below, 2 above range)

wide-rung-wordround (n=40, range 6302.0-11138.5 ns)
   6302.0 |########################################
   6543.8 |
   6785.6 |
   7027.5 |
   7269.3 |
   7511.1 |
   7752.9 |
   7994.8 |
   8236.6 |
   8478.4 |
   8720.2 |
   8962.1 |
   9203.9 |
   9445.7 |
   9687.5 |
   9929.4 |
  10171.2 |
  10413.0 |
  10654.8 |
  10896.7 |##########
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 4989.9-6346.1 ns)
   4989.9 |#################
   5057.7 |####
   5125.5 |####
   5193.3 |
   5261.1 |
   5328.9 |
   5396.8 |
   5464.6 |
   5532.4 |
   5600.2 |
   5668.0 |
   5735.8 |
   5803.6 |
   5871.4 |
   5939.2 |
   6007.0 |
   6074.8 |######################
   6142.6 |########################################
   6210.5 |#################
   6278.3 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: CV=32.7% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=27.7% (high variance, measurements may be unstable)
