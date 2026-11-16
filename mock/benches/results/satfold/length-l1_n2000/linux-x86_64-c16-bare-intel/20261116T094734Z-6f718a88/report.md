# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-iterfold, satfold-lanes16) are a dead heat (<1%)

satfold-iterfold (4.41 us) and satfold-lanes16 (4.43 us) differ by 0.44%, inside the noise, even though the wider field spreads 22.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.69)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.00 us) is smaller than the fastest variant's own run-to-run std-dev (1.50 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (4.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### satfold-neon is inconsistent: worst-20% is 2.5x its best-20%

satfold-neon's best 20% of batches run at 4.32 us but its worst 20% at 10.99 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 4408.2 ns median
- 5 variants significantly slower than baseline
- Spread: 1.23x (fastest 4408.2 ns, slowest 5411.1 ns)

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
| satfold-iterfold | 5005ns | 4466ns | 4313ns | 4593ns | 6934ns | base |
| satfold-lanes16 | 5073ns | 4496ns | 4364ns | 4656ns | 7032ns | +1.35% |
| satfold-lanes16-constl | 5539ns | 4569ns | 4349ns | 4570ns | 9636ns | +10.66% |
| satfold-lanes4-idx | 5518ns | 5252ns | 4318ns | 4966ns | 8373ns | +10.24% |
| satfold-lanes64 | 5471ns | 5277ns | 4429ns | 5246ns | 7189ns | +9.31% |
| satfold-neon | 6666ns | 5515ns | 4378ns | 5932ns | 11156ns | +33.18% |
| satfold-neon8 | 5514ns | 5364ns | 4487ns | 5287ns | 7220ns | +10.16% |
| satfold-nolaw | 5388ns | 4538ns | 4442ns | 4528ns | 8914ns | +7.64% |
| satfold-seq | 5623ns | 5458ns | 4639ns | 5455ns | 7112ns | +12.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 4931ns | 4254ns | 6818ns | base | 6.646 |
| satfold-lanes16 | 4998ns | 4305ns | 6908ns | +1.37% | 6.556 |
| satfold-lanes16-constl | 5458ns | 4290ns | 9467ns | +10.69% | 6.004 |
| satfold-lanes4-idx | 5434ns | 4259ns | 8215ns | +10.20% | 6.030 |
| satfold-lanes64 | 5390ns | 4370ns | 7064ns | +9.31% | 6.080 |
| satfold-neon | 6568ns | 4319ns | 10989ns | +33.20% | 4.989 |
| satfold-neon8 | 5432ns | 4424ns | 7091ns | +10.16% | 6.033 |
| satfold-nolaw | 5309ns | 4383ns | 8752ns | +7.66% | 6.173 |
| satfold-seq | 5543ns | 4580ns | 6988ns | +12.41% | 5.912 |

## Performance model

- Peak throughput: **7.702 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 7.433 | 96.5% |
| satfold-lanes16 | 7.401 | 96.1% |
| satfold-lanes16-constl | 7.273 | 94.4% |
| satfold-lanes4-idx | 6.326 | 82.1% |
| satfold-lanes64 | 6.301 | 81.8% |
| satfold-neon | 6.056 | 78.6% |
| satfold-neon8 | 6.193 | 80.4% |
| satfold-nolaw | 7.315 | 95.0% |
| satfold-seq | 6.084 | 79.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5005ns | 5005ns | base |
| satfold-lanes16 | 5073ns | 5073ns | +1.35% |
| satfold-lanes16-constl | 5539ns | 5539ns | +10.66% |
| satfold-lanes4-idx | 5518ns | 5518ns | +10.24% |
| satfold-lanes64 | 5471ns | 5471ns | +9.31% |
| satfold-neon | 6666ns | 6666ns | +33.18% |
| satfold-neon8 | 5514ns | 5514ns | +10.16% |
| satfold-nolaw | 5388ns | 5388ns | +7.64% |
| satfold-seq | 5623ns | 5623ns | +12.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4408ns | base | --- | [4350, 4523] | --- | --- | --- | --- |
| satfold-lanes16 | 4428ns | no significant difference | [-110, +320]ns | [4353, 4623] | no | 0.4296 | 0.4296 | 0 |
| satfold-lanes16-constl | 4505ns | +90.2ns (+2.0%) | [+27, +146]ns | [4451, 4531] | YES | 0.0265 | 0.0166 | 0 |
| satfold-lanes4-idx | 5180ns | no significant difference | [-41, +218]ns | [4395, 5245] | no | 0.4296 | 0.4296 | 0 |
| satfold-lanes64 | 5201ns | +513.4ns (+11.6%) | [+53, +860]ns | [5115, 5271] | YES | 0.0129 | 0.0064 | 0 |
| satfold-neon | 5411ns | +947.8ns (+21.5%) | [+105, +1203]ns | [4479, 5604] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 5292ns | +376.2ns (+8.5%) | [+94, +947]ns | [5170, 5350] | YES | 0.0059 | 0.0022 | 0 |
| satfold-nolaw | 4480ns | no significant difference | [-34, +98]ns | [4416, 4502] | no | 0.4296 | 0.4296 | 0 |
| satfold-seq | 5386ns | +713.7ns (+16.2%) | [+261, +1030]ns | [5368, 5455] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4391ns | -1.5% | +3.2% | -3.0% | +23.0% | +0.9% | +20.3% | +0.6% | +4.4% |
| 1 | 2 | 4398ns | +20.9% | +2.3% | +17.9% | +19.0% | +24.7% | +1.4% | +0.1% | +22.7% |
| 1 | 3 | 4257ns | +4.0% | +4.4% | +2.8% | +3.4% | +2.6% | +28.1% | +5.8% | +26.2% |
| 1 | 4 | 5353ns | -19.3% | -13.3% | -1.7% | -3.9% | +98.1% | -0.9% | -16.0% | +2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.191 | ok |
| satfold-lanes16 | -0.391 | moderate- |
| satfold-lanes16-constl | -0.343 | moderate- |
| satfold-lanes4-idx | -0.687 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.202 | moderate- |
| satfold-neon | -0.206 | moderate- |
| satfold-neon8 | -0.433 | moderate- |
| satfold-nolaw | 0.151 | ok |
| satfold-seq | -0.079 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 2/4, lost 2/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 1/4, lost 2/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.6ns | 4930.7ns | 0.5% |  |
| satfold-lanes16 | 22.9ns | 4998.3ns | 0.5% |  |
| satfold-lanes16-constl | 24.4ns | 5457.7ns | 0.4% |  |
| satfold-lanes4-idx | 24.7ns | 5433.9ns | 0.5% |  |
| satfold-lanes64 | 24.8ns | 5389.6ns | 0.5% |  |
| satfold-neon | 28.7ns | 6567.8ns | 0.4% |  |
| satfold-neon8 | 24.7ns | 5431.9ns | 0.5% |  |
| satfold-nolaw | 23.9ns | 5308.6ns | 0.5% |  |
| satfold-seq | 24.4ns | 5542.6ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4254.5-6817.5 ns)
   4254.5 |########################################
   4382.7 |########################
   4510.8 |#########
   4639.0 |
   4767.1 |
   4895.3 |
   5023.4 |
   5151.6 |############
   5279.7 |###############
   5407.9 |###
   5536.0 |
   5664.2 |
   5792.3 |
   5920.5 |
   6048.6 |
   6176.8 |
   6304.9 |
   6433.1 |
   6561.2 |
   6689.4 |
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 4305.2-6907.7 ns)
   4305.2 |########################################
   4435.4 |########
   4565.5 |######
   4695.6 |
   4825.7 |
   4955.9 |##
   5086.0 |
   5216.1 |####################
   5346.2 |##
   5476.3 |
   5606.5 |
   5736.6 |
   5866.7 |
   5996.8 |
   6126.9 |
   6257.1 |
   6387.2 |
   6517.3 |
   6647.4 |
   6777.6 |
  (2 below, 2 above range)

satfold-lanes16-constl (n=40, range 4289.6-9466.7 ns)
   4289.6 |########################################
   4548.5 |######
   4807.3 |#
   5066.2 |
   5325.0 |
   5583.9 |
   5842.7 |
   6101.6 |
   6360.4 |
   6619.3 |
   6878.2 |
   7137.0 |
   7395.9 |
   7654.7 |
   7913.6 |
   8172.4 |
   8431.3 |
   8690.1 |
   8949.0 |
   9207.8 |
  (4 below, 5 above range)

satfold-lanes4-idx (n=40, range 4258.6-8214.6 ns)
   4258.6 |########################################
   4456.4 |###
   4654.2 |
   4852.0 |
   5049.8 |###########################
   5247.6 |########################
   5445.4 |######
   5643.2 |
   5841.0 |
   6038.8 |
   6236.6 |
   6434.4 |
   6632.2 |
   6830.0 |
   7027.8 |
   7225.6 |###
   7423.4 |
   7621.2 |
   7819.0 |
   8016.8 |
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 4370.0-7064.3 ns)
   4370.0 |###
   4504.7 |##########
   4639.5 |
   4774.2 |
   4908.9 |#############
   5043.6 |##########
   5178.3 |########################################
   5313.0 |##########################
   5447.7 |######
   5582.4 |
   5717.2 |
   5851.9 |
   5986.6 |
   6121.3 |
   6256.0 |
   6390.7 |
   6525.4 |
   6660.1 |
   6794.9 |
   6929.6 |
  (5 below, 2 above range)

satfold-neon (n=40, range 4319.0-10988.8 ns)
   4319.0 |########################################
   4652.5 |
   4985.9 |
   5319.4 |#################################
   5652.9 |
   5986.4 |
   6319.9 |
   6653.4 |
   6986.9 |
   7320.4 |
   7653.9 |###
   7987.3 |
   8320.8 |
   8654.3 |
   8987.8 |
   9321.3 |
   9654.8 |
   9988.3 |
  10321.8 |###########################
  10655.3 |###
  (4 below, 1 above range)

satfold-neon8 (n=40, range 4424.4-7091.1 ns)
   4424.4 |#########################
   4557.7 |
   4691.1 |
   4824.4 |
   4957.7 |##########
   5091.1 |#######
   5224.4 |########################################
   5357.7 |#####################
   5491.1 |##############
   5624.4 |#######
   5757.7 |
   5891.1 |
   6024.4 |
   6157.7 |
   6291.1 |
   6424.4 |
   6557.7 |
   6691.1 |
   6824.4 |
   6957.7 |
  (3 below, 2 above range)

satfold-nolaw (n=40, range 4383.1-8752.4 ns)
   4383.1 |########################################
   4601.6 |
   4820.1 |
   5038.5 |
   5257.0 |
   5475.5 |
   5693.9 |
   5912.4 |
   6130.9 |
   6349.3 |
   6567.8 |
   6786.2 |#
   7004.7 |
   7223.2 |
   7441.6 |
   7660.1 |
   7878.6 |
   8097.0 |
   8315.5 |
   8534.0 |
  (4 below, 4 above range)

satfold-seq (n=40, range 4579.9-6987.8 ns)
   4579.9 |#####
   4700.3 |#######
   4820.7 |
   4941.1 |
   5061.5 |
   5181.9 |
   5302.3 |########################################
   5422.7 |#################
   5543.1 |##
   5663.5 |#######
   5783.9 |
   5904.3 |##
   6024.7 |
   6145.0 |
   6265.4 |
   6385.8 |
   6506.2 |
   6626.6 |
   6747.0 |##
   6867.4 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=30.4% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=32.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=47.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=38.4% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=28.6% (high variance, measurements may be unstable)
- **satfold-neon**: CV=42.3% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=28.6% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=45.8% (high variance, measurements may be unstable)
- **satfold-seq**: CV=24.0% (high variance, measurements may be unstable)
