# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw dominates: 19% faster than the next best (satfold-lanes16)

satfold-nolaw (1.98 us) leads satfold-lanes16 (2.35 us) by 19%, a clear separation rather than a photo finish. CV 119.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes64 is an outlier: 2.8x slower than the field

satfold-lanes64 (5.50 us) is 2.8x the fastest (1.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-nolaw is fastest but the noisiest (CV 119.3%)

satfold-nolaw wins on median (1.98 us) yet has the highest variance (CV 119.3%), while satfold-lanes64 is the steadiest (CV 44.0%, 5.50 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.75)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-nolaw, satfold-lanes16, satfold-lanes4-idx, satfold-neon, satfold-neon8, satfold-iterfold, satfold-seq, satfold-lanes16-constl} vs {satfold-lanes64} (119% apart)

The field splits into a fast tier {satfold-nolaw, satfold-lanes16, satfold-lanes4-idx, satfold-neon, satfold-neon8, satfold-iterfold, satfold-seq, satfold-lanes16-constl} and a slow tier {satfold-lanes64} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-neon is inconsistent: worst-20% is 4.2x its best-20%

satfold-neon's best 20% of batches run at 1.98 us but its worst 20% at 8.39 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 1981.0 ns median (-20.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 2.78x (fastest 1981.0 ns, slowest 5502.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112766038 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 3719ns | 2578ns | 1954ns | 2833ns | 8145ns | base |
| satfold-lanes16 | 3966ns | 2416ns | 1975ns | 3369ns | 7750ns | +6.63% |
| satfold-lanes16-constl | 4134ns | 2582ns | 2069ns | 3346ns | 8564ns | +11.15% |
| satfold-lanes4-idx | 4398ns | 2462ns | 2137ns | 3861ns | 8270ns | +18.25% |
| satfold-lanes64 | 4793ns | 5646ns | 2031ns | 4648ns | 7989ns | +28.87% |
| satfold-neon | 3847ns | 2483ns | 2030ns | 2841ns | 8680ns | +3.42% |
| satfold-neon8 | 4133ns | 2485ns | 2032ns | 3526ns | 8053ns | +11.11% |
| satfold-nolaw | 3682ns | 2033ns | 2031ns | 2835ns | 7872ns | -1.01% |
| satfold-seq | 3998ns | 2574ns | 1986ns | 3419ns | 7747ns | +7.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3589ns | 1898ns | 7845ns | base | 9.131 |
| satfold-lanes16 | 3847ns | 1921ns | 7474ns | +7.19% | 8.518 |
| satfold-lanes16-constl | 4008ns | 2016ns | 8242ns | +11.69% | 8.175 |
| satfold-lanes4-idx | 4270ns | 2078ns | 8000ns | +18.98% | 7.674 |
| satfold-lanes64 | 4657ns | 1980ns | 7720ns | +29.78% | 7.036 |
| satfold-neon | 3735ns | 1979ns | 8390ns | +4.08% | 8.773 |
| satfold-neon8 | 4016ns | 1981ns | 7787ns | +11.90% | 8.160 |
| satfold-nolaw | 3575ns | 1979ns | 7611ns | -0.38% | 9.165 |
| satfold-seq | 3881ns | 1932ns | 7488ns | +8.14% | 8.444 |

## Performance model

- Peak throughput: **17.268 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 13.208 | 76.5% |
| satfold-lanes16 | 13.939 | 80.7% |
| satfold-lanes16-constl | 13.017 | 75.4% |
| satfold-lanes4-idx | 13.660 | 79.1% |
| satfold-lanes64 | 5.955 | 34.5% |
| satfold-neon | 13.539 | 78.4% |
| satfold-neon8 | 13.527 | 78.3% |
| satfold-nolaw | 16.542 | 95.8% |
| satfold-seq | 13.071 | 75.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3719ns | 3719ns | base |
| satfold-lanes16 | 3966ns | 3966ns | +6.63% |
| satfold-lanes16-constl | 4134ns | 4134ns | +11.15% |
| satfold-lanes4-idx | 4398ns | 4398ns | +18.25% |
| satfold-lanes64 | 4793ns | 4793ns | +28.87% |
| satfold-neon | 3847ns | 3847ns | +3.42% |
| satfold-neon8 | 4133ns | 4133ns | +11.11% |
| satfold-nolaw | 3682ns | 3682ns | -1.01% |
| satfold-seq | 3998ns | 3998ns | +7.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 2481ns | base | --- | [1948, 2990] | --- | --- | --- | --- |
| satfold-lanes16 | 2351ns | no significant difference | [-79, +372]ns | [2315, 4829] | no | 1.0000 | 1.0000 | 0 |
| satfold-lanes16-constl | 2517ns | +104.3ns (+4.2%) | [+40, +377]ns | [2459, 2578] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| satfold-lanes4-idx | 2399ns | no significant difference | [-14, +482]ns | [2351, 5849] | no | 0.4103 | 0.1539 | 0 |
| satfold-lanes64 | 5503ns | no significant difference | [-10, +1206]ns | [1990, 6265] | no | 0.4291 | 0.2682 | 0 |
| satfold-neon | 2420ns | no significant difference | [-400, +509]ns | [2206, 2433] | no | 0.7267 | 0.6358 | 0 |
| satfold-neon8 | 2422ns | +441.7ns (+17.8%) | [+78, +510]ns | [2420, 4586] | YES | 0.0178 | 0.0022 | 0 |
| satfold-nolaw | 1981ns | no significant difference | [-5, +80]ns | [1980, 2899] | no | 0.4291 | 0.2682 | 0 |
| satfold-seq | 2507ns | no significant difference | [-464, +590]ns | [2355, 4638] | no | 0.7267 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 2984ns | -19.0% | -14.8% | -21.0% | -33.5% | -33.7% | -2.9% | -33.4% | -21.3% |
| 1 | 2 | 1913ns | +21.5% | +7.8% | +31.7% | +238.9% | +26.9% | +3.8% | +94.9% | +31.6% |
| 1 | 3 | 1952ns | +84.5% | +30.9% | +22.9% | +68.5% | +24.3% | +24.7% | +1.5% | +112.8% |
| 1 | 4 | 1984ns | -0.5% | +3.3% | +86.2% | +0.3% | +0.3% | +22.1% | -0.2% | +133.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.119 | ok |
| satfold-lanes16 | -0.558 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.750 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.134 | ok |
| satfold-lanes64 | -0.345 | moderate- |
| satfold-neon | -0.250 | moderate- |
| satfold-neon8 | -0.498 | moderate- |
| satfold-nolaw | -0.415 | moderate- |
| satfold-seq | 0.301 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 1/4, lost 3/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 1/4, lost 3/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 39.6ns | 3588.7ns | 1.1% |  |
| satfold-lanes16 | 38.5ns | 3846.7ns | 1.0% |  |
| satfold-lanes16-constl | 39.5ns | 4008.4ns | 1.0% |  |
| satfold-lanes4-idx | 42.4ns | 4269.9ns | 1.0% |  |
| satfold-lanes64 | 45.5ns | 4657.4ns | 1.0% |  |
| satfold-neon | 36.7ns | 3735.1ns | 1.0% |  |
| satfold-neon8 | 39.3ns | 4015.6ns | 1.0% |  |
| satfold-nolaw | 35.1ns | 3575.2ns | 1.0% |  |
| satfold-seq | 38.6ns | 3880.8ns | 1.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1897.6-7845.4 ns)
   1897.6 |########################################
   2195.0 |
   2492.4 |
   2789.8 |####################
   3087.2 |
   3384.6 |
   3682.0 |##
   3979.4 |
   4276.8 |
   4574.1 |#####
   4871.5 |##
   5168.9 |
   5466.3 |
   5763.7 |
   6061.1 |
   6358.5 |##
   6655.9 |##
   6953.3 |##
   7250.6 |##
   7548.0 |##
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1920.7-7473.9 ns)
   1920.7 |##############
   2198.3 |########################################
   2476.0 |
   2753.6 |
   3031.3 |
   3309.0 |
   3586.6 |
   3864.3 |
   4141.9 |
   4419.6 |##
   4697.3 |
   4974.9 |##
   5252.6 |
   5530.3 |##
   5807.9 |##
   6085.6 |#####
   6363.2 |########
   6640.9 |##
   6918.6 |#####
   7196.2 |##
  (6 below, 2 above range)

satfold-lanes16-constl (n=40, range 2016.4-8241.5 ns)
   2016.4 |####################
   2327.7 |########################################
   2638.9 |
   2950.2 |
   3261.5 |
   3572.7 |
   3884.0 |
   4195.2 |
   4506.5 |
   4817.7 |
   5129.0 |
   5440.2 |##
   5751.5 |
   6062.8 |##
   6374.0 |
   6685.3 |#####
   6996.5 |##
   7307.8 |##
   7619.0 |#####
   7930.3 |#####
  (6 below, 3 above range)

satfold-lanes4-idx (n=40, range 2078.4-7999.8 ns)
   2078.4 |########################################
   2374.5 |###########
   2670.6 |
   2966.6 |
   3262.7 |##
   3558.8 |
   3854.8 |
   4150.9 |
   4447.0 |
   4743.0 |
   5039.1 |
   5335.2 |#####
   5631.2 |
   5927.3 |##
   6223.4 |##
   6519.4 |##
   6815.5 |#####
   7111.6 |#####
   7407.6 |
   7703.7 |########
  (5 below, 4 above range)

satfold-lanes64 (n=40, range 1979.8-7720.0 ns)
   1979.8 |########################################
   2266.8 |
   2553.8 |
   2840.8 |
   3127.8 |
   3414.8 |
   3701.8 |
   3988.8 |
   4275.8 |
   4562.9 |###
   4849.9 |###
   5136.9 |###
   5423.9 |
   5710.9 |###
   5997.9 |#####################
   6284.9 |
   6571.9 |############
   6859.0 |###
   7146.0 |######
   7433.0 |###
  (4 below, 4 above range)

satfold-neon (n=40, range 1979.0-8389.7 ns)
   1979.0 |############################
   2299.6 |########################################
   2620.1 |##
   2940.7 |
   3261.2 |
   3581.7 |
   3902.3 |
   4222.8 |
   4543.3 |
   4863.9 |
   5184.4 |##
   5504.9 |##
   5825.5 |
   6146.0 |
   6466.5 |
   6787.1 |##
   7107.6 |##
   7428.1 |
   7748.7 |##
   8069.2 |##
  (4 below, 5 above range)

satfold-neon8 (n=40, range 1980.7-7787.1 ns)
   1980.7 |##################
   2271.0 |########################################
   2561.4 |
   2851.7 |
   3142.0 |
   3432.3 |
   3722.6 |######
   4012.9 |
   4303.3 |
   4593.6 |
   4883.9 |
   5174.2 |###
   5464.5 |###
   5754.9 |
   6045.2 |###
   6335.5 |###
   6625.8 |###
   6916.1 |######
   7206.5 |#########
   7496.8 |
  (5 below, 4 above range)

satfold-nolaw (n=40, range 1979.1-7611.4 ns)
   1979.1 |########################################
   2260.7 |
   2542.3 |
   2823.9 |
   3105.6 |
   3387.2 |
   3668.8 |###
   3950.4 |
   4232.0 |
   4513.6 |
   4795.2 |
   5076.9 |#
   5358.5 |#
   5640.1 |
   5921.7 |#
   6203.3 |#
   6484.9 |###
   6766.5 |#
   7048.1 |#
   7329.8 |#
  (4 below, 3 above range)

satfold-seq (n=40, range 1932.0-7488.4 ns)
   1932.0 |############
   2209.8 |########################################
   2487.6 |####################
   2765.4 |####
   3043.3 |####
   3321.1 |
   3598.9 |
   3876.7 |####
   4154.6 |
   4432.4 |
   4710.2 |
   4988.0 |
   5265.8 |########
   5543.7 |############
   5821.5 |########
   6099.3 |####
   6377.1 |####
   6655.0 |####
   6932.8 |####
   7210.6 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=64.8% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-lanes16**: CV=60.1% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=63.8% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=58.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes64**: CV=52.0% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-neon**: CV=68.2% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-neon8**: CV=59.3% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-nolaw**: CV=66.1% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-seq**: CV=57.8% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 3.9x (possible bimodal distribution)
