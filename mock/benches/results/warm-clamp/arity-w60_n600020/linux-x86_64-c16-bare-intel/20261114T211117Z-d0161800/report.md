# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 12% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (2.47 us) leads warm-clamp-accfit (2.78 us) by 12%, a clear separation rather than a photo finish. CV 113.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is fastest but the noisiest (CV 113.9%)

warm-clamp-min-lanes wins on median (2.47 us) yet has the highest variance (CV 113.9%), while warm-clamp-accfit is the steadiest (CV 31.7%, 2.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.54)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (954 ns) is smaller than the fastest variant's own run-to-run std-dev (2.82 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.4x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.08 us but its worst 20% at 4.82 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2473.2 ns median (-19.8% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.39x (fastest 2473.2 ns, slowest 3427.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3149ns | 3304ns | 1162ns | 3121ns | 5218ns | base |
| warm-clamp-accfit | 2998ns | 3020ns | 1614ns | 3036ns | 4266ns | -4.80% |
| warm-clamp-accfit-dyn | 3236ns | 3241ns | 1390ns | 3123ns | 5421ns | +2.76% |
| warm-clamp-head | 4049ns | 3166ns | 2775ns | 3165ns | 7973ns | +28.59% |
| warm-clamp-min-lanes | 4671ns | 2571ns | 2211ns | 3962ns | 9254ns | +48.33% |
| warm-clamp-minimum | 3631ns | 3674ns | 1967ns | 3624ns | 5315ns | +15.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2929ns | 1085ns | 4815ns | base | 2.796 |
| warm-clamp-accfit | 2771ns | 1501ns | 3929ns | -5.41% | 2.956 |
| warm-clamp-accfit-dyn | 2997ns | 1293ns | 5014ns | +2.31% | 2.733 |
| warm-clamp-head | 3921ns | 2688ns | 7677ns | +33.84% | 2.089 |
| warm-clamp-min-lanes | 4487ns | 2129ns | 8856ns | +53.16% | 1.826 |
| warm-clamp-minimum | 3371ns | 1834ns | 4900ns | +15.09% | 2.430 |

## Performance model

- Peak throughput: **7.551 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.655 | 35.2% |
| warm-clamp-accfit | 2.947 | 39.0% |
| warm-clamp-accfit-dyn | 2.723 | 36.1% |
| warm-clamp-head | 2.660 | 35.2% |
| warm-clamp-min-lanes | 3.312 | 43.9% |
| warm-clamp-minimum | 2.390 | 31.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3149ns | 3149ns | base |
| warm-clamp-accfit | 2998ns | 2998ns | -4.80% |
| warm-clamp-accfit-dyn | 3236ns | 3236ns | +2.76% |
| warm-clamp-head | 4049ns | 4049ns | +28.59% |
| warm-clamp-min-lanes | 4671ns | 4671ns | +48.33% |
| warm-clamp-minimum | 3631ns | 3631ns | +15.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3086ns | base | --- | [2355, 3758] | --- | --- | --- | --- |
| warm-clamp-accfit | 2780ns | -296.6ns (-9.6%) | [-593, -105]ns | [2595, 2987] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 3008ns | -218.9ns (-7.1%) | [-664, -107]ns | [2700, 3276] | YES | 0.0276 | 0.0166 | 0 |
| warm-clamp-head | 3079ns | +477.3ns (+15.5%) | [+11, +1876]ns | [3075, 3084] | YES | 0.0385 | 0.0385 | 0 |
| warm-clamp-min-lanes | 2473ns | +1268.4ns (+41.1%) | [+820, +1984]ns | [2465, 5231] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-minimum | 3428ns | +634.2ns (+20.6%) | [+216, +729]ns | [3053, 3842] | YES | 0.0161 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3954ns | -29.7% | -17.2% | -34.5% | +27.9% | -7.1% |
| 1 | 2 | 1202ns | +37.4% | +95.8% | +155.6% | +68.3% | +60.1% |
| 1 | 3 | 3086ns | -17.1% | -37.4% | -0.1% | -19.9% | +8.1% |
| 1 | 4 | 3086ns | +1.3% | -5.0% | -0.1% | -20.1% | +28.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.539 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.191 | ok |
| warm-clamp-accfit-dyn | -0.190 | ok |
| warm-clamp-head | -0.066 | ok |
| warm-clamp-min-lanes | -0.210 | moderate- |
| warm-clamp-minimum | -0.266 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 60.4ns | 2929.4ns | 2.1% |  |
| warm-clamp-accfit | 63.9ns | 2771.0ns | 2.3% |  |
| warm-clamp-accfit-dyn | 67.1ns | 2997.1ns | 2.2% |  |
| warm-clamp-head | 35.2ns | 3920.8ns | 0.9% |  |
| warm-clamp-min-lanes | 47.3ns | 4486.6ns | 1.1% |  |
| warm-clamp-minimum | 72.5ns | 3371.3ns | 2.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1084.9-4815.0 ns)
   1084.9 |########################################
   1271.4 |#####
   1457.9 |
   1644.4 |
   1830.9 |#####
   2017.4 |
   2203.9 |
   2390.4 |
   2576.9 |#####
   2763.4 |#####
   2949.9 |##############################
   3136.5 |###############
   3323.0 |
   3509.5 |#####
   3696.0 |
   3882.5 |###############
   4069.0 |#####
   4255.5 |##############################
   4442.0 |
   4628.5 |
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 1501.2-3929.2 ns)
   1501.2 |
   1622.6 |
   1744.0 |
   1865.4 |
   1986.8 |
   2108.2 |
   2229.6 |########
   2351.0 |################################
   2472.4 |################################
   2593.8 |########################
   2715.2 |########################################
   2836.6 |########################
   2958.0 |########################
   3079.4 |################
   3200.8 |################
   3322.2 |################
   3443.6 |########
   3565.0 |########
   3686.4 |
   3807.8 |
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1293.5-5014.0 ns)
   1293.5 |
   1479.5 |#####
   1665.5 |
   1851.5 |########################################
   2037.6 |
   2223.6 |
   2409.6 |
   2595.7 |###########
   2781.7 |######################
   2967.7 |######################
   3153.7 |############################
   3339.8 |#####
   3525.8 |###########
   3711.8 |###########
   3897.8 |
   4083.9 |#####
   4269.9 |#####
   4455.9 |
   4641.9 |#####
   4828.0 |#####
  (5 below, 3 above range)

warm-clamp-head (n=40, range 2688.5-7677.1 ns)
   2688.5 |
   2937.9 |########################################
   3187.3 |
   3436.8 |
   3686.2 |
   3935.6 |
   4185.1 |
   4434.5 |
   4683.9 |
   4933.4 |
   5182.8 |
   5432.2 |
   5681.7 |
   5931.1 |#
   6180.5 |
   6430.0 |
   6679.4 |#
   6928.8 |
   7178.3 |
   7427.7 |
  (6 below, 5 above range)

warm-clamp-min-lanes (n=40, range 2128.9-8856.4 ns)
   2128.9 |###################################
   2465.2 |########################################
   2801.6 |########
   3138.0 |
   3474.4 |
   3810.7 |
   4147.1 |
   4483.5 |####
   4819.9 |
   5156.2 |
   5492.6 |####
   5829.0 |
   6165.4 |
   6501.7 |
   6838.1 |####
   7174.5 |####
   7510.9 |####
   7847.2 |#############
   8183.6 |########
   8520.0 |####
  (6 below, 4 above range)

warm-clamp-minimum (n=40, range 1833.8-4900.4 ns)
   1833.8 |########################################
   1987.1 |#####
   2140.4 |
   2293.8 |
   2447.1 |#####
   2600.4 |
   2753.7 |#################
   2907.1 |#####
   3060.4 |######################
   3213.7 |###########
   3367.1 |#####
   3520.4 |######################
   3673.7 |
   3827.1 |######################
   3980.4 |###########
   4133.7 |#####
   4287.0 |############################
   4440.4 |#####
   4593.7 |
   4747.0 |
  (1 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=49.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=31.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=44.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=54.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=62.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=33.6% (high variance, measurements may be unstable)
