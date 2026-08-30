# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (501 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 267 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 48% (significant)

warm-container-native is -242 ns (48%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (267 ns) and warm-container-minimum (268 ns) differ by 0.30%, inside the noise, even though the wider field spreads 87.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.87)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-kernel} vs {warm-container-headroom} (57% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-kernel} and a slow tier {warm-container-headroom} with a 57% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-native** at 267.3 ns median (-46.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.87x (fastest 267.3 ns, slowest 500.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 568ns | 563ns | 558ns | 565ns | 586ns | base |
| warm-container-kernel | 388ns | 394ns | 340ns | 391ns | 428ns | -31.65% |
| warm-container-minimum | 355ns | 330ns | 323ns | 340ns | 428ns | -37.57% |
| warm-container-native | 330ns | 329ns | 323ns | 329ns | 340ns | -41.85% |
| warm-container-plusone | 389ns | 384ns | 325ns | 383ns | 469ns | -31.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 504ns | 499ns | 513ns | base | 81.307 |
| warm-container-kernel | 314ns | 275ns | 347ns | -37.63% | 130.370 |
| warm-container-minimum | 288ns | 262ns | 347ns | -42.84% | 142.247 |
| warm-container-native | 268ns | 262ns | 276ns | -46.87% | 153.021 |
| warm-container-plusone | 315ns | 263ns | 380ns | -37.41% | 129.904 |

## Performance model

- Peak throughput: **156.343 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 81.789 | 52.3% |
| warm-container-kernel | 128.160 | 82.0% |
| warm-container-minimum | 152.779 | 97.7% |
| warm-container-native | 153.236 | 98.0% |
| warm-container-plusone | 131.874 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 568ns | 568ns | base |
| warm-container-kernel | 388ns | 388ns | -31.65% |
| warm-container-minimum | 355ns | 355ns | -37.57% |
| warm-container-native | 330ns | 330ns | -41.85% |
| warm-container-plusone | 389ns | 389ns | -31.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 501ns | base | --- | [500, 502] | --- | --- | --- | --- |
| warm-container-kernel | 320ns | -180.8ns (-36.1%) | [-183, -179]ns | [318, 320] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 268ns | -234.6ns (-46.8%) | [-237, -232]ns | [267, 270] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 267ns | -237.5ns (-47.4%) | [-239, -234]ns | [265, 268] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 311ns | -194.5ns (-38.8%) | [-237, -152]ns | [269, 348] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 500ns | -36.5% | -46.6% | -47.1% | -31.1% |
| 2 | 500ns | -35.8% | -46.7% | -47.5% | -30.6% |
| 3 | 499ns | -36.3% | -46.8% | -46.8% | -30.7% |
| 4 | 497ns | -35.7% | -45.9% | -47.1% | -30.0% |
| 5 | 500ns | -36.4% | -46.3% | -47.5% | -31.0% |
| 6 | 498ns | -35.8% | -46.3% | -47.0% | -29.4% |
| 7 | 501ns | -35.9% | -46.7% | -47.7% | -30.7% |
| 8 | 501ns | -36.3% | -46.6% | -47.1% | -31.4% |
| 9 | 501ns | -35.9% | -46.8% | -47.6% | -29.9% |
| 10 | 500ns | -47.5% | -47.0% | -46.5% | -30.1% |
| 11 | 512ns | -45.5% | -47.6% | -47.6% | -47.4% |
| 12 | 510ns | -45.7% | -47.5% | -46.9% | -47.5% |
| 13 | 510ns | -45.6% | -47.3% | -47.3% | -47.4% |
| 14 | 510ns | -45.3% | -47.9% | -46.9% | -46.8% |
| 15 | 510ns | -47.1% | -47.3% | -47.1% | -47.3% |
| 16 | 514ns | -46.1% | -47.9% | -47.9% | -48.1% |
| 17 | 512ns | -45.0% | -47.4% | -47.4% | -46.5% |
| 18 | 510ns | -45.5% | -46.6% | -47.7% | -45.5% |
| 19 | 506ns | -44.9% | -46.5% | -47.7% | -45.8% |
| 20 | 509ns | -44.8% | -46.9% | -47.7% | -47.4% |
| 21 | 500ns | -35.8% | -47.4% | -45.5% | -46.5% |
| 22 | 500ns | -36.0% | -47.6% | -46.6% | -46.9% |
| 23 | 499ns | -35.9% | -47.1% | -46.3% | -47.8% |
| 24 | 500ns | -36.1% | -48.3% | -46.5% | -47.4% |
| 25 | 499ns | -36.1% | -47.3% | -46.3% | -47.5% |
| 26 | 501ns | -36.5% | -47.8% | -45.9% | -47.4% |
| 27 | 501ns | -36.1% | -32.6% | -46.5% | -47.2% |
| 28 | 500ns | -36.2% | -47.2% | -46.0% | -47.3% |
| 29 | 508ns | -37.0% | -48.7% | -46.9% | -48.7% |
| 30 | 528ns | -39.9% | -50.1% | -49.0% | -50.0% |
| 31 | 500ns | -30.8% | -31.2% | -47.8% | -24.1% |
| 32 | 500ns | -30.8% | -30.3% | -47.0% | -24.1% |
| 33 | 502ns | -30.8% | -31.0% | -47.8% | -24.1% |
| 34 | 500ns | -30.7% | -30.3% | -46.8% | -24.4% |
| 35 | 501ns | -31.3% | -30.8% | -47.5% | -24.3% |
| 36 | 501ns | -31.7% | -31.1% | -47.3% | -24.2% |
| 37 | 503ns | -31.6% | -31.2% | -47.7% | -24.4% |
| 38 | 500ns | -30.1% | -31.1% | -37.1% | -23.8% |
| 39 | 500ns | -30.8% | -30.8% | -46.5% | -24.1% |
| 40 | 508ns | -31.7% | -32.4% | -47.4% | -25.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.427 | moderate+ |
| warm-container-kernel | 0.859 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.786 | HIGH+ (drift/warm-up) |
| warm-container-native | -0.002 | ok |
| warm-container-plusone | 0.872 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.2ns | 503.8ns | 0.4% |  |
| warm-container-kernel | 2.4ns | 314.2ns | 0.8% |  |
| warm-container-minimum | 2.9ns | 288.0ns | 1.0% |  |
| warm-container-native | 2.7ns | 267.7ns | 1.0% |  |
| warm-container-plusone | 2.3ns | 315.3ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 498.9-513.2 ns)
    498.9 |########################
    499.6 |####################
    500.4 |########################################
    501.1 |############
    501.8 |
    502.5 |####
    503.2 |
    503.9 |
    504.6 |
    505.3 |
    506.1 |####
    506.8 |
    507.5 |####
    508.2 |########
    508.9 |############
    509.6 |####
    510.3 |####
    511.0 |####
    511.8 |####
    512.5 |
  (2 below, 2 above range)

warm-container-kernel (n=40, range 274.8-346.7 ns)
    274.8 |###########
    278.4 |##############
    282.0 |
    285.6 |
    289.2 |
    292.8 |
    296.4 |
    300.0 |
    303.6 |
    307.1 |
    310.7 |
    314.3 |##############
    317.9 |########################################
    321.5 |
    325.1 |
    328.7 |
    332.3 |
    335.9 |
    339.5 |##
    343.1 |##############
  (2 below, 4 above range)

warm-container-minimum (n=40, range 262.0-346.5 ns)
    262.0 |########################
    266.2 |########################################
    270.4 |#####
    274.7 |
    278.9 |
    283.1 |
    287.4 |
    291.6 |
    295.8 |
    300.0 |
    304.3 |
    308.5 |
    312.7 |
    316.9 |
    321.2 |
    325.4 |
    329.6 |
    333.9 |##
    338.1 |
    342.3 |##################
  (3 below, 3 above range)

warm-container-native (n=40, range 262.3-276.1 ns)
    262.3 |########################################
    263.0 |
    263.7 |#############
    264.4 |####################
    265.0 |#############
    265.7 |#############
    266.4 |#############
    267.1 |##########################
    267.8 |#################################
    268.5 |
    269.2 |##########################
    269.9 |####################
    270.6 |#############
    271.3 |
    271.9 |######
    272.6 |
    273.3 |
    274.0 |
    274.7 |
    275.4 |
  (3 below, 1 above range)

warm-container-plusone (n=40, range 262.8-380.2 ns)
    262.8 |########################################
    268.6 |##############
    274.5 |#######
    280.4 |
    286.2 |
    292.1 |
    298.0 |
    303.9 |
    309.8 |
    315.6 |
    321.5 |
    327.4 |
    333.2 |
    339.1 |##########
    345.0 |##################
    350.9 |#######
    356.8 |
    362.6 |
    368.5 |
    374.4 |#####################
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.87 (measurement drift or warm-up artifact)
