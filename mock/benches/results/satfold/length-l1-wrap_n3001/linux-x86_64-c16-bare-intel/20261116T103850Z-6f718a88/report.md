# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 is fastest but the noisiest (CV 53.3%)

satfold-neon8 wins on median (3.26 us) yet has the highest variance (CV 53.3%), while satfold-neon is the steadiest (CV 32.8%, 3.92 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.72)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (693 ns) is smaller than the fastest variant's own run-to-run std-dev (1.74 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-neon8 is inconsistent: worst-20% is 4.3x its best-20%

satfold-neon8's best 20% of batches run at 1.28 us but its worst 20% at 5.56 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (-42 ns, 1.15%)

satfold-seq differs from baseline satfold-iterfold by -42 ns (1.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 3259.2 ns median (-10.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 3259.2 ns, slowest 3952.2 ns)

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
| satfold-iterfold | 3636ns | 3816ns | 1456ns | 3784ns | 5374ns | base |
| satfold-lanes16 | 3677ns | 4109ns | 1342ns | 3943ns | 5214ns | +1.13% |
| satfold-lanes16-constl | 3625ns | 3916ns | 1346ns | 3739ns | 5561ns | -0.31% |
| satfold-lanes4-idx | 3521ns | 4141ns | 1218ns | 3749ns | 5140ns | -3.17% |
| satfold-lanes64 | 3783ns | 4122ns | 1401ns | 4041ns | 5388ns | +4.03% |
| satfold-neon | 3735ns | 4118ns | 1341ns | 4042ns | 5207ns | +2.72% |
| satfold-neon8 | 3224ns | 3415ns | 1344ns | 2967ns | 5872ns | -11.34% |
| satfold-nolaw | 3528ns | 3798ns | 1388ns | 3614ns | 5410ns | -2.98% |
| satfold-seq | 3566ns | 3740ns | 1376ns | 3758ns | 5182ns | -1.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 3457ns | 1387ns | 5097ns | base | 9.478 |
| satfold-lanes16 | 3499ns | 1281ns | 4928ns | +1.22% | 9.364 |
| satfold-lanes16-constl | 3452ns | 1287ns | 5283ns | -0.14% | 9.492 |
| satfold-lanes4-idx | 3348ns | 1163ns | 4857ns | -3.16% | 9.787 |
| satfold-lanes64 | 3591ns | 1334ns | 5090ns | +3.87% | 9.125 |
| satfold-neon | 3547ns | 1277ns | 4921ns | +2.59% | 9.239 |
| satfold-neon8 | 3065ns | 1283ns | 5555ns | -11.34% | 10.691 |
| satfold-nolaw | 3356ns | 1325ns | 5119ns | -2.92% | 9.763 |
| satfold-seq | 3394ns | 1314ns | 4922ns | -1.82% | 9.654 |

## Performance model

- Peak throughput: **28.186 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 9.006 | 32.0% |
| satfold-lanes16 | 8.341 | 29.6% |
| satfold-lanes16-constl | 8.755 | 31.1% |
| satfold-lanes4-idx | 8.291 | 29.4% |
| satfold-lanes64 | 8.352 | 29.6% |
| satfold-neon | 8.357 | 29.7% |
| satfold-neon8 | 10.054 | 35.7% |
| satfold-nolaw | 9.035 | 32.1% |
| satfold-seq | 9.177 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 3636ns | 3636ns | base |
| satfold-lanes16 | 3677ns | 3677ns | +1.13% |
| satfold-lanes16-constl | 3625ns | 3625ns | -0.31% |
| satfold-lanes4-idx | 3521ns | 3521ns | -3.17% |
| satfold-lanes64 | 3783ns | 3783ns | +4.03% |
| satfold-neon | 3735ns | 3735ns | +2.72% |
| satfold-neon8 | 3224ns | 3224ns | -11.34% |
| satfold-nolaw | 3528ns | 3528ns | -2.98% |
| satfold-seq | 3566ns | 3566ns | -1.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 3638ns | base | --- | [3317, 3965] | --- | --- | --- | --- |
| satfold-lanes16 | 3929ns | +89.3ns (+2.5%) | [+17, +338]ns | [3504, 4082] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| satfold-lanes16-constl | 3743ns | no significant difference | [-158, +344]ns | [3263, 4144] | no | 0.3576 | 0.2682 | 0 |
| satfold-lanes4-idx | 3952ns | no significant difference | [-404, +353]ns | [3513, 4292] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes64 | 3924ns | no significant difference | [-10, +539]ns | [3437, 4253] | no | 0.3077 | 0.1539 | 0 |
| satfold-neon | 3921ns | no significant difference | [-16, +502]ns | [3497, 4254] | no | 0.3576 | 0.2682 | 0 |
| satfold-neon8 | 3259ns | no significant difference | [-1385, +40]ns | [1342, 3971] | no | 0.3077 | 0.1539 | 0 |
| satfold-nolaw | 3627ns | no significant difference | [-684, +222]ns | [3275, 3876] | no | 0.8746 | 0.8746 | 0 |
| satfold-seq | 3571ns | -41.8ns (-1.1%) | [-65, -4]ns | [3292, 3966] | YES (adj: no) | 0.1539 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3572ns | +7.4% | +8.0% | +17.0% | +14.6% | +12.7% | -63.1% | +3.0% | -0.5% |
| 1 | 2 | 4560ns | +1.5% | +4.3% | -18.7% | -1.4% | +1.4% | -5.2% | -12.3% | -26.0% |
| 1 | 3 | 3338ns | +8.9% | +9.7% | +35.6% | +3.0% | +5.5% | +1.5% | +1.6% | +31.3% |
| 1 | 4 | 3538ns | +14.7% | -35.3% | +11.7% | +10.5% | +14.2% | -62.8% | -62.6% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.434 | moderate- |
| satfold-lanes16 | -0.663 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | 0.076 | ok |
| satfold-lanes4-idx | -0.720 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.315 | moderate- |
| satfold-neon | -0.511 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.266 | moderate- |
| satfold-nolaw | 0.061 | ok |
| satfold-seq | -0.530 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 1/4, lost 3/4
- **satfold-lanes4-idx**: won 1/4, lost 3/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 62.0ns | 3457.2ns | 1.8% |  |
| satfold-lanes16 | 67.2ns | 3499.4ns | 1.9% |  |
| satfold-lanes16-constl | 60.8ns | 3452.3ns | 1.8% |  |
| satfold-lanes4-idx | 59.8ns | 3348.1ns | 1.8% |  |
| satfold-lanes64 | 64.0ns | 3591.0ns | 1.8% |  |
| satfold-neon | 63.3ns | 3546.6ns | 1.8% |  |
| satfold-neon8 | 55.0ns | 3065.1ns | 1.8% |  |
| satfold-nolaw | 61.0ns | 3356.3ns | 1.8% |  |
| satfold-seq | 60.7ns | 3394.2ns | 1.8% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 1387.1-5097.4 ns)
   1387.1 |
   1572.6 |
   1758.1 |################
   1943.6 |
   2129.1 |########
   2314.7 |
   2500.2 |
   2685.7 |
   2871.2 |
   3056.7 |########################
   3242.3 |########################################
   3427.8 |########
   3613.3 |########################################
   3798.8 |################
   3984.3 |################
   4169.8 |########
   4355.4 |########
   4540.9 |
   4726.4 |########################################
   4911.9 |################
  (7 below, 3 above range)

satfold-lanes16 (n=40, range 1281.5-4927.6 ns)
   1281.5 |########################################
   1463.8 |
   1646.1 |
   1828.4 |
   2010.7 |
   2193.0 |
   2375.3 |
   2557.6 |
   2740.0 |#####
   2922.3 |
   3104.6 |#####
   3286.9 |#################
   3469.2 |#####
   3651.5 |######################
   3833.8 |###########
   4016.1 |##################################
   4198.4 |#####
   4380.7 |############################
   4563.0 |
   4745.3 |############################
  (2 below, 2 above range)

satfold-lanes16-constl (n=40, range 1286.5-5283.4 ns)
   1286.5 |########################################
   1486.4 |#####
   1686.2 |
   1886.1 |
   2085.9 |
   2285.7 |
   2485.6 |#####
   2685.4 |
   2885.3 |
   3085.1 |##########
   3285.0 |#####
   3484.8 |###############
   3684.7 |####################
   3884.5 |##########
   4084.3 |####################
   4284.2 |#####
   4484.0 |#####
   4683.9 |####################
   4883.7 |
   5083.6 |#####
  (2 below, 5 above range)

satfold-lanes4-idx (n=40, range 1162.6-4856.6 ns)
   1162.6 |############################
   1347.3 |#####
   1532.0 |#####
   1716.7 |
   1901.4 |
   2086.1 |
   2270.8 |
   2455.5 |#####
   2640.2 |
   2824.9 |
   3009.6 |
   3194.3 |
   3379.0 |#####
   3563.7 |#################
   3748.4 |#####
   3933.1 |############################
   4117.8 |######################
   4302.5 |#################
   4487.2 |#####
   4671.9 |########################################
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 1334.2-5089.6 ns)
   1334.2 |########
   1522.0 |
   1709.7 |
   1897.5 |
   2085.3 |
   2273.1 |
   2460.8 |########
   2648.6 |
   2836.4 |
   3024.1 |################
   3211.9 |################
   3399.7 |################
   3587.5 |########################
   3775.2 |################################
   3963.0 |################
   4150.8 |########################################
   4338.5 |########
   4526.3 |########################
   4714.1 |################################
   4901.9 |
  (7 below, 3 above range)

satfold-neon (n=40, range 1276.6-4921.0 ns)
   1276.6 |########################################
   1458.8 |
   1641.1 |
   1823.3 |
   2005.5 |
   2187.7 |#####
   2369.9 |
   2552.2 |
   2734.4 |
   2916.6 |
   3098.8 |###########
   3281.0 |#################
   3463.2 |###########
   3645.5 |#####
   3827.7 |############################
   4009.9 |#################
   4192.1 |######################
   4374.3 |###########
   4556.6 |###########
   4738.8 |######################
  (1 below, 3 above range)

satfold-neon8 (n=40, range 1283.0-5555.1 ns)
   1283.0 |########################################
   1496.6 |
   1710.2 |##
   1923.8 |
   2137.4 |
   2351.0 |
   2564.6 |
   2778.2 |##
   2991.8 |##
   3205.4 |#######
   3419.1 |##
   3632.7 |##
   3846.3 |#####
   4059.9 |
   4273.5 |#####
   4487.1 |#####
   4700.7 |#######
   4914.3 |
   5127.9 |#####
   5341.5 |
  (1 below, 4 above range)

satfold-nolaw (n=40, range 1325.4-5119.2 ns)
   1325.4 |#############
   1515.1 |#############
   1704.8 |
   1894.5 |
   2084.2 |
   2273.9 |
   2463.5 |
   2653.2 |######
   2842.9 |
   3032.6 |####################
   3222.3 |####################
   3412.0 |####################
   3601.7 |########################################
   3791.3 |##########################
   3981.0 |######
   4170.7 |#############
   4360.4 |#############
   4550.1 |
   4739.8 |#############
   4929.5 |
  (6 below, 3 above range)

satfold-seq (n=40, range 1314.2-4922.1 ns)
   1314.2 |########################################
   1494.5 |
   1674.9 |
   1855.3 |######
   2035.7 |
   2216.1 |
   2396.5 |
   2576.9 |######
   2757.3 |######
   2937.7 |
   3118.1 |####################
   3298.5 |#################################
   3478.9 |##########################
   3659.3 |######
   3839.7 |####################
   4020.1 |
   4200.5 |####################
   4380.9 |######
   4561.3 |######
   4741.7 |#################################
  (2 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=37.5% (high variance, measurements may be unstable)
- **satfold-iterfold**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-lanes16**: CV=37.1% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=42.6% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=43.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **satfold-lanes64**: CV=36.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-neon**: CV=36.2% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-neon8**: CV=56.6% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **satfold-nolaw**: CV=40.6% (high variance, measurements may be unstable)
- **satfold-nolaw**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-seq**: CV=37.0% (high variance, measurements may be unstable)
- **satfold-seq**: worst_20/best_20 = 3.7x (possible bimodal distribution)
