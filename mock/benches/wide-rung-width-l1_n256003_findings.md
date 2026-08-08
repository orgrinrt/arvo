# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-ragged (5.15 us) and wide-rung-wordround-alias (5.16 us) differ by 0.26%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.71)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.71, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.4% of the fastest

All 5 variants sit between 5.15 us and 5.32 us - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (-7 ns, 0.13%)

wide-rung-ragged differs from baseline wide-rung-align16 by -7 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 5151.0 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 5151.0 ns, slowest 5324.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5340ns | 5390ns | 5208ns | 5350ns | 5443ns | base |
| wide-rung-ragged | 5277ns | 5224ns | 5204ns | 5262ns | 5395ns | -1.19% |
| wide-rung-ragged-overread | 5348ns | 5373ns | 5206ns | 5347ns | 5489ns | +0.14% |
| wide-rung-wordround | 5379ns | 5391ns | 5221ns | 5381ns | 5533ns | +0.73% |
| wide-rung-wordround-alias | 5326ns | 5225ns | 5208ns | 5287ns | 5559ns | -0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5275ns | 5145ns | 5376ns | base | 1.553 |
| wide-rung-ragged | 5211ns | 5141ns | 5330ns | -1.21% | 1.572 |
| wide-rung-ragged-overread | 5280ns | 5144ns | 5413ns | +0.09% | 1.551 |
| wide-rung-wordround | 5312ns | 5158ns | 5463ns | +0.70% | 1.542 |
| wide-rung-wordround-alias | 5261ns | 5145ns | 5493ns | -0.26% | 1.557 |

## Performance model

- Peak throughput: **1.593 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.539 | 96.6% |
| wide-rung-ragged | 1.590 | 99.8% |
| wide-rung-ragged-overread | 1.543 | 96.8% |
| wide-rung-wordround | 1.539 | 96.6% |
| wide-rung-wordround-alias | 1.586 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5340ns | 5340ns | base |
| wide-rung-ragged | 5277ns | 5277ns | -1.19% |
| wide-rung-ragged-overread | 5348ns | 5348ns | +0.14% |
| wide-rung-wordround | 5379ns | 5379ns | +0.73% |
| wide-rung-wordround-alias | 5326ns | 5326ns | -0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5324ns | base | --- | [5246, 5327] | --- | --- | --- | --- |
| wide-rung-ragged | 5151ns | -21.7ns (-0.4%) | [-120, -2]ns | [5145, 5235] | YES | 0.0204 | 0.0051 | 2 |
| wide-rung-ragged-overread | 5310ns | no significant difference | [-7, +34]ns | [5219, 5330] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround | 5324ns | no significant difference | [-5, +6]ns | [5315, 5329] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround-alias | 5165ns | no significant difference | [-165, +24]ns | [5156, 5303] | no | 0.8478 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 5355ns | -3.9% | -1.4% | -0.6% | -3.9% |
| 2 | 5340ns | -3.7% | -1.0% | -0.2% | -3.6% |
| 3 | 5357ns | -4.0% | -1.3% | -0.6% | -3.0% |
| 4 | 5354ns | -4.0% | -4.0% | +2.5% | -3.7% |
| 5 | 5327ns | -3.4% | -2.6% | +4.4% | -0.9% |
| 6 | 5520ns | -6.7% | -4.6% | -2.9% | -6.4% |
| 7 | 5334ns | -3.5% | -3.5% | -1.4% | -3.4% |
| 8 | 5247ns | -1.9% | +1.0% | -1.9% | -1.9% |
| 9 | 5141ns | +1.5% | +5.1% | +0.0% | +0.1% |
| 10 | 5150ns | +0.0% | +3.4% | -0.0% | -0.1% |
| 11 | 5323ns | -3.4% | -3.3% | -0.7% | +0.6% |
| 12 | 5405ns | -1.0% | -4.9% | -1.4% | -0.3% |
| 13 | 5158ns | +1.0% | +1.2% | +3.7% | +3.3% |
| 14 | 5145ns | +0.0% | +0.1% | +5.2% | +4.9% |
| 15 | 5143ns | -0.0% | +0.5% | +3.7% | +4.6% |
| 16 | 5146ns | -0.0% | +0.0% | +5.9% | +4.7% |
| 17 | 5146ns | -0.1% | +0.2% | +2.0% | +6.9% |
| 18 | 5170ns | -0.5% | -0.1% | +6.7% | +3.2% |
| 19 | 5147ns | -0.1% | -0.2% | +0.0% | +2.4% |
| 20 | 5153ns | -0.2% | -0.2% | +1.8% | -0.1% |
| 21 | 5244ns | +1.5% | +2.2% | +0.0% | +5.3% |
| 22 | 5265ns | +1.1% | +2.3% | +0.8% | +2.1% |
| 23 | 5341ns | -0.4% | +0.1% | -0.4% | -3.7% |
| 24 | 5326ns | -2.0% | +2.0% | -0.1% | -2.9% |
| 25 | 5330ns | -3.5% | +1.8% | -0.1% | -3.2% |
| 26 | 5326ns | -3.3% | +1.6% | +2.5% | -3.4% |
| 27 | 5327ns | -3.5% | +0.4% | +0.1% | -3.2% |
| 28 | 5328ns | -3.5% | +2.2% | +0.0% | -3.3% |
| 29 | 5321ns | -3.4% | -1.9% | +0.0% | -3.3% |
| 30 | 5335ns | -2.5% | -2.4% | -0.1% | -3.5% |
| 31 | 5322ns | -1.3% | -0.0% | +2.2% | +3.9% |
| 32 | 5325ns | -1.1% | -0.0% | -0.0% | +4.0% |
| 33 | 5327ns | -0.1% | -0.1% | +0.0% | +3.9% |
| 34 | 5325ns | +0.0% | -0.1% | +1.0% | +3.9% |
| 35 | 5322ns | +0.1% | -0.0% | +0.1% | -1.7% |
| 36 | 5328ns | +0.2% | +0.8% | -0.0% | -3.1% |
| 37 | 5333ns | -0.1% | +0.2% | -1.9% | -3.2% |
| 38 | 5231ns | +1.8% | +2.2% | -1.6% | -1.4% |
| 39 | 5144ns | +3.4% | +5.7% | +0.1% | +0.3% |
| 40 | 5154ns | +3.2% | +3.5% | -0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.643 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.713 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.639 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.396 | moderate+ |
| wide-rung-wordround-alias | 0.599 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 22/40, lost 8/40
- **wide-rung-ragged-overread**: won 15/40, lost 19/40
- **wide-rung-wordround**: won 14/40, lost 15/40
- **wide-rung-wordround-alias**: won 22/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.0ns | 5275.4ns | 0.0% |  |
| wide-rung-ragged | 2.0ns | 5211.5ns | 0.0% |  |
| wide-rung-ragged-overread | 1.8ns | 5280.2ns | 0.0% |  |
| wide-rung-wordround | 1.9ns | 5312.3ns | 0.0% |  |
| wide-rung-wordround-alias | 1.8ns | 5261.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5145.3-5375.8 ns)
   5145.3 |##################
   5156.8 |###
   5168.4 |###
   5179.9 |
   5191.4 |
   5202.9 |
   5214.5 |
   5226.0 |###
   5237.5 |######
   5249.0 |
   5260.6 |###
   5272.1 |
   5283.6 |
   5295.1 |
   5306.7 |
   5318.2 |########################################
   5329.7 |##################
   5341.3 |
   5352.8 |#########
   5364.3 |
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 5141.2-5330.0 ns)
   5141.2 |########################################
   5150.6 |#######
   5160.1 |
   5169.5 |
   5179.0 |
   5188.4 |
   5197.8 |#####
   5207.3 |
   5216.7 |#####
   5226.2 |
   5235.6 |
   5245.0 |##
   5254.5 |
   5263.9 |##
   5273.4 |
   5282.8 |
   5292.2 |
   5301.7 |
   5311.1 |#####
   5320.5 |#################
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 5143.8-5413.4 ns)
   5143.8 |#################################
   5157.3 |#############
   5170.8 |
   5184.3 |######
   5197.7 |######
   5211.2 |#############
   5224.7 |
   5238.2 |
   5251.7 |
   5265.1 |#############
   5278.6 |#############
   5292.1 |######
   5305.6 |
   5319.0 |########################################
   5332.5 |####################
   5346.0 |#############
   5359.5 |#############
   5372.9 |######
   5386.4 |
   5399.9 |#############
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 5157.7-5462.6 ns)
   5157.7 |
   5173.0 |
   5188.2 |
   5203.4 |
   5218.7 |####
   5233.9 |#############
   5249.2 |####
   5264.4 |
   5279.7 |####
   5294.9 |####
   5310.1 |###############################
   5325.4 |########################################
   5340.6 |####
   5355.9 |####
   5371.1 |####
   5386.4 |
   5401.6 |####
   5416.8 |
   5432.1 |####
   5447.3 |########
  (7 below, 3 above range)

wide-rung-wordround-alias (n=40, range 5145.2-5493.0 ns)
   5145.2 |########################################
   5162.6 |########
   5180.0 |##
   5197.4 |
   5214.8 |
   5232.1 |##
   5249.5 |
   5266.9 |#####
   5284.3 |
   5301.7 |
   5319.1 |#####
   5336.5 |
   5353.9 |##
   5371.2 |#####
   5388.6 |########
   5406.0 |
   5423.4 |
   5440.8 |
   5458.2 |
   5475.6 |
  (4 below, 6 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.60 (measurement drift or warm-up artifact)
