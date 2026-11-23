# The same arms at the longest reduction length over a 16 MiB column: the row where the hand-written arm reaches the load-bandwidth ceiling on the small column, so it is the row where a working set past every cache level can cap it

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 beats baseline by 96% (significant)

satfold-lanes64 is -11.07 ms (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 23.7x slower than the field

satfold-seq (11.57 ms) is 23.7x the fastest (488.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.56)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-lanes16, satfold-neon, satfold-lanes16-constl, satfold-lanes4-idx} vs {satfold-nolaw, satfold-iterfold, satfold-seq} (709% apart)

The field splits into a fast tier {satfold-lanes64, satfold-lanes16, satfold-neon, satfold-lanes16-constl, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-iterfold, satfold-seq} with a 709% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 23.7x the fastest

Fastest satfold-lanes64 (488.35 us) to slowest satfold-seq (11.57 ms): 23.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-lanes64** at 488349.5 ns median (-95.8% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 23.69x (fastest 488349.5 ns, slowest 11567998.5 ns)

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
| Counter frequency | 2112733120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 11588125ns | 11562315ns | 11541713ns | 11565740ns | 11701692ns | base |
| satfold-lanes16 | 526289ns | 528813ns | 500734ns | 522737ns | 562501ns | -95.46% |
| satfold-lanes16-constl | 562143ns | 535979ns | 496709ns | 560127ns | 633627ns | -95.15% |
| satfold-lanes4-idx | 1142773ns | 1104641ns | 1100878ns | 1123788ns | 1241620ns | -90.14% |
| satfold-lanes64 | 500725ns | 489121ns | 480760ns | 496330ns | 533874ns | -95.68% |
| satfold-neon | 538960ns | 533683ns | 515272ns | 537467ns | 567128ns | -95.35% |
| satfold-nolaw | 8976074ns | 8935227ns | 8912981ns | 8953696ns | 9106302ns | -22.54% |
| satfold-seq | 11595866ns | 11568761ns | 11552985ns | 11568345ns | 11721311ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 11587303ns | 11540955ns | 11700839ns | base | 1.448 |
| satfold-lanes16 | 525676ns | 500182ns | 561925ns | -95.46% | 31.915 |
| satfold-lanes16-constl | 561212ns | 496126ns | 632653ns | -95.16% | 29.895 |
| satfold-lanes4-idx | 1142020ns | 1100039ns | 1240918ns | -90.14% | 14.691 |
| satfold-lanes64 | 499947ns | 479914ns | 533169ns | -95.69% | 33.558 |
| satfold-neon | 538093ns | 514431ns | 566088ns | -95.36% | 31.179 |
| satfold-nolaw | 8975217ns | 8912170ns | 9105495ns | -22.54% | 1.869 |
| satfold-seq | 11595018ns | 11552101ns | 11720467ns | +0.07% | 1.447 |

## Performance model

- Peak throughput: **34.959 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.451 | 4.2% |
| satfold-lanes16 | 31.800 | 91.0% |
| satfold-lanes16-constl | 31.362 | 89.7% |
| satfold-lanes4-idx | 15.200 | 43.5% |
| satfold-lanes64 | 34.355 | 98.3% |
| satfold-neon | 31.487 | 90.1% |
| satfold-nolaw | 1.878 | 5.4% |
| satfold-seq | 1.450 | 4.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 11588125ns | 11588125ns | base |
| satfold-lanes16 | 526289ns | 526289ns | -95.46% |
| satfold-lanes16-constl | 562143ns | 562143ns | -95.15% |
| satfold-lanes4-idx | 1142773ns | 1142773ns | -90.14% |
| satfold-lanes64 | 500725ns | 500725ns | -95.68% |
| satfold-neon | 538960ns | 538960ns | -95.35% |
| satfold-nolaw | 8976074ns | 8976074ns | -22.54% |
| satfold-seq | 11595866ns | 11595866ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 11561447ns | base | --- | [11548374, 11581580] | --- | --- | --- | --- |
| satfold-lanes16 | 527593ns | -11044012.7ns (-95.5%) | [-11048155, -11037660]ns | [506450, 533478] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 534958ns | -11008373.2ns (-95.2%) | [-11034091, -10962524]ns | [530667, 589787] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1103754ns | -10444947.2ns (-90.3%) | [-10457921, -10435503]ns | [1101709, 1117919] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 488350ns | -11067126.6ns (-95.7%) | [-11096636, -11059347]ns | [481916, 516314] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 532827ns | -11031792.8ns (-95.4%) | [-11057617, -11013271]ns | [522564, 553500] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8934236ns | -2625754.1ns (-22.7%) | [-2653177, -2604944]ns | [8925516, 8956927] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 11567998ns | no significant difference | [-4386, +19978]ns | [11562219, 11572543] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11555746ns | -95.6% | -94.9% | -90.2% | -95.5% | -95.5% | -22.6% | -0.0% |
| 1 | 2 | 11550854ns | -95.7% | -95.7% | -90.5% | -95.5% | -95.2% | -22.8% | +0.2% |
| 1 | 3 | 11546104ns | -95.4% | -95.4% | -89.3% | -95.8% | -95.2% | -21.3% | +0.2% |
| 1 | 4 | 11585047ns | -95.3% | -94.6% | -90.5% | -95.8% | -95.5% | -23.0% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.212 | moderate- |
| satfold-lanes16 | 0.253 | moderate+ |
| satfold-lanes16-constl | -0.179 | ok |
| satfold-lanes4-idx | -0.564 | HIGH- (thermal bounce) |
| satfold-lanes64 | 0.277 | moderate+ |
| satfold-neon | -0.244 | moderate- |
| satfold-nolaw | -0.550 | HIGH- (thermal bounce) |
| satfold-seq | -0.176 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 350.2ns | 11587303.3ns | 0.0% |  |
| satfold-lanes16 | 114.8ns | 525676.5ns | 0.0% |  |
| satfold-lanes16-constl | 204.6ns | 561211.6ns | 0.0% |  |
| satfold-lanes4-idx | 184.8ns | 1142020.1ns | 0.0% |  |
| satfold-lanes64 | 123.0ns | 499947.4ns | 0.0% |  |
| satfold-neon | 100.0ns | 538093.2ns | 0.0% |  |
| satfold-nolaw | 294.9ns | 8975216.9ns | 0.0% |  |
| satfold-seq | 361.4ns | 11595018.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 11540955.3-11700839.1 ns)
  11540955.3 |########################################
  11548949.5 |##########
  11556943.7 |##########
  11564937.8 |###
  11572932.0 |##########
  11580926.2 |#########################
  11588920.4 |#######
  11596914.6 |
  11604908.8 |
  11612903.0 |
  11620897.2 |
  11628891.4 |
  11636885.6 |
  11644879.8 |
  11652873.9 |
  11660868.1 |
  11668862.3 |
  11676856.5 |#######
  11684850.7 |#######
  11692844.9 |
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 500182.3-561925.1 ns)
  500182.3 |########################################
  503269.5 |########################################
  506356.6 |########################
  509443.7 |
  512530.9 |
  515618.0 |
  518705.2 |
  521792.3 |
  524879.4 |################################
  527966.6 |########################
  531053.7 |########################
  534140.9 |################
  537228.0 |################################
  540315.1 |################
  543402.3 |
  546489.4 |
  549576.5 |
  552663.7 |
  555750.8 |
  558838.0 |
  (4 below, 5 above range)

satfold-lanes16-constl (n=40, range 496126.4-632653.2 ns)
  496126.4 |#################
  502952.7 |####
  509779.0 |
  516605.4 |
  523431.7 |########
  530258.1 |########################################
  537084.4 |
  543910.8 |
  550737.1 |
  557563.5 |
  564389.8 |
  571216.1 |####
  578042.5 |
  584868.8 |###############################
  591695.2 |
  598521.5 |
  605347.9 |####
  612174.2 |
  619000.5 |
  625826.9 |########################################
  (5 below, 1 above range)

satfold-lanes4-idx (n=40, range 1100039.3-1240917.5 ns)
  1100039.3 |########################################
  1107083.2 |########
  1114127.1 |
  1121171.0 |##
  1128214.9 |
  1135258.8 |
  1142302.7 |
  1149346.6 |##
  1156390.6 |
  1163434.5 |
  1170478.4 |
  1177522.3 |##
  1184566.2 |
  1191610.1 |
  1198654.0 |##
  1205697.9 |####
  1212741.9 |
  1219785.8 |
  1226829.7 |##
  1233873.6 |########
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 479913.5-533168.8 ns)
  479913.5 |########################################
  482576.3 |###
  485239.0 |###
  487901.8 |##########
  490564.6 |###
  493227.3 |
  495890.1 |###
  498552.9 |
  501215.6 |
  503878.4 |###
  506541.2 |
  509203.9 |
  511866.7 |
  514529.4 |######
  517192.2 |##########
  519855.0 |######
  522517.7 |#############
  525180.5 |###
  527843.3 |###
  530506.0 |
  (5 below, 2 above range)

satfold-neon (n=40, range 514430.9-566087.5 ns)
  514430.9 |
  517013.8 |#########################
  519596.6 |##############################
  522179.4 |####################
  524762.2 |#####
  527345.1 |
  529927.9 |#####
  532510.7 |#####
  535093.6 |
  537676.4 |#####
  540259.2 |#####
  542842.0 |
  545424.9 |
  548007.7 |
  550590.5 |
  553173.4 |########################################
  555756.2 |##########
  558339.0 |###############
  560921.8 |#####
  563504.7 |
  (3 below, 3 above range)

satfold-nolaw (n=40, range 8912169.8-9105494.9 ns)
  8912169.8 |########################################
  8921836.0 |###################################
  8931502.3 |####################
  8941168.5 |###############
  8950834.8 |##########
  8960501.0 |#####
  8970167.3 |#####
  8979833.6 |
  8989499.8 |
  8999166.1 |
  9008832.3 |
  9018498.6 |
  9028164.8 |
  9037831.1 |
  9047497.4 |#####
  9057163.6 |
  9066829.9 |
  9076496.1 |###############
  9086162.4 |###############
  9095828.6 |#####
  (3 below, 3 above range)

satfold-seq (n=40, range 11552100.9-11720467.1 ns)
  11552100.9 |##########################
  11560519.2 |########################################
  11568937.5 |####################
  11577355.8 |######
  11585774.2 |
  11594192.5 |
  11602610.8 |
  11611029.1 |###
  11619447.4 |
  11627865.7 |
  11636284.0 |
  11644702.3 |###
  11653120.6 |
  11661538.9 |
  11669957.3 |
  11678375.6 |
  11686793.9 |###
  11695212.2 |
  11703630.5 |######
  11712048.8 |###
  (4 below, 2 above range)

```
