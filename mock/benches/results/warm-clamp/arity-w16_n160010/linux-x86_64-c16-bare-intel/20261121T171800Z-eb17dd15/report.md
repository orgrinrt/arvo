# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn beats baseline by 21% (significant)

warm-clamp-accfit-dyn is -673 ns (21%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-clamp-accfit-dyn, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-accfit-dyn (2.62 us) and warm-clamp-min-lanes (2.63 us) differ by 0.35%, inside the noise, even though the wider field spreads 25.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.76)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader warm-clamp-accfit-dyn vs stability leader warm-clamp-accfit (+9% speed for 1.5x steadier)

warm-clamp-accfit-dyn is fastest (2.62 us, CV 13.8%); warm-clamp-accfit gives up 9.2% median for 1.5x lower variance (CV 9.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.49 us but its worst 20% at 6.01 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2624.4 ns median (-18.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.26x (fastest 2624.4 ns, slowest 3303.2 ns)

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
| warm-clamp-acc64 | 3692ns | 3360ns | 1557ns | 3532ns | 6309ns | base |
| warm-clamp-accfit | 3148ns | 3092ns | 2766ns | 3126ns | 3598ns | -14.74% |
| warm-clamp-accfit-dyn | 2901ns | 2831ns | 2425ns | 2855ns | 3514ns | -21.43% |
| warm-clamp-head | 3122ns | 3143ns | 2458ns | 3181ns | 3612ns | -15.44% |
| warm-clamp-min-lanes | 2888ns | 2797ns | 2567ns | 2850ns | 3322ns | -21.79% |
| warm-clamp-minimum | 3184ns | 3509ns | 1236ns | 3432ns | 4387ns | -13.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3532ns | 1493ns | 6011ns | base | 2.319 |
| warm-clamp-accfit | 2905ns | 2566ns | 3289ns | -17.76% | 2.820 |
| warm-clamp-accfit-dyn | 2679ns | 2247ns | 3231ns | -24.15% | 3.058 |
| warm-clamp-head | 2908ns | 2295ns | 3323ns | -17.67% | 2.817 |
| warm-clamp-min-lanes | 2688ns | 2362ns | 3077ns | -23.90% | 3.048 |
| warm-clamp-minimum | 2986ns | 1164ns | 4091ns | -15.47% | 2.744 |

## Performance model

- Peak throughput: **7.041 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.540 | 36.1% |
| warm-clamp-accfit | 2.860 | 40.6% |
| warm-clamp-accfit-dyn | 3.121 | 44.3% |
| warm-clamp-head | 2.789 | 39.6% |
| warm-clamp-min-lanes | 3.111 | 44.2% |
| warm-clamp-minimum | 2.480 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3692ns | 3692ns | base |
| warm-clamp-accfit | 3148ns | 3148ns | -14.74% |
| warm-clamp-accfit-dyn | 2901ns | 2901ns | -21.43% |
| warm-clamp-head | 3122ns | 3122ns | -15.44% |
| warm-clamp-min-lanes | 2888ns | 2888ns | -21.79% |
| warm-clamp-minimum | 3184ns | 3184ns | -13.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3225ns | base | --- | [2502, 4526] | --- | --- | --- | --- |
| warm-clamp-accfit | 2865ns | no significant difference | [-1431, +357]ns | [2862, 2887] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 2624ns | -672.7ns (-20.9%) | [-1838, -99]ns | [2479, 2843] | YES (adj: no) | 0.0641 | 0.0166 | 0 |
| warm-clamp-head | 2937ns | no significant difference | [-1243, +244]ns | [2910, 3090] | no | 0.7948 | 0.6358 | 0 |
| warm-clamp-min-lanes | 2634ns | -651.4ns (-20.2%) | [-1480, -87]ns | [2576, 2795] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-minimum | 3303ns | -477.9ns (-14.8%) | [-1243, -85]ns | [2924, 3625] | YES (adj: no) | 0.0641 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4147ns | -30.9% | -42.4% | -27.9% | -32.6% | -27.6% |
| 1 | 2 | 4365ns | -34.3% | -33.4% | -33.3% | -36.0% | -18.8% |
| 1 | 3 | 2509ns | +14.1% | -3.9% | +14.4% | -3.8% | +38.1% |
| 1 | 4 | 2587ns | +12.6% | +3.8% | +23.0% | +1.3% | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.199 | ok |
| warm-clamp-accfit | -0.126 | ok |
| warm-clamp-accfit-dyn | -0.763 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.243 | moderate- |
| warm-clamp-min-lanes | -0.059 | ok |
| warm-clamp-minimum | -0.240 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 61.4ns | 3532.0ns | 1.7% |  |
| warm-clamp-accfit | 87.1ns | 2904.8ns | 3.0% |  |
| warm-clamp-accfit-dyn | 78.5ns | 2679.2ns | 2.9% |  |
| warm-clamp-head | 75.7ns | 2907.9ns | 2.6% |  |
| warm-clamp-min-lanes | 70.5ns | 2687.8ns | 2.6% |  |
| warm-clamp-minimum | 68.1ns | 2985.6ns | 2.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1493.4-6010.9 ns)
   1493.4 |########################################
   1719.3 |
   1945.1 |
   2171.0 |
   2396.9 |###################################
   2622.8 |#####
   2848.6 |
   3074.5 |
   3300.4 |
   3526.3 |#####
   3752.2 |#####
   3978.0 |##########
   4203.9 |#####
   4429.8 |##########
   4655.7 |
   4881.5 |###############
   5107.4 |
   5333.3 |#########################
   5559.2 |##########
   5785.1 |#####
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 2565.6-3289.2 ns)
   2565.6 |##
   2601.8 |####
   2638.0 |##
   2674.2 |
   2710.3 |
   2746.5 |
   2782.7 |####
   2818.9 |
   2855.0 |########################################
   2891.2 |####
   2927.4 |##
   2963.6 |##
   2999.8 |##
   3035.9 |##
   3072.1 |
   3108.3 |
   3144.5 |######
   3180.7 |##
   3216.8 |####
   3253.0 |
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 2247.1-3230.7 ns)
   2247.1 |
   2296.3 |
   2345.5 |########################################
   2394.7 |######
   2443.8 |#############
   2493.0 |######
   2542.2 |
   2591.4 |##########################
   2640.6 |####################
   2689.7 |#############
   2738.9 |
   2788.1 |######
   2837.3 |#################################
   2886.4 |####################
   2935.6 |
   2984.8 |
   3034.0 |
   3083.2 |######
   3132.3 |
   3181.5 |#############
  (6 below, 3 above range)

warm-clamp-head (n=40, range 2294.7-3322.6 ns)
   2294.7 |
   2346.1 |
   2397.5 |
   2448.9 |###########
   2500.3 |#####
   2551.7 |
   2603.1 |
   2654.5 |############################
   2705.9 |#####
   2757.3 |###########
   2808.7 |
   2860.0 |#####
   2911.4 |########################################
   2962.8 |#################
   3014.2 |###########
   3065.6 |
   3117.0 |#####
   3168.4 |#####
   3219.8 |########################################
   3271.2 |###########
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2361.9-3076.8 ns)
   2361.9 |######
   2397.7 |##########################
   2433.4 |######
   2469.2 |
   2504.9 |####################
   2540.6 |#############
   2576.4 |##########################
   2612.1 |####################
   2647.9 |######
   2683.6 |######
   2719.3 |######
   2755.1 |
   2790.8 |########################################
   2826.6 |####################
   2862.3 |
   2898.1 |
   2933.8 |######
   2969.5 |######
   3005.3 |######
   3041.0 |
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 1163.5-4090.6 ns)
   1163.5 |
   1309.9 |
   1456.2 |
   1602.6 |
   1748.9 |
   1895.3 |
   2041.6 |
   2188.0 |#####
   2334.3 |#####
   2480.7 |
   2627.0 |###########
   2773.4 |#################
   2919.7 |#################
   3066.1 |
   3212.4 |########################################
   3358.8 |#####
   3505.1 |##################################
   3651.5 |######################
   3797.8 |#####
   3944.2 |#####
  (7 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=50.4% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=35.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.5x (possible bimodal distribution)
