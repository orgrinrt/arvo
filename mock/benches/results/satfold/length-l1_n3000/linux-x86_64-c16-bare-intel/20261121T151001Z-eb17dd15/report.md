# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-seq is fastest but the noisiest (CV 54.9%)

satfold-seq wins on median (4.72 us) yet has the highest variance (CV 54.9%), while satfold-lanes4-idx is the steadiest (CV 12.8%, 8.69 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.80)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-seq, satfold-lanes16, satfold-nolaw, satfold-neon8, satfold-iterfold, satfold-lanes16-constl, satfold-lanes64, satfold-neon} vs {satfold-lanes4-idx} (60% apart)

The field splits into a fast tier {satfold-seq, satfold-lanes16, satfold-nolaw, satfold-neon8, satfold-iterfold, satfold-lanes16-constl, satfold-lanes64, satfold-neon} and a slow tier {satfold-lanes4-idx} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-neon is inconsistent: worst-20% is 2.7x its best-20%

satfold-neon's best 20% of batches run at 4.22 us but its worst 20% at 11.18 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-seq** at 4719.1 ns median (-10.0% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.84x (fastest 4719.1 ns, slowest 8693.4 ns)

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
| satfold-iterfold | 5663ns | 5316ns | 4736ns | 5330ns | 7589ns | base |
| satfold-lanes16 | 5008ns | 5122ns | 4212ns | 4812ns | 6392ns | -11.56% |
| satfold-lanes16-constl | 5675ns | 5330ns | 4294ns | 5360ns | 8004ns | +0.21% |
| satfold-lanes4-idx | 9227ns | 8771ns | 8309ns | 8890ns | 11155ns | +62.93% |
| satfold-lanes64 | 5731ns | 5407ns | 4373ns | 5385ns | 8124ns | +1.19% |
| satfold-neon | 6878ns | 5500ns | 4273ns | 6247ns | 11373ns | +21.44% |
| satfold-neon8 | 5503ns | 5216ns | 4317ns | 5002ns | 8190ns | -2.83% |
| satfold-nolaw | 5646ns | 5212ns | 4337ns | 5201ns | 8291ns | -0.30% |
| satfold-seq | 5725ns | 4785ns | 4319ns | 4822ns | 9844ns | +1.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5581ns | 4678ns | 7455ns | base | 5.872 |
| satfold-lanes16 | 4931ns | 4154ns | 6270ns | -11.63% | 6.645 |
| satfold-lanes16-constl | 5590ns | 4236ns | 7851ns | +0.18% | 5.862 |
| satfold-lanes4-idx | 9160ns | 8246ns | 11078ns | +64.15% | 3.577 |
| satfold-lanes64 | 5644ns | 4315ns | 7968ns | +1.14% | 5.806 |
| satfold-neon | 6777ns | 4215ns | 11178ns | +21.44% | 4.835 |
| satfold-neon8 | 5422ns | 4258ns | 8039ns | -2.84% | 6.044 |
| satfold-nolaw | 5560ns | 4279ns | 8130ns | -0.37% | 5.893 |
| satfold-seq | 5641ns | 4261ns | 9667ns | +1.09% | 5.809 |

## Performance model

- Peak throughput: **7.888 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.250 | 79.2% |
| satfold-lanes16 | 6.498 | 82.4% |
| satfold-lanes16-constl | 6.233 | 79.0% |
| satfold-lanes4-idx | 3.769 | 47.8% |
| satfold-lanes64 | 6.147 | 77.9% |
| satfold-neon | 6.037 | 76.5% |
| satfold-neon8 | 6.368 | 80.7% |
| satfold-nolaw | 6.374 | 80.8% |
| satfold-seq | 6.944 | 88.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5663ns | 5663ns | base |
| satfold-lanes16 | 5008ns | 5008ns | -11.56% |
| satfold-lanes16-constl | 5675ns | 5675ns | +0.21% |
| satfold-lanes4-idx | 9227ns | 9227ns | +62.93% |
| satfold-lanes64 | 5731ns | 5731ns | +1.19% |
| satfold-neon | 6878ns | 6878ns | +21.44% |
| satfold-neon8 | 5503ns | 5503ns | -2.83% |
| satfold-nolaw | 5646ns | 5646ns | -0.30% |
| satfold-seq | 5725ns | 5725ns | +1.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5243ns | base | --- | [5206, 5338] | --- | --- | --- | --- |
| satfold-lanes16 | 5042ns | -294.8ns (-5.6%) | [-497, -223]ns | [4286, 5062] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5257ns | no significant difference | [-154, +106]ns | [5181, 5379] | no | 0.7267 | 0.6358 | 0 |
| satfold-lanes4-idx | 8693ns | +3282.2ns (+62.6%) | [+3205, +3444]ns | [8535, 8761] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5331ns | no significant difference | [-169, +151]ns | [5304, 5415] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon | 5428ns | no significant difference | [-349, +748]ns | [4452, 6893] | no | 0.8746 | 0.8746 | 0 |
| satfold-neon8 | 5146ns | -119.0ns (-2.3%) | [-316, -30]ns | [4448, 5318] | YES | 0.0129 | 0.0064 | 0 |
| satfold-nolaw | 5141ns | -218.2ns (-4.2%) | [-296, -73]ns | [5083, 5205] | YES | 0.0059 | 0.0022 | 0 |
| satfold-seq | 4719ns | -276.7ns (-5.3%) | [-818, -49]ns | [4413, 5108] | YES (adj: no) | 0.1291 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4726ns | -9.3% | -10.8% | +134.2% | +15.2% | -7.5% | -5.9% | -9.4% | +0.3% |
| 1 | 2 | 5368ns | -22.7% | -3.4% | +54.0% | -1.2% | -19.0% | +1.3% | -5.1% | -0.2% |
| 1 | 3 | 5290ns | -2.3% | +4.3% | +61.7% | -18.3% | +2.8% | -18.5% | -1.1% | -17.1% |
| 1 | 4 | 5291ns | -4.4% | -1.2% | +65.5% | +0.9% | +90.8% | -0.1% | +0.1% | -19.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.186 | ok |
| satfold-lanes16 | 0.163 | ok |
| satfold-lanes16-constl | 0.042 | ok |
| satfold-lanes4-idx | -0.185 | ok |
| satfold-lanes64 | -0.332 | moderate- |
| satfold-neon | 0.065 | ok |
| satfold-neon8 | -0.801 | HIGH- (thermal bounce) |
| satfold-nolaw | 0.046 | ok |
| satfold-seq | -0.048 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 2/4, lost 2/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 3/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.7ns | 5580.5ns | 0.4% |  |
| satfold-lanes16 | 22.4ns | 4931.2ns | 0.5% |  |
| satfold-lanes16-constl | 25.0ns | 5590.3ns | 0.4% |  |
| satfold-lanes4-idx | 20.5ns | 9160.4ns | 0.2% |  |
| satfold-lanes64 | 25.8ns | 5644.0ns | 0.5% |  |
| satfold-neon | 29.7ns | 6777.0ns | 0.4% |  |
| satfold-neon8 | 25.0ns | 5421.8ns | 0.5% |  |
| satfold-nolaw | 25.3ns | 5560.1ns | 0.5% |  |
| satfold-seq | 24.8ns | 5641.4ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4678.3-7455.3 ns)
   4678.3 |################
   4817.2 |########
   4956.0 |
   5094.9 |####################################
   5233.7 |########################################
   5372.6 |########################
   5511.4 |############
   5650.3 |
   5789.1 |
   5928.0 |
   6066.8 |
   6205.7 |
   6344.5 |
   6483.4 |
   6622.2 |
   6761.1 |
   6899.9 |
   7038.8 |
   7177.6 |
   7316.5 |
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 4153.9-6269.8 ns)
   4153.9 |#####################
   4259.7 |##############
   4365.5 |#######
   4471.3 |
   4577.1 |
   4682.9 |
   4788.7 |
   4894.5 |
   5000.3 |########################################
   5106.1 |##################
   5211.9 |###
   5317.7 |#######
   5423.5 |
   5529.3 |
   5635.1 |
   5740.9 |
   5846.7 |
   5952.5 |
   6058.3 |
   6164.1 |
  (6 below, 3 above range)

satfold-lanes16-constl (n=40, range 4235.8-7850.7 ns)
   4235.8 |#####
   4416.6 |##
   4597.3 |
   4778.1 |
   4958.8 |
   5139.6 |########################################
   5320.3 |#####################
   5501.0 |#############
   5681.8 |
   5862.5 |
   6043.3 |
   6224.0 |
   6404.7 |
   6585.5 |
   6766.2 |
   6947.0 |
   7127.7 |
   7308.5 |
   7489.2 |
   7669.9 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 8246.0-11077.5 ns)
   8246.0 |#####################
   8387.6 |##############
   8529.2 |##################
   8670.7 |########################################
   8812.3 |
   8953.9 |
   9095.5 |
   9237.0 |
   9378.6 |
   9520.2 |
   9661.8 |
   9803.3 |
   9944.9 |
  10086.5 |
  10228.1 |
  10369.6 |
  10511.2 |
  10652.8 |
  10794.4 |
  10935.9 |#########################
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 4315.1-7968.0 ns)
   4315.1 |############
   4497.8 |
   4680.4 |
   4863.1 |
   5045.7 |
   5228.4 |########################################
   5411.0 |#########################
   5593.6 |##
   5776.3 |
   5958.9 |
   6141.6 |
   6324.2 |
   6506.9 |
   6689.5 |
   6872.1 |
   7054.8 |
   7237.4 |
   7420.1 |
   7602.7 |
   7785.4 |
  (4 below, 4 above range)

satfold-neon (n=40, range 4215.2-11177.9 ns)
   4215.2 |########################################
   4563.4 |###
   4911.5 |###
   5259.6 |########################
   5607.8 |
   5955.9 |
   6304.0 |###
   6652.2 |
   7000.3 |###
   7348.4 |
   7696.6 |
   8044.7 |
   8392.8 |
   8741.0 |
   9089.1 |
   9437.2 |
   9785.3 |########################
  10133.5 |######
  10481.6 |
  10829.7 |
  (2 below, 3 above range)

satfold-neon8 (n=40, range 4257.9-8039.4 ns)
   4257.9 |########################################
   4447.0 |####################
   4636.1 |
   4825.2 |
   5014.2 |############
   5203.3 |############################
   5392.4 |################################
   5581.5 |
   5770.5 |
   5959.6 |
   6148.7 |
   6337.7 |
   6526.8 |
   6715.9 |
   6905.0 |
   7094.0 |
   7283.1 |
   7472.2 |
   7661.3 |
   7850.3 |
  (3 below, 4 above range)

satfold-nolaw (n=40, range 4279.0-8130.5 ns)
   4279.0 |##########
   4471.6 |
   4664.1 |
   4856.7 |
   5049.3 |########################################
   5241.9 |############
   5434.4 |####
   5627.0 |
   5819.6 |
   6012.2 |
   6204.8 |
   6397.3 |
   6589.9 |
   6782.5 |
   6975.0 |
   7167.6 |
   7360.2 |
   7552.8 |##
   7745.4 |
   7937.9 |
  (4 below, 3 above range)

satfold-seq (n=40, range 4260.9-9666.7 ns)
   4260.9 |########################################
   4531.2 |################
   4801.5 |##########
   5071.8 |################
   5342.0 |##########
   5612.3 |
   5882.6 |###
   6152.9 |
   6423.2 |
   6693.5 |
   6963.8 |
   7234.1 |
   7504.4 |
   7774.7 |
   8045.0 |###
   8315.3 |
   8585.6 |
   8855.9 |
   9126.2 |
   9396.4 |
  (6 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=29.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=29.8% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=29.5% (high variance, measurements may be unstable)
- **satfold-neon**: CV=42.9% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=34.7% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=34.0% (high variance, measurements may be unstable)
- **satfold-seq**: CV=45.9% (high variance, measurements may be unstable)
