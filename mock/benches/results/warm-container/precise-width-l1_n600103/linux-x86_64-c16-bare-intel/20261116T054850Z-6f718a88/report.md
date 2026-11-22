# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (9.37 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 199% faster than the next best (warm-container-minimum)

warm-container-kernel (1.11 us) leads warm-container-minimum (3.31 us) by 199%, a clear separation rather than a photo finish. CV 27.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 89% (significant)

warm-container-kernel is -8.30 us (89%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 8.5x slower than the field

warm-container-headroom (9.37 us) is 8.5x the fastest (1.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.74)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.5x the fastest

Fastest warm-container-kernel (1.11 us) to slowest warm-container-headroom (9.37 us): 8.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 2.7x its best-20%

warm-container-minimum's best 20% of batches run at 3.30 us but its worst 20% at 9.01 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1106.1 ns median (-88.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 8.48x (fastest 1106.1 ns, slowest 9374.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112536030 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9451ns | 9463ns | 8492ns | 9449ns | 10418ns | base |
| warm-container-kernel | 1457ns | 1332ns | 1137ns | 1368ns | 2045ns | -84.58% |
| warm-container-minimum | 4678ns | 3377ns | 3365ns | 3591ns | 9252ns | -50.50% |
| warm-container-native | 4867ns | 4115ns | 3417ns | 4114ns | 8574ns | -48.51% |
| warm-container-plusone | 4469ns | 4114ns | 3354ns | 3778ns | 7657ns | -52.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9372ns | 8426ns | 10322ns | base | 3.496 |
| warm-container-kernel | 1173ns | 918ns | 1686ns | -87.48% | 27.926 |
| warm-container-minimum | 4574ns | 3299ns | 9015ns | -51.20% | 7.164 |
| warm-container-native | 4759ns | 3350ns | 8352ns | -49.22% | 6.885 |
| warm-container-plusone | 4370ns | 3288ns | 7458ns | -53.38% | 7.499 |

## Performance model

- Peak throughput: **35.711 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.495 | 9.8% |
| warm-container-kernel | 29.626 | 83.0% |
| warm-container-minimum | 9.901 | 27.7% |
| warm-container-native | 8.124 | 22.8% |
| warm-container-plusone | 8.123 | 22.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9451ns | 9451ns | base |
| warm-container-kernel | 1457ns | 1457ns | -84.58% |
| warm-container-minimum | 4678ns | 4678ns | -50.50% |
| warm-container-native | 4867ns | 4867ns | -48.51% |
| warm-container-plusone | 4469ns | 4469ns | -52.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 9374ns | base | --- | [8443, 10302] | --- | --- | --- | --- |
| warm-container-kernel | 1106ns | -8299.5ns (-88.5%) | [-9195, -7452]ns | [1006, 1126] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3310ns | -5143.6ns (-54.9%) | [-6258, -5130]ns | [3300, 4033] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 4033ns | -5147.0ns (-54.9%) | [-6267, -4582]ns | [4025, 4044] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4034ns | -5152.7ns (-55.0%) | [-6267, -5132]ns | [3301, 4035] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 8430ns | -81.3% | -60.9% | -52.0% | -61.0% |
| 1 | 2 | 8435ns | -88.8% | -60.8% | -60.8% | -52.1% |
| 1 | 3 | 10302ns | -89.3% | -67.9% | -61.0% | -68.0% |
| 1 | 4 | 10303ns | -90.7% | -60.8% | -60.7% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.251 | moderate+ |
| warm-container-kernel | -0.267 | moderate- |
| warm-container-minimum | -0.081 | ok |
| warm-container-native | -0.409 | moderate- |
| warm-container-plusone | -0.744 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 22.7ns | 9372.2ns | 0.2% |  |
| warm-container-kernel | 101.4ns | 1173.4ns | 8.6% | HIGH |
| warm-container-minimum | 28.0ns | 4573.9ns | 0.6% |  |
| warm-container-native | 29.2ns | 4759.5ns | 0.6% |  |
| warm-container-plusone | 27.1ns | 4369.7ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8426.3-10322.3 ns)
   8426.3 |########################################
   8521.1 |
   8615.9 |
   8710.7 |
   8805.5 |
   8900.3 |
   8995.1 |
   9089.9 |
   9184.7 |
   9279.5 |
   9374.3 |
   9469.1 |
   9563.9 |
   9658.7 |
   9753.5 |
   9848.3 |
   9943.1 |
  10037.9 |
  10132.7 |
  10227.5 |########################################
  (2 below, 2 above range)

warm-container-kernel (n=40, range 917.6-1685.8 ns)
    917.6 |############################
    956.0 |##################################
    994.4 |##################################
   1032.8 |
   1071.2 |########################################
   1109.6 |#####
   1148.1 |#####
   1186.5 |#####
   1224.9 |#####
   1263.3 |
   1301.7 |
   1340.1 |
   1378.5 |#################
   1416.9 |
   1455.3 |#####
   1493.7 |
   1532.2 |
   1570.6 |#################
   1609.0 |
   1647.4 |
  (2 below, 3 above range)

warm-container-minimum (n=40, range 3299.1-9014.6 ns)
   3299.1 |########################################
   3584.9 |
   3870.6 |##############
   4156.4 |
   4442.2 |
   4728.0 |
   5013.7 |
   5299.5 |#
   5585.3 |
   5871.1 |#
   6156.9 |
   6442.6 |
   6728.4 |
   7014.2 |
   7300.0 |
   7585.7 |
   7871.5 |
   8157.3 |
   8443.1 |
   8728.8 |
  (3 below, 5 above range)

warm-container-native (n=40, range 3349.9-8352.4 ns)
   3349.9 |
   3600.0 |#
   3850.1 |########################################
   4100.2 |#
   4350.4 |
   4600.5 |
   4850.6 |
   5100.7 |
   5350.9 |
   5601.0 |
   5851.1 |
   6101.2 |
   6351.4 |
   6601.5 |
   6851.6 |
   7101.7 |
   7351.9 |
   7602.0 |
   7852.1 |
   8102.2 |
  (7 below, 5 above range)

warm-container-plusone (n=40, range 3287.8-7457.8 ns)
   3287.8 |##################################
   3496.3 |
   3704.8 |
   3913.3 |########################################
   4121.8 |##
   4330.3 |
   4538.8 |
   4747.3 |
   4955.8 |
   5164.3 |##
   5372.8 |
   5581.3 |
   5789.8 |
   5998.3 |
   6206.8 |
   6415.3 |
   6623.8 |
   6832.3 |##
   7040.8 |
   7249.3 |
  (6 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=26.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=58.7% (high variance, measurements may be unstable)
- **warm-container-native**: CV=49.7% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=50.7% (high variance, measurements may be unstable)
