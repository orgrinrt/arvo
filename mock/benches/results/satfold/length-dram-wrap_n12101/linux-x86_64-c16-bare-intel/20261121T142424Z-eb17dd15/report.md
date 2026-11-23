# The wrapping ceiling over the same 16 MiB column

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.6x slower than the field

satfold-lanes4-idx (1.11 ms) is 2.6x the fastest (424.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.61)

satfold-neon's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (424.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 424991.2 ns median
- 7 variants significantly slower than baseline
- Spread: 2.60x (fastest 424991.2 ns, slowest 1106929.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The wrapping ceiling over the same 16 MiB column |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112754634 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 438479ns | 425421ns | 415563ns | 433730ns | 475643ns | base |
| satfold-lanes16 | 532067ns | 528489ns | 497376ns | 530264ns | 572166ns | +21.34% |
| satfold-lanes16-constl | 532581ns | 529001ns | 485187ns | 532611ns | 579886ns | +21.46% |
| satfold-lanes4-idx | 1140211ns | 1107735ns | 1084079ns | 1130946ns | 1224137ns | +160.04% |
| satfold-lanes64 | 487132ns | 479247ns | 455660ns | 482324ns | 533026ns | +11.10% |
| satfold-neon | 496541ns | 490735ns | 484650ns | 491699ns | 522958ns | +13.24% |
| satfold-nolaw | 737190ns | 753172ns | 676766ns | 740261ns | 788401ns | +68.12% |
| satfold-seq | 462827ns | 460299ns | 427008ns | 459273ns | 509307ns | +5.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 437937ns | 414956ns | 474964ns | base | 38.310 |
| satfold-lanes16 | 531452ns | 496793ns | 571405ns | +21.35% | 31.569 |
| satfold-lanes16-constl | 531964ns | 484482ns | 579322ns | +21.47% | 31.538 |
| satfold-lanes4-idx | 1139592ns | 1083273ns | 1223753ns | +160.22% | 14.722 |
| satfold-lanes64 | 486248ns | 454686ns | 532023ns | +11.03% | 34.503 |
| satfold-neon | 495685ns | 483771ns | 521988ns | +13.19% | 33.847 |
| satfold-nolaw | 736521ns | 676186ns | 787795ns | +68.18% | 22.779 |
| satfold-seq | 462058ns | 426410ns | 508242ns | +5.51% | 36.310 |

## Performance model

- Peak throughput: **40.431 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 39.477 | 97.6% |
| satfold-lanes16 | 31.782 | 78.6% |
| satfold-lanes16-constl | 31.755 | 78.5% |
| satfold-lanes4-idx | 15.157 | 37.5% |
| satfold-lanes64 | 35.068 | 86.7% |
| satfold-neon | 34.228 | 84.7% |
| satfold-nolaw | 22.306 | 55.2% |
| satfold-seq | 36.508 | 90.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 438479ns | 438479ns | base |
| satfold-lanes16 | 532067ns | 532067ns | +21.34% |
| satfold-lanes16-constl | 532581ns | 532581ns | +21.46% |
| satfold-lanes4-idx | 1140211ns | 1140211ns | +160.04% |
| satfold-lanes64 | 487132ns | 487132ns | +11.10% |
| satfold-neon | 496541ns | 496541ns | +13.24% |
| satfold-nolaw | 737190ns | 737190ns | +68.12% |
| satfold-seq | 462827ns | 462827ns | +5.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 424991ns | base | --- | [421598, 452967] | --- | --- | --- | --- |
| satfold-lanes16 | 527891ns | +87465.2ns (+20.6%) | [+78071, +98250]ns | [520257, 531710] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 528329ns | +102463.9ns (+24.1%) | [+78788, +108953]ns | [523571, 539847] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1106930ns | +674361.9ns (+158.7%) | [+666114, +737517]ns | [1086360, 1190024] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 478419ns | +43185.8ns (+10.2%) | [+32300, +54368]ns | [474449, 489431] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 490156ns | +67553.2ns (+15.9%) | [+56300, +70975]ns | [488410, 492604] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 752130ns | +296367.7ns (+69.7%) | [+280526, +331805]ns | [718228, 755041] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 459554ns | +14303.1ns (+3.4%) | [+1358, +53583]ns | [434390, 473337] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 422632ns | +23.4% | +24.9% | +185.2% | +12.3% | +16.0% | +69.6% | +1.0% |
| 1 | 2 | 415006ns | +37.1% | +27.0% | +184.0% | +25.9% | +17.7% | +81.6% | +13.6% |
| 1 | 3 | 423476ns | +20.5% | +14.5% | +156.7% | +7.5% | +17.3% | +84.6% | +19.0% |
| 1 | 4 | 456218ns | +16.3% | +24.0% | +137.9% | +5.6% | +6.1% | +57.1% | -5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.022 | ok |
| satfold-lanes16 | -0.603 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.509 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | 0.290 | moderate+ |
| satfold-lanes64 | -0.580 | HIGH- (thermal bounce) |
| satfold-neon | -0.607 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.283 | moderate- |
| satfold-seq | -0.263 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 121.5ns | 437936.7ns | 0.0% |  |
| satfold-lanes16 | 141.9ns | 531451.6ns | 0.0% |  |
| satfold-lanes16-constl | 129.8ns | 531964.0ns | 0.0% |  |
| satfold-lanes4-idx | 138.0ns | 1139592.1ns | 0.0% |  |
| satfold-lanes64 | 155.9ns | 486247.5ns | 0.0% |  |
| satfold-neon | 143.6ns | 495685.0ns | 0.0% |  |
| satfold-nolaw | 140.3ns | 736520.7ns | 0.0% |  |
| satfold-seq | 117.3ns | 462058.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 414956.2-474963.7 ns)
  414956.2 |####################
  417956.5 |##########################
  420956.9 |########################################
  423957.3 |##########################
  426957.7 |######
  429958.0 |######
  432958.4 |
  435958.8 |
  438959.2 |
  441959.6 |######
  444959.9 |
  447960.3 |
  450960.7 |##########################
  453961.1 |#############
  456961.4 |####################
  459961.8 |######
  462962.2 |
  465962.6 |######
  468962.9 |#############
  471963.3 |
  (5 below, 2 above range)

satfold-lanes16 (n=40, range 496792.6-571405.1 ns)
  496792.6 |################
  500523.3 |########
  504253.9 |
  507984.5 |################
  511715.1 |################################
  515445.7 |########
  519176.4 |########################################
  522907.0 |################
  526637.6 |################
  530368.2 |########################################
  534098.9 |########
  537829.5 |
  541560.1 |
  545290.7 |########
  549021.3 |
  552752.0 |
  556482.6 |
  560213.2 |################
  563943.8 |########################
  567674.5 |################################
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 484482.2-579322.2 ns)
  484482.2 |#################################
  489224.2 |
  493966.2 |
  498708.2 |
  503450.2 |######
  508192.2 |
  512934.2 |######
  517676.2 |#############
  522418.2 |########################################
  527160.2 |##########################
  531902.2 |######
  536644.2 |##########################
  541386.2 |
  546128.2 |
  550870.2 |
  555612.2 |
  560354.2 |####################
  565096.2 |#################################
  569838.2 |######
  574580.2 |######
  (4 below, 2 above range)

satfold-lanes4-idx (n=40, range 1083273.4-1223752.6 ns)
  1083273.4 |########################################
  1090297.4 |##
  1097321.3 |
  1104345.3 |#####
  1111369.3 |##
  1118393.2 |
  1125417.2 |
  1132441.1 |##
  1139465.1 |
  1146489.0 |
  1153513.0 |
  1160536.9 |##
  1167560.9 |
  1174584.9 |
  1181608.8 |
  1188632.8 |####################
  1195656.7 |##
  1202680.7 |##############
  1209704.6 |#####
  1216728.6 |
  (4 below, 1 above range)

satfold-lanes64 (n=40, range 454686.5-532023.1 ns)
  454686.5 |#################
  458553.3 |####
  462420.1 |####
  466287.0 |
  470153.8 |
  474020.6 |########################################
  477887.5 |#############
  481754.3 |#############
  485621.1 |
  489488.0 |
  493354.8 |#################
  497221.6 |
  501088.5 |
  504955.3 |
  508822.1 |
  512689.0 |########
  516555.8 |####
  520422.6 |#################
  524289.5 |####
  528156.3 |
  (5 below, 2 above range)

satfold-neon (n=40, range 483771.3-521988.2 ns)
  483771.3 |#################
  485682.1 |########
  487593.0 |###############################
  489503.8 |########################################
  491414.7 |
  493325.5 |####
  495236.4 |#################
  497147.2 |#############
  499058.1 |####
  500968.9 |
  502879.8 |####
  504790.6 |
  506701.5 |
  508612.3 |
  510523.1 |
  512434.0 |
  514344.8 |
  516255.7 |
  518166.5 |
  520077.4 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 676185.7-787794.7 ns)
  676185.7 |
  681766.2 |######
  687346.6 |
  692927.1 |####################
  698507.5 |#############
  704088.0 |#############
  709668.4 |######
  715248.9 |####################
  720829.3 |##########################
  726409.8 |
  731990.2 |
  737570.7 |
  743151.1 |
  748731.6 |########################################
  754312.0 |##########################
  759892.5 |
  765472.9 |######
  771053.4 |
  776633.8 |####################
  782214.3 |####################
  (3 below, 4 above range)

satfold-seq (n=40, range 426409.9-508242.0 ns)
  426409.9 |##################################
  430501.5 |######################
  434593.1 |###########
  438684.7 |#####
  442776.3 |
  446867.9 |###########
  450959.5 |#####
  455051.1 |
  459142.7 |
  463234.3 |#####
  467325.9 |######################
  471417.5 |#################
  475509.1 |
  479600.8 |
  483692.4 |
  487784.0 |#####
  491875.6 |#####
  495967.2 |
  500058.8 |########################################
  504150.4 |
  (4 below, 3 above range)

```
