# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-lanes64, satfold-neon) are a dead heat (<1%)

satfold-lanes64 (5.13 us) and satfold-neon (5.15 us) differ by 0.40%, inside the noise, even though the wider field spreads 30.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16 shows alternating (throttle bounce) (autocorr -0.64)

satfold-lanes16's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-lanes4-idx is inconsistent: worst-20% is 2.4x its best-20%

satfold-lanes4-idx's best 20% of batches run at 5.48 us but its worst 20% at 13.28 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 5134.4 ns median (-3.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.30x (fastest 5134.4 ns, slowest 6688.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 6317ns | 5370ns | 5167ns | 5630ns | 9527ns | base |
| satfold-lanes16 | 5722ns | 5275ns | 5169ns | 5292ns | 7562ns | -9.42% |
| satfold-lanes16-constl | 6743ns | 5457ns | 5265ns | 6143ns | 10024ns | +6.75% |
| satfold-lanes4-idx | 8524ns | 6761ns | 5543ns | 7872ns | 13462ns | +34.94% |
| satfold-lanes64 | 5414ns | 5206ns | 5029ns | 5192ns | 6464ns | -14.30% |
| satfold-neon | 5945ns | 5226ns | 5017ns | 5213ns | 9067ns | -5.89% |
| satfold-neon8 | 6633ns | 5459ns | 5071ns | 5971ns | 10184ns | +5.01% |
| satfold-nolaw | 5852ns | 5406ns | 4439ns | 5381ns | 8678ns | -7.36% |
| satfold-seq | 5707ns | 5385ns | 4420ns | 5309ns | 8186ns | -9.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 6222ns | 5095ns | 9377ns | base | 5.266 |
| satfold-lanes16 | 5634ns | 5096ns | 7429ns | -9.45% | 5.816 |
| satfold-lanes16-constl | 6639ns | 5184ns | 9834ns | +6.70% | 4.936 |
| satfold-lanes4-idx | 8422ns | 5485ns | 13279ns | +35.35% | 3.891 |
| satfold-lanes64 | 5327ns | 4957ns | 6328ns | -14.38% | 6.151 |
| satfold-neon | 5849ns | 4943ns | 8884ns | -5.99% | 5.602 |
| satfold-neon8 | 6535ns | 4999ns | 10018ns | +5.02% | 5.015 |
| satfold-nolaw | 5762ns | 4380ns | 8506ns | -7.39% | 5.686 |
| satfold-seq | 5619ns | 4355ns | 8027ns | -9.70% | 5.832 |

## Performance model

- Peak throughput: **7.524 Gops/s** (satfold-seq; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.184 | 82.2% |
| satfold-lanes16 | 6.307 | 83.8% |
| satfold-lanes16-constl | 6.084 | 80.9% |
| satfold-lanes4-idx | 4.899 | 65.1% |
| satfold-lanes64 | 6.382 | 84.8% |
| satfold-neon | 6.357 | 84.5% |
| satfold-neon8 | 6.083 | 80.8% |
| satfold-nolaw | 6.143 | 81.6% |
| satfold-seq | 6.170 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 6317ns | 6317ns | base |
| satfold-lanes16 | 5722ns | 5722ns | -9.42% |
| satfold-lanes16-constl | 6743ns | 6743ns | +6.75% |
| satfold-lanes4-idx | 8524ns | 8524ns | +34.94% |
| satfold-lanes64 | 5414ns | 5414ns | -14.30% |
| satfold-neon | 5945ns | 5945ns | -5.89% |
| satfold-neon8 | 6633ns | 6633ns | +5.01% |
| satfold-nolaw | 5852ns | 5852ns | -7.36% |
| satfold-seq | 5707ns | 5707ns | -9.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5299ns | base | --- | [5224, 5349] | --- | --- | --- | --- |
| satfold-lanes16 | 5196ns | no significant difference | [-139, +65]ns | [5160, 5274] | no | 0.8478 | 0.6358 | 0 |
| satfold-lanes16-constl | 5386ns | no significant difference | [-71, +295]ns | [5326, 5479] | no | 0.8478 | 0.6358 | 0 |
| satfold-lanes4-idx | 6689ns | +1547.7ns (+29.2%) | [+1420, +1622]ns | [6680, 6739] | YES | 0.0054 | 0.0007 | 0 |
| satfold-lanes64 | 5134ns | -186.9ns (-3.5%) | [-238, -48]ns | [5061, 5162] | YES | 0.0442 | 0.0166 | 0 |
| satfold-neon | 5155ns | -165.6ns (-3.1%) | [-243, -74]ns | [5058, 5218] | YES | 0.0089 | 0.0022 | 0 |
| satfold-neon8 | 5387ns | no significant difference | [-47, +65]ns | [5216, 5508] | no | 0.8478 | 0.4296 | 0 |
| satfold-nolaw | 5334ns | no significant difference | [-835, +154]ns | [5307, 5399] | no | 0.9996 | 0.8746 | 0 |
| satfold-seq | 5311ns | no significant difference | [-925, +122]ns | [5298, 5321] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5193ns | -0.1% | +83.3% | +28.7% | -0.6% | +0.5% | +1.0% | -15.6% | +2.7% |
| 1 | 2 | 5293ns | -0.7% | -1.2% | +144.7% | -5.3% | -2.2% | +2.8% | +2.3% | -17.6% |
| 1 | 3 | 9314ns | -45.2% | -42.8% | -41.1% | -45.0% | -45.5% | -0.4% | -43.0% | -43.0% |
| 1 | 4 | 5165ns | +3.2% | +5.2% | +29.5% | -0.7% | -1.0% | -2.3% | +3.5% | +2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.415 | moderate- |
| satfold-lanes16 | -0.639 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.107 | ok |
| satfold-lanes4-idx | -0.454 | moderate- |
| satfold-lanes64 | -0.484 | moderate- |
| satfold-neon | 0.196 | ok |
| satfold-neon8 | -0.430 | moderate- |
| satfold-nolaw | -0.163 | ok |
| satfold-seq | -0.428 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 1/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 1/4, lost 3/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 29.3ns | 6222.2ns | 0.5% |  |
| satfold-lanes16 | 26.6ns | 5634.1ns | 0.5% |  |
| satfold-lanes16-constl | 30.5ns | 6639.1ns | 0.5% |  |
| satfold-lanes4-idx | 29.0ns | 8422.0ns | 0.3% |  |
| satfold-lanes64 | 25.9ns | 5327.4ns | 0.5% |  |
| satfold-neon | 27.9ns | 5849.5ns | 0.5% |  |
| satfold-neon8 | 30.6ns | 6534.6ns | 0.5% |  |
| satfold-nolaw | 25.9ns | 5762.5ns | 0.4% |  |
| satfold-seq | 25.2ns | 5618.7ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5094.8-9376.8 ns)
   5094.8 |########################################
   5308.9 |###############
   5523.0 |
   5737.1 |
   5951.2 |
   6165.3 |
   6379.4 |
   6593.5 |
   6807.6 |
   7021.7 |
   7235.8 |
   7449.9 |
   7664.0 |##
   7878.1 |
   8092.2 |
   8306.3 |
   8520.4 |
   8734.5 |
   8948.6 |
   9162.7 |#################
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 5095.9-7428.8 ns)
   5095.9 |########################################
   5212.5 |######################
   5329.2 |########
   5445.8 |##
   5562.4 |
   5679.1 |
   5795.7 |
   5912.4 |
   6029.0 |
   6145.7 |
   6262.3 |
   6379.0 |##
   6495.6 |
   6612.2 |
   6728.9 |
   6845.5 |
   6962.2 |
   7078.8 |
   7195.5 |
   7312.1 |
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 5184.3-9833.5 ns)
   5184.3 |########################################
   5416.7 |###############
   5649.2 |##
   5881.6 |
   6114.1 |
   6346.6 |
   6579.0 |
   6811.5 |
   7044.0 |
   7276.4 |
   7508.9 |
   7741.4 |
   7973.8 |
   8206.3 |
   8438.7 |
   8671.2 |
   8903.7 |
   9136.1 |##
   9368.6 |#########################
   9601.1 |
  (5 below, 1 above range)

satfold-lanes4-idx (n=40, range 5484.6-13278.8 ns)
   5484.6 |########
   5874.3 |
   6264.0 |
   6653.7 |########################################
   7043.4 |
   7433.1 |
   7822.9 |
   8212.6 |
   8602.3 |
   8992.0 |##
   9381.7 |
   9771.4 |
  10161.1 |
  10550.8 |
  10940.5 |
  11330.2 |
  11719.9 |
  12109.6 |
  12499.4 |
  12889.1 |########################
  (5 below, 1 above range)

satfold-lanes64 (n=40, range 4957.2-6327.9 ns)
   4957.2 |##########################
   5025.8 |##########
   5094.3 |########################################
   5162.8 |####################
   5231.4 |##########
   5299.9 |###
   5368.4 |
   5437.0 |###
   5505.5 |
   5574.0 |
   5642.6 |
   5711.1 |
   5779.6 |
   5848.2 |
   5916.7 |
   5985.2 |
   6053.8 |
   6122.3 |
   6190.8 |
   6259.4 |
  (4 below, 2 above range)

satfold-neon (n=40, range 4942.8-8884.3 ns)
   4942.8 |###################################
   5139.9 |########################################
   5337.0 |#####
   5534.0 |
   5731.1 |
   5928.2 |
   6125.3 |
   6322.3 |
   6519.4 |
   6716.5 |
   6913.5 |
   7110.6 |
   7307.7 |
   7504.8 |
   7701.8 |
   7898.9 |##
   8096.0 |
   8293.0 |
   8490.1 |
   8687.2 |
  (4 below, 3 above range)

satfold-neon8 (n=40, range 4998.8-10017.6 ns)
   4998.8 |########################################
   5249.8 |##############################
   5500.7 |######
   5751.7 |
   6002.6 |
   6253.5 |
   6504.5 |
   6755.4 |
   7006.3 |###
   7257.3 |
   7508.2 |
   7759.1 |
   8010.1 |
   8261.0 |
   8511.9 |
   8762.9 |
   9013.8 |###
   9264.8 |###########################
   9515.7 |
   9766.6 |
  (3 below, 1 above range)

satfold-nolaw (n=40, range 4379.5-8506.1 ns)
   4379.5 |########
   4585.9 |
   4792.2 |
   4998.5 |
   5204.8 |########################################
   5411.2 |############
   5617.5 |
   5823.8 |
   6030.2 |
   6236.5 |
   6442.8 |
   6649.1 |
   6855.5 |
   7061.8 |
   7268.1 |
   7474.5 |
   7680.8 |##
   7887.1 |
   8093.4 |
   8299.8 |
  (5 below, 4 above range)

satfold-seq (n=40, range 4354.9-8027.3 ns)
   4354.9 |#######
   4538.6 |
   4722.2 |
   4905.8 |
   5089.4 |
   5273.0 |########################################
   5456.6 |
   5640.3 |
   5823.9 |
   6007.5 |
   6191.1 |
   6374.7 |#
   6558.4 |
   6742.0 |
   6925.6 |
   7109.2 |
   7292.8 |
   7476.4 |
   7660.1 |
   7843.7 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=28.0% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=31.3% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=30.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=37.9% (high variance, measurements may be unstable)
- **satfold-neon**: CV=40.4% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=34.1% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=34.1% (high variance, measurements may be unstable)
- **satfold-seq**: CV=36.6% (high variance, measurements may be unstable)
