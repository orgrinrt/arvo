# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 924 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 82% (significant)

warm-container-minimum is -3.38 us (82%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 4.4x slower than the field

warm-container-headroom (4.11 us) is 4.4x the fastest (924 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (924 ns) and warm-container-native (927 ns) differ by 0.31%, inside the noise, even though the wider field spreads 344.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-lanes-deferred, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (305% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-lanes-deferred, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 305% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest warm-container-minimum (924 ns) to slowest warm-container-headroom (4.11 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-minimum vs stability leader warm-container-kernel (+1% speed for 2.5x steadier)

warm-container-minimum is fastest (924 ns, CV 13.0%); warm-container-kernel gives up 0.8% median for 2.5x lower variance (CV 5.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.0x its best-20%

warm-container-plusone's best 20% of batches run at 1.58 us but its worst 20% at 6.29 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (8 ns, 0.19%)

warm-container-plusone differs from baseline warm-container-headroom by 8 ns (0.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 924.4 ns median (-77.5% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.45x (fastest 924.4 ns, slowest 4111.7 ns)

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
| Counter frequency | 2112787121 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3772ns | 4257ns | 1662ns | 3621ns | 6332ns | base |
| warm-container-kernel | 1186ns | 1159ns | 1136ns | 1163ns | 1308ns | -68.55% |
| warm-container-lanes-deferred | 1147ns | 1156ns | 1041ns | 1129ns | 1309ns | -69.58% |
| warm-container-minimum | 1065ns | 1153ns | 825ns | 1083ns | 1249ns | -71.77% |
| warm-container-native | 1125ns | 1154ns | 1016ns | 1124ns | 1237ns | -70.16% |
| warm-container-plusone | 3934ns | 3903ns | 1635ns | 3823ns | 6569ns | +4.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3631ns | 1606ns | 6076ns | base | 9.024 |
| warm-container-kernel | 948ns | 915ns | 1029ns | -73.89% | 34.563 |
| warm-container-lanes-deferred | 914ns | 838ns | 1015ns | -74.83% | 35.854 |
| warm-container-minimum | 848ns | 664ns | 982ns | -76.64% | 38.632 |
| warm-container-native | 896ns | 813ns | 967ns | -75.32% | 36.568 |
| warm-container-plusone | 3789ns | 1579ns | 6287ns | +4.34% | 8.648 |

## Performance model

- Peak throughput: **49.380 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.969 | 16.1% |
| warm-container-kernel | 35.176 | 71.2% |
| warm-container-lanes-deferred | 35.206 | 71.3% |
| warm-container-minimum | 35.448 | 71.8% |
| warm-container-native | 35.339 | 71.6% |
| warm-container-plusone | 8.691 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3772ns | 3772ns | base |
| warm-container-kernel | 1186ns | 1186ns | -68.55% |
| warm-container-lanes-deferred | 1147ns | 1147ns | -69.58% |
| warm-container-minimum | 1065ns | 1065ns | -71.77% |
| warm-container-native | 1125ns | 1125ns | -70.16% |
| warm-container-plusone | 3934ns | 3934ns | +4.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4112ns | base | --- | [1790, 4576] | --- | --- | --- | --- |
| warm-container-kernel | 932ns | -3179.1ns (-77.3%) | [-3629, -873]ns | [931, 933] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 931ns | -3245.6ns (-78.9%) | [-3658, -938]ns | [849, 932] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 924ns | -3380.4ns (-82.2%) | [-3782, -866]ns | [792, 929] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 927ns | -3319.3ns (-80.7%) | [-3703, -861]ns | [845, 929] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3770ns | +7.8ns (+0.2%) | [+1, +323]ns | [1792, 5387] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4245ns | -78.0% | -80.0% | -84.4% | -80.1% | -45.6% |
| 1 | 2 | 3276ns | -71.6% | -74.1% | -71.6% | -71.6% | -6.0% |
| 1 | 3 | 1796ns | -48.1% | -48.1% | -48.2% | -48.2% | +126.7% |
| 1 | 4 | 4275ns | -78.2% | -78.2% | -80.7% | -80.2% | +19.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.323 | moderate- |
| warm-container-kernel | -0.397 | moderate- |
| warm-container-lanes-deferred | 0.246 | moderate+ |
| warm-container-minimum | -0.181 | ok |
| warm-container-native | -0.250 | moderate- |
| warm-container-plusone | 0.257 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 48.7ns | 3631.4ns | 1.3% |  |
| warm-container-kernel | 87.0ns | 948.1ns | 9.2% | HIGH |
| warm-container-lanes-deferred | 84.6ns | 913.9ns | 9.3% | HIGH |
| warm-container-minimum | 78.1ns | 848.2ns | 9.2% | HIGH |
| warm-container-native | 80.7ns | 896.1ns | 9.0% | HIGH |
| warm-container-plusone | 50.6ns | 3788.9ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1605.6-6076.1 ns)
   1605.6 |########################################
   1829.1 |
   2052.6 |
   2276.1 |
   2499.7 |
   2723.2 |###
   2946.7 |
   3170.2 |
   3393.8 |
   3617.3 |
   3840.8 |##########
   4064.3 |#############
   4287.9 |
   4511.4 |#############
   4734.9 |##########
   4958.4 |######
   5182.0 |###
   5405.5 |
   5629.0 |###
   5852.5 |###
  (4 below, 4 above range)

warm-container-kernel (n=40, range 914.7-1029.0 ns)
    914.7 |
    920.5 |
    926.2 |########################################
    931.9 |#####################
    937.6 |##
    943.3 |
    949.0 |
    954.8 |
    960.5 |##
    966.2 |
    971.9 |
    977.6 |
    983.3 |##
    989.0 |
    994.8 |
   1000.5 |
   1006.2 |
   1011.9 |##
   1017.6 |##
   1023.3 |##
  (2 below, 3 above range)

warm-container-lanes-deferred (n=40, range 837.9-1014.6 ns)
    837.9 |#######
    846.7 |#########################
    855.6 |
    864.4 |
    873.2 |
    882.1 |
    890.9 |##
    899.7 |
    908.6 |
    917.4 |
    926.2 |########################################
    935.1 |
    943.9 |
    952.7 |
    961.6 |
    970.4 |
    979.2 |##
    988.1 |
    996.9 |
   1005.7 |
  (1 below, 6 above range)

warm-container-minimum (n=40, range 663.6-981.6 ns)
    663.6 |#######
    679.5 |
    695.4 |##
    711.3 |##
    727.2 |
    743.1 |
    759.0 |#####
    774.9 |#####
    790.8 |
    806.7 |##
    822.6 |
    838.5 |##########
    854.4 |
    870.3 |
    886.2 |
    902.1 |
    918.0 |########################################
    933.9 |
    949.8 |
    965.7 |##
  (5 below, 4 above range)

warm-container-native (n=40, range 813.4-967.2 ns)
    813.4 |###
    821.1 |
    828.8 |
    836.5 |############
    844.2 |########################
    851.9 |
    859.6 |
    867.3 |
    875.0 |
    882.6 |
    890.3 |
    898.0 |
    905.7 |###
    913.4 |
    921.1 |##################
    928.8 |########################################
    936.5 |
    944.2 |
    951.9 |
    959.6 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1579.2-6287.2 ns)
   1579.2 |########################################
   1814.6 |
   2050.0 |
   2285.4 |
   2520.8 |
   2756.2 |
   2991.6 |######
   3227.0 |
   3462.4 |
   3697.8 |
   3933.2 |
   4168.6 |###
   4404.0 |
   4639.4 |###
   4874.8 |###
   5110.2 |###
   5345.6 |#########
   5581.0 |#########
   5816.4 |###############
   6051.8 |###
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=48.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
