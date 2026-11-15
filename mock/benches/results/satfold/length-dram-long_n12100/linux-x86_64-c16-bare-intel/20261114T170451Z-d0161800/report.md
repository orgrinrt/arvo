# The same arms at the longest reduction length over a 16 MiB column: the row where the hand-written arm reaches the load-bandwidth ceiling on the small column, so it is the row where a working set past every cache level can cap it

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (11.58 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 498.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon beats baseline by 96% (significant)

satfold-neon is -11.10 ms (96%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 23.2x slower than the field

satfold-iterfold (11.58 ms) is 23.2x the fastest (498.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.69)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (707% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 707% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 23.2x the fastest

Fastest satfold-neon (498.74 us) to slowest satfold-iterfold (11.58 ms): 23.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 498739.8 ns median (-95.7% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 23.23x (fastest 498739.8 ns, slowest 11583606.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 11586081ns | 11584373ns | 11572835ns | 11584409ns | 11604342ns | base |
| satfold-lanes16 | 535723ns | 529868ns | 499485ns | 531704ns | 584018ns | -95.38% |
| satfold-lanes16-constl | 543490ns | 557908ns | 499200ns | 545113ns | 582911ns | -95.31% |
| satfold-lanes4-idx | 1115535ns | 1103666ns | 1083751ns | 1103746ns | 1182686ns | -90.37% |
| satfold-lanes64 | 511837ns | 518843ns | 479875ns | 513411ns | 539079ns | -95.58% |
| satfold-neon | 496176ns | 499842ns | 480527ns | 495763ns | 513066ns | -95.72% |
| satfold-nolaw | 8895131ns | 8897933ns | 8874433ns | 8897586ns | 8908462ns | -23.23% |
| satfold-seq | 11573636ns | 11580270ns | 11536421ns | 11577827ns | 11598280ns | -0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 11585234ns | 11571996ns | 11603416ns | base | 1.448 |
| satfold-lanes16 | 535081ns | 498663ns | 583401ns | -95.38% | 31.355 |
| satfold-lanes16-constl | 542696ns | 498702ns | 581781ns | -95.32% | 30.915 |
| satfold-lanes4-idx | 1114758ns | 1082986ns | 1181854ns | -90.38% | 15.050 |
| satfold-lanes64 | 511099ns | 478968ns | 538363ns | -95.59% | 32.826 |
| satfold-neon | 495226ns | 479901ns | 511966ns | -95.73% | 33.878 |
| satfold-nolaw | 8894192ns | 8873302ns | 8907685ns | -23.23% | 1.886 |
| satfold-seq | 11572951ns | 11535755ns | 11597586ns | -0.11% | 1.450 |

## Performance model

- Peak throughput: **35.028 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.448 | 4.1% |
| satfold-lanes16 | 31.722 | 90.6% |
| satfold-lanes16-constl | 30.121 | 86.0% |
| satfold-lanes4-idx | 15.212 | 43.4% |
| satfold-lanes64 | 32.397 | 92.5% |
| satfold-neon | 33.639 | 96.0% |
| satfold-nolaw | 1.886 | 5.4% |
| satfold-seq | 1.449 | 4.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 11586081ns | 11586081ns | base |
| satfold-lanes16 | 535723ns | 535723ns | -95.38% |
| satfold-lanes16-constl | 543490ns | 543490ns | -95.31% |
| satfold-lanes4-idx | 1115535ns | 1115535ns | -90.37% |
| satfold-lanes64 | 511837ns | 511837ns | -95.58% |
| satfold-neon | 496176ns | 496176ns | -95.72% |
| satfold-nolaw | 8895131ns | 8895131ns | -23.23% |
| satfold-seq | 11573636ns | 11573636ns | -0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 11583606ns | base | --- | [11576416, 11588620] | --- | --- | --- | --- |
| satfold-lanes16 | 528885ns | -11048316.4ns (-95.4%) | [-11065959, -11041854]ns | [508321, 538230] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 556985ns | -11029318.5ns (-95.2%) | [-11071937, -11018131]ns | [506016, 558815] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1102881ns | -10479893.5ns (-90.5%) | [-10491345, -10470363]ns | [1099675, 1107228] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 517857ns | -11066064.3ns (-95.5%) | [-11077280, -11052874]ns | [513585, 522624] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 498740ns | -11096723.9ns (-95.8%) | [-11099277, -11091160]ns | [486619, 499434] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8896991ns | -2690770.9ns (-23.2%) | [-2693485, -2685774]ns | [8891160, 8902301] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 11579580ns | -9266.4ns (-0.1%) | [-15988, -3656]ns | [11574201, 11585422] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11575314ns | -95.6% | -95.5% | -90.5% | -95.5% | -95.6% | -23.2% | -0.3% |
| 1 | 2 | 11586982ns | -95.0% | -95.2% | -89.8% | -95.6% | -95.8% | -23.2% | -0.1% |
| 1 | 3 | 11601430ns | -95.7% | -95.7% | -90.5% | -95.9% | -95.7% | -23.2% | -0.0% |
| 1 | 4 | 11576314ns | -95.4% | -95.0% | -90.4% | -95.5% | -95.9% | -23.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.292 | moderate- |
| satfold-lanes16 | -0.677 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.687 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.475 | moderate- |
| satfold-lanes64 | -0.424 | moderate- |
| satfold-neon | -0.497 | moderate- |
| satfold-nolaw | -0.230 | moderate- |
| satfold-seq | 0.109 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 233.5ns | 11585234.3ns | 0.0% |  |
| satfold-lanes16 | 126.8ns | 535080.7ns | 0.0% |  |
| satfold-lanes16-constl | 169.3ns | 542695.8ns | 0.0% |  |
| satfold-lanes4-idx | 149.2ns | 1114757.9ns | 0.0% |  |
| satfold-lanes64 | 79.1ns | 511098.5ns | 0.0% |  |
| satfold-neon | 74.5ns | 495225.6ns | 0.0% |  |
| satfold-nolaw | 202.2ns | 8894191.6ns | 0.0% |  |
| satfold-seq | 255.5ns | 11572951.1ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 11571996.1-11603416.1 ns)
  11571996.1 |#####
  11573567.1 |###########
  11575138.1 |########################################
  11576709.1 |###########
  11578280.1 |
  11579851.1 |###########
  11581422.1 |
  11582993.1 |######################
  11584564.1 |
  11586135.1 |###########
  11587706.1 |###########
  11589277.1 |#####
  11590848.1 |
  11592419.1 |
  11593990.1 |#####
  11595561.1 |#################
  11597132.1 |
  11598703.1 |#################
  11600274.1 |
  11601845.1 |#####
  (5 below, 4 above range)

satfold-lanes16 (n=40, range 498663.3-583400.6 ns)
  498663.3 |#################
  502900.2 |####
  507137.0 |###################################
  511373.9 |
  515610.8 |
  519847.6 |
  524084.5 |
  528321.4 |########################################
  532558.2 |
  536795.1 |
  541032.0 |
  545268.8 |####
  549505.7 |
  553742.6 |
  557979.4 |
  562216.3 |
  566453.2 |####
  570690.0 |###############################
  574926.9 |########
  579163.8 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 498701.5-581781.4 ns)
  498701.5 |########################################
  502855.5 |########
  507009.5 |
  511163.5 |
  515317.5 |
  519471.5 |
  523625.5 |
  527779.5 |
  531933.5 |
  536087.5 |
  540241.5 |
  544395.5 |####
  548549.5 |
  552703.5 |#############
  556857.5 |########################################
  561011.5 |
  565165.5 |
  569319.5 |####
  573473.4 |####
  577627.4 |###################################
  (4 below, 2 above range)

satfold-lanes4-idx (n=40, range 1082986.3-1181854.0 ns)
  1082986.3 |############
  1087929.7 |
  1092873.1 |####################
  1097816.5 |########################
  1102759.9 |########################################
  1107703.2 |################
  1112646.6 |
  1117590.0 |
  1122533.4 |
  1127476.8 |
  1132420.2 |
  1137363.5 |
  1142306.9 |
  1147250.3 |
  1152193.7 |####
  1157137.1 |
  1162080.5 |####
  1167023.8 |
  1171967.2 |
  1176910.6 |
  (5 below, 5 above range)

satfold-lanes64 (n=40, range 478968.5-538362.8 ns)
  478968.5 |############################
  481938.2 |#################
  484907.9 |
  487877.6 |
  490847.3 |#####
  493817.0 |
  496786.7 |
  499756.5 |
  502726.2 |
  505695.9 |
  508665.6 |
  511635.3 |#####
  514605.0 |##################################
  517574.7 |#####
  520544.5 |##################################
  523514.2 |########################################
  526483.9 |###########
  529453.6 |###########
  532423.3 |
  535393.0 |
  (4 below, 2 above range)

satfold-neon (n=40, range 479900.5-511965.6 ns)
  479900.5 |###########
  481503.8 |#####
  483107.1 |
  484710.3 |######################
  486313.6 |##################################
  487916.8 |
  489520.1 |
  491123.3 |
  492726.6 |
  494329.8 |
  495933.1 |
  497536.3 |########################################
  499139.6 |#################
  500742.8 |
  502346.1 |
  503949.3 |##################################
  505552.6 |###########
  507155.8 |###########
  508759.1 |
  510362.3 |
  (5 below, 2 above range)

satfold-nolaw (n=40, range 8873302.5-8907685.3 ns)
  8873302.5 |################
  8875021.6 |########
  8876740.8 |
  8878459.9 |
  8880179.0 |
  8881898.2 |
  8883617.3 |################
  8885336.5 |########
  8887055.6 |
  8888774.8 |########################
  8890493.9 |################
  8892213.0 |########################
  8893932.2 |########
  8895651.3 |########
  8897370.5 |########
  8899089.6 |################
  8900808.8 |########################
  8902527.9 |########################
  8904247.1 |########################################
  8905966.2 |################
  (4 below, 4 above range)

satfold-seq (n=40, range 11535754.9-11597586.4 ns)
  11535754.9 |################
  11538846.5 |
  11541938.0 |########################
  11545029.6 |
  11548121.2 |
  11551212.8 |
  11554304.3 |
  11557395.9 |
  11560487.5 |
  11563579.1 |
  11566670.7 |########
  11569762.2 |################
  11572853.8 |################
  11575945.4 |########################################
  11579037.0 |########################
  11582128.5 |########################
  11585220.1 |########################################
  11588311.7 |########
  11591403.3 |
  11594494.9 |########################################
  (5 below, 3 above range)

```
