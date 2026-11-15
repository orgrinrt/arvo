# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 86% (significant)

warm-container-kernel is -4.61 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 8.1x slower than the field

warm-container-plusone (5.49 us) is 8.1x the fastest (674 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 5.2%)

warm-container-kernel wins on median (674 ns) yet has the highest variance (CV 5.2%), while warm-container-minimum is the steadiest (CV 1.3%, 5.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.87)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (630% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 630% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.1x the fastest

Fastest warm-container-kernel (674 ns) to slowest warm-container-plusone (5.49 us): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum's edge over baseline is significant but tiny (-30 ns, 0.56%)

warm-container-minimum differs from baseline warm-container-headroom by -30 ns (0.56%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 674.2 ns median (-87.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 8.15x (fastest 674.2 ns, slowest 5492.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5430ns | 5410ns | 5310ns | 5416ns | 5594ns | base |
| warm-container-kernel | 748ns | 734ns | 726ns | 738ns | 800ns | -86.22% |
| warm-container-lanes-deferred | 783ns | 794ns | 723ns | 789ns | 823ns | -85.59% |
| warm-container-minimum | 5373ns | 5378ns | 5290ns | 5367ns | 5476ns | -1.05% |
| warm-container-native | 5397ns | 5350ns | 5282ns | 5374ns | 5581ns | -0.62% |
| warm-container-plusone | 5530ns | 5554ns | 5285ns | 5475ns | 5938ns | +1.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5365ns | 5248ns | 5522ns | base | 3.054 |
| warm-container-kernel | 684ns | 665ns | 728ns | -87.24% | 23.939 |
| warm-container-lanes-deferred | 716ns | 662ns | 754ns | -86.65% | 22.871 |
| warm-container-minimum | 5308ns | 5221ns | 5411ns | -1.06% | 3.086 |
| warm-container-native | 5334ns | 5224ns | 5512ns | -0.59% | 3.072 |
| warm-container-plusone | 5465ns | 5225ns | 5868ns | +1.87% | 2.998 |

## Performance model

- Peak throughput: **24.767 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.066 | 12.4% |
| warm-container-kernel | 24.301 | 98.1% |
| warm-container-lanes-deferred | 22.599 | 91.2% |
| warm-container-minimum | 3.083 | 12.4% |
| warm-container-native | 3.098 | 12.5% |
| warm-container-plusone | 2.983 | 12.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5430ns | 5430ns | base |
| warm-container-kernel | 748ns | 748ns | -86.22% |
| warm-container-lanes-deferred | 783ns | 783ns | -85.59% |
| warm-container-minimum | 5373ns | 5373ns | -1.05% |
| warm-container-native | 5397ns | 5397ns | -0.62% |
| warm-container-plusone | 5530ns | 5530ns | +1.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5344ns | base | --- | [5306, 5398] | --- | --- | --- | --- |
| warm-container-kernel | 674ns | -4653.3ns (-87.1%) | [-4706, -4633]ns | [670, 679] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 725ns | -4644.0ns (-86.9%) | [-4681, -4609]ns | [700, 749] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5315ns | -61.7ns (-1.2%) | [-88, -8]ns | [5306, 5323] | YES | 0.0276 | 0.0166 | 0 |
| warm-container-native | 5289ns | no significant difference | [-74, +12]ns | [5243, 5331] | no | 0.4296 | 0.4296 | 0 |
| warm-container-plusone | 5493ns | no significant difference | [-12, +164]ns | [5301, 5514] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5340ns | -86.9% | -86.9% | -0.4% | +3.1% | +3.3% |
| 2 | 5252ns | -86.7% | -86.7% | +1.4% | +4.9% | +5.2% |
| 3 | 5407ns | -86.4% | -87.1% | -1.6% | +2.1% | +2.5% |
| 4 | 5251ns | -86.8% | -86.7% | +1.3% | +5.0% | +5.0% |
| 5 | 5403ns | -87.1% | -87.0% | -1.5% | -1.9% | +1.9% |
| 6 | 5495ns | -87.3% | -87.2% | -3.3% | -4.8% | +0.2% |
| 7 | 5466ns | -87.1% | -87.2% | -2.6% | -3.1% | +1.0% |
| 8 | 5429ns | -87.0% | -87.1% | -0.9% | -3.6% | +1.6% |
| 9 | 5398ns | -87.1% | -87.0% | -1.0% | -2.8% | +2.2% |
| 10 | 5348ns | -83.6% | -86.9% | -0.0% | +3.6% | +3.1% |
| 11 | 5417ns | -87.8% | -86.1% | +1.5% | -3.5% | +9.5% |
| 12 | 5368ns | -87.6% | -86.0% | +1.6% | -2.7% | +10.4% |
| 13 | 5399ns | -87.7% | -86.1% | -1.7% | -3.3% | +9.7% |
| 14 | 5509ns | -87.9% | -86.3% | -2.8% | -4.8% | +7.6% |
| 15 | 5374ns | -87.5% | -86.0% | -0.2% | -2.7% | +10.6% |
| 16 | 5230ns | -87.1% | -85.5% | +1.9% | -0.1% | +12.9% |
| 17 | 5274ns | -87.2% | -85.7% | +0.7% | -0.9% | -0.2% |
| 18 | 5239ns | -87.1% | -85.6% | +1.3% | -0.3% | -0.3% |
| 19 | 5234ns | -87.2% | -85.6% | +1.5% | -0.2% | -0.2% |
| 20 | 5240ns | -87.3% | -85.6% | +3.0% | +0.4% | +0.1% |
| 21 | 5499ns | -87.8% | -86.3% | -5.1% | +0.1% | +6.3% |
| 22 | 5498ns | -87.9% | -86.3% | -5.1% | +0.0% | -3.1% |
| 23 | 5441ns | -87.5% | -86.3% | -4.0% | +1.1% | -2.2% |
| 24 | 5506ns | -87.9% | -86.4% | -5.2% | -0.1% | -3.7% |
| 25 | 5310ns | -87.3% | -85.9% | -1.7% | +3.4% | -0.1% |
| 26 | 5347ns | -87.4% | -86.0% | -2.1% | +2.5% | +2.8% |
| 27 | 5303ns | -87.5% | -85.9% | -1.6% | +1.5% | +3.6% |
| 28 | 5300ns | -87.4% | -85.8% | -1.5% | +0.6% | +3.6% |
| 29 | 5331ns | -87.0% | -85.9% | -1.9% | +0.0% | +3.0% |
| 30 | 5276ns | -87.2% | -85.8% | -0.7% | +0.6% | +4.1% |
| 31 | 5297ns | -87.3% | -87.5% | -1.2% | +2.9% | -1.4% |
| 32 | 5300ns | -87.4% | -87.2% | -1.1% | -1.1% | -1.4% |
| 33 | 5312ns | -87.4% | -87.6% | -1.3% | -0.8% | -1.3% |
| 34 | 5315ns | -87.4% | -87.6% | -0.1% | -1.0% | -1.7% |
| 35 | 5333ns | -87.4% | -87.5% | +0.2% | -1.9% | -2.0% |
| 36 | 5518ns | -87.8% | -88.0% | -3.9% | -5.3% | -5.1% |
| 37 | 5686ns | -87.9% | -88.3% | -5.9% | -7.9% | -8.0% |
| 38 | 5302ns | -87.2% | -87.5% | +3.6% | -0.3% | -1.4% |
| 39 | 5261ns | -87.2% | -87.4% | +1.0% | +1.0% | -0.7% |
| 40 | 5396ns | -87.5% | -87.7% | -1.7% | -1.7% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.366 | moderate+ |
| warm-container-kernel | 0.119 | ok |
| warm-container-lanes-deferred | 0.873 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.618 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.616 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.699 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 27/40, lost 12/40
- **warm-container-native**: won 22/40, lost 14/40
- **warm-container-plusone**: won 17/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 5365.1ns | 0.1% |  |
| warm-container-kernel | 3.0ns | 684.4ns | 0.4% |  |
| warm-container-lanes-deferred | 2.9ns | 716.4ns | 0.4% |  |
| warm-container-minimum | 2.7ns | 5308.4ns | 0.1% |  |
| warm-container-native | 3.1ns | 5333.6ns | 0.1% |  |
| warm-container-plusone | 3.1ns | 5465.4ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5247.6-5522.2 ns)
   5247.6 |##############################
   5261.3 |##########
   5275.1 |##########
   5288.8 |########################################
   5302.5 |########################################
   5316.2 |
   5330.0 |##############################
   5343.7 |####################
   5357.4 |##########
   5371.2 |##########
   5384.9 |####################
   5398.6 |##############################
   5412.3 |##########
   5426.1 |##########
   5439.8 |##########
   5453.5 |##########
   5467.3 |
   5481.0 |
   5494.7 |########################################
   5508.4 |####################
  (4 below, 1 above range)

warm-container-kernel (n=40, range 665.3-727.5 ns)
    665.3 |##################################
    668.4 |########################################
    671.5 |############################
    674.7 |#################
    677.8 |###########
    680.9 |#####
    684.0 |
    687.1 |#####
    690.2 |#####
    693.3 |###########
    696.4 |###########
    699.5 |###########
    702.6 |###########
    705.7 |
    708.8 |
    712.0 |
    715.1 |
    718.2 |
    721.3 |
    724.4 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 661.5-753.7 ns)
    661.5 |#####################
    666.1 |
    670.7 |
    675.3 |###
    680.0 |
    684.6 |
    689.2 |
    693.8 |##########
    698.4 |#########################
    703.0 |
    707.6 |
    712.2 |
    716.8 |
    721.4 |
    726.0 |
    730.6 |
    735.2 |
    739.8 |
    744.4 |##################
    749.0 |########################################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 5221.4-5410.8 ns)
   5221.4 |####################
   5230.8 |##########################
   5240.3 |######
   5249.8 |
   5259.3 |
   5268.7 |
   5278.2 |
   5287.7 |
   5297.1 |#############
   5306.6 |########################################
   5316.1 |########################################
   5325.5 |#############
   5335.0 |#############
   5344.5 |#############
   5354.0 |#############
   5363.4 |
   5372.9 |######
   5382.4 |
   5391.8 |######
   5401.3 |
  (5 below, 3 above range)

warm-container-native (n=40, range 5224.4-5512.1 ns)
   5224.4 |########################################
   5238.8 |############
   5253.2 |########
   5267.6 |####
   5282.0 |########
   5296.4 |############
   5310.7 |####
   5325.1 |########
   5339.5 |
   5353.9 |
   5368.3 |####
   5382.6 |
   5397.0 |
   5411.4 |
   5425.8 |
   5440.2 |####
   5454.6 |
   5468.9 |####
   5483.3 |####
   5497.7 |########################
  (3 below, 3 above range)

warm-container-plusone (n=40, range 5225.0-5868.4 ns)
   5225.0 |########################################
   5257.1 |#####
   5289.3 |###############
   5321.5 |##########
   5353.6 |
   5385.8 |
   5418.0 |
   5450.1 |
   5482.3 |########################################
   5514.5 |###################################
   5546.7 |
   5578.8 |
   5611.0 |
   5643.2 |
   5675.3 |
   5707.5 |
   5739.7 |
   5771.9 |
   5804.0 |
   5836.2 |#####
  (4 below, 6 above range)

```

## Diagnostics

- **warm-container-lanes-deferred**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.70 (measurement drift or warm-up artifact)
