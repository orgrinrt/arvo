# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-headroom, warm-container-kernel) are a dead heat (<1%)

warm-container-headroom (5.22 us) and warm-container-kernel (5.23 us) differ by 0.14%, inside the noise, even though the wider field spreads 4.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.91)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.91, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (5.22 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 4.3% of the fastest

All 6 variants sit between 5.22 us and 5.45 us - a 4.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### warm-container-kernel's edge over baseline is significant but tiny (-1 ns, 0.02%)

warm-container-kernel differs from baseline warm-container-headroom by -1 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 5220.6 ns median
- 4 variants significantly slower than baseline
- Spread: 1.04x (fastest 5220.6 ns, slowest 5446.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5330ns | 5312ns | 5306ns | 5316ns | 5396ns | base |
| warm-container-kernel | 5475ns | 5294ns | 5276ns | 5377ns | 5969ns | +2.72% |
| warm-container-lanes-deferred | 5583ns | 5418ns | 5283ns | 5546ns | 5992ns | +4.73% |
| warm-container-minimum | 5560ns | 5506ns | 5277ns | 5512ns | 5985ns | +4.30% |
| warm-container-native | 5359ns | 5300ns | 5276ns | 5323ns | 5553ns | +0.55% |
| warm-container-plusone | 5470ns | 5395ns | 5312ns | 5400ns | 5838ns | +2.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5239ns | 5217ns | 5303ns | base | 3.128 |
| warm-container-kernel | 5409ns | 5213ns | 5898ns | +3.25% | 3.029 |
| warm-container-lanes-deferred | 5513ns | 5220ns | 5917ns | +5.24% | 2.972 |
| warm-container-minimum | 5499ns | 5220ns | 5921ns | +4.96% | 2.980 |
| warm-container-native | 5297ns | 5219ns | 5487ns | +1.11% | 3.093 |
| warm-container-plusone | 5371ns | 5219ns | 5735ns | +2.53% | 3.050 |

## Performance model

- Peak throughput: **3.143 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.138 | 99.9% |
| warm-container-kernel | 3.134 | 99.7% |
| warm-container-lanes-deferred | 3.065 | 97.5% |
| warm-container-minimum | 3.008 | 95.7% |
| warm-container-native | 3.130 | 99.6% |
| warm-container-plusone | 3.090 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5330ns | 5330ns | base |
| warm-container-kernel | 5475ns | 5475ns | +2.72% |
| warm-container-lanes-deferred | 5583ns | 5583ns | +4.73% |
| warm-container-minimum | 5560ns | 5560ns | +4.30% |
| warm-container-native | 5359ns | 5359ns | +0.55% |
| warm-container-plusone | 5470ns | 5470ns | +2.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5221ns | base | --- | [5219, 5231] | --- | --- | --- | --- |
| warm-container-kernel | 5228ns | no significant difference | [-4, +252]ns | [5218, 5485] | no | 0.6358 | 0.6358 | 0 |
| warm-container-lanes-deferred | 5345ns | +118.0ns (+2.3%) | [+64, +411]ns | [5297, 5676] | YES | 0.0002 | 0.0000 | 0 |
| warm-container-minimum | 5446ns | +170.0ns (+3.3%) | [+88, +283]ns | [5317, 5503] | YES | 0.0011 | 0.0007 | 0 |
| warm-container-native | 5234ns | +4.8ns (+0.1%) | [+2, +74]ns | [5221, 5294] | YES | 0.0080 | 0.0064 | 0 |
| warm-container-plusone | 5302ns | +65.0ns (+1.2%) | [+22, +105]ns | [5264, 5324] | YES | 0.0005 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5298ns | -1.2% | -1.5% | -1.4% | +3.9% | -1.5% |
| 2 | 5318ns | -1.9% | -1.9% | -1.9% | +3.7% | -0.9% |
| 3 | 5227ns | -0.1% | -0.1% | -0.0% | +5.2% | +1.2% |
| 4 | 5235ns | -0.3% | -0.3% | -0.3% | +4.6% | +1.3% |
| 5 | 5217ns | +0.3% | +0.0% | +0.1% | +1.9% | +2.0% |
| 6 | 5222ns | -0.0% | -0.1% | -0.0% | +3.5% | +1.9% |
| 7 | 5219ns | +0.0% | -0.0% | +2.2% | +3.7% | +2.0% |
| 8 | 5220ns | +0.0% | +0.1% | +1.6% | +1.6% | +2.7% |
| 9 | 5219ns | -0.0% | +0.1% | +1.7% | +2.9% | +1.8% |
| 10 | 5227ns | -0.3% | +0.4% | +1.9% | +1.4% | +0.8% |
| 11 | 5342ns | -2.3% | -2.3% | +1.1% | -2.0% | +10.8% |
| 12 | 5220ns | +0.9% | +3.8% | +5.5% | +0.0% | +13.4% |
| 13 | 5219ns | -0.1% | +13.4% | +5.4% | +0.1% | +13.4% |
| 14 | 5218ns | -0.1% | +13.5% | +5.3% | +0.1% | +13.4% |
| 15 | 5220ns | +6.1% | +13.3% | +5.8% | +0.4% | +10.7% |
| 16 | 5294ns | +11.7% | +11.7% | +3.9% | -1.0% | -1.4% |
| 17 | 5217ns | +13.3% | +13.3% | +5.4% | +0.1% | +0.0% |
| 18 | 5219ns | +13.3% | +13.3% | +5.2% | +0.5% | +0.8% |
| 19 | 5213ns | +0.0% | +13.4% | +5.3% | +1.4% | +0.1% |
| 20 | 5219ns | -0.1% | +13.3% | +2.5% | +0.3% | +3.1% |
| 21 | 5218ns | -0.1% | +13.3% | +1.7% | +0.3% | +5.7% |
| 22 | 5218ns | -0.1% | +13.5% | +13.5% | +0.1% | +0.5% |
| 23 | 5215ns | +0.1% | +13.4% | +13.5% | +0.1% | +1.1% |
| 24 | 5222ns | +13.3% | +13.3% | +13.4% | -0.0% | +0.1% |
| 25 | 5219ns | +13.4% | +13.3% | +13.5% | +0.1% | +0.4% |
| 26 | 5278ns | +11.0% | +12.1% | +12.2% | -0.9% | -0.9% |
| 27 | 5288ns | -0.7% | +2.9% | +12.0% | -1.2% | -1.3% |
| 28 | 5220ns | -0.2% | +1.3% | +13.5% | +0.0% | +0.1% |
| 29 | 5267ns | -1.0% | +0.6% | +12.4% | -0.9% | -0.9% |
| 30 | 5219ns | -0.1% | +1.5% | +13.4% | +0.0% | +0.1% |
| 31 | 5338ns | +10.8% | +0.1% | -2.2% | -2.2% | -0.3% |
| 32 | 5221ns | +11.8% | +2.3% | +2.5% | -0.0% | +3.1% |
| 33 | 5218ns | +6.5% | +3.5% | +2.6% | -0.0% | +5.3% |
| 34 | 5233ns | +4.8% | +2.0% | +5.4% | -0.3% | +2.9% |
| 35 | 5218ns | +5.1% | +1.6% | +10.1% | +0.1% | +3.9% |
| 36 | 5230ns | +4.9% | +2.2% | +12.7% | -0.3% | +2.3% |
| 37 | 5232ns | +4.8% | +3.3% | +0.4% | +2.8% | +1.3% |
| 38 | 5232ns | +6.9% | +2.0% | -0.3% | +3.2% | +1.2% |
| 39 | 5235ns | +0.0% | +1.2% | -0.3% | +6.9% | +1.6% |
| 40 | 5242ns | -0.5% | +3.2% | -0.4% | +4.8% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.023 | ok |
| warm-container-kernel | 0.588 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.908 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.722 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.763 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.718 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 11/40, lost 17/40
- **warm-container-lanes-deferred**: won 4/40, lost 31/40
- **warm-container-minimum**: won 7/40, lost 30/40
- **warm-container-native**: won 8/40, lost 20/40
- **warm-container-plusone**: won 8/40, lost 29/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 5238.6ns | 0.1% |  |
| warm-container-kernel | 2.6ns | 5409.1ns | 0.0% |  |
| warm-container-lanes-deferred | 3.1ns | 5513.3ns | 0.1% |  |
| warm-container-minimum | 3.2ns | 5498.5ns | 0.1% |  |
| warm-container-native | 2.8ns | 5297.0ns | 0.1% |  |
| warm-container-plusone | 3.0ns | 5371.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5216.7-5302.8 ns)
   5216.7 |########################################
   5221.0 |####
   5225.3 |####
   5229.6 |########
   5233.9 |####
   5238.2 |##
   5242.5 |
   5246.8 |
   5251.1 |
   5255.4 |
   5259.7 |
   5264.0 |##
   5268.3 |
   5272.6 |
   5276.9 |##
   5281.2 |
   5285.5 |##
   5289.8 |##
   5294.2 |##
   5298.5 |
  (2 below, 3 above range)

warm-container-kernel (n=40, range 5213.2-5897.8 ns)
   5213.2 |########################################
   5247.4 |####
   5281.6 |
   5315.9 |
   5350.1 |
   5384.3 |
   5418.5 |
   5452.8 |########
   5487.0 |
   5521.2 |##
   5555.5 |##
   5589.7 |##
   5623.9 |
   5658.1 |
   5692.4 |
   5726.6 |
   5760.8 |
   5795.1 |
   5829.3 |####
   5863.5 |
  (3 below, 6 above range)

warm-container-lanes-deferred (n=40, range 5219.5-5917.4 ns)
   5219.5 |####################
   5254.4 |###
   5289.3 |#############
   5324.2 |################
   5359.1 |
   5394.0 |#############
   5428.9 |###
   5463.8 |
   5498.7 |
   5533.6 |
   5568.5 |
   5603.4 |
   5638.3 |
   5673.2 |
   5708.1 |
   5743.0 |
   5777.9 |
   5812.7 |
   5847.6 |
   5882.5 |########################################
  (5 below, 2 above range)

warm-container-minimum (n=40, range 5219.7-5921.1 ns)
   5219.7 |########################################
   5254.8 |
   5289.9 |#################
   5324.9 |############################
   5360.0 |
   5395.1 |#####
   5430.1 |
   5465.2 |############################
   5500.3 |######################
   5535.3 |
   5570.4 |
   5605.5 |
   5640.5 |
   5675.6 |
   5710.7 |#####
   5745.7 |
   5780.8 |
   5815.9 |
   5850.9 |
   5886.0 |########################################
  (4 below, 3 above range)

warm-container-native (n=40, range 5219.1-5486.8 ns)
   5219.1 |########################################
   5232.5 |##############
   5245.9 |
   5259.3 |
   5272.7 |
   5286.0 |##
   5299.4 |####
   5312.8 |##
   5326.2 |
   5339.6 |
   5352.9 |
   5366.3 |##
   5379.7 |##
   5393.1 |####
   5406.5 |##
   5419.9 |
   5433.2 |
   5446.6 |
   5460.0 |##
   5473.4 |
  (2 below, 5 above range)

warm-container-plusone (n=40, range 5219.4-5735.4 ns)
   5219.4 |########################################
   5245.2 |#############
   5271.0 |#############
   5296.8 |##########################
   5322.6 |########
   5348.4 |########
   5374.2 |#############
   5400.0 |####
   5425.8 |
   5451.6 |
   5477.4 |####
   5503.2 |####
   5529.0 |
   5554.8 |
   5580.6 |
   5606.4 |
   5632.2 |
   5658.0 |
   5683.8 |
   5709.6 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.91 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.72 (measurement drift or warm-up artifact)
