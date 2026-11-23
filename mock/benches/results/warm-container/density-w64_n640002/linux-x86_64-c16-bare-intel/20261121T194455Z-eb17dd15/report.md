# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 52% (significant)

warm-container-minimum is -1.66 us (52%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is fastest but the noisiest (CV 85.3%)

warm-container-minimum wins on median (2.23 us) yet has the highest variance (CV 85.3%), while warm-container-native is the steadiest (CV 47.5%, 3.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-minimum, warm-container-kernel) are a dead heat (<1%)

warm-container-minimum (2.23 us) and warm-container-kernel (2.24 us) differ by 0.10%, inside the noise, even though the wider field spreads 72.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.72)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.61 us) is smaller than the fastest variant's own run-to-run std-dev (1.91 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-kernel} vs {warm-container-headroom, warm-container-plusone, warm-container-native} (44% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-native} with a 44% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 4.5x its best-20%

warm-container-minimum's best 20% of batches run at 1.26 us but its worst 20% at 5.71 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2234.1 ns median (-30.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.72x (fastest 2234.1 ns, slowest 3849.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 64 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112747662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4690ns | 3282ns | 3269ns | 3578ns | 9446ns | base |
| warm-container-kernel | 3430ns | 2355ns | 1485ns | 3156ns | 6196ns | -26.86% |
| warm-container-minimum | 3377ns | 2354ns | 1331ns | 3164ns | 6060ns | -28.00% |
| warm-container-native | 3657ns | 4060ns | 1430ns | 3599ns | 6061ns | -22.01% |
| warm-container-plusone | 4299ns | 3422ns | 2802ns | 3518ns | 8140ns | -8.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4584ns | 3203ns | 9203ns | base | 5.361 |
| warm-container-kernel | 3248ns | 1410ns | 5839ns | -29.14% | 7.566 |
| warm-container-minimum | 3195ns | 1263ns | 5708ns | -30.30% | 7.692 |
| warm-container-native | 3461ns | 1356ns | 5716ns | -24.51% | 7.102 |
| warm-container-plusone | 4194ns | 2735ns | 7911ns | -8.50% | 5.859 |

## Performance model

- Peak throughput: **19.451 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.643 | 39.3% |
| warm-container-kernel | 10.990 | 56.5% |
| warm-container-minimum | 11.001 | 56.6% |
| warm-container-native | 6.385 | 32.8% |
| warm-container-plusone | 7.356 | 37.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4690ns | 4690ns | base |
| warm-container-kernel | 3430ns | 3430ns | -26.86% |
| warm-container-minimum | 3377ns | 3377ns | -28.00% |
| warm-container-native | 3657ns | 3657ns | -22.01% |
| warm-container-plusone | 4299ns | 4299ns | -8.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3216ns | base | --- | [3206, 3916] | --- | --- | --- | --- |
| warm-container-kernel | 2236ns | -1658.9ns (-51.6%) | [-1946, -1579]ns | [1549, 4588] | YES | 0.0129 | 0.0064 | 0 |
| warm-container-minimum | 2234ns | -1659.6ns (-51.6%) | [-1940, -1221]ns | [1545, 4750] | YES | 0.0089 | 0.0022 | 0 |
| warm-container-native | 3849ns | -1659.2ns (-51.6%) | [-1867, -506]ns | [1587, 4827] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| warm-container-plusone | 3341ns | no significant difference | [-593, +139]ns | [3210, 3916] | no | 1.0000 | 1.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3208ns | -4.2% | +3.7% | +43.5% | +4.2% |
| 1 | 2 | 3204ns | +29.5% | -40.6% | +10.6% | +22.4% |
| 1 | 3 | 3217ns | +36.7% | +29.0% | -35.6% | -0.2% |
| 1 | 4 | 3922ns | -60.5% | -67.7% | -6.1% | -30.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.070 | ok |
| warm-container-kernel | -0.231 | moderate- |
| warm-container-minimum | -0.717 | HIGH- (thermal bounce) |
| warm-container-native | -0.094 | ok |
| warm-container-plusone | 0.028 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.5ns | 4584.2ns | 0.6% |  |
| warm-container-kernel | 54.2ns | 3248.4ns | 1.7% |  |
| warm-container-minimum | 53.5ns | 3195.0ns | 1.7% |  |
| warm-container-native | 58.9ns | 3460.6ns | 1.7% |  |
| warm-container-plusone | 29.2ns | 4194.3ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3203.0-9202.6 ns)
   3203.0 |########################################
   3503.0 |
   3802.9 |################
   4102.9 |
   4402.9 |##
   4702.9 |
   5002.9 |
   5302.8 |
   5602.8 |
   5902.8 |
   6202.8 |
   6502.8 |####
   6802.7 |
   7102.7 |
   7402.7 |##
   7702.7 |
   8002.7 |
   8302.6 |
   8602.6 |
   8902.6 |
  (3 below, 5 above range)

warm-container-kernel (n=40, range 1410.2-5838.9 ns)
   1410.2 |########################################
   1631.7 |
   1853.1 |
   2074.5 |
   2296.0 |
   2517.4 |
   2738.8 |##
   2960.3 |
   3181.7 |
   3403.1 |
   3624.6 |
   3846.0 |##
   4067.4 |##
   4288.9 |#####
   4510.3 |############
   4731.8 |
   4953.2 |#####
   5174.6 |#######
   5396.1 |#####
   5617.5 |##
  (4 below, 2 above range)

warm-container-minimum (n=40, range 1263.5-5707.7 ns)
   1263.5 |##############################
   1485.7 |########################################
   1707.9 |
   1930.1 |#####
   2152.3 |
   2374.5 |#####
   2596.8 |
   2819.0 |#####
   3041.2 |
   3263.4 |
   3485.6 |#####
   3707.8 |
   3930.0 |
   4152.2 |
   4374.4 |
   4596.7 |####################
   4818.9 |
   5041.1 |###############
   5263.3 |###############
   5485.5 |#########################
  (5 below, 2 above range)

warm-container-native (n=40, range 1356.3-5715.9 ns)
   1356.3 |########################################
   1574.3 |#############
   1792.3 |
   2010.3 |####
   2228.3 |
   2446.2 |####
   2664.2 |####
   2882.2 |
   3100.2 |
   3318.1 |####
   3536.1 |
   3754.1 |
   3972.1 |
   4190.0 |########
   4408.0 |#############
   4626.0 |####
   4844.0 |########
   5061.9 |########
   5279.9 |####
   5497.9 |##########################
  (4 below, 3 above range)

warm-container-plusone (n=40, range 2734.6-7911.3 ns)
   2734.6 |########
   2993.5 |###################################
   3252.3 |###################################
   3511.1 |
   3770.0 |########################################
   4028.8 |
   4287.6 |
   4546.5 |####
   4805.3 |
   5064.1 |
   5323.0 |
   5581.8 |
   5840.6 |
   6099.4 |
   6358.3 |
   6617.1 |
   6875.9 |
   7134.8 |
   7393.6 |####
   7652.4 |
  (7 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=54.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=57.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=59.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-native**: CV=52.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=56.5% (high variance, measurements may be unstable)
