# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw dominates: 21% faster than the next best (satfold-lanes64)

satfold-nolaw (4.07 us) leads satfold-lanes64 (4.93 us) by 21%, a clear separation rather than a photo finish. CV 63.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-nolaw is fastest but the noisiest (CV 63.0%)

satfold-nolaw wins on median (4.07 us) yet has the highest variance (CV 63.0%), while satfold-lanes4-idx is the steadiest (CV 25.7%, 5.79 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.73)

satfold-neon's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.73 us) is smaller than the fastest variant's own run-to-run std-dev (2.56 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-nolaw is inconsistent: worst-20% is 2.3x its best-20%

satfold-nolaw's best 20% of batches run at 4.05 us but its worst 20% at 9.35 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4065.6 ns median (-18.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.43x (fastest 4065.6 ns, slowest 5793.8 ns)

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
| satfold-iterfold | 5760ns | 5038ns | 4997ns | 5070ns | 8593ns | base |
| satfold-lanes16 | 6995ns | 5716ns | 5677ns | 5810ns | 11869ns | +21.45% |
| satfold-lanes16-constl | 5945ns | 5672ns | 4653ns | 5277ns | 9244ns | +3.22% |
| satfold-lanes4-idx | 5970ns | 5865ns | 4882ns | 5651ns | 8018ns | +3.65% |
| satfold-lanes64 | 5468ns | 5006ns | 4161ns | 5003ns | 8168ns | -5.08% |
| satfold-neon | 5237ns | 5027ns | 4115ns | 4723ns | 7899ns | -9.08% |
| satfold-neon8 | 5470ns | 5022ns | 4994ns | 5035ns | 7250ns | -5.04% |
| satfold-nolaw | 5205ns | 4124ns | 4110ns | 4127ns | 9532ns | -9.64% |
| satfold-seq | 5494ns | 5118ns | 4118ns | 5052ns | 8197ns | -4.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5667ns | 4926ns | 8414ns | base | 5.783 |
| satfold-lanes16 | 6899ns | 5604ns | 11696ns | +21.75% | 4.750 |
| satfold-lanes16-constl | 5859ns | 4594ns | 9071ns | +3.40% | 5.592 |
| satfold-lanes4-idx | 5891ns | 4823ns | 7878ns | +3.95% | 5.563 |
| satfold-lanes64 | 5381ns | 4102ns | 8012ns | -5.04% | 6.090 |
| satfold-neon | 5153ns | 4055ns | 7750ns | -9.06% | 6.359 |
| satfold-neon8 | 5380ns | 4922ns | 7089ns | -5.06% | 6.091 |
| satfold-nolaw | 5122ns | 4051ns | 9352ns | -9.62% | 6.398 |
| satfold-seq | 5406ns | 4059ns | 8033ns | -4.59% | 6.061 |

## Performance model

- Peak throughput: **8.088 Gops/s** (satfold-nolaw; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.598 | 81.6% |
| satfold-lanes16 | 5.808 | 71.8% |
| satfold-lanes16-constl | 5.850 | 72.3% |
| satfold-lanes4-idx | 5.656 | 69.9% |
| satfold-lanes64 | 6.641 | 82.1% |
| satfold-neon | 6.618 | 81.8% |
| satfold-neon8 | 6.620 | 81.9% |
| satfold-nolaw | 8.060 | 99.6% |
| satfold-seq | 6.501 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5760ns | 5760ns | base |
| satfold-lanes16 | 6995ns | 6995ns | +21.45% |
| satfold-lanes16-constl | 5945ns | 5945ns | +3.22% |
| satfold-lanes4-idx | 5970ns | 5970ns | +3.65% |
| satfold-lanes64 | 5468ns | 5468ns | -5.08% |
| satfold-neon | 5237ns | 5237ns | -9.08% |
| satfold-neon8 | 5470ns | 5470ns | -5.04% |
| satfold-nolaw | 5205ns | 5205ns | -9.64% |
| satfold-seq | 5494ns | 5494ns | -4.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4967ns | base | --- | [4944, 5052] | --- | --- | --- | --- |
| satfold-lanes16 | 5642ns | +649.1ns (+13.1%) | [+579, +702]ns | [5625, 5654] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5601ns | no significant difference | [-320, +617]ns | [4633, 5619] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes4-idx | 5794ns | no significant difference | [-69, +933]ns | [5057, 6086] | no | 0.4291 | 0.2682 | 0 |
| satfold-lanes64 | 4934ns | -84.7ns (-1.7%) | [-235, -31]ns | [4921, 4946] | YES | 0.0171 | 0.0064 | 0 |
| satfold-neon | 4951ns | no significant difference | [-828, +6]ns | [4154, 4963] | no | 0.3077 | 0.1539 | 0 |
| satfold-neon8 | 4950ns | no significant difference | [-75, +19]ns | [4939, 4979] | no | 0.4910 | 0.4296 | 0 |
| satfold-nolaw | 4066ns | -880.1ns (-17.7%) | [-901, -876]ns | [4059, 4074] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 5041ns | no significant difference | [-162, +35]ns | [4935, 5080] | no | 0.4910 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4938ns | +14.0% | -6.2% | +24.9% | -16.9% | -17.8% | +2.3% | -17.7% | -18.1% |
| 1 | 2 | 4937ns | +89.6% | +13.6% | +19.9% | +0.5% | +0.5% | +0.6% | -17.8% | +2.5% |
| 1 | 3 | 5082ns | +11.0% | +10.9% | -4.3% | -2.9% | -18.7% | -2.8% | -20.1% | +0.1% |
| 1 | 4 | 4993ns | +12.4% | -7.9% | +1.3% | -1.2% | -0.5% | -1.2% | -18.2% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.127 | ok |
| satfold-lanes16 | -0.412 | moderate- |
| satfold-lanes16-constl | -0.250 | moderate- |
| satfold-lanes4-idx | 0.242 | moderate+ |
| satfold-lanes64 | -0.110 | ok |
| satfold-neon | -0.726 | HIGH- (thermal bounce) |
| satfold-neon8 | 0.119 | ok |
| satfold-nolaw | -0.038 | ok |
| satfold-seq | -0.087 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 1/4, lost 3/4
- **satfold-lanes64**: won 3/4, lost 1/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 27.1ns | 5666.7ns | 0.5% |  |
| satfold-lanes16 | 29.6ns | 6899.2ns | 0.4% |  |
| satfold-lanes16-constl | 25.3ns | 5859.4ns | 0.4% |  |
| satfold-lanes4-idx | 23.7ns | 5890.8ns | 0.4% |  |
| satfold-lanes64 | 25.9ns | 5381.0ns | 0.5% |  |
| satfold-neon | 24.8ns | 5153.2ns | 0.5% |  |
| satfold-neon8 | 26.1ns | 5380.1ns | 0.5% |  |
| satfold-nolaw | 24.6ns | 5121.7ns | 0.5% |  |
| satfold-seq | 25.8ns | 5406.4ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4925.7-8413.8 ns)
   4925.7 |########################################
   5100.1 |####
   5274.5 |##
   5448.9 |
   5623.3 |
   5797.7 |
   5972.1 |
   6146.5 |
   6320.9 |
   6495.3 |
   6669.8 |
   6844.2 |
   7018.6 |
   7193.0 |
   7367.4 |
   7541.8 |
   7716.2 |
   7890.6 |
   8065.0 |
   8239.4 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 5604.4-11695.6 ns)
   5604.4 |########################################
   5909.0 |
   6213.5 |
   6518.1 |###
   6822.6 |
   7127.2 |
   7431.7 |#
   7736.3 |
   8040.9 |
   8345.4 |#
   8650.0 |
   8954.5 |
   9259.1 |
   9563.7 |
   9868.2 |#
  10172.8 |#
  10477.3 |
  10781.9 |
  11086.4 |
  11391.0 |
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 4594.2-9071.4 ns)
   4594.2 |################################
   4818.0 |
   5041.9 |
   5265.7 |
   5489.6 |########################################
   5713.5 |##
   5937.3 |
   6161.2 |
   6385.0 |
   6608.9 |
   6832.8 |##
   7056.6 |
   7280.5 |
   7504.4 |
   7728.2 |
   7952.1 |
   8175.9 |
   8399.8 |
   8623.7 |
   8847.5 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 4822.7-7878.2 ns)
   4822.7 |########################################
   4975.5 |#############
   5128.3 |########
   5281.1 |
   5433.8 |####
   5586.6 |
   5739.4 |#################
   5892.1 |####
   6044.9 |###################################
   6197.7 |#################
   6350.4 |
   6503.2 |
   6656.0 |
   6808.8 |####
   6961.5 |
   7114.3 |
   7267.1 |
   7419.8 |
   7572.6 |
   7725.4 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 4102.2-8012.0 ns)
   4102.2 |####
   4297.7 |
   4493.2 |
   4688.7 |#
   4884.2 |########################################
   5079.6 |
   5275.1 |
   5470.6 |
   5666.1 |
   5861.6 |
   6057.1 |
   6252.6 |
   6448.1 |#
   6643.6 |
   6839.1 |
   7034.5 |
   7230.0 |
   7425.5 |
   7621.0 |
   7816.5 |
  (5 below, 3 above range)

satfold-neon (n=40, range 4055.3-7749.5 ns)
   4055.3 |##################################
   4240.0 |
   4424.7 |
   4609.4 |
   4794.1 |########################################
   4978.8 |##############
   5163.5 |
   5348.3 |
   5533.0 |##
   5717.7 |
   5902.4 |
   6087.1 |
   6271.8 |
   6456.5 |
   6641.3 |
   6826.0 |
   7010.7 |
   7195.4 |
   7380.1 |
   7564.8 |
  (5 below, 3 above range)

satfold-neon8 (n=40, range 4922.4-7089.0 ns)
   4922.4 |########################################
   5030.8 |#########
   5139.1 |
   5247.4 |
   5355.8 |
   5464.1 |
   5572.4 |
   5680.7 |
   5789.1 |
   5897.4 |
   6005.7 |
   6114.1 |
   6222.4 |
   6330.7 |
   6439.1 |
   6547.4 |
   6655.7 |
   6764.1 |
   6872.4 |
   6980.7 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 4051.3-9351.9 ns)
   4051.3 |########################################
   4316.3 |
   4581.4 |
   4846.4 |#
   5111.4 |
   5376.4 |
   5641.5 |
   5906.5 |
   6171.5 |
   6436.6 |
   6701.6 |#
   6966.6 |
   7231.6 |
   7496.7 |
   7761.7 |
   8026.7 |
   8291.8 |
   8556.8 |
   8821.8 |
   9086.8 |
  (3 below, 5 above range)

satfold-seq (n=40, range 4058.9-8033.0 ns)
   4058.9 |########
   4257.6 |
   4456.3 |
   4655.0 |
   4853.7 |##################################
   5052.4 |########################################
   5251.1 |
   5449.8 |##
   5648.5 |
   5847.2 |
   6045.9 |
   6244.6 |
   6443.3 |
   6642.0 |
   6840.7 |
   7039.4 |
   7238.1 |
   7436.9 |
   7635.6 |
   7834.3 |##
  (6 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=35.6% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=39.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=42.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=25.2% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=39.5% (high variance, measurements may be unstable)
- **satfold-neon**: CV=39.2% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=26.7% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=50.0% (high variance, measurements may be unstable)
- **satfold-seq**: CV=36.7% (high variance, measurements may be unstable)
