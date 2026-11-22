# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (25.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes64 at 2.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon beats baseline by 91% (significant)

satfold-neon is -23.35 us (91%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 12.3x slower than the field

satfold-iterfold (25.58 us) is 12.3x the fastest (2.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.75)

satfold-seq's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.3x the fastest

Fastest satfold-lanes64 (2.08 us) to slowest satfold-iterfold (25.58 us): 12.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16 is inconsistent: worst-20% is 3.5x its best-20%

satfold-lanes16's best 20% of batches run at 1.25 us but its worst 20% at 4.36 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 2079.7 ns median (-91.9% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 12.30x (fastest 2079.7 ns, slowest 25579.6 ns)

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
| Counter frequency | 2112779582 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 24477ns | 25655ns | 20915ns | 25256ns | 25699ns | base |
| satfold-lanes16 | 3471ns | 3968ns | 1324ns | 3790ns | 4658ns | -85.82% |
| satfold-lanes16-constl | 3370ns | 3472ns | 2042ns | 3478ns | 4373ns | -86.23% |
| satfold-lanes4-idx | 4858ns | 4224ns | 3556ns | 4228ns | 8047ns | -80.15% |
| satfold-lanes64 | 2358ns | 2296ns | 2039ns | 2313ns | 2813ns | -90.37% |
| satfold-neon | 2336ns | 2300ns | 2060ns | 2319ns | 2662ns | -90.46% |
| satfold-nolaw | 9731ns | 9729ns | 8738ns | 9722ns | 10748ns | -60.25% |
| satfold-seq | 23101ns | 23114ns | 20780ns | 23097ns | 25433ns | -5.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 24400ns | 20854ns | 25610ns | base | 1.343 |
| satfold-lanes16 | 3271ns | 1253ns | 4360ns | -86.60% | 10.019 |
| satfold-lanes16-constl | 3174ns | 1930ns | 4101ns | -86.99% | 10.325 |
| satfold-lanes4-idx | 4765ns | 3496ns | 7865ns | -80.47% | 6.876 |
| satfold-lanes64 | 2138ns | 1851ns | 2543ns | -91.24% | 15.327 |
| satfold-neon | 2122ns | 1882ns | 2393ns | -91.30% | 15.439 |
| satfold-nolaw | 9660ns | 8678ns | 10670ns | -60.41% | 3.392 |
| satfold-seq | 23028ns | 20719ns | 25348ns | -5.63% | 1.423 |

## Performance model

- Peak throughput: **26.156 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.281 | 4.9% |
| satfold-lanes16 | 8.754 | 33.5% |
| satfold-lanes16-constl | 9.980 | 38.2% |
| satfold-lanes4-idx | 7.894 | 30.2% |
| satfold-lanes64 | 15.756 | 60.2% |
| satfold-neon | 15.577 | 59.6% |
| satfold-nolaw | 3.395 | 13.0% |
| satfold-seq | 1.423 | 5.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 24477ns | 24477ns | base |
| satfold-lanes16 | 3471ns | 3471ns | -85.82% |
| satfold-lanes16-constl | 3370ns | 3370ns | -86.23% |
| satfold-lanes4-idx | 4858ns | 4858ns | -80.15% |
| satfold-lanes64 | 2358ns | 2358ns | -90.37% |
| satfold-neon | 2336ns | 2336ns | -90.46% |
| satfold-nolaw | 9731ns | 9731ns | -60.25% |
| satfold-seq | 23101ns | 23101ns | -5.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 25580ns | base | --- | [25572, 25587] | --- | --- | --- | --- |
| satfold-lanes16 | 3743ns | -21630.7ns (-84.6%) | [-22233, -21240]ns | [3059, 3964] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3283ns | -21859.3ns (-85.5%) | [-22217, -21526]ns | [3021, 3530] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 4151ns | -21416.5ns (-83.7%) | [-21429, -21280]ns | [4150, 4163] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2080ns | -23270.0ns (-91.0%) | [-23450, -23096]ns | [2040, 2214] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2104ns | -23354.6ns (-91.3%) | [-23451, -23259]ns | [2049, 2167] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 9652ns | -15864.2ns (-62.0%) | [-16884, -14988]ns | [8704, 10591] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 23023ns | -2556.2ns (-10.0%) | [-4859, -250]ns | [20722, 25325] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20930ns | -81.1% | -85.5% | -83.7% | -90.6% | -90.8% | -49.1% | +21.0% |
| 1 | 2 | 25583ns | -84.6% | -86.3% | -83.8% | -90.4% | -91.8% | -58.6% | -19.0% |
| 1 | 3 | 25581ns | -84.3% | -84.6% | -83.8% | -92.6% | -91.4% | -66.1% | -1.0% |
| 1 | 4 | 25593ns | -88.0% | -88.1% | -83.8% | -91.7% | -91.7% | -66.0% | -19.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.083 | ok |
| satfold-lanes16 | -0.128 | ok |
| satfold-lanes16-constl | -0.289 | moderate- |
| satfold-lanes4-idx | -0.085 | ok |
| satfold-lanes64 | -0.689 | HIGH- (thermal bounce) |
| satfold-neon | 0.066 | ok |
| satfold-nolaw | 0.255 | moderate+ |
| satfold-seq | -0.750 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.1ns | 24400.5ns | 0.1% |  |
| satfold-lanes16 | 70.3ns | 3270.7ns | 2.1% |  |
| satfold-lanes16-constl | 71.3ns | 3173.7ns | 2.2% |  |
| satfold-lanes4-idx | 27.5ns | 4765.4ns | 0.6% |  |
| satfold-lanes64 | 74.7ns | 2138.0ns | 3.5% |  |
| satfold-neon | 73.2ns | 2122.4ns | 3.4% |  |
| satfold-nolaw | 21.9ns | 9660.0ns | 0.2% |  |
| satfold-seq | 21.8ns | 23027.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20853.8-25609.5 ns)
  20853.8 |##########
  21091.6 |
  21329.4 |
  21567.1 |
  21804.9 |
  22042.7 |
  22280.5 |
  22518.3 |
  22756.1 |
  22993.9 |
  23231.7 |
  23469.5 |
  23707.2 |
  23945.0 |
  24182.8 |
  24420.6 |
  24658.4 |
  24896.2 |
  25134.0 |
  25371.8 |########################################
  (3 below, 2 above range)

satfold-lanes16 (n=40, range 1252.8-4360.1 ns)
   1252.8 |
   1408.2 |
   1563.5 |
   1718.9 |
   1874.3 |
   2029.6 |
   2185.0 |###
   2340.4 |
   2495.7 |
   2651.1 |
   2806.4 |###
   2961.8 |#####################
   3117.2 |
   3272.5 |##########
   3427.9 |
   3583.3 |#######
   3738.6 |#######
   3894.0 |########################################
   4049.4 |###
   4204.7 |###
  (7 below, 5 above range)

satfold-lanes16-constl (n=40, range 1930.1-4101.0 ns)
   1930.1 |
   2038.6 |
   2147.2 |
   2255.7 |######
   2364.3 |
   2472.8 |
   2581.4 |
   2689.9 |######
   2798.4 |########################################
   2907.0 |#############
   3015.5 |####################
   3124.1 |######
   3232.6 |#################################
   3341.2 |#############
   3449.7 |######
   3558.3 |##########################
   3666.8 |
   3775.4 |######
   3883.9 |########################################
   3992.5 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 3495.9-7864.8 ns)
   3495.9 |
   3714.3 |
   3932.8 |########################################
   4151.2 |################################
   4369.7 |
   4588.1 |
   4806.5 |##
   5025.0 |##
   5243.4 |
   5461.9 |
   5680.3 |
   5898.8 |
   6117.2 |
   6335.7 |
   6554.1 |
   6772.6 |
   6991.0 |
   7209.4 |
   7427.9 |
   7646.3 |
  (7 below, 4 above range)

satfold-lanes64 (n=40, range 1850.6-2543.2 ns)
   1850.6 |
   1885.3 |########################################
   1919.9 |#####
   1954.5 |#####
   1989.2 |
   2023.8 |##############################
   2058.4 |#####
   2093.1 |####################
   2127.7 |#####
   2162.3 |#####
   2196.9 |
   2231.6 |##########
   2266.2 |###############
   2300.8 |
   2335.5 |##########
   2370.1 |#####
   2404.7 |
   2439.4 |##########
   2474.0 |
   2508.6 |#####
  (3 below, 3 above range)

satfold-neon (n=40, range 1882.3-2393.0 ns)
   1882.3 |######
   1907.8 |
   1933.4 |#############
   1958.9 |####################
   1984.4 |######
   2010.0 |
   2035.5 |########################################
   2061.0 |######
   2086.6 |##########################
   2112.1 |
   2137.7 |##########################
   2163.2 |######
   2188.7 |
   2214.3 |##########################
   2239.8 |######
   2265.3 |
   2290.9 |####################
   2316.4 |#############
   2342.0 |
   2367.5 |
  (4 below, 3 above range)

satfold-nolaw (n=40, range 8677.6-10669.8 ns)
   8677.6 |###################################
   8777.2 |
   8876.8 |
   8976.4 |
   9076.0 |
   9175.6 |
   9275.2 |
   9374.8 |
   9474.5 |
   9574.1 |
   9673.7 |
   9773.3 |
   9872.9 |
   9972.5 |
  10072.1 |
  10171.7 |
  10271.3 |
  10371.0 |
  10470.6 |
  10570.2 |########################################
  (5 below, 3 above range)

satfold-seq (n=40, range 20719.4-25347.5 ns)
  20719.4 |###################################
  20950.8 |
  21182.2 |
  21413.6 |
  21645.0 |
  21876.4 |
  22107.8 |
  22339.3 |
  22570.7 |
  22802.1 |
  23033.5 |
  23264.9 |
  23496.3 |
  23727.7 |
  23959.1 |
  24190.5 |
  24421.9 |
  24653.3 |
  24884.7 |
  25116.1 |########################################
  (3 below, 1 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=34.3% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=25.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=45.4% (high variance, measurements may be unstable)
