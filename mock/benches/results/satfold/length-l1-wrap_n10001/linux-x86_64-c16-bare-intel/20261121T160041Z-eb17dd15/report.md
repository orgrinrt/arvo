# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is an outlier: 4.0x slower than the field

satfold-nolaw (3.16 us) is 4.0x the fastest (795 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.77)

satfold-seq's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-iterfold, satfold-neon, satfold-lanes64, satfold-seq} vs {satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx, satfold-nolaw} (98% apart)

The field splits into a fast tier {satfold-neon8, satfold-iterfold, satfold-neon, satfold-lanes64, satfold-seq} and a slow tier {satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx, satfold-nolaw} with a 98% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest satfold-neon8 (795 ns) to slowest satfold-nolaw (3.16 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader satfold-neon8 vs stability leader satfold-lanes64 (+9% speed for 1.6x steadier)

satfold-neon8 is fastest (795 ns, CV 8.7%); satfold-lanes64 gives up 8.8% median for 1.6x lower variance (CV 5.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.8x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.29 us but its worst 20% at 8.69 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon8** at 795.1 ns median (-1.8% vs baseline)
- 6 variants significantly slower than baseline
- Spread: 3.97x (fastest 795.1 ns, slowest 3159.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112745822 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1025ns | 1021ns | 909ns | 1026ns | 1142ns | base |
| satfold-lanes16 | 2344ns | 2098ns | 1631ns | 2147ns | 3647ns | +128.56% |
| satfold-lanes16-constl | 2408ns | 2336ns | 1640ns | 2366ns | 3302ns | +134.79% |
| satfold-lanes4-idx | 4367ns | 3057ns | 2347ns | 3508ns | 8967ns | +325.89% |
| satfold-lanes64 | 1096ns | 1070ns | 1016ns | 1083ns | 1216ns | +6.90% |
| satfold-neon | 1063ns | 1057ns | 948ns | 1044ns | 1238ns | +3.70% |
| satfold-neon8 | 1049ns | 1001ns | 947ns | 1033ns | 1201ns | +2.31% |
| satfold-nolaw | 3305ns | 3382ns | 2319ns | 3393ns | 4026ns | +222.28% |
| satfold-seq | 1133ns | 1175ns | 956ns | 1136ns | 1303ns | +10.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 822ns | 736ns | 894ns | base | 39.850 |
| satfold-lanes16 | 2121ns | 1456ns | 3375ns | +157.91% | 15.451 |
| satfold-lanes16-constl | 2184ns | 1456ns | 3007ns | +165.62% | 15.003 |
| satfold-lanes4-idx | 4251ns | 2289ns | 8690ns | +416.93% | 7.709 |
| satfold-lanes64 | 878ns | 822ns | 957ns | +6.72% | 37.342 |
| satfold-neon | 852ns | 765ns | 981ns | +3.56% | 38.481 |
| satfold-neon8 | 838ns | 767ns | 952ns | +1.95% | 39.086 |
| satfold-nolaw | 3109ns | 2186ns | 3776ns | +278.04% | 10.541 |
| satfold-seq | 908ns | 774ns | 1019ns | +10.47% | 36.074 |

## Performance model

- Peak throughput: **44.527 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 40.487 | 90.9% |
| satfold-lanes16 | 17.516 | 39.3% |
| satfold-lanes16-constl | 15.806 | 35.5% |
| satfold-lanes4-idx | 10.974 | 24.6% |
| satfold-lanes64 | 37.880 | 85.1% |
| satfold-neon | 39.510 | 88.7% |
| satfold-neon8 | 41.210 | 92.6% |
| satfold-nolaw | 10.371 | 23.3% |
| satfold-seq | 34.638 | 77.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1025ns | 1025ns | base |
| satfold-lanes16 | 2344ns | 2344ns | +128.56% |
| satfold-lanes16-constl | 2408ns | 2408ns | +134.79% |
| satfold-lanes4-idx | 4367ns | 4367ns | +325.89% |
| satfold-lanes64 | 1096ns | 1096ns | +6.90% |
| satfold-neon | 1063ns | 1063ns | +3.70% |
| satfold-neon8 | 1049ns | 1049ns | +2.31% |
| satfold-nolaw | 3305ns | 3305ns | +222.28% |
| satfold-seq | 1133ns | 1133ns | +10.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 809ns | base | --- | [796, 869] | --- | --- | --- | --- |
| satfold-lanes16 | 1871ns | +1068.2ns (+132.0%) | [+975, +1163]ns | [1869, 1996] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2073ns | +1280.2ns (+158.2%) | [+1053, +1532]ns | [1905, 2344] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2986ns | +2193.9ns (+271.1%) | [+2107, +2250]ns | [2970, 2992] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 865ns | +66.1ns (+8.2%) | [+34, +75]ns | [865, 866] | YES | 0.0265 | 0.0166 | 0 |
| satfold-neon | 829ns | no significant difference | [-6, +54]ns | [791, 866] | no | 0.9996 | 0.8746 | 0 |
| satfold-neon8 | 795ns | no significant difference | [-6, +57]ns | [794, 866] | no | 1.0000 | 1.0000 | 0 |
| satfold-nolaw | 3160ns | +2369.6ns (+292.8%) | [+2200, +2512]ns | [3055, 3287] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 946ns | +87.0ns (+10.7%) | [+3, +160]ns | [870, 957] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 870ns | +117.8% | +154.5% | +243.3% | +1.6% | -0.4% | +9.1% | +256.3% | +0.1% |
| 1 | 2 | 832ns | +126.2% | +228.8% | +193.5% | +5.3% | -5.1% | -5.0% | +341.9% | +15.3% |
| 1 | 3 | 861ns | +158.1% | +96.6% | +344.1% | +0.3% | +12.6% | -7.7% | +253.1% | -7.4% |
| 1 | 4 | 736ns | +107.7% | +239.2% | +305.5% | +17.6% | +7.3% | +7.8% | +329.2% | +29.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.240 | moderate- |
| satfold-lanes16 | -0.502 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.703 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.495 | moderate- |
| satfold-lanes64 | 0.256 | moderate+ |
| satfold-neon | -0.718 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.108 | ok |
| satfold-nolaw | -0.520 | HIGH- (thermal bounce) |
| satfold-seq | -0.765 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 2/4, lost 2/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 75.0ns | 822.3ns | 9.1% | HIGH |
| satfold-lanes16 | 81.5ns | 2120.7ns | 3.8% |  |
| satfold-lanes16-constl | 79.7ns | 2184.1ns | 3.6% |  |
| satfold-lanes4-idx | 35.2ns | 4250.6ns | 0.8% |  |
| satfold-lanes64 | 79.0ns | 877.5ns | 9.0% | HIGH |
| satfold-neon | 78.0ns | 851.5ns | 9.2% | HIGH |
| satfold-neon8 | 75.3ns | 838.4ns | 9.0% | HIGH |
| satfold-nolaw | 67.9ns | 3108.5ns | 2.2% |  |
| satfold-seq | 82.2ns | 908.4ns | 9.0% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 735.9-893.6 ns)
    735.9 |######################
    743.8 |
    751.7 |
    759.6 |####
    767.4 |
    775.3 |
    783.2 |
    791.1 |########################################
    799.0 |########
    806.9 |
    814.7 |####
    822.6 |
    830.5 |
    838.4 |####
    846.3 |
    854.1 |####
    862.0 |##########################
    869.9 |###############################
    877.8 |####
    885.7 |
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 1456.4-3375.5 ns)
   1456.4 |#########
   1552.4 |#########
   1648.3 |######
   1744.3 |
   1840.2 |########################################
   1936.2 |
   2032.1 |######
   2128.1 |###
   2224.0 |###
   2320.0 |
   2415.9 |######
   2511.9 |
   2607.8 |###
   2703.8 |
   2799.8 |###
   2895.7 |###
   2991.7 |
   3087.6 |###
   3183.6 |
   3279.5 |
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 1456.2-3007.2 ns)
   1456.2 |########
   1533.7 |
   1611.3 |
   1688.8 |########################################
   1766.4 |########
   1843.9 |################################
   1921.5 |########
   1999.0 |################################
   2076.6 |########
   2154.1 |################
   2231.7 |########
   2309.2 |########################
   2386.8 |################
   2464.3 |
   2541.9 |
   2619.4 |########################
   2697.0 |########
   2774.6 |################
   2852.1 |
   2929.7 |################
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 2289.4-8690.3 ns)
   2289.4 |#################
   2609.5 |##
   2929.5 |########################################
   3249.6 |
   3569.6 |
   3889.6 |
   4209.7 |
   4529.7 |
   4849.8 |
   5169.8 |#####
   5489.9 |
   5809.9 |
   6130.0 |
   6450.0 |##
   6770.1 |##
   7090.1 |##
   7410.2 |#####
   7730.2 |##
   8050.2 |#####
   8370.3 |
  (4 below, 2 above range)

satfold-lanes64 (n=40, range 821.9-956.5 ns)
    821.9 |#
    828.7 |
    835.4 |
    842.1 |
    848.8 |
    855.6 |###
    862.3 |########################################
    869.0 |
    875.8 |
    882.5 |#
    889.2 |
    895.9 |#
    902.7 |
    909.4 |#
    916.1 |#
    922.9 |
    929.6 |
    936.3 |
    943.1 |#
    949.8 |#########
  (4 below, 1 above range)

satfold-neon (n=40, range 764.7-981.2 ns)
    764.7 |
    775.5 |##
    786.3 |########################################
    797.1 |
    808.0 |
    818.8 |##
    829.6 |##
    840.5 |
    851.3 |##
    862.1 |################
    872.9 |
    883.8 |##
    894.6 |##
    905.4 |
    916.2 |
    927.1 |##
    937.9 |
    948.7 |########
    959.5 |##
    970.4 |#####
  (3 below, 3 above range)

satfold-neon8 (n=40, range 766.9-952.1 ns)
    766.9 |##
    776.2 |
    785.4 |########################################
    794.7 |###########
    803.9 |
    813.2 |
    822.5 |
    831.7 |
    841.0 |
    850.2 |
    859.5 |############################
    868.8 |
    878.0 |
    887.3 |
    896.6 |##
    905.8 |
    915.1 |
    924.4 |
    933.6 |
    942.9 |#################
  (3 below, 1 above range)

satfold-nolaw (n=40, range 2186.1-3776.3 ns)
   2186.1 |
   2265.6 |########
   2345.1 |
   2424.6 |################
   2504.1 |
   2583.6 |
   2663.1 |
   2742.7 |
   2822.2 |########################################
   2901.7 |
   2981.2 |################################
   3060.7 |########################
   3140.2 |################################
   3219.7 |########################################
   3299.2 |########
   3378.7 |
   3458.2 |################
   3537.8 |########################
   3617.3 |########################
   3696.8 |
  (3 below, 4 above range)

satfold-seq (n=40, range 774.0-1019.2 ns)
    774.0 |
    786.2 |############
    798.5 |############
    810.8 |
    823.0 |
    835.3 |####
    847.6 |
    859.8 |############################
    872.1 |
    884.3 |
    896.6 |
    908.9 |
    921.1 |########
    933.4 |####
    945.6 |########################################
    957.9 |####################
    970.2 |
    982.4 |####
    994.7 |
   1007.0 |
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=33.7% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=25.5% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=59.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.8x (possible bimodal distribution)
