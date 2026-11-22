# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 21% faster than the next best (warm-container-plusone)

warm-container-minimum (3.11 us) leads warm-container-plusone (3.76 us) by 21%, a clear separation rather than a photo finish. CV 62.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 46% (significant)

warm-container-minimum is -3.74 us (46%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 3.3x slower than the field

warm-container-kernel (10.29 us) is 3.3x the fastest (3.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.70)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-plusone, warm-container-native} vs {warm-container-headroom, warm-container-kernel} (116% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-plusone, warm-container-native} and a slow tier {warm-container-headroom, warm-container-kernel} with a 116% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest warm-container-minimum (3.11 us) to slowest warm-container-kernel (10.29 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone is inconsistent: worst-20% is 3.2x its best-20%

warm-container-plusone's best 20% of batches run at 2.81 us but its worst 20% at 9.04 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 3106.8 ns median (-61.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.31x (fastest 3106.8 ns, slowest 10288.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112840787 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 7610ns | 8252ns | 6623ns | 7667ns | 8427ns | base |
| warm-container-kernel | 9849ns | 10419ns | 8830ns | 9868ns | 10814ns | +29.42% |
| warm-container-minimum | 4072ns | 3173ns | 3144ns | 3335ns | 7209ns | -46.50% |
| warm-container-native | 4835ns | 3852ns | 3514ns | 3853ns | 9101ns | -36.47% |
| warm-container-plusone | 4744ns | 3848ns | 2877ns | 3848ns | 9302ns | -37.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 7530ns | 6557ns | 8333ns | base | 5.439 |
| warm-container-kernel | 9768ns | 8759ns | 10721ns | +29.72% | 4.193 |
| warm-container-minimum | 3975ns | 3076ns | 7002ns | -47.22% | 10.305 |
| warm-container-native | 4719ns | 3432ns | 8847ns | -37.34% | 8.680 |
| warm-container-plusone | 4629ns | 2810ns | 9039ns | -38.53% | 8.849 |

## Performance model

- Peak throughput: **14.574 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.039 | 34.6% |
| warm-container-kernel | 3.981 | 27.3% |
| warm-container-minimum | 13.184 | 90.5% |
| warm-container-native | 10.863 | 74.5% |
| warm-container-plusone | 10.879 | 74.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 7610ns | 7610ns | base |
| warm-container-kernel | 9849ns | 9849ns | +29.42% |
| warm-container-minimum | 4072ns | 4072ns | -46.50% |
| warm-container-native | 4835ns | 4835ns | -36.47% |
| warm-container-plusone | 4744ns | 4744ns | -37.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8128ns | base | --- | [6815, 8328] | --- | --- | --- | --- |
| warm-container-kernel | 10289ns | +2381.3ns (+29.3%) | [+1953, +2394]ns | [8769, 10713] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3107ns | -3741.8ns (-46.0%) | [-4557, -3477]ns | [3090, 3441] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 3771ns | -3051.6ns (-37.5%) | [-4535, -2800]ns | [3757, 3784] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3765ns | -3048.2ns (-37.5%) | [-4558, -2790]ns | [3758, 3771] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 8329ns | +28.7% | -54.8% | -54.5% | -66.3% |
| 1 | 2 | 6815ns | +28.7% | -54.9% | -44.8% | -44.7% |
| 1 | 3 | 8329ns | +5.3% | -62.9% | -58.7% | -54.8% |
| 1 | 4 | 6562ns | +63.3% | -52.5% | -42.5% | -42.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.703 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.250 | moderate- |
| warm-container-minimum | -0.107 | ok |
| warm-container-native | -0.369 | moderate- |
| warm-container-plusone | -0.088 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 22.8ns | 7530.1ns | 0.3% |  |
| warm-container-kernel | 22.6ns | 9768.1ns | 0.2% |  |
| warm-container-minimum | 26.2ns | 3974.6ns | 0.7% |  |
| warm-container-native | 31.9ns | 4718.7ns | 0.7% |  |
| warm-container-plusone | 31.2ns | 4628.8ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6556.7-8333.4 ns)
   6556.7 |##############
   6645.6 |
   6734.4 |############
   6823.2 |####
   6912.1 |
   7000.9 |
   7089.7 |
   7178.6 |
   7267.4 |
   7356.2 |
   7445.0 |
   7533.9 |
   7622.7 |
   7711.5 |
   7800.4 |
   7889.2 |##
   7978.0 |
   8066.9 |
   8155.7 |
   8244.5 |########################################
  (4 below, 1 above range)

warm-container-kernel (n=40, range 8758.7-10720.6 ns)
   8758.7 |###################################
   8856.8 |
   8954.9 |
   9053.0 |
   9151.1 |
   9249.2 |
   9347.3 |
   9445.3 |
   9543.4 |
   9641.5 |
   9739.6 |
   9837.7 |##
   9935.8 |
  10033.9 |
  10132.0 |
  10230.1 |
  10328.2 |
  10426.3 |
  10524.4 |
  10622.5 |########################################
  (3 below, 2 above range)

warm-container-minimum (n=40, range 3075.9-7002.1 ns)
   3075.9 |########################################
   3272.2 |
   3468.5 |
   3664.8 |#################
   3861.1 |
   4057.4 |
   4253.8 |
   4450.1 |
   4646.4 |
   4842.7 |
   5039.0 |
   5235.3 |
   5431.6 |
   5627.9 |
   5824.3 |
   6020.6 |
   6216.9 |
   6413.2 |
   6609.5 |
   6805.8 |#
  (5 below, 4 above range)

warm-container-native (n=40, range 3432.2-8846.9 ns)
   3432.2 |######
   3702.9 |########################################
   3973.6 |
   4244.4 |#
   4515.1 |
   4785.8 |
   5056.6 |
   5327.3 |
   5598.1 |
   5868.8 |
   6139.5 |
   6410.3 |
   6681.0 |
   6951.8 |
   7222.5 |
   7493.2 |
   7764.0 |###
   8034.7 |
   8305.5 |
   8576.2 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 2810.4-9039.2 ns)
   2810.4 |###
   3121.9 |
   3433.3 |
   3744.7 |########################################
   4056.2 |
   4367.6 |
   4679.1 |
   4990.5 |
   5301.9 |
   5613.4 |
   5924.8 |
   6236.3 |
   6547.7 |
   6859.2 |
   7170.6 |#
   7482.0 |#
   7793.5 |
   8104.9 |
   8416.4 |
   8727.8 |
  (6 below, 5 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=48.8% (high variance, measurements may be unstable)
- **warm-container-native**: CV=53.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=54.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.2x (possible bimodal distribution)
