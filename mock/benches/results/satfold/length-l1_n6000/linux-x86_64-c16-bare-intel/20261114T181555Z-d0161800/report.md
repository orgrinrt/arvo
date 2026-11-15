# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 123% faster than the next best (satfold-lanes16)

satfold-lanes4-idx (2.43 us) leads satfold-lanes16 (5.40 us) by 123%, a clear separation rather than a photo finish. CV 106.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 70% (significant)

satfold-lanes4-idx is -6.21 us (70%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-neon8 is an outlier: 4.7x slower than the field

satfold-neon8 (11.46 us) is 4.7x the fastest (2.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 106.5%)

satfold-lanes4-idx wins on median (2.43 us) yet has the highest variance (CV 106.5%), while satfold-seq is the steadiest (CV 2.6%, 10.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.60)

satfold-neon's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes4-idx} vs {satfold-lanes16, satfold-lanes16-constl, satfold-nolaw, satfold-iterfold, satfold-seq, satfold-neon, satfold-lanes64, satfold-neon8} (123% apart)

The field splits into a fast tier {satfold-lanes4-idx} and a slow tier {satfold-lanes16, satfold-lanes16-constl, satfold-nolaw, satfold-iterfold, satfold-seq, satfold-neon, satfold-lanes64, satfold-neon8} with a 123% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest satfold-lanes4-idx (2.43 us) to slowest satfold-neon8 (11.46 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.6x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.42 us but its worst 20% at 8.61 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2426.2 ns median (-72.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 4.72x (fastest 2426.2 ns, slowest 11461.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 11190ns | 8906ns | 8476ns | 10852ns | 14915ns | base |
| satfold-lanes16 | 5923ns | 5477ns | 4486ns | 5440ns | 8808ns | -47.07% |
| satfold-lanes16-constl | 6014ns | 5526ns | 5504ns | 5531ns | 7972ns | -46.26% |
| satfold-lanes4-idx | 4308ns | 2490ns | 2482ns | 3377ns | 8924ns | -61.50% |
| satfold-lanes64 | 11120ns | 11300ns | 10137ns | 11214ns | 11821ns | -0.62% |
| satfold-neon | 10648ns | 11168ns | 8226ns | 10974ns | 12092ns | -4.84% |
| satfold-neon8 | 12204ns | 11535ns | 10944ns | 11829ns | 14587ns | +9.06% |
| satfold-nolaw | 6099ns | 5617ns | 5586ns | 5618ns | 8057ns | -45.49% |
| satfold-seq | 10293ns | 10266ns | 9918ns | 10286ns | 10689ns | -8.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 11109ns | 8403ns | 14823ns | base | 2.950 |
| satfold-lanes16 | 5834ns | 4426ns | 8643ns | -47.48% | 5.617 |
| satfold-lanes16-constl | 5921ns | 5433ns | 7797ns | -46.70% | 5.534 |
| satfold-lanes4-idx | 4181ns | 2419ns | 8612ns | -62.36% | 7.837 |
| satfold-lanes64 | 11041ns | 10065ns | 11739ns | -0.61% | 2.968 |
| satfold-neon | 10574ns | 8167ns | 12013ns | -4.81% | 3.099 |
| satfold-neon8 | 12124ns | 10863ns | 14509ns | +9.14% | 2.703 |
| satfold-nolaw | 6011ns | 5515ns | 7901ns | -45.89% | 5.452 |
| satfold-seq | 10214ns | 9840ns | 10600ns | -8.06% | 3.208 |

## Performance model

- Peak throughput: **13.544 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.717 | 27.4% |
| satfold-lanes16 | 6.063 | 44.8% |
| satfold-lanes16-constl | 6.008 | 44.4% |
| satfold-lanes4-idx | 13.506 | 99.7% |
| satfold-lanes64 | 2.924 | 21.6% |
| satfold-neon | 2.959 | 21.8% |
| satfold-neon8 | 2.859 | 21.1% |
| satfold-nolaw | 5.909 | 43.6% |
| satfold-seq | 3.214 | 23.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 11190ns | 11190ns | base |
| satfold-lanes16 | 5923ns | 5923ns | -47.07% |
| satfold-lanes16-constl | 6014ns | 6014ns | -46.26% |
| satfold-lanes4-idx | 4308ns | 4308ns | -61.50% |
| satfold-lanes64 | 11120ns | 11120ns | -0.62% |
| satfold-neon | 10648ns | 10648ns | -4.84% |
| satfold-neon8 | 12204ns | 12204ns | +9.06% |
| satfold-nolaw | 6099ns | 6099ns | -45.49% |
| satfold-seq | 10293ns | 10293ns | -8.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 8816ns | base | --- | [8593, 14466] | --- | --- | --- | --- |
| satfold-lanes16 | 5404ns | -3966.1ns (-45.0%) | [-6506, -3367]ns | [5391, 5418] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5454ns | -3235.7ns (-36.7%) | [-7743, -3036]ns | [5449, 5466] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2426ns | -6207.0ns (-70.4%) | [-6520, -6019]ns | [2420, 3098] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 11207ns | no significant difference | [-2975, +2363]ns | [10823, 11503] | no | 0.4296 | 0.4296 | 0 |
| satfold-neon | 11074ns | no significant difference | [-3213, +195]ns | [10380, 11357] | no | 0.3576 | 0.2682 | 0 |
| satfold-neon8 | 11462ns | no significant difference | [-364, +2556]ns | [11187, 11930] | no | 0.3576 | 0.2682 | 0 |
| satfold-nolaw | 5545ns | -3164.0ns (-35.9%) | [-7189, -3023]ns | [5535, 5556] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 10194ns | no significant difference | [-4065, +1372]ns | [10103, 10280] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8593ns | -36.8% | -35.8% | -71.8% | +34.0% | +39.5% | +29.9% | -35.6% | +17.0% |
| 1 | 2 | 8413ns | -47.4% | -35.2% | -71.2% | +28.2% | -2.5% | +31.9% | -34.2% | +21.9% |
| 1 | 3 | 14841ns | -63.6% | -63.2% | -83.6% | -23.3% | -24.8% | -2.5% | -62.5% | -31.5% |
| 1 | 4 | 14547ns | -62.8% | -62.5% | -83.3% | -28.3% | -26.3% | -22.0% | -61.8% | -29.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.229 | moderate+ |
| satfold-lanes16 | -0.424 | moderate- |
| satfold-lanes16-constl | -0.134 | ok |
| satfold-lanes4-idx | -0.591 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.535 | HIGH- (thermal bounce) |
| satfold-neon | -0.598 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.387 | moderate- |
| satfold-nolaw | 0.160 | ok |
| satfold-seq | -0.429 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.5ns | 11108.6ns | 0.2% |  |
| satfold-lanes16 | 25.8ns | 5833.8ns | 0.4% |  |
| satfold-lanes16-constl | 26.3ns | 5921.3ns | 0.4% |  |
| satfold-lanes4-idx | 41.5ns | 4181.4ns | 1.0% |  |
| satfold-lanes64 | 24.6ns | 11040.8ns | 0.2% |  |
| satfold-neon | 23.5ns | 10574.4ns | 0.2% |  |
| satfold-neon8 | 24.7ns | 12124.1ns | 0.2% |  |
| satfold-nolaw | 25.8ns | 6010.6ns | 0.4% |  |
| satfold-seq | 24.4ns | 10213.7ns | 0.2% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 8403.0-14823.3 ns)
   8403.0 |########################################
   8724.1 |########
   9045.1 |
   9366.1 |##
   9687.1 |
  10008.1 |
  10329.1 |
  10650.1 |
  10971.1 |
  11292.2 |
  11613.2 |##
  11934.2 |
  12255.2 |
  12576.2 |
  12897.2 |
  13218.2 |
  13539.2 |
  13860.3 |
  14181.3 |########
  14502.3 |######################
  (5 below, 5 above range)

satfold-lanes16 (n=40, range 4426.1-8643.4 ns)
   4426.1 |######
   4637.0 |
   4847.9 |
   5058.7 |
   5269.6 |########################################
   5480.4 |#
   5691.3 |
   5902.2 |
   6113.0 |
   6323.9 |
   6534.8 |
   6745.6 |
   6956.5 |
   7167.4 |
   7378.2 |
   7589.1 |
   7800.0 |#
   8010.8 |
   8221.7 |
   8432.5 |
  (5 below, 3 above range)

satfold-lanes16-constl (n=40, range 5433.2-7796.8 ns)
   5433.2 |########################################
   5551.3 |####
   5669.5 |
   5787.7 |
   5905.9 |
   6024.1 |
   6142.3 |
   6260.5 |
   6378.6 |
   6496.8 |
   6615.0 |
   6733.2 |
   6851.4 |
   6969.6 |
   7087.7 |
   7205.9 |
   7324.1 |
   7442.3 |
   7560.5 |
   7678.7 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 2419.5-8612.5 ns)
   2419.5 |########################################
   2729.1 |
   3038.8 |
   3348.4 |
   3658.1 |#
   3967.7 |
   4277.4 |
   4587.0 |
   4896.7 |
   5206.3 |#####
   5516.0 |
   5825.6 |
   6135.3 |
   6444.9 |
   6754.6 |
   7064.2 |
   7373.9 |#
   7683.5 |#
   7993.2 |#
   8302.8 |###
  (4 below, 5 above range)

satfold-lanes64 (n=40, range 10064.9-11739.5 ns)
  10064.9 |################
  10148.7 |
  10232.4 |########################
  10316.1 |########
  10399.8 |########
  10483.6 |
  10567.3 |################
  10651.0 |
  10734.8 |########
  10818.5 |########################
  10902.2 |
  10985.9 |########################
  11069.7 |########
  11153.4 |
  11237.1 |
  11320.8 |########################
  11404.6 |################
  11488.3 |########################################
  11572.0 |################################
  11655.7 |########################
  (3 below, 3 above range)

satfold-neon (n=40, range 8167.3-12013.3 ns)
   8167.3 |######
   8359.6 |#############
   8551.9 |
   8744.2 |######
   8936.5 |
   9128.8 |######
   9321.1 |
   9513.4 |
   9705.7 |
   9898.0 |
  10090.3 |######
  10282.6 |##########################
  10474.9 |
  10667.2 |#############
  10859.5 |####################
  11051.8 |##########################
  11244.1 |####################
  11436.4 |######
  11628.7 |#############
  11821.0 |########################################
  (5 below, 4 above range)

satfold-neon8 (n=40, range 10863.0-14509.0 ns)
  10863.0 |##############################
  11045.3 |########################################
  11227.6 |####################
  11409.9 |##########
  11592.2 |
  11774.5 |####################
  11956.8 |###############
  12139.1 |#####
  12321.4 |
  12503.7 |
  12686.0 |
  12868.3 |
  13050.6 |
  13232.9 |
  13415.2 |
  13597.5 |
  13779.8 |
  13962.1 |
  14144.4 |
  14326.7 |##############################
  (2 below, 4 above range)

satfold-nolaw (n=40, range 5514.7-7901.0 ns)
   5514.7 |########################################
   5634.0 |
   5753.3 |
   5872.7 |
   5992.0 |
   6111.3 |
   6230.6 |#
   6349.9 |
   6469.2 |
   6588.5 |
   6707.8 |
   6827.2 |
   6946.5 |
   7065.8 |
   7185.1 |
   7304.4 |
   7423.7 |
   7543.0 |
   7662.3 |
   7781.7 |
  (4 below, 4 above range)

satfold-seq (n=40, range 9839.6-10599.8 ns)
   9839.6 |##########
   9877.6 |####################
   9915.6 |
   9953.6 |####################
   9991.6 |##########
  10029.6 |##############################
  10067.6 |####################
  10105.7 |####################
  10143.7 |##########
  10181.7 |########################################
  10219.7 |##############################
  10257.7 |##############################
  10295.7 |####################
  10333.7 |
  10371.8 |
  10409.8 |##########
  10447.8 |##########
  10485.8 |####################
  10523.8 |##########
  10561.8 |####################
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=26.6% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=38.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=25.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=61.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-nolaw**: CV=24.6% (high variance, measurements may be unstable)
