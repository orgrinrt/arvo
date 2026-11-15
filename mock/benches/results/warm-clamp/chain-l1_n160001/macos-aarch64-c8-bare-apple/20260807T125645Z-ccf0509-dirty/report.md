# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 52% (significant)

warm-clamp-minimum is -165 ns (52%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-head is an outlier: 2.1x slower than the field

warm-clamp-head (321 ns) is 2.1x the fastest (155 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-min-lanes, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-min-lanes (155 ns) and warm-clamp-minimum (155 ns) differ by 0.00%, inside the noise, even though the wider field spreads 107.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.68)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.68, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum} vs {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} (106% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum} and a slow tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} with a 106% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-head's comparison is tie-heavy (12% tied pairs)

12% of paired samples for warm-clamp-head are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### warm-clamp-accfit's edge over baseline is significant but tiny (-1 ns, 0.41%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -1 ns (0.41%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 154.6 ns median (-51.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.07x (fastest 154.6 ns, slowest 320.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 387ns | 382ns | 379ns | 382ns | 407ns | base |
| warm-clamp-accfit | 380ns | 380ns | 376ns | 380ns | 384ns | -1.68% |
| warm-clamp-accfit-dyn | 380ns | 380ns | 377ns | 380ns | 385ns | -1.61% |
| warm-clamp-head | 384ns | 381ns | 378ns | 383ns | 394ns | -0.63% |
| warm-clamp-min-lanes | 215ns | 215ns | 212ns | 214ns | 222ns | -44.30% |
| warm-clamp-minimum | 216ns | 216ns | 212ns | 216ns | 223ns | -44.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 324ns | 316ns | 343ns | base | 25.294 |
| warm-clamp-accfit | 319ns | 316ns | 322ns | -1.62% | 25.710 |
| warm-clamp-accfit-dyn | 319ns | 315ns | 323ns | -1.53% | 25.687 |
| warm-clamp-head | 323ns | 318ns | 331ns | -0.37% | 25.387 |
| warm-clamp-min-lanes | 156ns | 152ns | 162ns | -51.97% | 52.668 |
| warm-clamp-minimum | 154ns | 150ns | 159ns | -52.32% | 53.046 |

## Performance model

- Peak throughput: **54.559 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 25.632 | 47.0% |
| warm-clamp-accfit | 25.737 | 47.2% |
| warm-clamp-accfit-dyn | 25.664 | 47.0% |
| warm-clamp-head | 25.552 | 46.8% |
| warm-clamp-min-lanes | 52.988 | 97.1% |
| warm-clamp-minimum | 52.988 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 387ns | 387ns | base |
| warm-clamp-accfit | 380ns | 380ns | -1.68% |
| warm-clamp-accfit-dyn | 380ns | 380ns | -1.61% |
| warm-clamp-head | 384ns | 384ns | -0.63% |
| warm-clamp-min-lanes | 215ns | 215ns | -44.30% |
| warm-clamp-minimum | 216ns | 216ns | -44.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 320ns | base | --- | [319, 321] | --- | --- | --- | --- |
| warm-clamp-accfit | 318ns | no significant difference | [-3, +0]ns | [318, 319] | no | 0.0588 | 0.0470 | 3 |
| warm-clamp-accfit-dyn | 319ns | no significant difference | [-4, +0]ns | [318, 320] | no | 0.0588 | 0.0470 | 3 |
| warm-clamp-head | 321ns | no significant difference | [-1, +3]ns | [320, 322] | no | 0.1539 | 0.1539 | 0 |
| warm-clamp-min-lanes | 155ns | -165.6ns (-51.8%) | [-166, -164]ns | [154, 155] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 155ns | -165.8ns (-51.9%) | [-169, -164]ns | [153, 155] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 320ns | +2.1% | -0.2% | +3.4% | -51.6% | -51.2% |
| 2 | 316ns | +0.2% | +2.8% | -0.3% | -50.7% | -49.5% |
| 3 | 323ns | -1.0% | -1.8% | -1.8% | -52.1% | -50.2% |
| 4 | 315ns | +2.6% | +0.3% | +1.5% | -49.9% | -48.8% |
| 5 | 316ns | +0.7% | +1.2% | +2.4% | -51.0% | -50.0% |
| 6 | 320ns | -0.5% | -0.8% | +0.2% | -51.7% | -51.7% |
| 7 | 318ns | -0.1% | -0.4% | +1.3% | -52.2% | -49.5% |
| 8 | 319ns | -1.4% | +0.0% | -0.3% | -38.9% | -51.3% |
| 9 | 316ns | +1.5% | +1.2% | +1.1% | -51.7% | -49.7% |
| 10 | 319ns | +0.1% | -0.4% | -0.3% | -51.3% | -50.7% |
| 11 | 321ns | -0.5% | -1.7% | -0.9% | -53.1% | -53.2% |
| 12 | 319ns | +0.3% | -1.6% | +0.1% | -51.1% | -53.1% |
| 13 | 324ns | -2.6% | -1.3% | -1.0% | -53.3% | -53.3% |
| 14 | 318ns | +1.6% | -1.0% | +0.7% | -52.4% | -50.3% |
| 15 | 317ns | +1.2% | +1.3% | +0.9% | -51.1% | -51.8% |
| 16 | 320ns | -0.5% | -0.5% | +0.1% | -52.3% | -51.2% |
| 17 | 317ns | +0.9% | +0.7% | +0.9% | -51.0% | -51.7% |
| 18 | 320ns | -0.9% | -0.8% | -0.3% | -51.5% | -52.3% |
| 19 | 318ns | -0.7% | +1.2% | +1.3% | -52.2% | -53.1% |
| 20 | 318ns | +0.0% | +0.7% | +1.0% | -50.9% | -51.5% |
| 21 | 320ns | -0.9% | +3.4% | +0.2% | -51.5% | -51.7% |
| 22 | 318ns | -0.4% | +0.0% | +0.2% | -51.4% | -51.2% |
| 23 | 334ns | -5.3% | -4.0% | -4.5% | -53.7% | -54.4% |
| 24 | 325ns | -2.0% | -1.4% | -1.4% | -51.0% | -52.9% |
| 25 | 439ns | -26.2% | -27.1% | -27.0% | -64.4% | -64.6% |
| 26 | 318ns | -0.3% | +0.9% | +0.1% | -50.7% | -51.4% |
| 27 | 320ns | +0.0% | -1.8% | +0.3% | -52.5% | -52.5% |
| 28 | 316ns | +0.8% | +0.4% | +0.8% | -51.2% | -51.7% |
| 29 | 324ns | -1.9% | -1.1% | -0.9% | -52.5% | -52.4% |
| 30 | 319ns | +0.0% | -0.8% | -0.2% | -51.0% | -52.9% |
| 31 | 330ns | -4.0% | -3.5% | +0.1% | -52.8% | -53.2% |
| 32 | 323ns | -1.7% | -0.9% | +2.3% | -52.7% | -53.4% |
| 33 | 320ns | -0.8% | -0.4% | +3.2% | -51.7% | -53.2% |
| 34 | 322ns | -1.1% | -2.3% | +3.0% | -51.0% | -51.7% |
| 35 | 319ns | -0.1% | +0.6% | +3.8% | -52.2% | -51.2% |
| 36 | 321ns | -0.7% | -0.2% | +2.7% | -51.6% | -51.7% |
| 37 | 318ns | +0.5% | +0.0% | +3.8% | -50.8% | -50.8% |
| 38 | 330ns | -3.8% | -3.4% | -0.5% | -54.0% | -54.2% |
| 39 | 332ns | -5.4% | -3.5% | -0.3% | -53.7% | -54.6% |
| 40 | 332ns | -4.3% | -4.4% | -0.3% | -53.4% | -53.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.009 | ok |
| warm-clamp-accfit | -0.074 | ok |
| warm-clamp-accfit-dyn | -0.010 | ok |
| warm-clamp-head | 0.683 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | -0.136 | ok |
| warm-clamp-minimum | 0.423 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 25/40, lost 12/40
- **warm-clamp-accfit-dyn**: won 25/40, lost 12/40
- **warm-clamp-head**: won 15/40, lost 25/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.1ns | 323.9ns | 0.6% |  |
| warm-clamp-accfit | 2.5ns | 318.6ns | 0.8% |  |
| warm-clamp-accfit-dyn | 2.2ns | 318.9ns | 0.7% |  |
| warm-clamp-head | 2.1ns | 322.7ns | 0.6% |  |
| warm-clamp-min-lanes | 2.0ns | 155.5ns | 1.3% |  |
| warm-clamp-minimum | 2.3ns | 154.4ns | 1.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 316.2-343.4 ns)
    316.2 |######################
    317.5 |########################################
    318.9 |########################################
    320.3 |##################################
    321.6 |###########
    323.0 |#################
    324.3 |#####
    325.7 |
    327.1 |
    328.4 |
    329.8 |###########
    331.1 |###########
    332.5 |#####
    333.9 |
    335.2 |
    336.6 |
    337.9 |
    339.3 |
    340.7 |
    342.0 |
  (4 below, 1 above range)

warm-clamp-accfit (n=40, range 315.8-322.2 ns)
    315.8 |
    316.2 |######
    316.5 |#############
    316.8 |####################
    317.1 |
    317.4 |##########################
    317.7 |#################################
    318.1 |#############
    318.4 |
    318.7 |########################################
    319.0 |######
    319.3 |####################
    319.6 |
    320.0 |####################
    320.3 |#############
    320.6 |
    320.9 |
    321.2 |
    321.6 |
    321.9 |
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 315.0-322.7 ns)
    315.0 |
    315.4 |####################
    315.8 |
    316.2 |##########
    316.6 |##########
    316.9 |##############################
    317.3 |##########
    317.7 |####################
    318.1 |####################
    318.5 |##############################
    318.9 |##############################
    319.3 |########################################
    319.6 |##############################
    320.0 |########################################
    320.4 |##############################
    320.8 |
    321.2 |##########
    321.6 |##########
    322.0 |
    322.4 |
  (4 below, 2 above range)

warm-clamp-head (n=40, range 317.7-331.0 ns)
    317.7 |########################################
    318.4 |####################
    319.1 |#################################
    319.7 |######
    320.4 |########################################
    321.1 |####################
    321.7 |#############
    322.4 |
    323.0 |######
    323.7 |
    324.4 |
    325.0 |
    325.7 |
    326.4 |
    327.0 |
    327.7 |
    328.3 |######
    329.0 |
    329.7 |######
    330.3 |##########################
  (2 below, 5 above range)

warm-clamp-min-lanes (n=40, range 151.7-161.9 ns)
    151.7 |######################
    152.2 |###########
    152.7 |###########
    153.2 |
    153.7 |#################
    154.2 |########################################
    154.7 |#################
    155.3 |######################
    155.8 |##################################
    156.3 |###########
    156.8 |
    157.3 |###########
    157.8 |
    158.3 |
    158.8 |
    159.4 |#####
    159.9 |
    160.4 |
    160.9 |
    161.4 |
  (3 below, 1 above range)

warm-clamp-minimum (n=40, range 150.2-159.3 ns)
    150.2 |
    150.6 |################
    151.1 |################
    151.5 |########
    152.0 |########
    152.4 |########################
    152.9 |################
    153.3 |
    153.8 |################
    154.3 |########################################
    154.7 |########
    155.2 |################################
    155.6 |########
    156.1 |################
    156.5 |########
    157.0 |
    157.4 |########
    157.9 |################
    158.4 |########
    158.8 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-head**: autocorrelation=0.68 (measurement drift or warm-up artifact)
