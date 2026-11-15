# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (696 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 336 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-plusone beats baseline by 58% (significant)

warm-container-plusone is -407 ns (58%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.1x slower than the field

warm-container-headroom (696 ns) is 2.1x the fastest (336 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (336 ns) and warm-container-minimum (338 ns) differ by 0.43%, inside the noise, even though the wider field spreads 106.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.86)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-kernel} vs {warm-container-headroom} (64% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-plusone, warm-container-kernel} and a slow tier {warm-container-headroom} with a 64% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-native vs stability leader warm-container-plusone (+1% speed for 1.7x steadier)

warm-container-native is fastest (336 ns, CV 8.9%); warm-container-plusone gives up 1.1% median for 1.7x lower variance (CV 5.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: warm-container-native** at 336.4 ns median (-51.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.07x (fastest 336.4 ns, slowest 696.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 800ns | 760ns | 720ns | 777ns | 949ns | base |
| warm-container-kernel | 494ns | 488ns | 476ns | 489ns | 526ns | -38.26% |
| warm-container-minimum | 418ns | 400ns | 393ns | 406ns | 479ns | -47.76% |
| warm-container-native | 407ns | 398ns | 393ns | 399ns | 444ns | -49.13% |
| warm-container-plusone | 414ns | 404ns | 395ns | 410ns | 448ns | -48.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 731ns | 659ns | 868ns | base | 56.032 |
| warm-container-kernel | 431ns | 415ns | 457ns | -41.09% | 95.120 |
| warm-container-minimum | 354ns | 334ns | 406ns | -51.60% | 115.761 |
| warm-container-native | 344ns | 334ns | 375ns | -52.90% | 118.963 |
| warm-container-plusone | 350ns | 335ns | 379ns | -52.15% | 117.101 |

## Performance model

- Peak throughput: **122.754 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 58.829 | 47.9% |
| warm-container-kernel | 96.195 | 78.4% |
| warm-container-minimum | 121.219 | 98.7% |
| warm-container-native | 121.742 | 99.2% |
| warm-container-plusone | 120.471 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 800ns | 800ns | base |
| warm-container-kernel | 494ns | 494ns | -38.26% |
| warm-container-minimum | 418ns | 418ns | -47.76% |
| warm-container-native | 407ns | 407ns | -49.13% |
| warm-container-plusone | 414ns | 414ns | -48.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 696ns | base | --- | [673, 703] | --- | --- | --- | --- |
| warm-container-kernel | 426ns | -268.3ns (-38.5%) | [-285, -242]ns | [420, 435] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 338ns | -356.1ns (-51.1%) | [-368, -334]ns | [335, 341] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 336ns | -338.6ns (-48.6%) | [-350, -333]ns | [335, 341] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 340ns | -352.5ns (-50.6%) | [-367, -334]ns | [339, 346] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 696ns | -39.4% | -51.0% | -49.8% | -51.0% |
| 2 | 698ns | -39.9% | -52.1% | -49.4% | -50.2% |
| 3 | 701ns | -40.6% | -52.2% | -49.9% | -52.2% |
| 4 | 699ns | -38.6% | -52.2% | -49.4% | -52.1% |
| 5 | 703ns | -37.9% | -52.5% | -49.9% | -52.1% |
| 6 | 697ns | -40.3% | -51.8% | -48.9% | -51.9% |
| 7 | 702ns | -40.8% | -51.3% | -51.7% | -52.4% |
| 8 | 699ns | -40.4% | -51.2% | -49.4% | -52.2% |
| 9 | 768ns | -46.0% | -56.4% | -54.1% | -55.8% |
| 10 | 825ns | -49.4% | -59.2% | -36.2% | -58.9% |
| 11 | 671ns | -37.2% | -49.7% | -49.1% | -49.1% |
| 12 | 673ns | -36.8% | -49.3% | -49.3% | -49.7% |
| 13 | 674ns | -37.7% | -49.5% | -49.6% | -50.0% |
| 14 | 677ns | -37.5% | -49.8% | -49.7% | -49.8% |
| 15 | 671ns | -36.7% | -49.5% | -49.5% | -49.8% |
| 16 | 713ns | -40.2% | -52.6% | -52.0% | -51.7% |
| 17 | 674ns | -35.2% | -49.8% | -49.4% | -48.0% |
| 18 | 673ns | -35.0% | -49.5% | -49.8% | -48.9% |
| 19 | 672ns | -34.7% | -49.7% | -49.8% | -49.9% |
| 20 | 672ns | -34.8% | -49.8% | -49.3% | -49.6% |
| 21 | 868ns | -36.9% | -53.5% | -61.6% | -60.8% |
| 22 | 867ns | -50.6% | -53.3% | -61.4% | -58.9% |
| 23 | 870ns | -52.2% | -53.3% | -61.6% | -60.0% |
| 24 | 868ns | -52.2% | -53.7% | -61.6% | -60.9% |
| 25 | 868ns | -52.1% | -53.2% | -61.5% | -61.0% |
| 26 | 869ns | -52.3% | -52.9% | -61.6% | -60.9% |
| 27 | 867ns | -52.0% | -53.5% | -61.5% | -60.9% |
| 28 | 869ns | -52.1% | -53.1% | -61.5% | -61.1% |
| 29 | 867ns | -52.0% | -53.3% | -61.4% | -61.1% |
| 30 | 869ns | -44.6% | -53.2% | -61.6% | -60.6% |
| 31 | 669ns | -34.5% | -50.2% | -50.0% | -43.4% |
| 32 | 664ns | -33.8% | -49.7% | -49.8% | -42.9% |
| 33 | 655ns | -33.4% | -48.9% | -48.9% | -42.3% |
| 34 | 660ns | -34.1% | -49.1% | -49.2% | -42.5% |
| 35 | 660ns | -34.1% | -49.4% | -49.5% | -42.7% |
| 36 | 660ns | -34.2% | -49.4% | -49.2% | -42.9% |
| 37 | 658ns | -33.8% | -49.2% | -49.2% | -42.2% |
| 38 | 658ns | -33.6% | -49.2% | -49.1% | -42.2% |
| 39 | 657ns | -33.7% | -49.4% | -49.1% | -42.7% |
| 40 | 660ns | -33.7% | -49.4% | -49.2% | -42.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.791 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.175 | ok |
| warm-container-minimum | 0.853 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.091 | ok |
| warm-container-plusone | 0.861 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.6ns | 731.0ns | 0.4% |  |
| warm-container-kernel | 2.8ns | 430.6ns | 0.7% |  |
| warm-container-minimum | 3.0ns | 353.8ns | 0.8% |  |
| warm-container-native | 2.7ns | 344.3ns | 0.8% |  |
| warm-container-plusone | 2.7ns | 349.8ns | 0.8% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 658.6-868.5 ns)
    658.6 |##########################
    669.1 |########################################
    679.6 |
    690.1 |######################
    700.6 |#############
    711.1 |####
    721.5 |
    732.0 |
    742.5 |
    753.0 |
    763.5 |####
    774.0 |
    784.5 |
    795.0 |
    805.5 |
    816.0 |####
    826.5 |
    837.0 |
    847.5 |
    858.0 |##########################
  (4 below, 4 above range)

warm-container-kernel (n=40, range 415.4-457.3 ns)
    415.4 |########################################
    417.4 |
    419.5 |############
    421.6 |########
    423.7 |########
    425.8 |####
    427.9 |########
    430.0 |
    432.1 |
    434.2 |########################
    436.3 |################################
    438.4 |####
    440.5 |
    442.6 |
    444.7 |
    446.8 |
    448.9 |
    451.0 |
    453.1 |
    455.2 |
  (3 below, 2 above range)

warm-container-minimum (n=40, range 333.7-406.1 ns)
    333.7 |########################################
    337.3 |########################
    340.9 |##########
    344.5 |
    348.2 |
    351.8 |
    355.4 |
    359.0 |
    362.6 |
    366.3 |
    369.9 |
    373.5 |
    377.1 |
    380.7 |
    384.4 |
    388.0 |
    391.6 |
    395.2 |
    398.8 |##
    402.5 |##########
  (2 below, 5 above range)

warm-container-native (n=40, range 333.7-374.8 ns)
    333.7 |########################################
    335.7 |##
    337.8 |########
    339.8 |###########
    341.9 |##
    344.0 |
    346.0 |
    348.1 |##
    350.1 |####
    352.2 |########
    354.2 |##
    356.3 |
    358.3 |
    360.4 |
    362.5 |
    364.5 |
    366.6 |
    368.6 |
    370.7 |
    372.7 |
  (2 below, 1 above range)

warm-container-plusone (n=40, range 335.3-379.3 ns)
    335.3 |#########################
    337.5 |########################################
    339.7 |#########################
    341.9 |#####
    344.1 |##########
    346.3 |##########
    348.5 |#####
    350.7 |
    352.9 |
    355.1 |#####
    357.3 |
    359.5 |
    361.7 |
    363.9 |
    366.1 |
    368.3 |
    370.5 |
    372.7 |
    374.9 |#####
    377.1 |##############################
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.86 (measurement drift or warm-up artifact)
