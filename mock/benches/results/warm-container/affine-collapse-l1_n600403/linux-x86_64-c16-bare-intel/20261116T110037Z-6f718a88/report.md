# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 1.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 42% (significant)

warm-container-native is -1.19 us (42%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is fastest but the noisiest (CV 114.8%)

warm-container-plusone wins on median (1.87 us) yet has the highest variance (CV 114.8%), while warm-container-headroom is the steadiest (CV 76.9%, 2.82 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-plusone, warm-container-minimum) are a dead heat (<1%)

warm-container-plusone (1.87 us) and warm-container-minimum (1.88 us) differ by 0.64%, inside the noise, even though the wider field spreads 50.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (952 ns) is smaller than the fastest variant's own run-to-run std-dev (2.15 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-plusone, warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom} (42% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-minimum, warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 4.5x its best-20%

warm-container-minimum's best 20% of batches run at 1.49 us but its worst 20% at 6.65 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 1869.9 ns median (-33.7% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.51x (fastest 1869.9 ns, slowest 2821.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112700793 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4140ns | 2888ns | 2876ns | 3157ns | 8350ns | base |
| warm-container-kernel | 3471ns | 2067ns | 1690ns | 2890ns | 6998ns | -16.14% |
| warm-container-lanes-deferred | 3584ns | 2073ns | 1692ns | 3068ns | 7027ns | -13.41% |
| warm-container-minimum | 3448ns | 1963ns | 1561ns | 2890ns | 7008ns | -16.71% |
| warm-container-native | 3582ns | 2066ns | 1689ns | 2990ns | 7248ns | -13.48% |
| warm-container-plusone | 3687ns | 1951ns | 1835ns | 3039ns | 7484ns | -10.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4035ns | 2810ns | 8115ns | base | 8.122 |
| warm-container-kernel | 3322ns | 1622ns | 6660ns | -17.66% | 9.864 |
| warm-container-lanes-deferred | 3431ns | 1626ns | 6695ns | -14.96% | 9.551 |
| warm-container-minimum | 3290ns | 1495ns | 6653ns | -18.45% | 9.959 |
| warm-container-native | 3428ns | 1622ns | 6914ns | -15.05% | 9.560 |
| warm-container-plusone | 3517ns | 1756ns | 7109ns | -12.84% | 9.318 |

## Performance model

- Peak throughput: **21.919 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.613 | 53.0% |
| warm-container-kernel | 16.508 | 75.3% |
| warm-container-lanes-deferred | 16.452 | 75.1% |
| warm-container-minimum | 17.412 | 79.4% |
| warm-container-native | 16.537 | 75.4% |
| warm-container-plusone | 17.524 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4140ns | 4140ns | base |
| warm-container-kernel | 3471ns | 3471ns | -16.14% |
| warm-container-lanes-deferred | 3584ns | 3584ns | -13.41% |
| warm-container-minimum | 3448ns | 3448ns | -16.71% |
| warm-container-native | 3582ns | 3582ns | -13.48% |
| warm-container-plusone | 3687ns | 3687ns | -10.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2822ns | base | --- | [2811, 3435] | --- | --- | --- | --- |
| warm-container-kernel | 1985ns | -1186.8ns (-42.1%) | [-1196, -827]ns | [1633, 4465] | YES | 0.0008 | 0.0007 | 0 |
| warm-container-lanes-deferred | 1992ns | -1161.8ns (-41.2%) | [-1188, -846]ns | [1975, 4592] | YES | 0.0008 | 0.0007 | 0 |
| warm-container-minimum | 1882ns | -988.6ns (-35.0%) | [-1294, -933]ns | [1823, 4329] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-native | 1982ns | -1187.7ns (-42.1%) | [-1199, -841]ns | [1976, 3873] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-plusone | 1870ns | -952.7ns (-33.8%) | [-1152, -940]ns | [1864, 4207] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2811ns | -11.6% | -10.7% | -32.9% | -2.0% | -8.3% |
| 1 | 2 | 2821ns | +58.3% | +62.8% | +25.7% | -29.6% | +49.1% |
| 1 | 3 | 3435ns | -52.5% | -42.0% | -56.5% | -41.8% | -45.7% |
| 1 | 4 | 2815ns | -29.5% | -30.0% | -34.2% | -42.4% | -33.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.409 | moderate- |
| warm-container-kernel | -0.305 | moderate- |
| warm-container-lanes-deferred | -0.273 | moderate- |
| warm-container-minimum | -0.443 | moderate- |
| warm-container-native | -0.025 | ok |
| warm-container-plusone | -0.191 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.2ns | 4034.6ns | 0.7% |  |
| warm-container-kernel | 40.9ns | 3322.1ns | 1.2% |  |
| warm-container-lanes-deferred | 42.1ns | 3431.0ns | 1.2% |  |
| warm-container-minimum | 44.3ns | 3290.3ns | 1.3% |  |
| warm-container-native | 42.0ns | 3427.6ns | 1.2% |  |
| warm-container-plusone | 48.1ns | 3516.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2810.2-8115.5 ns)
   2810.2 |########################################
   3075.5 |
   3340.7 |######################
   3606.0 |
   3871.2 |
   4136.5 |
   4401.8 |##
   4667.0 |
   4932.3 |
   5197.6 |
   5462.8 |
   5728.1 |
   5993.4 |
   6258.6 |
   6523.9 |
   6789.1 |
   7054.4 |##
   7319.7 |
   7584.9 |
   7850.2 |##
  (4 below, 5 above range)

warm-container-kernel (n=40, range 1622.5-6659.7 ns)
   1622.5 |########################################
   1874.3 |################################
   2126.2 |
   2378.1 |
   2629.9 |
   2881.8 |
   3133.7 |###
   3385.5 |
   3637.4 |
   3889.2 |
   4141.1 |###
   4393.0 |###
   4644.8 |#######
   4896.7 |#######
   5148.5 |
   5400.4 |##########
   5652.3 |
   5904.1 |###
   6156.0 |###
   6407.9 |###
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1625.6-6694.7 ns)
   1625.6 |#####
   1879.0 |########################################
   2132.5 |##
   2385.9 |
   2639.4 |
   2892.8 |
   3146.3 |##
   3399.7 |
   3653.2 |
   3906.7 |
   4160.1 |
   4413.6 |##
   4667.0 |#####
   4920.5 |##
   5173.9 |##
   5427.4 |##########
   5680.9 |
   5934.3 |##
   6187.8 |
   6441.2 |##
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1494.9-6653.2 ns)
   1494.9 |###############
   1752.8 |########################################
   2010.7 |######
   2268.7 |
   2526.6 |
   2784.5 |
   3042.4 |
   3300.3 |
   3558.2 |###
   3816.1 |
   4074.0 |
   4331.9 |
   4589.9 |
   4847.8 |######
   5105.7 |######
   5363.6 |######
   5621.5 |######
   5879.4 |
   6137.3 |######
   6395.2 |###
  (5 below, 3 above range)

warm-container-native (n=40, range 1622.5-6913.5 ns)
   1622.5 |####################
   1887.0 |########################################
   2151.6 |
   2416.1 |
   2680.7 |
   2945.2 |
   3209.8 |
   3474.3 |######
   3738.9 |###
   4003.4 |
   4268.0 |###
   4532.5 |
   4797.1 |
   5061.6 |
   5326.2 |######
   5590.7 |
   5855.3 |###
   6119.8 |######
   6384.4 |###
   6648.9 |######
  (6 below, 4 above range)

warm-container-plusone (n=40, range 1756.0-7109.0 ns)
   1756.0 |########################################
   2023.6 |
   2291.3 |
   2558.9 |
   2826.6 |
   3094.2 |##
   3361.9 |
   3629.5 |
   3897.2 |##
   4164.8 |##########
   4432.5 |##
   4700.1 |##
   4967.8 |
   5235.4 |##
   5503.1 |##
   5770.7 |
   6038.4 |##
   6306.0 |
   6573.7 |##
   6841.3 |##
  (2 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=62.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.0% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=64.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-native**: CV=62.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=61.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
