# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 13% faster than the next best (warm-container-kernel)

warm-container-minimum (2.16 us) leads warm-container-kernel (2.45 us) by 13%, a clear separation rather than a photo finish. CV 105.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum is fastest but the noisiest (CV 105.0%)

warm-container-minimum wins on median (2.16 us) yet has the highest variance (CV 105.0%), while warm-container-lanes-deferred is the steadiest (CV 31.9%, 4.22 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.77)

warm-container-native's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.06 us) is smaller than the fastest variant's own run-to-run std-dev (2.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-kernel} vs {warm-container-headroom, warm-container-native, warm-container-plusone, warm-container-lanes-deferred} (50% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-native, warm-container-plusone, warm-container-lanes-deferred} with a 50% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-headroom is inconsistent: worst-20% is 5.2x its best-20%

warm-container-headroom's best 20% of batches run at 1.29 us but its worst 20% at 6.66 us (5.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2162.1 ns median (-41.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.95x (fastest 2162.1 ns, slowest 4217.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3582ns | 3877ns | 1356ns | 3176ns | 7028ns | base |
| warm-container-kernel | 3578ns | 2581ns | 1348ns | 3247ns | 6802ns | -0.11% |
| warm-container-lanes-deferred | 4034ns | 4436ns | 1650ns | 4295ns | 5635ns | +12.61% |
| warm-container-minimum | 3687ns | 2272ns | 1473ns | 3242ns | 7235ns | +2.92% |
| warm-container-native | 3966ns | 4310ns | 1537ns | 4191ns | 5719ns | +10.71% |
| warm-container-plusone | 3911ns | 4369ns | 1618ns | 3961ns | 6057ns | +9.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3419ns | 1289ns | 6664ns | base | 4.792 |
| warm-container-kernel | 3387ns | 1281ns | 6402ns | -0.93% | 4.837 |
| warm-container-lanes-deferred | 3827ns | 1569ns | 5314ns | +11.94% | 4.281 |
| warm-container-minimum | 3502ns | 1402ns | 6844ns | +2.44% | 4.678 |
| warm-container-native | 3757ns | 1461ns | 5409ns | +9.89% | 4.361 |
| warm-container-plusone | 3707ns | 1539ns | 5712ns | +8.43% | 4.420 |

## Performance model

- Peak throughput: **12.788 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.445 | 34.8% |
| warm-container-kernel | 6.681 | 52.2% |
| warm-container-lanes-deferred | 3.885 | 30.4% |
| warm-container-minimum | 7.578 | 59.3% |
| warm-container-native | 3.998 | 31.3% |
| warm-container-plusone | 3.942 | 30.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3582ns | 3582ns | base |
| warm-container-kernel | 3578ns | 3578ns | -0.11% |
| warm-container-lanes-deferred | 4034ns | 4034ns | +12.61% |
| warm-container-minimum | 3687ns | 3687ns | +2.92% |
| warm-container-native | 3966ns | 3966ns | +10.71% |
| warm-container-plusone | 3911ns | 3911ns | +9.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3686ns | base | --- | [1574, 4382] | --- | --- | --- | --- |
| warm-container-kernel | 2452ns | no significant difference | [-17, +3]ns | [1563, 4871] | no | 0.2564 | 0.1539 | 0 |
| warm-container-lanes-deferred | 4217ns | +298.4ns (+8.1%) | [+270, +883]ns | [3845, 4574] | YES | 0.0321 | 0.0064 | 0 |
| warm-container-minimum | 2162ns | no significant difference | [-9, +860]ns | [1621, 4701] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 4098ns | no significant difference | [-168, +795]ns | [3736, 4500] | no | 0.8746 | 0.8746 | 0 |
| warm-container-plusone | 4156ns | +314.6ns (+8.5%) | [+0, +818]ns | [2563, 4850] | YES (adj: no) | 0.0962 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1581ns | +78.2% | +150.9% | +32.8% | +197.6% | +105.2% |
| 1 | 2 | 4397ns | +7.1% | +1.3% | -62.1% | -13.7% | -35.4% |
| 1 | 3 | 3990ns | -67.8% | +17.3% | +22.0% | +16.2% | +18.2% |
| 1 | 4 | 1290ns | -0.2% | +222.9% | +7.3% | +207.7% | +221.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.244 | moderate- |
| warm-container-kernel | -0.068 | ok |
| warm-container-lanes-deferred | -0.179 | ok |
| warm-container-minimum | -0.556 | HIGH- (thermal bounce) |
| warm-container-native | -0.767 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.013 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 0/4, lost 4/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 46.0ns | 3419.0ns | 1.3% |  |
| warm-container-kernel | 53.3ns | 3387.3ns | 1.6% |  |
| warm-container-lanes-deferred | 59.8ns | 3827.3ns | 1.6% |  |
| warm-container-minimum | 51.2ns | 3502.4ns | 1.5% |  |
| warm-container-native | 59.0ns | 3757.0ns | 1.6% |  |
| warm-container-plusone | 59.7ns | 3707.2ns | 1.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1289.0-6663.8 ns)
   1289.0 |############################
   1557.8 |############################
   1826.5 |
   2095.3 |
   2364.0 |
   2632.7 |
   2901.5 |
   3170.2 |####
   3439.0 |####
   3707.7 |####
   3976.4 |########
   4245.2 |########################################
   4513.9 |
   4782.7 |####
   5051.4 |####
   5320.1 |
   5588.9 |
   5857.6 |
   6126.4 |####
   6395.1 |####
  (4 below, 3 above range)

warm-container-kernel (n=40, range 1281.2-6401.8 ns)
   1281.2 |########################################
   1537.2 |###################################
   1793.2 |
   2049.3 |
   2305.3 |
   2561.3 |
   2817.3 |
   3073.4 |#####
   3329.4 |
   3585.4 |#####
   3841.5 |#####
   4097.5 |
   4353.5 |#####
   4609.5 |##########
   4865.6 |#####
   5121.6 |###############
   5377.6 |#####
   5633.7 |####################
   5889.7 |
   6145.7 |##########
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1569.3-5313.5 ns)
   1569.3 |################################
   1756.5 |
   1943.7 |
   2130.9 |
   2318.2 |
   2505.4 |
   2692.6 |########
   2879.8 |
   3067.0 |
   3254.2 |
   3441.4 |################
   3628.6 |################
   3815.8 |################
   4003.0 |################################
   4190.3 |################################
   4377.5 |########
   4564.7 |########################
   4751.9 |########################################
   4939.1 |
   5126.3 |################################
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1401.9-6843.8 ns)
   1401.9 |########################################
   1674.0 |##
   1946.1 |
   2218.2 |
   2490.3 |##
   2762.4 |
   3034.4 |
   3306.5 |
   3578.6 |##
   3850.7 |########
   4122.8 |
   4394.9 |##
   4667.0 |#####
   4939.1 |
   5211.2 |##
   5483.3 |########
   5755.4 |#####
   6027.5 |#####
   6299.6 |#####
   6571.7 |
  (5 below, 2 above range)

warm-container-native (n=40, range 1460.7-5409.2 ns)
   1460.7 |########################################
   1658.1 |
   1855.5 |
   2052.9 |
   2250.4 |
   2447.8 |######
   2645.2 |
   2842.7 |######
   3040.1 |
   3237.5 |
   3434.9 |#############
   3632.4 |#############
   3829.8 |####################
   4027.2 |##########################
   4224.6 |####################
   4422.1 |####################
   4619.5 |##########################
   4816.9 |######
   5014.4 |####################
   5211.8 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1538.7-5712.1 ns)
   1538.7 |########################################
   1747.3 |
   1956.0 |
   2164.7 |
   2373.4 |
   2582.0 |
   2790.7 |
   2999.4 |
   3208.1 |
   3416.7 |###
   3625.4 |######
   3834.1 |###
   4042.8 |#########
   4251.4 |######
   4460.1 |###
   4668.8 |######
   4877.5 |#########
   5086.1 |#########
   5294.8 |
   5503.5 |############
  (1 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=65.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **warm-container-kernel**: CV=63.4% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 5.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=35.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=64.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **warm-container-native**: CV=37.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=45.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
