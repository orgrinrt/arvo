# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-nolaw, satfold-lanes64) are a dead heat (<1%)

satfold-nolaw (4.32 us) and satfold-lanes64 (4.32 us) differ by 0.09%, inside the noise, even though the wider field spreads 87.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.70)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-nolaw, satfold-lanes64, satfold-lanes16, satfold-iterfold, satfold-seq, satfold-neon, satfold-lanes16-constl} vs {satfold-lanes4-idx} (53% apart)

The field splits into a fast tier {satfold-nolaw, satfold-lanes64, satfold-lanes16, satfold-iterfold, satfold-seq, satfold-neon, satfold-lanes16-constl} and a slow tier {satfold-lanes4-idx} with a 53% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-lanes16 is inconsistent: worst-20% is 2.7x its best-20%

satfold-lanes16's best 20% of batches run at 4.13 us but its worst 20% at 11.11 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4320.5 ns median (-2.5% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.88x (fastest 4320.5 ns, slowest 8117.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112835759 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5508ns | 4490ns | 4250ns | 4595ns | 9504ns | base |
| satfold-lanes16 | 6047ns | 4405ns | 4193ns | 4914ns | 11299ns | +9.79% |
| satfold-lanes16-constl | 5494ns | 5372ns | 4293ns | 5057ns | 8006ns | -0.25% |
| satfold-lanes4-idx | 8075ns | 8209ns | 6993ns | 8101ns | 9082ns | +46.62% |
| satfold-lanes64 | 4931ns | 4383ns | 4200ns | 4487ns | 6993ns | -10.47% |
| satfold-neon | 5696ns | 5339ns | 4516ns | 5320ns | 8008ns | +3.43% |
| satfold-nolaw | 5222ns | 4378ns | 4343ns | 4593ns | 7987ns | -5.19% |
| satfold-seq | 5206ns | 5210ns | 4378ns | 4958ns | 6780ns | -5.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5425ns | 4192ns | 9330ns | base | 6.041 |
| satfold-lanes16 | 5956ns | 4135ns | 11113ns | +9.79% | 5.502 |
| satfold-lanes16-constl | 5411ns | 4235ns | 7847ns | -0.26% | 6.056 |
| satfold-lanes4-idx | 8004ns | 6936ns | 9004ns | +47.55% | 4.094 |
| satfold-lanes64 | 4855ns | 4142ns | 6857ns | -10.50% | 6.749 |
| satfold-neon | 5613ns | 4457ns | 7859ns | +3.46% | 5.838 |
| satfold-nolaw | 5142ns | 4284ns | 7832ns | -5.21% | 6.372 |
| satfold-seq | 5130ns | 4315ns | 6664ns | -5.43% | 6.388 |

## Performance model

- Peak throughput: **7.925 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 7.394 | 93.3% |
| satfold-lanes16 | 7.541 | 95.2% |
| satfold-lanes16-constl | 6.181 | 78.0% |
| satfold-lanes4-idx | 4.037 | 50.9% |
| satfold-lanes64 | 7.578 | 95.6% |
| satfold-neon | 6.222 | 78.5% |
| satfold-nolaw | 7.584 | 95.7% |
| satfold-seq | 6.378 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5508ns | 5508ns | base |
| satfold-lanes16 | 6047ns | 6047ns | +9.79% |
| satfold-lanes16-constl | 5494ns | 5494ns | -0.25% |
| satfold-lanes4-idx | 8075ns | 8075ns | +46.62% |
| satfold-lanes64 | 4931ns | 4931ns | -10.47% |
| satfold-neon | 5696ns | 5696ns | +3.43% |
| satfold-nolaw | 5222ns | 5222ns | -5.19% |
| satfold-seq | 5206ns | 5206ns | -5.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4432ns | base | --- | [4312, 4529] | --- | --- | --- | --- |
| satfold-lanes16 | 4345ns | no significant difference | [-99, +71]ns | [4255, 4500] | no | 0.7418 | 0.6358 | 0 |
| satfold-lanes16-constl | 5301ns | no significant difference | [-81, +851]ns | [4426, 5334] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes4-idx | 8117ns | +2839.4ns (+64.1%) | [+2691, +4559]ns | [7257, 8786] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4324ns | no significant difference | [-352, +37]ns | [4251, 4468] | no | 0.2693 | 0.1539 | 0 |
| satfold-neon | 5266ns | +188.4ns (+4.3%) | [+5, +1075]ns | [5183, 5305] | YES (adj: no) | 0.1883 | 0.0807 | 0 |
| satfold-nolaw | 4320ns | no significant difference | [-4, +108]ns | [4306, 4760] | no | 0.1883 | 0.0807 | 0 |
| satfold-seq | 5138ns | no significant difference | [-149, +953]ns | [4485, 5280] | no | 0.7418 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4488ns | +0.0% | +18.5% | +62.2% | -4.5% | +0.7% | +15.5% | -1.3% |
| 1 | 2 | 5304ns | +78.9% | -16.6% | +30.9% | -19.8% | -2.3% | -18.7% | -18.3% |
| 1 | 3 | 4194ns | +1.6% | +28.0% | +109.4% | +19.1% | +25.8% | +2.3% | +28.1% |
| 1 | 4 | 4299ns | -3.7% | -1.9% | +109.1% | -0.4% | +30.0% | +0.6% | +21.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.309 | moderate- |
| satfold-lanes16 | -0.373 | moderate- |
| satfold-lanes16-constl | -0.699 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | 0.219 | moderate+ |
| satfold-lanes64 | -0.428 | moderate- |
| satfold-neon | 0.068 | ok |
| satfold-nolaw | -0.075 | ok |
| satfold-seq | 0.175 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 2/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 3/4, lost 1/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-nolaw**: won 1/4, lost 3/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.6ns | 5424.7ns | 0.5% |  |
| satfold-lanes16 | 26.4ns | 5955.8ns | 0.4% |  |
| satfold-lanes16-constl | 24.2ns | 5410.6ns | 0.4% |  |
| satfold-lanes4-idx | 21.9ns | 8004.4ns | 0.3% |  |
| satfold-lanes64 | 22.5ns | 4854.9ns | 0.5% |  |
| satfold-neon | 25.2ns | 5612.6ns | 0.4% |  |
| satfold-nolaw | 23.5ns | 5142.3ns | 0.5% |  |
| satfold-seq | 23.3ns | 5130.0ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4191.5-9330.2 ns)
   4191.5 |########################################
   4448.4 |##############
   4705.4 |
   4962.3 |#########
   5219.2 |#########
   5476.2 |
   5733.1 |##
   5990.0 |
   6247.0 |
   6503.9 |
   6760.8 |
   7017.8 |
   7274.7 |
   7531.7 |
   7788.6 |
   8045.5 |
   8302.5 |
   8559.4 |
   8816.3 |
   9073.3 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 4134.6-11113.5 ns)
   4134.6 |########################################
   4483.6 |##########
   4832.5 |
   5181.5 |##
   5530.4 |
   5879.4 |##
   6228.3 |
   6577.2 |
   6926.2 |##
   7275.1 |##
   7624.1 |
   7973.0 |
   8321.9 |##
   8670.9 |##
   9019.8 |
   9368.8 |
   9717.7 |
  10066.6 |##
  10415.6 |##
  10764.5 |##
  (3 below, 4 above range)

satfold-lanes16-constl (n=40, range 4234.6-7847.0 ns)
   4234.6 |##############################
   4415.2 |#########################
   4595.9 |
   4776.5 |
   4957.1 |
   5137.7 |########################################
   5318.3 |#########################
   5498.9 |####################
   5679.6 |##########
   5860.2 |#####
   6040.8 |
   6221.4 |
   6402.0 |
   6582.7 |
   6763.3 |
   6943.9 |
   7124.5 |
   7305.1 |
   7485.8 |
   7666.4 |
  (6 below, 3 above range)

satfold-lanes4-idx (n=40, range 6935.9-9003.7 ns)
   6935.9 |############################
   7039.3 |
   7142.7 |#################
   7246.1 |############################
   7349.4 |#####
   7452.8 |#####
   7556.2 |
   7659.6 |
   7763.0 |
   7866.4 |
   7969.8 |
   8073.2 |
   8176.5 |
   8279.9 |
   8383.3 |
   8486.7 |
   8590.1 |
   8693.5 |########################################
   8796.9 |######################
   8900.3 |############################
  (5 below, 4 above range)

satfold-lanes64 (n=40, range 4142.0-6857.2 ns)
   4142.0 |########################################
   4277.8 |##########
   4413.5 |############
   4549.3 |
   4685.0 |
   4820.8 |##########
   4956.6 |#####
   5092.3 |#######
   5228.1 |
   5363.9 |
   5499.6 |
   5635.4 |
   5771.1 |
   5906.9 |
   6042.7 |##
   6178.4 |
   6314.2 |
   6450.0 |
   6585.7 |
   6721.5 |
  (3 below, 2 above range)

satfold-neon (n=40, range 4457.1-7859.4 ns)
   4457.1 |#######
   4627.2 |##
   4797.3 |##
   4967.4 |
   5137.5 |########################################
   5307.7 |###########
   5477.8 |###########
   5647.9 |
   5818.0 |
   5988.1 |
   6158.2 |
   6328.4 |
   6498.5 |##
   6668.6 |
   6838.7 |
   7008.8 |
   7178.9 |
   7349.0 |
   7519.2 |
   7689.3 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 4284.5-7831.6 ns)
   4284.5 |########################################
   4461.8 |
   4639.2 |#
   4816.5 |#
   4993.9 |###
   5171.2 |#############
   5348.6 |
   5526.0 |
   5703.3 |
   5880.7 |
   6058.0 |#
   6235.4 |
   6412.7 |
   6590.1 |
   6767.4 |
   6944.8 |
   7122.1 |
   7299.5 |
   7476.8 |
   7654.2 |
  (4 below, 3 above range)

satfold-seq (n=40, range 4314.5-6663.5 ns)
   4314.5 |##############################
   4432.0 |########################################
   4549.4 |
   4666.9 |
   4784.3 |
   4901.8 |
   5019.2 |#####
   5136.7 |#########################
   5254.1 |#########################
   5371.6 |########################################
   5489.0 |
   5606.5 |
   5723.9 |
   5841.4 |
   5958.8 |
   6076.3 |
   6193.7 |
   6311.2 |
   6428.6 |
   6546.1 |
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=49.2% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=46.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=34.9% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=33.2% (high variance, measurements may be unstable)
- **satfold-neon**: CV=30.6% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=39.5% (high variance, measurements may be unstable)
- **satfold-seq**: CV=25.5% (high variance, measurements may be unstable)
