# The same arms at the longest reduction length over a 16 MiB column: the row where the hand-written arm reaches the load-bandwidth ceiling on the small column, so it is the row where a working set past every cache level can cap it

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 96% (significant)

satfold-neon is -11.05 ms (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 22.4x slower than the field

satfold-seq (11.57 ms) is 22.4x the fastest (515.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon, satfold-lanes64) are a dead heat (<1%)

satfold-neon (515.57 us) and satfold-lanes64 (517.17 us) differ by 0.31%, inside the noise, even though the wider field spreads 2143.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.52)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (639% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 639% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.4x the fastest

Fastest satfold-neon (515.57 us) to slowest satfold-seq (11.57 ms): 22.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 515570.4 ns median (-95.5% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 22.44x (fastest 515570.4 ns, slowest 11569095.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms at the longest reduction length over a 16 MiB column: the row where the hand-written arm reaches the load-bandwidth ceiling on the small column, so it is the row where a working set past every cache level can cap it |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112725069 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 11564733ns | 11562784ns | 11549443ns | 11562523ns | 11586654ns | base |
| satfold-lanes16 | 565075ns | 566433ns | 530143ns | 567663ns | 592242ns | -95.11% |
| satfold-lanes16-constl | 533013ns | 519320ns | 505277ns | 522610ns | 591959ns | -95.39% |
| satfold-lanes4-idx | 1179580ns | 1204125ns | 1109198ns | 1189102ns | 1221395ns | -89.80% |
| satfold-lanes64 | 520087ns | 517716ns | 501244ns | 518293ns | 544313ns | -95.50% |
| satfold-neon | 516548ns | 516568ns | 494283ns | 516054ns | 540297ns | -95.53% |
| satfold-nolaw | 8915969ns | 8896709ns | 8869932ns | 8899623ns | 9011044ns | -22.90% |
| satfold-seq | 11576791ns | 11569904ns | 11527908ns | 11571833ns | 11640549ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 11563963ns | 11548852ns | 11585799ns | base | 1.451 |
| satfold-lanes16 | 564321ns | 529660ns | 591506ns | -95.12% | 29.730 |
| satfold-lanes16-constl | 532222ns | 504676ns | 591140ns | -95.40% | 31.523 |
| satfold-lanes4-idx | 1178769ns | 1108397ns | 1220536ns | -89.81% | 14.233 |
| satfold-lanes64 | 519335ns | 500290ns | 543673ns | -95.51% | 32.305 |
| satfold-neon | 515655ns | 493638ns | 539385ns | -95.54% | 32.536 |
| satfold-nolaw | 8915075ns | 8869043ns | 9010138ns | -22.91% | 1.882 |
| satfold-seq | 11575933ns | 11526990ns | 11639597ns | +0.10% | 1.449 |

## Performance model

- Peak throughput: **33.987 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.451 | 4.3% |
| satfold-lanes16 | 29.664 | 87.3% |
| satfold-lanes16-constl | 32.362 | 95.2% |
| satfold-lanes4-idx | 13.940 | 41.0% |
| satfold-lanes64 | 32.440 | 95.4% |
| satfold-neon | 32.541 | 95.7% |
| satfold-nolaw | 1.886 | 5.5% |
| satfold-seq | 1.450 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 11564733ns | 11564733ns | base |
| satfold-lanes16 | 565075ns | 565075ns | -95.11% |
| satfold-lanes16-constl | 533013ns | 533013ns | -95.39% |
| satfold-lanes4-idx | 1179580ns | 1179580ns | -89.80% |
| satfold-lanes64 | 520087ns | 520087ns | -95.50% |
| satfold-neon | 516548ns | 516548ns | -95.53% |
| satfold-nolaw | 8915969ns | 8915969ns | -22.90% |
| satfold-seq | 11576791ns | 11576791ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 11561886ns | base | --- | [11557834, 11567088] | --- | --- | --- | --- |
| satfold-lanes16 | 565570ns | -11001929.4ns (-95.2%) | [-11008118, -10991011]ns | [563930, 566544] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 518429ns | -11042638.2ns (-95.5%) | [-11044949, -11037992]ns | [510008, 521963] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1203492ns | -10366807.2ns (-89.7%) | [-10394444, -10361019]ns | [1202082, 1205792] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 517170ns | -11043023.6ns (-95.5%) | [-11055363, -11035344]ns | [511281, 527235] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 515570ns | -11048481.4ns (-95.6%) | [-11050481, -11044288]ns | [512662, 518580] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8895889ns | -2667991.0ns (-23.1%) | [-2677200, -2654692]ns | [8885443, 8912941] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 11569096ns | no significant difference | [-7334, +7537]ns | [11561901, 11573004] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11562787ns | -95.0% | -95.5% | -89.6% | -95.7% | -95.6% | -23.2% | -0.0% |
| 1 | 2 | 11572251ns | -95.1% | -94.9% | -89.6% | -95.4% | -95.4% | -22.9% | -0.0% |
| 1 | 3 | 11559038ns | -95.1% | -95.6% | -89.9% | -95.5% | -95.5% | -23.0% | +0.3% |
| 1 | 4 | 11548768ns | -95.4% | -95.6% | -90.4% | -95.4% | -95.7% | -23.2% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.087 | ok |
| satfold-lanes16 | -0.057 | ok |
| satfold-lanes16-constl | -0.271 | moderate- |
| satfold-lanes4-idx | 0.210 | moderate+ |
| satfold-lanes64 | -0.522 | HIGH- (thermal bounce) |
| satfold-neon | -0.055 | ok |
| satfold-nolaw | -0.174 | ok |
| satfold-seq | -0.439 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 204.7ns | 11563963.4ns | 0.0% |  |
| satfold-lanes16 | 115.3ns | 564320.6ns | 0.0% |  |
| satfold-lanes16-constl | 138.8ns | 532222.3ns | 0.0% |  |
| satfold-lanes4-idx | 121.2ns | 1178768.9ns | 0.0% |  |
| satfold-lanes64 | 69.2ns | 519335.1ns | 0.0% |  |
| satfold-neon | 93.7ns | 515655.2ns | 0.0% |  |
| satfold-nolaw | 238.3ns | 8915075.2ns | 0.0% |  |
| satfold-seq | 187.7ns | 11575932.7ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 11548851.6-11585799.0 ns)
  11548851.6 |##########
  11550699.0 |##########
  11552546.4 |########################################
  11554393.7 |####################
  11556241.1 |##########
  11558088.5 |########################################
  11559935.8 |##########
  11561783.2 |##############################
  11563630.6 |####################
  11565478.0 |########################################
  11567325.3 |##############################
  11569172.7 |##########
  11571020.1 |##########
  11572867.4 |##############################
  11574714.8 |
  11576562.2 |##########
  11578409.5 |
  11580256.9 |##########
  11582104.3 |
  11583951.7 |
  (5 below, 2 above range)

satfold-lanes16 (n=40, range 529660.3-591505.9 ns)
  529660.3 |#########
  532752.6 |
  535844.9 |
  538937.1 |
  542029.4 |
  545121.7 |
  548214.0 |###
  551306.3 |
  554398.5 |
  557490.8 |
  560583.1 |############
  563675.4 |########################################
  566767.7 |###
  569859.9 |
  572952.2 |
  576044.5 |########################
  579136.8 |
  582229.1 |###
  585321.4 |
  588413.6 |######
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 504676.4-591140.1 ns)
  504676.4 |######################
  508999.6 |########################################
  513322.7 |####
  517645.9 |##########################
  521969.1 |#################
  526292.3 |
  530615.5 |
  534938.7 |
  539261.8 |
  543585.0 |
  547908.2 |
  552231.4 |
  556554.6 |
  560877.8 |
  565201.0 |
  569524.1 |
  573847.3 |
  578170.5 |
  582493.7 |
  586816.9 |###################################
  (5 below, 2 above range)

satfold-lanes4-idx (n=40, range 1108396.9-1220536.4 ns)
  1108396.9 |################
  1114003.9 |##
  1119610.8 |########
  1125217.8 |
  1130824.8 |
  1136431.8 |
  1142038.7 |
  1147645.7 |##
  1153252.7 |
  1158859.7 |
  1164466.7 |
  1170073.6 |
  1175680.6 |
  1181287.6 |
  1186894.6 |
  1192501.6 |
  1198108.5 |##################
  1203715.5 |########################################
  1209322.5 |
  1214929.5 |#####
  (2 below, 3 above range)

satfold-lanes64 (n=40, range 500289.6-543672.9 ns)
  500289.6 |############################
  502458.8 |######################
  504627.9 |
  506797.1 |#####
  508966.3 |
  511135.4 |
  513304.6 |###########
  515473.8 |############################
  517642.9 |#####
  519812.1 |#####
  521981.2 |
  524150.4 |
  526319.6 |########################################
  528488.7 |######################
  530657.9 |
  532827.1 |
  534996.2 |######################
  537165.4 |
  539334.6 |
  541503.7 |
  (4 below, 2 above range)

satfold-neon (n=40, range 493638.4-539385.4 ns)
  493638.4 |####
  495925.8 |
  498213.1 |##########################
  500500.5 |
  502787.8 |
  505075.2 |
  507362.5 |
  509649.8 |
  511937.2 |########################################
  514224.5 |#############
  516511.9 |######################
  518799.2 |########
  521086.6 |########
  523373.9 |####
  525661.3 |########
  527948.6 |####
  530236.0 |
  532523.3 |
  534810.7 |####
  537098.0 |####
  (3 below, 3 above range)

satfold-nolaw (n=40, range 8869042.6-9010138.3 ns)
  8869042.6 |########################################
  8876097.4 |################################
  8883152.2 |########################################
  8890207.0 |########################
  8897261.8 |########################################
  8904316.5 |########
  8911371.3 |################
  8918426.1 |########
  8925480.9 |########################################
  8932535.7 |########
  8939590.5 |########
  8946645.2 |
  8953700.0 |
  8960754.8 |
  8967809.6 |
  8974864.4 |
  8981919.1 |
  8988973.9 |
  8996028.7 |
  9003083.5 |########
  (3 below, 3 above range)

satfold-seq (n=40, range 11526989.7-11639596.9 ns)
  11526989.7 |#################
  11532620.0 |#####
  11538250.4 |
  11543880.8 |
  11549511.1 |###########
  11555141.5 |###########
  11560771.9 |############################
  11566402.2 |########################################
  11572032.6 |######################
  11577662.9 |
  11583293.3 |
  11588923.7 |###########
  11594554.0 |######################
  11600184.4 |######################
  11605814.7 |
  11611445.1 |
  11617075.5 |
  11622705.8 |
  11628336.2 |
  11633966.6 |
  (4 below, 2 above range)

```
