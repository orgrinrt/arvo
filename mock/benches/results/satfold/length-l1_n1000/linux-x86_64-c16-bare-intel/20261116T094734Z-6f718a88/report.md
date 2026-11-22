# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-seq is fastest but the noisiest (CV 33.9%)

satfold-seq wins on median (4.97 us) yet has the highest variance (CV 33.9%), while satfold-lanes64 is the steadiest (CV 1.8%, 6.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.74)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-lanes16-constl is inconsistent: worst-20% is 1.7x its best-20%

satfold-lanes16-constl's best 20% of batches run at 6.12 us but its worst 20% at 10.46 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-seq** at 4967.6 ns median (-11.9% vs baseline)
- 5 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.35x (fastest 4967.6 ns, slowest 6697.6 ns)

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
| satfold-iterfold | 6001ns | 5694ns | 5416ns | 5804ns | 7178ns | base |
| satfold-lanes16 | 5768ns | 5085ns | 5019ns | 5296ns | 7933ns | -3.89% |
| satfold-lanes16-constl | 7418ns | 6772ns | 6185ns | 6767ns | 10601ns | +23.60% |
| satfold-lanes4-idx | 5587ns | 5876ns | 4908ns | 5579ns | 6287ns | -6.91% |
| satfold-lanes64 | 6402ns | 6424ns | 6223ns | 6415ns | 6541ns | +6.67% |
| satfold-neon | 5718ns | 5234ns | 5121ns | 5232ns | 7775ns | -4.72% |
| satfold-neon8 | 5864ns | 5274ns | 5136ns | 5435ns | 7876ns | -2.30% |
| satfold-nolaw | 6137ns | 6272ns | 5134ns | 5891ns | 7879ns | +2.26% |
| satfold-seq | 5700ns | 5027ns | 4989ns | 5207ns | 7890ns | -5.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5933ns | 5357ns | 7088ns | base | 5.523 |
| satfold-lanes16 | 5693ns | 4961ns | 7799ns | -4.05% | 5.756 |
| satfold-lanes16-constl | 7331ns | 6118ns | 10464ns | +23.56% | 4.470 |
| satfold-lanes4-idx | 5513ns | 4849ns | 6195ns | -7.08% | 5.944 |
| satfold-lanes64 | 6324ns | 6150ns | 6456ns | +6.59% | 5.181 |
| satfold-neon | 5644ns | 5062ns | 7641ns | -4.87% | 5.806 |
| satfold-neon8 | 5791ns | 5073ns | 7759ns | -2.40% | 5.659 |
| satfold-nolaw | 6061ns | 5075ns | 7770ns | +2.15% | 5.407 |
| satfold-seq | 5625ns | 4930ns | 7762ns | -5.19% | 5.826 |

## Performance model

- Peak throughput: **6.757 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 5.815 | 86.1% |
| satfold-lanes16 | 6.519 | 96.5% |
| satfold-lanes16-constl | 4.892 | 72.4% |
| satfold-lanes4-idx | 5.688 | 84.2% |
| satfold-lanes64 | 5.160 | 76.4% |
| satfold-neon | 6.332 | 93.7% |
| satfold-neon8 | 6.283 | 93.0% |
| satfold-nolaw | 5.285 | 78.2% |
| satfold-seq | 6.596 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 6001ns | 6001ns | base |
| satfold-lanes16 | 5768ns | 5768ns | -3.89% |
| satfold-lanes16-constl | 7418ns | 7418ns | +23.60% |
| satfold-lanes4-idx | 5587ns | 5587ns | -6.91% |
| satfold-lanes64 | 6402ns | 6402ns | +6.67% |
| satfold-neon | 5718ns | 5718ns | -4.72% |
| satfold-neon8 | 5864ns | 5864ns | -2.30% |
| satfold-nolaw | 6137ns | 6137ns | +2.26% |
| satfold-seq | 5700ns | 5700ns | -5.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5635ns | base | --- | [5486, 5790] | --- | --- | --- | --- |
| satfold-lanes16 | 5026ns | -437.9ns (-7.8%) | [-605, -356]ns | [4972, 5039] | YES | 0.0440 | 0.0385 | 0 |
| satfold-lanes16-constl | 6698ns | +960.7ns (+17.0%) | [+468, +1160]ns | [6643, 6728] | YES | 0.0086 | 0.0064 | 0 |
| satfold-lanes4-idx | 5761ns | -542.4ns (-9.6%) | [-669, -420]ns | [4966, 5975] | YES | 0.0086 | 0.0064 | 0 |
| satfold-lanes64 | 6350ns | +657.3ns (+11.7%) | [+543, +763]ns | [6295, 6390] | YES | 0.0086 | 0.0064 | 0 |
| satfold-neon | 5175ns | -481.5ns (-8.5%) | [-555, -293]ns | [5155, 5197] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 5215ns | -358.4ns (-6.4%) | [-428, -301]ns | [5126, 5361] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 6200ns | no significant difference | [-473, +760]ns | [5108, 6325] | no | 0.8746 | 0.8746 | 0 |
| satfold-seq | 4968ns | -503.2ns (-8.9%) | [-756, -402]ns | [4942, 4982] | YES | 0.0086 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5444ns | -8.8% | +49.0% | -11.0% | +16.4% | -4.7% | -6.0% | +19.8% | -9.3% |
| 1 | 2 | 5559ns | +10.4% | +18.3% | +8.5% | +16.1% | -5.2% | -8.6% | -8.3% | -10.5% |
| 1 | 3 | 5584ns | -9.9% | +20.4% | -11.3% | +10.3% | -9.0% | -5.5% | +12.0% | +8.9% |
| 1 | 4 | 6817ns | -27.1% | -1.5% | -12.3% | -6.1% | -24.1% | -8.2% | -25.3% | -27.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.049 | ok |
| satfold-lanes16 | -0.404 | moderate- |
| satfold-lanes16-constl | -0.161 | ok |
| satfold-lanes4-idx | -0.735 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.654 | HIGH- (thermal bounce) |
| satfold-neon | -0.449 | moderate- |
| satfold-neon8 | 0.039 | ok |
| satfold-nolaw | -0.682 | HIGH- (thermal bounce) |
| satfold-seq | -0.407 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 3/4, lost 1/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 3/4, lost 1/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.2ns | 5932.9ns | 0.4% |  |
| satfold-lanes16 | 22.8ns | 5692.6ns | 0.4% |  |
| satfold-lanes16-constl | 26.8ns | 7330.9ns | 0.4% |  |
| satfold-lanes4-idx | 21.8ns | 5513.2ns | 0.4% |  |
| satfold-lanes64 | 24.5ns | 6324.2ns | 0.4% |  |
| satfold-neon | 22.1ns | 5644.3ns | 0.4% |  |
| satfold-neon8 | 22.3ns | 5790.7ns | 0.4% |  |
| satfold-nolaw | 23.4ns | 6060.8ns | 0.4% |  |
| satfold-seq | 22.6ns | 5624.8ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5357.3-7087.5 ns)
   5357.3 |########################################
   5443.8 |######################
   5530.3 |#################
   5616.8 |##################################
   5703.4 |###########
   5789.9 |#################
   5876.4 |
   5962.9 |
   6049.4 |
   6135.9 |
   6222.4 |
   6308.9 |
   6395.4 |
   6481.9 |
   6568.4 |
   6655.0 |######################
   6741.5 |#####
   6828.0 |###########
   6914.5 |###########
   7001.0 |
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 4960.7-7799.2 ns)
   4960.7 |########################################
   5102.6 |
   5244.5 |
   5386.5 |
   5528.4 |
   5670.3 |
   5812.2 |
   5954.2 |
   6096.1 |################
   6238.0 |
   6379.9 |
   6521.9 |
   6663.8 |
   6805.7 |
   6947.6 |
   7089.6 |
   7231.5 |
   7373.4 |
   7515.3 |
   7657.3 |
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 6117.8-10464.5 ns)
   6117.8 |
   6335.1 |#####
   6552.4 |########################################
   6769.8 |#####
   6987.1 |
   7204.4 |
   7421.8 |###
   7639.1 |
   7856.4 |
   8073.8 |
   8291.1 |
   8508.5 |
   8725.8 |#
   8943.1 |
   9160.5 |
   9377.8 |
   9595.1 |
   9812.5 |
  10029.8 |
  10247.1 |#
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 4849.3-6194.7 ns)
   4849.3 |#############
   4916.5 |##########################
   4983.8 |####
   5051.1 |########
   5118.4 |####
   5185.6 |
   5252.9 |
   5320.2 |
   5387.4 |
   5454.7 |
   5522.0 |####
   5589.2 |
   5656.5 |
   5723.8 |
   5791.1 |
   5858.3 |
   5925.6 |########################################
   5992.9 |######################
   6060.1 |#################
   6127.4 |
  (6 below, 2 above range)

satfold-lanes64 (n=40, range 6150.0-6455.7 ns)
   6150.0 |########################################
   6165.3 |##############################
   6180.5 |
   6195.8 |
   6211.1 |
   6226.4 |##########
   6241.7 |
   6257.0 |##########
   6272.3 |##########
   6287.5 |####################
   6302.8 |##########
   6318.1 |##########
   6333.4 |####################
   6348.7 |########################################
   6364.0 |####################
   6379.3 |####################
   6394.5 |##########
   6409.8 |##########
   6425.1 |####################
   6440.4 |########################################
  (3 below, 5 above range)

satfold-neon (n=40, range 5062.2-7640.9 ns)
   5062.2 |########################################
   5191.1 |##########################
   5320.1 |
   5449.0 |
   5577.9 |
   5706.9 |
   5835.8 |
   5964.7 |
   6093.7 |##
   6222.6 |
   6351.5 |
   6480.5 |
   6609.4 |
   6738.3 |
   6867.3 |
   6996.2 |
   7125.1 |
   7254.1 |
   7383.0 |
   7511.9 |
  (3 below, 3 above range)

satfold-neon8 (n=40, range 5073.2-7759.4 ns)
   5073.2 |########################################
   5207.5 |##################
   5341.8 |##
   5476.1 |##
   5610.4 |
   5744.7 |
   5879.0 |
   6013.4 |
   6147.7 |################
   6282.0 |##
   6416.3 |#####
   6550.6 |##
   6684.9 |
   6819.2 |
   6953.5 |
   7087.9 |
   7222.2 |
   7356.5 |
   7490.8 |
   7625.1 |
  (4 below, 2 above range)

satfold-nolaw (n=40, range 5074.6-7770.0 ns)
   5074.6 |########################################
   5209.4 |###
   5344.1 |
   5478.9 |
   5613.7 |
   5748.5 |
   5883.2 |
   6018.0 |
   6152.8 |#####################
   6287.5 |##################
   6422.3 |######
   6557.1 |###############
   6691.8 |
   6826.6 |
   6961.4 |
   7096.1 |
   7230.9 |
   7365.7 |
   7500.5 |
   7635.2 |
  (4 below, 2 above range)

satfold-seq (n=40, range 4930.4-7762.1 ns)
   4930.4 |########################################
   5072.0 |
   5213.6 |
   5355.1 |
   5496.7 |
   5638.3 |
   5779.9 |
   5921.5 |
   6063.1 |###############
   6204.6 |
   6346.2 |#
   6487.8 |
   6629.4 |
   6771.0 |
   6912.6 |
   7054.1 |
   7195.7 |
   7337.3 |
   7478.9 |
   7620.5 |
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=25.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=29.4% (high variance, measurements may be unstable)
- **satfold-neon**: CV=29.4% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=27.8% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=23.7% (high variance, measurements may be unstable)
- **satfold-seq**: CV=30.0% (high variance, measurements may be unstable)
