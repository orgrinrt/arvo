# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 51% (significant)

warm-container-kernel is -262 ns (51%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-native, warm-container-kernel) are a dead heat (<1%)

warm-container-native (256 ns) and warm-container-kernel (257 ns) differ by 0.67%, inside the noise, even though the wider field spreads 99.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.84)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (98% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 98% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone's edge over baseline is significant but tiny (-4 ns, 0.73%)

warm-container-plusone differs from baseline warm-container-headroom by -4 ns (0.73%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 255.6 ns median (-49.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.00x (fastest 255.6 ns, slowest 510.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 592ns | 571ns | 561ns | 578ns | 664ns | base |
| warm-container-kernel | 321ns | 320ns | 314ns | 321ns | 327ns | -45.82% |
| warm-container-minimum | 325ns | 321ns | 313ns | 320ns | 355ns | -44.99% |
| warm-container-native | 324ns | 320ns | 314ns | 321ns | 343ns | -45.28% |
| warm-container-plusone | 577ns | 574ns | 569ns | 574ns | 595ns | -2.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 525ns | 500ns | 582ns | base | 77.976 |
| warm-container-kernel | 257ns | 253ns | 262ns | -50.99% | 159.103 |
| warm-container-minimum | 263ns | 253ns | 287ns | -50.01% | 155.994 |
| warm-container-native | 260ns | 252ns | 277ns | -50.48% | 157.472 |
| warm-container-plusone | 511ns | 507ns | 517ns | -2.64% | 80.092 |

## Performance model

- Peak throughput: **162.515 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 80.251 | 49.4% |
| warm-container-kernel | 159.192 | 98.0% |
| warm-container-minimum | 158.945 | 97.8% |
| warm-container-native | 160.250 | 98.6% |
| warm-container-plusone | 80.251 | 49.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 592ns | 592ns | base |
| warm-container-kernel | 321ns | 321ns | -45.82% |
| warm-container-minimum | 325ns | 325ns | -44.99% |
| warm-container-native | 324ns | 324ns | -45.28% |
| warm-container-plusone | 577ns | 577ns | -2.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 510ns | base | --- | [507, 525] | --- | --- | --- | --- |
| warm-container-kernel | 257ns | -257.3ns (-50.4%) | [-267, -252]ns | [256, 259] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 258ns | -256.6ns (-50.3%) | [-266, -253]ns | [257, 260] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 256ns | -255.6ns (-50.1%) | [-261, -253]ns | [254, 260] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 510ns | no significant difference | [-12, +6]ns | [509, 512] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 508ns | -50.1% | -50.2% | -50.1% | +2.2% |
| 2 | 505ns | -49.9% | -50.0% | -49.9% | +2.0% |
| 3 | 507ns | -49.7% | -49.8% | -49.9% | +1.9% |
| 4 | 506ns | -49.7% | -50.2% | -50.4% | +1.6% |
| 5 | 510ns | -50.7% | -50.5% | -50.3% | +0.2% |
| 6 | 506ns | -49.5% | -50.2% | -50.2% | +1.9% |
| 7 | 507ns | -50.0% | -50.0% | -50.1% | +1.0% |
| 8 | 511ns | -50.3% | -50.5% | -50.2% | +1.0% |
| 9 | 505ns | -49.8% | -49.7% | -49.9% | +0.6% |
| 10 | 510ns | -50.5% | -50.2% | -50.6% | +1.5% |
| 11 | 500ns | -46.8% | -49.2% | -49.3% | +2.0% |
| 12 | 507ns | -47.1% | -50.0% | -50.2% | +0.5% |
| 13 | 500ns | -48.8% | -43.3% | -49.7% | +1.7% |
| 14 | 501ns | -48.0% | -43.0% | -49.5% | +1.6% |
| 15 | 500ns | -49.0% | -42.8% | -49.4% | +1.6% |
| 16 | 500ns | -48.6% | -42.2% | -30.9% | +1.5% |
| 17 | 501ns | -48.3% | -42.8% | -49.1% | +1.1% |
| 18 | 502ns | -49.1% | -42.8% | -49.8% | +1.2% |
| 19 | 501ns | -48.7% | -42.4% | -49.6% | +1.5% |
| 20 | 500ns | -48.7% | -42.7% | -49.2% | +1.7% |
| 21 | 530ns | -50.9% | -51.5% | -52.0% | -1.2% |
| 22 | 526ns | -51.2% | -51.0% | -49.0% | -2.8% |
| 23 | 531ns | -50.9% | -51.6% | -50.5% | -4.1% |
| 24 | 531ns | -50.5% | -50.9% | -51.7% | -4.5% |
| 25 | 530ns | -51.2% | -51.1% | -51.1% | -3.5% |
| 26 | 945ns | -72.9% | -72.5% | -72.1% | -45.4% |
| 27 | 520ns | -50.1% | -50.0% | -49.5% | -2.1% |
| 28 | 507ns | -49.0% | -49.1% | -46.8% | +0.2% |
| 29 | 513ns | -49.8% | -49.4% | -48.2% | -0.6% |
| 30 | 529ns | -51.2% | -50.8% | -49.6% | -3.4% |
| 31 | 531ns | -51.4% | -51.6% | -49.9% | -3.7% |
| 32 | 521ns | -50.0% | -50.5% | -48.5% | -1.7% |
| 33 | 527ns | -50.8% | -50.6% | -49.1% | -2.2% |
| 34 | 527ns | -51.1% | -50.7% | -50.2% | -3.2% |
| 35 | 528ns | -51.0% | -51.2% | -50.8% | -3.4% |
| 36 | 524ns | -50.4% | -50.6% | -50.6% | -2.3% |
| 37 | 528ns | -51.3% | -51.4% | -50.5% | -3.8% |
| 38 | 525ns | -51.4% | -51.0% | -50.7% | -2.8% |
| 39 | 526ns | -51.2% | -51.5% | -50.7% | -3.2% |
| 40 | 526ns | -50.6% | -50.4% | -50.4% | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.045 | ok |
| warm-container-kernel | 0.377 | moderate+ |
| warm-container-minimum | 0.836 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.015 | ok |
| warm-container-plusone | 0.166 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 19/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.0ns | 525.3ns | 0.4% |  |
| warm-container-kernel | 2.5ns | 257.4ns | 1.0% |  |
| warm-container-minimum | 2.6ns | 262.6ns | 1.0% |  |
| warm-container-native | 2.3ns | 260.1ns | 0.9% |  |
| warm-container-plusone | 1.9ns | 511.4ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 500.3-581.9 ns)
    500.3 |######################
    504.4 |########################################
    508.5 |#############
    512.5 |####
    516.6 |####
    520.7 |########
    524.8 |###################################
    528.8 |##########################
    532.9 |
    537.0 |
    541.1 |
    545.2 |
    549.2 |
    553.3 |
    557.4 |
    561.5 |
    565.6 |
    569.6 |
    573.7 |
    577.8 |
  (4 below, 1 above range)

warm-container-kernel (n=40, range 253.1-262.3 ns)
    253.1 |##########################
    253.6 |######
    254.0 |######
    254.5 |
    254.9 |#############
    255.4 |####################
    255.9 |#############
    256.3 |##########################
    256.8 |######
    257.2 |####################
    257.7 |######
    258.1 |#############
    258.6 |####################
    259.1 |######
    259.5 |######
    260.0 |########################################
    260.4 |
    260.9 |
    261.4 |
    261.8 |
  (2 below, 3 above range)

warm-container-minimum (n=40, range 252.8-286.6 ns)
    252.8 |########################################
    254.5 |####
    256.2 |###############################
    257.9 |#################
    259.6 |###################################
    261.2 |
    262.9 |
    264.6 |
    266.3 |
    268.0 |
    269.7 |
    271.4 |
    273.1 |
    274.7 |
    276.4 |
    278.1 |
    279.8 |
    281.5 |
    283.2 |####
    284.9 |########
  (3 below, 5 above range)

warm-container-native (n=40, range 252.0-277.2 ns)
    252.0 |########################################
    253.3 |########################################
    254.6 |#################
    255.8 |#####
    257.1 |
    258.3 |######################
    259.6 |#####
    260.9 |#################
    262.1 |#################
    263.4 |
    264.6 |###########
    265.9 |#####
    267.1 |#################
    268.4 |
    269.7 |#####
    270.9 |
    272.2 |
    273.4 |
    274.7 |
    276.0 |
  (3 below, 1 above range)

warm-container-plusone (n=40, range 507.5-517.5 ns)
    507.5 |################################
    508.0 |########################
    508.5 |################
    509.0 |################
    509.5 |################
    510.0 |########################################
    510.5 |################
    511.0 |########################
    511.5 |
    512.0 |########################
    512.5 |
    513.0 |########
    513.5 |########
    514.0 |
    514.5 |
    515.0 |########################
    515.5 |########
    516.0 |################
    516.5 |
    517.0 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: autocorrelation=0.84 (measurement drift or warm-up artifact)
