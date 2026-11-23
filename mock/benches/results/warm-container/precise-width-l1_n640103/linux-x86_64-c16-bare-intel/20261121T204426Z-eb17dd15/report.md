# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 3514% faster than the next best (warm-container-native)

warm-container-kernel (65 ns) leads warm-container-native (2.35 us) by 3514%, a clear separation rather than a photo finish. CV 118.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -9.30 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 159.9x slower than the field

warm-container-plusone (10.40 us) is 159.9x the fastest (65 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 118.2%)

warm-container-kernel wins on median (65 ns) yet has the highest variance (CV 118.2%), while warm-container-headroom is the steadiest (CV 9.6%, 9.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.75)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (3514% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 3514% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 159.9x the fastest

Fastest warm-container-kernel (65 ns) to slowest warm-container-plusone (10.40 us): 159.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.7x its best-20%

warm-container-native's best 20% of batches run at 2.35 us but its worst 20% at 8.65 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 65.0 ns median (-99.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 159.86x (fastest 65.0 ns, slowest 10399.1 ns)

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
| Counter frequency | 2112691662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9954ns | 9421ns | 9416ns | 9611ns | 11521ns | base |
| warm-container-kernel | 406ns | 356ns | 252ns | 389ns | 610ns | -95.93% |
| warm-container-minimum | 4284ns | 2962ns | 2956ns | 3198ns | 8868ns | -56.97% |
| warm-container-native | 4008ns | 2419ns | 2417ns | 2890ns | 8952ns | -59.73% |
| warm-container-plusone | 10470ns | 10497ns | 9414ns | 10468ns | 11529ns | +5.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9877ns | 9348ns | 11431ns | base | 3.318 |
| warm-container-kernel | 83ns | 47ns | 159ns | -99.16% | 393.681 |
| warm-container-minimum | 4154ns | 2873ns | 8570ns | -57.94% | 7.888 |
| warm-container-native | 3887ns | 2351ns | 8654ns | -60.64% | 8.430 |
| warm-container-plusone | 10390ns | 9347ns | 11434ns | +5.20% | 3.154 |

## Performance model

- Peak throughput: **699.797 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.504 | 0.5% |
| warm-container-kernel | 503.736 | 72.0% |
| warm-container-minimum | 11.380 | 1.6% |
| warm-container-native | 13.937 | 2.0% |
| warm-container-plusone | 3.151 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9954ns | 9954ns | base |
| warm-container-kernel | 406ns | 406ns | -95.93% |
| warm-container-minimum | 4284ns | 4284ns | -56.97% |
| warm-container-native | 4008ns | 4008ns | -59.73% |
| warm-container-plusone | 10470ns | 10470ns | +5.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 9351ns | base | --- | [9351, 9353] | --- | --- | --- | --- |
| warm-container-kernel | 65ns | -9295.8ns (-99.4%) | [-9306, -9286]ns | [65, 74] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2880ns | -6476.1ns (-69.3%) | [-6477, -6464]ns | [2874, 2887] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2351ns | -6999.2ns (-74.9%) | [-7000, -6988]ns | [2351, 2362] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 10399ns | no significant difference | [-1, +2076]ns | [9351, 11427] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 11429ns | -99.1% | -74.9% | -79.4% | -18.2% |
| 1 | 2 | 9350ns | -99.3% | -69.2% | -74.9% | +22.2% |
| 1 | 3 | 9350ns | -99.2% | -69.1% | -74.9% | -0.0% |
| 1 | 4 | 9351ns | -99.5% | -69.2% | -74.8% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.083 | ok |
| warm-container-kernel | -0.171 | ok |
| warm-container-minimum | -0.020 | ok |
| warm-container-native | -0.088 | ok |
| warm-container-plusone | -0.750 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 21.0ns | 9876.6ns | 0.2% |  |
| warm-container-kernel | 121.4ns | 83.2ns | 145.8% | HIGH |
| warm-container-minimum | 35.0ns | 4154.0ns | 0.8% |  |
| warm-container-native | 32.9ns | 3887.0ns | 0.8% |  |
| warm-container-plusone | 22.0ns | 10390.4ns | 0.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 9347.5-11430.5 ns)
   9347.5 |########################################
   9451.7 |###
   9555.8 |
   9660.0 |
   9764.1 |
   9868.3 |
   9972.4 |
  10076.6 |
  10180.7 |
  10284.9 |
  10389.0 |
  10493.2 |
  10597.3 |
  10701.5 |
  10805.6 |
  10909.8 |
  11013.9 |
  11118.1 |
  11222.2 |
  11326.4 |##########
  (2 below, 3 above range)

warm-container-kernel (n=40, range 46.8-158.9 ns)
     46.8 |####################
     52.4 |
     58.0 |
     63.6 |########################################
     69.2 |#######################
     74.8 |
     80.5 |
     86.1 |###
     91.7 |
     97.3 |######
    102.9 |#######################
    108.5 |
    114.1 |
    119.7 |
    125.3 |
    130.9 |
    136.5 |
    142.1 |
    147.7 |
    153.3 |
  (4 below, 1 above range)

warm-container-minimum (n=40, range 2873.4-8570.3 ns)
   2873.4 |########################################
   3158.3 |
   3443.1 |
   3727.9 |
   4012.8 |
   4297.6 |
   4582.5 |
   4867.3 |#
   5152.2 |
   5437.0 |
   5721.8 |
   6006.7 |
   6291.5 |#
   6576.4 |
   6861.2 |
   7146.1 |###
   7430.9 |
   7715.7 |#
   8000.6 |#
   8285.4 |
  (5 below, 4 above range)

warm-container-native (n=40, range 2350.9-8653.7 ns)
   2350.9 |########################################
   2666.0 |
   2981.1 |
   3296.3 |
   3611.4 |
   3926.6 |
   4241.7 |#
   4556.8 |
   4872.0 |
   5187.1 |
   5502.3 |
   5817.4 |
   6132.6 |#
   6447.7 |
   6762.8 |
   7078.0 |
   7393.1 |#
   7708.3 |###
   8023.4 |#
   8338.5 |###
  (3 below, 3 above range)

warm-container-plusone (n=40, range 9347.3-11434.0 ns)
   9347.3 |########################################
   9451.6 |
   9555.9 |
   9660.3 |
   9764.6 |
   9869.0 |
   9973.3 |
  10077.6 |
  10182.0 |
  10286.3 |
  10390.6 |
  10495.0 |
  10599.3 |
  10703.6 |
  10808.0 |
  10912.3 |
  11016.7 |
  11121.0 |
  11225.3 |
  11329.7 |#####################################
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=92.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-kernel**: bridge=164.5% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=56.6% (high variance, measurements may be unstable)
- **warm-container-native**: CV=67.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.7x (possible bimodal distribution)
