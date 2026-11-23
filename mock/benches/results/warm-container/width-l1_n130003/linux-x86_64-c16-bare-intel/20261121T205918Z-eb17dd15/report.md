# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 31% faster than the next best (warm-container-headroom)

warm-container-native (1.43 us) leads warm-container-headroom (1.87 us) by 31%, a clear separation rather than a photo finish. CV 24.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 30% (significant)

warm-container-native is -556 ns (30%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-lanes-deferred is an outlier: 3.1x slower than the field

warm-container-lanes-deferred (4.43 us) is 3.1x the fastest (1.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native, warm-container-headroom} vs {warm-container-minimum, warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred} (65% apart)

The field splits into a fast tier {warm-container-native, warm-container-headroom} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred} with a 65% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest warm-container-native (1.43 us) to slowest warm-container-lanes-deferred (4.43 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 4.3x its best-20%

warm-container-headroom's best 20% of batches run at 1.47 us but its worst 20% at 6.32 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1427.2 ns median (-23.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.10x (fastest 1427.2 ns, slowest 4429.4 ns)

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
| warm-container-headroom | 3443ns | 1940ns | 1518ns | 3036ns | 6591ns | base |
| warm-container-kernel | 3712ns | 4064ns | 1529ns | 3752ns | 5774ns | +7.81% |
| warm-container-lanes-deferred | 3726ns | 4601ns | 1418ns | 3861ns | 5628ns | +8.21% |
| warm-container-minimum | 3807ns | 3194ns | 1640ns | 3612ns | 6557ns | +10.56% |
| warm-container-native | 1776ns | 1652ns | 1400ns | 1684ns | 2428ns | -48.42% |
| warm-container-plusone | 4030ns | 3541ns | 1847ns | 3835ns | 6798ns | +17.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3315ns | 1467ns | 6321ns | base | 9.886 |
| warm-container-kernel | 3563ns | 1472ns | 5523ns | +7.49% | 9.197 |
| warm-container-lanes-deferred | 3577ns | 1365ns | 5377ns | +7.91% | 9.161 |
| warm-container-minimum | 3665ns | 1584ns | 6291ns | +10.57% | 8.940 |
| warm-container-native | 1569ns | 1243ns | 2148ns | -52.66% | 20.883 |
| warm-container-plusone | 3881ns | 1785ns | 6507ns | +17.09% | 8.442 |

## Performance model

- Peak throughput: **26.359 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.484 | 66.3% |
| warm-container-kernel | 8.378 | 31.8% |
| warm-container-lanes-deferred | 7.398 | 28.1% |
| warm-container-minimum | 10.621 | 40.3% |
| warm-container-native | 22.960 | 87.1% |
| warm-container-plusone | 9.581 | 36.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3443ns | 3443ns | base |
| warm-container-kernel | 3712ns | 3712ns | +7.81% |
| warm-container-lanes-deferred | 3726ns | 3726ns | +8.21% |
| warm-container-minimum | 3807ns | 3807ns | +10.56% |
| warm-container-native | 1776ns | 1776ns | -48.42% |
| warm-container-plusone | 4030ns | 4030ns | +17.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1874ns | base | --- | [1758, 4553] | --- | --- | --- | --- |
| warm-container-kernel | 3911ns | +154.9ns (+8.3%) | [+75, +438]ns | [2662, 4487] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-container-lanes-deferred | 4429ns | no significant difference | [-147, +152]ns | [1779, 4862] | no | 1.0000 | 1.0000 | 0 |
| warm-container-minimum | 3085ns | no significant difference | [-3, +498]ns | [1784, 5223] | no | 0.1009 | 0.0807 | 0 |
| warm-container-native | 1427ns | -555.7ns (-29.6%) | [-2294, -269]ns | [1313, 1633] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3420ns | +316.2ns (+16.9%) | [+63, +549]ns | [1785, 5730] | YES (adj: no) | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4302ns | -8.1% | +3.5% | -22.9% | -53.3% | -46.2% |
| 1 | 2 | 1803ns | +107.4% | +152.7% | +71.1% | -11.6% | -0.6% |
| 1 | 3 | 1469ns | +197.3% | +84.9% | +108.7% | -9.0% | +239.8% |
| 1 | 4 | 3302ns | +33.2% | +34.1% | +4.8% | -61.6% | +47.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.197 | ok |
| warm-container-kernel | 0.120 | ok |
| warm-container-lanes-deferred | -0.422 | moderate- |
| warm-container-minimum | -0.240 | moderate- |
| warm-container-native | 0.210 | moderate+ |
| warm-container-plusone | 0.180 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 44.3ns | 3314.7ns | 1.3% |  |
| warm-container-kernel | 53.0ns | 3563.1ns | 1.5% |  |
| warm-container-lanes-deferred | 51.6ns | 3577.0ns | 1.4% |  |
| warm-container-minimum | 48.5ns | 3665.2ns | 1.3% |  |
| warm-container-native | 73.3ns | 1569.1ns | 4.7% |  |
| warm-container-plusone | 51.9ns | 3881.4ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1466.9-6321.0 ns)
   1466.9 |########################################
   1709.6 |###################################
   1952.3 |
   2195.0 |####
   2437.7 |
   2680.4 |
   2923.1 |
   3165.8 |
   3408.5 |
   3651.3 |
   3894.0 |####
   4136.7 |########
   4379.4 |########
   4622.1 |########
   4864.8 |#############
   5107.5 |####
   5350.2 |####
   5592.9 |
   5835.6 |####
   6078.3 |
  (4 below, 5 above range)

warm-container-kernel (n=40, range 1471.9-5523.2 ns)
   1471.9 |########################################
   1674.5 |
   1877.1 |
   2079.6 |##########
   2282.2 |
   2484.8 |
   2687.3 |
   2889.9 |
   3092.5 |#####
   3295.0 |
   3497.6 |###############
   3700.2 |##########
   3902.7 |##########
   4105.3 |##########
   4307.9 |###############
   4510.4 |
   4713.0 |####################
   4915.5 |#####
   5118.1 |####################
   5320.7 |#####
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1365.0-5376.6 ns)
   1365.0 |
   1565.6 |########################################
   1766.2 |#####
   1966.8 |
   2167.4 |
   2367.9 |
   2568.5 |
   2769.1 |
   2969.7 |
   3170.2 |#####
   3370.8 |
   3571.4 |
   3772.0 |
   3972.6 |###########
   4173.1 |###########
   4373.7 |#################
   4574.3 |#####
   4774.9 |##################################
   4975.5 |######################
   5176.0 |######################
  (7 below, 2 above range)

warm-container-minimum (n=40, range 1583.9-6291.0 ns)
   1583.9 |########################################
   1819.3 |
   2054.6 |
   2290.0 |
   2525.3 |
   2760.7 |
   2996.0 |
   3231.4 |
   3466.8 |
   3702.1 |
   3937.5 |
   4172.8 |##
   4408.2 |
   4643.5 |#####
   4878.9 |#####
   5114.3 |#####
   5349.6 |########
   5585.0 |##
   5820.3 |########
   6055.7 |##
  (5 below, 5 above range)

warm-container-native (n=40, range 1243.1-2148.1 ns)
   1243.1 |########
   1288.4 |########################################
   1333.6 |####
   1378.9 |#################
   1424.1 |####
   1469.4 |#############
   1514.6 |
   1559.9 |####
   1605.1 |########
   1650.4 |####
   1695.6 |####
   1740.9 |
   1786.1 |########
   1831.4 |
   1876.6 |####
   1921.9 |####
   1967.1 |#############
   2012.4 |####
   2057.6 |
   2102.9 |
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1784.6-6507.4 ns)
   1784.6 |########################################
   2020.8 |
   2256.9 |
   2493.0 |
   2729.2 |##
   2965.3 |
   3201.4 |
   3437.6 |
   3673.7 |
   3909.9 |##
   4146.0 |
   4382.1 |
   4618.3 |##
   4854.4 |
   5090.5 |##
   5326.7 |##
   5562.8 |########
   5799.0 |########
   6035.1 |##
   6271.2 |####################
  (5 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=59.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=43.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=46.6% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=55.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=22.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=54.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.6x (possible bimodal distribution)
