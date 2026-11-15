# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 72% (significant)

warm-container-lanes-deferred is -2.76 us (72%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.5x slower than the field

warm-container-headroom (3.84 us) is 3.5x the fastest (1.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (1.09 us) and warm-container-native (1.09 us) differ by 0.37%, inside the noise, even though the wider field spreads 252.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-lanes-deferred} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (174% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-lanes-deferred} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 174% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.5x the fastest

Fastest warm-container-kernel (1.09 us) to slowest warm-container-headroom (3.84 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 4.4x its best-20%

warm-container-minimum's best 20% of batches run at 1.49 us but its worst 20% at 6.60 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-46 ns, 1.21%)

warm-container-minimum differs from baseline warm-container-headroom by -46 ns (1.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1088.7 ns median (-71.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 3.53x (fastest 1088.7 ns, slowest 3837.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3874ns | 3969ns | 1554ns | 3684ns | 6761ns | base |
| warm-container-kernel | 1338ns | 1316ns | 1014ns | 1345ns | 1640ns | -65.46% |
| warm-container-lanes-deferred | 1411ns | 1461ns | 1058ns | 1423ns | 1729ns | -63.57% |
| warm-container-minimum | 3650ns | 3432ns | 1541ns | 3276ns | 6884ns | -5.76% |
| warm-container-native | 1302ns | 1319ns | 825ns | 1315ns | 1740ns | -66.38% |
| warm-container-plusone | 3833ns | 3614ns | 1713ns | 3580ns | 6710ns | -1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3732ns | 1503ns | 6485ns | base | 4.390 |
| warm-container-kernel | 1101ns | 840ns | 1349ns | -70.50% | 14.880 |
| warm-container-lanes-deferred | 1158ns | 876ns | 1407ns | -68.99% | 14.154 |
| warm-container-minimum | 3517ns | 1490ns | 6603ns | -5.78% | 4.659 |
| warm-container-native | 1068ns | 683ns | 1406ns | -71.40% | 15.347 |
| warm-container-plusone | 3694ns | 1656ns | 6445ns | -1.02% | 4.435 |

## Performance model

- Peak throughput: **23.975 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.269 | 17.8% |
| warm-container-kernel | 15.050 | 62.8% |
| warm-container-lanes-deferred | 13.539 | 56.5% |
| warm-container-minimum | 4.938 | 20.6% |
| warm-container-native | 14.995 | 62.5% |
| warm-container-plusone | 4.689 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3874ns | 3874ns | base |
| warm-container-kernel | 1338ns | 1338ns | -65.46% |
| warm-container-lanes-deferred | 1411ns | 1411ns | -63.57% |
| warm-container-minimum | 3650ns | 3650ns | -5.76% |
| warm-container-native | 1302ns | 1302ns | -66.38% |
| warm-container-plusone | 3833ns | 3833ns | -1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3838ns | base | --- | [1836, 4970] | --- | --- | --- | --- |
| warm-container-kernel | 1089ns | -2687.4ns (-70.0%) | [-3949, -746]ns | [1088, 1110] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1210ns | -2756.4ns (-71.8%) | [-3649, -956]ns | [1102, 1214] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3318ns | -46.5ns (-1.2%) | [-270, -13]ns | [1661, 4407] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-native | 1093ns | -2684.6ns (-70.0%) | [-4256, -819]ns | [1078, 1185] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3494ns | no significant difference | [-15, +255]ns | [1823, 5135] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1507ns | -27.8% | -22.9% | -0.8% | -13.9% | +163.5% |
| 1 | 2 | 1841ns | -32.2% | -50.7% | -19.1% | -38.8% | +140.2% |
| 1 | 3 | 4440ns | -81.1% | -72.6% | -3.4% | -84.6% | -38.8% |
| 1 | 4 | 5567ns | -80.4% | -75.5% | -13.9% | -80.4% | -67.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.301 | moderate+ |
| warm-container-kernel | -0.493 | moderate- |
| warm-container-lanes-deferred | -0.028 | ok |
| warm-container-minimum | 0.280 | moderate+ |
| warm-container-native | -0.118 | ok |
| warm-container-plusone | 0.235 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 49.1ns | 3732.4ns | 1.3% |  |
| warm-container-kernel | 92.0ns | 1101.0ns | 8.4% | HIGH |
| warm-container-lanes-deferred | 95.6ns | 1157.6ns | 8.3% | HIGH |
| warm-container-minimum | 46.6ns | 3516.7ns | 1.3% |  |
| warm-container-native | 84.6ns | 1067.6ns | 7.9% | HIGH |
| warm-container-plusone | 49.0ns | 3694.2ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1502.8-6485.3 ns)
   1502.8 |####
   1751.9 |########################################
   2001.0 |
   2250.1 |
   2499.3 |
   2748.4 |
   2997.5 |
   3246.6 |####
   3495.8 |
   3744.9 |########
   3994.0 |####
   4243.1 |########
   4492.3 |
   4741.4 |########
   4990.5 |########
   5239.6 |
   5488.8 |########
   5737.9 |
   5987.0 |################
   6236.1 |
  (7 below, 6 above range)

warm-container-kernel (n=40, range 840.2-1349.0 ns)
    840.2 |##
    865.7 |
    891.1 |##
    916.6 |
    942.0 |
    967.4 |
    992.9 |
   1018.3 |
   1043.7 |
   1069.2 |########################################
   1094.6 |
   1120.0 |##
   1145.5 |##
   1170.9 |##
   1196.3 |###########
   1221.8 |##
   1247.2 |
   1272.7 |####
   1298.1 |
   1323.5 |
  (7 below, 3 above range)

warm-container-lanes-deferred (n=40, range 875.5-1407.1 ns)
    875.5 |
    902.1 |#################
    928.7 |
    955.3 |
    981.9 |
   1008.4 |
   1035.0 |
   1061.6 |
   1088.2 |##############
   1114.8 |
   1141.3 |
   1167.9 |##
   1194.5 |########################################
   1221.1 |
   1247.6 |
   1274.2 |
   1300.8 |#####
   1327.4 |
   1354.0 |#################
   1380.5 |##
  (4 below, 1 above range)

warm-container-minimum (n=40, range 1489.7-6602.8 ns)
   1489.7 |########################################
   1745.4 |#############
   2001.0 |###
   2256.7 |
   2512.3 |###
   2768.0 |
   3023.6 |
   3279.3 |
   3535.0 |
   3790.6 |######
   4046.3 |##########
   4301.9 |######
   4557.6 |######
   4813.2 |
   5068.9 |###
   5324.5 |###
   5580.2 |###
   5835.8 |##########
   6091.5 |
   6347.1 |###
  (2 below, 4 above range)

warm-container-native (n=40, range 683.4-1406.2 ns)
    683.4 |####################
    719.5 |####
    755.7 |
    791.8 |
    827.9 |
    864.1 |
    900.2 |
    936.4 |
    972.5 |############
   1008.6 |
   1044.8 |####
   1080.9 |########################################
   1117.1 |
   1153.2 |########
   1189.3 |############################
   1225.5 |####
   1261.6 |
   1297.8 |
   1333.9 |############
   1370.0 |
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1656.5-6444.7 ns)
   1656.5 |########################################
   1895.9 |
   2135.3 |
   2374.7 |
   2614.1 |
   2853.5 |
   3092.9 |
   3332.3 |##
   3571.8 |#####
   3811.2 |
   4050.6 |
   4290.0 |
   4529.4 |#####
   4768.8 |##
   5008.2 |#####
   5247.6 |########
   5487.1 |
   5726.5 |########
   5965.9 |##
   6205.3 |
  (4 below, 6 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-minimum**: CV=58.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-native**: CV=24.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=53.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
