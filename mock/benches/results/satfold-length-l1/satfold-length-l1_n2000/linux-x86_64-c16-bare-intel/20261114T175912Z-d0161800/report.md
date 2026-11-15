# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-lanes4-idx, satfold-neon) are a dead heat (<1%)

satfold-lanes4-idx (5.23 us) and satfold-neon (5.24 us) differ by 0.03%, inside the noise, even though the wider field spreads 5.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (290 ns) is smaller than the fastest variant's own run-to-run std-dev (1.59 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-lanes4-idx vs stability leader satfold-lanes64 (+2% speed for 3.2x steadier)

satfold-lanes4-idx is fastest (5.23 us, CV 30.4%); satfold-lanes64 gives up 1.9% median for 3.2x lower variance (CV 9.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-seq is inconsistent: worst-20% is 2.2x its best-20%

satfold-seq's best 20% of batches run at 4.48 us but its worst 20% at 9.73 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes4-idx** at 5233.9 ns median (-0.7% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.06x (fastest 5233.9 ns, slowest 5523.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5570ns | 5350ns | 4380ns | 5320ns | 7510ns | base |
| satfold-lanes16 | 5901ns | 5508ns | 4473ns | 5442ns | 8705ns | +5.93% |
| satfold-lanes16-constl | 5533ns | 5432ns | 4373ns | 5385ns | 7136ns | -0.67% |
| satfold-lanes4-idx | 5789ns | 5306ns | 5244ns | 5312ns | 7766ns | +3.92% |
| satfold-lanes64 | 5581ns | 5408ns | 5268ns | 5419ns | 6381ns | +0.20% |
| satfold-neon | 5358ns | 5310ns | 4338ns | 5106ns | 7136ns | -3.81% |
| satfold-neon8 | 5809ns | 5508ns | 5352ns | 5507ns | 7174ns | +4.29% |
| satfold-nolaw | 6129ns | 5500ns | 5449ns | 5570ns | 8486ns | +10.03% |
| satfold-seq | 6418ns | 5615ns | 4538ns | 5883ns | 9903ns | +15.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5486ns | 4320ns | 7374ns | base | 5.973 |
| satfold-lanes16 | 5812ns | 4410ns | 8538ns | +5.94% | 5.638 |
| satfold-lanes16-constl | 5448ns | 4314ns | 6989ns | -0.69% | 6.014 |
| satfold-lanes4-idx | 5697ns | 5170ns | 7595ns | +3.84% | 5.752 |
| satfold-lanes64 | 5495ns | 5196ns | 6249ns | +0.15% | 5.964 |
| satfold-neon | 5278ns | 4278ns | 7008ns | -3.79% | 6.208 |
| satfold-neon8 | 5722ns | 5279ns | 7024ns | +4.29% | 5.727 |
| satfold-nolaw | 6042ns | 5376ns | 8346ns | +10.13% | 5.423 |
| satfold-seq | 6325ns | 4480ns | 9727ns | +15.28% | 5.181 |

## Performance model

- Peak throughput: **7.660 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.216 | 81.2% |
| satfold-lanes16 | 6.028 | 78.7% |
| satfold-lanes16-constl | 6.113 | 79.8% |
| satfold-lanes4-idx | 6.261 | 81.7% |
| satfold-lanes64 | 6.142 | 80.2% |
| satfold-neon | 6.259 | 81.7% |
| satfold-neon8 | 6.027 | 78.7% |
| satfold-nolaw | 6.058 | 79.1% |
| satfold-seq | 5.932 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5570ns | 5570ns | base |
| satfold-lanes16 | 5901ns | 5901ns | +5.93% |
| satfold-lanes16-constl | 5533ns | 5533ns | -0.67% |
| satfold-lanes4-idx | 5789ns | 5789ns | +3.92% |
| satfold-lanes64 | 5581ns | 5581ns | +0.20% |
| satfold-neon | 5358ns | 5358ns | -3.81% |
| satfold-neon8 | 5809ns | 5809ns | +4.29% |
| satfold-nolaw | 6129ns | 6129ns | +10.03% |
| satfold-seq | 6418ns | 6418ns | +15.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5271ns | base | --- | [5243, 5294] | --- | --- | --- | --- |
| satfold-lanes16 | 5436ns | +147.9ns (+2.8%) | [+23, +269]ns | [5364, 5545] | YES | 0.0442 | 0.0166 | 0 |
| satfold-lanes16-constl | 5360ns | no significant difference | [-86, +166]ns | [5276, 5436] | no | 0.3065 | 0.2682 | 0 |
| satfold-lanes4-idx | 5234ns | no significant difference | [-66, +112]ns | [5222, 5249] | no | 0.4296 | 0.4296 | 0 |
| satfold-lanes64 | 5335ns | no significant difference | [-14, +153]ns | [5274, 5394] | no | 0.2462 | 0.1539 | 0 |
| satfold-neon | 5236ns | no significant difference | [-40, +126]ns | [4546, 5403] | no | 0.3065 | 0.2682 | 0 |
| satfold-neon8 | 5437ns | +196.8ns (+3.7%) | [+72, +269]ns | [5360, 5513] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 5409ns | +295.8ns (+5.6%) | [+187, +438]ns | [5393, 5644] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 5524ns | +684.9ns (+13.0%) | [+64, +1351]ns | [5406, 5748] | YES (adj: no) | 0.0770 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5301ns | -16.1% | +2.6% | -1.4% | +0.6% | +2.5% | +0.5% | +4.7% | +67.3% |
| 1 | 2 | 5286ns | +2.3% | -18.2% | -1.2% | +1.1% | -18.1% | +3.7% | +2.0% | -14.9% |
| 1 | 3 | 4338ns | +28.1% | +23.7% | +20.7% | +23.6% | +3.9% | +25.6% | +24.2% | +29.0% |
| 1 | 4 | 5276ns | +3.4% | +1.9% | -0.4% | +0.3% | +2.1% | +2.8% | +7.4% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.398 | moderate- |
| satfold-lanes16 | -0.000 | ok |
| satfold-lanes16-constl | -0.445 | moderate- |
| satfold-lanes4-idx | 0.136 | ok |
| satfold-lanes64 | -0.352 | moderate- |
| satfold-neon | -0.258 | moderate- |
| satfold-neon8 | -0.286 | moderate- |
| satfold-nolaw | -0.223 | moderate- |
| satfold-seq | -0.300 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 3/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 3/4, lost 1/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 25.2ns | 5486.4ns | 0.5% |  |
| satfold-lanes16 | 26.2ns | 5812.1ns | 0.5% |  |
| satfold-lanes16-constl | 24.4ns | 5448.4ns | 0.4% |  |
| satfold-lanes4-idx | 26.5ns | 5696.8ns | 0.5% |  |
| satfold-lanes64 | 25.0ns | 5494.6ns | 0.5% |  |
| satfold-neon | 23.9ns | 5278.3ns | 0.5% |  |
| satfold-neon8 | 25.1ns | 5721.9ns | 0.4% |  |
| satfold-nolaw | 26.6ns | 6042.3ns | 0.4% |  |
| satfold-seq | 27.9ns | 6324.9ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4320.0-7374.1 ns)
   4320.0 |#######
   4472.8 |##
   4625.5 |
   4778.2 |
   4930.9 |
   5083.6 |###########
   5236.3 |########################################
   5389.0 |##############
   5541.7 |
   5694.4 |
   5847.1 |
   5999.8 |
   6152.5 |
   6305.2 |
   6457.9 |
   6610.6 |
   6763.3 |
   6916.0 |
   7068.7 |
   7221.4 |
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 4410.1-8538.1 ns)
   4410.1 |########################
   4616.5 |
   4822.9 |
   5029.3 |
   5235.7 |#################################
   5442.1 |########################################
   5648.5 |######
   5854.9 |
   6061.3 |
   6267.7 |
   6474.1 |
   6680.5 |
   6886.9 |
   7093.3 |
   7299.7 |
   7506.1 |
   7712.5 |
   7918.9 |
   8125.3 |######
   8331.7 |
  (2 below, 2 above range)

satfold-lanes16-constl (n=40, range 4314.2-6989.1 ns)
   4314.2 |##################
   4447.9 |
   4581.7 |
   4715.4 |
   4849.2 |
   4982.9 |
   5116.6 |##################
   5250.4 |####################################
   5384.1 |########################################
   5517.9 |###
   5651.6 |
   5785.4 |
   5919.1 |
   6052.9 |
   6186.6 |
   6320.3 |
   6454.1 |###
   6587.8 |
   6721.6 |
   6855.3 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 5169.9-7595.4 ns)
   5169.9 |########################################
   5291.2 |###
   5412.5 |###
   5533.7 |
   5655.0 |
   5776.3 |#
   5897.6 |
   6018.8 |
   6140.1 |
   6261.4 |
   6382.6 |
   6503.9 |
   6625.2 |
   6746.5 |
   6867.7 |
   6989.0 |
   7110.3 |
   7231.5 |
   7352.8 |#
   7474.1 |
  (6 below, 3 above range)

satfold-lanes64 (n=40, range 5196.3-6249.0 ns)
   5196.3 |#################
   5248.9 |########################################
   5301.6 |######################
   5354.2 |###################################
   5406.8 |########
   5459.5 |########
   5512.1 |########
   5564.7 |
   5617.4 |
   5670.0 |
   5722.7 |
   5775.3 |
   5827.9 |####
   5880.6 |
   5933.2 |
   5985.8 |
   6038.5 |
   6091.1 |
   6143.7 |
   6196.4 |
  (3 below, 4 above range)

satfold-neon (n=40, range 4278.0-7008.4 ns)
   4278.0 |##############################
   4414.5 |#########################
   4551.0 |###############
   4687.5 |
   4824.1 |
   4960.6 |
   5097.1 |###############
   5233.6 |####################
   5370.1 |########################################
   5506.7 |##########
   5643.2 |#####
   5779.7 |#####
   5916.2 |
   6052.8 |
   6189.3 |
   6325.8 |
   6462.3 |#####
   6598.9 |
   6735.4 |#####
   6871.9 |
  (3 below, 2 above range)

satfold-neon8 (n=40, range 5279.4-7024.1 ns)
   5279.4 |########################################
   5366.7 |################################
   5453.9 |#########################
   5541.1 |##################
   5628.4 |###
   5715.6 |###
   5802.9 |###
   5890.1 |
   5977.3 |
   6064.6 |
   6151.8 |
   6239.0 |
   6326.3 |
   6413.5 |
   6500.7 |
   6588.0 |
   6675.2 |
   6762.4 |
   6849.7 |
   6936.9 |
  (3 below, 2 above range)

satfold-nolaw (n=40, range 5375.9-8346.3 ns)
   5375.9 |########################################
   5524.4 |############
   5673.0 |##############
   5821.5 |
   5970.0 |
   6118.5 |
   6267.0 |
   6415.6 |
   6564.1 |
   6712.6 |
   6861.1 |
   7009.6 |
   7158.2 |
   7306.7 |##
   7455.2 |
   7603.7 |
   7752.2 |
   7900.8 |
   8049.3 |##
   8197.8 |
  (4 below, 2 above range)

satfold-seq (n=40, range 4479.8-9726.7 ns)
   4479.8 |#############
   4742.1 |
   5004.5 |
   5266.8 |########################################
   5529.2 |####################
   5791.5 |
   6053.9 |
   6316.2 |######
   6578.6 |
   6840.9 |###
   7103.3 |
   7365.6 |###
   7628.0 |
   7890.3 |
   8152.7 |###
   8415.0 |
   8677.4 |###
   8939.7 |###
   9202.1 |###
   9464.4 |##########
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=30.5% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=37.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=22.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=28.0% (high variance, measurements may be unstable)
- **satfold-neon**: CV=26.9% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=26.4% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=32.7% (high variance, measurements may be unstable)
- **satfold-seq**: CV=30.4% (high variance, measurements may be unstable)
