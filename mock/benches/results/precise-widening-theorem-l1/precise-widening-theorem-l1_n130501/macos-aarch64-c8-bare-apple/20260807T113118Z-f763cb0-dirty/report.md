# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 87% (significant)

warm-container-kernel is -4.60 us (87%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native is an outlier: 7.8x slower than the field

warm-container-native (5.30 us) is 7.8x the fastest (679 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows warm-up / thermal drift (autocorr +0.80)

warm-container-native's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} (665% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} with a 665% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.8x the fastest

Fastest warm-container-kernel (679 ns) to slowest warm-container-native (5.30 us): 7.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum's edge over baseline is significant but tiny (45 ns, 0.86%)

warm-container-minimum differs from baseline warm-container-headroom by 45 ns (0.86%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 679.2 ns median (-87.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.80x (fastest 679.2 ns, slowest 5300.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5356ns | 5323ns | 5279ns | 5330ns | 5510ns | base |
| warm-container-kernel | 754ns | 740ns | 736ns | 746ns | 795ns | -85.93% |
| warm-container-lanes-deferred | 765ns | 749ns | 737ns | 749ns | 844ns | -85.71% |
| warm-container-minimum | 5613ns | 5353ns | 5275ns | 5497ns | 6300ns | +4.81% |
| warm-container-native | 5545ns | 5355ns | 5280ns | 5452ns | 6088ns | +3.53% |
| warm-container-plusone | 5588ns | 5361ns | 5279ns | 5453ns | 6302ns | +4.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5292ns | 5218ns | 5445ns | base | 3.096 |
| warm-container-kernel | 692ns | 677ns | 728ns | -86.92% | 23.678 |
| warm-container-lanes-deferred | 703ns | 678ns | 776ns | -86.71% | 23.300 |
| warm-container-minimum | 5548ns | 5219ns | 6225ns | +4.85% | 2.953 |
| warm-container-native | 5481ns | 5218ns | 6019ns | +3.57% | 2.989 |
| warm-container-plusone | 5524ns | 5219ns | 6234ns | +4.40% | 2.966 |

## Performance model

- Peak throughput: **24.207 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.114 | 12.9% |
| warm-container-kernel | 24.122 | 99.7% |
| warm-container-lanes-deferred | 23.810 | 98.4% |
| warm-container-minimum | 3.096 | 12.8% |
| warm-container-native | 3.091 | 12.8% |
| warm-container-plusone | 3.091 | 12.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5356ns | 5356ns | base |
| warm-container-kernel | 754ns | 754ns | -85.93% |
| warm-container-lanes-deferred | 765ns | 765ns | -85.71% |
| warm-container-minimum | 5613ns | 5613ns | +4.81% |
| warm-container-native | 5545ns | 5545ns | +3.53% |
| warm-container-plusone | 5588ns | 5588ns | +4.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5261ns | base | --- | [5225, 5299] | --- | --- | --- | --- |
| warm-container-kernel | 679ns | -4567.7ns (-86.8%) | [-4598, -4544]ns | [678, 681] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 688ns | -4546.2ns (-86.4%) | [-4590, -4534]ns | [680, 691] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5293ns | no significant difference | [-1, +346]ns | [5236, 5567] | no | 0.1345 | 0.0807 | 0 |
| warm-container-native | 5300ns | no significant difference | [-1, +98]ns | [5226, 5371] | no | 0.1352 | 0.1081 | 1 |
| warm-container-plusone | 5300ns | no significant difference | [-19, +141]ns | [5242, 5421] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5236ns | -87.0% | -86.8% | -0.3% | -0.3% | -0.3% |
| 2 | 5253ns | -87.1% | -86.8% | -0.7% | +0.2% | -0.6% |
| 3 | 5255ns | -87.1% | -86.6% | -0.7% | +0.8% | -0.1% |
| 4 | 5218ns | -87.0% | -86.8% | +0.0% | +1.6% | +1.4% |
| 5 | 5216ns | -87.0% | -86.8% | +0.1% | +16.6% | +1.8% |
| 6 | 5220ns | -86.9% | -86.8% | +0.0% | +21.3% | +2.7% |
| 7 | 5217ns | -87.0% | -86.8% | +0.1% | +17.4% | +1.6% |
| 8 | 5234ns | -87.0% | -86.5% | +0.1% | +2.9% | +1.7% |
| 9 | 5219ns | -87.0% | -86.8% | +5.2% | +1.8% | +1.4% |
| 10 | 5297ns | -87.2% | -86.7% | +3.7% | +1.0% | -1.5% |
| 11 | 5295ns | -87.2% | -84.4% | +0.4% | -1.5% | +19.6% |
| 12 | 5323ns | -87.3% | -84.5% | +0.3% | -1.6% | +18.9% |
| 13 | 5275ns | -87.1% | -84.4% | +1.1% | -1.1% | +20.0% |
| 14 | 5222ns | -87.0% | -84.2% | +8.1% | -0.0% | +21.2% |
| 15 | 5262ns | -87.1% | -86.2% | -0.5% | -0.8% | +20.3% |
| 16 | 5220ns | -87.0% | -87.0% | +1.3% | -0.0% | +21.3% |
| 17 | 5223ns | -87.0% | -87.0% | -0.1% | +1.1% | +7.3% |
| 18 | 5226ns | -87.0% | -87.0% | -0.1% | +1.9% | +0.2% |
| 19 | 5261ns | -87.1% | -87.1% | -0.5% | +1.8% | -0.8% |
| 20 | 5280ns | -87.2% | -86.9% | -1.1% | +0.9% | -0.5% |
| 21 | 5311ns | -86.6% | -85.5% | -0.6% | -1.8% | -1.7% |
| 22 | 5310ns | -86.6% | -86.5% | -0.2% | -1.8% | -0.2% |
| 23 | 5329ns | -86.6% | -87.2% | +11.1% | -2.0% | -2.0% |
| 24 | 5272ns | -86.4% | -87.1% | +12.3% | -1.0% | -1.1% |
| 25 | 5222ns | -86.3% | -86.8% | +13.4% | -0.1% | +1.5% |
| 26 | 5218ns | -86.3% | -86.8% | +14.2% | +0.3% | +14.2% |
| 27 | 5223ns | -86.3% | -87.0% | +13.4% | +0.2% | +13.4% |
| 28 | 5220ns | -86.3% | -86.8% | +13.4% | +0.0% | +13.4% |
| 29 | 5220ns | -86.3% | -86.8% | +13.1% | -0.0% | +4.7% |
| 30 | 5219ns | -86.3% | -86.8% | +0.3% | +0.0% | -0.0% |
| 31 | 5671ns | -88.0% | -88.1% | -7.6% | -4.2% | -5.2% |
| 32 | 5324ns | -87.3% | -87.2% | -1.3% | +11.3% | -0.4% |
| 33 | 5338ns | -87.3% | -87.3% | -2.2% | +10.9% | -2.2% |
| 34 | 5325ns | -87.2% | -87.3% | -2.0% | +11.2% | -1.9% |
| 35 | 5699ns | -88.1% | -88.1% | +9.6% | +4.0% | -8.4% |
| 36 | 5457ns | -87.6% | -87.6% | +16.1% | +8.5% | -4.4% |
| 37 | 5353ns | -87.3% | -87.3% | +18.3% | +10.7% | -2.4% |
| 38 | 5386ns | -87.4% | -87.4% | +17.6% | +9.9% | +2.6% |
| 39 | 5322ns | -85.7% | -87.3% | +19.0% | +11.2% | +11.2% |
| 40 | 5302ns | -85.4% | -87.2% | +19.4% | +11.7% | +11.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.362 | moderate+ |
| warm-container-kernel | 0.616 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.742 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.780 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.801 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.759 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 13/40, lost 22/40
- **warm-container-native**: won 10/40, lost 24/40
- **warm-container-plusone**: won 16/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.0ns | 5291.9ns | 0.1% |  |
| warm-container-kernel | 2.4ns | 692.0ns | 0.3% |  |
| warm-container-lanes-deferred | 2.5ns | 703.2ns | 0.4% |  |
| warm-container-minimum | 3.0ns | 5548.4ns | 0.1% |  |
| warm-container-native | 2.9ns | 5481.0ns | 0.1% |  |
| warm-container-plusone | 3.0ns | 5524.5ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5218.3-5444.7 ns)
   5218.3 |########################################
   5229.6 |######
   5240.9 |
   5252.3 |############
   5263.6 |###
   5274.9 |######
   5286.2 |######
   5297.5 |###
   5308.9 |######
   5320.2 |###############
   5331.5 |###
   5342.8 |###
   5354.2 |
   5365.5 |
   5376.8 |###
   5388.1 |
   5399.4 |
   5410.8 |
   5422.1 |
   5433.4 |
  (2 below, 3 above range)

warm-container-kernel (n=40, range 676.8-728.4 ns)
    676.8 |########################################
    679.4 |###########
    682.0 |##
    684.6 |
    687.1 |
    689.7 |
    692.3 |
    694.9 |
    697.5 |
    700.0 |
    702.6 |
    705.2 |
    707.8 |
    710.3 |##
    712.9 |#############
    715.5 |######
    718.1 |
    720.7 |
    723.2 |
    725.8 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 677.6-776.2 ns)
    677.6 |########################################
    682.6 |############
    687.5 |####################################
    692.4 |########
    697.4 |####
    702.3 |########
    707.2 |
    712.2 |####
    717.1 |
    722.0 |####
    726.9 |
    731.9 |
    736.8 |
    741.7 |
    746.7 |
    751.6 |
    756.5 |
    761.5 |
    766.4 |####
    771.3 |
  (6 below, 4 above range)

warm-container-minimum (n=40, range 5218.9-6224.9 ns)
   5218.9 |########################################
   5269.2 |###########
   5319.5 |#####
   5369.8 |
   5420.1 |
   5470.4 |#####
   5520.7 |
   5571.0 |
   5621.3 |##
   5671.6 |
   5721.9 |
   5772.2 |
   5822.5 |
   5872.8 |#################
   5923.1 |##
   5973.4 |
   6023.7 |
   6074.0 |
   6124.3 |
   6174.6 |
  (4 below, 6 above range)

warm-container-native (n=40, range 5218.1-6019.4 ns)
   5218.1 |########################################
   5258.1 |#####
   5298.2 |##############
   5338.3 |#####
   5378.3 |##
   5418.4 |##
   5458.5 |
   5498.5 |
   5538.6 |
   5578.6 |
   5618.7 |
   5658.8 |
   5698.8 |
   5738.9 |
   5779.0 |
   5819.0 |
   5859.1 |
   5899.2 |#########################
   5939.2 |
   5979.3 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 5218.9-6233.8 ns)
   5218.9 |########################################
   5269.7 |#####################
   5320.4 |######
   5371.2 |###
   5421.9 |###
   5472.6 |
   5523.4 |###
   5574.1 |###
   5624.9 |
   5675.6 |
   5726.4 |
   5777.1 |
   5827.9 |
   5878.6 |############
   5929.4 |###
   5980.1 |
   6030.9 |
   6081.6 |
   6132.4 |
   6183.1 |
  (3 below, 6 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.76 (measurement drift or warm-up artifact)
