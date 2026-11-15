# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (17.69 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon8 at 2.02 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon8 beats baseline by 89% (significant)

satfold-neon8 is -15.65 us (89%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 8.7x slower than the field

satfold-iterfold (17.69 us) is 8.7x the fastest (2.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.65)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.7x the fastest

Fastest satfold-neon8 (2.02 us) to slowest satfold-iterfold (17.69 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 5.4x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.86 us but its worst 20% at 9.96 us (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 2024.2 ns median (-88.6% vs baseline)
- 8 variants significantly faster than baseline
- Spread: 8.74x (fastest 2024.2 ns, slowest 17688.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 17780ns | 17762ns | 17748ns | 17762ns | 17867ns | base |
| satfold-lanes16 | 4675ns | 3391ns | 2928ns | 3462ns | 10059ns | -73.71% |
| satfold-lanes16-constl | 4600ns | 3420ns | 2789ns | 3530ns | 9622ns | -74.13% |
| satfold-lanes4-idx | 4085ns | 2335ns | 1909ns | 2766ns | 10220ns | -77.02% |
| satfold-lanes64 | 4736ns | 5446ns | 2063ns | 4711ns | 7485ns | -73.36% |
| satfold-neon | 4120ns | 2200ns | 1832ns | 3814ns | 7327ns | -76.83% |
| satfold-neon8 | 4273ns | 2090ns | 2062ns | 3838ns | 7790ns | -75.97% |
| satfold-nolaw | 10091ns | 9425ns | 9405ns | 9654ns | 12090ns | -43.24% |
| satfold-seq | 17801ns | 17482ns | 17471ns | 17483ns | 19085ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 17700ns | 17675ns | 17760ns | base | 1.851 |
| satfold-lanes16 | 4577ns | 2873ns | 9822ns | -74.14% | 7.160 |
| satfold-lanes16-constl | 4503ns | 2738ns | 9382ns | -74.56% | 7.277 |
| satfold-lanes4-idx | 3981ns | 1858ns | 9958ns | -77.51% | 8.232 |
| satfold-lanes64 | 4575ns | 1997ns | 7207ns | -74.15% | 7.163 |
| satfold-neon | 3977ns | 1772ns | 7049ns | -77.53% | 8.240 |
| satfold-neon8 | 4121ns | 1996ns | 7458ns | -76.72% | 7.952 |
| satfold-nolaw | 9992ns | 9334ns | 11930ns | -43.55% | 3.280 |
| satfold-seq | 17720ns | 17398ns | 18975ns | +0.11% | 1.849 |

## Performance model

- Peak throughput: **18.488 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.852 | 10.0% |
| satfold-lanes16 | 9.846 | 53.3% |
| satfold-lanes16-constl | 9.759 | 52.8% |
| satfold-lanes4-idx | 14.422 | 78.0% |
| satfold-lanes64 | 6.231 | 33.7% |
| satfold-neon | 15.388 | 83.2% |
| satfold-neon8 | 16.188 | 87.6% |
| satfold-nolaw | 3.503 | 19.0% |
| satfold-seq | 1.882 | 10.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 17780ns | 17780ns | base |
| satfold-lanes16 | 4675ns | 4675ns | -73.71% |
| satfold-lanes16-constl | 4600ns | 4600ns | -74.13% |
| satfold-lanes4-idx | 4085ns | 4085ns | -77.02% |
| satfold-lanes64 | 4736ns | 4736ns | -73.36% |
| satfold-neon | 4120ns | 4120ns | -76.83% |
| satfold-neon8 | 4273ns | 4273ns | -75.97% |
| satfold-nolaw | 10091ns | 10091ns | -43.24% |
| satfold-seq | 17801ns | 17801ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 17689ns | base | --- | [17685, 17692] | --- | --- | --- | --- |
| satfold-lanes16 | 3328ns | -14353.9ns (-81.1%) | [-14365, -14347]ns | [3326, 3335] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3358ns | -14328.2ns (-81.0%) | [-14572, -14297]ns | [3346, 3389] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2272ns | -15406.5ns (-87.1%) | [-15815, -14821]ns | [1860, 2880] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5259ns | -12421.3ns (-70.2%) | [-15671, -11685]ns | [2012, 6036] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2130ns | -15553.8ns (-87.9%) | [-15663, -11870]ns | [2022, 5825] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2024ns | -15654.9ns (-88.5%) | [-15671, -11567]ns | [2010, 6097] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 9353ns | -8332.7ns (-47.1%) | [-8338, -8317]ns | [9351, 9368] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17409ns | -281.1ns (-1.6%) | [-285, -278]ns | [17407, 17411] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17684ns | -81.2% | -84.5% | -89.5% | -81.2% | -88.5% | -68.3% | -38.1% | -1.6% |
| 1 | 2 | 17688ns | -84.6% | -84.5% | -89.3% | -73.3% | -83.2% | -81.5% | -47.1% | -1.6% |
| 1 | 3 | 17691ns | -81.2% | -81.1% | -89.5% | -88.7% | -81.9% | -88.7% | -47.1% | -1.6% |
| 1 | 4 | 17689ns | -81.2% | -80.8% | -86.3% | -65.4% | -76.7% | -88.6% | -47.2% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.175 | ok |
| satfold-lanes16 | -0.408 | moderate- |
| satfold-lanes16-constl | 0.263 | moderate+ |
| satfold-lanes4-idx | -0.120 | ok |
| satfold-lanes64 | -0.648 | HIGH- (thermal bounce) |
| satfold-neon | 0.103 | ok |
| satfold-neon8 | 0.177 | ok |
| satfold-nolaw | -0.081 | ok |
| satfold-seq | -0.587 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.9ns | 17700.0ns | 0.1% |  |
| satfold-lanes16 | 33.4ns | 4576.5ns | 0.7% |  |
| satfold-lanes16-constl | 31.9ns | 4502.9ns | 0.7% |  |
| satfold-lanes4-idx | 34.1ns | 3980.5ns | 0.9% |  |
| satfold-lanes64 | 54.0ns | 4574.8ns | 1.2% |  |
| satfold-neon | 47.0ns | 3976.9ns | 1.2% |  |
| satfold-neon8 | 48.7ns | 4120.8ns | 1.2% |  |
| satfold-nolaw | 29.1ns | 9991.6ns | 0.3% |  |
| satfold-seq | 24.5ns | 17720.0ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 17675.0-17760.1 ns)
  17675.0 |####
  17679.2 |###################################
  17683.5 |#############
  17687.7 |########################################
  17692.0 |###################################
  17696.3 |########
  17700.5 |########
  17704.8 |
  17709.0 |####
  17713.3 |
  17717.5 |####
  17721.8 |
  17726.0 |
  17730.3 |
  17734.6 |
  17738.8 |
  17743.1 |
  17747.3 |
  17751.6 |
  17755.8 |
  (4 below, 1 above range)

satfold-lanes16 (n=40, range 2872.8-9822.5 ns)
   2872.8 |
   3220.3 |########################################
   3567.7 |#
   3915.2 |
   4262.7 |#
   4610.2 |
   4957.7 |
   5305.2 |
   5652.6 |
   6000.1 |
   6347.6 |
   6695.1 |
   7042.6 |
   7390.1 |
   7737.6 |
   8085.0 |#
   8432.5 |
   8780.0 |#
   9127.5 |#
   9475.0 |#
  (6 below, 4 above range)

satfold-lanes16-constl (n=40, range 2738.0-9382.4 ns)
   2738.0 |######################
   3070.2 |########################################
   3402.4 |
   3734.7 |
   4066.9 |##
   4399.1 |##
   4731.3 |
   5063.5 |
   5395.8 |
   5728.0 |
   6060.2 |
   6392.4 |##
   6724.6 |
   7056.9 |
   7389.1 |
   7721.3 |##
   8053.5 |##
   8385.8 |
   8718.0 |##
   9050.2 |#####
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 1858.2-9958.5 ns)
   1858.2 |########################################
   2263.2 |##############
   2668.2 |####
   3073.2 |
   3478.3 |####
   3883.3 |
   4288.3 |
   4693.3 |
   5098.3 |
   5503.3 |#######
   5908.3 |##
   6313.4 |
   6718.4 |
   7123.4 |
   7528.4 |
   7933.4 |
   8338.4 |##
   8743.4 |##
   9148.4 |
   9553.5 |##
  (2 below, 4 above range)

satfold-lanes64 (n=40, range 1997.1-7207.1 ns)
   1997.1 |########################################
   2257.6 |
   2518.1 |
   2778.6 |
   3039.1 |
   3299.6 |
   3560.1 |
   3820.6 |###
   4081.1 |
   4341.6 |
   4602.1 |##########
   4862.6 |
   5123.1 |#######
   5383.6 |##########
   5644.1 |
   5904.6 |##########
   6165.1 |##########
   6425.6 |#######
   6686.1 |###
   6946.6 |##########
  (4 below, 4 above range)

satfold-neon (n=40, range 1772.4-7049.4 ns)
   1772.4 |########################################
   2036.3 |##
   2300.1 |
   2564.0 |
   2827.8 |
   3091.7 |
   3355.5 |
   3619.3 |
   3883.2 |
   4147.0 |#####
   4410.9 |
   4674.7 |
   4938.6 |
   5202.4 |##
   5466.3 |#####
   5730.1 |##
   5994.0 |########
   6257.8 |#####
   6521.7 |#####
   6785.5 |##
  (5 below, 5 above range)

satfold-neon8 (n=40, range 1995.8-7458.0 ns)
   1995.8 |########################################
   2268.9 |
   2542.1 |
   2815.2 |
   3088.3 |
   3361.4 |
   3634.5 |
   3907.6 |
   4180.7 |
   4453.8 |##
   4726.9 |
   5000.0 |
   5273.1 |##
   5546.3 |##
   5819.4 |
   6092.5 |#######
   6365.6 |####
   6638.7 |####
   6911.8 |##
   7184.9 |###########
  (5 below, 2 above range)

satfold-nolaw (n=40, range 9333.5-11930.4 ns)
   9333.5 |########################################
   9463.4 |
   9593.2 |#
   9723.0 |
   9852.9 |
   9982.7 |
  10112.6 |
  10242.4 |
  10372.3 |
  10502.1 |
  10632.0 |
  10761.8 |
  10891.6 |###############
  11021.5 |
  11151.3 |
  11281.2 |
  11411.0 |
  11540.9 |
  11670.7 |
  11800.6 |
  (2 below, 1 above range)

satfold-seq (n=40, range 17397.9-18975.2 ns)
  17397.9 |########################################
  17476.8 |
  17555.7 |
  17634.5 |
  17713.4 |
  17792.2 |
  17871.1 |
  17950.0 |
  18028.8 |
  18107.7 |
  18186.6 |
  18265.4 |
  18344.3 |
  18423.1 |
  18502.0 |
  18580.9 |
  18659.7 |
  18738.6 |
  18817.5 |
  18896.3 |
  (3 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=58.5% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=57.3% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=82.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **satfold-lanes64**: CV=46.6% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-neon**: CV=56.2% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-neon8**: CV=58.3% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.7x (possible bimodal distribution)
