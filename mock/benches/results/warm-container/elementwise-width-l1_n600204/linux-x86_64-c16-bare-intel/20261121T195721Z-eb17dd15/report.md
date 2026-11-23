# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native shows alternating (throttle bounce) (autocorr -0.53)

warm-container-native's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (96 ns) is smaller than the fastest variant's own run-to-run std-dev (2.26 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (2.27 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader warm-container-headroom vs stability leader warm-container-minimum (+2% speed for 1.1x steadier)

warm-container-headroom is fastest (2.27 us, CV 99.5%); warm-container-minimum gives up 2.0% median for 1.1x lower variance (CV 92.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-native is inconsistent: worst-20% is 4.3x its best-20%

warm-container-native's best 20% of batches run at 1.89 us but its worst 20% at 8.05 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.2% of the fastest

All 5 variants sit between 2.27 us and 2.37 us - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 2272.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 2272.2 ns, slowest 2367.9 ns)

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
| warm-container-headroom | 3754ns | 2340ns | 2265ns | 2824ns | 8031ns | base |
| warm-container-kernel | 3905ns | 2401ns | 1960ns | 3213ns | 7925ns | +4.02% |
| warm-container-minimum | 3505ns | 2399ns | 1954ns | 2682ns | 7524ns | -6.64% |
| warm-container-native | 4005ns | 2397ns | 1954ns | 3224ns | 8400ns | +6.70% |
| warm-container-plusone | 3814ns | 2449ns | 2021ns | 2832ns | 8554ns | +1.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3636ns | 2200ns | 7751ns | base | 11.264 |
| warm-container-kernel | 3762ns | 1893ns | 7610ns | +3.45% | 10.888 |
| warm-container-minimum | 3374ns | 1888ns | 7211ns | -7.23% | 12.141 |
| warm-container-native | 3856ns | 1888ns | 8051ns | +6.04% | 10.623 |
| warm-container-plusone | 3676ns | 1953ns | 8214ns | +1.09% | 11.143 |

## Performance model

- Peak throughput: **21.699 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 18.026 | 83.1% |
| warm-container-kernel | 17.672 | 81.4% |
| warm-container-minimum | 17.677 | 81.5% |
| warm-container-native | 17.680 | 81.5% |
| warm-container-plusone | 17.298 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3754ns | 3754ns | base |
| warm-container-kernel | 3905ns | 3905ns | +4.02% |
| warm-container-minimum | 3505ns | 3505ns | -6.64% |
| warm-container-native | 4005ns | 4005ns | +6.70% |
| warm-container-plusone | 3814ns | 3814ns | +1.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2272ns | base | --- | [2270, 2284] | --- | --- | --- | --- |
| warm-container-kernel | 2318ns | no significant difference | [-307, +47]ns | [2113, 3374] | no | 1.0000 | 1.0000 | 0 |
| warm-container-minimum | 2317ns | no significant difference | [-375, +48]ns | [2317, 2324] | no | 1.0000 | 0.8746 | 0 |
| warm-container-native | 2317ns | no significant difference | [-298, +38]ns | [2299, 3911] | no | 1.0000 | 0.6358 | 0 |
| warm-container-plusone | 2368ns | +96.9ns (+4.3%) | [+41, +104]ns | [2344, 2376] | YES (adj: no) | 0.0664 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2271ns | -16.6% | -16.3% | -16.4% | -16.2% |
| 1 | 2 | 2205ns | -13.9% | +5.1% | +37.5% | +7.4% |
| 1 | 3 | 2279ns | +2.0% | +1.7% | +2.0% | +1.8% |
| 1 | 4 | 2275ns | +2.5% | +23.0% | +1.4% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.389 | moderate- |
| warm-container-kernel | 0.257 | moderate+ |
| warm-container-minimum | -0.001 | ok |
| warm-container-native | -0.529 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.146 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 32.1ns | 3636.3ns | 0.9% |  |
| warm-container-kernel | 39.2ns | 3762.0ns | 1.0% |  |
| warm-container-minimum | 35.4ns | 3373.6ns | 1.0% |  |
| warm-container-native | 40.4ns | 3855.9ns | 1.0% |  |
| warm-container-plusone | 38.0ns | 3675.8ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2199.5-7751.2 ns)
   2199.5 |########################################
   2477.1 |
   2754.7 |
   3032.3 |
   3309.9 |
   3587.5 |
   3865.1 |#
   4142.6 |
   4420.2 |
   4697.8 |#
   4975.4 |
   5253.0 |
   5530.6 |#
   5808.1 |#
   6085.7 |
   6363.3 |
   6640.9 |#
   6918.5 |#
   7196.1 |#
   7473.6 |###
  (2 below, 3 above range)

warm-container-kernel (n=40, range 1893.4-7609.9 ns)
   1893.4 |##########################
   2179.2 |########################################
   2465.0 |
   2750.9 |
   3036.7 |
   3322.5 |
   3608.3 |
   3894.2 |
   4180.0 |###
   4465.8 |
   4751.6 |
   5037.4 |###
   5323.3 |
   5609.1 |
   5894.9 |######
   6180.7 |###
   6466.6 |
   6752.4 |###
   7038.2 |##########
   7324.0 |###
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1887.7-7210.8 ns)
   1887.7 |##################
   2153.8 |########################################
   2420.0 |
   2686.2 |
   2952.3 |
   3218.5 |
   3484.6 |##
   3750.8 |
   4016.9 |
   4283.1 |
   4549.2 |
   4815.4 |
   5081.6 |
   5347.7 |##
   5613.9 |##
   5880.0 |##
   6146.2 |##
   6412.3 |
   6678.5 |####
   6944.7 |##
  (4 below, 3 above range)

warm-container-native (n=40, range 1888.0-8051.3 ns)
   1888.0 |########################
   2196.2 |########################################
   2504.3 |
   2812.5 |
   3120.7 |
   3428.8 |###
   3737.0 |
   4045.2 |###
   4353.3 |
   4661.5 |
   4969.6 |###
   5277.8 |
   5586.0 |###
   5894.1 |######
   6202.3 |###
   6510.5 |###
   6818.6 |
   7126.8 |
   7435.0 |######
   7743.1 |###
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1952.9-8214.4 ns)
   1952.9 |
   2266.0 |########################################
   2579.1 |#
   2892.1 |
   3205.2 |
   3518.3 |
   3831.4 |
   4144.4 |
   4457.5 |#
   4770.6 |#
   5083.6 |
   5396.7 |
   5709.8 |
   6022.9 |
   6335.9 |#
   6649.0 |
   6962.1 |
   7275.2 |#
   7588.2 |#
   7901.3 |
  (7 below, 6 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-kernel**: CV=62.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=63.3% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=64.1% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=66.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.2x (possible bimodal distribution)
