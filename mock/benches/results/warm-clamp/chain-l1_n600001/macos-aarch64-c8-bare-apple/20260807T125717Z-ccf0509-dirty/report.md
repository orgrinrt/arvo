# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-accfit-dyn, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-accfit-dyn (645 ns) and warm-clamp-min-lanes (647 ns) differ by 0.39%, inside the noise, even though the wider field spreads 5.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-min-lanes shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit's edge over baseline is significant but tiny (-10 ns, 1.59%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -10 ns (1.59%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 644.8 ns median (-1.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.05x (fastest 644.8 ns, slowest 677.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 743ns | 715ns | 709ns | 723ns | 837ns | base |
| warm-clamp-accfit | 727ns | 720ns | 711ns | 726ns | 746ns | -2.18% |
| warm-clamp-accfit-dyn | 713ns | 706ns | 699ns | 708ns | 739ns | -4.09% |
| warm-clamp-head | 767ns | 714ns | 709ns | 737ns | 918ns | +3.27% |
| warm-clamp-min-lanes | 714ns | 710ns | 699ns | 711ns | 740ns | -3.84% |
| warm-clamp-minimum | 751ns | 741ns | 712ns | 748ns | 798ns | +1.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 679ns | 648ns | 764ns | base | 12.067 |
| warm-clamp-accfit | 663ns | 649ns | 680ns | -2.39% | 12.363 |
| warm-clamp-accfit-dyn | 651ns | 638ns | 675ns | -4.06% | 12.578 |
| warm-clamp-head | 702ns | 646ns | 845ns | +3.35% | 11.676 |
| warm-clamp-min-lanes | 653ns | 639ns | 677ns | -3.87% | 12.553 |
| warm-clamp-minimum | 685ns | 651ns | 728ns | +0.96% | 11.952 |

## Performance model

- Peak throughput: **12.831 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 12.539 | 97.7% |
| warm-clamp-accfit | 12.474 | 97.2% |
| warm-clamp-accfit-dyn | 12.705 | 99.0% |
| warm-clamp-head | 12.551 | 97.8% |
| warm-clamp-min-lanes | 12.656 | 98.6% |
| warm-clamp-minimum | 12.099 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 743ns | 743ns | base |
| warm-clamp-accfit | 727ns | 727ns | -2.18% |
| warm-clamp-accfit-dyn | 713ns | 713ns | -4.09% |
| warm-clamp-head | 767ns | 767ns | +3.27% |
| warm-clamp-min-lanes | 714ns | 714ns | -3.84% |
| warm-clamp-minimum | 751ns | 751ns | +1.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 653ns | base | --- | [652, 674] | --- | --- | --- | --- |
| warm-clamp-accfit | 657ns | no significant difference | [-23, +21]ns | [652, 672] | no | 1.0000 | 1.0000 | 1 |
| warm-clamp-accfit-dyn | 645ns | -4.4ns (-0.7%) | [-12, -2]ns | [641, 652] | YES | 0.0009 | 0.0002 | 0 |
| warm-clamp-head | 653ns | no significant difference | [-7, +1]ns | [651, 655] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-min-lanes | 647ns | -12.7ns (-1.9%) | [-21, -9]ns | [641, 653] | YES | 0.0056 | 0.0022 | 0 |
| warm-clamp-minimum | 677ns | no significant difference | [-1, +23]ns | [675, 681] | no | 0.3352 | 0.2559 | 2 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 642ns | +4.3% | -0.1% | +21.2% | -0.5% | +13.1% |
| 2 | 641ns | +5.7% | -0.2% | +21.3% | -0.1% | +13.9% |
| 3 | 653ns | +4.0% | -1.9% | +18.7% | -1.5% | +11.5% |
| 4 | 778ns | -13.6% | -17.5% | -0.4% | -17.9% | -6.8% |
| 5 | 779ns | -15.5% | -17.7% | -0.9% | -17.5% | -7.0% |
| 6 | 777ns | -16.0% | -17.9% | +0.3% | -17.7% | -6.1% |
| 7 | 774ns | -16.1% | -17.0% | +0.3% | -17.4% | -6.0% |
| 8 | 774ns | -16.5% | -17.3% | +0.3% | -17.1% | -5.7% |
| 9 | 775ns | -16.0% | -17.5% | +0.4% | -17.3% | -5.8% |
| 10 | 778ns | -16.3% | -17.7% | -0.7% | -17.4% | -6.6% |
| 11 | 652ns | +3.1% | -1.7% | +0.2% | -1.2% | +7.9% |
| 12 | 649ns | +0.0% | -1.4% | -0.4% | -1.4% | -1.7% |
| 13 | 650ns | +6.6% | -2.0% | +0.1% | -1.6% | -1.5% |
| 14 | 651ns | +0.1% | -1.9% | -1.0% | -1.5% | -1.7% |
| 15 | 654ns | -0.5% | -2.0% | +1.0% | -2.0% | -2.2% |
| 16 | 652ns | +0.3% | -1.7% | +70.2% | -2.0% | +6.1% |
| 17 | 652ns | +2.3% | -2.3% | -1.7% | -1.8% | -0.1% |
| 18 | 654ns | -0.7% | -2.1% | +49.9% | -1.9% | +1.9% |
| 19 | 652ns | +2.6% | -1.6% | -1.0% | -2.0% | +4.6% |
| 20 | 652ns | +0.1% | -2.2% | -0.1% | -1.9% | +3.4% |
| 21 | 675ns | -3.5% | -0.6% | -3.9% | -3.1% | +0.9% |
| 22 | 672ns | -2.3% | +0.3% | -3.0% | -3.0% | +0.4% |
| 23 | 677ns | -3.9% | -0.7% | -3.7% | -3.7% | -1.7% |
| 24 | 675ns | -4.2% | -0.6% | -4.1% | -3.0% | -1.9% |
| 25 | 677ns | -3.6% | -0.5% | -3.9% | -3.6% | +0.0% |
| 26 | 674ns | -3.4% | +0.1% | -2.7% | -3.1% | +1.0% |
| 27 | 675ns | -2.7% | +0.2% | -3.3% | -3.0% | -0.3% |
| 28 | 674ns | -3.6% | +0.6% | -3.2% | -3.3% | +0.5% |
| 29 | 675ns | -3.8% | +0.4% | -3.3% | -3.6% | +0.3% |
| 30 | 674ns | -3.6% | -0.9% | -4.0% | -3.2% | +0.0% |
| 31 | 650ns | +4.3% | +0.3% | +0.4% | +4.2% | +4.4% |
| 32 | 655ns | +3.7% | -0.1% | -1.1% | +3.2% | +2.9% |
| 33 | 648ns | +4.4% | +0.8% | -0.2% | +4.0% | +4.1% |
| 34 | 650ns | +4.7% | -0.5% | +0.3% | +4.0% | +4.1% |
| 35 | 651ns | +4.0% | -0.2% | -0.2% | +4.2% | +3.8% |
| 36 | 652ns | +3.4% | -0.3% | +0.4% | +3.7% | +3.9% |
| 37 | 652ns | +3.8% | -0.3% | -0.3% | +4.1% | +3.5% |
| 38 | 653ns | +3.8% | -0.1% | -0.5% | +2.9% | +3.7% |
| 39 | 652ns | +3.6% | +0.2% | -0.4% | +3.6% | +4.0% |
| 40 | 652ns | +3.9% | -0.3% | +0.2% | +3.5% | +3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.793 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.455 | moderate+ |
| warm-clamp-accfit-dyn | 0.889 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.099 | ok |
| warm-clamp-min-lanes | 0.900 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.801 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 19/40, lost 18/40
- **warm-clamp-accfit-dyn**: won 30/40, lost 7/40
- **warm-clamp-head**: won 23/40, lost 15/40
- **warm-clamp-min-lanes**: won 29/40, lost 10/40
- **warm-clamp-minimum**: won 14/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.7ns | 678.9ns | 0.4% |  |
| warm-clamp-accfit | 3.2ns | 662.6ns | 0.5% |  |
| warm-clamp-accfit-dyn | 3.0ns | 651.3ns | 0.5% |  |
| warm-clamp-head | 2.7ns | 701.6ns | 0.4% |  |
| warm-clamp-min-lanes | 2.8ns | 652.6ns | 0.4% |  |
| warm-clamp-minimum | 3.0ns | 685.4ns | 0.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 647.9-763.9 ns)
    647.9 |########################################
    653.7 |######
    659.5 |
    665.3 |
    671.1 |#################
    676.9 |####
    682.7 |
    688.5 |
    694.3 |
    700.1 |
    705.9 |
    711.7 |
    717.5 |
    723.3 |
    729.1 |
    734.9 |
    740.7 |
    746.5 |
    752.3 |
    758.1 |
  (2 below, 7 above range)

warm-clamp-accfit (n=40, range 648.8-680.4 ns)
    648.8 |##################################
    650.3 |########################################
    651.9 |#################
    653.5 |#####
    655.1 |
    656.7 |###########
    658.2 |#####
    659.8 |
    661.4 |
    663.0 |
    664.6 |
    666.1 |#####
    667.7 |
    669.3 |###########
    670.9 |###########
    672.5 |
    674.0 |#####
    675.6 |############################
    677.2 |######################
    678.8 |#####
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 638.5-674.8 ns)
    638.5 |##################################
    640.3 |########################################
    642.1 |#################
    643.9 |
    645.7 |#####
    647.5 |#####
    649.4 |#################
    651.2 |###########
    653.0 |#################
    654.8 |
    656.6 |
    658.4 |
    660.3 |
    662.1 |
    663.9 |
    665.7 |
    667.5 |#####
    669.3 |#####
    671.2 |###########
    673.0 |#################
  (4 below, 3 above range)

warm-clamp-head (n=40, range 645.9-844.5 ns)
    645.9 |########################################
    655.8 |#
    665.8 |
    675.7 |
    685.6 |
    695.5 |
    705.5 |
    715.4 |
    725.3 |
    735.3 |
    745.2 |
    755.1 |
    765.1 |######
    775.0 |##########
    784.9 |
    794.9 |
    804.8 |
    814.7 |
    824.7 |
    834.6 |
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 639.4-676.8 ns)
    639.4 |########################################
    641.3 |##########
    643.1 |###
    645.0 |
    646.9 |
    648.8 |###
    650.6 |##########
    652.5 |#############
    654.4 |######
    656.2 |
    658.1 |
    660.0 |
    661.8 |
    663.7 |
    665.6 |
    667.4 |
    669.3 |
    671.2 |###
    673.1 |######
    674.9 |#############
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 650.5-728.5 ns)
    650.5 |###
    654.4 |
    658.3 |
    662.2 |######
    666.1 |###
    670.0 |######
    673.9 |########################################
    677.8 |################
    681.7 |###
    685.6 |
    689.5 |###
    693.4 |
    697.3 |
    701.2 |###
    705.1 |
    709.0 |
    712.9 |
    716.8 |
    720.7 |###
    724.6 |################
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.80 (measurement drift or warm-up artifact)
