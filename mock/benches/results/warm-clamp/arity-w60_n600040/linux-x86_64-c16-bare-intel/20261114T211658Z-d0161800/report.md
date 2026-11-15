# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.68)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (2.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-clamp-head is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-head's best 20% of batches run at 2.07 us but its worst 20% at 8.83 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 2180.4 ns median
- 4 variants significantly slower than baseline
- Spread: 1.40x (fastest 2180.4 ns, slowest 3055.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2413ns | 2413ns | 2163ns | 2392ns | 2726ns | base |
| warm-clamp-accfit | 2554ns | 2493ns | 2289ns | 2536ns | 2874ns | +5.86% |
| warm-clamp-accfit-dyn | 2630ns | 2603ns | 2313ns | 2596ns | 3048ns | +8.99% |
| warm-clamp-head | 4432ns | 3140ns | 2144ns | 3630ns | 9126ns | +83.69% |
| warm-clamp-min-lanes | 3947ns | 2881ns | 2511ns | 2966ns | 8326ns | +63.58% |
| warm-clamp-minimum | 2666ns | 2501ns | 2210ns | 2596ns | 3335ns | +10.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2172ns | 1958ns | 2428ns | base | 3.772 |
| warm-clamp-accfit | 2298ns | 2060ns | 2567ns | +5.81% | 3.565 |
| warm-clamp-accfit-dyn | 2366ns | 2087ns | 2731ns | +8.95% | 3.462 |
| warm-clamp-head | 4301ns | 2074ns | 8831ns | +98.07% | 1.905 |
| warm-clamp-min-lanes | 3814ns | 2433ns | 8027ns | +75.63% | 2.148 |
| warm-clamp-minimum | 2404ns | 2005ns | 2993ns | +10.69% | 3.408 |

## Performance model

- Peak throughput: **4.184 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.757 | 89.8% |
| warm-clamp-accfit | 3.634 | 86.9% |
| warm-clamp-accfit-dyn | 3.484 | 83.3% |
| warm-clamp-head | 2.681 | 64.1% |
| warm-clamp-min-lanes | 2.941 | 70.3% |
| warm-clamp-minimum | 3.617 | 86.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2413ns | 2413ns | base |
| warm-clamp-accfit | 2554ns | 2554ns | +5.86% |
| warm-clamp-accfit-dyn | 2630ns | 2630ns | +8.99% |
| warm-clamp-head | 4432ns | 4432ns | +83.69% |
| warm-clamp-min-lanes | 3947ns | 3947ns | +63.58% |
| warm-clamp-minimum | 2666ns | 2666ns | +10.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2180ns | base | --- | [2067, 2227] | --- | --- | --- | --- |
| warm-clamp-accfit | 2254ns | no significant difference | [-0, +96]ns | [2224, 2398] | no | 0.1539 | 0.1539 | 0 |
| warm-clamp-accfit-dyn | 2351ns | +130.4ns (+6.0%) | [+82, +202]ns | [2241, 2411] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 3055ns | +725.2ns (+33.3%) | [+548, +1309]ns | [2504, 3481] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2785ns | +684.8ns (+31.4%) | [+477, +905]ns | [2733, 2906] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2265ns | +103.1ns (+4.7%) | [+17, +210]ns | [2178, 2504] | YES | 0.0481 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2247ns | -0.4% | -0.9% | +37.3% | +23.5% | +8.1% |
| 1 | 2 | 2255ns | -1.4% | +6.9% | +7.9% | +12.1% | -2.1% |
| 1 | 3 | 2068ns | +7.4% | +25.7% | +21.8% | +76.1% | +38.1% |
| 1 | 4 | 2039ns | +21.5% | +8.8% | +22.8% | +42.6% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.269 | moderate+ |
| warm-clamp-accfit | -0.108 | ok |
| warm-clamp-accfit-dyn | -0.303 | moderate- |
| warm-clamp-head | -0.188 | ok |
| warm-clamp-min-lanes | -0.365 | moderate- |
| warm-clamp-minimum | -0.677 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 66.2ns | 2171.6ns | 3.0% |  |
| warm-clamp-accfit | 68.4ns | 2297.8ns | 3.0% |  |
| warm-clamp-accfit-dyn | 72.0ns | 2366.0ns | 3.0% |  |
| warm-clamp-head | 35.0ns | 4301.3ns | 0.8% |  |
| warm-clamp-min-lanes | 32.7ns | 3813.9ns | 0.9% |  |
| warm-clamp-minimum | 71.7ns | 2403.8ns | 3.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1958.1-2427.5 ns)
   1958.1 |######
   1981.6 |
   2005.0 |###
   2028.5 |
   2052.0 |########################################
   2075.4 |
   2098.9 |
   2122.4 |###
   2145.9 |
   2169.3 |
   2192.8 |
   2216.3 |#######################
   2239.7 |######
   2263.2 |##########
   2286.7 |###
   2310.2 |
   2333.6 |
   2357.1 |
   2380.6 |
   2404.0 |#############
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 2059.6-2567.2 ns)
   2059.6 |###################################
   2085.0 |
   2110.4 |
   2135.7 |#####
   2161.1 |##########
   2186.5 |#####
   2211.9 |###################################
   2237.3 |###############
   2262.6 |
   2288.0 |
   2313.4 |##########
   2338.8 |#####
   2364.2 |
   2389.5 |########################################
   2414.9 |
   2440.3 |
   2465.7 |
   2491.0 |
   2516.4 |#####
   2541.8 |###############
  (1 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2087.4-2730.9 ns)
   2087.4 |#####
   2119.5 |#################
   2151.7 |#####
   2183.9 |###########
   2216.1 |#################
   2248.3 |#################
   2280.4 |
   2312.6 |#################
   2344.8 |#####
   2377.0 |#####
   2409.2 |########################################
   2441.3 |###########
   2473.5 |#####
   2505.7 |
   2537.9 |#####
   2570.0 |
   2602.2 |###########
   2634.4 |
   2666.6 |###########
   2698.8 |
  (4 below, 3 above range)

warm-clamp-head (n=40, range 2074.4-8831.4 ns)
   2074.4 |##
   2412.3 |########################################
   2750.1 |#################
   3088.0 |##
   3425.8 |
   3763.7 |#####
   4101.5 |
   4439.4 |##
   4777.2 |
   5115.1 |
   5452.9 |
   5790.8 |
   6128.6 |
   6466.5 |
   6804.3 |
   7142.2 |
   7480.0 |#####
   7817.9 |#####
   8155.7 |##
   8493.6 |########
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2433.0-8027.3 ns)
   2433.0 |##################
   2712.7 |########################################
   2992.4 |##
   3272.2 |
   3551.9 |
   3831.6 |
   4111.3 |
   4391.0 |
   4670.7 |##
   4950.4 |
   5230.1 |
   5509.9 |
   5789.6 |
   6069.3 |
   6349.0 |
   6628.7 |##
   6908.4 |####
   7188.1 |
   7467.8 |##
   7747.6 |##
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 2004.6-2993.1 ns)
   2004.6 |##############################
   2054.1 |########################################
   2103.5 |##############################
   2152.9 |####################
   2202.3 |########################################
   2251.8 |####################
   2301.2 |
   2350.6 |##########
   2400.0 |##############################
   2449.5 |##########
   2498.9 |##########
   2548.3 |
   2597.7 |########################################
   2647.2 |##############################
   2696.6 |
   2746.0 |
   2795.4 |##########
   2844.8 |##########
   2894.3 |
   2943.7 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-head**: CV=62.9% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=57.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.3x (possible bimodal distribution)
