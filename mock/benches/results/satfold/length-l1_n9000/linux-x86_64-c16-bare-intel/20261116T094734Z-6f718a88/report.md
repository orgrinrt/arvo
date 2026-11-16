# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (21.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes4-idx at 2.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes4-idx dominates: 11% faster than the next best (satfold-neon)

satfold-lanes4-idx (2.66 us) leads satfold-neon (2.95 us) by 11%, a clear separation rather than a photo finish. CV 84.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon8 beats baseline by 88% (significant)

satfold-neon8 is -18.92 us (88%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 8.1x slower than the field

satfold-iterfold (21.59 us) is 8.1x the fastest (2.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 84.1%)

satfold-lanes4-idx wins on median (2.66 us) yet has the highest variance (CV 84.1%), while satfold-lanes64 is the steadiest (CV 8.4%, 3.05 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.51)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes4-idx, satfold-neon, satfold-lanes64, satfold-neon8, satfold-lanes16-constl, satfold-lanes16, satfold-nolaw} vs {satfold-seq, satfold-iterfold} (165% apart)

The field splits into a fast tier {satfold-lanes4-idx, satfold-neon, satfold-lanes64, satfold-neon8, satfold-lanes16-constl, satfold-lanes16, satfold-nolaw} and a slow tier {satfold-seq, satfold-iterfold} with a 165% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.1x the fastest

Fastest satfold-lanes4-idx (2.66 us) to slowest satfold-iterfold (21.59 us): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-constl is inconsistent: worst-20% is 4.3x its best-20%

satfold-lanes16-constl's best 20% of batches run at 1.36 us but its worst 20% at 5.89 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (25 ns, 0.11%)

satfold-seq differs from baseline satfold-iterfold by 25 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2659.9 ns median (-87.7% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 8.12x (fastest 2659.9 ns, slowest 21585.8 ns)

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
| satfold-iterfold | 21202ns | 21653ns | 19048ns | 21214ns | 23319ns | base |
| satfold-lanes16 | 3813ns | 4497ns | 1532ns | 3845ns | 5999ns | -82.02% |
| satfold-lanes16-constl | 3605ns | 4293ns | 1415ns | 3482ns | 6162ns | -83.00% |
| satfold-lanes4-idx | 4190ns | 2732ns | 2167ns | 3594ns | 8003ns | -80.24% |
| satfold-lanes64 | 3313ns | 3238ns | 2968ns | 3273ns | 3780ns | -84.37% |
| satfold-neon | 3215ns | 3128ns | 2834ns | 3170ns | 3733ns | -84.83% |
| satfold-neon8 | 3242ns | 3381ns | 2004ns | 3356ns | 4139ns | -84.71% |
| satfold-nolaw | 7653ns | 7234ns | 7215ns | 7375ns | 8922ns | -63.91% |
| satfold-seq | 20062ns | 19085ns | 18721ns | 19418ns | 23332ns | -5.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 21130ns | 18988ns | 23233ns | base | 1.551 |
| satfold-lanes16 | 3664ns | 1476ns | 5742ns | -82.66% | 8.944 |
| satfold-lanes16-constl | 3462ns | 1364ns | 5894ns | -83.62% | 9.465 |
| satfold-lanes4-idx | 4054ns | 2106ns | 7709ns | -80.81% | 8.082 |
| satfold-lanes64 | 3110ns | 2796ns | 3503ns | -85.28% | 10.537 |
| satfold-neon | 3016ns | 2668ns | 3466ns | -85.72% | 10.864 |
| satfold-neon8 | 3034ns | 1885ns | 3861ns | -85.64% | 10.801 |
| satfold-nolaw | 7585ns | 7155ns | 8843ns | -64.10% | 4.320 |
| satfold-seq | 19991ns | 18660ns | 23252ns | -5.39% | 1.639 |

## Performance model

- Peak throughput: **24.028 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.518 | 6.3% |
| satfold-lanes16 | 7.563 | 31.5% |
| satfold-lanes16-constl | 7.925 | 33.0% |
| satfold-lanes4-idx | 12.319 | 51.3% |
| satfold-lanes64 | 10.746 | 44.7% |
| satfold-neon | 11.125 | 46.3% |
| satfold-neon8 | 10.301 | 42.9% |
| satfold-nolaw | 4.569 | 19.0% |
| satfold-seq | 1.723 | 7.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 21202ns | 21202ns | base |
| satfold-lanes16 | 3813ns | 3813ns | -82.02% |
| satfold-lanes16-constl | 3605ns | 3605ns | -83.00% |
| satfold-lanes4-idx | 4190ns | 4190ns | -80.24% |
| satfold-lanes64 | 3313ns | 3313ns | -84.37% |
| satfold-neon | 3215ns | 3215ns | -84.83% |
| satfold-neon8 | 3242ns | 3242ns | -84.71% |
| satfold-nolaw | 7653ns | 7653ns | -63.91% |
| satfold-seq | 20062ns | 20062ns | -5.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 21586ns | base | --- | [18992, 23211] | --- | --- | --- | --- |
| satfold-lanes16 | 4333ns | -17853.2ns (-82.7%) | [-18496, -17322]ns | [2014, 4720] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 4135ns | -17630.3ns (-81.7%) | [-18398, -17498]ns | [1674, 4637] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2660ns | -16879.4ns (-78.2%) | [-18928, -15874]ns | [2576, 4261] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 3049ns | -18445.0ns (-85.5%) | [-20009, -16033]ns | [2953, 3213] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2945ns | -18499.8ns (-85.7%) | [-20044, -16226]ns | [2838, 3091] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 3181ns | -18924.2ns (-87.7%) | [-20056, -16029]ns | [2942, 3339] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7171ns | -13935.2ns (-64.6%) | [-16043, -11820]ns | [7164, 7177] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 19021ns | +24.8ns (+0.1%) | [+15, +30]ns | [19018, 19022] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 18991ns | -87.1% | -92.8% | -83.5% | -84.4% | -85.4% | -82.5% | -53.5% | +0.2% |
| 1 | 2 | 18990ns | -76.1% | -76.2% | -82.3% | -83.0% | -83.7% | -81.5% | -62.3% | +0.2% |
| 1 | 3 | 23215ns | -81.3% | -84.8% | -88.9% | -86.8% | -87.1% | -87.3% | -69.1% | +0.1% |
| 1 | 4 | 23213ns | -80.7% | -81.6% | -90.8% | -86.9% | -86.7% | -87.3% | -69.1% | -19.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.250 | moderate+ |
| satfold-lanes16 | -0.154 | ok |
| satfold-lanes16-constl | -0.329 | moderate- |
| satfold-lanes4-idx | 0.229 | moderate+ |
| satfold-lanes64 | -0.507 | HIGH- (thermal bounce) |
| satfold-neon | -0.340 | moderate- |
| satfold-neon8 | 0.099 | ok |
| satfold-nolaw | -0.084 | ok |
| satfold-seq | -0.458 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.8ns | 21129.5ns | 0.1% |  |
| satfold-lanes16 | 52.3ns | 3663.7ns | 1.4% |  |
| satfold-lanes16-constl | 49.4ns | 3462.0ns | 1.4% |  |
| satfold-lanes4-idx | 40.0ns | 4054.3ns | 1.0% |  |
| satfold-lanes64 | 68.8ns | 3109.8ns | 2.2% |  |
| satfold-neon | 66.4ns | 3016.3ns | 2.2% |  |
| satfold-neon8 | 67.3ns | 3033.9ns | 2.2% |  |
| satfold-nolaw | 20.6ns | 7584.9ns | 0.3% |  |
| satfold-seq | 21.1ns | 19991.2ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 18987.6-23233.3 ns)
  18987.6 |###################################
  19199.9 |
  19412.2 |
  19624.5 |
  19836.7 |##
  20049.0 |
  20261.3 |
  20473.6 |
  20685.9 |
  20898.2 |
  21110.4 |
  21322.7 |
  21535.0 |
  21747.3 |
  21959.6 |
  22171.9 |
  22384.1 |
  22596.4 |
  22808.7 |
  23021.0 |########################################
  (2 below, 1 above range)

satfold-lanes16 (n=40, range 1475.8-5741.6 ns)
   1475.8 |########################################
   1689.1 |#####
   1902.4 |
   2115.7 |
   2329.0 |#####
   2542.3 |
   2755.6 |
   2968.9 |#####
   3182.2 |#####
   3395.4 |
   3608.7 |
   3822.0 |
   4035.3 |#####
   4248.6 |###############
   4461.9 |####################
   4675.2 |##########
   4888.5 |###############
   5101.8 |####################
   5315.1 |##########
   5528.4 |
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 1363.8-5894.1 ns)
   1363.8 |########################################
   1590.3 |######################
   1816.8 |####
   2043.3 |
   2269.8 |####
   2496.3 |####
   2722.8 |
   2949.4 |
   3175.9 |
   3402.4 |
   3628.9 |
   3855.4 |####
   4081.9 |########
   4308.4 |####
   4535.0 |#################
   4761.5 |
   4988.0 |#################
   5214.5 |####
   5441.0 |#############
   5667.5 |####
  (2 below, 4 above range)

satfold-lanes4-idx (n=40, range 2106.5-7709.0 ns)
   2106.5 |###################################
   2386.6 |########################################
   2666.7 |####
   2946.8 |#################
   3227.0 |
   3507.1 |
   3787.2 |####
   4067.4 |
   4347.5 |####
   4627.6 |
   4907.7 |
   5187.9 |####
   5468.0 |
   5748.1 |####
   6028.2 |#############
   6308.4 |
   6588.5 |########
   6868.6 |########
   7148.8 |
   7428.9 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 2795.7-3503.0 ns)
   2795.7 |####################
   2831.1 |####################
   2866.4 |##########
   2901.8 |##############################
   2937.2 |##############################
   2972.5 |##############################
   3007.9 |##########
   3043.2 |####################
   3078.6 |
   3114.0 |##########
   3149.3 |####################
   3184.7 |####################
   3220.1 |########################################
   3255.4 |##########
   3290.8 |##########
   3326.2 |
   3361.5 |##########
   3396.9 |##########
   3432.2 |
   3467.6 |##############################
  (4 below, 3 above range)

satfold-neon (n=40, range 2668.4-3465.9 ns)
   2668.4 |######
   2708.2 |#############
   2748.1 |########################################
   2788.0 |#############
   2827.9 |######
   2867.7 |
   2907.6 |########################################
   2947.5 |
   2987.4 |######
   3027.2 |#############
   3067.1 |####################
   3107.0 |
   3146.9 |####################
   3186.7 |######
   3226.6 |#############
   3266.5 |######
   3306.4 |
   3346.2 |######
   3386.1 |
   3426.0 |
  (3 below, 5 above range)

satfold-neon8 (n=40, range 1884.9-3861.1 ns)
   1884.9 |
   1983.7 |
   2082.5 |
   2181.3 |
   2280.1 |
   2378.9 |
   2477.8 |
   2576.6 |
   2675.4 |########################################
   2774.2 |######
   2873.0 |#################################
   2971.8 |######
   3070.6 |#############
   3169.4 |########################################
   3268.2 |#############
   3367.0 |######
   3465.8 |#################################
   3564.6 |
   3663.5 |#############
   3762.3 |######
  (4 below, 4 above range)

satfold-nolaw (n=40, range 7155.1-8843.4 ns)
   7155.1 |########################################
   7239.5 |
   7324.0 |
   7408.4 |
   7492.8 |
   7577.2 |
   7661.6 |
   7746.0 |
   7830.4 |
   7914.8 |
   7999.3 |
   8083.7 |
   8168.1 |
   8252.5 |
   8336.9 |
   8421.3 |
   8505.7 |
   8590.1 |
   8674.6 |
   8759.0 |########
  (3 below, 4 above range)

satfold-seq (n=40, range 18660.2-23252.0 ns)
  18660.2 |##########
  18889.8 |########################################
  19119.4 |
  19349.0 |
  19578.6 |
  19808.2 |
  20037.8 |
  20267.4 |
  20496.9 |
  20726.5 |
  20956.1 |
  21185.7 |
  21415.3 |
  21644.9 |
  21874.5 |
  22104.1 |
  22333.7 |
  22563.2 |
  22792.8 |
  23022.4 |##############
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=46.8% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=53.6% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=55.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-neon8**: CV=25.6% (high variance, measurements may be unstable)
