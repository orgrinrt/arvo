# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 beats baseline by 86% (significant)

satfold-neon8 is -12.22 us (86%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 7.9x slower than the field

satfold-seq (16.02 us) is 7.9x the fastest (2.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon8, satfold-lanes64) are a dead heat (<1%)

satfold-neon8 (2.02 us) and satfold-lanes64 (2.02 us) differ by 0.40%, inside the noise, even though the wider field spreads 695.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.75)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.9x the fastest

Fastest satfold-neon8 (2.02 us) to slowest satfold-seq (16.02 us): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 5.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.86 us but its worst 20% at 9.35 us (5.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 2015.2 ns median (-85.9% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.95x (fastest 2015.2 ns, slowest 16020.6 ns)

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
| satfold-iterfold | 14320ns | 14306ns | 14286ns | 14305ns | 14400ns | base |
| satfold-lanes16 | 4438ns | 3371ns | 2767ns | 3282ns | 9578ns | -69.01% |
| satfold-lanes16-constl | 4586ns | 3412ns | 2872ns | 3464ns | 9668ns | -67.97% |
| satfold-lanes4-idx | 3970ns | 2334ns | 1910ns | 2772ns | 9621ns | -72.28% |
| satfold-lanes64 | 3947ns | 2089ns | 1687ns | 3640ns | 7131ns | -72.43% |
| satfold-neon | 3885ns | 2090ns | 1699ns | 3388ns | 7561ns | -72.87% |
| satfold-neon8 | 3861ns | 2086ns | 1795ns | 3189ns | 7943ns | -73.04% |
| satfold-nolaw | 8122ns | 7705ns | 7667ns | 7844ns | 9413ns | -43.28% |
| satfold-seq | 16045ns | 16085ns | 14305ns | 16024ns | 17850ns | +12.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 14255ns | 14226ns | 14322ns | base | 2.299 |
| satfold-lanes16 | 4345ns | 2716ns | 9347ns | -69.52% | 7.541 |
| satfold-lanes16-constl | 4491ns | 2818ns | 9437ns | -68.50% | 7.297 |
| satfold-lanes4-idx | 3862ns | 1858ns | 9351ns | -72.91% | 8.484 |
| satfold-lanes64 | 3812ns | 1633ns | 6865ns | -73.26% | 8.596 |
| satfold-neon | 3751ns | 1646ns | 7269ns | -73.69% | 8.736 |
| satfold-neon8 | 3726ns | 1739ns | 7629ns | -73.86% | 8.795 |
| satfold-nolaw | 8056ns | 7609ns | 9335ns | -43.49% | 4.067 |
| satfold-seq | 15975ns | 14245ns | 17766ns | +12.06% | 2.051 |

## Performance model

- Peak throughput: **20.062 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.301 | 11.5% |
| satfold-lanes16 | 9.907 | 49.4% |
| satfold-lanes16-constl | 9.784 | 48.8% |
| satfold-lanes4-idx | 14.428 | 71.9% |
| satfold-lanes64 | 16.195 | 80.7% |
| satfold-neon | 16.188 | 80.7% |
| satfold-neon8 | 16.260 | 81.0% |
| satfold-nolaw | 4.289 | 21.4% |
| satfold-seq | 2.045 | 10.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 14320ns | 14320ns | base |
| satfold-lanes16 | 4438ns | 4438ns | -69.01% |
| satfold-lanes16-constl | 4586ns | 4586ns | -67.97% |
| satfold-lanes4-idx | 3970ns | 3970ns | -72.28% |
| satfold-lanes64 | 3947ns | 3947ns | -72.43% |
| satfold-neon | 3885ns | 3885ns | -72.87% |
| satfold-neon8 | 3861ns | 3861ns | -73.04% |
| satfold-nolaw | 8122ns | 8122ns | -43.28% |
| satfold-seq | 16045ns | 16045ns | +12.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 14243ns | base | --- | [14238, 14249] | --- | --- | --- | --- |
| satfold-lanes16 | 3308ns | -10935.2ns (-76.8%) | [-11506, -10921]ns | [2728, 3322] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3349ns | -10883.6ns (-76.4%) | [-10895, -10874]ns | [3348, 3364] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2271ns | -11965.9ns (-84.0%) | [-11980, -11871]ns | [2270, 2377] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2023ns | -12202.0ns (-85.7%) | [-12236, -8818]ns | [2003, 5419] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2024ns | -12200.5ns (-85.7%) | [-12236, -8899]ns | [2004, 5351] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2015ns | -12220.5ns (-85.8%) | [-12236, -10684]ns | [2006, 3556] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7640ns | -6594.4ns (-46.3%) | [-6614, -6586]ns | [7627, 7654] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 16021ns | +1784.7ns (+12.5%) | [+256, +3171]ns | [14503, 17411] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14237ns | -80.9% | -76.4% | -86.9% | -75.3% | -85.8% | -67.2% | -46.5% | +0.2% |
| 1 | 2 | 14240ns | -76.7% | -76.4% | -84.0% | -85.8% | -78.0% | -88.4% | -46.3% | +22.3% |
| 1 | 3 | 14245ns | -76.7% | -76.4% | -84.0% | -64.9% | -67.6% | -85.7% | -46.4% | +23.1% |
| 1 | 4 | 14253ns | -80.9% | -80.7% | -86.9% | -87.6% | -88.4% | -85.9% | -34.5% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.211 | moderate+ |
| satfold-lanes16 | -0.250 | moderate- |
| satfold-lanes16-constl | -0.085 | ok |
| satfold-lanes4-idx | -0.250 | moderate- |
| satfold-lanes64 | -0.746 | HIGH- (thermal bounce) |
| satfold-neon | -0.350 | moderate- |
| satfold-neon8 | -0.188 | ok |
| satfold-nolaw | -0.090 | ok |
| satfold-seq | -0.249 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 19.0ns | 14255.4ns | 0.1% |  |
| satfold-lanes16 | 30.0ns | 4345.3ns | 0.7% |  |
| satfold-lanes16-constl | 31.0ns | 4490.8ns | 0.7% |  |
| satfold-lanes4-idx | 34.7ns | 3862.5ns | 0.9% |  |
| satfold-lanes64 | 44.2ns | 3812.1ns | 1.2% |  |
| satfold-neon | 43.2ns | 3751.1ns | 1.2% |  |
| satfold-neon8 | 43.0ns | 3725.9ns | 1.2% |  |
| satfold-nolaw | 20.4ns | 8056.1ns | 0.3% |  |
| satfold-seq | 21.1ns | 15975.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14225.9-14322.3 ns)
  14225.9 |####################
  14230.7 |##########
  14235.6 |####################
  14240.4 |###################################
  14245.2 |##############################
  14250.0 |########################################
  14254.8 |#####
  14259.7 |##########
  14264.5 |
  14269.3 |
  14274.1 |
  14278.9 |
  14283.8 |
  14288.6 |
  14293.4 |
  14298.2 |
  14303.0 |
  14307.8 |
  14312.7 |
  14317.5 |
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 2715.9-9347.5 ns)
   2715.9 |################################
   3047.4 |########################################
   3379.0 |
   3710.6 |
   4042.2 |#####
   4373.8 |
   4705.3 |
   5036.9 |##
   5368.5 |
   5700.1 |
   6031.7 |
   6363.2 |
   6694.8 |
   7026.4 |
   7358.0 |
   7689.6 |##
   8021.1 |
   8352.7 |
   8684.3 |
   9015.9 |#####
  (3 below, 4 above range)

satfold-lanes16-constl (n=40, range 2818.4-9437.3 ns)
   2818.4 |
   3149.3 |########################################
   3480.3 |
   3811.2 |###
   4142.2 |
   4473.1 |
   4804.1 |
   5135.0 |
   5466.0 |
   5796.9 |#
   6127.9 |
   6458.8 |
   6789.8 |#
   7120.7 |
   7451.6 |
   7782.6 |
   8113.5 |
   8444.5 |
   8775.4 |
   9106.4 |
  (7 below, 6 above range)

satfold-lanes4-idx (n=40, range 1858.2-9351.1 ns)
   1858.2 |#########################
   2232.9 |########################################
   2607.5 |
   2982.2 |#####
   3356.8 |
   3731.4 |
   4106.1 |
   4480.7 |
   4855.4 |#####
   5230.0 |
   5604.7 |
   5979.3 |
   6354.0 |##
   6728.6 |
   7103.2 |
   7477.9 |
   7852.5 |########
   8227.2 |#####
   8601.8 |
   8976.5 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 1633.3-6864.6 ns)
   1633.3 |##############
   1894.9 |########################################
   2156.4 |
   2418.0 |
   2679.6 |
   2941.1 |
   3202.7 |
   3464.3 |
   3725.8 |
   3987.4 |###
   4249.0 |
   4510.5 |
   4772.1 |#######
   5033.7 |###
   5295.2 |#######
   5556.8 |##########
   5818.4 |#######
   6079.9 |###
   6341.5 |###
   6603.0 |#######
  (6 below, 4 above range)

satfold-neon (n=40, range 1646.0-7269.4 ns)
   1646.0 |####################
   1927.2 |########################################
   2208.3 |
   2489.5 |###
   2770.7 |
   3051.9 |
   3333.0 |
   3614.2 |
   3895.4 |###
   4176.5 |###
   4457.7 |
   4738.9 |
   5020.1 |
   5301.2 |######
   5582.4 |###
   5863.6 |##########
   6144.8 |###
   6425.9 |######
   6707.1 |###
   6988.3 |######
  (4 below, 3 above range)

satfold-neon8 (n=40, range 1738.7-7628.7 ns)
   1738.7 |########################################
   2033.2 |#####
   2327.7 |#####
   2622.2 |
   2916.7 |
   3211.2 |##
   3505.7 |##
   3800.2 |
   4094.7 |
   4389.2 |
   4683.7 |
   4978.2 |
   5272.7 |
   5567.2 |##
   5861.7 |##
   6156.2 |
   6450.7 |##
   6745.2 |########
   7039.7 |#####
   7334.2 |#####
  (6 below, 3 above range)

satfold-nolaw (n=40, range 7608.6-9335.5 ns)
   7608.6 |########################################
   7694.9 |
   7781.3 |
   7867.6 |
   7954.0 |
   8040.3 |
   8126.7 |
   8213.0 |
   8299.4 |
   8385.7 |
   8472.0 |
   8558.4 |
   8644.7 |
   8731.1 |
   8817.4 |
   8903.8 |
   8990.1 |
   9076.5 |
   9162.8 |
   9249.1 |#########
  (4 below, 4 above range)

satfold-seq (n=40, range 14244.6-17766.0 ns)
  14244.6 |##################################
  14420.7 |##################################
  14596.7 |#################
  14772.8 |
  14948.9 |
  15125.0 |
  15301.0 |
  15477.1 |
  15653.2 |
  15829.2 |
  16005.3 |
  16181.4 |
  16357.4 |
  16533.5 |
  16709.6 |
  16885.6 |
  17061.7 |
  17237.8 |########################################
  17413.8 |############################
  17589.9 |######################
  (5 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=61.3% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=58.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=77.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 5.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=56.7% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-neon**: CV=61.7% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **satfold-neon8**: CV=65.8% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 4.4x (possible bimodal distribution)
