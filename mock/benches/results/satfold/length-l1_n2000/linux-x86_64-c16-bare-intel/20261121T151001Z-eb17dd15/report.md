# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.74)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.12 us) is smaller than the fastest variant's own run-to-run std-dev (1.55 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-lanes4-idx is inconsistent: worst-20% is 2.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 4.22 us but its worst 20% at 8.45 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4353.5 ns median (-14.2% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.26x (fastest 4353.5 ns, slowest 5473.8 ns)

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
| satfold-iterfold | 5390ns | 5143ns | 4256ns | 5030ns | 7604ns | base |
| satfold-lanes16 | 5601ns | 5549ns | 4500ns | 5490ns | 7033ns | +3.91% |
| satfold-lanes16-constl | 5453ns | 5332ns | 4207ns | 5262ns | 7274ns | +1.16% |
| satfold-lanes4-idx | 5558ns | 5304ns | 4275ns | 4964ns | 8619ns | +3.10% |
| satfold-lanes64 | 5970ns | 5460ns | 4510ns | 5455ns | 8974ns | +10.75% |
| satfold-neon | 5333ns | 4513ns | 4371ns | 4665ns | 8296ns | -1.07% |
| satfold-neon8 | 5529ns | 5504ns | 4568ns | 5189ns | 7508ns | +2.56% |
| satfold-nolaw | 5079ns | 4411ns | 4298ns | 4640ns | 7175ns | -5.78% |
| satfold-seq | 5369ns | 5233ns | 4451ns | 5207ns | 6772ns | -0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5308ns | 4199ns | 7472ns | base | 6.173 |
| satfold-lanes16 | 5521ns | 4441ns | 6909ns | +4.00% | 5.935 |
| satfold-lanes16-constl | 5372ns | 4149ns | 7144ns | +1.19% | 6.100 |
| satfold-lanes4-idx | 5473ns | 4217ns | 8455ns | +3.11% | 5.987 |
| satfold-lanes64 | 5883ns | 4452ns | 8809ns | +10.83% | 5.570 |
| satfold-neon | 5253ns | 4313ns | 8137ns | -1.04% | 6.238 |
| satfold-neon8 | 5451ns | 4510ns | 7381ns | +2.69% | 6.011 |
| satfold-nolaw | 5001ns | 4239ns | 7036ns | -5.79% | 6.553 |
| satfold-seq | 5289ns | 4383ns | 6660ns | -0.37% | 6.196 |

## Performance model

- Peak throughput: **7.898 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.458 | 81.8% |
| satfold-lanes16 | 5.986 | 75.8% |
| satfold-lanes16-constl | 6.229 | 78.9% |
| satfold-lanes4-idx | 6.263 | 79.3% |
| satfold-lanes64 | 6.079 | 77.0% |
| satfold-neon | 7.356 | 93.1% |
| satfold-neon8 | 6.030 | 76.3% |
| satfold-nolaw | 7.527 | 95.3% |
| satfold-seq | 6.347 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5390ns | 5390ns | base |
| satfold-lanes16 | 5601ns | 5601ns | +3.91% |
| satfold-lanes16-constl | 5453ns | 5453ns | +1.16% |
| satfold-lanes4-idx | 5558ns | 5558ns | +3.10% |
| satfold-lanes64 | 5970ns | 5970ns | +10.75% |
| satfold-neon | 5333ns | 5333ns | -1.07% |
| satfold-neon8 | 5529ns | 5529ns | +2.56% |
| satfold-nolaw | 5079ns | 5079ns | -5.78% |
| satfold-seq | 5369ns | 5369ns | -0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5074ns | base | --- | [4356, 5286] | --- | --- | --- | --- |
| satfold-lanes16 | 5474ns | +427.7ns (+8.4%) | [+218, +1150]ns | [5414, 5495] | YES | 0.0257 | 0.0064 | 0 |
| satfold-lanes16-constl | 5260ns | no significant difference | [-542, +783]ns | [5009, 5412] | no | 1.0000 | 1.0000 | 0 |
| satfold-lanes4-idx | 5232ns | -182.7ns (-3.6%) | [-690, -108]ns | [4310, 5260] | YES (adj: no) | 0.0616 | 0.0385 | 0 |
| satfold-lanes64 | 5390ns | +160.8ns (+3.2%) | [+45, +597]ns | [5378, 5459] | YES (adj: no) | 0.0616 | 0.0385 | 0 |
| satfold-neon | 4455ns | -664.8ns (-13.1%) | [-849, -18]ns | [4362, 4531] | YES (adj: no) | 0.0616 | 0.0385 | 0 |
| satfold-neon8 | 5434ns | +278.7ns (+5.5%) | [+167, +335]ns | [4666, 5457] | YES | 0.0054 | 0.0007 | 0 |
| satfold-nolaw | 4353ns | no significant difference | [-240, +86]ns | [4330, 4858] | no | 0.5728 | 0.4296 | 0 |
| satfold-seq | 5163ns | no significant difference | [-162, +403]ns | [4976, 5269] | no | 0.7267 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4363ns | +26.6% | +22.2% | -3.3% | +2.2% | -0.9% | +7.0% | -3.0% | +1.1% |
| 1 | 2 | 4203ns | +28.8% | +19.2% | +26.2% | +28.1% | +26.4% | +9.0% | +3.6% | +25.2% |
| 1 | 3 | 5156ns | +6.5% | -19.4% | -16.4% | +9.0% | -15.4% | +5.6% | +1.6% | +1.6% |
| 1 | 4 | 5919ns | -24.9% | -8.4% | -11.2% | -9.0% | -24.4% | -7.5% | -27.0% | -10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.246 | moderate+ |
| satfold-lanes16 | -0.124 | ok |
| satfold-lanes16-constl | -0.379 | moderate- |
| satfold-lanes4-idx | -0.737 | HIGH- (thermal bounce) |
| satfold-lanes64 | 0.015 | ok |
| satfold-neon | -0.529 | HIGH- (thermal bounce) |
| satfold-neon8 | 0.230 | moderate+ |
| satfold-nolaw | -0.337 | moderate- |
| satfold-seq | -0.092 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 3/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 3/4, lost 1/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.8ns | 5308.4ns | 0.4% |  |
| satfold-lanes16 | 25.2ns | 5520.8ns | 0.5% |  |
| satfold-lanes16-constl | 24.2ns | 5371.7ns | 0.4% |  |
| satfold-lanes4-idx | 24.6ns | 5473.3ns | 0.4% |  |
| satfold-lanes64 | 25.9ns | 5883.4ns | 0.4% |  |
| satfold-neon | 23.5ns | 5253.2ns | 0.4% |  |
| satfold-neon8 | 23.4ns | 5451.3ns | 0.4% |  |
| satfold-nolaw | 22.7ns | 5000.8ns | 0.5% |  |
| satfold-seq | 24.4ns | 5288.6ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4199.1-7472.0 ns)
   4199.1 |########################################
   4362.8 |########
   4526.4 |
   4690.1 |
   4853.7 |
   5017.3 |####################
   5181.0 |####################
   5344.6 |############
   5508.3 |
   5671.9 |
   5835.6 |####################
   5999.2 |########
   6162.8 |
   6326.5 |####
   6490.1 |
   6653.8 |
   6817.4 |
   6981.1 |
   7144.7 |
   7308.3 |
  (5 below, 2 above range)

satfold-lanes16 (n=40, range 4440.7-6909.4 ns)
   4440.7 |########
   4564.1 |
   4687.5 |
   4811.0 |
   4934.4 |
   5057.8 |
   5181.3 |
   5304.7 |###############
   5428.1 |########################################
   5551.6 |####
   5675.0 |##
   5798.4 |
   5921.9 |
   6045.3 |
   6168.7 |
   6292.2 |
   6415.6 |
   6539.0 |
   6662.5 |
   6785.9 |
  (5 below, 3 above range)

satfold-lanes16-constl (n=40, range 4148.7-7144.0 ns)
   4148.7 |################
   4298.5 |
   4448.3 |
   4598.0 |
   4747.8 |
   4897.6 |#######################
   5047.3 |##########
   5197.1 |##########
   5346.9 |########################################
   5496.6 |##########
   5646.4 |
   5796.1 |
   5945.9 |
   6095.7 |
   6245.4 |
   6395.2 |
   6545.0 |
   6694.7 |
   6844.5 |
   6994.3 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 4216.6-8454.6 ns)
   4216.6 |########################################
   4428.5 |#######
   4640.4 |
   4852.3 |###
   5064.2 |########################################
   5276.1 |#####################
   5488.0 |###
   5699.9 |
   5911.8 |
   6123.7 |
   6335.6 |
   6547.5 |
   6759.4 |
   6971.3 |
   7183.2 |
   7395.1 |
   7607.0 |###
   7818.9 |
   8030.8 |
   8242.7 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 4452.3-8809.0 ns)
   4452.3 |###########
   4670.2 |
   4888.0 |
   5105.8 |
   5323.7 |########################################
   5541.5 |#########
   5759.3 |
   5977.2 |
   6195.0 |
   6412.8 |#
   6630.7 |
   6848.5 |
   7066.3 |
   7284.2 |
   7502.0 |#
   7719.8 |
   7937.7 |
   8155.5 |
   8373.3 |
   8591.2 |
  (3 below, 3 above range)

satfold-neon (n=40, range 4313.3-8137.4 ns)
   4313.3 |########################################
   4504.5 |#####
   4695.7 |
   4886.9 |
   5078.1 |###
   5269.3 |#############
   5460.5 |
   5651.7 |
   5843.0 |
   6034.2 |
   6225.4 |
   6416.6 |
   6607.8 |
   6799.0 |
   6990.2 |
   7181.4 |
   7372.6 |
   7563.8 |
   7755.0 |
   7946.2 |#
  (3 below, 3 above range)

satfold-neon8 (n=40, range 4510.2-7380.6 ns)
   4510.2 |#####################
   4653.7 |################
   4797.3 |
   4940.8 |
   5084.3 |
   5227.8 |
   5371.3 |########################################
   5514.9 |
   5658.4 |##
   5801.9 |
   5945.4 |#####
   6088.9 |
   6232.4 |
   6376.0 |
   6519.5 |
   6663.0 |
   6806.5 |##
   6950.0 |
   7093.6 |
   7237.1 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 4239.1-7035.5 ns)
   4239.1 |########################################
   4379.0 |##########
   4518.8 |
   4658.6 |
   4798.4 |
   4938.2 |
   5078.1 |
   5217.9 |########################
   5357.7 |##
   5497.5 |
   5637.3 |
   5777.1 |
   5917.0 |
   6056.8 |##
   6196.6 |##
   6336.4 |
   6476.2 |
   6616.0 |
   6755.9 |
   6895.7 |
  (7 below, 2 above range)

satfold-seq (n=40, range 4383.2-6659.8 ns)
   4383.2 |#####
   4497.0 |
   4610.9 |###########
   4724.7 |#####
   4838.5 |#####
   4952.4 |########################################
   5066.2 |############################
   5180.0 |########################################
   5293.9 |##################################
   5407.7 |#################
   5521.5 |
   5635.3 |
   5749.2 |
   5863.0 |
   5976.8 |
   6090.7 |
   6204.5 |
   6318.3 |
   6432.2 |
   6546.0 |
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=30.8% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=20.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=27.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=40.9% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=37.5% (high variance, measurements may be unstable)
- **satfold-neon**: CV=38.7% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=26.5% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=30.9% (high variance, measurements may be unstable)
- **satfold-seq**: CV=23.5% (high variance, measurements may be unstable)
