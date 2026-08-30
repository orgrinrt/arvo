# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 61% (significant)

warm-container-native is -406 ns (61%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.6x slower than the field

warm-container-plusone (664 ns) is 2.6x the fastest (252 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 13.6%)

warm-container-native wins on median (252 ns) yet has the highest variance (CV 13.6%), while warm-container-headroom is the steadiest (CV 1.1%, 661 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.88)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (159% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 159% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone's edge over baseline is significant but tiny (6 ns, 0.98%)

warm-container-plusone differs from baseline warm-container-headroom by 6 ns (0.98%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 252.1 ns median (-61.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.63x (fastest 252.1 ns, slowest 664.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 725ns | 722ns | 718ns | 723ns | 740ns | base |
| warm-container-kernel | 333ns | 318ns | 312ns | 324ns | 381ns | -54.07% |
| warm-container-minimum | 320ns | 319ns | 311ns | 320ns | 331ns | -55.83% |
| warm-container-native | 320ns | 314ns | 311ns | 314ns | 347ns | -55.89% |
| warm-container-plusone | 791ns | 730ns | 719ns | 763ns | 948ns | +9.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 664ns | 658ns | 676ns | base | 61.648 |
| warm-container-kernel | 268ns | 252ns | 307ns | -59.70% | 152.964 |
| warm-container-minimum | 258ns | 252ns | 266ns | -61.20% | 158.875 |
| warm-container-native | 259ns | 251ns | 284ns | -61.09% | 158.442 |
| warm-container-plusone | 724ns | 659ns | 869ns | +8.94% | 56.588 |

## Performance model

- Peak throughput: **163.285 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 61.925 | 37.9% |
| warm-container-kernel | 160.376 | 98.2% |
| warm-container-minimum | 160.250 | 98.1% |
| warm-container-native | 162.475 | 99.5% |
| warm-container-plusone | 61.668 | 37.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 725ns | 725ns | base |
| warm-container-kernel | 333ns | 333ns | -54.07% |
| warm-container-minimum | 320ns | 320ns | -55.83% |
| warm-container-native | 320ns | 320ns | -55.89% |
| warm-container-plusone | 791ns | 791ns | +9.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 661ns | base | --- | [661, 663] | --- | --- | --- | --- |
| warm-container-kernel | 255ns | -406.1ns (-61.4%) | [-408, -404]ns | [255, 256] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 256ns | -404.2ns (-61.1%) | [-408, -402]ns | [255, 259] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 252ns | -408.8ns (-61.8%) | [-412, -407]ns | [252, 253] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 664ns | +4.2ns (+0.6%) | [+1, +20]ns | [662, 678] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 659ns | -61.7% | -61.3% | -60.9% | +0.2% |
| 2 | 660ns | -61.6% | -61.3% | -60.9% | +0.1% |
| 3 | 660ns | -61.7% | -61.3% | -61.2% | +25.3% |
| 4 | 661ns | -61.6% | -60.9% | -61.4% | +0.1% |
| 5 | 660ns | -61.9% | -61.3% | -61.5% | +2.0% |
| 6 | 662ns | -62.1% | -61.5% | -61.7% | +0.2% |
| 7 | 675ns | -62.7% | -62.2% | -62.1% | -2.6% |
| 8 | 680ns | -62.9% | -62.4% | -62.5% | -2.9% |
| 9 | 667ns | -62.2% | -61.9% | -61.9% | +8.4% |
| 10 | 692ns | -63.6% | -63.2% | -63.2% | +15.4% |
| 11 | 661ns | -61.3% | -60.9% | -61.7% | +0.4% |
| 12 | 658ns | -61.2% | -60.8% | -61.6% | +0.3% |
| 13 | 659ns | -61.3% | -61.2% | -61.7% | +2.5% |
| 14 | 659ns | -61.4% | -61.2% | -61.7% | +2.6% |
| 15 | 663ns | -61.7% | -61.6% | -62.2% | -0.4% |
| 16 | 658ns | -61.2% | -61.1% | -61.8% | +0.9% |
| 17 | 660ns | -61.1% | -61.5% | -61.9% | +1.5% |
| 18 | 663ns | -61.5% | -60.8% | -62.1% | +0.1% |
| 19 | 661ns | -61.2% | -59.7% | -62.0% | +1.5% |
| 20 | 658ns | -61.3% | -59.5% | -61.7% | +3.3% |
| 21 | 662ns | -53.8% | -59.6% | -62.2% | +31.2% |
| 22 | 662ns | -53.7% | -59.6% | -62.1% | +31.2% |
| 23 | 661ns | -53.0% | -59.8% | -62.0% | +31.3% |
| 24 | 661ns | -53.8% | -60.0% | -61.7% | +31.3% |
| 25 | 663ns | -53.9% | -60.3% | -62.1% | +30.5% |
| 26 | 657ns | -53.2% | -59.8% | -61.8% | +32.2% |
| 27 | 660ns | -53.5% | -59.6% | -61.8% | +31.7% |
| 28 | 661ns | -53.6% | -60.1% | -61.9% | +31.6% |
| 29 | 663ns | -53.7% | -60.1% | -62.0% | +31.0% |
| 30 | 656ns | -53.5% | -59.5% | -61.6% | +32.5% |
| 31 | 670ns | -60.4% | -62.4% | -62.3% | -1.4% |
| 32 | 682ns | -62.6% | -63.1% | -63.0% | -2.9% |
| 33 | 670ns | -61.8% | -62.4% | -62.5% | -1.4% |
| 34 | 670ns | -62.1% | -62.3% | -62.4% | -1.6% |
| 35 | 670ns | -61.8% | -62.6% | -62.3% | -1.8% |
| 36 | 670ns | -61.4% | -62.4% | -62.4% | -1.6% |
| 37 | 670ns | -61.7% | -62.4% | -62.5% | -1.6% |
| 38 | 671ns | -62.0% | -62.6% | -29.6% | -1.2% |
| 39 | 662ns | -61.5% | -62.0% | -60.8% | -0.4% |
| 40 | 660ns | -61.2% | -61.5% | -61.0% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.369 | moderate+ |
| warm-container-kernel | 0.882 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.843 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.002 | ok |
| warm-container-plusone | 0.735 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 13/40, lost 27/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.3ns | 664.4ns | 0.4% |  |
| warm-container-kernel | 2.5ns | 267.8ns | 0.9% |  |
| warm-container-minimum | 2.6ns | 257.8ns | 1.0% |  |
| warm-container-native | 2.8ns | 258.5ns | 1.1% |  |
| warm-container-plusone | 2.5ns | 723.8ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 657.9-676.5 ns)
    657.9 |####################
    658.9 |#############
    659.8 |#################################
    660.7 |########################################
    661.6 |##########################
    662.6 |##########################
    663.5 |
    664.4 |
    665.3 |
    666.3 |######
    667.2 |
    668.1 |
    669.0 |####################
    670.0 |##########################
    670.9 |
    671.8 |
    672.7 |
    673.7 |
    674.6 |######
    675.5 |
  (4 below, 3 above range)

warm-container-kernel (n=40, range 252.0-307.1 ns)
    252.0 |########################################
    254.8 |########################################
    257.5 |###
    260.3 |
    263.1 |###
    265.8 |
    268.6 |
    271.3 |
    274.1 |
    276.8 |
    279.6 |
    282.3 |
    285.1 |
    287.8 |
    290.6 |
    293.4 |
    296.1 |
    298.9 |
    301.6 |
    304.4 |########################
  (2 below, 2 above range)

warm-container-minimum (n=40, range 251.6-266.3 ns)
    251.6 |##################################
    252.4 |#####
    253.1 |#####
    253.8 |###########
    254.6 |############################
    255.3 |########################################
    256.0 |
    256.8 |
    257.5 |#####
    258.2 |###########
    259.0 |
    259.7 |#####
    260.4 |
    261.2 |
    261.9 |
    262.6 |#####
    263.4 |#####
    264.1 |#################
    264.8 |
    265.6 |######################
  (2 below, 3 above range)

warm-container-native (n=40, range 250.8-283.9 ns)
    250.8 |########################################
    252.5 |###########
    254.2 |###########
    255.8 |##
    257.5 |######
    259.1 |##
    260.8 |
    262.4 |
    264.1 |
    265.7 |
    267.4 |
    269.1 |
    270.7 |
    272.4 |
    274.0 |
    275.7 |
    277.3 |
    279.0 |
    280.6 |
    282.3 |
  (6 below, 1 above range)

warm-container-plusone (n=40, range 658.9-868.9 ns)
    658.9 |########################################
    669.4 |##############
    679.9 |
    690.4 |
    700.9 |
    711.4 |
    721.9 |##
    732.4 |
    742.9 |
    753.4 |
    763.9 |
    774.4 |
    784.9 |
    795.4 |##
    805.9 |
    816.4 |##
    826.9 |
    837.4 |
    847.9 |
    858.4 |################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.74 (measurement drift or warm-up artifact)
