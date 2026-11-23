# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 12% faster than the next best (warm-clamp-minimum)

warm-clamp-min-lanes (2.45 us) leads warm-clamp-minimum (2.75 us) by 12%, a clear separation rather than a photo finish. CV 92.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 32% (significant)

warm-clamp-min-lanes is -999 ns (32%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.74)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (652 ns) is smaller than the fastest variant's own run-to-run std-dev (2.26 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-minimum is inconsistent: worst-20% is 4.4x its best-20%

warm-clamp-minimum's best 20% of batches run at 2.47 us but its worst 20% at 10.96 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2452.6 ns median (-20.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.27x (fastest 2452.6 ns, slowest 3104.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891686 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4283ns | 3181ns | 3090ns | 3170ns | 8816ns | base |
| warm-clamp-accfit | 4801ns | 3187ns | 2552ns | 4197ns | 8860ns | +12.08% |
| warm-clamp-accfit-dyn | 4328ns | 3093ns | 2593ns | 3202ns | 9440ns | +1.04% |
| warm-clamp-head | 4140ns | 3142ns | 2567ns | 3051ns | 8982ns | -3.33% |
| warm-clamp-min-lanes | 3781ns | 2534ns | 2073ns | 3031ns | 7737ns | -11.72% |
| warm-clamp-minimum | 4801ns | 2820ns | 2538ns | 3390ns | 11296ns | +12.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 4161ns | 3009ns | 8534ns | base | 1.969 |
| warm-clamp-accfit | 4664ns | 2486ns | 8574ns | +12.09% | 1.756 |
| warm-clamp-accfit-dyn | 4204ns | 2526ns | 9137ns | +1.03% | 1.949 |
| warm-clamp-head | 4017ns | 2488ns | 8684ns | -3.47% | 2.039 |
| warm-clamp-min-lanes | 3649ns | 2007ns | 7438ns | -12.31% | 2.245 |
| warm-clamp-minimum | 4667ns | 2471ns | 10956ns | +12.15% | 1.755 |

## Performance model

- Peak throughput: **4.082 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.643 | 64.7% |
| warm-clamp-accfit | 2.639 | 64.6% |
| warm-clamp-accfit-dyn | 2.721 | 66.7% |
| warm-clamp-head | 2.677 | 65.6% |
| warm-clamp-min-lanes | 3.340 | 81.8% |
| warm-clamp-minimum | 2.981 | 73.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4283ns | 4283ns | base |
| warm-clamp-accfit | 4801ns | 4801ns | +12.08% |
| warm-clamp-accfit-dyn | 4328ns | 4328ns | +1.04% |
| warm-clamp-head | 4140ns | 4140ns | -3.33% |
| warm-clamp-min-lanes | 3781ns | 3781ns | -11.72% |
| warm-clamp-minimum | 4801ns | 4801ns | +12.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3100ns | base | --- | [3025, 3109] | --- | --- | --- | --- |
| warm-clamp-accfit | 3104ns | no significant difference | [-469, +91]ns | [2815, 5634] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 3011ns | -166.7ns (-5.4%) | [-491, -10]ns | [2828, 3090] | YES | 0.0005 | 0.0002 | 0 |
| warm-clamp-head | 3060ns | -53.6ns (-1.7%) | [-527, -33]ns | [2519, 3070] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-min-lanes | 2453ns | -999.4ns (-32.2%) | [-1006, -654]ns | [2014, 2883] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-minimum | 2748ns | -494.3ns (-15.9%) | [-553, -82]ns | [2527, 3105] | YES | 0.0276 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3015ns | -15.7% | -0.2% | +1.9% | -33.4% | -16.0% |
| 1 | 2 | 3106ns | +123.7% | -18.4% | -19.1% | +38.5% | -0.0% |
| 1 | 3 | 3018ns | +2.7% | -16.0% | -17.5% | -33.3% | -18.3% |
| 1 | 4 | 3107ns | -20.3% | -0.3% | -1.5% | -21.1% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.737 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.376 | moderate- |
| warm-clamp-accfit-dyn | -0.246 | moderate- |
| warm-clamp-head | -0.250 | moderate- |
| warm-clamp-min-lanes | -0.568 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.445 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 32.6ns | 4161.2ns | 0.8% |  |
| warm-clamp-accfit | 36.3ns | 4664.3ns | 0.8% |  |
| warm-clamp-accfit-dyn | 34.2ns | 4204.1ns | 0.8% |  |
| warm-clamp-head | 33.9ns | 4016.8ns | 0.8% |  |
| warm-clamp-min-lanes | 37.3ns | 3649.1ns | 1.0% |  |
| warm-clamp-minimum | 36.3ns | 4666.6ns | 0.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 3008.7-8533.9 ns)
   3008.7 |########################################
   3285.0 |
   3561.2 |
   3837.5 |#
   4113.7 |
   4390.0 |
   4666.3 |
   4942.5 |
   5218.8 |
   5495.0 |
   5771.3 |
   6047.5 |#
   6323.8 |
   6600.1 |
   6876.3 |
   7152.6 |#
   7428.8 |
   7705.1 |
   7981.4 |
   8257.6 |
  (5 below, 5 above range)

warm-clamp-accfit (n=40, range 2486.0-8573.7 ns)
   2486.0 |########################################
   2790.4 |####################
   3094.8 |####################
   3399.2 |#####
   3703.6 |
   4007.9 |
   4312.3 |
   4616.7 |#####
   4921.1 |
   5225.5 |#####
   5529.9 |###############
   5834.3 |
   6138.6 |
   6443.0 |#####
   6747.4 |#####
   7051.8 |###############
   7356.2 |#####
   7660.6 |##########
   7965.0 |
   8269.4 |#####
  (6 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2526.3-9137.4 ns)
   2526.3 |#########################
   2856.9 |########################################
   3187.4 |##
   3518.0 |
   3848.6 |
   4179.1 |
   4509.7 |
   4840.2 |
   5170.8 |
   5501.3 |
   5831.9 |
   6162.4 |
   6493.0 |
   6823.6 |
   7154.1 |#####
   7484.7 |
   7815.2 |
   8145.8 |##
   8476.3 |##
   8806.9 |##
  (4 below, 4 above range)

warm-clamp-head (n=40, range 2488.1-8683.6 ns)
   2488.1 |#########################
   2797.8 |########################################
   3107.6 |
   3417.4 |
   3727.2 |
   4037.0 |
   4346.7 |
   4656.5 |##
   4966.3 |
   5276.1 |
   5585.8 |##
   5895.6 |
   6205.4 |
   6515.2 |
   6825.0 |
   7134.7 |##
   7444.5 |
   7754.3 |##
   8064.1 |
   8373.8 |##
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2006.9-7437.8 ns)
   2006.9 |########################################
   2278.4 |#####################
   2550.0 |
   2821.5 |##
   3093.1 |##
   3364.6 |
   3636.2 |
   3907.7 |
   4179.2 |
   4450.8 |
   4722.3 |
   4993.9 |
   5265.4 |
   5537.0 |##
   5808.5 |##
   6080.0 |##
   6351.6 |#####
   6623.1 |##
   6894.7 |#####
   7166.2 |##
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 2471.0-10955.8 ns)
   2471.0 |########################################
   2895.2 |######################
   3319.5 |
   3743.7 |
   4167.9 |##
   4592.2 |
   5016.4 |
   5440.7 |##
   5864.9 |
   6289.2 |##
   6713.4 |
   7137.6 |
   7561.9 |########
   7986.1 |########
   8410.4 |##
   8834.6 |
   9258.9 |
   9683.1 |
  10107.3 |
  10531.6 |
  (6 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=58.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=52.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=62.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-head**: CV=61.9% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=61.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=95.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.4x (possible bimodal distribution)
