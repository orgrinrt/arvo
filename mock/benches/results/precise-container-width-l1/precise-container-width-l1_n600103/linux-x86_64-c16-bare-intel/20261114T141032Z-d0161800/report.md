# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (10.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.38 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 191% faster than the next best (warm-container-native)

warm-container-kernel (1.38 us) leads warm-container-native (4.03 us) by 191%, a clear separation rather than a photo finish. CV 15.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 86% (significant)

warm-container-kernel is -8.90 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 7.4x slower than the field

warm-container-headroom (10.30 us) is 7.4x the fastest (1.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.76)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.4x the fastest

Fastest warm-container-kernel (1.38 us) to slowest warm-container-headroom (10.30 us): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 2.4x its best-20%

warm-container-native's best 20% of batches run at 3.30 us but its worst 20% at 7.98 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1383.5 ns median (-86.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 7.45x (fastest 1383.5 ns, slowest 10301.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9918ns | 10383ns | 8488ns | 10228ns | 10415ns | base |
| warm-container-kernel | 1715ns | 1666ns | 1368ns | 1696ns | 2118ns | -82.71% |
| warm-container-minimum | 5039ns | 4125ns | 3371ns | 4617ns | 7974ns | -49.19% |
| warm-container-native | 4767ns | 4115ns | 3368ns | 4088ns | 8203ns | -51.93% |
| warm-container-plusone | 4329ns | 4116ns | 3355ns | 3809ns | 6865ns | -56.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9833ns | 8421ns | 10310ns | base | 3.332 |
| warm-container-kernel | 1411ns | 1136ns | 1711ns | -85.65% | 23.217 |
| warm-container-minimum | 4932ns | 3302ns | 7790ns | -49.84% | 6.644 |
| warm-container-native | 4661ns | 3301ns | 7983ns | -52.60% | 7.030 |
| warm-container-plusone | 4229ns | 3288ns | 6659ns | -56.99% | 7.748 |

## Performance model

- Peak throughput: **28.839 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.181 | 11.0% |
| warm-container-kernel | 23.685 | 82.1% |
| warm-container-minimum | 8.103 | 28.1% |
| warm-container-native | 8.125 | 28.2% |
| warm-container-plusone | 8.122 | 28.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9918ns | 9918ns | base |
| warm-container-kernel | 1715ns | 1715ns | -82.71% |
| warm-container-minimum | 5039ns | 5039ns | -49.19% |
| warm-container-native | 4767ns | 4767ns | -51.93% |
| warm-container-plusone | 4329ns | 4329ns | -56.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 10302ns | base | --- | [10296, 10303] | --- | --- | --- | --- |
| warm-container-kernel | 1384ns | -8896.8ns (-86.4%) | [-8929, -8693]ns | [1383, 1465] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4044ns | -5125.4ns (-49.8%) | [-6254, -5111]ns | [4032, 4054] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 4033ns | -6257.1ns (-60.7%) | [-6263, -5965]ns | [4032, 4044] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4034ns | -6263.0ns (-60.8%) | [-6269, -6244]ns | [3306, 4037] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 8424ns | -81.7% | -60.8% | -60.8% | -60.8% |
| 1 | 2 | 10305ns | -86.6% | -60.9% | -60.8% | -60.8% |
| 1 | 3 | 10297ns | -84.6% | -30.9% | -60.7% | -68.1% |
| 1 | 4 | 10302ns | -89.2% | -60.7% | -60.8% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.086 | ok |
| warm-container-kernel | -0.432 | moderate- |
| warm-container-minimum | -0.246 | moderate- |
| warm-container-native | -0.080 | ok |
| warm-container-plusone | -0.756 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.7ns | 9833.0ns | 0.2% |  |
| warm-container-kernel | 114.0ns | 1411.4ns | 8.1% | HIGH |
| warm-container-minimum | 30.2ns | 4932.3ns | 0.6% |  |
| warm-container-native | 28.6ns | 4661.1ns | 0.6% |  |
| warm-container-plusone | 26.0ns | 4229.2ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8421.2-10310.1 ns)
   8421.2 |###########
   8515.6 |
   8610.1 |
   8704.5 |
   8799.0 |
   8893.4 |
   8987.8 |
   9082.3 |
   9176.7 |
   9271.2 |
   9365.6 |
   9460.1 |
   9554.5 |
   9648.9 |
   9743.4 |
   9837.8 |
   9932.3 |
  10026.7 |
  10121.2 |
  10215.6 |########################################
  (2 below, 3 above range)

warm-container-kernel (n=40, range 1136.2-1711.3 ns)
   1136.2 |
   1165.0 |
   1193.8 |###
   1222.5 |##################
   1251.3 |###
   1280.0 |
   1308.8 |
   1337.5 |
   1366.3 |########################################
   1395.0 |###
   1423.8 |###
   1452.5 |
   1481.3 |###
   1510.0 |
   1538.8 |
   1567.5 |#########################
   1596.3 |
   1625.1 |###
   1653.8 |###
   1682.6 |
  (6 below, 4 above range)

warm-container-minimum (n=40, range 3301.6-7789.8 ns)
   3301.6 |######
   3526.0 |
   3750.4 |
   3974.8 |########################################
   4199.2 |
   4423.6 |
   4648.0 |
   4872.5 |
   5096.9 |
   5321.3 |
   5545.7 |
   5770.1 |
   5994.5 |
   6218.9 |
   6443.4 |
   6667.8 |
   6892.2 |######
   7116.6 |##############
   7341.0 |
   7565.4 |
  (6 below, 2 above range)

warm-container-native (n=40, range 3301.4-7983.1 ns)
   3301.4 |####
   3535.5 |
   3769.6 |
   4003.7 |########################################
   4237.8 |
   4471.9 |#
   4706.0 |
   4940.0 |
   5174.1 |
   5408.2 |
   5642.3 |
   5876.4 |
   6110.5 |
   6344.5 |
   6578.6 |
   6812.7 |
   7046.8 |
   7280.9 |#
   7515.0 |
   7749.1 |
  (6 below, 3 above range)

warm-container-plusone (n=40, range 3287.9-6658.8 ns)
   3287.9 |##########################
   3456.4 |
   3625.0 |
   3793.5 |
   3962.1 |########################################
   4130.6 |
   4299.2 |
   4467.7 |
   4636.3 |
   4804.8 |
   4973.3 |
   5141.9 |##
   5310.4 |
   5479.0 |
   5647.5 |
   5816.1 |
   5984.6 |
   6153.2 |
   6321.7 |
   6490.3 |
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=38.2% (high variance, measurements may be unstable)
- **warm-container-native**: CV=54.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=44.2% (high variance, measurements may be unstable)
