# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is fastest but the noisiest (CV 55.2%)

satfold-nolaw wins on median (4.28 us) yet has the highest variance (CV 55.2%), while satfold-lanes4-idx is the steadiest (CV 9.9%, 7.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (satfold-nolaw, satfold-lanes64) are a dead heat (<1%)

satfold-nolaw (4.28 us) and satfold-lanes64 (4.30 us) differ by 0.56%, inside the noise, even though the wider field spreads 73.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.80)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-nolaw, satfold-lanes64, satfold-iterfold, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-seq, satfold-neon8} vs {satfold-lanes4-idx} (40% apart)

The field splits into a fast tier {satfold-nolaw, satfold-lanes64, satfold-iterfold, satfold-neon, satfold-lanes16-constl, satfold-lanes16, satfold-seq, satfold-neon8} and a slow tier {satfold-lanes4-idx} with a 40% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-nolaw is inconsistent: worst-20% is 2.1x its best-20%

satfold-nolaw's best 20% of batches run at 4.24 us but its worst 20% at 8.74 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4280.8 ns median (-1.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.74x (fastest 4280.8 ns, slowest 7428.9 ns)

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
| satfold-iterfold | 5224ns | 4390ns | 4108ns | 4554ns | 8353ns | base |
| satfold-lanes16 | 5440ns | 5160ns | 4302ns | 4912ns | 8166ns | +4.14% |
| satfold-lanes16-constl | 5286ns | 5111ns | 4222ns | 4858ns | 7636ns | +1.18% |
| satfold-lanes4-idx | 7305ns | 7501ns | 6073ns | 7464ns | 8057ns | +39.82% |
| satfold-lanes64 | 4837ns | 4363ns | 4114ns | 4383ns | 6922ns | -7.42% |
| satfold-neon | 5373ns | 4484ns | 4429ns | 4660ns | 8455ns | +2.84% |
| satfold-neon8 | 5344ns | 5402ns | 4378ns | 5049ns | 7193ns | +2.29% |
| satfold-nolaw | 5251ns | 4340ns | 4295ns | 4352ns | 8906ns | +0.52% |
| satfold-seq | 5221ns | 5192ns | 4265ns | 5151ns | 6386ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5143ns | 4050ns | 8189ns | base | 6.371 |
| satfold-lanes16 | 5357ns | 4243ns | 8008ns | +4.15% | 6.117 |
| satfold-lanes16-constl | 5204ns | 4163ns | 7484ns | +1.19% | 6.296 |
| satfold-lanes4-idx | 7229ns | 6015ns | 7959ns | +40.55% | 4.533 |
| satfold-lanes64 | 4761ns | 4055ns | 6777ns | -7.44% | 6.883 |
| satfold-neon | 5294ns | 4371ns | 8310ns | +2.93% | 6.190 |
| satfold-neon8 | 5266ns | 4320ns | 7070ns | +2.38% | 6.223 |
| satfold-nolaw | 5171ns | 4236ns | 8741ns | +0.55% | 6.336 |
| satfold-seq | 5143ns | 4208ns | 6278ns | +0.00% | 6.371 |

## Performance model

- Peak throughput: **8.091 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 7.563 | 93.5% |
| satfold-lanes16 | 6.442 | 79.6% |
| satfold-lanes16-constl | 6.503 | 80.4% |
| satfold-lanes4-idx | 4.411 | 54.5% |
| satfold-lanes64 | 7.612 | 94.1% |
| satfold-neon | 7.407 | 91.5% |
| satfold-neon8 | 6.164 | 76.2% |
| satfold-nolaw | 7.655 | 94.6% |
| satfold-seq | 6.427 | 79.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5224ns | 5224ns | base |
| satfold-lanes16 | 5440ns | 5440ns | +4.14% |
| satfold-lanes16-constl | 5286ns | 5286ns | +1.18% |
| satfold-lanes4-idx | 7305ns | 7305ns | +39.82% |
| satfold-lanes64 | 4837ns | 4837ns | -7.42% |
| satfold-neon | 5373ns | 5373ns | +2.84% |
| satfold-neon8 | 5344ns | 5344ns | +2.29% |
| satfold-nolaw | 5251ns | 5251ns | +0.52% |
| satfold-seq | 5221ns | 5221ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 4332ns | base | --- | [4204, 4712] | --- | --- | --- | --- |
| satfold-lanes16 | 5086ns | no significant difference | [-46, +265]ns | [4356, 5150] | no | 0.2682 | 0.2682 | 0 |
| satfold-lanes16-constl | 5039ns | no significant difference | [-77, +823]ns | [4305, 5199] | no | 0.1614 | 0.0807 | 0 |
| satfold-lanes4-idx | 7429ns | +2558.4ns (+59.1%) | [+2105, +3046]ns | [7280, 7569] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4305ns | no significant difference | [-250, +35]ns | [4244, 4404] | no | 0.2682 | 0.2682 | 0 |
| satfold-neon | 4424ns | +183.7ns (+4.2%) | [+3, +299]ns | [4407, 4442] | YES (adj: no) | 0.1614 | 0.0807 | 0 |
| satfold-neon8 | 5316ns | +223.4ns (+5.2%) | [+138, +299]ns | [4358, 5355] | YES | 0.0007 | 0.0002 | 0 |
| satfold-nolaw | 4281ns | no significant difference | [-129, +140]ns | [4259, 4312] | no | 0.2682 | 0.2682 | 0 |
| satfold-seq | 5099ns | no significant difference | [-51, +927]ns | [4982, 5206] | no | 0.2462 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4301ns | -0.4% | +17.8% | +75.7% | -0.2% | +2.8% | +24.8% | -0.9% | +0.2% |
| 1 | 2 | 4164ns | +24.0% | +1.9% | +75.3% | +4.3% | +27.3% | +4.0% | +3.9% | +23.7% |
| 1 | 3 | 4159ns | +3.2% | +26.5% | +46.5% | -1.1% | +5.6% | +4.5% | +2.5% | +27.6% |
| 1 | 4 | 5269ns | -2.2% | -18.3% | +45.0% | -14.6% | -17.0% | +1.7% | -18.2% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.116 | ok |
| satfold-lanes16 | -0.751 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.797 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.399 | moderate- |
| satfold-lanes64 | -0.577 | HIGH- (thermal bounce) |
| satfold-neon | -0.382 | moderate- |
| satfold-neon8 | -0.250 | moderate- |
| satfold-nolaw | -0.794 | HIGH- (thermal bounce) |
| satfold-seq | -0.043 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 3/4, lost 1/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.9ns | 5143.3ns | 0.5% |  |
| satfold-lanes16 | 24.0ns | 5356.6ns | 0.4% |  |
| satfold-lanes16-constl | 24.4ns | 5204.5ns | 0.5% |  |
| satfold-lanes4-idx | 23.2ns | 7228.7ns | 0.3% |  |
| satfold-lanes64 | 21.8ns | 4760.6ns | 0.5% |  |
| satfold-neon | 22.9ns | 5294.1ns | 0.4% |  |
| satfold-neon8 | 23.3ns | 5265.9ns | 0.4% |  |
| satfold-nolaw | 23.4ns | 5171.4ns | 0.5% |  |
| satfold-seq | 24.1ns | 5143.3ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4049.9-8189.4 ns)
   4049.9 |########################################
   4256.8 |#################
   4463.8 |##
   4670.8 |
   4877.8 |##
   5084.8 |###########
   5291.7 |########
   5498.7 |##
   5705.7 |##
   5912.7 |
   6119.6 |
   6326.6 |
   6533.6 |
   6740.6 |##
   6947.6 |
   7154.5 |
   7361.5 |
   7568.5 |
   7775.5 |
   7982.4 |
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 4242.8-8007.9 ns)
   4242.8 |#####################################
   4431.1 |##
   4619.3 |##
   4807.6 |
   4995.8 |########################################
   5184.1 |#####
   5372.3 |#####
   5560.6 |
   5748.9 |
   5937.1 |
   6125.4 |
   6313.6 |
   6501.9 |
   6690.1 |##
   6878.4 |
   7066.6 |
   7254.9 |
   7443.2 |
   7631.4 |
   7819.7 |
  (1 below, 3 above range)

satfold-lanes16-constl (n=40, range 4163.1-7484.4 ns)
   4163.1 |########################################
   4329.2 |############
   4495.3 |
   4661.3 |
   4827.4 |####
   4993.5 |########################
   5159.5 |############################
   5325.6 |############
   5491.7 |####
   5657.7 |
   5823.8 |
   5989.9 |
   6155.9 |
   6322.0 |
   6488.1 |
   6654.1 |
   6820.2 |
   6986.3 |####
   7152.3 |
   7318.4 |
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 6014.5-7959.1 ns)
   6014.5 |#####
   6111.7 |####################
   6209.0 |
   6306.2 |
   6403.4 |
   6500.6 |
   6597.9 |
   6695.1 |
   6792.3 |
   6889.6 |
   6986.8 |
   7084.0 |
   7181.2 |#########################
   7278.5 |####################
   7375.7 |####################
   7472.9 |####################
   7570.1 |########################################
   7667.4 |###############
   7764.6 |##########
   7861.8 |
  (4 below, 1 above range)

satfold-lanes64 (n=40, range 4054.8-6776.6 ns)
   4054.8 |####################
   4190.9 |########################################
   4327.0 |#######################
   4463.1 |######
   4599.2 |######
   4735.3 |######
   4871.3 |###
   5007.4 |###
   5143.5 |
   5279.6 |
   5415.7 |
   5551.8 |
   5687.9 |
   5824.0 |
   5960.1 |
   6096.2 |
   6232.2 |###
   6368.3 |
   6504.4 |
   6640.5 |
  (4 below, 2 above range)

satfold-neon (n=40, range 4370.7-8310.4 ns)
   4370.7 |########################################
   4567.6 |
   4764.6 |
   4961.6 |
   5158.6 |################
   5355.6 |
   5552.6 |
   5749.6 |
   5946.5 |
   6143.5 |
   6340.5 |
   6537.5 |
   6734.5 |
   6931.5 |
   7128.5 |
   7325.4 |
   7522.4 |
   7719.4 |
   7916.4 |
   8113.4 |
  (5 below, 4 above range)

satfold-neon8 (n=40, range 4320.1-7069.8 ns)
   4320.1 |#####################################
   4457.6 |
   4595.1 |
   4732.6 |
   4870.0 |
   5007.5 |
   5145.0 |
   5282.5 |########################################
   5420.0 |########
   5557.5 |###########
   5695.0 |
   5832.4 |
   5969.9 |
   6107.4 |
   6244.9 |
   6382.4 |
   6519.9 |
   6657.4 |
   6794.8 |
   6932.3 |
  (4 below, 2 above range)

satfold-nolaw (n=40, range 4236.3-8740.6 ns)
   4236.3 |########################################
   4461.5 |#
   4686.7 |
   4911.9 |
   5137.2 |
   5362.4 |#
   5587.6 |
   5812.8 |
   6038.0 |
   6263.2 |
   6488.5 |
   6713.7 |
   6938.9 |
   7164.1 |#
   7389.3 |
   7614.6 |
   7839.8 |
   8065.0 |
   8290.2 |
   8515.4 |
  (4 below, 4 above range)

satfold-seq (n=40, range 4207.9-6278.0 ns)
   4207.9 |####
   4311.4 |#############
   4414.9 |####
   4518.4 |
   4621.9 |
   4725.4 |
   4828.9 |
   4932.4 |########################################
   5035.9 |#############
   5139.4 |###################################
   5243.0 |#################
   5346.5 |#################
   5450.0 |####
   5553.5 |
   5657.0 |####
   5760.5 |
   5864.0 |
   5967.5 |
   6071.0 |
   6174.5 |
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=41.7% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=37.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=31.8% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=35.1% (high variance, measurements may be unstable)
- **satfold-neon**: CV=40.1% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=29.7% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=45.7% (high variance, measurements may be unstable)
- **satfold-seq**: CV=23.4% (high variance, measurements may be unstable)
