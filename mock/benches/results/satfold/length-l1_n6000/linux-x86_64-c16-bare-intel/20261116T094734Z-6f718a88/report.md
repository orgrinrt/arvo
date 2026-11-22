# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 83% faster than the next best (satfold-lanes16)

satfold-lanes4-idx (2.42 us) leads satfold-lanes16 (4.43 us) by 83%, a clear separation rather than a photo finish. CV 101.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 61% (significant)

satfold-lanes4-idx is -4.87 us (61%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-neon8 is an outlier: 4.3x slower than the field

satfold-neon8 (10.49 us) is 4.3x the fastest (2.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 101.7%)

satfold-lanes4-idx wins on median (2.42 us) yet has the highest variance (CV 101.7%), while satfold-seq is the steadiest (CV 8.8%, 10.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.78)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest satfold-lanes4-idx (2.42 us) to slowest satfold-neon8 (10.49 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.98 us but its worst 20% at 8.02 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2420.1 ns median (-69.7% vs baseline)
- 4 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 4.33x (fastest 2420.1 ns, slowest 10490.4 ns)

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
| satfold-iterfold | 8016ns | 8059ns | 6961ns | 7948ns | 9275ns | base |
| satfold-lanes16 | 5358ns | 4492ns | 4455ns | 4688ns | 8272ns | -33.15% |
| satfold-lanes16-constl | 5974ns | 4589ns | 4515ns | 5168ns | 9850ns | -25.48% |
| satfold-lanes4-idx | 3853ns | 2484ns | 2031ns | 2982ns | 8289ns | -51.93% |
| satfold-lanes64 | 9771ns | 9014ns | 8347ns | 9312ns | 12571ns | +21.89% |
| satfold-neon | 9825ns | 9493ns | 8585ns | 9591ns | 11768ns | +22.57% |
| satfold-neon8 | 10504ns | 10556ns | 8820ns | 10575ns | 11972ns | +31.03% |
| satfold-nolaw | 5318ns | 4596ns | 4566ns | 4765ns | 7727ns | -33.66% |
| satfold-seq | 9848ns | 10178ns | 8331ns | 10043ns | 10779ns | +22.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 7945ns | 6897ns | 9196ns | base | 4.124 |
| satfold-lanes16 | 5279ns | 4397ns | 8128ns | -33.55% | 6.207 |
| satfold-lanes16-constl | 5882ns | 4454ns | 9683ns | -25.96% | 5.571 |
| satfold-lanes4-idx | 3742ns | 1980ns | 8016ns | -52.90% | 8.756 |
| satfold-lanes64 | 9699ns | 8288ns | 12472ns | +22.07% | 3.379 |
| satfold-neon | 9758ns | 8526ns | 11688ns | +22.81% | 3.358 |
| satfold-neon8 | 10436ns | 8755ns | 11907ns | +31.35% | 3.140 |
| satfold-nolaw | 5239ns | 4507ns | 7589ns | -34.06% | 6.254 |
| satfold-seq | 9772ns | 8272ns | 10697ns | +22.99% | 3.353 |

## Performance model

- Peak throughput: **16.550 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 4.100 | 24.8% |
| satfold-lanes16 | 7.394 | 44.7% |
| satfold-lanes16-constl | 7.236 | 43.7% |
| satfold-lanes4-idx | 13.540 | 81.8% |
| satfold-lanes64 | 3.659 | 22.1% |
| satfold-neon | 3.473 | 21.0% |
| satfold-neon8 | 3.124 | 18.9% |
| satfold-nolaw | 7.223 | 43.6% |
| satfold-seq | 3.242 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 8016ns | 8016ns | base |
| satfold-lanes16 | 5358ns | 5358ns | -33.15% |
| satfold-lanes16-constl | 5974ns | 5974ns | -25.48% |
| satfold-lanes4-idx | 3853ns | 3853ns | -51.93% |
| satfold-lanes64 | 9771ns | 9771ns | +21.89% |
| satfold-neon | 9825ns | 9825ns | +22.57% |
| satfold-neon8 | 10504ns | 10504ns | +31.03% |
| satfold-nolaw | 5318ns | 5318ns | -33.66% |
| satfold-seq | 9848ns | 9848ns | +22.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 7993ns | base | --- | [7217, 8451] | --- | --- | --- | --- |
| satfold-lanes16 | 4432ns | -2870.6ns (-35.9%) | [-3016, -2677]ns | [4411, 4447] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 4528ns | -2612.8ns (-32.7%) | [-2824, -2434]ns | [4471, 5479] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2420ns | -4866.4ns (-60.9%) | [-5119, -4669]ns | [1981, 2808] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 8955ns | +1741.0ns (+21.8%) | [+1268, +1982]ns | [8904, 9543] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 9434ns | +1610.1ns (+20.1%) | [+1007, +1830]ns | [9049, 9574] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 10490ns | +2445.5ns (+30.6%) | [+2193, +2741]ns | [9359, 11687] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 4537ns | -2759.3ns (-34.5%) | [-3220, -2611]ns | [4528, 4557] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 10106ns | +2223.6ns (+27.8%) | [+1357, +2988]ns | [10049, 10186] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 6960ns | -36.4% | -35.8% | -71.5% | +28.6% | +66.9% | +31.6% | -35.0% | +49.0% |
| 1 | 2 | 8415ns | -35.8% | -34.9% | -76.4% | +33.9% | +13.6% | +40.5% | -46.4% | +19.7% |
| 1 | 3 | 7213ns | -38.9% | -38.0% | -66.4% | +19.2% | +24.6% | +25.0% | -37.0% | +40.6% |
| 1 | 4 | 8801ns | -49.9% | +3.1% | -77.4% | +5.1% | -0.6% | +32.4% | -37.1% | -5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.606 | HIGH- (thermal bounce) |
| satfold-lanes16 | -0.406 | moderate- |
| satfold-lanes16-constl | -0.236 | moderate- |
| satfold-lanes4-idx | -0.409 | moderate- |
| satfold-lanes64 | -0.548 | HIGH- (thermal bounce) |
| satfold-neon | 0.103 | ok |
| satfold-neon8 | -0.776 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.064 | ok |
| satfold-seq | -0.081 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.9ns | 7945.1ns | 0.3% |  |
| satfold-lanes16 | 23.5ns | 5279.5ns | 0.4% |  |
| satfold-lanes16-constl | 26.1ns | 5882.4ns | 0.4% |  |
| satfold-lanes4-idx | 37.2ns | 3742.3ns | 1.0% |  |
| satfold-lanes64 | 21.7ns | 9698.9ns | 0.2% |  |
| satfold-neon | 21.3ns | 9757.7ns | 0.2% |  |
| satfold-neon8 | 21.4ns | 10435.6ns | 0.2% |  |
| satfold-nolaw | 22.7ns | 5239.3ns | 0.4% |  |
| satfold-seq | 23.3ns | 9771.9ns | 0.2% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 6896.8-9196.0 ns)
   6896.8 |#############
   7011.7 |########################################
   7126.7 |####################
   7241.7 |##########################
   7356.6 |
   7471.6 |
   7586.5 |######
   7701.5 |
   7816.5 |
   7931.4 |
   8046.4 |
   8161.3 |
   8276.3 |####################
   8391.3 |##########################
   8506.2 |
   8621.2 |##########################
   8736.1 |####################
   8851.1 |######
   8966.1 |
   9081.0 |######
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 4396.8-8128.1 ns)
   4396.8 |########################################
   4583.4 |
   4769.9 |
   4956.5 |
   5143.0 |
   5329.6 |###############
   5516.2 |
   5702.7 |
   5889.3 |#
   6075.9 |
   6262.4 |
   6449.0 |
   6635.6 |
   6822.1 |
   7008.7 |
   7195.2 |
   7381.8 |
   7568.4 |
   7754.9 |
   7941.5 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 4454.3-9682.5 ns)
   4454.3 |########################################
   4715.7 |
   4977.1 |
   5238.5 |##################
   5499.9 |##
   5761.3 |
   6022.8 |
   6284.2 |
   6545.6 |
   6807.0 |
   7068.4 |##
   7329.8 |##
   7591.2 |
   7852.6 |
   8114.1 |
   8375.5 |
   8636.9 |
   8898.3 |##############
   9159.7 |
   9421.1 |
  (4 below, 2 above range)

satfold-lanes4-idx (n=40, range 1979.9-8016.1 ns)
   1979.9 |########################################
   2281.7 |##################
   2583.5 |
   2885.3 |##
   3187.1 |
   3489.0 |
   3790.8 |
   4092.6 |#####
   4394.4 |
   4696.2 |
   4998.0 |
   5299.8 |
   5601.6 |
   5903.4 |##
   6205.2 |##
   6507.0 |##
   6808.9 |
   7110.7 |#####
   7412.5 |##
   7714.3 |##
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 8287.6-12471.6 ns)
   8287.6 |#############
   8496.8 |#############
   8706.0 |##########################
   8915.2 |########################################
   9124.4 |
   9333.6 |########
   9542.8 |#############
   9752.0 |####
   9961.2 |
  10170.4 |
  10379.6 |
  10588.8 |####
  10798.0 |####
  11007.2 |########
  11216.4 |########
  11425.6 |########
  11634.8 |####
  11844.0 |
  12053.2 |
  12262.4 |
  (3 below, 1 above range)

satfold-neon (n=40, range 8526.1-11688.0 ns)
   8526.1 |
   8684.2 |#####
   8842.3 |############################
   9000.4 |############################
   9158.5 |#####
   9316.6 |######################
   9474.7 |########################################
   9632.8 |#####
   9790.9 |
   9949.0 |#####
  10107.0 |
  10265.1 |
  10423.2 |
  10581.3 |
  10739.4 |
  10897.5 |
  11055.6 |
  11213.7 |
  11371.8 |#################
  11529.9 |#################
  (5 below, 4 above range)

satfold-neon8 (n=40, range 8755.0-11907.2 ns)
   8755.0 |########
   8912.6 |################
   9070.2 |############
   9227.8 |############
   9385.4 |########
   9543.0 |########
   9700.6 |####
   9858.2 |
  10015.9 |
  10173.5 |
  10331.1 |
  10488.7 |
  10646.3 |
  10803.9 |
  10961.5 |
  11119.1 |####
  11276.8 |
  11434.4 |########
  11592.0 |####################
  11749.6 |########################################
  (3 below, 2 above range)

satfold-nolaw (n=40, range 4506.7-7589.1 ns)
   4506.7 |########################################
   4660.8 |
   4815.0 |
   4969.1 |
   5123.2 |
   5277.3 |
   5431.4 |#############
   5585.6 |
   5739.7 |
   5893.8 |
   6047.9 |
   6202.0 |
   6356.2 |
   6510.3 |
   6664.4 |
   6818.5 |
   6972.6 |
   7126.8 |
   7280.9 |
   7435.0 |
  (2 below, 3 above range)

satfold-seq (n=40, range 8271.7-10697.4 ns)
   8271.7 |########
   8393.0 |#################
   8514.3 |
   8635.5 |
   8756.8 |
   8878.1 |
   8999.4 |
   9120.7 |
   9242.0 |
   9363.2 |
   9484.5 |
   9605.8 |####
   9727.1 |####
   9848.4 |####
   9969.7 |##########################
  10090.9 |########################################
  10212.2 |######################
  10333.5 |####
  10454.8 |####
  10576.1 |########
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=40.0% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=35.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=65.8% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=20.8% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=33.1% (high variance, measurements may be unstable)
