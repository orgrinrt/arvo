# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 80% faster than the next best (warm-container-native)

warm-container-minimum (2.00 us) leads warm-container-native (3.60 us) by 80%, a clear separation rather than a photo finish. CV 111.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 49% (significant)

warm-container-minimum is -1.95 us (49%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 2.5x slower than the field

warm-container-kernel (4.93 us) is 2.5x the fastest (2.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 111.5%)

warm-container-minimum wins on median (2.00 us) yet has the highest variance (CV 111.5%), while warm-container-kernel is the steadiest (CV 35.7%, 4.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.81)

warm-container-native's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum} vs {warm-container-native, warm-container-plusone, warm-container-headroom, warm-container-kernel} (80% apart)

The field splits into a fast tier {warm-container-minimum} and a slow tier {warm-container-native, warm-container-plusone, warm-container-headroom, warm-container-kernel} with a 80% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 3.7x its best-20%

warm-container-kernel's best 20% of batches run at 1.55 us but its worst 20% at 5.78 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2002.6 ns median (-49.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.46x (fastest 2002.6 ns, slowest 4933.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4835ns | 4036ns | 4028ns | 4037ns | 8037ns | base |
| warm-container-kernel | 4229ns | 5100ns | 1635ns | 4452ns | 6155ns | -12.54% |
| warm-container-minimum | 3664ns | 2085ns | 2064ns | 2803ns | 7847ns | -24.24% |
| warm-container-native | 4332ns | 3750ns | 2040ns | 4071ns | 7405ns | -10.42% |
| warm-container-plusone | 5229ns | 4034ns | 4023ns | 4035ns | 10018ns | +8.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4725ns | 3946ns | 7813ns | base | 5.201 |
| warm-container-kernel | 4010ns | 1552ns | 5783ns | -15.13% | 6.128 |
| warm-container-minimum | 3481ns | 1982ns | 7347ns | -26.34% | 7.061 |
| warm-container-native | 4144ns | 1957ns | 7043ns | -12.29% | 5.930 |
| warm-container-plusone | 5110ns | 3939ns | 9757ns | +8.15% | 4.809 |

## Performance model

- Peak throughput: **15.834 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.216 | 39.3% |
| warm-container-kernel | 4.981 | 31.5% |
| warm-container-minimum | 12.272 | 77.5% |
| warm-container-native | 6.831 | 43.1% |
| warm-container-plusone | 6.220 | 39.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4835ns | 4835ns | base |
| warm-container-kernel | 4229ns | 4229ns | -12.54% |
| warm-container-minimum | 3664ns | 3664ns | -24.24% |
| warm-container-native | 4332ns | 4332ns | -10.42% |
| warm-container-plusone | 5229ns | 5229ns | +8.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3954ns | base | --- | [3950, 3961] | --- | --- | --- | --- |
| warm-container-kernel | 4934ns | no significant difference | [-2300, +1116]ns | [2996, 5212] | no | 1.0000 | 1.0000 | 0 |
| warm-container-minimum | 2003ns | -1953.1ns (-49.4%) | [-1967, -1935]ns | [1994, 2038] | YES | 0.0027 | 0.0007 | 0 |
| warm-container-native | 3598ns | no significant difference | [-1985, +138]ns | [1973, 5981] | no | 0.3077 | 0.1539 | 0 |
| warm-container-plusone | 3951ns | no significant difference | [-7, +2]ns | [3947, 3956] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3956ns | +29.7% | -49.7% | -32.6% | -0.2% |
| 1 | 2 | 3954ns | +19.4% | -48.5% | +39.1% | -0.0% |
| 1 | 3 | 3955ns | +30.7% | -49.5% | -50.3% | -0.0% |
| 1 | 4 | 3955ns | -47.0% | -49.6% | +23.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.556 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.182 | ok |
| warm-container-minimum | -0.437 | moderate- |
| warm-container-native | -0.809 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.152 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 30.5ns | 4725.1ns | 0.6% |  |
| warm-container-kernel | 64.1ns | 4010.2ns | 1.6% |  |
| warm-container-minimum | 44.4ns | 3480.5ns | 1.3% |  |
| warm-container-native | 53.4ns | 4144.5ns | 1.3% |  |
| warm-container-plusone | 33.1ns | 5110.3ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3946.4-7812.8 ns)
   3946.4 |########################################
   4139.7 |
   4333.0 |
   4526.3 |
   4719.7 |
   4913.0 |
   5106.3 |
   5299.6 |
   5492.9 |
   5686.3 |
   5879.6 |
   6072.9 |
   6266.2 |
   6459.6 |
   6652.9 |
   6846.2 |
   7039.5 |#
   7232.8 |
   7426.2 |
   7619.5 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1552.1-5782.6 ns)
   1552.1 |##############################
   1763.7 |##########
   1975.2 |
   2186.7 |
   2398.2 |
   2609.8 |#####
   2821.3 |
   3032.8 |
   3244.3 |#####
   3455.8 |
   3667.4 |
   3878.9 |
   4090.4 |
   4301.9 |#####
   4513.4 |##########
   4725.0 |##########
   4936.5 |####################
   5148.0 |####################
   5359.5 |##########
   5571.0 |########################################
  (5 below, 2 above range)

warm-container-minimum (n=40, range 1981.8-7347.0 ns)
   1981.8 |########################################
   2250.1 |
   2518.3 |
   2786.6 |
   3054.9 |
   3323.1 |
   3591.4 |
   3859.6 |
   4127.9 |#
   4396.2 |
   4664.4 |#
   4932.7 |#
   5200.9 |
   5469.2 |#
   5737.5 |
   6005.7 |
   6274.0 |
   6542.2 |#
   6810.5 |
   7078.8 |##########
  (4 below, 2 above range)

warm-container-native (n=40, range 1957.0-7043.1 ns)
   1957.0 |########################################
   2211.3 |
   2465.6 |
   2719.9 |
   2974.2 |
   3228.5 |##
   3482.8 |
   3737.1 |##
   3991.4 |##
   4245.7 |
   4500.0 |
   4754.3 |
   5008.6 |##
   5262.9 |##
   5517.2 |
   5771.5 |########
   6025.8 |##
   6280.2 |#####
   6534.5 |########
   6788.8 |##
  (4 below, 6 above range)

warm-container-plusone (n=40, range 3939.5-9757.5 ns)
   3939.5 |########################################
   4230.4 |
   4521.3 |
   4812.2 |
   5103.1 |#
   5394.0 |
   5684.9 |
   5975.8 |
   6266.7 |
   6557.6 |
   6848.5 |
   7139.4 |
   7430.3 |
   7721.2 |
   8012.1 |
   8303.0 |
   8593.9 |#
   8884.8 |
   9175.7 |
   9466.6 |
  (3 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=46.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=43.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=64.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=53.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.7% (high variance, measurements may be unstable)
