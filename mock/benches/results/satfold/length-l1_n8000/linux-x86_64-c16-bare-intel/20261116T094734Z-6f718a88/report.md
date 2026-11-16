# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 beats baseline by 86% (significant)

satfold-lanes64 is -12.48 us (86%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 7.9x slower than the field

satfold-seq (15.84 us) is 7.9x the fastest (2.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-lanes64, satfold-neon8) are a dead heat (<1%)

satfold-lanes64 (2.00 us) and satfold-neon8 (2.01 us) differ by 0.60%, inside the noise, even though the wider field spreads 693.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-nolaw shows alternating (throttle bounce) (autocorr -0.76)

satfold-nolaw's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-neon8, satfold-lanes4-idx, satfold-neon, satfold-lanes16, satfold-lanes16-constl} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (155% apart)

The field splits into a fast tier {satfold-lanes64, satfold-neon8, satfold-lanes4-idx, satfold-neon, satfold-lanes16, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 155% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.9x the fastest

Fastest satfold-lanes64 (2.00 us) to slowest satfold-seq (15.84 us): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.9x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.86 us but its worst 20% at 9.09 us (4.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 1996.1 ns median (-86.2% vs baseline)
- 8 variants significantly faster than baseline
- Spread: 7.94x (fastest 1996.1 ns, slowest 15843.2 ns)

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
| satfold-iterfold | 15342ns | 14539ns | 14516ns | 14810ns | 17762ns | base |
| satfold-lanes16 | 4561ns | 3396ns | 2928ns | 3418ns | 9624ns | -70.27% |
| satfold-lanes16-constl | 4534ns | 3408ns | 2789ns | 3476ns | 9451ns | -70.45% |
| satfold-lanes4-idx | 4184ns | 3152ns | 1912ns | 3234ns | 9307ns | -72.73% |
| satfold-lanes64 | 3680ns | 2062ns | 1689ns | 3070ns | 7500ns | -76.02% |
| satfold-neon | 3925ns | 3325ns | 1688ns | 3649ns | 6991ns | -74.42% |
| satfold-neon8 | 3952ns | 2074ns | 2056ns | 3369ns | 7598ns | -74.24% |
| satfold-nolaw | 8586ns | 8615ns | 7721ns | 8585ns | 9458ns | -44.03% |
| satfold-seq | 15905ns | 15910ns | 14296ns | 15898ns | 17534ns | +3.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 15274ns | 14457ns | 17682ns | base | 2.145 |
| satfold-lanes16 | 4465ns | 2873ns | 9390ns | -70.77% | 7.339 |
| satfold-lanes16-constl | 4437ns | 2738ns | 9209ns | -70.95% | 7.386 |
| satfold-lanes4-idx | 4085ns | 1861ns | 9095ns | -73.26% | 8.022 |
| satfold-lanes64 | 3548ns | 1635ns | 7191ns | -76.77% | 9.236 |
| satfold-neon | 3780ns | 1634ns | 6689ns | -75.25% | 8.668 |
| satfold-neon8 | 3813ns | 1990ns | 7290ns | -75.04% | 8.594 |
| satfold-nolaw | 8514ns | 7662ns | 9373ns | -44.26% | 3.849 |
| satfold-seq | 15833ns | 14235ns | 17449ns | +3.66% | 2.070 |

## Performance model

- Peak throughput: **20.050 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.263 | 11.3% |
| satfold-lanes16 | 9.832 | 49.0% |
| satfold-lanes16-constl | 9.794 | 48.8% |
| satfold-lanes4-idx | 10.590 | 52.8% |
| satfold-lanes64 | 16.416 | 81.9% |
| satfold-neon | 10.179 | 50.8% |
| satfold-neon8 | 16.318 | 81.4% |
| satfold-nolaw | 3.846 | 19.2% |
| satfold-seq | 2.068 | 10.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 15342ns | 15342ns | base |
| satfold-lanes16 | 4561ns | 4561ns | -70.27% |
| satfold-lanes16-constl | 4534ns | 4534ns | -70.45% |
| satfold-lanes4-idx | 4184ns | 4184ns | -72.73% |
| satfold-lanes64 | 3680ns | 3680ns | -76.02% |
| satfold-neon | 3925ns | 3925ns | -74.42% |
| satfold-neon8 | 3952ns | 3952ns | -74.24% |
| satfold-nolaw | 8586ns | 8586ns | -44.03% |
| satfold-seq | 15905ns | 15905ns | +3.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 14480ns | base | --- | [14468, 14490] | --- | --- | --- | --- |
| satfold-lanes16 | 3333ns | -11153.6ns (-77.0%) | [-11738, -11135]ns | [3325, 3338] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3346ns | -11723.7ns (-81.0%) | [-11730, -11128]ns | [3046, 3356] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 3094ns | -11783.8ns (-81.4%) | [-12603, -11261]ns | [1924, 3212] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1996ns | -12480.9ns (-86.2%) | [-12819, -10526]ns | [1655, 4865] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 3219ns | -12029.2ns (-83.1%) | [-12645, -9756]ns | [2004, 5228] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2008ns | -12453.1ns (-86.0%) | [-12466, -11095]ns | [2003, 5057] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8520ns | -5956.6ns (-41.1%) | [-6792, -5139]ns | [7688, 9334] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 15843ns | -224.6ns (-1.6%) | [-237, -208]ns | [14249, 17401] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14474ns | -77.0% | -76.8% | -79.0% | -68.8% | -78.8% | -81.2% | -46.9% | +20.3% |
| 1 | 2 | 14476ns | -77.0% | -81.0% | -78.6% | -86.2% | -73.7% | -85.2% | -35.3% | -1.6% |
| 1 | 3 | 17677ns | -81.1% | -81.0% | -85.6% | -90.7% | -90.7% | -88.7% | -56.6% | -1.5% |
| 1 | 4 | 14467ns | -81.1% | -81.1% | -80.6% | -84.3% | -67.4% | -86.1% | -35.5% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.418 | moderate- |
| satfold-lanes16 | -0.090 | ok |
| satfold-lanes16-constl | -0.747 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.076 | ok |
| satfold-lanes64 | -0.049 | ok |
| satfold-neon | -0.651 | HIGH- (thermal bounce) |
| satfold-neon8 | 0.072 | ok |
| satfold-nolaw | -0.758 | HIGH- (thermal bounce) |
| satfold-seq | -0.751 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.7ns | 15274.0ns | 0.1% |  |
| satfold-lanes16 | 31.7ns | 4465.0ns | 0.7% |  |
| satfold-lanes16-constl | 31.6ns | 4436.7ns | 0.7% |  |
| satfold-lanes4-idx | 32.3ns | 4084.7ns | 0.8% |  |
| satfold-lanes64 | 42.1ns | 3547.7ns | 1.2% |  |
| satfold-neon | 44.8ns | 3780.5ns | 1.2% |  |
| satfold-neon8 | 46.8ns | 3813.0ns | 1.2% |  |
| satfold-nolaw | 21.7ns | 8514.3ns | 0.3% |  |
| satfold-seq | 21.9ns | 15833.4ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14456.6-17682.0 ns)
  14456.6 |########################################
  14617.9 |
  14779.2 |
  14940.4 |
  15101.7 |
  15263.0 |
  15424.2 |
  15585.5 |
  15746.8 |
  15908.1 |
  16069.3 |
  16230.6 |
  16391.9 |
  16553.1 |
  16714.4 |
  16875.7 |
  17037.0 |
  17198.2 |
  17359.5 |
  17520.8 |###########
  (3 below, 2 above range)

satfold-lanes16 (n=40, range 2872.6-9389.8 ns)
   2872.6 |
   3198.5 |########################################
   3524.4 |
   3850.2 |#
   4176.1 |
   4501.9 |
   4827.8 |
   5153.6 |
   5479.5 |
   5805.3 |
   6131.2 |
   6457.1 |
   6782.9 |
   7108.8 |#
   7434.6 |
   7760.5 |#
   8086.3 |#
   8412.2 |#
   8738.0 |#
   9063.9 |
  (6 below, 3 above range)

satfold-lanes16-constl (n=40, range 2738.0-9209.3 ns)
   2738.0 |###########################
   3061.6 |########################################
   3385.1 |
   3708.7 |
   4032.3 |
   4355.8 |
   4679.4 |
   5003.0 |
   5326.5 |##
   5650.1 |
   5973.7 |
   6297.2 |##
   6620.8 |##
   6944.4 |
   7267.9 |
   7591.5 |#####
   7915.1 |
   8238.6 |##
   8562.2 |##
   8885.8 |
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 1860.8-9095.0 ns)
   1860.8 |########################################
   2222.5 |######
   2584.2 |######
   2945.9 |##############################
   3307.6 |
   3669.3 |
   4031.0 |
   4392.7 |
   4754.5 |###
   5116.2 |
   5477.9 |
   5839.6 |######
   6201.3 |###
   6563.0 |
   6924.7 |###
   7286.4 |###
   7648.1 |
   8009.9 |
   8371.6 |###
   8733.3 |
  (3 below, 5 above range)

satfold-lanes64 (n=40, range 1634.8-7191.0 ns)
   1634.8 |########################################
   1912.6 |#########################
   2190.4 |
   2468.2 |
   2746.0 |###
   3023.8 |
   3301.6 |
   3579.5 |
   3857.3 |###
   4135.1 |
   4412.9 |###
   4690.7 |
   4968.5 |#######
   5246.3 |#######
   5524.1 |###
   5801.9 |#######
   6079.7 |###
   6357.6 |
   6635.4 |###
   6913.2 |#######
  (5 below, 3 above range)

satfold-neon (n=40, range 1634.3-6689.0 ns)
   1634.3 |########################################
   1887.1 |######################
   2139.8 |####
   2392.5 |####
   2645.3 |
   2898.0 |
   3150.7 |
   3403.5 |
   3656.2 |####
   3908.9 |####
   4161.7 |
   4414.4 |####
   4667.1 |####
   4919.9 |########
   5172.6 |########
   5425.3 |########
   5678.1 |
   5930.8 |#############
   6183.5 |########
   6436.3 |########
  (4 below, 3 above range)

satfold-neon8 (n=40, range 1989.7-7290.1 ns)
   1989.7 |########################################
   2254.7 |##
   2519.8 |
   2784.8 |
   3049.8 |
   3314.8 |##
   3579.8 |
   3844.9 |
   4109.9 |
   4374.9 |
   4639.9 |
   4904.9 |##
   5170.0 |
   5435.0 |
   5700.0 |##
   5965.0 |####
   6230.1 |##
   6495.1 |####
   6760.1 |####
   7025.1 |########
  (3 below, 2 above range)

satfold-nolaw (n=40, range 7661.8-9372.5 ns)
   7661.8 |########################################
   7747.3 |
   7832.8 |
   7918.4 |
   8003.9 |
   8089.5 |
   8175.0 |
   8260.5 |
   8346.1 |
   8431.6 |
   8517.1 |
   8602.7 |
   8688.2 |
   8773.8 |
   8859.3 |
   8944.8 |
   9030.4 |
   9115.9 |
   9201.4 |
   9287.0 |########################################
  (4 below, 4 above range)

satfold-seq (n=40, range 14234.8-17449.1 ns)
  14234.8 |#################################
  14395.5 |
  14556.2 |
  14716.9 |
  14877.7 |
  15038.4 |
  15199.1 |
  15359.8 |
  15520.5 |
  15681.2 |
  15842.0 |
  16002.7 |
  16163.4 |
  16324.1 |
  16484.8 |
  16645.5 |
  16806.3 |
  16967.0 |
  17127.7 |
  17288.4 |########################################
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=57.8% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=58.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=68.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **satfold-lanes64**: CV=64.7% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **satfold-neon**: CV=55.3% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-neon8**: CV=60.6% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.7x (possible bimodal distribution)
