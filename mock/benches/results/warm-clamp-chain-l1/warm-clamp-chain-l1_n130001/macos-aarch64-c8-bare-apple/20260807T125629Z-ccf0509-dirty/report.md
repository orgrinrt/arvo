# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 52% (significant)

warm-clamp-min-lanes is -164 ns (52%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit is an outlier: 2.5x slower than the field

warm-clamp-accfit (374 ns) is 2.5x the fastest (151 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-min-lanes, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-min-lanes (151 ns) and warm-clamp-minimum (151 ns) differ by 0.00%, inside the noise, even though the wider field spreads 146.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.89)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit} (107% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit} with a 107% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-head is inconsistent: worst-20% is 1.5x its best-20%

warm-clamp-head's best 20% of batches run at 312 ns but its worst 20% at 478 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit's edge over baseline is significant but tiny (-0 ns, 0.06%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -0 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 151.4 ns median (-51.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.47x (fastest 151.4 ns, slowest 373.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 415ns | 375ns | 372ns | 388ns | 538ns | base |
| warm-clamp-accfit | 437ns | 451ns | 372ns | 437ns | 500ns | +5.27% |
| warm-clamp-accfit-dyn | 386ns | 377ns | 371ns | 377ns | 430ns | -6.84% |
| warm-clamp-head | 450ns | 422ns | 371ns | 437ns | 570ns | +8.57% |
| warm-clamp-min-lanes | 211ns | 211ns | 208ns | 211ns | 216ns | -49.05% |
| warm-clamp-minimum | 216ns | 212ns | 209ns | 213ns | 234ns | -47.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 348ns | 311ns | 451ns | base | 23.557 |
| warm-clamp-accfit | 362ns | 312ns | 413ns | +4.14% | 22.619 |
| warm-clamp-accfit-dyn | 324ns | 311ns | 361ns | -6.83% | 25.283 |
| warm-clamp-head | 379ns | 312ns | 478ns | +8.89% | 21.633 |
| warm-clamp-min-lanes | 152ns | 148ns | 156ns | -56.41% | 54.045 |
| warm-clamp-minimum | 155ns | 149ns | 172ns | -55.38% | 52.792 |

## Performance model

- Peak throughput: **55.370 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 26.073 | 47.1% |
| warm-clamp-accfit | 21.907 | 39.6% |
| warm-clamp-accfit-dyn | 25.924 | 46.8% |
| warm-clamp-head | 23.050 | 41.6% |
| warm-clamp-min-lanes | 54.090 | 97.7% |
| warm-clamp-minimum | 54.090 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 415ns | 415ns | base |
| warm-clamp-accfit | 437ns | 437ns | +5.27% |
| warm-clamp-accfit-dyn | 386ns | 386ns | -6.84% |
| warm-clamp-head | 450ns | 450ns | +8.57% |
| warm-clamp-min-lanes | 211ns | 211ns | -49.05% |
| warm-clamp-minimum | 216ns | 216ns | -47.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 314ns | base | --- | [313, 315] | --- | --- | --- | --- |
| warm-clamp-accfit | 374ns | no significant difference | [-1, +54]ns | [315, 411] | no | 0.7493 | 0.7493 | 1 |
| warm-clamp-accfit-dyn | 316ns | no significant difference | [-1, +4]ns | [314, 318] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-head | 355ns | +2.1ns (+0.7%) | [+1, +38]ns | [316, 406] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-min-lanes | 151ns | -164.6ns (-52.4%) | [-165, -162]ns | [150, 152] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 151ns | -163.8ns (-52.1%) | [-166, -162]ns | [151, 153] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 450ns | -8.7% | -30.8% | +0.4% | -66.1% | -66.8% |
| 2 | 450ns | -8.7% | -30.4% | +0.3% | -66.5% | -65.9% |
| 3 | 450ns | -8.6% | -30.2% | +1.1% | -65.5% | -66.4% |
| 4 | 452ns | -8.5% | -30.9% | -0.6% | -64.8% | -65.0% |
| 5 | 448ns | -8.2% | -29.4% | +0.6% | -65.4% | -65.5% |
| 6 | 451ns | -9.0% | -31.1% | +0.2% | -65.8% | -66.1% |
| 7 | 451ns | -18.0% | -30.7% | -0.6% | -66.0% | -66.2% |
| 8 | 450ns | -30.2% | -31.1% | +0.1% | -66.2% | -66.0% |
| 9 | 452ns | -30.7% | -30.7% | -0.3% | -65.3% | -64.9% |
| 10 | 451ns | -31.1% | -29.6% | +0.3% | -67.3% | -38.1% |
| 11 | 315ns | +0.1% | +31.5% | -0.7% | -53.0% | -52.8% |
| 12 | 315ns | -0.4% | +31.2% | -0.1% | -51.1% | -50.7% |
| 13 | 313ns | -0.5% | +32.2% | +0.1% | -51.8% | -50.3% |
| 14 | 313ns | -0.8% | +11.0% | +1.1% | -51.7% | -51.6% |
| 15 | 315ns | -0.3% | +1.7% | -0.8% | -52.6% | -50.1% |
| 16 | 311ns | -0.1% | +0.8% | +0.4% | -50.9% | -49.5% |
| 17 | 312ns | +0.1% | +1.1% | +0.8% | -52.8% | -51.7% |
| 18 | 314ns | -0.2% | +1.1% | -0.8% | -52.6% | -52.1% |
| 19 | 310ns | +1.7% | +2.8% | +0.9% | -52.0% | -51.2% |
| 20 | 315ns | -0.1% | +2.3% | -1.2% | -51.7% | -52.3% |
| 21 | 312ns | +32.5% | +0.9% | +13.5% | -50.0% | -51.9% |
| 22 | 312ns | +32.2% | +2.1% | +13.6% | -51.3% | -51.7% |
| 23 | 317ns | +30.7% | +1.0% | +12.2% | -52.0% | -51.9% |
| 24 | 316ns | +30.7% | +5.4% | +12.3% | -52.1% | -52.2% |
| 25 | 310ns | +33.4% | +3.8% | +17.0% | -51.8% | -51.0% |
| 26 | 315ns | +30.5% | +0.9% | +14.0% | -51.6% | -53.2% |
| 27 | 312ns | +31.3% | +2.7% | +13.9% | -52.1% | -52.3% |
| 28 | 312ns | +31.6% | +1.6% | +15.1% | -52.4% | -51.7% |
| 29 | 315ns | +30.8% | +1.5% | +12.0% | -52.6% | -52.7% |
| 30 | 313ns | +32.0% | +1.2% | +13.5% | -53.0% | -52.1% |
| 31 | 312ns | +0.0% | -0.1% | +62.1% | -51.7% | -50.3% |
| 32 | 313ns | +0.9% | -0.1% | +60.6% | -52.9% | -51.5% |
| 33 | 313ns | -0.8% | +0.9% | +61.1% | -51.5% | -50.5% |
| 34 | 312ns | +1.4% | +1.3% | +61.1% | -49.9% | -50.9% |
| 35 | 314ns | +0.1% | -0.5% | +13.4% | -52.6% | -52.5% |
| 36 | 314ns | +23.9% | -0.3% | +0.1% | -50.7% | -52.7% |
| 37 | 313ns | +14.4% | -0.7% | +0.3% | -51.5% | -52.4% |
| 38 | 316ns | +19.7% | -1.3% | -1.6% | -52.1% | -53.5% |
| 39 | 315ns | +20.1% | -1.2% | -0.8% | -51.5% | -51.3% |
| 40 | 314ns | +20.7% | +0.6% | +0.3% | -52.5% | -51.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.893 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.797 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.724 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.792 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.205 | moderate+ |
| warm-clamp-minimum | 0.007 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 18/40, lost 21/40
- **warm-clamp-accfit-dyn**: won 17/40, lost 23/40
- **warm-clamp-head**: won 10/40, lost 29/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 347.8ns | 0.7% |  |
| warm-clamp-accfit | 2.7ns | 362.2ns | 0.7% |  |
| warm-clamp-accfit-dyn | 2.4ns | 324.0ns | 0.8% |  |
| warm-clamp-head | 2.9ns | 378.7ns | 0.8% |  |
| warm-clamp-min-lanes | 2.4ns | 151.6ns | 1.6% |  |
| warm-clamp-minimum | 2.6ns | 155.2ns | 1.7% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 311.5-451.1 ns)
    311.5 |########################################
    318.4 |
    325.4 |
    332.4 |
    339.4 |
    346.4 |
    353.3 |
    360.3 |
    367.3 |
    374.3 |
    381.3 |
    388.2 |
    395.2 |
    402.2 |
    409.2 |
    416.2 |
    423.2 |
    430.1 |
    437.1 |
    444.1 |##########
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 311.6-413.4 ns)
    311.6 |########################################
    316.7 |
    321.8 |
    326.9 |
    332.0 |
    337.1 |
    342.1 |
    347.2 |
    352.3 |
    357.4 |###
    362.5 |
    367.6 |###
    372.7 |
    377.8 |#########
    382.9 |
    388.0 |###
    393.1 |
    398.2 |
    403.3 |
    408.4 |####################################
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 311.2-360.8 ns)
    311.2 |########################################
    313.7 |##########################
    316.2 |########################################
    318.7 |#################
    321.2 |########
    323.6 |
    326.1 |
    328.6 |
    331.1 |####
    333.6 |
    336.0 |
    338.5 |
    341.0 |
    343.5 |
    345.9 |####
    348.4 |
    350.9 |
    353.4 |
    355.9 |
    358.3 |
  (5 below, 3 above range)

warm-clamp-head (n=40, range 312.0-478.2 ns)
    312.0 |########################################
    320.3 |
    328.6 |
    336.9 |
    345.2 |###
    353.5 |##############################
    361.8 |###
    370.2 |
    378.5 |
    386.8 |
    395.1 |
    403.4 |
    411.7 |
    420.0 |
    428.4 |
    436.7 |
    445.0 |##############################
    453.3 |###
    461.6 |
    469.9 |
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 148.0-155.9 ns)
    148.0 |
    148.3 |################################
    148.7 |
    149.1 |################
    149.5 |########################
    149.9 |
    150.3 |
    150.7 |########################
    151.1 |########################
    151.5 |########
    151.9 |########################################
    152.3 |################
    152.7 |################
    153.1 |########
    153.5 |########
    153.9 |########
    154.3 |########
    154.7 |########
    155.1 |########
    155.5 |
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 148.5-172.0 ns)
    148.5 |##############################
    149.7 |########################################
    150.8 |#########################
    152.0 |#########################
    153.2 |###############
    154.4 |#########################
    155.5 |
    156.7 |##########
    157.9 |##########
    159.1 |
    160.2 |
    161.4 |
    162.6 |
    163.8 |
    164.9 |
    166.1 |
    167.3 |
    168.5 |
    169.6 |
    170.8 |
  (3 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.79 (measurement drift or warm-up artifact)
