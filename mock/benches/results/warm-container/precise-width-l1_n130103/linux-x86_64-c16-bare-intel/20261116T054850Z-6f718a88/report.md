# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 288% faster than the next best (warm-container-headroom)

warm-container-kernel (1.03 us) leads warm-container-headroom (4.01 us) by 288%, a clear separation rather than a photo finish. CV 9.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 72% (significant)

warm-container-kernel is -2.88 us (72%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native is an outlier: 8.6x slower than the field

warm-container-native (8.92 us) is 8.6x the fastest (1.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.56)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} (288% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} with a 288% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.6x the fastest

Fastest warm-container-kernel (1.03 us) to slowest warm-container-native (8.92 us): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 2.6x its best-20%

warm-container-headroom's best 20% of batches run at 3.28 us but its worst 20% at 8.40 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1034.5 ns median (-74.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 8.63x (fastest 1034.5 ns, slowest 8924.5 ns)

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
| warm-container-headroom | 4658ns | 4075ns | 3334ns | 3788ns | 8592ns | base |
| warm-container-kernel | 1273ns | 1303ns | 1114ns | 1269ns | 1445ns | -72.67% |
| warm-container-minimum | 7172ns | 5914ns | 4884ns | 6256ns | 12206ns | +53.96% |
| warm-container-native | 9249ns | 8997ns | 8069ns | 9192ns | 10603ns | +98.57% |
| warm-container-plusone | 7185ns | 6171ns | 5202ns | 6746ns | 10484ns | +54.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4574ns | 3283ns | 8398ns | base | 7.164 |
| warm-container-kernel | 1049ns | 926ns | 1166ns | -77.07% | 31.251 |
| warm-container-minimum | 7092ns | 4825ns | 12089ns | +55.06% | 4.620 |
| warm-container-native | 9169ns | 7995ns | 10507ns | +100.48% | 3.574 |
| warm-container-plusone | 7102ns | 5142ns | 10350ns | +55.29% | 4.614 |

## Performance model

- Peak throughput: **35.389 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.167 | 23.1% |
| warm-container-kernel | 31.675 | 89.5% |
| warm-container-minimum | 5.629 | 15.9% |
| warm-container-native | 3.672 | 10.4% |
| warm-container-plusone | 5.373 | 15.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4658ns | 4658ns | base |
| warm-container-kernel | 1273ns | 1273ns | -72.67% |
| warm-container-minimum | 7172ns | 7172ns | +53.96% |
| warm-container-native | 9249ns | 9249ns | +98.57% |
| warm-container-plusone | 7185ns | 7185ns | +54.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4012ns | base | --- | [3294, 4013] | --- | --- | --- | --- |
| warm-container-kernel | 1034ns | -2884.2ns (-71.9%) | [-2912, -2360]ns | [1006, 1107] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5821ns | +1607.5ns (+40.1%) | [+1464, +2184]ns | [5067, 6222] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 8925ns | +4909.9ns (+122.4%) | [+4735, +5611]ns | [8777, 9878] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6099ns | +2092.8ns (+52.2%) | [+2034, +3121]ns | [6008, 7966] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3287ns | -66.3% | +267.5% | +164.5% | +62.4% |
| 1 | 2 | 4025ns | -75.0% | +26.4% | +100.3% | +50.6% |
| 1 | 3 | 4013ns | -72.4% | +54.7% | +143.1% | +99.0% |
| 1 | 4 | 3286ns | -71.3% | +48.4% | +202.2% | +124.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.250 | moderate- |
| warm-container-kernel | -0.561 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.184 | ok |
| warm-container-native | 0.121 | ok |
| warm-container-plusone | 0.214 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 0/4, lost 4/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 28.4ns | 4573.7ns | 0.6% |  |
| warm-container-kernel | 94.9ns | 1048.5ns | 9.0% | HIGH |
| warm-container-minimum | 25.8ns | 7092.0ns | 0.4% |  |
| warm-container-native | 25.6ns | 9169.3ns | 0.3% |  |
| warm-container-plusone | 26.6ns | 7102.3ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3282.7-8397.9 ns)
   3282.7 |##################################
   3538.4 |##
   3794.2 |########################################
   4050.0 |##
   4305.7 |##
   4561.5 |
   4817.2 |
   5073.0 |
   5328.8 |
   5584.5 |
   5840.3 |
   6096.1 |
   6351.8 |
   6607.6 |
   6863.4 |
   7119.1 |
   7374.9 |
   7630.7 |
   7886.4 |
   8142.2 |
  (4 below, 5 above range)

warm-container-kernel (n=40, range 925.9-1166.0 ns)
    925.9 |
    937.9 |##
    949.9 |##
    961.9 |#####
    974.0 |
    986.0 |
    998.0 |#########################
   1010.0 |
   1022.0 |
   1034.0 |
   1046.0 |
   1058.0 |##
   1070.0 |
   1082.0 |
   1094.0 |
   1106.0 |########################################
   1118.0 |
   1130.0 |
   1142.0 |##
   1154.0 |##
  (7 below, 3 above range)

warm-container-minimum (n=40, range 4825.4-12089.3 ns)
   4825.4 |########################################
   5188.6 |#############
   5551.8 |
   5915.0 |##############################
   6278.2 |###
   6641.4 |
   7004.5 |
   7367.7 |
   7730.9 |
   8094.1 |
   8457.3 |
   8820.5 |
   9183.7 |
   9546.9 |
   9910.1 |
  10273.3 |
  10636.5 |
  10999.7 |
  11362.9 |
  11726.1 |##########################
  (4 below, 2 above range)

warm-container-native (n=40, range 7995.0-10507.4 ns)
   7995.0 |########################################
   8120.6 |
   8246.2 |
   8371.8 |
   8497.5 |###########
   8623.1 |
   8748.7 |######################
   8874.3 |############################
   8999.9 |#####
   9125.6 |#####
   9251.2 |
   9376.8 |
   9502.4 |
   9628.0 |
   9753.7 |###########
   9879.3 |########################################
  10004.9 |#####
  10130.5 |
  10256.1 |
  10381.8 |###########
  (4 below, 4 above range)

warm-container-plusone (n=40, range 5142.1-10349.9 ns)
   5142.1 |###################################
   5402.5 |
   5662.9 |###############
   5923.3 |########################################
   6183.7 |
   6444.1 |##########
   6704.5 |#####
   6964.8 |
   7225.2 |
   7485.6 |
   7746.0 |##############################
   8006.4 |##############################
   8266.8 |
   8527.2 |
   8787.6 |
   9047.9 |#####
   9308.3 |
   9568.7 |
   9829.1 |
  10089.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=41.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=32.1% (high variance, measurements may be unstable)
