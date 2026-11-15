# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 2.2x slower than the field

warm-clamp-accfit-dyn (1.16 us) is 2.2x the fastest (524 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.81)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64} vs {warm-clamp-min-lanes, warm-clamp-accfit-dyn} (89% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64} and a slow tier {warm-clamp-min-lanes, warm-clamp-accfit-dyn} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-minimum** at 524.4 ns median (-11.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.21x (fastest 524.4 ns, slowest 1159.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 700ns | 659ns | 641ns | 662ns | 874ns | base |
| warm-clamp-accfit | 637ns | 599ns | 578ns | 598ns | 812ns | -9.01% |
| warm-clamp-accfit-dyn | 1219ns | 1224ns | 1190ns | 1220ns | 1247ns | +74.13% |
| warm-clamp-head | 597ns | 600ns | 577ns | 599ns | 610ns | -14.76% |
| warm-clamp-min-lanes | 1279ns | 1188ns | 1146ns | 1185ns | 1692ns | +82.65% |
| warm-clamp-minimum | 589ns | 585ns | 576ns | 587ns | 609ns | -15.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 633ns | 579ns | 792ns | base | 12.944 |
| warm-clamp-accfit | 570ns | 518ns | 726ns | -9.91% | 14.367 |
| warm-clamp-accfit-dyn | 1156ns | 1129ns | 1179ns | +82.58% | 7.089 |
| warm-clamp-head | 533ns | 516ns | 544ns | -15.83% | 15.378 |
| warm-clamp-min-lanes | 1186ns | 1088ns | 1471ns | +87.37% | 6.908 |
| warm-clamp-minimum | 527ns | 516ns | 543ns | -16.70% | 15.539 |

## Performance model

- Peak throughput: **15.882 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 13.763 | 86.7% |
| warm-clamp-accfit | 15.288 | 96.3% |
| warm-clamp-accfit-dyn | 7.063 | 44.5% |
| warm-clamp-head | 15.301 | 96.3% |
| warm-clamp-min-lanes | 7.272 | 45.8% |
| warm-clamp-minimum | 15.622 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 700ns | 700ns | base |
| warm-clamp-accfit | 637ns | 637ns | -9.01% |
| warm-clamp-accfit-dyn | 1219ns | 1219ns | +74.13% |
| warm-clamp-head | 597ns | 597ns | -14.76% |
| warm-clamp-min-lanes | 1279ns | 1279ns | +82.65% |
| warm-clamp-minimum | 589ns | 589ns | -15.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 595ns | base | --- | [592, 605] | --- | --- | --- | --- |
| warm-clamp-accfit | 536ns | -66.2ns (-11.1%) | [-73, -55]ns | [527, 545] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1160ns | +560.4ns (+94.2%) | [+540, +572]ns | [1139, 1171] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 535ns | -61.8ns (-10.4%) | [-72, -57]ns | [534, 537] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1126ns | +534.4ns (+89.8%) | [+515, +547]ns | [1125, 1128] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 524ns | -72.9ns (-12.2%) | [-82, -69]ns | [523, 526] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 608ns | -9.9% | +93.4% | -11.6% | +85.4% | -13.3% |
| 2 | 610ns | -10.7% | +92.2% | -12.6% | +84.8% | -13.6% |
| 3 | 1288ns | -57.6% | -8.3% | -58.8% | -12.1% | -59.3% |
| 4 | 1344ns | -59.3% | -12.7% | -60.0% | -16.3% | -61.0% |
| 5 | 614ns | -11.5% | +91.4% | -12.6% | +83.6% | -14.8% |
| 6 | 605ns | -9.8% | +93.9% | -11.4% | +86.1% | -13.6% |
| 7 | 599ns | -8.4% | +96.0% | -9.9% | +88.7% | -12.1% |
| 8 | 594ns | -7.5% | +97.6% | -10.2% | +89.3% | -11.5% |
| 9 | 588ns | -7.1% | +99.9% | -9.1% | +91.2% | -10.3% |
| 10 | 591ns | -8.2% | +98.1% | -9.7% | +90.6% | -11.3% |
| 11 | 595ns | -11.5% | +90.0% | -10.1% | +83.2% | -11.9% |
| 12 | 597ns | -11.9% | +90.1% | -10.5% | +81.9% | -12.5% |
| 13 | 595ns | -11.2% | +90.1% | -9.6% | +83.2% | -11.6% |
| 14 | 593ns | -10.7% | +91.4% | -10.3% | +83.9% | -11.6% |
| 15 | 595ns | -11.5% | +94.0% | -9.4% | +82.4% | -12.2% |
| 16 | 611ns | -13.6% | +88.9% | -12.4% | +78.0% | -14.3% |
| 17 | 612ns | -14.2% | +86.2% | -11.7% | +78.0% | -14.2% |
| 18 | 606ns | -13.2% | +86.8% | -11.8% | +79.1% | -13.8% |
| 19 | 593ns | -11.1% | +90.7% | -9.9% | +83.4% | -11.6% |
| 20 | 602ns | -12.3% | +87.6% | -10.9% | +81.3% | -13.3% |
| 21 | 623ns | -17.0% | +81.2% | -17.4% | +113.1% | -17.2% |
| 22 | 625ns | -17.4% | +80.6% | -17.0% | +111.1% | -17.8% |
| 23 | 618ns | -15.9% | +82.6% | -16.1% | +111.2% | -16.7% |
| 24 | 611ns | -15.4% | +89.9% | -15.3% | +247.7% | -15.5% |
| 25 | 612ns | -15.2% | +84.0% | -7.1% | +103.2% | -16.0% |
| 26 | 614ns | -15.6% | +95.6% | -16.2% | +82.8% | -15.1% |
| 27 | 591ns | -12.1% | +93.2% | -12.7% | +90.4% | -11.4% |
| 28 | 594ns | -12.8% | +90.7% | -13.1% | +125.4% | -13.6% |
| 29 | 600ns | -13.6% | +94.7% | -14.4% | +191.6% | -12.4% |
| 30 | 591ns | -10.8% | +97.9% | -12.3% | +129.6% | -12.4% |
| 31 | 580ns | -5.7% | +102.2% | -7.0% | +94.5% | -6.1% |
| 32 | 582ns | -6.4% | +99.0% | -6.8% | +93.6% | -7.0% |
| 33 | 580ns | -6.1% | +99.8% | -6.3% | +94.3% | -6.2% |
| 34 | 579ns | +81.0% | +101.9% | -7.2% | +94.9% | -6.1% |
| 35 | 579ns | +137.2% | +102.2% | -7.6% | +94.7% | -6.5% |
| 36 | 580ns | +12.1% | +102.4% | -7.5% | +94.2% | -6.5% |
| 37 | 580ns | -6.0% | +101.5% | -7.4% | +94.3% | -6.3% |
| 38 | 579ns | -5.8% | +103.7% | -7.1% | +94.6% | -6.3% |
| 39 | 582ns | -6.6% | +94.3% | -8.0% | +93.8% | -7.0% |
| 40 | 577ns | -5.9% | +97.3% | -7.1% | +95.1% | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.495 | moderate+ |
| warm-clamp-accfit | 0.521 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.442 | moderate+ |
| warm-clamp-head | 0.207 | moderate+ |
| warm-clamp-min-lanes | 0.341 | moderate+ |
| warm-clamp-minimum | 0.805 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 37/40, lost 3/40
- **warm-clamp-accfit-dyn**: won 2/40, lost 38/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 2/40, lost 38/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.2ns | 632.9ns | 0.5% |  |
| warm-clamp-accfit | 3.2ns | 570.2ns | 0.6% |  |
| warm-clamp-accfit-dyn | 2.3ns | 1155.6ns | 0.2% |  |
| warm-clamp-head | 2.6ns | 532.7ns | 0.5% |  |
| warm-clamp-min-lanes | 2.7ns | 1185.9ns | 0.2% |  |
| warm-clamp-minimum | 3.1ns | 527.2ns | 0.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 579.2-792.1 ns)
    579.2 |#####################
    589.9 |########################################
    600.5 |###############
    611.2 |#####################
    621.8 |######
    632.5 |
    643.1 |
    653.8 |
    664.4 |
    675.0 |
    685.7 |
    696.3 |
    707.0 |
    717.6 |
    728.3 |
    738.9 |
    749.6 |
    760.2 |
    770.9 |
    781.5 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 517.9-726.4 ns)
    517.9 |###################################
    528.3 |#######
    538.7 |########################################
    549.1 |##
    559.6 |
    570.0 |
    580.4 |
    590.8 |
    601.3 |
    611.7 |
    622.1 |
    632.5 |
    643.0 |##
    653.4 |
    663.8 |
    674.2 |
    684.7 |
    695.1 |
    705.5 |
    715.9 |
  (3 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1129.2-1179.3 ns)
   1129.2 |########################################
   1131.7 |###########
   1134.2 |#####
   1136.7 |#####
   1139.2 |###########
   1141.7 |
   1144.2 |
   1146.8 |
   1149.3 |
   1151.8 |
   1154.3 |###########
   1156.8 |###########
   1159.3 |#####
   1161.8 |
   1164.3 |
   1166.8 |######################
   1169.3 |###########
   1171.8 |########################################
   1174.3 |#################
   1176.8 |#####
  (3 below, 2 above range)

warm-clamp-head (n=40, range 516.0-544.0 ns)
    516.0 |#####
    517.4 |####################
    518.8 |
    520.2 |
    521.6 |
    523.0 |
    524.4 |
    525.8 |
    527.2 |
    528.6 |
    530.0 |#####
    531.4 |#####
    532.8 |###############
    534.2 |########################################
    535.6 |####################
    537.0 |###################################
    538.4 |###############
    539.8 |#####
    541.2 |
    542.6 |##########
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 1087.7-1470.6 ns)
   1087.7 |#################
   1106.8 |###############
   1126.0 |########################################
   1145.1 |
   1164.3 |
   1183.4 |
   1202.6 |
   1221.7 |
   1240.8 |##
   1260.0 |
   1279.1 |
   1298.3 |##
   1317.4 |#####
   1336.6 |##
   1355.7 |##
   1374.8 |
   1394.0 |
   1413.1 |
   1432.3 |
   1451.4 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 515.8-543.0 ns)
    515.8 |#####
    517.2 |#####
    518.5 |
    519.9 |#####
    521.2 |######################
    522.6 |########################################
    524.0 |######################
    525.3 |######################
    526.7 |#################
    528.0 |
    529.4 |
    530.8 |
    532.1 |
    533.5 |
    534.8 |
    536.2 |
    537.5 |
    538.9 |
    540.3 |###########
    541.6 |######################
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=24.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=26.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.81 (measurement drift or warm-up artifact)
