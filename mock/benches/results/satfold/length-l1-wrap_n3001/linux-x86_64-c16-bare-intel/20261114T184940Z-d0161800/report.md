# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-neon8, satfold-nolaw) are a dead heat (<1%)

satfold-neon8 (3.64 us) and satfold-nolaw (3.66 us) differ by 0.50%, inside the noise, even though the wider field spreads 17.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (643 ns) is smaller than the fastest variant's own run-to-run std-dev (1.21 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-neon8 vs stability leader satfold-lanes64 (+1% speed for 1.5x steadier)

satfold-neon8 is fastest (3.64 us, CV 33.1%); satfold-lanes64 gives up 0.6% median for 1.5x lower variance (CV 21.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-nolaw is inconsistent: worst-20% is 4.2x its best-20%

satfold-nolaw's best 20% of batches run at 1.31 us but its worst 20% at 5.56 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 3641.7 ns median (-3.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.18x (fastest 3641.7 ns, slowest 4284.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 3607ns | 3935ns | 1319ns | 3850ns | 5165ns | base |
| satfold-lanes16 | 4147ns | 4270ns | 2631ns | 4307ns | 5186ns | +14.99% |
| satfold-lanes16-constl | 3966ns | 4264ns | 1530ns | 4268ns | 5494ns | +9.96% |
| satfold-lanes4-idx | 3722ns | 4302ns | 1378ns | 3900ns | 5533ns | +3.20% |
| satfold-lanes64 | 3864ns | 3848ns | 2619ns | 3940ns | 4882ns | +7.15% |
| satfold-neon | 4198ns | 4437ns | 2336ns | 4404ns | 5441ns | +16.39% |
| satfold-neon8 | 3631ns | 3830ns | 1475ns | 3835ns | 5174ns | +0.68% |
| satfold-nolaw | 3770ns | 3834ns | 1375ns | 3853ns | 5913ns | +4.52% |
| satfold-seq | 4012ns | 4488ns | 1618ns | 4411ns | 5209ns | +11.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3421ns | 1257ns | 4846ns | base | 9.579 |
| satfold-lanes16 | 3952ns | 2514ns | 4903ns | +15.53% | 8.291 |
| satfold-lanes16-constl | 3769ns | 1462ns | 5141ns | +10.17% | 8.695 |
| satfold-lanes4-idx | 3534ns | 1316ns | 5207ns | +3.32% | 9.271 |
| satfold-lanes64 | 3667ns | 2493ns | 4599ns | +7.18% | 8.937 |
| satfold-neon | 3984ns | 2225ns | 5109ns | +16.47% | 8.224 |
| satfold-neon8 | 3452ns | 1407ns | 4899ns | +0.91% | 9.493 |
| satfold-nolaw | 3583ns | 1313ns | 5565ns | +4.74% | 9.145 |
| satfold-seq | 3814ns | 1545ns | 4895ns | +11.50% | 8.591 |

## Performance model

- Peak throughput: **26.062 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 8.717 | 33.4% |
| satfold-lanes16 | 8.062 | 30.9% |
| satfold-lanes16-constl | 8.039 | 30.8% |
| satfold-lanes4-idx | 8.051 | 30.9% |
| satfold-lanes64 | 8.941 | 34.3% |
| satfold-neon | 7.753 | 29.7% |
| satfold-neon8 | 8.998 | 34.5% |
| satfold-nolaw | 8.953 | 34.4% |
| satfold-seq | 7.647 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3607ns | 3607ns | base |
| satfold-lanes16 | 4147ns | 4147ns | +14.99% |
| satfold-lanes16-constl | 3966ns | 3966ns | +9.96% |
| satfold-lanes4-idx | 3722ns | 3722ns | +3.20% |
| satfold-lanes64 | 3864ns | 3864ns | +7.15% |
| satfold-neon | 4198ns | 4198ns | +16.39% |
| satfold-neon8 | 3631ns | 3631ns | +0.68% |
| satfold-nolaw | 3770ns | 3770ns | +4.52% |
| satfold-seq | 4012ns | 4012ns | +11.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 3759ns | base | --- | [3501, 3994] | --- | --- | --- | --- |
| satfold-lanes16 | 4064ns | +134.2ns (+3.6%) | [+48, +715]ns | [3860, 4409] | YES | 0.0054 | 0.0007 | 0 |
| satfold-lanes16-constl | 4076ns | +273.4ns (+7.3%) | [+73, +525]ns | [3779, 4486] | YES | 0.0089 | 0.0022 | 0 |
| satfold-lanes4-idx | 4070ns | no significant difference | [-13, +334]ns | [3755, 4398] | no | 0.2462 | 0.1539 | 0 |
| satfold-lanes64 | 3665ns | no significant difference | [-90, +295]ns | [3596, 3952] | no | 0.9996 | 0.8746 | 0 |
| satfold-neon | 4226ns | +441.2ns (+11.7%) | [+221, +730]ns | [3923, 4537] | YES | 0.0129 | 0.0064 | 0 |
| satfold-neon8 | 3642ns | no significant difference | [-260, +272]ns | [3273, 3971] | no | 1.0000 | 1.0000 | 0 |
| satfold-nolaw | 3660ns | no significant difference | [-151, +160]ns | [3394, 3996] | no | 0.8478 | 0.6358 | 0 |
| satfold-seq | 4285ns | +329.8ns (+8.8%) | [+59, +472]ns | [3806, 4501] | YES | 0.0129 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4092ns | +0.7% | +2.6% | -0.5% | -16.7% | +17.3% | -26.4% | -67.7% | +8.3% |
| 1 | 2 | 3855ns | +2.8% | +6.7% | +12.7% | +2.1% | +6.6% | -15.1% | +3.2% | +14.3% |
| 1 | 3 | 3291ns | +19.6% | +21.8% | +27.1% | +24.7% | +21.9% | +26.0% | +21.4% | +29.0% |
| 1 | 4 | 3567ns | +24.3% | +17.0% | -26.2% | +1.3% | +19.5% | +5.4% | -7.6% | +10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.144 | ok |
| satfold-lanes16 | -0.198 | ok |
| satfold-lanes16-constl | -0.249 | moderate- |
| satfold-lanes4-idx | -0.052 | ok |
| satfold-lanes64 | -0.185 | ok |
| satfold-neon | -0.086 | ok |
| satfold-neon8 | 0.144 | ok |
| satfold-nolaw | -0.144 | ok |
| satfold-seq | 0.182 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 2/4, lost 2/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 61.1ns | 3420.8ns | 1.8% |  |
| satfold-lanes16 | 69.3ns | 3952.2ns | 1.8% |  |
| satfold-lanes16-constl | 67.3ns | 3768.6ns | 1.8% |  |
| satfold-lanes4-idx | 62.4ns | 3534.5ns | 1.8% |  |
| satfold-lanes64 | 66.1ns | 3666.6ns | 1.8% |  |
| satfold-neon | 72.0ns | 3984.3ns | 1.8% |  |
| satfold-neon8 | 63.7ns | 3451.8ns | 1.8% |  |
| satfold-nolaw | 63.9ns | 3583.0ns | 1.8% |  |
| satfold-seq | 67.9ns | 3814.2ns | 1.8% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1257.3-4845.6 ns)
   1257.3 |########################################
   1436.7 |
   1616.1 |
   1795.5 |
   1975.0 |
   2154.4 |#####
   2333.8 |
   2513.2 |
   2692.6 |
   2872.0 |#####
   3051.5 |
   3230.9 |
   3410.3 |######################
   3589.7 |##################################
   3769.1 |#####
   3948.5 |############################
   4128.0 |#####
   4307.4 |######################
   4486.8 |
   4666.2 |######################
  (2 below, 4 above range)

satfold-lanes16 (n=40, range 2513.8-4902.9 ns)
   2513.8 |
   2633.2 |
   2752.7 |
   2872.1 |
   2991.6 |
   3111.0 |
   3230.5 |
   3350.0 |################################
   3469.4 |
   3588.9 |########
   3708.3 |################################
   3827.8 |################
   3947.3 |########################################
   4066.7 |################################
   4186.2 |
   4305.6 |################################
   4425.1 |########################
   4544.5 |
   4664.0 |########################
   4783.5 |################################
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 1461.9-5141.4 ns)
   1461.9 |
   1645.9 |
   1829.9 |
   2013.8 |
   2197.8 |#####
   2381.8 |#####
   2565.8 |
   2749.7 |
   2933.7 |
   3117.7 |
   3301.7 |#####
   3485.6 |###########
   3669.6 |#################
   3853.6 |######################
   4037.5 |#################
   4221.5 |###########
   4405.5 |############################
   4589.5 |#####
   4773.4 |########################################
   4957.4 |#####
  (7 below, 2 above range)

satfold-lanes4-idx (n=40, range 1315.6-5207.2 ns)
   1315.6 |########################################
   1510.2 |#####
   1704.8 |
   1899.3 |
   2093.9 |
   2288.5 |#####
   2483.1 |
   2677.7 |
   2872.2 |
   3066.8 |
   3261.4 |
   3456.0 |
   3650.6 |###########
   3845.1 |#################
   4039.7 |#################
   4234.3 |##################################
   4428.9 |
   4623.4 |############################
   4818.0 |#################
   5012.6 |###########
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 2493.3-4599.1 ns)
   2493.3 |
   2598.6 |
   2703.8 |########
   2809.1 |
   2914.4 |
   3019.7 |########
   3125.0 |########################
   3230.3 |
   3335.6 |################################
   3440.9 |########
   3546.2 |########################################
   3651.5 |########################
   3756.8 |########
   3862.1 |########################################
   3967.4 |########################
   4072.7 |
   4177.9 |########
   4283.2 |################################
   4388.5 |########
   4493.8 |
  (3 below, 4 above range)

satfold-neon (n=40, range 2224.6-5109.4 ns)
   2224.6 |
   2368.8 |
   2513.1 |######
   2657.3 |
   2801.5 |
   2945.8 |######
   3090.0 |
   3234.3 |######
   3378.5 |
   3522.8 |####################
   3667.0 |####################
   3811.2 |####################
   3955.5 |#############
   4099.7 |####################
   4244.0 |####################
   4388.2 |#############
   4532.5 |######
   4676.7 |########################################
   4821.0 |######
   4965.2 |##########################
  (4 below, 2 above range)

satfold-neon8 (n=40, range 1407.5-4898.7 ns)
   1407.5 |
   1582.0 |
   1756.6 |
   1931.1 |########
   2105.7 |
   2280.2 |########
   2454.8 |
   2629.4 |
   2803.9 |
   2978.5 |########################
   3153.1 |########################
   3327.6 |################
   3502.2 |########################################
   3676.7 |########
   3851.3 |################################
   4025.9 |
   4200.4 |########################
   4375.0 |################
   4549.5 |################
   4724.1 |################################
  (7 below, 2 above range)

satfold-nolaw (n=40, range 1313.3-5564.9 ns)
   1313.3 |########################################
   1525.9 |
   1738.5 |
   1951.1 |
   2163.7 |
   2376.2 |
   2588.8 |
   2801.4 |#####
   3014.0 |#################
   3226.6 |############################
   3439.1 |###########
   3651.7 |############################
   3864.3 |######################
   4076.9 |###########
   4289.5 |###########
   4502.0 |###########
   4714.6 |###########
   4927.2 |#####
   5139.8 |
   5352.4 |
  (1 below, 3 above range)

satfold-seq (n=40, range 1544.8-4895.3 ns)
   1544.8 |
   1712.3 |
   1879.8 |
   2047.4 |
   2214.9 |
   2382.4 |
   2549.9 |
   2717.5 |
   2885.0 |
   3052.5 |####
   3220.0 |
   3387.6 |####
   3555.1 |#################
   3722.6 |########
   3890.1 |####
   4057.7 |#############
   4225.2 |#############
   4392.7 |######################
   4560.2 |####
   4727.8 |########################################
  (7 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=37.6% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes16**: CV=23.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=34.8% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=43.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=21.8% (high variance, measurements may be unstable)
- **satfold-neon**: CV=27.3% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=35.0% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-nolaw**: CV=41.1% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-seq**: CV=32.4% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 3.2x (possible bimodal distribution)
