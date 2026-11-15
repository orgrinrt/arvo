# The wrapping ceiling over the same 16 MiB column

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.6x slower than the field

satfold-lanes4-idx (1.10 ms) is 2.6x the fastest (415.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.73)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (415.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 415027.9 ns median
- 6 variants significantly slower than baseline
- Spread: 2.64x (fastest 415027.9 ns, slowest 1095479.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 434634ns | 415599ns | 408512ns | 427081ns | 483415ns | base |
| satfold-lanes16 | 517849ns | 522211ns | 489418ns | 521858ns | 534254ns | +19.15% |
| satfold-lanes16-constl | 522800ns | 520700ns | 492797ns | 518490ns | 565735ns | +20.29% |
| satfold-lanes4-idx | 1120352ns | 1096396ns | 1088735ns | 1104984ns | 1198071ns | +157.77% |
| satfold-lanes64 | 477895ns | 477008ns | 455973ns | 475063ns | 508313ns | +9.95% |
| satfold-neon | 494956ns | 488478ns | 474998ns | 490275ns | 528956ns | +13.88% |
| satfold-nolaw | 719676ns | 748663ns | 629064ns | 729427ns | 781034ns | +65.58% |
| satfold-seq | 448587ns | 456829ns | 413671ns | 452149ns | 472815ns | +3.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 434072ns | 407821ns | 483174ns | base | 38.651 |
| satfold-lanes16 | 516889ns | 488970ns | 533069ns | +19.08% | 32.458 |
| satfold-lanes16-constl | 521761ns | 491624ns | 564985ns | +20.20% | 32.155 |
| satfold-lanes4-idx | 1119545ns | 1087797ns | 1197290ns | +157.92% | 14.986 |
| satfold-lanes64 | 477207ns | 455090ns | 507554ns | +9.94% | 35.157 |
| satfold-neon | 494118ns | 474544ns | 528082ns | +13.83% | 33.954 |
| satfold-nolaw | 718923ns | 628236ns | 780445ns | +65.62% | 23.337 |
| satfold-seq | 447762ns | 412684ns | 471930ns | +3.15% | 37.469 |

## Performance model

- Peak throughput: **41.139 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 40.424 | 98.3% |
| satfold-lanes16 | 32.194 | 78.3% |
| satfold-lanes16-constl | 32.273 | 78.4% |
| satfold-lanes4-idx | 15.315 | 37.2% |
| satfold-lanes64 | 35.219 | 85.6% |
| satfold-neon | 34.428 | 83.7% |
| satfold-nolaw | 22.435 | 54.5% |
| satfold-seq | 36.769 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 434634ns | 434634ns | base |
| satfold-lanes16 | 517849ns | 517849ns | +19.15% |
| satfold-lanes16-constl | 522800ns | 522800ns | +20.29% |
| satfold-lanes4-idx | 1120352ns | 1120352ns | +157.77% |
| satfold-lanes64 | 477895ns | 477895ns | +9.95% |
| satfold-neon | 494956ns | 494956ns | +13.88% |
| satfold-nolaw | 719676ns | 719676ns | +65.58% |
| satfold-seq | 448587ns | 448587ns | +3.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 415028ns | base | --- | [413395, 438645] | --- | --- | --- | --- |
| satfold-lanes16 | 521121ns | +107058.5ns (+25.8%) | [+101974, +110745]ns | [520440, 524150] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 519851ns | +79284.3ns (+19.1%) | [+73555, +108901]ns | [514252, 520720] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1095479ns | +678169.1ns (+163.4%) | [+674326, +681711]ns | [1090810, 1121131] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 476372ns | +48908.5ns (+11.8%) | [+47035, +63617]ns | [461753, 482224] | YES | 0.0026 | 0.0022 | 0 |
| satfold-neon | 487308ns | +70252.8ns (+16.9%) | [+65555, +75725]ns | [482288, 494945] | YES | 0.0001 | 0.0000 | 0 |
| satfold-nolaw | 747816ns | +303320.9ns (+73.1%) | [+264980, +309591]ns | [718364, 768650] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 456281ns | no significant difference | [-750, +33068]ns | [441521, 457893] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 408048ns | +28.0% | +38.1% | +167.1% | +11.6% | +27.6% | +55.0% | +8.2% |
| 1 | 2 | 412572ns | +27.5% | +26.2% | +186.1% | +20.8% | +18.2% | +86.2% | +0.1% |
| 1 | 3 | 476214ns | +2.3% | +8.1% | +135.5% | -3.0% | -0.3% | +63.8% | -4.0% |
| 1 | 4 | 415062ns | +25.5% | +18.6% | +162.6% | +15.5% | +16.1% | +75.6% | +12.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.339 | moderate- |
| satfold-lanes16 | -0.414 | moderate- |
| satfold-lanes16-constl | 0.064 | ok |
| satfold-lanes4-idx | -0.331 | moderate- |
| satfold-lanes64 | -0.728 | HIGH- (thermal bounce) |
| satfold-neon | 0.086 | ok |
| satfold-nolaw | -0.122 | ok |
| satfold-seq | -0.016 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 123.7ns | 434071.9ns | 0.0% |  |
| satfold-lanes16 | 177.1ns | 516889.5ns | 0.0% |  |
| satfold-lanes16-constl | 179.8ns | 521761.2ns | 0.0% |  |
| satfold-lanes4-idx | 172.7ns | 1119545.4ns | 0.0% |  |
| satfold-lanes64 | 139.9ns | 477206.5ns | 0.0% |  |
| satfold-neon | 172.8ns | 494117.7ns | 0.0% |  |
| satfold-nolaw | 165.1ns | 718923.5ns | 0.0% |  |
| satfold-seq | 158.2ns | 447762.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 407820.9-483174.0 ns)
  407820.9 |#####################
  411588.6 |########################################
  415356.2 |##########
  419123.9 |
  422891.5 |###
  426659.2 |
  430426.9 |
  434194.5 |
  437962.2 |
  441729.8 |
  445497.5 |
  449265.1 |#######
  453032.8 |
  456800.4 |###
  460568.1 |
  464335.8 |###
  468103.4 |
  471871.1 |##############
  475638.7 |##################
  479406.4 |
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 488970.3-533069.5 ns)
  488970.3 |
  491175.3 |
  493380.3 |###
  495585.2 |
  497790.2 |######
  499995.1 |
  502200.1 |
  504405.0 |
  506610.0 |
  508815.0 |###
  511019.9 |
  513224.9 |
  515429.8 |
  517634.8 |######
  519839.7 |####################################
  522044.7 |##########
  524249.7 |########################################
  526454.6 |
  528659.6 |###
  530864.5 |
  (6 below, 1 above range)

satfold-lanes16-constl (n=40, range 491624.2-564984.7 ns)
  491624.2 |#########################
  495292.2 |#####
  498960.2 |#####
  502628.2 |
  506296.3 |
  509964.3 |###############
  513632.3 |##############################
  517300.4 |########################################
  520968.4 |###############
  524636.4 |
  528304.4 |##########
  531972.5 |
  535640.5 |
  539308.5 |
  542976.5 |
  546644.6 |#####
  550312.6 |
  553980.6 |#####
  557648.7 |
  561316.7 |###############
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 1087796.6-1197290.4 ns)
  1087796.6 |########################################
  1093271.3 |#############
  1098746.0 |
  1104220.7 |
  1109695.4 |
  1115170.1 |#####
  1120644.8 |#####################
  1126119.4 |
  1131594.1 |
  1137068.8 |
  1142543.5 |
  1148018.2 |
  1153492.9 |
  1158967.6 |
  1164442.2 |##
  1169916.9 |
  1175391.6 |
  1180866.3 |
  1186341.0 |
  1191815.7 |##########
  (2 below, 3 above range)

satfold-lanes64 (n=40, range 455090.3-507553.6 ns)
  455090.3 |#################
  457713.5 |####
  460336.6 |###################################
  462959.8 |########
  465583.0 |
  468206.1 |
  470829.3 |
  473452.5 |####
  476075.6 |########
  478698.8 |#################
  481322.0 |####
  483945.1 |####
  486568.3 |
  489191.5 |
  491814.6 |
  494437.8 |
  497061.0 |########################################
  499684.1 |
  502307.3 |
  504930.5 |
  (4 below, 3 above range)

satfold-neon (n=40, range 474544.4-528081.7 ns)
  474544.4 |#################
  477221.2 |####
  479898.1 |##########################
  482575.0 |####
  485251.8 |########################################
  487928.7 |
  490605.6 |
  493282.5 |#############
  495959.3 |####
  498636.2 |
  501313.1 |########
  503989.9 |
  506666.8 |
  509343.7 |
  512020.5 |
  514697.4 |####
  517374.3 |####
  520051.1 |#################
  522728.0 |####
  525404.9 |
  (4 below, 2 above range)

satfold-nolaw (n=40, range 628235.9-780444.8 ns)
  628235.9 |####################################
  635846.3 |####
  643456.8 |
  651067.2 |
  658677.7 |
  666288.1 |
  673898.6 |
  681509.0 |
  689119.5 |
  696729.9 |
  704340.4 |####
  711950.8 |
  719561.2 |
  727171.7 |########################
  734782.1 |
  742392.6 |
  750003.0 |
  757613.5 |
  765223.9 |########################################
  772834.4 |########################
  (3 below, 4 above range)

satfold-seq (n=40, range 412683.8-471930.2 ns)
  412683.8 |################
  415646.1 |########
  418608.4 |
  421570.7 |
  424533.1 |
  427495.4 |
  430457.7 |
  433420.0 |########
  436382.4 |################
  439344.7 |########################################
  442307.0 |################
  445269.3 |
  448231.6 |
  451194.0 |
  454156.3 |########################################
  457118.6 |################################
  460080.9 |################
  463043.3 |################################
  466005.6 |########################
  468967.9 |########
  (5 below, 3 above range)

```
