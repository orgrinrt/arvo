# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 30% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (2.34 us) leads warm-clamp-min-lanes (3.03 us) by 30%, a clear separation rather than a photo finish. CV 93.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum is fastest but the noisiest (CV 93.5%)

warm-clamp-minimum wins on median (2.34 us) yet has the highest variance (CV 93.5%), while warm-clamp-acc64 is the steadiest (CV 26.5%, 3.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.68)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.93 us) is smaller than the fastest variant's own run-to-run std-dev (2.19 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.50 us but its worst 20% at 6.01 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2338.4 ns median (-37.7% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.82x (fastest 2338.4 ns, slowest 4266.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112729874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3580ns | 3927ns | 1698ns | 3880ns | 4558ns | base |
| warm-clamp-accfit | 3778ns | 4437ns | 1545ns | 3747ns | 6106ns | +5.56% |
| warm-clamp-accfit-dyn | 3726ns | 3849ns | 1566ns | 3588ns | 6297ns | +4.08% |
| warm-clamp-head | 3744ns | 4119ns | 1461ns | 4079ns | 5021ns | +4.58% |
| warm-clamp-min-lanes | 4114ns | 3112ns | 1959ns | 3824ns | 7137ns | +14.93% |
| warm-clamp-minimum | 3917ns | 2404ns | 1960ns | 3441ns | 7305ns | +9.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3411ns | 1623ns | 4304ns | base | 2.402 |
| warm-clamp-accfit | 3621ns | 1484ns | 5839ns | +6.16% | 2.263 |
| warm-clamp-accfit-dyn | 3570ns | 1504ns | 6011ns | +4.66% | 2.295 |
| warm-clamp-head | 3545ns | 1387ns | 4728ns | +3.95% | 2.311 |
| warm-clamp-min-lanes | 3995ns | 1908ns | 6897ns | +17.12% | 2.051 |
| warm-clamp-minimum | 3803ns | 1908ns | 7061ns | +11.51% | 2.154 |

## Performance model

- Peak throughput: **5.905 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.183 | 37.0% |
| warm-clamp-accfit | 1.920 | 32.5% |
| warm-clamp-accfit-dyn | 2.214 | 37.5% |
| warm-clamp-head | 2.093 | 35.4% |
| warm-clamp-min-lanes | 2.703 | 45.8% |
| warm-clamp-minimum | 3.503 | 59.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3580ns | 3580ns | base |
| warm-clamp-accfit | 3778ns | 3778ns | +5.56% |
| warm-clamp-accfit-dyn | 3726ns | 3726ns | +4.08% |
| warm-clamp-head | 3744ns | 3744ns | +4.58% |
| warm-clamp-min-lanes | 4114ns | 4114ns | +14.93% |
| warm-clamp-minimum | 3917ns | 3917ns | +9.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3752ns | base | --- | [3497, 3914] | --- | --- | --- | --- |
| warm-clamp-accfit | 4266ns | +607.9ns (+16.2%) | [+362, +742]ns | [1709, 4668] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 3700ns | no significant difference | [-712, +994]ns | [1666, 4993] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-head | 3914ns | +131.1ns (+3.5%) | [+80, +228]ns | [3606, 4281] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-min-lanes | 3030ns | no significant difference | [-456, +1864]ns | [1908, 5815] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-minimum | 2338ns | no significant difference | [-1275, +1537]ns | [1920, 5578] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3766ns | +15.3% | -10.2% | +3.9% | -15.5% | -37.9% |
| 1 | 2 | 3764ns | -17.1% | +6.9% | -3.2% | -23.1% | -42.1% |
| 1 | 3 | 3799ns | -23.2% | -56.1% | +14.2% | -49.6% | -24.0% |
| 1 | 4 | 3498ns | +22.3% | +43.4% | +6.1% | +61.1% | -26.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.181 | ok |
| warm-clamp-accfit | -0.249 | moderate- |
| warm-clamp-accfit-dyn | -0.634 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.678 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.326 | moderate- |
| warm-clamp-minimum | -0.145 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 59.7ns | 3410.8ns | 1.8% |  |
| warm-clamp-accfit | 51.7ns | 3620.8ns | 1.4% |  |
| warm-clamp-accfit-dyn | 51.2ns | 3569.8ns | 1.4% |  |
| warm-clamp-head | 71.2ns | 3545.3ns | 2.0% |  |
| warm-clamp-min-lanes | 40.1ns | 3994.7ns | 1.0% |  |
| warm-clamp-minimum | 37.7ns | 3803.2ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1623.2-4304.1 ns)
   1623.2 |
   1757.2 |
   1891.3 |
   2025.3 |
   2159.4 |#####
   2293.4 |
   2427.5 |#####
   2561.5 |
   2695.6 |#####
   2829.6 |
   2963.7 |
   3097.7 |#####
   3231.8 |###########
   3365.8 |#################
   3499.9 |#################
   3633.9 |######################
   3767.9 |############################
   3902.0 |###########
   4036.0 |########################################
   4170.1 |#####
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 1484.4-5838.5 ns)
   1484.4 |########################################
   1702.1 |##########
   1919.8 |
   2137.5 |
   2355.2 |
   2572.9 |#####
   2790.6 |#####
   3008.3 |
   3226.0 |
   3443.7 |
   3661.4 |
   3879.2 |
   4096.9 |####################
   4314.6 |###############
   4532.3 |###############
   4750.0 |##########
   4967.7 |###############
   5185.4 |
   5403.1 |###############
   5620.8 |#####
  (5 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1504.3-6010.9 ns)
   1504.3 |########################################
   1729.6 |
   1954.9 |###
   2180.3 |
   2405.6 |
   2630.9 |
   2856.3 |###
   3081.6 |###
   3306.9 |
   3532.3 |
   3757.6 |
   3982.9 |###
   4208.3 |
   4433.6 |###
   4658.9 |#########
   4884.3 |############
   5109.6 |###
   5334.9 |######
   5560.2 |######
   5785.6 |###
  (4 below, 5 above range)

warm-clamp-head (n=40, range 1387.2-4727.9 ns)
   1387.2 |
   1554.2 |
   1721.3 |
   1888.3 |
   2055.3 |
   2222.4 |
   2389.4 |
   2556.4 |
   2723.5 |######
   2890.5 |
   3057.5 |###
   3224.6 |###
   3391.6 |
   3558.7 |################
   3725.7 |######
   3892.7 |################
   4059.8 |###
   4226.8 |########################################
   4393.8 |
   4560.9 |###
  (7 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1908.0-6896.9 ns)
   1908.0 |########################################
   2157.5 |##
   2406.9 |
   2656.4 |
   2905.8 |
   3155.2 |
   3404.7 |
   3654.1 |##
   3903.6 |##
   4153.0 |
   4402.4 |##
   4651.9 |
   4901.3 |
   5150.8 |
   5400.2 |#####
   5649.7 |##
   5899.1 |#######
   6148.5 |#######
   6398.0 |#####
   6647.4 |#####
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 1908.0-7060.7 ns)
   1908.0 |########################################
   2165.7 |#######################
   2423.3 |###
   2680.9 |
   2938.6 |
   3196.2 |###
   3453.8 |
   3711.5 |###
   3969.1 |
   4226.8 |
   4484.4 |
   4742.0 |
   4999.7 |
   5257.3 |###
   5514.9 |
   5772.6 |#############
   6030.2 |######
   6287.8 |##########
   6545.5 |##########
   6803.1 |
  (3 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=29.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=47.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=52.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-head**: CV=34.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=54.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=57.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
