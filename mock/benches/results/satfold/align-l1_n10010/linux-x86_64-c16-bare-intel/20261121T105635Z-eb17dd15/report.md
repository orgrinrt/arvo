# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (25.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes64 at 2.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes64 dominates: 13% faster than the next best (satfold-neon)

satfold-lanes64 (2.05 us) leads satfold-neon (2.32 us) by 13%, a clear separation rather than a photo finish. CV 10.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes64 beats baseline by 92% (significant)

satfold-lanes64 is -23.22 us (92%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 12.3x slower than the field

satfold-iterfold (25.32 us) is 12.3x the fastest (2.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.67)

satfold-neon's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.3x the fastest

Fastest satfold-lanes64 (2.05 us) to slowest satfold-iterfold (25.32 us): 12.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.9x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.08 us but its worst 20% at 8.18 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 2053.4 ns median (-91.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 12.33x (fastest 2053.4 ns, slowest 25320.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112835759 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 24268ns | 25394ns | 20774ns | 25045ns | 25434ns | base |
| satfold-lanes16 | 3555ns | 3770ns | 2315ns | 3703ns | 4351ns | -85.35% |
| satfold-lanes16-constl | 3544ns | 3720ns | 2375ns | 3684ns | 4293ns | -85.40% |
| satfold-lanes4-idx | 4570ns | 3276ns | 2138ns | 4080ns | 8471ns | -81.17% |
| satfold-lanes64 | 2270ns | 2249ns | 2011ns | 2223ns | 2670ns | -90.65% |
| satfold-neon | 2647ns | 2555ns | 2103ns | 2589ns | 3364ns | -89.09% |
| satfold-nolaw | 9982ns | 8664ns | 8640ns | 9182ns | 13725ns | -58.87% |
| satfold-seq | 24241ns | 25389ns | 20767ns | 25006ns | 25421ns | -0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24194ns | 20714ns | 25345ns | base | 1.354 |
| satfold-lanes16 | 3352ns | 2189ns | 4072ns | -86.15% | 9.776 |
| satfold-lanes16-constl | 3338ns | 2242ns | 4019ns | -86.20% | 9.818 |
| satfold-lanes4-idx | 4438ns | 2079ns | 8184ns | -81.66% | 7.384 |
| satfold-lanes64 | 2063ns | 1837ns | 2398ns | -91.47% | 15.887 |
| satfold-neon | 2408ns | 1919ns | 3049ns | -90.05% | 13.608 |
| satfold-nolaw | 9909ns | 8580ns | 13617ns | -59.04% | 3.307 |
| satfold-seq | 24167ns | 20707ns | 25331ns | -0.11% | 1.356 |

## Performance model

- Peak throughput: **17.833 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.294 | 7.3% |
| satfold-lanes16 | 9.184 | 51.5% |
| satfold-lanes16-constl | 9.348 | 52.4% |
| satfold-lanes4-idx | 10.225 | 57.3% |
| satfold-lanes64 | 15.958 | 89.5% |
| satfold-neon | 14.111 | 79.1% |
| satfold-nolaw | 3.808 | 21.4% |
| satfold-seq | 1.294 | 7.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24268ns | 24268ns | base |
| satfold-lanes16 | 3555ns | 3555ns | -85.35% |
| satfold-lanes16-constl | 3544ns | 3544ns | -85.40% |
| satfold-lanes4-idx | 4570ns | 4570ns | -81.17% |
| satfold-lanes64 | 2270ns | 2270ns | -90.65% |
| satfold-neon | 2647ns | 2647ns | -89.09% |
| satfold-nolaw | 9982ns | 9982ns | -58.87% |
| satfold-seq | 24241ns | 24241ns | -0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 25320ns | base | --- | [25315, 25325] | --- | --- | --- | --- |
| satfold-lanes16 | 3568ns | -21462.2ns (-84.8%) | [-21874, -21353]ns | [3227, 3813] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3505ns | -21583.4ns (-85.2%) | [-21963, -21406]ns | [3256, 3640] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 3205ns | -19955.0ns (-78.8%) | [-22143, -18713]ns | [2568, 5887] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2053ns | -23222.6ns (-91.7%) | [-23280, -23028]ns | [1932, 2074] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2322ns | -23009.0ns (-90.9%) | [-23174, -22596]ns | [2122, 2611] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8606ns | -16716.3ns (-66.0%) | [-16728, -15475]ns | [8600, 9739] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 25317ns | no significant difference | [-14, +6]ns | [25308, 25321] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20715ns | -82.8% | -83.0% | -79.8% | -90.2% | -86.6% | -49.1% | +22.2% |
| 1 | 2 | 25324ns | -84.6% | -84.6% | -89.9% | -92.5% | -92.0% | -65.3% | -0.0% |
| 1 | 3 | 25329ns | -85.1% | -85.9% | -87.3% | -91.1% | -89.8% | -66.1% | -18.2% |
| 1 | 4 | 25324ns | -88.6% | -88.1% | -76.1% | -92.1% | -91.8% | -66.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.083 | ok |
| satfold-lanes16 | -0.088 | ok |
| satfold-lanes16-constl | 0.002 | ok |
| satfold-lanes4-idx | -0.106 | ok |
| satfold-lanes64 | -0.594 | HIGH- (thermal bounce) |
| satfold-neon | -0.674 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.008 | ok |
| satfold-seq | -0.416 | moderate- |

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
| satfold-iterfold | 22.3ns | 24194.1ns | 0.1% |  |
| satfold-lanes16 | 71.2ns | 3351.8ns | 2.1% |  |
| satfold-lanes16-constl | 72.0ns | 3337.6ns | 2.2% |  |
| satfold-lanes4-idx | 39.0ns | 4438.0ns | 0.9% |  |
| satfold-lanes64 | 71.1ns | 2062.6ns | 3.4% |  |
| satfold-neon | 80.3ns | 2408.0ns | 3.3% |  |
| satfold-nolaw | 22.3ns | 9908.8ns | 0.2% |  |
| satfold-seq | 22.4ns | 24166.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20713.8-25345.5 ns)
  20713.8 |#######
  20945.3 |
  21176.9 |
  21408.5 |#
  21640.1 |
  21871.7 |
  22103.3 |
  22334.9 |
  22566.4 |
  22798.0 |
  23029.6 |
  23261.2 |
  23492.8 |
  23724.4 |
  23956.0 |
  24187.5 |
  24419.1 |
  24650.7 |
  24882.3 |
  25113.9 |########################################
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 2189.0-4071.6 ns)
   2189.0 |
   2283.1 |
   2377.2 |#####
   2471.4 |
   2565.5 |##########
   2659.6 |
   2753.8 |##########
   2847.9 |##########
   2942.0 |
   3036.1 |###############
   3130.3 |#####
   3224.4 |##########
   3318.5 |##########
   3412.7 |
   3506.8 |#########################
   3600.9 |##########
   3695.0 |#####
   3789.2 |#####
   3883.3 |########################################
   3977.4 |###############
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 2241.6-4018.7 ns)
   2241.6 |#####
   2330.4 |
   2419.3 |
   2508.1 |
   2597.0 |
   2685.8 |#####
   2774.7 |##########
   2863.6 |
   2952.4 |###############
   3041.3 |#####
   3130.1 |
   3219.0 |###################################
   3307.8 |
   3396.7 |#####
   3485.5 |#########################
   3574.4 |###############
   3663.2 |
   3752.1 |
   3841.0 |########################################
   3929.8 |###############
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 2079.0-8184.4 ns)
   2079.0 |####################
   2384.3 |########################################
   2689.5 |
   2994.8 |###################################
   3300.1 |#####
   3605.4 |
   3910.6 |
   4215.9 |
   4521.2 |
   4826.4 |#####
   5131.7 |
   5437.0 |
   5742.3 |##########
   6047.5 |#####
   6352.8 |###############
   6658.1 |##########
   6963.3 |###############
   7268.6 |
   7573.9 |
   7879.2 |
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 1837.5-2398.4 ns)
   1837.5 |
   1865.5 |##########
   1893.6 |###################################
   1921.6 |####################
   1949.6 |
   1977.7 |
   2005.7 |#####
   2033.8 |########################################
   2061.8 |#####
   2089.9 |##########
   2117.9 |
   2146.0 |#####
   2174.0 |
   2202.1 |
   2230.1 |####################
   2258.1 |#####
   2286.2 |##########
   2314.2 |
   2342.3 |
   2370.3 |
  (4 below, 3 above range)

satfold-neon (n=40, range 1918.9-3049.0 ns)
   1918.9 |####################
   1975.4 |######
   2031.9 |########################################
   2088.4 |
   2144.9 |######
   2201.4 |##########################
   2257.9 |
   2314.4 |####################
   2370.9 |######
   2427.4 |
   2483.9 |
   2540.4 |#############
   2596.9 |#################################
   2653.4 |
   2709.9 |######
   2766.5 |######
   2823.0 |######
   2879.5 |#############
   2936.0 |#############
   2992.5 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 8580.3-13616.5 ns)
   8580.3 |########################################
   8832.1 |####
   9084.0 |
   9335.8 |
   9587.6 |
   9839.4 |
  10091.2 |
  10343.0 |####################
  10594.8 |##
  10846.6 |
  11098.4 |
  11350.2 |
  11602.1 |
  11853.9 |
  12105.7 |
  12357.5 |
  12609.3 |
  12861.1 |
  13112.9 |
  13364.7 |##
  (4 below, 2 above range)

satfold-seq (n=40, range 20706.9-25330.6 ns)
  20706.9 |#######
  20938.1 |
  21169.3 |
  21400.5 |
  21631.6 |
  21862.8 |
  22094.0 |
  22325.2 |
  22556.4 |
  22787.6 |
  23018.8 |
  23250.0 |
  23481.1 |
  23712.3 |
  23943.5 |
  24174.7 |
  24405.9 |
  24637.1 |
  24868.3 |
  25099.5 |########################################
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=22.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=21.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=54.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-nolaw**: CV=29.8% (high variance, measurements may be unstable)
