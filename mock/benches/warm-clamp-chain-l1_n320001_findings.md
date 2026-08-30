# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 52% (significant)

warm-clamp-min-lanes is -336 ns (52%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-head is an outlier: 2.1x slower than the field

warm-clamp-head (653 ns) is 2.1x the fastest (315 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (315 ns) and warm-clamp-min-lanes (315 ns) differ by 0.13%, inside the noise, even though the wider field spreads 107.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows warm-up / thermal drift (autocorr +0.85)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} (104% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} with a 104% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit's edge over baseline is significant but tiny (-4 ns, 0.64%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -4 ns (0.64%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 314.6 ns median (-51.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.07x (fastest 314.6 ns, slowest 652.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 734ns | 710ns | 699ns | 718ns | 817ns | base |
| warm-clamp-accfit | 713ns | 702ns | 699ns | 704ns | 755ns | -2.77% |
| warm-clamp-accfit-dyn | 760ns | 708ns | 700ns | 726ns | 922ns | +3.60% |
| warm-clamp-head | 750ns | 713ns | 706ns | 731ns | 851ns | +2.26% |
| warm-clamp-min-lanes | 386ns | 376ns | 372ns | 376ns | 430ns | -47.36% |
| warm-clamp-minimum | 379ns | 375ns | 372ns | 376ns | 392ns | -48.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 670ns | 638ns | 746ns | base | 12.220 |
| warm-clamp-accfit | 651ns | 639ns | 688ns | -2.89% | 12.583 |
| warm-clamp-accfit-dyn | 694ns | 639ns | 843ns | +3.58% | 11.798 |
| warm-clamp-head | 686ns | 646ns | 778ns | +2.26% | 11.950 |
| warm-clamp-min-lanes | 324ns | 313ns | 363ns | -51.60% | 25.250 |
| warm-clamp-minimum | 318ns | 312ns | 330ns | -52.58% | 25.770 |

## Performance model

- Peak throughput: **26.272 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 12.615 | 48.0% |
| warm-clamp-accfit | 12.766 | 48.6% |
| warm-clamp-accfit-dyn | 12.693 | 48.3% |
| warm-clamp-head | 12.551 | 47.8% |
| warm-clamp-min-lanes | 26.006 | 99.0% |
| warm-clamp-minimum | 26.039 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 734ns | 734ns | base |
| warm-clamp-accfit | 713ns | 713ns | -2.77% |
| warm-clamp-accfit-dyn | 760ns | 760ns | +3.60% |
| warm-clamp-head | 750ns | 750ns | +2.26% |
| warm-clamp-min-lanes | 386ns | 386ns | -47.36% |
| warm-clamp-minimum | 379ns | 379ns | -48.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 649ns | base | --- | [641, 658] | --- | --- | --- | --- |
| warm-clamp-accfit | 642ns | no significant difference | [-6, +1]ns | [641, 643] | no | 0.5224 | 0.5224 | 1 |
| warm-clamp-accfit-dyn | 645ns | +2.1ns (+0.3%) | [+1, +4]ns | [642, 651] | YES | 0.0296 | 0.0237 | 1 |
| warm-clamp-head | 653ns | +9.8ns (+1.5%) | [+6, +13]ns | [650, 656] | YES | 0.0018 | 0.0011 | 1 |
| warm-clamp-min-lanes | 315ns | -328.4ns (-50.6%) | [-337, -326]ns | [315, 317] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 315ns | -334.6ns (-51.5%) | [-339, -326]ns | [313, 316] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 651ns | -0.2% | -0.3% | -0.2% | -51.4% | -51.6% |
| 2 | 652ns | -0.3% | -0.6% | -0.6% | -51.5% | -52.3% |
| 3 | 672ns | -2.5% | -2.5% | -3.2% | -52.9% | -53.8% |
| 4 | 653ns | -1.9% | +3.4% | -0.4% | -52.2% | -51.8% |
| 5 | 649ns | +0.6% | +2.5% | +0.0% | -51.6% | -51.9% |
| 6 | 646ns | -0.8% | +0.4% | +1.0% | -51.3% | -51.3% |
| 7 | 650ns | +6.4% | -0.1% | +2.4% | -52.0% | -51.9% |
| 8 | 651ns | +0.1% | -0.2% | -0.4% | -51.7% | -52.0% |
| 9 | 673ns | -3.3% | -3.8% | +15.8% | -52.1% | -53.1% |
| 10 | 651ns | -0.6% | +0.3% | -0.8% | -50.1% | -51.7% |
| 11 | 639ns | +0.5% | +0.6% | +1.2% | -51.3% | -48.1% |
| 12 | 638ns | +0.7% | +0.1% | +0.3% | -50.9% | -48.4% |
| 13 | 644ns | -0.5% | -1.0% | +0.6% | -51.1% | -49.2% |
| 14 | 639ns | +0.4% | +0.3% | +0.4% | -50.7% | -48.5% |
| 15 | 640ns | +0.3% | +0.1% | +1.4% | -51.0% | -49.3% |
| 16 | 642ns | -0.1% | -0.2% | +1.9% | -51.0% | -50.7% |
| 17 | 640ns | +0.1% | +0.3% | +2.1% | -44.6% | -48.7% |
| 18 | 639ns | +0.3% | +0.3% | +2.6% | -51.1% | -48.2% |
| 19 | 667ns | -4.1% | -4.3% | -1.9% | -52.3% | -50.5% |
| 20 | 662ns | -2.9% | -3.5% | -1.4% | -52.4% | -50.4% |
| 21 | 767ns | -16.4% | +9.6% | +1.7% | -45.9% | -58.1% |
| 22 | 729ns | -12.1% | +15.0% | +6.7% | -43.0% | -56.7% |
| 23 | 724ns | -11.7% | +16.1% | +7.2% | -43.7% | -56.2% |
| 24 | 797ns | -7.3% | +5.5% | -3.3% | -57.4% | -58.5% |
| 25 | 728ns | -11.7% | +16.0% | +6.6% | -56.7% | -55.9% |
| 26 | 771ns | -17.1% | +9.4% | +0.8% | -58.7% | -59.6% |
| 27 | 725ns | +2.7% | +16.3% | +7.4% | -56.3% | -56.9% |
| 28 | 727ns | -12.1% | +15.5% | +6.6% | -56.6% | -56.9% |
| 29 | 728ns | -1.0% | +15.9% | +6.6% | -56.1% | -57.0% |
| 30 | 725ns | -11.1% | +16.3% | +7.0% | -55.5% | -56.9% |
| 31 | 640ns | -0.3% | +0.5% | +2.5% | -51.0% | -50.7% |
| 32 | 641ns | +0.3% | -0.2% | +2.2% | -50.9% | -51.0% |
| 33 | 640ns | -0.3% | +0.5% | +4.4% | -50.5% | -50.9% |
| 34 | 639ns | +0.4% | +0.3% | +1.6% | -51.0% | -51.0% |
| 35 | 640ns | +0.1% | +0.5% | +1.3% | -50.4% | -51.2% |
| 36 | 638ns | +0.0% | +0.4% | +1.7% | -51.0% | -51.0% |
| 37 | 636ns | +1.3% | +0.9% | +2.0% | -50.7% | -50.8% |
| 38 | 641ns | +0.1% | -0.1% | +1.2% | -50.9% | -51.3% |
| 39 | 642ns | +0.1% | +0.1% | +1.8% | -51.0% | -50.8% |
| 40 | 639ns | +0.1% | +0.0% | +1.4% | -50.7% | -50.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.756 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | -0.133 | ok |
| warm-clamp-accfit-dyn | 0.851 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.725 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.649 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.709 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 22/40, lost 16/40
- **warm-clamp-accfit-dyn**: won 10/40, lost 27/40
- **warm-clamp-head**: won 9/40, lost 30/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.3ns | 670.4ns | 0.5% |  |
| warm-clamp-accfit | 2.5ns | 651.0ns | 0.4% |  |
| warm-clamp-accfit-dyn | 2.9ns | 694.4ns | 0.4% |  |
| warm-clamp-head | 2.9ns | 685.5ns | 0.4% |  |
| warm-clamp-min-lanes | 2.5ns | 324.4ns | 0.8% |  |
| warm-clamp-minimum | 2.4ns | 317.9ns | 0.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 638.3-746.5 ns)
    638.3 |########################################
    643.7 |#####
    649.1 |####################
    654.5 |
    659.9 |##
    665.3 |##
    670.7 |#####
    676.2 |
    681.6 |
    687.0 |
    692.4 |
    697.8 |
    703.2 |
    708.6 |
    714.0 |
    719.4 |##
    724.8 |#################
    730.2 |
    735.6 |
    741.0 |
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 638.9-688.3 ns)
    638.9 |########################################
    641.4 |#################################
    643.9 |######
    646.3 |###
    648.8 |##########
    651.3 |######
    653.7 |###
    656.2 |
    658.7 |
    661.1 |
    663.6 |
    666.1 |
    668.6 |
    671.0 |
    673.5 |
    676.0 |
    678.4 |
    680.9 |
    683.4 |
    685.8 |
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 639.1-842.5 ns)
    639.1 |########################################
    649.3 |######
    659.5 |##
    669.6 |##
    679.8 |
    690.0 |
    700.1 |
    710.3 |
    720.5 |
    730.6 |
    740.8 |
    751.0 |
    761.1 |
    771.3 |
    781.5 |
    791.7 |
    801.8 |
    812.0 |
    822.2 |
    832.3 |##########
  (5 below, 5 above range)

warm-clamp-head (n=40, range 645.6-777.6 ns)
    645.6 |########################################
    652.2 |##################
    658.8 |##
    665.4 |##
    672.0 |
    678.6 |
    685.2 |
    691.8 |
    698.4 |
    705.0 |
    711.6 |
    718.2 |
    724.8 |
    731.4 |
    738.0 |
    744.6 |
    751.2 |
    757.8 |
    764.4 |##
    771.0 |################
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 312.6-362.6 ns)
    312.6 |########################################
    315.1 |##################
    317.6 |#######
    320.1 |####
    322.6 |##
    325.1 |
    327.6 |
    330.1 |
    332.6 |
    335.1 |
    337.6 |##
    340.1 |
    342.6 |
    345.1 |
    347.6 |
    350.1 |
    352.6 |##
    355.1 |
    357.6 |
    360.1 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 311.8-329.8 ns)
    311.8 |########################################
    312.7 |############################
    313.6 |######################
    314.5 |##################################
    315.4 |#####
    316.3 |#####
    317.2 |#####
    318.1 |
    319.0 |
    319.9 |#####
    320.8 |#####
    321.7 |
    322.6 |
    323.5 |
    324.4 |#####
    325.3 |
    326.2 |
    327.1 |#####
    328.0 |###########
    328.9 |###########
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.71 (measurement drift or warm-up artifact)
