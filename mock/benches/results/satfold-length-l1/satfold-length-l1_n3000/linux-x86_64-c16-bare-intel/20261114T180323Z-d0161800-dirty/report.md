# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.0x slower than the field

satfold-lanes4-idx (10.55 us) is 2.0x the fastest (5.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-lanes16, satfold-iterfold) are a dead heat (<1%)

satfold-lanes16 (5.17 us) and satfold-iterfold (5.17 us) differ by 0.08%, inside the noise, even though the wider field spreads 104.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {satfold-lanes16, satfold-iterfold, satfold-neon, satfold-neon8, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-lanes64} vs {satfold-lanes4-idx} (92% apart)

The field splits into a fast tier {satfold-lanes16, satfold-iterfold, satfold-neon, satfold-neon8, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-lanes64} and a slow tier {satfold-lanes4-idx} with a 92% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader satfold-lanes16 vs stability leader satfold-neon (+1% speed for 4.8x steadier)

satfold-lanes16 is fastest (5.17 us, CV 30.2%); satfold-neon gives up 1.4% median for 4.8x lower variance (CV 6.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes16-constl is inconsistent: worst-20% is 1.7x its best-20%

satfold-lanes16-constl's best 20% of batches run at 4.33 us but its worst 20% at 7.25 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 5167.1 ns median (-0.1% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 2.04x (fastest 5167.1 ns, slowest 10545.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 5452ns | 5243ns | 5093ns | 5254ns | 6406ns | base |
| satfold-lanes16 | 5463ns | 5239ns | 4448ns | 5234ns | 7164ns | +0.20% |
| satfold-lanes16-constl | 5552ns | 5331ns | 4386ns | 5323ns | 7403ns | +1.83% |
| satfold-lanes4-idx | 10150ns | 10618ns | 8327ns | 10418ns | 11169ns | +86.17% |
| satfold-lanes64 | 5934ns | 5574ns | 5241ns | 5555ns | 7763ns | +8.84% |
| satfold-neon | 5172ns | 5311ns | 4589ns | 5271ns | 5459ns | -5.13% |
| satfold-neon8 | 5696ns | 5334ns | 5280ns | 5339ns | 7185ns | +4.48% |
| satfold-nolaw | 5435ns | 5362ns | 4371ns | 5328ns | 6818ns | -0.32% |
| satfold-seq | 5516ns | 5409ns | 5179ns | 5399ns | 6201ns | +1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5369ns | 5015ns | 6294ns | base | 6.103 |
| satfold-lanes16 | 5381ns | 4382ns | 7042ns | +0.23% | 6.089 |
| satfold-lanes16-constl | 5465ns | 4327ns | 7246ns | +1.78% | 5.996 |
| satfold-lanes4-idx | 10075ns | 8264ns | 11089ns | +87.64% | 3.252 |
| satfold-lanes64 | 5848ns | 5163ns | 7633ns | +8.91% | 5.603 |
| satfold-neon | 5099ns | 4526ns | 5385ns | -5.03% | 6.426 |
| satfold-neon8 | 5610ns | 5207ns | 7059ns | +4.48% | 5.841 |
| satfold-nolaw | 5355ns | 4312ns | 6698ns | -0.27% | 6.119 |
| satfold-seq | 5436ns | 5101ns | 6104ns | +1.24% | 6.028 |

## Performance model

- Peak throughput: **7.599 Gops/s** (satfold-nolaw; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.337 | 83.4% |
| satfold-lanes16 | 6.342 | 83.5% |
| satfold-lanes16-constl | 6.232 | 82.0% |
| satfold-lanes4-idx | 3.107 | 40.9% |
| satfold-lanes64 | 5.967 | 78.5% |
| satfold-neon | 6.256 | 82.3% |
| satfold-neon8 | 6.234 | 82.0% |
| satfold-nolaw | 6.195 | 81.5% |
| satfold-seq | 6.145 | 80.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5452ns | 5452ns | base |
| satfold-lanes16 | 5463ns | 5463ns | +0.20% |
| satfold-lanes16-constl | 5552ns | 5552ns | +1.83% |
| satfold-lanes4-idx | 10150ns | 10150ns | +86.17% |
| satfold-lanes64 | 5934ns | 5934ns | +8.84% |
| satfold-neon | 5172ns | 5172ns | -5.13% |
| satfold-neon8 | 5696ns | 5696ns | +4.48% |
| satfold-nolaw | 5435ns | 5435ns | -0.32% |
| satfold-seq | 5516ns | 5516ns | +1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5171ns | base | --- | [5166, 5188] | --- | --- | --- | --- |
| satfold-lanes16 | 5167ns | no significant difference | [-114, +150]ns | [5101, 5302] | no | 0.6358 | 0.6358 | 0 |
| satfold-lanes16-constl | 5258ns | +88.3ns (+1.7%) | [+27, +144]ns | [5190, 5300] | YES (adj: no) | 0.0616 | 0.0385 | 0 |
| satfold-lanes4-idx | 10545ns | +5327.7ns (+103.0%) | [+5069, +5522]ns | [10247, 10683] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5492ns | +211.5ns (+4.1%) | [+150, +443]ns | [5320, 5611] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 5238ns | no significant difference | [-61, +67]ns | [5218, 5278] | no | 0.6358 | 0.6358 | 0 |
| satfold-neon8 | 5256ns | +88.2ns (+1.7%) | [+73, +162]ns | [5245, 5274] | YES | 0.0014 | 0.0007 | 0 |
| satfold-nolaw | 5289ns | no significant difference | [-10, +131]ns | [5252, 5319] | no | 0.1076 | 0.0807 | 0 |
| satfold-seq | 5332ns | +154.1ns (+3.0%) | [+107, +198]ns | [5293, 5374] | YES | 0.0005 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5027ns | +5.8% | +3.2% | +120.2% | +13.7% | +7.0% | +3.9% | -14.1% | +2.0% |
| 1 | 2 | 5290ns | -0.3% | +0.5% | +56.8% | +4.0% | -0.4% | -0.5% | -0.0% | +0.3% |
| 1 | 3 | 5183ns | -15.1% | -16.5% | +98.1% | +3.6% | -12.4% | +1.5% | +1.3% | +3.0% |
| 1 | 4 | 5166ns | +3.1% | +2.1% | +106.6% | +2.3% | +1.1% | +2.3% | +4.3% | +5.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.435 | moderate- |
| satfold-lanes16 | -0.413 | moderate- |
| satfold-lanes16-constl | -0.500 | moderate- |
| satfold-lanes4-idx | -0.438 | moderate- |
| satfold-lanes64 | 0.216 | moderate+ |
| satfold-neon | -0.265 | moderate- |
| satfold-neon8 | -0.087 | ok |
| satfold-nolaw | -0.085 | ok |
| satfold-seq | 0.077 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 2/4, lost 2/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 1/4, lost 2/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 25.1ns | 5369.4ns | 0.5% |  |
| satfold-lanes16 | 24.2ns | 5381.5ns | 0.4% |  |
| satfold-lanes16-constl | 24.7ns | 5464.8ns | 0.5% |  |
| satfold-lanes4-idx | 23.0ns | 10075.4ns | 0.2% |  |
| satfold-lanes64 | 26.1ns | 5847.8ns | 0.4% |  |
| satfold-neon | 23.5ns | 5099.2ns | 0.5% |  |
| satfold-neon8 | 25.7ns | 5609.9ns | 0.5% |  |
| satfold-nolaw | 24.4ns | 5354.8ns | 0.5% |  |
| satfold-seq | 24.9ns | 5436.1ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5015.2-6293.8 ns)
   5015.2 |########
   5079.1 |##
   5143.1 |########################################
   5207.0 |########
   5270.9 |######
   5334.9 |########
   5398.8 |
   5462.7 |
   5526.6 |
   5590.6 |
   5654.5 |
   5718.4 |
   5782.4 |
   5846.3 |
   5910.2 |
   5974.2 |
   6038.1 |
   6102.0 |
   6166.0 |
   6229.9 |
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 4382.4-7042.4 ns)
   4382.4 |##############
   4515.4 |###
   4648.4 |
   4781.4 |
   4914.4 |
   5047.4 |########################################
   5180.4 |##################
   5313.4 |#############################
   5446.4 |##############
   5579.4 |
   5712.4 |
   5845.4 |
   5978.4 |
   6111.4 |
   6244.4 |
   6377.4 |
   6510.4 |
   6643.4 |
   6776.4 |
   6909.4 |
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 4326.7-7245.6 ns)
   4326.7 |###########
   4472.6 |
   4618.6 |
   4764.5 |
   4910.4 |
   5056.4 |#########################
   5202.3 |########################################
   5348.3 |########
   5494.2 |#####
   5640.2 |
   5786.1 |##
   5932.1 |
   6078.0 |##
   6223.9 |
   6369.9 |
   6515.8 |
   6661.8 |
   6807.7 |
   6953.7 |
   7099.6 |
  (4 below, 2 above range)

satfold-lanes4-idx (n=40, range 8263.5-11089.0 ns)
   8263.5 |########################################
   8404.8 |######
   8546.1 |
   8687.3 |
   8828.6 |
   8969.9 |
   9111.2 |
   9252.4 |
   9393.7 |
   9535.0 |
   9676.3 |
   9817.5 |
   9958.8 |######
  10100.1 |####################
  10241.4 |##########################
  10382.6 |#############
  10523.9 |##########################
  10665.2 |########################################
  10806.5 |#############
  10947.7 |##########################
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 5163.2-7633.1 ns)
   5163.2 |###############################
   5286.7 |###############################
   5410.2 |##########################
   5533.7 |##########################
   5657.2 |########################################
   5780.7 |
   5904.2 |
   6027.7 |
   6151.2 |
   6274.7 |
   6398.2 |
   6521.7 |
   6645.2 |
   6768.7 |
   6892.1 |
   7015.6 |
   7139.1 |
   7262.6 |
   7386.1 |
   7509.6 |
  (3 below, 2 above range)

satfold-neon (n=40, range 4525.6-5384.5 ns)
   4525.6 |##########################
   4568.5 |########
   4611.5 |
   4654.4 |
   4697.4 |
   4740.3 |
   4783.3 |
   4826.2 |
   4869.2 |
   4912.1 |
   4955.1 |
   4998.0 |
   5041.0 |
   5083.9 |
   5126.9 |########
   5169.8 |####
   5212.8 |########################################
   5255.7 |###############################
   5298.6 |#############
   5341.6 |#################
  (2 below, 4 above range)

satfold-neon8 (n=40, range 5206.5-7058.6 ns)
   5206.5 |########################################
   5299.1 |####
   5391.7 |#####
   5484.3 |
   5576.9 |
   5669.5 |
   5762.1 |
   5854.7 |
   5947.3 |
   6039.9 |
   6132.5 |
   6225.2 |
   6317.8 |
   6410.4 |
   6503.0 |
   6595.6 |
   6688.2 |
   6780.8 |
   6873.4 |
   6966.0 |
  (4 below, 2 above range)

satfold-nolaw (n=40, range 4312.0-6697.9 ns)
   4312.0 |###########
   4431.3 |
   4550.6 |
   4669.9 |
   4789.2 |
   4908.5 |
   5027.8 |
   5147.1 |###########
   5266.4 |########################################
   5385.7 |###########
   5505.0 |##
   5624.3 |
   5743.6 |
   5862.9 |
   5982.2 |
   6101.5 |
   6220.8 |
   6340.1 |
   6459.4 |
   6578.7 |
  (4 below, 2 above range)

satfold-seq (n=40, range 5100.7-6103.7 ns)
   5100.7 |###########
   5150.9 |###########
   5201.0 |######################
   5251.2 |######################
   5301.3 |########################################
   5351.5 |##################################
   5401.6 |######################
   5451.8 |
   5501.9 |#####
   5552.1 |
   5602.2 |###########
   5652.4 |#####
   5702.5 |#####
   5752.7 |
   5802.8 |
   5853.0 |#####
   5903.1 |
   5953.3 |
   6003.4 |
   6053.6 |
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=23.0% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=29.0% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=30.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=29.8% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=27.9% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=23.2% (high variance, measurements may be unstable)
