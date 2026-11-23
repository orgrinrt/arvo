# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 14% faster than the next best (warm-container-plusone)

warm-container-minimum (2.36 us) leads warm-container-plusone (2.69 us) by 14%, a clear separation rather than a photo finish. CV 103.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum is fastest but the noisiest (CV 103.9%)

warm-container-minimum wins on median (2.36 us) yet has the highest variance (CV 103.9%), while warm-container-lanes-deferred is the steadiest (CV 54.2%, 4.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.50)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.77 us) is smaller than the fastest variant's own run-to-run std-dev (2.46 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} vs {warm-container-kernel, warm-container-lanes-deferred} (32% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} and a slow tier {warm-container-kernel, warm-container-lanes-deferred} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 5.4x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.38 us but its worst 20% at 7.53 us (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2364.3 ns median (-17.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.75x (fastest 2364.3 ns, slowest 4137.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112763739 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4486ns | 2969ns | 2066ns | 3595ns | 9578ns | base |
| warm-container-kernel | 4958ns | 3868ns | 2839ns | 3789ns | 10584ns | +10.53% |
| warm-container-lanes-deferred | 4369ns | 4333ns | 1457ns | 4186ns | 7830ns | -2.60% |
| warm-container-minimum | 4391ns | 2443ns | 2089ns | 3840ns | 8348ns | -2.11% |
| warm-container-native | 4177ns | 2892ns | 2243ns | 3237ns | 8934ns | -6.87% |
| warm-container-plusone | 3987ns | 2782ns | 2282ns | 2950ns | 8803ns | -11.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4314ns | 1992ns | 9224ns | base | 3.798 |
| warm-container-kernel | 4820ns | 2755ns | 10252ns | +11.75% | 3.399 |
| warm-container-lanes-deferred | 4180ns | 1384ns | 7529ns | -3.10% | 3.919 |
| warm-container-minimum | 4235ns | 2015ns | 8015ns | -1.82% | 3.868 |
| warm-container-native | 4031ns | 2168ns | 8593ns | -6.55% | 4.064 |
| warm-container-plusone | 3842ns | 2204ns | 8450ns | -10.93% | 4.264 |

## Performance model

- Peak throughput: **11.835 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.714 | 48.3% |
| warm-container-kernel | 4.335 | 36.6% |
| warm-container-lanes-deferred | 3.960 | 33.5% |
| warm-container-minimum | 6.930 | 58.6% |
| warm-container-native | 5.849 | 49.4% |
| warm-container-plusone | 6.087 | 51.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4486ns | 4486ns | base |
| warm-container-kernel | 4958ns | 4958ns | +10.53% |
| warm-container-lanes-deferred | 4369ns | 4369ns | -2.60% |
| warm-container-minimum | 4391ns | 4391ns | -2.11% |
| warm-container-native | 4177ns | 4177ns | -6.87% |
| warm-container-plusone | 3987ns | 3987ns | -11.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2867ns | base | --- | [2831, 4230] | --- | --- | --- | --- |
| warm-container-kernel | 3779ns | no significant difference | [-82, +906]ns | [3277, 3973] | no | 0.2564 | 0.1539 | 0 |
| warm-container-lanes-deferred | 4137ns | no significant difference | [-307, +153]ns | [2841, 4814] | no | 0.4296 | 0.4296 | 0 |
| warm-container-minimum | 2364ns | no significant difference | [-787, +396]ns | [2189, 5759] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 2801ns | -224.7ns (-7.8%) | [-374, -3]ns | [2575, 2891] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-container-plusone | 2691ns | -277.8ns (-9.7%) | [-498, -3]ns | [2526, 2865] | YES (adj: no) | 0.0829 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2873ns | +34.9% | +107.9% | +115.4% | +3.1% | -0.1% |
| 1 | 2 | 2867ns | +4.0% | -0.9% | -28.5% | -12.1% | -13.1% |
| 1 | 3 | 2280ns | +40.1% | +91.9% | -1.0% | -3.5% | -2.6% |
| 1 | 4 | 4217ns | -5.0% | +1.7% | -46.4% | -33.6% | -36.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.355 | moderate- |
| warm-container-kernel | -0.236 | moderate- |
| warm-container-lanes-deferred | -0.500 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.124 | ok |
| warm-container-native | -0.202 | moderate- |
| warm-container-plusone | -0.169 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 44.1ns | 4313.8ns | 1.0% |  |
| warm-container-kernel | 33.9ns | 4820.4ns | 0.7% |  |
| warm-container-lanes-deferred | 50.9ns | 4180.2ns | 1.2% |  |
| warm-container-minimum | 38.1ns | 4235.5ns | 0.9% |  |
| warm-container-native | 37.0ns | 4031.2ns | 0.9% |  |
| warm-container-plusone | 35.1ns | 3842.1ns | 0.9% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1991.5-9224.2 ns)
   1991.5 |####################
   2353.2 |######
   2714.8 |########################################
   3076.4 |
   3438.1 |
   3799.7 |#############
   4161.3 |######
   4523.0 |##########
   4884.6 |###
   5246.2 |
   5607.9 |
   5969.5 |
   6331.1 |
   6692.8 |
   7054.4 |
   7416.0 |
   7777.6 |###
   8139.3 |
   8500.9 |######
   8862.5 |
  (2 below, 5 above range)

warm-container-kernel (n=40, range 2754.8-10251.8 ns)
   2754.8 |########################################
   3129.7 |#########################
   3504.5 |###################################
   3879.4 |########################################
   4254.2 |
   4629.1 |#####
   5003.9 |
   5378.8 |
   5753.6 |
   6128.5 |#####
   6503.3 |
   6878.1 |
   7253.0 |
   7627.8 |
   8002.7 |#####
   8377.5 |##########
   8752.4 |
   9127.2 |
   9502.1 |
   9876.9 |
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1384.4-7529.4 ns)
   1384.4 |###############
   1691.6 |#####
   1998.9 |#####
   2306.1 |
   2613.4 |########################################
   2920.6 |
   3227.9 |
   3535.1 |
   3842.4 |###############
   4149.7 |###############
   4456.9 |##########
   4764.2 |###############
   5071.4 |#####
   5378.7 |##########
   5685.9 |##########
   5993.2 |###############
   6300.4 |
   6607.7 |
   6914.9 |
   7222.2 |
  (4 below, 4 above range)

warm-container-minimum (n=40, range 2015.1-8015.1 ns)
   2015.1 |########################################
   2315.1 |##
   2615.1 |
   2915.1 |##
   3215.1 |
   3515.1 |
   3815.1 |##
   4115.1 |
   4415.1 |
   4715.1 |
   5015.1 |##
   5315.1 |##
   5615.1 |#######
   5915.1 |##
   6215.1 |####
   6515.1 |
   6815.1 |##
   7115.1 |####
   7415.1 |####
   7715.1 |
  (3 below, 4 above range)

warm-container-native (n=40, range 2167.8-8592.9 ns)
   2167.8 |####################
   2489.1 |########################################
   2810.3 |################################
   3131.6 |####
   3452.8 |
   3774.1 |
   4095.4 |
   4416.6 |
   4737.9 |
   5059.1 |####
   5380.4 |
   5701.6 |
   6022.9 |####
   6344.1 |########
   6665.4 |
   6986.6 |####
   7307.9 |
   7629.1 |####
   7950.4 |
   8271.7 |
  (5 below, 5 above range)

warm-container-plusone (n=40, range 2204.2-8450.2 ns)
   2204.2 |########################################
   2516.5 |################################
   2828.8 |################################
   3141.1 |
   3453.4 |
   3765.7 |####
   4078.0 |
   4390.3 |
   4702.6 |
   5014.9 |
   5327.2 |####
   5639.5 |
   5951.8 |########
   6264.1 |
   6576.4 |
   6888.7 |
   7201.0 |
   7513.3 |
   7825.6 |
   8137.9 |
  (4 below, 6 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-kernel**: CV=62.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=53.6% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=58.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=62.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-plusone**: CV=63.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
