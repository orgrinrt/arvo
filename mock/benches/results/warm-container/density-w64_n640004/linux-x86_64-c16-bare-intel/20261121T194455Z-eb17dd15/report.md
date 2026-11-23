# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 66% (significant)

warm-container-kernel is -3.65 us (66%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.9x slower than the field

warm-container-headroom (5.57 us) is 2.9x the fastest (1.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-minimum) are a dead heat (<1%)

warm-container-kernel (1.92 us) and warm-container-minimum (1.93 us) differ by 0.61%, inside the noise, even though the wider field spreads 190.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.71)

warm-container-native's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-minimum is inconsistent: worst-20% is 4.3x its best-20%

warm-container-minimum's best 20% of batches run at 1.62 us but its worst 20% at 6.92 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1919.7 ns median (-65.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.90x (fastest 1919.7 ns, slowest 5573.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 64 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112747662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5958ns | 5655ns | 5642ns | 5653ns | 7190ns | base |
| warm-container-kernel | 3379ns | 2001ns | 1636ns | 2771ns | 6946ns | -43.29% |
| warm-container-minimum | 3918ns | 2014ns | 1684ns | 3545ns | 7274ns | -34.24% |
| warm-container-native | 3750ns | 3235ns | 1773ns | 3411ns | 6741ns | -37.07% |
| warm-container-plusone | 5461ns | 5644ns | 4624ns | 5322ns | 6715ns | -8.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5864ns | 5561ns | 7049ns | base | 6.985 |
| warm-container-kernel | 3234ns | 1570ns | 6622ns | -44.85% | 12.666 |
| warm-container-minimum | 3747ns | 1616ns | 6920ns | -36.10% | 10.931 |
| warm-container-native | 3585ns | 1701ns | 6411ns | -38.88% | 11.427 |
| warm-container-plusone | 5374ns | 4558ns | 6592ns | -8.36% | 7.622 |

## Performance model

- Peak throughput: **26.084 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.349 | 28.2% |
| warm-container-kernel | 21.337 | 81.8% |
| warm-container-minimum | 21.207 | 81.3% |
| warm-container-native | 13.213 | 50.7% |
| warm-container-plusone | 7.363 | 28.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5958ns | 5958ns | base |
| warm-container-kernel | 3379ns | 3379ns | -43.29% |
| warm-container-minimum | 3918ns | 3918ns | -34.24% |
| warm-container-native | 3750ns | 3750ns | -37.07% |
| warm-container-plusone | 5461ns | 5461ns | -8.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5573ns | base | --- | [5563, 5575] | --- | --- | --- | --- |
| warm-container-kernel | 1920ns | -3653.8ns (-65.6%) | [-3991, -3088]ns | [1576, 4282] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1931ns | -3641.7ns (-65.3%) | [-3655, -362]ns | [1919, 5475] | YES | 0.0030 | 0.0022 | 0 |
| warm-container-native | 3100ns | -3290.2ns (-59.0%) | [-3644, -1177]ns | [1918, 4600] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5563ns | -393.8ns (-7.1%) | [-671, -1]ns | [4910, 5574] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 5573ns | -71.8% | -17.8% | -44.6% | +0.1% |
| 1 | 2 | 5573ns | -65.6% | -71.7% | -65.6% | +0.1% |
| 1 | 3 | 5573ns | -19.1% | -30.5% | -15.2% | -11.8% |
| 1 | 4 | 5573ns | -66.6% | -65.4% | -65.5% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.179 | ok |
| warm-container-kernel | -0.330 | moderate- |
| warm-container-minimum | -0.689 | HIGH- (thermal bounce) |
| warm-container-native | -0.714 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.286 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 26.7ns | 5864.4ns | 0.5% |  |
| warm-container-kernel | 42.5ns | 3234.0ns | 1.3% |  |
| warm-container-minimum | 50.5ns | 3747.2ns | 1.3% |  |
| warm-container-native | 48.1ns | 3584.5ns | 1.3% |  |
| warm-container-plusone | 25.3ns | 5373.9ns | 0.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5561.3-7049.3 ns)
   5561.3 |########################################
   5635.7 |
   5710.1 |#
   5784.5 |
   5858.9 |
   5933.3 |
   6007.7 |
   6082.1 |
   6156.5 |
   6230.9 |
   6305.3 |
   6379.7 |
   6454.1 |#
   6528.5 |
   6602.9 |
   6677.3 |
   6751.7 |
   6826.1 |
   6900.5 |
   6974.9 |
  (3 below, 2 above range)

warm-container-kernel (n=40, range 1570.3-6621.5 ns)
   1570.3 |########################################
   1822.9 |#########################
   2075.4 |###
   2328.0 |###
   2580.6 |
   2833.1 |
   3085.7 |
   3338.2 |
   3590.8 |
   3843.4 |
   4095.9 |###
   4348.5 |###
   4601.0 |#######
   4853.6 |#######
   5106.2 |
   5358.7 |#######
   5611.3 |###
   5863.8 |
   6116.4 |###
   6369.0 |###
  (5 below, 4 above range)

warm-container-minimum (n=40, range 1616.2-6920.0 ns)
   1616.2 |
   1881.4 |########################################
   2146.6 |##
   2411.8 |
   2677.0 |
   2942.2 |
   3207.4 |
   3472.6 |
   3737.8 |
   4002.9 |##
   4268.1 |
   4533.3 |
   4798.5 |
   5063.7 |#####
   5328.9 |########
   5594.1 |#####
   5859.3 |#####
   6124.5 |#####
   6389.7 |
   6654.8 |########
  (7 below, 3 above range)

warm-container-native (n=40, range 1701.3-6410.7 ns)
   1701.3 |########################################
   1936.8 |
   2172.3 |
   2407.7 |##
   2643.2 |
   2878.7 |
   3114.1 |
   3349.6 |##
   3585.1 |
   3820.6 |
   4056.0 |#####
   4291.5 |#####
   4527.0 |#####
   4762.4 |###########
   4997.9 |
   5233.4 |########
   5468.8 |#####
   5704.3 |
   5939.8 |
   6175.2 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 4557.6-6591.7 ns)
   4557.6 |###############
   4659.3 |
   4761.0 |
   4862.7 |###########################
   4964.4 |
   5066.1 |
   5167.8 |
   5269.6 |
   5371.3 |
   5473.0 |#####################
   5574.7 |########################################
   5676.4 |
   5778.1 |
   5879.8 |
   5981.5 |
   6083.2 |
   6184.9 |
   6286.6 |
   6388.3 |
   6490.0 |
  (5 below, 1 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=20.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=64.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=59.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-native**: CV=53.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=26.0% (high variance, measurements may be unstable)
