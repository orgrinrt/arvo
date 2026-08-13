# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 60% faster than the next best (satfold-neon)

satfold-lanes4-idx (2.46 us) leads satfold-neon (3.93 us) by 60%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 90% (significant)

satfold-lanes4-idx is -23.93 us (90%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 10.9x slower than the field

satfold-seq (26.83 us) is 10.9x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 5.6%)

satfold-lanes4-idx wins on median (2.46 us) yet has the highest variance (CV 5.6%), while satfold-lanes64 is the steadiest (CV 1.5%, 26.80 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes4-idx shows warm-up / thermal drift (autocorr +0.80)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes4-idx, satfold-neon, satfold-neon8} vs {satfold-lanes16-constl, satfold-lanes16, satfold-nolaw, satfold-iterfold, satfold-lanes64, satfold-seq} (151% apart)

The field splits into a fast tier {satfold-lanes4-idx, satfold-neon, satfold-neon8} and a slow tier {satfold-lanes16-constl, satfold-lanes16, satfold-nolaw, satfold-iterfold, satfold-lanes64, satfold-seq} with a 151% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.9x the fastest

Fastest satfold-lanes4-idx (2.46 us) to slowest satfold-seq (26.83 us): 10.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2455.2 ns median (-90.8% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 10.93x (fastest 2455.2 ns, slowest 26826.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 27197ns | 26829ns | 26337ns | 27015ns | 28602ns | base |
| satfold-lanes16 | 10543ns | 10376ns | 10166ns | 10382ns | 11404ns | -61.23% |
| satfold-lanes16-constl | 10505ns | 10315ns | 10112ns | 10365ns | 11316ns | -61.38% |
| satfold-lanes4-idx | 2557ns | 2519ns | 2438ns | 2521ns | 2785ns | -90.60% |
| satfold-lanes64 | 27028ns | 26870ns | 26596ns | 26957ns | 27673ns | -0.62% |
| satfold-neon | 4026ns | 3995ns | 3989ns | 3999ns | 4146ns | -85.20% |
| satfold-neon8 | 4160ns | 4147ns | 3948ns | 4131ns | 4457ns | -84.71% |
| satfold-nolaw | 21269ns | 21233ns | 20571ns | 21123ns | 22406ns | -21.80% |
| satfold-seq | 27093ns | 26896ns | 26580ns | 27005ns | 27872ns | -0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 27108ns | 26254ns | 28494ns | base | 1.209 |
| satfold-lanes16 | 10480ns | 10105ns | 11335ns | -61.34% | 3.127 |
| satfold-lanes16-constl | 10439ns | 10053ns | 11247ns | -61.49% | 3.139 |
| satfold-lanes4-idx | 2492ns | 2374ns | 2715ns | -90.81% | 13.151 |
| satfold-lanes64 | 26941ns | 26515ns | 27586ns | -0.62% | 1.216 |
| satfold-neon | 3963ns | 3928ns | 4081ns | -85.38% | 8.268 |
| satfold-neon8 | 4096ns | 3887ns | 4390ns | -84.89% | 8.000 |
| satfold-nolaw | 21192ns | 20500ns | 22307ns | -21.82% | 1.546 |
| satfold-seq | 27011ns | 26492ns | 27780ns | -0.36% | 1.213 |

## Performance model

- Peak throughput: **13.802 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.226 | 8.9% |
| satfold-lanes16 | 3.177 | 23.0% |
| satfold-lanes16-constl | 3.195 | 23.1% |
| satfold-lanes4-idx | 13.346 | 96.7% |
| satfold-lanes64 | 1.223 | 8.9% |
| satfold-neon | 8.333 | 60.4% |
| satfold-neon8 | 8.027 | 58.2% |
| satfold-nolaw | 1.548 | 11.2% |
| satfold-seq | 1.221 | 8.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 27197ns | 27197ns | base |
| satfold-lanes16 | 10543ns | 10543ns | -61.23% |
| satfold-lanes16-constl | 10505ns | 10505ns | -61.38% |
| satfold-lanes4-idx | 2557ns | 2557ns | -90.60% |
| satfold-lanes64 | 27028ns | 27028ns | -0.62% |
| satfold-neon | 4026ns | 4026ns | -85.20% |
| satfold-neon8 | 4160ns | 4160ns | -84.71% |
| satfold-nolaw | 21269ns | 21269ns | -21.80% |
| satfold-seq | 27093ns | 27093ns | -0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 26736ns | base | --- | [26708, 27149] | --- | --- | --- | --- |
| satfold-lanes16 | 10315ns | -16578.5ns (-62.0%) | [-17000, -16528]ns | [10161, 10424] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 10257ns | -16644.6ns (-62.3%) | [-17035, -16439]ns | [10085, 10451] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2455ns | -24345.0ns (-91.1%) | [-24739, -24232]ns | [2383, 2458] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 26799ns | no significant difference | [-422, +284]ns | [26729, 26963] | no | 0.3065 | 0.2682 | 0 |
| satfold-neon | 3932ns | -22795.7ns (-85.3%) | [-23206, -22778]ns | [3931, 3939] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 4082ns | -22729.4ns (-85.0%) | [-23116, -22627]ns | [3946, 4087] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 21166ns | -6198.1ns (-23.2%) | [-6271, -5813]ns | [20828, 21247] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 26827ns | no significant difference | [-309, +85]ns | [26794, 26930] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 26680ns | -62.1% | -62.4% | -91.1% | +1.6% | -85.3% | -84.9% | -23.3% | +0.7% |
| 2 | 26724ns | -62.2% | -61.0% | -91.1% | +1.1% | -85.3% | -85.2% | -23.2% | +0.5% |
| 3 | 26684ns | -62.1% | -62.2% | -91.1% | +3.7% | -85.3% | -85.2% | -22.4% | +0.5% |
| 4 | 27094ns | -62.6% | -62.8% | -91.2% | +2.0% | -85.5% | -85.4% | -23.7% | -1.1% |
| 5 | 27697ns | -63.3% | -63.6% | -91.4% | -2.7% | -85.8% | -85.8% | -24.3% | -2.5% |
| 6 | 27682ns | -63.3% | -63.4% | -91.4% | -3.5% | -85.8% | -85.7% | -25.5% | -3.2% |
| 7 | 27661ns | -61.9% | -63.2% | -91.4% | -3.3% | -85.8% | -85.8% | -22.0% | -3.0% |
| 8 | 27657ns | -63.5% | -63.6% | -91.4% | -3.4% | -85.7% | -85.7% | -23.2% | -3.1% |
| 9 | 27166ns | -62.7% | -62.9% | -91.2% | -1.8% | -85.4% | -83.9% | -21.7% | -1.2% |
| 10 | 26738ns | -62.3% | -60.9% | -91.1% | +1.7% | -85.2% | -84.4% | -22.2% | +0.3% |
| 11 | 29771ns | -65.0% | -65.1% | -91.1% | -10.2% | -86.3% | -86.8% | -26.7% | -8.4% |
| 12 | 29729ns | -64.9% | -65.0% | -91.1% | -8.5% | -86.3% | -86.7% | -28.6% | -6.7% |
| 13 | 29577ns | -63.7% | -63.8% | -91.0% | -9.7% | -86.7% | -86.7% | -27.7% | -4.7% |
| 14 | 27618ns | -62.0% | -60.6% | -90.4% | -3.4% | -85.8% | -85.8% | -22.9% | +0.3% |
| 15 | 27904ns | -62.7% | -62.1% | -90.5% | -2.4% | -85.9% | -86.1% | -24.1% | -0.8% |
| 16 | 27932ns | -62.6% | -62.6% | -89.7% | -4.4% | -85.9% | -86.1% | -24.3% | -1.5% |
| 17 | 27522ns | -62.2% | -62.5% | -89.8% | -2.9% | -85.7% | -85.9% | -24.3% | -2.8% |
| 18 | 26995ns | -61.4% | -62.8% | -89.7% | +0.6% | -85.5% | -85.7% | -23.9% | -1.5% |
| 19 | 26710ns | -61.0% | -62.5% | -90.1% | +0.1% | -85.3% | -85.5% | -23.3% | +0.4% |
| 20 | 26252ns | -61.6% | -61.8% | -89.9% | +1.7% | -84.8% | -85.3% | -21.1% | +0.0% |
| 21 | 26269ns | -55.7% | -56.5% | -90.7% | +5.5% | -85.0% | -83.3% | -12.8% | +5.5% |
| 22 | 26693ns | -57.9% | -57.8% | -90.8% | +2.2% | -85.2% | -83.6% | -14.0% | +3.6% |
| 23 | 26209ns | -57.1% | -57.1% | -90.6% | +2.8% | -84.7% | -83.3% | -10.8% | +5.6% |
| 24 | 26207ns | -57.1% | -57.2% | -90.6% | +1.5% | -84.9% | -83.2% | -13.1% | +5.4% |
| 25 | 26755ns | -57.8% | -57.9% | -90.8% | +0.5% | -85.3% | -83.6% | -20.8% | +4.1% |
| 26 | 26321ns | -57.3% | -57.4% | -90.7% | +1.0% | -85.1% | -83.3% | -18.7% | +2.8% |
| 27 | 26226ns | -57.0% | -56.8% | -90.6% | +0.9% | -84.3% | -83.3% | -18.1% | +1.9% |
| 28 | 26672ns | -56.8% | -58.6% | -90.8% | -1.7% | -85.3% | -83.5% | -20.6% | -1.3% |
| 29 | 26344ns | -60.2% | -60.3% | -90.7% | +0.1% | -84.6% | -83.3% | -18.1% | -0.1% |
| 30 | 26207ns | -60.2% | -60.1% | -90.6% | +1.2% | -83.3% | -83.3% | -18.5% | +0.2% |
| 31 | 26720ns | -62.0% | -61.0% | -90.3% | +0.9% | -85.3% | -84.7% | -23.3% | +0.3% |
| 32 | 26733ns | -62.0% | -62.1% | -90.2% | +0.2% | -85.3% | -84.7% | -23.3% | +0.2% |
| 33 | 27194ns | -62.8% | -62.9% | -91.3% | -1.4% | -85.5% | -85.0% | -22.5% | -1.6% |
| 34 | 26705ns | -62.1% | -62.2% | -91.1% | +4.5% | -85.3% | -84.7% | -20.5% | +0.3% |
| 35 | 26762ns | -62.2% | -62.4% | -91.1% | +3.3% | -85.3% | -84.7% | -20.6% | +0.1% |
| 36 | 27233ns | -62.6% | -62.9% | -91.3% | -1.7% | -85.6% | -85.0% | -23.1% | -1.5% |
| 37 | 26713ns | -61.8% | -62.3% | -91.1% | +3.4% | -85.3% | -84.7% | -21.4% | +0.3% |
| 38 | 26699ns | -61.8% | -62.1% | -90.7% | +0.8% | -85.3% | -84.7% | -23.2% | +0.1% |
| 39 | 27133ns | -62.4% | -62.8% | -91.3% | -1.2% | -85.5% | -84.9% | -24.3% | +0.3% |
| 40 | 26729ns | -62.0% | -62.3% | -91.1% | +0.4% | -85.3% | -85.0% | -23.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.713 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.714 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.781 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.801 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.339 | moderate+ |
| satfold-neon | 0.145 | ok |
| satfold-neon8 | 0.775 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.655 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.672 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 16/40, lost 24/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 17/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.6ns | 27107.9ns | 0.0% |  |
| satfold-lanes16 | 3.0ns | 10480.1ns | 0.0% |  |
| satfold-lanes16-constl | 2.6ns | 10438.9ns | 0.0% |  |
| satfold-lanes4-idx | 3.2ns | 2491.7ns | 0.1% |  |
| satfold-lanes64 | 3.1ns | 26940.5ns | 0.0% |  |
| satfold-neon | 2.3ns | 3963.3ns | 0.1% |  |
| satfold-neon8 | 2.4ns | 4095.9ns | 0.1% |  |
| satfold-nolaw | 2.8ns | 21192.0ns | 0.0% |  |
| satfold-seq | 2.3ns | 27010.5ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 26254.3-28494.1 ns)
  26254.3 |############
  26366.3 |
  26478.3 |
  26590.3 |####################
  26702.3 |########################################
  26814.3 |
  26926.2 |####
  27038.2 |########
  27150.2 |############
  27262.2 |
  27374.2 |
  27486.2 |####
  27598.2 |####################
  27710.2 |
  27822.2 |########
  27934.1 |
  28046.1 |
  28158.1 |
  28270.1 |
  28382.1 |
  (5 below, 3 above range)

satfold-lanes16 (n=40, range 10104.9-11334.6 ns)
  10104.9 |########################################
  10166.3 |################
  10227.8 |
  10289.3 |
  10350.8 |
  10412.3 |##########################
  10473.8 |######
  10535.3 |###
  10596.8 |
  10658.3 |
  10719.7 |###
  10781.2 |
  10842.7 |
  10904.2 |
  10965.7 |
  11027.2 |
  11088.7 |
  11150.2 |
  11211.7 |#############
  11273.1 |######
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 10052.8-11247.2 ns)
  10052.8 |########################################
  10112.5 |##########
  10172.2 |###
  10231.9 |
  10291.7 |###
  10351.4 |######
  10411.1 |####################
  10470.8 |
  10530.5 |###
  10590.3 |
  10650.0 |###
  10709.7 |
  10769.4 |
  10829.2 |###
  10888.9 |
  10948.6 |
  11008.3 |###
  11068.1 |
  11127.8 |
  11187.5 |##########
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 2374.1-2714.7 ns)
   2374.1 |########################################
   2391.2 |
   2408.2 |
   2425.2 |
   2442.2 |#################################
   2459.3 |###
   2476.3 |
   2493.3 |
   2510.4 |
   2527.4 |
   2544.4 |
   2561.5 |
   2578.5 |###
   2595.5 |
   2612.6 |###
   2629.6 |###
   2646.6 |####################
   2663.6 |
   2680.7 |
   2697.7 |
  (5 below, 3 above range)

satfold-lanes64 (n=40, range 26514.9-27585.5 ns)
  26514.9 |#####
  26568.4 |###########
  26622.0 |#####
  26675.5 |########################################
  26729.0 |############################
  26782.5 |#################
  26836.1 |#####
  26889.6 |#####
  26943.1 |#################
  26996.7 |#####
  27050.2 |
  27103.7 |#####
  27157.3 |#################
  27210.8 |#####
  27264.3 |#####
  27317.9 |
  27371.4 |
  27424.9 |
  27478.5 |
  27532.0 |
  (3 below, 6 above range)

satfold-neon (n=40, range 3927.9-4081.2 ns)
   3927.9 |########################################
   3935.5 |#####
   3943.2 |###
   3950.9 |#####
   3958.6 |
   3966.2 |
   3973.9 |
   3981.6 |
   3989.2 |#
   3996.9 |#
   4004.6 |
   4012.2 |
   4019.9 |
   4027.6 |
   4035.2 |
   4042.9 |#
   4050.6 |
   4058.2 |
   4065.9 |###
   4073.6 |
  (4 below, 2 above range)

satfold-neon8 (n=40, range 3887.2-4389.5 ns)
   3887.2 |####
   3912.3 |#############
   3937.4 |###################################
   3962.5 |
   3987.7 |####
   4012.8 |####
   4037.9 |
   4063.0 |########################################
   4088.1 |
   4113.2 |
   4138.4 |
   4163.5 |####
   4188.6 |
   4213.7 |
   4238.8 |
   4263.9 |
   4289.1 |
   4314.2 |
   4339.3 |
   4364.4 |##########################
  (5 below, 5 above range)

satfold-nolaw (n=40, range 20499.8-22306.6 ns)
  20499.8 |########################################
  20590.1 |################
  20680.5 |################
  20770.8 |################
  20861.1 |########
  20951.5 |################
  21041.8 |########
  21132.2 |########################################
  21222.5 |########################################
  21312.8 |########################
  21403.2 |########
  21493.5 |########
  21583.8 |########
  21674.2 |
  21764.5 |########
  21854.9 |
  21945.2 |
  22035.5 |
  22125.9 |
  22216.2 |
  (4 below, 4 above range)

satfold-seq (n=40, range 26491.8-27779.8 ns)
  26491.8 |
  26556.2 |###
  26620.6 |
  26685.0 |##########
  26749.4 |########################################
  26813.8 |#########################
  26878.2 |
  26942.6 |###
  27007.0 |###
  27071.4 |
  27135.8 |
  27200.2 |###
  27264.6 |###
  27329.0 |
  27393.4 |
  27457.8 |###
  27522.2 |
  27586.6 |###
  27651.0 |##################
  27715.4 |###
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.67 (measurement drift or warm-up artifact)
