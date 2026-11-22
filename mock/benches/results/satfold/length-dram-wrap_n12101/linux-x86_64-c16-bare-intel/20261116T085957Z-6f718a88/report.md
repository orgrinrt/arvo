# The wrapping ceiling over the same 16 MiB column

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.4x slower than the field

satfold-lanes4-idx (1.09 ms) is 2.4x the fastest (456.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-iterfold, satfold-lanes64) are a dead heat (<1%)

satfold-iterfold (456.86 us) and satfold-lanes64 (461.19 us) differ by 0.95%, inside the noise, even though the wider field spreads 139.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (456.86 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 456864.9 ns median
- 6 variants significantly slower than baseline
- Spread: 2.39x (fastest 456864.9 ns, slowest 1092499.1 ns)

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
| Counter frequency | 2112795968 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 449015ns | 457816ns | 415811ns | 451001ns | 476264ns | base |
| satfold-lanes16 | 513438ns | 518654ns | 488831ns | 510740ns | 546136ns | +14.35% |
| satfold-lanes16-constl | 523940ns | 523669ns | 481022ns | 522355ns | 571612ns | +16.69% |
| satfold-lanes4-idx | 1106118ns | 1093119ns | 1089635ns | 1093231ns | 1161265ns | +146.34% |
| satfold-lanes64 | 471182ns | 462155ns | 455538ns | 466451ns | 501020ns | +4.94% |
| satfold-neon | 495106ns | 494174ns | 466745ns | 491699ns | 533691ns | +10.26% |
| satfold-nolaw | 759373ns | 771123ns | 683953ns | 773530ns | 792322ns | +69.12% |
| satfold-seq | 462014ns | 475942ns | 430320ns | 463013ns | 490709ns | +2.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 448133ns | 414725ns | 475357ns | base | 37.438 |
| satfold-lanes16 | 512711ns | 488294ns | 545399ns | +14.41% | 32.723 |
| satfold-lanes16-constl | 523072ns | 480295ns | 570615ns | +16.72% | 32.074 |
| satfold-lanes4-idx | 1105391ns | 1088802ns | 1160608ns | +146.67% | 15.178 |
| satfold-lanes64 | 470334ns | 454520ns | 500164ns | +4.95% | 35.671 |
| satfold-neon | 494340ns | 465950ns | 532816ns | +10.31% | 33.939 |
| satfold-nolaw | 758694ns | 683126ns | 791628ns | +69.30% | 22.113 |
| satfold-seq | 461267ns | 429754ns | 489977ns | +2.93% | 36.372 |

## Performance model

- Peak throughput: **40.454 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 36.722 | 90.8% |
| satfold-lanes16 | 32.406 | 80.1% |
| satfold-lanes16-constl | 32.090 | 79.3% |
| satfold-lanes4-idx | 15.357 | 38.0% |
| satfold-lanes64 | 36.378 | 89.9% |
| satfold-neon | 34.022 | 84.1% |
| satfold-nolaw | 21.774 | 53.8% |
| satfold-seq | 35.308 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 449015ns | 449015ns | base |
| satfold-lanes16 | 513438ns | 513438ns | +14.35% |
| satfold-lanes16-constl | 523940ns | 523940ns | +16.69% |
| satfold-lanes4-idx | 1106118ns | 1106118ns | +146.34% |
| satfold-lanes64 | 471182ns | 471182ns | +4.94% |
| satfold-neon | 495106ns | 495106ns | +10.26% |
| satfold-nolaw | 759373ns | 759373ns | +69.12% |
| satfold-seq | 462014ns | 462014ns | +2.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 456865ns | base | --- | [436455, 459162] | --- | --- | --- | --- |
| satfold-lanes16 | 517716ns | +57619.1ns (+12.6%) | [+32220, +102812]ns | [492126, 523368] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 522819ns | +66764.0ns (+14.6%) | [+64166, +83087]ns | [521277, 524216] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1092499ns | +656802.6ns (+143.8%) | [+639238, +663897]ns | [1090190, 1093966] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 461189ns | +24957.0ns (+5.5%) | [+20925, +30204]ns | [459774, 472801] | YES | 0.0002 | 0.0002 | 0 |
| satfold-neon | 493127ns | +42999.5ns (+9.4%) | [+38705, +46213]ns | [480756, 498632] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 770527ns | +314540.8ns (+68.8%) | [+303101, +330623]ns | [766210, 788933] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 475168ns | no significant difference | [-13266, +40855]ns | [445245, 476317] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 415262ns | +26.0% | +15.7% | +162.3% | +9.7% | +21.7% | +75.4% | +14.7% |
| 1 | 2 | 436227ns | +23.6% | +20.0% | +150.7% | +5.3% | +9.8% | +81.3% | +9.6% |
| 1 | 3 | 464989ns | +5.3% | +12.2% | +134.4% | -1.0% | +3.2% | +64.8% | -7.5% |
| 1 | 4 | 457227ns | +7.5% | +19.1% | +152.9% | +5.7% | +9.2% | +72.5% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.230 | moderate+ |
| satfold-lanes16 | 0.088 | ok |
| satfold-lanes16-constl | -0.038 | ok |
| satfold-lanes4-idx | -0.117 | ok |
| satfold-lanes64 | -0.013 | ok |
| satfold-neon | -0.247 | moderate- |
| satfold-nolaw | -0.387 | moderate- |
| satfold-seq | 0.098 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 115.1ns | 448132.7ns | 0.0% |  |
| satfold-lanes16 | 137.8ns | 512710.5ns | 0.0% |  |
| satfold-lanes16-constl | 145.8ns | 523071.5ns | 0.0% |  |
| satfold-lanes4-idx | 130.7ns | 1105391.0ns | 0.0% |  |
| satfold-lanes64 | 94.9ns | 470333.7ns | 0.0% |  |
| satfold-neon | 96.1ns | 494339.7ns | 0.0% |  |
| satfold-nolaw | 138.8ns | 758694.4ns | 0.0% |  |
| satfold-seq | 132.2ns | 461266.7ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 414724.7-475356.5 ns)
  414724.7 |##########################
  417756.3 |
  420787.9 |
  423819.5 |
  426851.1 |
  429882.7 |######
  432914.3 |####################
  435945.8 |##########################
  438977.4 |######
  442009.0 |
  445040.6 |
  448072.2 |######
  451103.8 |
  454135.4 |##########################
  457167.0 |#################################
  460198.6 |######
  463230.2 |########################################
  466261.8 |#############
  469293.4 |######
  472325.0 |
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 488293.5-545398.7 ns)
  488293.5 |##################################
  491148.8 |########################################
  494004.0 |#####
  496859.3 |
  499714.6 |
  502569.8 |
  505425.1 |
  508280.3 |
  511135.6 |#####
  513990.8 |
  516846.1 |
  519701.4 |###########
  522556.6 |########################################
  525411.9 |
  528267.1 |
  531122.4 |
  533977.6 |######################
  536832.9 |###########
  539688.1 |###########
  542543.4 |#####
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 480295.1-570614.7 ns)
  480295.1 |################
  484811.1 |
  489327.0 |
  493843.0 |
  498359.0 |
  502875.0 |
  507391.0 |
  511906.9 |
  516422.9 |##########
  520938.9 |########################################
  525454.9 |########
  529970.9 |##
  534486.8 |
  539002.8 |
  543518.8 |
  548034.8 |
  552550.8 |
  557066.8 |
  561582.7 |#####
  566098.7 |#####
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1088802.2-1160607.6 ns)
  1088802.2 |########################################
  1092392.5 |####################
  1095982.7 |###########
  1099573.0 |
  1103163.3 |
  1106753.5 |
  1110343.8 |
  1113934.1 |
  1117524.4 |##
  1121114.6 |
  1124704.9 |
  1128295.2 |
  1131885.4 |
  1135475.7 |
  1139066.0 |
  1142656.2 |
  1146246.5 |
  1149836.8 |
  1153427.0 |
  1157017.3 |
  (2 below, 5 above range)

satfold-lanes64 (n=40, range 454520.0-500163.8 ns)
  454520.0 |########################################
  456802.2 |#################
  459084.4 |########################################
  461366.6 |###########
  463648.8 |###########
  465930.9 |#####
  468213.1 |#####
  470495.3 |
  472777.5 |
  475059.7 |#####
  477341.9 |#####
  479624.1 |###########
  481906.2 |#################
  484188.4 |
  486470.6 |#####
  488752.8 |#####
  491035.0 |#####
  493317.2 |
  495599.4 |
  497881.6 |#####
  (3 below, 3 above range)

satfold-neon (n=40, range 465949.6-532815.9 ns)
  465949.6 |
  469292.9 |
  472636.2 |######
  475979.5 |#################################
  479322.8 |########################################
  482666.1 |
  486009.5 |#############
  489352.8 |
  492696.1 |########################################
  496039.4 |####################
  499382.7 |##########################
  502726.0 |
  506069.3 |######
  509412.7 |######
  512756.0 |
  516099.3 |
  519442.6 |######
  522785.9 |######
  526129.2 |######
  529472.5 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 683126.0-791627.6 ns)
  683126.0 |
  688551.0 |
  693976.1 |
  699401.2 |
  704826.3 |
  710251.4 |
  715676.4 |
  721101.5 |
  726526.6 |###########
  731951.7 |####
  737376.8 |
  742801.8 |
  748226.9 |
  753652.0 |
  759077.1 |
  764502.2 |#####################
  769927.2 |##
  775352.3 |
  780777.4 |
  786202.5 |########################################
  (4 below, 2 above range)

satfold-seq (n=40, range 429753.9-489976.9 ns)
  429753.9 |############
  432765.0 |###
  435776.2 |
  438787.3 |
  441798.5 |#########
  444809.6 |##################
  447820.8 |
  450831.9 |
  453843.1 |
  456854.2 |
  459865.4 |
  462876.5 |###
  465887.7 |
  468898.8 |
  471910.0 |###
  474921.1 |########################################
  477932.3 |############
  480943.5 |
  483954.6 |
  486965.8 |
  (4 below, 3 above range)

```
