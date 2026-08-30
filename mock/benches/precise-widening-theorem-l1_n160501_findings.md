# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 679 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 83% (significant)

warm-container-lanes-deferred is -4.83 us (83%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 8.5x slower than the field

warm-container-headroom (5.79 us) is 8.5x the fastest (679 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.81)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-kernel} vs {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} (647% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} with a 647% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.5x the fastest

Fastest warm-container-lanes-deferred (679 ns) to slowest warm-container-headroom (5.79 us): 8.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native's edge over baseline is significant but tiny (-1 ns, 0.02%)

warm-container-native differs from baseline warm-container-headroom by -1 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 679.2 ns median (-88.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 8.52x (fastest 679.2 ns, slowest 5786.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5675ns | 5849ns | 5279ns | 5694ns | 6015ns | base |
| warm-container-kernel | 775ns | 765ns | 736ns | 767ns | 838ns | -86.35% |
| warm-container-lanes-deferred | 745ns | 742ns | 736ns | 743ns | 761ns | -86.86% |
| warm-container-minimum | 5407ns | 5294ns | 5278ns | 5318ns | 5803ns | -4.72% |
| warm-container-native | 5558ns | 5327ns | 5276ns | 5474ns | 6092ns | -2.06% |
| warm-container-plusone | 5412ns | 5315ns | 5279ns | 5352ns | 5724ns | -4.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5610ns | 5222ns | 5945ns | base | 2.920 |
| warm-container-kernel | 711ns | 677ns | 768ns | -87.33% | 23.041 |
| warm-container-lanes-deferred | 684ns | 676ns | 699ns | -87.81% | 23.963 |
| warm-container-minimum | 5345ns | 5220ns | 5737ns | -4.73% | 3.065 |
| warm-container-native | 5495ns | 5219ns | 6019ns | -2.06% | 2.982 |
| warm-container-plusone | 5351ns | 5222ns | 5660ns | -4.62% | 3.062 |

## Performance model

- Peak throughput: **24.251 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.832 | 11.7% |
| warm-container-kernel | 23.384 | 96.4% |
| warm-container-lanes-deferred | 24.122 | 99.5% |
| warm-container-minimum | 3.129 | 12.9% |
| warm-container-native | 3.115 | 12.8% |
| warm-container-plusone | 3.117 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5675ns | 5675ns | base |
| warm-container-kernel | 775ns | 775ns | -86.35% |
| warm-container-lanes-deferred | 745ns | 745ns | -86.86% |
| warm-container-minimum | 5407ns | 5407ns | -4.72% |
| warm-container-native | 5558ns | 5558ns | -2.06% |
| warm-container-plusone | 5412ns | 5412ns | -4.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5786ns | base | --- | [5287, 5921] | --- | --- | --- | --- |
| warm-container-kernel | 701ns | -5094.1ns (-88.0%) | [-5208, -4547]ns | [688, 712] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 679ns | -5109.2ns (-88.3%) | [-5236, -4598]ns | [678, 686] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5235ns | -11.8ns (-0.2%) | [-590, -2]ns | [5225, 5300] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-native | 5260ns | no significant difference | [-10, +3]ns | [5234, 5446] | no | 0.4296 | 0.4296 | 0 |
| warm-container-plusone | 5257ns | -386.7ns (-6.7%) | [-436, -4]ns | [5234, 5301] | YES | 0.0173 | 0.0139 | 2 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5734ns | -88.2% | -88.2% | +3.6% | +4.3% | -8.2% |
| 2 | 5921ns | -88.5% | -88.4% | -0.0% | +1.0% | -11.5% |
| 3 | 5925ns | -88.6% | -88.6% | +0.1% | +0.9% | -11.7% |
| 4 | 5932ns | -88.4% | -88.6% | -0.2% | -1.7% | -11.5% |
| 5 | 5924ns | -88.6% | -88.5% | -0.1% | -11.3% | -11.6% |
| 6 | 5925ns | -88.6% | -88.2% | -10.0% | -11.6% | -10.9% |
| 7 | 5926ns | -88.5% | -88.4% | -6.9% | -11.5% | -11.7% |
| 8 | 5922ns | -88.6% | -88.0% | -11.7% | -11.7% | -11.8% |
| 9 | 5286ns | -87.2% | -86.7% | -1.3% | +3.1% | -0.5% |
| 10 | 5315ns | -87.3% | -87.1% | -0.1% | +11.5% | -1.4% |
| 11 | 5287ns | -85.5% | -87.2% | -1.2% | -0.5% | +12.7% |
| 12 | 5228ns | -85.4% | -87.0% | -0.2% | -0.2% | +16.1% |
| 13 | 5222ns | -85.4% | -87.1% | +2.3% | +4.1% | +3.1% |
| 14 | 5231ns | -85.3% | -87.0% | +1.3% | +0.6% | +1.5% |
| 15 | 5335ns | -85.6% | -87.3% | +0.6% | +0.1% | +0.7% |
| 16 | 5302ns | -85.6% | -87.2% | +1.5% | -1.4% | -1.0% |
| 17 | 5961ns | -87.1% | -88.7% | -11.1% | -8.8% | -11.8% |
| 18 | 5960ns | -87.1% | -88.7% | -11.1% | -11.1% | -11.2% |
| 19 | 5995ns | -87.2% | -88.7% | -11.3% | -13.0% | -12.3% |
| 20 | 5700ns | -86.5% | -88.2% | -8.4% | -8.4% | +1.2% |
| 21 | 5224ns | -87.0% | -86.8% | +0.3% | -0.1% | -0.1% |
| 22 | 5219ns | -87.0% | -86.8% | -0.0% | +0.3% | +0.0% |
| 23 | 5228ns | -86.9% | -86.9% | -0.1% | -0.2% | -0.1% |
| 24 | 5229ns | -87.0% | -86.9% | -0.1% | -0.2% | +0.0% |
| 25 | 5222ns | -86.8% | -86.7% | +0.0% | -0.0% | +0.2% |
| 26 | 5221ns | -86.8% | -86.7% | +0.0% | -0.0% | +0.7% |
| 27 | 5222ns | -86.7% | -86.9% | -0.0% | +0.3% | +0.0% |
| 28 | 5229ns | -86.8% | -86.9% | -0.1% | -0.1% | -0.1% |
| 29 | 5220ns | -86.8% | -86.5% | +0.1% | -0.0% | +0.0% |
| 30 | 5222ns | -86.8% | -86.9% | +0.2% | +0.1% | +0.0% |
| 31 | 5916ns | -88.0% | -88.4% | -11.3% | +7.0% | -11.6% |
| 32 | 5921ns | -87.9% | -88.3% | -11.6% | +3.2% | -11.7% |
| 33 | 5922ns | -88.0% | -88.5% | -11.6% | -0.0% | -7.4% |
| 34 | 5926ns | -88.0% | -88.6% | -11.9% | -10.4% | -7.3% |
| 35 | 5924ns | -88.0% | -88.6% | -11.6% | -11.7% | -7.2% |
| 36 | 5933ns | -88.0% | -88.6% | -12.0% | -0.2% | -7.3% |
| 37 | 5922ns | -87.9% | -88.6% | -11.9% | +0.0% | -7.2% |
| 38 | 5921ns | -88.0% | -88.6% | -11.8% | +0.0% | -7.2% |
| 39 | 5920ns | -88.0% | -88.5% | -11.3% | +0.0% | -7.2% |
| 40 | 5838ns | -87.9% | -88.4% | -10.1% | +1.5% | -6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.802 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.806 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.538 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.787 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.597 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.456 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 24/40, lost 7/40
- **warm-container-native**: won 18/40, lost 12/40
- **warm-container-plusone**: won 25/40, lost 8/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 5610.3ns | 0.0% |  |
| warm-container-kernel | 2.8ns | 711.1ns | 0.4% |  |
| warm-container-lanes-deferred | 2.7ns | 683.7ns | 0.4% |  |
| warm-container-minimum | 2.7ns | 5344.8ns | 0.1% |  |
| warm-container-native | 2.7ns | 5494.8ns | 0.0% |  |
| warm-container-plusone | 2.8ns | 5351.3ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5221.6-5944.7 ns)
   5221.6 |#########################
   5257.7 |#####
   5293.9 |#####
   5330.0 |##
   5366.2 |
   5402.4 |
   5438.5 |
   5474.7 |
   5510.8 |
   5547.0 |
   5583.2 |
   5619.3 |
   5655.5 |
   5691.6 |##
   5727.8 |##
   5763.9 |
   5800.1 |
   5836.3 |##
   5872.4 |
   5908.6 |########################################
  (3 below, 3 above range)

warm-container-kernel (n=40, range 676.7-767.9 ns)
    676.7 |###################################
    681.3 |#####
    685.8 |####################
    690.4 |###############
    695.0 |
    699.5 |
    704.1 |
    708.6 |########################################
    713.2 |##########
    717.8 |
    722.3 |
    726.9 |
    731.4 |
    736.0 |
    740.5 |
    745.1 |
    749.7 |
    754.2 |
    758.8 |
    763.3 |##############################
  (5 below, 4 above range)

warm-container-lanes-deferred (n=40, range 675.6-698.7 ns)
    675.6 |########################################
    676.8 |####################
    677.9 |####
    679.1 |############
    680.2 |
    681.4 |####
    682.5 |
    683.7 |####
    684.8 |############
    686.0 |################
    687.1 |########
    688.3 |
    689.5 |########
    690.6 |
    691.8 |
    692.9 |####
    694.1 |
    695.2 |
    696.4 |####
    697.5 |
  (2 below, 4 above range)

warm-container-minimum (n=40, range 5219.8-5736.6 ns)
   5219.8 |########################################
   5245.7 |######
   5271.5 |
   5297.3 |###########
   5323.2 |####
   5349.0 |##
   5374.9 |##
   5400.7 |
   5426.5 |
   5452.4 |
   5478.2 |
   5504.1 |##
   5529.9 |
   5555.7 |
   5581.6 |
   5607.4 |
   5633.3 |
   5659.1 |
   5684.9 |
   5710.8 |
  (4 below, 5 above range)

warm-container-native (n=40, range 5219.4-6019.0 ns)
   5219.4 |########################################
   5259.4 |#####
   5299.3 |#######
   5339.3 |
   5379.3 |
   5419.3 |#######
   5459.2 |
   5499.2 |
   5539.2 |
   5579.2 |
   5619.2 |
   5659.1 |
   5699.1 |
   5739.1 |
   5779.1 |
   5819.1 |##
   5859.0 |
   5899.0 |#################
   5939.0 |#######
   5979.0 |
  (3 below, 2 above range)

warm-container-plusone (n=40, range 5222.2-5660.2 ns)
   5222.2 |########################################
   5244.1 |#####################
   5266.0 |###
   5287.9 |######
   5309.8 |
   5331.7 |
   5353.6 |###
   5375.5 |###
   5397.4 |
   5419.3 |
   5441.2 |
   5463.1 |###
   5485.0 |#####################
   5506.9 |
   5528.8 |
   5550.7 |
   5572.6 |
   5594.5 |
   5616.4 |
   5638.3 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.60 (measurement drift or warm-up artifact)
