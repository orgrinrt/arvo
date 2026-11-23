# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit at 1.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit dominates: 22% faster than the next best (warm-clamp-minimum)

warm-clamp-accfit (1.93 us) leads warm-clamp-minimum (2.34 us) by 22%, a clear separation rather than a photo finish. CV 107.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit is fastest but the noisiest (CV 107.5%)

warm-clamp-accfit wins on median (1.93 us) yet has the highest variance (CV 107.5%), while warm-clamp-head is the steadiest (CV 35.2%, 3.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.63)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.67 us) is smaller than the fastest variant's own run-to-run std-dev (2.07 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-minimum is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.91 us but its worst 20% at 7.87 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 1926.7 ns median (-46.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.87x (fastest 1926.7 ns, slowest 3593.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761915 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3521ns | 3757ns | 1421ns | 3625ns | 5312ns | base |
| warm-clamp-accfit | 3832ns | 1978ns | 1961ns | 3463ns | 6809ns | +8.81% |
| warm-clamp-accfit-dyn | 3914ns | 2412ns | 1964ns | 3487ns | 7146ns | +11.16% |
| warm-clamp-head | 3306ns | 3500ns | 1180ns | 3519ns | 4796ns | -6.11% |
| warm-clamp-min-lanes | 4114ns | 2441ns | 2155ns | 3487ns | 7954ns | +16.83% |
| warm-clamp-minimum | 4347ns | 2408ns | 1963ns | 3880ns | 8133ns | +23.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3353ns | 1358ns | 5038ns | base | 2.443 |
| warm-clamp-accfit | 3721ns | 1910ns | 6581ns | +10.97% | 2.201 |
| warm-clamp-accfit-dyn | 3802ns | 1912ns | 6910ns | +13.37% | 2.155 |
| warm-clamp-head | 3124ns | 1120ns | 4507ns | -6.85% | 2.622 |
| warm-clamp-min-lanes | 3997ns | 2099ns | 7706ns | +19.19% | 2.050 |
| warm-clamp-minimum | 4223ns | 1911ns | 7873ns | +25.94% | 1.940 |

## Performance model

- Peak throughput: **7.312 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.280 | 31.2% |
| warm-clamp-accfit | 4.252 | 58.2% |
| warm-clamp-accfit-dyn | 3.488 | 47.7% |
| warm-clamp-head | 2.465 | 33.7% |
| warm-clamp-min-lanes | 3.444 | 47.1% |
| warm-clamp-minimum | 3.494 | 47.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3521ns | 3521ns | base |
| warm-clamp-accfit | 3832ns | 3832ns | +8.81% |
| warm-clamp-accfit-dyn | 3914ns | 3914ns | +11.16% |
| warm-clamp-head | 3306ns | 3306ns | -6.11% |
| warm-clamp-min-lanes | 4114ns | 4114ns | +16.83% |
| warm-clamp-minimum | 4347ns | 4347ns | +23.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3593ns | base | --- | [3278, 4074] | --- | --- | --- | --- |
| warm-clamp-accfit | 1927ns | +556.5ns (+15.5%) | [+346, +1170]ns | [1914, 5718] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 2349ns | +563.2ns (+15.7%) | [+463, +1098]ns | [1924, 5501] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-head | 3323ns | -207.5ns (-5.8%) | [-486, -118]ns | [3048, 3631] | YES | 0.0111 | 0.0022 | 0 |
| warm-clamp-min-lanes | 2378ns | no significant difference | [-109, +981]ns | [2366, 4639] | no | 0.0807 | 0.0807 | 0 |
| warm-clamp-minimum | 2345ns | +949.9ns (+26.4%) | [+395, +1977]ns | [2334, 5442] | YES | 0.0415 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3423ns | +1.8% | -31.0% | -4.4% | -30.5% | +54.2% |
| 1 | 2 | 2913ns | -33.9% | -8.9% | +27.5% | +17.5% | -34.2% |
| 1 | 3 | 3748ns | -22.0% | -6.1% | -6.5% | -36.7% | -37.4% |
| 1 | 4 | 4354ns | -45.0% | -55.9% | -21.1% | -45.5% | -46.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.125 | ok |
| warm-clamp-accfit | -0.632 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.440 | moderate- |
| warm-clamp-head | -0.439 | moderate- |
| warm-clamp-min-lanes | -0.414 | moderate- |
| warm-clamp-minimum | -0.191 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 58.7ns | 3353.3ns | 1.8% |  |
| warm-clamp-accfit | 36.7ns | 3721.1ns | 1.0% |  |
| warm-clamp-accfit-dyn | 37.3ns | 3801.8ns | 1.0% |  |
| warm-clamp-head | 64.2ns | 3123.8ns | 2.1% |  |
| warm-clamp-min-lanes | 39.6ns | 3996.8ns | 1.0% |  |
| warm-clamp-minimum | 41.4ns | 4223.3ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1357.8-5038.5 ns)
   1357.8 |################################
   1541.9 |
   1725.9 |
   1909.9 |########
   2094.0 |
   2278.0 |########
   2462.0 |
   2646.1 |
   2830.1 |
   3014.1 |########
   3198.2 |########################
   3382.2 |########################
   3566.2 |################################
   3750.3 |################
   3934.3 |################
   4118.3 |########
   4302.3 |########
   4486.4 |################################
   4670.4 |
   4854.4 |########################################
  (6 below, 2 above range)

warm-clamp-accfit (n=40, range 1909.8-6581.1 ns)
   1909.8 |########################################
   2143.3 |
   2376.9 |
   2610.5 |##
   2844.0 |##
   3077.6 |
   3311.2 |
   3544.7 |
   3778.3 |##
   4011.9 |
   4245.4 |
   4479.0 |
   4712.6 |
   4946.1 |##
   5179.7 |
   5413.3 |
   5646.8 |#######
   5880.4 |##########
   6114.0 |#####
   6347.5 |#######
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1912.3-6910.3 ns)
   1912.3 |########################################
   2162.2 |#####################
   2412.1 |###
   2662.0 |
   2911.9 |
   3161.8 |###
   3411.7 |
   3661.6 |
   3911.5 |
   4161.4 |
   4411.3 |
   4661.2 |
   4911.1 |###
   5161.0 |###
   5410.9 |###
   5660.8 |###
   5910.7 |##################
   6160.6 |
   6410.5 |#######
   6660.4 |###
  (5 below, 4 above range)

warm-clamp-head (n=40, range 1120.4-4506.6 ns)
   1120.4 |########################################
   1289.7 |
   1459.0 |
   1628.4 |########
   1797.7 |
   1967.0 |
   2136.3 |
   2305.6 |
   2474.9 |
   2644.2 |########
   2813.5 |########
   2982.8 |################################
   3152.1 |################################
   3321.5 |################
   3490.8 |########################################
   3660.1 |################
   3829.4 |########################
   3998.7 |########
   4168.0 |################################
   4337.3 |
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2099.1-7706.3 ns)
   2099.1 |########################################
   2379.5 |#######
   2659.8 |
   2940.2 |##
   3220.6 |
   3500.9 |
   3781.3 |
   4061.6 |
   4342.0 |##
   4622.3 |##
   4902.7 |
   5183.1 |
   5463.4 |##
   5743.8 |##
   6024.1 |##
   6304.5 |##
   6584.8 |##
   6865.2 |#####
   7145.6 |##
   7425.9 |##
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 1911.0-7872.7 ns)
   1911.0 |####################
   2209.1 |########################################
   2507.2 |
   2805.3 |
   3103.4 |
   3401.5 |
   3699.6 |
   3997.6 |
   4295.7 |###
   4593.8 |###
   4891.9 |
   5190.0 |######
   5488.1 |######
   5786.1 |###
   6084.2 |
   6382.3 |
   6680.4 |
   6978.5 |##########
   7276.6 |######
   7574.7 |######
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=40.6% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=55.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=56.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-head**: CV=37.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=57.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=57.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
