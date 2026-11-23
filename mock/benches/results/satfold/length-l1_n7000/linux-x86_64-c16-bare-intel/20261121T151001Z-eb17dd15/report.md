# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 beats baseline by 80% (significant)

satfold-lanes64 is -12.49 us (80%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 8.8x slower than the field

satfold-seq (17.21 us) is 8.8x the fastest (1.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon, satfold-neon8) are a dead heat (<1%)

satfold-neon (1.95 us) and satfold-neon8 (1.95 us) differ by 0.17%, inside the noise, even though the wider field spreads 782.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16 shows alternating (throttle bounce) (autocorr -0.60)

satfold-lanes16's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.8x the fastest

Fastest satfold-neon (1.95 us) to slowest satfold-seq (17.21 us): 8.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-neon8 is inconsistent: worst-20% is 4.4x its best-20%

satfold-neon8's best 20% of batches run at 1.58 us but its worst 20% at 6.90 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 1949.2 ns median (-87.5% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 8.83x (fastest 1949.2 ns, slowest 17205.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 15726ns | 15720ns | 14122ns | 15707ns | 17390ns | base |
| satfold-lanes16 | 4170ns | 2950ns | 2537ns | 3302ns | 8406ns | -73.48% |
| satfold-lanes16-constl | 4051ns | 3081ns | 2519ns | 2968ns | 8833ns | -74.24% |
| satfold-lanes4-idx | 3898ns | 2228ns | 1821ns | 3335ns | 7663ns | -75.21% |
| satfold-lanes64 | 3819ns | 3759ns | 1725ns | 3449ns | 7024ns | -75.71% |
| satfold-neon | 3746ns | 2015ns | 1639ns | 3324ns | 7119ns | -76.18% |
| satfold-neon8 | 3666ns | 2018ns | 1634ns | 3172ns | 7181ns | -76.69% |
| satfold-nolaw | 6863ns | 7481ns | 6135ns | 6878ns | 7548ns | -56.36% |
| satfold-seq | 17287ns | 17278ns | 17256ns | 17279ns | 17340ns | +9.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 15656ns | 14063ns | 17305ns | base | 2.093 |
| satfold-lanes16 | 4075ns | 2486ns | 8184ns | -73.97% | 8.041 |
| satfold-lanes16-constl | 3958ns | 2467ns | 8607ns | -74.72% | 8.279 |
| satfold-lanes4-idx | 3777ns | 1770ns | 7398ns | -75.88% | 8.676 |
| satfold-lanes64 | 3683ns | 1668ns | 6748ns | -76.47% | 8.896 |
| satfold-neon | 3611ns | 1585ns | 6827ns | -76.94% | 9.075 |
| satfold-neon8 | 3535ns | 1581ns | 6897ns | -77.42% | 9.270 |
| satfold-nolaw | 6793ns | 6077ns | 7453ns | -56.61% | 4.824 |
| satfold-seq | 17209ns | 17183ns | 17245ns | +9.92% | 1.904 |

## Performance model

- Peak throughput: **20.731 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.094 | 10.1% |
| satfold-lanes16 | 11.337 | 54.7% |
| satfold-lanes16-constl | 10.859 | 52.4% |
| satfold-lanes4-idx | 15.138 | 73.0% |
| satfold-lanes64 | 9.018 | 43.5% |
| satfold-neon | 16.811 | 81.1% |
| satfold-neon8 | 16.782 | 81.0% |
| satfold-nolaw | 4.421 | 21.3% |
| satfold-seq | 1.905 | 9.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 15726ns | 15726ns | base |
| satfold-lanes16 | 4170ns | 4170ns | -73.48% |
| satfold-lanes16-constl | 4051ns | 4051ns | -74.24% |
| satfold-lanes4-idx | 3898ns | 3898ns | -75.21% |
| satfold-lanes64 | 3819ns | 3819ns | -75.71% |
| satfold-neon | 3746ns | 3746ns | -76.18% |
| satfold-neon8 | 3666ns | 3666ns | -76.69% |
| satfold-nolaw | 6863ns | 6863ns | -56.36% |
| satfold-seq | 17287ns | 17287ns | +9.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 15646ns | base | --- | [14076, 17198] | --- | --- | --- | --- |
| satfold-lanes16 | 2890ns | -11310.6ns (-72.3%) | [-13014, -11015]ns | [2492, 3049] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3018ns | -11598.0ns (-74.1%) | [-14135, -11054]ns | [2748, 3030] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2165ns | -11962.2ns (-76.5%) | [-12303, -11805]ns | [1775, 5093] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 3633ns | -12493.7ns (-79.8%) | [-12774, -12124]ns | [1942, 4451] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1949ns | -12134.6ns (-77.6%) | [-12155, -12033]ns | [1938, 4969] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1953ns | -12483.3ns (-79.8%) | [-12498, -11176]ns | [1940, 5012] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7411ns | -8878.3ns (-56.7%) | [-9767, -7993]ns | [6089, 7423] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17206ns | +1636.5ns (+10.5%) | [+19, +3118]ns | [17199, 17214] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14071ns | -82.3% | -78.6% | -87.4% | -86.9% | -86.3% | -78.3% | -56.8% | +22.2% |
| 1 | 2 | 14065ns | -78.4% | -82.4% | -84.6% | -79.2% | -86.2% | -88.7% | -56.7% | +22.4% |
| 1 | 3 | 17200ns | -85.5% | -85.6% | -89.7% | -88.7% | -72.2% | -88.7% | -56.9% | -0.0% |
| 1 | 4 | 17201ns | -84.8% | -82.4% | -70.4% | -74.5% | -81.0% | -79.1% | -56.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.250 | moderate+ |
| satfold-lanes16 | -0.596 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.250 | moderate- |
| satfold-lanes4-idx | -0.159 | ok |
| satfold-lanes64 | -0.383 | moderate- |
| satfold-neon | -0.048 | ok |
| satfold-neon8 | -0.205 | moderate- |
| satfold-nolaw | 0.253 | moderate+ |
| satfold-seq | -0.449 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.1ns | 15655.7ns | 0.1% |  |
| satfold-lanes16 | 31.1ns | 4075.1ns | 0.8% |  |
| satfold-lanes16-constl | 31.1ns | 3957.8ns | 0.8% |  |
| satfold-lanes4-idx | 40.9ns | 3776.7ns | 1.1% |  |
| satfold-lanes64 | 43.5ns | 3683.5ns | 1.2% |  |
| satfold-neon | 42.8ns | 3610.8ns | 1.2% |  |
| satfold-neon8 | 41.8ns | 3534.8ns | 1.2% |  |
| satfold-nolaw | 21.5ns | 6792.6ns | 0.3% |  |
| satfold-seq | 23.2ns | 17209.5ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14063.1-17304.7 ns)
  14063.1 |###################################
  14225.2 |
  14387.3 |
  14549.3 |
  14711.4 |
  14873.5 |
  15035.6 |
  15197.6 |
  15359.7 |
  15521.8 |
  15683.9 |
  15846.0 |
  16008.0 |
  16170.1 |
  16332.2 |
  16494.3 |
  16656.3 |
  16818.4 |
  16980.5 |
  17142.6 |########################################
  (3 below, 1 above range)

satfold-lanes16 (n=40, range 2486.1-8184.2 ns)
   2486.1 |########################################
   2771.0 |#####################
   3055.9 |
   3340.8 |
   3625.7 |
   3910.6 |
   4195.5 |
   4480.4 |
   4765.3 |
   5050.2 |
   5335.1 |
   5620.0 |
   5904.9 |
   6189.9 |##
   6474.8 |
   6759.7 |####
   7044.6 |##
   7329.5 |####
   7614.4 |####
   7899.3 |
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 2467.5-8607.1 ns)
   2467.5 |#########################
   2774.4 |########################################
   3081.4 |
   3388.4 |##
   3695.4 |
   4002.4 |
   4309.3 |
   4616.3 |
   4923.3 |
   5230.3 |
   5537.3 |
   5844.3 |
   6151.2 |##
   6458.2 |##
   6765.2 |
   7072.2 |
   7379.2 |
   7686.1 |##
   7993.1 |##
   8300.1 |
  (3 below, 4 above range)

satfold-lanes4-idx (n=40, range 1770.3-7398.4 ns)
   1770.3 |########################################
   2051.7 |#################
   2333.1 |
   2614.5 |
   2895.9 |##
   3177.3 |
   3458.7 |
   3740.1 |##
   4021.6 |##
   4303.0 |
   4584.4 |
   4865.8 |##
   5147.2 |##
   5428.6 |########
   5710.0 |
   5991.4 |##
   6272.8 |##
   6554.2 |#####
   6835.6 |
   7117.0 |#####
  (2 below, 4 above range)

satfold-lanes64 (n=40, range 1668.2-6747.9 ns)
   1668.2 |
   1922.2 |########################################
   2176.2 |
   2430.2 |
   2684.1 |
   2938.1 |
   3192.1 |
   3446.1 |######
   3700.1 |###
   3954.1 |
   4208.1 |###############
   4462.0 |######
   4716.0 |###
   4970.0 |
   5224.0 |###
   5478.0 |###
   5732.0 |###
   5985.9 |######
   6239.9 |###
   6493.9 |###
  (6 below, 3 above range)

satfold-neon (n=40, range 1585.0-6827.0 ns)
   1585.0 |#########
   1847.1 |########################################
   2109.2 |###
   2371.3 |
   2633.4 |
   2895.5 |
   3157.6 |###
   3419.7 |
   3681.8 |
   3943.9 |
   4206.0 |
   4468.1 |###
   4730.2 |######
   4992.3 |######
   5254.4 |#########
   5516.5 |###
   5778.6 |#########
   6040.7 |
   6302.8 |
   6564.9 |
  (5 below, 5 above range)

satfold-neon8 (n=40, range 1580.6-6896.6 ns)
   1580.6 |#####################
   1846.4 |########################################
   2112.2 |
   2378.0 |
   2643.8 |
   2909.6 |
   3175.4 |
   3441.2 |
   3707.0 |
   3972.8 |
   4238.6 |
   4504.4 |###
   4770.2 |###
   5036.0 |###
   5301.8 |######
   5567.6 |###
   5833.4 |###
   6099.2 |######
   6365.0 |#########
   6630.8 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 6077.2-7453.2 ns)
   6077.2 |################################
   6146.0 |
   6214.8 |
   6283.6 |
   6352.4 |
   6421.2 |
   6490.0 |
   6558.8 |
   6627.6 |
   6696.4 |
   6765.2 |
   6834.0 |
   6902.8 |
   6971.6 |
   7040.4 |
   7109.2 |
   7178.0 |
   7246.8 |
   7315.6 |
   7384.4 |########################################
  (3 below, 1 above range)

satfold-seq (n=40, range 17183.2-17245.2 ns)
  17183.2 |##########
  17186.3 |
  17189.4 |##########
  17192.5 |########################################
  17195.6 |########################################
  17198.7 |####################
  17201.8 |##############################
  17204.9 |########################################
  17208.0 |####################
  17211.1 |##########
  17214.2 |##############################
  17217.3 |##############################
  17220.4 |##############################
  17223.5 |##########
  17226.6 |####################
  17229.7 |##########
  17232.8 |
  17235.9 |
  17239.0 |
  17242.1 |
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=57.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=61.8% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=61.3% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-lanes64**: CV=54.1% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-neon**: CV=59.1% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **satfold-neon8**: CV=62.8% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 4.4x (possible bimodal distribution)
