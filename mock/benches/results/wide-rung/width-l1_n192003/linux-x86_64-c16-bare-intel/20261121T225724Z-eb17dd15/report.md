# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (4.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 3.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread dominates: 20% faster than the next best (wide-rung-wordround-alias)

wide-rung-ragged-overread (3.34 us) leads wide-rung-wordround-alias (4.02 us) by 20%, a clear separation rather than a photo finish. CV 42.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.52)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (764 ns) is smaller than the fastest variant's own run-to-run std-dev (1.43 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-wordround is inconsistent: worst-20% is 2.5x its best-20%

wide-rung-wordround's best 20% of batches run at 3.29 us but its worst 20% at 8.35 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-ragged's edge over baseline is significant but tiny (-33 ns, 0.81%)

wide-rung-ragged differs from baseline wide-rung-align16 by -33 ns (0.81%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 3340.8 ns median (-18.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.23x (fastest 3340.8 ns, slowest 4105.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112636733 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4501ns | 4186ns | 3429ns | 4126ns | 6700ns | base |
| wide-rung-ragged | 4516ns | 4160ns | 3356ns | 4159ns | 6746ns | +0.32% |
| wide-rung-ragged-overread | 3992ns | 3410ns | 3396ns | 3533ns | 5966ns | -11.31% |
| wide-rung-wordround | 4710ns | 4156ns | 3354ns | 3875ns | 8569ns | +4.63% |
| wide-rung-wordround-alias | 4635ns | 4096ns | 3383ns | 3854ns | 8228ns | +2.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4404ns | 3363ns | 6521ns | base | 1.860 |
| wide-rung-ragged | 4417ns | 3288ns | 6564ns | +0.29% | 1.855 |
| wide-rung-ragged-overread | 3907ns | 3330ns | 5815ns | -11.30% | 2.097 |
| wide-rung-wordround | 4608ns | 3288ns | 8354ns | +4.64% | 1.778 |
| wide-rung-wordround-alias | 4535ns | 3317ns | 8022ns | +2.98% | 1.806 |

## Performance model

- Peak throughput: **2.492 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.996 | 80.1% |
| wide-rung-ragged | 2.008 | 80.6% |
| wide-rung-ragged-overread | 2.452 | 98.4% |
| wide-rung-wordround | 2.010 | 80.7% |
| wide-rung-wordround-alias | 2.040 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4501ns | 4501ns | base |
| wide-rung-ragged | 4516ns | 4516ns | +0.32% |
| wide-rung-ragged-overread | 3992ns | 3992ns | -11.31% |
| wide-rung-wordround | 4710ns | 4710ns | +4.63% |
| wide-rung-wordround-alias | 4635ns | 4635ns | +2.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 4105ns | base | --- | [4016, 4107] | --- | --- | --- | --- |
| wide-rung-ragged | 4079ns | -33.2ns (-0.8%) | [-48, -19]ns | [4073, 4083] | YES | 0.0129 | 0.0064 | 0 |
| wide-rung-ragged-overread | 3341ns | -680.7ns (-16.6%) | [-688, -33]ns | [3338, 3349] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 4075ns | no significant difference | [-30, +52]ns | [3334, 4081] | no | 0.1539 | 0.1539 | 0 |
| wide-rung-wordround-alias | 4016ns | -144.5ns (-3.5%) | [-709, -15]ns | [3351, 4028] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 4016ns | -18.1% | -17.1% | +1.6% | -16.8% |
| 1 | 2 | 4119ns | -1.1% | -19.0% | -0.8% | -18.9% |
| 1 | 3 | 3364ns | +21.2% | -0.8% | -1.0% | +19.5% |
| 1 | 4 | 4111ns | -0.7% | -0.8% | -20.0% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.520 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.079 | ok |
| wide-rung-ragged-overread | -0.079 | ok |
| wide-rung-wordround | 0.260 | moderate+ |
| wide-rung-wordround-alias | 0.270 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 24.9ns | 4404.0ns | 0.6% |  |
| wide-rung-ragged | 25.9ns | 4417.0ns | 0.6% |  |
| wide-rung-ragged-overread | 23.0ns | 3906.5ns | 0.6% |  |
| wide-rung-wordround | 26.4ns | 4608.2ns | 0.6% |  |
| wide-rung-wordround-alias | 26.3ns | 4535.2ns | 0.6% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3362.8-6521.1 ns)
   3362.8 |######
   3520.8 |
   3678.7 |
   3836.6 |
   3994.5 |########################################
   4152.4 |
   4310.3 |
   4468.3 |
   4626.2 |
   4784.1 |
   4942.0 |
   5099.9 |#
   5257.8 |
   5415.7 |
   5573.7 |
   5731.6 |
   5889.5 |###
   6047.4 |
   6205.3 |
   6363.2 |
  (5 below, 2 above range)

wide-rung-ragged (n=40, range 3288.4-6563.9 ns)
   3288.4 |####
   3452.1 |
   3615.9 |
   3779.7 |
   3943.5 |########################################
   4107.2 |#
   4271.0 |
   4434.8 |
   4598.6 |
   4762.3 |
   4926.1 |
   5089.9 |#
   5253.7 |
   5417.4 |
   5581.2 |
   5745.0 |
   5908.8 |
   6072.5 |
   6236.3 |
   6400.1 |#
  (5 below, 3 above range)

wide-rung-ragged-overread (n=40, range 3330.2-5814.6 ns)
   3330.2 |########################################
   3454.4 |
   3578.7 |
   3702.9 |
   3827.1 |
   3951.3 |#########
   4075.5 |#######
   4199.8 |
   4324.0 |
   4448.2 |
   4572.4 |
   4696.6 |
   4820.8 |
   4945.1 |
   5069.3 |
   5193.5 |
   5317.7 |
   5441.9 |
   5566.2 |
   5690.4 |
  (6 below, 3 above range)

wide-rung-wordround (n=40, range 3287.8-8354.2 ns)
   3287.8 |##############################
   3541.2 |
   3794.5 |
   4047.8 |########################################
   4301.1 |
   4554.4 |
   4807.7 |
   5061.1 |
   5314.4 |##
   5567.7 |
   5821.0 |##
   6074.3 |
   6327.7 |
   6581.0 |
   6834.3 |
   7087.6 |
   7340.9 |
   7594.2 |
   7847.6 |
   8100.9 |
  (5 below, 5 above range)

wide-rung-wordround-alias (n=40, range 3316.7-8022.1 ns)
   3316.7 |########################################
   3552.0 |##
   3787.2 |##############
   4022.5 |###############################
   4257.8 |
   4493.0 |
   4728.3 |
   4963.6 |##
   5198.8 |
   5434.1 |
   5669.4 |
   5904.6 |
   6139.9 |
   6375.2 |
   6610.4 |
   6845.7 |
   7081.0 |
   7316.2 |
   7551.5 |
   7786.8 |
  (3 below, 5 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=38.6% (high variance, measurements may be unstable)
- **wide-rung-ragged**: CV=34.9% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: CV=36.6% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=49.4% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=48.8% (high variance, measurements may be unstable)
