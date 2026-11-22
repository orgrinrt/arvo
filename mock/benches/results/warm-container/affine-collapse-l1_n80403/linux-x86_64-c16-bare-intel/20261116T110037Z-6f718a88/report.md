# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 78% (significant)

warm-container-kernel is -3.44 us (78%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 5.2x slower than the field

warm-container-plusone (4.39 us) is 5.2x the fastest (844 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.59)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-kernel, warm-container-native, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-plusone} (368% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel, warm-container-native, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-plusone} with a 368% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest warm-container-minimum (844 ns) to slowest warm-container-plusone (4.39 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 3.9x its best-20%

warm-container-headroom's best 20% of batches run at 1.46 us but its worst 20% at 5.73 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 844.0 ns median (-80.7% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.20x (fastest 844.0 ns, slowest 4385.4 ns)

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
| warm-container-headroom | 3806ns | 4535ns | 1511ns | 3837ns | 6009ns | base |
| warm-container-kernel | 1146ns | 1091ns | 953ns | 1133ns | 1380ns | -69.89% |
| warm-container-lanes-deferred | 1175ns | 1167ns | 970ns | 1175ns | 1383ns | -69.12% |
| warm-container-minimum | 1181ns | 1053ns | 934ns | 1100ns | 1673ns | -68.97% |
| warm-container-native | 1219ns | 1159ns | 1153ns | 1197ns | 1348ns | -67.98% |
| warm-container-plusone | 3987ns | 4542ns | 1661ns | 4000ns | 6274ns | +4.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3660ns | 1460ns | 5733ns | base | 8.954 |
| warm-container-kernel | 913ns | 765ns | 1098ns | -75.05% | 35.883 |
| warm-container-lanes-deferred | 935ns | 782ns | 1104ns | -74.45% | 35.046 |
| warm-container-minimum | 925ns | 752ns | 1295ns | -74.72% | 35.418 |
| warm-container-native | 970ns | 926ns | 1057ns | -73.49% | 33.776 |
| warm-container-plusone | 3839ns | 1604ns | 6018ns | +4.89% | 8.536 |

## Performance model

- Peak throughput: **43.561 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.479 | 17.2% |
| warm-container-kernel | 37.563 | 86.2% |
| warm-container-lanes-deferred | 35.031 | 80.4% |
| warm-container-minimum | 38.822 | 89.1% |
| warm-container-native | 35.212 | 80.8% |
| warm-container-plusone | 7.472 | 17.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3806ns | 3806ns | base |
| warm-container-kernel | 1146ns | 1146ns | -69.89% |
| warm-container-lanes-deferred | 1175ns | 1175ns | -69.12% |
| warm-container-minimum | 1181ns | 1181ns | -68.97% |
| warm-container-native | 1219ns | 1219ns | -67.98% |
| warm-container-plusone | 3987ns | 3987ns | +4.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4381ns | base | --- | [1791, 4637] | --- | --- | --- | --- |
| warm-container-kernel | 872ns | -3437.1ns (-78.5%) | [-3746, -1069]ns | [851, 936] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 935ns | -3395.6ns (-77.5%) | [-3833, -854]ns | [927, 942] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 844ns | -3241.1ns (-74.0%) | [-3820, -959]ns | [775, 846] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 931ns | -3425.4ns (-78.2%) | [-3711, -862]ns | [929, 981] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4385ns | +58.2ns (+1.3%) | [+6, +321]ns | [2022, 4938] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4216ns | -78.8% | -77.8% | -68.6% | -75.5% | +5.8% |
| 1 | 2 | 4232ns | -79.4% | -81.5% | -81.7% | -78.0% | +3.5% |
| 1 | 3 | 4617ns | -77.4% | -77.6% | -82.8% | -79.9% | +8.7% |
| 1 | 4 | 3072ns | -74.5% | -69.4% | -72.5% | -68.9% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.306 | moderate- |
| warm-container-kernel | -0.586 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.485 | moderate- |
| warm-container-minimum | -0.131 | ok |
| warm-container-native | -0.142 | ok |
| warm-container-plusone | -0.367 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 49.8ns | 3659.6ns | 1.4% |  |
| warm-container-kernel | 84.2ns | 913.2ns | 9.2% | HIGH |
| warm-container-lanes-deferred | 85.6ns | 935.0ns | 9.2% | HIGH |
| warm-container-minimum | 88.6ns | 925.2ns | 9.6% | HIGH |
| warm-container-native | 90.3ns | 970.2ns | 9.3% | HIGH |
| warm-container-plusone | 52.6ns | 3838.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1459.6-5733.2 ns)
   1459.6 |########################################
   1673.2 |#########################
   1886.9 |
   2100.6 |
   2314.3 |
   2528.0 |
   2741.7 |
   2955.3 |
   3169.0 |
   3382.7 |#####
   3596.4 |
   3810.1 |#####
   4023.7 |#####
   4237.4 |###############
   4451.1 |##############################
   4664.8 |
   4878.5 |#########################
   5092.1 |#####
   5305.8 |##########
   5519.5 |##########
  (2 below, 3 above range)

warm-container-kernel (n=40, range 764.5-1097.6 ns)
    764.5 |
    781.2 |########################
    797.8 |
    814.5 |
    831.1 |####
    847.8 |########################################
    864.4 |
    881.1 |####
    897.7 |####
    914.4 |
    931.0 |########################
    947.7 |####
    964.3 |
    981.0 |
    997.7 |
   1014.3 |
   1031.0 |############################
   1047.6 |####
   1064.3 |
   1080.9 |####
  (3 below, 2 above range)

warm-container-lanes-deferred (n=40, range 782.2-1104.0 ns)
    782.2 |############
    798.2 |
    814.3 |
    830.4 |######
    846.5 |######
    862.6 |###
    878.7 |
    894.8 |
    910.9 |###
    927.0 |########################################
    943.1 |######
    959.2 |
    975.3 |
    991.4 |###
   1007.5 |
   1023.6 |###############
   1039.7 |###
   1055.8 |
   1071.8 |
   1087.9 |
  (4 below, 4 above range)

warm-container-minimum (n=40, range 752.2-1295.5 ns)
    752.2 |########################################
    779.4 |
    806.6 |#######
    833.7 |####################################
    860.9 |###
    888.0 |
    915.2 |###
    942.4 |
    969.5 |
    996.7 |###
   1023.9 |
   1051.0 |
   1078.2 |
   1105.3 |
   1132.5 |###
   1159.7 |##########
   1186.8 |
   1214.0 |
   1241.2 |
   1268.3 |
  (4 below, 6 above range)

warm-container-native (n=40, range 925.6-1057.4 ns)
    925.6 |########################################
    932.2 |
    938.8 |
    945.4 |
    952.0 |
    958.6 |
    965.2 |##
    971.8 |####
    978.3 |##
    984.9 |
    991.5 |
    998.1 |####
   1004.7 |
   1011.3 |
   1017.9 |
   1024.5 |##
   1031.1 |##############
   1037.7 |
   1044.2 |
   1050.8 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1603.6-6017.8 ns)
   1603.6 |########################################
   1824.3 |########
   2045.0 |
   2265.7 |
   2486.4 |
   2707.1 |
   2927.9 |
   3148.6 |
   3369.3 |
   3590.0 |
   3810.7 |
   4031.4 |########
   4252.1 |#################
   4472.8 |#############
   4693.5 |####
   4914.3 |######################
   5135.0 |####
   5355.7 |####
   5576.4 |########
   5797.1 |#############
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=46.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=22.7% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=45.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
