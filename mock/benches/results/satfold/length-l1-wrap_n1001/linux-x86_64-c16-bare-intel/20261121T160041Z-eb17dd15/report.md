# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw shows alternating (throttle bounce) (autocorr -0.51)

satfold-nolaw's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (45 ns) is smaller than the fastest variant's own run-to-run std-dev (2.37 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-lanes4-idx vs stability leader satfold-iterfold (+0% speed for 1.1x steadier)

satfold-lanes4-idx is fastest (2.42 us, CV 97.9%); satfold-iterfold gives up 0.5% median for 1.1x lower variance (CV 92.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-nolaw is inconsistent: worst-20% is 4.0x its best-20%

satfold-nolaw's best 20% of batches run at 1.97 us but its worst 20% at 7.94 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.8% of the fastest

All 9 variants sit between 2.42 us and 2.46 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2416.4 ns median (-0.5% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.02x (fastest 2416.4 ns, slowest 2461.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112745822 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 4071ns | 2491ns | 2032ns | 3647ns | 7383ns | base |
| satfold-lanes16 | 4202ns | 2499ns | 2252ns | 3518ns | 8205ns | +3.22% |
| satfold-lanes16-constl | 4344ns | 2529ns | 2521ns | 3662ns | 8211ns | +6.70% |
| satfold-lanes4-idx | 4160ns | 2480ns | 2022ns | 3608ns | 7955ns | +2.19% |
| satfold-lanes64 | 4175ns | 2495ns | 2195ns | 3622ns | 7811ns | +2.54% |
| satfold-neon | 4285ns | 2498ns | 2031ns | 3912ns | 7657ns | +5.25% |
| satfold-neon8 | 4033ns | 2483ns | 2031ns | 3391ns | 7963ns | -0.93% |
| satfold-nolaw | 4330ns | 2484ns | 2020ns | 3814ns | 8186ns | +6.35% |
| satfold-seq | 4486ns | 2499ns | 2484ns | 3662ns | 8959ns | +10.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3958ns | 1980ns | 7147ns | base | 8.279 |
| satfold-lanes16 | 4084ns | 2195ns | 7944ns | +3.18% | 8.024 |
| satfold-lanes16-constl | 4225ns | 2458ns | 7955ns | +6.74% | 7.756 |
| satfold-lanes4-idx | 4043ns | 1970ns | 7698ns | +2.16% | 8.104 |
| satfold-lanes64 | 4058ns | 2139ns | 7561ns | +2.53% | 8.075 |
| satfold-neon | 4165ns | 1979ns | 7410ns | +5.24% | 7.867 |
| satfold-neon8 | 3921ns | 1979ns | 7710ns | -0.94% | 8.358 |
| satfold-nolaw | 4208ns | 1967ns | 7935ns | +6.32% | 7.787 |
| satfold-seq | 4358ns | 2421ns | 8663ns | +10.10% | 7.520 |

## Performance model

- Peak throughput: **16.658 Gops/s** (satfold-nolaw; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 13.498 | 81.0% |
| satfold-lanes16 | 13.454 | 80.8% |
| satfold-lanes16-constl | 13.315 | 79.9% |
| satfold-lanes4-idx | 13.560 | 81.4% |
| satfold-lanes64 | 13.476 | 80.9% |
| satfold-neon | 13.473 | 80.9% |
| satfold-neon8 | 13.543 | 81.3% |
| satfold-nolaw | 13.537 | 81.3% |
| satfold-seq | 13.456 | 80.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 4071ns | 4071ns | base |
| satfold-lanes16 | 4202ns | 4202ns | +3.22% |
| satfold-lanes16-constl | 4344ns | 4344ns | +6.70% |
| satfold-lanes4-idx | 4160ns | 4160ns | +2.19% |
| satfold-lanes64 | 4175ns | 4175ns | +2.54% |
| satfold-neon | 4285ns | 4285ns | +5.25% |
| satfold-neon8 | 4033ns | 4033ns | -0.93% |
| satfold-nolaw | 4330ns | 4330ns | +6.35% |
| satfold-seq | 4486ns | 4486ns | +10.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 2428ns | base | --- | [1992, 5781] | --- | --- | --- | --- |
| satfold-lanes16 | 2436ns | +438.7ns (+18.1%) | [+153, +448]ns | [2426, 5321] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| satfold-lanes16-constl | 2461ns | +476.9ns (+19.6%) | [+428, +480]ns | [2460, 4846] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2416ns | no significant difference | [-46, +422]ns | [1986, 5653] | no | 0.9996 | 0.8746 | 0 |
| satfold-lanes64 | 2432ns | +329.5ns (+13.6%) | [+7, +433]ns | [2422, 4623] | YES | 0.0059 | 0.0022 | 0 |
| satfold-neon | 2432ns | no significant difference | [-6, +269]ns | [1991, 6166] | no | 0.9996 | 0.8746 | 0 |
| satfold-neon8 | 2420ns | no significant difference | [-193, +336]ns | [1992, 5516] | no | 1.0000 | 1.0000 | 0 |
| satfold-nolaw | 2421ns | no significant difference | [-12, +428]ns | [2400, 5893] | no | 0.9996 | 0.8746 | 0 |
| satfold-seq | 2435ns | +441.8ns (+18.2%) | [+434, +460]ns | [2423, 3939] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2417ns | +1.0% | +1.8% | -18.2% | -9.8% | +19.5% | +61.2% | -18.3% | +0.7% |
| 1 | 2 | 1987ns | +22.4% | +23.9% | +21.9% | +22.9% | +28.6% | +22.2% | +116.8% | +22.5% |
| 1 | 3 | 2435ns | +66.4% | +1.2% | +98.3% | -0.2% | -14.2% | -18.7% | -0.6% | +0.1% |
| 1 | 4 | 2470ns | -1.7% | +0.1% | +85.8% | +6.6% | +64.6% | +68.0% | -2.0% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.334 | moderate- |
| satfold-lanes16 | -0.423 | moderate- |
| satfold-lanes16-constl | 0.188 | ok |
| satfold-lanes4-idx | 0.259 | moderate+ |
| satfold-lanes64 | -0.028 | ok |
| satfold-neon | -0.310 | moderate- |
| satfold-neon8 | -0.271 | moderate- |
| satfold-nolaw | -0.509 | HIGH- (thermal bounce) |
| satfold-seq | -0.270 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 3/4
- **satfold-lanes16-constl**: won 0/4, lost 3/4
- **satfold-lanes4-idx**: won 1/4, lost 3/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 37.8ns | 3957.9ns | 1.0% |  |
| satfold-lanes16 | 39.1ns | 4083.9ns | 1.0% |  |
| satfold-lanes16-constl | 39.6ns | 4224.7ns | 0.9% |  |
| satfold-lanes4-idx | 40.9ns | 4043.3ns | 1.0% |  |
| satfold-lanes64 | 38.6ns | 4058.0ns | 1.0% |  |
| satfold-neon | 39.8ns | 4165.3ns | 1.0% |  |
| satfold-neon8 | 37.3ns | 3920.6ns | 1.0% |  |
| satfold-nolaw | 41.6ns | 4208.0ns | 1.0% |  |
| satfold-seq | 41.5ns | 4357.7ns | 1.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1980.3-7146.6 ns)
   1980.3 |########################################
   2238.6 |##################
   2496.9 |
   2755.2 |######
   3013.5 |
   3271.8 |
   3530.2 |
   3788.5 |
   4046.8 |
   4305.1 |
   4563.4 |
   4821.7 |
   5080.1 |
   5338.4 |######
   5596.7 |
   5855.0 |###
   6113.3 |######
   6371.6 |#########
   6630.0 |######
   6888.3 |######
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 2194.5-7944.0 ns)
   2194.5 |########################################
   2482.0 |####
   2769.5 |
   3057.0 |
   3344.4 |
   3631.9 |
   3919.4 |
   4206.8 |
   4494.3 |
   4781.8 |##
   5069.3 |
   5356.7 |##
   5644.2 |####
   5931.7 |##
   6219.1 |####
   6506.6 |####
   6794.1 |
   7081.6 |##
   7369.0 |
   7656.5 |##
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 2458.1-7955.3 ns)
   2458.1 |########################################
   2732.9 |
   3007.8 |
   3282.6 |
   3557.5 |
   3832.4 |#
   4107.2 |
   4382.1 |
   4657.0 |
   4931.8 |
   5206.7 |
   5481.5 |#
   5756.4 |
   6031.3 |#
   6306.1 |
   6581.0 |###
   6855.9 |
   7130.7 |#####
   7405.6 |###
   7680.4 |#
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1970.4-7697.6 ns)
   1970.4 |########################################
   2256.7 |####################
   2543.1 |
   2829.5 |
   3115.8 |###
   3402.2 |
   3688.5 |
   3974.9 |###
   4261.3 |
   4547.6 |###
   4834.0 |
   5120.4 |
   5406.7 |###
   5693.1 |##########
   5979.5 |###
   6265.8 |######
   6552.2 |######
   6838.5 |
   7124.9 |###
   7411.3 |######
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 2138.8-7560.8 ns)
   2138.8 |########
   2409.9 |########################################
   2681.0 |#####
   2952.1 |
   3223.2 |
   3494.3 |
   3765.4 |
   4036.5 |
   4307.6 |##
   4578.7 |##
   4849.8 |
   5120.9 |
   5392.0 |
   5663.1 |
   5934.2 |
   6205.3 |##
   6476.4 |##
   6747.5 |########
   7018.6 |#####
   7289.7 |##
  (5 below, 5 above range)

satfold-neon (n=40, range 1979.4-7409.8 ns)
   1979.4 |########################################
   2251.0 |###############
   2522.5 |
   2794.0 |
   3065.5 |###
   3337.0 |
   3608.6 |###
   3880.1 |
   4151.6 |
   4423.1 |
   4694.6 |
   4966.2 |
   5237.7 |
   5509.2 |###
   5780.7 |###
   6052.2 |############
   6323.8 |###
   6595.3 |#########
   6866.8 |#########
   7138.3 |###
  (3 below, 3 above range)

satfold-neon8 (n=40, range 1978.9-7710.1 ns)
   1978.9 |########################################
   2265.4 |#################
   2552.0 |#####
   2838.6 |
   3125.1 |
   3411.7 |
   3698.3 |
   3984.8 |
   4271.4 |
   4557.9 |
   4844.5 |
   5131.1 |
   5417.6 |#####
   5704.2 |#####
   5990.8 |########
   6277.3 |##
   6563.9 |##
   6850.4 |
   7137.0 |##
   7423.6 |##
  (3 below, 4 above range)

satfold-nolaw (n=40, range 1967.1-7935.1 ns)
   1967.1 |####################
   2265.5 |########################################
   2563.9 |
   2862.3 |
   3160.7 |###
   3459.1 |###
   3757.5 |
   4055.9 |
   4354.3 |
   4652.7 |
   4951.1 |
   5249.5 |###
   5547.9 |###
   5846.3 |##########
   6144.7 |###
   6443.1 |###
   6741.5 |
   7039.9 |######
   7338.3 |###
   7636.7 |######
  (4 below, 4 above range)

satfold-seq (n=40, range 2420.9-8663.5 ns)
   2420.9 |########################################
   2733.0 |
   3045.2 |
   3357.3 |
   3669.4 |
   3981.5 |
   4293.7 |
   4605.8 |
   4917.9 |
   5230.1 |#
   5542.2 |
   5854.3 |
   6166.4 |
   6478.6 |#
   6790.7 |
   7102.8 |###
   7415.0 |#
   7727.1 |###
   8039.2 |
   8351.3 |#####
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=56.5% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes16**: CV=57.8% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=56.5% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=58.5% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes64**: CV=56.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-neon**: CV=56.2% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-neon8**: CV=60.9% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-nolaw**: CV=57.8% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-seq**: CV=61.9% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 3.6x (possible bimodal distribution)
