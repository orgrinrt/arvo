# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 15% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (108 ns) leads warm-clamp-min-lanes (124 ns) by 15%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 89% (significant)

warm-clamp-minimum is -766 ns (89%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 38.2x slower than the field

warm-clamp-accfit-dyn (4.12 us) is 38.2x the fastest (108 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (377% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 377% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 38.2x the fastest

Fastest warm-clamp-minimum (108 ns) to slowest warm-clamp-accfit-dyn (4.12 us): 38.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.5x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 105 ns but its worst 20% at 368 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 107.9 ns median (-87.5% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 38.18x (fastest 107.9 ns, slowest 4119.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 933ns | 925ns | 921ns | 930ns | 955ns | base |
| warm-clamp-accfit | 374ns | 293ns | 228ns | 306ns | 724ns | -59.91% |
| warm-clamp-accfit-dyn | 4968ns | 4174ns | 4146ns | 4189ns | 8125ns | +432.32% |
| warm-clamp-head | 362ns | 377ns | 313ns | 358ns | 422ns | -61.23% |
| warm-clamp-min-lanes | 280ns | 193ns | 163ns | 221ns | 574ns | -69.97% |
| warm-clamp-minimum | 173ns | 167ns | 164ns | 171ns | 188ns | -81.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 872ns | 861ns | 892ns | base | 9.400 |
| warm-clamp-accfit | 283ns | 170ns | 552ns | -67.58% | 28.997 |
| warm-clamp-accfit-dyn | 4899ns | 4091ns | 8005ns | +462.10% | 1.672 |
| warm-clamp-head | 293ns | 252ns | 346ns | -66.38% | 27.957 |
| warm-clamp-min-lanes | 181ns | 105ns | 368ns | -79.21% | 45.211 |
| warm-clamp-minimum | 112ns | 106ns | 122ns | -87.14% | 73.096 |

## Performance model

- Peak throughput: **77.880 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.477 | 12.2% |
| warm-clamp-accfit | 37.777 | 48.5% |
| warm-clamp-accfit-dyn | 1.988 | 2.6% |
| warm-clamp-head | 26.912 | 34.6% |
| warm-clamp-min-lanes | 66.091 | 84.9% |
| warm-clamp-minimum | 75.922 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 933ns | 933ns | base |
| warm-clamp-accfit | 374ns | 374ns | -59.91% |
| warm-clamp-accfit-dyn | 4968ns | 4968ns | +432.32% |
| warm-clamp-head | 362ns | 362ns | -61.23% |
| warm-clamp-min-lanes | 280ns | 280ns | -69.97% |
| warm-clamp-minimum | 173ns | 173ns | -81.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 864ns | base | --- | [863, 876] | --- | --- | --- | --- |
| warm-clamp-accfit | 217ns | -673.1ns (-77.9%) | [-688, -655]ns | [176, 222] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4120ns | +3246.0ns (+375.5%) | [+3232, +3293]ns | [4095, 4166] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 304ns | -578.2ns (-66.9%) | [-606, -557]ns | [260, 307] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 124ns | -749.0ns (-86.6%) | [-755, -737]ns | [109, 143] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 108ns | -755.7ns (-87.4%) | [-757, -755]ns | [108, 115] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 864ns | -36.2% | +1934.3% | -70.8% | -57.4% | -87.5% |
| 2 | 859ns | -35.6% | +1402.8% | -70.6% | -57.0% | -87.4% |
| 3 | 865ns | -36.3% | +1164.4% | -71.0% | -57.8% | -87.5% |
| 4 | 862ns | -36.1% | +382.4% | -70.7% | -57.3% | -87.6% |
| 5 | 862ns | -36.0% | +387.7% | -70.9% | -57.3% | -87.7% |
| 6 | 865ns | -36.7% | +388.5% | -70.6% | -57.5% | -87.2% |
| 7 | 862ns | -36.1% | +387.7% | -70.8% | -57.2% | -87.8% |
| 8 | 863ns | -35.7% | +376.5% | -70.9% | -57.2% | -87.5% |
| 9 | 863ns | -36.5% | +374.6% | -70.7% | -57.4% | -87.5% |
| 10 | 885ns | -37.5% | +366.6% | -71.5% | -58.5% | -88.1% |
| 11 | 885ns | -75.2% | +416.0% | -62.4% | -83.5% | -86.3% |
| 12 | 881ns | -75.0% | +371.5% | -62.6% | -83.5% | -86.2% |
| 13 | 895ns | -75.9% | +364.5% | -63.1% | -83.9% | -86.4% |
| 14 | 889ns | -75.1% | +370.6% | -62.8% | -84.1% | -86.4% |
| 15 | 882ns | -75.3% | +373.1% | -62.5% | -84.1% | -86.2% |
| 16 | 875ns | -74.6% | +379.0% | -62.0% | -84.4% | -86.2% |
| 17 | 876ns | -74.7% | +374.2% | -62.0% | -83.2% | -86.4% |
| 18 | 876ns | -74.7% | +396.7% | -62.0% | -84.4% | -86.4% |
| 19 | 877ns | -75.0% | +377.7% | -62.2% | -83.9% | -86.5% |
| 20 | 875ns | -74.9% | +377.6% | -62.1% | -83.7% | -86.3% |
| 21 | 859ns | -80.5% | +376.3% | -68.9% | -88.0% | -87.6% |
| 22 | 862ns | -80.6% | +374.6% | -70.1% | -87.6% | -87.5% |
| 23 | 862ns | -80.6% | +381.7% | -69.6% | -87.3% | -87.5% |
| 24 | 862ns | -65.4% | +375.1% | -68.7% | -87.2% | -87.5% |
| 25 | 863ns | -80.6% | +373.9% | -70.4% | -87.6% | -87.4% |
| 26 | 863ns | -80.2% | +374.0% | -70.5% | -87.7% | -87.5% |
| 27 | 869ns | -80.3% | +371.1% | -69.7% | -87.8% | -87.7% |
| 28 | 878ns | -80.6% | +365.8% | -55.3% | -88.0% | -87.9% |
| 29 | 882ns | -80.0% | +364.1% | -70.9% | -87.7% | -88.0% |
| 30 | 888ns | -80.0% | +493.4% | -70.1% | -88.1% | -87.8% |
| 31 | 904ns | -80.7% | +352.7% | -66.0% | -87.9% | -88.2% |
| 32 | 862ns | -79.7% | +374.8% | -64.4% | -87.9% | -87.5% |
| 33 | 905ns | -80.5% | +353.2% | -57.8% | -88.1% | -88.3% |
| 34 | 858ns | -79.5% | +377.9% | -64.3% | -87.2% | -87.6% |
| 35 | 864ns | -79.7% | +374.1% | -64.6% | -87.1% | -87.6% |
| 36 | 863ns | -79.6% | +374.7% | -64.7% | -87.7% | -87.1% |
| 37 | 863ns | -79.7% | +374.5% | -64.5% | -87.5% | -86.1% |
| 38 | 864ns | -79.7% | +373.4% | -64.8% | -87.8% | -85.8% |
| 39 | 864ns | -79.7% | +373.4% | -64.7% | -87.6% | -85.9% |
| 40 | 865ns | -79.5% | +373.1% | -64.4% | -87.3% | -85.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.376 | moderate+ |
| warm-clamp-accfit | 0.883 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.581 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.489 | moderate+ |
| warm-clamp-min-lanes | 0.904 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.776 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.6ns | 871.5ns | 0.3% |  |
| warm-clamp-accfit | 4.2ns | 282.5ns | 1.5% |  |
| warm-clamp-accfit-dyn | 3.4ns | 4898.8ns | 0.1% |  |
| warm-clamp-head | 2.6ns | 293.0ns | 0.9% |  |
| warm-clamp-min-lanes | 4.4ns | 181.2ns | 2.5% |  |
| warm-clamp-minimum | 2.6ns | 112.1ns | 2.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 860.6-891.6 ns)
    860.6 |##################################
    862.1 |########################################
    863.7 |########################################
    865.2 |
    866.8 |
    868.3 |#####
    869.9 |
    871.4 |
    873.0 |
    874.5 |#################
    876.1 |###########
    877.7 |#####
    879.2 |
    880.8 |###########
    882.3 |#####
    883.9 |###########
    885.4 |
    887.0 |#####
    888.5 |#####
    890.1 |
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 169.5-552.1 ns)
    169.5 |########################################
    188.7 |
    207.8 |##########################
    226.9 |
    246.0 |
    265.2 |
    284.3 |##
    303.4 |
    322.5 |
    341.7 |
    360.8 |
    379.9 |
    399.1 |
    418.2 |
    437.3 |
    456.4 |
    475.6 |
    494.7 |
    513.8 |
    532.9 |##################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 4091.1-8004.6 ns)
   4091.1 |########################################
   4286.8 |#
   4482.4 |#
   4678.1 |
   4873.8 |
   5069.5 |
   5265.1 |#
   5460.8 |
   5656.5 |
   5852.2 |
   6047.8 |
   6243.5 |
   6439.2 |
   6634.9 |
   6830.5 |
   7026.2 |
   7221.9 |
   7417.6 |
   7613.2 |
   7808.9 |
  (2 below, 3 above range)

warm-clamp-head (n=40, range 251.8-346.1 ns)
    251.8 |###################################
    256.5 |########
    261.2 |#############
    266.0 |########
    270.7 |
    275.4 |
    280.1 |
    284.8 |
    289.5 |
    294.3 |
    299.0 |
    303.7 |########################################
    308.4 |
    313.1 |
    317.8 |
    322.6 |
    327.3 |##########################
    332.0 |#################
    336.7 |
    341.4 |
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 105.2-368.4 ns)
    105.2 |########################################
    118.3 |
    131.5 |################
    144.7 |#######
    157.8 |
    171.0 |
    184.1 |
    197.3 |
    210.5 |
    223.6 |
    236.8 |
    249.9 |
    263.1 |
    276.3 |
    289.4 |
    302.6 |
    315.7 |
    328.9 |
    342.1 |
    355.2 |################
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 105.9-121.8 ns)
    105.9 |#########################
    106.7 |#######
    107.5 |########################################
    108.3 |
    109.1 |
    109.9 |###
    110.7 |
    111.5 |###
    112.3 |
    113.1 |
    113.9 |
    114.7 |
    115.5 |
    116.3 |
    117.1 |
    117.9 |###
    118.7 |#######
    119.5 |#######
    120.2 |###
    121.0 |#####################
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-accfit**: CV=55.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-clamp-accfit**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: CV=54.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.78 (measurement drift or warm-up artifact)
