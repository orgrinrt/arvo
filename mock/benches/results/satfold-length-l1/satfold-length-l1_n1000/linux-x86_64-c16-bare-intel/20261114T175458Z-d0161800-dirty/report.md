# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-seq, satfold-lanes4-idx) are a dead heat (<1%)

satfold-seq (6.05 us) and satfold-lanes4-idx (6.08 us) differ by 0.39%, inside the noise, even though the wider field spreads 11.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.74)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (677 ns) is smaller than the fastest variant's own run-to-run std-dev (1.78 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-seq vs stability leader satfold-neon (+4% speed for 2.0x steadier)

satfold-seq is fastest (6.05 us, CV 29.4%); satfold-neon gives up 3.7% median for 2.0x lower variance (CV 14.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes16-constl is inconsistent: worst-20% is 2.0x its best-20%

satfold-lanes16-constl's best 20% of batches run at 6.51 us but its worst 20% at 13.34 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-seq** at 6054.4 ns median (-8.6% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.11x (fastest 6054.4 ns, slowest 6731.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 6824ns | 6700ns | 5647ns | 6735ns | 8268ns | base |
| satfold-lanes16 | 6136ns | 6177ns | 5032ns | 6121ns | 7283ns | -10.09% |
| satfold-lanes16-constl | 8603ns | 6803ns | 6585ns | 7640ns | 13509ns | +26.07% |
| satfold-lanes4-idx | 6107ns | 6152ns | 4911ns | 5693ns | 8548ns | -10.50% |
| satfold-lanes64 | 6544ns | 6352ns | 5244ns | 6361ns | 8393ns | -4.10% |
| satfold-neon | 6268ns | 6349ns | 5183ns | 6279ns | 7318ns | -8.16% |
| satfold-neon8 | 6209ns | 6312ns | 5289ns | 6029ns | 7669ns | -9.01% |
| satfold-nolaw | 6436ns | 6299ns | 5315ns | 6335ns | 7859ns | -5.69% |
| satfold-seq | 6097ns | 6125ns | 4993ns | 5721ns | 8327ns | -10.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 6745ns | 5589ns | 8160ns | base | 4.858 |
| satfold-lanes16 | 6055ns | 4973ns | 7158ns | -10.23% | 5.412 |
| satfold-lanes16-constl | 8501ns | 6513ns | 13340ns | +26.03% | 3.855 |
| satfold-lanes4-idx | 6025ns | 4852ns | 8398ns | -10.68% | 5.438 |
| satfold-lanes64 | 6460ns | 5185ns | 8266ns | -4.22% | 5.072 |
| satfold-neon | 6190ns | 5124ns | 7212ns | -8.24% | 5.294 |
| satfold-neon8 | 6129ns | 5230ns | 7529ns | -9.13% | 5.346 |
| satfold-nolaw | 6355ns | 5256ns | 7734ns | -5.79% | 5.156 |
| satfold-seq | 6016ns | 4933ns | 8184ns | -10.82% | 5.447 |

## Performance model

- Peak throughput: **6.754 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 4.944 | 73.2% |
| satfold-lanes16 | 5.368 | 79.5% |
| satfold-lanes16-constl | 4.868 | 72.1% |
| satfold-lanes4-idx | 5.391 | 79.8% |
| satfold-lanes64 | 5.223 | 77.3% |
| satfold-neon | 5.220 | 77.3% |
| satfold-neon8 | 5.252 | 77.8% |
| satfold-nolaw | 5.262 | 77.9% |
| satfold-seq | 5.412 | 80.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 6824ns | 6824ns | base |
| satfold-lanes16 | 6136ns | 6136ns | -10.09% |
| satfold-lanes16-constl | 8603ns | 8603ns | +26.07% |
| satfold-lanes4-idx | 6107ns | 6107ns | -10.50% |
| satfold-lanes64 | 6544ns | 6544ns | -4.10% |
| satfold-neon | 6268ns | 6268ns | -8.16% |
| satfold-neon8 | 6209ns | 6209ns | -9.01% |
| satfold-nolaw | 6436ns | 6436ns | -5.69% |
| satfold-seq | 6097ns | 6097ns | -10.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 6627ns | base | --- | [6593, 6813] | --- | --- | --- | --- |
| satfold-lanes16 | 6104ns | -631.0ns (-9.5%) | [-697, -511]ns | [6072, 6113] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 6732ns | +578.9ns (+8.7%) | [+44, +1118]ns | [6698, 6803] | YES | 0.0166 | 0.0166 | 0 |
| satfold-lanes4-idx | 6078ns | -788.2ns (-11.9%) | [-850, -593]ns | [4887, 6174] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 6274ns | -468.8ns (-7.1%) | [-504, -403]ns | [6188, 6395] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 6277ns | -519.1ns (-7.8%) | [-560, -451]ns | [6125, 6322] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 6239ns | -466.3ns (-7.0%) | [-667, -344]ns | [5526, 6269] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 6227ns | -466.7ns (-7.0%) | [-792, -329]ns | [6152, 6460] | YES | 0.0073 | 0.0064 | 0 |
| satfold-seq | 6054ns | -678.4ns (-10.2%) | [-919, -565]ns | [4982, 6065] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5662ns | -12.1% | +19.7% | -14.2% | -8.3% | -9.1% | -6.4% | +16.6% | -12.0% |
| 1 | 2 | 6627ns | -7.6% | +1.2% | -7.2% | -3.5% | -3.2% | -5.4% | -20.0% | -8.3% |
| 1 | 3 | 6727ns | -9.0% | +95.0% | -27.3% | -8.2% | -8.9% | -18.1% | -8.0% | -9.9% |
| 1 | 4 | 6920ns | -12.3% | -3.9% | -10.6% | -7.4% | -9.3% | -8.9% | -10.2% | -28.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.024 | ok |
| satfold-lanes16 | -0.096 | ok |
| satfold-lanes16-constl | -0.430 | moderate- |
| satfold-lanes4-idx | -0.737 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.203 | moderate- |
| satfold-neon | -0.263 | moderate- |
| satfold-neon8 | -0.661 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.526 | HIGH- (thermal bounce) |
| satfold-seq | -0.249 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.1ns | 6745.3ns | 0.4% |  |
| satfold-lanes16 | 24.3ns | 6055.1ns | 0.4% |  |
| satfold-lanes16-constl | 29.5ns | 8501.0ns | 0.3% |  |
| satfold-lanes4-idx | 23.6ns | 6025.2ns | 0.4% |  |
| satfold-lanes64 | 25.3ns | 6460.4ns | 0.4% |  |
| satfold-neon | 24.3ns | 6189.6ns | 0.4% |  |
| satfold-neon8 | 23.6ns | 6129.4ns | 0.4% |  |
| satfold-nolaw | 24.3ns | 6354.8ns | 0.4% |  |
| satfold-seq | 24.3ns | 6015.7ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5588.7-8160.4 ns)
   5588.7 |########################
   5717.2 |
   5845.8 |
   5974.4 |
   6103.0 |
   6231.6 |
   6360.2 |
   6488.8 |########################################
   6617.4 |########################
   6745.9 |########
   6874.5 |########################
   7003.1 |####################
   7131.7 |####
   7260.3 |
   7388.9 |
   7517.5 |
   7646.1 |
   7774.6 |
   7903.2 |
   8031.8 |
  (3 below, 1 above range)

satfold-lanes16 (n=40, range 4973.1-7158.2 ns)
   4973.1 |######
   5082.3 |
   5191.6 |
   5300.8 |
   5410.1 |
   5519.3 |
   5628.6 |
   5737.9 |
   5847.1 |
   5956.4 |######
   6065.6 |########################################
   6174.9 |
   6284.1 |
   6393.4 |
   6502.7 |#
   6611.9 |
   6721.2 |#
   6830.4 |
   6939.7 |
   7048.9 |
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 6512.9-13340.1 ns)
   6512.9 |########################################
   6854.3 |
   7195.7 |
   7537.0 |#
   7878.4 |
   8219.7 |
   8561.1 |
   8902.5 |
   9243.8 |
   9585.2 |
   9926.5 |
  10267.9 |
  10609.2 |
  10950.6 |
  11292.0 |
  11633.3 |
  11974.7 |
  12316.0 |
  12657.4 |
  12998.8 |################
  (3 below, 1 above range)

satfold-lanes4-idx (n=40, range 4852.0-8397.5 ns)
   4852.0 |########################################
   5029.3 |
   5206.5 |
   5383.8 |
   5561.1 |
   5738.4 |
   5915.6 |#####
   6092.9 |#####################################
   6270.2 |#####
   6447.5 |
   6624.8 |
   6802.0 |
   6979.3 |
   7156.6 |##
   7333.9 |
   7511.1 |
   7688.4 |
   7865.7 |##
   8043.0 |
   8220.2 |
  (3 below, 2 above range)

satfold-lanes64 (n=40, range 5185.3-8265.6 ns)
   5185.3 |##########
   5339.3 |
   5493.3 |
   5647.3 |
   5801.3 |
   5955.3 |
   6109.4 |########################################
   6263.4 |####################################
   6417.4 |####################
   6571.4 |###
   6725.4 |
   6879.4 |
   7033.4 |
   7187.5 |
   7341.5 |
   7495.5 |
   7649.5 |
   7803.5 |
   7957.5 |
   8111.5 |
  (5 below, 2 above range)

satfold-neon (n=40, range 5124.2-7212.4 ns)
   5124.2 |##########################
   5228.6 |
   5333.0 |
   5437.4 |
   5541.8 |
   5646.2 |
   5750.7 |
   5855.1 |
   5959.5 |####
   6063.9 |##########################
   6168.3 |#################
   6272.7 |########################################
   6377.1 |###################################
   6481.5 |####
   6585.9 |
   6690.3 |
   6794.8 |
   6899.2 |
   7003.6 |
   7108.0 |
  (3 below, 2 above range)

satfold-neon8 (n=40, range 5230.5-7529.0 ns)
   5230.5 |
   5345.4 |##########
   5460.3 |#####################
   5575.3 |##########
   5690.2 |
   5805.1 |
   5920.0 |
   6035.0 |
   6149.9 |##################
   6264.8 |########################################
   6379.7 |###
   6494.7 |
   6609.6 |
   6724.5 |###
   6839.4 |
   6954.4 |
   7069.3 |###
   7184.2 |
   7299.1 |
   7414.1 |
  (6 below, 3 above range)

satfold-nolaw (n=40, range 5255.8-7733.9 ns)
   5255.8 |##############################
   5379.7 |
   5503.6 |
   5627.5 |
   5751.5 |
   5875.4 |
   5999.3 |###############
   6123.2 |########################################
   6247.1 |####################
   6371.0 |###############
   6494.9 |##############################
   6618.8 |####################
   6742.7 |#####
   6866.6 |
   6990.5 |
   7114.4 |
   7238.3 |
   7362.2 |
   7486.1 |
   7610.0 |
  (3 below, 2 above range)

satfold-seq (n=40, range 4932.6-8183.9 ns)
   4932.6 |########################################
   5095.2 |
   5257.7 |
   5420.3 |
   5582.9 |
   5745.4 |
   5908.0 |########################################
   6070.6 |#######################
   6233.1 |
   6395.7 |###
   6558.3 |
   6720.8 |
   6883.4 |
   7046.0 |
   7208.5 |
   7371.1 |
   7533.7 |
   7696.2 |
   7858.8 |
   8021.4 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=23.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=34.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=34.2% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=26.7% (high variance, measurements may be unstable)
- **satfold-seq**: CV=29.6% (high variance, measurements may be unstable)
