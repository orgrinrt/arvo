# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn dominates: 21% faster than the next best (warm-clamp-min-lanes)

warm-clamp-accfit-dyn (2.20 us) leads warm-clamp-min-lanes (2.67 us) by 21%, a clear separation rather than a photo finish. CV 103.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn beats baseline by 37% (significant)

warm-clamp-accfit-dyn is -1.33 us (37%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 103.6%)

warm-clamp-accfit-dyn wins on median (2.20 us) yet has the highest variance (CV 103.6%), while warm-clamp-acc64 is the steadiest (CV 69.9%, 3.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.53)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.41 us) is smaller than the fastest variant's own run-to-run std-dev (2.28 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 2.18 us but its worst 20% at 8.66 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2200.6 ns median (-38.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.64x (fastest 2200.6 ns, slowest 3605.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5345ns | 3684ns | 3078ns | 4660ns | 9665ns | base |
| warm-clamp-accfit | 4328ns | 3130ns | 2105ns | 3899ns | 7841ns | -19.02% |
| warm-clamp-accfit-dyn | 3899ns | 2288ns | 2246ns | 3118ns | 7897ns | -27.04% |
| warm-clamp-head | 4886ns | 3174ns | 3164ns | 3582ns | 10520ns | -8.58% |
| warm-clamp-min-lanes | 3947ns | 2753ns | 2255ns | 2828ns | 8996ns | -26.15% |
| warm-clamp-minimum | 5531ns | 3698ns | 2912ns | 4596ns | 10956ns | +3.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 5207ns | 3004ns | 9384ns | base | 1.573 |
| warm-clamp-accfit | 4145ns | 2025ns | 7480ns | -20.39% | 1.976 |
| warm-clamp-accfit-dyn | 3740ns | 2160ns | 7544ns | -28.18% | 2.190 |
| warm-clamp-head | 4741ns | 3077ns | 10172ns | -8.96% | 1.728 |
| warm-clamp-min-lanes | 3814ns | 2185ns | 8664ns | -26.76% | 2.148 |
| warm-clamp-minimum | 5389ns | 2842ns | 10639ns | +3.49% | 1.520 |

## Performance model

- Peak throughput: **4.045 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.277 | 56.3% |
| warm-clamp-accfit | 2.741 | 67.8% |
| warm-clamp-accfit-dyn | 3.723 | 92.0% |
| warm-clamp-head | 2.654 | 65.6% |
| warm-clamp-min-lanes | 3.072 | 75.9% |
| warm-clamp-minimum | 2.272 | 56.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 5345ns | 5345ns | base |
| warm-clamp-accfit | 4328ns | 4328ns | -19.02% |
| warm-clamp-accfit-dyn | 3899ns | 3899ns | -27.04% |
| warm-clamp-head | 4886ns | 4886ns | -8.58% |
| warm-clamp-min-lanes | 3947ns | 3947ns | -26.15% |
| warm-clamp-minimum | 5531ns | 5531ns | +3.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3597ns | base | --- | [3558, 5489] | --- | --- | --- | --- |
| warm-clamp-accfit | 2988ns | -994.8ns (-27.7%) | [-1331, -608]ns | [2972, 5074] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2201ns | -1332.2ns (-37.0%) | [-1447, -922]ns | [2188, 2505] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 3087ns | -472.0ns (-13.1%) | [-517, -426]ns | [3082, 3094] | YES | 0.0207 | 0.0166 | 0 |
| warm-clamp-min-lanes | 2667ns | -1302.5ns (-36.2%) | [-1363, -919]ns | [2660, 2688] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 3606ns | no significant difference | [-685, +689]ns | [3535, 4542] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2841ns | +69.2% | -22.7% | +8.6% | -2.4% | +24.3% |
| 1 | 2 | 6080ns | -4.5% | -63.9% | -49.2% | -55.9% | -40.6% |
| 1 | 3 | 3523ns | -37.9% | +19.5% | -12.4% | -37.8% | -19.4% |
| 1 | 4 | 3596ns | -17.1% | -40.0% | -14.0% | -25.8% | +139.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.533 | HIGH- (thermal bounce) |
| warm-clamp-accfit | 0.003 | ok |
| warm-clamp-accfit-dyn | -0.427 | moderate- |
| warm-clamp-head | -0.258 | moderate- |
| warm-clamp-min-lanes | -0.264 | moderate- |
| warm-clamp-minimum | -0.194 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 34.9ns | 5207.2ns | 0.7% |  |
| warm-clamp-accfit | 47.3ns | 4145.5ns | 1.1% |  |
| warm-clamp-accfit-dyn | 40.1ns | 3739.9ns | 1.1% |  |
| warm-clamp-head | 36.4ns | 4740.6ns | 0.8% |  |
| warm-clamp-min-lanes | 34.4ns | 3813.9ns | 0.9% |  |
| warm-clamp-minimum | 36.1ns | 5389.1ns | 0.7% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 3004.2-9384.0 ns)
   3004.2 |
   3323.2 |########################################
   3642.2 |
   3961.2 |#####
   4280.2 |##
   4599.1 |##
   4918.1 |
   5237.1 |##
   5556.1 |##
   5875.1 |
   6194.1 |##
   6513.1 |#####
   6832.1 |##
   7151.1 |##
   7470.1 |##
   7789.1 |##
   8108.1 |##
   8427.1 |##
   8746.1 |
   9065.0 |##
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 2025.3-7480.1 ns)
   2025.3 |########################################
   2298.0 |
   2570.7 |
   2843.5 |####################################
   3116.2 |
   3389.0 |####
   3661.7 |
   3934.4 |
   4207.2 |
   4479.9 |
   4752.7 |############
   5025.4 |####
   5298.1 |########
   5570.9 |####
   5843.6 |########
   6116.4 |
   6389.1 |########
   6661.9 |
   6934.6 |####
   7207.3 |
  (3 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 2160.4-7544.5 ns)
   2160.4 |########################################
   2429.6 |#
   2698.8 |
   2968.0 |
   3237.2 |
   3506.4 |
   3775.6 |
   4044.8 |
   4314.0 |#
   4583.2 |
   4852.4 |
   5121.6 |
   5390.8 |
   5660.0 |
   5929.2 |###
   6198.4 |#
   6467.6 |#
   6736.8 |#
   7006.1 |
   7275.3 |#####
  (5 below, 4 above range)

warm-clamp-head (n=40, range 3077.4-10171.5 ns)
   3077.4 |########################################
   3432.1 |#
   3786.8 |
   4141.5 |
   4496.2 |
   4850.9 |
   5205.6 |
   5560.4 |
   5915.1 |
   6269.8 |#
   6624.5 |
   6979.2 |
   7333.9 |
   7688.6 |
   8043.3 |
   8398.0 |
   8752.7 |#
   9107.4 |#
   9462.1 |###
   9816.8 |#
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2185.0-8663.8 ns)
   2185.0 |######
   2508.9 |########################################
   2832.9 |
   3156.8 |
   3480.7 |
   3804.7 |#
   4128.6 |
   4452.6 |
   4776.5 |
   5100.4 |
   5424.4 |
   5748.3 |
   6072.3 |
   6396.2 |
   6720.1 |
   7044.1 |#
   7368.0 |
   7692.0 |
   8015.9 |###
   8339.8 |#
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 2841.6-10639.0 ns)
   2841.6 |########
   3231.5 |########################################
   3621.4 |########
   4011.2 |##
   4401.1 |
   4791.0 |#####
   5180.8 |
   5570.7 |
   5960.6 |
   6350.4 |
   6740.3 |##
   7130.2 |
   7520.0 |
   7909.9 |
   8299.8 |###########
   8689.6 |
   9079.5 |##
   9469.4 |
   9859.2 |##
  10249.1 |##
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=48.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=50.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=61.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=61.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=64.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=56.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
