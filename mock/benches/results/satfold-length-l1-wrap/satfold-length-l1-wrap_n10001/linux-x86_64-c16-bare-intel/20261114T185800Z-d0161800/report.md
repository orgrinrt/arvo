# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-nolaw is an outlier: 4.5x slower than the field

satfold-nolaw (3.58 us) is 4.5x the fastest (787 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-lanes64, satfold-neon8) are a dead heat (<1%)

satfold-lanes64 (787 ns) and satfold-neon8 (794 ns) differ by 0.83%, inside the noise, even though the wider field spreads 354.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.77)

satfold-seq's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.5x the fastest

Fastest satfold-lanes64 (787 ns) to slowest satfold-nolaw (3.58 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader satfold-lanes64 vs stability leader satfold-neon8 (+1% speed for 2.4x steadier)

satfold-lanes64 is fastest (787 ns, CV 10.9%); satfold-neon8 gives up 0.8% median for 2.4x lower variance (CV 4.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes4-idx is inconsistent: worst-20% is 6.1x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.38 us but its worst 20% at 8.43 us (6.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-lanes64's edge over baseline is significant but tiny (-10 ns, 1.20%)

satfold-lanes64 differs from baseline satfold-iterfold by -10 ns (1.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes64** at 787.0 ns median (-1.4% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 4.55x (fastest 787.0 ns, slowest 3578.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 1014ns | 988ns | 910ns | 1009ns | 1130ns | base |
| satfold-lanes16 | 2324ns | 2142ns | 1624ns | 2199ns | 3402ns | +129.31% |
| satfold-lanes16-constl | 2271ns | 2016ns | 1809ns | 2172ns | 3031ns | +124.02% |
| satfold-lanes4-idx | 3975ns | 2403ns | 1449ns | 3223ns | 8755ns | +292.12% |
| satfold-lanes64 | 1026ns | 979ns | 927ns | 1000ns | 1205ns | +1.24% |
| satfold-neon | 1091ns | 1055ns | 969ns | 1070ns | 1276ns | +7.65% |
| satfold-neon8 | 1004ns | 982ns | 934ns | 988ns | 1121ns | -0.99% |
| satfold-nolaw | 3453ns | 3790ns | 1195ns | 3678ns | 5034ns | +240.61% |
| satfold-seq | 1145ns | 1180ns | 985ns | 1131ns | 1349ns | +12.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 810ns | 735ns | 886ns | base | 40.469 |
| satfold-lanes16 | 2107ns | 1448ns | 3121ns | +160.19% | 15.554 |
| satfold-lanes16-constl | 2026ns | 1603ns | 2704ns | +150.20% | 16.175 |
| satfold-lanes4-idx | 3821ns | 1382ns | 8429ns | +371.87% | 8.576 |
| satfold-lanes64 | 813ns | 746ns | 926ns | +0.37% | 40.320 |
| satfold-neon | 866ns | 778ns | 993ns | +6.98% | 37.830 |
| satfold-neon8 | 801ns | 755ns | 858ns | -1.06% | 40.901 |
| satfold-nolaw | 3233ns | 1125ns | 4658ns | +299.29% | 10.135 |
| satfold-seq | 916ns | 797ns | 1064ns | +13.18% | 35.755 |

## Performance model

- Peak throughput: **44.578 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 41.058 | 92.1% |
| satfold-lanes16 | 16.879 | 37.9% |
| satfold-lanes16-constl | 18.451 | 41.4% |
| satfold-lanes4-idx | 14.031 | 31.5% |
| satfold-lanes64 | 41.637 | 93.4% |
| satfold-neon | 38.598 | 86.6% |
| satfold-neon8 | 41.296 | 92.6% |
| satfold-nolaw | 9.157 | 20.5% |
| satfold-seq | 35.834 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 1014ns | 1014ns | base |
| satfold-lanes16 | 2324ns | 2324ns | +129.31% |
| satfold-lanes16-constl | 2271ns | 2271ns | +124.02% |
| satfold-lanes4-idx | 3975ns | 3975ns | +292.12% |
| satfold-lanes64 | 1026ns | 1026ns | +1.24% |
| satfold-neon | 1091ns | 1091ns | +7.65% |
| satfold-neon8 | 1004ns | 1004ns | -0.99% |
| satfold-nolaw | 3453ns | 3453ns | +240.61% |
| satfold-seq | 1145ns | 1145ns | +12.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 798ns | base | --- | [797, 811] | --- | --- | --- | --- |
| satfold-lanes16 | 1941ns | +1148.1ns (+143.9%) | [+834, +1315]ns | [1662, 2182] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1776ns | +1027.8ns (+128.8%) | [+930, +1366]ns | [1745, 2158] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2335ns | +1608.5ns (+201.5%) | [+911, +3291]ns | [1709, 4161] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 787ns | -9.6ns (-1.2%) | [-17, -5]ns | [782, 794] | YES | 0.0009 | 0.0007 | 0 |
| satfold-neon | 849ns | no significant difference | [-17, +126]ns | [794, 897] | no | 0.4910 | 0.4296 | 0 |
| satfold-neon8 | 794ns | no significant difference | [-5, +17]ns | [793, 795] | no | 0.8746 | 0.8746 | 0 |
| satfold-nolaw | 3579ns | +2744.6ns (+343.9%) | [+2419, +2896]ns | [3169, 3750] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 914ns | +85.4ns (+10.7%) | [+56, +129]ns | [870, 960] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 794ns | +100.7% | +122.7% | +193.2% | -2.0% | +19.2% | -0.2% | +459.2% | +21.5% |
| 1 | 2 | 785ns | +275.6% | +131.1% | +180.1% | -0.1% | +8.2% | +1.3% | +319.3% | +3.5% |
| 1 | 3 | 870ns | +183.6% | +101.4% | +333.7% | -1.9% | -10.3% | -9.6% | +300.6% | +9.9% |
| 1 | 4 | 808ns | +115.8% | +183.0% | +143.7% | -2.3% | -3.1% | -0.7% | +357.3% | +4.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.318 | moderate- |
| satfold-lanes16 | -0.301 | moderate- |
| satfold-lanes16-constl | -0.162 | ok |
| satfold-lanes4-idx | -0.538 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.308 | moderate- |
| satfold-neon | 0.210 | moderate+ |
| satfold-neon8 | -0.542 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.261 | moderate- |
| satfold-seq | -0.774 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 0/4, lost 4/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 2/4, lost 2/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 0/4, lost 4/4
- **satfold-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 74.9ns | 809.7ns | 9.2% | HIGH |
| satfold-lanes16 | 76.1ns | 2106.7ns | 3.6% |  |
| satfold-lanes16-constl | 84.5ns | 2025.9ns | 4.2% |  |
| satfold-lanes4-idx | 49.3ns | 3820.7ns | 1.3% |  |
| satfold-lanes64 | 73.9ns | 812.7ns | 9.1% | HIGH |
| satfold-neon | 78.7ns | 866.2ns | 9.1% | HIGH |
| satfold-neon8 | 73.0ns | 801.2ns | 9.1% | HIGH |
| satfold-nolaw | 74.7ns | 3233.0ns | 2.3% |  |
| satfold-seq | 83.0ns | 916.5ns | 9.1% | HIGH |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 735.1-886.2 ns)
    735.1 |######
    742.6 |
    750.2 |
    757.7 |
    765.3 |
    772.9 |###
    780.4 |#########
    788.0 |###
    795.5 |########################################
    803.1 |###
    810.6 |###
    818.2 |
    825.7 |###
    833.3 |
    840.8 |
    848.4 |
    856.0 |#########
    863.5 |##################
    871.1 |###
    878.6 |
  (5 below, 2 above range)

satfold-lanes16 (n=40, range 1448.4-3121.1 ns)
   1448.4 |#############
   1532.1 |########################################
   1615.7 |######
   1699.3 |##########################
   1783.0 |
   1866.6 |#############
   1950.2 |####################
   2033.9 |####################
   2117.5 |
   2201.2 |######
   2284.8 |
   2368.4 |
   2452.1 |######
   2535.7 |
   2619.3 |##########################
   2703.0 |######
   2786.6 |
   2870.2 |######
   2953.9 |######
   3037.5 |
  (5 below, 5 above range)

satfold-lanes16-constl (n=40, range 1602.7-2703.7 ns)
   1602.7 |######
   1657.7 |###
   1712.8 |########################################
   1767.8 |#############
   1822.9 |###
   1877.9 |
   1933.0 |######
   1988.0 |
   2043.1 |###
   2098.2 |
   2153.2 |
   2208.3 |###
   2263.3 |######
   2318.4 |###
   2373.4 |###
   2428.5 |
   2483.5 |
   2538.6 |#############
   2593.6 |###
   2648.7 |
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1381.7-8428.9 ns)
   1381.7 |########################################
   1734.0 |########
   2086.4 |########################
   2438.8 |####
   2791.1 |
   3143.5 |####
   3495.8 |
   3848.2 |############
   4200.6 |####
   4552.9 |
   4905.3 |
   5257.6 |####
   5610.0 |
   5962.4 |####
   6314.7 |####
   6667.1 |####
   7019.5 |####
   7371.8 |
   7724.2 |############
   8076.5 |####
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 746.0-926.4 ns)
    746.0 |
    755.1 |####
    764.1 |
    773.1 |########################################
    782.1 |###################################
    791.1 |######################
    800.1 |
    809.2 |####
    818.2 |####
    827.2 |
    836.2 |
    845.2 |######################
    854.2 |####
    863.2 |####
    872.3 |
    881.3 |
    890.3 |
    899.3 |########
    908.3 |
    917.3 |####
  (4 below, 1 above range)

satfold-neon (n=40, range 778.1-992.5 ns)
    778.1 |########################################
    788.8 |######
    799.5 |
    810.3 |
    821.0 |
    831.7 |###
    842.4 |####################
    853.1 |######
    863.9 |###
    874.6 |
    885.3 |###
    896.0 |###
    906.8 |
    917.5 |
    928.2 |################
    938.9 |##########
    949.6 |###
    960.4 |
    971.1 |
    981.8 |
  (1 below, 4 above range)

satfold-neon8 (n=40, range 755.3-857.9 ns)
    755.3 |
    760.5 |
    765.6 |
    770.7 |##
    775.8 |
    781.0 |######
    786.1 |##
    791.2 |########################################
    796.4 |##
    801.5 |
    806.6 |##
    811.7 |
    816.9 |
    822.0 |
    827.1 |##
    832.3 |####
    837.4 |##
    842.5 |##
    847.6 |
    852.8 |
  (4 below, 5 above range)

satfold-nolaw (n=40, range 1124.6-4658.2 ns)
   1124.6 |
   1301.2 |
   1477.9 |
   1654.6 |#####
   1831.3 |
   2008.0 |#####
   2184.7 |
   2361.4 |
   2538.0 |#####
   2714.7 |
   2891.4 |#####
   3068.1 |######################
   3244.8 |#################
   3421.5 |#################
   3598.1 |########################################
   3774.8 |###########
   3951.5 |#################
   4128.2 |###########
   4304.9 |
   4481.6 |
  (7 below, 5 above range)

satfold-seq (n=40, range 796.9-1064.2 ns)
    796.9 |#########################
    810.2 |#####
    823.6 |#####
    837.0 |#####
    850.3 |
    863.7 |###################################
    877.1 |#####
    890.4 |
    903.8 |
    917.2 |
    930.5 |
    943.9 |##############################
    957.3 |########################################
    970.6 |
    984.0 |
    997.4 |#####
   1010.7 |
   1024.1 |
   1037.5 |#####
   1050.8 |##########
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=30.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=20.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=72.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 6.1x (possible bimodal distribution)
- **satfold-nolaw**: CV=37.8% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 4.1x (possible bimodal distribution)
