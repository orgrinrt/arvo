# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 929 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 82% (significant)

warm-container-minimum is -4.15 us (82%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 5.5x slower than the field

warm-container-headroom (5.09 us) is 5.5x the fastest (929 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-minimum (929 ns) and warm-container-lanes-deferred (933 ns) differ by 0.39%, inside the noise, even though the wider field spreads 447.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.71)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-lanes-deferred, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (421% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-lanes-deferred, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 421% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.5x the fastest

Fastest warm-container-minimum (929 ns) to slowest warm-container-headroom (5.09 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+1% speed for 1.2x steadier)

warm-container-minimum is fastest (929 ns, CV 10.7%); warm-container-native gives up 1.0% median for 1.2x lower variance (CV 8.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 3.7x its best-20%

warm-container-plusone's best 20% of batches run at 1.70 us but its worst 20% at 6.33 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 929.3 ns median (-81.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.48x (fastest 929.3 ns, slowest 5090.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4390ns | 5272ns | 1841ns | 4487ns | 6647ns | base |
| warm-container-kernel | 1283ns | 1228ns | 1164ns | 1241ns | 1527ns | -70.77% |
| warm-container-lanes-deferred | 1235ns | 1169ns | 982ns | 1205ns | 1578ns | -71.86% |
| warm-container-minimum | 1184ns | 1160ns | 1021ns | 1173ns | 1378ns | -73.04% |
| warm-container-native | 1243ns | 1166ns | 1155ns | 1195ns | 1476ns | -71.69% |
| warm-container-plusone | 4189ns | 5105ns | 1760ns | 4191ns | 6611ns | -4.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4227ns | 1778ns | 6361ns | base | 7.753 |
| warm-container-kernel | 1006ns | 935ns | 1160ns | -76.19% | 32.558 |
| warm-container-lanes-deferred | 990ns | 787ns | 1265ns | -76.57% | 33.093 |
| warm-container-minimum | 940ns | 820ns | 1073ns | -77.77% | 34.868 |
| warm-container-native | 981ns | 928ns | 1114ns | -76.80% | 33.412 |
| warm-container-plusone | 4033ns | 1698ns | 6327ns | -4.59% | 8.126 |

## Performance model

- Peak throughput: **41.613 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.437 | 15.5% |
| warm-container-kernel | 34.604 | 83.2% |
| warm-container-lanes-deferred | 35.125 | 84.4% |
| warm-container-minimum | 35.261 | 84.7% |
| warm-container-native | 34.910 | 83.9% |
| warm-container-plusone | 6.647 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4390ns | 4390ns | base |
| warm-container-kernel | 1283ns | 1283ns | -70.77% |
| warm-container-lanes-deferred | 1235ns | 1235ns | -71.86% |
| warm-container-minimum | 1184ns | 1184ns | -73.04% |
| warm-container-native | 1243ns | 1243ns | -71.69% |
| warm-container-plusone | 4189ns | 4189ns | -4.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5091ns | base | --- | [2180, 5652] | --- | --- | --- | --- |
| warm-container-kernel | 947ns | -3988.7ns (-78.3%) | [-4615, -1221]ns | [938, 1038] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 933ns | -3909.8ns (-76.8%) | [-4581, -1402]ns | [932, 1017] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 929ns | -4153.5ns (-81.6%) | [-4732, -1365]ns | [856, 1018] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 939ns | -4112.0ns (-80.8%) | [-4477, -1242]ns | [932, 943] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4929ns | no significant difference | [-350, +1]ns | [2365, 5222] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 5097ns | -77.0% | -79.7% | -79.6% | -81.8% | -48.3% |
| 1 | 2 | 4066ns | -76.9% | -80.9% | -79.2% | -76.9% | +22.3% |
| 1 | 3 | 5090ns | -79.6% | -79.6% | -83.2% | -79.6% | -6.5% |
| 1 | 4 | 4959ns | -81.1% | -81.2% | -79.1% | -81.2% | -23.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.521 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.415 | moderate- |
| warm-container-lanes-deferred | -0.706 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.251 | moderate- |
| warm-container-native | -0.386 | moderate- |
| warm-container-plusone | -0.238 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 57.7ns | 4226.7ns | 1.4% |  |
| warm-container-kernel | 96.7ns | 1006.4ns | 9.6% | HIGH |
| warm-container-lanes-deferred | 88.8ns | 990.2ns | 9.0% | HIGH |
| warm-container-minimum | 90.2ns | 939.8ns | 9.6% | HIGH |
| warm-container-native | 92.5ns | 980.7ns | 9.4% | HIGH |
| warm-container-plusone | 56.4ns | 4032.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1777.8-6360.8 ns)
   1777.8 |########################################
   2007.0 |
   2236.1 |
   2465.3 |###
   2694.4 |
   2923.6 |
   3152.7 |
   3381.9 |
   3611.0 |
   3840.2 |
   4069.3 |
   4298.5 |#######
   4527.6 |###
   4756.8 |#######
   4985.9 |
   5215.1 |##################
   5444.2 |###
   5673.4 |##############
   5902.5 |##########
   6131.7 |###
  (3 below, 6 above range)

warm-container-kernel (n=40, range 935.1-1159.6 ns)
    935.1 |########################################
    946.4 |##
    957.6 |
    968.8 |
    980.0 |####
    991.2 |##
   1002.5 |
   1013.7 |
   1024.9 |
   1036.1 |##################
   1047.4 |
   1058.6 |
   1069.8 |
   1081.0 |
   1092.3 |##
   1103.5 |
   1114.7 |
   1125.9 |
   1137.2 |
   1148.4 |##
  (3 below, 6 above range)

warm-container-lanes-deferred (n=40, range 787.4-1265.1 ns)
    787.4 |
    811.3 |###
    835.2 |###
    859.1 |
    883.0 |
    906.9 |##########
    930.7 |########################################
    954.6 |###
    978.5 |##########
   1002.4 |
   1026.3 |####################################
   1050.2 |
   1074.1 |
   1097.9 |###
   1121.8 |
   1145.7 |
   1169.6 |
   1193.5 |###
   1217.4 |
   1241.3 |
  (6 below, 2 above range)

warm-container-minimum (n=40, range 820.4-1072.9 ns)
    820.4 |
    833.0 |########################################
    845.7 |####
    858.3 |####
    870.9 |
    883.5 |
    896.1 |####
    908.8 |####
    921.4 |########################
    934.0 |####
    946.6 |
    959.3 |####
    971.9 |
    984.5 |
    997.1 |####
   1009.8 |
   1022.4 |################
   1035.0 |########################
   1047.6 |####
   1060.2 |
  (3 below, 3 above range)

warm-container-native (n=40, range 928.1-1114.5 ns)
    928.1 |########################################
    937.5 |######################
    946.8 |
    956.1 |
    965.4 |
    974.7 |
    984.1 |##
    993.4 |
   1002.7 |
   1012.0 |
   1021.3 |########
   1030.6 |#####
   1040.0 |########
   1049.3 |
   1058.6 |
   1067.9 |
   1077.2 |
   1086.5 |
   1095.9 |
   1105.2 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1697.7-6326.7 ns)
   1697.7 |########################################
   1929.1 |
   2160.6 |###
   2392.0 |#######
   2623.5 |
   2854.9 |
   3086.4 |
   3317.8 |###
   3549.3 |
   3780.7 |
   4012.2 |
   4243.6 |
   4475.1 |#######
   4706.5 |###
   4938.0 |#####################
   5169.4 |###
   5400.9 |##########
   5632.3 |##########
   5863.8 |#######
   6095.2 |
  (2 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=45.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=24.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=46.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
