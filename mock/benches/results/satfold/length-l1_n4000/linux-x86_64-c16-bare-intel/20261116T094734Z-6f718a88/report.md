# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-iterfold dominates: 16% faster than the next best (satfold-neon)

satfold-iterfold (4.38 us) leads satfold-neon (5.08 us) by 16%, a clear separation rather than a photo finish. CV 56.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-iterfold is fastest but the noisiest (CV 56.1%)

satfold-iterfold wins on median (4.38 us) yet has the highest variance (CV 56.1%), while satfold-lanes4-idx is the steadiest (CV 12.9%, 6.67 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.76)

satfold-neon's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.29 us) is smaller than the fastest variant's own run-to-run std-dev (2.46 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (4.38 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### satfold-neon is inconsistent: worst-20% is 2.4x its best-20%

satfold-neon's best 20% of batches run at 4.15 us but its worst 20% at 10.10 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 4377.2 ns median
- 4 variants significantly slower than baseline
- Spread: 1.52x (fastest 4377.2 ns, slowest 6671.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112716245 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5571ns | 4436ns | 4240ns | 4639ns | 9698ns | base |
| satfold-lanes16 | 5511ns | 5189ns | 4246ns | 5214ns | 7669ns | -1.08% |
| satfold-lanes16-constl | 5570ns | 5350ns | 4327ns | 5068ns | 8318ns | -0.02% |
| satfold-lanes4-idx | 6318ns | 6747ns | 5465ns | 6257ns | 7354ns | +13.40% |
| satfold-lanes64 | 5611ns | 5292ns | 4254ns | 5227ns | 8121ns | +0.72% |
| satfold-neon | 5901ns | 5154ns | 4208ns | 5000ns | 10298ns | +5.92% |
| satfold-neon8 | 5432ns | 5200ns | 4074ns | 4927ns | 8304ns | -2.50% |
| satfold-nolaw | 5672ns | 5418ns | 4416ns | 5412ns | 7708ns | +1.81% |
| satfold-seq | 5469ns | 5382ns | 4418ns | 5016ns | 7879ns | -1.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5485ns | 4181ns | 9517ns | base | 5.974 |
| satfold-lanes16 | 5426ns | 4188ns | 7516ns | -1.08% | 6.040 |
| satfold-lanes16-constl | 5486ns | 4269ns | 8161ns | +0.02% | 5.973 |
| satfold-lanes4-idx | 6242ns | 5407ns | 7244ns | +13.81% | 5.249 |
| satfold-lanes64 | 5525ns | 4196ns | 7964ns | +0.74% | 5.930 |
| satfold-neon | 5807ns | 4149ns | 10098ns | +5.88% | 5.643 |
| satfold-neon8 | 5346ns | 4016ns | 8138ns | -2.54% | 6.130 |
| satfold-nolaw | 5588ns | 4357ns | 7570ns | +1.87% | 5.864 |
| satfold-seq | 5385ns | 4358ns | 7725ns | -1.82% | 6.085 |

## Performance model

- Peak throughput: **8.160 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 7.486 | 91.7% |
| satfold-lanes16 | 6.403 | 78.5% |
| satfold-lanes16-constl | 6.208 | 76.1% |
| satfold-lanes4-idx | 4.911 | 60.2% |
| satfold-lanes64 | 6.276 | 76.9% |
| satfold-neon | 6.454 | 79.1% |
| satfold-neon8 | 6.392 | 78.3% |
| satfold-nolaw | 6.139 | 75.2% |
| satfold-seq | 6.172 | 75.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5571ns | 5571ns | base |
| satfold-lanes16 | 5511ns | 5511ns | -1.08% |
| satfold-lanes16-constl | 5570ns | 5570ns | -0.02% |
| satfold-lanes4-idx | 6318ns | 6318ns | +13.40% |
| satfold-lanes64 | 5611ns | 5611ns | +0.72% |
| satfold-neon | 5901ns | 5901ns | +5.92% |
| satfold-neon8 | 5432ns | 5432ns | -2.50% |
| satfold-nolaw | 5672ns | 5672ns | +1.81% |
| satfold-seq | 5469ns | 5469ns | -1.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4377ns | base | --- | [4245, 5091] | --- | --- | --- | --- |
| satfold-lanes16 | 5117ns | no significant difference | [-39, +835]ns | [5089, 5176] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes16-constl | 5278ns | +109.1ns (+2.5%) | [+55, +181]ns | [4401, 5324] | YES | 0.0007 | 0.0002 | 0 |
| satfold-lanes4-idx | 6672ns | +1264.6ns (+28.9%) | [+1174, +1499]ns | [5486, 6694] | YES | 0.0001 | 0.0000 | 0 |
| satfold-lanes64 | 5221ns | no significant difference | [-75, +921]ns | [5105, 5250] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon | 5078ns | +137.7ns (+3.1%) | [+35, +367]ns | [4536, 5186] | YES | 0.0442 | 0.0166 | 0 |
| satfold-neon8 | 5127ns | no significant difference | [-185, +56]ns | [4254, 5251] | no | 0.5728 | 0.4296 | 0 |
| satfold-nolaw | 5338ns | +1049.8ns (+24.0%) | [+157, +1110]ns | [5316, 5387] | YES (adj: no) | 0.1291 | 0.0807 | 0 |
| satfold-seq | 5309ns | no significant difference | [-332, +892]ns | [4409, 5329] | no | 0.1291 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4214ns | -0.4% | +26.3% | +59.6% | +24.9% | +7.1% | -4.8% | +26.1% | +3.5% |
| 1 | 2 | 4294ns | +22.0% | +1.3% | +26.3% | +22.3% | +19.0% | +22.7% | +26.1% | +24.0% |
| 1 | 3 | 4266ns | +20.3% | +0.8% | +28.6% | -0.3% | -1.0% | -0.4% | +26.4% | +25.6% |
| 1 | 4 | 5163ns | -1.5% | +3.4% | +29.5% | +0.5% | +0.9% | -0.5% | -15.6% | -14.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.089 | ok |
| satfold-lanes16 | -0.180 | ok |
| satfold-lanes16-constl | -0.251 | moderate- |
| satfold-lanes4-idx | -0.251 | moderate- |
| satfold-lanes64 | -0.371 | moderate- |
| satfold-neon | -0.765 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.710 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.089 | ok |
| satfold-seq | -0.249 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 1/4, lost 3/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 25.2ns | 5485.0ns | 0.5% |  |
| satfold-lanes16 | 25.6ns | 5425.6ns | 0.5% |  |
| satfold-lanes16-constl | 25.6ns | 5485.9ns | 0.5% |  |
| satfold-lanes4-idx | 22.7ns | 6242.5ns | 0.4% |  |
| satfold-lanes64 | 26.0ns | 5525.3ns | 0.5% |  |
| satfold-neon | 27.6ns | 5807.3ns | 0.5% |  |
| satfold-neon8 | 25.6ns | 5345.8ns | 0.5% |  |
| satfold-nolaw | 25.6ns | 5587.6ns | 0.5% |  |
| satfold-seq | 24.4ns | 5385.1ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4181.3-9516.8 ns)
   4181.3 |########################################
   4448.1 |####
   4714.9 |##
   4981.7 |#################
   5248.4 |##
   5515.2 |
   5782.0 |
   6048.8 |
   6315.5 |
   6582.3 |
   6849.1 |##
   7115.9 |
   7382.6 |
   7649.4 |
   7916.2 |
   8183.0 |
   8449.7 |
   8716.5 |
   8983.3 |
   9250.1 |
  (4 below, 5 above range)

satfold-lanes16 (n=40, range 4187.8-7516.2 ns)
   4187.8 |######
   4354.3 |
   4520.7 |
   4687.1 |
   4853.5 |
   5019.9 |########################################
   5186.3 |########
   5352.8 |######
   5519.2 |
   5685.6 |##
   5852.0 |
   6018.4 |
   6184.8 |##
   6351.3 |
   6517.7 |
   6684.1 |
   6850.5 |
   7016.9 |
   7183.4 |
   7349.8 |
  (5 below, 3 above range)

satfold-lanes16-constl (n=40, range 4268.8-8160.7 ns)
   4268.8 |#############################
   4463.4 |##
   4658.0 |
   4852.6 |
   5047.2 |#####
   5241.8 |########################################
   5436.4 |#####
   5631.0 |
   5825.6 |
   6020.2 |##
   6214.8 |
   6409.3 |
   6603.9 |
   6798.5 |
   6993.1 |
   7187.7 |
   7382.3 |##
   7576.9 |
   7771.5 |
   7966.1 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 5406.7-7243.6 ns)
   5406.7 |########################################
   5498.6 |######
   5590.4 |
   5682.3 |
   5774.1 |
   5865.9 |
   5957.8 |
   6049.6 |
   6141.5 |
   6233.3 |
   6325.2 |
   6417.0 |
   6508.9 |
   6600.7 |##########################
   6692.6 |####################################
   6784.4 |###
   6876.3 |
   6968.1 |
   7060.0 |
   7151.8 |
  (4 below, 2 above range)

satfold-lanes64 (n=40, range 4195.8-7964.2 ns)
   4195.8 |#######
   4384.2 |####
   4572.6 |
   4761.1 |
   4949.5 |###########
   5137.9 |########################################
   5326.3 |###########
   5514.8 |
   5703.2 |
   5891.6 |
   6080.0 |
   6268.4 |
   6456.9 |
   6645.3 |
   6833.7 |
   7022.1 |
   7210.6 |
   7399.0 |
   7587.4 |
   7775.8 |##
  (4 below, 3 above range)

satfold-neon (n=40, range 4149.0-10097.7 ns)
   4149.0 |##############################
   4446.4 |##########
   4743.9 |#############
   5041.3 |########################################
   5338.7 |###
   5636.2 |###
   5933.6 |
   6231.0 |
   6528.5 |
   6825.9 |
   7123.3 |###
   7420.8 |###
   7718.2 |
   8015.6 |
   8313.1 |###
   8610.5 |
   8907.9 |
   9205.4 |
   9502.8 |
   9800.2 |
  (3 below, 4 above range)

satfold-neon8 (n=40, range 4015.6-8138.1 ns)
   4015.6 |########################
   4221.7 |################
   4427.8 |
   4633.9 |
   4840.1 |
   5046.2 |########################################
   5252.3 |####################################
   5458.4 |
   5664.6 |
   5870.7 |
   6076.8 |
   6283.0 |
   6489.1 |
   6695.2 |
   6901.3 |
   7107.5 |####
   7313.6 |####
   7519.7 |
   7725.8 |
   7932.0 |
  (6 below, 3 above range)

satfold-nolaw (n=40, range 4356.6-7569.6 ns)
   4356.6 |######
   4517.2 |
   4677.9 |
   4838.5 |
   4999.2 |##
   5159.8 |##############
   5320.5 |########################################
   5481.1 |##
   5641.8 |
   5802.4 |
   5963.1 |
   6123.7 |
   6284.4 |
   6445.0 |
   6605.7 |
   6766.3 |
   6927.0 |
   7087.6 |
   7248.3 |
   7408.9 |
  (5 below, 3 above range)

satfold-seq (n=40, range 4358.1-7724.5 ns)
   4358.1 |########################################
   4526.4 |
   4694.7 |
   4863.1 |
   5031.4 |
   5199.7 |########################################
   5368.0 |########
   5536.3 |
   5704.7 |
   5873.0 |
   6041.3 |
   6209.6 |
   6377.9 |
   6546.3 |
   6714.6 |
   6882.9 |
   7051.2 |
   7219.5 |
   7387.9 |
   7556.2 |
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=44.8% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=30.3% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=36.2% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=32.5% (high variance, measurements may be unstable)
- **satfold-neon**: CV=44.8% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=36.7% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=30.1% (high variance, measurements may be unstable)
- **satfold-seq**: CV=31.0% (high variance, measurements may be unstable)
