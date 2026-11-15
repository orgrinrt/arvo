# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 61% (significant)

warm-container-minimum is -206 ns (61%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.8x slower than the field

warm-container-plusone (364 ns) is 2.8x the fastest (130 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (130 ns) and warm-container-native (130 ns) differ by 0.00%, inside the noise, even though the wider field spreads 180.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.88)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-headroom, warm-container-plusone} (157% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone} with a 157% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+0% speed for 7.0x steadier)

warm-container-minimum is fastest (130 ns, CV 9.5%); warm-container-native gives up 0.0% median for 7.0x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: warm-container-minimum** at 129.6 ns median (-61.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.81x (fastest 129.6 ns, slowest 363.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 428ns | 397ns | 393ns | 410ns | 520ns | base |
| warm-container-kernel | 196ns | 192ns | 189ns | 192ns | 215ns | -54.21% |
| warm-container-minimum | 194ns | 192ns | 188ns | 192ns | 206ns | -54.67% |
| warm-container-native | 192ns | 192ns | 188ns | 191ns | 197ns | -55.26% |
| warm-container-plusone | 435ns | 428ns | 399ns | 430ns | 484ns | +1.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 362ns | 333ns | 439ns | base | 113.162 |
| warm-container-kernel | 135ns | 129ns | 151ns | -62.83% | 304.433 |
| warm-container-minimum | 132ns | 128ns | 142ns | -63.54% | 310.356 |
| warm-container-native | 130ns | 128ns | 133ns | -64.09% | 315.083 |
| warm-container-plusone | 367ns | 339ns | 405ns | +1.32% | 111.683 |

## Performance model

- Peak throughput: **320.219 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 121.814 | 38.0% |
| warm-container-kernel | 313.150 | 97.8% |
| warm-container-minimum | 316.049 | 98.7% |
| warm-container-native | 316.049 | 98.7% |
| warm-container-plusone | 112.605 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 428ns | 428ns | base |
| warm-container-kernel | 196ns | 196ns | -54.21% |
| warm-container-minimum | 194ns | 194ns | -54.67% |
| warm-container-native | 192ns | 192ns | -55.26% |
| warm-container-plusone | 435ns | 435ns | +1.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 336ns | base | --- | [334, 342] | --- | --- | --- | --- |
| warm-container-kernel | 131ns | -204.5ns (-60.8%) | [-209, -204]ns | [130, 131] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 130ns | -205.8ns (-61.2%) | [-210, -205]ns | [129, 130] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 130ns | -206.4ns (-61.4%) | [-211, -205]ns | [129, 130] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 364ns | no significant difference | [-2, +45]ns | [341, 378] | no | 0.3368 | 0.3368 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 333ns | -60.8% | -61.7% | -61.4% | +13.4% |
| 2 | 333ns | -60.6% | -61.1% | -60.9% | +13.6% |
| 3 | 334ns | -60.8% | -61.2% | -61.4% | +13.0% |
| 4 | 334ns | -60.8% | -37.6% | -60.7% | +13.0% |
| 5 | 333ns | -60.8% | -61.6% | -60.9% | +14.6% |
| 6 | 333ns | -60.9% | -61.5% | -60.5% | +12.6% |
| 7 | 333ns | -61.4% | -61.0% | -61.1% | +13.3% |
| 8 | 334ns | -60.8% | -61.6% | -61.2% | +13.8% |
| 9 | 334ns | -61.1% | -61.4% | -61.3% | +13.0% |
| 10 | 333ns | -61.4% | -61.2% | -60.8% | +14.1% |
| 11 | 436ns | -69.7% | -70.3% | -70.3% | -19.9% |
| 12 | 432ns | -70.0% | -70.1% | -70.2% | -21.5% |
| 13 | 438ns | -69.9% | -70.3% | -70.5% | -22.5% |
| 14 | 434ns | -69.8% | -70.1% | -70.2% | -22.4% |
| 15 | 441ns | -70.6% | -70.6% | -70.5% | -22.0% |
| 16 | 440ns | -70.2% | -70.8% | -70.5% | -20.0% |
| 17 | 438ns | -70.0% | -70.6% | -70.6% | -19.7% |
| 18 | 437ns | -70.0% | -70.1% | -70.4% | -20.4% |
| 19 | 439ns | -70.3% | -70.4% | -71.0% | -22.8% |
| 20 | 440ns | -70.4% | -70.7% | -70.8% | -23.0% |
| 21 | 338ns | -59.4% | -60.6% | -61.1% | +0.7% |
| 22 | 340ns | -22.2% | -60.6% | -61.2% | +0.5% |
| 23 | 340ns | -60.6% | -61.4% | -61.6% | -0.1% |
| 24 | 340ns | -61.2% | -60.9% | -61.0% | -0.4% |
| 25 | 343ns | -61.0% | -61.5% | -61.9% | -0.7% |
| 26 | 340ns | -61.5% | -61.5% | -60.7% | +0.4% |
| 27 | 350ns | -62.5% | -61.4% | -62.3% | -3.2% |
| 28 | 352ns | -62.2% | -62.4% | -61.5% | -3.6% |
| 29 | 352ns | -62.3% | -62.9% | -62.2% | -3.2% |
| 30 | 340ns | -57.6% | -61.4% | -61.5% | +0.0% |
| 31 | 333ns | -61.2% | -61.2% | -60.9% | +21.3% |
| 32 | 333ns | -60.7% | -61.8% | -61.2% | +22.0% |
| 33 | 334ns | -61.6% | -61.1% | -61.6% | +21.1% |
| 34 | 333ns | -61.4% | -60.9% | -61.6% | +21.3% |
| 35 | 333ns | -60.9% | -61.6% | -61.9% | +21.4% |
| 36 | 334ns | -61.2% | -61.6% | -61.7% | +20.9% |
| 37 | 334ns | -61.0% | -61.3% | -61.3% | +20.8% |
| 38 | 333ns | -60.6% | -60.6% | -60.6% | +22.5% |
| 39 | 333ns | -61.5% | -60.6% | -61.6% | +22.3% |
| 40 | 334ns | -60.9% | -61.3% | -61.7% | +20.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.851 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.042 | ok |
| warm-container-minimum | -0.050 | ok |
| warm-container-native | 0.578 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.880 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 16/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.2ns | 362.0ns | 0.6% |  |
| warm-container-kernel | 1.8ns | 134.5ns | 1.4% |  |
| warm-container-minimum | 2.8ns | 132.0ns | 2.1% |  |
| warm-container-native | 2.4ns | 130.0ns | 1.8% |  |
| warm-container-plusone | 2.5ns | 366.8ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 333.1-438.6 ns)
    333.1 |########################################
    338.4 |##############
    343.7 |
    348.9 |#######
    354.2 |
    359.5 |
    364.8 |
    370.0 |
    375.3 |
    380.6 |
    385.9 |
    391.1 |
    396.4 |
    401.7 |
    407.0 |
    412.3 |
    417.5 |
    422.8 |
    428.1 |##
    433.4 |###########
  (4 below, 4 above range)

warm-container-kernel (n=40, range 128.9-151.4 ns)
    128.9 |###############
    130.0 |########################################
    131.2 |##############################
    132.3 |######
    133.4 |######
    134.5 |
    135.7 |
    136.8 |###
    137.9 |
    139.0 |
    140.2 |
    141.3 |
    142.4 |
    143.5 |###
    144.6 |
    145.8 |
    146.9 |
    148.0 |
    149.1 |
    150.3 |
  (5 below, 1 above range)

warm-container-minimum (n=40, range 127.9-142.3 ns)
    127.9 |#################
    128.7 |###################################
    129.4 |########################################
    130.1 |#############
    130.8 |######################
    131.5 |########
    132.3 |####
    133.0 |####
    133.7 |####
    134.4 |####
    135.1 |
    135.9 |
    136.6 |
    137.3 |
    138.0 |
    138.7 |
    139.5 |
    140.2 |
    140.9 |
    141.6 |
  (4 below, 1 above range)

warm-container-native (n=40, range 127.9-132.7 ns)
    127.9 |
    128.2 |######
    128.4 |
    128.6 |#################################
    128.9 |
    129.1 |########################################
    129.4 |
    129.6 |####################
    129.8 |##########################
    130.1 |
    130.3 |#############
    130.6 |
    130.8 |####################
    131.0 |#############
    131.3 |
    131.5 |####################
    131.8 |
    132.0 |######
    132.2 |
    132.5 |######
  (6 below, 3 above range)

warm-container-plusone (n=40, range 338.7-405.3 ns)
    338.7 |########################################
    342.1 |###
    345.4 |###
    348.7 |######
    352.0 |###
    355.4 |
    358.7 |
    362.0 |
    365.4 |
    368.7 |
    372.0 |
    375.3 |#####################
    378.7 |######
    382.0 |###
    385.3 |
    388.7 |
    392.0 |
    395.3 |
    398.6 |
    402.0 |#####################
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.88 (measurement drift or warm-up artifact)
