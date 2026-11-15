# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-seq, satfold-iterfold) are a dead heat (<1%)

satfold-seq (2.38 us) and satfold-iterfold (2.40 us) differ by 0.88%, inside the noise, even though the wider field spreads 4.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.74)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (110 ns) is smaller than the fastest variant's own run-to-run std-dev (2.40 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader satfold-seq vs stability leader satfold-lanes16 (+2% speed for 1.1x steadier)

satfold-seq is fastest (2.38 us, CV 100.8%); satfold-lanes16 gives up 1.8% median for 1.1x lower variance (CV 89.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-neon8 is inconsistent: worst-20% is 4.2x its best-20%

satfold-neon8's best 20% of batches run at 1.98 us but its worst 20% at 8.24 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.6% of the fastest

All 9 variants sit between 2.38 us and 2.49 us - a 4.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: satfold-seq** at 2380.5 ns median (-0.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.05x (fastest 2380.5 ns, slowest 2490.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 3955ns | 2465ns | 2005ns | 3214ns | 8126ns | base |
| satfold-lanes16 | 4126ns | 2486ns | 2130ns | 3679ns | 7464ns | +4.34% |
| satfold-lanes16-constl | 4117ns | 2554ns | 2162ns | 3366ns | 8325ns | +4.11% |
| satfold-lanes4-idx | 4181ns | 2483ns | 2080ns | 3745ns | 7593ns | +5.73% |
| satfold-lanes64 | 3922ns | 2484ns | 2031ns | 3025ns | 8502ns | -0.84% |
| satfold-neon | 3958ns | 2484ns | 2031ns | 3257ns | 7988ns | +0.08% |
| satfold-neon8 | 4166ns | 2488ns | 2033ns | 3423ns | 8529ns | +5.35% |
| satfold-nolaw | 4289ns | 2488ns | 2482ns | 3343ns | 8936ns | +8.46% |
| satfold-seq | 3879ns | 2445ns | 2087ns | 2985ns | 8351ns | -1.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3839ns | 1953ns | 7852ns | base | 8.536 |
| satfold-lanes16 | 4005ns | 2072ns | 7205ns | +4.32% | 8.183 |
| satfold-lanes16-constl | 4001ns | 2107ns | 8051ns | +4.21% | 8.191 |
| satfold-lanes4-idx | 4053ns | 2024ns | 7310ns | +5.57% | 8.085 |
| satfold-lanes64 | 3809ns | 1979ns | 8221ns | -0.79% | 8.604 |
| satfold-neon | 3846ns | 1979ns | 7729ns | +0.18% | 8.521 |
| satfold-neon8 | 4047ns | 1982ns | 8240ns | +5.41% | 8.098 |
| satfold-nolaw | 4164ns | 2419ns | 8622ns | +8.46% | 7.870 |
| satfold-seq | 3764ns | 2030ns | 8071ns | -1.94% | 8.705 |

## Performance model

- Peak throughput: **16.777 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 13.645 | 81.3% |
| satfold-lanes16 | 13.523 | 80.6% |
| satfold-lanes16-constl | 13.156 | 78.4% |
| satfold-lanes4-idx | 13.540 | 80.7% |
| satfold-lanes64 | 13.538 | 80.7% |
| satfold-neon | 13.534 | 80.7% |
| satfold-neon8 | 13.510 | 80.5% |
| satfold-nolaw | 13.510 | 80.5% |
| satfold-seq | 13.765 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3955ns | 3955ns | base |
| satfold-lanes16 | 4126ns | 4126ns | +4.34% |
| satfold-lanes16-constl | 4117ns | 4117ns | +4.11% |
| satfold-lanes4-idx | 4181ns | 4181ns | +5.73% |
| satfold-lanes64 | 3922ns | 3922ns | -0.84% |
| satfold-neon | 3958ns | 3958ns | +0.08% |
| satfold-neon8 | 4166ns | 4166ns | +5.35% |
| satfold-nolaw | 4289ns | 4289ns | +8.46% |
| satfold-seq | 3879ns | 3879ns | -1.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 2402ns | base | --- | [2339, 3506] | --- | --- | --- | --- |
| satfold-lanes16 | 2423ns | no significant difference | [-232, +367]ns | [2326, 5280] | no | 1.0000 | 1.0000 | 0 |
| satfold-lanes16-constl | 2491ns | +149.4ns (+6.2%) | [+86, +486]ns | [2461, 4213] | YES (adj: no) | 0.0664 | 0.0166 | 0 |
| satfold-lanes4-idx | 2420ns | no significant difference | [-38, +365]ns | [2377, 5345] | no | 0.4910 | 0.4296 | 0 |
| satfold-lanes64 | 2420ns | no significant difference | [-0, +72]ns | [2418, 2781] | no | 0.4910 | 0.4296 | 0 |
| satfold-neon | 2421ns | no significant difference | [-355, +71]ns | [1985, 3330] | no | 0.2462 | 0.1539 | 0 |
| satfold-neon8 | 2425ns | +72.9ns (+3.0%) | [+4, +423]ns | [2422, 4066] | YES (adj: no) | 0.1026 | 0.0385 | 0 |
| satfold-nolaw | 2425ns | +264.1ns (+11.0%) | [+76, +479]ns | [2420, 2793] | YES | 0.0015 | 0.0002 | 0 |
| satfold-seq | 2380ns | no significant difference | [-7, +53]ns | [2354, 2434] | no | 0.2462 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1985ns | +169.0% | +25.0% | +21.2% | -0.3% | +33.2% | +88.2% | +27.6% | +0.2% |
| 1 | 2 | 2419ns | +126.1% | +89.4% | +118.9% | +0.7% | -1.2% | -17.6% | +11.4% | -0.0% |
| 1 | 3 | 2369ns | +0.5% | +6.5% | +1.8% | +2.4% | -16.4% | +2.6% | +2.2% | +0.1% |
| 1 | 4 | 4865ns | -52.4% | -56.8% | +11.3% | -44.4% | -50.0% | -50.0% | -50.2% | -50.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.065 | ok |
| satfold-lanes16 | 0.244 | moderate+ |
| satfold-lanes16-constl | -0.282 | moderate- |
| satfold-lanes4-idx | -0.738 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.021 | ok |
| satfold-neon | -0.127 | ok |
| satfold-neon8 | -0.307 | moderate- |
| satfold-nolaw | -0.095 | ok |
| satfold-seq | -0.154 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 1/4, lost 3/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 2/4, lost 2/4
- **satfold-nolaw**: won 1/4, lost 3/4
- **satfold-seq**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 43.9ns | 3838.9ns | 1.1% |  |
| satfold-lanes16 | 40.1ns | 4004.6ns | 1.0% |  |
| satfold-lanes16-constl | 38.1ns | 4000.7ns | 1.0% |  |
| satfold-lanes4-idx | 40.7ns | 4052.8ns | 1.0% |  |
| satfold-lanes64 | 37.3ns | 3808.6ns | 1.0% |  |
| satfold-neon | 38.9ns | 3845.7ns | 1.0% |  |
| satfold-neon8 | 39.6ns | 4046.5ns | 1.0% |  |
| satfold-nolaw | 41.0ns | 4163.5ns | 1.0% |  |
| satfold-seq | 37.5ns | 3764.3ns | 1.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1953.1-7851.8 ns)
   1953.1 |##################
   2248.0 |########################################
   2543.0 |
   2837.9 |
   3132.8 |##
   3427.8 |
   3722.7 |#####
   4017.6 |
   4312.6 |
   4607.5 |
   4902.5 |
   5197.4 |
   5492.3 |
   5787.3 |##
   6082.2 |#####
   6377.1 |#####
   6672.1 |##
   6967.0 |#####
   7261.9 |
   7556.9 |
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 2072.4-7204.8 ns)
   2072.4 |########################################
   2329.1 |########################
   2585.7 |
   2842.3 |
   3098.9 |####
   3355.5 |####
   3612.2 |
   3868.8 |
   4125.4 |
   4382.0 |
   4638.6 |
   4895.3 |########
   5151.9 |########
   5408.5 |############
   5665.1 |############
   5921.7 |
   6178.4 |################
   6435.0 |
   6691.6 |####
   6948.2 |
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 2106.7-8051.4 ns)
   2106.7 |##########
   2403.9 |########################################
   2701.1 |
   2998.4 |
   3295.6 |##
   3592.8 |
   3890.1 |
   4187.3 |
   4484.6 |
   4781.8 |##
   5079.0 |#####
   5376.3 |
   5673.5 |##
   5970.7 |##
   6268.0 |#####
   6565.2 |
   6862.4 |
   7159.7 |#####
   7456.9 |
   7754.1 |
  (6 below, 5 above range)

satfold-lanes4-idx (n=40, range 2023.5-7309.6 ns)
   2023.5 |
   2287.8 |########################################
   2552.1 |
   2816.4 |
   3080.7 |
   3345.0 |
   3609.3 |##
   3873.6 |
   4138.0 |
   4402.3 |
   4666.6 |
   4930.9 |
   5195.2 |##########
   5459.5 |##
   5723.8 |#######
   5988.1 |#####
   6252.4 |#####
   6516.7 |
   6781.0 |##
   7045.3 |##
  (6 below, 3 above range)

satfold-lanes64 (n=40, range 1979.0-8220.9 ns)
   1979.0 |##############################
   2291.1 |########################################
   2603.2 |###
   2915.3 |
   3227.4 |###
   3539.5 |
   3851.6 |###
   4163.7 |
   4475.8 |
   4787.8 |
   5099.9 |
   5412.0 |
   5724.1 |
   6036.2 |######
   6348.3 |
   6660.4 |###
   6972.5 |######
   7284.6 |######
   7596.7 |
   7908.8 |###
  (3 below, 3 above range)

satfold-neon (n=40, range 1979.0-7729.0 ns)
   1979.0 |########################################
   2266.5 |####################
   2554.0 |###
   2841.5 |###
   3129.0 |######
   3416.5 |
   3704.0 |
   3991.5 |
   4279.0 |
   4566.5 |
   4854.0 |
   5141.5 |
   5429.0 |
   5716.5 |###
   6004.0 |###
   6291.5 |######
   6579.0 |######
   6866.5 |###
   7154.0 |###
   7441.5 |###
  (5 below, 4 above range)

satfold-neon8 (n=40, range 1982.0-8239.6 ns)
   1982.0 |####################
   2294.9 |########################################
   2607.8 |
   2920.6 |
   3233.5 |##
   3546.4 |
   3859.3 |
   4172.2 |
   4485.0 |
   4797.9 |
   5110.8 |
   5423.7 |########
   5736.6 |
   6049.4 |
   6362.3 |#####
   6675.2 |##
   6988.1 |#####
   7300.9 |##
   7613.8 |##
   7926.7 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 2419.0-8622.1 ns)
   2419.0 |########################################
   2729.2 |#
   3039.3 |
   3349.5 |
   3659.7 |
   3969.8 |
   4280.0 |
   4590.1 |
   4900.3 |
   5210.4 |#
   5520.6 |#
   5830.7 |
   6140.9 |
   6451.0 |###
   6761.2 |
   7071.3 |###
   7381.5 |#
   7691.7 |#
   8001.8 |#
   8312.0 |
  (3 below, 4 above range)

satfold-seq (n=40, range 2030.1-8071.1 ns)
   2030.1 |######
   2332.2 |########################################
   2634.2 |
   2936.3 |
   3238.3 |####
   3540.4 |
   3842.4 |
   4144.5 |
   4446.5 |
   4748.6 |##
   5050.6 |
   5352.7 |
   5654.7 |
   5956.8 |##
   6258.8 |
   6560.8 |##
   6862.9 |##
   7164.9 |####
   7467.0 |##
   7769.0 |
  (6 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=62.1% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes16**: CV=53.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=59.4% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=53.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes64**: CV=66.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-neon**: CV=62.0% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-neon8**: CV=62.9% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-nolaw**: CV=62.8% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-seq**: CV=63.7% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 4.0x (possible bimodal distribution)
