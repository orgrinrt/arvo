# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.45 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 841 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum dominates: 18% faster than the next best (warm-container-native)

warm-container-minimum (841 ns) leads warm-container-native (990 ns) by 18%, a clear separation rather than a photo finish. CV 35.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 77% (significant)

warm-container-native is -3.42 us (77%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 5.3x slower than the field

warm-container-headroom (4.45 us) is 5.3x the fastest (841 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.63)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-lanes-deferred, warm-container-kernel, warm-container-plusone, warm-container-headroom} (110% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-lanes-deferred, warm-container-kernel, warm-container-plusone, warm-container-headroom} with a 110% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.3x the fastest

Fastest warm-container-minimum (841 ns) to slowest warm-container-headroom (4.45 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.68 us but its worst 20% at 6.94 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 840.9 ns median (-81.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.29x (fastest 840.9 ns, slowest 4451.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4077ns | 4608ns | 1843ns | 3929ns | 6755ns | base |
| warm-container-kernel | 3942ns | 2192ns | 2026ns | 3440ns | 7362ns | -3.31% |
| warm-container-lanes-deferred | 3817ns | 2144ns | 1737ns | 3379ns | 7211ns | -6.37% |
| warm-container-minimum | 1235ns | 1051ns | 908ns | 1126ns | 1890ns | -69.70% |
| warm-container-native | 1247ns | 1239ns | 1155ns | 1227ns | 1402ns | -69.41% |
| warm-container-plusone | 3910ns | 3279ns | 1845ns | 3602ns | 6900ns | -4.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3925ns | 1780ns | 6480ns | base | 8.348 |
| warm-container-kernel | 3812ns | 1965ns | 7100ns | -2.89% | 8.596 |
| warm-container-lanes-deferred | 3689ns | 1684ns | 6936ns | -6.02% | 8.883 |
| warm-container-minimum | 971ns | 728ns | 1497ns | -75.27% | 33.756 |
| warm-container-native | 993ns | 928ns | 1089ns | -74.71% | 33.008 |
| warm-container-plusone | 3765ns | 1780ns | 6620ns | -4.07% | 8.702 |

## Performance model

- Peak throughput: **45.015 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.362 | 16.4% |
| warm-container-kernel | 15.416 | 34.2% |
| warm-container-lanes-deferred | 15.750 | 35.0% |
| warm-container-minimum | 38.968 | 86.6% |
| warm-container-native | 33.091 | 73.5% |
| warm-container-plusone | 10.345 | 23.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4077ns | 4077ns | base |
| warm-container-kernel | 3942ns | 3942ns | -3.31% |
| warm-container-lanes-deferred | 3817ns | 3817ns | -6.37% |
| warm-container-minimum | 1235ns | 1235ns | -69.70% |
| warm-container-native | 1247ns | 1247ns | -69.41% |
| warm-container-plusone | 3910ns | 3910ns | -4.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4451ns | base | --- | [1789, 5487] | --- | --- | --- | --- |
| warm-container-kernel | 2126ns | no significant difference | [-102, +284]ns | [2062, 4712] | no | 0.1923 | 0.1539 | 0 |
| warm-container-lanes-deferred | 2080ns | +270.9ns (+6.1%) | [+24, +297]ns | [2059, 5652] | YES (adj: no) | 0.1345 | 0.0807 | 0 |
| warm-container-minimum | 841ns | -3256.8ns (-73.2%) | [-4139, -1071]ns | [784, 893] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 990ns | -3417.3ns (-76.8%) | [-4479, -859]ns | [931, 1032] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3168ns | no significant difference | [-94, +9]ns | [1793, 4978] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2876ns | -28.1% | +40.1% | -70.4% | -64.3% | +11.0% |
| 1 | 2 | 4952ns | -58.2% | +14.1% | -70.0% | -79.2% | -63.6% |
| 1 | 3 | 3940ns | +19.6% | -37.2% | -81.5% | -76.4% | +30.7% |
| 1 | 4 | 2362ns | -12.5% | -28.7% | -66.1% | -57.2% | +11.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.209 | moderate- |
| warm-container-kernel | -0.416 | moderate- |
| warm-container-lanes-deferred | 0.091 | ok |
| warm-container-minimum | -0.393 | moderate- |
| warm-container-native | -0.323 | moderate- |
| warm-container-plusone | -0.627 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 54.6ns | 3925.4ns | 1.4% |  |
| warm-container-kernel | 44.5ns | 3811.9ns | 1.2% |  |
| warm-container-lanes-deferred | 43.0ns | 3689.0ns | 1.2% |  |
| warm-container-minimum | 92.5ns | 970.7ns | 9.5% | HIGH |
| warm-container-native | 97.9ns | 992.7ns | 9.9% | HIGH |
| warm-container-plusone | 51.1ns | 3765.4ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1779.5-6479.5 ns)
   1779.5 |########################################
   2014.5 |
   2249.5 |###
   2484.5 |
   2719.5 |###
   2954.5 |
   3189.5 |
   3424.5 |
   3659.5 |
   3894.5 |###
   4129.5 |
   4364.5 |
   4599.5 |
   4834.5 |##############
   5069.5 |
   5304.5 |##########
   5539.5 |###
   5774.5 |##########
   6009.5 |###
   6244.5 |##################
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1965.3-7099.8 ns)
   1965.3 |########################################
   2222.0 |
   2478.7 |
   2735.5 |
   2992.2 |##
   3248.9 |
   3505.6 |##
   3762.4 |
   4019.1 |
   4275.8 |
   4532.5 |########
   4789.3 |##
   5046.0 |##
   5302.7 |####
   5559.4 |
   5816.2 |
   6072.9 |
   6329.6 |####
   6586.3 |##
   6843.1 |####
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1683.7-6936.3 ns)
   1683.7 |##################
   1946.4 |########################################
   2209.0 |
   2471.6 |
   2734.3 |###
   2996.9 |
   3259.5 |
   3522.1 |###
   3784.8 |
   4047.4 |
   4310.0 |
   4572.7 |
   4835.3 |
   5097.9 |
   5360.6 |###
   5623.2 |###
   5885.8 |#########
   6148.4 |######
   6411.1 |#########
   6673.7 |###
  (4 below, 4 above range)

warm-container-minimum (n=40, range 727.9-1497.5 ns)
    727.9 |###############
    766.4 |###################################
    804.9 |########################################
    843.4 |###############
    881.8 |
    920.3 |####################
    958.8 |
    997.3 |
   1035.8 |
   1074.2 |
   1112.7 |
   1151.2 |
   1189.7 |
   1228.1 |
   1266.6 |#####
   1305.1 |###############
   1343.6 |
   1382.0 |
   1420.5 |#####
   1459.0 |
  (5 below, 5 above range)

warm-container-native (n=40, range 928.0-1089.4 ns)
    928.0 |########################################
    936.1 |
    944.2 |
    952.2 |
    960.3 |##
    968.4 |
    976.4 |
    984.5 |#####
    992.6 |
   1000.7 |##
   1008.7 |
   1016.8 |##
   1024.9 |####################
   1032.9 |#################
   1041.0 |
   1049.1 |
   1057.1 |
   1065.2 |
   1073.3 |
   1081.3 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1780.4-6620.2 ns)
   1780.4 |########################################
   2022.4 |
   2264.4 |
   2506.4 |
   2748.4 |##
   2990.4 |
   3232.4 |
   3474.3 |##
   3716.3 |
   3958.3 |
   4200.3 |
   4442.3 |##
   4684.3 |#####
   4926.3 |########
   5168.3 |##
   5410.3 |#####
   5652.3 |
   5894.3 |#####
   6136.3 |##
   6378.2 |##############
  (5 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-kernel**: CV=54.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=30.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=54.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
