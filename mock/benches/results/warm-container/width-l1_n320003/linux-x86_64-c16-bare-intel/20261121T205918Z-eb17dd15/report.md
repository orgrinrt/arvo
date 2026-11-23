# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 1.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-plusone dominates: 78% faster than the next best (warm-container-minimum)

warm-container-plusone (1.73 us) leads warm-container-minimum (3.08 us) by 78%, a clear separation rather than a photo finish. CV 123.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 26% (significant)

warm-container-native is -1.22 us (26%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.8x slower than the field

warm-container-headroom (4.77 us) is 2.8x the fastest (1.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone is fastest but the noisiest (CV 123.0%)

warm-container-plusone wins on median (1.73 us) yet has the highest variance (CV 123.0%), while warm-container-lanes-deferred is the steadiest (CV 13.4%, 3.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.78)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-plusone} vs {warm-container-minimum, warm-container-lanes-deferred, warm-container-kernel, warm-container-native, warm-container-headroom} (78% apart)

The field splits into a fast tier {warm-container-plusone} and a slow tier {warm-container-minimum, warm-container-lanes-deferred, warm-container-kernel, warm-container-native, warm-container-headroom} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 4.7x its best-20%

warm-container-plusone's best 20% of batches run at 1.42 us but its worst 20% at 6.68 us (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 1732.2 ns median (-63.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.75x (fastest 1732.2 ns, slowest 4766.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112855779 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4015ns | 4990ns | 1813ns | 3887ns | 6598ns | base |
| warm-container-kernel | 3179ns | 3369ns | 1921ns | 3358ns | 3900ns | -20.82% |
| warm-container-lanes-deferred | 3278ns | 3369ns | 2599ns | 3344ns | 3758ns | -18.35% |
| warm-container-minimum | 3246ns | 3287ns | 2298ns | 3329ns | 3944ns | -19.15% |
| warm-container-native | 3309ns | 3796ns | 1218ns | 3665ns | 4332ns | -17.58% |
| warm-container-plusone | 3195ns | 1813ns | 1483ns | 2478ns | 7060ns | -20.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3825ns | 1732ns | 6252ns | base | 8.567 |
| warm-container-kernel | 2974ns | 1804ns | 3619ns | -22.24% | 11.017 |
| warm-container-lanes-deferred | 3067ns | 2439ns | 3499ns | -19.82% | 10.684 |
| warm-container-minimum | 3039ns | 2160ns | 3662ns | -20.54% | 10.782 |
| warm-container-native | 3097ns | 1145ns | 4018ns | -19.03% | 10.580 |
| warm-container-plusone | 3037ns | 1418ns | 6679ns | -20.59% | 10.788 |

## Performance model

- Peak throughput: **28.621 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.875 | 24.0% |
| warm-container-kernel | 10.355 | 36.2% |
| warm-container-lanes-deferred | 10.356 | 36.2% |
| warm-container-minimum | 10.639 | 37.2% |
| warm-container-native | 9.179 | 32.1% |
| warm-container-plusone | 18.916 | 66.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4015ns | 4015ns | base |
| warm-container-kernel | 3179ns | 3179ns | -20.82% |
| warm-container-lanes-deferred | 3278ns | 3278ns | -18.35% |
| warm-container-minimum | 3246ns | 3246ns | -19.15% |
| warm-container-native | 3309ns | 3309ns | -17.58% |
| warm-container-plusone | 3195ns | 3195ns | -20.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4766ns | base | --- | [1744, 5347] | --- | --- | --- | --- |
| warm-container-kernel | 3165ns | no significant difference | [-1841, +539]ns | [2943, 3388] | no | 0.4470 | 0.2682 | 0 |
| warm-container-lanes-deferred | 3164ns | no significant difference | [-2196, +939]ns | [2901, 3212] | no | 0.6358 | 0.6358 | 0 |
| warm-container-minimum | 3080ns | no significant difference | [-1969, +780]ns | [2955, 3273] | no | 0.5370 | 0.4296 | 0 |
| warm-container-native | 3570ns | -1216.0ns (-25.5%) | [-1626, -740]ns | [3265, 3588] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-plusone | 1732ns | -315.4ns (-6.6%) | [-635, -304]ns | [1418, 3273] | YES | 0.0034 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1738ns | +97.0% | +92.6% | +82.2% | +105.5% | -0.3% |
| 1 | 2 | 4786ns | -32.9% | -39.9% | -34.4% | -25.5% | -70.4% |
| 1 | 3 | 4981ns | -33.6% | -33.1% | -28.9% | -28.2% | -15.6% |
| 1 | 4 | 3866ns | -30.3% | -21.8% | -28.9% | -11.9% | -63.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.134 | ok |
| warm-container-kernel | -0.152 | ok |
| warm-container-lanes-deferred | -0.783 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.512 | HIGH- (thermal bounce) |
| warm-container-native | -0.112 | ok |
| warm-container-plusone | -0.506 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 53.4ns | 3824.9ns | 1.4% |  |
| warm-container-kernel | 74.4ns | 2974.2ns | 2.5% |  |
| warm-container-lanes-deferred | 79.0ns | 3067.0ns | 2.6% |  |
| warm-container-minimum | 72.5ns | 3039.2ns | 2.4% |  |
| warm-container-native | 73.3ns | 3097.2ns | 2.4% |  |
| warm-container-plusone | 42.6ns | 3037.3ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1732.2-6252.2 ns)
   1732.2 |########################################
   1958.2 |##
   2184.2 |##
   2410.2 |
   2636.2 |
   2862.2 |
   3088.2 |
   3314.2 |
   3540.2 |
   3766.2 |
   3992.2 |
   4218.2 |
   4444.2 |##
   4670.2 |##
   4896.2 |#####
   5122.2 |########
   5348.2 |#####
   5574.2 |########
   5800.2 |##
   6026.2 |##############
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1803.8-3619.4 ns)
   1803.8 |
   1894.6 |
   1985.4 |
   2076.2 |
   2166.9 |
   2257.7 |
   2348.5 |
   2439.3 |###############
   2530.1 |
   2620.9 |##########
   2711.6 |#####
   2802.4 |#####
   2893.2 |#########################
   2984.0 |#####
   3074.8 |####################
   3165.5 |###############
   3256.3 |#####
   3347.1 |##########
   3437.9 |########################################
   3528.7 |##########
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 2439.2-3499.1 ns)
   2439.2 |
   2492.2 |
   2545.2 |####
   2598.2 |
   2651.1 |######################
   2704.1 |
   2757.1 |
   2810.1 |####
   2863.1 |##########################
   2916.1 |
   2969.1 |####
   3022.1 |
   3075.1 |####
   3128.1 |###################################
   3181.1 |########
   3234.1 |####
   3287.1 |
   3340.1 |
   3393.1 |
   3446.1 |########################################
  (2 below, 3 above range)

warm-container-minimum (n=40, range 2159.8-3661.7 ns)
   2159.8 |
   2234.9 |#####
   2310.0 |
   2385.1 |
   2460.2 |#################
   2535.3 |#####
   2610.4 |
   2685.5 |###########
   2760.6 |#################
   2835.7 |
   2910.8 |######################
   2985.9 |######################
   3061.0 |###########
   3136.1 |
   3211.2 |############################
   3286.3 |
   3361.3 |
   3436.4 |#####
   3511.5 |########################################
   3586.6 |#################
  (2 below, 2 above range)

warm-container-native (n=40, range 1144.9-4017.9 ns)
   1144.9 |
   1288.5 |
   1432.2 |
   1575.8 |########
   1719.5 |
   1863.1 |
   2006.8 |
   2150.4 |
   2294.1 |
   2437.7 |####
   2581.4 |
   2725.0 |
   2868.7 |####
   3012.3 |
   3156.0 |######################
   3299.6 |
   3443.3 |########################################
   3586.9 |###################################
   3730.6 |####
   3874.2 |######################
  (6 below, 2 above range)

warm-container-plusone (n=40, range 1417.5-6679.0 ns)
   1417.5 |########################################
   1680.6 |##################
   1943.7 |
   2206.7 |###
   2469.8 |###
   2732.9 |
   2996.0 |
   3259.0 |
   3522.1 |
   3785.2 |######
   4048.2 |
   4311.3 |###
   4574.4 |######
   4837.5 |###
   5100.5 |###
   5363.6 |
   5626.7 |
   5889.7 |
   6152.8 |######
   6415.9 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-kernel**: CV=24.7% (high variance, measurements may be unstable)
- **warm-container-native**: CV=33.3% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=70.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.7x (possible bimodal distribution)
