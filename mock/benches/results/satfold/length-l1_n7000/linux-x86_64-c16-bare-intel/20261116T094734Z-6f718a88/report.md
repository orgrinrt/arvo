# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 beats baseline by 86% (significant)

satfold-lanes64 is -12.12 us (86%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 8.8x slower than the field

satfold-seq (17.20 us) is 8.8x the fastest (1.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 is fastest but the noisiest (CV 113.2%)

satfold-neon8 wins on median (1.95 us) yet has the highest variance (CV 113.2%), while satfold-seq is the steadiest (CV 5.0%, 17.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.8x the fastest

Fastest satfold-neon8 (1.95 us) to slowest satfold-seq (17.20 us): 8.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-neon is inconsistent: worst-20% is 4.0x its best-20%

satfold-neon's best 20% of batches run at 1.58 us but its worst 20% at 6.40 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 1946.8 ns median (-86.2% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 8.84x (fastest 1946.8 ns, slowest 17202.3 ns)

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
| satfold-iterfold | 14935ns | 14143ns | 14117ns | 14417ns | 17308ns | base |
| satfold-lanes16 | 4680ns | 3152ns | 3143ns | 3349ns | 10208ns | -68.67% |
| satfold-lanes16-constl | 4736ns | 3140ns | 2642ns | 3737ns | 9827ns | -68.29% |
| satfold-lanes4-idx | 4071ns | 3890ns | 1824ns | 3853ns | 6973ns | -72.74% |
| satfold-lanes64 | 3914ns | 2033ns | 1994ns | 3395ns | 7392ns | -73.79% |
| satfold-neon | 3839ns | 3854ns | 1639ns | 3625ns | 6679ns | -74.30% |
| satfold-neon8 | 3932ns | 2013ns | 1986ns | 3476ns | 7247ns | -73.67% |
| satfold-nolaw | 6881ns | 7467ns | 6112ns | 6867ns | 7692ns | -53.93% |
| satfold-seq | 17422ns | 17279ns | 17257ns | 17279ns | 18016ns | +16.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 14861ns | 14052ns | 17217ns | base | 2.205 |
| satfold-lanes16 | 4570ns | 3081ns | 9925ns | -69.25% | 7.170 |
| satfold-lanes16-constl | 4628ns | 2589ns | 9568ns | -68.86% | 7.080 |
| satfold-lanes4-idx | 3947ns | 1773ns | 6728ns | -73.44% | 8.302 |
| satfold-lanes64 | 3768ns | 1924ns | 7097ns | -74.64% | 8.696 |
| satfold-neon | 3701ns | 1585ns | 6403ns | -75.09% | 8.854 |
| satfold-neon8 | 3790ns | 1920ns | 6953ns | -74.49% | 8.645 |
| satfold-nolaw | 6809ns | 6053ns | 7599ns | -54.18% | 4.812 |
| satfold-seq | 17340ns | 17181ns | 17916ns | +16.68% | 1.890 |

## Performance model

- Peak throughput: **20.675 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.328 | 11.3% |
| satfold-lanes16 | 10.627 | 51.4% |
| satfold-lanes16-constl | 10.648 | 51.5% |
| satfold-lanes4-idx | 8.669 | 41.9% |
| satfold-lanes64 | 16.661 | 80.6% |
| satfold-neon | 8.794 | 42.5% |
| satfold-neon8 | 16.832 | 81.4% |
| satfold-nolaw | 4.431 | 21.4% |
| satfold-seq | 1.905 | 9.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 14935ns | 14935ns | base |
| satfold-lanes16 | 4680ns | 4680ns | -68.67% |
| satfold-lanes16-constl | 4736ns | 4736ns | -68.29% |
| satfold-lanes4-idx | 4071ns | 4071ns | -72.74% |
| satfold-lanes64 | 3914ns | 3914ns | -73.79% |
| satfold-neon | 3839ns | 3839ns | -74.30% |
| satfold-neon8 | 3932ns | 3932ns | -73.67% |
| satfold-nolaw | 6881ns | 6881ns | -53.93% |
| satfold-seq | 17422ns | 17422ns | +16.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 14076ns | base | --- | [14072, 14084] | --- | --- | --- | --- |
| satfold-lanes16 | 3083ns | -10982.8ns (-78.0%) | [-10992, -10967]ns | [3082, 3095] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3077ns | -10995.5ns (-78.1%) | [-11522, -10986]ns | [3076, 3089] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 3780ns | -11691.8ns (-83.1%) | [-11907, -9084]ns | [2167, 5384] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1967ns | -12119.8ns (-86.1%) | [-12137, -9992]ns | [1947, 5182] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 3726ns | -11947.6ns (-84.9%) | [-12297, -9806]ns | [1764, 5283] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1947ns | -12109.8ns (-86.0%) | [-12133, -10221]ns | [1936, 5445] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7396ns | -6799.8ns (-48.3%) | [-8013, -6660]ns | [6070, 7417] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17202ns | +3126.1ns (+22.2%) | [+3111, +3132]ns | [17195, 17206] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14079ns | -78.1% | -78.1% | -72.5% | -86.1% | -79.4% | -70.7% | -47.3% | +22.2% |
| 1 | 2 | 14063ns | -78.0% | -82.1% | -64.5% | -86.3% | -67.0% | -86.3% | -56.9% | +22.3% |
| 1 | 3 | 17203ns | -82.1% | -82.1% | -79.9% | -88.5% | -81.0% | -88.8% | -64.7% | -0.0% |
| 1 | 4 | 14073ns | -78.1% | -78.1% | -84.6% | -63.3% | -81.6% | -86.1% | -47.1% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.419 | moderate- |
| satfold-lanes16 | -0.260 | moderate- |
| satfold-lanes16-constl | -0.409 | moderate- |
| satfold-lanes4-idx | 0.087 | ok |
| satfold-lanes64 | -0.073 | ok |
| satfold-neon | -0.255 | moderate- |
| satfold-neon8 | -0.087 | ok |
| satfold-nolaw | -0.250 | moderate- |
| satfold-seq | -0.187 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.6ns | 14861.0ns | 0.1% |  |
| satfold-lanes16 | 35.3ns | 4570.0ns | 0.8% |  |
| satfold-lanes16-constl | 35.8ns | 4628.4ns | 0.8% |  |
| satfold-lanes4-idx | 43.8ns | 3947.0ns | 1.1% |  |
| satfold-lanes64 | 46.2ns | 3768.3ns | 1.2% |  |
| satfold-neon | 45.0ns | 3701.1ns | 1.2% |  |
| satfold-neon8 | 46.5ns | 3790.4ns | 1.2% |  |
| satfold-nolaw | 22.1ns | 6809.1ns | 0.3% |  |
| satfold-seq | 23.9ns | 17340.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14052.2-17217.4 ns)
  14052.2 |########################################
  14210.4 |#
  14368.7 |
  14527.0 |
  14685.2 |
  14843.5 |
  15001.7 |
  15160.0 |
  15318.3 |
  15476.5 |
  15634.8 |
  15793.0 |
  15951.3 |
  16109.6 |
  16267.8 |
  16426.1 |
  16584.3 |
  16742.6 |
  16900.9 |
  17059.1 |#########
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 3080.6-9925.0 ns)
   3080.6 |########################################
   3422.8 |
   3765.0 |#
   4107.2 |
   4449.4 |#
   4791.7 |
   5133.9 |
   5476.1 |#
   5818.3 |
   6160.5 |
   6502.8 |
   6845.0 |
   7187.2 |#
   7529.4 |
   7871.7 |
   8213.9 |
   8556.1 |
   8898.3 |
   9240.5 |#
   9582.8 |##
  (1 below, 4 above range)

satfold-lanes16-constl (n=40, range 2589.5-9567.9 ns)
   2589.5 |
   2938.4 |########################################
   3287.3 |
   3636.2 |
   3985.2 |
   4334.1 |#
   4683.0 |
   5031.9 |
   5380.9 |
   5729.8 |#
   6078.7 |
   6427.6 |
   6776.5 |#
   7125.5 |
   7474.4 |
   7823.3 |
   8172.2 |
   8521.1 |###
   8870.1 |###
   9219.0 |###
  (7 below, 3 above range)

satfold-lanes4-idx (n=40, range 1773.0-6727.8 ns)
   1773.0 |########################################
   2020.8 |##########################
   2268.5 |
   2516.2 |########
   2764.0 |
   3011.7 |
   3259.4 |
   3507.2 |
   3754.9 |
   4002.7 |
   4250.4 |
   4498.1 |
   4745.9 |####
   4993.6 |######################
   5241.3 |
   5489.1 |#################
   5736.8 |########
   5984.6 |########
   6232.3 |########
   6480.0 |####
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 1924.5-7097.3 ns)
   1924.5 |########################################
   2183.1 |##
   2441.8 |
   2700.4 |
   2959.1 |
   3217.7 |
   3476.3 |##
   3735.0 |
   3993.6 |
   4252.3 |
   4510.9 |
   4769.5 |####
   5028.2 |
   5286.8 |####
   5545.5 |
   5804.1 |####
   6062.7 |##
   6321.4 |##
   6580.0 |##
   6838.6 |#########
  (5 below, 3 above range)

satfold-neon (n=40, range 1584.9-6402.5 ns)
   1584.9 |########################################
   1825.8 |####################
   2066.7 |
   2307.5 |
   2548.4 |
   2789.3 |
   3030.2 |####
   3271.1 |
   3511.9 |
   3752.8 |
   3993.7 |####
   4234.6 |####
   4475.5 |
   4716.4 |####
   4957.2 |############
   5198.1 |################
   5439.0 |####
   5679.9 |############
   5920.8 |####
   6161.7 |####
  (4 below, 4 above range)

satfold-neon8 (n=40, range 1920.4-6953.5 ns)
   1920.4 |########################################
   2172.1 |
   2423.7 |
   2675.4 |##
   2927.0 |
   3178.7 |
   3430.3 |
   3682.0 |
   3933.6 |##
   4185.3 |
   4436.9 |
   4688.6 |##
   4940.2 |
   5191.9 |##
   5443.5 |##
   5695.2 |##
   5946.9 |
   6198.5 |######
   6450.2 |####
   6701.8 |
  (3 below, 7 above range)

satfold-nolaw (n=40, range 6053.1-7598.6 ns)
   6053.1 |########################################
   6130.4 |
   6207.7 |
   6284.9 |
   6362.2 |
   6439.5 |
   6516.8 |
   6594.0 |
   6671.3 |
   6748.6 |
   6825.9 |
   6903.1 |
   6980.4 |
   7057.7 |
   7135.0 |
   7212.2 |
   7289.5 |
   7366.8 |#####################################
   7444.1 |################
   7521.3 |
  (4 below, 1 above range)

satfold-seq (n=40, range 17181.3-17916.2 ns)
  17181.3 |########################################
  17218.1 |##
  17254.8 |#
  17291.5 |
  17328.3 |
  17365.0 |
  17401.8 |
  17438.5 |
  17475.3 |
  17512.0 |
  17548.7 |
  17585.5 |
  17622.2 |
  17659.0 |
  17695.7 |
  17732.5 |
  17769.2 |
  17805.9 |
  17842.7 |
  17879.4 |
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=60.3% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=59.5% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=52.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes64**: CV=58.7% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-neon**: CV=55.0% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-neon8**: CV=58.2% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.6x (possible bimodal distribution)
