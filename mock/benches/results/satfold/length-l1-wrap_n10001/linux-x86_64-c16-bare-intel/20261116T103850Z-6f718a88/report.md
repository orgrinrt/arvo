# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is an outlier: 3.9x slower than the field

satfold-nolaw (3.03 us) is 3.9x the fastest (785 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.70)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-iterfold, satfold-lanes64, satfold-neon8, satfold-seq} vs {satfold-lanes16-constl, satfold-lanes4-idx, satfold-lanes16, satfold-nolaw} (143% apart)

The field splits into a fast tier {satfold-neon, satfold-iterfold, satfold-lanes64, satfold-neon8, satfold-seq} and a slow tier {satfold-lanes16-constl, satfold-lanes4-idx, satfold-lanes16, satfold-nolaw} with a 143% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.9x the fastest

Fastest satfold-neon (785 ns) to slowest satfold-nolaw (3.03 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader satfold-neon vs stability leader satfold-lanes64 (+9% speed for 3.9x steadier)

satfold-neon is fastest (785 ns, CV 21.4%); satfold-lanes64 gives up 8.7% median for 3.9x lower variance (CV 5.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes4-idx is inconsistent: worst-20% is 7.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.15 us but its worst 20% at 8.03 us (7.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 785.0 ns median (-2.0% vs baseline)
- 7 variants significantly slower than baseline
- Spread: 3.86x (fastest 785.0 ns, slowest 3026.8 ns)

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
| Counter frequency | 2112766038 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1055ns | 1013ns | 935ns | 1039ns | 1223ns | base |
| satfold-lanes16 | 2717ns | 2608ns | 1681ns | 2646ns | 3962ns | +157.44% |
| satfold-lanes16-constl | 2336ns | 2321ns | 1877ns | 2312ns | 2865ns | +121.34% |
| satfold-lanes4-idx | 3954ns | 2299ns | 1203ns | 3412ns | 8332ns | +274.76% |
| satfold-lanes64 | 1081ns | 1060ns | 987ns | 1061ns | 1235ns | +2.42% |
| satfold-neon | 1096ns | 982ns | 909ns | 1031ns | 1478ns | +3.90% |
| satfold-neon8 | 1174ns | 1078ns | 980ns | 1130ns | 1501ns | +11.28% |
| satfold-nolaw | 3093ns | 3222ns | 1369ns | 3234ns | 4391ns | +193.09% |
| satfold-seq | 1128ns | 1075ns | 911ns | 1084ns | 1477ns | +6.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 843ns | 756ns | 959ns | base | 38.890 |
| satfold-lanes16 | 2486ns | 1490ns | 3698ns | +194.99% | 13.183 |
| satfold-lanes16-constl | 2101ns | 1676ns | 2597ns | +149.35% | 15.597 |
| satfold-lanes4-idx | 3803ns | 1147ns | 8032ns | +351.29% | 8.617 |
| satfold-lanes64 | 857ns | 797ns | 923ns | +1.70% | 38.239 |
| satfold-neon | 874ns | 733ns | 1177ns | +3.70% | 37.503 |
| satfold-neon8 | 935ns | 790ns | 1185ns | +11.00% | 35.036 |
| satfold-nolaw | 2894ns | 1286ns | 4091ns | +243.50% | 11.322 |
| satfold-seq | 900ns | 736ns | 1172ns | +6.83% | 36.403 |

## Performance model

- Peak throughput: **44.707 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 40.911 | 91.5% |
| satfold-lanes16 | 13.666 | 30.6% |
| satfold-lanes16-constl | 15.476 | 34.6% |
| satfold-lanes4-idx | 14.801 | 33.1% |
| satfold-lanes64 | 38.408 | 85.9% |
| satfold-neon | 41.745 | 93.4% |
| satfold-neon8 | 37.795 | 84.5% |
| satfold-nolaw | 10.826 | 24.2% |
| satfold-seq | 37.680 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1055ns | 1055ns | base |
| satfold-lanes16 | 2717ns | 2717ns | +157.44% |
| satfold-lanes16-constl | 2336ns | 2336ns | +121.34% |
| satfold-lanes4-idx | 3954ns | 3954ns | +274.76% |
| satfold-lanes64 | 1081ns | 1081ns | +2.42% |
| satfold-neon | 1096ns | 1096ns | +3.90% |
| satfold-neon8 | 1174ns | 1174ns | +11.28% |
| satfold-nolaw | 3093ns | 3093ns | +193.09% |
| satfold-seq | 1128ns | 1128ns | +6.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 801ns | base | --- | [793, 865] | --- | --- | --- | --- |
| satfold-lanes16 | 2398ns | +1526.5ns (+190.6%) | [+1212, +1855]ns | [2018, 2791] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2117ns | +1275.8ns (+159.3%) | [+1103, +1394]ns | [1967, 2190] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2214ns | +1416.6ns (+176.9%) | [+793, +3919]ns | [1525, 4773] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 853ns | +49.7ns (+6.2%) | [+18, +59]ns | [851, 862] | YES | 0.0190 | 0.0166 | 0 |
| satfold-neon | 785ns | no significant difference | [-75, +62]ns | [781, 794] | no | 0.6358 | 0.6358 | 0 |
| satfold-neon8 | 867ns | +101.4ns (+12.7%) | [+33, +135]ns | [864, 935] | YES | 0.0036 | 0.0022 | 0 |
| satfold-nolaw | 3027ns | +2221.2ns (+277.3%) | [+1926, +2477]ns | [2698, 3409] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 870ns | +71.0ns (+8.9%) | [+8, +84]ns | [797, 871] | YES | 0.0086 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 808ns | +128.4% | +172.5% | +89.1% | +7.1% | -3.4% | +6.1% | +339.2% | +7.7% |
| 1 | 2 | 955ns | +121.9% | +77.5% | +371.0% | -10.8% | -18.5% | +24.5% | +251.9% | +22.1% |
| 1 | 3 | 788ns | +314.4% | +192.9% | +340.1% | +8.9% | +49.1% | +9.6% | +261.8% | -6.6% |
| 1 | 4 | 779ns | +230.3% | +160.3% | +425.5% | +6.9% | +1.9% | +14.5% | +245.7% | +10.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.300 | moderate- |
| satfold-lanes16 | 0.027 | ok |
| satfold-lanes16-constl | -0.697 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.372 | moderate- |
| satfold-lanes64 | -0.187 | ok |
| satfold-neon | -0.396 | moderate- |
| satfold-neon8 | -0.495 | moderate- |
| satfold-nolaw | 0.309 | moderate+ |
| satfold-seq | -0.458 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 2/4, lost 2/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 76.4ns | 842.6ns | 9.1% | HIGH |
| satfold-lanes16 | 80.4ns | 2485.6ns | 3.2% |  |
| satfold-lanes16-constl | 81.5ns | 2101.0ns | 3.9% |  |
| satfold-lanes4-idx | 47.9ns | 3802.5ns | 1.3% |  |
| satfold-lanes64 | 78.4ns | 856.9ns | 9.2% | HIGH |
| satfold-neon | 80.1ns | 873.8ns | 9.2% | HIGH |
| satfold-neon8 | 86.4ns | 935.3ns | 9.2% | HIGH |
| satfold-nolaw | 70.7ns | 2894.2ns | 2.4% |  |
| satfold-seq | 81.6ns | 900.1ns | 9.1% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 755.8-959.2 ns)
    755.8 |
    766.0 |#####
    776.1 |#################
    786.3 |##################################
    796.5 |########################################
    806.6 |
    816.8 |###########
    827.0 |#####
    837.2 |
    847.3 |
    857.5 |#################
    867.7 |#####
    877.8 |
    888.0 |
    898.2 |
    908.4 |#####
    918.5 |#####
    928.7 |
    938.9 |#################
    949.0 |##################################
  (4 below, 1 above range)

satfold-lanes16 (n=40, range 1490.1-3697.5 ns)
   1490.1 |########################################
   1600.5 |
   1710.8 |##########
   1821.2 |##############################
   1931.6 |##########
   2042.0 |####################
   2152.3 |##############################
   2262.7 |##########
   2373.1 |##########
   2483.4 |####################
   2593.8 |##############################
   2704.2 |
   2814.6 |####################
   2924.9 |####################
   3035.3 |
   3145.7 |##########
   3256.0 |##########
   3366.4 |##############################
   3476.8 |##########
   3587.2 |
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 1675.8-2596.5 ns)
   1675.8 |########################################
   1721.8 |##########
   1767.8 |##########
   1813.9 |##############################
   1859.9 |##########
   1906.0 |
   1952.0 |####################
   1998.0 |####################
   2044.1 |####################
   2090.1 |########################################
   2136.2 |##############################
   2182.2 |##############################
   2228.2 |
   2274.3 |####################
   2320.3 |##########
   2366.3 |
   2412.4 |
   2458.4 |##########
   2504.5 |##############################
   2550.5 |####################
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 1146.8-8032.2 ns)
   1146.8 |########################################
   1491.1 |###########
   1835.3 |############################
   2179.6 |#####
   2523.9 |
   2868.2 |
   3212.4 |
   3556.7 |
   3901.0 |###########
   4245.2 |#################
   4589.5 |
   4933.8 |###########
   5278.1 |
   5622.3 |#####
   5966.6 |#####
   6310.9 |###########
   6655.1 |
   6999.4 |###########
   7343.7 |
   7688.0 |###########
  (6 below, 4 above range)

satfold-lanes64 (n=40, range 796.6-923.1 ns)
    796.6 |
    802.9 |
    809.2 |#######
    815.5 |
    821.9 |
    828.2 |
    834.5 |
    840.9 |##########
    847.2 |########################################
    853.5 |##########
    859.8 |##################
    866.2 |##############
    872.5 |
    878.8 |
    885.2 |#######
    891.5 |
    897.8 |
    904.2 |
    910.5 |
    916.8 |
  (6 below, 4 above range)

satfold-neon (n=40, range 733.0-1177.5 ns)
    733.0 |####
    755.2 |####
    777.4 |########################################
    799.6 |
    821.9 |##
    844.1 |
    866.3 |
    888.5 |##
    910.8 |##
    933.0 |
    955.2 |
    977.4 |
    999.7 |
   1021.9 |######
   1044.1 |
   1066.4 |
   1088.6 |
   1110.8 |
   1133.0 |
   1155.3 |######
  (5 below, 4 above range)

satfold-neon8 (n=40, range 790.1-1185.0 ns)
    790.1 |##
    809.9 |
    829.6 |
    849.4 |########################################
    869.1 |
    888.9 |##
    908.6 |#####
    928.3 |
    948.1 |##########
    967.8 |
    987.6 |
   1007.3 |
   1027.1 |##
   1046.8 |##
   1066.6 |
   1086.3 |
   1106.1 |##
   1125.8 |
   1145.6 |
   1165.3 |##
  (6 below, 6 above range)

satfold-nolaw (n=40, range 1285.7-4090.7 ns)
   1285.7 |
   1426.0 |
   1566.2 |
   1706.5 |
   1846.7 |
   1987.0 |
   2127.2 |########
   2267.5 |
   2407.7 |################
   2548.0 |########################################
   2688.2 |########################
   2828.5 |################
   2968.7 |########################
   3109.0 |################
   3249.2 |########
   3389.4 |########################################
   3529.7 |########################
   3669.9 |########
   3810.2 |########
   3950.4 |########
  (6 below, 4 above range)

satfold-seq (n=40, range 735.9-1172.3 ns)
    735.9 |####################
    757.7 |####
    779.5 |########################
    801.4 |
    823.2 |
    845.0 |####
    866.8 |########################################
    888.6 |
    910.5 |########
    932.3 |####
    954.1 |
    975.9 |
    997.7 |
   1019.5 |
   1041.4 |####
   1063.2 |########
   1085.0 |
   1106.8 |####
   1128.6 |####
   1150.5 |
  (4 below, 5 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=32.3% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=70.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 7.0x (possible bimodal distribution)
- **satfold-nolaw**: CV=34.6% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 3.2x (possible bimodal distribution)
