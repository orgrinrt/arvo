# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 3.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 28% (significant)

warm-container-kernel is -1.23 us (28%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.81)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-container-native vs stability leader warm-container-kernel (+1% speed for 1.2x steadier)

warm-container-native is fastest (3.19 us, CV 20.3%); warm-container-kernel gives up 1.3% median for 1.2x lower variance (CV 16.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.5x its best-20%

warm-container-plusone's best 20% of batches run at 1.43 us but its worst 20% at 6.43 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 3190.2 ns median (-28.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.39x (fastest 3190.2 ns, slowest 4432.1 ns)

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
| warm-container-headroom | 4003ns | 4643ns | 1484ns | 4172ns | 6013ns | base |
| warm-container-kernel | 3322ns | 3440ns | 2437ns | 3393ns | 3995ns | -17.01% |
| warm-container-lanes-deferred | 3418ns | 3440ns | 2431ns | 3412ns | 4423ns | -14.60% |
| warm-container-minimum | 3456ns | 3781ns | 1983ns | 3701ns | 4193ns | -13.66% |
| warm-container-native | 3283ns | 3394ns | 2256ns | 3379ns | 4021ns | -17.99% |
| warm-container-plusone | 3752ns | 3590ns | 1493ns | 3497ns | 6776ns | -6.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3811ns | 1417ns | 5700ns | base | 8.597 |
| warm-container-kernel | 3108ns | 2287ns | 3711ns | -18.45% | 10.542 |
| warm-container-lanes-deferred | 3202ns | 2286ns | 4121ns | -16.00% | 10.235 |
| warm-container-minimum | 3237ns | 1864ns | 3897ns | -15.07% | 10.122 |
| warm-container-native | 3072ns | 2120ns | 3739ns | -19.39% | 10.665 |
| warm-container-plusone | 3571ns | 1426ns | 6427ns | -6.31% | 9.176 |

## Performance model

- Peak throughput: **23.125 Gops/s** (warm-container-headroom; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.393 | 32.0% |
| warm-container-kernel | 10.136 | 43.8% |
| warm-container-lanes-deferred | 10.140 | 43.8% |
| warm-container-minimum | 9.218 | 39.9% |
| warm-container-native | 10.271 | 44.4% |
| warm-container-plusone | 9.555 | 41.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4003ns | 4003ns | base |
| warm-container-kernel | 3322ns | 3322ns | -17.01% |
| warm-container-lanes-deferred | 3418ns | 3418ns | -14.60% |
| warm-container-minimum | 3456ns | 3456ns | -13.66% |
| warm-container-native | 3283ns | 3283ns | -17.99% |
| warm-container-plusone | 3752ns | 3752ns | -6.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4432ns | base | --- | [3907, 4718] | --- | --- | --- | --- |
| warm-container-kernel | 3233ns | -1231.7ns (-27.8%) | [-1555, -621]ns | [2965, 3435] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-lanes-deferred | 3231ns | -884.0ns (-19.9%) | [-1364, -399]ns | [2876, 3556] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-minimum | 3555ns | -849.6ns (-19.2%) | [-1058, -430]ns | [3390, 3576] | YES | 0.0034 | 0.0007 | 0 |
| warm-container-native | 3190ns | -1102.5ns (-24.9%) | [-1501, -881]ns | [2976, 3478] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-plusone | 3430ns | no significant difference | [-2, +157]ns | [1732, 4731] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4153ns | -14.4% | -9.1% | -13.9% | -35.2% | -24.3% |
| 1 | 2 | 4534ns | -28.7% | -42.0% | -21.5% | -22.8% | -21.8% |
| 1 | 3 | 5278ns | -36.1% | -32.6% | -36.6% | -32.1% | -43.3% |
| 1 | 4 | 4159ns | -38.4% | -27.0% | -14.5% | -25.7% | -16.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.330 | moderate- |
| warm-container-kernel | -0.159 | ok |
| warm-container-lanes-deferred | -0.727 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.346 | moderate- |
| warm-container-native | -0.181 | ok |
| warm-container-plusone | -0.805 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 54.4ns | 3811.5ns | 1.4% |  |
| warm-container-kernel | 75.6ns | 3108.4ns | 2.4% |  |
| warm-container-lanes-deferred | 80.4ns | 3201.7ns | 2.5% |  |
| warm-container-minimum | 79.1ns | 3237.2ns | 2.4% |  |
| warm-container-native | 76.2ns | 3072.5ns | 2.5% |  |
| warm-container-plusone | 51.0ns | 3571.1ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1417.0-5700.4 ns)
   1417.0 |########################################
   1631.1 |####################
   1845.3 |
   2059.5 |
   2273.7 |######
   2487.8 |
   2702.0 |
   2916.2 |
   3130.3 |
   3344.5 |
   3558.7 |
   3772.8 |#############
   3987.0 |#############
   4201.2 |####################
   4415.3 |##########################
   4629.5 |##########################
   4843.7 |######
   5057.9 |#################################
   5272.0 |######
   5486.2 |#############
  (3 below, 3 above range)

warm-container-kernel (n=40, range 2286.6-3710.8 ns)
   2286.6 |
   2357.8 |###########
   2429.0 |#####
   2500.2 |###########
   2571.4 |#####
   2642.6 |#####
   2713.9 |###########
   2785.1 |
   2856.3 |
   2927.5 |##################################
   2998.7 |#####
   3069.9 |
   3141.1 |#####
   3212.3 |##################################
   3283.5 |
   3354.7 |#####
   3425.9 |#####
   3497.1 |########################################
   3568.4 |###########
   3639.6 |#####
  (2 below, 3 above range)

warm-container-lanes-deferred (n=40, range 2285.8-4121.0 ns)
   2285.8 |##########
   2377.6 |
   2469.3 |##########
   2561.1 |#####
   2652.9 |#########################
   2744.6 |##########
   2836.4 |
   2928.1 |##########
   3019.9 |##########
   3111.6 |
   3203.4 |#########################
   3295.1 |
   3386.9 |
   3478.7 |########################################
   3570.4 |##########
   3662.2 |
   3753.9 |
   3845.7 |
   3937.4 |####################
   4029.2 |
  (2 below, 3 above range)

warm-container-minimum (n=40, range 1863.7-3896.9 ns)
   1863.7 |
   1965.4 |
   2067.0 |
   2168.7 |
   2270.4 |
   2372.0 |
   2473.7 |#######
   2575.3 |
   2677.0 |
   2778.7 |
   2880.3 |#######
   2982.0 |###
   3083.7 |###
   3185.3 |##########
   3287.0 |###
   3388.6 |###
   3490.3 |########################################
   3592.0 |#########################
   3693.6 |###
   3795.3 |###
  (4 below, 5 above range)

warm-container-native (n=40, range 2119.6-3738.6 ns)
   2119.6 |######
   2200.5 |
   2281.5 |
   2362.4 |
   2443.4 |######
   2524.3 |####################
   2605.3 |#############
   2686.2 |#############
   2767.2 |######
   2848.1 |######
   2929.1 |##########################
   3010.0 |######
   3091.0 |######
   3171.9 |#################################
   3252.9 |######
   3333.8 |
   3414.8 |######
   3495.7 |########################################
   3576.7 |##########################
   3657.6 |######
  (2 below, 3 above range)

warm-container-plusone (n=40, range 1426.1-6427.1 ns)
   1426.1 |########
   1676.1 |########################################
   1926.2 |
   2176.2 |
   2426.3 |########
   2676.3 |
   2926.4 |
   3176.4 |
   3426.5 |
   3676.5 |
   3926.6 |
   4176.6 |########
   4426.7 |################
   4676.7 |########
   4926.8 |########
   5176.8 |####
   5426.9 |
   5676.9 |############
   5927.0 |
   6177.0 |################
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=43.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=21.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=25.4% (high variance, measurements may be unstable)
- **warm-container-native**: CV=21.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=56.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.5x (possible bimodal distribution)
