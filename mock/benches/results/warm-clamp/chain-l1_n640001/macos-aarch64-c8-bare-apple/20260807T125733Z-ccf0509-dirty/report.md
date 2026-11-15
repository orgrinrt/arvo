# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (646 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.9% of the fastest

All 6 variants sit between 646 ns and 652 ns - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### warm-clamp-accfit's edge over baseline is significant but tiny (-1 ns, 0.12%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -1 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 646.0 ns median
- Spread: 1.01x (fastest 646.0 ns, slowest 652.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 708ns | 706ns | 699ns | 706ns | 723ns | base |
| warm-clamp-accfit | 722ns | 713ns | 699ns | 713ns | 772ns | +1.95% |
| warm-clamp-accfit-dyn | 747ns | 710ns | 700ns | 709ns | 907ns | +5.47% |
| warm-clamp-head | 762ns | 710ns | 698ns | 729ns | 926ns | +7.58% |
| warm-clamp-min-lanes | 710ns | 712ns | 701ns | 711ns | 717ns | +0.26% |
| warm-clamp-minimum | 718ns | 711ns | 698ns | 714ns | 748ns | +1.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 649ns | 640ns | 663ns | base | 12.631 |
| warm-clamp-accfit | 659ns | 639ns | 707ns | +1.59% | 12.433 |
| warm-clamp-accfit-dyn | 683ns | 641ns | 829ns | +5.37% | 11.987 |
| warm-clamp-head | 696ns | 638ns | 843ns | +7.28% | 11.774 |
| warm-clamp-min-lanes | 649ns | 641ns | 654ns | +0.04% | 12.626 |
| warm-clamp-minimum | 656ns | 639ns | 683ns | +1.17% | 12.484 |

## Performance model

- Peak throughput: **12.842 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 12.680 | 98.7% |
| warm-clamp-accfit | 12.636 | 98.4% |
| warm-clamp-accfit-dyn | 12.599 | 98.1% |
| warm-clamp-head | 12.615 | 98.2% |
| warm-clamp-min-lanes | 12.599 | 98.1% |
| warm-clamp-minimum | 12.562 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 708ns | 708ns | base |
| warm-clamp-accfit | 722ns | 722ns | +1.95% |
| warm-clamp-accfit-dyn | 747ns | 747ns | +5.47% |
| warm-clamp-head | 762ns | 762ns | +7.58% |
| warm-clamp-min-lanes | 710ns | 710ns | +0.26% |
| warm-clamp-minimum | 718ns | 718ns | +1.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 646ns | base | --- | [643, 651] | --- | --- | --- | --- |
| warm-clamp-accfit | 648ns | no significant difference | [-2, +6]ns | [647, 651] | no | 0.5370 | 0.3368 | 1 |
| warm-clamp-accfit-dyn | 650ns | no significant difference | [-4, +8]ns | [644, 652] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-head | 649ns | no significant difference | [-5, +10]ns | [648, 652] | no | 0.5370 | 0.3368 | 1 |
| warm-clamp-min-lanes | 650ns | no significant difference | [-1, +9]ns | [649, 651] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-minimum | 652ns | no significant difference | [-3, +13]ns | [642, 659] | no | 0.5370 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 644ns | -0.9% | +1.4% | +31.0% | +1.6% | +4.5% |
| 2 | 642ns | -0.7% | +1.0% | +31.3% | +1.4% | +4.5% |
| 3 | 642ns | -0.5% | +1.4% | +31.3% | +1.6% | +5.4% |
| 4 | 644ns | -0.7% | +0.2% | +31.1% | +0.9% | +4.5% |
| 5 | 642ns | +0.2% | +1.6% | +31.3% | +1.4% | +4.9% |
| 6 | 642ns | -0.3% | +1.3% | +31.2% | +1.5% | +5.0% |
| 7 | 639ns | -0.1% | +2.1% | +31.7% | +1.7% | +4.9% |
| 8 | 641ns | -0.3% | +1.9% | +31.9% | +1.4% | +5.2% |
| 9 | 641ns | -0.2% | +2.2% | +31.2% | +2.2% | +4.8% |
| 10 | 640ns | +0.2% | +1.8% | +31.4% | +1.4% | +5.7% |
| 11 | 645ns | +15.1% | -0.7% | +1.7% | +1.0% | +2.2% |
| 12 | 643ns | +1.3% | -0.3% | +0.5% | +1.4% | +15.4% |
| 13 | 645ns | +0.5% | -0.7% | +0.7% | +0.8% | +1.7% |
| 14 | 642ns | +0.5% | +0.2% | +0.8% | +1.6% | -0.1% |
| 15 | 638ns | +2.5% | +9.7% | +1.9% | +1.9% | +0.5% |
| 16 | 640ns | +4.2% | +43.9% | +1.5% | +1.4% | +0.6% |
| 17 | 640ns | +1.2% | +44.9% | +1.4% | +1.8% | +0.1% |
| 18 | 640ns | +1.2% | +44.8% | +1.3% | +1.8% | +0.3% |
| 19 | 640ns | +1.1% | +44.5% | +2.4% | +2.0% | +0.1% |
| 20 | 644ns | +0.9% | +43.6% | +1.3% | +1.1% | -0.2% |
| 21 | 673ns | -3.8% | -4.6% | -5.2% | -4.6% | -2.5% |
| 22 | 649ns | +0.2% | -1.5% | -1.5% | +1.0% | +0.2% |
| 23 | 653ns | -0.8% | -2.0% | -2.0% | +0.6% | +0.1% |
| 24 | 650ns | +28.2% | -1.2% | -1.7% | +0.6% | +0.3% |
| 25 | 655ns | +1.9% | -1.8% | -2.5% | -0.1% | -0.7% |
| 26 | 653ns | +11.2% | -1.7% | -2.4% | -0.1% | +2.6% |
| 27 | 651ns | -0.6% | -1.2% | -2.0% | -0.1% | +0.1% |
| 28 | 650ns | -0.3% | -1.2% | -2.3% | -0.2% | +2.4% |
| 29 | 666ns | -2.9% | -3.4% | -3.4% | -2.6% | -1.2% |
| 30 | 676ns | -4.1% | -5.0% | -5.6% | -3.9% | -2.6% |
| 31 | 658ns | +2.9% | -0.5% | -1.3% | -2.2% | -2.5% |
| 32 | 665ns | -2.3% | -2.4% | -2.3% | -2.7% | -4.0% |
| 33 | 655ns | +2.5% | -0.9% | -1.1% | -2.0% | -2.5% |
| 34 | 650ns | +0.3% | +1.1% | +0.2% | -1.4% | -1.4% |
| 35 | 649ns | +0.9% | +0.4% | +0.1% | -0.8% | -1.1% |
| 36 | 651ns | +0.0% | +0.4% | +0.0% | -1.7% | -1.8% |
| 37 | 647ns | +0.6% | +0.4% | +3.5% | -1.0% | -1.3% |
| 38 | 652ns | -0.3% | +0.1% | -0.4% | -1.7% | -2.0% |
| 39 | 653ns | +2.1% | -0.8% | -0.9% | -1.7% | -1.8% |
| 40 | 655ns | +2.9% | -0.6% | -0.5% | -2.5% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.605 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.034 | ok |
| warm-clamp-accfit-dyn | 0.799 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.897 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.654 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.435 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 16/40, lost 23/40
- **warm-clamp-accfit-dyn**: won 18/40, lost 21/40
- **warm-clamp-head**: won 16/40, lost 22/40
- **warm-clamp-min-lanes**: won 17/40, lost 23/40
- **warm-clamp-minimum**: won 15/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.6ns | 648.6ns | 0.4% |  |
| warm-clamp-accfit | 2.5ns | 658.9ns | 0.4% |  |
| warm-clamp-accfit-dyn | 2.9ns | 683.4ns | 0.4% |  |
| warm-clamp-head | 2.9ns | 695.8ns | 0.4% |  |
| warm-clamp-min-lanes | 2.4ns | 648.8ns | 0.4% |  |
| warm-clamp-minimum | 2.6ns | 656.2ns | 0.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 639.6-662.8 ns)
    639.6 |################
    640.8 |########################
    642.0 |########################################
    643.1 |########################
    644.3 |################
    645.4 |
    646.6 |########
    647.7 |################
    648.9 |########
    650.1 |################################
    651.2 |########
    652.4 |########################
    653.5 |########
    654.7 |################
    655.9 |
    657.0 |########
    658.2 |
    659.3 |
    660.5 |
    661.6 |
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 638.6-707.4 ns)
    638.6 |##################
    642.1 |###
    645.5 |########################################
    649.0 |#############################
    652.4 |#######
    655.8 |
    659.3 |
    662.7 |
    666.2 |##########
    669.6 |###
    673.0 |###
    676.5 |###
    679.9 |
    683.4 |
    686.8 |
    690.2 |
    693.7 |
    697.1 |
    700.5 |
    704.0 |
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 641.0-829.3 ns)
    641.0 |########################################
    650.4 |############################
    659.8 |
    669.2 |
    678.7 |
    688.1 |
    697.5 |##
    706.9 |
    716.3 |
    725.7 |
    735.2 |
    744.6 |
    754.0 |
    763.4 |
    772.8 |
    782.2 |
    791.7 |
    801.1 |
    810.5 |
    819.9 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 637.9-843.1 ns)
    637.9 |###############################
    648.2 |########################################
    658.4 |
    668.7 |##
    679.0 |
    689.2 |
    699.5 |
    709.7 |
    720.0 |
    730.3 |
    740.5 |
    750.8 |
    761.0 |
    771.3 |
    781.6 |
    791.8 |
    802.1 |
    812.3 |
    822.6 |
    832.9 |#################
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 640.8-654.3 ns)
    640.8 |#############
    641.5 |####################
    642.1 |
    642.8 |######
    643.5 |######
    644.2 |
    644.8 |
    645.5 |
    646.2 |
    646.9 |######
    647.6 |
    648.2 |#############
    648.9 |####################
    649.6 |##########################
    650.3 |########################################
    650.9 |######
    651.6 |#################################
    652.3 |#############
    653.0 |
    653.6 |#############
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 639.2-682.9 ns)
    639.2 |########################################
    641.4 |######################
    643.6 |#####
    645.8 |
    648.0 |###########
    650.1 |###########
    652.3 |#####
    654.5 |###########
    656.7 |###########
    658.9 |#####
    661.1 |
    663.2 |
    665.4 |#####
    667.6 |
    669.8 |######################
    672.0 |#################
    674.1 |#################
    676.3 |#####
    678.5 |
    680.7 |
  (5 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.65 (measurement drift or warm-up artifact)
