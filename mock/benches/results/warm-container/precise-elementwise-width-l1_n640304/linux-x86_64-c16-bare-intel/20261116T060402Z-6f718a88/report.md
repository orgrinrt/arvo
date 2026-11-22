# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 4.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 22% faster than the next best (warm-container-native)

warm-container-kernel (4.15 us) leads warm-container-native (5.05 us) by 22%, a clear separation rather than a photo finish. CV 61.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 35% (significant)

warm-container-kernel is -2.73 us (35%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is fastest but the noisiest (CV 61.2%)

warm-container-kernel wins on median (4.15 us) yet has the highest variance (CV 61.2%), while warm-container-plusone is the steadiest (CV 16.3%, 7.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.64)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (54% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 2.2x its best-20%

warm-container-kernel's best 20% of batches run at 4.13 us but its worst 20% at 9.21 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 4146.5 ns median (-46.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.88x (fastest 4146.5 ns, slowest 7783.1 ns)

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
| warm-container-headroom | 8603ns | 7873ns | 6439ns | 7814ns | 13131ns | base |
| warm-container-kernel | 5387ns | 4214ns | 4201ns | 4442ns | 9404ns | -37.39% |
| warm-container-minimum | 5477ns | 5151ns | 4216ns | 5149ns | 7721ns | -36.34% |
| warm-container-native | 5722ns | 5134ns | 4147ns | 5114ns | 9120ns | -33.49% |
| warm-container-plusone | 7245ns | 7867ns | 5918ns | 7240ns | 8587ns | -15.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8499ns | 6365ns | 12960ns | base | 4.819 |
| warm-container-kernel | 5292ns | 4135ns | 9210ns | -37.74% | 7.740 |
| warm-container-minimum | 5379ns | 4149ns | 7547ns | -36.71% | 7.615 |
| warm-container-native | 5619ns | 4079ns | 8918ns | -33.88% | 7.289 |
| warm-container-plusone | 7157ns | 5847ns | 8471ns | -15.79% | 5.723 |

## Performance model

- Peak throughput: **10.043 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.263 | 52.4% |
| warm-container-kernel | 9.878 | 98.4% |
| warm-container-minimum | 8.083 | 80.5% |
| warm-container-native | 8.107 | 80.7% |
| warm-container-plusone | 5.266 | 52.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8603ns | 8603ns | base |
| warm-container-kernel | 5387ns | 5387ns | -37.39% |
| warm-container-minimum | 5477ns | 5477ns | -36.34% |
| warm-container-native | 5722ns | 5722ns | -33.49% |
| warm-container-plusone | 7245ns | 7245ns | -15.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7783ns | base | --- | [7774, 7787] | --- | --- | --- | --- |
| warm-container-kernel | 4146ns | -2734.8ns (-35.1%) | [-3639, -2705]ns | [4145, 4590] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5067ns | -2711.9ns (-34.8%) | [-2721, -2708]ns | [5060, 5071] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 5052ns | -2723.0ns (-35.0%) | [-2768, -2704]ns | [4993, 5064] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7778ns | -510.8ns (-6.6%) | [-958, -9]ns | [6376, 7789] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 6368ns | -34.9% | -34.8% | -35.9% | -8.1% |
| 1 | 2 | 7783ns | -34.9% | -34.9% | -34.9% | +0.1% |
| 1 | 3 | 7781ns | -46.8% | -34.9% | -35.8% | -18.1% |
| 1 | 4 | 8979ns | -53.9% | -43.5% | -43.6% | -13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.002 | ok |
| warm-container-kernel | -0.413 | moderate- |
| warm-container-minimum | -0.081 | ok |
| warm-container-native | -0.127 | ok |
| warm-container-plusone | -0.639 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.6ns | 8499.1ns | 0.3% |  |
| warm-container-kernel | 25.9ns | 5291.7ns | 0.5% |  |
| warm-container-minimum | 26.4ns | 5378.8ns | 0.5% |  |
| warm-container-native | 27.8ns | 5619.2ns | 0.5% |  |
| warm-container-plusone | 23.6ns | 7157.4ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6364.6-12959.8 ns)
   6364.6 |########
   6694.3 |
   7024.1 |
   7353.9 |
   7683.6 |########################################
   8013.4 |#
   8343.1 |
   8672.9 |
   9002.6 |
   9332.4 |
   9662.2 |#
   9991.9 |
  10321.7 |
  10651.4 |
  10981.2 |
  11311.0 |
  11640.7 |#
  11970.5 |#
  12300.2 |
  12630.0 |
  (4 below, 3 above range)

warm-container-kernel (n=40, range 4134.6-9210.5 ns)
   4134.6 |########################################
   4388.3 |
   4642.1 |
   4895.9 |###############
   5149.7 |
   5403.5 |
   5657.3 |
   5911.1 |
   6164.9 |
   6418.7 |
   6672.5 |
   6926.3 |
   7180.1 |
   7433.9 |
   7687.7 |
   7941.5 |
   8195.3 |
   8449.1 |
   8702.9 |
   8956.7 |
  (3 below, 5 above range)

warm-container-minimum (n=40, range 4148.9-7547.3 ns)
   4148.9 |#####
   4318.8 |
   4488.7 |
   4658.7 |
   4828.6 |
   4998.5 |########################################
   5168.4 |
   5338.3 |
   5508.3 |
   5678.2 |
   5848.1 |
   6018.0 |
   6187.9 |#
   6357.8 |
   6527.8 |
   6697.7 |
   6867.6 |
   7037.5 |
   7207.4 |
   7377.4 |
  (4 below, 4 above range)

warm-container-native (n=40, range 4078.6-8918.3 ns)
   4078.6 |#######
   4320.6 |
   4562.6 |
   4804.6 |#####################
   5046.5 |########################################
   5288.5 |
   5530.5 |
   5772.5 |
   6014.5 |
   6256.5 |
   6498.5 |
   6740.5 |
   6982.4 |
   7224.4 |
   7466.4 |
   7708.4 |##
   7950.4 |##
   8192.4 |##
   8434.4 |
   8676.4 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 5847.2-8470.6 ns)
   5847.2 |#############
   5978.4 |
   6109.5 |
   6240.7 |######
   6371.9 |#############
   6503.1 |
   6634.2 |
   6765.4 |
   6896.6 |
   7027.7 |
   7158.9 |##
   7290.1 |
   7421.3 |
   7552.4 |
   7683.6 |########################################
   7814.8 |####
   7945.9 |
   8077.1 |
   8208.3 |
   8339.5 |
  (3 below, 1 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=35.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=48.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=30.0% (high variance, measurements may be unstable)
- **warm-container-native**: CV=38.9% (high variance, measurements may be unstable)
