# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged-overread, wide-rung-wordround) are a dead heat (<1%)

wide-rung-ragged-overread (5.45 us) and wide-rung-wordround (5.46 us) differ by 0.24%, inside the noise, even though the wider field spreads 8.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.65)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.65, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-wordround's edge over baseline is significant but tiny (-48 ns, 0.87%)

wide-rung-wordround differs from baseline wide-rung-align16 by -48 ns (0.87%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 5447.1 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 5447.1 ns, slowest 5900.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5553ns | 5526ns | 5473ns | 5525ns | 5718ns | base |
| wide-rung-ragged | 5961ns | 5966ns | 5869ns | 5948ns | 6093ns | +7.35% |
| wide-rung-ragged-overread | 5536ns | 5520ns | 5475ns | 5524ns | 5630ns | -0.32% |
| wide-rung-wordround | 5544ns | 5526ns | 5475ns | 5531ns | 5651ns | -0.17% |
| wide-rung-wordround-alias | 5542ns | 5527ns | 5474ns | 5534ns | 5635ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5488ns | 5409ns | 5647ns | base | 1.493 |
| wide-rung-ragged | 5893ns | 5805ns | 6015ns | +7.39% | 1.390 |
| wide-rung-ragged-overread | 5469ns | 5411ns | 5559ns | -0.35% | 1.498 |
| wide-rung-wordround | 5476ns | 5410ns | 5579ns | -0.22% | 1.496 |
| wide-rung-wordround-alias | 5474ns | 5411ns | 5565ns | -0.25% | 1.497 |

## Performance model

- Peak throughput: **1.515 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.500 | 99.0% |
| wide-rung-ragged | 1.388 | 91.7% |
| wide-rung-ragged-overread | 1.504 | 99.3% |
| wide-rung-wordround | 1.500 | 99.1% |
| wide-rung-wordround-alias | 1.500 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5553ns | 5553ns | base |
| wide-rung-ragged | 5961ns | 5961ns | +7.35% |
| wide-rung-ragged-overread | 5536ns | 5536ns | -0.32% |
| wide-rung-wordround | 5544ns | 5544ns | -0.17% |
| wide-rung-wordround-alias | 5542ns | 5542ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5461ns | base | --- | [5417, 5506] | --- | --- | --- | --- |
| wide-rung-ragged | 5901ns | +397.3ns (+7.3%) | [+388, +405]ns | [5849, 5910] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 5447ns | no significant difference | [-7, +6]ns | [5418, 5504] | no | 0.6965 | 0.5224 | 1 |
| wide-rung-wordround | 5460ns | no significant difference | [-6, +9]ns | [5421, 5499] | no | 0.7493 | 0.7493 | 1 |
| wide-rung-wordround-alias | 5461ns | no significant difference | [-1, +8]ns | [5427, 5510] | no | 0.3077 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5408ns | +13.0% | +0.1% | +2.6% | +0.0% |
| 2 | 5413ns | +12.9% | +0.5% | +5.1% | +1.3% |
| 3 | 5476ns | +11.2% | -0.9% | +0.9% | -0.9% |
| 4 | 5498ns | +7.8% | -1.5% | -0.0% | -0.8% |
| 5 | 5841ns | +1.4% | -6.3% | -6.0% | -7.3% |
| 6 | 5597ns | +5.8% | -3.2% | -1.3% | -2.3% |
| 7 | 5980ns | -0.9% | -9.5% | -9.0% | -7.8% |
| 8 | 5438ns | +10.4% | +0.3% | -0.5% | +2.2% |
| 9 | 5410ns | +9.4% | +0.1% | +0.3% | +2.8% |
| 10 | 5537ns | +7.3% | -2.1% | -2.3% | -0.8% |
| 11 | 5511ns | +7.3% | -0.1% | +0.2% | +0.1% |
| 12 | 5507ns | +7.3% | -0.0% | +0.2% | +0.1% |
| 13 | 5518ns | +7.0% | -0.1% | +0.1% | -0.1% |
| 14 | 5511ns | +7.1% | -0.2% | +0.1% | -0.0% |
| 15 | 5505ns | +7.2% | +0.1% | +0.3% | +0.2% |
| 16 | 5506ns | +7.2% | +0.1% | -0.1% | +0.3% |
| 17 | 5513ns | +6.9% | +0.6% | -0.2% | +0.3% |
| 18 | 5510ns | +7.1% | +0.0% | -0.3% | +0.0% |
| 19 | 5506ns | +7.3% | +0.0% | +0.0% | +1.4% |
| 20 | 5505ns | +7.4% | +0.2% | -0.1% | +0.1% |
| 21 | 5411ns | +9.0% | +6.2% | +1.4% | +0.4% |
| 22 | 5414ns | +11.1% | +2.9% | -0.1% | +0.0% |
| 23 | 5472ns | +6.1% | +1.1% | -1.1% | -1.1% |
| 24 | 5412ns | +8.0% | +1.7% | +0.7% | +1.1% |
| 25 | 5442ns | +6.7% | +1.2% | +3.5% | +0.9% |
| 26 | 5429ns | +6.9% | -0.2% | +4.4% | -0.3% |
| 27 | 5416ns | +7.2% | +1.4% | +0.4% | +0.0% |
| 28 | 5410ns | +7.9% | +0.1% | +0.0% | +0.5% |
| 29 | 5537ns | +5.7% | -2.3% | -2.2% | -1.6% |
| 30 | 5654ns | +2.7% | -4.2% | -4.3% | -4.3% |
| 31 | 5408ns | +7.4% | +0.1% | +0.0% | +4.9% |
| 32 | 5410ns | +9.5% | +2.2% | +0.1% | +0.1% |
| 33 | 5418ns | +7.2% | -0.2% | +1.0% | +0.1% |
| 34 | 5419ns | +7.1% | -0.0% | -0.1% | +2.7% |
| 35 | 5408ns | +8.2% | +0.0% | +0.2% | +0.0% |
| 36 | 5408ns | +8.5% | +1.2% | +0.5% | +0.0% |
| 37 | 5450ns | +6.6% | -0.6% | -0.5% | -0.0% |
| 38 | 5410ns | +7.3% | +0.0% | -0.0% | -0.0% |
| 39 | 5408ns | +7.4% | +0.1% | +0.1% | +0.0% |
| 40 | 5476ns | +6.6% | -1.1% | -1.1% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.266 | moderate+ |
| wide-rung-ragged | 0.645 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.519 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.541 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.201 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 1/40, lost 39/40
- **wide-rung-ragged-overread**: won 14/40, lost 14/40
- **wide-rung-wordround**: won 15/40, lost 16/40
- **wide-rung-wordround-alias**: won 12/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 5487.6ns | 0.0% |  |
| wide-rung-ragged | 1.9ns | 5893.0ns | 0.0% |  |
| wide-rung-ragged-overread | 2.3ns | 5468.5ns | 0.0% |  |
| wide-rung-wordround | 1.9ns | 5475.8ns | 0.0% |  |
| wide-rung-wordround-alias | 1.9ns | 5473.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5408.8-5647.2 ns)
   5408.8 |########################################
   5420.8 |###
   5432.7 |#######
   5444.6 |###
   5456.5 |
   5468.4 |##########
   5480.4 |
   5492.3 |###
   5504.2 |################################
   5516.1 |###
   5528.0 |#######
   5540.0 |
   5551.9 |
   5563.8 |
   5575.7 |
   5587.6 |###
   5599.6 |
   5611.5 |
   5623.4 |
   5635.3 |
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 5805.3-6015.2 ns)
   5805.3 |########################################
   5815.8 |
   5826.3 |
   5836.8 |###############
   5847.3 |##########
   5857.8 |#####
   5868.3 |
   5878.8 |
   5889.3 |#####
   5899.8 |###################################
   5910.3 |####################
   5920.8 |#########################
   5931.3 |#####
   5941.8 |
   5952.3 |
   5962.8 |
   5973.3 |
   5983.8 |
   5994.2 |#####
   6004.7 |
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 5411.3-5558.7 ns)
   5411.3 |########################################
   5418.7 |######
   5426.0 |###
   5433.4 |###
   5440.8 |
   5448.1 |###
   5455.5 |
   5462.9 |
   5470.2 |######
   5477.6 |
   5485.0 |
   5492.4 |###
   5499.7 |#############
   5507.1 |####################
   5514.5 |###
   5521.8 |
   5529.2 |######
   5536.6 |
   5543.9 |###
   5551.3 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 5410.3-5579.1 ns)
   5410.3 |########################################
   5418.8 |#######
   5427.2 |###
   5435.7 |###
   5444.1 |#######
   5452.5 |
   5461.0 |
   5469.4 |###
   5477.9 |
   5486.3 |#######
   5494.7 |##################
   5503.2 |###
   5511.6 |#######
   5520.0 |##################
   5528.5 |
   5536.9 |
   5545.4 |###
   5553.8 |
   5562.2 |
   5570.7 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 5410.7-5564.8 ns)
   5410.7 |########################################
   5418.4 |#####
   5426.1 |#####
   5433.8 |##########
   5441.5 |##########
   5449.2 |#####
   5456.9 |
   5464.6 |#####
   5472.3 |#####
   5480.0 |#####
   5487.7 |##########
   5495.4 |
   5503.1 |###############
   5510.9 |####################
   5518.6 |##########
   5526.3 |#####
   5534.0 |
   5541.7 |
   5549.4 |#####
   5557.1 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.54 (measurement drift or warm-up artifact)
