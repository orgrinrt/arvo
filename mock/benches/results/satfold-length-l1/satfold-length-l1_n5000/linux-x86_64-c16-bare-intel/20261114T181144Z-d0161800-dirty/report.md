# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-neon, satfold-neon8) are a dead heat (<1%)

satfold-neon (4.94 us) and satfold-neon8 (4.96 us) differ by 0.24%, inside the noise, even though the wider field spreads 21.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.76)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.06 us) is smaller than the fastest variant's own run-to-run std-dev (1.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-seq is inconsistent: worst-20% is 1.8x its best-20%

satfold-seq's best 20% of batches run at 4.92 us but its worst 20% at 8.93 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-neon's edge over baseline is significant but tiny (-26 ns, 0.51%)

satfold-neon differs from baseline satfold-iterfold by -26 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon** at 4944.1 ns median (-1.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.21x (fastest 4944.1 ns, slowest 6003.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5447ns | 5099ns | 4997ns | 5106ns | 6920ns | base |
| satfold-lanes16 | 6074ns | 5699ns | 5675ns | 5704ns | 7581ns | +11.50% |
| satfold-lanes16-constl | 5901ns | 5698ns | 5677ns | 5699ns | 6733ns | +8.33% |
| satfold-lanes4-idx | 6262ns | 6107ns | 5851ns | 6138ns | 7044ns | +14.95% |
| satfold-lanes64 | 5583ns | 5046ns | 5015ns | 5059ns | 7720ns | +2.49% |
| satfold-neon | 5462ns | 5017ns | 4998ns | 5018ns | 7256ns | +0.27% |
| satfold-neon8 | 5590ns | 5030ns | 5002ns | 5036ns | 7842ns | +2.63% |
| satfold-nolaw | 5868ns | 5043ns | 5022ns | 5056ns | 9153ns | +7.73% |
| satfold-seq | 5895ns | 5124ns | 4989ns | 5114ns | 9145ns | +8.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5360ns | 4926ns | 6770ns | base | 6.114 |
| satfold-lanes16 | 5987ns | 5602ns | 7447ns | +11.71% | 5.473 |
| satfold-lanes16-constl | 5818ns | 5606ns | 6607ns | +8.54% | 5.633 |
| satfold-lanes4-idx | 6181ns | 5776ns | 6944ns | +15.32% | 5.302 |
| satfold-lanes64 | 5491ns | 4943ns | 7561ns | +2.45% | 5.967 |
| satfold-neon | 5370ns | 4926ns | 7088ns | +0.20% | 6.101 |
| satfold-neon8 | 5497ns | 4930ns | 7661ns | +2.55% | 5.961 |
| satfold-nolaw | 5773ns | 4951ns | 8967ns | +7.72% | 5.676 |
| satfold-seq | 5794ns | 4917ns | 8929ns | +8.10% | 5.656 |

## Performance model

- Peak throughput: **6.664 Gops/s** (satfold-seq; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.518 | 97.8% |
| satfold-lanes16 | 5.823 | 87.4% |
| satfold-lanes16-constl | 5.825 | 87.4% |
| satfold-lanes4-idx | 5.459 | 81.9% |
| satfold-lanes64 | 6.590 | 98.9% |
| satfold-neon | 6.628 | 99.4% |
| satfold-neon8 | 6.611 | 99.2% |
| satfold-nolaw | 6.590 | 98.9% |
| satfold-seq | 6.489 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5447ns | 5447ns | base |
| satfold-lanes16 | 6074ns | 6074ns | +11.50% |
| satfold-lanes16-constl | 5901ns | 5901ns | +8.33% |
| satfold-lanes4-idx | 6262ns | 6262ns | +14.95% |
| satfold-lanes64 | 5583ns | 5583ns | +2.49% |
| satfold-neon | 5462ns | 5462ns | +0.27% |
| satfold-neon8 | 5590ns | 5590ns | +2.63% |
| satfold-nolaw | 5868ns | 5868ns | +7.73% |
| satfold-seq | 5895ns | 5895ns | +8.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5027ns | base | --- | [4963, 5122] | --- | --- | --- | --- |
| satfold-lanes16 | 5627ns | +621.7ns (+12.4%) | [+552, +675]ns | [5620, 5640] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5625ns | +606.9ns (+12.1%) | [+542, +670]ns | [5618, 5633] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 6003ns | +967.7ns (+19.2%) | [+815, +1122]ns | [5948, 6164] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4972ns | no significant difference | [-85, +27]ns | [4963, 4989] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon | 4944ns | -25.5ns (-0.5%) | [-90, -1]ns | [4940, 4947] | YES | 0.0332 | 0.0166 | 0 |
| satfold-neon8 | 4956ns | no significant difference | [-67, +14]ns | [4950, 4970] | no | 0.5728 | 0.4296 | 0 |
| satfold-nolaw | 4972ns | no significant difference | [-78, +31]ns | [4963, 4996] | no | 1.0000 | 1.0000 | 0 |
| satfold-seq | 5050ns | no significant difference | [-35, +86]ns | [5006, 5072] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5009ns | +12.7% | +12.4% | +26.3% | -0.9% | -1.3% | -0.8% | -0.9% | -0.3% |
| 1 | 2 | 5031ns | +11.7% | +11.7% | +19.6% | -1.2% | -1.8% | -1.7% | -1.4% | +0.8% |
| 1 | 3 | 5029ns | +12.3% | +11.7% | +18.3% | -0.2% | -1.7% | -1.2% | -1.2% | +0.1% |
| 1 | 4 | 5037ns | +11.4% | +11.7% | +18.2% | -1.3% | -1.7% | -1.7% | +0.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.076 | ok |
| satfold-lanes16 | -0.679 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.164 | ok |
| satfold-lanes4-idx | 0.057 | ok |
| satfold-lanes64 | -0.340 | moderate- |
| satfold-neon | 0.129 | ok |
| satfold-neon8 | -0.765 | HIGH- (thermal bounce) |
| satfold-nolaw | 0.007 | ok |
| satfold-seq | -0.488 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 3/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 25.3ns | 5359.8ns | 0.5% |  |
| satfold-lanes16 | 25.6ns | 5987.4ns | 0.4% |  |
| satfold-lanes16-constl | 24.7ns | 5817.6ns | 0.4% |  |
| satfold-lanes4-idx | 24.6ns | 6180.8ns | 0.4% |  |
| satfold-lanes64 | 26.3ns | 5491.2ns | 0.5% |  |
| satfold-neon | 25.9ns | 5370.5ns | 0.5% |  |
| satfold-neon8 | 26.3ns | 5496.6ns | 0.5% |  |
| satfold-nolaw | 27.8ns | 5773.3ns | 0.5% |  |
| satfold-seq | 28.0ns | 5793.7ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4925.9-6770.1 ns)
   4925.9 |########################################
   5018.1 |################
   5110.3 |##########################
   5202.5 |
   5294.7 |##
   5386.9 |
   5479.1 |
   5571.3 |
   5663.6 |
   5755.8 |
   5848.0 |
   5940.2 |
   6032.4 |
   6124.6 |
   6216.8 |
   6309.0 |
   6401.2 |
   6493.4 |
   6585.6 |##
   6677.9 |##
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 5601.9-7446.7 ns)
   5601.9 |########################################
   5694.1 |##
   5786.3 |
   5878.6 |
   5970.8 |#
   6063.1 |
   6155.3 |
   6247.5 |
   6339.8 |
   6432.0 |
   6524.3 |
   6616.5 |
   6708.7 |
   6801.0 |
   6893.2 |
   6985.5 |
   7077.7 |
   7169.9 |
   7262.2 |
   7354.4 |
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 5605.7-6606.6 ns)
   5605.7 |########################################
   5655.8 |
   5705.8 |
   5755.9 |
   5805.9 |
   5856.0 |
   5906.0 |
   5956.1 |
   6006.1 |
   6056.1 |
   6106.2 |
   6156.2 |
   6206.3 |
   6256.3 |
   6306.4 |
   6356.4 |
   6406.5 |
   6456.5 |
   6506.5 |
   6556.6 |
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 5776.5-6943.7 ns)
   5776.5 |###########
   5834.8 |######################
   5893.2 |############################
   5951.6 |########################################
   6009.9 |###########
   6068.3 |
   6126.6 |############################
   6185.0 |#####
   6243.4 |
   6301.7 |########################################
   6360.1 |###########
   6418.4 |
   6476.8 |
   6535.1 |
   6593.5 |
   6651.9 |
   6710.2 |
   6768.6 |
   6826.9 |
   6885.3 |
  (3 below, 2 above range)

satfold-lanes64 (n=40, range 4943.2-7561.4 ns)
   4943.2 |########################################
   5074.1 |#####
   5205.0 |
   5335.9 |
   5466.9 |
   5597.8 |
   5728.7 |
   5859.6 |
   5990.5 |
   6121.4 |
   6252.3 |
   6383.2 |
   6514.1 |
   6645.0 |
   6775.9 |
   6906.8 |
   7037.7 |
   7168.6 |
   7299.5 |
   7430.5 |#
  (4 below, 3 above range)

satfold-neon (n=40, range 4926.4-7088.2 ns)
   4926.4 |########################################
   5034.5 |#
   5142.6 |##
   5250.7 |
   5358.8 |
   5466.9 |
   5575.0 |
   5683.1 |
   5791.2 |
   5899.3 |
   6007.3 |
   6115.4 |
   6223.5 |
   6331.6 |
   6439.7 |
   6547.8 |
   6655.9 |
   6764.0 |
   6872.1 |
   6980.2 |
  (5 below, 4 above range)

satfold-neon8 (n=40, range 4929.6-7661.0 ns)
   4929.6 |########################################
   5066.2 |####
   5202.7 |#
   5339.3 |
   5475.9 |
   5612.4 |
   5749.0 |
   5885.6 |
   6022.1 |
   6158.7 |
   6295.3 |
   6431.8 |
   6568.4 |
   6705.0 |
   6841.5 |
   6978.1 |
   7114.7 |
   7251.2 |
   7387.8 |
   7524.4 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 4950.8-8967.0 ns)
   4950.8 |########################################
   5151.6 |
   5352.4 |
   5553.2 |
   5754.0 |
   5954.8 |
   6155.6 |
   6356.5 |
   6557.3 |
   6758.1 |
   6958.9 |#
   7159.7 |
   7360.5 |
   7561.3 |##
   7762.1 |
   7963.0 |
   8163.8 |
   8364.6 |
   8565.4 |
   8766.2 |
  (4 below, 3 above range)

satfold-seq (n=40, range 4916.9-8929.1 ns)
   4916.9 |########################################
   5117.5 |####
   5318.1 |#
   5518.8 |#
   5719.4 |
   5920.0 |
   6120.6 |
   6321.2 |
   6521.8 |
   6722.4 |
   6923.0 |
   7123.6 |
   7324.3 |
   7524.9 |
   7725.5 |
   7926.1 |
   8126.7 |
   8327.3 |
   8527.9 |
   8728.5 |
  (6 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=20.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=26.5% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=31.6% (high variance, measurements may be unstable)
- **satfold-neon**: CV=23.6% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=29.1% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=39.1% (high variance, measurements may be unstable)
- **satfold-seq**: CV=39.5% (high variance, measurements may be unstable)
