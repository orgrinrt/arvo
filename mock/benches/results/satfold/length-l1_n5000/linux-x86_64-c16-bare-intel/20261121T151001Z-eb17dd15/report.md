# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw dominates: 11% faster than the next best (satfold-lanes16-constl)

satfold-nolaw (4.14 us) leads satfold-lanes16-constl (4.59 us) by 11%, a clear separation rather than a photo finish. CV 32.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-seq is inconsistent: worst-20% is 2.1x its best-20%

satfold-seq's best 20% of batches run at 4.04 us but its worst 20% at 8.43 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4139.2 ns median (-16.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.48x (fastest 4139.2 ns, slowest 6129.1 ns)

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
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5205ns | 5015ns | 4992ns | 5030ns | 5942ns | base |
| satfold-lanes16 | 5562ns | 5665ns | 4658ns | 5269ns | 7345ns | +6.87% |
| satfold-lanes16-constl | 5203ns | 4653ns | 4637ns | 4662ns | 7393ns | -0.03% |
| satfold-lanes4-idx | 6007ns | 6202ns | 5093ns | 5789ns | 7574ns | +15.41% |
| satfold-lanes64 | 5260ns | 5083ns | 4118ns | 5048ns | 7041ns | +1.07% |
| satfold-neon | 5257ns | 5028ns | 4183ns | 5043ns | 6973ns | +1.00% |
| satfold-neon8 | 5306ns | 5029ns | 4112ns | 4673ns | 8396ns | +1.94% |
| satfold-nolaw | 4718ns | 4196ns | 4106ns | 4344ns | 6451ns | -9.35% |
| satfold-seq | 5533ns | 5008ns | 4098ns | 4990ns | 8600ns | +6.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5124ns | 4921ns | 5834ns | base | 6.395 |
| satfold-lanes16 | 5485ns | 4598ns | 7226ns | +7.06% | 5.974 |
| satfold-lanes16-constl | 5128ns | 4579ns | 7256ns | +0.09% | 6.389 |
| satfold-lanes4-idx | 5931ns | 5028ns | 7464ns | +15.75% | 5.525 |
| satfold-lanes64 | 5179ns | 4059ns | 6919ns | +1.07% | 6.328 |
| satfold-neon | 5173ns | 4125ns | 6832ns | +0.97% | 6.334 |
| satfold-neon8 | 5220ns | 4053ns | 8226ns | +1.88% | 6.277 |
| satfold-nolaw | 4644ns | 4044ns | 6338ns | -9.37% | 7.057 |
| satfold-seq | 5446ns | 4040ns | 8433ns | +6.28% | 6.017 |

## Performance model

- Peak throughput: **8.111 Gops/s** (satfold-seq; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.630 | 81.7% |
| satfold-lanes16 | 5.857 | 72.2% |
| satfold-lanes16-constl | 7.132 | 87.9% |
| satfold-lanes4-idx | 5.346 | 65.9% |
| satfold-lanes64 | 6.556 | 80.8% |
| satfold-neon | 6.611 | 81.5% |
| satfold-neon8 | 6.609 | 81.5% |
| satfold-nolaw | 7.916 | 97.6% |
| satfold-seq | 6.635 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5205ns | 5205ns | base |
| satfold-lanes16 | 5562ns | 5562ns | +6.87% |
| satfold-lanes16-constl | 5203ns | 5203ns | -0.03% |
| satfold-lanes4-idx | 6007ns | 6007ns | +15.41% |
| satfold-lanes64 | 5260ns | 5260ns | +1.07% |
| satfold-neon | 5257ns | 5257ns | +1.00% |
| satfold-neon8 | 5306ns | 5306ns | +1.94% |
| satfold-nolaw | 4718ns | 4718ns | -9.35% |
| satfold-seq | 5533ns | 5533ns | +6.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4942ns | base | --- | [4932, 4968] | --- | --- | --- | --- |
| satfold-lanes16 | 5594ns | no significant difference | [-312, +614]ns | [4636, 5610] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes16-constl | 4595ns | -346.7ns (-7.0%) | [-353, -339]ns | [4590, 4619] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 6129ns | +851.8ns (+17.2%) | [+256, +1231]ns | [5202, 6180] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4998ns | no significant difference | [-25, +55]ns | [4969, 5042] | no | 0.4291 | 0.2682 | 0 |
| satfold-neon | 4957ns | no significant difference | [-61, +30]ns | [4943, 5038] | no | 0.5728 | 0.4296 | 0 |
| satfold-neon8 | 4958ns | no significant difference | [-867, +35]ns | [4072, 4979] | no | 0.4291 | 0.2682 | 0 |
| satfold-nolaw | 4139ns | -834.4ns (-16.9%) | [-873, -707]ns | [4100, 4226] | YES | 0.0005 | 0.0002 | 0 |
| satfold-seq | 4939ns | no significant difference | [-100, +26]ns | [4926, 4960] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4945ns | +13.8% | -7.2% | +5.4% | +0.5% | +0.2% | -17.8% | -17.5% | -18.2% |
| 1 | 2 | 4940ns | -6.8% | -6.1% | +2.0% | +2.8% | -16.0% | -17.8% | +2.5% | +0.4% |
| 1 | 3 | 4929ns | -5.9% | -6.8% | +25.3% | -17.6% | +0.6% | +1.4% | -16.4% | +0.5% |
| 1 | 4 | 5008ns | +12.0% | -8.3% | +25.6% | +0.9% | +1.3% | -0.7% | -17.3% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.209 | moderate- |
| satfold-lanes16 | -0.250 | moderate- |
| satfold-lanes16-constl | -0.415 | moderate- |
| satfold-lanes4-idx | 0.228 | moderate+ |
| satfold-lanes64 | -0.491 | moderate- |
| satfold-neon | -0.310 | moderate- |
| satfold-neon8 | 0.242 | moderate+ |
| satfold-nolaw | -0.455 | moderate- |
| satfold-seq | -0.091 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.2ns | 5123.6ns | 0.5% |  |
| satfold-lanes16 | 22.6ns | 5485.2ns | 0.4% |  |
| satfold-lanes16-constl | 21.6ns | 5128.5ns | 0.4% |  |
| satfold-lanes4-idx | 22.9ns | 5930.8ns | 0.4% |  |
| satfold-lanes64 | 24.7ns | 5178.7ns | 0.5% |  |
| satfold-neon | 24.1ns | 5173.3ns | 0.5% |  |
| satfold-neon8 | 24.7ns | 5220.1ns | 0.5% |  |
| satfold-nolaw | 21.8ns | 4643.5ns | 0.5% |  |
| satfold-seq | 25.7ns | 5445.6ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4921.5-5834.0 ns)
   4921.5 |########################################
   4967.1 |#######
   5012.7 |#########
   5058.3 |###
   5104.0 |
   5149.6 |#
   5195.2 |
   5240.8 |
   5286.5 |
   5332.1 |
   5377.7 |
   5423.4 |
   5469.0 |
   5514.6 |
   5560.2 |
   5605.9 |
   5651.5 |
   5697.1 |
   5742.7 |
   5788.4 |
  (5 below, 2 above range)

satfold-lanes16 (n=40, range 4598.2-7225.5 ns)
   4598.2 |##########################
   4729.6 |
   4861.0 |
   4992.3 |
   5123.7 |
   5255.0 |
   5386.4 |
   5517.8 |########################################
   5649.1 |##
   5780.5 |
   5911.9 |
   6043.2 |
   6174.6 |
   6306.0 |
   6437.3 |
   6568.7 |
   6700.0 |
   6831.4 |##
   6962.8 |
   7094.1 |
  (6 below, 2 above range)

satfold-lanes16-constl (n=40, range 4579.1-7255.6 ns)
   4579.1 |########################################
   4712.9 |#
   4846.7 |#
   4980.5 |
   5114.4 |
   5248.2 |
   5382.0 |
   5515.9 |
   5649.7 |
   5783.5 |
   5917.3 |
   6051.2 |
   6185.0 |
   6318.8 |
   6452.7 |
   6586.5 |
   6720.3 |
   6854.1 |
   6988.0 |
   7121.8 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 5028.1-7463.8 ns)
   5028.1 |##############
   5149.9 |#########################
   5271.7 |
   5393.5 |
   5515.2 |
   5637.0 |
   5758.8 |
   5880.6 |
   6002.4 |
   6124.2 |########################################
   6245.9 |###########
   6367.7 |
   6489.5 |##
   6611.3 |
   6733.1 |
   6854.8 |
   6976.6 |
   7098.4 |
   7220.2 |
   7342.0 |
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 4059.5-6919.3 ns)
   4059.5 |########
   4202.5 |
   4345.5 |
   4488.5 |
   4631.4 |
   4774.4 |
   4917.4 |########################################
   5060.4 |################
   5203.4 |
   5346.4 |
   5489.4 |
   5632.4 |
   5775.4 |##
   5918.3 |
   6061.3 |
   6204.3 |
   6347.3 |
   6490.3 |
   6633.3 |
   6776.3 |
  (5 below, 2 above range)

satfold-neon (n=40, range 4125.1-6832.5 ns)
   4125.1 |#############
   4260.5 |
   4395.9 |
   4531.2 |
   4666.6 |##
   4802.0 |########
   4937.3 |########################################
   5072.7 |###########
   5208.1 |
   5343.4 |
   5478.8 |
   5614.2 |##
   5749.5 |
   5884.9 |
   6020.3 |
   6155.6 |
   6291.0 |##
   6426.4 |
   6561.7 |
   6697.1 |
  (2 below, 2 above range)

satfold-neon8 (n=40, range 4053.1-8226.1 ns)
   4053.1 |################################
   4261.7 |
   4470.4 |
   4679.0 |
   4887.7 |########################################
   5096.3 |##
   5305.0 |
   5513.6 |
   5722.3 |
   5930.9 |
   6139.6 |
   6348.2 |
   6556.9 |
   6765.5 |
   6974.2 |
   7182.8 |
   7391.5 |
   7600.1 |
   7808.8 |
   8017.4 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 4044.3-6338.2 ns)
   4044.3 |########################################
   4159.0 |###
   4273.7 |#
   4388.4 |
   4503.1 |
   4617.8 |
   4732.5 |
   4847.2 |###
   4961.9 |######
   5076.6 |#####
   5191.3 |
   5306.0 |
   5420.7 |
   5535.4 |
   5650.1 |
   5764.8 |
   5879.4 |
   5994.1 |
   6108.8 |
   6223.5 |#
  (2 below, 2 above range)

satfold-seq (n=40, range 4040.0-8432.8 ns)
   4040.0 |#########
   4259.6 |
   4479.3 |
   4698.9 |#
   4918.5 |########################################
   5138.2 |#######
   5357.8 |
   5577.5 |
   5797.1 |
   6016.7 |
   6236.4 |
   6456.0 |
   6675.7 |
   6895.3 |
   7114.9 |
   7334.6 |
   7554.2 |
   7773.9 |
   7993.5 |
   8213.1 |#
  (4 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=28.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=31.1% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=21.5% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=31.0% (high variance, measurements may be unstable)
- **satfold-neon**: CV=30.0% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=41.5% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=29.3% (high variance, measurements may be unstable)
- **satfold-seq**: CV=37.0% (high variance, measurements may be unstable)
