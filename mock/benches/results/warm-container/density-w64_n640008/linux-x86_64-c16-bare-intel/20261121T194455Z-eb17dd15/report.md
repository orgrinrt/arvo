# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 56% (significant)

warm-container-kernel is -4.36 us (56%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.8x slower than the field

warm-container-plusone (9.57 us) is 2.8x the fastest (3.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 67.3%)

warm-container-kernel wins on median (3.47 us) yet has the highest variance (CV 67.3%), while warm-container-headroom is the steadiest (CV 4.6%, 7.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.75)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (75% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 75% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 2.9x its best-20%

warm-container-kernel's best 20% of batches run at 2.84 us but its worst 20% at 8.20 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 3468.0 ns median (-55.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.76x (fastest 3468.0 ns, slowest 9568.6 ns)

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
| warm-container-headroom | 7714ns | 7903ns | 7090ns | 7837ns | 7967ns | base |
| warm-container-kernel | 4271ns | 3548ns | 2904ns | 3339ns | 8431ns | -44.64% |
| warm-container-minimum | 4488ns | 3740ns | 3431ns | 3903ns | 7303ns | -41.82% |
| warm-container-native | 4993ns | 4569ns | 3742ns | 4570ns | 7511ns | -35.28% |
| warm-container-plusone | 9227ns | 9653ns | 7898ns | 9509ns | 9707ns | +19.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 7643ns | 7024ns | 7886ns | base | 9.647 |
| warm-container-kernel | 4165ns | 2838ns | 8204ns | -45.50% | 17.702 |
| warm-container-minimum | 4394ns | 3364ns | 7112ns | -42.50% | 16.778 |
| warm-container-native | 4895ns | 3674ns | 7337ns | -35.95% | 15.062 |
| warm-container-plusone | 9140ns | 7829ns | 9601ns | +19.59% | 8.066 |

## Performance model

- Peak throughput: **25.978 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.408 | 36.2% |
| warm-container-kernel | 21.260 | 81.8% |
| warm-container-minimum | 20.079 | 77.3% |
| warm-container-native | 16.431 | 63.2% |
| warm-container-plusone | 7.705 | 29.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 7714ns | 7714ns | base |
| warm-container-kernel | 4271ns | 4271ns | -44.64% |
| warm-container-minimum | 4488ns | 4488ns | -41.82% |
| warm-container-native | 4993ns | 4993ns | -35.28% |
| warm-container-plusone | 9227ns | 9227ns | +19.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7836ns | base | --- | [7828, 7837] | --- | --- | --- | --- |
| warm-container-kernel | 3468ns | -4357.1ns (-55.6%) | [-4366, -4185]ns | [2849, 3471] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3672ns | -3676.5ns (-46.9%) | [-4157, -3577]ns | [3670, 3682] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 4487ns | -3343.4ns (-42.7%) | [-3352, -3311]ns | [4486, 4494] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 9569ns | +1732.6ns (+22.1%) | [+1725, +1742]ns | [9562, 9572] | YES | 0.0007 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7028ns | -59.6% | -52.2% | -36.0% | +36.2% |
| 1 | 2 | 7837ns | -55.7% | -53.1% | -53.1% | +22.1% |
| 1 | 3 | 7837ns | -63.7% | -42.7% | -42.7% | -0.0% |
| 1 | 4 | 7839ns | -55.7% | -53.2% | -42.7% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.083 | ok |
| warm-container-kernel | -0.748 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.170 | ok |
| warm-container-native | -0.419 | moderate- |
| warm-container-plusone | -0.417 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 20.9ns | 7642.6ns | 0.3% |  |
| warm-container-kernel | 30.4ns | 4164.9ns | 0.7% |  |
| warm-container-minimum | 26.0ns | 4394.3ns | 0.6% |  |
| warm-container-native | 28.3ns | 4895.0ns | 0.6% |  |
| warm-container-plusone | 24.5ns | 9140.1ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7024.1-7885.7 ns)
   7024.1 |##########
   7067.2 |
   7110.3 |
   7153.3 |
   7196.4 |
   7239.5 |
   7282.6 |
   7325.7 |
   7368.7 |
   7411.8 |
   7454.9 |
   7498.0 |
   7541.1 |
   7584.1 |
   7627.2 |
   7670.3 |
   7713.4 |
   7756.5 |
   7799.5 |########################################
   7842.6 |####
  (3 below, 1 above range)

warm-container-kernel (n=40, range 2838.1-8203.9 ns)
   2838.1 |#########################
   3106.4 |
   3374.7 |########################################
   3643.0 |
   3911.2 |
   4179.5 |
   4447.8 |
   4716.1 |
   4984.4 |
   5252.7 |
   5521.0 |##
   5789.3 |
   6057.6 |
   6325.9 |
   6594.2 |
   6862.5 |##
   7130.8 |
   7399.0 |
   7667.3 |
   7935.6 |
  (5 below, 5 above range)

warm-container-minimum (n=40, range 3364.2-7112.2 ns)
   3364.2 |####
   3551.6 |########################################
   3739.0 |
   3926.4 |
   4113.8 |
   4301.2 |#############
   4488.6 |######
   4676.0 |
   4863.4 |
   5050.8 |
   5238.2 |
   5425.6 |
   5613.0 |
   5800.4 |
   5987.8 |
   6175.2 |
   6362.6 |
   6550.0 |
   6737.4 |
   6924.8 |##
  (7 below, 3 above range)

warm-container-native (n=40, range 3674.4-7337.0 ns)
   3674.4 |####
   3857.5 |
   4040.6 |
   4223.8 |
   4406.9 |########################################
   4590.0 |
   4773.2 |
   4956.3 |
   5139.4 |
   5322.6 |
   5505.7 |
   5688.8 |
   5872.0 |
   6055.1 |
   6238.2 |
   6421.4 |
   6604.5 |
   6787.6 |
   6970.7 |#
   7153.9 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 7829.3-9600.9 ns)
   7829.3 |##########
   7917.9 |
   8006.4 |
   8095.0 |
   8183.6 |
   8272.2 |
   8360.8 |
   8449.3 |
   8537.9 |
   8626.5 |
   8715.1 |
   8803.7 |
   8892.2 |
   8980.8 |
   9069.4 |
   9158.0 |
   9246.6 |
   9335.1 |
   9423.7 |
   9512.3 |########################################
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=56.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=43.4% (high variance, measurements may be unstable)
- **warm-container-native**: CV=39.0% (high variance, measurements may be unstable)
