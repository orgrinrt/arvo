# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-lanes16, satfold-nolaw) are a dead heat (<1%)

satfold-lanes16 (5.21 us) and satfold-nolaw (5.22 us) differ by 0.13%, inside the noise, even though the wider field spreads 94.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {satfold-lanes16, satfold-nolaw, satfold-lanes64, satfold-lanes16-constl, satfold-iterfold, satfold-seq, satfold-neon} vs {satfold-lanes4-idx} (89% apart)

The field splits into a fast tier {satfold-lanes16, satfold-nolaw, satfold-lanes64, satfold-lanes16-constl, satfold-iterfold, satfold-seq, satfold-neon} and a slow tier {satfold-lanes4-idx} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-lanes64 is inconsistent: worst-20% is 2.2x its best-20%

satfold-lanes64's best 20% of batches run at 4.28 us but its worst 20% at 9.49 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 5212.2 ns median (-1.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.95x (fastest 5212.2 ns, slowest 10143.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 6087ns | 5352ns | 5282ns | 5348ns | 9105ns | base |
| satfold-lanes16 | 5738ns | 5285ns | 4346ns | 5285ns | 8486ns | -5.73% |
| satfold-lanes16-constl | 5830ns | 5322ns | 4318ns | 5387ns | 8672ns | -4.21% |
| satfold-lanes4-idx | 10186ns | 10231ns | 9769ns | 10212ns | 10525ns | +67.35% |
| satfold-lanes64 | 6003ns | 5319ns | 4336ns | 5333ns | 9681ns | -1.37% |
| satfold-neon | 5973ns | 5443ns | 5254ns | 5482ns | 8165ns | -1.86% |
| satfold-nolaw | 5751ns | 5293ns | 4474ns | 5333ns | 8281ns | -5.52% |
| satfold-seq | 5616ns | 5350ns | 4467ns | 5349ns | 7566ns | -7.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5994ns | 5210ns | 8930ns | base | 5.467 |
| satfold-lanes16 | 5646ns | 4287ns | 8302ns | -5.80% | 5.804 |
| satfold-lanes16-constl | 5739ns | 4258ns | 8493ns | -4.26% | 5.710 |
| satfold-lanes4-idx | 10108ns | 9691ns | 10445ns | +68.65% | 3.242 |
| satfold-lanes64 | 5909ns | 4277ns | 9488ns | -1.41% | 5.546 |
| satfold-neon | 5883ns | 5176ns | 8008ns | -1.84% | 5.570 |
| satfold-nolaw | 5657ns | 4413ns | 8097ns | -5.62% | 5.792 |
| satfold-seq | 5529ns | 4408ns | 7410ns | -7.75% | 5.927 |

## Performance model

- Peak throughput: **7.695 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.207 | 80.7% |
| satfold-lanes16 | 6.287 | 81.7% |
| satfold-lanes16-constl | 6.243 | 81.1% |
| satfold-lanes4-idx | 3.230 | 42.0% |
| satfold-lanes64 | 6.246 | 81.2% |
| satfold-neon | 6.101 | 79.3% |
| satfold-nolaw | 6.278 | 81.6% |
| satfold-seq | 6.207 | 80.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 6087ns | 6087ns | base |
| satfold-lanes16 | 5738ns | 5738ns | -5.73% |
| satfold-lanes16-constl | 5830ns | 5830ns | -4.21% |
| satfold-lanes4-idx | 10186ns | 10186ns | +67.35% |
| satfold-lanes64 | 6003ns | 6003ns | -1.37% |
| satfold-neon | 5973ns | 5973ns | -1.86% |
| satfold-nolaw | 5751ns | 5751ns | -5.52% |
| satfold-seq | 5616ns | 5616ns | -7.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5279ns | base | --- | [5248, 5300] | --- | --- | --- | --- |
| satfold-lanes16 | 5212ns | -160.3ns (-3.0%) | [-686, -48]ns | [5153, 5312] | YES | 0.0225 | 0.0064 | 0 |
| satfold-lanes16-constl | 5249ns | -76.3ns (-1.4%) | [-179, -4]ns | [5213, 5482] | YES (adj: no) | 0.1130 | 0.0807 | 0 |
| satfold-lanes4-idx | 10144ns | +4816.9ns (+91.2%) | [+4676, +4923]ns | [10017, 10250] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5246ns | no significant difference | [-140, +21]ns | [5191, 5347] | no | 0.4296 | 0.4296 | 0 |
| satfold-neon | 5371ns | +54.1ns (+1.0%) | [+7, +160]ns | [5315, 5519] | YES (adj: no) | 0.0898 | 0.0385 | 0 |
| satfold-nolaw | 5219ns | -118.5ns (-2.2%) | [-309, -13]ns | [5173, 5448] | YES (adj: no) | 0.1130 | 0.0807 | 0 |
| satfold-seq | 5279ns | no significant difference | [-271, +21]ns | [5228, 5342] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5219ns | -16.8% | -18.6% | +99.5% | +2.9% | +4.5% | -13.7% | -14.8% |
| 1 | 2 | 5273ns | +0.0% | -1.1% | +84.3% | -16.4% | +3.9% | +3.9% | +4.6% |
| 1 | 3 | 5238ns | +2.8% | +8.7% | +91.2% | +0.2% | +1.0% | +4.4% | +0.3% |
| 1 | 4 | 5319ns | -3.1% | -1.3% | +92.2% | -0.7% | +0.4% | -2.3% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.371 | moderate- |
| satfold-lanes16 | -0.065 | ok |
| satfold-lanes16-constl | 0.053 | ok |
| satfold-lanes4-idx | -0.381 | moderate- |
| satfold-lanes64 | -0.453 | moderate- |
| satfold-neon | 0.079 | ok |
| satfold-nolaw | -0.162 | ok |
| satfold-seq | -0.299 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 1/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 27.5ns | 5993.6ns | 0.5% |  |
| satfold-lanes16 | 25.6ns | 5646.1ns | 0.5% |  |
| satfold-lanes16-constl | 25.6ns | 5738.6ns | 0.4% |  |
| satfold-lanes4-idx | 24.4ns | 10108.1ns | 0.2% |  |
| satfold-lanes64 | 27.2ns | 5908.9ns | 0.5% |  |
| satfold-neon | 26.7ns | 5883.2ns | 0.5% |  |
| satfold-nolaw | 25.9ns | 5657.0ns | 0.5% |  |
| satfold-seq | 25.0ns | 5529.0ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5210.0-8929.8 ns)
   5210.0 |########################################
   5396.0 |
   5582.0 |
   5768.0 |#
   5954.0 |
   6140.0 |
   6325.9 |
   6511.9 |#
   6697.9 |#
   6883.9 |
   7069.9 |
   7255.9 |#
   7441.9 |
   7627.9 |
   7813.9 |
   7999.9 |
   8185.9 |
   8371.9 |
   8557.8 |
   8743.8 |
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 4287.1-8302.2 ns)
   4287.1 |##########
   4487.8 |##
   4688.6 |
   4889.4 |
   5090.1 |########################################
   5290.9 |####################
   5491.6 |#######
   5692.4 |
   5893.1 |
   6093.9 |
   6294.6 |
   6495.4 |
   6696.1 |
   6896.9 |
   7097.7 |
   7298.4 |
   7499.2 |
   7699.9 |
   7900.7 |##
   8101.4 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 4258.3-8493.0 ns)
   4258.3 |#####
   4470.0 |##
   4681.8 |
   4893.5 |
   5105.3 |########################################
   5317.0 |########
   5528.7 |#############
   5740.5 |#####
   5952.2 |##
   6163.9 |
   6375.7 |
   6587.4 |
   6799.1 |
   7010.9 |
   7222.6 |
   7434.3 |##
   7646.1 |
   7857.8 |
   8069.6 |
   8281.3 |
  (6 below, 4 above range)

satfold-lanes4-idx (n=40, range 9690.5-10445.0 ns)
   9690.5 |##############################
   9728.2 |##########
   9766.0 |
   9803.7 |####################
   9841.4 |
   9879.1 |
   9916.9 |##########
   9954.6 |##############################
   9992.3 |####################
  10030.0 |####################
  10067.8 |##########
  10105.5 |####################
  10143.2 |####################
  10180.9 |##############################
  10218.6 |##########
  10256.4 |##############################
  10294.1 |##########
  10331.8 |##########
  10369.5 |##############################
  10407.3 |########################################
  (3 below, 2 above range)

satfold-lanes64 (n=40, range 4276.9-9488.3 ns)
   4276.9 |##########
   4537.4 |
   4798.0 |##
   5058.6 |########################################
   5319.2 |#########################
   5579.7 |
   5840.3 |
   6100.9 |
   6361.5 |
   6622.0 |
   6882.6 |
   7143.2 |
   7403.7 |
   7664.3 |
   7924.9 |
   8185.5 |
   8446.0 |
   8706.6 |
   8967.2 |
   9227.8 |
  (4 below, 5 above range)

satfold-neon (n=40, range 5176.4-8008.3 ns)
   5176.4 |####################################
   5317.9 |########################################
   5459.5 |#####################
   5601.1 |##########
   5742.7 |###
   5884.3 |###
   6025.9 |
   6167.5 |
   6309.1 |
   6450.7 |###
   6592.3 |
   6733.9 |
   6875.5 |
   7017.1 |
   7158.7 |
   7300.3 |
   7441.9 |
   7583.5 |
   7725.1 |
   7866.7 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 4412.7-8096.6 ns)
   4412.7 |##################
   4596.9 |
   4781.1 |
   4965.2 |######
   5149.5 |########################################
   5333.7 |###########################
   5517.9 |#########
   5702.1 |
   5886.2 |
   6070.5 |
   6254.6 |
   6438.9 |###
   6623.1 |
   6807.2 |
   6991.4 |
   7175.6 |
   7359.9 |
   7544.0 |
   7728.2 |
   7912.4 |
  (3 below, 3 above range)

satfold-seq (n=40, range 4408.0-7410.4 ns)
   4408.0 |##
   4558.2 |#######
   4708.3 |
   4858.4 |
   5008.5 |
   5158.6 |########################################
   5308.8 |#######
   5458.9 |####################
   5609.0 |#####
   5759.1 |
   5909.2 |
   6059.4 |
   6209.5 |
   6359.6 |
   6509.7 |
   6659.8 |
   6809.9 |
   6960.1 |
   7110.2 |
   7260.3 |
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=37.1% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=36.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=33.8% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=40.4% (high variance, measurements may be unstable)
- **satfold-neon**: CV=29.4% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=34.7% (high variance, measurements may be unstable)
- **satfold-seq**: CV=31.3% (high variance, measurements may be unstable)
