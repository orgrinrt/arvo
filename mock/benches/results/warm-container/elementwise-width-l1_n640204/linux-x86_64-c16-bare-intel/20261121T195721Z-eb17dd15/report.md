# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 58% (significant)

warm-container-kernel is -3.04 us (58%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.7x slower than the field

warm-container-plusone (5.26 us) is 2.7x the fastest (1.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 117.8%)

warm-container-kernel wins on median (1.97 us) yet has the highest variance (CV 117.8%), while warm-container-plusone is the steadiest (CV 19.3%, 5.26 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.59)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (155% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 155% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 4.4x its best-20%

warm-container-kernel's best 20% of batches run at 1.61 us but its worst 20% at 7.16 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1967.9 ns median (-62.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.67x (fastest 1967.9 ns, slowest 5257.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112697097 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5329ns | 5333ns | 3873ns | 5090ns | 7502ns | base |
| warm-container-kernel | 3596ns | 2049ns | 1676ns | 2933ns | 7506ns | -32.52% |
| warm-container-minimum | 3781ns | 2140ns | 1785ns | 3368ns | 7015ns | -29.05% |
| warm-container-native | 3645ns | 2120ns | 1771ns | 3061ns | 7273ns | -31.59% |
| warm-container-plusone | 5169ns | 5338ns | 4375ns | 5037ns | 6362ns | -2.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5234ns | 3805ns | 7347ns | base | 7.825 |
| warm-container-kernel | 3444ns | 1610ns | 7156ns | -34.21% | 11.894 |
| warm-container-minimum | 3618ns | 1713ns | 6675ns | -30.88% | 11.321 |
| warm-container-native | 3489ns | 1701ns | 6926ns | -33.34% | 11.739 |
| warm-container-plusone | 5081ns | 4306ns | 6231ns | -2.94% | 8.062 |

## Performance model

- Peak throughput: **25.444 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.801 | 30.7% |
| warm-container-kernel | 20.814 | 81.8% |
| warm-container-minimum | 19.905 | 78.2% |
| warm-container-native | 20.107 | 79.0% |
| warm-container-plusone | 7.791 | 30.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5329ns | 5329ns | base |
| warm-container-kernel | 3596ns | 3596ns | -32.52% |
| warm-container-minimum | 3781ns | 3781ns | -29.05% |
| warm-container-native | 3645ns | 3645ns | -31.59% |
| warm-container-plusone | 5169ns | 5169ns | -2.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5250ns | base | --- | [4654, 5261] | --- | --- | --- | --- |
| warm-container-kernel | 1968ns | -3038.0ns (-57.9%) | [-3252, -1825]ns | [1793, 3413] | YES | 0.0030 | 0.0022 | 0 |
| warm-container-minimum | 2058ns | -2677.8ns (-51.0%) | [-2717, -1283]ns | [1978, 5014] | YES | 0.0030 | 0.0022 | 0 |
| warm-container-native | 2037ns | -3030.5ns (-57.7%) | [-3214, -1834]ns | [1968, 3172] | YES | 0.0030 | 0.0022 | 0 |
| warm-container-plusone | 5257ns | no significant difference | [-4, +612]ns | [4649, 5260] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 5266ns | -69.3% | -56.4% | -61.3% | -18.3% |
| 1 | 2 | 4649ns | -65.4% | -57.4% | -65.2% | +13.4% |
| 1 | 3 | 3804ns | -27.9% | +16.1% | -42.6% | +22.4% |
| 1 | 4 | 5263ns | -62.5% | -68.5% | -53.8% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.311 | moderate- |
| warm-container-kernel | -0.179 | ok |
| warm-container-minimum | -0.565 | HIGH- (thermal bounce) |
| warm-container-native | 0.009 | ok |
| warm-container-plusone | -0.585 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.6ns | 5234.5ns | 0.5% |  |
| warm-container-kernel | 42.1ns | 3443.7ns | 1.2% |  |
| warm-container-minimum | 43.7ns | 3617.9ns | 1.2% |  |
| warm-container-native | 42.3ns | 3489.1ns | 1.2% |  |
| warm-container-plusone | 24.2ns | 5080.6ns | 0.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3805.3-7347.1 ns)
   3805.3 |####
   3982.4 |
   4159.5 |
   4336.6 |
   4513.7 |######################
   4690.8 |
   4867.9 |
   5045.0 |
   5222.0 |########################################
   5399.1 |
   5576.2 |
   5753.3 |
   5930.4 |
   6107.5 |
   6284.6 |
   6461.7 |
   6638.8 |
   6815.8 |
   6992.9 |
   7170.0 |
  (6 below, 4 above range)

warm-container-kernel (n=40, range 1609.8-7156.4 ns)
   1609.8 |########################################
   1887.1 |####################################
   2164.5 |
   2441.8 |
   2719.1 |
   2996.4 |
   3273.8 |######
   3551.1 |
   3828.4 |
   4105.8 |
   4383.1 |
   4660.4 |###
   4937.8 |
   5215.1 |
   5492.4 |###
   5769.7 |
   6047.1 |###
   6324.4 |##########
   6601.7 |######
   6879.1 |######
  (2 below, 3 above range)

warm-container-minimum (n=40, range 1712.8-6675.0 ns)
   1712.8 |
   1960.9 |########################################
   2209.1 |
   2457.2 |##
   2705.3 |
   2953.4 |##
   3201.5 |##
   3449.6 |##
   3697.7 |
   3945.8 |
   4193.9 |
   4442.0 |
   4690.2 |##
   4938.3 |##
   5186.4 |
   5434.5 |##
   5682.6 |########
   5930.7 |#####
   6178.8 |##
   6426.9 |########
  (6 below, 3 above range)

warm-container-native (n=40, range 1701.0-6926.2 ns)
   1701.0 |
   1962.2 |########################################
   2223.5 |##
   2484.8 |
   2746.0 |##
   3007.3 |
   3268.5 |##
   3529.8 |
   3791.1 |
   4052.3 |
   4313.6 |
   4574.9 |
   4836.1 |
   5097.4 |
   5358.6 |##
   5619.9 |
   5881.2 |####
   6142.4 |####
   6403.7 |########
   6665.0 |##
  (6 below, 3 above range)

warm-container-plusone (n=40, range 4306.3-6231.0 ns)
   4306.3 |################
   4402.6 |
   4498.8 |
   4595.0 |####################################
   4691.3 |
   4787.5 |
   4883.7 |
   4980.0 |
   5076.2 |
   5172.4 |########################################
   5268.6 |####################################
   5364.9 |
   5461.1 |
   5557.3 |
   5653.6 |
   5749.8 |
   5846.0 |
   5942.3 |
   6038.5 |
   6134.7 |
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=29.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=67.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=56.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=62.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.1x (possible bimodal distribution)
