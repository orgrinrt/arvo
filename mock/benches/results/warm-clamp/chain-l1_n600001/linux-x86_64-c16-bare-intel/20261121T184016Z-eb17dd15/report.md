# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-acc64, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-acc64 (340 ns) and warm-clamp-accfit (343 ns) differ by 0.93%, inside the noise, even though the wider field spreads 12.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.64)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (340 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader warm-clamp-acc64 vs stability leader warm-clamp-accfit-dyn (+5% speed for 1.1x steadier)

warm-clamp-acc64 is fastest (340 ns, CV 11.1%); warm-clamp-accfit-dyn gives up 5.1% median for 1.1x lower variance (CV 9.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-minimum is inconsistent: worst-20% is 1.6x its best-20%

warm-clamp-minimum's best 20% of batches run at 305 ns but its worst 20% at 489 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-accfit-dyn's edge over baseline is significant but tiny (-3 ns, 1.00%)

warm-clamp-accfit-dyn differs from baseline warm-clamp-acc64 by -3 ns (1.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 340.2 ns median
- 1 variant significantly faster than baseline
- Spread: 1.12x (fastest 340.2 ns, slowest 381.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112832026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 642ns | 602ns | 568ns | 625ns | 769ns | base |
| warm-clamp-accfit | 649ns | 619ns | 534ns | 640ns | 792ns | +1.05% |
| warm-clamp-accfit-dyn | 634ns | 657ns | 528ns | 632ns | 748ns | -1.22% |
| warm-clamp-head | 699ns | 665ns | 589ns | 665ns | 912ns | +8.86% |
| warm-clamp-min-lanes | 624ns | 663ns | 483ns | 614ns | 796ns | -2.81% |
| warm-clamp-minimum | 674ns | 663ns | 530ns | 654ns | 880ns | +5.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 364ns | 327ns | 421ns | base | 22.489 |
| warm-clamp-accfit | 366ns | 306ns | 438ns | +0.50% | 22.377 |
| warm-clamp-accfit-dyn | 350ns | 302ns | 388ns | -3.84% | 23.388 |
| warm-clamp-head | 399ns | 338ns | 535ns | +9.44% | 20.550 |
| warm-clamp-min-lanes | 347ns | 276ns | 424ns | -4.67% | 23.591 |
| warm-clamp-minimum | 381ns | 305ns | 489ns | +4.53% | 21.514 |

## Performance model

- Peak throughput: **29.672 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 24.076 | 81.1% |
| warm-clamp-accfit | 23.856 | 80.4% |
| warm-clamp-accfit-dyn | 22.918 | 77.2% |
| warm-clamp-head | 21.498 | 72.5% |
| warm-clamp-min-lanes | 21.980 | 74.1% |
| warm-clamp-minimum | 22.733 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 642ns | 642ns | base |
| warm-clamp-accfit | 649ns | 649ns | +1.05% |
| warm-clamp-accfit-dyn | 634ns | 634ns | -1.22% |
| warm-clamp-head | 699ns | 699ns | +8.86% |
| warm-clamp-min-lanes | 624ns | 624ns | -2.81% |
| warm-clamp-minimum | 674ns | 674ns | +5.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 340ns | base | --- | [339, 381] | --- | --- | --- | --- |
| warm-clamp-accfit | 343ns | no significant difference | [-33, +3]ns | [341, 384] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 357ns | -3.4ns (-1.0%) | [-29, -3]ns | [336, 378] | YES | 0.0111 | 0.0022 | 0 |
| warm-clamp-head | 381ns | no significant difference | [-0, +35]ns | [340, 382] | no | 0.8746 | 0.4296 | 0 |
| warm-clamp-min-lanes | 373ns | no significant difference | [-63, +41]ns | [304, 380] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-minimum | 360ns | no significant difference | [-16, +42]ns | [339, 395] | no | 0.8746 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 382ns | -10.0% | -1.0% | -11.2% | -20.4% | +14.1% |
| 1 | 2 | 338ns | -9.4% | +11.7% | +0.2% | +12.1% | -9.8% |
| 1 | 3 | 339ns | +29.0% | -0.9% | +36.0% | -18.6% | +12.7% |
| 1 | 4 | 372ns | -8.3% | -18.7% | +2.5% | +15.8% | -8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.246 | moderate- |
| warm-clamp-accfit | -0.491 | moderate- |
| warm-clamp-accfit-dyn | 0.267 | moderate+ |
| warm-clamp-head | -0.153 | ok |
| warm-clamp-min-lanes | -0.642 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.598 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 100.4ns | 364.3ns | 27.6% | HIGH |
| warm-clamp-accfit | 101.8ns | 366.1ns | 27.8% | HIGH |
| warm-clamp-accfit-dyn | 98.7ns | 350.3ns | 28.2% | HIGH |
| warm-clamp-head | 109.4ns | 398.6ns | 27.4% | HIGH |
| warm-clamp-min-lanes | 100.6ns | 347.2ns | 29.0% | HIGH |
| warm-clamp-minimum | 105.5ns | 380.8ns | 27.7% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 327.2-421.2 ns)
    327.2 |
    331.9 |
    336.6 |########################################
    341.3 |
    346.0 |
    350.7 |
    355.4 |
    360.1 |##
    364.8 |
    369.5 |
    374.2 |##
    378.9 |######################
    383.6 |
    388.3 |##
    393.0 |##
    397.7 |
    402.4 |
    407.1 |
    411.8 |##
    416.5 |
  (3 below, 4 above range)

warm-clamp-accfit (n=40, range 306.3-438.2 ns)
    306.3 |################
    312.9 |
    319.5 |
    326.1 |
    332.7 |
    339.3 |########################################
    345.9 |
    352.5 |
    359.1 |
    365.7 |######
    372.3 |
    378.9 |################
    385.5 |###
    392.1 |
    398.7 |###
    405.3 |###
    411.9 |
    418.5 |
    425.1 |
    431.7 |####################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 302.4-388.3 ns)
    302.4 |#############
    306.7 |
    311.0 |
    315.3 |##
    319.6 |
    323.9 |
    328.2 |
    332.5 |#############
    336.8 |######
    341.1 |
    345.4 |
    349.7 |
    354.0 |
    358.3 |
    362.5 |
    366.8 |
    371.1 |
    375.4 |########################################
    379.7 |
    384.0 |
  (4 below, 2 above range)

warm-clamp-head (n=40, range 338.3-535.2 ns)
    338.3 |########################################
    348.1 |######
    358.0 |###
    367.8 |
    377.7 |###########################
    387.5 |
    397.3 |
    407.2 |
    417.0 |
    426.9 |###############
    436.7 |
    446.6 |
    456.4 |
    466.3 |
    476.1 |
    486.0 |###
    495.8 |
    505.7 |#########
    515.5 |
    525.4 |###
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 276.1-424.0 ns)
    276.1 |#########################
    283.5 |
    290.9 |
    298.3 |########################################
    305.7 |
    313.1 |
    320.5 |#####
    327.9 |
    335.3 |#####
    342.7 |
    350.0 |
    357.4 |
    364.8 |#####
    372.2 |##############################
    379.6 |###################################
    387.0 |
    394.4 |
    401.8 |
    409.2 |#####
    416.6 |
  (4 below, 6 above range)

warm-clamp-minimum (n=40, range 305.1-489.3 ns)
    305.1 |################################
    314.3 |
    323.5 |
    332.7 |########################################
    341.9 |
    351.2 |
    360.4 |
    369.6 |
    378.8 |########################
    388.0 |
    397.2 |####
    406.4 |
    415.6 |
    424.9 |
    434.1 |################################
    443.3 |
    452.5 |####
    461.7 |
    470.9 |
    480.1 |####
  (2 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=28.1% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=28.0% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=28.1% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=20.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=28.0% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=28.6% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=28.1% of algo (FFI overhead may distort results)
