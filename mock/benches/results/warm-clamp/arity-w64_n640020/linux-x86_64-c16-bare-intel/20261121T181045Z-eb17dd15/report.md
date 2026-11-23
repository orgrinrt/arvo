# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-acc64, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-acc64 (2.32 us) and warm-clamp-min-lanes (2.34 us) differ by 0.77%, inside the noise, even though the wider field spreads 14.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.77)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (341 ns) is smaller than the fastest variant's own run-to-run std-dev (2.15 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (2.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.38 us but its worst 20% at 6.23 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 2318.3 ns median
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.15x (fastest 2318.3 ns, slowest 2659.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112579971 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3733ns | 2413ns | 2066ns | 2981ns | 7654ns | base |
| warm-clamp-accfit | 3755ns | 2775ns | 1602ns | 3468ns | 6768ns | +0.59% |
| warm-clamp-accfit-dyn | 3594ns | 2568ns | 1447ns | 3327ns | 6541ns | -3.73% |
| warm-clamp-head | 3933ns | 2583ns | 2182ns | 3176ns | 7957ns | +5.37% |
| warm-clamp-min-lanes | 3853ns | 2429ns | 2166ns | 3137ns | 7690ns | +3.22% |
| warm-clamp-minimum | 4072ns | 2655ns | 2157ns | 3299ns | 8305ns | +9.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3577ns | 1984ns | 7302ns | base | 2.290 |
| warm-clamp-accfit | 3586ns | 1535ns | 6440ns | +0.27% | 2.284 |
| warm-clamp-accfit-dyn | 3430ns | 1380ns | 6229ns | -4.09% | 2.388 |
| warm-clamp-head | 3810ns | 2113ns | 7685ns | +6.52% | 2.150 |
| warm-clamp-min-lanes | 3691ns | 2082ns | 7326ns | +3.19% | 2.219 |
| warm-clamp-minimum | 3919ns | 2081ns | 7962ns | +9.56% | 2.091 |

## Performance model

- Peak throughput: **5.938 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.534 | 59.5% |
| warm-clamp-accfit | 3.080 | 51.9% |
| warm-clamp-accfit-dyn | 3.350 | 56.4% |
| warm-clamp-head | 3.257 | 54.8% |
| warm-clamp-min-lanes | 3.506 | 59.1% |
| warm-clamp-minimum | 3.197 | 53.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3733ns | 3733ns | base |
| warm-clamp-accfit | 3755ns | 3755ns | +0.59% |
| warm-clamp-accfit-dyn | 3594ns | 3594ns | -3.73% |
| warm-clamp-head | 3933ns | 3933ns | +5.37% |
| warm-clamp-min-lanes | 3853ns | 3853ns | +3.22% |
| warm-clamp-minimum | 4072ns | 4072ns | +9.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2318ns | base | --- | [2297, 2334] | --- | --- | --- | --- |
| warm-clamp-accfit | 2660ns | -418.3ns (-18.0%) | [-509, -377]ns | [1876, 4906] | YES | 0.0056 | 0.0022 | 0 |
| warm-clamp-accfit-dyn | 2445ns | -399.2ns (-17.2%) | [-768, -9]ns | [1874, 4641] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-head | 2516ns | no significant difference | [-170, +202]ns | [2505, 2837] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-min-lanes | 2336ns | no significant difference | [-9, +20]ns | [2312, 3390] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-minimum | 2562ns | +245.1ns (+10.6%) | [+217, +278]ns | [2558, 2923] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2321ns | -18.6% | -42.1% | +8.4% | +0.7% | +10.4% |
| 1 | 2 | 2308ns | +81.2% | +92.3% | +47.2% | +93.9% | +11.9% |
| 1 | 3 | 2290ns | -17.8% | +60.7% | -7.4% | +0.3% | +12.1% |
| 1 | 4 | 1891ns | +110.9% | -0.3% | +33.1% | +22.6% | +28.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.043 | ok |
| warm-clamp-accfit | -0.770 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.287 | moderate- |
| warm-clamp-head | -0.483 | moderate- |
| warm-clamp-min-lanes | -0.414 | moderate- |
| warm-clamp-minimum | -0.035 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 38.6ns | 3576.8ns | 1.1% |  |
| warm-clamp-accfit | 47.9ns | 3586.4ns | 1.3% |  |
| warm-clamp-accfit-dyn | 46.8ns | 3430.4ns | 1.4% |  |
| warm-clamp-head | 33.5ns | 3810.1ns | 0.9% |  |
| warm-clamp-min-lanes | 40.4ns | 3691.0ns | 1.1% |  |
| warm-clamp-minimum | 38.2ns | 3918.6ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1984.0-7302.3 ns)
   1984.0 |
   2249.9 |########################################
   2515.8 |
   2781.7 |
   3047.6 |#
   3313.5 |
   3579.4 |
   3845.4 |
   4111.3 |
   4377.2 |#
   4643.1 |#
   4909.0 |
   5174.9 |
   5440.9 |
   5706.8 |
   5972.7 |#
   6238.6 |###
   6504.5 |#
   6770.4 |###
   7036.4 |###
  (6 below, 2 above range)

warm-clamp-accfit (n=40, range 1535.3-6439.8 ns)
   1535.3 |######
   1780.5 |########################################
   2025.8 |
   2271.0 |
   2516.2 |
   2761.4 |
   3006.7 |
   3251.9 |###
   3497.1 |###
   3742.3 |
   3987.6 |
   4232.8 |
   4478.0 |######
   4723.2 |##########
   4968.5 |######
   5213.7 |##########
   5458.9 |###
   5704.1 |###
   5949.4 |###
   6194.6 |
  (6 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1379.7-6228.9 ns)
   1379.7 |############
   1622.1 |
   1864.6 |########################################
   2107.1 |####
   2349.5 |
   2592.0 |####
   2834.4 |####
   3076.9 |
   3319.4 |
   3561.8 |
   3804.3 |
   4046.8 |####
   4289.2 |########
   4531.7 |########
   4774.1 |########
   5016.6 |########
   5259.1 |####
   5501.5 |####
   5744.0 |####
   5986.5 |########
  (6 below, 4 above range)

warm-clamp-head (n=40, range 2113.1-7684.8 ns)
   2113.1 |##################
   2391.7 |########################################
   2670.2 |
   2948.8 |##
   3227.4 |
   3506.0 |
   3784.6 |
   4063.2 |##
   4341.8 |
   4620.3 |##
   4898.9 |
   5177.5 |
   5456.1 |
   5734.7 |#####
   6013.3 |##
   6291.8 |
   6570.4 |#####
   6849.0 |##
   7127.6 |##
   7406.2 |##
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2081.8-7326.4 ns)
   2081.8 |########################################
   2344.0 |##
   2606.3 |
   2868.5 |##
   3130.7 |
   3393.0 |##
   3655.2 |##
   3917.4 |
   4179.6 |
   4441.9 |
   4704.1 |
   4966.3 |
   5228.5 |##
   5490.8 |##
   5753.0 |######
   6015.2 |
   6277.5 |##
   6539.7 |##
   6801.9 |##
   7064.1 |
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 2080.9-7962.2 ns)
   2080.9 |################
   2375.0 |########################################
   2669.1 |##
   2963.1 |##
   3257.2 |##
   3551.3 |
   3845.3 |
   4139.4 |
   4433.4 |
   4727.5 |
   5021.6 |
   5315.6 |
   5609.7 |
   5903.7 |########
   6197.8 |##
   6491.9 |#####
   6785.9 |#####
   7080.0 |
   7374.0 |##
   7668.1 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=60.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=55.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=56.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=58.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=57.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=61.2% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
