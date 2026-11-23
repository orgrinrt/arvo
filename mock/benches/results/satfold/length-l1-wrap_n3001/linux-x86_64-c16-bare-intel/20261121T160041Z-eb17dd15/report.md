# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (487 ns) is smaller than the fastest variant's own run-to-run std-dev (996 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-lanes16 vs stability leader satfold-neon (+3% speed for 1.6x steadier)

satfold-lanes16 is fastest (3.49 us, CV 28.6%); satfold-neon gives up 3.1% median for 1.6x lower variance (CV 17.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes16-constl is inconsistent: worst-20% is 4.1x its best-20%

satfold-lanes16-constl's best 20% of batches run at 1.28 us but its worst 20% at 5.17 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 3487.8 ns median (-9.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.14x (fastest 3487.8 ns, slowest 3974.6 ns)

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
| satfold-iterfold | 3835ns | 4089ns | 1405ns | 4096ns | 5482ns | base |
| satfold-lanes16 | 3599ns | 3656ns | 1952ns | 3710ns | 4915ns | -6.15% |
| satfold-lanes16-constl | 3663ns | 3801ns | 1335ns | 3843ns | 5450ns | -4.48% |
| satfold-lanes4-idx | 3483ns | 3737ns | 1274ns | 3696ns | 5053ns | -9.18% |
| satfold-lanes64 | 3558ns | 3837ns | 1354ns | 3671ns | 5422ns | -7.23% |
| satfold-neon | 3671ns | 3766ns | 2736ns | 3736ns | 4413ns | -4.27% |
| satfold-neon8 | 3799ns | 3834ns | 2666ns | 3814ns | 4886ns | -0.94% |
| satfold-nolaw | 3460ns | 3698ns | 2053ns | 3677ns | 4214ns | -9.79% |
| satfold-seq | 3770ns | 4162ns | 1577ns | 4018ns | 5218ns | -1.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3614ns | 1341ns | 5018ns | base | 9.066 |
| satfold-lanes16 | 3427ns | 1865ns | 4663ns | -5.19% | 9.563 |
| satfold-lanes16-constl | 3481ns | 1275ns | 5172ns | -3.68% | 9.412 |
| satfold-lanes4-idx | 3312ns | 1216ns | 4772ns | -8.37% | 9.894 |
| satfold-lanes64 | 3384ns | 1292ns | 5147ns | -6.38% | 9.684 |
| satfold-neon | 3496ns | 2611ns | 4179ns | -3.27% | 9.372 |
| satfold-neon8 | 3613ns | 2541ns | 4613ns | -0.04% | 9.069 |
| satfold-nolaw | 3287ns | 1959ns | 3947ns | -9.05% | 9.968 |
| satfold-seq | 3589ns | 1506ns | 4956ns | -0.70% | 9.130 |

## Performance model

- Peak throughput: **26.941 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 8.490 | 31.5% |
| satfold-lanes16 | 9.395 | 34.9% |
| satfold-lanes16-constl | 9.032 | 33.5% |
| satfold-lanes4-idx | 9.193 | 34.1% |
| satfold-lanes64 | 8.951 | 33.2% |
| satfold-neon | 9.110 | 33.8% |
| satfold-neon8 | 8.950 | 33.2% |
| satfold-nolaw | 9.279 | 34.4% |
| satfold-seq | 8.244 | 30.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3835ns | 3835ns | base |
| satfold-lanes16 | 3599ns | 3599ns | -6.15% |
| satfold-lanes16-constl | 3663ns | 3663ns | -4.48% |
| satfold-lanes4-idx | 3483ns | 3483ns | -9.18% |
| satfold-lanes64 | 3558ns | 3558ns | -7.23% |
| satfold-neon | 3671ns | 3671ns | -4.27% |
| satfold-neon8 | 3799ns | 3799ns | -0.94% |
| satfold-nolaw | 3460ns | 3460ns | -9.79% |
| satfold-seq | 3770ns | 3770ns | -1.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 3860ns | base | --- | [3443, 4461] | --- | --- | --- | --- |
| satfold-lanes16 | 3488ns | -200.8ns (-5.2%) | [-342, -106]ns | [3275, 3731] | YES | 0.0007 | 0.0002 | 0 |
| satfold-lanes16-constl | 3628ns | no significant difference | [-304, +71]ns | [3320, 3960] | no | 0.6873 | 0.4296 | 0 |
| satfold-lanes4-idx | 3565ns | -252.5ns (-6.5%) | [-413, -201]ns | [3205, 3793] | YES | 0.0001 | 0.0000 | 0 |
| satfold-lanes64 | 3661ns | no significant difference | [-489, +148]ns | [3298, 4059] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon | 3597ns | no significant difference | [-491, +206]ns | [3386, 3757] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon8 | 3661ns | no significant difference | [-682, +136]ns | [3446, 3829] | no | 1.0000 | 1.0000 | 0 |
| satfold-nolaw | 3532ns | no significant difference | [-833, +23]ns | [3387, 3653] | no | 0.5364 | 0.2682 | 0 |
| satfold-seq | 3975ns | no significant difference | [-7, +123]ns | [3523, 4150] | no | 0.5364 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4137ns | -12.9% | -11.6% | -14.8% | -55.5% | -6.7% | -17.2% | -14.4% | +1.6% |
| 1 | 2 | 3443ns | -6.1% | +1.4% | -6.9% | +6.3% | +8.4% | +3.8% | +0.9% | +5.9% |
| 1 | 3 | 4671ns | -21.1% | -27.9% | -19.7% | -5.8% | -27.4% | -22.3% | -25.3% | -18.2% |
| 1 | 4 | 4569ns | -2.5% | -9.4% | -5.9% | -17.8% | -25.8% | -2.5% | -20.1% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.143 | ok |
| satfold-lanes16 | 0.077 | ok |
| satfold-lanes16-constl | -0.258 | moderate- |
| satfold-lanes4-idx | 0.140 | ok |
| satfold-lanes64 | 0.053 | ok |
| satfold-neon | 0.289 | moderate+ |
| satfold-neon8 | -0.002 | ok |
| satfold-nolaw | -0.140 | ok |
| satfold-seq | -0.167 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 3/4, lost 1/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 62.7ns | 3614.4ns | 1.7% |  |
| satfold-lanes16 | 58.6ns | 3426.7ns | 1.7% |  |
| satfold-lanes16-constl | 59.6ns | 3481.4ns | 1.7% |  |
| satfold-lanes4-idx | 57.6ns | 3312.0ns | 1.7% |  |
| satfold-lanes64 | 59.0ns | 3383.6ns | 1.7% |  |
| satfold-neon | 60.5ns | 3496.3ns | 1.7% |  |
| satfold-neon8 | 62.4ns | 3613.1ns | 1.7% |  |
| satfold-nolaw | 58.1ns | 3287.4ns | 1.8% |  |
| satfold-seq | 62.0ns | 3589.1ns | 1.7% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1341.1-5017.7 ns)
   1341.1 |#################################
   1524.9 |
   1708.8 |
   1892.6 |
   2076.4 |
   2260.2 |
   2444.1 |
   2627.9 |#############
   2811.7 |
   2995.6 |#############
   3179.4 |######
   3363.2 |####################
   3547.1 |#############
   3730.9 |####################
   3914.7 |######
   4098.6 |######
   4282.4 |##########################
   4466.2 |######
   4650.1 |########################################
   4833.9 |####################
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 1864.9-4663.2 ns)
   1864.9 |
   2004.8 |
   2144.7 |
   2284.6 |
   2424.5 |
   2564.4 |######
   2704.4 |
   2844.3 |####################
   2984.2 |######
   3124.1 |##########################
   3264.1 |#############
   3404.0 |########################################
   3543.9 |#############
   3683.8 |##########################
   3823.7 |
   3963.6 |####################
   4103.6 |######
   4243.5 |#############
   4383.4 |######
   4523.3 |######
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 1275.3-5172.0 ns)
   1275.3 |################
   1470.2 |
   1665.0 |########
   1859.8 |
   2054.7 |
   2249.5 |
   2444.3 |
   2639.2 |
   2834.0 |########
   3028.8 |########
   3223.7 |########################################
   3418.5 |########################################
   3613.3 |########################
   3808.2 |########################
   4003.0 |########################################
   4197.8 |
   4392.6 |################
   4587.5 |
   4782.3 |########################
   4977.1 |########
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 1216.3-4772.0 ns)
   1216.3 |################
   1394.1 |
   1571.9 |########
   1749.6 |
   1927.4 |
   2105.2 |
   2283.0 |########
   2460.8 |
   2638.6 |
   2816.3 |################
   2994.1 |################
   3171.9 |################
   3349.7 |########################################
   3527.5 |################################
   3705.3 |########################
   3883.1 |################
   4060.8 |################
   4238.6 |########
   4416.4 |########################
   4594.2 |########################
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 1292.3-5147.4 ns)
   1292.3 |########################################
   1485.0 |
   1677.8 |
   1870.5 |
   2063.3 |
   2256.1 |########
   2448.8 |
   2641.6 |
   2834.3 |
   3027.1 |########
   3219.8 |####
   3412.6 |########
   3605.4 |######################
   3798.1 |#############
   3990.9 |########
   4183.6 |#############
   4376.4 |########
   4569.1 |########
   4761.9 |########
   4954.7 |####
  (1 below, 3 above range)

satfold-neon (n=40, range 2611.5-4178.5 ns)
   2611.5 |
   2689.8 |
   2768.2 |
   2846.5 |
   2924.9 |
   3003.2 |######
   3081.6 |######
   3159.9 |########################################
   3238.3 |######
   3316.6 |########################################
   3395.0 |
   3473.3 |######
   3551.7 |#############
   3630.1 |######
   3708.4 |#################################
   3786.8 |####################
   3865.1 |####################
   3943.5 |#############
   4021.8 |######
   4100.2 |######
  (3 below, 3 above range)

satfold-neon8 (n=40, range 2541.5-4613.4 ns)
   2541.5 |
   2645.0 |
   2748.6 |
   2852.2 |
   2955.8 |################
   3059.4 |########
   3163.0 |########################################
   3266.6 |
   3370.2 |################################
   3473.8 |################
   3577.4 |########################################
   3681.0 |################################
   3784.6 |
   3888.2 |################################
   3991.8 |########################
   4095.4 |
   4199.0 |########
   4302.6 |
   4406.2 |################
   4509.8 |
  (3 below, 4 above range)

satfold-nolaw (n=40, range 1958.6-3947.3 ns)
   1958.6 |
   2058.1 |
   2157.5 |
   2256.9 |
   2356.4 |
   2455.8 |
   2555.2 |
   2654.7 |
   2754.1 |
   2853.5 |####
   2953.0 |####
   3052.4 |
   3151.8 |######################
   3251.3 |
   3350.7 |#################
   3450.1 |######################
   3549.6 |########
   3649.0 |########################################
   3748.4 |#############
   3847.9 |
  (5 below, 5 above range)

satfold-seq (n=40, range 1505.6-4956.2 ns)
   1505.6 |
   1678.1 |
   1850.6 |
   2023.2 |
   2195.7 |
   2368.2 |
   2540.7 |####################
   2713.3 |
   2885.8 |
   3058.3 |
   3230.9 |######
   3403.4 |##########################
   3575.9 |####################
   3748.5 |######
   3921.0 |########################################
   4093.5 |####################
   4266.0 |#############
   4438.6 |#############
   4611.1 |
   4783.6 |########################################
  (7 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=36.4% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-lanes16**: CV=29.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=37.6% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=36.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes64**: CV=41.7% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-neon8**: CV=21.7% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=24.1% (high variance, measurements may be unstable)
- **satfold-seq**: CV=33.7% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
